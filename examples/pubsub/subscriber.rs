use so_rust::so::pubsub::*;

fn ma(topic:String,message:String)
{
    println!("{} : {} ",topic,message);
}

fn main()
{
    let mut subs = Subscriber::new(String::from("tcp://127.0.0.1:12345"));
    let topic = String::from("lamp");
    subs.start();
    subs.subscribe(topic,ma);
    loop {
        subs.process();
    }

}