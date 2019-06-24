
use pos::Position;
use pos::Point;
use pos::Move;

use log::GameLogWriter;

pub fn search(mut position: &mut Position) -> Vec<Move> {
    let mut history = Vec::new();

    let mut logger = GameLogWriter::new();

    let mut current_score = 0.0;
    let mut rest_unwrapped = position.count_unwrapped();
    let mut prev_target = Option::None;
    let beam_width = 3;
    let beam_depth = 3;

    loop {
        logger.write_pos(&position);
        logger.write_score(current_score, rest_unwrapped);

        let mut max = MaxData{
            score: current_score,
            mo: Move::Z
        };
        if 0 < position.rest_b {
            let mut point = Point{
                x: 1,
                y: (position.manipulators.len() as i32 + 1) / 2
            };
            if position.manipulators.len() % 2 == 1 { point.y = -point.y };
            max.mo = Move::B(point.counter_rotate(position.direction));
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
                        results.push(position.apply(mo).unwrap());
                    }
                    dfs(&mut nodes, &mut node.history.clone(), &mut position, node.score, 3);
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
                let mut target = if prev_target.is_none() {
                    position.find_target()
                } else {
                    prev_target.unwrap()
                };
                if let Option::Some(target_move) = target.moves.pop_front() {
                    max.mo = target_move;
                    prev_target = Option::Some(target);
                } else {
                    prev_target = Option::None;
                }
            } else {
                prev_target = Option::None;
            }
        }
        {
            history.push(max.mo);
            let result = position.apply(&max.mo).unwrap();
            rest_unwrapped -= result.wrapped.len() as i32;
            current_score += result.get_score(position.width, position.height);
            eprintln!("{}", max.score);
        }
        if rest_unwrapped <= 0 {
            break;
        }
    }

    logger.write_pos(&position);
    logger.write_score(current_score, rest_unwrapped);
    logger.finish();

    return history;
}

fn dfs(nodes:&mut Vec<Node>, history:&mut Vec<Move>, position:&mut Position, current_score:f64, rest:i32){
    let moves = position.get_available_moves();
    for next_move in &moves {
        let mut score = current_score;
        match (history.last(), next_move) {
            (Option::Some(Move::W), Move::W) => score += 0.000001,
            (Option::Some(Move::A), Move::A) => score += 0.000001,
            (Option::Some(Move::S), Move::S) => score += 0.000001,
            (Option::Some(Move::D), Move::D) => score += 0.000001,
            (Option::Some(Move::W), Move::S) => continue,
            (Option::Some(Move::A), Move::D) => continue,
            (Option::Some(Move::S), Move::W) => continue,
            (Option::Some(Move::D), Move::A) => continue,
            (Option::Some(Move::Clockwize), Move::Anticlockwise) => continue,
            (Option::Some(Move::Anticlockwise), Move::Clockwize) => continue,
            _ => {}
        }
        match position.apply(next_move) {
            Option::Some(result) => {
                history.push(*next_move);
                score += result.get_score(position.width, position.height) * (1.0 + rest as f64 * 0.1);
                if 0 < rest {
                    dfs(nodes, history, position, score, rest - 1);
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

struct MaxData {
    score:f64,
    mo:Move,
}

struct Node {
    score:f64,
    history:Vec<Move>,
}
