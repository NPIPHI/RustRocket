use serde::Deserialize;


#[derive(PartialEq, Debug, PartialOrd, Deserialize)]
pub struct RocketData {
    pub timestamp: u32,
    pub ax: f64,
    pub ay: f64,
    pub az: f64,
    pub gx: f64,
    pub gy: f64,
    pub gz: f64,
    pub mx: f64,
    pub my: f64,
    pub mz: f64,
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f64,
    pub satellite_count: u32,
    pub position_lock: u32,
    pub temperature: f64,
    pub pressure: f64,
    pub barometer_altitude: f64,
    pub rocket_state: u32,
    pub l1_extension: f64,
    pub l2_extension: f64,
}

impl Default for RocketData {
    fn default() -> Self {
        return RocketData {
            timestamp: 0,
            ax: 0.0,
            ay: 0.0,
            az: 0.0,
            gx: 0.0,
            gy: 0.0,
            gz: 0.0,
            mx: 0.0,
            my: 0.0,
            mz: 0.0,
            latitude: 0.0,
            longitude: 0.0,
            altitude: 0.0,
            satellite_count: 0,
            position_lock: 0,
            temperature: 0.0,
            pressure: 0.0,
            barometer_altitude: 0.0,
            rocket_state: 0,
            l1_extension: 0.0,
            l2_extension: 0.0,
        };
    }
}
