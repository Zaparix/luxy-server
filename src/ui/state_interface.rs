use rocket::Route;
use controller::bridge;

pub fn get_routes() -> Vec<Route> {
    routes![index]
}

#[derive(FromForm)]
pub struct StateChangeParameter {
    id: i32,
    state: bool
}

#[get("/switch?<params>")]
pub fn index(params: StateChangeParameter) -> String {
    bridge::send_state(params.id, params.state);
    format!("Changed state of {} to {}", params.id, params.state)
}
