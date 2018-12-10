use chrono;
use std::num::ParseIntError;
use nom::{digit, multispace1};
use chrono::Timelike;
use std::collections::BTreeMap;
use itertools::*;

fn from_ts(input: &str) -> Result<chrono::NaiveDateTime, chrono::ParseError> {
    chrono::NaiveDateTime::parse_from_str(input, "%Y-%m-%d %H:%M")
}

fn from_str_i32(input: &str) -> Result<i32, ParseIntError> {
    i32::from_str_radix(input, 10)
}

#[derive(Debug,PartialEq)]
enum LogEvent {
    BeginShift(i32),
    FallAsleep,
    WakeUp
}
type DT = chrono::NaiveDateTime;

named!(lb<&str, char>, char!('['));
named!(rb<&str, char>, char!(']'));

named!(timestamp<&str, DT>,
    map_res!(delimited!(lb, take!(16), rb), from_ts)
);

named!(int32 <&str, i32>,
    map_res!(digit, from_str_i32)
);

named!(begin_shift<&str, LogEvent>,
    do_parse!(
        tag!("Guard #") >>
        id: int32 >>
        (LogEvent::BeginShift(id))
    )
);

named!(fall_asleep<&str, LogEvent>, do_parse!(tag!("falls asleep") >> (LogEvent::FallAsleep)));
named!(wake_up<&str, LogEvent>, do_parse!(tag!("wakes up") >> (LogEvent::WakeUp)));

named!(event<&str, LogEvent>,
    alt!(
        begin_shift |
        fall_asleep |
        wake_up
    )
);

named!(log_row<&str, (DT, LogEvent)>,
    do_parse!(
        ts: timestamp >>
        multispace1 >>
        ev: event >>
        take_until!("\n") >>
        (ts, ev)
    )
);

named!(log_rows<&str, Vec<(DT, LogEvent)>>,
    separated_list_complete!(multispace1, log_row)
);


#[derive(Debug,PartialEq)]
pub struct SleepSection {
    id: i32,
    min_start: i32,
    len: i32,
}

pub fn advent4(s: String) -> Result<i32, &'static str> {
    let mut rows = log_rows(&s).expect("unable to parse logs").1;
    rows.sort_unstable_by_key(|x| x.0);

    let mut sleepsections = Vec::new();
    let mut curr_id: i32 = 0;
    let mut sleep_start_min: i32 = 0;

    for row in rows.iter() {
        match row.1 {
            LogEvent::BeginShift(id) => curr_id = id,
            LogEvent::FallAsleep => sleep_start_min = row.0.minute() as i32,
            LogEvent::WakeUp => sleepsections.push(SleepSection { id: curr_id, min_start: sleep_start_min, len: row.0.minute() as i32 - sleep_start_min }),
        }
    }

    let mut sleep_totals = BTreeMap::new();
    for ss in sleepsections.iter() {
        *sleep_totals.entry(ss.id).or_insert(0) += ss.len;
    }

    let sorted_sleep_totals = sleep_totals.into_iter().sorted_by_key(|x| x.1).collect::<Vec<(i32, i32)>>();
    let res = sorted_sleep_totals.into_iter().next_back().unwrap();

    println!("hello1 id={:?} minutes={:?}", res.0, res.1);
    let best_id = res.0;

    let mut minutes = BTreeMap::new();
    for ss in sleepsections.iter() {
        if ss.id == best_id {
            for m in ss.min_start..ss.min_start + ss.len {
                *minutes.entry(m).or_insert(0) += 1;
            }
        }
    }
    let sorted_minute_counts = minutes.into_iter().sorted_by_key(|x| x.1).collect::<Vec<(i32, i32)>>();
    let res = sorted_minute_counts.into_iter().next_back().unwrap();
    println!("best minute={:?}", res);
    let best_minute = res.0;
    /*
    println!("hello2 {:?}", timestamp("[1518-11-01 00:00]"));
    println!("hello3 {:?}", log_row("[1518-11-01 00:00] wakes up"));
    println!("hello3 {:?}", log_row("[1518-11-01 00:00] falls asleep"));
    println!("hello4 {:?}", log_row("[1518-11-01 00:00] Guard #123 begins shift"));
    */

    Ok(best_id * best_minute)
}
