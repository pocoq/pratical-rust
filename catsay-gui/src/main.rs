use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Image, Label, Orientation, Box};

const APP_ID: &str = "org.pocoq.catsay-gui";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
	let layout_box = Box::new(Orientation::Vertical, 0);
	let label = Label::new(Some("Meow!\n \\\n \\"));
	layout_box.append(&label);

	let cat_image = Image::from_file("./images/cat.png");
	layout_box.append(&cat_image);

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Cat say")
		.height_request(70)
		.width_request(350)
        .child(&layout_box)
        .build();

    // Present window
    window.present();
}
