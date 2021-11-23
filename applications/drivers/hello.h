#pragma once

#include "tock.h"

#define DRIVER_NUM_HELLO 0xa0000

bool hello_subscribe(subscribe_upcall callback, void *user_data);

bool hello_print(int times);