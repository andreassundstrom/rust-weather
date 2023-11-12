use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Forecast {
    pub approvedTime: String,
    pub referenceTime: String,
    pub timeSeries: Vec<TimeSeries>,
}

impl Forecast {
    pub fn get_current_temperature(&self) -> f32 {
        let mut temp: f32 = 0.0;
        for parameter in &self.timeSeries[0].parameters {
            if parameter.name == "t" {
                temp = parameter.values[0];
            }
        }
        temp
    }
}

#[derive(Deserialize, Debug)]
pub struct TimeSeries {
    pub validTime: String,
    pub parameters: Vec<Parameter>,
}

#[derive(Deserialize, Debug)]
pub struct Parameter {
    name: String,
    levelType: String,
    level: f32,
    unit: String,
    values: Vec<f32>,
}
