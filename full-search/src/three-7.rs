fn main() {
    let n = 3usize;
    let s: Vec<char> = "125".chars().collect();
    let si: Vec<usize> = s.iter().map(|&c| c as usize - '0' as usize).collect();

    let max = 2usize.pow((n - 1) as u32);
    let mut ans = 0;
    for i in 0..max {
        let mut sm = 0;
        let mut now = si[0];
        for j in 0..n - 1 {
            if (i >> j) & 1 == 1 {
                sm += now;
                now = si[j + 1];
            } else {
                now = now * 10 + si[j + 1];
            }
        }
        ans += sm + now;
    }
    println!("{}", ans);
}
