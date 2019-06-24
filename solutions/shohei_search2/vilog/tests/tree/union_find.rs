
use visual_log::trlog::write::*;
use visual_log::vilog::write::*;
use visual_log::trlog::config::TrlogConfig;
use visual_log::vilog::config::VilogConfig;
use visual_log::log::write::LogWriter;
use visual_log::log::write::LogLayerWriter;

use std::fs::File;
use std::io::BufWriter;

use rand::isaac::IsaacRng;
use rand::Rng;
use rand::SeedableRng;
use std::mem::swap;

const WIDTH:u16 = 10;
const HEIGHT:u16 = 10;
const SCALE:f32 = 40.0;
const SIZE:usize = (WIDTH * HEIGHT) as usize;

#[test]
pub fn union_find() {
    let mut writer = Writer::new();

    let mut noise = vec![false; SIZE];
    let mut rng = IsaacRng::from_seed([0; 32]);

    for i in 0..SIZE {
        if rng.gen_range(0, 2) == 0 {
            noise[i] = true;
        }
    }
    writer.draw_background(&noise);

    let mut tree = vec![-1; SIZE];
    writer.draw_tree(&tree);
    for i in 0..SIZE {
        let x = i % WIDTH as usize;
        let y = i / WIDTH as usize;
        writer.trace(&format!("target: ({}, {})", x, y));
        writer.draw_cursor(x, y);

        if y > 0 {
            let j = x + (y - 1) * WIDTH as usize;
            if noise[j] == noise[i] {
                unite(&mut tree, j, i, &mut writer);
            }
        }
        if x > 0 {
            let j = x - 1 + y * WIDTH as usize;
            if noise[j] == noise[i] {
                unite(&mut tree, j, i, &mut writer);
            }
        }
        writer.clear_trace();
    }

    writer.cursor().clear();
    writer.finish();
}

pub fn unite(tree:&mut Vec<i32>, a:usize, b:usize, writer:&mut Writer) {
    let mut rb = root(tree, b, false, writer);
    let mut ra = root(tree, a, true , writer);
    writer.target(true).clear();
    writer.target(false).clear();
    
    if ra != rb {
        let rank_a = tree[ra];
        let rank_b = tree[rb];
        if rank_a > rank_b {
            swap(&mut ra, &mut rb);
        } else if rank_a == rank_b {
            tree[ra] -= 1;
        }
        if tree[rb] != ra as i32 {
            tree[rb] = ra as i32;
            writer.draw_tree(tree);
        }
    }

    let ax = a % WIDTH as usize;
    let ay = a / WIDTH as usize;
    let bx = b % WIDTH as usize;
    let by = b / WIDTH as usize;
    writer.trace(&format!("unite: ({}, {}) to ({}, {})", bx, by, ax, ay));
    writer.step();
}
pub fn root(tree:&mut Vec<i32>, a:usize, is_target:bool, writer:&mut Writer) -> usize {
    let b = tree[a];
    
    let ax = a % WIDTH as usize;
    let ay = a / WIDTH as usize;
    writer.draw_target(ax, ay, is_target);

    if b < 0 { 
        a 
    } else { 
        let value = root(tree, b as usize, is_target, writer) as i32;
        if tree[a] != value {
            tree[a] = value;
            writer.draw_tree(tree);
            writer.trace(&format!("compress path: ({}, {})", ax, ay));
            writer.step();
        }
        value as usize
    }
}

const BACKGROUND_LAYER:i32 = -1;
const CONTENT_LAYER:i32 = 0;
const TEXT_LAYER:i32 = 1;
const CURSOR_LAYER:i32 = 2;
const TARGET_LAYER1:i32 = 3;
const TARGET_LAYER2:i32 = 4;

const TRACE_LAYER:i32 = 0;

pub struct Writer {
    pub trlog:TrlogWriter<BufWriter<File>>,
    pub vilog:VilogWriter<BufWriter<File>>,
}

impl Writer {
    pub fn new() -> Writer {
        Writer {
            trlog: TrlogWriter::new(
                BufWriter::new(File::create("../../sample/ds/union_find.trlog").unwrap()),
                TrlogConfig::new()
            ),
            vilog: VilogWriter::new(
                BufWriter::new(File::create("../../sample/ds/union_find.vilog").unwrap()),
                VilogConfig::new(
                    (WIDTH as f32 * SCALE) as u32, 
                    (HEIGHT as f32 * SCALE) as u32
                )
            ),
        }
    }

    fn background(&mut self)  -> &mut VilogLayerWriter {
        self.vilog.layer(BACKGROUND_LAYER)
    }
    fn content(&mut self)  -> &mut VilogLayerWriter {
        self.vilog.layer(CONTENT_LAYER)
    }
    fn cursor(&mut self)  -> &mut VilogLayerWriter {
        self.vilog.layer(CURSOR_LAYER)
    }
    fn target(&mut self, is_target:bool)  -> &mut VilogLayerWriter {
        self.vilog.layer(if is_target { TARGET_LAYER1 } else { TARGET_LAYER2 })
    }
    fn text(&mut self)  -> &mut VilogLayerWriter {
        self.vilog.layer(TEXT_LAYER)
    }

    pub fn step(&mut self) {
        self.trlog.step().unwrap();
        self.vilog.step().unwrap();
    }

    pub fn finish(&mut self) {
        self.trlog.finish().unwrap();
        self.vilog.finish().unwrap();
    }

    pub fn clear_trace(&mut self) {
        self.trlog.layer(TRACE_LAYER).clear();
    }
    pub fn trace(&mut self, string:&str) {
        self.trlog.layer(TRACE_LAYER).println(string).unwrap();
    }
    pub fn draw_background(&mut self, noise:&[bool]) {
        let background = self.background();
        background.reset_transform(SCALE, 0., 0., SCALE, 0., 0.).unwrap();
        background.set_line_alpha(0.).unwrap();
        background.set_fill_alpha(1.).unwrap();
        background.set_line_color(0xBBBBBB).unwrap();

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if noise[x as usize + y as usize * WIDTH as usize] {
                    background.set_fill_color(0x11EE44).unwrap();
                } else {
                    background.set_fill_color(0x4455EE).unwrap();
                }
                background.draw_rounded_rectangle(x as f32, y as f32, 1., 1., 0.3).unwrap();
            }
        }
    
        background.set_line_alpha(1.).unwrap();
        background.set_fill_alpha(0.).unwrap();
        background.set_line_thickness(0.03).unwrap();
        background.draw_grid(0., 0., WIDTH, HEIGHT, 1., 1.).unwrap();
    }

    pub fn draw_tree(&mut self, tree:&[i32]) {
        {
            let content = self.content();
            content.clear();
            content.reset_transform(SCALE, 0., 0., SCALE, SCALE / 2., SCALE / 2.).unwrap();
            content.set_line_thickness(0.04).unwrap();
            content.set_fill_alpha(0.).unwrap();
            content.set_line_color(0xFFFFFF).unwrap();

            for y in 0..HEIGHT {
                for x in 0..WIDTH {
                    let src = x as usize + y as usize * WIDTH as usize;
                    let dst = tree[src];
                    if dst >= 0 && src != dst as usize {
                        let dst = dst as usize;
                        let ax = x as f32;
                        let ay = y as f32;
                        let bx = (dst % WIDTH as usize) as f32;
                        let by = (dst / WIDTH as usize) as f32;

                        content.set_fill_alpha(0.).unwrap();
                        content.move_to(ax, ay).unwrap();

                        let diff_x = bx - ax;
                        let diff_y = by - ay;
                        let ctrl_x = (ax + bx) / 2. - diff_y * 0.2;
                        let ctrl_y = (ay + by) / 2. + diff_x * 0.2;
                        content.curve_arrow_to(ctrl_x, ctrl_y, bx, by, false, true, true, 0.3, 0.2).unwrap();
                    }
                }
            }
        }
        {
            let text = self.text();
            text.clear();
            text.reset_transform(SCALE, 0., 0., SCALE, SCALE / 3., SCALE / 3.).unwrap();
            text.set_fill_alpha(1.).unwrap();
            text.set_font_size(0.5).unwrap();
            text.set_fill_color(0xFF99FF).unwrap();
            text.set_text_horizontal_align(0.5).unwrap();
            text.set_text_vertical_align(0.5).unwrap();
            
            for y in 0..HEIGHT {
                for x in 0..WIDTH {
                    let src = x as usize + y as usize * WIDTH as usize;
                    let dst = tree[src];
                    if dst < 0 {
                        text.draw_text(x as f32, y as f32, &(-dst).to_string()).unwrap();
                    }
                }
            }
        }
    }
    
    pub fn draw_cursor(&mut self, x:usize, y:usize) {
        {
            let cursor = self.cursor();
            cursor.clear();
            cursor.reset_transform(SCALE, 0., 0., SCALE, SCALE / 2., SCALE / 2.).unwrap();
            cursor.set_line_thickness(0.08).unwrap();
            cursor.set_line_alpha(1.).unwrap();
            cursor.set_fill_alpha(0.).unwrap();
            cursor.set_line_color(0xFFFF33).unwrap();
            cursor.draw_regular_polygon(
                x as f32, y as f32, 
                5, 
                0.15,
                0.0
            ).unwrap();
        }
        self.step();
    }
    pub fn draw_target(&mut self, x:usize, y:usize, is_target:bool) {
        {
            let target = self.target(is_target);
            target.clear();
            target.reset_transform(SCALE, 0., 0., SCALE, SCALE / 2., SCALE / 2.).unwrap();
            target.set_line_thickness(0.08).unwrap();
            target.set_line_alpha(1.).unwrap();
            target.set_fill_alpha(0.).unwrap();
            target.set_line_color(if is_target { 0xFF33FF } else { 0xFFFF33 }).unwrap();
            target.draw_star(
                x as f32, y as f32, 
                5, 
                0.3, 0.15,
                0.0
            ).unwrap();
        }
        self.step();
    }
}
