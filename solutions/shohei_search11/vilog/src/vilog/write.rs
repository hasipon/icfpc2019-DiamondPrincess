use vilog::config::*;
use log::write::*;

use std::io::{Write, Seek};
use std::collections::BTreeMap;
use std::io::Result;
use super::VilogCommandKind;
use super::BlendMode;
use std::io::{Error, ErrorKind};

pub struct VilogWriter<W:Write+Seek> {
    writer:BaseWriter<W>,
    last_step:u32,
    step:u32,
    texture_required:bool,
    pub layers:BTreeMap<i32, VilogLayerWriter>,
}

impl<W:Write+Seek> VilogWriter<W> {
    pub fn new(
        writer:W,
        config: VilogConfig
    ) -> VilogWriter<W> {
        let mut base_writer = BaseWriter::new(writer);
        base_writer.write("vilo".as_bytes()).unwrap();
        base_writer.write_u32(1).unwrap();
        base_writer.write_u32(29).unwrap();

        base_writer.write_u32(config.width).unwrap();
        base_writer.write_u32(config.height).unwrap();
        base_writer.write_f32(config.fps).unwrap();
        base_writer.write_u32(config.background).unwrap();

        base_writer.write_u8(2).unwrap();
        base_writer.write_u32(0xFFFFFFFF).unwrap();
        base_writer.write_u32(0xFFFFFFFF).unwrap();
        base_writer.write_u32(0xFFFFFFFF).unwrap();

        VilogWriter {
            writer : base_writer,
            layers : BTreeMap::new(),
            step : 0,
            last_step : 0,
            texture_required: false
        }
    }
    
    pub fn layer(&mut self, index:i32) -> &mut VilogLayerWriter {
        self.layers.entry(index).or_insert(VilogLayerWriter::new())
    }

    pub fn get_step(&self) -> u32 {
        self.step
    }

    fn _step(&mut self) -> Result<()> {
        for (index, layer) in &mut self.layers {
            try!(layer.writer.write_entries(
                &mut self.writer,
                &mut self.last_step,
                self.step,
                *index
            ));
            self.texture_required = layer.texture_required;
        }
        Ok(())
    }
}

impl<W:Write+Seek> LogWriter for VilogWriter<W> {
    fn finish(&mut self) -> Result<()> {
        try!(self._step());
        let offset = self.writer.offset - 41;
        let entry_number = self.writer.entry_number;

        try!(self.writer.seek(-(offset as i64) -13));
        try!(self.writer.write_u8(if self.texture_required {1} else {0}));
        try!(self.writer.write_u32(self.step));
        try!(self.writer.write_u32(entry_number));
        try!(self.writer.write_u32(offset));
        try!(self.writer.seek(offset as i64));

        self.writer.finish()
    }

    fn step(&mut self) -> Result<()> {
        self.skip(1)
    }

    fn skip(&mut self, offset:u32) -> Result<()> {
        try!(self._step());
        self.step += offset;
        Ok(())
    }
}

pub struct VilogLayerWriter {
    writer: BaseLayerWriter,
    texture_required: bool,
}

impl VilogLayerWriter {
    pub fn new() -> VilogLayerWriter {
        VilogLayerWriter {
            writer: BaseLayerWriter::new(),
            texture_required: false
        }
    }

    pub fn line_to(&mut self, to_x:f32, to_y:f32) -> Result<()> {
        let entry = self.writer.entry();
        try!(entry.write_u8(VilogCommandKind::LineTo as u8));
        try!(entry.write_f32(to_x));
        try!(entry.write_f32(to_y));
        Ok(())
    }
    
    pub fn move_to(&mut self, to_x:f32, to_y:f32) -> Result<()> {
        let entry = self.writer.entry();
        try!(entry.write_u8(VilogCommandKind::MoveTo as u8));
        try!(entry.write_f32(to_x));
        try!(entry.write_f32(to_y));
        Ok(())
    }

    pub fn quaratic_curve_to(&mut self, ctrl_x:f32, ctrl_y:f32, to_x:f32, to_y:f32) -> Result<()>{
        let entry = self.writer.entry();
        try!(entry.write_u8(VilogCommandKind::QuaraticCurveTo as u8));
        try!(entry.write_f32(ctrl_x));
        try!(entry.write_f32(ctrl_y));
        try!(entry.write_f32(to_x));
        try!(entry.write_f32(to_y));
        Ok(())
    }

    pub fn bezier_curve_to(&mut self, ctrl_x:f32, ctrl_y:f32, ctrl2_x:f32, ctrl2_y:f32, to_x:f32, to_y:f32) -> Result<()>{
        let entry = self.writer.entry();
        try!(entry.write_u8(VilogCommandKind::BezierCurveTo as u8));
        try!(entry.write_f32(ctrl_x));
        try!(entry.write_f32(ctrl_y));
        try!(entry.write_f32(ctrl2_x));
        try!(entry.write_f32(ctrl2_y));
        try!(entry.write_f32(to_x));
        try!(entry.write_f32(to_y));
        Ok(())
    }

    pub fn arc(&mut self, ctrl_x:f32, ctrl_y:f32, radius:f32, start_angle:f32, end_angle:f32, anticlockwise:bool) -> Result<()>{
        let entry = self.writer.entry();
        try!(entry.write_u8(VilogCommandKind::Arc as u8));
        try!(entry.write_f32(ctrl_x));
        try!(entry.write_f32(ctrl_y));
        try!(entry.write_f32(radius));
        try!(entry.write_rotation(start_angle, 1));
        try!(entry.write_rotation(end_angle, 1));
        try!(entry.write_u8(if anticlockwise {1} else {0}));
        Ok(())
    }

    pub fn arc_to(&mut self, ctrl_x:f32, ctrl_y:f32, to_x:f32, to_y:f32, radius:f32) -> Result<()>{
        let entry = self.writer.entry();
        try!(entry.write_u8(VilogCommandKind::ArcTo as u8));
        try!(entry.write_f32(ctrl_x));
        try!(entry.write_f32(ctrl_y));
        try!(entry.write_f32(to_x));
        try!(entry.write_f32(to_y));
        try!(entry.write_f32(radius));
        Ok(())
    }
    
    pub fn fill(&mut self) -> Result<()> {
        let entry = self.writer.entry();
        try!(entry.write_u8(VilogCommandKind::Fill as u8));
        Ok(())
    }

    pub fn close_path(&mut self) -> Result<()> {
        let entry = self.writer.entry();
        try!(entry.write_u8(VilogCommandKind::ClosePath as u8));
        Ok(())
    }
    
    pub fn draw_circle(&mut self, x:f32, y:f32, radius:f32) -> Result<()> {
        let entry = self.writer.entry();
        try!(entry.write_u8(VilogCommandKind::DrawCircle as u8));
        try!(entry.write_f32(x));
        try!(entry.write_f32(y));
        try!(entry.write_f32(radius));
        Ok(())
    }

    pub fn draw_ellipse(&mut self, x:f32, y:f32, width:f32, height:f32) -> Result<()> {
        let entry = self.writer.entry();
        try!(entry.write_u8(VilogCommandKind::DrawEllipse as u8));
        try!(entry.write_f32(x));
        try!(entry.write_f32(y));
        try!(entry.write_f32(width));
        try!(entry.write_f32(height));
        Ok(())
    }

    pub fn draw_rectangle(&mut self, x:f32, y:f32, width:f32, height:f32) -> Result<()> {
        let entry = self.writer.entry();
        try!(entry.write_u8(VilogCommandKind::DrawRectangle as u8));
        try!(entry.write_f32(x));
        try!(entry.write_f32(y));
        try!(entry.write_f32(width));
        try!(entry.write_f32(height));
        Ok(())
    }

    pub fn draw_rounded_rectangle(&mut self, x:f32, y:f32, width:f32, height:f32, radius:f32) -> Result<()> {
        let entry = self.writer.entry();
        try!(entry.write_u8(VilogCommandKind::DrawRoundRectagle as u8));
        try!(entry.write_f32(x));
        try!(entry.write_f32(y));
        try!(entry.write_f32(width));
        try!(entry.write_f32(height));
        try!(entry.write_f32(radius));
        Ok(())
    }
    
    pub fn draw_regular_polygon(&mut self, x:f32, y:f32, points:u16, radius:f32, rotation:f32) -> Result<()> {
        let entry = self.writer.entry();
        try!(entry.write_u8(VilogCommandKind::DrawRegularPolygon as u8));
        try!(entry.write_f32(x));
        try!(entry.write_f32(y));
        try!(entry.write_u16(points));
        try!(entry.write_f32(radius));
        try!(entry.write_rotation(rotation, points));
        Ok(())
    }
    
    pub fn draw_star(&mut self, x:f32, y:f32, points:u16, radius:f32, inner_radius:f32, rotation:f32) -> Result<()> {
        let entry = self.writer.entry();
        try!(entry.write_u8(VilogCommandKind::DrawStar as u8));
        try!(entry.write_f32(x));
        try!(entry.write_f32(y));
        try!(entry.write_u16(points));
        try!(entry.write_f32(radius));
        try!(entry.write_f32(inner_radius));
        try!(entry.write_rotation(rotation, points));
        Ok(())
    }

    pub fn draw_grid(&mut self, x:f32, y:f32, grid_width:u16, grid_height:u16, cell_width:f32, cell_height:f32) -> Result<()> {
        let entry = self.writer.entry();
        try!(entry.write_u8(VilogCommandKind::DrawGrid as u8));
        try!(entry.write_f32(x));
        try!(entry.write_f32(y));
        try!(entry.write_u16(grid_width));
        try!(entry.write_u16(grid_height));
        try!(entry.write_f32(cell_width));
        try!(entry.write_f32(cell_height));
        Ok(())
    }
    
    pub fn draw_columns(&mut self, x:f32, bottom:f32, column_width:f32, margin:f32, heights:&[f32]) -> Result<()> {
        self.draw_columns_with_scale(x, bottom, column_width, margin, heights, 1.0)
    }
    pub fn draw_columns_with_scale(&mut self, x:f32, bottom:f32, column_width:f32, margin:f32, heights:&[f32], scale_y:f32) -> Result<()> {
        let entry = self.writer.entry();
        try!(entry.write_u8(VilogCommandKind::DrawRectangle as u8));
        try!(entry.write_f32(x));
        try!(entry.write_f32(bottom));
        try!(entry.write_f32(column_width));
        try!(entry.write_f32(margin));
        let len = heights.len();
        if len > ::std::u16::MAX as usize {
            return Err(
                Error::new(
                    ErrorKind::InvalidInput,
                    format!("heights has too long length: {}", len)
                )
            );
        }
        try!(entry.write_u16(len as u16));
        for height in heights {
            try!(entry.write_f32(height * scale_y));
        }
        Ok(())
    }
    pub fn arrow_to(
        &mut self, 
        to_x:f32, 
        to_y:f32, 
        tail_visible:bool,
        body_visible:bool,
        head_visible:bool,
        arrow_width:f32,
        arrow_height:f32
    ) -> Result<()>
    {
        let entry = self.writer.entry();
        try!(entry.write_u8(VilogCommandKind::ArrowTo as u8));
        try!(entry.write_f32(to_x));
        try!(entry.write_f32(to_y));
        try!(entry.write_u8(
            (if tail_visible { 1 } else { 0 }) |
            (if body_visible { 1 } else { 0 }) << 1 |
            (if head_visible { 1 } else { 0 }) << 2
        ));
        try!(entry.write_f32(arrow_width));
        try!(entry.write_f32(arrow_height));
        Ok(())
    }
    pub fn curve_arrow_to(
        &mut self, 
        ctrl_x:f32, 
        ctrl_y:f32, 
        to_x:f32, 
        to_y:f32, 
        tail_visible:bool,
        body_visible:bool,
        head_visible:bool,
        arrow_width:f32,
        arrow_height:f32
    ) -> Result<()>
    {
        let entry = self.writer.entry();
        try!(entry.write_u8(VilogCommandKind::CurveArrowTo as u8));
        try!(entry.write_f32(ctrl_x));
        try!(entry.write_f32(ctrl_y));
        try!(entry.write_f32(to_x));
        try!(entry.write_f32(to_y));
        try!(entry.write_u8(
            (if tail_visible { 1 } else { 0 }) |
            (if body_visible { 1 } else { 0 }) << 1 |
            (if head_visible { 1 } else { 0 }) << 2
        ));
        try!(entry.write_f32(arrow_width));
        try!(entry.write_f32(arrow_height));
        Ok(())
    }
    pub fn arc_arrow(
        &mut self, 
        ctrl_x:f32, 
        ctrl_y:f32, 
        radius:f32, 
        start_angle:f32, 
        end_angle:f32,
        anticlockwise:bool,
        tail_visible:bool,
        body_visible:bool,
        head_visible:bool,
        arrow_width:f32,
        arrow_height:f32
    ) -> Result<()>{
        let entry = self.writer.entry();
        try!(entry.write_u8(VilogCommandKind::ArcArrowTo as u8));
        try!(entry.write_f32(ctrl_x));
        try!(entry.write_f32(ctrl_y));
        try!(entry.write_f32(radius));
        try!(entry.write_rotation(start_angle, 1));
        try!(entry.write_rotation(end_angle, 1));
        try!(entry.write_u8(
            (if anticlockwise { 1 } else { 0 })  |
            (if tail_visible  { 1 } else { 0 }) << 1 |
            (if body_visible  { 1 } else { 0 }) << 2 |
            (if head_visible  { 1 } else { 0 }) << 3 
        ));
        try!(entry.write_f32(arrow_width));
        try!(entry.write_f32(arrow_height));
        Ok(())
    }

    pub fn multply_transform(
        &mut self, 
        a:f32, 
        b:f32, 
        c:f32,
        d:f32,
        tx:f32,
        ty:f32
    ) -> Result<()>
    {
        let entry = self.writer.entry();
        try!(entry.write_u8(VilogCommandKind::MultiplyTranform as u8));
        try!(entry.write_f32(a));
        try!(entry.write_f32(b));
        try!(entry.write_f32(c));
        try!(entry.write_f32(d));
        try!(entry.write_f32(tx));
        try!(entry.write_f32(ty));
        Ok(())
    }
    pub fn reset_transform(
        &mut self, 
        a:f32, 
        b:f32, 
        c:f32,
        d:f32,
        tx:f32,
        ty:f32
    ) -> Result<()>
    {
        let entry = self.writer.entry();
        try!(entry.write_u8(VilogCommandKind::ResetTranform as u8));
        try!(entry.write_f32(a));
        try!(entry.write_f32(b));
        try!(entry.write_f32(c));
        try!(entry.write_f32(d));
        try!(entry.write_f32(tx));
        try!(entry.write_f32(ty));
        Ok(())
    }

    pub fn multply_alpha(
        &mut self, 
        alpha:f32, 
    ) -> Result<()>
    {
        let entry = self.writer.entry();
        try!(entry.write_u8(VilogCommandKind::MultiplyAlpha as u8));
        try!(entry.write_alpha(alpha));
        Ok(())
    }
    pub fn reset_alpha(
        &mut self, 
        alpha:f32
    ) -> Result<()>
    {
        let entry = self.writer.entry();
        try!(entry.write_u8(VilogCommandKind::ResetAlpha as u8));
        try!(entry.write_alpha(alpha));
        Ok(())
    }

    pub fn set_line_alpha(
        &mut self, 
        alpha:f32, 
    ) -> Result<()>
    {
        let entry = self.writer.entry();
        try!(entry.write_u8(VilogCommandKind::SetLineAlpha as u8));
        try!(entry.write_alpha(alpha));
        Ok(())
    }
    pub fn set_fill_alpha(
        &mut self, 
        alpha:f32
    ) -> Result<()>
    {
        let entry = self.writer.entry();
        try!(entry.write_u8(VilogCommandKind::SetFillAlpha as u8));
        try!(entry.write_alpha(alpha));
        Ok(())
    }

    pub fn set_line_color(
        &mut self, 
        rgb:u32
    ) -> Result<()>
    {
        let entry = self.writer.entry();
        try!(entry.write_u8(VilogCommandKind::SetLineColor as u8));
        try!(entry.write_u24(rgb));
        Ok(())
    }
    pub fn set_fill_color(
        &mut self, 
        rgb:u32
    ) -> Result<()>
    {
        let entry = self.writer.entry();
        try!(entry.write_u8(VilogCommandKind::SetFillColor as u8));
        try!(entry.write_u24(rgb));
        Ok(())
    }

    pub fn set_line_thickness(
        &mut self, 
        thickness:f32
    ) -> Result<()>
    {
        let entry = self.writer.entry();
        try!(entry.write_u8(VilogCommandKind::SetLineThickness as u8));
        try!(entry.write_f32(thickness));
        Ok(())
    }
    pub fn set_blend_mode(
        &mut self, 
        blend_mode:BlendMode
    ) -> Result<()>
    {
        let entry = self.writer.entry();
        try!(entry.write_u8(VilogCommandKind::SetBlendMode as u8));
        try!(entry.write_u8(blend_mode as u8));
        Ok(())
    }
    pub fn set_line_alignment(
        &mut self, 
        alignment:f32
    ) -> Result<()>
    {
        let entry = self.writer.entry();
        try!(entry.write_u8(VilogCommandKind::SetLineAlignment as u8));
        try!(entry.write_f32(alignment));
        Ok(())
    }

    pub fn set_fonts<T: AsRef<str>>(
        &mut self, 
        font_names:&[T]
    ) -> Result<()>
    {
        let entry = self.writer.entry();
        try!(entry.write_u8(VilogCommandKind::SetFonts as u8));

        let len = font_names.len();
        if len > ::std::u16::MAX as usize {
            return Err(
                Error::new(
                    ErrorKind::InvalidInput,
                    format!("font_names has too long length: {}", len)
                )
            );
        }
        try!(entry.write_u16(len as u16));
        for name in font_names {
            try!(entry.write_short_str(name.as_ref()));
        }
        Ok(())
    }
    pub fn set_font_size(
        &mut self, 
        size:f32
    ) -> Result<()>
    {
        let entry = self.writer.entry();
        try!(entry.write_u8(VilogCommandKind::SetFontSize as u8));
        try!(entry.write_f32(size));
        Ok(())
    }
    pub fn set_font_style(
        &mut self, 
        bold:bool,
        italic:bool
    ) -> Result<()>
    {
        let entry = self.writer.entry();
        try!(entry.write_u8(VilogCommandKind::SetFontStyle as u8));
        try!(entry.write_u8(
            (if bold   { 1 } else { 0 }) |
            (if italic { 1 } else { 0 }) << 1
        ));
        Ok(())
    }
    pub fn set_text_horizontal_align(
        &mut self, 
        align:f32
    ) -> Result<()>
    {
        let entry = self.writer.entry();
        try!(entry.write_u8(VilogCommandKind::SetTextHorizontalAlign as u8));
        try!(entry.write_f32(align));
        Ok(())
    }
    pub fn set_text_vertical_align(
        &mut self, 
        align:f32
    ) -> Result<()>
    {
        let entry = self.writer.entry();
        try!(entry.write_u8(VilogCommandKind::SetTextVerticalAlign as u8));
        try!(entry.write_f32(align));
        Ok(())
    }
    pub fn draw_text(
        &mut self, 
        x:f32,
        y:f32,
        text:&str
    ) -> Result<()>
    {
        let entry = self.writer.entry();
        try!(entry.write_u8(VilogCommandKind::DrawText as u8));
        try!(entry.write_f32(x));
        try!(entry.write_f32(y));
        try!(entry.write_long_str(text));
        Ok(())
    }

    pub fn set_image_horizontal_align(
        &mut self, 
        align:f32
    ) -> Result<()>
    {
        let entry = self.writer.entry();
        try!(entry.write_u8(VilogCommandKind::SetImageHorizontalAlign as u8));
        try!(entry.write_f32(align));
        Ok(())
    }
    pub fn set_image_vertical_align(
        &mut self, 
        align:f32
    ) -> Result<()>
    {
        let entry = self.writer.entry();
        try!(entry.write_u8(VilogCommandKind::SetImageVerticalAlign as u8));
        try!(entry.write_f32(align));
        Ok(())
    }
    pub fn draw_image(
        &mut self, 
        x:f32,
        y:f32,
        path:&str
    ) -> Result<()>
    {
        let entry = self.writer.entry();
        try!(entry.write_u8(VilogCommandKind::DrawImage as u8));
        try!(entry.write_f32(x));
        try!(entry.write_f32(y));
        try!(entry.write_short_str(path));
        Ok(())
    }

    // ======================
    // MASK COMMAND
    // ======================
    pub fn start_masking_region(
        &mut self
    ) -> Result<()>
    {
        let entry = self.writer.entry();
        try!(entry.write_u8(VilogCommandKind::StartMaskingRegion as u8));
        Ok(())
    }
    pub fn start_masked_region(
        &mut self,
        mask_index_from_last:u16
    ) -> Result<()>
    {
        let entry = self.writer.entry();
        try!(entry.write_u8(VilogCommandKind::StartMaskedRegion as u8));
        try!(entry.write_u16(mask_index_from_last));
        Ok(())
    }
    pub fn start_mask_region(
        &mut self
    ) -> Result<()>
    {
        let entry = self.writer.entry();
        try!(entry.write_u8(VilogCommandKind::EndMaskRegion as u8));
        Ok(())
    }
}

impl LogLayerWriter for VilogLayerWriter {
    fn clear(&mut self) {
        self.writer.clear()
    }
}
