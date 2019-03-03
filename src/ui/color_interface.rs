use color::Color;
use rocket::Route;
use controller::bridge;

pub fn get_routes() -> Vec<Route> {
    routes![index]
}

#[derive(FromForm)]
pub struct ColorParameter {
    id: i32,
    hue: i32,
}

#[get("/color?<params>")]
pub fn index(params: ColorParameter) -> String {
    let color = Color {
        hue: params.hue,
    };

    let result = format!("Sending Hue {} to {}", color.hue, params.id);
    bridge::send_color(params.id, color);
    result
}
