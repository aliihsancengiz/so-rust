use crate::so::socket::{Socket,Type};

type Callback = fn(msg: &String) -> String;

pub struct ReqRepServer {
    socket: Socket,
}

pub struct ReqRepClient {
    socket: Socket,
}

pub trait SyncReqRepServer {
    fn new(socket_addr: String) -> Self;
    fn start(&self);
    fn server_process(&self, cb: Callback);
}

pub trait SyncReqRepClient {
    fn new(socket_addr: String) -> Self;
    fn start(&self);
    fn client_process(&self, req: &String, resp: &mut String);
}

impl SyncReqRepServer for ReqRepServer {
    fn new(socket_addr: String) -> Self {
        Self {
            socket: Socket::new(socket_addr,Type::Server),
        }
    }

    fn start(&self) {
        self.socket.start();
    }

    fn server_process(&self, cb: Callback) {
        self.socket.send(&(cb)(&self.socket.recv()));
    }
}

impl SyncReqRepClient for ReqRepClient {
    fn new(socket_addr: String) -> Self {
        Self {
            socket:  Socket::new(socket_addr,Type::Client),
        }
    }

    fn start(&self) {
        self.socket.start();
    }

    fn client_process(&self, req: &String, resp: &mut String) {
        self.socket.send(&req);
        *resp = self.socket.recv();
    }
}
