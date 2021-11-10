use glm;
use glm::Vector3;
use glm::Matrix4;
use map_3d;
use num;
use cgmath::One;

use crate::csvreader::RocketData;

// static rocket_data_row: RocketData = RocketData::default();

fn get_look_at() -> Matrix4<f64> {  // TODO: Change to reflect ecef coordinates
  let eye = Vector3::new(0.0, 0.0, 0.0);
  let center = Vector3::new(0.0, 1.0, 0.0);
  let up = Vector3::new(0.0, 1.0, 0.0);
  return glm::ext::look_at(eye, center, up);
}

fn get_rotate() -> Matrix4<f64> {
  let angle = 0.0;
  let axis = Vector3::new(0.0, 1.0, 0.0);
  return glm::ext::rotate(&num::one(), angle, axis);
}

fn get_rocket_translate(rocket_data_row: &RocketData) -> Matrix4<f64> {
  let (ecef_x, ecef_y, ecef_z) = map_3d::geodetic2ecef(rocket_data_row.latitude, rocket_data_row.longitude, rocket_data_row.altitude);
  let ecef = Vector3::new(ecef_x, ecef_y, ecef_z);
  return glm::ext::translate(&num::one(), ecef);
}

fn get_earth_translate() -> Matrix4<f64> {
  todo!();
}

fn get_scale() -> Matrix4<f64> {
  return glm::ext::scale(&num::one(), Vector3::one());
}

pub fn get_mvp(csv_row_num: usize, rocket_data: &Vec<RocketData>) -> Matrix4<f64> {
  let rocket_data_row = &rocket_data[csv_row_num];
  let rocket_translate = get_rocket_translate(rocket_data_row);
  let rotate = get_rotate();
  let scale = get_scale();
  return rocket_translate * rotate * scale;
}
