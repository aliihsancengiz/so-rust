pub enum Type
{
	Server,
	Client,
	Publisher,
	Subscriber,
	InputChannel,
	OutputChannel,
}


pub struct Socket {
    pub socket_handle: zmq::Socket,
    pub socket_addr: &'static str,
	pub socket_type : Type,
}

fn socket_type_mapper(typ:&Type) -> zmq::SocketType {
	match typ {
		Type::Server => zmq::REP,
		Type::Client => zmq::REQ,
		Type::Publisher => zmq::PUB,
		Type::Subscriber => zmq::SUB,
		Type::InputChannel => zmq::PULL,
		Type::OutputChannel => zmq::PUSH,
	}
}

impl Socket {
	pub fn new(socket_addr: String,sock_type:Type) -> Self
	{
		Self {
			socket_handle: zmq::Context::new().socket(socket_type_mapper(&sock_type)).unwrap(),
			socket_addr: Box::leak(socket_addr.into_boxed_str()),
			socket_type: sock_type,
		}
	}
    pub fn send(&self, msg: &String) {
        let resp = zmq::Message::from(&msg);
        self.socket_handle.send(resp, 0).unwrap();
    }

    pub fn recv(&self) -> String {
        let mut msg = zmq::Message::new();
        self.socket_handle.recv(&mut msg, 0).unwrap();
        let msg_str: String = String::from_utf8(msg.as_str().unwrap().as_bytes().to_vec()).unwrap();
        msg_str
    }
}
