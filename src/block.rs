use na::{Point3, Point2, Translation3};
use kiss3d::scene::SceneNode;
use kiss3d::window::Window;
use kiss3d::resource::Texture;
use std::rc::Rc;
use std::collections::HashMap;

use crate::map;

//pub const BLOCK_DIM: f32 = 1.0;
pub const CHUNK_DIM: u8 = 16;     // In blocks

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum BlockType {
    Grass,
}

pub struct Block {
    pub block_type: BlockType,
    //pub scene_node: SceneNode,
}

impl Block {
    pub fn new(block_type: BlockType) -> Block {
        /*
        let mut scene_node = node.add_cube(BLOCK_DIM, BLOCK_DIM, BLOCK_DIM);
        scene_node.set_texture(texture);
        scene_node.enable_backface_culling(true);
        //scene_node.set_color(0.0, 1.0, 0.0);
        scene_node.set_local_translation(Translation3::new(pos.x as f32 * BLOCK_DIM, pos.y as f32 * BLOCK_DIM, pos.z as f32 * BLOCK_DIM));
        */

        Block {
            block_type,
        }
    }

    /*
    pub fn remove_scene_node(&mut self, window: &mut Window) {
        window.remove_node(&mut self.scene_node);
    }
    */
}


pub struct Chunk {
    pub pos: Point2<i32>,  // Always at 0 at y
    pub blocks: HashMap<Point3<u8>, Block>,   // Position is relative to chunk for each block
    pub scene_node: SceneNode,
    pub block_scene_nodes: HashMap<Point3<u8>, SceneNode>,
}

impl Chunk {
    pub fn new(window: &mut Window, pos: Point2<i32>, blocks: HashMap<Point3<u8>, Block>) -> Chunk {
        let mut chunk = Chunk {
            pos,
            blocks,
            scene_node: window.add_group(),
            block_scene_nodes: HashMap::new(),
        };

        chunk.register_blocks();

        chunk
    }

    pub fn generate_flat_chunk(window: &mut Window, pos: Point2<i32>) -> Chunk {
        let mut blocks: HashMap<Point3<u8>, Block> = HashMap::new();

        for x in 0..CHUNK_DIM {
            for y in 0..map::FLAT_WORLD_THICKNESS {
                for z in 0..CHUNK_DIM {
                    blocks.insert(Point3::new(x, y as u8, z), Block::new(BlockType::Grass));
                }
            }
        }

        let mut chunk = Chunk {
            pos,
            blocks,
            scene_node: window.add_group(),
            block_scene_nodes: HashMap::new(),
        };

        chunk.register_blocks();

        chunk
    }


    // Initialises block_scene_nodes
    fn register_blocks(&mut self) {
        for (pos, b) in self.blocks.iter() {
            let world_pos = Self::get_block_pos_from_chunk_and_rel_pos(&self.pos, pos);
            let mut c = self.scene_node.add_cube(1.0, 1.0, 1.0);
            c.set_local_translation(Translation3::new(world_pos.x as f32, world_pos.y as f32, world_pos.z as f32));
            c.enable_backface_culling(true);

            self.block_scene_nodes.insert(pos.clone(), c);
        }
    }

    #[inline]
    fn add_block_scene(&mut self, block_pos: &Point3<u8>, block_type: &BlockType) {
        if !self.block_scene_nodes.contains_key(block_pos) {
            let world_pos = Self::get_block_pos_from_chunk_and_rel_pos(&self.pos, block_pos);
            let mut c = self.scene_node.add_cube(1.0, 1.0, 1.0);
            c.set_local_translation(Translation3::new(world_pos.x as f32, world_pos.y as f32, world_pos.z as f32));
            c.enable_backface_culling(true);

            self.block_scene_nodes.insert(block_pos.clone(), c);
        } else {
            panic!("Tried adding a block to scene node in chunk which has already been added.");
        }
    }

    pub fn update(&mut self) {
        
    }

    fn get_block_neighbours(&self, pos: &Point3<u8>) -> Vec<Point3<u8>> {
        let temp_pos = Point3::new(pos.x as i16, pos.y as i16, pos.z as i16);
        let mut temp: Vec<Point3<i16>> = vec![ // 26 in total
            Point3::new(temp_pos.x - 1, temp_pos.y - 1, temp_pos.z - 1),
            Point3::new(temp_pos.x - 1, temp_pos.y - 1, temp_pos.z),
            Point3::new(temp_pos.x - 1, temp_pos.y - 1, temp_pos.z + 1),

            Point3::new(temp_pos.x - 1, temp_pos.y, temp_pos.z - 1),
            Point3::new(temp_pos.x - 1, temp_pos.y, temp_pos.z),
            Point3::new(temp_pos.x - 1, temp_pos.y, temp_pos.z + 1),

            Point3::new(temp_pos.x - 1, temp_pos.y + 1, temp_pos.z - 1),
            Point3::new(temp_pos.x - 1, temp_pos.y + 1, temp_pos.z),
            Point3::new(temp_pos.x - 1, temp_pos.y + 1, temp_pos.z + 1),


            Point3::new(temp_pos.x, temp_pos.y - 1, temp_pos.z - 1),
            Point3::new(temp_pos.x, temp_pos.y - 1, temp_pos.z),
            Point3::new(temp_pos.x, temp_pos.y - 1, temp_pos.z + 1),

            Point3::new(temp_pos.x, temp_pos.y, temp_pos.z - 1),
            Point3::new(temp_pos.x, temp_pos.y, 1),

            Point3::new(temp_pos.x, temp_pos.y + 1, temp_pos.z - 1),
            Point3::new(temp_pos.x, temp_pos.y + 1, temp_pos.z),
            Point3::new(temp_pos.x, temp_pos.y + 1, temp_pos.z + 1),


            Point3::new(temp_pos.x + 1, temp_pos.y - 1, temp_pos.z - 1),
            Point3::new(temp_pos.x + 1, temp_pos.y - 1, temp_pos.z),
            Point3::new(temp_pos.x + 1, temp_pos.y - 1, temp_pos.z + 1),

            Point3::new(temp_pos.x + 1, temp_pos.y, temp_pos.z - 1),
            Point3::new(temp_pos.x + 1, temp_pos.y, temp_pos.z),
            Point3::new(temp_pos.x + 1, temp_pos.y, temp_pos.z + 1),

            Point3::new(temp_pos.x + 1, temp_pos.y + 1, temp_pos.z - 1),
            Point3::new(temp_pos.x + 1, temp_pos.y + 1, temp_pos.z),
            Point3::new(temp_pos.x + 1, temp_pos.y + 1, temp_pos.z + 1),
        ];

        // Only look in current chunk
        temp.retain(|i| i.x > 0 && i.x < 256 && i.y > 0 && i.y < 256 && i.z > 0 && i.z < 256);

        let mut out: Vec<Point3<u8>> = vec![];
        for p in temp.iter() {
            out.push(Point3::new(p.x as u8, p.y as u8, p.z as u8));
        }
        
        out.retain(|i| self.blocks.contains_key(&i));

        out
    }


    // Position conversions:
    // Block pos is Point3<i32>, can be below floor. This is for whole map.
    // Chunk pos is Point2<i32>, has no y since it is height of map.
    // Relative block pos is the position of a block inside it's parent chunk. Point3<u8>, since chunk dimensions is 16 * 16 * 256, so max is 256

    #[inline]
    pub fn get_chunk_pos_at_world_pos(pos: &Point3<f32>) -> Point2<i32> {
        Point2::new(pos.x.floor() as i32/CHUNK_DIM as i32,  pos.z.floor() as i32/CHUNK_DIM as i32)
    }

    #[inline]
    pub fn get_chunk_pos_at_block_pos(pos: &Point3<i32>) -> Point2<i32> {
        Point2::new(pos.x/CHUNK_DIM as i32, pos.y/CHUNK_DIM as i32)
    }

    #[inline]
    pub fn get_block_pos_at_world_pos(pos: &Point3<f32>) -> Point3<i32> {      // Absolute position of block (without taking chunk into consideration)
        Point3::new(pos.x.floor() as i32, pos.y.floor() as i32, pos.z.floor() as i32)
    }

    pub fn get_block_relative_pos_at_block_pos(pos: &Point3<i32>) -> (Point2<i32>, Point3<u8>) {
        let chunk_pos = Self::get_chunk_pos_at_block_pos(pos);
        let rel_pos = Point3::new(
            (pos.x - chunk_pos.x) as u8,
            pos.y as u8,
            (pos.z - chunk_pos.y) as u8     // Chunk's y is actually z
        );
        
        (chunk_pos, rel_pos)
    }

    #[inline]
    pub fn get_block_relative_pos_at_world_pos(pos: &Point3<f32>) -> (Point2<i32>, Point3<u8>) {  // Returns chunk pos and block pos in chunk
        let block_pos = Self::get_block_pos_at_world_pos(pos);
        Self::get_block_relative_pos_at_block_pos(&block_pos)
    }

    #[inline]
    pub fn get_block_pos_from_chunk_and_rel_pos(chunk_pos: &Point2<i32>, block_rel_pos: &Point3<u8>) -> Point3<i32> {
        Point3::new(
            chunk_pos.x + block_rel_pos.x as i32,
            block_rel_pos.y as i32,
            chunk_pos.y + block_rel_pos.z as i32
        )
    }
}
