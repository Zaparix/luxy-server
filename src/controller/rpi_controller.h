#include <RF24/RF24.h>
#include <iostream>
using namespace std;

struct Color {
  int hue;
};

extern "C" {
  void controller_setup();
  void controller_send_state(int id, bool state);
  void controller_send_color(int id, int hue);
  void controller_send_brightness(int id, int brightness);
}
