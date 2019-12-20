// `say_hello`という名のシンプルなマクロ
macro_rules! say_hello {
    // `()`はマクロが引数を取らないことを示す。
    () => {
        // マクロはプリコンパイルの段階でこのブロック内の内容に展開される。
        println!("Hello!");
    };
}

fn main() {
    // この関数呼び出しは`println!("Hello!");`に置き換えられる。
    say_hello!()
}
