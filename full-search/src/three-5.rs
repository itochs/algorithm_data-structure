fn solve(a: Vec<usize>) -> Option<usize> {
    let mut div2_cnt_min: Option<usize> = None;
    // all can div 2
    for i in 0..a.len() {
        if a[i] % 2 != 0 {
            return None;
        }
    }
    for i in 0..a.len() {
        if let Some(cnt) = div2_cnt_min {
            if cnt > a[i] / 2 {
                div2_cnt_min = Some(a[i] / 2);
            }
        } else {
            div2_cnt_min = Some(a[i] / 2);
        }
    }
    return div2_cnt_min;
}

fn main() {
    let a = [2, 6, 4, 3, 5, 1];
    let ans = solve(a.to_vec());
    println!("{}", if let Some(aa) = ans { aa } else { 0 });
    let b = [6, 4, 8, 6, 10];
    let ans = solve(b.to_vec());
    println!("{}", if let Some(aa) = ans { aa } else { 0 });
}
