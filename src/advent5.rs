use itertools::Itertools;
use itertools::free::enumerate;

pub fn reacts(c1: char, c2: char) -> bool {
    return c1.to_uppercase().to_string() == c2.to_uppercase().to_string() &&
        ((c1.is_lowercase() && c2.is_uppercase())
            || (c1.is_uppercase() && c2.is_lowercase()))
}

pub fn find_reaction(s: &String) -> Option<usize> {
    let windows = s.chars().tuple_windows::<(char, char)>();
    for (ix, (l, r)) in enumerate(windows) {
        if reacts(l,r) {
            return Some(ix)
        }
    }
    None
}

pub fn step(s: String) -> String {
    if let Some(ix) = find_reaction(&s) {
        // react
        let mut res = String::from(&s[..ix]);
        res.push_str(&s[ix+2..]);
        res
    } else {
        s
    }
}

pub fn advent5(s: String) -> Result<i32, &'static str> {
    println!("start: {}", &s);
    let mut result = String::from(s.trim());
    let mut length = result.len();
    loop {
        result = step(result);
        if result.len() == length {
            break
        }
        length = result.len();
        if length % 100 == 0 {
            println!("len: {}", length);
        }
    }
    Ok(length as i32)
}
