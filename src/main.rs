use iced::widget::column;
use iced::widget::image;
use iced::widget::text;
use iced::{Element, Size, keyboard, window};
use std::fs;
use std::path::PathBuf;

fn main() -> iced::Result {
    iced::application(boot, update, view)
        .title(|_state: &State| "Image Viewer".into())
        .window(window::Settings {
            size: Size::new(800.0, 600.0),
            ..Default::default()
        })
        .subscription(subscription)
        .run()
}

#[derive(Debug, Clone)]
enum Message {
    KeyboardEvent(keyboard::Event),
}

fn subscription(_state: &State) -> iced::Subscription<Message> {
    keyboard::listen().map(Message::KeyboardEvent)
}

fn update(state: &mut State, message: Message) {
    match message {
        Message::KeyboardEvent(event) => {
            if let keyboard::Event::KeyPressed { key, .. } = event {
                if matches!(key, keyboard::Key::Named(keyboard::key::Named::Enter)) {
                    state.show_sigma2 = state.show_sigma2 + 1;
                }
            }
        }
    }
}

fn view(state: &State) -> Element<'_, Message> {
    // let path = if state.show_sigma2 {
    //     "/home/koushikk/Desktop/sigma2.jpg"
    // } else {
    //     "/home/koushikk/Desktop/sigma.jpg"
    // };
    let images = list_jpg_files();
    let num = state.show_sigma2;
    let first = &images[num];
    let img = image::Handle::from_path(first);

    column![text(format!("Image: {}", first.display())), image(img)].into()
}

struct State {
    show_sigma2: usize,
}

fn boot() -> State {
    State { show_sigma2: 0 }
}

fn list_jpg_files() -> Vec<PathBuf> {
    let dir = "/home/koushikk/Desktop";
    let mut jpg_files = Vec::new();

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(ext) = path.extension() {
                if ext.eq_ignore_ascii_case("jpg") {
                    jpg_files.push(path);
                }
            }
        }
    }

    jpg_files
}
