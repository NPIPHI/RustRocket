use glm;
use glm::Vector3;
use glm::Matrix4;
use map_3d;
use num;

use crate::csvreader::RocketData;
use crate::csvreader::ROCKET_DATA;

static rocket_data_row: RocketData = RocketData::default();

pub fn get_look_at() -> Matrix4<f64> {
  let eye = Vector3::new(0.0, 0.0, 0.0);
  let center = Vector3::new(0.0, 1.0, 0.0);
  let up = Vector3::new(0.0, 1.0, 0.0);
  return glm::ext::look_at(eye, center, up);
}

pub fn get_rotate() -> Matrix4<f64> {
  glm::mat4 m4( 1.0f );
  let angle;
  let axis;
  return glm::ext::rotate();
}

pub fn get_rocket_translate() -> Matrix4<f64> {
  let ecef = map_3d::geodetic2ecef(rocket_data_row.latitude, rocket_data_row.longitude, rocket_data_row.altitude).to_vec();
  return glm::ext::translate(&num::one(), ecef);
}

pub fn get_earth_translate() -> Matrix4<f64> {
  todo!();
}

pub fn get_mvp(csv_row_num: usize) -> Matrix4<f64> {
  rocket_data_row = ROCKET_DATA[csv_row_num];
  let rocket_translate = get_rocket_translate();
  todo!();
}
