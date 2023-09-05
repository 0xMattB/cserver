use std::env;

use cserver::modules::console;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
	
	if args.len() == 2 {
		console::output(&format!["\ncserver ({})\n", VERSION], true);
		let _ = cserver::run(&args[1]).await;
	} else {
		console::output("invalid number of arguments; usage:", true);
		console::output("cserver.exe (filename)", true);
	}
}