#include "lispy.h"
#include <gtk/gtk.h>



static void print_hello(GtkWidget *widget, gpointer data) {
  g_print("Hello World\n");
}

static void activate(GtkApplication *app, gpointer user_data) {
    GtkWidget *window;
    GtkWidget *button;

    window = gtk_application_window_new (app);
    gtk_window_set_title (GTK_WINDOW (window), "Hello");
    gtk_window_set_default_size (GTK_WINDOW (window), 200, 200);

    button = gtk_button_new_with_label ("Hello World");
    g_signal_connect (button, "clicked", G_CALLBACK (print_hello), NULL);
    gtk_window_set_child (GTK_WINDOW (window), button);

    gtk_window_present (GTK_WINDOW (window));
}


void hello_gtk(context_t ctx, value_t* args, size_t args_len, kwargs_t kwargs, output_t ret_value) {
    printf("Hello GTK\n");
    GtkApplication *app;
    int status;
    char** argv = NULL;

    app = gtk_application_new ("org.gtk.example", G_APPLICATION_DEFAULT_FLAGS);
    g_signal_connect (app, "activate", G_CALLBACK (activate), NULL);
    status = g_application_run (G_APPLICATION (app), 0, argv);
    g_object_unref (app);

    set_return_value(ret_value, value_new_nil());
}

void lispy_load_module(bindings_t bindings) {
    char** arg_names = NULL;
    size_t* arg_name_lens = NULL;
    fun_shape_t shape = new_function_shape(arg_names, 0, arg_name_lens);
    bindings_add_binding(bindings, "hello_gtk", 9, "hello-gtk", 9, shape);
}

