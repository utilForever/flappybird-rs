use bracket_lib::prelude::*;

const SCREEN_WIDTH: i32 = 40;
const SCREEN_HEIGHT: i32 = 25;

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
