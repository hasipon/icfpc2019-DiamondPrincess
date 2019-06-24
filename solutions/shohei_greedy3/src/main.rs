extern crate visual_log;

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
    let path = if args.len() <= 1 {
        "../../problems/prob-002.desc"
    } else {
        &args[1]
    };

    let mut position = read::read_file(Path::new(path));
    position.wrap(&mut Vec::new());
    let result = search::search(&mut position);
    let mut out = stdout();
    for mo in result {
        let command:String = match mo {
            Move::W => "W".to_string(),
            Move::A => "A".to_string(),
            Move::S => "S".to_string(),
            Move::D => "D".to_string(),
            Move::Clockwize => "E".to_string(),
            Move::Anticlockwise => "Q".to_string(),
            Move::B(point) => format!("B({},{})", point.x, point.y),
            Move::F => "F".to_string(),
            Move::L => "L".to_string(),
            Move::R => "R".to_string(),
            Move::T(x, y) => format!("T({},{})", x, y),
        };
        write!(out, "{}", command).unwrap();
    }
    writeln!(out, "");
    out.flush();
}
