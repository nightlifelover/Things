fn fib(n: int) -> int {
	
	match n {
		0 => return 0,
		1 => return 1,
		_ => return fib(n - 1) + fib(n - 2)
	}
}

fn main() {
	let n = fib(7);
	io::println(fmt!("%d", n));
}