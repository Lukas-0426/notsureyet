

use zmq::*; 
use miette::*; 




struct clinet {
context: zmq::Context,
requester: zmq::Socket,

}


impl client {

fn new(self) -> self {

    let context= zmq::Context::new();
    let requestor = context.socket(zmq::REQ).unwrap();
    self {
        context : context,
        requestor : requestor,
    }

}

fn connetct(&mut self, address: &str ) -> Result<()> {

    assert!(self.requestor.connetct(address).is_ok()); //make sure connects else will panic 
    Ok(())
}


fn send_request(&mut self, request : &str) -> Result<()> {

       if  self.requestor.send(request).is_ok {
            Ok(())
       }
       else {
        Err(Error::msg::new("could not send request"))
       }

}

fn recive_request(&mut self, request: Message) -> Result<()> {
    
}


}