

use zmq::*; 
use miette::{Result, Error}; 




pub struct ClientZmq {
context: zmq::Context,
requester: zmq::Socket,

}


impl ClientZmq {

pub fn new() -> Self {

    let context= zmq::Context::new();
    let requestor = context.socket(zmq::REQ).unwrap();
    Self {
        context : context,
        requester : requestor,
    }

}

pub fn connetct_to_addr(&mut self, address: &str ) -> Result<()> {

    if self.requester.connect(address).is_ok() { //make sure connects else will panic 
    Ok(())
    }
    else {
        Err(Error::msg("was not able to connect"))
    }
}


pub fn send_request(&mut self, request : &str, flags: i32) -> Result<()> {

       if  self.requester.send(request, flags).is_ok() {
            Ok(())
       }
       else {
        Err(Error::msg("could not send request"))
       }

}

pub fn recive_request(&mut self, flags: i32) -> Result<Message> {

let mut request = zmq::Message::new();
    if   self.requester.recv( &mut request, flags).is_ok() {

        println!("Recivced  {:?}" , request.as_str()); 
        Ok(request)
    }
    else {
        Err(Error::msg("was not recived correctly"))
    }
}


}