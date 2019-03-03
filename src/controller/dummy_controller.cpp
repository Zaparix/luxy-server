#include "dummy_controller.h"

void controller_setup() {
  cout << "Dummy controller initialized!" << endl;
}

void controller_send_state(int id, bool state) {
  cout << "Change state of " << id << " to " << state << endl;
}

void controller_send_color(int id, int hue) {
  cout << "Sending hue " << hue << " to " << id << endl;
}

void controller_send_brightness(int id, int brightness) {
  cout << "Change brightness of " << id << " to " << brightness << endl;
}
