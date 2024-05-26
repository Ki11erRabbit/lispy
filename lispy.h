#include <stddef.h>
#include <stdint.h>

typedef void* value_t;
typedef void* context_t;
typedef void* exception_t;
typedef void* output_t;
typedef void* fun_shape_t;
typedef void* kwarg_t;
typedef void* frame_t;
typedef void* module_t;
typedef void* bindings_t;

extern value_t value_new_nil(void);
extern value_t value_new_string(const char* s, size_t len, context_t ctx);
extern value_t value_new_integer(const char*, size_t len);
extern value_t value_new_float(double f);
extern value_t value_new_boolean(char b);
extern value_t value_new_symbol(char** s, size_t len, size_t* str_lens, context_t ctx);
extern value_t value_new_char(uint32_t c);
extern value_t value_new_pair(value_t car, value_t cdr, context_t ctx);
extern value_t value_new_vector(value_t* vec, size_t len, context_t ctx);
extern value_t value_new_c_value(void* value, void (*free)(void*), context_t ctx);
extern value_t value_new_function(void (*fun)(context_t, value_t args, size_t arg_len, kwarg_t kwargs, output_t output), fun_shape_t shape, context_t ctx);
extern value_t value_new_bytevector(uint8_t* vec, size_t len, context_t ctx);

extern value_t value_get_string(value_t v, context_t ctx);
extern void value_free_string(char* s);
extern double value_get_float(value_t v, context_t ctx);
extern char value_get_boolean(value_t v, context_t ctx);
extern char** value_get_symbol(value_t v, context_t ctx);
extern void value_free_symbol(char** s);
extern uint32_t value_get_char(value_t v, context_t ctx);
extern value_t* value_get_pair(value_t v, context_t ctx);
extern void value_free_pair(value_t* p);
extern value_t* value_get_vector(value_t v, context_t ctx);
extern void value_free_vector(value_t* v);
extern uint8_t* value_get_bytevector(value_t v, context_t ctx);
extern void value_free_bytevector(uint8_t* v);

extern exception_t exception_new(char** who, size_t symbol_len, size_t* symbol_lens, char* message, size_t string_len, context_t ctx);

extern void set_return_value(output_t output, value_t value);
extern void set_exception_value(output_t output, exception_t value);

extern fun_shape_t new_function_shape(char** args, size_t len, size_t* str_lens);

extern void context_define(context_t ctx, char* name, size_t len, value_t value);
extern void context_push_frame(context_t ctx);
extern frame_t context_pop_frame(context_t ctx);
extern void context_add_module(context_t ctx, char* name, module_t module);

extern module_t module_new_loaded(frame_t frame);

extern void bindings_add_binding(bindings_t bindings, char* name, size_t len, char* binding, size_t binding_len, fun_shape_t shape);

// Implement this function in C
void load_module(bindings_t bindings);
