use bracket_lib::prelude::*;

const SCREEN_WIDTH: i32 = 40;
const SCREEN_HEIGHT: i32 = 25;

const DRAGON_FRAMES: [u16; 6] = [64, 1, 2, 3, 2, 1];

struct Player {
    x: i32,
    y: f32,
    velocity: f32,
    frame: usize,
}

impl Player {
    fn new(x: i32, y: i32) -> Self {
        Player {
            x,
            y: y as f32,
            velocity: 0.0,
            frame: 0,
        }
    }

    fn apply_gravity_and_move(&mut self) {
        if self.velocity < 2.0 {
            self.velocity += 0.1;
        }

        self.y += self.velocity;

        if self.y < 0.0 {
            self.y = 0.0;
        }

        self.x += 1;
        self.frame = (self.frame + 1) % 6;
    }

    fn flap(&mut self) {
        self.velocity = -2.0;
    }

    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(1);
        ctx.cls();
        ctx.set_fancy(
            PointF::new(0.0, self.y),
            1,
            Degrees::new(0.0),
            PointF::new(2.0, 2.0),
            WHITE,
            NAVY,
            DRAGON_FRAMES[self.frame],
        );
        ctx.set_active_console(0);
    }
}

struct Obstacle {
    x: i32,
    y_gap: i32,
    size: i32,
}

impl Obstacle {
    fn new(x: i32, score: i32) -> Self {
        let mut random = RandomNumberGenerator::new();

        Obstacle {
            x,
            y_gap: random.range(5, 20),
            size: random.range(2, 10 - score),
        }
    }

    fn render(&mut self, ctx: &mut BTerm, player_pos_x: i32) {
        // Ground
        for x in 0..SCREEN_WIDTH {
            ctx.set(x, SCREEN_HEIGHT - 1, WHITE, WHITE, to_cp437('#'));
        }

        let screen_pos_x = self.x - player_pos_x;
        let size_half = self.size / 2;

        // Top wall
        for y in 0..self.y_gap - size_half {
            ctx.set(screen_pos_x, y, WHITE, NAVY, 179);
        }

        // Bottom wall
        for y in self.y_gap + size_half..SCREEN_HEIGHT - 1 {
            ctx.set(screen_pos_x, y, WHITE, NAVY, 179);
        }
    }

    fn is_hit(&self, player: &Player) -> bool {
        let size_half = self.size / 2;

        player.x == self.x
            && ((player.y as i32) < self.y_gap - size_half
                || (player.y as i32) > self.y_gap + size_half)
    }
}

enum GameMode {
    Menu,
    Playing,
    End,
}

struct State {
    mode: GameMode,
    score: i32,
}

impl State {
    fn new() -> Self {
        State {
            mode: GameMode::Menu,
            score: 0,
        }
    }

    fn menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_color_centered(5, YELLOW, BLACK, "Welcome to Flappy Bird!");
        ctx.print_color_centered(8, CYAN, BLACK, "(P) Play Game");
        ctx.print_color_centered(9, CYAN, BLACK, "(Q) Quit Game");
    }

    fn end(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_color_centered(5, RED, BLACK, "You are dead!");
        ctx.print_centered(6, &format!("Your score: {}", self.score));
        ctx.print_color_centered(8, CYAN, BLACK, "(P) Play Again");
        ctx.print_color_centered(9, CYAN, BLACK, "(Q) Quit Game");
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.menu(ctx),
            GameMode::Playing => {}
            GameMode::End => self.end(ctx),
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_font("../res/flappy32.png", 32, 32)
        .with_simple_console(SCREEN_WIDTH, SCREEN_HEIGHT, "../res/flappy32.png")
        .with_fancy_console(SCREEN_WIDTH, SCREEN_HEIGHT, "../res/flappy32.png")
        .with_title("Flappy Bird")
        .with_tile_dimensions(16, 16)
        .build()?;

    main_loop(context, State::new())
}
