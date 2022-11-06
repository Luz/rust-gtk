use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Orientation};
use gtk4 as gtk;

fn main() {
    let name = env!("CARGO_PKG_NAME");
    let app = Application::builder().application_id(name).build();
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let frame: gtk::Box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    let button1 = Button::with_label("Exit(0)");
    button1.connect_clicked(|_| {
        println!("Clicked the button to exit with 0!");
        std::process::exit(0);
    });
    let button2 = Button::with_label("Exit(1)");
    button2.connect_clicked(|_| {
        println!("Clicked the button to exit with 1!");
        std::process::exit(1);
    });

    frame.append(&button1);
    frame.append(&button2);

    let name = env!("CARGO_PKG_NAME");
    let win = ApplicationWindow::builder()
        .application(app)
        .default_width(300)
        .default_height(300)
        .title(name)
        .child(&frame)
        .build();

    win.present();
}
