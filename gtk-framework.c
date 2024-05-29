#include "lispy.h"
#include <gtk/gtk.h>

GtkApplication* app;

void window_free(void* window) {
}

void window_new(context_t ctx, value_t* args, size_t args_len, kwargs_t, output_t ret_value) {
    GtkWidget* window = gtk_application_window_new(app);
    value_t window_val = value_new_c_value(window, window_free, ctx);
    set_return_value(ret_value, window_val);
}

void window_add(context_t ctx, value_t* args, size_t args_len, kwargs_t kwargs, output_t ret_value) {
    GtkWidget* window = value_get_c_value(args[0]);
    GtkWidget* child = value_get_c_value(args[1]);
    gtk_window_set_child(GTK_WINDOW(window), child);
    set_return_value(ret_value, value_new_nil());
}

void virtical_box_new(context_t ctx, value_t* args, size_t args_len, kwargs_t, output_t ret_value) {
    GtkWidget* box = gtk_box_new(GTK_ORIENTATION_VERTICAL, 0);
    value_t box_val = value_new_c_value(box, window_free, ctx);
    set_return_value(ret_value, box_val);
}

void virtical_box_add(context_t ctx, value_t* args, size_t args_len, kwargs_t kwargs, output_t ret_value) {
    GtkWidget* box = value_get_c_value(args[0]);
    GtkWidget* child = value_get_c_value(args[1]);
    gtk_box_append(GTK_BOX(box), child);
    set_return_value(ret_value, value_new_nil());
}

void virtical_box_get(context_t ctx, value_t* args, size_t args_len, kwargs_t kwargs, output_t ret_value) {
    GtkWidget* box = value_get_c_value(args[0]);
    int index = value_get_integer_as_i64(args[1]);
    GtkWidget* child = gtk_box_get_child(GTK_BOX(box), index);
    value_t child_val = value_new_c_value(child, window_free, ctx);
    set_return_value(ret_value, child_val);
}

void virtical_box_render(context_t ctx, value_t* args, size_t args_len, kwargs_t kwargs, output_t ret_value) {
    GtkWidget* box = value_get_c_value(args[0]);
    gtk_widget_show_all(box);
    set_return_value(ret_value, value_new_nil());
}

void button(context_t ctx, value_t* args, size_t args_len, kwargs_t kwargs, output_t ret_value) {
    GtkWidget* button = gtk_button_new_with_label("label");
    value_t button_val = value_new_c_value(button, window_free, ctx);
    set_return_value(ret_value, button_val);
}


void on_click_wrapper(GtkWidget* widget, gpointer data) {
    char** callback_data = data;
    context_t ctx = callback_data[0];
    value_t callback = callback_data[1];

}

void button_on_click_set(context_t ctx, value_t* args, size_t args_len, kwargs_t kwargs, output_t ret_value) {
    GtkWidget* button = value_get_c_value(args[0]);
    value_t callback = args[1];
    char** callback_data = malloc(2 * sizeof(char*));
    callback_data[0] = ctx;
    callback_data[1] = callback;
    g_signal_connect(button, "clicked", G_CALLBACK(on_click_wrapper), callback_data);
    set_return_value(ret_value, value_new_nil());
}
