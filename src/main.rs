use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::time::Duration;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

const GRID_X: i32 = 40;
const GRIX_Y: i32 = 40;
const DOT_SIZE: u32 = 20;

struct Point {
    x: i32,
    y: i32,
}

enum GameState {
    Playing,
    Paused,
}

enum PlayerDirection {
    Up,
    Down,
    Right,
    Left,
}

struct GameContext {
    player_position: Vec<Point>,
    player_direction: PlayerDirection,
    state: GameState,
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Demo", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut points: Vec<Point> = Vec::new();

    initialize_points(&mut points);

    let mut player = GameContext {
        player_direction: PlayerDirection::Down,
        player_position: vec![Point { x: 5, y: 5 }, Point { x: 4, y: 5 }],
        state: GameState::Playing,
    };

    let mut i = 0;
    'running: loop {
        i += 1;
        if i % 5 == 0 {
            i = 0;
            for point in &mut player.player_position {
                point.x += 1;
            }
        }
        canvas.clear();
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        for point in &points {
            canvas
                .draw_rect(Rect::new(point.x * 20, point.y * 20, DOT_SIZE, DOT_SIZE))
                .unwrap();
        }

        draw_grid(&player, &mut canvas);

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        canvas.present();
    }
}

fn initialize_points(points: &mut Vec<Point>) {
    for i in 0..GRID_X {
        for j in 0..GRIX_Y {
            points.push(Point { x: i, y: j });
        }
    }
}

fn draw_grid(player: &GameContext, canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGB(255, 0, 0));
    for point in &player.player_position {
        canvas
            .fill_rect(Rect::new(point.x * 20, point.y * 20, DOT_SIZE, DOT_SIZE))
            .unwrap();
    }
}
