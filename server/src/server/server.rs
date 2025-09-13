
use zmq::*; 
use miette::*; 
use std::thread;
use std::time::Duration;



pub struct server {
    
    context: Context,
    responder :  Socket,
}

impl server {

    fn new() -> server {
        
          let context = zmq::Context::new();
          let responder = context.socket(zmq::REP).unwrap();
         
            {
            context;
            responder;
            }
    }

    ///bind to new address
    fn bind(mut self, address : &str) -> Result<()> {

        assert!(self.responder.bind(address).is_ok())
    } 

    ///create and publish new message 
    fn new_message(self, msg: Message) -> Result<()> {
        
        self.responder.recv(msg, 0).unwrap();
        println!("Recieved {}", msg.as_str().unwrap());
        thread::sleep(Duration::from_millis(1000));
        responder.send("world", 0).unwrap();

    }

}