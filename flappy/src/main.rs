use bracket_lib::prelude::*;

enum GameMode {
    Menu,
    Playing,
    End,
}

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 75.0;

struct State {
    player: Player,
    frame_time: f32,
    obstacle: Obstacle,
    mode: GameMode,
    score: i32,
    walls: Walls,
}

struct Player {
    x: f32,
    y: f32,
    velocity: f32,
}

struct Obstacle {
    x: i32,
    gap_y: i32,
    size: i32,
}

struct Walls;

impl Walls {
    fn new() -> Self {
        Walls
    }

    fn render(&mut self, ctx: &mut BTerm) {
        for x in 0..SCREEN_WIDTH {
            ctx.set(x, SCREEN_HEIGHT - 1, RED, BLACK, to_cp437('='));
        }

        for x in 0..SCREEN_WIDTH {
            ctx.set(x, 0, RED, BLACK, to_cp437('='));
        }
    }
}

impl State {
    fn new() -> Self {
        State {
            player: Player::new(5.0, 25.0),
            frame_time: 0.0,
            obstacle: Obstacle::new(SCREEN_WIDTH, 0),
            mode: GameMode::Menu,
            score: 0,
            walls: Walls::new(),
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY);
        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.gravity_and_move();
        }
        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }
        self.walls.render(ctx);
        self.player.render(ctx);
        ctx.print(0, 0, "Press SPACE to flap.");
        ctx.print(0, 1, &format!("Score: {}", self.score));

        self.obstacle.render(ctx, self.player.x);
        if self.player.x as i32 > self.obstacle.x {
            self.score += 1;
            self.obstacle = Obstacle::new(self.player.x as i32 + SCREEN_WIDTH, self.score);
        }
        if self.player.y as i32 > SCREEN_HEIGHT || self.obstacle.hit_obstacle(&self.player) {
            self.mode = GameMode::End;
        }
    }

    fn restart(&mut self) {
        self.player = Player::new(5.0, 25.0);
        self.frame_time = 0.0;
        self.obstacle = Obstacle::new(SCREEN_WIDTH, 0);
        self.mode = GameMode::Playing;
        self.score = 0;
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Welcome to Flappy Dragon");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }
    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "You are dead!");
        ctx.print_centered(6, &format!("You earned {} points", self.score));
        ctx.print_centered(8, "(P) Play Again");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }
}

impl Player {
    fn new(x: f32, y: f32) -> Self {
        Player {
            x,
            y,
            velocity: 0.0,
        }
    }

    fn render(&mut self, ctx: &mut BTerm) {
        // ctx.set(0, self.y, YELLOW, BLACK, to_cp437('@'));
        ctx.set_active_console(1);
        ctx.cls();
        ctx.set_fancy(
            PointF { x: 0.0, y: self.y },
            0,
            Radians::new(0.0),
            PointF { x: 4.0, y: 4.0 },
            YELLOW,
            BLACK,
            to_cp437('@'),
        );
        ctx.set_active_console(0);
    }

    fn gravity_and_move(&mut self) {
        if self.velocity < 2.0 {
            self.velocity += 0.2;
        }
        self.y += self.velocity;
        self.x += 1.0;
        if self.y < 0.0 {
            self.y = 0.0;
        }
    }

    fn flap(&mut self) {
        self.velocity = -2.0
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::End => self.dead(ctx),
            GameMode::Playing => self.play(ctx),
        }
    }
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
    fn render(&mut self, ctx: &mut BTerm, player_x: f32) {
        let screen_x = self.x - player_x as i32;
        let half_size = self.size / 2;

        for y in 0..self.gap_y - half_size {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }

        for y in self.gap_y + half_size..SCREEN_HEIGHT {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }
    }
    fn hit_obstacle(&self, player: &Player) -> bool {
        let half_size = self.size / 2;
        let does_x_match = player.x as i32 == self.x;
        let player_above_gap = player.y < (self.gap_y - half_size) as f32;
        let player_below_gap = player.y > (self.gap_y + half_size) as f32;
        does_x_match && (player_above_gap || player_below_gap)
    }
}

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_font("../resources/flappy32.png", 32, 32)
        .with_simple_console(SCREEN_WIDTH, SCREEN_HEIGHT, "../resources/flappy32.png")
        .with_fancy_console(SCREEN_WIDTH, SCREEN_HEIGHT, "../resources/flappy32.png")
        .with_title("Flappy Dragon Enhanced")
        .with_tile_dimensions(16, 16)
        .build()?;
    main_loop(context, State::new())
}
