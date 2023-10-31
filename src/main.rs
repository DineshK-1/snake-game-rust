use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

const GRID_X: i32 = 20;
const GRID_Y: i32 = HEIGHT as i32 / DOT_SIZE as i32;
const DOT_SIZE: u32 = WIDTH / GRID_X as u32;

struct Point {
    x: i32,
    y: i32,
}

enum GameState {
    Playing,
    Paused,
}

#[derive(PartialEq)]
enum PlayerDirection {
    Up,
    Down,
    Right,
    Left,
}

struct GameContext {
    state: GameState,
    food_index: Food,
}

struct PlayerObject {
    player_position: Vec<Point>,
    player_direction: PlayerDirection,
}

impl PlayerObject {
    fn add_tail(&mut self) {
        let tail = &self.player_position[self.player_position.len() - 1];

        let mut x = tail.x;
        let mut y = tail.y;

        match self.player_direction {
            PlayerDirection::Down => y-=1,
            PlayerDirection::Up => y+=1,
            PlayerDirection::Left => x+=1,
            PlayerDirection::Right => x-=1,
        }
        self.player_position.push(Point {
            x: x,
            y: y,
        });
    }
}

struct Food {
    location: Point,
}

impl Food {
    fn collides_with_player(&self, player: &PlayerObject) -> bool {
        for player_point in &player.player_position {
            if player_point.x == self.location.x && player_point.y == self.location.y {
                return true;
            }
        }
        false
    }
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

    //Game Context
    let mut player = PlayerObject {
        player_direction: PlayerDirection::Down,
        player_position: vec![Point { x: 5, y: 5 }, Point { x: 4, y: 5 }],
    };

    let mut game = GameContext {
        state: GameState::Playing,
        food_index: generate_food(),
    };

    let mut i = 0;

    'running: loop {
        if game.food_index.collides_with_player(&player) {
            game.food_index = generate_food();
            player.add_tail();
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(0, 255, 0));
        canvas
            .fill_rect(Rect::new(
                game.food_index.location.x * DOT_SIZE as i32,
                game.food_index.location.y * DOT_SIZE as i32,
                DOT_SIZE,
                DOT_SIZE,
            ))
            .unwrap();

        i += 1;

        if i % 20 == 0 {
            player_movement(&mut player);
        }
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        for point in &points {
            canvas
                .draw_rect(Rect::new(
                    point.x * DOT_SIZE as i32,
                    point.y * DOT_SIZE as i32,
                    DOT_SIZE,
                    DOT_SIZE,
                ))
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
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => match keycode {
                    Keycode::Up => {
                        if player.player_direction != PlayerDirection::Down {
                            player.player_direction = PlayerDirection::Up;
                        }
                    }
                    Keycode::Down => {
                        if player.player_direction != PlayerDirection::Up {
                            player.player_direction = PlayerDirection::Down
                        }
                    }
                    Keycode::Left => {
                        if player.player_direction != PlayerDirection::Right {
                            player.player_direction = PlayerDirection::Left
                        }
                    }
                    Keycode::Right => {
                        if player.player_direction != PlayerDirection::Left {
                            player.player_direction = PlayerDirection::Right
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        canvas.present();
    }
}

fn generate_food() -> Food {
    let mut rng = rand::thread_rng();
    return Food {
        location: Point {
            x: rng.gen_range(0..=GRID_X-1),
            y: rng.gen_range(0..=GRID_Y-1),
        },
    };
}

fn initialize_points(points: &mut Vec<Point>) {
    for i in 0..GRID_X {
        for j in 0..GRID_Y {
            points.push(Point { x: i, y: j });
        }
    }
}

fn draw_grid(player: &PlayerObject, canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGB(255, 0, 0));
    for point in &player.player_position {
        canvas
            .fill_rect(Rect::new(
                point.x * DOT_SIZE as i32,
                point.y * DOT_SIZE as i32,
                DOT_SIZE,
                DOT_SIZE,
            ))
            .unwrap();
    }
}

fn player_movement(player: &mut PlayerObject) {
    let head = &player.player_position[0];

    match player.player_direction {
        PlayerDirection::Up => {
            player.player_position.insert(
                0,
                Point {
                    x: head.x,
                    y: head.y - 1,
                },
            );
        }
        PlayerDirection::Down => {
            player.player_position.insert(
                0,
                Point {
                    x: head.x,
                    y: head.y + 1,
                },
            );
        }
        PlayerDirection::Left => {
            player.player_position.insert(
                0,
                Point {
                    x: head.x - 1,
                    y: head.y,
                },
            );
        }
        PlayerDirection::Right => {
            player.player_position.insert(
                0,
                Point {
                    x: head.x + 1,
                    y: head.y,
                },
            );
        }
    }

    player.player_position.pop();
}
