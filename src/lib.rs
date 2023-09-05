pub mod modules;

use crate::modules::file;
use crate::modules::console;
use crate::modules::defaults;
use crate::modules::command;
use crate::modules::command::CommandType;
use crate::modules::ip::IP;
use crate::modules::list::List;

use tokio::net::tcp::{ReadHalf, WriteHalf};
use tokio::net::TcpListener;
use tokio::sync::broadcast::{Sender, Receiver};
use std::error::Error;
use std::process;
use std::sync::{Arc, Mutex};

enum ServerState {
	UserPrompt,
	UserReceive,
	UserCheck,
	PasswordPrompt,
	PasswordReceive,
	PasswordCheck,
	MessageLogIn,
	MessageInvalidPassword,
	NewUserPrompt,
	NewUserReceive,
	NewPassword1Prompt,
	NewPassword1Receive,
	NewPassword2Prompt,
	NewPassword2Receive,
	NewPasswordCheck,
	NewUserAdd,
	NewUserMessage,
	Connected,
	Disconnect,
}

pub async fn run(filename: &str) -> Result<(), Box<dyn Error>> {
	let ulist: List;
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
		ulist = dlist;
	} else {
		console::output("error loading data contents", true);
		process::exit(0);
	}
	
	// create entites shared across client tasks
	let user_list = Arc::new(Mutex::new(ulist));
	let fname = Arc::new(Mutex::new(String::from(filename)));
	let (tx_broadcast, _): (Sender<_>, Receiver<_>) = tokio::sync::broadcast::channel(16);
	
    // listen for connections on specified port
	let listener = TcpListener::bind(&ip).await?;
    console::output(&format!["Listening on: {}", ip], true);
	
	// connection check loop
	loop {
		match listener.accept().await {
			Ok((mut socket, addr)) => {
				console::output(&format!["Connection from: {}", addr], true);
				let user_list = user_list.clone();
				let fname = fname.clone();
				let tx_broadcast = tx_broadcast.clone();
				let mut rx_broadcast = tx_broadcast.subscribe();
				
				tokio::spawn(async move {
					let mut state = ServerState::UserPrompt;	
					let mut user = String::default();
					let mut password = String::default();
					let mut password1 = String::default();
					let mut password2 = String::default();
					let (mut reader, mut writer) = socket.split();
					
					loop {
						match state {
							ServerState::UserPrompt => {
								transmit(&mut writer, "Username: ").await;
								state = ServerState::UserReceive;
							},
							
							ServerState::UserReceive => {
								if let Some(s) = receive(&mut reader).await {
									user = String::from(s);
									state = ServerState::UserCheck;
								}
							},
							
							ServerState::UserCheck => {
								let user_list = user_list.lock().unwrap();
								
								if user_list.check_key(&user) {
									state = ServerState::PasswordPrompt;
								} else {
									state = ServerState::NewUserPrompt;
								}
							},
							
							ServerState::PasswordPrompt => {
								transmit(&mut writer, "Password: ").await;
								state = ServerState::PasswordReceive;
							},
							
							ServerState::PasswordReceive => {
								if let Some(s) = receive(&mut reader).await {
									password = String::from(s);
									state = ServerState::PasswordCheck;
								}
							},
							
							ServerState::PasswordCheck => {
								let user_list = user_list.lock().unwrap();
								
								if user_list.check(&user, &password) {
									state = ServerState::MessageLogIn;
								} else {
									state = ServerState::MessageInvalidPassword;
								}
							},
							
							ServerState::MessageLogIn => {
								transmit(&mut writer, "Logged in").await;
								state = ServerState::Connected;
							}
							
							ServerState::MessageInvalidPassword => {
								transmit(&mut writer, "Invalid password").await;
								state = ServerState::Disconnect;
							}
							
							ServerState::NewUserPrompt => {
								transmit(&mut writer, "User not found - create new account? (y/n): ").await;
								state = ServerState::NewUserReceive;
							},
							
							ServerState::NewUserReceive => {
								if let Some(s) = receive(&mut reader).await {
									if s == "y" || s == "Y" {
										state = ServerState::NewPassword1Prompt;
									} else {
										transmit(&mut writer, "Invalid username").await;
										state = ServerState::Disconnect;
									}
								}
							},
							
							ServerState::NewPassword1Prompt => {
								transmit(&mut writer, "Enter new password: ").await;
								state = ServerState::NewPassword1Receive;
								
							},
							
							ServerState::NewPassword1Receive => {
								if let Some(s) = receive(&mut reader).await {
									password1 = String::from(s);
									state = ServerState::NewPassword2Prompt;
								}
							},
							
							ServerState::NewPassword2Prompt => {
								transmit(&mut writer, "Re-enter password : ").await;
								state = ServerState::NewPassword2Receive;
							},
							
							ServerState::NewPassword2Receive => {
								if let Some(s) = receive(&mut reader).await {
									password2 = String::from(s);
									state = ServerState::NewPasswordCheck;
								}
							},
							
							ServerState::NewPasswordCheck => {
								if password1 == password2 {
									state = ServerState::NewUserAdd;
								} else {
									transmit(&mut writer, "Invalid (passwords don't match)").await;
									state = ServerState::Disconnect;
								}
							},
							
							ServerState::NewUserAdd => {
								let mut user_list = user_list.lock().unwrap();
								let fname = fname.lock().unwrap();
								
								user_list.add(&user, &password1);
								let _ = file::write(&fname, &get_user_list_as_string(&user_list));
								state = ServerState::NewUserMessage;
								
							},
							
							ServerState::NewUserMessage => {
								transmit(&mut writer, "Logged in").await;
								state = ServerState::Connected;
							}
							
							ServerState::Connected => {

								tokio::select! {
									// data available from a client, which is then broadcast to all clients
									result = receive(&mut reader) => {
										match result {
											Some(data) => {
												match command::command(&data) {
													CommandType::None => {
														let updated = format!["{}: {}", user, data];
														tx_broadcast.send(updated.into_bytes()).unwrap();
													},
													CommandType::Party => {
														let updated = format!["*** {} is having a party! ***", user];
														tx_broadcast.send(updated.into_bytes()).unwrap();
													},
													CommandType::Exit => {
														let updated = format!["!exit"];
														tx_broadcast.send(updated.into_bytes()).unwrap();
														process::exit(0);
													},
												}
											},
											None => {},
										}
									}
									
									// broadcast available, to be sent to all logged clients
									result = rx_broadcast.recv() => {
										let msg = result.unwrap();
										let msg = std::str::from_utf8(&msg).unwrap();
										transmit(&mut writer, &msg).await;
									}
								}
							},
							
							ServerState::Disconnect => {
								break;
							},
						}
					}
				});
			},
			Err(_) => {},  // connection error
		};
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

async fn transmit(writer: &mut WriteHalf<'_>, s: &str) {
	writer.try_write(s.as_bytes()).expect("failed to write data to socket");
}

async fn receive(reader: &mut ReadHalf<'_>) -> Option<String> {
	let mut buf = vec![0; 1024];

	match reader.try_read(&mut buf) {
		Ok(_) => {
			let s = match std::str::from_utf8(&buf) {
				Ok(v) => { v },
				Err(_) => { return None; },
			};
			
			let s = clean_string(&String::from(s));
			let s = trim_null(&s);

			return Some(s);
		},
		Err(_) => {
			return None;
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