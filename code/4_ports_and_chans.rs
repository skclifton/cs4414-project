use std::task;
use std::comm::channel;

fn main() {
    let mut count: int = 0;
    let (tx, rx)  = channel();
    
    for _ in range(0, 10000) {
        let child_tx = tx.clone();
        let (chano, porto) = channel();
        child_tx.send(2);
        let count2 = count;
        
        task::spawn( proc(){ 
            let mut mut_count = count2;
            
	    for _ in range (0,100) {
                mut_count += 1;
            }
            chano.send(mut_count);
        });
        count = porto.recv();
    }
    
    println!("Result should be 1000000");
    println!("main: done with both count = {}", count);
}
