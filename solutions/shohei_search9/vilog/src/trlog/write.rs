use trlog::config::*;
use log::write::*;

use std::io::{Write, Seek};
use std::collections::BTreeMap;
use std::io::Result;
use super::TrlogCommandKind;

pub struct TrlogWriter<W: Write + Seek> {
    writer: BaseWriter<W>,
    last_step: u32,
    step: u32,
    pub layers: BTreeMap<i32, TrlogLayerWriter>,
}

impl<W: Write + Seek> TrlogWriter<W> {
    pub fn new(writer: W, config: TrlogConfig) -> TrlogWriter<W> {
        let mut base_writer = BaseWriter::new(writer);
        base_writer.write("trlo".as_bytes()).unwrap();
        base_writer.write_u32(1).unwrap();
        base_writer.write_u32(16).unwrap();

        base_writer.write_f32(config.fps).unwrap();

        base_writer.write_u32(0xFFFFFFFF).unwrap();
        base_writer.write_u32(0xFFFFFFFF).unwrap();
        base_writer.write_u32(0xFFFFFFFF).unwrap();

        TrlogWriter {
            writer: base_writer,
            layers: BTreeMap::new(),
            step: 0,
            last_step: 0,
        }
    }

    pub fn layer(&mut self, index: i32) -> &mut TrlogLayerWriter {
        self.layers.entry(index).or_insert(TrlogLayerWriter::new())
    }

    pub fn get_step(&self) -> u32 {
        self.step
    }

    fn _step(&mut self) -> Result<()> {
        for (index, layer) in &mut self.layers {
            try!(layer.writer
                      .write_entries(&mut self.writer, &mut self.last_step, self.step, *index));
        }
        Ok(())
    }
}

impl<W: Write + Seek> LogWriter for TrlogWriter<W> {
    fn finish(&mut self) -> Result<()> {
        try!(self._step());
        if self.step == self.last_step {
            self.step += 1;
        }
        let offset = self.writer.offset - 28;
        let entry_number = self.writer.entry_number;

        try!(self.writer.seek(-(offset as i64) - 12));
        try!(self.writer.write_u32(self.step));
        try!(self.writer.write_u32(entry_number));
        try!(self.writer.write_u32(offset));
        try!(self.writer.seek(offset as i64));

        self.writer.finish()
    }

    fn step(&mut self) -> Result<()> {
        self.skip(1)
    }

    fn skip(&mut self, offset: u32) -> Result<()> {
        try!(self._step());
        self.step += offset;
        Ok(())
    }
}

pub struct TrlogLayerWriter {
    writer: BaseLayerWriter,
}

impl TrlogLayerWriter {
    pub fn new() -> TrlogLayerWriter {
        TrlogLayerWriter { writer: BaseLayerWriter::new() }
    }

    pub fn print(&mut self, text: &str) -> Result<()> {
        let entry = self.writer.entry();
        try!(entry.write_u8(TrlogCommandKind::Print as u8));
        try!(entry.write_long_str(text));
        Ok(())
    }
    pub fn println(&mut self, text: &str) -> Result<()> {
        self.print(&format!("{}\n", text))
    }
}

impl LogLayerWriter for TrlogLayerWriter {
    fn clear(&mut self) {
        self.writer.clear()
    }
}
