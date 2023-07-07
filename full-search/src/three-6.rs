fn main() {
    let k = 5;
    let n = 10;
    let mut ans = 0;
    for x in 1..=k {
        for y in 1..=k {
            let z = n - x - y;
            if z <= k {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
    // (1,4,5)と(1,5,4)が満たすけど組だから違うものでいいんだっけ？
}
