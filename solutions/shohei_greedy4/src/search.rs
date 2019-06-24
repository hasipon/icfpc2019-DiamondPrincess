
use pos::Position;
use pos::Point;
use pos::Move;

use log::GameLogWriter;

pub struct Node {
    pub score: f64,
    pub prev: Move,
    pub rest_unwrapped: i32,
}

pub fn search(position: &mut Position) -> Vec<Move> {
    let default_moves = vec![Move::W,
                             Move::A,
                             Move::S,
                             Move::D,
                             Move::Clockwize,
                             Move::Anticlockwise];
    let mut history = Vec::new();

    let mut logger = GameLogWriter::new();

    let mut current_score = 0.0;
    let mut rest_unwrapped = position.count_unwrapped();
    let mut prev_target = Option::None;


    loop {
        logger.write_pos(&position);
        logger.write_score(current_score, rest_unwrapped);

        let mut max_score = current_score;
        let mut max_move = Move::Z;

        if 0 < position.rest_b
        {
            let mut point = Point{
                x: 1,
                y: (position.manipulators.len() as i32 + 1) / 2
            };
            if position.manipulators.len() % 2 == 1 { point.y = -point.y };
            max_move = Move::B(point.counter_rotate(position.direction));
        }
        else if 0 < position.rest_f && position.fast <= 0
        {
            max_move = Move::F;
        }
        else
        {
            
            for mo in &default_moves {
                match position.apply(mo)
                {
                    Option::Some(result) => {
                        let score = current_score + result.get_score(position.width, position.height);
                        position.back(mo, &result);
                        if max_score < score {
                            max_score = score;
                            max_move = *mo;
                        }
                    }
                    Option::None => {}
                } 
            }
            
            if max_score == current_score {
                let mut target = if prev_target.is_none() {
                    position.find_target()
                } else {
                    prev_target.unwrap()
                };
                if let Option::Some(target_move) = target.moves.pop_front() {
                    max_move = target_move;
                    prev_target = Option::Some(target);
                } else {
                    prev_target = Option::None;
                }
            } else {
                prev_target = Option::None;
            }
        }
        {
            history.push(max_move);
            let result = position.apply(&max_move).unwrap();
            rest_unwrapped -= result.wrapped.len() as i32;
            current_score = max_score;
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
