use std::task;

static mut counter : uint = 0;

fn mytask(s: &str) {
    println!("before {:s}", s);
    for _ in range(0, 10000) {
        unsafe {
            println!("before - counter is: {}", counter);
            counter += 1;
            println!("after - counter is: {}", counter);
        }
    }
    println!("after {:s}", s);
}

fn main() {
    
    for i in range(0, 100) {
        task::spawn( proc(){ mytask(i.to_str()); });
    }
    
    println!("Result should be 1000000");
    unsafe {
        println!("main: done with both counter = {:u}", counter);
    }
}
