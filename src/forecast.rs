use crate::scale::{self, scale_value};
use drawille;

pub mod json;
const WIDTH: u32 = 100;
const HEIGHT: u32 = 50;
const P_BOTTOM: u32 = 10;
const P_LEFT: u32 = 10;
const P_TOP: u32 = 0;

#[allow(dead_code)]
pub struct ForecastWrapper {
    forecast: json::Forecast,
    pub min_temp: f32,
    pub max_temp: f32,
    canvas: drawille::Canvas,
    temperatures: Vec<(f32, f32)>, // temp, hours
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
        // y-axis
        self.canvas.line(P_LEFT, P_TOP, P_LEFT, HEIGHT - P_BOTTOM);

        // x-axis
        self.canvas.line(
            P_LEFT,
            P_TOP + HEIGHT - P_BOTTOM,
            WIDTH,
            P_TOP + HEIGHT - P_BOTTOM,
        );

        let half = self.max_temp / self.min_temp;

        // middle y-scale text
        self.canvas.text(
            0,
            P_TOP
                + scale_value(
                    half,
                    self.min_temp,
                    self.max_temp,
                    0,
                    (HEIGHT - P_BOTTOM - P_TOP).try_into().unwrap(),
                ) as u32,
            10,
            &format!("{:.1}", half),
        );

        // max y-scale
        self.canvas
            .text(0, P_TOP, 10, &format!("{:.1}", self.max_temp));

        // min y-scale
        self.canvas.text(
            0,
            HEIGHT - P_TOP - P_BOTTOM as u32,
            10,
            &format!("{:.1}", self.min_temp),
        );

        for number in 0..self.temperatures.len() {
            let scaled_y = scale::scale_value(
                self.temperatures[number].0,
                self.min_temp,
                self.max_temp,
                10,
                (HEIGHT - 10).try_into().unwrap(),
            );
            self.canvas
                .set((number + P_LEFT as usize) as u32, HEIGHT - scaled_y as u32);

            if number % 10 == 0 {
                let display: String;

                if self.temperatures[number].1 > 24.0 {
                    display = format!("{:.1}d", self.temperatures[number].1 / 24.0);
                } else {
                    display = format!("{:.0}h", self.temperatures[number].1);
                }

                self.canvas.text(
                    (number + P_LEFT as usize) as u32,
                    HEIGHT - P_BOTTOM + 4,
                    10,
                    &display,
                );
            }
        }

        println!("{}", self.canvas.frame());
    }
}
