use so_rust::so::channel::*;


fn main() {
	let writer : ChannelOut = ChannelOut::new(String::from("tcp://127.0.0.1:4540"));
	writer.start();

	loop {
		let msg :String = String::from("I am sent by Writer");
		writer.push(msg);
	}
}