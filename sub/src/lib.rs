pub mod ccc {
    use printer_yanne::yanne;
    pub fn adder(x: i32, y: i32) -> i32 {
	return x + y;
    }

    pub fn check(x: i32, y: i32) {
	let num = adder(x, y);
	let text = format!("{:?}", num);
	yanne::print_yanne(text);
    }
}
