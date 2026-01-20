use hudhook::imgui::{TableColumnSetup, TableFlags, TreeNodeFlags, Ui};
use pelite::pe64::Pe;

use darksouls3::{app_menu::*, sprj::*};

use super::{DebugDisplay, StatefulDebugDisplay};

impl DebugDisplay for MenuMan {
    fn render_debug(&mut self, ui: &&mut Ui) {
        ui.text(format!("Load screen: {}", self.is_load_screen()));
        ui.text(format!("Menu: {}", self.is_menu_mode()));

        if ui.collapsing_header("Flags", TreeNodeFlags::empty())
            && let Some(_t) = ui.begin_table_header_with_flags(
                "menu-man-flags",
                [TableColumnSetup::new("ID"), TableColumnSetup::new("Value")],
                TableFlags::RESIZABLE
                    | TableFlags::BORDERS
                    | TableFlags::ROW_BG
                    | TableFlags::SIZING_STRETCH_PROP,
            )
        {
            ui.indent();
            for (i, value) in self.flags.iter().enumerate() {
                ui.table_next_column();
                ui.text(format!("{}", i));

                ui.table_next_column();
                ui.text(format!("{:x}", value));
            }
            ui.unindent();
        }

        if let Some(cmd) = self.grant_item_command.as_option() {
            ui.text(format!("Item ID: {:?}", cmd.item_id()));
            ui.text(format!("Durability: {}", cmd.durability));
            ui.text(format!("Quantity: {}", cmd.quantity));
        } else {
            ui.text("<no grant item command>");
        }
    }
}

impl DebugDisplay for NewMenuSystem {
    fn render_debug(&mut self, ui: &&mut Ui) {
        ui.text("Windows:");
        for (i, window) in self.windows.iter_mut().enumerate() {
            let window = unsafe { window.as_mut() };
            let name = match MenuWindowSubclass::from(&*window) {
                MenuWindowSubclass::GaitemSelectMenu(_) => "GaitemSelect",
                _ => "Unknown",
            };
            if ui.collapsing_header(format!("#{}: {}", i, name), TreeNodeFlags::empty()) {
                ui.indent();
                DebugDisplay::render_debug(window, ui);
                ui.unindent();
            }
        }
    }
}

impl DebugDisplay for MenuWindow {
    fn render_debug(&mut self, ui: &&mut Ui) {
        match MenuWindowSubclassMut::from(self) {
            MenuWindowSubclassMut::GaitemSelectMenu(player) => {
                DebugDisplay::render_debug(player, ui)
            }
            MenuWindowSubclassMut::MenuWindow(window) => {
                ui.text(format!("Address: {:p}", window));

                ui.text(format!(
                    "Vtable RVA: {:x}",
                    crate::Program::current()
                        .va_to_rva(window.vftable as u64)
                        .unwrap()
                ));
            }
            _ => {
                ui.text("Unknown MenuWindow type");
            }
        }
    }
}

impl DebugDisplay for GaitemSelectMenu {
    fn render_debug(&mut self, ui: &&mut Ui) {
        ui.text(format!("Address: {:p}", self));
        if let Some(_t) = ui.begin_table_header_with_flags(
            "item-select-menu-items",
            [
                TableColumnSetup::new("ID"),
                TableColumnSetup::new("price"),
                TableColumnSetup::new("quantity"),
                TableColumnSetup::new("category"),
                TableColumnSetup::new("ShopLineupParam"),
            ],
            TableFlags::RESIZABLE
                | TableFlags::BORDERS
                | TableFlags::ROW_BG
                | TableFlags::SIZING_STRETCH_PROP,
        ) {
            for item in self.items() {
                ui.table_next_column();
                ui.text(format!("{:?}|{}", item.id.category(), item.id.param_id()));
                ui.table_next_column();
                ui.text(format!("{}", item.price));
                ui.table_next_column();
                ui.text(format!("{}", item.quantity));
                ui.table_next_column();
                ui.text(format!("{:?}", item.category));
                ui.table_next_column();
                ui.text(format!("{}", item.shop_lineup_param));
            }
        }
    }
}

#[derive(Default)]
pub struct ItemGetMenuManDebugState {
    item_id: String,
    quantity: String,
    in_box: bool,
}

impl StatefulDebugDisplay for ItemGetMenuMan {
    type State = ItemGetMenuManDebugState;

    fn render_debug(&mut self, ui: &&mut Ui, state: &mut Self::State) {
        {
            let _tok = ui.push_item_width(150.);
            ui.input_text("Item ID ", &mut state.item_id).build();
        }

        ui.same_line();
        {
            let _tok = ui.push_item_width(100.);
            ui.input_text("Quantity", &mut state.quantity).build();
        }

        ui.checkbox("In Box", &mut state.in_box);

        let item_id = state
            .item_id
            .parse::<u32>()
            .ok()
            .and_then(|i| ItemId::try_from(i).ok());

        let quantity = state.quantity.parse::<u32>();

        ui.same_line_with_pos(ui.window_content_region_max()[0] - 200.);
        {
            let _tok = ui.begin_enabled(item_id.is_some() && quantity.is_ok());
            if ui.button("Show Popup") {
                self.show_item(item_id.unwrap(), quantity.unwrap(), state.in_box);
            }
        }
    }
}
