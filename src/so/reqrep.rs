
#[allow(dead_code)]
pub enum Type {
    Server,
    Client,
}

type Callback = fn(msg: &String) -> String;

pub struct ReqRep {
    socket_handle: zmq::Socket,
    socket_type: Type,
    socket_addr: &'static str,
}

pub trait SyncReqRep {
    fn new(socket_type: Type, socket_addr: String) -> Self;
    fn start(&self);
    fn send(&self, msg: &String);
    fn recv(&self) -> String;
    fn client_process(&self, req: &String, resp: &mut String);
	fn server_process(&self,cb:Callback);
}

impl SyncReqRep for ReqRep {
    fn new(socket_type: Type, socket_addr: String) -> Self {
        let _type = match socket_type {
            Type::Server => zmq::REP,
            Type::Client => zmq::REQ,
        };
        Self {
            socket_type: socket_type,
            socket_handle: zmq::Context::new().socket(_type).unwrap(),
            socket_addr: Box::leak(socket_addr.into_boxed_str()),
        }
    }

    fn start(&self) {
        match self.socket_type {
            Type::Server => self.socket_handle.bind(self.socket_addr).unwrap(),
            Type::Client => self.socket_handle.connect(self.socket_addr).unwrap(),
        }
    }

    fn client_process(&self, req: &String, resp: &mut String) {
        self.send(&req);
        *resp = self.recv();
    }

	fn server_process(&self,cb:Callback)
	{
		self.send(&(cb)(&self.recv()));
	}

    fn send(&self, msg: &String) {
        let resp = zmq::Message::from(&msg);
        self.socket_handle.send(resp, 0).unwrap();
    }

    fn recv(&self) -> String {
        let mut msg = zmq::Message::new();
        self.socket_handle.recv(&mut msg, 0).unwrap();
        let msg_str: String = String::from_utf8(msg.as_str().unwrap().as_bytes().to_vec()).unwrap();
        msg_str
    }
}

