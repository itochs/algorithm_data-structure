fn main() {
    let a = [2, 6, 4, 6, 5, 1];
    let v = 6;
    let mut cnt = 0;
    for i in 0..a.len() {
        if a[i] == v {
            cnt += 1;
        }
    }
    println!("{}", cnt);
}
