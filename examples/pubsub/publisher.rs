use so_rust::so::pubsub::*;
use so_rust::so::layer::Layer;

fn main()
{
    let publisher = Publisher::new(Layer::ipc("service","pub-sub"));
    publisher.start();
    loop {
        let topic = String::from("lamp");
        let payload = String::from("on");
        publisher.publish(topic,payload);
    }
}