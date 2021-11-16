
use wasm_bindgen::JsCast;
use web_sys::*;



fn get_canvas() -> Option<web_sys::HtmlCanvasElement> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    canvas.dyn_into::<web_sys::HtmlCanvasElement>().ok()
}

pub fn get_context() -> Option<WebGl2RenderingContext> {
    get_canvas()?
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

pub fn link_program(
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