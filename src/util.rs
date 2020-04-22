use tetra::math::Vec2;

pub fn collision(
    obj_one: Vec2<f32>,
    obj_two: Vec2<f32>,
    obj_one_width: f32,
    obj_one_height: f32,
    obj_two_width: f32,
    obj_two_height: f32
) -> bool {
    // self.player.position.x < tile.position.x + (tile.texture.width() as f32) &&
    // self.player.position.x + (48.0) > tile.position.x &&
    // self.player.position.y < (tile.position.y + tile.texture.height() as f32) &&
    // self.player.position.y + (48.0) > tile.position.y && tile.collidable == true

    if obj_one.x < obj_two.x + obj_two_width &&
       obj_one.x + obj_one_width > obj_two.x &&
       obj_one.y < obj_two.y + obj_two_height &&
       obj_one.y + obj_one_height > obj_two.y {
        true
    } else {
        false
    }
}
