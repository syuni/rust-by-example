struct Owner(i32);

impl Owner {
    // 通常の関数と同様にライフタイムを明示
    fn add_one<'a>(&'a mut self) {
        self.0 += 1
    }

    fn print<'a>(&'a self) {
        println!("`print`: {}", self.0);
    }
}

fn main() {
    let mut owner = Owner(18);

    owner.add_one();
    owner.print();
}
