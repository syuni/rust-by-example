// 以下では、変数の作成から破棄までのライフタイムを線で示しています。
// `1`は最長のライフタイムを持ち、そのスコープは`borrow1`と`borrow2`のスコープを完全に包含します。
// `borrow1`と`borrow2`の存続期間は一切重なりません。
fn main() {
    let i = 3; // `i`のライフタイムが開始
    {
        let borrow1 = &i; // `borrow1`のライフタイム開始
        println!("borrow1: {}", borrow1);
    }

    {
        let borrow2 = &i; // `borrow2`のライフタイム開始
        println!("borrow2: {}", borrow2);
    }
}
