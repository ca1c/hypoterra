mod level1;
mod util;
mod game_structs;

use tetra::graphics::{self, Color, Texture, Rectangle, Camera};
use tetra::graphics::animation::Animation;
use tetra::{Context, ContextBuilder, State};
use tetra::input::{self, Key};
use tetra::math::Vec2;
use std::time::Duration;
// use std::{thread, time};
// use tetra::window;

use util::collision;
use game_structs::{Tile, Player, PlayerAttackSphere, Enemy, GameState};

const WINDOW_WIDTH: f32 = 1280.0;
const WINDOW_HEIGHT: f32 = 960.0;

impl Tile {
    fn new(
        texture: Texture,
        position: Vec2<f32>,
        collidable: bool,
    ) -> Tile {
        Tile {
            texture,
            position,
            collidable,
        }
    }
}

impl Player {
    fn new(
        animation: Animation,
        position: Vec2<f32>,
        velocity_x: f32,
        colliding: bool,
        facing: i8,
        prev_facing: i8,
        alive: bool,
    ) -> Player {
        Player {
            animation,
            position,
            velocity_x,
            colliding,
            facing,
            prev_facing,
            alive,
        }
    }
}

impl PlayerAttackSphere {
    fn new(
        animation: Animation,
        position: Vec2<f32>,
        velocity: f32,
        facing: i8,
        visible: bool,
    ) -> PlayerAttackSphere {
        PlayerAttackSphere{
            animation,
            position,
            velocity,
            facing,
            visible,
        }
    }
}

impl Enemy {
    fn new(
        animation: Animation,
        position: Vec2<f32>,
        velocity: f32,
        range_end: f32,
        range_start: f32,
        facing: i8,
    ) -> Enemy {
        Enemy {
            animation,
            position,
            velocity,
            range_end,
            range_start,
            facing,
        }
    }
}


impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState>{

        let quarter_second = Duration::from_millis(250);
        let twentieth_second = Duration::from_millis(50);

        let player_texture = Texture::new(ctx, "./resources/sorcerer_idle_down.png")?;
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
        let player_prev_facing = 4;
        let player_alive = true;

        let player_attack_instances: Vec<PlayerAttackSphere> = Vec::new();


        let mut tiles: Vec<Tile> = Vec::new();

        let enemy_positions: Vec<Vec2<f32>> = vec![Vec2::new(960.0, 320.0), Vec2::new(896.0, 512.0)];

        let mut enemies: Vec<Enemy> = Vec::new();

        for pos in enemy_positions {
            let enemy_texture = Texture::new(ctx, "./resources/beer_idle.png")?;
            let enemy_animation = Animation::new(
                enemy_texture,
                Rectangle::row(0.0, 0.0, 48.0, 48.0).take(24).collect(),
                twentieth_second,
            );
            let enemy_position = pos;
            let enemy_velocity = 3.0;
            let enemy_range_end = &enemy_position.x - 200.0;
            let enemy_range_start = &enemy_position.x + 5.0;
            let enemy_facing = 0;

            enemies.push(Enemy::new(
                enemy_animation,
                enemy_position,
                enemy_velocity,
                enemy_range_end,
                enemy_range_start,
                enemy_facing,
            ));
        }

        for (y, row) in level1::TILEMAP.iter_mut().enumerate() {
            for (x, col) in row.iter_mut().enumerate() {
                let mut stone = 1;
                let mut woodplank = 2;
                let mut grass = 4;

                if col == &mut stone {
                    let tile_texture = Texture::new(ctx, "./resources/stone_tile.png")?;
                    let tile_position = Vec2::new (
                        x as f32 * 32.0,
                        y as f32 * 32.0,
                    );

                    tiles.push(Tile::new(tile_texture, tile_position, true));
                } else if col == &mut woodplank {
                    let tile_texture = Texture::new(ctx, "./resources/woodplank_tile.png")?;
                    let tile_position = Vec2::new (
                        x as f32 * 32.0,
                        y as f32 * 32.0,
                    );

                    tiles.push(Tile::new(tile_texture, tile_position, false));
                } else if col == &mut grass {
                    let tile_texture = Texture::new(ctx, "./resources/grass_tile.png")?;
                    let tile_position = Vec2::new (
                        x as f32 * 32.0,
                        y as f32 * 32.0,
                    );

                    tiles.push(Tile::new(tile_texture, tile_position, false));
                } else {

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
                player_alive,
            ),
            tiles: tiles,
            player_attack_instances: player_attack_instances,
            enemy_instances: enemies,
            camera: Camera::with_window_size(ctx),
        })
    }
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {

        // self.player.position.x < tile.position.x + (tile.texture.width() as f32) &&
        // self.player.position.x + (48.0) > tile.position.x &&
        // self.player.position.y < (tile.position.y + tile.texture.height() as f32) &&
        // self.player.position.y + (48.0) > tile.position.y
        if self.player.alive == true {
            self.camera.position.x = self.player.position.x + 24.0;
            self.camera.position.y = self.player.position.y + 24.0;
            self.camera.update();
        }

        for tile in &self.tiles {
            if collision(self.player.position, tile.position, 48.0, 48.0, 32.0, 32.0) == true && tile.collidable == true {

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

            let mut index = 0;
            for attack in &mut self.player_attack_instances {
                if collision(
                    attack.position,
                    tile.position,
                    32.0,
                    32.0,
                    32.0,
                    32.0,) == true &&
                    tile.collidable == true {

                        // The commented code was a bug but it might be an interesting concept to add
                        // later on in the game
                        // // self.tiles.remove(index);

                        self.player_attack_instances.remove(index);

                        break;
                } else {
                    index += 1;
                }
            }
        }

        // attack remover
        for attack in self.player_attack_instances.iter() {
            if attack.visible == false {
                let index_2 = self.player_attack_instances.iter().position(|r| r.position == attack.position).unwrap();

                &self.player_attack_instances.remove(index_2);

                break;
            } else {
                //continue
            }
        }

        // let attacks = &self.player_attack_instances;
        // let mut enemies = &self.enemy_instances;
        for mut attack in &mut self.player_attack_instances {
            for enemy in &self.enemy_instances {
                // enemy collision with attack instance
                if collision(attack.position, enemy.position, 32.0, 32.0, 48.0, 48.0) == true &&
                enemy.position.x < self.camera.viewport_width as f32 &&
                enemy.position.y < self.camera.viewport_height as f32 &&
                enemy.position.x > (self.camera.viewport_width as f32) - (self.camera.viewport_width as f32) &&
                enemy.position.y > (self.camera.viewport_height as f32) - (self.camera.viewport_height as f32) {
                    let index_1 = self.enemy_instances.iter().position(|r| r.position == enemy.position).unwrap();

                    &self.enemy_instances.remove(index_1);
                    attack.visible = false;

                    break;
                }
            }

            // can put more collision detection here
        }

        for enemy in &self.enemy_instances {
            if collision(self.player.position, enemy.position, 48.0, 48.0, 48.0, 48.0) == true {
                self.player.alive = false;
            }
        }

        for mut enemy in &mut self.enemy_instances {
            if enemy.position.x < enemy.range_end && enemy.facing == 0 {
                enemy.facing = 1;
                enemy.position.x += enemy.velocity;
            } else if enemy.position.x > enemy.range_end && enemy.position.x < enemy.range_start && enemy.facing == 0 {
                enemy.facing = 0;
                enemy.position.x -= enemy.velocity;
            } else if enemy.position.x > enemy.range_end && enemy.position.x < enemy.range_start && enemy.facing == 1 {
                enemy.facing = 1;
                enemy.position.x += enemy.velocity;
            } else if enemy.position.x > enemy.range_start && enemy.facing == 1 {
                enemy.facing = 0;
                enemy.position.x -= enemy.velocity;
            } else if enemy.position.x > enemy.range_start && enemy.facing == 0 {
                enemy.facing = 0;
                enemy.position.x -= enemy.velocity;
            }
        }

        // Attack Instance Loop
        for mut attack in &mut self.player_attack_instances {
            if attack.facing == 1 {
                attack.position.x += attack.velocity;
            } else if attack.facing == 2 {
                attack.position.x += attack.velocity;
            } else if attack.facing == 3 {
                attack.position.y += attack.velocity;
            } else if attack.facing == 4 {
                attack.position.y += attack.velocity;
            }

        }

        // Move Left
        if input::is_key_down(ctx, Key::A) && self.player.colliding == false {
            self.player.velocity_x = -6.0;
            self.player.position.x += self.player.velocity_x;
            self.player.facing = 2;

            self.player.prev_facing = self.player.facing;
        } else if input::is_key_down(ctx, Key::D) && self.player.colliding == false {
            self.player.velocity_x = 6.0;
            self.player.position.x += self.player.velocity_x;
            self.player.facing  = 1;

            self.player.prev_facing = self.player.facing;
        } else if input::is_key_down(ctx, Key::W) && self.player.colliding == false {
            self.player.velocity_x = 6.0;
            self.player.position.y -= self.player.velocity_x;
            self.player.facing = 3;

            self.player.prev_facing = self.player.facing;
        } else if input::is_key_down(ctx, Key::S) && self.player.colliding == false {
            self.player.velocity_x = 6.0;
            self.player.position.y += self.player.velocity_x;
            self.player.facing = 4;

            self.player.prev_facing = self.player.facing;
        } else {
            self.player.prev_facing = self.player.prev_facing;
            self.player.facing = 0;
        }

        // Attack input handling
        if input::is_key_pressed(ctx, Key::Space) && self.player_attack_instances.len() < 6 && self.player.alive == true {
            let tenth_second = Duration::from_millis(100);
            let attack_sphere_texture = Texture::new(ctx, "./resources/attack_ball.png")?;
            let attack_sphere_animation = Animation::new(
                attack_sphere_texture,
                Rectangle::row(0.0, 0.0, 32.0, 32.0).take(2).collect(),
                tenth_second,
            );
            let attack_sphere_position = Vec2::new (
                self.player.position.x + (48.0 / 4.0),
                self.player.position.y + (48.0 / 4.0),
            );

            let mut attack_sphere_velocity = 0.0;

            let mut attack_sphere_facing: i8 = 1;

            let attack_sphere_visible: bool = true;

            if self.player.facing != 0 {
                attack_sphere_facing = self.player.facing;
            }



            if self.player.facing == 1 {
                // facing right
                attack_sphere_velocity = 10.0;
            } else if self.player.facing == 2 {
                // facing left
                attack_sphere_velocity = -10.0;
            } else if self.player.facing == 3 {
                // facing up
                attack_sphere_velocity = -10.0;
            } else if self.player.facing == 4 {
                // facing down
                attack_sphere_velocity = 10.0;
            } else if self.player.facing == 0 {
                // go back to the previous facing value because this is the idle pos
                match self.player.prev_facing {
                    1 => {
                        attack_sphere_velocity = 10.0;
                        attack_sphere_facing = self.player.prev_facing;
                    },
                    2 => {
                        attack_sphere_velocity = -10.0;
                        attack_sphere_facing = self.player.prev_facing;
                    },
                    3 => {
                        attack_sphere_velocity = -10.0;
                        attack_sphere_facing = self.player.prev_facing;
                    },
                    4 => {
                        attack_sphere_velocity = 10.0;
                        attack_sphere_facing = self.player.prev_facing;
                    },
                    _ => {
                        //nothing
                    },
                }
            }

            self.player_attack_instances.push(PlayerAttackSphere::new(
                attack_sphere_animation,
                attack_sphere_position,
                attack_sphere_velocity,
                attack_sphere_facing,
                attack_sphere_visible,
            ));
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {

        graphics::clear(ctx, Color::rgb(0.08, 0.08, 0.08));

        graphics::set_transform_matrix(ctx, self.camera.as_matrix());

        let _quarter_second = Duration::from_millis(250);

        let mut player_texture: Texture = Texture::new(ctx, "./resources/sorcerer_walking_down.png")?;

        if input::is_key_down(ctx, Key::D) {
            player_texture = Texture::new(ctx, "./resources/sorcerer_walking_right.png")?;
        } else if input::is_key_down(ctx, Key::A) {
            player_texture = Texture::new(ctx, "./resources/sorcerer_walking_left.png")?;
        } else if input::is_key_down(ctx, Key::W) {
            player_texture = Texture::new(ctx, "./resources/sorcerer_walking_up.png")?;
        } else if input::is_key_down(ctx, Key::S) {
            player_texture = Texture::new(ctx, "./resources/sorcerer_walking_down.png")?;
        } else {

            // 0: sorcerer_idle, facing none
            // 1: sorcerer_walking_right, facing right
            // 2: sorcerer_walking_left, facing left
            // 3: sorcerer_walking_up, facing up
            // 4: sorcerer_walking_down, facing down

            if self.player.prev_facing == 1 {
                player_texture = Texture::new(ctx, "./resources/sorcerer_idle_right.png")?;
            } else if self.player.prev_facing == 2 {
                player_texture = Texture::new(ctx, "./resources/sorcerer_idle_left.png")?;
            } else if self.player.prev_facing == 3 {
                player_texture = Texture::new(ctx, "./resources/sorcerer_idle_up.png")?;
            } else if self.player.prev_facing == 4 {
                player_texture = Texture::new(ctx, "./resources/sorcerer_idle_down.png")?;
            }
        }

        self.player.animation.set_texture(player_texture);
        self.player.animation.advance(ctx);

        // This will be inside a loop later
        // graphics::draw(ctx, &self.attack_ball.animation, self.attack_ball.position);


        for x in &self.tiles {
            if x.position.x < self.camera.viewport_width as f32 &&
            x.position.y < self.camera.viewport_height as f32 &&
            x.position.x > (self.camera.viewport_width as f32) - (self.camera.viewport_width as f32) &&
            x.position.y > (self.camera.viewport_height as f32) - (self.camera.viewport_height as f32) {
                graphics::draw(ctx, &x.texture, x.position);
            }
        }

        for x in &mut self.player_attack_instances {
            graphics::draw(ctx, &x.animation, x.position);
            x.animation.advance(ctx);
        }

        if self.player.alive == true {
            graphics::draw(ctx, &self.player.animation, self.player.position);
        }

        for x in &mut self.enemy_instances {
            graphics::draw(ctx, &x.animation, x.position);
            x.animation.advance(ctx);
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
