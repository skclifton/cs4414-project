static mut counter : uint = 0;

fn mytask(s: &str) {
    println!("before {:s}", s);
    for _ in range(0, 10000000) {
        unsafe {
            counter += 1;
        }
    }
    println!("after {:s}", s);
}

fn main() {
    
    for i in range(0, 100) {
        do spawn { mytask(i.to_str()); };
    }
    
    println!("Result should be 1000000000");
    unsafe {
        println!("main: done with both counter = {:u}", counter);
    }
}
