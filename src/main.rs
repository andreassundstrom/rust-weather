use weather::weather_client;

fn main() {
    let weather_client = weather_client::WeatherClient::new(
        String::from("https://opendata-download-metfcst.smhi.se"),
        17.98,
        59.32,
    );

    let mut forecast = weather_client.get_forecast();
    forecast.draw_forecast();
}
