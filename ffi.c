
#include <stdio.h>
#include "lispy.h"



void hello_c(void* context, void* args, size_t args_len, void* kwargs, void* ret_value) {
    printf("Hello from C\n");
    set_return_value(ret_value, value_new_nil());
}

