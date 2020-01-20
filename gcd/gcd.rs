fn main(){
    let m : u64 = 47;
    let n : u64 = 20;
    let ans = gcd(m,n);
    println!("GCD : {}", ans)
}

fn gcd(mut m : u64, mut n : u64) -> u64{
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    return n;
}