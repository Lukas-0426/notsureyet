//#![crate_name = "client"]

//use async_zmq::{Result, StreamExt}; 
pub mod client;
use crate::client::client::ClientZmq;
use crate::client::cli::Args;
use clap::*;

 fn main()  {

    let args = client::cli::Args::parse();

    println!("Connecting to hello world server...\n");


    let mut new_client = ClientZmq::new(); 

        let addr = args.port_number;
    let _ = new_client.connetct_to_addr("tcp://127.0.0.1:5555"); 

   

    

    for request_nbr in 0..10 {
        println!("Sending Hello {}...", request_nbr);
        new_client.send_request("Hello", 0).unwrap();

        new_client.recive_request(   0).unwrap();
        if let Ok(request) = new_client.recive_request(0) {
        println!("Received World {}: {}", request.as_str().unwrap(), request_nbr);
        }

    }
}
