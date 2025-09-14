

use clap::*;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {

     /// what channel would you like to connect to
    #[arg(short, long)]
   pub  channel_name: String, 
    
    ///what port number to connect over <2000 
    #[arg(short, long, default_value_t = 3000)]
    pub port_number: u32,

    ///message to send to server options are x 
    #[arg(short, long)]
    pub message : String, 

}


impl Args {
    


}



