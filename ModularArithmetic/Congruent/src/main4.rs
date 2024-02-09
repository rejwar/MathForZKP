fn ModularExp(mut base: u64 , mut exp: u64 , modulus: u64) -> u64 {
    let mut result = 1;
    base = base % modulus;

    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base ) % modulus;
        }

        base = (base * base  ) % modulus;
        exp = exp / 2;

    }

    result
}

fn ModularExp() {
    let a = 7;
    let b = 128;
    let n = 13;

    let result = ModularExp(a ,b,n);
    println!("{} ^{} mod  {}= {} ", a,b,n, result);
}