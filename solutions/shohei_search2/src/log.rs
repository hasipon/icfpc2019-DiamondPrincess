

use pos::Position;
use pos::CellState;
use std::fs::File;
use std::path::Path;

#[cfg(debug_assertions)]
use visual_log::log::write::LogWriter;
#[cfg(debug_assertions)]
use visual_log::log::write::LogLayerWriter;
#[cfg(debug_assertions)]
use visual_log::trlog::write::TrlogWriter;
#[cfg(debug_assertions)]
use visual_log::trlog::config::TrlogConfig;

#[cfg(debug_assertions)]
pub struct GameLogWriter {
    writer: TrlogWriter<File>,
}
#[cfg(not(debug_assertions))]
pub struct GameLogWriter {}

impl GameLogWriter {
    
    #[cfg(debug_assertions)]
    pub fn new() -> GameLogWriter {
        let file = File::create("pos.trlog").unwrap();
        GameLogWriter { writer: TrlogWriter::new(file, TrlogConfig::new()) }
    }

    #[cfg(debug_assertions)]
    pub fn write_score(&mut self, score: f64, rest: i32) {
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

    #[cfg(debug_assertions)]
    pub fn write_pos(&mut self, pos: &Position) {
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
    
    #[cfg(debug_assertions)]
    pub fn finish(&mut self) {
        self.writer.finish().unwrap();
    }

    
    #[cfg(not(debug_assertions))]
    pub fn new() -> GameLogWriter {
        GameLogWriter {}
    }

    #[cfg(not(debug_assertions))]
    pub fn write_score(&mut self, score: f64, rest: i32) {
    }

    #[cfg(not(debug_assertions))]
    pub fn write_pos(&mut self, pos: &Position) {
    }
    
    #[cfg(not(debug_assertions))]
    pub fn finish(&mut self) {
    }
}
