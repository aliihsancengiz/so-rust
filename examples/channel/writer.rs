use so_rust::so::channel::*;
use so_rust::so::layer::Layer;

fn main() {
    let writer : ChannelOut = ChannelOut::new(Layer::ipc("service","reader-writer"));
    writer.start();

    loop {
        let msg :String = String::from("I am sent by Writer");
        writer.push(msg);
    }
}