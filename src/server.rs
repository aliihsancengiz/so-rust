mod reqrep;
use crate::reqrep::Sync;

fn process_request(request: &String) -> String {
    "RESP : I am Rust Server".to_string()
}

fn main() {
    let my_server = reqrep::ReqRep::new(reqrep::Type::Server, String::from("tcp://127.0.0.1:1234"));
    my_server.start();
	loop {
		my_server.server_process(process_request);
	}
}
