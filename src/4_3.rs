fn main() {
    // 変数を宣言
    let a_binding;
    {
        let x = 2;

        // 変数を初期化
        a_binding = x * x;
    }

    println!("a binding: {}", a_binding);

    let another_binding;

    // Error! Use of uninitialized binding
    // println!("another binding: {}", another_binding);

    another_binding = 1;

    println!("another binding: {}", another_binding);
}
