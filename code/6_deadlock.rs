extern crate sync;

use sync::{Mutex, Arc};

fn increment( count: int) -> int{
    return count + 1;    
}

fn main() {    

    let mutex1 = Arc::new(Mutex::new(0));
    let mutex2 = mutex1.clone();
        
    spawn(proc() {
    
        let mut val = mutex2.lock();
        *val = increment(*val);
        

    });
    
    let value = mutex1.lock();
    while *value != 2 {
        value.cond.wait();
    }
    
    println!("Lock was never released in spawned proc(), so I will never be reached!");
}

