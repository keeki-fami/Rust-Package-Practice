use practice_rust::bbb;
use sub::ccc;

fn main() {
    bbb::hello_from_bbb();
    let num1: i32 = 3;
    let num2: i32 = 5;
    let num = ccc::adder(num1 , num2);
    println!("3+5={:?}", num);
}
