use chrono::prelude::*;
use serde_derive::Deserialize;

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct Forecast {
    pub approvedTime: String,
    pub referenceTime: String,
    pub timeSeries: Vec<TimeSeries>,
}

#[allow(non_snake_case)]
impl Forecast {
    pub fn new(
        approvedTime: String,
        referenceTime: String,
        timeSeries: Vec<TimeSeries>,
    ) -> Forecast {
        Forecast {
            approvedTime,
            referenceTime,
            timeSeries,
        }
    }

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

    pub fn temperature(&self) -> Vec<(f32, f32)> {
        let mut temps: Vec<(f32, f32)> = Vec::new();
        let now = Local::now();
        for ts in &self.timeSeries {
            let time = ts.validTime.parse::<DateTime<Local>>().unwrap();
            let diff = time - now;

            for parameter in &ts.parameters {
                if parameter.name == "t" {
                    temps.push((parameter.values[0], diff.num_hours() as f32));
                }
            }
        }
        temps
    }
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct TimeSeries {
    pub validTime: String,
    pub parameters: Vec<Parameter>,
}

#[allow(non_snake_case)]
impl TimeSeries {
    pub fn new(validTime: String, parameters: Vec<Parameter>) -> TimeSeries {
        TimeSeries {
            validTime,
            parameters,
        }
    }
}

#[allow(non_snake_case, dead_code)]
#[derive(Deserialize, Debug)]
pub struct Parameter {
    name: String,
    levelType: String,
    level: f32,
    unit: String,
    values: Vec<f32>,
}

#[allow(non_snake_case)]
impl Parameter {
    pub fn new(
        name: String,
        levelType: String,
        level: f32,
        unit: String,
        values: Vec<f32>,
    ) -> Parameter {
        Parameter {
            name,
            levelType,
            level,
            unit,
            values,
        }
    }
}
