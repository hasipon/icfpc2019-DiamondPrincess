extern crate image;
use std::path::Path;
use std::fs;
use std::fs::File;
use std::io::Read;

fn main() {
    exec_dir(&Path::new("../problems"));
}

fn exec_dir(dir: &Path) {
    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if !path.is_dir() && path.file_name().unwrap().to_str().unwrap().ends_with(".desc") {
            exec_file(&path);
        }
    }
}
fn exec_file(path: &Path) {
    let out_name = format!("result/{}.png", path.file_stem().unwrap().to_str().unwrap());
    println!("{}", out_name);

    let out_path = Path::new(&out_name);
    let mut width: usize = 0;
    let mut height: usize = 0;

    let mut string = String::new();
    let mut file = File::open(path).unwrap();
    file.read_to_string(&mut string);
    let mut string = string.replace("),(", ")$(").replace("(", "").replace(")", "");

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

    let mut map = Vec::new();
    map.resize(width * height, 0);
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
        let mut flag = 0;
        for x in 0..width {
            if map[y * width + x] == 1 {
                flag ^= 1;
            }
            map[y * width + x] = flag;
        }
    }
    {
        let worker: Vec<&str> = seg[1].split(",").collect();
        let x: usize = worker[0].parse().unwrap();
        let y: usize = worker[1].parse().unwrap();
        map[y * width + x] = 2;
    }
    if seg[3] != "" {
        for boosts in seg[3].split(";") {
            let key = match &boosts[0..1] {
                "B" => 3,
                "F" => 4,
                "L" => 5,
                "X" => 6,
                "R" => 7,
                "C" => 8,
                k => panic!("unknow_boost:{}", k),
            };

            let value: Vec<&str> = boosts[1..].split(",").collect();
            let x: usize = value[0].parse().unwrap();
            let y: usize = value[1].parse().unwrap();
            if map[y * width + x] != 1 {
                panic!("conflict:{} {} {}", x, y, map[y * width + x])
            }
            map[y * width + x] = key;
        }
    }

    reverse_map(&mut map, width, height);

    let ow: usize = width * 4;
    let oh: usize = height * 4;
    let mut buffer: Vec<u8> = vec![0xFF; 4 * ow * oh];
    for x in 0..width {
        for y in 0..height {
            let value = map[y * width + x];
            for dx in 0..3 {
                let ox = x * 4 + dx;
                for dy in 0..3 {
                    let oy = y * 4 + dy;
                    let base = (oy * ow + ox) * 4;
                    if value == 0 {
                        buffer[base + 0] = 0x30;
                        buffer[base + 1] = 0x30;
                        buffer[base + 2] = 0x30;
                    } else if value == 1 {
                        buffer[base + 0] = 0xFF;
                        buffer[base + 1] = 0xFF;
                        buffer[base + 2] = 0xB0;
                    } else if value == 2 {
                        buffer[base + 0] = 0xF0;
                        buffer[base + 1] = 0x30;
                        buffer[base + 2] = 0x30;
                    } else if value == 3 {
                        // B
                        buffer[base + 0] = 0xF0;
                        buffer[base + 1] = 0xE0;
                        buffer[base + 2] = 0x00;
                    } else if value == 4 {
                        // F
                        buffer[base + 0] = 0x90;
                        buffer[base + 1] = 0x60;
                        buffer[base + 2] = 0x00;
                    } else if value == 5 {
                        // L
                        buffer[base + 0] = 0x20;
                        buffer[base + 1] = 0xF0;
                        buffer[base + 2] = 0x00;
                    } else if value == 6 {
                        // X
                        buffer[base + 0] = 0x00;
                        buffer[base + 1] = 0x00;
                        buffer[base + 2] = 0xF0;
                    } else if value == 7 {
                        // R
                        buffer[base + 0] = 0xF0;
                        buffer[base + 1] = 0x40;
                        buffer[base + 2] = 0xF0;
                    } else if value == 8 {
                        // C
                        buffer[base + 0] = 0x90;
                        buffer[base + 1] = 0x90;
                        buffer[base + 2] = 0xF0;
                    }
                }
            }
        }
    }

    image::save_buffer(&out_path, &buffer, ow as u32, oh as u32, image::RGBA(8)).unwrap();
}

fn mark_map(map: &mut Vec<i32>, points: &mut Vec<usize>, width: usize) {
    if points[1] == points[3] {
        let point = points.remove(0);
        points.push(point);
        let point = points.remove(0);
        points.push(point);
    }

    for line in points.chunks(4) {
        let x0 = line[0];
        let mut y0 = line[1];
        let x1 = line[2];
        let mut y1 = line[3];

        if y0 > y1 {
            std::mem::swap(&mut y0, &mut y1);
        }

        if width <= x0 {
            continue;
        }

        for y in y0..y1 {
            map[y * width + x0] ^= 1;
        }
    }
}

fn reverse_map(map: &mut Vec<i32>, width: usize, height: usize) {
    for y0 in 0..(height / 2) as usize {
        let y1 = height - y0 - 1;
        for x in 0..width {
            map.swap(y0 * width + x, y1 * width + x);
        }
    }
}
