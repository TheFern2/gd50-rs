extern crate sfml;

use sfml::graphics::{
    CircleShape, Color, Font, RectangleShape, RenderTarget, RenderWindow, Shape, Text,
    Transformable,
};
use sfml::system::{Clock, Vector2f};
use sfml::window::{Event, Key, Style};

/*
* Pong 3: The Paddle Update
*/
fn main() {
    const WINDOW_WIDTH: u32 = 1280;
    const WINDOW_HEIGHT: u32 = 720;
    let mut paddle_speed = 200.;

    let mut player_1_score = 0;
    let mut player_2_score = 0;

    let mut player_1_y = 20;
    let mut player_2_y = WINDOW_HEIGHT - 120;

    let mut window = RenderWindow::new(
        (WINDOW_WIDTH, WINDOW_HEIGHT),
        "Pong 3",
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
    welcome.set_position((WINDOW_WIDTH as f32 / 2. - 100., 40.));

    // Scores
    let mut player_1_score_text = Text::default();
    player_1_score_text.set_font(&font);
    player_1_score_text.set_fill_color(Color::WHITE);
    player_1_score_text.set_character_size(80);
    player_1_score_text.set_string("0");
    player_1_score_text.set_position((WINDOW_WIDTH as f32 / 2. - 120., 60.));

    let mut player_2_score_text = Text::default();
    player_2_score_text.set_font(&font);
    player_2_score_text.set_fill_color(Color::WHITE);
    player_2_score_text.set_character_size(80);
    player_2_score_text.set_string("0");
    player_2_score_text.set_position((WINDOW_WIDTH as f32 / 2. + 60., 60.));

    let paddle_size = Vector2f::new(25., 100.);
    let ball_size = Vector2f::new(20., 20.);

    // Create the right paddle
    let mut left_paddle = RectangleShape::new();
    left_paddle.set_size(paddle_size - 3.);
    left_paddle.set_fill_color(Color::WHITE);
    left_paddle.set_position((10., 20.));

    // Create the left paddle
    let mut right_paddle = RectangleShape::new();
    right_paddle.set_size(paddle_size - 3.);
    right_paddle.set_fill_color(Color::WHITE);
    right_paddle.set_position((WINDOW_WIDTH as f32 - 30., WINDOW_HEIGHT as f32 - 120.));

    // Create the ball
    let mut ball = RectangleShape::new();
    ball.set_size(ball_size - 3.);
    ball.set_fill_color(Color::WHITE);
    ball.set_position((
        WINDOW_WIDTH as f32 / 2. - 2.,
        WINDOW_HEIGHT as f32 / 2. - 2.,
    ));
    let mut clock = Clock::start();
    let mut is_playing = false;
    let mut up1 = false;
    let mut down1 = false;
    let mut up2 = false;
    let mut down2 = false;

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed
                | Event::KeyPressed {
                    code: Key::ESCAPE, ..
                } => window.close(),
                Event::KeyPressed { code: Key::M, .. } => println!("M pressed"),
                Event::KeyPressed {
                    code: Key::SPACE, ..
                } if !is_playing => {
                    is_playing = true;
                    clock.restart();
                }
                Event::KeyPressed { code: Key::W, .. } => up1 = true,
                Event::KeyReleased { code: Key::W, .. } => up1 = false,
                Event::KeyPressed { code: Key::S, .. } => down1 = true,
                Event::KeyReleased { code: Key::S, .. } => down1 = false,
                Event::KeyPressed { code: Key::UP, .. } => up2 = true,
                Event::KeyReleased { code: Key::UP, .. } => up2 = false,
                Event::KeyPressed {
                    code: Key::DOWN, ..
                } => down2 = true,
                Event::KeyReleased {
                    code: Key::DOWN, ..
                } => down2 = false,
                _ => { /* Do nothing */ }
            }
        }
        if is_playing {
            let delta_time = clock.restart().as_seconds();

            // left paddle
            if up1 && (left_paddle.position().y - 3. > 5.) {
                left_paddle.move_((0., -paddle_speed * delta_time));
            }

            if down1 && (left_paddle.position().y + paddle_size.y < WINDOW_HEIGHT as f32 - 5.) {
                left_paddle.move_((0., paddle_speed * delta_time));
            }

            // right paddle
            if up2 && (right_paddle.position().y - 3. > 5.) {
                right_paddle.move_((0., -paddle_speed * delta_time));
            }

            if down2 && (right_paddle.position().y + paddle_size.y < WINDOW_HEIGHT as f32 - 5.) {
                right_paddle.move_((0., paddle_speed * delta_time));
            }
        }
        // Clear the window
        window.clear(Color::BLACK);
        // Draw the shape
        window.draw(&welcome);
        window.draw(&player_1_score_text);
        window.draw(&player_2_score_text);
        window.draw(&right_paddle);
        window.draw(&left_paddle);
        window.draw(&ball);
        // Display things on screen
        window.display();
    }
}
