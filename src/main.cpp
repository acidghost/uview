#include <assert.h>
#include <MicroView.h>


#define FONT5X7             0
#define FONT8X16            1
#define SEVENSEGMENT        2
#define FONTLARGENUMBERS    3
#define SPACE01             4
#define SPACE02             5
#define SPACE03             6

typedef enum {
  CHART = 0,
  FONT
} display_mode_t;

typedef struct {
  long value;
  display_mode_t display_mode;
} packet_t;


const uint8_t XPAD = 2;
const uint8_t YPAD = XPAD;

uint8_t values[LCDWIDTH];
uint8_t currentx;


void displayStr(uint8_t x, uint8_t y, String str)
{
  uView.setCursor(x, y);
  uView.print(str);
  uView.display();
}


void displayClear(void)
{
  uView.clear(PAGE);
	uView.display();
}


void drawChartAxes(void)
{
  // X-axis
  uView.lineH(XPAD, LCDHEIGHT - 1 - YPAD, LCDWIDTH - XPAD);
  // Y-axis
  uView.lineV(XPAD, YPAD, LCDHEIGHT - YPAD);

  const uint8_t nticks = 5;
  const uint8_t ymargin = LCDHEIGHT / nticks;
  const uint8_t xmargin = LCDWIDTH / nticks;
  for (size_t i = 1; i < nticks; i++) {
    // Y-axis ticks
    uView.lineH(0, i * ymargin, XPAD);
    // X-axis ticks
    uView.lineV(i * xmargin, LCDHEIGHT - YPAD, YPAD);
  }
}


void displayValues(void)
{
  for (size_t i = 0; i < sizeof(values); i++)
    uView.lineV(XPAD + i, LCDHEIGHT - YPAD - values[i], values[i]);
  uView.display();
}


void parseInput(String input, packet_t *packet)
{
  #define TMPSIZE 255
  const char *str = input.c_str();
  char tmp[TMPSIZE] = { 0 };

  size_t i = 1, j = 0;
  assert(str[0] == 'v');
  while (j < TMPSIZE && str[i] != 'm') {
    tmp[j] = str[i];
    i++;
    j++;
  }
  tmp[j] = '\0';
  packet->value = atol(tmp);

  // could be removed...
  assert(str[i] == 'm');

  switch (str[i+1]) {
  case '0':
    packet->display_mode = CHART;
    break;
  case '1':
    packet->display_mode = FONT;
    break;
  default:
    packet->display_mode = CHART;
  }
  #undef TMPSIZE
}


void setup(void)
{
  Serial.begin(115200);

  memset(values, 0, sizeof(values));
  currentx = 0;

  uView.begin();
  uView.setFontType(FONTLARGENUMBERS);
  displayClear();
}


void loop(void)
{
  packet_t packet;
  parseInput(Serial.readStringUntil('\n'), &packet);
  values[currentx] = constrain(packet.value, 0, LCDHEIGHT - (YPAD * 2));

  displayClear();
  switch (packet.display_mode) {
  case FONT:
    char tmp[64];
    ltoa(packet.value, tmp, 10);
    displayStr(0, 0, tmp);
    break;
  case CHART:
  default:
    drawChartAxes();
    displayValues();
  }

  delay(60);

  currentx = constrain(currentx + 1, 0, LCDWIDTH - 1);
  if (currentx == LCDWIDTH - 1)
    for (size_t i = 1; i < sizeof(values); i++)
      values[i-1] = values[i];
}
