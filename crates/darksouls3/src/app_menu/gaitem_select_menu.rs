use crate::CxxVec;
use super::{MenuWindow, SceneObjProxy, MenuWindowCallback};

use shared::UnknownStruct;

#[repr(C)]
// Source of name: RTTI
pub struct GaitemSelectBaseMenu {
    pub menu_window: MenuWindow,
    _unk9d8: SceneObjProxy,
    pub menu_bg: GaitemSelectBaseMenuBG,
    _unkc18: u64,
    _grid_control_1: GridControl,
    _grid_control_2: GridControl,
    _window_list_item_name: u8,
    _unk1891: [u8; 0x5F],
    _window_list_item_simple_status: u8,
    _unk18f1: [u8; 0x5F],
    _item_select_control: UnknownStruct<0x18>,
    _unk1968: UnknownStruct<0x28>,
    _unk1990: UnknownStruct<0x80>,
    _unk1a10: CxxVec<u64>,
    pub detail_status_view: GaitemSelectDetailStatusView,
    _unk1ff0: UnknownStruct<0x90>,
}

type GridControl = UnknownStruct<0x638>;

#[repr(C)]
// Source of name: debug text
pub struct GaitemSelectBaseMenuBG {
    pub small: [u8; 0x60],
    pub large: [u8; 0x60],
    pub item_status: [u8; 0x60],
    pub item_detail_status: [u8; 0x60],
    pub player_status: [u8; 0x60],
}

#[repr(C)]
// Source of name: debug text
pub struct GaitemSelectDetailStatusView {
    _callback1: MenuWindowCallback,
    _callback2: MenuWindowCallback,
    _callback3: MenuWindowCallback,
    _callback4: MenuWindowCallback,
    _unk80: SceneObjProxy,
    _status_uchiku: u64,
    _unke8: [u8; 8],
    _status_item: u64,
    _unkf8: [u8; 8],
    _status_player: u64,
    _unk108: [u8; 0x388],
    _unk490: u32,
    _unk494: [u8; 0x10c],
    _unk5a0: u64,
    _unk5a8: u8,
    _unk5b0: usize,
    _unk5b8: u16,
}

#[repr(C)]
// Source of name: RTTI
pub struct GaitemSelectMenu {
    pub base: GaitemSelectBaseMenu,
    _unk2080: u64,
    _unk2088: u64,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn proper_sizes() {
        assert_eq!(0x1e0, size_of::<GaitemSelectBaseMenuBG>());
        assert_eq!(0x5c0, size_of::<GaitemSelectDetailStatusView>());
        assert_eq!(0x2080, size_of::<GaitemSelectBaseMenu>());
        assert_eq!(0x2090, size_of::<GaitemSelectMenu>());
    }
}
