/* vim: set sw=2 expandtab tw=80: */

#include <stdio.h>
#include <timer.h>

#include "hello.h"

static void done_function(__attribute__((__unused__)) int a, __attribute__((__unused__)) int b, __attribute__((__unused__)) int c, void *user_data)
{
  bool *done = (bool *)user_data;
  printf("write done\n");
  *done = true;
}

int main(void)
{
  // while (true) {
  bool done = false;
  if (hello_subscribe(done_function, &done))
  {
    hello_print(12);

    // yield_for (&done);

    while (!done)
    {
      yield();
    }

    while (true)
    {
      printf("hello from app\n");
      delay_ms(2000);
    }
  }
  else
  {
    printf("Subscribe error\n");
  }

  // delay_ms(3000);
  // }
  return 0;
}
