use egui::{FontData, FontDefinitions, FontFamily};

use crate::{main_window, project_creator, project_loader, project_selector, AppConfig, AppState};

pub struct AstraApp {
    pub config: AppConfig,
    pub state: AppState,
    pub next_state: Option<AppState>,
}

impl AstraApp {
    pub fn new(cc: &eframe::CreationContext<'_>, config: AppConfig) -> Self {
        catppuccin_egui::set_theme(&cc.egui_ctx, config.theme.into());

        let mut font_definitions = FontDefinitions::default();
        font_definitions.font_data.insert(
            "noto_sans".to_owned(),
            FontData::from_static(include_bytes!("../assets/NotoSans-Regular.ttf")),
        );
        font_definitions.font_data.insert(
            "noto_sans_jp".to_owned(),
            FontData::from_static(include_bytes!("../assets/NotoSansJP-Regular.otf")),
        );
        let proportional = font_definitions
            .families
            .get_mut(&FontFamily::Proportional)
            .unwrap();
        proportional.insert(0, "noto_sans".into());
        proportional.push("noto_sans_jp".into());
        font_definitions
            .families
            .get_mut(&FontFamily::Monospace)
            .unwrap()
            .push("noto_sans_jp".to_owned());
        cc.egui_ctx.set_fonts(font_definitions);

        AstraApp {
            config,
            state: AppState::SelectProject,
            next_state: None,
        }
    }
}

impl eframe::App for AstraApp {
    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        if let Err(err) = self.config.save() {
            println!("{:?}", err);
        }
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Some(state) = std::mem::take(&mut self.next_state) {
            self.state = state;
        }
        match &mut self.state {
            AppState::CreateProject(state) => {
                project_creator(state, &mut self.config, &mut self.next_state, ctx)
            }
            AppState::SelectProject => {
                project_selector(&mut self.config, &mut self.next_state, ctx)
            }
            AppState::LoadProject(state) => {
                project_loader(state, &self.config, &mut self.next_state, ctx)
            }
            AppState::Main(state) => {
                main_window(state, &mut self.next_state, &mut self.config, ctx)
            }
        }
    }
}
