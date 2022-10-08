use crate::so::socket::Socket;

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
            socket: Socket {
                socket_handle: zmq::Context::new().socket(zmq::REP).unwrap(),
                socket_addr: Box::leak(socket_addr.into_boxed_str()),
            },
        }
    }

    fn start(&self) {
        self.socket
            .socket_handle
            .bind(self.socket.socket_addr)
            .unwrap();
    }

    fn server_process(&self, cb: Callback) {
        self.socket.send(&(cb)(&self.socket.recv()));
    }
}

impl SyncReqRepClient for ReqRepClient {
    fn new(socket_addr: String) -> Self {
        Self {
            socket: Socket {
                socket_handle: zmq::Context::new().socket(zmq::REQ).unwrap(),
                socket_addr: Box::leak(socket_addr.into_boxed_str()),
            },
        }
    }

    fn start(&self) {
        self.socket.socket_handle
            .connect(self.socket.socket_addr)
            .unwrap();
    }

    fn client_process(&self, req: &String, resp: &mut String) {
        self.socket.send(&req);
        *resp = self.socket.recv();
    }
}
