mod my {
    // パブリックなフィールド`T`（ジェネリック型）を持つパブリックな構造体
    pub struct WhiteBox<T> {
        pub contents: T,
    }

    // プライベートなフィールド`T`（ジェネリック型）を持つパブリックな構造体
    #[allow(dead_code)]
    pub struct BlackBox<T> {
        contents: T,
    }

    impl<T> BlackBox<T> {
        // パブリックなコンストラクタメソッドを持つ構造体
        pub fn new(contents: T) -> BlackBox<T> {
            BlackBox { contents }
        }
    }
}

fn main() {
    // パブリックなフィールドを持つパブリックな構造体は、通常通りインスタンス化できる。
    let white_box = my::WhiteBox {
        contents: "public information",
    };

    // フィールドにも普通にアクセスできる。
    println!("The white box contains: {}", white_box.contents);

    // プライベートなフィールドを持つ構造体は、
    // インスタンス化する際にフィールド名を指定することができない。
    // let black_box = my::BlackBox { contents: "classified information" };

    // そのような場合でも、パブリックなコンストラクタを介して作成することは可能。
    let _black_box = my::BlackBox::new("classified information");

    // たとえパブリックな構造体でも、プライベートなフィールドにはアクセス出来ない
    // println!("The black box contains: {}", _black_box.contents);
}
