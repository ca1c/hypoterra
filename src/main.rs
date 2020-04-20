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
    colliding: bool,
    facing: i8,
    prev_facing: i8,
}

impl Player {
    fn new(
        animation: Animation,
        position: Vec2<f32>,
        velocity_x: f32,
        colliding: bool,
        facing: i8,
        prev_facing: i8,
    ) -> Player {
        Player {
            animation,
            position,
            velocity_x,
            colliding,
            facing,
            prev_facing,
        }
    }
}

struct PlayerAttackSphere {
    animation: Animation,
    position: Vec2<f32>,
    velocity: f32,
}

impl PlayerAttackSphere {
    fn new(
        animation: Animation,
        position: Vec2<f32>,
        velocity: f32,
    ) -> PlayerAttackSphere {
        PlayerAttackSphere{
            animation,
            position,
            velocity,
        }
    }
}

// Attack balls will probably be an array so that you can shoot multiple at once

struct GameState {
    player: Player,
    attack_ball: PlayerAttackSphere,
    tiles: Vec<Tile>,
    player_attack_instances: Vec<PlayerAttackSphere>,
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState>{

        let quarter_second = Duration::from_millis(250);
        let tenth_second = Duration::from_millis(100);

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
        let player_colliding = false;
        // 0: sorcerer_idle, facing none
        // 1: sorcerer_walking_right, facing right
        // 2: sorcerer_walking_left, facing left
        // 3: sorcerer_walking_up, facing up
        // 4: sorcerer_walking_down, facing down
        let player_facing = 0;
        let player_prev_facing = 0;

        let attack_sphere_texture = Texture::new(ctx, "./resources/attack_ball.png")?;
        let attack_sphere_animation = Animation::new(
            attack_sphere_texture,
            Rectangle::row(0.0, 0.0, 32.0, 32.0).take(2).collect(),
            tenth_second,
        );
        let attack_sphere_position = Vec2::new (
            WINDOW_WIDTH / 2.0 - 48.0 / 2.0,
            WINDOW_HEIGHT / 2.0 - 48.0 as f32 / 2.0
        );
        let attack_sphere_velocity = 0.0;

        let mut player_attack_instances: Vec<PlayerAttackSphere> = Vec::new();


        let mut tiles: Vec<Tile> = Vec::new();
        let mut tile_map = [
            [1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1,],
            [1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1,],
            [1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1,],
            [1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1,],
            [1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1,],
            [1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1,],
            [1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1,],
            [1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1,],
            [1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1,],
            [1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1,],
            [1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1,],
            [1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1,],
            [1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1,],
            [1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1,],
            [1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1,],
            [1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1,],
            [1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1,],
            [1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1,],
            [1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1,],
            [1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1,],
            [1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1,],
            [1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1,],
            [1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1,],
            [1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1,],
            [1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1,],
            [1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1,],
            [1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1,],
            [1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1,],
            [1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1,],
            [1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1,],
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
                player_colliding,
                player_facing,
                player_prev_facing,
            ),
            attack_ball: PlayerAttackSphere::new(
                attack_sphere_animation,
                attack_sphere_position,
                attack_sphere_velocity,
            ),
            tiles: tiles,
            player_attack_instances: player_attack_instances,
        })
    }
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {

        for tile in &self.tiles {
            if self.player.position.x < tile.position.x + (tile.texture.width() as f32) &&
                self.player.position.x + (48.0) > tile.position.x &&
                self.player.position.y < (tile.position.y + tile.texture.height() as f32) &&
                self.player.position.y + (48.0) > tile.position.y {

                    if self.player.facing == 2 {
                        self.player.colliding = true;
                        self.player.position.x = self.player.position.x + 7.0;
                    } else if self.player.facing == 1 {
                        self.player.colliding = true;
                        self.player.position.x = self.player.position.x - 7.0;
                    } else if self.player.facing == 3 {
                        self.player.colliding = true;
                        self.player.position.y = self.player.position.y + 7.0;
                    } else if self.player.facing == 4 {
                        self.player.colliding = true;
                        self.player.position.y = self.player.position.y - 7.0;
                    }


                    break;
                } else {
                    self.player.colliding = false;
                }
        }

        // Attack Instance Loop


        // Move Left
        if input::is_key_down(ctx, Key::A) && self.player.colliding == false {
            if self.player.position.x > 0.0 {
                self.player.velocity_x = -6.0;
                self.player.position.x += self.player.velocity_x;

                self.player.facing = 2;
            }
        } else if input::is_key_down(ctx, Key::D) && self.player.colliding == false {
            if self.player.position.x < WINDOW_WIDTH - self.player.animation.texture().width() as f32 {
                self.player.velocity_x = 6.0;
                self.player.position.x += self.player.velocity_x;

                self.player.facing  = 1;
            }
        } else if input::is_key_down(ctx, Key::W) && self.player.colliding == false {
            if self.player.position.y > WINDOW_HEIGHT - WINDOW_HEIGHT {
                self.player.velocity_x = 6.0;
                self.player.position.y -= self.player.velocity_x;

                self.player.facing = 3;
            }
        } else if input::is_key_down(ctx, Key::S) && self.player.colliding == false {
            if self.player.position.y + (self.player.animation.texture().height() as f32) < WINDOW_HEIGHT {
                self.player.velocity_x = 6.0;
                self.player.position.y += self.player.velocity_x;

                self.player.facing = 4;
            }
        } else {
            self.player.prev_facing = self.player.facing;
            self.player.facing = 0;
        }

        // Attack input handling
        if input::is_key_pressed(ctx, Key::Space) && self.player_attack_instances.len() < 6 {
            let tenth_second = Duration::from_millis(100);
            let attack_sphere_texture = Texture::new(ctx, "./resources/attack_ball.png")?;
            let attack_sphere_animation = Animation::new(
                attack_sphere_texture,
                Rectangle::row(0.0, 0.0, 32.0, 32.0).take(2).collect(),
                tenth_second,
            );
            let attack_sphere_position = Vec2::new (
                self.player.position.x,
                self.player.position.y,
            );
            let attack_sphere_velocity = 6.0;

            self.player_attack_instances.push(PlayerAttackSphere::new(
                attack_sphere_animation,
                attack_sphere_position,
                attack_sphere_velocity,
            ));
        }

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
        } else if input::is_key_down(ctx, Key::W) {
            let player_texture_walking_left: Texture = Texture::new(ctx, "./resources/sorcerer_walking_up.png")?;
            self.player.animation.set_texture(player_texture_walking_left);
            self.player.animation.advance(ctx);
        } else if input::is_key_down(ctx, Key::S) {
            let player_texture_walking_left: Texture = Texture::new(ctx, "./resources/sorcerer_walking_down.png")?;
            self.player.animation.set_texture(player_texture_walking_left);
            self.player.animation.advance(ctx);
        } else {
            let player_texture_idle: Texture = Texture::new(ctx, "./resources/sorcerer_idle.png")?;
            self.player.animation.set_texture(player_texture_idle);
            self.player.animation.advance(ctx);
        }

        graphics::draw(ctx, &self.player.animation, self.player.position);

        // This will be inside a loop later
        // graphics::draw(ctx, &self.attack_ball.animation, self.attack_ball.position);

        for x in &mut self.player_attack_instances {
            graphics::draw(ctx, &x.animation, x.position);
            x.animation.advance(ctx);
        }

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
