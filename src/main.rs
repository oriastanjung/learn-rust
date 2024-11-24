fn main() {
    let message = "Hello, dunia!";
    println!("{}", message);
}

#[test]
fn learn_variable() {
//    immutable
    let x = 5;
//    mutable
    let mut var1 = 21;
    println!("{}",var1);
    var1 = 5;
    println!("{}",var1);
    var1 = 6;
    println!("x: {}, var2: {}", x, var1);
}