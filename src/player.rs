use tetra::math::Vec2;
use tetra::graphics::animation::Animation;

pub struct Player {
    animation: Animation,
    position: Vec2<f32>,
    velocity_x: f32,
    velocity_y: f32,
    jumping: bool,
}

impl Player {
    pub fn new(
        animation: Animation,
        position: Vec2<f32>,
        velocity_x: f32,
        velocity_y: f32,
        jumping: bool,
    ) -> Player {
        Player {
            animation,
            position,
            velocity_x,
            velocity_y,
            jumping,
        }
    }

    pub fn bounds(&self) -> Rectangle {
        Rectangle::new(
            self.position.x,
            self.position.y,
            self.animation.texture().width() as f32,
            self.animation.texture().height() as f32,
        )
    }
}
