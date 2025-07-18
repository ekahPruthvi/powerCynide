use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Box as GtkBox, Orientation, Scale, Button, Label, CssProvider};
use gtk4::gdk::Display;
use gtk4_layer_shell::{LayerShell, Layer, Edge};
use std::process::exit;

fn main() {
    let app = Application::builder()
        .application_id("com.example.shutdownscreen")
        .build();

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    // Window setup
    let window = ApplicationWindow::new(app);
    window.init_layer_shell();
    window.set_layer(Layer::Overlay);
    window.auto_exclusive_zone_enable();
    window.fullscreen();
    window.set_decorated(false);
    window.set_namespace(Some("powerCynage"));

    for (edge, anchor) in [
        (Edge::Left, true),
        (Edge::Right, true),
        (Edge::Top, true),
        (Edge::Bottom, true),
    ] {
        window.set_anchor(edge, anchor);
    }

    let css = CssProvider::new();
    css.load_from_data(
        "
        window {
            background-color: rgba(20, 20, 20, 0);
        }
        scale {
            all:unset;
            min-height: 52px;
            padding: 10px;
            background: rgba(0, 0, 0, 0.51);
            border-radius: 60px;
        }

        scale trough {
            border-radius: 50px;
            background: linear-gradient(to right,rgba(243, 157, 18, 0.1),rgba(241, 30, 15, 0.16));
            min-height: 50px;
        }

        scale slider {
            background: rgb(0, 0, 0);
            border-radius: 25px;
            min-width: 80px;
            min-height: 50px;
            box-shadow: 0 0 4px rgba(0,0,0,0.4);
        }

        button {
            all:unset;
            background: rgba(0, 0, 0, 0);
            padding: 10px;
            padding-right: 30px;
            padding-left: 30px;
            border-radius: 50px;
            margin: 10px;
            transform: scale(1.0);
            transition: background 0.2s ease, transform 0.2s ease;
            font-size: 16px;
            font-weight: 500;
        }

        
        button.x:hover {
            background: rgba(255, 75, 75, 0.61);
        }

        button:hover {
            background: rgba(228, 228, 228, 0.1);
            transform: scale(1.1);
            font-weight: 800;
        }

        #buttonsbox {
            min-height: 30px;
            padding: 10px;
            padding-right: 15px; 
            padding-left: 15px;
            border-radius: 50px;
            background: rgba(0, 0, 0, 0.25);
            border: 0.5px solid rgba(255, 255, 255, 0.18);
        }

        .shadows {
            color: rgba(0, 0, 0, 0);
            min-height: 30px;
            padding: 10px;
            padding-right: 15px; 
            padding-left: 15px;
            border-radius: 50px;
            background: rgba(0, 0, 0, 0.25);
            box-shadow: rgba(0, 0, 0, 0.25) 0px 54px 55px, rgba(0, 0, 0, 0.12) 0px -12px 30px, rgba(0, 0, 0, 0.12) 0px 4px 6px, rgba(0, 0, 0, 0.17) 0px 12px 13px, rgba(0, 0, 0, 0.09) 0px -3px 5px;
        }

        @keyframes gradient {
            0% {
                background-position: 0% 50%;
            }
            50% {
                background-position: 100% 50%;
            }
            100% {
                background-position: 0% 50%;
            }
        }


        .title {
            font-size: 30px;
            font-weight: 700;
            text-shadow: black 1px 0 3px;
            color: rgba(255, 255, 255, 0.75);
        }

    ",
    );

    gtk4::style_context_add_provider_for_display(
        &Display::default().unwrap(),
        &css,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    // UI container
    let vbox = GtkBox::new(Orientation::Vertical, 20);
    vbox.set_valign(gtk4::Align::Center);
    vbox.set_halign(gtk4::Align::Center);
    vbox.set_margin_top(50);
    vbox.set_margin_bottom(50);
    vbox.set_margin_start(50);
    vbox.set_margin_end(50);

    // Slide label
    let slide_label = Label::new(Some("Slide to Shutdown"));
    slide_label.set_css_classes(&["title"]);
    slide_label.set_halign(gtk4::Align::Start);
    slide_label.set_margin_start(40);

    // Slider
    let scale = Scale::with_range(Orientation::Horizontal, 0.0, 100.0, 1.0);
    scale.set_draw_value(false);
    scale.set_hexpand(true);
    scale.set_width_request(400);
    scale.set_value(5.0);

    let scale_clone = scale.clone();
    scale.connect_value_changed(move |s| {
        let _ = std::process::Command::new("pamixer")
            .args(["--set-volume", "80"])
            .status();
        if s.value() >= 100.0 {
            shutdown_computer();
        }
    });

    // Buttons
    let hbox = GtkBox::new(Orientation::Horizontal, 10);
    hbox.set_halign(gtk4::Align::Center);
    hbox.set_widget_name("buttonsbox");

    let restart_btn = Button::with_label("Restart");

    let logout_btn = Button::with_label("Logout");

    let close_btn = Button::with_label("Close");
    close_btn.set_css_classes(&["x"]);

    restart_btn.connect_clicked(|_| {
        let _ = std::process::Command::new("pamixer")
            .args(["--set-volume", "80"])
            .status();
        restart_computer();
    });

    logout_btn.connect_clicked(|_| {
        let _ = std::process::Command::new("pamixer")
            .args(["--set-volume", "80"])
            .status();
        logout_session();
    });

    close_btn.connect_clicked(|_|{
        exit(0);
    });

    hbox.append(&scale_clone);
    hbox.append(&restart_btn);
    hbox.append(&logout_btn);
    hbox.append(&close_btn);

    vbox.append(&slide_label);
    // vbox.append(&scale_clone);
    vbox.append(&hbox);
    window.set_child(Some(&vbox));


    let shadow = ApplicationWindow::new(app);
    shadow.init_layer_shell();
    shadow.set_layer(Layer::Overlay);
    shadow.auto_exclusive_zone_enable();
    shadow.fullscreen();
    shadow.set_decorated(false);
    shadow.set_namespace(Some("powershadow"));

    for (edge, anchor) in [
        (Edge::Left, true),
        (Edge::Right, true),
        (Edge::Top, true),
        (Edge::Bottom, true),
    ] {
        shadow.set_anchor(edge, anchor);
    }

    let shodow_box = GtkBox::new(Orientation::Horizontal, 0);
    shodow_box.append(&Label::new(Some("This is supposed to be invisible")));
    shodow_box.set_css_classes(&["shadows"]);
    shodow_box.set_halign(gtk4::Align::Center);
    shodow_box.set_valign(gtk4::Align::Center);
    shodow_box.set_size_request(854, 95);
    shodow_box.set_margin_top(55);

    shadow.set_child(Some(&shodow_box));
    shadow.show();

    window.show();

}

// You can replace these with real commands (e.g., using `std::process::Command`)
fn shutdown_computer() {
    std::process::Command::new("systemctl")
        .arg("poweroff")
        .spawn()
        .expect("Failed to shutdown");
}

fn restart_computer() {
    std::process::Command::new("systemctl")
        .arg("reboot")
        .spawn()
        .expect("Failed to restart");
}

fn logout_session() {
    std::process::Command::new("hyprctl")
        .args(["dispatch", "exit", "0"])
        .spawn()
        .expect("Failed to logout");
}
