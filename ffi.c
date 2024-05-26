
#include <stdio.h>


extern void new_value(void* ret_value, void* value);
extern void* value_new_nil();




void hello_c(void* context, void* args, size_t args_len, void* kwargs, void* ret_value) {
    printf("Hello from C\n");
    new_value(ret_value, value_new_nil());
}

