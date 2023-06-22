use twsapi::core::errors::IBKRApiLibError;
use twsapi::core::client::*;
use twsapi::core::streamer::{Streamer, TcpStreamer};
use twsapi::core::contract::*;
// use twsapi::core::wrapper::Wrapper;
// use twsapi::examples::test_helpers::TestWrapper;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::neowrapper::NeoWrapper;

mod neowrapper;
use neowrapper::sq;

fn main() -> Result<(), IBKRApiLibError> {
   println!("grink...");

   let asdf = Contract::new;

   let wrapper = Arc::new(Mutex::new(NeoWrapper::<TcpStreamer>::new()));
   //let app = Arc::new(Mutex::new(EClient::new(wrapper.clone())));

   let mut scum = EClient::new(wrapper.clone());
   scum.connect("127.0.0.1", 4002, 0)?;

   let flag = scum.is_connected();

   if flag {
      println!("grint!");
   }
   //println!("getting connection...");

   //wrapper.lock().expect("Wrapper mutex was poisoned").client = Option::from(app.clone());

   let a = 3;
   let b = 5;
   let a = sq(a);

   /*
   To initialize new EClient object, it accepts an Arc<Mutex<T>>
   where T is a Wrapper.

   The problem is, we need to write our own Wrapper class that
   inherits from Wrapper.
    */

   //app.lock()
   //   .expect("EClient mutex was poisoned")
   //   .connect("127.0.0.1", 4002, 0)?;

   println!("gronk {}", a+b);

   thread::sleep(Duration::new(5, 0));

   Ok(())
}