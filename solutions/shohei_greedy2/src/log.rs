

use search::Node;
use pos::Position;
use pos::Cell;
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

    pub fn write_pos(&mut self, pos: &Position) {
        let layer = self.writer.layer(0);
        for y in 0..pos.height {
            for x in 0..pos.width {
                let cell = pos.map[(y * pos.width + x) as usize];
                let str = if pos.body.x == x && pos.body.y == y {
                    "@"
                } else {
                    match cell {
                        Cell::Wall => "#",
                        Cell::Empty => ".",
                        Cell::Wrapped => "-",
                        Cell::B => "B",
                        Cell::F => "F",
                        Cell::L => "L",
                        Cell::R => "R",
                        Cell::X => "X",
                        Cell::C => "C",
                    }
                };
                layer.print(&str).unwrap();
            }
            layer.println("").unwrap();
        }
    }
    pub fn finish(&mut self) {
        self.writer.finish().unwrap();
    }
}
