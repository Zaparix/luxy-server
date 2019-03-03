#include "rpi_controller.h"

RF24 radio(RPI_V2_GPIO_P1_22, RPI_V2_GPIO_P1_24, BCM2835_SPI_SPEED_1MHZ);
const uint8_t addr[6] = "00001";

void controller_setup() {
  radio.begin();
  radio.setRetries(15, 15);
  radio.setDataRate(RF24_250KBPS);
  radio.setPALevel(RF24_PA_MAX);
  radio.setPayloadSize(sizeof(Color));
  radio.openWritingPipe(addr);
  radio.stopListening();
  radio.printDetails();
}

void controller_send_state(int id, bool state) {
  cout << "Change state of " << id << " to " << state << endl;
}

void controller_send_color(int id, int hue) {
  cout << "Sending hue " << hue << " to " << id << endl;

  Color payload = { hue };
  radio.write(&payload, sizeof(payload));
}

void controller_send_brightness(int id, int brightness) {
  cout << "Change brightness of " << id << " to " << brightness << endl;
}
