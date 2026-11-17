use practice_rust::bbb;
use sub::ccc;

fn main() {
    bbb::hello_from_bbb();
    let num1: i32 = 3;
    let num2: i32 = 5;
    ccc::check(num1, num2);
}
