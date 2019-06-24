use std::io::{Write, Seek};
use std::io::Result;
use std::f32::consts::PI;
use std::io::{Error, ErrorKind,SeekFrom};

pub struct BaseWriter<W:Write+Seek> {
    writer: W,
    pub offset: u32,
    pub entry_number:u32
}

impl<W:Write+Seek> BaseWriter<W> {
    pub fn new(writer:W) -> BaseWriter<W> {
        BaseWriter {
            writer,
            offset: 0,
            entry_number: 0,
        }
    }
    pub fn seek(&mut self, offset:i64)  -> Result<u64> {
        self.writer.seek(SeekFrom::Current(offset))
    }

    pub fn write_u8(&mut self, value:u8) -> Result<usize> {
        let result = self.writer.write(&[value]);
        self.offset += 1;
        result
    }
    pub fn write_u32(&mut self, value:u32) -> Result<usize> {
        let result = self.writer.write(
            &[
                (value & 0xff) as u8,
                ((value >> 8) & 0xff) as u8,
                ((value >> 16) & 0xff) as u8,
                ((value >> 24) & 0xff) as u8,
            ]
        );
        self.offset += 4;
        result
    }
    pub fn write_i32(&mut self, value:i32) -> Result<usize> {
        self.write_u32(value as u32)
    }
    pub fn write_f32(&mut self, value:f32) -> Result<usize> {
        let n = unsafe { *(&value as *const f32 as *const u32) };
        self.write_u32(n)
    }
    pub fn write(&mut self, buffer:&[u8]) -> Result<usize> {
        let result = self.writer.write(buffer);
        self.offset += buffer.len() as u32;
        result
    }

    pub fn finish(&mut self) -> Result<()> {
        self.writer.flush()
    }

    fn write_entry(
        &mut self,
        skip_frame_size:u32,
        layer_index:i32,
        clear:bool,
        buffer:&[u8]
    ) -> Result<usize> {
        self.entry_number += 1;
        try!(self.write_u32(skip_frame_size));
        try!(self.write_i32(layer_index));
        try!(self.write_u8(if clear { 1 } else { 0 }));
        try!(self.write_u32(buffer.len() as u32));
        self.write(buffer)
    }
}

pub struct BaseLayerWriter {
    entries:Vec<LayerEntry>,
}

impl BaseLayerWriter {
    pub fn new() -> BaseLayerWriter {
        BaseLayerWriter {
            entries: Vec::new(),
        }
    }

    pub fn entry(&mut self) -> &mut LayerEntry {
        if self.entries.is_empty() {
            self.entries.push(LayerEntry::new(false));
        }
        self.entries.last_mut().unwrap()
    }

    pub fn clear(&mut self) {
        self.entries.push(LayerEntry::new(true));
    }

    pub fn write_entries<W:Write+Seek>(
        &mut self,
        writer:&mut BaseWriter<W>,
        last_step: &mut u32,
        step: u32,
        layer_index: i32
    ) -> Result<()>
    {
        for entry in &mut self.entries {
            try!(writer.write_entry(
                step - *last_step,
                layer_index,
                entry.should_clear,
                &entry.buffer
            ));
            *last_step = step;
        }

        self.entries.clear();
        Ok(())
    }

}

pub struct LayerEntry {
    buffer:Vec<u8>,
    should_clear:bool
}

impl LayerEntry {
    pub fn new(should_clear:bool) -> LayerEntry {
        LayerEntry {
            buffer: Vec::new(),
            should_clear
        }
    }
    pub fn write_u8(&mut self, value:u8) -> Result<usize> {
        self.write(&[value])
    }
    pub fn write_u32(&mut self, value:u32) -> Result<usize> {
        self.write(
            &[
                (value & 0xff) as u8,
                ((value >> 8) & 0xff) as u8,
                ((value >> 16) & 0xff) as u8,
                ((value >> 24) & 0xff) as u8,
            ]
        )
    }
    pub fn write_u24(&mut self, value:u32) -> Result<usize> {
        if ((value >> 24) & 0xff) != 0 {
            return Err(
                Error::new(
                    ErrorKind::InvalidInput, 
                    format!("parameter value is over u24: {}", value)
                )
            );
        }
        self.write(
            &[
                (value & 0xff) as u8,
                ((value >> 8) & 0xff) as u8,
                ((value >> 16) & 0xff) as u8,
            ]
        )
    }
    pub fn write_u16(&mut self, value:u16) -> Result<usize> {
        self.write(
            &[
                ((value >> 8) & 0xff) as u8,
                (value & 0xff) as u8
            ]
        )
    }
    pub fn write_i32(&mut self, value:i32) -> Result<usize> {
        self.write_u32(value as u32)
    }
    pub fn write_f32(&mut self, value:f32) -> Result<usize> {
        let n = unsafe { *(&value as *const f32 as *const u32) };
        self.write_u32(n)
    }
    pub fn write_rotation(&mut self, value:f32, points:u16) -> Result<usize> {
        let max = PI * 2.0 / (points as f32);
        let value = ((value / max * 65535.0) % 65535.0 + 65535.0) % 65535.0; 
        self.write_u16(value as u16)
    }
    pub fn write_alpha(&mut self, value:f32) -> Result<usize> {
        self.write_u16(
            if value >= 1.0 { 65535 } else if value <= 0.0 { 0 } else { (value * 65535.0) as u16 }
        )
    }
    pub fn write(&mut self, buffer:&[u8]) -> Result<usize> {
        self.buffer.write(buffer)
    }
    pub fn write_short_str(&mut self, string:&str) -> Result<()> {
        let bytes = string.as_bytes();
        let len = bytes.len();
        if len > ::std::u16::MAX as usize {
            return Err(
                Error::new(
                    ErrorKind::InvalidInput,
                    format!("string has too long length: {}", len)
                )
            );
        }
        try!(self.write_u16(len as u16));
        try!(self.write(bytes));
        Ok(())
    }
    pub fn write_long_str(&mut self, string:&str) -> Result<()> {
        let bytes = string.as_bytes();
        let len = bytes.len();
        if len > 0xFFFFFF {
            return Err(
                Error::new(
                    ErrorKind::InvalidInput,
                    format!("string has too long length: {}", len)
                )
            );
        }
        try!(self.write_u24(len as u32));
        try!(self.write(bytes));
        Ok(())
    }
}

pub trait LogWriter {
    fn finish(&mut self) -> Result<()>;
    fn step(&mut self) -> Result<()>;
    fn skip(&mut self, offset:u32) -> Result<()>;
}

pub trait LogLayerWriter {
    fn clear(&mut self);
}
