pub struct Layer;

impl Layer
{
    pub fn tcp(ip:&str,port:u32) -> String
    {
        "tcp://".to_owned()+&ip+&":".to_owned()+&port.to_string()
    }
    pub fn ipc(service:&str,subservice:&str) -> String
    {
        let path_to_service = "/tmp/so_zmq_".to_owned()+&service+&"_".to_owned()+&subservice;
        if !std::path::Path::new(&path_to_service).exists() {
            let _file = std::fs::OpenOptions::new().write(true)
                .append(true)
                .create_new(true)
                .open(&path_to_service)
                .unwrap();
        }
        "ipc://".to_owned()+&path_to_service
    }
}


#[cfg(test)]
mod tests {
	use super::*;
    #[test]
    fn layer_tcp() {
        assert_eq!(Layer::tcp("127.0.0.1",1234), "tcp://127.0.0.1:1234");
    }
	#[test]
	fn layer_ipc() {
		assert_eq!(Layer::ipc("test","layer"), "ipc:///tmp/so_zmq_test_layer");
	}

	#[test]
	fn layer_ipc_file_exists() {
		_ = Layer::ipc("test","layer");
		assert!(std::path::Path::new("/tmp/so_zmq_test_layer").exists());
	}
}