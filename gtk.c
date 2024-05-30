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
    GtkWidget* level_bar = NULL;
    char* name;
    double value;
    if (args_len == 3) {
        value_t level_bar_val = args[0];
        level_bar = (GtkWidget*)value_get_c_value(level_bar_val);
        name = value_get_string(args[1], ctx);
        value = value_get_float(args[2], ctx);
    } else if (args_len == 2 && kwargs_len(kwargs) == 1) {
        value_t level_bar_val = args[0];
        level_bar = (GtkWidget*)value_get_c_value(level_bar_val);
        name = value_get_string(args[1], ctx);
        value = value_get_float(kwargs_get_value(kwargs, "value", 5), ctx);
    } else {
        char* who[] = {"level-bar-add-offset-value!"};
        size_t who_len[] = {27};
        exception_t e = exception_new(who, 1, who_len, "Invalid number of arguments", 27, ctx);
        set_exception_value(ret_value, e);
        return;
    }
    gtk_level_bar_add_offset_value(GTK_LEVEL_BAR(level_bar), name, value);
    set_return_value(ret_value, value_new_nil());
    value_free_string(name);
}

void lispy_gtk_level_bar_remove_offset_value(context_t ctx, value_t* args, size_t args_len, kwargs_t kwargs, output_t ret_value) {
    GtkWidget* level_bar = NULL;
    char* name;
    if (args_len == 2) {
        value_t level_bar_val = args[0];
        level_bar = (GtkWidget*)value_get_c_value(level_bar_val);
        name = value_get_string(args[1], ctx);
    } else if (args_len == 1 && kwargs_len(kwargs) == 1) {
        value_t level_bar_val = args[0];
        level_bar = (GtkWidget*)value_get_c_value(level_bar_val);
        name = value_get_string(kwargs_get_value(kwargs, "name", 4), ctx);
    } else {
        char* who[] = {"level-bar-remove-offset-value!"};
        size_t who_len[] = {30};
        exception_t e = exception_new(who, 1, who_len, "Invalid number of arguments", 27, ctx);
        set_exception_value(ret_value, e);
        return;
    }
    gtk_level_bar_remove_offset_value(GTK_LEVEL_BAR(level_bar), name);
    set_return_value(ret_value, value_new_nil());
}

/* Progress Bar */

void lispy_gtk_progress_bar_new(context_t ctx, value_t* args, size_t args_len, kwargs_t kwargs, output_t ret_value) {
    GtkWidget* progress_bar = gtk_progress_bar_new();
    value_t progress_bar_val = value_new_c_value(progress_bar, lispy_gtk_free, ctx);
    set_return_value(ret_value, progress_bar_val);
}

void lispy_gtk_progress_bar_fraction_set(context_t ctx, value_t* args, size_t args_len, kwargs_t kwargs, output_t ret_value) {
    GtkWidget* progress_bar = NULL;
    double fraction;
    if (args_len == 2) {
        value_t progress_bar_val = args[0];
        progress_bar = (GtkWidget*)value_get_c_value(progress_bar_val);
        fraction = value_get_float(args[1], ctx);
    } else if (args_len == 1 && kwargs_len(kwargs) == 1) {
        value_t progress_bar_val = args[0];
        progress_bar = (GtkWidget*)value_get_c_value(progress_bar_val);
        fraction = value_get_float(kwargs_get_value(kwargs, "fraction", 8), ctx);
    } else {
        char* who[] = {"progress-bar-fraction-set!"};
        size_t who_len[] = {24};
        exception_t e = exception_new(who, 1, who_len, "Invalid number of arguments", 27, ctx);
        set_exception_value(ret_value, e);
        return;
    }
    gtk_progress_bar_set_fraction(GTK_PROGRESS_BAR(progress_bar), fraction);
    set_return_value(ret_value, value_new_nil());
}

void lispy_gtk_progress_bar_fraction_get(context_t ctx, value_t* args, size_t args_len, kwargs_t kwargs, output_t ret_value) {
    GtkWidget* progress_bar = NULL;
    if (args_len == 1) {
        value_t progress_bar_val = args[0];
        progress_bar = (GtkWidget*)value_get_c_value(progress_bar_val);
    } else if (kwargs_len(kwargs) == 1) {
        value_t progress_bar_val = kwargs_get_value(kwargs, "progress-bar", 12);
        progress_bar = (GtkWidget*)value_get_c_value(progress_bar_val);
    } else {
        char* who[] = {"progress-bar-fraction-get"};
        size_t who_len[] = {24};
        exception_t e = exception_new(who, 1, who_len, "Invalid number of arguments", 27, ctx);
        set_exception_value(ret_value, e);
        return;
    }
    double fraction = gtk_progress_bar_get_fraction(GTK_PROGRESS_BAR(progress_bar));
    set_return_value(ret_value, value_new_float(fraction));
}

void lispy_gtk_progress_bar_pulse(context_t ctx, value_t* args, size_t args_len, kwargs_t kwargs, output_t ret_value) {
    GtkWidget* progress_bar = NULL;
    if (args_len == 1) {
        value_t progress_bar_val = args[0];
        progress_bar = (GtkWidget*)value_get_c_value(progress_bar_val);
    } else if (kwargs_len(kwargs) == 1) {
        value_t progress_bar_val = kwargs_get_value(kwargs, "progress-bar", 12);
        progress_bar = (GtkWidget*)value_get_c_value(progress_bar_val);
    } else {
        char* who[] = {"progress-bar-pulse"};
        size_t who_len[] = {18};
        exception_t e = exception_new(who, 1, who_len, "Invalid number of arguments", 27, ctx);
        set_exception_value(ret_value, e);
        return;
    }
    gtk_progress_bar_pulse(GTK_PROGRESS_BAR(progress_bar));
    set_return_value(ret_value, value_new_nil());
}

void lispy_gtk_progress_bar_text_set(context_t ctx, value_t* args, size_t args_len, kwargs_t kwargs, output_t ret_value) {
    GtkWidget* progress_bar = NULL;
    char* text;
    if (args_len == 2) {
        value_t progress_bar_val = args[0];
        progress_bar = (GtkWidget*)value_get_c_value(progress_bar_val);
        text = value_get_string(args[1], ctx);
    } else if (args_len == 1 && kwargs_len(kwargs) == 1) {
        value_t progress_bar_val = args[0];
        progress_bar = (GtkWidget*)value_get_c_value(progress_bar_val);
        text = value_get_string(kwargs_get_value(kwargs, "text", 4), ctx);
    } else {
        char* who[] = {"progress-bar-text-set!"};
        size_t who_len[] = {20};
        exception_t e = exception_new(who, 1, who_len, "Invalid number of arguments", 27, ctx);
        set_exception_value(ret_value, e);
        return;
    }
    gtk_progress_bar_set_text(GTK_PROGRESS_BAR(progress_bar), text);
    set_return_value(ret_value, value_new_nil());
    value_free_string(text);
}

void lispy_gtk_progress_bar_text_get(context_t ctx, value_t* args, size_t args_len, kwargs_t kwargs, output_t ret_value) {
    GtkWidget* progress_bar = NULL;
    if (args_len == 1) {
        value_t progress_bar_val = args[0];
        progress_bar = (GtkWidget*)value_get_c_value(progress_bar_val);
    } else if (kwargs_len(kwargs) == 1) {
        value_t progress_bar_val = kwargs_get_value(kwargs, "progress-bar", 12);
        progress_bar = (GtkWidget*)value_get_c_value(progress_bar_val);
    } else {
        char* who[] = {"progress-bar-text-get"};
        size_t who_len[] = {20};
        exception_t e = exception_new(who, 1, who_len, "Invalid number of arguments", 27, ctx);
        set_exception_value(ret_value, e);
        return;
    }
    const char* text = gtk_progress_bar_get_text(GTK_PROGRESS_BAR(progress_bar));
    set_return_value(ret_value, value_new_string(text, strlen(text), ctx));
}

void lispy_gtk_progress_bar_show_text_set(context_t ctx, value_t* args, size_t args_len, kwargs_t kwargs, output_t ret_value) {
    GtkWidget* progress_bar = NULL;
    bool show_text;
    if (args_len == 2) {
        value_t progress_bar_val = args[0];
        progress_bar = (GtkWidget*)value_get_c_value(progress_bar_val);
        show_text = value_get_boolean(args[1], ctx);
    } else if (args_len == 1 && kwargs_len(kwargs) == 1) {
        value_t progress_bar_val = args[0];
        progress_bar = (GtkWidget*)value_get_c_value(progress_bar_val);
        show_text = value_get_boolean(kwargs_get_value(kwargs, "show-text", 9), ctx);
    } else {
        char* who[] = {"progress-bar-show-text-set!"};
        size_t who_len[] = {26};
        exception_t e = exception_new(who, 1, who_len, "Invalid number of arguments", 27, ctx);
        set_exception_value(ret_value, e);
        return;
    }
    gtk_progress_bar_set_show_text(GTK_PROGRESS_BAR(progress_bar), show_text);
    set_return_value(ret_value, value_new_nil());
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

void lispy_gtk_scroll_bar_adjustment_get(context_t ctx, value_t* args, size_t args_len, kwargs_t kwargs, output_t ret_value) {
    GtkWidget* scroll_bar = NULL;
    if (args_len == 1) {
        value_t scroll_bar_val = args[0];
        scroll_bar = (GtkWidget*)value_get_c_value(scroll_bar_val);
    } else if (kwargs_len(kwargs) == 1) {
        value_t scroll_bar_val = kwargs_get_value(kwargs, "scroll-bar", 10);
        scroll_bar = (GtkWidget*)value_get_c_value(scroll_bar_val);
    } else {
        char* who[] = {"scroll-bar-adjustment-get"};
        size_t who_len[] = {26};
        exception_t e = exception_new(who, 1, who_len, "Invalid number of arguments", 27, ctx);
        set_exception_value(ret_value, e);
        return;
    }
    GtkAdjustment* adjustment = gtk_scrollbar_get_adjustment(GTK_SCROLLBAR(scroll_bar));
    value_t adjustment_val = value_new_c_value(adjustment, NULL, ctx);
    set_return_value(ret_value, adjustment_val);
}

void lispy_gtk_scroll_bar_adjustment_set(context_t ctx, value_t* args, size_t args_len, kwargs_t kwargs, output_t ret_value) {
    GtkWidget* scroll_bar = NULL;
    GtkAdjustment* adjustment = NULL;
    if (args_len == 2) {
        value_t scroll_bar_val = args[0];
        scroll_bar = (GtkWidget*)value_get_c_value(scroll_bar_val);
        value_t adjustment_val = args[1];
        adjustment = (GtkAdjustment*)value_get_c_value(adjustment_val);
    } else if (args_len == 1 && kwargs_len(kwargs) == 1) {
        value_t scroll_bar_val = args[0];
        scroll_bar = (GtkWidget*)value_get_c_value(scroll_bar_val);
        value_t adjustment_val = kwargs_get_value(kwargs, "adjustment", 10);
        adjustment = (GtkAdjustment*)value_get_c_value(adjustment_val);
    } else {
        char* who[] = {"scroll-bar-adjustment-set!"};
        size_t who_len[] = {26};
        exception_t e = exception_new(who, 1, who_len, "Invalid number of arguments", 27, ctx);
        set_exception_value(ret_value, e);
        return;
    }
    gtk_scrollbar_set_adjustment(GTK_SCROLLBAR(scroll_bar), adjustment);
    set_return_value(ret_value, value_new_nil());
}

/* Image */

void lispy_gtk_image_new(context_t ctx, value_t* args, size_t args_len, kwargs_t kwargs, output_t ret_value) {
    GtkWidget* image = gtk_image_new();
    value_t image_val = value_new_c_value(image, lispy_gtk_free, ctx);
    set_return_value(ret_value, image_val);
}

void lispy_gtk_image_from_file_new(context_t ctx, value_t* args, size_t args_len, kwargs_t kwargs, output_t ret_value) {
    char* filename = NULL;
    if (args_len == 1) {
        filename = value_get_string(args[0], ctx);
    } else if (kwargs_len(kwargs) == 1) {
        filename = value_get_string(kwargs_get_value(kwargs, "filename", 8), ctx);
    } else {
        char* who[] = {"image-from-file-new"};
        size_t who_len[] = {19};
        exception_t e = exception_new(who, 1, who_len, "Invalid number of arguments", 27, ctx);
        set_exception_value(ret_value, e);
        return;
    }
    GtkWidget* image = gtk_image_new_from_file(filename);
    value_t image_val = value_new_c_value(image, lispy_gtk_free, ctx);
    set_return_value(ret_value, image_val);
    value_free_string(filename);
}

void lispy_gtk_image_from_icon_name_new(context_t ctx, value_t* args, size_t args_len, kwargs_t kwargs, output_t ret_value) {
    char* icon_name = NULL;
    if (args_len == 1) {
        icon_name = value_get_string(args[0], ctx);
    } else if (kwargs_len(kwargs) == 1) {
        icon_name = value_get_string(kwargs_get_value(kwargs, "icon-name", 9), ctx);
    } else {
        char* who[] = {"image-from-icon-name-new"};
        size_t who_len[] = {23};
        exception_t e = exception_new(who, 1, who_len, "Invalid number of arguments", 27, ctx);
        set_exception_value(ret_value, e);
        return;
    }
    GtkWidget* image = gtk_image_new_from_icon_name(icon_name);
    value_t image_val = value_new_c_value(image, lispy_gtk_free, ctx);
    set_return_value(ret_value, image_val);
    value_free_string(icon_name);
}

void lispy_gtk_image_from_gicon_new(context_t ctx, value_t* args, size_t args_len, kwargs_t kwargs, output_t ret_value) {
    GIcon* icon = NULL;
    if (args_len == 1) {
        icon = (GIcon*)value_get_c_value(args[0]);
    } else if (kwargs_len(kwargs) == 1) {
        icon = (GIcon*)value_get_c_value(kwargs_get_value(kwargs, "icon", 4));
    } else {
        char* who[] = {"image-from-gicon-new"};
        size_t who_len[] = {20};
        exception_t e = exception_new(who, 1, who_len, "Invalid number of arguments", 27, ctx);
        set_exception_value(ret_value, e);
        return;
    }
    GtkWidget* image = gtk_image_new_from_gicon(icon);
    value_t image_val = value_new_c_value(image, lispy_gtk_free, ctx);
    set_return_value(ret_value, image_val);
}

void lispy_gtk_image_from_resource_new(context_t ctx, value_t* args, size_t args_len, kwargs_t kwargs, output_t ret_value) {
    /*char* resource_path = NULL;
    if (args_len == 1) {
        resource_path = value_get_string(args[0], ctx);
    } else if (kwargs_len(kwargs) == 1) {
        resource_path = value_get_string(kwargs_get_value(kwargs, "resource-path", 12), ctx);
    } else {
        char* who[] = {"image-from-resource-new"};
        size_t who_len[] = {24};
        exception_t e = exception_new(who, 1, who_len, "Invalid number of arguments", 27, ctx);
        set_exception_value(ret_value, e);
        return;
    }
    GBytes* bytes = g_resources_lookup_data(resource_path, 0, NULL);
    GInputStream* stream = g_memory_input_stream_new_from_bytes(bytes);
    GdkPixbuf* pixbuf = gdk_pixbuf_new_from_stream(stream, NULL, NULL);
    GtkWidget* image = gtk_image_new_from_pixbuf(pixbuf);
    value_t image_val = value_new_c_value(image, lispy_gtk_free, ctx);
    set_return_value(ret_value, image_val);
    value_free_string(resource_path);*/
}

void lispy_gtk_image_clear(context_t ctx, value_t* args, size_t args_len, kwargs_t kwargs, output_t ret_value) {
    GtkWidget* image = NULL;
    if (args_len == 1) {
        value_t image_val = args[0];
        image = (GtkWidget*)value_get_c_value(image_val);
    } else if (kwargs_len(kwargs) == 1) {
        value_t image_val = kwargs_get_value(kwargs, "image", 5);
        image = (GtkWidget*)value_get_c_value(image_val);
    } else {
        char* who[] = {"image-clear"};
        size_t who_len[] = {12};
        exception_t e = exception_new(who, 1, who_len, "Invalid number of arguments", 27, ctx);
        set_exception_value(ret_value, e);
        return;
    }
    gtk_image_clear(GTK_IMAGE(image));
    set_return_value(ret_value, value_new_nil());
}

void lispy_gtk_image_get_icon_name(context_t ctx, value_t* args, size_t args_len, kwargs_t kwargs, output_t ret_value) {
    GtkWidget* image = NULL;
    if (args_len == 1) {
        value_t image_val = args[0];
        image = (GtkWidget*)value_get_c_value(image_val);
    } else if (kwargs_len(kwargs) == 1) {
        value_t image_val = kwargs_get_value(kwargs, "image", 5);
        image = (GtkWidget*)value_get_c_value(image_val);
    } else {
        char* who[] = {"image-get-icon-name"};
        size_t who_len[] = {19};
        exception_t e = exception_new(who, 1, who_len, "Invalid number of arguments", 27, ctx);
        set_exception_value(ret_value, e);
        return;
    }
    const char* icon_name = gtk_image_get_icon_name(GTK_IMAGE(image));
    set_return_value(ret_value, value_new_string(icon_name, strlen(icon_name), ctx));
}

void lispy_gtk_image_get_gicon(context_t ctx, value_t* args, size_t args_len, kwargs_t kwargs, output_t ret_value) {
    GtkWidget* image = NULL;
    if (args_len == 1) {
        value_t image_val = args[0];
        image = (GtkWidget*)value_get_c_value(image_val);
    } else if (kwargs_len(kwargs) == 1) {
        value_t image_val = kwargs_get_value(kwargs, "image", 5);
        image = (GtkWidget*)value_get_c_value(image_val);
    } else {
        char* who[] = {"image-get-gicon"};
        size_t who_len[] = {15};
        exception_t e = exception_new(who, 1, who_len, "Invalid number of arguments", 27, ctx);
        set_exception_value(ret_value, e);
        return;
    }
    GIcon* icon = gtk_image_get_gicon(GTK_IMAGE(image));
    value_t icon_val = value_new_c_value(icon, NULL, ctx);
    set_return_value(ret_value, icon_val);
}

void lispy_gtk_image_icon_size_get(context_t ctx, value_t* args, size_t args_len, kwargs_t kwargs, output_t ret_value) {
    GtkWidget* image = NULL;
    if (args_len == 1) {
        value_t image_val = args[0];
        image = (GtkWidget*)value_get_c_value(image_val);
    } else if (kwargs_len(kwargs) == 1) {
        value_t image_val = kwargs_get_value(kwargs, "image", 5);
        image = (GtkWidget*)value_get_c_value(image_val);
    } else {
        char* who[] = {"image-icon-size-get"};
        size_t who_len[] = {19};
        exception_t e = exception_new(who, 1, who_len, "Invalid number of arguments", 27, ctx);
        set_exception_value(ret_value, e);
        return;
    }
    GtkIconSize icon_size = gtk_image_get_icon_size(GTK_IMAGE(image));
    set_return_value(ret_value, value_new_integer_from_ssize_t(icon_size));
}

void lispy_gtk_image_icon_size_set(context_t ctx, value_t* args, size_t args_len, kwargs_t kwargs, output_t ret_value) {
    GtkWidget* image = NULL;
    GtkIconSize icon_size;
    if (args_len == 2) {
        value_t image_val = args[0];
        image = (GtkWidget*)value_get_c_value(image_val);
        icon_size = value_get_integer_as_i64(args[1]);
    } else if (args_len == 1 && kwargs_len(kwargs) == 1) {
        value_t image_val = args[0];
        image = (GtkWidget*)value_get_c_value(image_val);
        icon_size = value_get_integer_as_i64(kwargs_get_value(kwargs, "icon-size", 9));
    } else {
        char* who[] = {"image-icon-size-set!"};
        size_t who_len[] = {19};
        exception_t e = exception_new(who, 1, who_len, "Invalid number of arguments", 27, ctx);
        set_exception_value(ret_value, e);
        return;
    }
    gtk_image_set_icon_size(GTK_IMAGE(image), icon_size);
    set_return_value(ret_value, value_new_nil());
}

void lispy_gtk_image_pixel_size_get(context_t ctx, value_t* args, size_t args_len, kwargs_t kwargs, output_t ret_value) {
    GtkWidget* image = NULL;
    if (args_len == 1) {
        value_t image_val = args[0];
        image = (GtkWidget*)value_get_c_value(image_val);
    } else if (kwargs_len(kwargs) == 1) {
        value_t image_val = kwargs_get_value(kwargs, "image", 5);
        image = (GtkWidget*)value_get_c_value(image_val);
    } else {
        char* who[] = {"image-pixel-size-get"};
        size_t who_len[] = {20};
        exception_t e = exception_new(who, 1, who_len, "Invalid number of arguments", 27, ctx);
        set_exception_value(ret_value, e);
        return;
    }
    int size = gtk_image_get_pixel_size(GTK_IMAGE(image));
    value_t size_val = value_new_integer_from_ssize_t(size);
    set_return_value(ret_value, size_val);
}

void lispy_gtk_image_pixel_size_set(context_t ctx, value_t* args, size_t args_len, kwargs_t kwargs, output_t ret_value) {
    /*GtkWidget* image = NULL;
    int width, height;
    if (args_len == 3) {
        value_t image_val = args[0];
        image = (GtkWidget*)value_get_c_value(image_val);
        width = value_get_integer_as_i64(args[1]);
        height = value_get_integer_as_i64(args[2]);
    } else if (args_len == 2 && kwargs_len(kwargs) == 2) {
        value_t image_val = args[0];
        image = (GtkWidget*)value_get_c_value(image_val);
        width = value_get_integer_as_i64(args[1]);
        height = value_get_int(kwargs_get_value(kwargs, "height", 6));
    } else {
        char* who[] = {"image-pixel-size-set!"};
        size_t who_len[] = {20};
        exception_t e = exception_new(who, 1, who_len, "Invalid number of arguments", 27, ctx);
        set_exception_value(ret_value, e);
        return;
    }
    gtk_image_set_pixel_size(GTK_IMAGE(image), width, height);
    set_return_value(ret_value, value_new_nil());*/
}

void lispy_gtk_image_from_file_set(context_t ctx, value_t* args, size_t args_len, kwargs_t kwargs, output_t ret_value) {
    GtkWidget* image = NULL;
    char* filename;
    if (args_len == 2) {
        value_t image_val = args[0];
        image = (GtkWidget*)value_get_c_value(image_val);
        filename = value_get_string(args[1], ctx);
    } else if (args_len == 1 && kwargs_len(kwargs) == 1) {
        value_t image_val = args[0];
        image = (GtkWidget*)value_get_c_value(image_val);
        filename = value_get_string(kwargs_get_value(kwargs, "filename", 8), ctx);
    } else {
        char* who[] = {"image-from-file-set!"};
        size_t who_len[] = {20};
        exception_t e = exception_new(who, 1, who_len, "Invalid number of arguments", 27, ctx);
        set_exception_value(ret_value, e);
        return;
    }
    gtk_image_set_from_file(GTK_IMAGE(image), filename);
    set_return_value(ret_value, value_new_nil());
    value_free_string(filename);
}

void lispy_gtk_image_from_icon_name_set(context_t ctx, value_t* args, size_t args_len, kwargs_t kwargs, output_t ret_value) {
    /*GtkWidget* image = NULL;
    char* icon_name;
    if (args_len == 2) {
        value_t image_val = args[0];
        image = (GtkWidget*)value_get_c_value(image_val);
        icon_name = value_get_string(args[1], ctx);
    } else if (args_len == 1 && kwargs_len(kwargs) == 1) {
        value_t image_val = args[0];
        image = (GtkWidget*)value_get_c_value(image_val);
        icon_name = value_get_string(kwargs_get_value(kwargs, "icon-name", 9), ctx);
    } else {
        char* who[] = {"image-from-icon-name-set!"};
        size_t who_len[] = {24};
        exception_t e = exception_new(who, 1, who_len, "Invalid number of arguments", 27, ctx);
        set_exception_value(ret_value, e);
        return;
    }
    gtk_image_set_from_icon_name(GTK_IMAGE(image), icon_name, GTK_ICON_SIZE_BUTTON);
    set_return_value(ret_value, value_new_nil());
    value_free_string(icon_name);*/
}

void lispy_gtk_image_from_gicon_set(context_t ctx, value_t* args, size_t args_len, kwargs_t kwargs, output_t ret_value) {
    /*GtkWidget* image = NULL;
    GIcon* icon;
    if (args_len == 2) {
        value_t image_val = args[0];
        image = (GtkWidget*)value_get_c_value(image_val);
        icon = (GIcon*)value_get_c_value(args[1]);
    } else if (args_len == 1 && kwargs_len(kwargs) == 1) {
        value_t image_val = args[0];
        image = (GtkWidget*)value_get_c_value(image_val);
        icon = (GIcon*)value_get_c_value(kwargs_get_value(kwargs, "icon", 4));
    } else {
        char* who[] = {"image-from-gicon-set!"};
        size_t who_len[] = {20};
        exception_t e = exception_new(who, 1, who_len, "Invalid number of arguments", 27, ctx);
        set_exception_value(ret_value, e);
        return;
    }
    gtk_image_set_from_gicon(GTK_IMAGE(image), icon, GTK_ICON_SIZE_BUTTON);
    set_return_value(ret_value, value_new_nil());*/
}

void lispy_gtk_image_from_resource_set(context_t ctx, value_t* args, size_t args_len, kwargs_t kwargs, output_t ret_value) {
    /*GtkWidget* image = NULL;
    char* resource_path;
    if (args_len == 2) {
        value_t image_val = args[0];
        image = (GtkWidget*)value_get_c_value(image_val);
        resource_path = value_get_string(args[1], ctx);
    } else if (args_len == 1 && kwargs_len(kwargs) == 1) {
        value_t image_val = args[0];
        image = (GtkWidget*)value_get_c_value(image_val);
        resource_path = value_get_string(kwargs_get_value(kwargs, "resource-path", 12), ctx);
    } else {
        char* who[] = {"image-from-resource-set!"};
        size_t who_len[] = {24};
        exception_t e = exception_new(who, 1, who_len, "Invalid number of arguments", 27, ctx);
        set_exception_value(ret_value, e);
        return;
    }
    GBytes* bytes = g_resources_lookup_data(resource_path, 0, NULL);
    GInputStream* stream = g_memory_input_stream_new_from_bytes(bytes);
    GdkPixbuf* pixbuf = gdk_pixbuf_new_from_stream(stream, NULL, NULL);
    gtk_image_set_from_pixbuf(GTK_IMAGE(image), pixbuf);
    set_return_value(ret_value, value_new_nil());
    value_free_string(resource_path);*/
}



/* Label Button */

void lispy_gtk_label_button_new(context_t ctx, value_t* args, size_t args_len, kwargs_t kwargs, output_t ret_value) {
    printf("Creating Label Button\n");
    char* label = NULL;
    if (args_len == 1) {
        printf("Getting Label\n");
        label = value_get_string(args[0], ctx);
    } else if (kwargs_len(kwargs) == 1) {
        printf("Getting Label\n");
        label = value_get_string(kwargs_get_value(kwargs, "label", 5), ctx);
    } else {
        char* who[] = {"label-button-new"};
        size_t who_len[] = {16};
        exception_t e = exception_new(who, 1, who_len, "Invalid number of arguments", 27, ctx);
        set_exception_value(ret_value, e);
        return;
    }
    printf("Creating Button\n");
    GtkWidget* button = gtk_button_new_with_label(label);
    value_t button_val = value_new_c_value(button, lispy_gtk_free, ctx);
    set_return_value(ret_value, button_val);
    value_free_string(label);
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
    // Display Widgets

    /* Label */
  
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

    /* Spinner */
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

    
    /* Level Bar */
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

    /* Progress Bar */
    char* arg_names_gtk_progress_bar_new[] = {};
    size_t arg_names_len_gtk_progress_bar_new[] = {};
    fun_shape_t fun_shape_gtk_progress_bar_new = new_function_shape(arg_names_gtk_progress_bar_new, 0, arg_names_len_gtk_progress_bar_new);
    bindings_add_binding(bindings, "lispy_gtk_progress_bar_new", 26, "progress-bar-new", 16, fun_shape_gtk_progress_bar_new);

    char* arg_names_gtk_progress_bar_fraction_set[] = {"progress-bar", "fraction"};
    size_t arg_names_len_gtk_progress_bar_fraction_set[] = {12, 8};
    fun_shape_t fun_shape_gtk_progress_bar_fraction_set = new_function_shape(arg_names_gtk_progress_bar_fraction_set, 2, arg_names_len_gtk_progress_bar_fraction_set);
    bindings_add_binding(bindings, "lispy_gtk_progress_bar_fraction_set", 35, "progress-bar-fraction-set!", 26, fun_shape_gtk_progress_bar_fraction_set);

    char* arg_names_gtk_progress_bar_fraction_get[] = {"progress-bar"};
    size_t arg_names_len_gtk_progress_bar_fraction_get[] = {12};
    fun_shape_t fun_shape_gtk_progress_bar_fraction_get = new_function_shape(arg_names_gtk_progress_bar_fraction_get, 1, arg_names_len_gtk_progress_bar_fraction_get);
    bindings_add_binding(bindings, "lispy_gtk_progress_bar_fraction_get", 35, "progress-bar-fraction-get", 25, fun_shape_gtk_progress_bar_fraction_get);

    char* arg_names_gtk_progress_bar_pulse[] = {"progress-bar"};
    size_t arg_names_len_gtk_progress_bar_pulse[] = {12};
    fun_shape_t fun_shape_gtk_progress_bar_pulse = new_function_shape(arg_names_gtk_progress_bar_pulse, 1, arg_names_len_gtk_progress_bar_pulse);
    bindings_add_binding(bindings, "lispy_gtk_progress_bar_pulse", 28, "progress-bar-pulse", 18, fun_shape_gtk_progress_bar_pulse);

    char* arg_names_gtk_progress_bar_text_set[] = {"progress-bar", "text"};
    size_t arg_names_len_gtk_progress_bar_text_set[] = {12, 4};
    fun_shape_t fun_shape_gtk_progress_bar_text_set = new_function_shape(arg_names_gtk_progress_bar_text_set, 2, arg_names_len_gtk_progress_bar_text_set);
    bindings_add_binding(bindings, "lispy_gtk_progress_bar_text_set", 31, "progress-bar-text-set!", 22, fun_shape_gtk_progress_bar_text_set);

    char* arg_names_gtk_progress_bar_text_get[] = {"progress-bar"};
    size_t arg_names_len_gtk_progress_bar_text_get[] = {12};
    fun_shape_t fun_shape_gtk_progress_bar_text_get = new_function_shape(arg_names_gtk_progress_bar_text_get, 1, arg_names_len_gtk_progress_bar_text_get);
    bindings_add_binding(bindings, "lispy_gtk_progress_bar_text_get", 31, "progress-bar-text-get", 21, fun_shape_gtk_progress_bar_text_get);

    char* arg_names_gtk_progress_bar_show_text_set[] = {"progress-bar", "show-text"};
    size_t arg_names_len_gtk_progress_bar_show_text_set[] = {12, 9};
    fun_shape_t fun_shape_gtk_progress_bar_show_text_set = new_function_shape(arg_names_gtk_progress_bar_show_text_set, 2, arg_names_len_gtk_progress_bar_show_text_set);
    bindings_add_binding(bindings, "lispy_gtk_progress_bar_show_text_set", 36, "progress-bar-show-text-set!", 27, fun_shape_gtk_progress_bar_show_text_set);

    
    /* Scroll Bar */
    char* arg_names_gtk_scroll_bar_vertical_new[] = {};
    size_t arg_names_len_gtk_scroll_bar_vertical_new[] = {};
    fun_shape_t fun_shape_gtk_scroll_bar_vertical_new = new_function_shape(arg_names_gtk_scroll_bar_vertical_new, 0, arg_names_len_gtk_scroll_bar_vertical_new);
    bindings_add_binding(bindings, "lispy_gtk_scroll_bar_vertical_new", 33, "scroll-bar-vertical-new", 23, fun_shape_gtk_scroll_bar_vertical_new);

    char* arg_names_gtk_scroll_bar_horizontal_new[] = {};
    size_t arg_names_len_gtk_scroll_bar_horizontal_new[] = {};
    fun_shape_t fun_shape_gtk_scroll_bar_horizontal_new = new_function_shape(arg_names_gtk_scroll_bar_horizontal_new, 0, arg_names_len_gtk_scroll_bar_horizontal_new);
    bindings_add_binding(bindings, "lispy_gtk_scroll_bar_horizontal_new", 36, "scroll-bar-horizontal-new", 25, fun_shape_gtk_scroll_bar_horizontal_new);

    char* arg_names_gtk_scroll_bar_adjustment_get[] = {"scroll-bar"};
    size_t arg_names_len_gtk_scroll_bar_adjustment_get[] = {10};
    fun_shape_t fun_shape_gtk_scroll_bar_adjustment_get = new_function_shape(arg_names_gtk_scroll_bar_adjustment_get, 1, arg_names_len_gtk_scroll_bar_adjustment_get);
    bindings_add_binding(bindings, "lispy_gtk_scroll_bar_adjustment_get", 35, "scroll-bar-adjustment-get", 25, fun_shape_gtk_scroll_bar_adjustment_get);

    char* arg_names_gtk_scroll_bar_adjustment_set[] = {"scroll-bar", "adjustment"};
    size_t arg_names_len_gtk_scroll_bar_adjustment_set[] = {10, 10};
    fun_shape_t fun_shape_gtk_scroll_bar_adjustment_set = new_function_shape(arg_names_gtk_scroll_bar_adjustment_set, 2, arg_names_len_gtk_scroll_bar_adjustment_set);
    bindings_add_binding(bindings, "lispy_gtk_scroll_bar_adjustment_set", 36, "scroll-bar-adjustment-set!", 27, fun_shape_gtk_scroll_bar_adjustment_set);

    /* Image */

    char* arg_names_gtk_image_new[] = {};
    size_t arg_names_len_gtk_image_new[] = {};
    fun_shape_t fun_shape_gtk_image_new = new_function_shape(arg_names_gtk_image_new, 0, arg_names_len_gtk_image_new);
    bindings_add_binding(bindings, "lispy_gtk_image_new", 19, "image-new", 9, fun_shape_gtk_image_new);

    char* arg_names_gtk_image_from_file_set[] = {"image", "filename"};
    size_t arg_names_len_gtk_image_from_file_set[] = {5, 8};
    fun_shape_t fun_shape_gtk_image_from_file_set = new_function_shape(arg_names_gtk_image_from_file_set, 2, arg_names_len_gtk_image_from_file_set);
    bindings_add_binding(bindings, "lispy_gtk_image_from_file_set", 29, "image-from-file-set!", 20, fun_shape_gtk_image_from_file_set);

    char* arg_names_gtk_image_from_icon_name_set[] = {"image", "icon-name"};
    size_t arg_names_len_gtk_image_from_icon_name_set[] = {5, 9};
    fun_shape_t fun_shape_gtk_image_from_icon_name_set = new_function_shape(arg_names_gtk_image_from_icon_name_set, 2, arg_names_len_gtk_image_from_icon_name_set);
    bindings_add_binding(bindings, "lispy_gtk_image_from_icon_name_set", 34, "image-from-icon-name-set!", 25, fun_shape_gtk_image_from_icon_name_set);

    char* arg_names_gtk_image_from_gicon_set[] = {"image", "icon"};
    size_t arg_names_len_gtk_image_from_gicon_set[] = {5, 4};
    fun_shape_t fun_shape_gtk_image_from_gicon_set = new_function_shape(arg_names_gtk_image_from_gicon_set, 2, arg_names_len_gtk_image_from_gicon_set);
    bindings_add_binding(bindings, "lispy_gtk_image_from_gicon_set", 30, "image-from-gicon-set!", 21, fun_shape_gtk_image_from_gicon_set);

    char* arg_names_gtk_image_from_resource_set[] = {"image", "resource-path"};
    size_t arg_names_len_gtk_image_from_resource_set[] = {5, 12};
    fun_shape_t fun_shape_gtk_image_from_resource_set = new_function_shape(arg_names_gtk_image_from_resource_set, 2, arg_names_len_gtk_image_from_resource_set);
    bindings_add_binding(bindings, "lispy_gtk_image_from_resource_set", 33, "image-from-resource-set!", 24, fun_shape_gtk_image_from_resource_set);



    /* Label Button */

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
