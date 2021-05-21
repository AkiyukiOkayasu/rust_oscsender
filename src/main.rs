use eframe::{egui, epi};
use nannou_osc as osc;
use nannou_osc::Type;
use osc::Connected;

pub struct OscSenderApp {
    value: f32,
    sender: osc::Sender<Connected>,
}

impl OscSenderApp {
    fn new(s: osc::Sender<Connected>) -> Self {
        Self {
            value: 0.1,
            sender: s,
        }
    }
}

impl epi::App for OscSenderApp {
    fn name(&self) -> &str {
        "RustOSCSender"
    }

    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>) {
        let Self { value, sender } = self;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("OSC Sender localhost port:8080");

            if ui
                .add(egui::Slider::new(value, 0.0..=10.0).text("/slider"))
                .dragged()
            {
                let addr = "/slider";
                let args = vec![Type::Float(*value)];
                sender.send((addr, args)).ok();
            };

            if ui.button("/button").clicked() {
                // Send OSC
                let addr = "/button";
                let args = vec![Type::Int(1)];
                sender.send((addr, args)).ok();
            }
        });
    }
}

fn main() {
    const IP_PORT: &str = "127.0.0.1:8080";
    let sender = osc::sender().unwrap().connect(IP_PORT).unwrap();
    let app = OscSenderApp::new(sender);
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
