1	fn printStr(s: &str) {
2	    println!( "{:s}", s );
3	}	
4
5	fn main() {
6	    do spawn { printStr("A"); }
7	    do spawn { printStr("B"); }
8	}