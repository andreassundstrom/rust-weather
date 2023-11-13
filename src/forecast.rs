use crate::scale::{self, scale_value};
use drawille;

pub mod json;
const WIDTH: u32 = 100;
const HEIGHT: u32 = 50;

pub struct ForecastWrapper {
    forecast: json::Forecast,
    pub min_temp: f32,
    pub max_temp: f32,
    canvas: drawille::Canvas,
    temperatures: Vec<f32>, // pub temp_y: [f32],
}

impl ForecastWrapper {
    pub fn new(forecast: json::Forecast) -> ForecastWrapper {
        let (min_temp, max_temp) = forecast.min_max_temp();
        let temperatures = forecast.temperature();
        ForecastWrapper {
            forecast,
            min_temp,
            max_temp,
            temperatures,
            canvas: drawille::Canvas::new(WIDTH, HEIGHT),
        }
    }

    pub fn draw_forecast(&mut self) {
        self.canvas.line(10, 0, 10, HEIGHT);
        self.canvas.line(10, HEIGHT, WIDTH, HEIGHT);

        let half = self.max_temp / self.min_temp;

        self.canvas.text(
            0,
            HEIGHT
                - scale_value(
                    half,
                    self.min_temp,
                    self.max_temp,
                    0,
                    (HEIGHT).try_into().unwrap(),
                ) as u32,
            10,
            &format!("{:.1}", half),
        );

        self.canvas.text(
            0,
            HEIGHT
                - scale_value(
                    self.max_temp,
                    self.min_temp,
                    self.max_temp,
                    0,
                    (HEIGHT).try_into().unwrap(),
                ) as u32,
            10,
            &format!("{:.1}", self.max_temp),
        );

        self.canvas.text(
            0,
            HEIGHT
                - scale_value(
                    self.min_temp,
                    self.min_temp,
                    self.max_temp,
                    0,
                    (HEIGHT).try_into().unwrap(),
                ) as u32,
            10,
            &format!("{:.1}", self.min_temp),
        );

        for number in 0..self.temperatures.len() {
            let scaled_y = scale::scale_value(
                self.temperatures[number],
                self.min_temp,
                self.max_temp,
                10,
                (HEIGHT - 10).try_into().unwrap(),
            );
            self.canvas
                .set((number + 10) as u32, HEIGHT - scaled_y as u32)
        }

        println!("{}", self.canvas.frame());
    }
}
