extern crate kiss3d;
extern crate nalgebra as na;

mod block;
mod map;

use na::{Point2};
use kiss3d::{
    window::Window,
    light::Light,
    resource::TextureManager,
};
use std::collections::HashMap;
use std::path::Path;

use crate::block::{BlockType, Chunk};
use crate::map::{MapType};

struct MainState {
    window: Window,
    block_textures: TextureManager,
    block_texture_name_map: HashMap<BlockType, String>,

    map_type: MapType,

    //blocks: HashMap<Point3<u32>, Block>,
    chunks: HashMap<Point2<i32>, Chunk>
}

impl MainState {
    fn new(map_type: MapType) -> MainState {
        let mut window = Window::new("MC");
        window.set_light(Light::StickToCamera);
        window.set_background_color(0.5898, 0.8477, 0.9844);

        let (block_textures, block_texture_name_map) = Self::load_block_textures();

        let mut main = MainState {
            window,
            block_textures,
            block_texture_name_map,

            map_type,

            chunks: HashMap::new(),
        };

        main.load_map(4, 4);

        main
    }

    fn mainloop(&mut self) {
        while self.window.render() {
        }
    }

    fn load_block_textures() -> (TextureManager, HashMap<BlockType, String>) {
        let mut tex_manager = TextureManager::new();
        let mut name_map = HashMap::new();

        // Grass
        tex_manager.add(&Path::new("./textures/blocks/grass_block.png"), "grass_block");
        name_map.insert(BlockType::Grass, String::from("grass_block"));

        (tex_manager, name_map)
    }

    fn load_map(&mut self, xwidth: i32, zwidth: i32) {
        self.chunks = HashMap::new();

        for x in 0..xwidth {
            for z in 0..zwidth {
                self.add_chunk(Point2::new(x * block::CHUNK_DIM as i32, z * block::CHUNK_DIM as i32));
            }
        }
    }

    fn add_chunk(&mut self, pos: Point2<i32>) {
        if !self.chunks.contains_key(&pos) {
            match self.map_type {
                MapType::Flat => {
                    self.chunks.insert(
                        pos.clone(),
                        Chunk::generate_flat_chunk(&mut self.window, &mut self.block_textures, &self.block_texture_name_map, pos)
                    );
                }
            }
        } else {
            panic!("Tried to add chunk that already exists: {} {}", pos.x, pos.y);
        }
    }
}

fn main() {
    let mut game = MainState::new(MapType::Flat);

    game.mainloop();
}
