use so_rust::so::pubsub::*;
use so_rust::so::layer::Layer;

fn ma(topic:String,message:String)
{
    println!("{} : {} ",topic,message);
}

fn main()
{
    let mut subs = Subscriber::new(Layer::ipc("service","pub-sub"));
    let topic = String::from("lamp");
    subs.start();
    subs.subscribe(topic,ma);
    loop {
        subs.process();
    }

}