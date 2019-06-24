
use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Clone, PartialEq, Debug)]
pub struct Cell {
    pub is_wrapped:bool,
    pub state:CellState,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum CellState {
    None,
    Wall,
    B,
    F,
    L,
    X,
    R,
    C,
}

#[derive(Clone)]
pub struct Position {
    pub index: i32,
    pub len:i32,
    pub bodies: Vec<Body>,
    pub map: Vec<Cell>,
    pub width: i32,
    pub height: i32,
    pub rest_c: i32,
}

pub struct MoveResult {
    pub len: i32,
    pub border_count: f64,
    pub wrapped: Vec<Point>,
    pub boost_item: Option<BoostItem>,
    pub boost_item2: Option<BoostItem>,
    pub move_fast: bool,
    
    pub prev_fast: i32,
    pub prev_rest_b: i32,
    pub prev_rest_f: i32,
    pub prev_rest_l: i32,
    pub prev_rest_c: i32,
    pub prev_rest_r: i32,
}


#[derive(Clone)]
pub struct Body {
    pub point:Point,
    pub direction: i32,
    pub manipulators: Vec<Point>,
    pub fast: i32,
    //pub drill: i32,
    
    pub rest_b: i32,
    pub rest_f: i32,
    pub rest_l: i32,
    pub rest_r: i32,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Move {
    W,
    A,
    S,
    D,
    Z,

    Clockwize,
    Anticlockwise,

    B(Point),
    F,
    L,
    //R,
    //T(i32, i32),
    C,
}

impl Move {
    pub fn get_string(&self) -> String {
        return match self {
            Move::W => "W".to_string(),
            Move::A => "A".to_string(),
            Move::S => "S".to_string(),
            Move::D => "D".to_string(),
            Move::Z => "Z".to_string(),
            Move::Clockwize => "E".to_string(),
            Move::Anticlockwise => "Q".to_string(),
            Move::B(point) => format!("B({},{})", point.x, point.y),
            Move::F => "F".to_string(),
            Move::L => "L".to_string(),
            //Move::R => "R".to_string(),
            //Move::T(x, y) => format!("T({},{})", x, y),
            Move::C => "C".to_string(),
        };
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn rotate(&self, direction:i32) -> Point {
        match direction {
            0 => Point{x:self.x, y:self.y},
            1 => Point{x:-self.y, y:self.x},
            2 => Point{x:-self.x, y:-self.y},
            3 => Point{x:self.y, y:-self.x},
            _ => panic!("unknown direction"),
        }
    }
    pub fn counter_rotate(&self, direction:i32) -> Point {
        match direction {
            0 => Point{x:self.x, y:self.y},
            1 => Point{x:self.y, y:-self.x},
            2 => Point{x:-self.x, y:-self.y},
            3 => Point{x:-self.y, y:self.x},
            _ => panic!("unknown direction"),
        }
    }
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
    pub fn count_unwrapped(&self) -> i32 {
        let mut count = 0;
        for cell in &self.map {
            if !cell.is_wrapped {
                count += 1;
            }
        }
        count
    }

    pub fn get_body(&self) -> &Body {
        &self.bodies[self.index as usize]
    }
    pub fn get_body_mut(&mut self) -> &mut Body {
        &mut self.bodies[self.index as usize]
    }

    pub fn wrap(&mut self, wrapped: &mut Vec<Point>) -> f64 {
        let body = &self.bodies[self.index as usize];
        let mut border_count = 0.0;

        if wrap_at(&mut self.map, body.point.x, body.point.y, self.width) {
            wrapped.push(body.point);
        }
        let manipulators = &body.manipulators;
        for point in manipulators {
            let Point{ x:dx, y:dy } = point.rotate(body.direction);
            let x = body.point.x + dx;
            let y = body.point.y + dy;

            if 0 <= x && x < self.width && 0 <= y && y < self.height &&
                self.is_reachable(x, y, self.width) &&
                wrap_at(&mut self.map, x, y, self.width) {
                if self.is_wrapped(x - 1, y) { border_count += 1.0 };
                if self.is_wrapped(x, y - 1) { border_count += 1.0 };
                if self.is_wrapped(x + 1, y) { border_count += 1.0 };
                if self.is_wrapped(x, y + 1) { border_count += 1.0 };
                wrapped.push(Point { x: x, y: y });
            }
        }

        border_count
    }

    fn is_reachable(&self, target_x: i32, target_y: i32, width: i32) -> bool {
        let body = self.get_body();
        let point = body.point;
        let dx = (target_x - body.point.x).abs();
        let dy = (target_y - body.point.y).abs();

        if dx < dy {
            let (x0, y0, x1, y1) = if point.y < target_y {
                (point.x, point.y, target_x, target_y)
            } else {
                (target_x, target_y, point.x, point.y)
            };
            let f = (x1 - x0) as f64 / dy as f64;
            for iy in 0..dy {
                let fx = 0.5 + x0 as f64 + (0.5 + iy as f64) * f;
                if (fx.floor() - fx).abs() > 0.0000000001 {
                    let x = fx as i32;
                    if get_at(&self.map, x, y0 + iy, width).state == CellState::Wall ||
                       get_at(&self.map, x, y0 + iy + 1, width).state == CellState::Wall {
                        return false;
                    }
                }
            }
        } else {
            let (x0, y0, x1, y1) = if point.x < target_x {
                (point.x, point.y, target_x, target_y)
            } else {
                (target_x, target_y, point.x, point.y)
            };
            let f = (y1 - y0) as f64 / dx as f64;
            for ix in 0..dx {
                let fy = 0.5 + y0 as f64 + (0.5 + ix as f64) * f;
                if (fy.floor() - fy).abs() > 0.0000000001 {
                    let y = fy as i32;
                    if get_at(&self.map, x0 + ix, y, width).state == CellState::Wall ||
                       get_at(&self.map, x0 + ix + 1, y, width).state == CellState::Wall {
                        return false;
                    }
                }
            }
        }
        return true;
    }

    pub fn apply(&mut self, for_search:bool, mo:&Move) -> Option<MoveResult> {
        let mut wrapped = Vec::new();
        let mut boost_item = Option::None;
        let mut boost_item2 = Option::None;
        let prev_len = self.len;
        let point;
        let x;
        let y;
        let prev_fast;
        let prev_rest_b;
        let prev_rest_f;
        let prev_rest_l;
        let prev_rest_r;
        let prev_rest_c;
        {
            let body = self.get_body();
            point = body.point;
            x = point.x;
            y = point.y;
            prev_fast = body.fast;
            prev_rest_b = body.rest_b;
            prev_rest_f = body.rest_f;
            prev_rest_l = body.rest_l;
            prev_rest_r = body.rest_r;
            prev_rest_c = self.rest_c;
        }
        let mut border_count = 0.0;
        let mut move_fast = false;
        
        match mo {
            Move::W => {
                if self.is_out(x, y - 1) { return Option::None; }
                self.get_body_mut().point.y -= 1;
                boost_item = self.get_boost_item();
                border_count += self.wrap(&mut wrapped);
            }
            Move::A => {
                if self.is_out(x - 1, y) { return Option::None; }
                self.get_body_mut().point.x -= 1;
                boost_item = self.get_boost_item();
                border_count += self.wrap(&mut wrapped);
            }
            Move::S => {
                if self.is_out(x, y + 1) { return Option::None; }
                self.get_body_mut().point.y += 1;
                boost_item = self.get_boost_item();
                border_count += self.wrap(&mut wrapped);
            }
            Move::D => {
                if self.is_out(x + 1, y) { return Option::None; }
                self.get_body_mut().point.x += 1;
                boost_item = self.get_boost_item();
                border_count += self.wrap(&mut wrapped);
            }
            Move::Z => {}
            Move::Clockwize => {
                let body = self.get_body_mut();
                body.direction += 1;
                body.direction %= 4;
                self.wrap(&mut wrapped);
            }
            Move::Anticlockwise => {
                let body = self.get_body_mut();
                body.direction += 3;
                body.direction %= 4;
                self.wrap(&mut wrapped);
            }
            Move::B(point) => {
                let body = self.get_body_mut();
                let point = point.rotate(body.direction);
                let point = Point{x:point.x, y:-point.y};
                body.manipulators.push(point);
                body.rest_b -= 1;
            }
            Move::F => {
                let body = self.get_body_mut();
                body.fast = 51;
                body.rest_f -= 1;
            }
            Move::C => {
                self.rest_c -= 1;
                let body = self.get_body();
                self.bodies.push(Body{
                    point: body.point,
                    direction: body.direction,
                    manipulators: vec![
                        Point { x: 1, y: -1 },
                        Point { x: 1, y: 0 },
                        Point { x: 1, y: 1 },
                    ],
                    fast: 0,
                    rest_b: 0,
                    rest_f: 0,
                    rest_l: 0,
                    rest_r: 0,
                });
            }
            _ => panic!("unknown move"),
        }
        
        if 0 < self.get_body().fast {
            let x = self.get_body().point.x;
            let y = self.get_body().point.y;
            match mo {
                Move::W => {
                    if !self.is_out(x, y - 1) {
                        self.get_body_mut().point.y -= 1;
                        boost_item2 = self.get_boost_item();
                        border_count += self.wrap(&mut wrapped);
                        move_fast = true;
                    }
                }
                Move::A => {
                    if !self.is_out(x - 1, y) {
                        self.get_body_mut().point.x -= 1;
                        boost_item2 = self.get_boost_item();
                        border_count += self.wrap(&mut wrapped);
                        move_fast = true;
                    }
                }
                Move::S => {
                    if !self.is_out(x, y + 1) {
                        self.get_body_mut().point.y += 1;
                        boost_item2 = self.get_boost_item();
                        border_count += self.wrap(&mut wrapped);
                        move_fast = true;
                    }
                }
                Move::D => {
                    if !self.is_out(x + 1, y) {
                        self.get_body_mut().point.x += 1;
                        boost_item2 = self.get_boost_item();
                        border_count += self.wrap(&mut wrapped);
                        move_fast = true;
                    }
                }
                _ => {}
            }
            self.get_body_mut().fast -= 1;
        }
        

        if !for_search {
            self.index += 1;
            if self.len == self.index {
                self.len = self.bodies.len() as i32;
                self.index = 0;
            }
        }

        Option::Some(MoveResult {
            len:prev_len,
            border_count,
            wrapped,
            boost_item,
            boost_item2,
            prev_fast,
            move_fast,
            prev_rest_b,
            prev_rest_f,
            prev_rest_l,
            prev_rest_c,
            prev_rest_r
        })
    }

    pub fn get_boost_item(&mut self) -> Option<BoostItem> {
        let point = self.get_body().point;
        let cell = get_at(&self.map, point.x, point.y, self.width);
        let result = match cell.state
        {
            CellState::B => { self.get_body_mut().rest_b += 1; Option::Some(BoostItem::B) }
            CellState::F => { self.get_body_mut().rest_f += 1; Option::Some(BoostItem::F) }
            CellState::L => { self.get_body_mut().rest_l += 1; Option::Some(BoostItem::L) }
            CellState::R => { self.get_body_mut().rest_r += 1; Option::Some(BoostItem::R) }
            CellState::C => { self.rest_c += 1; Option::Some(BoostItem::C) }
            _ => Option::None,
        };
        if result.is_some() {
            let cell = get_mut_at(&mut self.map, point.x, point.y, self.width);
            cell.state = CellState::None;
        }
        return result;
    }

    pub fn back(&mut self, mo:&Move, result:&MoveResult) {
        self.len = result.len;

        let boost_item = if result.move_fast { &result.boost_item2 } else { &result.boost_item };
        if let Option::Some(boost_item) = boost_item {
            let point = self.get_body().point;
            let mut cell = get_mut_at(&mut self.map, point.x, point.y, self.width);
            cell.state = match boost_item {
                BoostItem::B => { CellState::B },
                BoostItem::F => { CellState::F },
                BoostItem::L => { CellState::L },
                BoostItem::R => { CellState::R },
                BoostItem::C => { CellState::C },
            }
        }
        match mo {
            Move::W => {
                let body = self.get_body_mut();
                body.point.y += 1;
            }
            Move::A => {
                let body = self.get_body_mut();
                body.point.x += 1;
            }
            Move::S => {
                let body = self.get_body_mut();
                body.point.y -= 1;
            }
            Move::D => {
                let body = self.get_body_mut();
                body.point.x -= 1;
            }
            Move::Clockwize => {
                let body = self.get_body_mut();
                body.direction += 3;
                body.direction %= 4;
            }
            Move::Anticlockwise => {
                let body = self.get_body_mut();
                body.direction += 1;
                body.direction %= 4;
            }
            Move::B(_) => {
                let body = self.get_body_mut();
                body.manipulators.pop();
            }
            Move::F => {}
            Move::Z => {}
            _ => panic!("unknown move"),
        }
        if result.move_fast
        {
            if let Option::Some(boost_item) = &result.boost_item {
                let point = self.get_body().point;
                let mut cell = get_mut_at(&mut self.map, point.x, point.y, self.width);
                cell.state = match boost_item {
                    BoostItem::B => { CellState::B },
                    BoostItem::F => { CellState::F },
                    BoostItem::L => { CellState::L },
                    BoostItem::R => { CellState::R },
                    BoostItem::C => { CellState::C },
                }
            }
            match mo {
                Move::W => {
                     let body = self.get_body_mut();
                    body.point.y += 1;
                }
                Move::A => {
                    let body = self.get_body_mut();
                    body.point.x += 1;
                }
                Move::S => {
                    let body = self.get_body_mut();
                    body.point.y -= 1;
                }
                Move::D => {
                    let body = self.get_body_mut();
                    body.point.x -= 1;
                }
                _ => {}
            }
        }

        for point in &result.wrapped
        {
            get_mut_at(&mut self.map, point.x, point.y, self.width).is_wrapped = false;
        }
        {
            let body = self.get_body_mut();
            body.fast = result.prev_fast;
            body.rest_b = result.prev_rest_b; 
            body.rest_f = result.prev_rest_f; 
            body.rest_l = result.prev_rest_l; 
            body.rest_r = result.prev_rest_r; 
            self.rest_c = result.prev_rest_c; 
        }
    }

    pub fn is_out(&self, x:i32, y:i32) -> bool {
        x < 0 || self.width <= x || y < 0 || self.height <= y || get_at(&self.map, x, y, self.width).state == CellState::Wall
    }
    pub fn is_wrapped(&self, x:i32, y:i32) -> bool {
        self.is_out(x, y) || get_at(&self.map, x, y, self.width).is_wrapped
    }

    pub fn find_target(&self) -> Target {
        let body = self.get_body();
        let mut visited = HashSet::new();
        let mut nodes = VecDeque::new();
        

        nodes.push_back(FindTargetNode{
            x: body.point.x,
            y: body.point.y,
            moves: VecDeque::new(),
        });
        visited.insert((body.point.x, body.point.y));
        let mut rest = -1;
        let mut best_target = Option::None;
        let mut best_cost = 0.0;

        let mut dx = self.width as f64 / 2.0 - body.point.x as f64;
        let mut dy = self.height as f64 / 2.0 - body.point.y as f64;
        let d = (dx * dx + dy * dy).sqrt();

        if d != 0.0 {
            dx /= d;
            dy /= d;
        }
        let ds = [
            (Move::A, -1, 0),
            (Move::W, 0, -1),
            (Move::S, 0, 1),
            (Move::D, 1, 0),
        ];
        while let Option::Some(node) = nodes.pop_front() {
            rest -= 1;
            if rest == 0 { break; } 

            let x = node.x;
            let y = node.y;
            if !get_at(&self.map, x, y, self.width).is_wrapped
            {
                let cost = node.moves.len() as f64 + (x as f64 * dx + y as f64 * dy) * 0.8;
                
                if best_target.is_none() 
                {
                    rest = 100;
                    best_target = Option::Some(Target {
                        moves: node.moves.clone()
                    });
                    best_cost = cost;
                }
                else
                {
                    if best_cost > cost {
                        best_target = Option::Some(Target {
                            moves: node.moves.clone()
                        });
                        best_cost = cost;
                    }
                }
            }
            let is_fast = (body.fast - node.moves.len() as i32) > 0;

            for (mo, dx, dy) in &ds
            {
                let mut dx = *dx;
                let mut dy = *dy;
                if is_fast && !self.is_out(x + dx, y + dy) && !self.is_out(x + dx * 2, y + dy * 2) {
                    dx *= 2;
                    dy *= 2;
                }
                if !self.is_out(x + dx, y + dy) && !visited.contains(&(x + dx, y + dy)) { 
                    let mut moves = node.moves.clone();
                    moves.push_back(*mo);
                    nodes.push_back(
                        FindTargetNode {
                            x: x + dx,
                            y: y + dy,
                            moves
                        }
                    );
                    visited.insert((x + dx, y + dy));
                }
            }
        }
        return best_target.unwrap();
    }
    pub fn get_available_moves(&self) -> Vec<Move> {
        let body = self.get_body();
        let mut default_moves = vec![
            Move::W,
            Move::A,
            Move::S,
            Move::D,
            Move::Clockwize,
            Move::Anticlockwise
        ];
        if 0 < body.rest_f && body.fast <= 0
        {
            default_moves.push(Move::F);
        }
        return default_moves;
    }
    pub fn apply_boosts(&mut self, boosts:&Vec<BoostItem>) {
        let body = &mut self.bodies[self.index as usize];
        for boost in boosts {
            match boost {
                BoostItem::B => body.rest_b += 1,
                BoostItem::F => body.rest_f += 1,
                BoostItem::L => body.rest_l += 1,
                BoostItem::R => body.rest_r += 1,
                BoostItem::C => self.rest_c += 1,
            }
        }
    }
}

fn wrap_at(map: &mut Vec<Cell>, x: i32, y: i32, width: i32) -> bool {
    let mut cell = get_mut_at(map, x, y, width);
    if !cell.is_wrapped {
        cell.is_wrapped = true;
        true
    } else {
        false
    }
}

pub fn get_at(map: &Vec<Cell>, x: i32, y: i32, width: i32) -> &Cell {
    return &map[(y * width + x) as usize];
}
fn get_mut_at(map: &mut Vec<Cell>, x: i32, y: i32, width: i32) -> &mut Cell {
    return &mut map[(y * width + x) as usize];
}

pub enum BoostItem {
    B,
    F,
    L,
    R,
    C,
}

impl MoveResult {
    pub fn get_score(&self, width:i32, height:i32) -> f64 {
        let mut score = self.border_count * 4.1;
        for _ in &self.wrapped {
            score -= 3.5;
        }
        if let Option::Some(boost_item) = &self.boost_item {
            score += match boost_item {
                BoostItem::B => 25.0,
                BoostItem::F => 20.0,
                BoostItem::L => 0.0,
                BoostItem::R => 0.0,
                BoostItem::C => 0.0,
            }
        }
        if let Option::Some(boost_item2) = &self.boost_item2 {
            score += match boost_item2 {
                BoostItem::B => 25.0,
                BoostItem::F => 20.0,
                BoostItem::L => 0.0,
                BoostItem::R => 0.0,
                BoostItem::C => 0.0,
            }
        }
        score
    }
}
