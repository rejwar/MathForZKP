fn congruent(a: i32, b: i32, n: i32) -> bool {
    (a - b) % n == 0
}

fn main() {
    println!("Is 17 ≡ 2 mod 5? {}", congruent(17, 2, 5)); // true
    println!("Is 100 ≡ 1 mod 11? {}", congruent(100, 1, 11)); // true
    println!("Is 7 ≡ 3 mod 5? {}", congruent(7, 3, 5)); // false
}