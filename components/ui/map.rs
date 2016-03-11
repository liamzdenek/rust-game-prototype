use super::*;
use glium::backend::glutin_backend::GlutinFacade;

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32;2],
}

implement_vertex!(Vertex, position);

pub struct Map {
    program: Program,
    program2: Program,
    viewport: Viewport,
}

impl Map {
    pub fn new<T>(display: &T) -> Self
        where T: Facade
    {
        /*
        let shape = vec![
            Vertex{ position: [-0.5, -0.5] },
            Vertex{ position: [-0.5,  0.5] },
            Vertex{ position: [ 0.5, -0.5] },
            Vertex{ position: [ 0.5,  0.5] },
        ];
        let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();
        */

        let vertex_shader_src = r#"
            #version 140
            in vec2 position;
            uniform mat4 matrix;
            void main() {
                gl_Position = matrix * vec4(position, 0.0, 1.0);
            }
        "#;

        let fragment_shader_src = r#"
            #version 140
            out vec4 color;
            void main() {
                color = vec4(1.0, 0.0, 0.0, 1.0);
            }
        "#;

        let fragment_shader_src2 = r#"
            #version 140
            out vec4 color;
            void main() {
                color = vec4(0.0, 1.0, 0.0, 1.0);
            }
        "#;
        let program = glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap();
        let program2 = glium::Program::from_source(display, vertex_shader_src, fragment_shader_src2, None).unwrap();
        Map{
            program: program,
            program2: program2,
            viewport: Viewport::default(),
        }
    }
}

impl Renderer for Map {
    fn render(&mut self, display: &mut GlutinFacade, frame: &mut Frame)
    {
        /*
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);//TriangleFan);

        let (zoom, num_tiles) = self.viewport.get_render_info(frame.get_dimensions());

        let x_size = (2.0/num_tiles.0 as f32);
        let y_size = (2.0/num_tiles.1 as f32);
        for x in 0..num_tiles.0 {
            for y in 0..num_tiles.1 {
                let x = x as f32;
                let y = y as f32;
                //println!("drawing rect at x: {:?}", x);
                let x_translate = 1.0 - (2.0 * x / num_tiles.0 as f32) - x_size;
                let y_translate = 1.0 - (2.0 * y / num_tiles.1 as f32) - y_size; 
                let uniforms = uniform!{
                    matrix: [
                        [x_size, 0.0,  0.0,  0.0],
                        [0.0,  y_size, 0.0,  0.0],
                        [0.0,  0.0,  1.0,  0.0],
                        [x_translate, y_translate,  0.0,  1.0f32],
                    ]
                };

                frame.draw(&self.vertex_buffer, &indices, if x as u32 % 2 == 0 || y as u32 % 2 == 0 { &self.program } else { &self.program2 }, &uniforms, &Default::default()).unwrap();
            }
        }
        println!("map render");
        */
    }
}
