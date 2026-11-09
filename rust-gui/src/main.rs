use iced::widget::{button, column, text};
use iced::{Alignment, Element, Sandbox, Settings};

fn main() -> iced::Result {
    Counter::run(Settings::default())
}

struct Counter {
    value: i32,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
}

impl Sandbox for Counter {
    type Message = Message;

    // 1. Inisialisasi State
    fn new() -> Self {
        Self { value: 0 }
    }

    // 2. Judul Jendela
    fn title(&self) -> String {
        String::from("Exo Counter - Rust GUI")
    }

    // 3. Logika Perubahan (Update)
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => self.value += 1,
            Message::Decrement => self.value -= 1,
        }
    }

    // 4. Tampilan (View)
    fn view(&self) -> Element<Message> {
        column![
            button("Tambah").on_press(Message::Increment),
            text(self.value).size(50),
            button("Kurang").on_press(Message::Decrement),
        ]
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }
}
