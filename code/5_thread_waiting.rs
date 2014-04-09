/*
 * Makes sure each task finishes before the next starts
 */

use std::task;

static mut counter : uint = 0;

fn mytask(s: &str) {
    println!("before {:s}", s);
    for _ in range(0, 10000) {
        unsafe {
            counter += 1;
        }
    }
    println!("after {:s}", s);
}

fn main() {
    
    for i in range(0, 100) {
        let result: Result<~str, ~std::any::Any:Send> = do task::try { 
            mytask(i.to_str()); 
            ~"Done" };
        assert!(result.is_ok());
    }

    println!("Result should be 1000000");
    unsafe {
        println!("main: done with both counter = {:u}", counter);
    }
}
