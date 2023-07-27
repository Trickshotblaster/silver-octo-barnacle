fn main() {
    println!("Hello, world!");
    let mut a:i32 = 0;
    let mut b:i32 = 1;
    let mut c:i32 = 1;
    for _ in 1..1000 {
        c = fibonnacci(a, b);
        println!("{}", c);
        a = b;
        b = c
    }
}
// Don't know how to spell it
fn fibonnacci(a:i32, b:i32) -> i32 {
    return a+b;
}
