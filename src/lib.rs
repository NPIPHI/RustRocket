#![allow(unused_variables)]

mod csvreader;
mod mvpmatrix;
mod rocket_data;
mod webgl;

use webgl::*;
use nalgebra_glm as glm;


static VERT_SOURCE: &str =
    r##"#version 300 es

in vec3 position;
uniform mat4 mvp;

void main() {
    gl_Position = mvp * vec4(position, 1);
}
"##;

static FRAG_SOURCE: &str =
r##"#version 300 es

precision highp float;
out vec4 outColor;

void main() {
    outColor = vec4(1, 0, 0, 1);
}
"##;

use wasm_bindgen::prelude::*;
use web_sys::*;

struct GlobalData {
    pub ctx: WebGl2RenderingContext,
    pub program: WebGlProgram,
    pub vertex_array: WebGlBuffer,
    pub vao: WebGlVertexArrayObject,
    pub mvp_location: Option<WebGlUniformLocation>,
    pub vertex_location: i32,
    pub vertex_count: i32,
    pub frame_count: u64,
}

static mut GLOBAL_DATA: Option<GlobalData> = None;

#[wasm_bindgen]
pub fn start() -> Result<(), JsValue> {
    let context = get_context().unwrap();

    let program = make_program(&context, VERT_SOURCE, FRAG_SOURCE)?;
    context.use_program(Some(&program));

    let vertices: [f32; 9] = [-0.7, -0.7, 0.0, 0.7, -0.7, 0.0, 0.0, 0.7, 0.0];

    let vertex_buffer = make_buffer(&context, &vertices);
    let position_attribute_location = context.get_attrib_location(&program, "position");
    let mvp_uniform_location = context.get_uniform_location(&program, "mvp");

    let vao = make_vao(&context).unwrap();
    context.bind_vertex_array(Some(&vao));

    context.vertex_attrib_pointer_with_i32(0, //index
                                           3, //count per vertex
                                           WebGl2RenderingContext::FLOAT,
                                           false, //normalized
                                           0, //stride bytes, 0 = default
                                           0 //offset bytes
    );

    context.enable_vertex_attrib_array(position_attribute_location as u32);

    context.bind_vertex_array(Some(&vao));

    let vert_count = (vertices.len() / 3) as i32;

    // let: glm::Mat4 = glm::identity();
    let mat = glm::scale(&glm::identity(), &glm::vec3(0.5,1.0,1.0));
    context.uniform_matrix4fv_with_f32_array(mvp_uniform_location.as_ref(), false, mat.data.as_slice());

    context.clear_color(0.0, 0.0, 0.0, 1.0);

    unsafe {
        GLOBAL_DATA = Some(GlobalData{
            ctx: context,
            program,
            vertex_array: vertex_buffer,
            vao,
            mvp_location: mvp_uniform_location,
            vertex_location: position_attribute_location,
            vertex_count: (vertices.len() / 3) as i32,
            frame_count: 0

        });
    }
    Ok(())
}

#[wasm_bindgen]
pub fn run_frame() {
    let mut gd = unsafe {GLOBAL_DATA.take().unwrap()};

    let mat = glm::rotate(&glm::identity(), gd.frame_count as f32 / 100.0, &glm::vec3(0.0, 0.0, 1.0));

    gd.ctx.uniform_matrix4fv_with_f32_array(gd.mvp_location.as_ref(), false, mat.data.as_slice());

    gd.ctx.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT);

    gd.ctx.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, gd.vertex_count);

    gd.frame_count += 1;

    unsafe {
        GLOBAL_DATA = Some(gd);
    }
}
