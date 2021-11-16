extern crate nalgebra_glm as glm;

use nalgebra_glm::Mat4;
use nalgebra_glm::Vec3;
use map_3d;
use crate::rocket_data::RocketData;
// static rocket_data_row: RocketData = RocketData::default();

fn get_look_at() -> Mat4 {  // TODO: Change to reflect ecef coordinates
  let eye = Vec3::new(0.0, 0.0, 0.0);
  let center = Vec3::new(0.0, 1.0, 0.0);
  let up = Vec3::new(0.0, 1.0, 0.0);
  return glm::look_at(&eye, &center, &up);
}

fn get_rotate() -> Mat4 {
  let angle = 0.0;
  let axis = Vec3::new(0.0, 1.0, 0.0);
  return glm::rotate(&glm::identity(), angle, &axis);
}

fn get_rocket_translate(rocket_data_row: &RocketData) -> Mat4 {
  let (ecef_x, ecef_y, ecef_z) = map_3d::geodetic2ecef(rocket_data_row.latitude, rocket_data_row.longitude, rocket_data_row.altitude);
  let ecef = Vec3::new(ecef_x as f32, ecef_y as f32, ecef_z as f32);
  return glm::translate(&glm::identity(), &ecef);
}

fn get_earth_translate() -> Mat4 {
  todo!();
}

fn get_scale() -> Mat4 {
  return glm::scale(&glm::identity(), &Vec3::new(1.0,1.0,1.0));
}

pub fn get_mvp(csv_row_num: usize, rocket_data: &Vec<RocketData>) -> Mat4 {
  let rocket_data_row = &rocket_data[csv_row_num];
  let rocket_translate = get_rocket_translate(rocket_data_row);
  let rotate = get_rotate();
  let scale = get_scale();
  return rocket_translate * rotate * scale;
}
