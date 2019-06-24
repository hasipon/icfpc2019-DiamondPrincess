#[cfg(debug_assertions)]
extern crate visual_log;

extern crate rand;

pub mod pos;
pub mod read;
pub mod search;
pub mod log;

use std::path::Path;
use std::env;
use std::io::{stdout, Write};
use pos::Move;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path;
    let buy_path;
    
    if args.len() <= 1 {
        path = "../../problems/prob-002.desc";
        buy_path = Option::Some("dummy.buy");
    } else {
        path = &args[1];
        if args.len() <= 2 {
            buy_path = Option::None;
        }
        else
        {
            buy_path = Option::Some(&args[2]);
        }
    };

    let mut position = read::read_file(Path::new(path));
    if let Option::Some(buy_path) = &buy_path {
        let buy = read::read_buy_file(Path::new(buy_path));
        position.apply_boosts(&buy);
    }
    position.wrap(&mut Vec::new());
    let result = search::search(&mut position);
    let mut out = stdout();
    let mut commands:Vec<String> = Vec::new();
    for mo in result {
        let index = mo.0 as usize;
        let command:String = mo.1.get_string();
        while commands.len() <= index {
            commands.push(String::new());
        }
        commands[index].push_str(&command);
    }
    writeln!(out, "{}", commands.join("#"));
    out.flush();
}
