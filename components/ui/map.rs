use super::*;
use backend_traits::storage_thread::Storage;
use backend_traits::environment_thread::{Environment,LocalEntityData};
use glutin::{Event,ElementState,MouseButton,MouseScrollDelta};
use std::collections::HashMap;
use glium::texture::{ClientFormat, RawImage2d};
use glium::{VertexBuffer, index};
use common::Position;

#[derive(PartialEq)]
enum DrawCmdKind {
    Terrain(u64, bool), // bool = is_gray
    Entity(u64, bool), // bool = is_gray
}
struct DrawCmd {
    kind: DrawCmdKind,
    vertices: Vec<Vertex>,
}
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    position: [f32;2],
    tex_coords: [f32;2],
}

implement_vertex!(Vertex, position, tex_coords);

pub enum InputState {
    None,
    Input((i32, i32), ElementState, MouseButton),
    Panning,
}

pub struct MapBuilder {
    backend: Storage,
    environment: Environment,
}

impl MapBuilder {
    pub fn new(backend: Storage, environment: Environment) -> Self {
        MapBuilder{
            backend: backend,
            environment: environment,
        }
    }
}

impl RendererBuilder for MapBuilder {
    type O = Map;
    fn build(&mut self, im_gui: &mut ImGui, display: &mut GlutinFacade) -> Map {
        Map::new(im_gui, display, self.backend.clone(), self.environment.clone())
    }
}

pub struct Map {
    pub viewport: Viewport,

    mouse_state: InputState,
    last_pos: (i32, i32),
    px_tile_size: u32,
    window_size: (u32, u32),

    inspector: Inspector,
    //focused: Option<LocalEntityData>,

    backend: Storage,
    environment: Environment,
}

impl Map {
    pub fn new<T>(im_gui: &mut ImGui, display: &T, backend: Storage, environment: Environment) -> Self
        where T: Facade
    {
        Map{
            viewport: Viewport::default(),
            backend: backend,
            environment: environment,

            inspector: Inspector::new(),

            mouse_state: InputState::None,
            px_tile_size: 0,
            last_pos: (0,0),
            window_size: (0,0),
        }
    }

    pub fn on_click(&mut self) {
        let tile = self.viewport.get_tile_at_cursor(self.window_size, self.last_pos);
        let entity = self.environment.get_entities_by_area(tile.clone().into(), tile.clone().into());

        if entity.is_err() {
            return;   
        }
        let entity = entity.unwrap().into_iter().nth(0);

        // TODO: maybe remove this if?
        if entity.is_none() {
            return;
        }
        let entity = entity.unwrap();
        self.inspector.focus = InspectorFocus::Entity(entity);
        /*
        println!("setting tile at: {:?}", tile);
        ue common::Cell;
        self.backend.set_cell(tile.into(), Cell{
            terrain: 1,
            .. Cell::default()
        }).unwrap();
        */
    }

    

    fn render_bg(&mut self, cmds: &mut Vec<DrawCmd>, start_tile: Position, end_tile: Position, ogl_tile_size: (f32, f32), ogl_tile_ofs: (f32, f32), focused_tile: (i32, i32)) {
        match self.inspector.focus {
            InspectorFocus::Entity(ref entity) => {
                entity.ent.get_area(start_tile.clone().into(), end_tile.clone().into()).and_then(|vec| {
                    for cell in vec {
                        render_tile(cmds, cell.pos, focused_tile, ogl_tile_size, ogl_tile_ofs, DrawCmdKind::Terrain(cell.cell.terrain, cell.is_from_memory));
                    }
                    Ok(())
                }).unwrap();
            },
            _ => {
                self.backend.get_area(start_tile.clone().into(), end_tile.clone().into()).and_then(|vec| {
                    for (t_pos, cell) in vec {
                        render_tile(cmds, t_pos, focused_tile, ogl_tile_size, ogl_tile_ofs, DrawCmdKind::Terrain(cell.terrain, false));
                    }
                    Ok(())
                }).unwrap();
            },
        }
    }
}

fn render_tile(cmds: &mut Vec<DrawCmd>, t_pos: Position, focused_tile: (i32, i32), ogl_tile_size: (f32, f32), ogl_tile_ofs: (f32, f32), kind: DrawCmdKind) {
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
        if let Some(v) = cmds.iter_mut().find(|v| v.kind == kind) {
            v.vertices.append(&mut new_vert);
            was_found = true;
        }
    }
    if !was_found {
        let v = DrawCmd{
            kind: kind,
            vertices: new_vert,
        };
        cmds.push(v);
    }
}


impl ImguiRenderer for Map {
    fn render_ui<'ui>(&mut self, ui: &Ui<'ui>, app_data: &mut AppData, texcache: &mut TexCache, display: &mut GlutinFacade, frame: &mut Frame) {
        {
           self.inspector.render_ui(ui, app_data, texcache, display, frame); 
        }
    }
}

impl Renderer for Map {
    fn render(&mut self, texcache: &mut TexCache, display: &mut GlutinFacade, frame: &mut Frame) {

        
        let indices = index::NoIndices(index::PrimitiveType::TrianglesList);
        let (px_tile_size, ogl_tile_size, ogl_tile_ofs, start_tile, end_tile, focused_tile) = self.viewport.get_render_info(frame.get_dimensions());
        
        self.px_tile_size = px_tile_size;

        let mut cmds: Vec<DrawCmd> = vec![];
        //self.viewport.x += 0.1;

        self.render_bg(&mut cmds, start_tile.clone().into(), end_tile.clone().into(), ogl_tile_size, ogl_tile_ofs, focused_tile);

        self.environment.get_entities_by_area(start_tile.clone().into(), end_tile.clone().into()).and_then(|vec| {
            for ent in vec {
                render_tile(&mut cmds, ent.pos.clone().into(), focused_tile, ogl_tile_size, ogl_tile_ofs, DrawCmdKind::Entity(ent.id, false));
            }
            Ok(())
        }).unwrap();



        for v in cmds.into_iter() {
            let vertex_buffer = VertexBuffer::new(display, &v.vertices).unwrap();
            let (program, uniform) = match v.kind {
                DrawCmdKind::Terrain(kind, is_gray) => {
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
          
                    let coloroverlay = if is_gray {
                        [0.5, 0.5, 0.5, 0.5f32]
                    } else {
                        [1.0, 1.0, 1.0, 1.0f32] 
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
                            coloroverlay: coloroverlay,
                        }
                    )
                },
                DrawCmdKind::Entity(ent_id, is_gray) => {
                    let tex = &texcache.img_human;
          
                    let coloroverlay = if is_gray {
                        [0.5, 0.5, 0.5, 1.0f32]
                    } else {
                        [1.0, 1.0, 1.0, 1.0f32] 
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
                            coloroverlay: coloroverlay,
                        }
                    )
                }
                /*_ => {
                    &self.program
                }*/
            };

            frame.draw(&vertex_buffer, &indices, program, &uniform, &texcache.draw_params).unwrap();
        }

        self.window_size = frame.get_dimensions(); 
    }

    fn handle_events(&mut self, events: Vec<Event>) {
        for event in events {
            match event {
                Event::MouseInput(state, button) => {
                    if let InputState::Input(_, ElementState::Pressed, MouseButton::Left) = self.mouse_state {
                        if let ElementState::Released = state {
                            self.on_click();
                        }
                    } 
                    self.mouse_state = InputState::Input(self.last_pos, state, button);
                 
                }
                Event::MouseMoved(pos) => {
                    let should_pan = if let InputState::Input(start_pos, ElementState::Pressed, MouseButton::Left) = self.mouse_state {
                        if (start_pos.0 - self.last_pos.0).abs() + (start_pos.1 - self.last_pos.1).abs() > 10 {
                            self.mouse_state = InputState::Panning;
                            true
                        } else {
                            false
                        }
                    } else if let InputState::Panning = self.mouse_state {
                        true
                    } else {
                        false
                    };

                    if should_pan {
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
