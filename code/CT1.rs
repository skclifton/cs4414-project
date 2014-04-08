fn printStr(s: &str) {
    println!( "{:s}", s );
}	

fn main() {
    do spawn { printStr("A"); }
    do spawn { printStr("B"); }
}