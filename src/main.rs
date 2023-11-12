use weather::WeatherClient;

fn main() {
    let weather_client = WeatherClient::new(
        String::from("https://opendata-download-metfcst.smhi.se"),
        17.98,
        59.32,
    );

    let forecast = weather_client.get_forecast_string();
    println!(
        " ===== Vädret i Stockholm ===== \nTidpunkt: {}\nNuvarande temperatur: {}",
        forecast.referenceTime,
        forecast.get_current_temperature()
    );
}
