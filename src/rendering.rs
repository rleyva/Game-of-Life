///
/// Library used to manage OpenGL/Glium related calls.
///

mod shaders {

// Vertex shader
pub const VERTEX_SRC: &str = r#"
    #version 140
    in vec2 position;
    
    void main() {
        gl_Position = vec4(position, 0.0, 1.0);
    }
"#;

// Fragment shader
pub const FRAGMENT_SRC: &str = r#"
    #version 140
    out vec4 color;

    void main() {
        color = vec4(1.0, 1.0, 1.0, 1.0);
    }
"#;

}

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 2],
}
implement_vertex!(Vertex, position);

pub struct RasterGrid {
    /// Structure that maps a cell in a GridWorld instance, to the corresponding
    /// block in the shader.

    /// Vertices corresponding to each cell (they maintain the same index)
    // TODO: Flatten this - consider removing this entirely.
    vertices_cache: Vec<Vec<Vec<Vertex>>>,
}

impl RasterGrid {
    /// Helper container that take information pertaining to a game state, and generating
    /// all of the the required information to draw that state via OpenGL. The intent
    /// is to completely decouple the game portion of the code, and the rendering portion.
    pub fn new(grid_rows: usize, grid_cols: usize, is_padded: bool) -> RasterGrid {
        // Normalized size of each cell in the grid.
        let padding_offset = if is_padded {1_usize} else {0_usize};

        let norm_x_size: f32 = 2.0 / (grid_cols - padding_offset) as f32;
        let norm_y_size: f32 = 2.0 / (grid_rows - padding_offset) as f32;

        // Generate all of the vertices needed to paint squares for each cell onto the
        // shader - this is cached, and the appropriate shaders are fetched when a 
        // cell is marked as 'alive'.
        let mut vertices_grid: Vec<Vec<Vec<Vertex>>> = Vec::new();
        for row in padding_offset..(grid_rows - 1) {
            let mut row_vertices: Vec<Vec<Vertex>> = Vec::new();
            for col in padding_offset..(grid_cols - 1) {
                let cell_vertices = generate_cell_verticies(row, col, norm_x_size, norm_y_size);
                row_vertices.push(cell_vertices);
            }
            vertices_grid.push(row_vertices);
        }
        return RasterGrid {
            vertices_cache: vertices_grid,
        };
    }
}

///
/// Rendering functions
///
fn generate_cell_verticies(row: usize, col: usize, x_size: f32, y_size: f32) -> Vec<Vertex> {
    // Take coordinates of a cell (normalized), add the offsets, and then apply a transform
    // to place the coordinates in the right spot. If the grid is padded, then we have to
    // take the cells internal rep, and
    let x: f32 = ((col as f32) - 1.0) * x_size;
    let y: f32 = ((row as f32) - 1.0) * y_size;

    // The shader coordinate system has a length and witdth of 2 - the origin is located
    // in the center of this square - we have to transform our row-col to the shader
    // shader's coordinate system.

    // Calculate the coordinates of the cell in the shader's coord-system.
    // TODO: Explain magic numbers.
    let transform_to_shader_coord = |coord: [f32; 2]| -> [f32; 2] {
        return [(coord[0] as f32) - 1.0, -1_f32 * ((coord[1] as f32) - 1.0)];
    };

    return vec![
        Vertex {
            position: transform_to_shader_coord([x, y]),
        },
        Vertex {
            position: transform_to_shader_coord([x, y + y_size]),
        },
        Vertex {
            position: transform_to_shader_coord([x + x_size, y + y_size]),
        },
        Vertex {
            position: transform_to_shader_coord([x + x_size, y + y_size]),
        },
        Vertex {
            position: transform_to_shader_coord([x + x_size, y]),
        },
        Vertex {
            position: transform_to_shader_coord([x, y]),
        },
    ];
}

pub fn get_vertices_to_draw(raster_grid: &RasterGrid, state: &super::conway::GridWorld) -> Vec<Vertex> {
    // TODO: GridWorld should implement an iterator, so that this can be general.
    let mut vec_of_live_cell_vertices = Vec::new();
    for row in 1..state.rows_ - 1 {
        for col in 1..state.cols_ - 1 {
            let cell = state.grid_[row][col];
            if cell.state_ == super::conway::STATE::ALIVE {
                let vertices = &raster_grid.vertices_cache[row - 1][col - 1];
                vec_of_live_cell_vertices.extend(vertices);
            }
        }
    }
    return vec_of_live_cell_vertices; 
}

pub fn draw(raster_grid: &RasterGrid, state: &super::conway::GridWorld, display: glium::Display) {
    use glium::Surface;

    // Takes the game state and returns a vector of vertices corresponding to all of
    // the cells that should be drawn (those that are alive).
    let vertices = get_vertices_to_draw(&raster_grid, &state);

    let vertex_buffer = glium::VertexBuffer::new(&display, &vertices).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let program =
        glium::Program::from_source(&display, shaders::VERTEX_SRC, shaders::FRAGMENT_SRC, None)
            .unwrap();

    let mut target = display.draw();
    target.clear_color(0.0, 0.0, 0.0, 1.0);
    target
        .draw(
            &vertex_buffer,
            &indices,
            &program,
            &glium::uniforms::EmptyUniforms,
            &Default::default(),
        )
        .unwrap();
    target.finish().unwrap();
}
