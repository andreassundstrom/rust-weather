use weather::weather_client;
use windows::Devices::Geolocation::Geolocator;

fn main() {
    // Read position from windows geo location api
    let geo = Geolocator::new().unwrap();
    let geo_position = geo.GetGeopositionAsync().unwrap().get().unwrap();
    let coordinates = geo_position.Coordinate().unwrap();
    let position = coordinates.Point().unwrap().Position().unwrap();

    let weather_client = weather_client::WeatherClient::new(
        String::from("https://opendata-download-metfcst.smhi.se"),
        position.Longitude,
        position.Latitude,
    );

    let mut forecast = weather_client.get_forecast();
    let forecast_text = forecast.draw_forecast();
    println!("{}", forecast_text);
}
