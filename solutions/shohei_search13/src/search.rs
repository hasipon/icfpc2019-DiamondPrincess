
use pos::Position;
use pos::Point;
use pos::Move;
use pos::get_at;
use pos::*;
use std::collections::HashSet;
use std::collections::VecDeque;
use log::GameLogWriter;
use rand::prelude::ThreadRng;
use rand::Rng;

enum Mode {
    Normal(VecDeque<Move>),
    X(VecDeque<Move>),
    C(VecDeque<Move>),
}
pub fn search(position: &mut Position) -> Vec<(i32, Move)> {
    let beam_width = 3;
    let beam_depth = 1;
    let dfs_size = 3;
    let segment = 6;
    let repeat = 6;

    let mut min_cost = 100000000;
    let mut min_history:Vec<(i32, Move)> = Vec::new();

    let mut rng = rand::thread_rng(); 
    let mut logger = GameLogWriter::new();
    let mut rate = 0.0;

    for i in 0..segment {
        let mut cost = 0;
        let mut position = position.clone();
        let len = (min_history.len() as f64 * rate) as usize;
        eprintln!("l {} {}", min_history.len(), len);
        for mo in &min_history[0..len] {
            if mo.0 == 0 { cost += 1; }
            position.apply(false, &mo.1);
        }
        rate = 1.0 - (1.0 - rate) * 0.7;

        'outer: for j in 0..repeat {
            let mut cost = cost;
            let mut position = position.clone();
            let mut history = Vec::new();
            for mo in &min_history[0..len] {
                history.push(mo.clone());
            }
            let mut current_score = 0.0;
            let mut rest_unwrapped = position.count_unwrapped();
            let mut rest_c:HashSet<(i32, i32)> = position.get_map_c();
            let mut modes = Vec::new();
            
            loop {
                while modes.len() < position.len as usize {
                    modes.push(Mode::Normal(VecDeque::new()));
                }
                if position.index == 0 {
                    let mut searching_x = 0;
                    let mut normal = 0;
                    for i in 0..position.len {
                        match &modes[i as usize] {
                            Mode::X(_) => if position.rest_c == 0 { 
                                modes[i as usize] = Mode::Normal(VecDeque::new()); 
                            }
                            Mode::C(vec) => if vec.is_empty() { 
                                modes[i as usize] = Mode::Normal(VecDeque::new()); 
                            }
                            _ => {},
                        }
                        match &modes[i as usize] {
                            Mode::X(_) => searching_x += 1,
                            Mode::Normal(_) => normal += 1,
                            _ => {},
                        }
                    }
                    if searching_x == 0 && 0 < normal && 0 < position.rest_c {
                        let result = search_x(&position, &modes, &mut rng);
                        modes[result.index] = Mode::X(result.moves);
                        normal -= 1;
                    }
                    while 0 < normal && 0 < rest_c.len() {
                        let result = search_c(&position, &modes, &rest_c, &mut rng);
                        modes[result.index] = Mode::C(result.moves);
                        rest_c.remove(&result.c);
                        normal -= 1;
                    }
                }

                // logger.write_pos(&position);
                // logger.write_score(current_score, rest_unwrapped);
                
                let mut max = MaxData{
                    score: current_score,
                    mo: Move::Z
                };
                match &mut modes[position.index as usize]
                {
                    Mode::X(moves) => {
                        if let Option::Some(mo) = moves.pop_front() {
                            max.mo = mo;
                        } else {
                            max.mo = Move::C;
                        }
                    },
                    Mode::C(moves) => {
                        if let Option::Some(mo) = moves.pop_front() {
                            max.mo = mo;
                        } else {
                            panic!("c mode");
                        }
                    }
                    Mode::Normal(prev_target) => {
                        let body = position.get_body();
                        if 0 < body.rest_b {
                            let mut point = Point{
                                x: 1,
                                y: (body.manipulators.len() as i32 + 1) / 2
                            };
                            if body.manipulators.len() % 2 == 1 { point.y = -point.y };
                            max.mo = Move::B(point.counter_rotate(body.direction));
                        }
                        else
                        {
                            let mut results = Vec::new();
                            let mut prev_nodes = Vec::new();
                            prev_nodes.push(Node{ score:current_score, history:Vec::new() });

                            for _ in 0..beam_depth {
                                let mut nodes = Vec::new();
                                let mut len = prev_nodes.len();
                                if beam_width < len { len = beam_width; }
                                for i in 0..len {
                                    let node = &mut prev_nodes[i];
                                    for mo in &node.history {
                                        results.push(position.apply(true, mo).unwrap());
                                    }
                                    dfs(&mut nodes, &mut node.history.clone(), &mut position, node.score, dfs_size, &mut rng);
                                    node.history.reverse();
                                    for mo in &node.history {
                                        let result = results.pop().unwrap();
                                        position.back(mo, &result);
                                    }
                                }

                                nodes.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
                                prev_nodes = nodes;
                                if 0 < prev_nodes.len() && max.score + 0.001 > prev_nodes[0].score {
                                    break;
                                }
                            }
                            if 0 < prev_nodes.len() && max.score + 0.001 < prev_nodes[0].score {
                                max.score = prev_nodes[0].score;
                                max.mo = prev_nodes[0].history[0];
                            }

                            if max.score == current_score {
                                let mut mo = prev_target.pop_front();
                                if let Option::None = mo {
                                    if 0 < position.get_body().rest_l && rng.gen_range(0, 4) == 0 {
                                        mo = Option::Some(Move::L);
                                    } else {
                                        let mut target = position.find_target();
                                        while let Option::Some(mo) = target.moves.pop_front() {
                                            prev_target.push_back(mo);
                                        }
                                        mo = prev_target.pop_front();
                                    }
                                }
                                if let Option::Some(target_move) = mo {
                                    max.mo = target_move;
                                } else {
                                    prev_target.clear();
                                }
                            } else {
                                prev_target.clear();
                            }
                        }
                    }
                };
                {
                    if position.index == 0 { cost += 1; }
                    history.push((position.index, max.mo));
                    let result = position.apply(false, &max.mo).unwrap();
                    rest_unwrapped -= result.wrapped.len() as i32;
                    current_score += result.get_score(&position, position.width, position.height, &mut rng);
                    if min_cost < cost {
                        continue 'outer;
                    }
                }
                if rest_unwrapped <= 0 {
                    if position.index != 0 {
                        for i in position.index..position.len {
                            history.push((i, Move::Z));
                        }
                    }
                    break;
                }
            }
            
            // logger.write_pos(&position);
            // logger.write_score(current_score, rest_unwrapped);
            min_cost = cost;
            min_history = history;
            
            eprintln!("{} {} {}", min_cost, i, j);
        }
    }
    eprintln!("");

    logger.finish();
    return min_history;
}

fn dfs(nodes:&mut Vec<Node>, history:&mut Vec<Move>, position:&mut Position, current_score:f64, rest:i32, rng:&mut ThreadRng){
    let moves = position.get_available_moves();
    for next_move in &moves {
        let mut score = current_score;
        match (history.last(), next_move) {
            (Option::Some(Move::W), Move::W) => score += 0.00000001,
            (Option::Some(Move::A), Move::A) => score += 0.00000001,
            (Option::Some(Move::S), Move::S) => score += 0.00000001,
            (Option::Some(Move::D), Move::D) => score += 0.00000001,
            (Option::Some(Move::W), Move::S) => continue,
            (Option::Some(Move::A), Move::D) => continue,
            (Option::Some(Move::S), Move::W) => continue,
            (Option::Some(Move::D), Move::A) => continue,
            (Option::Some(Move::Clockwize), Move::Anticlockwise) => continue,
            (Option::Some(Move::Anticlockwise), Move::Clockwize) => continue,
            _ => {}
        }
        match position.apply(true, next_move) {
            Option::Some(result) => {
                history.push(*next_move);
                score += result.get_score(&position, position.width, position.height, rng) * (1.0 + rest as f64 * 0.1);
                if 0 < rest {
                    dfs(nodes, history, position, score, rest - 1, rng);
                } else {
                    nodes.push(Node{
                        score,
                        history: history.clone()
                    });
                }
                history.pop();
                position.back(next_move, &result);
            }
            Option::None => {}
        }
    }
}

struct SearchXResult {
    index:usize,
    moves:VecDeque<Move>,
}

struct SearchXNode {
    index: usize,
    x:i32,
    y:i32,
    fast:i32,
    drill:i32,
    moves: VecDeque<Move>,
}
fn search_x(position:&Position, modes:&Vec<Mode>, rng:&mut ThreadRng) -> SearchXResult {
    let mut visited = HashSet::new();
    let mut nodes = VecDeque::new();
    let mut index = 0;
    for mode in modes {
        if let Mode::Normal(_) = mode {
            let body = &position.bodies[index];
            nodes.push_back(SearchXNode{
                index,
                x: body.point.x,
                y: body.point.y,
                fast: body.fast,
                drill: body.drill,
                moves: VecDeque::new(),
            });
            visited.insert((body.point.x, body.point.y));
        }
        index += 1;
    }
    let mut ds = [
        (Move::A, -1, 0),
        (Move::W, 0, -1),
        (Move::S, 0, 1),
        (Move::D, 1, 0),
    ];
    loop {
        if rng.gen_range(0, 4) == 0 {
            ds.swap(0, 1);
        }
        if rng.gen_range(0, 4) == 0 {
            ds.swap(1, 2);
        }
        if rng.gen_range(0, 4) == 0 {
            ds.swap(1, 3);
        }
        let node = nodes.pop_front().unwrap();
        let x = node.x;
        let y = node.y;

        if get_at(&position.map, x, y, position.width).state == CellState::X {
            return SearchXResult {
                index: node.index,
                moves: node.moves,
            }
        }

        
        let is_fast = node.fast > 0;
        let is_drill = node.drill > 0;
        for (mo, dx, dy) in &ds
        {
            let mut dx = *dx;
            let mut dy = *dy;
            if is_fast && !position.is_out(x + dx, y + dy, is_drill) && !position.is_out(x + dx * 2, y + dy * 2, is_drill) {
                dx *= 2;
                dy *= 2;
            }
            if !position.is_out(x + dx, y + dy, is_drill) && !visited.contains(&(x + dx, y + dy)) { 
                let mut moves = node.moves.clone();
                moves.push_back(*mo);
                nodes.push_back(
                    SearchXNode {
                        index: node.index,
                        x: x + dx,
                        y: y + dy,
                        fast: node.fast - 1,
                        drill: node.drill - 1,
                        moves
                    }
                );
                visited.insert((x + dx, y + dy));
            }
        }
    }
}

struct SearchCResult {
    index:usize,
    moves:VecDeque<Move>,
    c:(i32, i32),
}
fn search_c(position:&Position, modes:&Vec<Mode>, rest_c:&HashSet<(i32, i32)>, rng:&mut ThreadRng) -> SearchCResult {
    let mut visited = HashSet::new();
    let mut nodes = VecDeque::new();
    let mut index = 0;
    for mode in modes {
        if let Mode::Normal(_) = mode {
            let body = &position.bodies[index];
            nodes.push_back(SearchXNode{
                index,
                x: body.point.x,
                y: body.point.y,
                fast: body.fast,
                drill: body.drill,
                moves: VecDeque::new(),
            });
            visited.insert((body.point.x, body.point.y));
        }
        index += 1;
    }
    let mut ds = [
        (Move::A, -1, 0),
        (Move::W, 0, -1),
        (Move::S, 0, 1),
        (Move::D, 1, 0),
    ];
    loop {
        if rng.gen_range(0, 4) == 0 {
            ds.swap(0, 1);
        }
        if rng.gen_range(0, 4) == 0 {
            ds.swap(1, 2);
        }
        if rng.gen_range(0, 4) == 0 {
            ds.swap(1, 3);
        }
        let node = nodes.pop_front().unwrap();
        let x = node.x;
        let y = node.y;

        if rest_c.contains(&(x, y)) {
            return SearchCResult {
                index: node.index,
                moves: node.moves,
                c: (x, y)
            }
        }
        let is_fast = node.fast > 0;
        let is_drill = node.drill > 0;
        for (mo, dx, dy) in &ds
        {
            let mut dx = *dx;
            let mut dy = *dy;
            if is_fast && !position.is_out(x + dx, y + dy, is_drill) && !position.is_out(x + dx * 2, y + dy * 2, is_drill) {
                dx *= 2;
                dy *= 2;
            }
            if !position.is_out(x + dx, y + dy, is_drill) && !visited.contains(&(x + dx, y + dy)) { 
                let mut moves = node.moves.clone();
                moves.push_back(*mo);
                nodes.push_back(
                    SearchXNode {
                        index: node.index,
                        x: x + dx,
                        y: y + dy,
                        fast: node.fast - 1,
                        drill: node.drill - 1,
                        moves
                    }
                );
                visited.insert((x + dx, y + dy));
            }
        }
    }
}

struct MaxData {
    score:f64,
    mo:Move,
}

struct Node {
    score:f64,
    history:Vec<Move>,
}
