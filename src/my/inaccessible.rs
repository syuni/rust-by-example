#[allow(dead_code)]
pub fn public_function() {
    println!("called `my::inaccessible::public_function()`");
}

// この関数は`my/mod.rs`の中で`pub mod`されていないため、
// `10_5.rs`からは呼び出すことができない。
