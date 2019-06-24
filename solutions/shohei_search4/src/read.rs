use super::pos::*;

use std::path::Path;
use std::fs::File;
use std::io::Read;

pub fn read_file(path: &Path) -> Position {
    let mut width: usize = 0;
    let mut height: usize = 0;

    let mut string = String::new();
    let mut file = File::open(path).unwrap();
    file.read_to_string(&mut string);

    let string = string.replace("),(", ")$(").replace("(", "").replace(")", "");

    let mut points: Vec<usize> = Vec::new();
    let seg: Vec<&str> = string.split("#").collect();
    let mines: Vec<&str> = seg[0].split("$").collect();
    for point_str in mines {
        let xy: Vec<&str> = point_str.split(",").collect();
        let x: usize = xy[0].parse().unwrap();
        let y: usize = xy[1].parse().unwrap();
        if width < x {
            width = x;
        }
        if height < y {
            height = y;
        }
        points.push(x);
        points.push(y);
    }

    let mut map: Vec<Cell> = vec![Cell{ state:CellState::Wall, is_wrapped:true }; width * height];
    mark_map(&mut map, &mut points, width);

    if seg[2] != "" {
        let obstacles: Vec<&str> = seg[2].split(";").collect();
        for obstacle_str in obstacles {
            let mut points: Vec<usize> = Vec::new();
            for point_str in obstacle_str.split("$") {
                let xy: Vec<&str> = point_str.split(",").collect();
                let x: usize = xy[0].parse().unwrap();
                let y: usize = xy[1].parse().unwrap();

                points.push(x);
                points.push(y);
            }
            mark_map(&mut map, &mut points, width);
        }
    }

    for y in 0..height {
        let mut is_wrapped = true;
        for x in 0..width {
            if !map[y * width + x].is_wrapped {
                is_wrapped = !is_wrapped;
            }
            let mut cell = &mut map[y * width + x];
            cell.is_wrapped = is_wrapped;
            cell.state = if is_wrapped { CellState::Wall } else { CellState::None };
        }
    }

    let worker: Vec<&str> = seg[1].split(",").collect();
    let worker_x: usize = worker[0].parse().unwrap();
    let worker_y: usize = worker[1].parse().unwrap();
    map[worker_y * width + worker_x].is_wrapped = true;
    let worker_y = height - worker_y - 1;


    if seg[3] != "" {
        for boosts in seg[3].split(";") {
            let key = match &boosts[0..1] {
                "B" => CellState::B,
                "F" => CellState::F,
                "L" => CellState::L,
                "X" => CellState::X,
                "R" => CellState::R,
                "C" => CellState::C,
                k => panic!("unknow_boost:{}", k),
            };

            let value: Vec<&str> = boosts[1..].split(",").collect();
            let x: usize = value[0].parse().unwrap();
            let y: usize = value[1].parse().unwrap();
            let cell = &mut map[y * width + x];
            if cell.is_wrapped || cell.state != CellState::None {
                panic!("conflict:{} {} {}", x, y, cell.state as i32)
            }
            cell.state = key;
        }
    }

    reverse_map(&mut map, width, height);
    Position {
        bodies: vec![
            Body {
                point: Point {
                    x: worker_x as i32,
                    y: worker_y as i32,
                },
                fast: 0,
                //drill: 0,
                direction: 0,
                manipulators: vec![
                    Point { x: 1, y: -1 },
                    Point { x: 1, y: 0 },
                    Point { x: 1, y: 1 },
                ],
                rest_b: 0,
                rest_f: 0,
                rest_l: 0,
                rest_r: 0,
            }
        ],
        index: 0,
        len: 1,
        rest_c: 0,
        map: map,
        width: width as i32,
        height: height as i32,
    }
}

fn mark_map(map: &mut Vec<Cell>, points: &mut Vec<usize>, width: usize) {
    if points[1] == points[3] {
        let point = points.remove(0);
        points.push(point);
        let point = points.remove(0);
        points.push(point);
    }

    for line in points.chunks(4) {
        let x0 = line[0];
        let mut y0 = line[1];
        let mut y1 = line[3];

        if y0 > y1 {
            std::mem::swap(&mut y0, &mut y1);
        }

        if width <= x0 {
            continue;
        }

        for y in y0..y1 {
            let mut cell = &mut map[y * width + x0];
            cell.is_wrapped = !cell.is_wrapped;
        }
    }
}

fn reverse_map(map: &mut Vec<Cell>, width: usize, height: usize) {
    for y0 in 0..(height / 2) as usize {
        let y1 = height - y0 - 1;
        for x in 0..width {
            map.swap(y0 * width + x, y1 * width + x);
        }
    }
}

pub fn read_buy_file(path: &Path) -> Vec<BoostItem> {
    let mut string = String::new();
    let mut file = File::open(path).unwrap();
    file.read_to_string(&mut string);
    let mut result = Vec::new();
    for seg in string.chars() {
        match seg {
            'B' => result.push(BoostItem::B),
            'F' => result.push(BoostItem::F),
            'L' => result.push(BoostItem::L),
            'R' => result.push(BoostItem::R),
            'C' => result.push(BoostItem::C),
            _ => {}
        }
    }
    return result;
}