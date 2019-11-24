use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::thread;
use std::time;

pub mod lib;


// this is main
fn main() {
    let canvas_width = 720_u32;
    let canvas_height = 720_u32;

    let columns = 25_u32;
    let rows = 25_u32;

    let cell_width = canvas_width / columns;

    let (mut canvas, mut events) = lib::init(canvas_width, canvas_height);

    let mut grid = lib::grid_init(columns, rows);
    let mut direction = (1, 0);

    thread::spawn(move || {
        // move whatever variables we need into the closure defined by ||
    });

    // constantly ask for what events are happening
    'game: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'game,
            
                _ => continue 'game,
            }
        }

        //lib::display_rectangle(&mut canvas, &canvas_width, &canvas_height);
        lib::display_frame(&mut canvas, &grid, &columns, &rows, &cell_width);

        thread::sleep(time::Duration::from_millis(800));
    }

}
