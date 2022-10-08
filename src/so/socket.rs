pub struct Socket {
    pub socket_handle: zmq::Socket,
    pub socket_addr: &'static str,
}

impl Socket {
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
