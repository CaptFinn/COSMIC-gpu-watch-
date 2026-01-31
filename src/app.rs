use cosmic::{
    iced::{
        Alignment, Length, Subscription,
        widget::{column, row, text, slider},
        platform_specific::shell::wayland::commands::popup::{destroy_popup, get_popup},
        window,
    },
    widget::divider,
    Application, Element, Task,
};
use std::time::Duration;
use std::process::Command;

pub fn run() -> cosmic::iced::Result {
    cosmic::applet::run::<GpuWatchApplet>(())
}

#[derive(Default, Clone)]
struct GpuStats {
    temp: i32,
    usage: i32,
}

struct GpuWatchApplet {
    core: cosmic::app::Core,
    stats: GpuStats,
    popup: Option<window::Id>,
    font_size: Option<f32>, // None = auto-size based on panel
}

#[derive(Debug, Clone)]
enum Message {
    Update,
    TogglePopup,
    PopupClosed(window::Id),
    FontSizeChanged(f32),
    ResetFontSize,
}

impl Application for GpuWatchApplet {
    type Executor = cosmic::executor::Default;
    type Flags = ();
    type Message = Message;
    const APP_ID: &'static str = "com.captfinn.CosmicAppletGpuWatch";

    fn core(&self) -> &cosmic::app::Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut cosmic::app::Core {
        &mut self.core
    }

    fn init(core: cosmic::app::Core, _flags: Self::Flags) -> (Self, Task<cosmic::Action<Self::Message>>) {
        let mut app = GpuWatchApplet {
            core,
            stats: GpuStats::default(),
            popup: None,
            font_size: None,
        };

        if let Ok(stats) = get_gpu_stats() {
            app.stats = stats;
        }

        (app, Task::none())
    }

    fn update(&mut self, message: Self::Message) -> Task<cosmic::Action<Self::Message>> {
        match message {
            Message::Update => {
                if let Ok(stats) = get_gpu_stats() {
                    self.stats = stats;
                }
                Task::none()
            }
            Message::TogglePopup => {
                if let Some(popup_id) = self.popup.take() {
                    destroy_popup(popup_id)
                } else {
                    let new_id = window::Id::unique();
                    self.popup.replace(new_id);
                    let popup_settings = self.core.applet.get_popup_settings(
                        self.core.main_window_id().unwrap(),
                        new_id,
                        None,
                        None,
                        None,
                    );
                    get_popup(popup_settings)
                }
            }
            Message::PopupClosed(id) => {
                if self.popup == Some(id) {
                    self.popup = None;
                }
                Task::none()
            }
            Message::FontSizeChanged(size) => {
                self.font_size = Some(size);
                Task::none()
            }
            Message::ResetFontSize => {
                self.font_size = None;
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        // Use manual font_size if set, otherwise auto-calculate from panel size
        let suggested = self.core.applet.suggested_size(false);
        let font_size = self.font_size.unwrap_or_else(|| {
            (suggested.0 as f32 * 0.45).clamp(10.0, 14.0)
        });

        let icon = get_temp_icon(self.stats.temp);

        let content = row![
            text(icon).size(font_size),
            text(format!("{}°", self.stats.temp)).size(font_size),
            text(format!("{}%", self.stats.usage)).size(font_size),
        ]
        .spacing(3)
        .align_y(Alignment::Center);

        let button = cosmic::widget::button::custom(content)
            .padding([4, 6])
            .on_press_down(Message::TogglePopup)
            .class(cosmic::theme::Button::AppletIcon);

        self.core.applet.autosize_window(button).into()
    }

    fn style(&self) -> Option<cosmic::iced_runtime::Appearance> {
        Some(cosmic::applet::style())
    }

    fn view_window(&self, _id: window::Id) -> Element<Self::Message> {
        let current_size = self.font_size.unwrap_or_else(|| {
            let suggested = self.core.applet.suggested_size(false);
            (suggested.0 as f32 * 0.45).clamp(10.0, 16.0)
        });

        let size_label = if self.font_size.is_some() {
            format!("{:.0}px", current_size)
        } else {
            format!("{:.0}px (auto)", current_size)
        };

        let content = column![
            text("GPU Watch").size(16),
            divider::horizontal::light(),
            text(format!("Temperature: {}°C", self.stats.temp)).size(14),
            text(format!("Usage: {}%", self.stats.usage)).size(14),
            divider::horizontal::light(),
            row![
                text("Label Size:").size(14),
                text(size_label).size(14),
            ]
            .spacing(8),
            slider(8.0..=20.0, current_size, Message::FontSizeChanged)
                .step(1.0)
                .width(Length::Fill),
            cosmic::widget::button::text("Reset to Auto")
                .on_press(Message::ResetFontSize),
        ]
        .spacing(12)
        .padding(16)
        .width(Length::Fixed(220.0))
        .align_x(Alignment::Start);

        self.core.applet.popup_container(content).into()
    }

    fn on_close_requested(&self, id: window::Id) -> Option<Self::Message> {
        Some(Message::PopupClosed(id))
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        cosmic::iced::time::every(Duration::from_secs(2))
            .map(|_| Message::Update)
    }
}

fn get_gpu_stats() -> Result<GpuStats, String> {
    let output = Command::new("nvidia-smi")
        .arg("--query-gpu=temperature.gpu,utilization.gpu")
        .arg("--format=csv,noheader,nounits")
        .output()
        .map_err(|e| format!("Failed to run nvidia-smi: {}", e))?;

    let output_str = String::from_utf8(output.stdout)
        .map_err(|e| format!("Invalid UTF-8: {}", e))?;

    let parts: Vec<&str> = output_str.trim().split(',').collect();

    if parts.len() != 2 {
        return Err(format!("Unexpected format"));
    }

    Ok(GpuStats {
        temp: parts[0].trim().parse().unwrap_or(0),
        usage: parts[1].trim().parse().unwrap_or(0),
    })
}

fn get_temp_icon(temp: i32) -> &'static str {
    if temp < 50 { "❄️" }
    else if temp < 65 { "🌡️" }
    else if temp < 80 { "🔥" }
    else { "🚨" }
}
