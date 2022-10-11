use so_rust::so::channel::*;

fn main() {
	let reader : ChannelIn = ChannelIn::new(String::from("tcp://127.0.0.1:4540"));
	reader.start();

	loop {
		let msg :String = reader.pull();
		println!("{}",msg);
	}
}