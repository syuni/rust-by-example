// `A`という具象型
struct A;

// `Single`という型を定義する際に`A`を使用しているが、その最初の使用よりも先に`<A>`がないため、
// また、`A`自身も具象型であるため、`Single`は具象型となる。
struct Single(A);
//            ^ Singleによる`A`の一番最初の使用はここ

// ここでは`<T>`が一番初めの`T`の使用よりも先に来ている。
// よって`SingleGen`はジェネリック型となる。なぜならば、型パラメータ`T`がジェネリックだからである。
// `T`はどんな型にもなりえるため、上で定義した`A`を受け取ることができる。
struct SingleGen<T>(T);

fn main() {
    // `Single`は具象型で、`A`のみを受け取る
    let _s = Single(A);

    // `char`という名の変数を生成する。これは`SingleGen<char>`という型で、値は`SingleGen('a')となる。
    // ここでは、`SingleGen`には明示的な型パラメータが与えられている。
    let _char: SingleGen<char> = SingleGen('a');

    // `SingleGen`型の変数には明示的に型パラメータを与えなくてもよい。
    let _t = SingleGen(A);
    let _i32 = SingleGen(6);
    let _char = SingleGen('a');
}
