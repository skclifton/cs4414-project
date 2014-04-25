use std::task;

static mut count: int = 0;


fn increment( count: int) -> int{
    return count + 1;    
}

fn main() {
    
    for i in range(0, 10000) {
        task::spawn( proc(){ 
            println!("before {:s}", i.to_str());
            for _ in range(0, 100) {
                unsafe {
                    println!("before - count is: {}", count);
                    count = increment(count);
                    println!("after - count is: {}", count);
                }
            }
        println!("after {:s}", i.to_str());
        });
    }
    
    println!("Result should be 1000000");
    unsafe {
        println!("main: done with both count = {}", count);
    }
}
