
#include <stdio.h>
#include "lispy.h"



void hello_c(void* context, void* args, size_t args_len, void* kwargs, void* ret_value) {
    printf("Hello from C\n");
    set_return_value(ret_value, value_new_nil());
}



void load_module(bindings_t bindings) {
    char** arg_names = NULL;
    size_t* arg_name_lens = NULL;
    fun_shape_t shape = new_function_shape(arg_names, 0, arg_name_lens);
    bindings_add_binding(bindings, "hello_c", 7, "hello-c", 7, shape);
}
