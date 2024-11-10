use iced::{
    color,
    widget::{column, container, image::Handle, scrollable},
    Element, Length, Task,
};

fn main() -> iced::Result {
    tracing_subscriber::fmt::fmt().compact().init();
    iced::application("Iced Image Overflow", update, view)
        .run_with(|| (App::default(), Task::done(Message::LoadImages)))
}

#[derive(Debug, Clone)]
enum Message {
    LoadImages,
    ImagesLoaded,
}

#[derive(Debug, Default)]
struct App {
    images: Vec<Handle>,
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::LoadImages => {
            let handle = Handle::from_path("assets/image.jpg");

            for _ in 0..100 {
                app.images.push(handle.clone());
            }

            Task::done(Message::ImagesLoaded)
        }
        Message::ImagesLoaded => Task::none(),
    }
}

fn view(app: &App) -> Element<Message> {
    let top_bar = container(iced::widget::text("TOP BAR"))
        .style(|_| container::Style {
            border: iced::Border {
                color: color!(0x00ff00),
                width: 1.0,
                ..Default::default()
            },
            ..Default::default()
        })
        .padding(20.0)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .width(Length::Fill)
        .height(Length::Fixed(200.0));

    let images = container(scrollable(column(app.images.iter().map(|image| {
        container(
            iced::widget::image(image.clone())
                .content_fit(iced::ContentFit::Cover)
                .height(Length::Fixed(150.0))
                .width(Length::Fixed(150.0)),
        )
        .into()
    }))));

    container(iced::widget::column!(top_bar, images))
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|_| container::Style {
            border: iced::Border {
                color: color!(0xff0000),
                width: 1.0,
                ..Default::default()
            },
            ..Default::default()
        })
        .into()
}
