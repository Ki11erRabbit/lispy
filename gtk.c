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


// Display Widgets

/* Label */

void lispy_gtk_label_new(context_t ctx, value_t* args, size_t args_len, kwargs_t kwargs, output_t ret_value) {
    char* text = NULL;
    if (args_len == 1) {
      text = value_get_string(args[0], ctx);
    } else if (kwargs_len(kwargs) == 1) {
        text = value_get_string(kwargs_get_value(kwargs, "text", 4), ctx);
    } else {
	    char* who[] = {"label-new"};
	    size_t who_len[] = {9};
	    exception_t e = exception_new(who, 1, who_len, "Invalid number of arguments", 27, ctx);
	    set_exception_value(ret_value, e);
	    return;
    }
    GtkWidget* label = gtk_label_new(text);
    value_t label_val = value_new_c_value(label, lispy_gtk_free, ctx);
    set_return_value(ret_value, label_val);
    value_free_string(text);
}

void lispy_gtk_label_text_set(context_t ctx, value_t* args, size_t args_len, kwargs_t kwargs, output_t ret_value) {
    GtkWidget* label = NULL;
    char* text = NULL;
    if (args_len == 2) {
        value_t label_val = args[0];
        label = (GtkWidget*)value_get_c_value(label_val);
        text = value_get_string(args[1], ctx);
    } else if (args_len == 1 && kwargs_len(kwargs) == 1) {
        value_t label_val = args[0];
        label = (GtkWidget*)value_get_c_value(label_val);
        text = value_get_string(kwargs_get_value(kwargs, "text", 4), ctx);
    } else {
        char* who[] = {"label-text-set!"};
        size_t who_len[] = {15};
        exception_t e = exception_new(who, 1, who_len, "Invalid number of arguments", 27, ctx);
        set_exception_value(ret_value, e);
        return;
    }
    gtk_label_set_text(GTK_LABEL(label), text);
    set_return_value(ret_value, value_new_nil());
    value_free_string(text);
}

void lispy_gtk_label_text_get(context_t ctx, value_t* args, size_t args_len, kwargs_t kwargs, output_t ret_value) {
    GtkWidget* label = NULL;
    if (args_len == 1) {
        value_t label_val = args[0];
        label = (GtkWidget*)value_get_c_value(label_val);
    } else if (kwargs_len(kwargs) == 1) {
        value_t label_val = kwargs_get_value(kwargs, "label", 5);
        label = (GtkWidget*)value_get_c_value(label_val);
    } else {
        char* who[] = {"label-text-get"};
        size_t who_len[] = {15};
        exception_t e = exception_new(who, 1, who_len, "Invalid number of arguments", 27, ctx);
        set_exception_value(ret_value, e);
        return;
    }
    const char* text = gtk_label_get_text(GTK_LABEL(label));
    set_return_value(ret_value, value_new_string(text, strlen(text),ctx));
}

void lispy_gtk_label_xalign_set(context_t ctx, value_t* args, size_t args_len, kwargs_t kwargs, output_t ret_value) {
    GtkWidget* label = NULL;
    double xalign;
    if (args_len == 2) {
        value_t label_val = args[0];
        label = (GtkWidget*)value_get_c_value(label_val);
        xalign = value_get_float(args[1], ctx);
    } else if (args_len == 1 && kwargs_len(kwargs) == 1) {
        value_t label_val = args[0];
        label = (GtkWidget*)value_get_c_value(label_val);
        xalign = value_get_float(kwargs_get_value(kwargs, "xalign", 6), ctx);
    } else {
        char* who[] = {"label-xalign-set!"};
        size_t who_len[] = {17};
        exception_t e = exception_new(who, 1, who_len, "Invalid number of arguments", 27, ctx);
        set_exception_value(ret_value, e);
        return;
    }
    gtk_label_set_xalign(GTK_LABEL(label), xalign);
    set_return_value(ret_value, value_new_nil());
}

void lispy_gtk_label_yalign_set(context_t ctx, value_t* args, size_t args_len, kwargs_t kwargs, output_t ret_value) {
    GtkWidget* label = NULL;
    double yalign;
    if (args_len == 2) {
        value_t label_val = args[0];
        label = (GtkWidget*)value_get_c_value(label_val);
        yalign = value_get_float(args[1], ctx);
    } else if (args_len == 1 && kwargs_len(kwargs) == 1) {
        value_t label_val = args[0];
        label = (GtkWidget*)value_get_c_value(label_val);
        yalign = value_get_float(kwargs_get_value(kwargs, "yalign", 6), ctx);
    } else {
        char* who[] = {"label-yalign-set!"};
        size_t who_len[] = {17};
        exception_t e = exception_new(who, 1, who_len, "Invalid number of arguments", 27, ctx);
        set_exception_value(ret_value, e);
        return;
    }
    gtk_label_set_yalign(GTK_LABEL(label), yalign);
    set_return_value(ret_value, value_new_nil());
}

/* Spinner */

void lispy_gtk_spinner_new(context_t ctx, value_t* args, size_t args_len, kwargs_t kwargs, output_t ret_value) {
    GtkWidget* spinner = gtk_spinner_new();
    value_t spinner_val = value_new_c_value(spinner, lispy_gtk_free, ctx);
    set_return_value(ret_value, spinner_val);
}

void lispy_gtk_spinner_start(context_t ctx, value_t* args, size_t args_len, kwargs_t kwargs, output_t ret_value) {
    GtkWidget* spinner = NULL;
    if (args_len == 1) {
        value_t spinner_val = args[0];
        spinner = (GtkWidget*)value_get_c_value(spinner_val);
    } else if (kwargs_len(kwargs) == 1) {
        value_t spinner_val = kwargs_get_value(kwargs, "spinner", 7);
        spinner = (GtkWidget*)value_get_c_value(spinner_val);
    } else {
        char* who[] = {"spinner-start"};
        size_t who_len[] = {13};
        exception_t e = exception_new(who, 1, who_len, "Invalid number of arguments", 27, ctx);
        set_exception_value(ret_value, e);
        return;
    }
    gtk_spinner_start(GTK_SPINNER(spinner));
    set_return_value(ret_value, value_new_nil());
}

void lispy_gtk_spinner_stop(context_t ctx, value_t* args, size_t args_len, kwargs_t kwargs, output_t ret_value) {
    GtkWidget* spinner = NULL;
    if (args_len == 1) {
        value_t spinner_val = args[0];
        spinner = (GtkWidget*)value_get_c_value(spinner_val);
    } else if (kwargs_len(kwargs) == 1) {
        value_t spinner_val = kwargs_get_value(kwargs, "spinner", 7);
        spinner = (GtkWidget*)value_get_c_value(spinner_val);
    } else {
        char* who[] = {"spinner-stop"};
        size_t who_len[] = {12};
        exception_t e = exception_new(who, 1, who_len, "Invalid number of arguments", 27, ctx);
        set_exception_value(ret_value, e);
        return;
    }
    gtk_spinner_stop(GTK_SPINNER(spinner));
    set_return_value(ret_value, value_new_nil());
}

/* Level Bar */

void lispy_gtk_level_bar_new(context_t ctx, value_t* args, size_t args_len, kwargs_t kwargs, output_t ret_value) {
    GtkWidget* level_bar = gtk_level_bar_new();
    value_t level_bar_val = value_new_c_value(level_bar, lispy_gtk_free, ctx);
    set_return_value(ret_value, level_bar_val);
}

void lispy_gtk_level_bar_value_set(context_t ctx, value_t* args, size_t args_len, kwargs_t kwargs, output_t ret_value) {
    GtkWidget* level_bar = NULL;
    double value;
    if (args_len == 2) {
        value_t level_bar_val = args[0];
        level_bar = (GtkWidget*)value_get_c_value(level_bar_val);
        value = value_get_float(args[1], ctx);
    } else if (args_len == 1 && kwargs_len(kwargs) == 1) {
        value_t level_bar_val = args[0];
        level_bar = (GtkWidget*)value_get_c_value(level_bar_val);
        value = value_get_float(kwargs_get_value(kwargs, "value", 5), ctx);
    } else {
        char* who[] = {"level-bar-value-set!"};
        size_t who_len[] = {19};
        exception_t e = exception_new(who, 1, who_len, "Invalid number of arguments", 27, ctx);
        set_exception_value(ret_value, e);
        return;
    }
    gtk_level_bar_set_value(GTK_LEVEL_BAR(level_bar), value);
    set_return_value(ret_value, value_new_nil());
}

void lispy_gtk_level_bar_value_get(context_t ctx, value_t* args, size_t args_len, kwargs_t kwargs, output_t ret_value) {
    GtkWidget* level_bar = NULL;
    if (args_len == 1) {
        value_t level_bar_val = args[0];
        level_bar = (GtkWidget*)value_get_c_value(level_bar_val);
    } else if (kwargs_len(kwargs) == 1) {
        value_t level_bar_val = kwargs_get_value(kwargs, "level-bar", 9);
        level_bar = (GtkWidget*)value_get_c_value(level_bar_val);
    } else {
        char* who[] = {"level-bar-value-get"};
        size_t who_len[] = {19};
        exception_t e = exception_new(who, 1, who_len, "Invalid number of arguments", 27, ctx);
        set_exception_value(ret_value, e);
        return;
    }
    double value = gtk_level_bar_get_value(GTK_LEVEL_BAR(level_bar));
    set_return_value(ret_value, value_new_float(value));
}

void lispy_gtk_level_bar_min_value_set(context_t ctx, value_t* args, size_t args_len, kwargs_t kwargs, output_t ret_value) {
    GtkWidget* level_bar = NULL;
    double min_value;
    if (args_len == 2) {
        value_t level_bar_val = args[0];
        level_bar = (GtkWidget*)value_get_c_value(level_bar_val);
        min_value = value_get_float(args[1], ctx);
    } else if (args_len == 1 && kwargs_len(kwargs) == 1) {
        value_t level_bar_val = args[0];
        level_bar = (GtkWidget*)value_get_c_value(level_bar_val);
        min_value = value_get_float(kwargs_get_value(kwargs, "min-value", 9), ctx);
    } else {
        char* who[] = {"level-bar-min-value-set!"};
        size_t who_len[] = {23};
        exception_t e = exception_new(who, 1, who_len, "Invalid number of arguments", 27, ctx);
        set_exception_value(ret_value, e);
        return;
    }
    gtk_level_bar_set_min_value(GTK_LEVEL_BAR(level_bar), min_value);
    set_return_value(ret_value, value_new_nil());
}

void lispy_gtk_level_bar_min_value_get(context_t ctx, value_t* args, size_t args_len, kwargs_t kwargs, output_t ret_value) {
    GtkWidget* level_bar = NULL;
    if (args_len == 1) {
        value_t level_bar_val = args[0];
        level_bar = (GtkWidget*)value_get_c_value(level_bar_val);
    } else if (kwargs_len(kwargs) == 1) {
        value_t level_bar_val = kwargs_get_value(kwargs, "level-bar", 9);
        level_bar = (GtkWidget*)value_get_c_value(level_bar_val);
    } else {
        char* who[] = {"level-bar-min-value-get"};
        size_t who_len[] = {23};
        exception_t e = exception_new(who, 1, who_len, "Invalid number of arguments", 27, ctx);
        set_exception_value(ret_value, e);
        return;
    }
    double min_value = gtk_level_bar_get_min_value(GTK_LEVEL_BAR(level_bar));
    set_return_value(ret_value, value_new_float(min_value));
}

void lispy_gtk_level_bar_max_value_set(context_t ctx, value_t* args, size_t args_len, kwargs_t kwargs, output_t ret_value) {
    GtkWidget* level_bar = NULL;
    double max_value;
    if (args_len == 2) {
        value_t level_bar_val = args[0];
        level_bar = (GtkWidget*)value_get_c_value(level_bar_val);
        max_value = value_get_float(args[1], ctx);
    } else if (args_len == 1 && kwargs_len(kwargs) == 1) {
        value_t level_bar_val = args[0];
        level_bar = (GtkWidget*)value_get_c_value(level_bar_val);
        max_value = value_get_float(kwargs_get_value(kwargs, "max-value", 9), ctx);
    } else {
        char* who[] = {"level-bar-max-value-set!"};
        size_t who_len[] = {23};
        exception_t e = exception_new(who, 1, who_len, "Invalid number of arguments", 27, ctx);
        set_exception_value(ret_value, e);
        return;
    }
    gtk_level_bar_set_max_value(GTK_LEVEL_BAR(level_bar), max_value);
    set_return_value(ret_value, value_new_nil());
}

void lispy_gtk_level_bar_max_value_get(context_t ctx, value_t* args, size_t args_len, kwargs_t kwargs, output_t ret_value) {
    GtkWidget* level_bar = NULL;
    if (args_len == 1) {
        value_t level_bar_val = args[0];
        level_bar = (GtkWidget*)value_get_c_value(level_bar_val);
    } else if (kwargs_len(kwargs) == 1) {
        value_t level_bar_val = kwargs_get_value(kwargs, "level-bar", 9);
        level_bar = (GtkWidget*)value_get_c_value(level_bar_val);
    } else {
        char* who[] = {"level-bar-max-value-get"};
        size_t who_len[] = {23};
        exception_t e = exception_new(who, 1, who_len, "Invalid number of arguments", 27, ctx);
        set_exception_value(ret_value, e);
        return;
    }
    double max_value = gtk_level_bar_get_max_value(GTK_LEVEL_BAR(level_bar));
    set_return_value(ret_value, value_new_float(max_value));
}

void lispy_gtk_level_bar_add_offset_value(context_t ctx, value_t* args, size_t args_len, kwargs_t kwargs, output_t ret_value) {
    /*GtkWidget* level_bar = NULL;
    double name, value;
    if (args_len == 3) {
        value_t level_bar_val = args[0];
        level_bar = (GtkWidget*)value_get_c_value(level_bar_val);
        name = value_get_float(args[1], ctx);
        value = value_get_float(args[2], ctx);
    } else if (args_len == 2 && kwargs_len(kwargs) == 1) {
        value_t level_bar_val = args[0];
        level_bar = (GtkWidget*)value_get_c_value(level_bar_val);
        name = value_get_float(args[1], ctx);
        value = value_get_float(kwargs_get_value(kwargs, "value", 5), ctx);
    } else {
        char* who[] = {"level-bar-add-offset-value!"};
        size_t who_len[] = {27};
        exception_t e = exception_new(who, 1, who_len, "Invalid number of arguments", 27, ctx);
        set_exception_value(ret_value, e);
        return;
    }
    gtk_level_bar_add_offset_value(GTK_LEVEL_BAR(level_bar), name, value);
    set_return_value(ret_value, value_new_nil());*/
}

void lispy_gtk_level_bar_remove_offset_value(context_t ctx, value_t* args, size_t args_len, kwargs_t kwargs, output_t ret_value) {
    /*GtkWidget* level_bar = NULL;
    double name;
    if (args_len == 2) {
        value_t level_bar_val = args[0];
        level_bar = (GtkWidget*)value_get_c_value(level_bar_val);
        name = value_get_float(args[1], ctx);
    } else if (args_len == 1 && kwargs_len(kwargs) == 1) {
        value_t level_bar_val = args[0];
        level_bar = (GtkWidget*)value_get_c_value(level_bar_val);
        name = value_get_float(kwargs_get_value(kwargs, "name", 4), ctx);
    } else {
        char* who[] = {"level-bar-remove-offset-value!"};
        size_t who_len[] = {30};
        exception_t e = exception_new(who, 1, who_len, "Invalid number of arguments", 27, ctx);
        set_exception_value(ret_value, e);
        return;
    }
    gtk_level_bar_remove_offset_value(GTK_LEVEL_BAR(level_bar), name);
    set_return_value(ret_value, value_new_nil());*/
}

/* Progress Bar */

void lispy_gtk_progress_bar_new(context_t ctx, value_t* args, size_t args_len, kwargs_t kwargs, output_t ret_value) {
    GtkWidget* progress_bar = gtk_progress_bar_new();
    value_t progress_bar_val = value_new_c_value(progress_bar, lispy_gtk_free, ctx);
    set_return_value(ret_value, progress_bar_val);
}


/* Scroll Bar */

void lispy_gtk_scroll_bar_vertical_new(context_t ctx, value_t* args, size_t args_len, kwargs_t kwargs, output_t ret_value) {
    GtkWidget* scroll_bar = gtk_scrollbar_new(GTK_ORIENTATION_VERTICAL, NULL);
    value_t scroll_bar_val = value_new_c_value(scroll_bar, lispy_gtk_free, ctx);
    set_return_value(ret_value, scroll_bar_val);
}

void lispy_gtk_scroll_bar_horizontal_new(context_t ctx, value_t* args, size_t args_len, kwargs_t kwargs, output_t ret_value) {
    GtkWidget* scroll_bar = gtk_scrollbar_new(GTK_ORIENTATION_HORIZONTAL, NULL);
    value_t scroll_bar_val = value_new_c_value(scroll_bar, lispy_gtk_free, ctx);
    set_return_value(ret_value, scroll_bar_val);
}


/* Label Button */

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

// Containers

/* Box */

void lispy_gtk_box_horizontal_new(context_t ctx, value_t* args, size_t args_len, kwargs_t kwargs, output_t ret_value) {
    GtkWidget* box = gtk_box_new(GTK_ORIENTATION_HORIZONTAL, 0);
    value_t box_val = value_new_c_value(box, lispy_gtk_free, ctx);
    set_return_value(ret_value, box_val);
}

void lispy_gtk_box_vertical_new(context_t ctx, value_t* args, size_t args_len, kwargs_t kwargs, output_t ret_value) {
    printf("Creating Box\n");
    GtkWidget* box = gtk_box_new(GTK_ORIENTATION_VERTICAL, 0);
    printf("Box: %p\n", box);
    value_t box_val = value_new_c_value(box, lispy_gtk_free, ctx);
    printf("Box Val: %p\n", box_val);
    set_return_value(ret_value, box_val);
}

void lispy_gtk_box_append(context_t ctx, value_t* args, size_t args_len, kwargs_t kwargs, output_t ret_value) {
    GtkWidget* box = NULL;
    GtkWidget* child = NULL;
    if (args_len == 2) {
        value_t box_val = args[0];
        box = (GtkWidget*)value_get_c_value(box_val);
        value_t child_val = args[1];
        child = (GtkWidget*)value_get_c_value(child_val);
    } else if (args_len == 1 && kwargs_len(kwargs) == 1) {
        value_t box_val = args[0];
        box = (GtkWidget*)value_get_c_value(box_val);
        value_t child_val = kwargs_get_value(kwargs, "child", 5);
        child = (GtkWidget*)value_get_c_value(child_val);
    } else {
        char* who[] = {"box-append"};
        size_t who_len[] = {10};
        exception_t e = exception_new(who, 1, who_len, "Invalid number of arguments", 27, ctx);
        set_exception_value(ret_value, e);
        return;
    }
    gtk_box_append(GTK_BOX(box), child);
    set_return_value(ret_value, value_new_nil());
}

void lispy_gtk_box_prepend(context_t ctx, value_t* args, size_t args_len, kwargs_t kwargs, output_t ret_value) {
    GtkWidget* box = NULL;
    GtkWidget* child = NULL;
    if (args_len == 2) {
        value_t box_val = args[0];
        box = (GtkWidget*)value_get_c_value(box_val);
        value_t child_val = args[1];
        child = (GtkWidget*)value_get_c_value(child_val);
    } else if (args_len == 1 && kwargs_len(kwargs) == 1) {
        value_t box_val = args[0];
        box = (GtkWidget*)value_get_c_value(box_val);
        value_t child_val = kwargs_get_value(kwargs, "child", 5);
        child = (GtkWidget*)value_get_c_value(child_val);
    } else {
        char* who[] = {"box-prepend"};
        size_t who_len[] = {11};
        exception_t e = exception_new(who, 1, who_len, "Invalid number of arguments", 27, ctx);
        set_exception_value(ret_value, e);
        return;
    }
    gtk_box_prepend(GTK_BOX(box), child);
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
  set_return_value(ret_value, value_new_integer_from_ssize_t(status));
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


    char* arg_names_gtk_label_new[] = {"text"};
    size_t arg_names_len_gtk_label_new[] = {4};
    fun_shape_t fun_shape_gtk_label_new = new_function_shape(arg_names_gtk_label_new, 1, arg_names_len_gtk_label_new);
    bindings_add_binding(bindings, "lispy_gtk_label_new", 19, "label-new", 9, fun_shape_gtk_label_new);

    char* arg_names_gtk_label_text_set[] = {"label", "text"};
    size_t arg_names_len_gtk_label_text_set[] = {5, 4};
    fun_shape_t fun_shape_gtk_label_text_set = new_function_shape(arg_names_gtk_label_text_set, 2, arg_names_len_gtk_label_text_set);
    bindings_add_binding(bindings, "lispy_gtk_label_text_set", 24, "label-text-set!", 15, fun_shape_gtk_label_text_set);

    char* arg_names_gtk_label_xalign_set[] = {"label", "xalign"};
    size_t arg_names_len_gtk_label_xalign_set[] = {5, 6};
    fun_shape_t fun_shape_gtk_label_xalign_set = new_function_shape(arg_names_gtk_label_xalign_set, 2, arg_names_len_gtk_label_xalign_set);
    bindings_add_binding(bindings, "lispy_gtk_label_xalign_set", 26, "label-xalign-set!", 17, fun_shape_gtk_label_xalign_set);

    char* arg_names_gtk_label_yalign_set[] = {"label", "yalign"};
    size_t arg_names_len_gtk_label_yalign_set[] = {5, 6};
    fun_shape_t fun_shape_gtk_label_yalign_set = new_function_shape(arg_names_gtk_label_yalign_set, 2, arg_names_len_gtk_label_yalign_set);
    bindings_add_binding(bindings, "lispy_gtk_label_yalign_set", 27, "label-yalign-set!", 17, fun_shape_gtk_label_yalign_set);

    char* arg_names_gtk_spinner_new[] = {};
    size_t arg_names_len_gtk_spinner_new[] = {};
    fun_shape_t fun_shape_gtk_spinner_new = new_function_shape(arg_names_gtk_spinner_new, 0, arg_names_len_gtk_spinner_new);
    bindings_add_binding(bindings, "lispy_gtk_spinner_new", 21, "spinner-new", 11, fun_shape_gtk_spinner_new);

    char* arg_names_gtk_spinner_start[] = {"spinner"};
    size_t arg_names_len_gtk_spinner_start[] = {7};
    fun_shape_t fun_shape_gtk_spinner_start = new_function_shape(arg_names_gtk_spinner_start, 1, arg_names_len_gtk_spinner_start);
    bindings_add_binding(bindings, "lispy_gtk_spinner_start", 23, "spinner-start", 13, fun_shape_gtk_spinner_start);

    char* arg_names_gtk_spinner_stop[] = {"spinner"};
    size_t arg_names_len_gtk_spinner_stop[] = {7};
    fun_shape_t fun_shape_gtk_spinner_stop = new_function_shape(arg_names_gtk_spinner_stop, 1, arg_names_len_gtk_spinner_stop);
    bindings_add_binding(bindings, "lispy_gtk_spinner_stop", 23, "spinner-stop", 12, fun_shape_gtk_spinner_stop);

    char* arg_names_gtk_level_bar_new[] = {};
    size_t arg_names_len_gtk_level_bar_new[] = {};
    fun_shape_t fun_shape_gtk_level_bar_new = new_function_shape(arg_names_gtk_level_bar_new, 0, arg_names_len_gtk_level_bar_new);
    bindings_add_binding(bindings, "lispy_gtk_level_bar_new", 23, "level-bar-new", 12, fun_shape_gtk_level_bar_new);

    char* arg_names_gtk_level_bar_value_set[] = {"level-bar", "value"};
    size_t arg_names_len_gtk_level_bar_value_set[] = {9, 5};
    fun_shape_t fun_shape_gtk_level_bar_value_set = new_function_shape(arg_names_gtk_level_bar_value_set, 2, arg_names_len_gtk_level_bar_value_set);
    bindings_add_binding(bindings, "lispy_gtk_level_bar_value_set", 29, "level-bar-value-set!", 20, fun_shape_gtk_level_bar_value_set);

    char* arg_names_gtk_level_bar_value_get[] = {"level-bar"};
    size_t arg_names_len_gtk_level_bar_value_get[] = {9};
    fun_shape_t fun_shape_gtk_level_bar_value_get = new_function_shape(arg_names_gtk_level_bar_value_get, 1, arg_names_len_gtk_level_bar_value_get);
    bindings_add_binding(bindings, "lispy_gtk_level_bar_value_get", 29, "level-bar-value-get", 20, fun_shape_gtk_level_bar_value_get);

    char* arg_names_gtk_level_bar_min_value_set[] = {"level-bar", "min-value"};
    size_t arg_names_len_gtk_level_bar_min_value_set[] = {9, 9};
    fun_shape_t fun_shape_gtk_level_bar_min_value_set = new_function_shape(arg_names_gtk_level_bar_min_value_set, 2, arg_names_len_gtk_level_bar_min_value_set);
    bindings_add_binding(bindings, "lispy_gtk_level_bar_min_value_set", 33, "level-bar-min-value-set!", 24, fun_shape_gtk_level_bar_min_value_set);

    char* arg_names_gtk_level_bar_min_value_get[] = {"level-bar"};
    size_t arg_names_len_gtk_level_bar_min_value_get[] = {9};
    fun_shape_t fun_shape_gtk_level_bar_min_value_get = new_function_shape(arg_names_gtk_level_bar_min_value_get, 1, arg_names_len_gtk_level_bar_min_value_get);
    bindings_add_binding(bindings, "lispy_gtk_level_bar_min_value_get", 33, "level-bar-min-value-get", 23, fun_shape_gtk_level_bar_min_value_get);

    char* arg_names_gtk_level_bar_max_value_set[] = {"level-bar", "max-value"};
    size_t arg_names_len_gtk_level_bar_max_value_set[] = {9, 9};
    fun_shape_t fun_shape_gtk_level_bar_max_value_set = new_function_shape(arg_names_gtk_level_bar_max_value_set, 2, arg_names_len_gtk_level_bar_max_value_set);
    bindings_add_binding(bindings, "lispy_gtk_level_bar_max_value_set", 33, "level-bar-max-value-set!", 22, fun_shape_gtk_level_bar_max_value_set);

    char* arg_names_gtk_level_bar_max_value_get[] = {"level-bar"};
    size_t arg_names_len_gtk_level_bar_max_value_get[] = {9};
    fun_shape_t fun_shape_gtk_level_bar_max_value_get = new_function_shape(arg_names_gtk_level_bar_max_value_get, 1, arg_names_len_gtk_level_bar_max_value_get);
    bindings_add_binding(bindings, "lispy_gtk_level_bar_max_value_get", 33, "level-bar-max-value-get", 23, fun_shape_gtk_level_bar_max_value_get);

    char* arg_names_gtk_level_bar_add_offset_value[] = {"level-bar", "name", "value"};
    size_t arg_names_len_gtk_level_bar_add_offset_value[] = {9, 4, 5};
    fun_shape_t fun_shape_gtk_level_bar_add_offset_value = new_function_shape(arg_names_gtk_level_bar_add_offset_value, 3, arg_names_len_gtk_level_bar_add_offset_value);
    bindings_add_binding(bindings, "lispy_gtk_level_bar_add_offset_value", 36, "level-bar-add-offset-value", 26, fun_shape_gtk_level_bar_add_offset_value);

    char* arg_names_gtk_level_bar_remove_offset_value[] = {"level-bar", "name"};
    size_t arg_names_len_gtk_level_bar_remove_offset_value[] = {9, 4};
    fun_shape_t fun_shape_gtk_level_bar_remove_offset_value = new_function_shape(arg_names_gtk_level_bar_remove_offset_value, 2, arg_names_len_gtk_level_bar_remove_offset_value);
    bindings_add_binding(bindings, "lispy_gtk_level_bar_remove_offset_value", 39, "level-bar-remove-offset-value", 29, fun_shape_gtk_level_bar_remove_offset_value);

    char* arg_names_gtk_progress_bar_new[] = {};
    size_t arg_names_len_gtk_progress_bar_new[] = {};
    fun_shape_t fun_shape_gtk_progress_bar_new = new_function_shape(arg_names_gtk_progress_bar_new, 0, arg_names_len_gtk_progress_bar_new);
    bindings_add_binding(bindings, "lispy_gtk_progress_bar_new", 26, "progress-bar-new", 16, fun_shape_gtk_progress_bar_new);


    char* arg_names_gtk_scroll_bar_vertical_new[] = {};
    size_t arg_names_len_gtk_scroll_bar_vertical_new[] = {};
    fun_shape_t fun_shape_gtk_scroll_bar_vertical_new = new_function_shape(arg_names_gtk_scroll_bar_vertical_new, 0, arg_names_len_gtk_scroll_bar_vertical_new);
    bindings_add_binding(bindings, "lispy_gtk_scroll_bar_vertical_new", 33, "scroll-bar-vertical-new", 23, fun_shape_gtk_scroll_bar_vertical_new);

    char* arg_names_gtk_scroll_bar_horizontal_new[] = {};
    size_t arg_names_len_gtk_scroll_bar_horizontal_new[] = {};
    fun_shape_t fun_shape_gtk_scroll_bar_horizontal_new = new_function_shape(arg_names_gtk_scroll_bar_horizontal_new, 0, arg_names_len_gtk_scroll_bar_horizontal_new);
    bindings_add_binding(bindings, "lispy_gtk_scroll_bar_horizontal_new", 36, "scroll-bar-horizontal-new", 25, fun_shape_gtk_scroll_bar_horizontal_new);


    

  char* arg_names_gtk_label_button_new[] = {"label"};
  size_t arg_names_len_gtk_label_button_new[] = {6};
  fun_shape_t fun_shape_gtk_label_button_new = new_function_shape(arg_names_gtk_label_button_new, 1, arg_names_len_gtk_label_button_new);
  bindings_add_binding(bindings, "lispy_gtk_label_button_new", 26, "label-button-new", 16, fun_shape_gtk_label_button_new);

  char* arg_names_gtk_button_onclick_set[] = {"button", "callback"};
  size_t arg_names_len_gtk_button_onclick_set[] = {6, 8};
  fun_shape_t fun_shape_gtk_button_onclick_set = new_function_shape(arg_names_gtk_button_onclick_set, 2, arg_names_len_gtk_button_onclick_set);
  bindings_add_binding(bindings, "lispy_gtk_button_onclick_set", 28, "button-onclick-set!", 19, fun_shape_gtk_button_onclick_set);

  // Containers

  /* Box */
  
    char* arg_names_gtk_box_horizontal_new[] = {};
    size_t arg_names_len_gtk_box_horizontal_new[] = {};
    fun_shape_t fun_shape_gtk_box_horizontal_new = new_function_shape(arg_names_gtk_box_horizontal_new, 0, arg_names_len_gtk_box_horizontal_new);
    bindings_add_binding(bindings, "lispy_gtk_box_horizontal_new", 28, "box-horizontal-new", 18, fun_shape_gtk_box_horizontal_new);

    char* arg_names_gtk_box_vertical_new[] = {};
    size_t arg_names_len_gtk_box_vertical_new[] = {};
    fun_shape_t fun_shape_gtk_box_vertical_new = new_function_shape(arg_names_gtk_box_vertical_new, 0, arg_names_len_gtk_box_vertical_new);
    bindings_add_binding(bindings, "lispy_gtk_box_vertical_new", 26, "box-vertical-new", 16, fun_shape_gtk_box_vertical_new);

    char* arg_names_gtk_box_append[] = {"box", "child"};
    size_t arg_names_len_gtk_box_append[] = {3, 5};
    fun_shape_t fun_shape_gtk_box_append = new_function_shape(arg_names_gtk_box_append, 2, arg_names_len_gtk_box_append);
    bindings_add_binding(bindings, "lispy_gtk_box_append", 20, "box-append", 10, fun_shape_gtk_box_append);

    char* arg_names_gtk_box_prepend[] = {"box", "child"};
    size_t arg_names_len_gtk_box_prepend[] = {3, 5};
    fun_shape_t fun_shape_gtk_box_prepend = new_function_shape(arg_names_gtk_box_prepend, 2, arg_names_len_gtk_box_prepend);
    bindings_add_binding(bindings, "lispy_gtk_box_prepend", 21, "box-prepend", 11, fun_shape_gtk_box_prepend);

  

  char* arg_names_g_signal_connect[] = {"widget", "signal", "callback"};
  size_t arg_names_len_g_signal_connect[] = {6, 6, 8};
  fun_shape_t fun_shape_g_signal_connect = new_function_shape(arg_names_g_signal_connect, 3, arg_names_len_g_signal_connect);
  bindings_add_binding(bindings, "lispy_g_signal_connect", 22, "g-signal-connect", 16, fun_shape_g_signal_connect);

  char* arg_names_g_application_run[] = {"app"};
  size_t arg_names_len_g_application_run[] = {3};
  fun_shape_t fun_shape_g_application_run = new_function_shape(arg_names_g_application_run, 1, arg_names_len_g_application_run);
  bindings_add_binding(bindings, "lispy_g_application_run", 23, "g-application-run", 17, fun_shape_g_application_run);
}
