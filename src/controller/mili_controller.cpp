#include "mili_controller.h"

void controller_setup() {
  cout << "Initializing MiLi interface" << endl;

  setup_mili();
}

void controller_send_state(int id, bool state) {
  cout << "Change state of " << id << " to " << state << endl;

  switch_state(id, state);
}

void controller_send_color(int id, int hue) {
  cout << "Sending hue " << hue << " to " << id << endl;

  set_color(id, hue);
}

void controller_send_brightness(int id, int brightness) {
  cout << "Change brightness of " << id << " to " << brightness << endl;

  set_brightness(id, brightness);
}
