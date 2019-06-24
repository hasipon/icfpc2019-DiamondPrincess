
use pos::Position;
use pos::Point;
use pos::Move;

use log::GameLogWriter;

pub struct Node {
    pub score: f64,
    pub prev: Move,
    pub rest_unwrapped: i32,
}

pub fn search(mut position: &mut Position) -> Vec<Move> {
    let mut history = Vec::new();

    let mut logger = GameLogWriter::new();

    let mut current_score = 0.0;
    let mut rest_unwrapped = position.count_unwrapped();
    let mut prev_target = Option::None;


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
        else if 0 < position.rest_f && position.fast <= 0
        {
            max.mo = Move::F;
        }
        else
        {
            let moves = position.get_available_moves();
            for move0 in moves {
                dfs(&move0, &move0, &mut position, &mut max, current_score, 2);
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
            current_score = max.score;
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

fn dfs(root_move:&Move, next_move:&Move, position:&mut Position, max:&mut MaxData, current_score:f64, rest:i32){
    match position.apply(next_move) {
        Option::Some(result) => {
            let score = current_score + result.get_score(position.width, position.height) * (1.0 + rest as f64 * 0.1);
            if 0 < rest {
                let moves = position.get_available_moves();
                for mo in moves {
                    dfs(root_move, &mo, position, max, score, rest - 1)
                }
            } else {
                if max.score < score {
                    max.score = score;
                    max.mo = *root_move;
                }
            }
            position.back(next_move, &result);
        }
        Option::None => {}
    }
}

struct MaxData {
    score:f64,
    mo:Move,
}