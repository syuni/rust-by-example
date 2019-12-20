// 同様に`mod inaccessible`、`mod
// nested`によって、`nested.rs`、`inaccessible.rs`の内容をこの中で使用することができるようになる。
// `pub`をつけない限り、この中でしか使用できない。
mod inaccessible;
pub mod nested;

pub fn function() {
    println!("called `my::function()`");
}

fn private_function() {
    println!("called `my::private_function()`");
}

pub fn indirect_access() {
    println!("called `my::indirect_access()`, that\n> ");

    private_function();
}