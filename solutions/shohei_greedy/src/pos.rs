
use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Cell {
    Wall = 0,
    Empty = 1,
    Wrapped = 2,
    B = 3,
    F = 4,
    L = 5,
    X = 6,
    R = 7,
    C = 8,
}

#[derive(Clone)]
pub struct Position {
    pub body: Point,
    pub map: Vec<Cell>,
    pub direction: i32,
    pub width: i32,
    pub height: i32,
    pub manipulators: Vec<Point>,
    pub rest_b: i32,
    pub rest_f: i32,
    pub rest_l: i32,
    pub fast: i32,
    pub drill: i32,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Move {
    W,
    A,
    S,
    D,

    Clockwize,
    Anticlockwise,

    B(i32, i32),
    F,
    L,
    R,
    T(i32, i32),
}

#[derive(Copy, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub struct Target {
    pub moves:VecDeque<Move>,
}

struct FindTargetNode {
    x:i32,
    y:i32,
    moves:VecDeque<Move>,
}

impl Position {
    pub fn count_empty(&self) -> i32 {
        let mut count = 0;
        for cell in &self.map {
            if *cell == Cell::Empty {
                count += 1;
            }
        }
        count
    }

    pub fn wrap(&mut self, wrapped: &mut Vec<Point>) {
        let body = self.body.clone();

        if wrap_at(&mut self.map, body.x, body.y, self.width) {
            wrapped.push(body);
        }
        let manipulators = &self.manipulators;
        for point in manipulators {
            let (dx, dy) = match self.direction {
                0 => (point.x, point.y),
                1 => (-point.y, point.x),
                2 => (-point.x, -point.y),
                3 => (point.y, -point.x),
                _ => panic!("unknown direction"),
            };
            let x = body.x + dx;
            let y = body.y + dy;

            if 0 <= x && x < self.width && 0 <= y && y < self.height &&
               self.is_reachable(x, y, self.width) &&
               wrap_at(&mut self.map, x, y, self.width) {
                wrapped.push(Point { x: x, y: y });
            }
        }
    }

    fn is_reachable(&self, target_x: i32, target_y: i32, width: i32) -> bool {
        let dx = (target_x - self.body.x).abs();
        let dy = (target_y - self.body.y).abs();

        if dx < dy {
            let (x0, y0, x1, y1) = if self.body.y < target_y {
                (self.body.x, self.body.y, target_x, target_y)
            } else {
                (target_x, target_y, self.body.x, self.body.y)
            };
            let f = (x1 - x0) as f64 / dy as f64;
            for iy in 0..dy {
                let fx = x0 as f64 + (0.5 + iy as f64) * f;
                if (fx.floor() - fx).abs() < 0.0000000001 {
                    let x = fx as i32;
                    if get_at(&self.map, x, y0 + iy, width) == Cell::Wall ||
                       get_at(&self.map, x, y0 + iy + 1, width) == Cell::Wall {
                        return false;
                    }
                }
            }
        } else if dx < dy {
            let (x0, y0, x1, y1) = if self.body.x < target_x {
                (self.body.x, self.body.y, target_x, target_y)
            } else {
                (target_x, target_y, self.body.x, self.body.y)
            };
            let f = (y1 - y0) as f64 / dx as f64;
            for ix in 0..dx {
                let fy = y0 as f64 + (0.5 + ix as f64) * f;
                if (fy.floor() - fy).abs() < 0.0000000001 {
                    let y = fy as i32;
                    if get_at(&self.map, x0 + ix, y, width) == Cell::Wall ||
                       get_at(&self.map, x0 + ix + 1, y, width) == Cell::Wall {
                        return false;
                    }
                }
            }
        }
        return true;
    }

    pub fn apply(&mut self, mo:&Move) -> Option<MoveResult> {
        let mut wrapped = Vec::new();
        let x = self.body.x;
        let y = self.body.y;
        match mo {
            Move::W => {
                if self.is_out(x, y - 1) { return Option::None; }
                self.body.y -= 1;
                self.wrap(&mut wrapped);
            }
            Move::A => {
                if self.is_out(x - 1, y) { return Option::None; }
                self.body.x -= 1;
                self.wrap(&mut wrapped);
            }
            Move::S => {
                if self.is_out(x, y + 1) { return Option::None; }
                self.body.y += 1;
                self.wrap(&mut wrapped);
            }
            Move::D => {
                if self.is_out(x + 1, y) { return Option::None; }
                self.body.x += 1;
                self.wrap(&mut wrapped);
            }
            Move::Clockwize => {
                self.direction += 1;
                self.direction %= 4;
                self.wrap(&mut wrapped);
            }
            Move::Anticlockwise => {
                self.direction += 3;
                self.direction %= 4;
                self.wrap(&mut wrapped);
            }
            _ => panic!("unknown move"),
        }
        Option::Some(MoveResult {
            wrapped
        })
    }
    
    pub fn back(&mut self, mo:&Move, result:&MoveResult) {
        match mo {
            Move::W => {
                self.body.y += 1;
            }
            Move::A => {
                self.body.x += 1;
            }
            Move::S => {
                self.body.y -= 1;
            }
            Move::D => {
                self.body.x -= 1;
            }
            Move::Clockwize => {
                self.direction += 3;
                self.direction %= 4;
            }
            Move::Anticlockwise => {
                self.direction += 1;
                self.direction %= 4;
            }
            _ => panic!("unknown move"),
        }
        for point in &result.wrapped
        {
            set_at(&mut self.map, point.x, point.y, self.width, Cell::Empty);
        }
    }
    pub fn is_out(&self, x:i32, y:i32) -> bool {
        x < 0 || self.width <= x || y < 0 || self.height <= y || get_at(&self.map, x, y, self.width) == Cell::Wall
    }

    pub fn find_target(&self) -> Target {
        let mut visited = HashSet::new();
        let mut nodes = VecDeque::new();

        nodes.push_back(FindTargetNode{
            x: self.body.x,
            y: self.body.y,
            moves: VecDeque::new(),
        });
        visited.insert((self.body.x, self.body.y));

        loop {
            if let Option::Some(node) = nodes.pop_front() {
                let x = node.x;
                let y = node.y;
                if (get_at(&self.map, x, y, self.width) == Cell::Empty)
                {
                    return Target {

                        moves: node.moves
                    }
                }

                if !self.is_out(x, y - 1) && !visited.contains(&(x, y - 1)) { 
                    let mut moves = node.moves.clone();
                    moves.push_back(Move::W);
                    nodes.push_back(
                        FindTargetNode {
                            x: x,
                            y: y - 1,
                            moves
                        }
                    );
                    visited.insert((x, y - 1));
                }
                if !self.is_out(x - 1, y)  && !visited.contains(&(x - 1, y)) { 
                    let mut moves = node.moves.clone();
                    moves.push_back(Move::A);
                    nodes.push_back(
                        FindTargetNode {
                            x: x - 1,
                            y: y,
                            moves
                        }
                    );
                    visited.insert((x - 1, y));
                 }
                if !self.is_out(x, y + 1)  && !visited.contains(&(x, y + 1)){ 
                    let mut moves = node.moves.clone();
                    moves.push_back(Move::S);
                    nodes.push_back(
                        FindTargetNode {
                            x: x,
                            y: y + 1,
                            moves
                        }
                    );
                    visited.insert((x, y + 1));
                 }
                if !self.is_out(x + 1, y) && !visited.contains(&(x + 1, y)){ 
                    let mut moves = node.moves;
                    moves.push_back(Move::D);
                    nodes.push_back(
                        FindTargetNode {
                            x: x + 1,
                            y: y,
                            moves
                        }
                    );
                    visited.insert((x + 1, y));
                 }
                
            } else {
                panic!("target not found");
            }
        }
    }
}

fn wrap_at(map: &mut Vec<Cell>, x: i32, y: i32, width: i32) -> bool {
    if get_at(map, x, y, width) == Cell::Empty {
        set_at(map, x, y, width, Cell::Wrapped);
        true
    } else {
        false
    }
}

fn set_at(map: &mut Vec<Cell>, x: i32, y: i32, width: i32, cell: Cell) {
    map[(y * width + x) as usize] = cell;
}

fn get_at(map: &Vec<Cell>, x: i32, y: i32, width: i32) -> Cell {
    return map[(y * width + x) as usize];
}


pub struct MoveResult {
    pub wrapped: Vec<Point>,
}

impl MoveResult {
    pub fn get_score(&self) -> f64 {
        self.wrapped.len() as f64
    }
}
