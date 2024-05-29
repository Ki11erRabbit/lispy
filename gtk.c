#include <gtk/gtk.h>
#include "lispy.h"

context_t context = NULL;


void lispy_gtk_free(void* data) {}

void lispy_callback_wrapper(GtkApplication* app, gpointer user_data) {
  value_t callback = user_data;
  value_t app_val = value_new_c_value(app, lispy_gtk_free, context);
  output_t out = output_new();
  kwargs_t kwargs = kwargs_new();
  value_call_function(callback, context, &app_val, 1, kwargs, out);
  output_free(out);
  kwargs_free(kwargs);
}

void lispy_gtk_application_new(context_t ctx, value_t* args, size_t args_len, kwargs_t kwargs, output_t ret_value) {
  char* app_id = NULL;
  if (args_len == 1) {
    app_id = value_get_string(args[0], ctx);
  } else if (kwargs_len(kwargs) == 1) {
    app_id = value_get_string(kwargs_get_value(kwargs, "app-id", 6), ctx);
  } else {
    char* who[] = {"application-new"};
    size_t who_len[] = {15};
    exception_t e = exception_new(who, 1, who_len, "Invalid number of arguments", 27, ctx);
    set_exception_value(ret_value, e);
    return;
  }
  GtkApplication* app = gtk_application_new(app_id, G_APPLICATION_DEFAULT_FLAGS);
  value_t app_val = value_new_c_value(app, lispy_gtk_free, ctx);
  set_return_value(ret_value, app_val);
  //value_free_string(app_id);
}


void lispy_gtk_application_window_new(context_t ctx, value_t* args, size_t args_len, kwargs_t kwargs, output_t ret_value) {
  GtkApplication* app = NULL;
  if (args_len == 1) {
    value_t app_val = args[0];
    app = (GtkApplication*)value_get_c_value(app_val);
  } else if (kwargs_len(kwargs) == 1) {
    value_t app_val = kwargs_get_value(kwargs, "app", 3);
    app = (GtkApplication*)value_get_c_value(app_val);
  } else {
    char* who[] = {"application-window-new"};
    size_t who_len[] = {22};
    exception_t e = exception_new(who, 1, who_len, "Invalid number of arguments", 27, ctx);
    set_exception_value(ret_value, e);
    return;
  }
  GtkWidget* window = gtk_application_window_new(app);
  value_t window_val = value_new_c_value(window, lispy_gtk_free, ctx);
  set_return_value(ret_value, window_val);
}

void lispy_gtk_window_title_set(context_t ctx, value_t* args, size_t args_len, kwargs_t kwargs, output_t ret_value) {
  GtkWidget* window = NULL;
  char* title = NULL;
  if (args_len == 2) {
    value_t window_val = args[0];
    window = (GtkWidget*)value_get_c_value(window_val);
    title = value_get_string(args[1], ctx);
  } else if (args_len == 1 && kwargs_len(kwargs) == 1) {
    value_t window_val = args[0];
    window = (GtkWidget*)value_get_c_value(window_val);
    title = value_get_string(kwargs_get_value(kwargs, "title", 5), ctx);
  } else if (kwargs_len(kwargs) == 2) {
    value_t window_val = kwargs_get_value(kwargs, "window", 6);
    window = (GtkWidget*)value_get_c_value(window_val);
    title = value_get_string(kwargs_get_value(kwargs, "title", 5), ctx);
  } else {
    char* who[] = {"window-title-set!"};
    size_t who_len[] = {17};
    exception_t e = exception_new(who, 1, who_len, "Invalid number of arguments", 27, ctx);
    set_exception_value(ret_value, e);
    return;
  }
  gtk_window_set_title(GTK_WINDOW(window), title);
  set_return_value(ret_value, value_new_nil());
  //value_free_string(title);
}

void lispy_gtk_window_default_size_set(context_t ctx, value_t* args, size_t args_len, kwargs_t kwargs, output_t ret_value) {
  GtkWidget* window = NULL;
  int width, height;
  if (args_len == 3) {
    value_t window_val = args[0];
    window = (GtkWidget*)value_get_c_value(window_val);
    width = value_get_integer_as_i64(args[1]);
    height = value_get_integer_as_i64(args[2]);
  } else if (args_len == 2 && kwargs_len(kwargs) == 1) {
    value_t window_val = args[0];
    window = (GtkWidget*)value_get_c_value(window_val);
    width = value_get_integer_as_i64(args[1]);
    height = value_get_integer_as_i64(kwargs_get_value(kwargs, "height", 6));
  } else if (args_len == 1 && kwargs_len(kwargs) == 2) {
    value_t window_val = args[0];
    window = (GtkWidget*)value_get_c_value(window_val);
    width = value_get_integer_as_i64(kwargs_get_value(kwargs, "width", 5));
    height = value_get_integer_as_i64(kwargs_get_value(kwargs, "height", 6));
  } else if (kwargs_len(kwargs) == 3) {
    value_t window_val = kwargs_get_value(kwargs, "window", 6);
    window = (GtkWidget*)value_get_c_value(window_val);
    width = value_get_integer_as_i64(kwargs_get_value(kwargs, "width", 5));
    height = value_get_integer_as_i64(kwargs_get_value(kwargs, "height", 6));
  } else {
    char* who[] = {"window-default-size-set!"};
    size_t who_len[] = {24};
    exception_t e = exception_new(who, 1, who_len, "Invalid number of arguments", 27, ctx);
    set_exception_value(ret_value, e);
    return;
  }
  gtk_window_set_default_size(GTK_WINDOW(window), width, height);
  set_return_value(ret_value, value_new_nil());
}

void lispy_gtk_window_child_set(context_t ctx, value_t* args, size_t args_len, kwargs_t kwargs, output_t ret_value) {
  GtkWidget* window = NULL;
  GtkWidget* child = NULL;
  if (args_len == 2) {
    value_t window_val = args[0];
    window = (GtkWidget*)value_get_c_value(window_val);
    value_t child_val = args[1];
    child = (GtkWidget*)value_get_c_value(child_val);
  } else if (args_len == 1 && kwargs_len(kwargs) == 1) {
    value_t window_val = args[0];
    window = (GtkWidget*)value_get_c_value(window_val);
    value_t child_val = kwargs_get_value(kwargs, "child", 5);
    child = (GtkWidget*)value_get_c_value(child_val);
  } else {
    char* who[] = {"window-child-set!"};
    size_t who_len[] = {17};
    exception_t e = exception_new(who, 1, who_len, "Invalid number of arguments", 27, ctx);
    set_exception_value(ret_value, e);
    return;
  }
  gtk_window_set_child(GTK_WINDOW(window), child);
  set_return_value(ret_value, value_new_nil());
}

void lispy_gtk_window_present(context_t ctx, value_t* args, size_t args_len, kwargs_t kwargs, output_t ret_value) {
  GtkWidget* window = NULL;
  if (args_len == 1) {
    value_t window_val = args[0];
    window = (GtkWidget*)value_get_c_value(window_val);
  } else if (kwargs_len(kwargs) == 1) {
    value_t window_val = kwargs_get_value(kwargs, "window", 6);
    window = (GtkWidget*)value_get_c_value(window_val);
  } else {
    char* who[] = {"window-present"};
    size_t who_len[] = {14};
    exception_t e = exception_new(who, 1, who_len, "Invalid number of arguments", 27, ctx);
    set_exception_value(ret_value, e);
    return;
  }
  gtk_window_present(GTK_WINDOW(window));
  set_return_value(ret_value, value_new_nil());
}

void lispy_gtk_label_button_new(context_t ctx, value_t* args, size_t args_len, kwargs_t kwargs, output_t ret_value) {
  char* label = NULL;
  if (args_len == 1) {
    label = value_get_string(args[0], ctx);
  } else if (kwargs_len(kwargs) == 1) {
    label = value_get_string(kwargs_get_value(kwargs, "label", 5), ctx);
  } else {
    char* who[] = {"label-button-new"};
    size_t who_len[] = {16};
    exception_t e = exception_new(who, 1, who_len, "Invalid number of arguments", 27, ctx);
    set_exception_value(ret_value, e);
    return;
  }
  GtkWidget* button = gtk_button_new_with_label(label);
  value_t button_val = value_new_c_value(button, lispy_gtk_free, ctx);
  set_return_value(ret_value, button_val);
  //value_free_string(label);
}

void lispy_gtk_button_onclick_set(context_t ctx, value_t* args, size_t args_len, kwargs_t kwargs, output_t ret_value) {
  GtkWidget* button = NULL;
  value_t callback = NULL;
  if (args_len == 2) {
    value_t button_val = args[0];
    button = (GtkWidget*)value_get_c_value(button_val);
    callback = args[1];
  } else if (args_len == 1 && kwargs_len(kwargs) == 1) {
    value_t button_val = args[0];
    button = (GtkWidget*)value_get_c_value(button_val);
    callback = kwargs_get_value(kwargs, "callback", 8);
  } else {
    char* who[] = {"button-onclick-set!"};
    size_t who_len[] = {19};
    exception_t e = exception_new(who, 1, who_len, "Invalid number of arguments", 27, ctx);
    set_exception_value(ret_value, e);
    return;
  }
  if (context == NULL) {
    context = ctx;
  }
  
  g_signal_connect(button, "clicked", G_CALLBACK(lispy_callback_wrapper), (gpointer)callback);
  set_return_value(ret_value, value_new_nil());
}

void lispy_g_signal_connect(context_t ctx, value_t* args, size_t args_len, kwargs_t kwargs, output_t ret_value) {
  GtkWidget* widget = NULL;
  char* signal = NULL;
  value_t callback = NULL;
  if (args_len == 3) {
    value_t widget_val = args[0];
    widget = (GtkWidget*)value_get_c_value(widget_val);
    signal = value_get_string(args[1], ctx);// Causes segfault
    callback = args[2];
  } else if (args_len == 2 && kwargs_len(kwargs) == 1) {
    value_t widget_val = args[0];
    widget = (GtkWidget*)value_get_c_value(widget_val);
    signal = value_get_string(args[1], ctx);
    callback = kwargs_get_value(kwargs, "callback", 8);
  } else if (args_len == 1 && kwargs_len(kwargs) == 2) {
    value_t widget_val = args[0];
    widget = (GtkWidget*)value_get_c_value(widget_val);
    signal = value_get_string(kwargs_get_value(kwargs, "signal", 6), ctx);
    callback = kwargs_get_value(kwargs, "callback", 8);
  } else if (kwargs_len(kwargs) == 3) {
    value_t widget_val = kwargs_get_value(kwargs, "widget", 6);
    widget = (GtkWidget*)value_get_c_value(widget_val);
    signal = value_get_string(kwargs_get_value(kwargs, "signal", 6), ctx);
    callback = kwargs_get_value(kwargs, "callback", 8);
  } else {
    char* who[] = {"g-signal-connect"};
    size_t who_len[] = {16};
    exception_t e = exception_new(who, 1, who_len, "Invalid number of arguments", 27, ctx);
    set_exception_value(ret_value, e);
    return;
  }
  if (context == NULL) {
    context = ctx;
  }
	
  g_signal_connect(widget, signal, G_CALLBACK(lispy_callback_wrapper), (gpointer)callback);
  set_return_value(ret_value, value_new_nil());
}

void lispy_g_application_run(context_t ctx, value_t* args, size_t args_len, kwargs_t kwargs, output_t ret_value) {
  GtkApplication* app = NULL;
  if (args_len == 1) {
    value_t app_val = args[0];
    app = (GtkApplication*)value_get_c_value(app_val);
  } else if (kwargs_len(kwargs) == 1) {
    value_t app_val = kwargs_get_value(kwargs, "app", 3);
    app = (GtkApplication*)value_get_c_value(app_val);
  } else {
    char* who[] = {"g-application-run"};
    size_t who_len[] = {17};
    exception_t e = exception_new(who, 1, who_len, "Invalid number of arguments", 27, ctx);
    set_exception_value(ret_value, e);
    return;
  }
  int status = g_application_run(G_APPLICATION(app), 0, NULL);
  //set_return_value(ret_value, value_new_integer(status));
}


void lispy_load_module(bindings_t bindings) {
  char* arg_names_gtk_application_new[] = {"app-id"};
  size_t arg_names_len_gtk_application_new[] = {6};
  fun_shape_t fun_shape_gtk_application_new = new_function_shape(arg_names_gtk_application_new, 1, arg_names_len_gtk_application_new);
  bindings_add_binding(bindings, "lispy_gtk_application_new", 25, "application-new", 15, fun_shape_gtk_application_new);

  char* arg_names_gtk_application_window_new[] = {"app"};
  size_t arg_names_len_gtk_application_window_new[] = {3};
  fun_shape_t fun_shape_gtk_application_window_new = new_function_shape(arg_names_gtk_application_window_new, 1, arg_names_len_gtk_application_window_new);
  bindings_add_binding(bindings, "lispy_gtk_application_window_new", 32, "application-window-new", 22, fun_shape_gtk_application_window_new);

  char* arg_names_gtk_window_title_set[] = {"window", "title"};
  size_t arg_names_len_gtk_window_title_set[] = {6, 5};
  fun_shape_t fun_shape_gtk_window_title_set = new_function_shape(arg_names_gtk_window_title_set, 2, arg_names_len_gtk_window_title_set);
  bindings_add_binding(bindings, "lispy_gtk_window_title_set", 26, "window-title-set!", 17, fun_shape_gtk_window_title_set);

  char* arg_names_gtk_window_default_size_set[] = {"window", "width", "height"};
  size_t arg_names_len_gtk_window_default_size_set[] = {6, 5, 6};
  fun_shape_t fun_shape_gtk_window_default_size_set = new_function_shape(arg_names_gtk_window_default_size_set, 3, arg_names_len_gtk_window_default_size_set);
  bindings_add_binding(bindings, "lispy_gtk_window_default_size_set", 33, "window-default-size-set!", 24, fun_shape_gtk_window_default_size_set);

  char* arg_names_gtk_window_child_set[] = {"window", "child"};
  size_t arg_names_len_gtk_window_child_set[] = {6, 5};
  fun_shape_t fun_shape_gtk_window_child_set = new_function_shape(arg_names_gtk_window_child_set, 2, arg_names_len_gtk_window_child_set);
  bindings_add_binding(bindings, "lispy_gtk_window_child_set", 26, "window-child-set!", 17, fun_shape_gtk_window_child_set);

  char* arg_names_gtk_window_present[] = {"window"};
  size_t arg_names_len_gtk_window_present[] = {6};
  fun_shape_t fun_shape_gtk_window_present = new_function_shape(arg_names_gtk_window_present, 1, arg_names_len_gtk_window_present);
  bindings_add_binding(bindings, "lispy_gtk_window_present", 24, "window-present", 14, fun_shape_gtk_window_present);

  char* arg_names_gtk_label_button_new[] = {"label"};
  size_t arg_names_len_gtk_label_button_new[] = {6};
  fun_shape_t fun_shape_gtk_label_button_new = new_function_shape(arg_names_gtk_label_button_new, 1, arg_names_len_gtk_label_button_new);
  bindings_add_binding(bindings, "lispy_gtk_label_button_new", 26, "label-button-new", 16, fun_shape_gtk_label_button_new);

  char* arg_names_gtk_button_onclick_set[] = {"button", "callback"};
  size_t arg_names_len_gtk_button_onclick_set[] = {6, 8};
  fun_shape_t fun_shape_gtk_button_onclick_set = new_function_shape(arg_names_gtk_button_onclick_set, 2, arg_names_len_gtk_button_onclick_set);
  bindings_add_binding(bindings, "lispy_gtk_button_onclick_set", 28, "button-onclick-set!", 19, fun_shape_gtk_button_onclick_set);

  char* arg_names_g_signal_connect[] = {"widget", "signal", "callback"};
  size_t arg_names_len_g_signal_connect[] = {6, 6, 8};
  fun_shape_t fun_shape_g_signal_connect = new_function_shape(arg_names_g_signal_connect, 3, arg_names_len_g_signal_connect);
  bindings_add_binding(bindings, "lispy_g_signal_connect", 22, "g-signal-connect", 16, fun_shape_g_signal_connect);

  char* arg_names_g_application_run[] = {"app"};
  size_t arg_names_len_g_application_run[] = {3};
  fun_shape_t fun_shape_g_application_run = new_function_shape(arg_names_g_application_run, 1, arg_names_len_g_application_run);
  bindings_add_binding(bindings, "lispy_g_application_run", 23, "g-application-run", 17, fun_shape_g_application_run);
}
