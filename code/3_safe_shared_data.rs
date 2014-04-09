extern mod extra;
use extra::arc::RWArc;

fn mytask(s: &str, counter:RWArc<uint>) {
    println!("before {:s}", s);
    for _ in range(0, 10000) {
        counter.write(|count| {*count += 1;});
    }
    println!("after {:s}", s);
}

fn main() {
    let counter: RWArc<uint> = RWArc::new(0);
    
    for i in range(0, 100) {
        let count = counter.clone();
        do spawn { mytask(i.to_str(), count.clone()); };
    }
    
    println!("Result should be 1000000");
    counter.read(|count| {println!("main: done with both counter = {:u}", *count);});
}
