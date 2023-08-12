use fltk::app;

pub fn theme_button_callbacks(mut app: app::App, theme: &str) {
    if theme == "GTK"{
        app.set_scheme(app::Scheme::Gtk)
    } else if theme == "OXY" {
        app.set_scheme(app::Scheme::Oxy)
    } else if theme == "BAS"{
        app.set_scheme(app::Scheme::Base)
    } else if theme == "PLA" {
        app.set_scheme(app::Scheme::Plastic)
    } else if theme == "GLE" {
        app.set_scheme(app::Scheme::Gleam)
    }
    else {
        panic!("Theme does not exist");
    }
}