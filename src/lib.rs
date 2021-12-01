#![allow(unused_variables)]

mod csvreader;
mod mvpmatrix;
mod rocket_data;
mod webgl;
mod load_model;

use webgl::*;
use nalgebra_glm as glm;
use mvpmatrix::get_model;
use wasm_bindgen::prelude::*;
use web_sys::*;
use js_sys::JsString;
use crate::rocket_data::RocketData;
use load_model::*;


static VERT_SOURCE: &str =
    r##"#version 300 es

in vec3 position;
in vec3 normal;
in vec2 uv;

out vec3 frag_normal;
out vec2 frag_uv;
uniform mat4 mvp;

void main() {
    frag_normal = normal;
    frag_uv = uv;
    gl_Position = mvp * vec4(position, 1);
}
"##;

static FRAG_SOURCE: &str =
r##"#version 300 es

precision highp float;
in vec3 frag_normal;
in vec2 frag_uv;

out vec4 outColor;
vec3 light = vec3(0.7,0.7,0);

uniform sampler2D tex;

void main() {
    float intensity = max(dot(light, frag_normal), 0.0) + 0.5;
    vec4 color = texture(tex, frag_uv);
    outColor = vec4(color.zyx * intensity, 1);
}
"##;


struct GlobalData {
    pub canvas: HtmlCanvasElement,
    pub ctx: WebGl2RenderingContext,
    pub program: WebGlProgram,
    pub rocket_vao: WebGlVertexArrayObject,
    pub rocket_tex: Option<WebGlTexture>,
    pub planet_vao: WebGlVertexArrayObject,
    pub planet_tex: Option<WebGlTexture>,
    pub tex_location: Option<WebGlUniformLocation>,
    pub mvp_location: Option<WebGlUniformLocation>,
    pub rocket_vertex_count: i32,
    pub planet_vertex_count: i32,
    pub frame_count: u64,
}

static mut GLOBAL_DATA: Option<GlobalData> = None;

static mut ROCKET_DATA_VEC: Vec<RocketData> = Vec::<RocketData>::new();
const ROCKET_DATA_TIMESTEP_SECONDS: f64 = 0.01;
const START_TIME_SECONDS: f64 = 7.0;
const TIME_SCALE: f64 = 1.0;
const FRAMES_PER_SECOND: f64 = 60.0;

async fn make_obj_vao(context: &WebGl2RenderingContext, program: &WebGlProgram, obj_path: &str) -> Result<(WebGlVertexArrayObject, i32), JsValue> {
    let (vertices, normals, uvs) = load_mesh(obj_path).await?;

    let position_attribute_location = context.get_attrib_location(&program, "position");
    let normal_attribute_location = context.get_attrib_location(&program, "normal");
    let uv_attribute_location = context.get_attrib_location(&program, "uv");

    let vertex_buffer = make_buffer(&context, vertices.as_slice());
    let normal_buffer = make_buffer(&context, normals.as_slice());
    let uv_buffer = make_buffer(&context, uvs.as_slice());

    let vao = make_vao(context).unwrap();
    context.bind_vertex_array(Some(&vao));

    bind_shader_array(context, Some(&vertex_buffer), position_attribute_location as u32, 3);

    bind_shader_array(context, Some(&normal_buffer), normal_attribute_location as u32, 3);

    bind_shader_array(context, Some(&uv_buffer), uv_attribute_location as u32, 2);

    return Ok((vao, (vertices.len()/3) as i32));
}

async fn make_texture_bmp(context: &WebGl2RenderingContext, bmp_path: &str) -> Result<Option<WebGlTexture>, JsValue> {
    let (texture_data, texture_width, texture_height) = load_bmp(bmp_path).await?;
    let texture = make_texture(context, texture_data.as_slice(), texture_width, texture_height);

    return Ok(texture);
}

#[wasm_bindgen]
pub async fn start(csv: String) -> Result<(), JsValue> {
    console::log_1(&JsValue::from_str("Start processing CSV"));
    unsafe {ROCKET_DATA_VEC = csvreader::get_csv_vec(csv.as_bytes()).unwrap()};
    // unsafe {ROCKET_DATA_VEC = csvreader::get_rocket_data().unwrap();}
    console::log_1(&JsValue::from_str("Finish processing CSV"));
    
    let canvas = get_canvas().unwrap();
    let context = get_context(&canvas).unwrap();
    context.enable(WebGl2RenderingContext::DEPTH_TEST);
    context.depth_func(WebGl2RenderingContext::LEQUAL);

    let program = make_program(&context, VERT_SOURCE, FRAG_SOURCE)?;
    context.use_program(Some(&program));

    let (rocket_vao, rocket_vert_count) = make_obj_vao(&context, &program,
                                  "Models/Ares_I_-_OBJ/Ares I/ares_I.obj").await?;

    let (planet_vao, planet_vert_count) = make_obj_vao(&context, &program,
                                 "Models/Earth/Earth_2K.obj").await?;



    let mvp_uniform_location = context.get_uniform_location(&program, "mvp");
    let texture_uniform_location = context.get_uniform_location(&program, "tex");
    let rocket_tex = make_texture_bmp(&context,"Models/Ares_I_-_OBJ/Ares I/ares_I.bmp").await?;
    let planet_tex = make_texture_bmp(&context,"Models/Earth/Textures/Diffuse_2K.bmp").await?;

    context.clear_color(0.0, 0.0, 0.0, 1.0);

    unsafe {
        GLOBAL_DATA = Some(GlobalData{
            canvas: canvas,
            ctx: context,
            program: program,
            rocket_vao: rocket_vao,
            rocket_tex: rocket_tex,
            planet_vao: planet_vao,
            planet_tex: planet_tex,
            mvp_location: mvp_uniform_location,
            tex_location: texture_uniform_location,
            rocket_vertex_count: rocket_vert_count,
            planet_vertex_count: planet_vert_count,
            frame_count: 0

        });
    }
    Ok(())
}

#[wasm_bindgen]
pub fn run_frame() {
    let mut gd = unsafe { GLOBAL_DATA.take().unwrap() };
    let cwidth = gd.canvas.width() as f32;
    let cheight = gd.canvas.height() as f32;

    // let rot = glm::rotate(&glm::identity(), gd.frame_count as f32 / 100.0, &glm::vec3(0.0, 0.0, 1.0));
    // let perspective = glm::perspective(cheight/cwidth, 90.0, 0.1, 100.0);

    let rocket_data_row_index = get_rocket_data_row_index(gd.frame_count);
    // console::log_1(&JsValue::from_f64(rocket_data_row_index as f64));
    let rd;
    unsafe {
        if rocket_data_row_index < ROCKET_DATA_VEC.len() {
            rd = &ROCKET_DATA_VEC[rocket_data_row_index];
        } else {
            rd = &ROCKET_DATA_VEC[ROCKET_DATA_VEC.len() - 1];
        }
    }

    let to_angle = |a: f64, b: f64| {
        let angle = (rd.mx / rd.mz).atan() as f32;
        let angle = if rd.mz > 0.0 { angle } else  { angle + 3.1415};
        return angle;
    };

    let roll = to_angle(rd.mx, rd.mz);
    // let pitch = to_angle(rd.my, rd.mx);
    // let yaw = to_angle(rd.mz, rd.my);


    let z = (rd.barometer_altitude * 1.0) as f32;
    // let z = gd.frame_count as f32;
    let rocket_model: glm::Mat4 =
        glm::translate(&glm::identity(), &glm::vec3(0.0, 0.0, z)) *
        glm::scale(&glm::identity(), &glm::vec3(0.1,0.1,0.1)) *
        // glm::rotate(&glm::identity(), pitch, &glm::vec3(0.0, 1.0, 0.0)) *
        // glm::rotate(&glm::identity(), yaw, &glm::vec3(1.0, 0.0, 0.0)) *
        glm::rotate(&glm::identity(), roll, &glm::vec3(0.0, 0.0, 1.0));

    let planet_model: glm::Mat4 =
        glm::translate(&glm::identity(), &glm::vec3(0.0, 0.0, -100.0)) *
        glm::scale(&glm::identity(), &glm::vec3(100.0,100.0,100.0)) *
        glm::rotate(&glm::identity(), 2.0, &glm::vec3(0.0,-0.2,1.0));
    
    let view: glm::Mat4 = glm::look_at(
        &glm::vec3(0.0,7.0,20.0 + z),
        &glm::vec3(0.0, 0.0, z),
        &glm::vec3(0.0,0.0,1.0)
    );

    let proj: glm::Mat4 = glm::perspective(cwidth/cheight, 45.0, 0.1, 100000.0);

    let mvp_rocket = proj * view * rocket_model;
    let mvp_planet = proj * view * planet_model;

    gd.ctx.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT);

    gd.ctx.bind_vertex_array(Some(&gd.rocket_vao));
    bind_shader_texture(&gd.ctx, gd.rocket_tex.as_ref(), gd.tex_location.as_ref(), 0);
    gd.ctx.uniform_matrix4fv_with_f32_array(gd.mvp_location.as_ref(), false, mvp_rocket.data.as_slice());
    gd.ctx.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, gd.rocket_vertex_count);
    gd.ctx.bind_vertex_array(Some(&gd.planet_vao));

    bind_shader_texture(&gd.ctx, gd.planet_tex.as_ref(), gd.tex_location.as_ref(), 0);
    gd.ctx.uniform_matrix4fv_with_f32_array(gd.mvp_location.as_ref(), false, mvp_planet.data.as_slice());
    gd.ctx.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, gd.planet_vertex_count);

    gd.frame_count += 1;

    unsafe {
        GLOBAL_DATA = Some(gd);
    }

    pub fn get_rocket_data_row_index(frame_count: u64) -> usize {
        let time_seconds = START_TIME_SECONDS + ((frame_count as f64) / FRAMES_PER_SECOND) * TIME_SCALE;
        return (time_seconds / ROCKET_DATA_TIMESTEP_SECONDS).round() as usize;
    }
}
