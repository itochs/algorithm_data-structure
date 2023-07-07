fn main() {
    println!("Hello, world!");
    let a = [1, 2, 3, 4, 5, 6];
    let b = [1, 2, 3, 4, 4, 6];
    let v = 4;
    let mut found_id: Option<usize> = None;
    for i in 0..a.len() {
        if a[i] == v {
            found_id = Some(i);
        }
    }
    if let Some(id) = found_id {
        println!("{}", id);
    } else {
        println!("not found {} in a", v);
    }
    let mut found_id: Option<usize> = None;
    for i in 0..b.len() {
        if b[i] == v {
            found_id = Some(i);
        }
    }
    if let Some(id) = found_id {
        println!("{}", id);
    } else {
        println!("not found {} in b", v);
    }
}
