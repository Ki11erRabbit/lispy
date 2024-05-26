

typedef void* value_t;
typedef void* context_t;

extern value_t value_new_nil(void);
extern value_t value_new_string(const char* s, size_t len, context_t ctx);
extern value_t value_new_integer(const char*, size_t len);
extern value_t value_new_float(double f);
extern value_t value_new_boolean(char b);
extern value_t value_new_symbol(char** s, size_t len, context_t ctx);
extern value_t value_new_char(uint32_t c);
extern value_t value_new_pair(value_t car, value_t cdr, context_t ctx);
extern value_t value_new_vector(value_t* vec, size_t len, context_t ctx);
extern value_t value_new_c_value(void* value, void (*free)(void*), context_t ctx);

