#![allow(warnings)]

use iced::widget;
use iced::{Sandbox, Settings};

// You can run every app you're developing in 2 ways:
// - cargo run
// - cargo watch -x run
// (Obviously, there are more ways. :D But you don't need to worry about that now.)

fn main() {
    Clicker::run(Settings::default());
}

struct Clicker {
    // Application State
    text: String,
}

impl Sandbox for Clicker {
    type Message = u8;

    // Here is where you should return the initial state of your app.
    fn new() -> Self {
        println!("App created");
        Self {
            text: "Hello Alicja!".to_string(),
        }
    }

    fn title(&self) -> String {
        println!("Title set");
        "Clicker on the Rocks".to_string()
    }

    // Application View
    // - how your app looks like
    // - what's there? buttons, text fields, etc...
    fn view(&self) -> iced::Element<'_, Self::Message> {
        let text_widget = widget::text(&self.text);
        let button_widget = widget::button("Say \"Hi\" back").on_press(13);

        println!("View rendered");
        // widget::row![text_widget, button_widget].into();

        // You can find other widgets here: https://docs.rs/iced/latest/iced/widget/index.html
        // Feel free to experiment. Just pay attention that you don't know how to send a Message
        // that contains addition data, like integers, &strs etc... We'll get to that next time.

        // Keep experimenting! ^-^
        let row1 = widget::row![widget::container(button_widget).padding(5)];
        // let row2 = widget::row![widget::text("Alicja ma małego siusiaka")];

        // let radio: widget::Radio<Self::Message> = widget::radio("dupa", 32, Some(12), |x| 3);

        widget::column![row1, text_widget].into()
    }

    // Application Update
    // - this is initated by interactions with View elements (widgets like buttons, sliders, etc...)
    // - this can potentially change the application state
    // - if the application state was changed the View will be updated accordingly
    fn update(&mut self, message: Self::Message) {
        println!("Message {} received by update method", message);
        if message == 13 {
            self.text = "Hello, Iced!".to_string();
        }
        println!("App state updated");
    }

    fn theme(&self) -> iced::Theme {
        println!("Theme set");
        iced::Theme::TokyoNightStorm
    }
}

// General loop looks like this:  View <=> Update
// more explicitly:
// - if you interact with view -> the Message is sent
// - when Message is sent -> update is executed
// - update changes state -> view is executed to (potentialy) reflects these changes
