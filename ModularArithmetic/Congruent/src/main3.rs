fn ModularInverse(a: i64 , n: i64) -> Option<i64> {
    let (mut t , mut new_t) = (0,1);
    let (mut r , mut new_r) = (n,a);


    while new_r != 0{
        let quotient = r / new_r;
        t =r- quotient * new_t;
        std::mem::swap(&mut t , &mut new_t);

        r = r - quotient *new_r;
        std::mem::swap(&mut r , &mut new_r);
    }

    if r > 1 {
        return None;
    }

    if t <0 {
        t += n;
    }

    Some(t)
}

fn ModularInverse() {
    let a = 3;
    let n = 7;

    match ModularInverse(a , n) {
        Some(inv) => println!("Modular inverse of {} = {} " , a,n,inv),
        None =>println!("No modular inverse exists."),
    }
}