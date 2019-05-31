use na::{Point3, Translation3};
use kiss3d::scene::SceneNode;
use kiss3d::window::Window;
use kiss3d::resource::Texture;
use std::rc::Rc;

pub const BLOCK_DIM: f32 = 10.0;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum BlockType {
    Grass,
    Stone,
}

pub struct Block {
    pub block_type: BlockType,
    pos: Point3<u32>,
    pub scene_node: SceneNode,
}

impl Block {
    pub fn new(window: &mut Window, texture: Rc<Texture>, block_type: BlockType, pos: Point3<u32>) -> Block {
        let mut scene_node = window.add_cube(BLOCK_DIM, BLOCK_DIM, BLOCK_DIM);
        scene_node.set_texture(texture);
        scene_node.enable_backface_culling(true);
        //scene_node.set_color(0.0, 1.0, 0.0);
        scene_node.set_local_translation(Translation3::new(pos.x as f32 * BLOCK_DIM, pos.y as f32 * BLOCK_DIM, pos.z as f32 * BLOCK_DIM));

        Block {
            block_type,
            pos,
            scene_node,
        }
    }

    pub fn remove_scene_node(&mut self, window: &mut Window) {
        window.remove_node(&mut self.scene_node);
    }
}
