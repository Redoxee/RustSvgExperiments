use ggez::*;
use glam::*;

mod utils;

mod font;
use crate::font::*;

mod signature;

mod grid;
use crate::grid::*;

mod application;
use crate::application::*;

fn main() {
    let scale = 5_f32;
    let mut c = conf::Conf::new();
    let width = 150_f32;
    let height = 100_f32;
    c.window_mode.width = width * scale;
    c.window_mode.height = height * scale;

    let font = Font::load("Medias/HersheySans1.svgfont");
    
    let col = 10;
    let row = 10;
    let tile_scale = 12_f32;
    let grid_size = Grid::hex_grid_size(col, row, tile_scale);
    let grid = Grid::hex_grid(col, row, tile_scale, Vec2::new((width * scale - grid_size.x) / 2_f32, (height * scale - grid_size.y) / 2_f32));

    let parameters = ApplicationParameters{
        animate_instructions: false,
        display_grid: false,
        print_grid: false,
        walk_parameters: RandomWalkParameters {
            slice_percentage: 0.5_f32,
            smooth_number_of_points: 4,
            smooth_sharpness: 0.9_f32,
        },
    };

    let mut application = Application::new(grid, scale, Vec2::new(width, height), font, parameters);

    application.random_walk_into_instrution();
    application.sign_into_instructions();
    
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
