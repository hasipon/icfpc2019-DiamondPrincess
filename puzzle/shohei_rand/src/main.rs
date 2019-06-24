extern crate visual_log;
extern crate rand;

use rand::{thread_rng, Rng};
use rand::prelude::ThreadRng;
pub mod read;
pub mod union;
pub mod pos;

use std::path::Path;
use std::env;
use std::io::{stdout, Write};
use pos::Puzzle;
use pos::value_at;
use pos::Task;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = if args.len() <= 1 {
        "../../chain-puzzle-examples/puzzle.cond"
    } else {
        &args[1]
    };

    let mut puzzle = read::read_file(Path::new(path));
    
    'outer: for _ in 0..100 {
        let mut fail_count = 0;
        let mut map = vec![true; (puzzle.size * puzzle.size) as usize];
        let mut rng = rand::thread_rng(); 
        let mut history = Vec::new();
        history.push(map.clone());

        loop {
            match progress(&puzzle, &mut map, &mut rng) {
                ProgressResult::Success => {
                    history.push(map.clone());
                    break;
                }
                ProgressResult::Continue => {
                    if rng.gen_range(0, 8) == 0 {
                        history.push(map.clone());
                    }
                }
                ProgressResult::Fail => {
                    if history.len() == 0 || 100 < fail_count{
                        continue 'outer;
                    } else {
                        eprintln!("fail {}", fail_count);
                        map = history.pop().unwrap();
                        fail_count += 1;
                    }
                }
            }
        }
        loop {
            match progress_vertex(&puzzle, &mut map, &mut rng) {
                ProgressResult::Success => {
                    history.push(map.clone());
                    break;
                }
                ProgressResult::Continue => {
                    if rng.gen_range(0, 8) == 0 {
                        history.push(map.clone());
                    }
                }
                ProgressResult::Fail => {
                    if history.len() == 0 || 200 < fail_count{
                        continue 'outer;
                    } else {
                        eprintln!("fail {}", fail_count);
                        map = history.pop().unwrap();
                        fail_count += 1;
                    }
                }
            }
        }

        let task = Task::new(&mut puzzle, &mut map);
        
        let mut out = stdout();
        let mut map_segs:Vec<String> = Vec::new();
        let mut boost_segs:Vec<String> = Vec::new();
        for point in task.map {
            map_segs.push(format!("({},{})", point.0, point.1))
        }
        for boost in task.boosts {
            boost_segs.push(format!("{}({},{})", boost.0, boost.1, boost.2));
        }
        writeln!(out, "{}#({},{})##{}", map_segs.join(","), task.worker.0, task.worker.1, boost_segs.join(";")); 
        out.flush().unwrap();
        return;
    }

    panic!("failed");
}

pub struct Node {
    map:Vec<bool>,
    score:f64,
}

enum ProgressResult {
    Success,
    Continue,
    Fail,
}

fn progress(puzzle:&Puzzle, map:&mut Vec<bool>, rng:&mut ThreadRng) -> ProgressResult {
    let result = puzzle.analyze(&map);
    let index = rng.gen_range(0, result.rest_black.len());
    let mut point = result.rest_black[index];
    let mut prev_point = point.clone();
    let (mut dx, mut dy) = match rng.gen_range(0i32, 4) {
        0 => (0, 1),
        1 => (-1, 0),
        2 => (0, -1),
        3 => (1, 0),
        _ => panic!("unknown rng result")
    };

    let mut curve_count = 0;
    while curve_count < 50 {
        map[(point.1 * puzzle.size + point.0) as usize] = false;
        let mut is_hit = false;
        
        if puzzle.white_list.contains(&(point.0 + dx, point.1 + dy)) {
            let tmp = dx;
            dx = -dy;
            dy = tmp;
            curve_count += 1;
            continue;
        }

        for i in 0..4 {
            let (ddx, ddy) = match i {
                0 => (0, 1),
                1 => (-1, 0),
                2 => (0, -1),
                3 => (1, 0),
                _ => panic!("unknown rng result")
            };

            let near_point = (point.0 + ddx, point.1 + ddy);
            if near_point == prev_point { continue; }

            if !value_at(map, near_point.0, near_point.1, puzzle.size) {
                
                is_hit = true;
                break;
            }
        }
        if is_hit {
            break;
        }

        prev_point = point;
        point = (prev_point.0 + dx, prev_point.1 + dy);
    }
    eprintln!("count {}", curve_count);
    
    let result = puzzle.analyze(map);
    if puzzle.is_faild(&result) {
        ProgressResult::Fail
    } else if result.rest_black.len() == 0 {
        ProgressResult::Success
    } else {
        ProgressResult::Continue
    }
}
fn progress_vertex(puzzle:&Puzzle, map:&mut Vec<bool>, rng:&mut ThreadRng) -> ProgressResult {
    let (mut dx, mut dy) = match rng.gen_range(0i32, 4) {
        0 => (0, 1),
        1 => (-1, 0),
        2 => (0, -1),
        3 => (1, 0),
        _ => panic!("unknown rng result")
    };

    let mut x = 0;
    let mut y = 0;
    loop
    {
        x = rng.gen_range(0i32, puzzle.size);
        y = rng.gen_range(0i32, puzzle.size);
        if value_at(map, x, y, puzzle.size) {
            break;
        }
    }
    loop {
        if value_at(map, x + dx, y + dy, puzzle.size) {
            x += dx;
            y += dy;
        } else {
            map[(y * puzzle.size + x) as usize] = false;
            break;
        }
    }

    let result = puzzle.analyze(map);
    if puzzle.is_faild(&result) {
        ProgressResult::Fail
    } else if puzzle.is_valid(&result) {
        ProgressResult::Success
    } else {
        ProgressResult::Continue
    }
}
