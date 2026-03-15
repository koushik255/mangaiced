use iced::widget::column;
use iced::widget::image;
use iced::widget::text;
use iced::{Element, Size, keyboard, window};
use rustma::list_cbz_files;
use rustma::list_jpg_files;
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
                    state.image_count = state.image_count + 1;
                    let cbzs = list_cbz_files();
                    cbzs.iter()
                        .for_each(|f| println!("cbz file {}", f.display()));
                }
                if matches!(key, keyboard::Key::Named(keyboard::key::Named::ArrowDown)) {
                    state.volume_count = state.volume_count + 1;
                }
            }
        }
    }
}

fn view(state: &State) -> Element<'_, Message> {
    let images = list_jpg_files();
    let num = state.image_count;
    let first = &images[num];
    let img = image::Handle::from_path(first);

    let cbz_paths = list_cbz_files();
    let volume_count_number = state.volume_count;
    let paths_volumes = &cbz_paths[volume_count_number];

    column![
        text(format!("Image: {}", first.display())),
        image(img),
        text(format!("volume count: {}", paths_volumes.display()))
    ]
    .into()
}

struct State {
    image_count: usize,
    volume_count: usize,
}

fn boot() -> State {
    State {
        image_count: 0,
        volume_count: 0,
    }
}
