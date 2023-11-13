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

    pub fn min_max_temp(&self) -> (f32, f32) {
        let mut min_temp = 0.0;
        let mut max_temp = 0.0;
        for ts in &self.timeSeries {
            for parameter in &ts.parameters {
                if parameter.name == "t" {
                    if parameter.values[0] > max_temp {
                        max_temp = parameter.values[0];
                    }
                    if parameter.values[0] < min_temp {
                        min_temp = parameter.values[0];
                    }
                }
            }
        }
        (min_temp, max_temp)
    }

    pub fn temperature(&self) -> Vec<f32> {
        let mut temps: Vec<f32> = Vec::new();

        for ts in &self.timeSeries {
            for parameter in &ts.parameters {
                if parameter.name == "t" {
                    temps.push(parameter.values[0]);
                }
            }
        }
        temps
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
