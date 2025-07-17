#![no_main]
#![no_std]

use core::ffi::CStr;

use flipperzero_rt::{entry, manifest};

// for panic handler
extern crate flipperzero_rt;

// for global allocator
extern crate alloc;
extern crate flipperzero_alloc;

use flipperzero::{
    dialogs::{DialogMessage, DialogMessageButton, DialogsApp},
    gui::canvas::Align,
};

manifest!(
    name = "Wiktor App",
    app_version = 1,
    has_icon = true,
    icon = "main.icon",
);

entry!(main);

fn main(_args: Option<&CStr>) -> i32 {
    let mut cur_page: usize = 0;
    let pages = [c"This is a test", c"Just a test", c"Flipper in Rust"];

    let mut dialogs = DialogsApp::open();
    let mut message = DialogMessage::new();

    loop {
        message.set_text(&pages[cur_page], 64, 32, Align::Center, Align::Center);
        let left = if cur_page > 0 { Some(c"Prev") } else { None };
        let right = if cur_page < pages.len() - 1 {
            Some(c"Next")
        } else {
            None
        };
        message.set_buttons(left, None, right);
        cur_page = match dialogs.show_message(&message) {
            DialogMessageButton::Left => cur_page - 1,
            DialogMessageButton::Right => cur_page + 1,
            DialogMessageButton::Back => return 0,
            DialogMessageButton::Center => continue,
        }
        .clamp(0, pages.len() - 1);
    }
}
