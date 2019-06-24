

use search::Node;
use pos::Position;
use pos::CellState;
use visual_log::log::write::LogWriter;
use visual_log::log::write::LogLayerWriter;
use visual_log::trlog::write::TrlogWriter;
use visual_log::trlog::config::TrlogConfig;
use std::fs::File;
use std::path::Path;

pub struct GameLogWriter {
    writer: TrlogWriter<File>,
}

impl GameLogWriter {
    pub fn new() -> GameLogWriter {
        let file = File::create("pos.trlog").unwrap();
        GameLogWriter { writer: TrlogWriter::new(file, TrlogConfig::new()) }
    }

    pub fn write_score(&mut self, score: f64, rest: i32) {
        if cfg!(debug_assertions) {
            {
                let layer = self.writer.layer(0);
                layer.println(&format!("score:{}", score)).unwrap();
                layer.println(&format!("rest:{}", rest)).unwrap();
                self.writer.step();
            }
            {
                let layer = self.writer.layer(0);
                layer.clear();
            }
        }
    }

    pub fn write_pos(&mut self, pos: &Position) {
        if cfg!(debug_assertions) {
            let layer = self.writer.layer(0);
            for y in 0..pos.height {
                for x in 0..pos.width {
                    let cell = &pos.map[(y * pos.width + x) as usize];
                    let str = if pos.body.x == x && pos.body.y == y {
                        "@"
                    } else {
                        match cell.state {
                            CellState::Wall => "#",
                            CellState::None => if cell.is_wrapped { "-" } else { "." },
                            CellState::B => "B",
                            CellState::F => "F",
                            CellState::L => "L",
                            CellState::R => "R",
                            CellState::X => "X",
                            CellState::C => "C",
                        }
                    };
                    layer.print(&str).unwrap();
                }
                layer.println("").unwrap();
            }
        }
    }
    pub fn finish(&mut self) {
        self.writer.finish().unwrap();
    }
}
