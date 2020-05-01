use tetra::math::Vec2;
use tetra::graphics::animation::Animation;
use tetra::graphics::{Texture, Camera};

pub struct Tile {
    pub texture: Texture,
    pub position: Vec2<f32>,
    pub collidable: bool,
}

pub struct Player {
    pub animation: Animation,
    pub position: Vec2<f32>,
    pub velocity_x: f32,
    pub colliding: bool,
    pub facing: i8,
    pub prev_facing: i8,
    pub alive: bool,
}

pub struct PlayerAttackSphere {
    pub animation: Animation,
    pub position: Vec2<f32>,
    pub velocity: f32,
    pub facing: i8,
    pub visible: bool,
}

pub struct Enemy {
    pub animation: Animation,
    pub position: Vec2<f32>,
    pub velocity: f32,
    pub range_end: f32,
    pub range_start: f32,
    pub facing: i8,
}

// pub struct Npc {
//     pub animation: Animation,
//     pub position: Vec2<f32>,
// }

pub struct GameState {
    pub player: Player,
    pub tiles: Vec<Tile>,
    pub player_attack_instances: Vec<PlayerAttackSphere>,
    pub enemy_instances: Vec<Enemy>,
    pub camera: Camera,
}
