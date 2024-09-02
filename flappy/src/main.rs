#![allow(unused)]
use std::fmt::format;

use bracket_lib::prelude::*;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 75.0;

struct Obstacle {
    x: i32,
    gap_y: i32,
    size: i32,
}
impl Obstacle {
    fn new(x: i32, score: i32) -> Self {
        let mut random = RandomNumberGenerator::new();
        Obstacle {
            x,
            gap_y: random.range(10, 40),
            size: i32::max(2, 20 - score),
        }
    }
    //渲染
    fn render(&mut self, ctx: &mut BTerm, player_x: i32) {
        let screen_x = self.x - player_x;
        let half_size = self.size / 2;

        for y in 0..(self.gap_y - half_size) {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('#'));
        }

        for y in (self.gap_y + half_size)..SCREEN_HEIGHT {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('#'));
        }
    }

    fn hit_obstacle(&self, player: &Player) -> bool {
        let half_size = self.size / 2;
        let does_x_match = player.x == self.x;
        let player_above_gap = player.y < self.gap_y - half_size;
        let player_below_gap = player.y > self.gap_y + half_size;
        println!(
            "here[{}]: half_size:{},player.x:{},self.x:{},player.y:{},self.gap_y:{}",
            line!(),
            half_size,
            player.x,
            self.x,
            player.y,
            self.gap_y
        );
        does_x_match && (player_above_gap || player_below_gap)
    }
}
enum GameMode {
    Menu,
    Playing,
    End,
}

struct Player {
    x: i32,
    y: i32,
    velocity: f32,
    screen_x_max: i32,
    screen_y_max: i32,
}

impl Player {
    fn new(x: i32, y: i32) -> Self {
        Player {
            x,
            y,
            velocity: 0.0,
            screen_x_max: SCREEN_WIDTH,
            screen_y_max: SCREEN_HEIGHT,
        }
    }

    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(0, self.y, YELLOW, BLACK, to_cp437('@'));
    }

    fn gravity_and_move(&mut self) {
        if self.velocity < 2.0 {
            self.velocity += 0.2;
        }
        self.y += self.velocity as i32;
        if self.y < 0 {
            self.y = 0;
        } else if self.y > (self.screen_y_max - 1) {
            self.y = self.screen_y_max - 1;
        }
        self.x += 1;
    }

    fn flap(&mut self) {
        self.velocity = -2.0; //原点坐标是0，0。所以减是往上飞。
    }
}

struct State {
    player: Player,
    frame_time: f32,
    obstacle1: Obstacle,
    obstacle2: Obstacle,
    obstacle3: Obstacle,
    mode: GameMode,
    score: i32,
}
impl State {
    fn new() -> Self {
        State {
            player: Player::new(5, 25),
            frame_time: 0.0,
            obstacle1: Obstacle::new(SCREEN_WIDTH, 0),
            obstacle2: Obstacle::new(SCREEN_WIDTH - 15, 0),
            obstacle3: Obstacle::new(SCREEN_WIDTH - 30, 0),
            mode: GameMode::Menu,
            score: 0,
        }
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Welcome to Flappy Dragon");
        ctx.print_centered(8, "Press (P) to start");
        ctx.print_centered(9, "Press (Q) to quit");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q | VirtualKeyCode::Escape => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY);
        self.frame_time += ctx.frame_time_ms;
        if self.frame_time >= FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.gravity_and_move();
        }

        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }
        self.player.render(ctx);
        ctx.print(0, 0, "Press SPACE to flap.");
        ctx.print(0, 1, &format!("Score: {}", self.score));
        self.obstacle1.render(ctx, self.player.x);
        if self.player.x > self.obstacle1.x {
            self.score += 1;
            self.obstacle1 = Obstacle::new(self.player.x + self.player.screen_x_max, self.score);
        }
        if self.player.y > self.player.screen_y_max || self.obstacle1.hit_obstacle(&self.player) {
            println!("here:{},gap_y:{}", line!(), self.obstacle1.gap_y);
            self.mode = GameMode::End;
        }

        self.obstacle2.render(ctx, self.player.x); //渲染
        if self.player.x > self.obstacle2.x {
            self.score += 1;
            self.obstacle2 = Obstacle::new(self.player.x + self.player.screen_x_max, self.score);
        }
        if self.player.y > self.player.screen_y_max || self.obstacle2.hit_obstacle(&self.player) {
            println!("here:{},gap_y:{}", line!(), self.obstacle2.gap_y);
            self.mode = GameMode::End;
        }

        self.obstacle3.render(ctx, self.player.x);
        if self.player.x > self.obstacle3.x {
            self.score += 1;
            self.obstacle3 = Obstacle::new(self.player.x + self.player.screen_x_max, self.score);
        }
        if self.player.y > self.player.screen_y_max || self.obstacle3.hit_obstacle(&self.player) {
            println!("here:{},gap_y:{}", line!(), self.obstacle3.gap_y);
            self.mode = GameMode::End;
        }

        if let Some(VirtualKeyCode::Escape) = ctx.key {
            self.mode = GameMode::Menu;
        }
    }

    fn dead(&mut self, ctx: &mut BTerm) {
        // ctx.cls();
        ctx.print_centered(5, "You are dead!");
        ctx.print_centered(6, &format!("You earned {} points", self.score));
        ctx.print_centered(9, "Press Q to quit");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn restart(&mut self) {
        self.player = Player::new(5, 25);
        self.frame_time = 0.0;
        self.obstacle1 = Obstacle::new(SCREEN_WIDTH, 0);
        self.obstacle2 = Obstacle::new(SCREEN_WIDTH - 15, 0);
        self.obstacle3 = Obstacle::new(SCREEN_WIDTH - 30, 0);
        self.mode = GameMode::Playing;
        self.score = 0;
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::Playing => self.play(ctx),
            GameMode::End => self.dead(ctx),
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;
    main_loop(context, State::new())
}
