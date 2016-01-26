extern crate sdl2;

use sdl2::event::{Event};
use sdl2::rect::{Rect};

pub mod viewport;

use viewport::Viewport;

fn main() {
    // start sdl2 with everything
    let ctx = sdl2::init().unwrap();
    let video_ctx = ctx.video().unwrap();
    
    // Create a window
    let window  = match video_ctx.window("eg03", 640, 480).position_centered().opengl().build() {
        Ok(window) => window,
        Err(err)   => panic!("failed to create window: {}", err)
    };

    // Create a rendering context
    let mut renderer = match window.renderer().build() {
        Ok(renderer) => renderer,
        Err(err) => panic!("failed to create renderer: {}", err)
    };

    /*

    // Set the drawing color to a light blue.
    let _ = renderer.set_draw_color(sdl2::pixels::Color::RGB(101, 208, 246));

    // Clear the buffer, using the light blue color set above.
    let _ = renderer.clear();

    // Set the drawing color to a darker blue.
    let _ = renderer.set_draw_color(sdl2::pixels::Color::RGB(0, 153, 204));

    // Create centered Rect, draw the outline of the Rect in our dark blue color.
    let border_rect = Rect::new(320-64, 240-64, 128, 128).unwrap().unwrap();
    let _ = renderer.draw_rect(border_rect);

    // Create a smaller centered Rect, filling it in the same dark blue.
    let inner_rect = Rect::new(320-60, 240-60, 120, 120).unwrap().unwrap();
    let _ = renderer.fill_rect(inner_rect);

    // Swap our buffer for the present buffer, displaying it.
    let _ = renderer.present();

    */

    let mut viewport = Viewport::new();

    viewport.set_window_size(640,480);

    let mut events = ctx.event_pump().unwrap();

    // loop until we receive a QuitEvent
    'event : loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit{..} => break 'event,
                _               => {
                    println!("Got event: {:?}",  event);
                    continue
                }
            }
        }
    }
}
