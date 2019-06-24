
use std::collections::HashSet;
use std::collections::VecDeque;
use union::union_find;

#[derive(Clone)]
pub struct Puzzle {
    pub size:i32,
    pub v_min:i32,
    pub v_max:i32,
    pub m_num:i32,
    pub f_num:i32,
    pub d_num:i32,
    pub r_num:i32,
    pub c_num:i32,
    pub x_num:i32,
    pub white_list:HashSet<(i32, i32)>,
    pub black_list:HashSet<(i32, i32)>,
}
#[derive(Debug)]
pub struct Task
{
    pub map:Vec<(i32, i32)>,
    pub boosts:Vec<(String, i32, i32)>,
    pub worker:(i32, i32),
}

impl Task {
    pub fn new(puzzle:&mut Puzzle, source:&Vec<bool>) -> Task {
        let mut phase = 0;
        let mut worker = (0, 0);
        let mut boosts = Vec::new();
        'outer: for x in 0..puzzle.size {
            for y in 0..puzzle.size {
                if value_at(&source, x, y, puzzle.size) {
                    loop {
                        match phase {
                            0 => {
                                worker = (x, y);
                                phase += 1;
                                break;
                            }
                            1 => if 0 == puzzle.m_num {
                                phase += 1;
                            } else {
                                puzzle.m_num -= 1;
                                boosts.push(("B".to_string(), x, y));
                                break;
                            }
                            2 => if 0 == puzzle.f_num {
                                phase += 1;
                            } else {
                                puzzle.f_num -= 1;
                                boosts.push(("F".to_string(), x, y));
                                break;
                            }
                            3 => if 0 == puzzle.d_num {
                                phase += 1;
                            } else {
                                puzzle.d_num -= 1;
                                boosts.push(("L".to_string(), x, y));
                                break;
                            }
                            4 => if 0 == puzzle.r_num {
                                phase += 1;
                            } else {
                                puzzle.r_num -= 1;
                                boosts.push(("R".to_string(), x, y));
                                break;
                            }
                            5 => if 0 == puzzle.x_num {
                                phase += 1;
                            } else {
                                puzzle.x_num -= 1;
                                boosts.push(("X".to_string(), x, y));
                                break;
                            }
                            6 => if 0 == puzzle.c_num {
                                phase += 1;
                            } else {
                                puzzle.c_num -= 1;
                                boosts.push(("C".to_string(), x, y));
                                break;
                            }
                            _ => break 'outer,
                        }
                    }
                }
            }
        }

        let mut map = Vec::new();
        let mut x = worker.0;
        let mut y = worker.1;
        let mut dx = 1;
        let mut dy = 0;
        let mut dir = 0;
        map.push((x, y));

        loop {
            x += dx; 
            y += dy;
            
            if value_at(source, x + dy, y - dx, puzzle.size) {
                let tmp = dx;
                dx = dy;
                dy = -tmp;
                match dir {
                    0 => map.push((x, y)),
                    1 => map.push((x, y + 1)),
                    2 => map.push((x + 1, y + 1)),
                    3 => map.push((x + 1, y)),
                    _ => panic!("1"),
                }
                dir += 1;
                dir %= 4;
            } else if value_at(source, x + dx, y + dy, puzzle.size) {
                
            } else if value_at(source, x - dy, y + dx, puzzle.size) {
                let tmp = dx;
                dx = -dy;
                dy = tmp;
                match dir {
                    0 => map.push((x + 1, y)),
                    1 => map.push((x, y)),
                    2 => map.push((x, y + 1)),
                    3 => map.push((x + 1, y + 1)),
                    _ => panic!("1"),
                }
                dir += 3;
                dir %= 4;
            } else {
                match dir {
                    0 => { 
                        map.push((x + 1, y));
                        map.push((x + 1, y + 1));
                    }
                    1 => { 
                        map.push((x, y));
                        map.push((x + 1, y));
                    }
                    2 => { 
                        map.push((x, y + 1));
                        map.push((x, y));
                    }
                    3 => { 
                        map.push((x + 1, y + 1));
                        map.push((x, y + 1));
                    }
                    _ => panic!("1"),
                }
                dx = -dx;
                dy = -dy;
                dir += 2;
                dir %= 4;
            }
            if x == worker.0 && y == worker.1 {
                break;
            }
        }
        
        loop {
            let point = map.pop().unwrap();
            if point == worker { break; }
        }

        Task {
            map,
            boosts,
            worker,
        }
    }
}

impl Puzzle 
{
    pub fn analyze(&self, map:&Vec<bool>) -> AnalyzeResult {
        let mut left = self.size;
        let mut top = self.size;
        let mut right = 0;
        let mut bottom = 0;
        let mut cell_count = 0;
        
        for x in 0..self.size {
            for y in 0..self.size {
                if map[(y * self.size + x) as usize] {
                    cell_count += 1;
                    if x < left {
                        left = x;
                    }
                    if right < x {
                        right = x;
                    }
                    if y < top {
                        top = y;
                    }
                    if bottom < y {
                        bottom = y;
                    }
                }
            }
        }

        let mut rest_white = Vec::new();
        let mut rest_black = Vec::new();
        for point in &self.white_list {
            if !map[(point.1 * self.size + point.0) as usize] {
                rest_white.push(point.clone());
            }
        }
        for point in &self.black_list {
            if map[(point.1 * self.size + point.0) as usize] {
                rest_black.push(point.clone());
            }
        }

        let union_count = union_find(map, self.size);
        let mut v_count = 0;
        
        for x in 0..self.size + 2 {
            for y in 0..self.size + 2 {
                let mut count = 0;
                if value_at(&map, x - 1, y, self.size) { count += 1; }
                if value_at(&map, x - 1, y - 1, self.size) { count += 1; }
                if value_at(&map, x, y - 1, self.size) { count += 1; }
                if value_at(&map, x, y, self.size) { count += 1; }
                if count == 1 || count == 3 {
                    v_count += 1;
                }
            }
        }

        AnalyzeResult {
            width: right - left,
            height: bottom - top,
            cell_count,
            rest_white,
            rest_black,
            v_count,
            union_count
        }
    }

    pub fn calc_score(&self, result:&AnalyzeResult) -> f64 {
        let mut score = 0.0;
        let max_wh = if result.width < result.height { result.height } else { result.width };
        score += (max_wh - self.size) as f64;
        score += result.cell_count as f64 * 0.05;
        score -= (result.v_count - (self.v_max + self.v_min) / 2).abs() as f64;
        score -= result.rest_white.len() as f64;
        score -= result.rest_black.len() as f64;
        score -= (result.union_count - 2).abs() as f64;
        score
    }

    pub fn is_valid(&self, result:&AnalyzeResult) -> bool {
        let max_wh = if result.width < result.height { result.height } else { result.width };

        eprintln!("v_count {}", result.v_count);

        let size = self.size as f64;
        if (max_wh as f64) < size * 0.9 { return false; }
        if (result.cell_count as f64) < size * size * 0.2 { return false; }
        if result.rest_black.len() != 0 { return false; }
        if result.rest_white.len() != 0 { return false; }
        if result.v_count < self.v_min || self.v_max < result.v_count { return false; }
        if result.union_count != 2 { return false; }

        true
    }
    pub fn is_faild(&self, result:&AnalyzeResult) -> bool {
        let max_wh = if result.width < result.height { result.height } else { result.width };

        eprintln!("union_count {} {}", result.union_count, result.rest_black.len());
        let size = self.size as f64;
        if (max_wh as f64) < size * 0.9 { return true; }
        if (result.cell_count as f64) < size * size * 0.2 { return true; }
        if result.rest_white.len() != 0 { return true; }
        if self.v_max < result.v_count  { return true; }
        if result.union_count != 2 { return true; }

        false
    }
}
#[derive(Debug)]
pub struct AnalyzeResult
{
    pub width:i32,
    pub height:i32,
    pub cell_count:i32,
    pub rest_white:Vec<(i32, i32)>,
    pub rest_black:Vec<(i32, i32)>,
    pub v_count:i32,
    pub union_count:i32,
}

pub fn value_at(map:&Vec<bool>, x:i32, y:i32, size:i32) -> bool {
    if x < 0 || size <= x || y < 0 || size <= y {
        false
    } else {
        map[(y * size + x) as usize]
    }
}