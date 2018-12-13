use nom::multispace1;
use std::collections::BTreeSet;
use itertools::Itertools;
use std::collections::BinaryHeap;

const NWORKERS: i32 = 5;
const ALL_DELAY: i32 = 60;


named!(in_row<&str, (&str, &str)>,
    do_parse!(
        tag!("Step ") >>
        l: take!(1) >>
        tag!(" must be finished before step ") >>
        r: take!(1) >>
        take_until!("\n") >>
        (l, r)
    )
);

named!(in_rows<&str, Vec<(&str, &str)>>,
    separated_list_complete!(multispace1, in_row)
);

type ItemSet<'a> = BTreeSet<&'a str>;
type ItemDeps<'a> = Vec<(&'a str, &'a str)>;
type Running<'a> = BinaryHeap<(i32, &'a str)>;

pub fn find_runnable<'a>(deps: &ItemDeps, all_items: &'a ItemSet, completed_items: &ItemSet, scheduled: &Running) -> Option<&'a str> {
    for try_item in all_items.iter() {
        if completed_items.contains(try_item)
            || scheduled.iter().find(|(_, it)| it == try_item).is_some() {
            continue;
        }
        let mut found_blocker = false;
        for (dep, it) in deps.iter() {
            if it == try_item && !completed_items.contains(dep) {
                found_blocker = true;
                break;
            }
        }
        if found_blocker { continue; }
        return Some(try_item);
    }
    return None
}

fn char_to_num(item: &str) -> i32 {
    // A = 1, etc
    let c = item.to_ascii_uppercase().chars().next().unwrap();
    (c as i32) - 64
}

fn schedule<'a, 'b>(current_time: i32, item: &'b str, running_items: &'a mut Running<'b>) {
    let completion_time = ALL_DELAY + char_to_num(item) + current_time;
    running_items.push((-completion_time, item));
    println!("schedule: {:?}", running_items);
}

fn sleep<'a, 'b>(running_items: &'a mut Running<'b>, completed: &'a mut BTreeSet<&'b str>) -> i32 {
    // sleep until a worker completes its task.
    let (newtime, item) = running_items.pop().expect("Unable to sleep, no tasks on queue");
    completed.insert(item);
    println!("sleep till {:?}", -newtime);
    -newtime
}

pub fn advent7(s: String) -> Result<i32, &'static str> {
    let mut rows = in_rows(&s).expect("unable to parse input").1;
    println!("rows: {:?}", rows);

    let mut all_items = BTreeSet::new();
    for (l, r) in rows.iter() {
        all_items.insert(*l);
        all_items.insert(*r);
    }
    let mut completed_items = BTreeSet::new();
    let mut running_items = BinaryHeap::new();
    let mut current_time = 0;

    while completed_items.len() < all_items.len() {
        // if we have no ready workers, just sleep
        let ready_workers = NWORKERS - running_items.len();
        if ready_workers == 0 {
            current_time = sleep(&mut running_items, &mut completed_items);
            continue;
        }

        // find a runnable item (an item not in completed_items that has no uncompleted prereqs)
        let result = find_runnable(&rows, &all_items, &completed_items, &running_items);

        match result {
            Some(try_item) =>
                // We have something runnable. Figure out how long it takes and add to running_items
                schedule(current_time, try_item, &mut running_items),
            None =>
                // Nothing to run, even though we have workers. Gotta sleep till a worker finishes
                current_time = sleep(&mut running_items, &mut completed_items)
        }
    }

    Ok(current_time)
}
