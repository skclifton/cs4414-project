use std::task;

static mut count: int = 0;

fn main() {
    
    for _ in range(0, 10000) {
        task::spawn( proc(){
            for _ in range(0, 100) {
                unsafe {
                    count += 1;
                }
            }
        });
    }
    
    println!("Result should be 1,000,000");
    unsafe {
        println!("main: done with both count = {}", count);
    }  
    
}
