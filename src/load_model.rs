use web_sys::{window, Response};
use wasm_bindgen_futures::{JsFuture};
use wasm_bindgen::{JsValue, JsCast};
use js_sys::JsString;
use obj::*;
use std::io::BufReader;
use wasm_bindgen::__rt::IntoJsResult;

pub async fn load_file(path: &str) -> Result<String, JsValue> {
    let response = JsFuture::from(window().unwrap().fetch_with_str(path)).await?;

    let response = response.dyn_into::<Response>()?;

    let text = JsFuture::from(response.text()?).await?;

    Ok(text.dyn_into::<JsString>().unwrap().into())
}

pub async fn load_obj(path: &str) -> Result<ObjData, JsValue> {
    let str = load_file(path).await?;

    let reader = BufReader::new(str.as_bytes());
    let obj = ObjData::load_buf(reader).ok().ok_or(JsValue::from_str("corrupt file"))?;

    return Ok(obj);
}

pub async fn load_mesh(path: &str) -> Result<(Vec<f32>, Vec<f32>, Vec<f32>), JsValue> {
    let obj = load_obj(path).await?;

    let mut verts = Vec::new();
    let mut norms = Vec::new();
    let mut uvs = Vec::new();

    for object in obj.objects {
        for group in object.groups {
            for poly in group.polys {
                for vert in poly.0 {
                    let pos_idx = vert.0;
                    let uv_idx = vert.1.unwrap();
                    let norm_idx = vert.2.unwrap();
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

    return Ok((verts, norms, uvs));
}