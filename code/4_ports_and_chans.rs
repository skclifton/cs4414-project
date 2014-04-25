use std::task;
use std::comm::channel;

fn increment( count: int) -> int{
    return count + 1;    
}

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
            println!("mutable count before: {}", mut_count);
            for _ in range (0,100) {
                mut_count = increment(mut_count);
            }
            println!("mutable count after: {}", mut_count);
            chano.send(mut_count);
        });
        count = porto.recv();
    }
    
    println!("Result should be 1000000");
    println!("main: done with both count = {}", count);
}
