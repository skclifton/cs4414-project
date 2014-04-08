use std::task;

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
    do spawn { mytask("A"); };
    let result2: Result<~str, ~std::any::Any:Send> = do task::try { 
        mytask("B");
        ~"Done"
    };
    assert!(result2.is_ok());
    unsafe {
        println!("main: done with both counter = {:u}", counter);
    }
}
