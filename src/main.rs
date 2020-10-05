extern crate ggez;

use ggez::conf;
use ggez::error::GameResult;
use ggez::event::{self, EventHandler};
use ggez::graphics;
use ggez::{Context, ContextBuilder};

const APP_NAME: &str = "Pong";
const APP_AUTHOR: &str = "Sufien Tout";

const APP_WIDTH: f32 = 800.0;
const APP_HEIGHT: f32 = 600.0;

struct MainState {}

impl MainState {
    pub fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let s = MainState {};
        Ok(s)
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
        graphics::present(ctx)
    }
}

fn main() -> GameResult<()> {
    let mut config = conf::Conf::new();
    config.window_setup.title = APP_NAME.to_string();
    config.window_mode.width = APP_WIDTH;
    config.window_mode.height = APP_HEIGHT;
    let (mut ctx, mut event_loop) = ContextBuilder::new(APP_NAME, APP_AUTHOR)
        .conf(config)
        .build()
        .expect("Context and EventLoop construction failed.");
    let state = &mut MainState::new(&mut ctx).expect("MainState construction failed.");
    event::run(&mut ctx, &mut event_loop, state)
}
