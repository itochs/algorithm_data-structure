fn main() {
    let a = [2, 6, 4, 3, 5, 1];
    let mut f_min: Option<usize> = None;
    let mut s_min: Option<usize> = None;
    for i in 0..a.len() {
        if let Some(fm) = f_min {
            if fm > a[i] {
                s_min = f_min;
                f_min = Some(a[i]);
                continue;
            }

            if let Some(sm) = s_min {
                if sm > a[i] {
                    s_min = Some(a[i]);
                }
            } else {
                s_min = Some(a[i]);
            }
        } else {
            f_min = Some(a[i]);
        }
    }

    if let Some(sm) = s_min {
        println!("{}", sm);
    } else {
        println!("a do not have second min");
    }
}
