use rocket::Route;
use ui::state_interface;
use ui::color_interface;
use ui::brightness_interface;

pub fn get_routes() -> Vec<Route> {
    let mut route = vec![];
    route.append(&mut state_interface::get_routes());
    route.append(&mut color_interface::get_routes());
    route.append(&mut brightness_interface::get_routes());

    route
}
