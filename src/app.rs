use cosmic::{
    iced::{
        Alignment, Subscription,
        widget::{row, text},
    },
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
    vram_used: i32,
    vram_total: i32,
}

struct GpuWatchApplet {
    core: cosmic::app::Core,
    stats: GpuStats,
}

#[derive(Debug, Clone)]
enum Message {
    Update,
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
        };

        // Initial update
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
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let temp_icon = get_temp_icon(self.stats.temp);

        row![
            text(temp_icon).size(16),
            text(format!("{}°C", self.stats.temp)).size(14),
            text(format!("{}%", self.stats.usage)).size(14),
        ]
        .spacing(4)
        .padding(4)
        .align_y(Alignment::Center)
        .into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        cosmic::iced::time::every(Duration::from_secs(2))
            .map(|_| Message::Update)
    }
}

// GPU reading function
fn get_gpu_stats() -> Result<GpuStats, String> {
    let output = Command::new("nvidia-smi")
        .arg("--query-gpu=temperature.gpu,utilization.gpu,memory.used,memory.total")
        .arg("--format=csv,noheader,nounits")
        .output()
        .map_err(|e| format!("Failed to run nvidia-smi: {}", e))?;

    let output_str = String::from_utf8(output.stdout)
        .map_err(|e| format!("Invalid UTF-8: {}", e))?;

    let parts: Vec<&str> = output_str.trim().split(',').collect();
    
    if parts.len() != 4 {
        return Err(format!("Unexpected format"));
    }

    Ok(GpuStats {
        temp: parts[0].trim().parse().unwrap_or(0),
        usage: parts[1].trim().parse().unwrap_or(0),
        vram_used: parts[2].trim().parse().unwrap_or(0),
        vram_total: parts[3].trim().parse().unwrap_or(0),
    })
}

fn get_temp_icon(temp: i32) -> &'static str {
    if temp < 50 { "❄️" }
    else if temp < 65 { "🌡️" }
    else if temp < 80 { "🔥" }
    else { "🚨" }
}
