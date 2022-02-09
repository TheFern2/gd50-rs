extern crate sfml;

use sfml::system::{Vector2f, Clock};
use sfml::window::{ Key, Event, Style};
use sfml::graphics::{RenderWindow, RenderTarget, RectangleShape, CircleShape, Color, Transformable, Shape, Text, Font};

fn main() {
    
    const WINDOW_WIDTH:u32 = 1280;
    const WINDOW_HEIGHT:u32 = 720;

    let mut window = RenderWindow::new(
        (WINDOW_WIDTH, WINDOW_HEIGHT),
        "Pong",
        Style::default(),
        &Default::default(),
    );

    window.set_framerate_limit(60);
    window.set_vertical_sync_enabled(true);

    let font = Font::from_file("res/font.ttf").unwrap();
    let mut welcome = Text::default();
    welcome.set_font(&font);
    welcome.set_fill_color(Color::WHITE);
    welcome.set_character_size(30);
    welcome.set_string("Hello Pong!");
    welcome.set_position((WINDOW_WIDTH as f32 / 2. -100., 40. ));

    // let mut clock = Clock::start();
    // let mut is_playing = false;

    // loop {
    //     while let Some(event) = window.poll_event() {
    //         match event {
    //             Event::Closed | Event::KeyPressed { code: Key::ESCAPE, .. } => window.close(),
    //             Event::KeyPressed {code: Key::M, ..} => println!("M pressed"),
    //             _ => { /* Do nothing */ }
    //         }          
    //     }

    //     if is_playing {
    //         let delta_time = clock.restart().as_seconds();
    //     }

    //     window.clear(Color::BLACK);
    //     window.display()
    // }

    let paddle_size = Vector2f::new(25., 100.);
    let ball_size = Vector2f::new(20., 20.);

    // Create the right paddle
    let mut right_paddle = RectangleShape::new();
    right_paddle.set_size(paddle_size - 3.);
    right_paddle.set_fill_color(Color::WHITE);
    right_paddle.set_position((10., 20.));

    // Create the left paddle
    let mut left_paddle = RectangleShape::new();
    left_paddle.set_size(paddle_size - 3.);
    left_paddle.set_fill_color(Color::WHITE);
    left_paddle.set_position((WINDOW_WIDTH as f32 - 30., WINDOW_HEIGHT as f32 - 120.));

    // Create the ball
    let mut ball = RectangleShape::new();
    ball.set_size(ball_size - 3.);
    ball.set_fill_color(Color::WHITE);
    ball.set_position((WINDOW_WIDTH as f32 / 2. - 2., WINDOW_HEIGHT as f32 / 2. - 2.));
    
    while window.is_open() {        
        
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed | Event::KeyPressed { code: Key::ESCAPE, .. } => window.close(),
                Event::KeyPressed {code: Key::M, ..} => println!("M pressed"),
                _ => { /* Do nothing */ }
            }          
        }        
        // Clear the window
        window.clear(Color::BLACK);
        // Draw the shape
        window.draw(&welcome);
        window.draw(&right_paddle);
        window.draw(&left_paddle);
        window.draw(&ball);
        // Display things on screen
        window.display();
    }
}