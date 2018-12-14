use std::collections::BTreeMap;
use ordered_float::*;

type Score = i64;

const PLAYERS: i32 = 413;
const LAST: i32 = 71082;

type Key = NotNaN<f64>;

fn avg(a: Key, b: Key) -> Key {
    let half = NotNaN::from(0.5);
    let result = a * half + b * half;
    if result == a || result == b {
        panic!("avg() ran out of precision");
    }
    result
}

type Order = BTreeMap<Key, i32>;

fn advance_cursor(order: &Order, cursor: Key, clockwise_n: i32) -> Key {
    // returns the cursor (key for) to the element in the map which is clockwise
    // n positions.

    if clockwise_n > 0 {
        let mut iter = order.range(cursor..);
        // move past 'cursor'
        iter.next();
        let mut result = NotNaN::from(0.0);
        for i in 0..clockwise_n {
            match iter.next() {
                None => {
                    // loop around
                    let fst = order.iter().next().unwrap().0;
                    iter = order.range(fst..);
                    result = *fst
                },
                Some(x) => result = *x.0
            }
        }
        result
    } else {
        let mut iter = order.range(..cursor);
        let mut result = NotNaN::from(0.0);
        for i in clockwise_n..0 {
            match iter.next_back() {
                None => {
                    // loop around
                    let last = order.iter().next_back().unwrap().0;
                    iter = order.range(..last);
                    result = *last;
                },
                Some(x) => {
                    result = *x.0;
                }
            }
        }
        result
    }

}

fn cursor_for_insert_after(order: &Order, cursor: Key) -> Key {
    // create a new cursor for inserting after
    let next = advance_cursor(order, cursor, 1);
    if next <= cursor {
        // it looped
        cursor * NotNaN::from(2.0)
    } else {
        avg(cursor, next)
    }
}

fn print_debug(player_id: i32, order: &Order, cursor: Key) {
    print!("{}: ", player_id);
    for (k, v) in order.iter() {
        if *k == cursor {
            print!(" ({})", v);
        } else {
            print!(" {}", v);
        }
    }
    println!();
}

pub fn advent9(s: String) -> Result<i64, &'static str> {
    let mut order = BTreeMap::new();

    let mut cursor = NotNaN::from(1.0);
    order.insert(cursor, 0);

    let mut player_id = 1;
    let mut player_score = BTreeMap::new();

    //print_debug(player_id, &order, cursor);

    for marble_id in 1..=LAST {

        if marble_id % 23 == 0 {
            let mut e = player_score.entry(player_id).or_insert(0 as i64);
            *e += marble_id as i64;
            let rmcursor = advance_cursor(&order, cursor, -7);
            cursor = advance_cursor(&order, rmcursor, 1);
            let taken_marble_id = order.remove(&rmcursor).unwrap();
            *e += taken_marble_id as i64;

        } else {
            cursor = advance_cursor(&order, cursor, 1);
            cursor = cursor_for_insert_after(&order, cursor);
            order.insert(cursor, marble_id);
        }
        //print_debug(player_id, &order, cursor);

        // at end:
        player_id += 1;
        if player_id > PLAYERS { player_id = 1; }
    }

    println!("Scores: {:?}", player_score);
    Ok(*player_score.values().max().unwrap())
}
