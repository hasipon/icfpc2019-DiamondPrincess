use super::pos::*;

use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::collections::HashSet;

pub fn read_file(path: &Path) -> Puzzle {
    let mut width: usize = 0;
    let mut height: usize = 0;

    let mut string = String::new();
    let mut file = File::open(path).unwrap();
    file.read_to_string(&mut string);
    let string = string.replace("),(", ")$(").replace("(", "").replace(")", "");
    let mut seg:Vec<&str> = string.split("#").collect();
    
    let params:Vec<&str> = seg[0].split(",").collect();
    let mut white_list = HashSet::new();

    for point_str in seg[1].split("$") {
        let xy:Vec<&str> = point_str.split(",").collect();
        let x = xy[0].parse().unwrap();
        let y = xy[1].parse().unwrap();

        white_list.insert((x, y));
    }
    let mut black_list = HashSet::new();
    for point_str in seg[2].split("$") {
        let xy:Vec<&str> = point_str.split(",").collect();
        let x = xy[0].parse().unwrap();
        let y = xy[1].parse().unwrap();

        black_list.insert((x, y));
    }

    Puzzle {
        size : params[2].parse().unwrap(),
        v_min: params[3].parse().unwrap(),
        v_max: params[4].parse().unwrap(),
        m_num: params[5].parse().unwrap(),
        f_num: params[6].parse().unwrap(),
        d_num: params[7].parse().unwrap(),
        r_num: params[8].parse().unwrap(),
        c_num: params[9].parse().unwrap(),
        x_num: params[10].parse().unwrap(),
        white_list,
        black_list,
    }
}
