use so_rust::so::reqrep::*;


fn main() {
    let client = ReqRep::new(Type::Client, String::from("tcp://127.0.0.1:1234"));
    client.start();
	loop {
		let req:String = String::from("REQ : Tell me who are you ?");
		let mut resp:String = "".to_string();
		client.client_process(&req,&mut resp);
		println!("{}\n{}\n\n",req,resp);
	}
}
