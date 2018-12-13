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

pub fn step_full(s: &mut Polymer) -> usize {
    // Do a step of finding all reactions, returning the number of reactions
    let reactions = find_reactions(s);
    let change_count = reactions.len();
    for r in reactions {
        s.remove(&r.0);
        s.remove(&r.1);
    }
    change_count
}

pub fn step_single(s: &mut Polymer, ix: i32) -> bool {
    // Do a step of seeing whether the two entries before and at 'ix' react.
    // Reacts them if so and returns true, otherwise returns false.
    // False if 'ix' is at start or past end of the sequence.
    let (&before, &c1) = match s.range(..ix).next_back() {
        Some(x) => x,
        None => return false
    };
    let (&after, &c2) = match s.range(ix..).next() {
        Some(x) => x,
        None => return false
    };
    if reacts(c1, c2) {
        s.remove(&before);
        s.remove(&after);
        true
    } else {
        false
    }
}

pub fn loop_single(s: &mut Polymer, ix: i32) {
    // do step_single till it returns false
    while step_single(s, ix) {}
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

fn parse(s: String, remove_c: i8) -> Polymer {
    let mut map = BTreeMap::new();
    map.extend(enumerate(
        s
            .trim()
            .bytes()
            .map(to_num)
            .filter(|c| *c != remove_c && *c != -remove_c)
    ).map(|(i, c)| (i as i32, c)));
    map
}

fn react_1(map: &mut Polymer) {
    // just repeat step_full till it stops
    let mut length = map.len();
    loop {
        let cc = step_full(map);
        if map.len() == length {
            break
        }
        length = map.len();
    }
}
fn react_2(map: &mut Polymer) {
    // do loop_single on all reactions
    let mut length = map.len();
    loop {
        for (s, e) in find_reactions(map) {
            loop_single(map, e);
        }
        if map.len() == length {
            break
        }
        length = map.len();
    }
}
fn react_1and2(map: &mut Polymer) {
    // do step_full 5 times, then loop_single till it stops
    for i in 0..5 {
        step_full(map);
    }
    react_2(map);
}

pub fn advent5(s: String) -> Result<i32, &'static str> {

    let mut map = parse(s.clone(), 0);
    let mut best = s.len();
    for remove_c in 1..27 {
        let mut map = parse(s.clone(), remove_c);
        react_1and2(&mut map);
        if map.len() < best {
            best = map.len()
        }
        println!("remove c {}: {}", remove_c, map.len());
    }

    Ok(best as i32)
}
