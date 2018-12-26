#[macro_use]
extern crate glium;

mod conway;
mod rendering;

fn main() {
    // Glium Initialization
    let mut event_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new()
        .with_title("Conway's Game of Life")
        .with_dimensions(glium::glutin::dpi::LogicalSize::new(500.0, 500.0));
    let context = glium::glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &event_loop).unwrap();

    // Game Initialization
    let mut generation = 0_u32;
    let mut state = conway::GridWorld::new(100, 100);
    let raster_grid = rendering::RasterGrid::new(state.cols_, state.rows_, true);

        std::thread::sleep(std::time::Duration::from_millis(1000));

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
