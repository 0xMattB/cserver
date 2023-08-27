pub mod modules;

use crate::modules::file;
use crate::modules::console;
use crate::modules::defaults;
use crate::modules::ip::IP;
use crate::modules::list::List;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use std::error::Error;
use std::process;

enum ServerState {
	UserPrompt,
	UserReceive,
	UserCheck,
	PasswordPrompt,
	PasswordReceive,
	PasswordCheck,
	NewUserPrompt,
	NewUserReceive,
	NewPassword1Prompt,
	NewPassword1Receive,
	NewPassword2Prompt,
	NewPassword2Receive,
	NewPasswordCheck,
	NewUserAdd,
	Connected,
	Disconnect,
}

pub async fn run(filename: &str) -> Result<(), Box<dyn Error>> {
	let mut user_list: List;
	let ip: String;

	// load ip address
	match IP::new(defaults::DEFAULT_IP) {
		Ok(ip_type) => {
			ip = ip_type.get();
		},
		Err(e) => {
			console::output(&format!["{}", e], true);
			process::exit(0);
		},
	}

	// load file data
	let file_data = load_data(filename);
	
	// store file data into memory
	if let Some(dlist) = create_data_list(&file_data) {
		user_list = dlist;
	} else {
		console::output("error loading data contents", true);
		process::exit(0);
	}
	
    let listener = TcpListener::bind(&ip).await?;
    println!("Listening on: {}", ip);
	
    loop {
        // Asynchronously wait for an inbound socket.
        let (mut socket, _) = listener.accept().await?;
		let mut state = ServerState::UserPrompt;	
		let mut user = String::default();
		let mut password = String::default();
		let mut password1 = String::default();
		let mut password2 = String::default();
		let fname = String::from(filename);

        // And this is where much of the magic of this server happens. We
        // crucially want all clients to make progress concurrently, rather than
        // blocking one on completion of another. To achieve this we use the
        // `tokio::spawn` function to execute the work in the background.
        //
        // Essentially here we're executing a new task to run concurrently,
        // which will allow all of our clients to be processed concurrently.

        user_list = tokio::spawn(async move {
            //let mut user = String::default();
            //let mut password = String::default();
            //let mut password1 = String::default();
            //let mut password2 = String::default();
			
			loop {
                //let mut buf = vec![0; 1024];
				
				match state {
					ServerState::UserPrompt => {
						transmit(&mut socket, "Username: ").await;
						state = ServerState::UserReceive;
					},
					
					ServerState::UserReceive => {
						match receive(&mut socket).await {
							Ok(s) => {
								let s = clean_string(&String::from(s));
								println!("Received: '{}'", s);
								user = String::from(s);
								user = trim_null(&user);
								state = ServerState::UserCheck;
							},
							Err(e) => {
								println!("{}", e);
								break;
							},
						}
					},
					
					ServerState::UserCheck => {
						if user_list.check_key(&user) {
							state = ServerState::PasswordPrompt;
						} else {
							state = ServerState::NewUserPrompt;
						}
					},
					
					ServerState::PasswordPrompt => {
						transmit(&mut socket, "Password: ").await;
						state = ServerState::PasswordReceive;
					},
					
					ServerState::PasswordReceive => {
						match receive(&mut socket).await {
							Ok(s) => {
								let s = clean_string(&String::from(s));
								println!("Received: '{}'", s);
								password = String::from(s);
								password = trim_null(&password);
								state = ServerState::PasswordCheck;
							},
							Err(e) => {
								println!("{}", e);
								break;
							},
						}
					},
					
					ServerState::PasswordCheck => {
						if user_list.check(&user, &password) {
							transmit(&mut socket, "Logged in").await;
							state = ServerState::Connected;
						} else {
							transmit(&mut socket, "Invalid password").await;
							state = ServerState::Disconnect;
						}
					},
					
					ServerState::NewUserPrompt => {
						transmit(&mut socket, "User not found - create new account? (y/n): ").await;
						state = ServerState::NewUserReceive;
					},
					
					ServerState::NewUserReceive => {
						match receive(&mut socket).await {
							Ok(s) => {
								let mut s = clean_string(&String::from(s));
								s = trim_null(&s);
								println!("Received: '{}'", s);
								
								if s == "y" || s == "Y" {
									state = ServerState::NewPassword1Prompt;
								} else {
									transmit(&mut socket, "Invalid username").await;
									state = ServerState::Disconnect;
								}
							},
							Err(e) => {
								println!("{}", e);
								break;
							},
						}
					},
					
					ServerState::NewPassword1Prompt => {
						transmit(&mut socket, "Enter new password: ").await;
						state = ServerState::NewPassword1Receive;
						
					},
					
					ServerState::NewPassword1Receive => {
						match receive(&mut socket).await {
							Ok(s) => {
								let s = clean_string(&String::from(s));
								println!("Received: '{}'", s);
								password1 = String::from(s);
								password1 = trim_null(&password1);
								state = ServerState::NewPassword2Prompt;
							},
							Err(e) => {
								println!("{}", e);
								break;
							},
						}
					},
					
					ServerState::NewPassword2Prompt => {
						transmit(&mut socket, "Re-enter password : ").await;
						state = ServerState::NewPassword2Receive;
					},
					
					ServerState::NewPassword2Receive => {
						match receive(&mut socket).await {
							Ok(s) => {
								let s = clean_string(&String::from(s));
								println!("Received: '{}'", s);
								password2 = String::from(s);
								password2 = trim_null(&password2);
								state = ServerState::NewPasswordCheck;
							},
							Err(e) => {
								println!("{}", e);
								break;
							},
						}
					},
					
					ServerState::NewPasswordCheck => {
						if password1 == password2 {
							state = ServerState::NewUserAdd;
						} else {
							transmit(&mut socket, "Invalid (passwords don't match)").await;
							state = ServerState::Disconnect;
						}
					},
					
					ServerState::NewUserAdd => {
						user_list.add(&user, &password1);
						let _ = file::write(&fname, &get_user_list_as_string(&user_list));
						transmit(&mut socket, "Logged in").await;
						state = ServerState::Connected;
					},
					
					ServerState::Connected => {
						match receive(&mut socket).await {
							Ok(s) => {
								let mut s = clean_string(&String::from(s));
								s = trim_null(&s);
								println!("Received: '{}'", s);
								
								if s == "!shutdown" {
									state = ServerState::Disconnect;
								} else {
									let response = format!["{}: {}", user, s];
									transmit(&mut socket, &response).await;
								}
							},
							Err(e) => {
								println!("{}", e);
								break;
							},
						}
						
						// TODO: change state
					},
					
					ServerState::Disconnect => {
						//process::exit(0);
						break;
					},
				}
            }
			
			return user_list;
        }).await.unwrap();
    }
}

fn load_data(filename: &str) -> String {
	if let Some(data) = file::read(filename) {
		data
	} else {
		console::output(&format!["{filename} not found, a new file will be created\n"], true);
		"".to_string()
	}
}

fn create_data_list(data: &str) -> Option<List> {
	let mut list = List::new();
	
	if data.len() > 0 {
		for line in data.split("\n") {
			if line.len() == 0 {
				continue;
			}
			
			let args: Vec<_> = line.split("\t").collect();
			
			if args.len() != 2 {
				return None;
			}
			
			list.add(args[0], args[1]);
		}
	}
	
	Some(list)
}

async fn transmit(socket: &mut TcpStream, s: &str) {
	socket.write_all(s.as_bytes()).await.expect("failed to write data to socket");
}

async fn receive(socket: &mut TcpStream) -> Result<String, String> {
	let mut buf = vec![0; 1024];
	let n = socket.read(&mut buf).await;

	match n {
		Ok(_) => {
			let s = match std::str::from_utf8(&buf) {
				Ok(v) => { v },
				Err(e) => { return Err(format!("Invalid UTF-8 sequence: {}", e)); },
			};
			
			return Ok(String::from(s));
		},
		Err(e) => {
			return Err(format!["error: {}", e]);
		},
	};
}

fn clean_string(s: &String) -> String {
	let s = s.replace("\r", "\0");
	let s = s.replace("\n", "\0");
	s
}

fn trim_null(s: &String) -> String {
	String::from(s.trim_matches(char::from(0)))
}

fn get_user_list_as_string(ulist: &List) -> String {
	let mut s = String::from("");
	
	if let Some(iter) = ulist.get()
	{
		for (k, v) in iter {
			s.push_str(&format!["{}\t{}\n", k, v]);
		}
	}
	
	s
}