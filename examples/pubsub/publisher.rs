use so_rust::so::pubsub::*;

fn main()
{
    let publisher = Publisher::new(String::from("tcp://127.0.0.1:12345"));
    publisher.start();
    loop {
        let topic = String::from("lamp");
        let payload = String::from("on");
        publisher.publish(topic,payload);
    }
}