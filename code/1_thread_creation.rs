use std::task;

fn printStr(s: &str) {
    println!( "{:s}", s );
}	

fn main() {
    task::spawn( proc(){ printStr("A"); });
    task::spawn( proc(){ printStr("B"); });
}