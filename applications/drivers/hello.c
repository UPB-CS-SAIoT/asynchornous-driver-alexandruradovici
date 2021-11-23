#include "tock.h"
#include "hello.h"

bool hello_subscribe(subscribe_upcall callback, void *user_data)
{
    subscribe_return_t subr = subscribe(DRIVER_NUM_HELLO, 0, callback, user_data);
    return subr.status == TOCK_STATUSCODE_SUCCESS;
}

bool hello_print(int times)
{
    syscall_return_t r = command(DRIVER_NUM_HELLO, 1, times, 0);
    return r.type == TOCK_SYSCALL_SUCCESS;
}