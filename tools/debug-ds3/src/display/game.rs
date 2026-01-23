use hudhook::imgui::{TreeNodeFlags, Ui};

use darksouls3::sprj::*;

use super::DebugDisplay;

impl DebugDisplay for GameDataMan {
    fn render_debug(&mut self, ui: &&mut Ui) {
        if ui.collapsing_header("Bloodstain", TreeNodeFlags::empty()) {
            ui.text(format!("coordinates: {:?}", self.bloodstain.coordinates));
            ui.text(format!("souls: {}", self.bloodstain.souls));
        }
    }
}
