//! Set of some utility functions for creating UI elements

use gtk::prelude::*;

use crate::strings;

/// Create an [Button](gtk::Button) with an icon of the given name
pub fn create_button_with_icon(icon_name: &str) -> gtk::Button {
	gtk::Button::from_icon_name(icon_name.into(), gtk::IconSize::Button)
}

/// Create a [MenuButton](gtk::MenuButton) with an icon of the given name
pub fn create_menu_button_with_icon(icon_name: &str) -> gtk::MenuButton {
    let menu_button = gtk::MenuButton::new();
    let img = gtk::Image::new();

    img.set_from_icon_name(icon_name.into(), gtk::IconSize::Button);
    menu_button.set_image(Some(&img));

    menu_button
}

/// Show about dialog
pub fn show_about_dialog(_button: &gtk::ModelButton) {
	let about = gtk::AboutDialog::builder()
        .title(strings::TITLE)
        .version(strings::VERSION)
        .license_type(gtk::License::Gpl30)
        .authors(vec![strings::AUTHOR.into()])
        .build();

    about.show_all();
}
