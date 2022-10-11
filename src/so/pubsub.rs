use crate::so::socket::Socket;
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
            socket: Socket {
                socket_handle: zmq::Context::new().socket(zmq::PUB).unwrap(),
                socket_addr: Box::leak(socket_addr.into_boxed_str()),
            },
        }
    }
    fn start(&self) {
        self.socket.socket_handle.bind(self.socket.socket_addr).unwrap();
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
            socket: Socket {
                socket_handle: zmq::Context::new().socket(zmq::SUB).unwrap(),
                socket_addr: Box::leak(socket_addr.into_boxed_str()),
            },
            callback_map : HashMap::new(),
        }
    }

    fn start(&self) {
        self.socket.socket_handle.connect(self.socket.socket_addr).unwrap();
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