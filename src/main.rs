
fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }

    n
}

fn main() {
    let n = 17;
    let m = 293;
    let x = gcd(n, m);
    println!("The GCD of {n} and {m} is {x}");
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(100, 500), 100);
    //TODO: Fix
    //assert_eq!(gcd(0, 0));
}