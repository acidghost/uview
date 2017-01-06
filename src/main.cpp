#include <MicroView.h>


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


void setup(void)
{
  Serial.begin(115200);

  memset(values, 0, sizeof(values));
  currentx = 0;

	uView.begin();
	displayClear();
}


void loop(void)
{
  while (!Serial.available());

  long x = Serial.parseInt(SKIP_ALL);

  displayClear();
  drawChartAxes();

  values[currentx] = constrain(x, 0, LCDHEIGHT - (YPAD * 2));
  displayValues();
  delay(60);

  currentx = constrain(currentx + 1, 0, LCDWIDTH - 1);
  if (currentx == LCDWIDTH - 1)
    for (size_t i = 1; i < sizeof(values); i++)
      values[i-1] = values[i];
}
