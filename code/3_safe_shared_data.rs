extern crate sync;

use sync::{RWLock, Arc};

fn increment( count: int) -> int{
    return count + 1;    
}

fn main() {    

    let lock1 = Arc::new(RWLock::new(0));
    
    for _ in range(0, 10000) {
    
        let lock2 = lock1.clone();
        
        spawn(proc() {
        
            let mut count = lock2.write();
            
            println!("before -  count = {}", *count);
            for _ in range (0, 100) {
                *count = increment(*count);
            }
            
            let count = count.downgrade();
            
            println!("after -  count = {}", *count);
        });
    }
    
    println!("Result should be 1000000");
    println!("Counter is: {}", *lock1.read());
}

