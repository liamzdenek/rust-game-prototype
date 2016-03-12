use super::*;
use glium::backend::glutin_backend::GlutinFacade;
use storage_traits::storage_thread::Storage;
use storage_traits::environment_thread::Environment;
use std::collections::HashMap;

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    position: [f32;2],
}

implement_vertex!(Vertex, position);

pub struct MapBuilder {
    storage: Storage,
    environment: Environment,
}

impl MapBuilder {
    pub fn new(storage: Storage, environment: Environment) -> Self {
        MapBuilder{
            storage: storage,
            environment: environment,
        }
    }
}

impl RendererBuilder for MapBuilder {
    type O = Map;
    fn build(&mut self, display: &mut GlutinFacade) -> Map {
        Map::new(display, self.storage.clone(), self.environment.clone())
    }
}

pub struct Map {
    pub viewport: Viewport,
    storage: Storage,
    environment: Environment,
    program: Program,
    program2: Program,
}

impl Map {
    pub fn new<T>(display: &T, storage: Storage, environment: Environment) -> Self
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
            storage: storage,
            environment: environment,
        }
    }
}

impl Renderer for Map {
    fn render(&mut self, display: &mut GlutinFacade, frame: &mut Frame) {
        struct DrawCmd {
            kind: String,
            vertices: Vec<Vertex>,
        }
        
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
        let (ogl_tile_size, ogl_tile_ofs, start_tile, end_tile, focused_tile) = self.viewport.get_render_info(frame.get_dimensions());
        
        println!("RENDER INFO: {:?}", self.viewport.get_render_info(frame.get_dimensions()));


        let mut cmds: Vec<DrawCmd> = vec![];
        self.viewport.x += 0.1;

        self.storage.get_area(start_tile.clone().into(), end_tile.clone().into()).and_then(|vec| {
            return Ok(());
            for (t_pos, cell) in vec {
                cell.and_then(|cell| {
                    let bounding_points = (
                        (
                            (t_pos.x - focused_tile.0 as i64) as f32 * ogl_tile_size.0 - ogl_tile_ofs.0,
                            (t_pos.y - focused_tile.1 as i64) as f32 * ogl_tile_size.1 - ogl_tile_ofs.1,
                        ),
                        (
                            (t_pos.x - focused_tile.0 as i64 + 1) as f32 * ogl_tile_size.0 - ogl_tile_ofs.0,
                            (t_pos.y - focused_tile.1 as i64 + 1) as f32 * ogl_tile_size.1 - ogl_tile_ofs.1,
                        ),
                    );
                    let mut new_vert = vec![
                        // bottom left triangle
                        Vertex{ position: [ (bounding_points.0).0, (bounding_points.0).1] },
                        Vertex{ position: [ (bounding_points.0).0, (bounding_points.1).1] },
                        Vertex{ position: [ (bounding_points.1).0, (bounding_points.0).1] },
                        // top right triangle
                        Vertex{ position: [ (bounding_points.0).0, (bounding_points.1).1] },
                        Vertex{ position: [ (bounding_points.1).0, (bounding_points.1).1] },
                        Vertex{ position: [ (bounding_points.1).0, (bounding_points.0).1] },
                    ];

                    let mut was_found = false;
                    {
                        if let Some(v) = cmds.iter_mut().find(|v| v.kind == cell.terrain) {
                            v.vertices.append(&mut new_vert);
                            was_found = true;
                        }
                    }
                    if !was_found {
                        let v = DrawCmd{
                            kind: cell.terrain,
                            vertices: new_vert,
                        };
                        cmds.push(v);
                    }
                    Ok(())
                }).unwrap()
            }
            Ok(())
        }).unwrap();

        for v in cmds.into_iter() {
            let vertex_buffer = glium::VertexBuffer::new(display, &v.vertices).unwrap();
            let program = if v.kind == "sand" {
                &self.program
            } else {
                &self.program2
            };
            let uniforms = uniform!{
                matrix: [
                    [1.0,  0.0,  0.0,  0.0],
                    [0.0,  1.0, 0.0,  0.0],
                    [0.0,  0.0,  1.0,  0.0],
                    [0.0,  0.0,  0.0,  1.0f32],
                ]
            };

            frame.draw(&vertex_buffer, &indices, program, &uniforms, &Default::default()).unwrap();
        }

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
        */
        println!("map render");
    }
}
