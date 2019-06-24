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

    let mut map: Vec<Cell> = vec![Cell::Wall; width * height];
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
        let mut flag = Cell::Wall;
        for x in 0..width {
            if map[y * width + x] == Cell::Empty {
                if flag == Cell::Empty {
                    flag = Cell::Wall;
                } else {
                    flag = Cell::Empty;
                }
            }
            let cell: Cell = flag;
            map[y * width + x] = cell;
        }
    }

    let worker: Vec<&str> = seg[1].split(",").collect();
    let worker_x: usize = worker[0].parse().unwrap();
    let worker_y: usize = worker[1].parse().unwrap();
    map[worker_y * width + worker_x] = Cell::Wrapped;
    let worker_y = height - worker_y - 1;


    if seg[3] != "" {
        for boosts in seg[3].split(";") {
            let key = match &boosts[0..1] {
                "B" => Cell::B,
                "F" => Cell::F,
                "L" => Cell::L,
                "X" => Cell::X,
                "R" => Cell::R,
                "C" => Cell::C,
                k => panic!("unknow_boost:{}", k),
            };

            let value: Vec<&str> = boosts[1..].split(",").collect();
            let x: usize = value[0].parse().unwrap();
            let y: usize = value[1].parse().unwrap();
            if map[y * width + x] != Cell::Empty {
                panic!("conflict:{} {} {}", x, y, map[y * width + x] as i32)
            }
            map[y * width + x] = key;
        }
    }

    reverse_map(&mut map, width, height);
    Position {
        body: Point {
            x: worker_x as i32,
            y: worker_y as i32,
        },
        map: map,
        direction: 0,
        width: width as i32,
        height: height as i32,
        manipulators: vec![
            Point { x: 1, y: -1 },
            Point { x: 1, y: 0 },
            Point { x: 1, y: 1 },
        ],
        rest_b: 0,
        rest_f: 0,
        rest_l: 0,
        fast: 0,
        drill: 0,
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
            if map[y * width + x0] == Cell::Empty {
                map[y * width + x0] = Cell::Wall;
            } else {
                map[y * width + x0] = Cell::Empty;
            }
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
