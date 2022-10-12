use crate::so::socket::{Socket,Type};
use std::collections::HashMap;

type MessageArrivedCallback = fn(topic:String,message:String);

pub struct Publisher
{
    socket: Socket,
}

pub trait SyncPublisher
{
    fn new(socket_addr: String) -> Self;
    fn start(&self);
    fn publish(&self,topic:String, message:String);
}

impl SyncPublisher for Publisher
{
    fn new(socket_addr: String) -> Self
    {
        Self {
            socket: Socket::new(socket_addr,Type::Publisher),
        }
    }
    fn start(&self) {
        self.socket.start();
    }
    fn publish(&self,topic:String, message:String)
    {
        self.socket.socket_handle.send(&topic, zmq::SNDMORE).unwrap();
        self.socket.socket_handle.send(&message, 0).unwrap();
    }
}

pub struct Subscriber 
{
    socket: Socket,
    callback_map : HashMap<String,MessageArrivedCallback>,
}

pub trait SyncSubscriber
{
    fn new(socket_addr: String) -> Self;
    fn start(&self);
    fn subscribe(&mut self,topic:String,cb:MessageArrivedCallback);
    fn process(&self);
}

impl SyncSubscriber for Subscriber
{
    fn new(socket_addr: String) -> Self
    {
        Self {
            socket: Socket::new(socket_addr,Type::Subscriber),
            callback_map : HashMap::new(),
        }
    }

    fn start(&self) {
        self.socket.start();
    }

    fn subscribe(&mut self,topic:String,cb:MessageArrivedCallback)
    {
        self.socket.socket_handle.set_subscribe(&topic[..].as_bytes()).unwrap();
        self.callback_map.insert(topic,cb);
    }
    fn process(&self)
    {
        let topic : String = self.socket.socket_handle.recv_msg(0).unwrap().as_str().unwrap().to_string();
        let payload : String = self.socket.socket_handle.recv_msg(0).unwrap().as_str().unwrap().to_string();
        if self.callback_map.contains_key(&topic) {
            self.callback_map[&topic](topic,payload);
        }

    }
}