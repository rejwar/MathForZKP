fn ModularArithmeticDemo() {
    let a: i64 = 17;
    let b: i64 = 5;
    let c: i64 = 12;


    let add = (a ,b) % n;
    let sub = (a - b + n ) % n ;
    let mul = (a *b) %n;

    println!("(a+b) mod n = {}", add);
    println!("(a-b) mod n = {}", sub);
    println!("(a*b) mod n = {}", mul);
}