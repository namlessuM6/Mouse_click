use mouse_rs::{types::keys::Keys, Mouse};
use std::{thread, time};


fn main() {
    //mouse button input 

    //wait time
    let wait_time = std::env::args().nth(2).unwrap();

   
    //works
    click_timer_test(wait_time);
}



fn click_timer_test(wait_time : String) {
   //creates mouse object
   let mouse = Mouse::new();
   //user input for time interval 
   //convert user input into a integer 
   let x: u64 = wait_time.trim().parse().expect("Input not an integer");
   //convert integer into time duration
   let interval = time::Duration::from_millis(x);
   //sleep
   thread::sleep(interval);
   //click
   mouse.press(&Keys::LEFT).expect("Unable to press button");
   mouse.release(&Keys::LEFT).expect("Unable to release button");
}
