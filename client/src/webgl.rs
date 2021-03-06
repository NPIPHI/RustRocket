use wasm_bindgen::{JsCast, JsValue};
use web_sys::*;

pub fn get_canvas() -> Option<web_sys::HtmlCanvasElement> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    canvas.dyn_into::<web_sys::HtmlCanvasElement>().ok()
}

pub fn get_context(canvas: &web_sys::HtmlCanvasElement) -> Option<WebGl2RenderingContext> {
    canvas
        .get_context("webgl2").ok()??
        .dyn_into::<WebGl2RenderingContext>()
        .ok()
}

pub fn make_program(ctx: &WebGl2RenderingContext, vert_code: &str, frag_code: &str) -> Result<WebGlProgram, String> {

    let vert_shader = compile_shader(
        ctx,
        WebGl2RenderingContext::VERTEX_SHADER,
        vert_code
    )?;

    let frag_shader = compile_shader(
        ctx,
        WebGl2RenderingContext::FRAGMENT_SHADER,
        frag_code
    )?;

    let program = link_program(ctx, &vert_shader, &frag_shader)?;
    Ok(program)
}

pub fn make_buffer(ctx: &WebGl2RenderingContext, data: &[f32]) -> WebGlBuffer {
    let buffer = ctx.create_buffer().expect("Failed to create buffer");
    ctx.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

    unsafe {
        let positions_array_buf_view = js_sys::Float32Array::view(data);

        ctx.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &positions_array_buf_view,
            WebGl2RenderingContext::STATIC_DRAW,
        );
    }

    return buffer;
}

fn compile_shader(
    context: &WebGl2RenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

fn link_program(
    context: &WebGl2RenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}

pub fn make_vao(ctx: &WebGl2RenderingContext) -> Option<WebGlVertexArrayObject> {
    ctx.create_vertex_array()
}

pub fn make_texture(ctx: &WebGl2RenderingContext, data: &[u8], width: i32, height: i32) -> Option<WebGlTexture> {
    let texture = ctx.create_texture();
    ctx.bind_texture(WebGl2RenderingContext::TEXTURE_2D, texture.as_ref());

    ctx.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
        WebGl2RenderingContext::TEXTURE_2D, //target
        0, //level
        WebGl2RenderingContext::RGB as i32, //internal format
        width, //width
        height, //height
        0, //border
        WebGl2RenderingContext::RGB, //format
        WebGl2RenderingContext::UNSIGNED_BYTE, //type
        Some(data) //data
    ).ok()?;

    ctx.generate_mipmap(WebGl2RenderingContext::TEXTURE_2D);
// gl.NEAREST is also allowed, instead of gl.LINEAR, as neither mipmap.
    ctx.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MIN_FILTER, WebGl2RenderingContext::LINEAR as i32);
// Prevents s-coordinate wrapping (repeating).
    ctx.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_WRAP_S, WebGl2RenderingContext::CLAMP_TO_EDGE as i32);
// Prevents t-coordinate wrapping (repeating).
    ctx.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_WRAP_T, WebGl2RenderingContext::CLAMP_TO_EDGE as i32);

    return texture;
}

pub fn bind_shader_array(ctx: &WebGl2RenderingContext, buffer: Option<&WebGlBuffer>, location: u32, float_per_vertex: i32){
    ctx.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, buffer);

    ctx.vertex_attrib_pointer_with_i32(location, //index
                                           float_per_vertex, //count per vertex
                                           WebGl2RenderingContext::FLOAT,
                                           false, //normalized
                                           0, //stride bytes, 0 = default
                                           0 //offset bytes
    );

    ctx.enable_vertex_attrib_array(location);
}

pub fn bind_shader_texture(ctx: &WebGl2RenderingContext, texture: Option<&WebGlTexture>, location: Option<&web_sys::WebGlUniformLocation>, texture_binding: u32){
    ctx.active_texture(WebGl2RenderingContext::TEXTURE0 + texture_binding);
    ctx.bind_texture(WebGl2RenderingContext::TEXTURE_2D, texture);
    ctx.uniform1i(location, texture_binding as i32);
}

pub fn make_vao_vert_norm_uv(context: &WebGl2RenderingContext, program: &WebGlProgram, vertices: &Vec<f32>, normals: &Vec<f32>, uvs: &Vec<f32>) -> Result<(WebGlVertexArrayObject, i32), JsValue> {
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
