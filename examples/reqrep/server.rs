use so_rust::so::reqrep::*;

fn process_request(_request: &String) -> String {
    "RESP : I am Rust Server".to_string()
}

fn main() {
    let my_server = ReqRepServer::new(String::from("tcp://127.0.0.1:1234"));
    my_server.start();
    loop {
        my_server.server_process(process_request);
    }
}
