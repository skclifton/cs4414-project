# Concurrency and Threads in Rust 0.10

## Processes and Threads - What are they?

### Cost of Creation
### Shared Memory


## Tasks are a special form of thread

### Native Thread vs. Green Thread
The default native thread uses one-to-one scheduling to map tasks to operating system (OS) kernel provided threads. Stated simply, for every task created by the user, there is a separate OS thread in which the task is run. 

### Shared Memory Space



#### Example 1: Thread Creation
```rust
use std::task;

fn printStr(s: &str) {
    println!( "{:s}", s );
}	

fn main() {
    task::spawn( proc(){ printStr("A"); });
    task::spawn( proc(){ printStr("B"); });
}
```

#### Example 2: Failed Data Sharing

```rust
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
```


#### Example 3: Safe Data Sharing

```rust
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
```

#### Example 4: Ports and Channels

```rust
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
```


#### Example 5: Thread Waiting

```rust
use std::task;

static mut counter : uint = 0;

fn mytask(s: &str) {
    println!("before {:s}", s);
    for _ in range(0, 100) {
        unsafe {
            counter += 1;
        }
    }
    println!("after {:s}", s);
}

fn main() {
    
    for i in range(0, 10000) {
        let result: Result<~str, ~std::any::Any:Send> = task::try(proc() { 
            mytask(i.to_str()); 
            ~"Done" });
        assert!(result.is_ok());
    }

    println!("Result should be 1000000");
    unsafe {
        println!("main: done with both counter = {:u}", counter);
    }
}
```

## Try it!

Here we explain the deadlock code
#### Example 6: Deadlock

```rust
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
```

Use features of Rust to write your own program that deadlocks.
