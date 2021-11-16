#![allow(unused_variables)]

mod csvreader;
mod mvpmatrix;
mod rocket_data;
mod webgl;
mod triangle;

use webgl::*;
use nalgebra_glm as glm;
use mvpmatrix::get_model;
use triangle::Triangle;


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
use crate::rocket_data::RocketData;

struct GlobalData {
    pub canvas: HtmlCanvasElement,
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

fn make_cylinder(num_pts: u32) -> Vec<Triangle> {
    let mut cylinder = Vec::new();

    for i in 0..num_pts {
        let step = 1.0 / (num_pts as f32) * std::f32::consts::PI * 2.0;
        let theta1 = i as f32 * step;
        let theta2 = theta1 + step;

        let x1 = theta1.cos();
        let x2 = theta2.cos();
        let y1 = -1.0;
        let y2 = 1.0;
        let z1 = theta1.sin();
        let z2 = theta2.sin();

        let p1 = glm::vec3(x1,y1,z1);
        let p2 = glm::vec3(x1,y2,z1);
        let p3 = glm::vec3(x2,y2,z2);
        let p4 = glm::vec3(x2,y1,z2);
        let c1 = glm::vec3(0.0,y1,0.0);
        let c2 = glm::vec3(0.0, y2 + (y2-y1)*2.0, 0.0);
        cylinder.push(Triangle::new(p1, p2, p3));
        cylinder.push(Triangle::new(p3,p4,p1));
        cylinder.push(Triangle::new(p1,c1,p4));
        cylinder.push(Triangle::new(p2,c2,p3));
    }

    return cylinder;
}

fn to_f32_vec(v: &Vec<Triangle>) -> Vec<f32>{
    let mut vertices: Vec<f32> = Vec::new();
    for (verts, normals) in v.iter().map(|t| t.to_array()) {
        for f in verts {
            vertices.push(f);
        }
    }

    return vertices;
}

#[wasm_bindgen]
pub fn start() -> Result<(), JsValue> {

    let canvas = get_canvas().unwrap();
    let context = get_context(&canvas).unwrap();

    let program = make_program(&context, VERT_SOURCE, FRAG_SOURCE)?;
    context.use_program(Some(&program));

    let vertices = to_f32_vec(&make_cylinder(100));
    // console::log_1(&format!("{:?}", vertices).into());

    // let vertices: [f32; 18] =
    //     [-0.5, -0.5, 0.0, -0.5, 0.5, 0.0, 0.5, 0.5, 0.0,
    //     0.5, 0.5, 0.0, 0.5, -0.5, 0.0, -0.5, -0.5, 0.0];

    let vertex_buffer = make_buffer(&context, vertices.as_slice());
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
            canvas: canvas,
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
    let cwidth = gd.canvas.width() as f32;
    let cheight = gd.canvas.height() as f32;

    // let rot = glm::rotate(&glm::identity(), gd.frame_count as f32 / 100.0, &glm::vec3(0.0, 0.0, 1.0));
    // let perspective = glm::perspective(cheight/cwidth, 90.0, 0.1, 100.0);

    let y = gd.frame_count as f32 / 100.0;
    let model: glm::Mat4 = glm::translate(&glm::identity(), &glm::vec3(0.0,y, 0.0));
    let view: glm::Mat4 = glm::look_at(
        &glm::vec3(0.0,0.0,4.0),
        &glm::vec3(0.0, y,0.0),
        &glm::vec3(0.0,1.0,0.0)
    );

    let proj: glm::Mat4 = glm::perspective(cwidth/cheight, 45.0, 0.1, 100.0);

    let mvp = proj * view * model;

    gd.ctx.uniform_matrix4fv_with_f32_array(gd.mvp_location.as_ref(), false, mvp.data.as_slice());

    gd.ctx.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT);

    gd.ctx.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, gd.vertex_count);

    gd.frame_count += 1;

    unsafe {
        GLOBAL_DATA = Some(gd);
    }
}
