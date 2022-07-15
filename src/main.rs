use bracket_lib::prelude::*;

const SCREEN_WIDTH: i32 = 40;
const SCREEN_HEIGHT: i32 = 25;

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
}

struct State {}

impl State {
    fn new() -> Self {
        State {}
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {}
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
