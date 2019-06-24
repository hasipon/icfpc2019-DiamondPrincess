
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

const WIDTH:u32 = 700;
const HEIGHT:u32 = 700;
const MARGIN:f32 = 5.;
const SIZE:usize = 30;

#[derive(PartialEq, PartialOrd)]
struct Point {
    x: f32,
    y: f32,
}

#[test]
fn monotone_chain() {
    let mut points = Vec::<Point>::new();
    let mut rng = IsaacRng::from_seed([4; 32]);
    
    for _ in 0..SIZE {
        points.push(
            Point{
                x: (rng.gen_range(MARGIN, WIDTH  as f32 - MARGIN) + rng.gen_range(MARGIN, WIDTH  as f32 - MARGIN)) / 2.,
                y: (rng.gen_range(MARGIN, HEIGHT as f32 - MARGIN) + rng.gen_range(MARGIN, HEIGHT as f32 - MARGIN)) / 2.,
            }
        );
    }

    let mut writer = Writer::new();
    writer.draw_background(&points);
    writer.step();
    
    points.sort_by(|a, b| a.partial_cmp(b).unwrap());
    writer.trace("sort");
    writer.draw_background_numbers(&points);
    writer.step();
    
    let mut upper = Vec::new();
    for i in 0..points.len() {
        writer.clear_trace();
        writer.trace(&format!("target: {}", i));
        let mut len = upper.len();
        while len >= 2 {
            writer.trace(&format!("compare: {}->{}, {}->{}", i, upper[len - 1], i, upper[len - 2]));
            writer.draw_arrow(&points[upper[len - 2]], &points[upper[len - 1]], &points[i]);
            if cross(&points[upper[len - 2]], &points[upper[len - 1]], &points[i]) <= 0.0 {
                writer.trace(&format!("pop"));
                upper.pop();
                writer.draw_hull(&points, &upper, true);
                len -= 1;
            } else {
                break;
            }
        }
        upper.push(i);
        writer.trace(&format!("push: {}", i));
        writer.draw_hull(&points, &upper, true);
    }
    let mut lower = Vec::new();
    for i in (0..points.len()).rev() {
        writer.clear_trace();
        writer.trace(&format!("target: {}", i));
        let mut len = lower.len();
        while len >= 2 {
            writer.trace(&format!("compare: {}->{}, {}->{}", i, lower[len - 1], i, lower[len - 2]));
            writer.draw_arrow(&points[lower[len - 2]], &points[lower[len - 1]], &points[i]);
            if cross(&points[lower[len - 2]], &points[lower[len - 1]], &points[i]) <= 0.0 {
                writer.trace(&format!("pop"));
                lower.pop();
                writer.draw_hull(&points, &lower, false);
                len -= 1;
            } else {
                break;
            }
        }
        lower.push(i);
        writer.trace(&format!("push: {}", i));
        writer.draw_hull(&points, &lower, false);
    }
    
    writer.clear_trace();
    writer.trace(&format!("finish"));
    writer.finish();
}

fn cross(a:&Point, b:&Point, o:&Point) -> f32 {
   (a.x - o.x) * (b.y - o.y) - (a.y - o.y) * (b.x - o.x)
}

const BACKGROUND_LAYER:i32 = -1;
const HULL_LAYER1:i32 = 0;
const HULL_LAYER2:i32 = 1;
const ARROW_LAYER:i32 = 2;

const TRACE_LAYER:i32 = 0;

struct Writer {
    trlog:TrlogWriter<BufWriter<File>>,
    vilog:VilogWriter<BufWriter<File>>,
}

impl Writer {
    fn new() -> Writer {
        Writer {
            trlog: TrlogWriter::new(
                BufWriter::new(File::create("../../sample/geom/monotone_chain.trlog").unwrap()),
                TrlogConfig::new()
            ),
            vilog: VilogWriter::new(
                BufWriter::new(File::create("../../sample/geom/monotone_chain.vilog").unwrap()),
                VilogConfig::new(
                    WIDTH,
                    HEIGHT
                )
            ),
        }
    }

    fn background(&mut self)  -> &mut VilogLayerWriter {
        self.vilog.layer(BACKGROUND_LAYER)
    }
    fn hull(&mut self, upper:bool)  -> &mut VilogLayerWriter {
        self.vilog.layer(if upper { HULL_LAYER1 } else { HULL_LAYER2 })
    }
    fn arrow(&mut self)  -> &mut VilogLayerWriter {
        self.vilog.layer(ARROW_LAYER)
    }

    fn step(&mut self) {
        self.trlog.step().unwrap();
        self.vilog.step().unwrap();
    }
    fn finish(&mut self) {
        self.trlog.finish().unwrap();
        self.vilog.finish().unwrap();
    }
    fn clear_trace(&mut self) {
        self.trlog.layer(TRACE_LAYER).clear();
    }
    fn trace(&mut self, string:&str) {
        self.trlog.layer(TRACE_LAYER).println(string).unwrap();
    }
    fn draw_background(&mut self, points:&[Point]) {
        let background = self.background();
        background.set_line_alpha(0.).unwrap();
        background.set_fill_alpha(1.).unwrap();
        background.set_fill_color(0x000000).unwrap();
        for point in points {
            background.draw_circle(point.x, point.y, 1.5).unwrap();
        }
    }
    fn draw_background_numbers(&mut self, points:&[Point]) {
        let background = self.background();
        background.set_font_size(11.).unwrap();

        let mut index = 0;
        for point in points {
            background.draw_text(point.x, point.y, &index.to_string()).unwrap();
            index += 1;
        }
    }

    fn draw_hull(&mut self, points:&[Point], indexes:&[usize], upper:bool) {
        {
            let layer = self.arrow();
            layer.clear();
        }
        {
            let layer = self.hull(upper);
            layer.clear();
            layer.set_line_alpha(1.).unwrap();
            layer.set_fill_alpha(0.).unwrap();
            layer.set_line_color(if upper { 0xFF0000 } else { 0xFF6600 }).unwrap();
            
            let start = &points[indexes[0]];
            layer.move_to(start.x, start.y).unwrap();
            
            for i in 1..indexes.len() {
                let point = &points[indexes[i]];
                layer.line_to(point.x, point.y).unwrap();
            }
            for i in 0..indexes.len() - 1 {
                let point = &points[indexes[i]];
                layer.draw_circle(point.x, point.y, 3.).unwrap();
            }
        }
        self.step();
    }
    
    fn draw_arrow(&mut self, a:&Point, b:&Point, o:&Point) {
        {
            let layer = self.arrow();
            layer.clear();
            layer.set_line_alpha(1.).unwrap();
            layer.set_fill_alpha(0.).unwrap();
            layer.set_line_color(0xFFBB99).unwrap();
            layer.move_to(o.x, o.y).unwrap();
            layer.arrow_to(b.x, b.y, false, true, true, 10., 6.).unwrap();
            
            layer.set_line_color(0x7799FF).unwrap();
            layer.move_to(o.x, o.y).unwrap();
            layer.arrow_to(a.x, a.y, false, true, true, 10., 6.).unwrap();
        }
        self.step();
    }
}
