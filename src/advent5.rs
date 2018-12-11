use itertools::Itertools;
use itertools::free::enumerate;
use std::collections::BTreeMap;
use std::ops::Neg;

type Polymer = BTreeMap<i32, i8>;

pub fn reacts(c1: i8, c2: i8) -> bool {
    c1 == -c2
}

pub fn find_reactions(s: &Polymer) -> Vec<(i32, i32)> {
    let windows = s.iter().tuple_windows::<((&i32, &i8), (&i32, &i8))>();
    let mut result = Vec::new();
    let mut last_key = -1;
    for (l, r) in windows {
        if *l.0 == last_key {
            continue;
        }
        if reacts(*l.1,*r.1) {
            result.push((*l.0, *r.0));
            // don't react again
            last_key = *r.0
        }
    }
    result
}

pub fn step(s: &mut Polymer) {
    let reactions = find_reactions(s);
    for r in reactions {
        s.remove(&r.0);
        s.remove(&r.1);
    }
}

fn to_num(c: u8) -> i8 {
    let ci = c as i8;
    let upper_base = 'A' as i8 - 1;
    let lower_base = 'a' as i8 - 1;
    if ci >= lower_base {
        -(ci - lower_base)
    } else {
        ci - upper_base
    }
}

fn parse(s: String) -> Polymer {
    let mut map = BTreeMap::new();
    map.extend(enumerate(s.trim().bytes().map(to_num)).map(|(i, c)| (i as i32, c)));
    map
}

fn divide(m: Polymer) -> Polymer {

    let mut current = m;

    if current.len() >= 100 {
        // divide
        let (start, end) = {
            (*current.iter().next().unwrap().0, *current.iter().next_back().unwrap().0)
        };

        let split = (start + end) / 2;
        let m2 = current.split_off(&split);

        current = divide(current);
        current.extend(divide(m2));
    }

    conquer(&mut current);

    current
}

fn conquer(m: &mut Polymer) {
    let mut map = m;
    let mut length = map.len();
    loop {
        step(&mut map);
        if map.len() == length {
            break
        }
        length = map.len();
    }
}

pub fn advent5(s: String) -> Result<i32, &'static str> {
    let mut map = parse(s);

    map = divide(map);

    Ok(map.len() as i32)
}
