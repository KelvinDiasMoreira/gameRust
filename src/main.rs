use rand::Rng;
use sdl2::{
    event::Event,
    keyboard,
    pixels::Color,
    rect::{Point, Rect},
    render::Canvas,
    sys::rand,
    video::Window,
};

use std::time::Duration;

extern crate sdl2;

#[derive(Debug)]
struct Rectangle2d {
    position_x: i32,
    position_y: i32,
    width: u32,
    height: u32,
    color: Color,
    is_bot: bool,
}
#[derive(Debug)]
struct Point2d {
    position_x: i32,
    position_y: i32,
    color: Color,
    initial_position: i32,
    is_increase_x: bool,
    is_increase_y: bool,
}

impl Rectangle2d {
    fn movement_bot(&mut self, point_pos: &Point2d) {
        if self.is_bot {
            if self.position_y > 0 {
                self.position_y -= SPEED_RECTANGLES;
            }
            if self.position_y != SCREEN_HEIGHT as i32 - self.height as i32 {
                self.position_y += SPEED_RECTANGLES;
            }
            self.position_y = point_pos.position_y - (self.height / 2) as i32
        }
    }
}

impl Point2d {
    fn movement(&mut self, speed_point_x: i32, speed_point_y: i32) {
        if self.is_increase_x {
            self.position_x += speed_point_x;
        } else {
            self.position_x -= speed_point_x;
        }
        if self.position_y < SCREEN_HEIGHT as i32 && self.is_increase_y == true {
            if self.position_y >= (SCREEN_HEIGHT - speed_point_y as u32) as i32 {
                self.is_increase_y = false;
            } else {
                self.position_y += speed_point_y;
            }
        } else {
            if self.position_y <= (0 + speed_point_y) && !self.is_increase_y {
                self.is_increase_y = true;
            } else {
                self.position_y -= speed_point_y;
            }
        }
    }
    fn check_colision(&mut self, player1_rectangle: &Rectangle2d, player2_rectangle: &Rectangle2d) {
        if self.position_x == player1_rectangle.position_x + player1_rectangle.width as i32
            && self.position_y >= player1_rectangle.position_y
            && self.position_y <= player1_rectangle.position_y + player1_rectangle.height as i32
        {
            self.is_increase_x = true;
        }
        if self.position_x == player2_rectangle.position_x - player2_rectangle.width as i32
            && self.position_y >= player2_rectangle.position_y
            && self.position_y <= player2_rectangle.position_y + player2_rectangle.height as i32
        {
            self.is_increase_x = false;
        }
    }
}

const SPEED_RECTANGLES: i32 = 10;
const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

fn main() {
    let speed_point_y: i32 = rand::thread_rng().gen_range(1..15);
    let speed_point_x: i32 = 10;
    let mut game_is_running = true;
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Impossivel", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .expect("failed to open");

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut first_rectangle = build_rectangle(10, 100, 0, Color::RGB(255, 255, 255), false);
    let mut second_rectangle = build_rectangle(
        10,
        100,
        SCREEN_WIDTH as i32 - 10,
        Color::RGB(255, 255, 255),
        true,
    );
    let mut point = build_point(
        SCREEN_WIDTH as i32 / 2,
        SCREEN_HEIGHT as i32 / 2,
        Color::RGB(255, 255, 255),
        SCREEN_WIDTH as i32 / 2,
    );
    while game_is_running {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    game_is_running = false;
                    break;
                }
                _ => {}
            }
        }

        handle_keyboard(
            event_pump.keyboard_state(),
            &mut first_rectangle,
            &mut second_rectangle,
        );
        update(
            &mut canvas,
            &first_rectangle,
            &mut point,
            &mut second_rectangle,
            speed_point_x,
            speed_point_y,
        );
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn handle_keyboard(
    keyboard_state: keyboard::KeyboardState,
    first_rectangle: &mut Rectangle2d,
    _second_rectangle: &mut Rectangle2d,
) {
    if keyboard_state.is_scancode_pressed(keyboard::Scancode::W) {
        if first_rectangle.position_y > 0 {
            first_rectangle.position_y -= SPEED_RECTANGLES;
        }
    }
    if keyboard_state.is_scancode_pressed(keyboard::Scancode::S) {
        if first_rectangle.position_y != SCREEN_HEIGHT as i32 - first_rectangle.height as i32 {
            first_rectangle.position_y += SPEED_RECTANGLES;
        }
    }
}

fn update(
    canvas: &mut Canvas<Window>,
    player1_rectangle: &Rectangle2d,
    point: &mut Point2d,
    player2_rectangle: &mut Rectangle2d,
    speed_point_x: i32,
    speed_point_y: i32,
) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    canvas.set_draw_color(point.color);
    let _ = canvas.draw_point(Point::new(point.position_x, point.position_y));

    canvas.set_draw_color(player1_rectangle.color);
    let _ = canvas.fill_rect(Rect::new(
        player1_rectangle.position_x,
        player1_rectangle.position_y,
        player1_rectangle.width,
        player1_rectangle.height,
    ));

    canvas.set_draw_color(player2_rectangle.color);
    let _ = canvas.fill_rect(Rect::new(
        player2_rectangle.position_x,
        player2_rectangle.position_y,
        player2_rectangle.width,
        player2_rectangle.height,
    ));

    update_position_point(
        point,
        player1_rectangle,
        player2_rectangle,
        speed_point_x,
        speed_point_y,
    );
    if player2_rectangle.is_bot {
        player2_rectangle.movement_bot(point);
    }
    canvas.present();
}

fn update_position_point(
    point: &mut Point2d,
    player1_rectangle: &Rectangle2d,
    player2_rectangle: &Rectangle2d,
    speed_point_x: i32,
    speed_point_y: i32,
) {
    point.movement(speed_point_x, speed_point_y);
    point.check_colision(player1_rectangle, player2_rectangle);
    // if point.position_x == 0 {
    //     point.position_x = point.initial_position
    // }
}

fn build_rectangle(
    width: u32,
    height: u32,
    position_x: i32,
    color: Color,
    is_bot: bool,
) -> Rectangle2d {
    Rectangle2d {
        position_x,
        position_y: (SCREEN_HEIGHT as i32 / 2) - height as i32 / 2,
        width,
        height,
        color,
        is_bot,
    }
}

fn build_point(position_x: i32, position_y: i32, color: Color, initial_position: i32) -> Point2d {
    Point2d {
        position_x,
        position_y,
        color,
        initial_position,
        is_increase_x: false,
        is_increase_y: if rand::thread_rng().gen_range(0..1000) > 500 {
            false
        } else {
            true
        },
    }
}
