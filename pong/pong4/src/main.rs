extern crate sfml;

use rand::{thread_rng, Rng};
use sfml::graphics::{
    CircleShape, Color, Font, RectangleShape, RenderTarget, RenderWindow, Shape, Text,
    Transformable,
};
use sfml::system::{Clock, SfStrConv, Vector2f};
use sfml::window::{Event, Key, Style};
use std::{env, f32::consts::PI};

/*
* Pong 4: The ball update.
*/
fn main() {
    let mut rng = thread_rng();

    const WINDOW_WIDTH: u32 = 1280;
    const WINDOW_HEIGHT: u32 = 720;
    let mut paddle_speed = 300.;

    let mut player_1_score = 0;
    let mut player_2_score = 0;

    let mut player_1_y = 20;
    let mut player_2_y = WINDOW_HEIGHT - 120;

    let mut window = RenderWindow::new(
        (WINDOW_WIDTH, WINDOW_HEIGHT),
        "Pong 4",
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
    player_1_score_text.set_string(&format!("{}", player_1_score));
    player_1_score_text.set_position((WINDOW_WIDTH as f32 / 2. - 120., 60.));

    let mut player_2_score_text = Text::default();
    player_2_score_text.set_font(&font);
    player_2_score_text.set_fill_color(Color::WHITE);
    player_2_score_text.set_character_size(80);
    player_2_score_text.set_string(&format!("{}", player_2_score));
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
    ball.set_size(ball_size);
    ball.set_fill_color(Color::WHITE);
    ball.set_position((
        WINDOW_WIDTH as f32 / 2. - 2.,
        WINDOW_HEIGHT as f32 / 2. - 2.,
    ));

    let mut ball_speed = 400.;
    let mut ball_angle = 0.;

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
                Event::KeyPressed { code: Key::R, .. } => is_playing = false,
                Event::KeyPressed {
                    code: Key::SPACE, ..
                } if !is_playing => {
                    is_playing = true;
                    ball_speed = 400.;
                    clock.restart();

                    ball.set_position((
                        WINDOW_WIDTH as f32 / 2. - 2.,
                        WINDOW_HEIGHT as f32 / 2. - 2.,
                    ));

                    // Reset the ball angle
                    loop {
                        // Make sure the ball initial angle is not too much vertical
                        ball_angle = rng.gen_range(0.0..360.) * 2. * PI / 360.;

                        if ball_angle.cos().abs() >= 0.7 {
                            break;
                        }
                    }
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

            // Move the ball
            let factor = ball_speed * delta_time;
            ball.move_((ball_angle.cos() * factor, ball_angle.sin() * factor));

            // Check collissions between the ball and the screen
            // detects screen collision on left wall
            if ball.position().x - ball_size.x + 30. < 0. {
                is_playing = false;
                player_2_score += 1;
                player_2_score_text.set_string(&format!("{}", player_2_score));
                // p1 you lost
            }

            // detetcts screen collision on right wall
            if ball.position().x + ball_size.x - 15. > WINDOW_WIDTH as f32 {
                is_playing = false;
                player_1_score += 1;
                player_1_score_text.set_string(&format!("{}", player_1_score));
                // p1 you won
            }

            // detects screen collision on top wall
            if ball.position().y - ball_size.x < -15. {
                on_bounce(&mut ball_speed);
                ball_angle = -ball_angle;
                let p = ball.position().x;
                ball.set_position((p, ball_size.x + 0.1))
            }

            // detects screen collision on bottom wall
            if ball.position().y + ball_size.x > WINDOW_HEIGHT as f32 {
                on_bounce(&mut ball_speed);
                ball_angle = -ball_angle;
                let p = ball.position().x;
                ball.set_position((p, WINDOW_HEIGHT as f32 - ball_size.x - 0.1))
            }

            // Check the collisions between the ball and the paddles
            // left paddle
            let (ball_pos, paddle_pos) = (ball.position(), left_paddle.position());
            if ball_pos.x - ball_size.x < paddle_pos.x + paddle_size.x / 2.
                && ball_pos.y + ball_size.x >= paddle_pos.y - paddle_size.y / 2.
                && ball_pos.y - ball_size.x <= paddle_pos.y + paddle_size.y / 2.
            {
                if ball_pos.y > paddle_pos.y {
                    ball_angle = PI - ball_angle + rng.gen_range(0.0..20.) * PI / 180.;
                } else {
                    ball_angle = PI - ball_angle - rng.gen_range(0.0..20.) * PI / 180.;
                }

                on_bounce(&mut ball_speed);
                ball.set_position((
                    paddle_pos.x + ball_size.x + paddle_size.x / 2. + 0.1,
                    ball_pos.y,
                ))
            }

            // right paddle
            let (ball_pos, paddle_pos) = (ball.position(), right_paddle.position());
            if ball_pos.x + ball_size.x > paddle_pos.x - paddle_size.x / 2.
                && ball_pos.y + ball_size.x >= paddle_pos.y - paddle_size.y / 2.
                && ball_pos.y - ball_size.x <= paddle_pos.y + paddle_size.y / 2.
            {
                if ball_pos.y > paddle_pos.y {
                    ball_angle = PI - ball_angle + rng.gen_range(0.0..20.) * PI / 180.;
                } else {
                    ball_angle = PI - ball_angle - rng.gen_range(0.0..20.) * PI / 180.;
                }

                on_bounce(&mut ball_speed);
                ball.set_position((
                    paddle_pos.x - ball_size.x - paddle_size.x / 2. + 0.1,
                    ball_pos.y,
                ))
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

fn on_bounce(ball_speed: &mut f32) {
    *ball_speed += 16.0;
}
