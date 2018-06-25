use bit_field::BitField;

use crate::bluetooth::BluetoothMode;
use crate::keycodes::KeyCode;

#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq)]
pub enum Action {
    Nop,
    /// Reset the key MCU. If Escape is also held down then it will
    /// boot into DFU mode.
    Reset,
    /// Fall-through to the next layer underneath
    Transparent,
    /// Toggle sending HID report over USB
    UsbToggle,

    Key(KeyCode), // = 0x10

    LayerMomentary(u8), // = 0x20,
    LayerToggle(u8),
    LayerOn(u8),
    LayerOff(u8),

    LedOn, // = 0x30,
    LedOff,
    LedToggle,
    LedNextTheme,
    LedNextBrightness,
    LedNextAnimationSpeed,
    LedTheme(u8),

    //Bluetooth = 0x40,
    BtOn,
    BtOff,
    BtSaveHost(u8),
    BtConnectHost(u8),
    BtDeleteHost(u8),
    BtBroadcast,
    BtLegacyMode(bool),
    BtToggleLegacyMode,
    BtHostListQuery, // TODO: remove? this shouldn't really be here
}

// Allow auto-conversion of KeyCodes to Action for nicer layout formatting
// and drop commas
macro_rules! layout {
    ( $( $e: expr )* ) => {
        [
            $(
                $e.to_action(),
            )*
        ]
    };
}

impl KeyCode {
    pub const fn to_action(self) -> Action {
        Action::Key(self)
    }
}

impl Action {
    pub const fn to_action(self) -> Action {
        self
    }
    pub fn to_color(
        &self,
        saved_hosts: u8,
        connected_host: u8,
        mode: BluetoothMode,
        keyboard_send_usb_report: bool,
    ) -> Option<(u8, u8, u8, u8)> {
        use self::Action::*;
        use crate::layout::LAYER_FN;
        use crate::led::LedMode;
        const ON: u8 = LedMode::On as u8;
        const FLASH: u8 = LedMode::Flash as u8;
        const WHITE: Option<(u8, u8, u8, u8)> = Some((0x44, 0x44, 0x44, ON));
        const RED: Option<(u8, u8, u8, u8)> = Some((0x44, 0, 0, ON));
        const GREEN: Option<(u8, u8, u8, u8)> = Some((0, 0x44, 0, ON));
        const BLUE: Option<(u8, u8, u8, u8)> = Some((0, 0, 0x44, ON));
        const CYAN: Option<(u8, u8, u8, u8)> = Some((0, 0x44, 0x44, ON));
        const YELLOW: Option<(u8, u8, u8, u8)> = Some((0x44, 0x44, 0, ON));

        let has_saved_host = |slot: u8| saved_hosts.get_bit(slot as usize - 1);

        match *self {
            UsbToggle if keyboard_send_usb_report => WHITE,
            UsbToggle => Some((0xff, 0xff, 0xff, FLASH)),
            BtHostListQuery | LedNextBrightness | LayerMomentary(LAYER_FN) => WHITE,
            Reset | LedOff | BtOff | Key(KeyCode::LMeta) | Key(KeyCode::RMeta)
            | Key(KeyCode::V) => RED,
            BtBroadcast | LedNextAnimationSpeed => GREEN,
            BtOn | Key(KeyCode::N6) | Key(KeyCode::N9) | Key(KeyCode::C) => BLUE,
            LayerToggle(_) => YELLOW,
            LayerOff(_) | LedNextTheme | LedOn => Some((0, 0xff, 0, FLASH)),

            Key(KeyCode::N1) if connected_host == 1 => CYAN,
            Key(KeyCode::N2) if connected_host == 2 => CYAN,
            Key(KeyCode::N3) if connected_host == 3 => CYAN,
            Key(KeyCode::N4) if connected_host == 4 => CYAN,
            BtConnectHost(slot) if slot == connected_host => Some((0, 0xff, 0xff, FLASH)),
            BtConnectHost(slot) if has_saved_host(slot) => GREEN,

            BtSaveHost(slot) if has_saved_host(slot) => CYAN,
            BtSaveHost(slot) if !has_saved_host(slot) => YELLOW,

            BtDeleteHost(slot) if has_saved_host(slot) => RED,
            BtToggleLegacyMode => match mode {
                BluetoothMode::Unknown => RED,
                BluetoothMode::Ble => GREEN,
                BluetoothMode::Legacy => YELLOW,
            },

            Key(code) if code.is_modifier() => GREEN,
            Key(code) if KeyCode::PScreen <= code && code <= KeyCode::Up => WHITE,

            _ => None,
        }
    }
}
