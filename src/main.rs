use ggez::{*, graphics::MeshBuilder};
use glam::*;

use rand::Rng;
use svg::Document;
use svg::node::element::Path;
use svg::node::element::path::Data;

mod utils;
use crate::utils::*;

mod font;
use crate::font::*;

struct Shape {
    points:Vec<Vec2>,
}

impl Shape {
    fn draw_mesh(&self, mb: &mut MeshBuilder){
        mb.polygon(graphics::DrawMode::Stroke(graphics::StrokeOptions::default().with_line_width(2_f32)), &self.points, graphics::Color::BLACK).unwrap();
    }
}

struct Application {
    size: Vec2,
    shapes:Vec<Shape>,
    scale: f32,

    instructions: Vec<Instruction>,

    is_mouse_down: bool,
    is_print_down: bool,
}

impl Application {
    fn new(size: Vec2, scale: f32) -> Application{
        Application {
            size,
            shapes: Vec::new(),
            is_mouse_down: false,
            is_print_down: false,
            instructions: Vec::new(),
            scale,
        }
    }

    fn add_hexagone(&mut self, position : Vec2, scale : f32) {
        let mut points = Vec::new();
        let scale = scale * self.scale;
        let o = 0_f32;
        let i = 1_f32 * scale;
        let si = std::f32::consts::FRAC_PI_6.sin() * scale;
        let co = std::f32::consts::FRAC_PI_6.cos() * scale;
        points.push(position + Vec2::new(o, i));
        points.push(position + Vec2::new(-co, si));
        points.push(position + Vec2::new(-co, -si));
        points.push(position + Vec2::new(o, -i));
        points.push(position + Vec2::new(co, -si));
        points.push(position + Vec2::new(co, si));
        self.shapes.push(Shape{points : points});
    }

    fn clear(&mut self) {
        self.shapes.clear();
    }

    fn print_shapes(&self) {
        if self.shapes.len() == 0 {
            return
        }

        let mut drawn_points = std::collections::HashSet::new();
        let mut data = Data::new();

        let mut shapes_to_draw : Vec<usize> = (0..self.shapes.len()).collect();
        let mut next_shape_index = (shapes_to_draw.len() - 1) / 2;
        let mut start_vertex = 0 as usize;
        let mut delta = 0;

        data = data.move_to((self.shapes[0].points[0] / self.scale).from());
        let mut current_position = Vec2::default();
        while !shapes_to_draw.is_empty() {
            let shape_index = shapes_to_draw[next_shape_index];
            shapes_to_draw.remove(next_shape_index);
            let shape = &self.shapes[shape_index];
            let mut current_vertex = shape.points[start_vertex];
            if current_vertex != current_position {
                data = data.move_to((current_vertex / self.scale).from());
            }

            for vert_index in 1..(shape.points.len() + 1) {
                let next_vertex = shape.points[(vert_index + start_vertex) % shape.points.len()];
                let pair = OrderedPair::new(current_vertex, next_vertex);
                let next_point = (next_vertex / self.scale).from();
                if !drawn_points.contains(&pair) {
                    drawn_points.insert(pair);
                    data = data.line_to(next_point);
                }
                else {
                    data = data.move_to(next_point);
                }

                current_vertex = next_vertex;
            }

            if shapes_to_draw.is_empty() {
                break;
            }

            let mut found = false;
            for other_index in 0..shapes_to_draw.len() {
                let other_shape = &self.shapes[shapes_to_draw[other_index]];
                for other_vertex_index in 0..other_shape.points.len() {
                    let other_vertex = other_shape.points[other_vertex_index];
                    if other_vertex == current_vertex {
                        start_vertex = other_vertex_index;
                        next_shape_index = other_index;
                        found = true;
                        break;
                    }
                }

                if found {
                    break;
                }
            }

            if !found {
                next_shape_index = (shapes_to_draw.len() - 1) / 2;
                start_vertex = delta % self.shapes[shapes_to_draw[next_shape_index]].points.len();
                delta = delta + 1;
            }

            current_position = current_vertex;
        }

        let path = Path::new()
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", 0.4)
            .set("d", data);

        let document = Document::new()
            .set("viewBox", (0, 0, self.size.x, self.size.y))
            .set("width", format!("{}mm",self.size.x))
            .set("height", format!("{}mm",self.size.y))
            .set("units", "mm")
            .add(path);

        svg::save("image.svg", &document).unwrap();
    }
}

impl Application
{
    fn hex_grid(&mut self) {
        let hex_scale= 4_f32;
        let grid_size = Vec2::new(20_f32, 4_f32);
        let child_chance = 0.3;
        let decay = 0.7;

        let hex_half_width = std::f32::consts::FRAC_PI_6.cos() * hex_scale * self.scale;
        let hex_width = hex_half_width * 2_f32;
        let hex_height = std::f32::consts::FRAC_PI_6.sin() * hex_scale * 3_f32 * self.scale;
        let total_size = grid_size * Vec2::new(hex_width,hex_height);
        let base_position = self.size / 2_f32 * self.scale - total_size / 2_f32 + Vec2::new(hex_half_width * 0.5, hex_height * 0.25);
        
        let mut random = rand::thread_rng();

        for x in 0..(grid_size.x as usize) {
            for y in 0..(grid_size.y as usize) {
                let fx = x as f32 * hex_width + (y % 2) as f32 * hex_half_width;
                let fy = y as f32 * hex_height;
                let mut child_scale = 1_f32;
                let mut is_child = true;
                while is_child {
                    is_child = random.gen_range(0_f32..1_f32) > (1_f32 - child_chance);
                    self.add_hexagone(Vec2::new(fx, fy) + base_position, hex_scale * child_scale);
                    child_scale = child_scale * decay;
                }
            }
        }
    }
}

impl ggez::event::EventHandler<GameError> for Application {
    fn update(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        
        let mouse_position = input::mouse::position(ctx);
        let mouse_position = Vec2::new(mouse_position.x, mouse_position.y);
        
        let was_pressed = self.is_mouse_down;
        self.is_mouse_down = input::mouse::button_pressed(ctx, event::MouseButton::Left);
        if was_pressed != self.is_mouse_down {
            if self.is_mouse_down {
                self.clear();
                self.hex_grid();
            }
        }

        let was_down = self.is_print_down;
        self.is_print_down = input::keyboard::is_key_pressed(ctx, input::keyboard::KeyCode::P);
        if was_down != self.is_print_down && self.is_print_down
        {
            self.print_shapes();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        graphics::clear(ctx, graphics::Color::WHITE);

        let mb = &mut graphics::MeshBuilder::new();

        if self.shapes.len() > 0 {
            for shape in &self.shapes {
                shape.draw_mesh(mb);
            }
            
            let mesh = mb.build(ctx)?;
            match graphics::draw(ctx, &mesh, graphics::DrawParam::new()) {
                Ok(_) => (),
                Err(e) => println!("ERROR : {:#?}", e)
            }
        }

        graphics::present(ctx)
    }
}

fn main() {
    let scale = 5_f32;
    let mut c = conf::Conf::new();
    c.window_mode.width = 150_f32 * scale;
    c.window_mode.height = 100_f32 * scale;

    let font = Font::load("Media/HersheySans1.svgfont");

    let mut application = Application::new(Vec2::new(150_f32,100_f32), scale);
    application.hex_grid();

    font.print_in_instructions("Hello World *# 0123", &Vec2::new(32_f32, 32_f32), &2.0, &mut application.instructions);

    let (ctx, event_loop) = ContextBuilder::new("SVG Experiment", "AntonMakesGames")
    .default_conf(c)
    .window_setup(conf::WindowSetup{
        title:String::from("SVG Generator"),
        samples: conf::NumSamples::One,
        vsync: true,
        srgb:true,
        icon:"".to_owned(),
    })
    .build()
    .unwrap();

    event::run(ctx, event_loop, application);
}
