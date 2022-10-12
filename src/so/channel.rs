use crate::so::socket::{Socket,Type};

pub struct ChannelIn
{
    socket: Socket,
}

pub struct ChannelOut
{
    socket: Socket,
}


pub trait SyncChannelIn
{
    fn new(socket_addr: String) -> Self;
    fn start(&self);
    fn pull(&self) -> String;
}

pub trait SyncChannelOut
{
    fn new(socket_addr: String) -> Self;
    fn start(&self);
    fn push(&self,msg:String);
}

impl SyncChannelIn for ChannelIn
{
    fn new(socket_addr: String) -> Self {
        Self {
            socket: Socket::new(socket_addr,Type::InputChannel),
        }
    }
    fn start(&self) {
        self.socket.start();
    }
    fn pull(&self)  -> String {
        self.socket.recv()
    }

}

impl SyncChannelOut for ChannelOut
{
    fn new(socket_addr: String) -> Self {
        Self {
            socket: Socket::new(socket_addr,Type::OutputChannel),
        }
    }
    fn start(&self) {
        self.socket.start();
    }
    fn push(&self,msg:String) {
        self.socket.send(&msg);
    }
}