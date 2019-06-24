
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
use std::f32::consts::PI;

const WIDTH:u32 = 400;
const HEIGHT:u32 = 400;
const MARGIN:f32 = 5.;
const SIZE:usize = 25;

struct Point {
    x: f32,
    y: f32,
}

#[test]
fn gift_wrapping() {
    let mut points = Vec::<Point>::new();
    let mut hull = Vec::<usize>::new();

    let mut rng = IsaacRng::from_seed([2; 32]);
    
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

    let mut start = 0;
    for i in 1..SIZE {
        let start_point = &points[start];
        let point = &points[i];
        if 
            start_point.x > point.x || 
            start_point.x == point.x && start_point.y < point.y
        {
            start = i;
        }
    }

    let mut current = start;
    let mut current_direction = 0.0;
    let mut current_point = &points[current];

    for _ in 0..SIZE {
        writer.add_hull(&points[current]);
        hull.push(current);
        writer.step();

        let mut next = 0;
        let mut next_direction = current_direction + PI * 2.0;
        let mut next_distance = -1.0;

        for i in 0..SIZE {
            let point = &points[i];

            if current_point.x != point.x || current_point.y != point.y {
                let x = point.x - current_point.x;
                let y = point.y - current_point.y;
                let mut direction = x.atan2(-y);
                if direction < current_direction {
                    direction += PI * 2.0;
                }
                
                writer.clear_trace();
                writer.trace(&format!("compare: {}", i));
                writer.add_current_line(
                    current_point, 
                    point, 
                    current_direction + PI, 
                    direction
                );

                let distance = x * x + y * y;
                if 
                    direction < next_direction || 
                    direction == next_direction && distance > next_distance
                {
                    next = i;
                    next_direction = direction;
                    next_distance = distance;
                    
                    writer.clear_trace();
                    writer.trace(&format!("select: {}", i));
                    writer.add_next_line(current_point, point);
                }
            }
        }

        current = next;
        current_point = &points[current];
        current_direction = next_direction;

        writer.next_layer().clear();
        writer.current_layer().clear();
        writer.add_line(current_point);
        if current == start { break; }
    }
    writer.clear_trace();
    writer.trace(&format!("finish"));
    writer.finish();
}

const BACKGROUND_LAYER:i32 = -1;
const CURRNT_LAYER:i32 = 0;
const NEXT_LAYER:i32 = 1;

const TRACE_LAYER:i32 = 0;

struct Writer {
    trlog:TrlogWriter<BufWriter<File>>,
    vilog:VilogWriter<BufWriter<File>>,
}

impl Writer {
    fn new() -> Writer {
        Writer {
            trlog: TrlogWriter::new(
                BufWriter::new(File::create("../../sample/geom/gift_wrapping.trlog").unwrap()),
                TrlogConfig::new()
            ),
            vilog: VilogWriter::new(
                BufWriter::new(File::create("../../sample/geom/gift_wrapping.vilog").unwrap()),
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
    fn next_layer(&mut self)  -> &mut VilogLayerWriter {
        self.vilog.layer(NEXT_LAYER)
    }
    fn current_layer(&mut self)  -> &mut VilogLayerWriter {
        self.vilog.layer(CURRNT_LAYER)
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
    
    fn add_hull(&mut self, point:&Point) {
        let layer = self.background();
        layer.set_line_alpha(1.).unwrap();
        layer.set_fill_alpha(0.).unwrap();
        layer.set_line_color(0xFF0000).unwrap();
        layer.draw_circle(point.x, point.y, 4.).unwrap();
        layer.move_to(point.x, point.y).unwrap();
    }
    fn add_line(&mut self, point:&Point) {
        let layer = self.background();
        layer.set_line_alpha(1.).unwrap();
        layer.set_fill_alpha(0.).unwrap();
        layer.set_line_color(0xFF0000).unwrap();
        layer.line_to(point.x, point.y).unwrap();
    }
    fn add_next_line(&mut self, point:&Point, current_point:&Point) {
        {
            let layer = self.next_layer();
            layer.clear();
            layer.set_line_alpha(1.).unwrap();
            layer.set_fill_alpha(0.).unwrap();
            layer.set_line_color(0xFFCCCC).unwrap();

            layer.move_to(point.x, point.y).unwrap();
            layer.line_to(current_point.x, current_point.y).unwrap();
        }
        self.step();
    }
    fn add_current_line(
        &mut self, 
        point:&Point, 
        current_point:&Point, 
        direction:f32,
        current_direction:f32
    ) {
        {
            let layer = self.current_layer();
            layer.clear();
            layer.set_line_alpha(1.).unwrap();
            layer.set_fill_alpha(0.).unwrap();
            layer.set_line_color(0x0000FF).unwrap();
            layer.move_to(point.x, point.y).unwrap();
            layer.arrow_to(current_point.x, current_point.y, false, true, true, 10., 6.).unwrap();
            
            layer.move_to(point.x, point.y).unwrap();
            layer.line_to(
                point.x + direction.sin() * 30., 
                point.y - direction.cos() * 30.
            ).unwrap();
            layer.arc_arrow(
                point.x, point.y,
                15.,
                direction - PI * 0.5,
                current_direction - PI * 0.5,
                false,
                false,
                true,
                true,
                4.,
                3.
            ).unwrap();
        }
        self.step();
    }
}
