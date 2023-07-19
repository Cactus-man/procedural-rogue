use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

const CONSOLE_FILL: egui::Color32 = egui::Color32::from_gray(27);

#[derive(Resource, Debug, Default, Reflect)]
pub struct DevConsole {
    text: String,
    open: bool,
}

fn draw_console(mut contexts: EguiContexts, mut console: ResMut<DevConsole>) {
    let DevConsole { text, open } = &mut *console;

    egui::Window::new("console")
        .frame(egui::Frame {
            fill: CONSOLE_FILL.gamma_multiply(0.8),
            rounding: egui::Rounding::none(),
            shadow: egui::epaint::Shadow::NONE,
            inner_margin: egui::Margin::same(5.0),
            ..Default::default()
        })
        .title_bar(false)
        .open(open)
        .show(contexts.ctx_mut(), |ui| {
            ui.code("Hello World!");
            ui.label("Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet.");

            ui.text_edit_singleline(text)
        });
}

#[derive(Debug, Default)]
#[non_exhaustive]
pub struct ConsolePlugin {}

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<DevConsole>()
            .insert_resource(DevConsole {
                open: true,
                ..Default::default()
            })
            .add_systems(Update, draw_console);
    }
}
