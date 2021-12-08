#![allow(unused_variables)]

use js_sys::JsString;
use nalgebra_glm as glm;
use wasm_bindgen::prelude::*;
use web_sys::*;

use load_model::*;
use mvpmatrix::get_model;
use webgl::*;

use crate::rocket_data::RocketData;

mod csvreader;
mod mvpmatrix;
mod rocket_data;
mod webgl;
mod load_model;

static VERT_SOURCE: &str =
    r##"#version 300 es

in vec3 position;
in vec3 normal;
in vec2 uv;

out vec3 frag_normal;
out vec2 frag_uv;
uniform mat4 mvp;
uniform mat4 rotate;

void main() {
    frag_normal = (rotate * vec4(normal, 0)).xyz;
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
uniform float opacity;

void main() {
    float intensity = max(dot(light, frag_normal), 0.0) + 0.7;
    vec4 color = texture(tex, frag_uv);
    outColor = vec4(color.zyx * intensity, opacity);
}
"##;


struct GlobalData {
    pub canvas: HtmlCanvasElement,
    pub ctx: WebGl2RenderingContext,
    pub program: WebGlProgram,
    pub rocket_vao: WebGlVertexArrayObject,
    pub rocket_tex: Option<WebGlTexture>,
    pub planet_models: Vec<(WebGlVertexArrayObject, i32)>,
    pub planet_textures: Vec<Option<WebGlTexture>>,
    pub tex_location: Option<WebGlUniformLocation>,
    pub mvp_location: Option<WebGlUniformLocation>,
    pub rotate_location: Option<WebGlUniformLocation>,
    pub opacity_location: Option<WebGlUniformLocation>,
    pub rocket_vertex_count: i32,
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

    return make_vao_vert_norm_uv(context, program, &vertices, &normals, &uvs);
}

async fn make_texture_bmp(context: &WebGl2RenderingContext, bmp_path: &str) -> Result<Option<WebGlTexture>, JsValue> {
    let (texture_data, texture_width, texture_height) = load_bmp(bmp_path).await?;
    let texture = make_texture(context, texture_data.as_slice(), texture_width, texture_height);

    return Ok(texture);
}

#[wasm_bindgen]
pub async fn start(csv: String) -> Result<(), JsValue> {
    unsafe {ROCKET_DATA_VEC = csvreader::get_csv_vec(csv.as_bytes()).unwrap()};

    let canvas = get_canvas().unwrap();
    let context = get_context(&canvas).unwrap();
    context.enable(WebGl2RenderingContext::CULL_FACE);
    context.enable(WebGl2RenderingContext::DEPTH_TEST);
    context.depth_func(WebGl2RenderingContext::LEQUAL);
    context.enable(WebGl2RenderingContext::BLEND);
    context.blend_func_separate(WebGl2RenderingContext::SRC_ALPHA, WebGl2RenderingContext::ONE_MINUS_SRC_ALPHA, WebGl2RenderingContext::ONE, WebGl2RenderingContext::ONE_MINUS_SRC_ALPHA);
    let program = make_program(&context, VERT_SOURCE, FRAG_SOURCE)?;
    context.use_program(Some(&program));

    let (rocket_vao, rocket_vert_count) = make_obj_vao(&context, &program,
                                  "Models/Ares_I_-_OBJ/Ares I/ares_I.obj").await?;

    let (verts, norms, uvs) = make_plane();
    let sphere = make_obj_vao(&context, &program, "Models/Earth/Earth_2K.obj").await?;
    let planet_models = vec![
        make_vao_vert_norm_uv(&context, &program, &verts, &norms, &uvs)?,
        make_vao_vert_norm_uv(&context, &program, &verts, &norms, &uvs)?,
        make_vao_vert_norm_uv(&context, &program, &verts, &norms, &uvs)?,
        make_vao_vert_norm_uv(&context, &program, &verts, &norms, &uvs)?,
        sphere
    ];


    let mvp_uniform_location = context.get_uniform_location(&program, "mvp");
    let rotate_uniform_location = context.get_uniform_location(&program, "rotate");
    let opacity_uniform_location = context.get_uniform_location(&program, "opacity");

    let texture_uniform_location = context.get_uniform_location(&program, "tex");
    let rocket_tex = make_texture_bmp(&context,"Models/Ares_I_-_OBJ/Ares I/ares_I.bmp").await?;

    let planet_textures = vec![
        make_texture_bmp(&context,"Models/Earth/Textures/close0.bmp").await?,
        make_texture_bmp(&context,"Models/Earth/Textures/close1.bmp").await?,
        make_texture_bmp(&context,"Models/Earth/Textures/close2.bmp").await?,
        make_texture_bmp(&context,"Models/Earth/Textures/close3.bmp").await?,
        make_texture_bmp(&context,"Models/Earth/Textures/Diffuse_2K.bmp").await?,
    ];

    context.clear_color(1.0, 0.0, 1.0, 1.0);

    unsafe {
        GLOBAL_DATA = Some(GlobalData{
            canvas: canvas,
            ctx: context,
            program: program,
            rocket_vao: rocket_vao,
            rocket_tex: rocket_tex,
            planet_models: planet_models,
            planet_textures: planet_textures,
            mvp_location: mvp_uniform_location,
            tex_location: texture_uniform_location,
            rotate_location: rotate_uniform_location,
            rocket_vertex_count: rocket_vert_count,
            opacity_location: opacity_uniform_location,
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

    let roll = rd.mx.atan2(rd.mz) as f32;

    let z = (rd.barometer_altitude * 1.0) as f32;

    let zoom_level =
        if z < 800.0{
            (0, z / 800.0)
        } else if z < 2000.0{
            (1, (z - 800.0) / 1200.0)
        } else if z < 4000.0 {
            (2, (z - 2000.0) / 2000.0)
        } else {
            (3, (z - 4000.0) / 4000.0)
        };

    let plane_scale0 = [1E3f32, 3E3f32, 1E4f32, 3E4f32, 1E5f32][zoom_level.0];
    let plane_scale1 = [1E3f32, 3E3f32, 1E4f32, 3E4f32, 1E5f32][zoom_level.0+1];

    let rocket_rotate: glm::Mat4 = glm::rotate(&glm::identity(), roll, &glm::vec3(0.0, 0.0, 1.0));

    let rocket_model: glm::Mat4 =
        glm::translate(&glm::identity(), &glm::vec3(0.0, 0.0, z)) *
        glm::scale(&glm::identity(), &glm::vec3(0.1,0.1,0.1)) *
        rocket_rotate
        ;

    let planet_rotate: glm::Mat4 = glm::identity();

    let planet_model0: glm::Mat4 =
        glm::translate(&glm::identity(), &glm::vec3(0.0, 0.0, 250.0))
        * glm::scale(&glm::identity(), &glm::vec3(plane_scale0, plane_scale0, plane_scale0))
    ;

    let planet_model1: glm::Mat4 =
        glm::translate(&glm::identity(), &glm::vec3(0.0, 0.0, 280.0))
            * glm::scale(&glm::identity(), &glm::vec3(plane_scale1, plane_scale1, plane_scale1))
        ;

    let view: glm::Mat4 = glm::look_at(
        &glm::vec3(0.0,7.0,20.0 + z),
        &glm::vec3(0.0, 0.0, z),
        &glm::vec3(0.0,0.0,1.0)
    );

    let proj: glm::Mat4 = glm::perspective(cwidth/cheight, 45.0, 0.1, 100000.0);

    let mvp_rocket = proj * view * rocket_model;
    let mvp_planet0 = proj * view * planet_model0;
    let mvp_planet1 = proj * view * planet_model1;

    gd.ctx.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT);



    gd.ctx.bind_vertex_array(Some(&gd.rocket_vao));
    bind_shader_texture(&gd.ctx, gd.rocket_tex.as_ref(), gd.tex_location.as_ref(), 0);
    gd.ctx.uniform_matrix4fv_with_f32_array(gd.mvp_location.as_ref(), false, mvp_rocket.data.as_slice());
    gd.ctx.uniform_matrix4fv_with_f32_array(gd.rotate_location.as_ref(), false, rocket_rotate.data.as_slice());
    gd.ctx.uniform1f(gd.opacity_location.as_ref(), 1.0);
    gd.ctx.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, gd.rocket_vertex_count);

    gd.ctx.bind_vertex_array(Some(&gd.planet_models[zoom_level.0].0));
    bind_shader_texture(&gd.ctx, gd.planet_textures[zoom_level.0].as_ref(), gd.tex_location.as_ref(), 0);
    gd.ctx.uniform_matrix4fv_with_f32_array(gd.mvp_location.as_ref(), false, mvp_planet0.data.as_slice());
    gd.ctx.uniform_matrix4fv_with_f32_array(gd.rotate_location.as_ref(), false, planet_rotate.data.as_slice());
    gd.ctx.uniform1f(gd.opacity_location.as_ref(), 1.0);
    gd.ctx.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, gd.planet_models[zoom_level.0].1);

    gd.ctx.bind_vertex_array(Some(&gd.planet_models[zoom_level.0 + 1].0));
    bind_shader_texture(&gd.ctx, gd.planet_textures[zoom_level.0 + 1].as_ref(), gd.tex_location.as_ref(), 0);
    gd.ctx.uniform_matrix4fv_with_f32_array(gd.mvp_location.as_ref(), false, mvp_planet1.data.as_slice());
    gd.ctx.uniform_matrix4fv_with_f32_array(gd.rotate_location.as_ref(), false, planet_rotate.data.as_slice());
    gd.ctx.uniform1f(gd.opacity_location.as_ref(), zoom_level.1);
    gd.ctx.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, gd.planet_models[zoom_level.0+1].1);


    gd.frame_count += 1;

    unsafe {
        GLOBAL_DATA = Some(gd);
    }

    pub fn get_rocket_data_row_index(frame_count: u64) -> usize {
        let time_seconds = START_TIME_SECONDS + ((frame_count as f64) / FRAMES_PER_SECOND) * TIME_SCALE;
        return (time_seconds / ROCKET_DATA_TIMESTEP_SECONDS).round() as usize;
    }
}
