mod scanner;
use std::str::FromStr;

use gtk::gdk;
use gtk::prelude::*;

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.flowbox"),
        Default::default(),
    );

    application.connect_activate(build_ui);
    application.run();
}

fn build_ui(app: &gtk::Application) {
    let window = gtk::ApplicationWindow::builder()
        .default_width(360)
        .default_height(288)
        .application(app)
        .title("Gallery")
        .build();

    let flow_box = gtk::FlowBox::builder()
        .valign(gtk::Align::Start)
        .max_children_per_line(30)
        .min_children_per_line(4)
        .selection_mode(gtk::SelectionMode::None)
        .build();

    
    for i in &scanner::get_images() {
        let color_widget = create_image_button(i);
        flow_box.insert(&color_widget, -1);
    }


    let scrolled_window = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never) // Disable horizontal scrolling
        .min_content_width(360)
        .child(&flow_box)
        .build();

    window.set_child(Some(&scrolled_window));
    window.show();
}

fn create_image_button(file_path: &String) -> gtk::Button {
    //println!("{}", file_path);

    let button = gtk::Button::new();

    let image = gtk::Image::from_file (file_path);
    button.set_child(Some(&image));
    button
}
