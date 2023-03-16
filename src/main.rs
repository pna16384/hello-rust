fn main() {
    println!("Hello, world!");
}

#[test]
fn test_gdc1() {
    assert_eq!(gcd(6, 4), 2);
}

#[test]
fn test_gdc2() {
    assert_eq!(gcd(15, 14), 1);
}

fn gcd(mut n:u64, mut m:u64)-> u64 {

    assert!(n!=0 && m!=0);
    while m !=0 {

        if m < n {
            let t = m;
            m = n;
            n = t;
        }

        m = m % n;
    }

    n
}

