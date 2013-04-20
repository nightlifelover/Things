fn rFib(n: int) -> int {
	match n {
		0 => 0,
		1 => 1,
		_ => rFib(n - 1) + rFib(n - 2)
	}
}

fn fib(n: int) -> int {

	fn doNormalCase(n: int) -> int {
		let mut prev = 1, prevPrev = 0, i = 2;

		while i != n+1 {
			let tmp = prev + prevPrev;
			prevPrev = prev;
			prev = tmp;
			i += 1;
		}

		prev
	}

	match n {
		0 => 0,
		1 => 1,
		_ => doNormalCase(n)
	}
}

fn main() {
	//let n = rFib(7);
	let n = fib(7);
	io::println(fmt!("%d", n));
}