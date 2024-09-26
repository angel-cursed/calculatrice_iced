use iced::{Application, executor, Command, Element, Renderer, Theme, Settings};
use iced::widget::text;

fn main() -> iced::Result{
    Calculator::run(Settings::default())
}

struct Calculator {
    content: String
}

#[derive(Debug, Clone)]
enum Message {
    Input(String),
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Application for Calculator {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new( _flags: Self::Flags) -> (Self, Command<Message>) {
        (Self {
            content: "".to_string()
        }, Command::none()
        )
    }

    fn title(&self) -> String {
        "Calculator".to_string()
    }

    fn update(&mut self, message: Message) -> Command<Message>{
        match message {
            _ => Command::none(),
        }
    }

    fn view(&self) -> Element<'_, Self::Message, Renderer<Self::Theme>> {
        text(&self.content).into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}