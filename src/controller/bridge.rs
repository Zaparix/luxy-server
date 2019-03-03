use color::Color;

extern {
    fn controller_setup();
    fn controller_send_state(id: i32, state: bool);
    fn controller_send_color(id: i32, hue: i32);
    fn controller_send_brightness(id: i32, brightness: i32);
}

pub fn init() {
    unsafe { controller_setup() };
}

pub fn send_state(id: i32, state: bool) {
    unsafe { controller_send_state(id, state) };
}

pub fn send_color(id: i32, color: Color) {
    unsafe { controller_send_color(id, color.hue) };
}

pub fn send_brightness(id: i32, brightness: i32) {
    unsafe { controller_send_brightness(id, brightness) };
}
