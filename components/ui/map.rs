use super::*;
use storage_traits::storage_thread::Storage;
use storage_traits::environment_thread::Environment;
use glium::glutin::{Event,ElementState,MouseButton,MouseScrollDelta};
use std::collections::HashMap;
use imgui::ImGui;
use glium::texture::{ClientFormat, RawImage2d};

use std::borrow::Cow;

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    position: [f32;2],
    tex_coords: [f32;2],
}

implement_vertex!(Vertex, position, tex_coords);

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
    fn build(&mut self, im_gui: &mut ImGui, display: &mut GlutinFacade) -> Map {
        Map::new(im_gui, display, self.storage.clone(), self.environment.clone())
    }
}

pub struct Map {
    pub viewport: Viewport,

    mouse_pressed: bool,
    last_pos: (i32, i32),
    px_tile_size: u32,
    window_size: (u32, u32),
   
    storage: Storage,
    environment: Environment,
}

impl Map {
    pub fn new<T>(im_gui: &mut ImGui, display: &T, storage: Storage, environment: Environment) -> Self
        where T: Facade
    {
        Map{
            viewport: Viewport::default(),
            storage: storage,
            environment: environment,
            
            mouse_pressed: false,
            px_tile_size: 0,
            last_pos: (0,0),
            window_size: (0,0),
        }
    }
}

impl Renderer for Map {
    fn render(&mut self, texcache: &mut TexCache, display: &mut GlutinFacade, frame: &mut Frame) {
        #[derive(PartialEq)]
        enum DrawCmdKind {
            Terrain(u64),
            Entity(u64),
        }
        struct DrawCmd {
            kind: DrawCmdKind,
            vertices: Vec<Vertex>,
        }
        
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
        let (px_tile_size, ogl_tile_size, ogl_tile_ofs, start_tile, end_tile, focused_tile) = self.viewport.get_render_info(frame.get_dimensions());
        
        self.px_tile_size = px_tile_size;

        let mut cmds: Vec<DrawCmd> = vec![];
        //self.viewport.x += 0.1;

        self.storage.get_area(start_tile.clone().into(), end_tile.clone().into()).and_then(|vec| {
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
                        Vertex{ position: [ (bounding_points.0).0, (bounding_points.0).1], tex_coords: [0.0, 0.0] },
                        Vertex{ position: [ (bounding_points.0).0, (bounding_points.1).1], tex_coords: [0.0, 1.0] },
                        Vertex{ position: [ (bounding_points.1).0, (bounding_points.0).1], tex_coords: [1.0, 0.0] },
                        // top right triangle
                        Vertex{ position: [ (bounding_points.0).0, (bounding_points.1).1], tex_coords: [0.0, 1.0] },
                        Vertex{ position: [ (bounding_points.1).0, (bounding_points.1).1], tex_coords: [1.0, 1.0] },
                        Vertex{ position: [ (bounding_points.1).0, (bounding_points.0).1], tex_coords: [1.0, 0.0] },
                    ];

                    let mut was_found = false;
                    {
                        if let Some(v) = cmds.iter_mut().find(|v| v.kind == DrawCmdKind::Terrain(cell.terrain)) {
                            v.vertices.append(&mut new_vert);
                            was_found = true;
                        }
                    }
                    if !was_found {
                        let v = DrawCmd{
                            kind: DrawCmdKind::Terrain(cell.terrain),
                            vertices: new_vert,
                        };
                        cmds.push(v);
                    }
                    Ok(())
                }).unwrap()
            }
            Ok(())
        }).unwrap();

        self.environment.get_entities_by_area(start_tile.clone().into(), end_tile.clone().into()).and_then(|vec| {
            for ent in vec {
                let bounding_points = (
                    (
                        (ent.pos.x - focused_tile.0 as i64) as f32 * ogl_tile_size.0 - ogl_tile_ofs.0,
                        (ent.pos.y - focused_tile.1 as i64) as f32 * ogl_tile_size.1 - ogl_tile_ofs.1,
                    ),
                    (
                        (ent.pos.x - focused_tile.0 as i64 + 1) as f32 * ogl_tile_size.0 - ogl_tile_ofs.0,
                        (ent.pos.y - focused_tile.1 as i64 + 1) as f32 * ogl_tile_size.1 - ogl_tile_ofs.1,
                    ),
                );
                let mut new_vert = vec![
                    // bottom left triangle
                    Vertex{ position: [ (bounding_points.0).0, (bounding_points.0).1], tex_coords: [0.0, 0.0] },
                    Vertex{ position: [ (bounding_points.0).0, (bounding_points.1).1], tex_coords: [0.0, 1.0] },
                    Vertex{ position: [ (bounding_points.1).0, (bounding_points.0).1], tex_coords: [1.0, 0.0] },
                    // top right triangle
                    Vertex{ position: [ (bounding_points.0).0, (bounding_points.1).1], tex_coords: [0.0, 1.0] },
                    Vertex{ position: [ (bounding_points.1).0, (bounding_points.1).1], tex_coords: [1.0, 1.0] },
                    Vertex{ position: [ (bounding_points.1).0, (bounding_points.0).1], tex_coords: [1.0, 0.0] },
                ];

                let mut was_found = false;
                {
                    if let Some(v) = cmds.iter_mut().find(|v| v.kind == DrawCmdKind::Entity(ent.id)) {
                        v.vertices.append(&mut new_vert);
                        was_found = true;
                    }
                }
                if !was_found {
                    let v = DrawCmd{
                        kind: DrawCmdKind::Entity(ent.id),
                        vertices: new_vert,
                    };
                    cmds.push(v);
                }
            }
            Ok(())
        }).unwrap();



        for v in cmds.into_iter() {
            let vertex_buffer = glium::VertexBuffer::new(display, &v.vertices).unwrap();
            let (program, uniform) = match v.kind {
                DrawCmdKind::Terrain(kind) => {
                    let tex = match kind {
                        0 => {
                            &texcache.img_gravel
                        },
                        1 => {
                            &texcache.img_sand
                        },
                        _ => {
                            &texcache.img_missing
                        },
                    };
                    
                    (
                        &texcache.program,
                        uniform!{
                            matrix: [
                                [1.0,  0.0,  0.0,  0.0],
                                [0.0,  1.0,  0.0,  0.0],
                                [0.0,  0.0,  1.0,  0.0],
                                [0.0,  0.0,  0.0,  1.0f32],
                            ],
                            tex: tex,
                        }
                    )
                },
                DrawCmdKind::Entity(ent_id) => {
                    let tex = &texcache.img_missing;
                    (
                        &texcache.program,
                        uniform!{
                            matrix: [
                                [1.0,  0.0,  0.0,  0.0],
                                [0.0,  1.0,  0.0,  0.0],
                                [0.0,  0.0,  1.0,  0.0],
                                [0.0,  0.0,  0.0,  1.0f32],
                            ],
                            tex: tex,
                        }
                    )
                }
                /*_ => {
                    &self.program
                }*/
            };

            frame.draw(&vertex_buffer, &indices, program, &uniform, &Default::default()).unwrap();
        }

        self.window_size = frame.get_dimensions(); 
    }

    fn handle_events(&mut self, events: Vec<Event>) {
        for event in events {
            match event {
                Event::MouseInput(state, MouseButton::Left) => {
                    self.mouse_pressed = state == ElementState::Pressed;
                    let tile = self.viewport.get_tile_at_cursor(self.window_size, self.last_pos);
                    println!("setting tile at: {:?}", tile);
                    use common::Cell;
                    self.storage.set_cell(tile.into(), Cell{
                        terrain: 1,
                        .. Cell::default()
                    }).unwrap(); 
                }
                Event::MouseMoved(pos) => {
                    if self.mouse_pressed {
                        let delta = (
                            self.last_pos.0 - pos.0, // no idea why this axis is reversed but whatever, it works
                            pos.1 - self.last_pos.1,
                        );
                        self.viewport.add(delta.0 as f32 / self.px_tile_size as f32, delta.1 as f32 / self.px_tile_size as f32);
                    }
                    self.last_pos = pos;
                }
                Event::MouseWheel(delta) => {
                    println!("Delta: {:?}", delta);
                    match delta {
                        MouseScrollDelta::LineDelta(dx, dy) => {
                            let direction = if dx != 0.0 {
                                dx
                            } else {
                                dy
                            };
                            self.viewport.update_zoom(direction, self.window_size, self.last_pos)
                        }
                        MouseScrollDelta::PixelDelta(dx, dy) => {
                            let direction = if dx != 0.0 {
                                dx
                            } else {
                                dy
                            };
                            self.viewport.update_zoom(direction, self.window_size, self.last_pos)
                        }
                    }
                }
                _ => {
                    // unhandled
                }
            }
        }
    }
}
