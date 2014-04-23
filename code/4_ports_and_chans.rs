extern mod extra;
use std::task;
use std::comm::Chan;

fn main() {
    let mut count: int = 0;
    let (port, chan)  = SharedChan::new();
    
    for _ in range(0, 10000) {
        let child_chan = chan.clone();
        let (porto, chano) = Chan::new();
        child_chan.send(2);
        let var = count;
        
        task::spawn( proc(){ 
            let mut mutvar = var;
            println!("var before: {}", mutvar);
            for _ in range (0,100) {
                mutvar += 1;
            }
            println!("var after: {}", mutvar);
            chano.send(mutvar);
        });
        count = porto.recv();
    }
    
    println!("Result should be 1000000");
    println!("main: done with both counter = {}", count);
}
