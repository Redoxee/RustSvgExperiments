use glam::*;
use rand::Rng;

use std::cmp::Ordering;
use std::collections::HashSet;

use crate::utils::*;

pub struct TileInfo {
    pub index : usize,
    pub position : Vec2,
    pub neighbors : Vec<usize>,
    pub vertices : Vec<Vec2>,
}

impl TileInfo {
    pub fn new(index: usize, position : Vec2) -> TileInfo {
        TileInfo {
            index,
            position,
            neighbors : Vec::new(),
            vertices : Vec::new(),
        }
    }
}

pub struct Grid {
    pub tiles : Vec<TileInfo>,
    pub tile_scale : f32,
}

#[derive(Copy, Clone)]
pub struct RandomWalkParameters {
    pub smooth_number_of_points : usize,
    pub smooth_sharpness : f32,
    pub slice_percentage : f32,
}

impl Grid {
    pub fn hex_grid_size(col : usize, row : usize, tile_scale : f32) -> Vec2{
        Vec2::new(col as f32 * tile_scale * 2_f32 * std::f32::consts::FRAC_PI_6.cos(), row as f32 * tile_scale * 3_f32 * std::f32::consts::FRAC_PI_6.sin())   
    }

    pub fn hex_grid(col : usize, row : usize, tile_scale : f32, base_position : Vec2) -> Grid {
        let mut grid = Grid {
            tiles: Vec::new(),
            tile_scale,
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

                let tile_index = grid.tiles.len();
                let mut tile = TileInfo::new(tile_index, position);

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

    pub fn random_walk(&self, parameters: RandomWalkParameters,instructions: &mut Vec<Instruction>) {
        let mut random = rand::thread_rng();
        let mut used_indexes = HashSet::new();
        let mut unused_indexes: Vec<usize> = (0..self.tiles.len()).collect();
        let mut tile_remaining = self.tiles.len();

        let mut current_index = random.gen_range(0..self.tiles.len());

        instructions.push(Instruction::MoveTo(self.tiles[current_index].position));

        let frame = 100;
        let mut counter = frame;

        let mut walks = Vec::new();
        let mut current_walk = Vec::new();

        current_walk.push(self.tiles[current_index].position);

        while tile_remaining > 0 {
            used_indexes.insert(current_index);
            unused_indexes.swap_remove(unused_indexes.iter().position(|&i| i == current_index).unwrap());
            tile_remaining = tile_remaining - 1;

            let valid_neighbors : Vec<&usize> = self.tiles[current_index].neighbors.iter().filter(|&it| !used_indexes.contains(it)).collect();
            if valid_neighbors.len() > 0 {
                let neighbor_index = *valid_neighbors[random.gen_range(0..valid_neighbors.len())];
                current_index = self.tiles[neighbor_index].index;
                
                current_walk.push(self.tiles[current_index].position);
            }
            else if tile_remaining > 0 {
                current_index = unused_indexes[random.gen_range(0..unused_indexes.len())];
                
                walks.push(current_walk);
                current_walk = Vec::new();
                current_walk.push(self.tiles[current_index].position);
            }

            counter = counter - 1;
            if counter < 1 {
                counter = frame;
                println!("{:.3} : {} / {}", tile_remaining as f32 / self.tiles.len() as f32, tile_remaining, self.tiles.len());
            }
        }

        walks.push(current_walk);
        walks.sort_by(|a ,b| {
            let cmp = a.len().cmp(&b.len());
            match cmp {
                Ordering::Equal => {
                    let delta = b[0].x - a[0].x;
                    if delta < -std::f32::MIN_POSITIVE {
                        Ordering::Greater
                    }
                    else if delta > std::f32::MIN_POSITIVE {
                        Ordering::Less
                    }
                    else
                    {
                        let delta = b[0].y - a[0].y;
                        if delta < -std::f32::MIN_POSITIVE {
                            Ordering::Greater
                        }
                        else if delta > std::f32::MIN_POSITIVE {
                            Ordering::Less
                        }
                        else {
                            Ordering::Equal
                        }
                    }
                    
                },
                _other =>{
                    cmp
                }
            }
        });

        walks = walks.into_iter().map(|w| smooth(w, parameters.smooth_number_of_points, parameters.smooth_sharpness)).collect();
        let (_, half) = walks.split_at((walks.len() as f32 * parameters.slice_percentage).round() as usize);
        let walks = half;

        for walk in walks {
            if walk.len() < 1 {
                continue;
            }

            if walk.len() < 2 {
                // print_circle_to_instructions(walk[0], self.tile_scale / 2_f32, 8, instructions);

                continue;
            }

            instructions.push(Instruction::MoveTo(walk[0]));
            for index in 1..walk.len() {
                instructions.push(Instruction::LineTo(walk[index]));
            }
        }
    }
}
