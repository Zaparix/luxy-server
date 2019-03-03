use rocket::Route;
use controller::bridge;

pub fn get_routes() -> Vec<Route> {
    routes![index]
}

#[derive(FromForm)]
pub struct BrightnessParameter {
    id: i32,
    brightness: i32
}

#[get("/brightness?<params>")]
pub fn index(params: BrightnessParameter) -> String {
    bridge::send_brightness(params.id, params.brightness);
    format!("Changed brightness of {} to {}", params.id, params.brightness)
}
