use tetra::graphics::{self, Color, Texture, Rectangle};
use tetra::graphics::animation::Animation;
use tetra::{Context, ContextBuilder, State};
use tetra::input::{self, Key};
use tetra::math::Vec2;
use std::time::Duration;
// use std::{thread, time};
// use tetra::window;

const WINDOW_WIDTH: f32 = 1280.0;
const WINDOW_HEIGHT: f32 = 960.0;

struct Tile {
    texture: Texture,
    position: Vec2<f32>,
}

impl Tile {
    fn new(
        texture: Texture,
        position: Vec2<f32>,
    ) -> Tile {
        Tile {
            texture,
            position,
        }
    }
}

struct Player {
    animation: Animation,
    position: Vec2<f32>,
    velocity_x: f32,
    velocity_y: f32,
    jumping: bool,
}

impl Player {
    fn new(
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
}

struct GameState {
    player: Player,
    tiles: Vec<Tile>,
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState>{

        let quarter_second = Duration::from_millis(250);

        let player_texture = Texture::new(ctx, "./resources/sorcerer_idle.png")?;
        let player_animation = Animation::new(
            player_texture,
            Rectangle::row(0.0, 0.0, 48.0, 48.0).take(2).collect(),
            quarter_second,
        );
        let player_position = Vec2::new (
            WINDOW_WIDTH / 2.0 - 48.0 / 2.0,
            WINDOW_HEIGHT / 2.0 - 48.0 as f32 / 2.0
        );
        let player_velocity_x = 0.0;
        let player_velocity_y = 0.0;
        let player_jumping = false;

        let mut tiles: Vec<Tile> = Vec::new();


        let mut tile_map = [
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,],
            [0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,],
        ];


        for (y, row) in tile_map.iter_mut().enumerate() {
            for (x, col) in row.iter_mut().enumerate() {
                let mut stone = 1;

                if col == &mut stone {
                    let tile_texture = Texture::new(ctx, "./resources/stone_tile.png")?;
                    let tile_position = Vec2::new (
                        x as f32 * 32.0,
                        y as f32 * 32.0,
                    );

                    tiles.push(Tile::new(tile_texture, tile_position));
                } else {
                    // do nothing
                }
            }
        }








        Ok(GameState {
            player: Player::new(
                player_animation,
                player_position,
                player_velocity_x,
                player_velocity_y,
                player_jumping,
            ),
            tiles: tiles,
        })
    }
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {

        if input::is_key_pressed(ctx, Key::Space) && self.player.jumping == false {
            self.player.velocity_y -= 18.0;
            self.player.jumping = true;
        }


        // Player Movement

        if input::is_key_pressed(ctx, Key::Space) == false {


            for tile in &self.tiles {

                // self.player.position.x < tile.position.x + (tile.texture.width() as f32) &&
                // self.player.position.x + (48.0) > tile.position.x &&
                // self.player.position.y + (48.0) > tile.position.y &&
                // self.player.position.y < (tile.position.y + tile.texture.height() as f32)

                if  self.player.position.y + (48.0) > tile.position.y &&
                    self.player.position.x + (48.0) > tile.position.x &&
                    self.player.position.x < tile.position.x + (tile.texture.width() as f32) &&
                    self.player.position.y < tile.position.y {

                    self.player.jumping = false;
                    self.player.velocity_y = 0.0;
                    self.player.position.y = tile.position.y - (self.player.animation.texture().height() as f32 - 0.1);


                    break;
                } else {
                    self.player.jumping = true;
                }
            }

        }


        // Move Left
        if input::is_key_down(ctx, Key::A) {
            if self.player.position.x > 0.0 {
                if self.player.jumping == true {
                    self.player.velocity_x = -6.0;
                } else {
                    self.player.velocity_x = -6.0;
                    self.player.position.x += self.player.velocity_x;
                }
            }
        } else if input::is_key_down(ctx, Key::D) {
            if self.player.position.x < WINDOW_WIDTH - self.player.animation.texture().width() as f32 {
                if self.player.jumping == true {
                    self.player.velocity_x = 6.0;
                } else {
                    self.player.velocity_x = 6.0;
                    self.player.position.x += self.player.velocity_x;
                }
            }
        } else if self.player.jumping == false {
            self.player.velocity_x *= 0.9;
            self.player.position.x += self.player.velocity_x;
        }

        if self.player.jumping == true {
            self.player.velocity_y += 1.0; // gravity
            self.player.position.x += self.player.velocity_x;
            self.player.position.y += self.player.velocity_y;
        }

        // if it hits the ground
        // if self.player.position.y > (WINDOW_HEIGHT / 2.0 - self.player.animation.texture().height() as f32 / 2.0) + 2.0 {
        //     self.player.jumping = false;
        //     self.player.velocity_y = 0.0;
        // }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.08, 0.08, 0.08));

        let _quarter_second = Duration::from_millis(250);

        if input::is_key_down(ctx, Key::D) {
            let player_texture_walking_right: Texture = Texture::new(ctx, "./resources/sorcerer_walking_right.png")?;
            self.player.animation.set_texture(player_texture_walking_right);
            self.player.animation.advance(ctx);
        } else if input::is_key_down(ctx, Key::A) {
            let player_texture_walking_left: Texture = Texture::new(ctx, "./resources/sorcerer_walking_left.png")?;
            self.player.animation.set_texture(player_texture_walking_left);
            self.player.animation.advance(ctx);
        } else {
            let player_texture_idle: Texture = Texture::new(ctx, "./resources/sorcerer_idle.png")?;
            self.player.animation.set_texture(player_texture_idle);
            self.player.animation.advance(ctx);
        }

        graphics::draw(ctx, &self.player.animation, self.player.position);

        for x in &self.tiles {
            graphics::draw(ctx, &x.texture, x.position);
        }


        Ok(())
    }
}

fn main() -> tetra::Result {
    ContextBuilder::new("HYPOTERRA", WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}
