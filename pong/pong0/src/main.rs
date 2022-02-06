extern crate sfml;

use sfml::system::Vector2f;
use sfml::window::{ContextSettings, VideoMode, Key, Event, Style};
use sfml::graphics::{RenderWindow, RenderTarget, CircleShape, Color, Transformable, Shape};

fn main() {
    
    let mut window = RenderWindow::new(
        (800, 600),
        "SFML Example",
        Style::default(),
        &Default::default(),
        //&ContextSettings::default()
    );

    // Create a CircleShape
    let mut circle = CircleShape::new(30., 30);
    // circle.set_radius(30.);
    circle.set_fill_color(Color::RED);
    circle.set_position(Vector2f::new(100., 100.));


    while window.is_open() {

         // Clear the window
        window.clear(Color::GREEN);
        // Draw the shape
        window.draw(&circle);
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