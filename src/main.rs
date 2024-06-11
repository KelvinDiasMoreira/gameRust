use std::time::Duration;

use sdl2::{
    event::Event,
    keyboard,
    pixels::Color,
    rect::{Point, Rect},
    render::Canvas,
    video::Window,
};

extern crate sdl2;

#[derive(Debug)]
struct Rectangle2d {
    position_x: i32,
    position_y: i32,
    width: u32,
    height: u32,
    color: Color,
}
#[derive(Debug)]
struct Point2d {
    position_x: i32,
    position_y: i32,
    color: Color,
    // initial_position: i32,
    is_increase_x: bool,
    is_increase_y: bool,
}

impl Point2d {
    fn movement(&mut self) {
        if self.is_increase_x {
            self.position_x += SPEED_POINT_X;
        } else {
            self.position_x -= SPEED_POINT_X;
        }
        if self.position_y < SCREEN_HEIGHT as i32 && self.is_increase_y {
            if self.position_y == (SCREEN_HEIGHT - SPEED_POINT_Y as u32) as i32 {
                self.is_increase_y = false;
            } else {
                self.position_y += SPEED_POINT_Y;
            }
        } else {
            if self.position_y < (0 + SPEED_POINT_Y) && !self.is_increase_y {
                self.is_increase_y = true;
            } else {
                self.position_y -= SPEED_POINT_Y;
            }
        }
    }
}
const SPEED_POINT_Y: i32 = 10;
const SPEED_POINT_X: i32 = 10;
const SPEED_RECTANGLES: i32 = 10;
const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

fn main() {
    let mut game_is_running = true;

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Bora bora", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .expect("failed to open");

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut first_rectangle = build_rectangle(10, 100, 0, Color::RGB(255, 255, 255));
    let second_rectangle =
        build_rectangle(10, 100, SCREEN_WIDTH as i32 - 10, Color::RGB(255, 255, 255));
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
                Event::KeyDown { keycode, .. } => match keycode {
                    Some(code) => {
                        if code == keyboard::Keycode::W {
                            if first_rectangle.position_y > 0 {
                                first_rectangle.position_y -= SPEED_RECTANGLES;
                            }
                        }
                        if code == keyboard::Keycode::S {
                            if first_rectangle.position_y
                                == SCREEN_HEIGHT as i32 - first_rectangle.height as i32
                            {
                                continue;
                            }

                            first_rectangle.position_y += SPEED_RECTANGLES;
                        }
                        // if code == keyboard::Keycode::Up {
                        //     if second_rectangle.position_y > 0 {
                        //         second_rectangle.position_y -= 10;
                        //     }
                        // }
                        // if code == keyboard::Keycode::Down {
                        //     if second_rectangle.position_y
                        //         == SCREEN_HEIGHT as i32 - second_rectangle.height as i32
                        //     {
                        //         continue;
                        //     }
                        //     second_rectangle.position_y += 10;
                        // }
                    }
                    None => todo!(),
                },
                _ => {}
            }
        }

        update(&mut canvas, &first_rectangle, &mut point, &second_rectangle);
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 20));
    }
}

fn update(
    canvas: &mut Canvas<Window>,
    player1_rectangle: &Rectangle2d,
    point: &mut Point2d,
    player2_rectangle: &Rectangle2d,
) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    canvas.set_draw_color(point.color);
    let _ = canvas.draw_point(Point::new(point.position_x, point.position_y));

    canvas.set_draw_color(player1_rectangle.color);
    let _ = canvas.draw_rect(Rect::new(
        player1_rectangle.position_x,
        player1_rectangle.position_y,
        player1_rectangle.width,
        player1_rectangle.height,
    ));

    canvas.set_draw_color(player2_rectangle.color);
    let _ = canvas.draw_rect(Rect::new(
        player2_rectangle.position_x,
        player2_rectangle.position_y,
        player2_rectangle.width,
        player2_rectangle.height,
    ));

    update_position_point(point, player1_rectangle, player2_rectangle);

    canvas.present();
}

fn update_position_point(
    point: &mut Point2d,
    player1_rectangle: &Rectangle2d,
    player2_rectangle: &Rectangle2d,
) {
    point.movement();

    check_colision(point, player1_rectangle, player2_rectangle);
    // if point.position_x == 0 {
    //     point.position_x = point.initial_position
    // }
}

fn check_colision(
    point: &mut Point2d,
    player1_rectangle: &Rectangle2d,
    player2_rectangle: &Rectangle2d,
) {
    println!("point: {:?}", point);
    // println!("rectangle: {:?}", player1_rectangle);
    if point.position_x == player1_rectangle.position_x + player1_rectangle.width as i32
        && point.position_y >= player1_rectangle.position_y
        && point.position_y <= player1_rectangle.position_y + player1_rectangle.height as i32
    {
        point.is_increase_x = true;
    }
    if point.position_x == player2_rectangle.position_x
        && point.position_y >= player2_rectangle.position_y
        && point.position_y <= player2_rectangle.position_y + player2_rectangle.height as i32
    {
        point.is_increase_x = false;
    }
}

fn build_rectangle(width: u32, height: u32, position_x: i32, color: Color) -> Rectangle2d {
    Rectangle2d {
        position_x,
        position_y: (SCREEN_HEIGHT as i32 / 2) - height as i32 / 2,
        width,
        height,
        color,
    }
}

fn build_point(position_x: i32, position_y: i32, color: Color, _initial_position: i32) -> Point2d {
    Point2d {
        position_x,
        position_y,
        color,
        // initial_position,
        is_increase_x: false,
        is_increase_y: true,
    }
}
