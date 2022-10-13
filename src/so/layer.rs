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
