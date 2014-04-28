use std::task;
use std::comm::channel;

fn main() {
    let mut count: int = 0;
    
    for _ in range(0, 10000) {
        let (tx, rx) = channel();
        let count2 = count;
        
        task::spawn( proc(){ 
            let mut mut_count = count2;
            
	    for _ in range (0,100) {
                mut_count += 1;
            }
            tx.send(mut_count);
        });
        count = rx.recv();
    }
    
    println!("Result should be 1000000");
    println!("main: done with both count = {}", count);
}
