use super::*;
use glium::texture::texture2d::Texture2d;
use glium::Blend;
use glium::draw_parameters::DrawParameters;
use glium::texture;
use std::io::Cursor;
use image;

pub struct TexCache<'a> {
    pub program: Program,

    pub draw_params: DrawParameters<'a>, 
 
    pub img_gravel: Texture2d,
    pub img_sand: Texture2d,
    pub img_human: Texture2d,
    pub img_missing: Texture2d,   
}

macro_rules! load_img {
    ($display: expr, $file: expr) => {{
        let image = image::load(Cursor::new(&include_bytes!($file)[..]),
                                image::PNG).unwrap().to_rgba();
        let image_dimensions = image.dimensions();
        let image = texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions);
        texture::Texture2d::new($display, image).unwrap()
    }}
}

impl<'a> TexCache<'a> {
    pub fn new(display: &GlutinFacade) -> Self {
        let vertex_shader_src = r#"
            #version 140

            in vec2 position;
            in vec2 tex_coords;
            out vec2 v_tex_coords;

            uniform mat4 matrix;

            void main() {
                v_tex_coords = tex_coords;
                gl_Position = matrix * vec4(position, 0.0, 1.0);
            }
        "#;

        let fragment_shader_src = r#"
           #version 140

            in vec2 v_tex_coords;
            out vec4 color;

            uniform sampler2D tex;
            uniform vec4 coloroverlay;

            void main() {
                color = texture(tex, v_tex_coords);
                color = coloroverlay * color;
            }
        "#;

        let program = Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap();

        let params = DrawParameters {
            blend: Blend::alpha_blending(),
            .. Default::default()
        };

        TexCache {
            program: program,
            draw_params: params,
            img_gravel: load_img!(display, "assets/img/gravel.png"),
            img_human: load_img!(display, "assets/img/human.png"),
            img_sand: load_img!(display, "assets/img/sand.png"),
            img_missing: load_img!(display, "assets/img/missing.png"),
        }
    }
}
