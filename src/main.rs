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

mod signature;
use crate::signature::*;

struct TileInfo {
    position : Vec2,
    neighbors : Vec<usize>,
    vertices : Vec<Vec2>,
}

impl TileInfo {
    fn new(position : Vec2) -> TileInfo {
        TileInfo {
            position,
            neighbors : Vec::new(),
            vertices : Vec::new(),
        }
    }
}

struct Grid {
    tiles : Vec<TileInfo>,
}

impl Grid {
    fn hex_grid_size(col : usize, row : usize, tile_scale : f32) -> Vec2{
        Vec2::new(col as f32 * tile_scale * 2_f32 * std::f32::consts::FRAC_PI_6.cos(), row as f32 * tile_scale * 3_f32 * std::f32::consts::FRAC_PI_6.sin())   
    }

    fn hex_grid(col : usize, row : usize, tile_scale : f32, base_position : Vec2) -> Grid {
        let mut grid = Grid {
            tiles: Vec::new()
        };

        let max_x = col - 1;
        let max_y = row - 1;

        let o = 0_f32;
        let i = 1_f32 * tile_scale;
        let si = std::f32::consts::FRAC_PI_6.sin() * tile_scale;
        let co = std::f32::consts::FRAC_PI_6.cos() * tile_scale;

        let tile_width = co * 2_f32;
        let tile_height = si * 3_f32;

        for y in 0..row {
            for x in 0..col {
                let mut position = base_position + Vec2::new(x as f32 * tile_width, y as f32 * tile_height);
                if y % 2 == 1 {
                    position.x = position.x + co;
                }

                let mut tile = TileInfo::new(position);
                let tile_index = grid.tiles.len();

                if x > 0 {
                    tile.neighbors.push(tile_index - 1);
                }
                
                if x < max_x {
                    tile.neighbors.push(tile_index + 1);
                }

                if y % 2 == 0 {
                    if y > 0 {
                        tile.neighbors.push(tile_index - col);

                        if x > 0 {
                            tile.neighbors.push(tile_index - col - 1);
                        }
                    }

                    if y < max_y {
                        tile.neighbors.push(tile_index + col);

                        if x > 0 {
                            tile.neighbors.push(tile_index + col - 1);
                        }
                    }
                }
                else {
                    if y > 0 {
                        tile.neighbors.push(tile_index - col);

                        if x < max_x {
                            tile.neighbors.push(tile_index - col + 1);
                        }
                    }

                    if y < max_y {
                        tile.neighbors.push(tile_index + col);

                        if x < max_x {
                            tile.neighbors.push(tile_index + col + 1);
                        }
                    }
                }
                
                tile.vertices.clear();
                tile.vertices.push(position + Vec2::new(o, i));
                tile.vertices.push(position + Vec2::new(-co, si));
                tile.vertices.push(position + Vec2::new(-co, -si));
                tile.vertices.push(position + Vec2::new(o, -i));
                tile.vertices.push(position + Vec2::new(co, -si));
                tile.vertices.push(position + Vec2::new(co, si));
                
                grid.tiles.push(tile);
            }
        }

        return grid;
    }
}

struct Application {
    grid: Grid,
    scale: f32,
    size: Vec2,
    font : Font,

    instructions: Vec<Instruction>,

    is_mouse_down: bool,
    is_print_down: bool,
}

impl Application {
    fn new(grid: Grid, scale: f32, size: Vec2, font : Font) -> Application{
        Application {
            grid,
            is_mouse_down: false,
            is_print_down: false,
            instructions: Vec::new(),
            scale,
            font,
            size,
        }
    }

    fn print_to_svg(&self) {
        let mut drawn_points = std::collections::HashSet::new();
        let mut data = Data::new();

        {
            let mut shapes_to_draw : Vec<usize> = (0..self.grid.tiles.len()).collect();
            let mut next_shape_index = (shapes_to_draw.len() - 1) / 2;
            let mut start_vertex = 0 as usize;
            let mut delta = 0;
            
            data = data.move_to((self.grid.tiles[0].vertices[0] / self.scale).from());
            let mut current_position = Vec2::default();
            while !shapes_to_draw.is_empty() {
                let shape_index = shapes_to_draw[next_shape_index];
                shapes_to_draw.remove(next_shape_index);
                let tile = &self.grid.tiles[shape_index];
                let mut current_vertex = tile.vertices[start_vertex];
                if current_vertex != current_position {
                    data = data.move_to((current_vertex / self.scale).from());
                }

                for vert_index in 1..(tile.vertices.len() + 1) {
                    let next_vertex = tile.vertices[(vert_index + start_vertex) % tile.vertices.len()];
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
                    let other_tile = &self.grid.tiles[shapes_to_draw[other_index]];
                    for other_vertex_index in 0..other_tile.vertices.len() {
                        let other_vertex = other_tile.vertices[other_vertex_index];
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
                    start_vertex = delta % self.grid.tiles[shapes_to_draw[next_shape_index]].vertices.len();
                    delta = delta + 1;
                }
                
                current_position = current_vertex;
            }
        }
        let mut current_position = Vec2::new(0_f32, 0_f32);
        for instruction in &self.instructions {
            match instruction {
                Instruction::MoveTo(pos) => {
                    data = data.move_to((*pos / self.scale).from());
                    current_position = *pos;
                },
                Instruction::LineTo(pos) => {
                    
                    let pair = OrderedPair::new(current_position, *pos);
                    if !drawn_points.contains(&pair) {
                        drawn_points.insert(pair);
                        data = data.line_to((*pos / self.scale).from());
                    }

                    current_position = *pos;
                }
            }
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

        let export_name = format!("Exports/AMG_{}.svg",get_signature_counter());

        match svg::save(export_name, &document) {
            Ok(_) => {
                match increment_signature_counter() {
                    Err(e) => panic!("{}", e),
                    _=>()
                }
            }
            
            Err(e)=> panic!("{}", e),
        }
    }
}

impl Application
{
    fn fill_mesh_builder(instructions : &Vec<Instruction>, mesh_builder : &mut MeshBuilder)
    {
        let line_width = 2_f32;
        let mut vertices = Vec::new();

        for instruction in instructions {
            match instruction {
                Instruction::LineTo(pos) => {
                    vertices.push(pos.to_owned());
                },
                Instruction::MoveTo(pos) => {
                    if vertices.len() > 0 {
                        let pts = vertices.to_owned().into_iter().map(|p| mint::Point2{x: p.x, y: p.y}).collect::<Vec<mint::Point2<f32>>>();
                        mesh_builder.line(&pts, line_width, graphics::Color::BLACK).unwrap();
                    }

                    vertices.clear();
                    vertices.push(pos.to_owned());
                }
            }
        }
        
        if vertices.len() > 0 {
            let pts = vertices.to_owned().into_iter().map(|p| mint::Point2{x: p.x, y: p.y}).collect::<Vec<mint::Point2<f32>>>();
            mesh_builder.line(&pts, line_width, graphics::Color::BLACK).unwrap();
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
                
            }
        }

        let was_down = self.is_print_down;
        self.is_print_down = input::keyboard::is_key_pressed(ctx, input::keyboard::KeyCode::P);
        if was_down != self.is_print_down && self.is_print_down
        {
            self.print_to_svg();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        graphics::clear(ctx, graphics::Color::WHITE);

        let mb = &mut graphics::MeshBuilder::new();
        
        Application::fill_mesh_builder(&self.instructions, mb);

        for tile in &self.grid.tiles {
            mb.polygon(graphics::DrawMode::Stroke(graphics::StrokeOptions::default().with_line_width(2_f32)), &tile.vertices, graphics::Color::BLACK).unwrap();
        
            for neighbor in &tile.neighbors {
                mb.line(&[tile.position, self.grid.tiles[*neighbor].position], 3_f32, graphics::Color::BLACK).unwrap();
            }
        }

        let mesh = mb.build(ctx)?;
        match graphics::draw(ctx, &mesh, graphics::DrawParam::new()) {
            Ok(_) => (),
            Err(e) => println!("ERROR : {:#?}", e)
        }

        graphics::present(ctx)
    }
}

fn main() {
    let scale = 5_f32;
    let mut c = conf::Conf::new();
    let width = 150_f32;
    let height = 100_f32;
    c.window_mode.width = width * scale;
    c.window_mode.height = height * scale;

    let font = Font::load("Medias/HersheySans1.svgfont");
    
    let col = 40;
    let row = 40;
    let tile_scale = 8_f32;
    let grid_size = Grid::hex_grid_size(col, row, tile_scale);
    let grid = Grid::hex_grid(col, row, tile_scale, Vec2::new((width * scale - grid_size.x) / 2_f32, (height * scale - grid_size.y) / 2_f32));
    let mut application = Application::new(grid, scale, Vec2::new(width, height), font);
    
    let signature = get_signature();
    let signature_height = 9.0_f32;
    let signature_width = application.font.get_width(signature, signature_height);

    application.font.print_in_instructions(get_signature(), Vec2::new(width * scale - signature_width- 3_f32, height* scale - 3_f32), signature_height, &mut application.instructions);

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
