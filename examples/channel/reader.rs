use so_rust::so::channel::*;
use so_rust::so::layer::Layer;

fn main() {
    let reader : ChannelIn = ChannelIn::new(Layer::ipc("service","reader-writer"));
    reader.start();

    loop {
        let msg :String = reader.pull();
        println!("{}",msg);
    }
}