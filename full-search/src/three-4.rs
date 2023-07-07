fn main() {
    let a = [2, 6, 4, 3, 5, 1];
    let mut max = a[0];
    let mut min = a[0];
    for i in 0..a.len() {
        if max < a[i] {
            max = a[i];
        }
        if min > a[i] {
            min = a[i]
        }
    }
    println!("{}", max - min);
}
