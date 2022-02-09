extern crate sfml;

use sfml::system::Vector2f;
use sfml::window::{ Key, Event, Style};
use sfml::graphics::{RenderWindow, RenderTarget, CircleShape, Color, Transformable, Shape, Text, Font};

fn main() {
    
    const WINDOW_WIDTH:u32 = 1280;
    const WINDOW_HEIGHT:u32 = 720;

    let mut window = RenderWindow::new(
        (WINDOW_WIDTH, WINDOW_HEIGHT),
        "Pong",
        Style::default(),
        &Default::default(),
    );

    // Create a CircleShape
    // let mut circle = CircleShape::new(30., 30);
    // circle.set_fill_color(Color::RED);
    // circle.set_position(Vector2f::new(100., 100.));

    let font = Font::from_file("res/font.ttf").unwrap();
    let mut welcome = Text::default();
    welcome.set_font(&font);
    welcome.set_fill_color(Color::WHITE);
    welcome.set_character_size(40);
    welcome.set_string("Hello Pong!");
    welcome.set_position((WINDOW_WIDTH as f32 / 2. -100., WINDOW_HEIGHT as f32/ 2. -40. ));

    
    while window.is_open() {

         // Clear the window
        window.clear(Color::BLACK);
        // Draw the shape
        window.draw(&welcome);
        // Display things on screen
        window.display();
        // }
        while let Some(event) = window.wait_event() {
            match event {
                Event::Closed | Event::KeyPressed { code: Key::ESCAPE, .. } => window.close(),
                Event::KeyPressed {code: Key::M, ..} => println!("M pressed"),
                _ => { /* Do nothing */ }
            }          
        }        
    }
}