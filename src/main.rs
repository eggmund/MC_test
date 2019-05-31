extern crate kiss3d;
extern crate nalgebra as na;

mod block;
mod map;

use na::{Point3, Vector3, UnitQuaternion, Translation3};
use kiss3d::{
    window::Window,
    light::Light,
    scene::SceneNode,
    resource::TextureManager,
};
use std::collections::HashMap;
use std::path::Path;

use block::{Block, BlockType};
use map::{MapType};

struct MainState {
    window: Window,
    block_scene_node: SceneNode,
    block_textures: TextureManager,
    block_texture_name_map: HashMap<BlockType, String>,

    blocks: HashMap<Point3<u32>, Block>,
}

impl MainState {
    fn new() -> MainState {
        let mut window = Window::new("MC");
        window.set_light(Light::StickToCamera);
        window.set_background_color(0.5898, 0.8477, 0.9844);
        let block_scene_node = window.add_group();

        let (block_textures, block_texture_name_map) = Self::load_block_textures();

        let mut main = MainState {
            window,
            block_scene_node,
            block_textures,
            block_texture_name_map,

            blocks: HashMap::new(),
        };

        main.load_map(MapType::Flat, 40, 40, 2);

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

    fn load_map(&mut self, map_type: MapType, xwidth: u32, zwidth: u32, depth: u32) {
        self.blocks = HashMap::new();

        for x in 0..xwidth {
            for y in 0..depth {
                for z in 0..zwidth {
                    self.add_block(BlockType::Grass, Point3::new(x, y, z));
                }
            }
        }
    }

    fn add_block(&mut self, block_type: BlockType, pos: Point3<u32>) {
        let texture_name = self.block_texture_name_map.get(&block_type).unwrap();

        let texture = match self.block_textures.get(texture_name) {
            Some(x) => x,
            None => panic!("Texture not found {}", texture_name)
        };
        
        if !self.blocks.contains_key(&pos) {
            self.blocks.insert(pos, Block::new(&mut self.window, texture, block_type, pos));
        } else {
            panic!("Tried to add block at position already occupied: {} {} {}", pos.x, pos.y, pos.z);
        }
    }
    
    
    fn remove_block(&mut self, pos: Point3<u32>) {
        match self.blocks.get_mut(&pos) {
            Some(b) => {
                b.remove_scene_node(&mut self.window);
                self.blocks.remove(&pos);
            },
            None => panic!("Tried to remove block that does not exist at: {} {} {}", pos.x, pos.y, pos.z)
        };
    }
    
}

fn main() {
    let mut game = MainState::new();

    game.mainloop();
}
