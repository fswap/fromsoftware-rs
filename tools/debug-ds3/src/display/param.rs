use darksouls3::sprj::{CSRegulationManager, SoloParamRepository};
use hudhook::imgui::{TableColumnSetup, TableFlags, TreeNodeFlags};

use super::DebugDisplay;

impl DebugDisplay for CSRegulationManager {
    fn render_debug(&mut self, ui: &&mut hudhook::imgui::Ui) {
        if ui.collapsing_header("Resources", TreeNodeFlags::empty())
            && let Some(_t) = ui.begin_table_header_with_flags(
                "fd4-param-repository-rescaps",
                [
                    TableColumnSetup::new("Name"),
                    TableColumnSetup::new("Row Count"),
                    TableColumnSetup::new("Bytes"),
                ],
                TableFlags::RESIZABLE
                    | TableFlags::BORDERS
                    | TableFlags::ROW_BG
                    | TableFlags::SIZING_STRETCH_PROP,
            )
        {
            ui.indent();
            for res_cap in &self.params {
                let table = &res_cap.param.table;
                ui.table_next_column();
                ui.text(table.name());

                ui.table_next_column();
                ui.text(format!("{}", table.length));

                ui.table_next_column();
                ui.text(format!("{:p}", table.data()));
            }
            ui.unindent();
        }
    }
}

impl DebugDisplay for SoloParamRepository {
    fn render_debug(&mut self, ui: &&mut hudhook::imgui::Ui) {
        if let Some(_t) = ui.begin_table_header_with_flags(
            "solo-param-repository-rescaps",
            [
                TableColumnSetup::new("Name"),
                TableColumnSetup::new("Row Count"),
                TableColumnSetup::new("Bytes"),
            ],
            TableFlags::RESIZABLE
                | TableFlags::BORDERS
                | TableFlags::ROW_BG
                | TableFlags::SIZING_STRETCH_PROP,
        ) {
            ui.indent();
            for res_cap in self.iter() {
                let table = &res_cap.param.table;
                ui.table_next_column();
                ui.text(format!("{}", res_cap.name));

                ui.table_next_column();
                ui.text(format!("{}", table.length));

                ui.table_next_column();
                ui.text(format!("{:p}", table.data()));
            }
            ui.unindent();
        }
    }
}
