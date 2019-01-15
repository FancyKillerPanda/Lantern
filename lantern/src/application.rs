pub trait Application {
	fn run(&self) {
		for i in 0..5 {
			println!("{}", i);
		}
	}
}
