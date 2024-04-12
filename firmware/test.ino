// #include <FastLED.h>

// #define NUM_LEDS 25
// #define WS2812B_PIN 12

// CRGB leds[NUM_LEDS];

// String command;
// String commandLED;

// void setup() {  // Highest baudrate which worked consistantly. 921600 is also the ESP32 upload speed.
//   Serial.begin(9600);

//   FastLED.addLeds<WS2812B, WS2812B_PIN>(leds, NUM_LEDS);

//   for (int i = 0; i < 25; i += 4) {
//     leds[i] = CRGB::Red;
//     FastLED.show();
//     delay(30);
//     leds[i + 1] = CRGB::Green;
//     FastLED.show();
//     delay(30);
//     leds[i + 2] = CRGB::Blue;
//     FastLED.show();
//     delay(30);
//     leds[i + 3] = CRGB::Yellow;
//     FastLED.show();
//     delay(30);
//   }
// }

// void loop() {
//   if (Serial.available() > 0) {
//     char received = Serial.read();

//     if (received == '\n') {
//       Serial.println("ESP32 Received: " + command);

//       commandLED = "";
//       Serial.println("commandLED: " + commandLED);
//       commandLED = command[0] + command[1] + command[2];
//       Serial.println("commandLED 2: " + commandLED);
//       if (commandLED == "49") { // send the number '1' over a serial communication to get the leds white, otherwise they'll be random colors. 
//         for (int i = 0; i < 25; i++) {
//           leds[i] = CRGB::White;
//           FastLED.show();
//         }
//       } else {
//         for (int i = 0; i < 25; i += 4) {
//           leds[i] = CRGB::Red;
//           FastLED.show();
//           leds[i + 1] = CRGB::Green;
//           FastLED.show();
//           leds[i + 2] = CRGB::Blue;
//           FastLED.show();
//           leds[i + 3] = CRGB::Yellow;
//           FastLED.show();
//         }
//       }
//       command = "";
//     } else {
//       command += received;
//     }
//   }
// }
