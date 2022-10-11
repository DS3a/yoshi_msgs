use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone, Copy)]
pub struct MsgForEsp {
    /*
    - action
        - pwm value for each wing
     */
    pub front_left: f64,
    pub front_right: f64,
    pub back_left: f64,
    pub back_right: f64,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, Copy)]
pub struct MsgFromEsp {
    /*
    - state or imu readings
     */
    pub acc_x: f64,
    pub acc_y: f64,
    pub acc_z: f64,
}

