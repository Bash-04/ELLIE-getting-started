#include <FastLED.h>

#define NUM_LEDS 25
#define LED_PIN 12

CRGB leds[NUM_LEDS];

const int LEDS_Y = 5;
const int LEDS_X = 5;
const int totalLEDS = LEDS_Y * LEDS_X;

const int bufferSize = totalLEDS * 3;
char buffer[bufferSize];

#define RX_BUFFER_SIZE 1300  // 256 * 5 + 1 = 1281 rounded up.

int buffer_index = 1;

#define STATUS_CODE_ERROR 0
#define STATUS_CODE_TOO_SLOW 1
#define STATUS_CODE_NON_MATCHING_CRC 2
#define STATUS_CODE_UNKNOWN_COMMAND 3
#define STATUS_CODE_OK 100
#define STATUS_CODE_NEXT 101
#define STATUS_CODE_RESET 255  // Emptying all serial buffers on both master and slave side, both not sending data for ~100 ms

void empty_rx_buffer() {
  while (Serial.available() > 0) {
    char __attribute__((unused)) c = Serial.read();  // Read and discard each character
  }
}

void finalize_command(int status_code) {
  memset(buffer, 0, RX_BUFFER_SIZE);

  // Making sure that the Serial RX buffer is empty.
  // THIS SHOULD BE EMPTY, AND THUS SHOULD NOT HAPPEN
  if (Serial.available() != 0) {
    Serial.write(STATUS_CODE_RESET);
    delay(60);
    empty_rx_buffer();
    delay(20);
  } else {
    Serial.write(status_code);
  }
}

void command_0_show_leds() {
  FastLED.show();

  // finalize_command(STATUS_CODE_OK);
}

void command_1_fill_solid() {
  fill_solid(leds, totalLEDS, CRGB(buffer[1], buffer[2], buffer[3]));

  // finalize_command(STATUS_CODE_OK);
}

void command_2_update_all() {
  for (int i = 0; i < totalLEDS; i++) {
    leds[i] = CRGB(buffer[buffer_index], buffer[buffer_index += 1], buffer[buffer_index += 2]);
    buffer_index++;
  }

  buffer_index = 1;

  // finalize_command(STATUS_CODE_OK);
}

void setup() {  // Highest baudrate which worked consistantly. 921600 is also the ESP32 upload speed.
  Serial.begin(115200);

  FastLED.setBrightness(10);

  FastLED.addLeds<WS2812B, LED_PIN, GRB>(leds, NUM_LEDS);
  // fill_solid(leds, 25, CRGB(0, 0, 0));
  // FastLED.show();

  pinMode(2, OUTPUT);

  command_1_fill_solid();
  command_0_show_leds();
}

void loop() {
  // int data = Serial.read();
  int bytes = Serial.readBytes(buffer, 76);

  if (bytes == 76 || bytes == 1) {
    // if (data >= 0) {
    // Serial.write(data);
    // buffer = data;
    // buffer[0] is the command given
    Serial.write("buffer: " + buffer[0]);
    switch (buffer[0]) {
      case 0:
        command_0_show_leds();
        break;

      case 1:
        command_1_fill_solid();
        break;

      case 2:
        command_2_update_all();
        break;

      default:
        fill_solid(leds, totalLEDS, CRGB(255, 255, 255));
        command_0_show_leds();
        break;
    }
  }
}
