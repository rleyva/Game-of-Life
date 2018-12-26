#[macro_use]
extern crate glium;

mod conway;
mod rendering;

/// Number of cells in the simulation.
const GRID_HEIGHT_SIZE: usize = 30;
const GRID_WIDTH_SIZE: usize = 30;

/// Multiplier used to set the window size based on the number of cells in 
/// the simulation.
const CELL_HEIGHT_MULTIPLIER: f64 = 5.0;
const CELL_WIDTH_MULTIPLIER: f64  = 5.0;

/// Size of the windows based on the multi
const WINDOW_HEIGHT: f64 = (CELL_HEIGHT_MULTIPLIER * GRID_HEIGHT_SIZE as f64);
const WINDOW_WIDTH: f64 = (CELL_WIDTH_MULTIPLIER * GRID_WIDTH_SIZE as f64);

fn main() {
    // Glium Initialization
    let mut event_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new()
        .with_title("Conway's Game of Life")
        .with_dimensions(glium::glutin::dpi::LogicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT));
    let context = glium::glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &event_loop).unwrap();

    // Game Initialization
    let mut generation = 0_u32;
    let mut state = conway::GridWorld::new(GRID_HEIGHT_SIZE, GRID_WIDTH_SIZE);
    let raster_grid = rendering::RasterGrid::new(state.cols_, state.rows_, true);

    // Event loop
    let mut closed = false;
    while !closed {
        // Carry out game logic - internally mutates passed members.
        state = conway::update(&state, generation);
        generation += 1;

        // Draw the state.
        rendering::draw(&raster_grid, &state, display.clone());
        std::thread::sleep(std::time::Duration::from_millis(50));

        // Manage window.
        event_loop.poll_events(|ev| match ev {
            glium::glutin::Event::WindowEvent { event, .. } => match event {
                glium::glutin::WindowEvent::CloseRequested => closed = true,
                _ => (),
            },
            _ => (),
        });
    }
}
