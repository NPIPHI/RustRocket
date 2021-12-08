use web_sys::{window, Response};
use wasm_bindgen_futures::{JsFuture};
use wasm_bindgen::{JsValue, JsCast};
use js_sys::{JsString, ArrayBuffer};
use embedded_graphics::prelude::*;
use obj::*;
use std::io::BufReader;
use tinybmp::{Bmp, DynamicBmp};
use embedded_graphics::pixelcolor::Rgb888;

pub async fn load_file(path: &str) -> Result<Vec<u8>, JsValue> {
    let response = JsFuture::from(window().unwrap().fetch_with_str(path)).await?;

    let response = response.dyn_into::<Response>()?;

    let buff = JsFuture::from(response.array_buffer()?).await?;

    let vec = js_sys::Uint8Array::new(&buff).to_vec();

    Ok(vec)
}

pub async fn load_obj(path: &str) -> Result<ObjData, JsValue> {
    let str = load_file(path).await?;

    let reader = BufReader::new(str.as_slice());
    let obj = ObjData::load_buf(reader).ok().ok_or(JsValue::from_str("corrupt file"))?;

    return Ok(obj);
}

pub async fn load_bmp(path: &str) -> Result<(Vec<u8>, i32, i32), JsValue> {
    let file = load_file(path).await?;
    let bmp = Bmp::<Rgb888>::from_slice(file.as_slice()).ok().ok_or(JsValue::from_str("bad bmp format"))?;

    let raw = bmp.as_raw();
    Ok((raw.image_data().iter().map(|x|*x).collect(), raw.header().image_size.width as i32, raw.header().image_size.height as i32))
}

pub fn make_plane() -> (Vec<f32>, Vec<f32>, Vec<f32>){
    let verts = vec![
        -1f32, 1f32, 0f32,
        -1f32, -1f32, 0f32,
        1f32, 1f32, 0f32,
        1f32, 1f32, 0f32,
        -1f32, -1f32, 0f32,
        1f32, -1f32, 0f32,
    ];

    let norms = vec![
        0f32, 0f32, 1f32,
        0f32, 0f32, 1f32,
        0f32, 0f32, 1f32,
        0f32, 0f32, 1f32,
        0f32, 0f32, 1f32,
        0f32, 0f32, 1f32,
    ];

    let uvs = vec![
        0f32, 1f32,
        0f32, 0f32,
        1f32, 1f32,
        1f32, 1f32,
        0f32, 0f32,
        1f32, 0f32,
    ];

    return (verts, norms, uvs);
}

pub async fn load_mesh(path: &str) -> Result<(Vec<f32>, Vec<f32>, Vec<f32>), JsValue> {
    let obj = load_obj(path).await?;

    let mut verts = Vec::new();
    let mut norms = Vec::new();
    let mut uvs = Vec::new();

    for object in obj.objects {
        for group in object.groups {
            for poly in group.polys {
                let idxs = poly.0;
                for vert in 0..(idxs.len() - 2) {
                    let start_vert = idxs[0];
                    let v1 = idxs[vert + 1];
                    let v2 = idxs[vert + 2];
                    {
                        let pos_idx = start_vert.0;
                        let norm_idx = start_vert.2.unwrap();
                        let uv_idx = start_vert.1.unwrap();
                        verts.push(obj.position[pos_idx][0]);
                        verts.push(obj.position[pos_idx][1]);
                        verts.push(obj.position[pos_idx][2]);
                        norms.push(obj.normal[norm_idx][0]);
                        norms.push(obj.normal[norm_idx][1]);
                        norms.push(obj.normal[norm_idx][2]);
                        uvs.push(obj.texture[uv_idx][0]);
                        uvs.push(obj.texture[uv_idx][1]);
                    }
                    {
                        let pos_idx = v1.0;
                        let norm_idx = v1.2.unwrap();
                        let uv_idx = v1.1.unwrap();
                        verts.push(obj.position[pos_idx][0]);
                        verts.push(obj.position[pos_idx][1]);
                        verts.push(obj.position[pos_idx][2]);
                        norms.push(obj.normal[norm_idx][0]);
                        norms.push(obj.normal[norm_idx][1]);
                        norms.push(obj.normal[norm_idx][2]);
                        uvs.push(obj.texture[uv_idx][0]);
                        uvs.push(obj.texture[uv_idx][1]);
                    }
                    {
                        let pos_idx = v2.0;
                        let norm_idx = v2.2.unwrap();
                        let uv_idx = v2.1.unwrap();
                        verts.push(obj.position[pos_idx][0]);
                        verts.push(obj.position[pos_idx][1]);
                        verts.push(obj.position[pos_idx][2]);
                        norms.push(obj.normal[norm_idx][0]);
                        norms.push(obj.normal[norm_idx][1]);
                        norms.push(obj.normal[norm_idx][2]);
                        uvs.push(obj.texture[uv_idx][0]);
                        uvs.push(obj.texture[uv_idx][1]);
                    }


                }
            }
        }
    }

    return Ok((verts, norms, uvs));
}