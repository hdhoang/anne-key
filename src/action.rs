use bit_field::BitField;
use bluetooth::BluetoothMode;
use keycodes::KeyCode;

#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq)]
pub enum Action {
    Nop,
    /// Reset the key MCU. If Escape is also held down then it will
    /// boot into DFU mode.
    Reset,
    Transparent,

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
    ) -> Option<(u8, u8, u8, u8)> {
        use self::Action::*;
        use layout::LAYER_FN;
        use led::LedMode;
        const ON: u8 = LedMode::On as u8;
        const FLASH: u8 = LedMode::Flash as u8;

        let has_saved_host = |slot: u8| saved_hosts.get_bit(slot as usize - 1);

        match *self {
            BtHostListQuery | LedNextBrightness => Some((0xff, 0xff, 0xff, ON)),
            Reset | LedOff | BtOff => Some((0xff, 0, 0, ON)),
            LayerToggle(_) => Some((0xff, 0xff, 0, ON)),
            LayerMomentary(LAYER_FN) => Some((0xff, 0xff, 0xff, ON)),
            LayerOff(_) | LedNextTheme | LedOn => Some((0, 0xff, 0, FLASH)),
            BtBroadcast | LedNextAnimationSpeed => Some((0, 0xff, 0, ON)),
            BtOn => Some((0, 0, 0xff, ON)),
            BtConnectHost(slot) => {
                if slot == connected_host {
                    Some((0, 0xff, 0xff, FLASH))
                } else if has_saved_host(slot) {
                    Some((0, 0xff, 0, ON))
                } else {
                    None
                }
            }
            BtSaveHost(slot) if has_saved_host(slot) => Some((0x00, 0xff, 0xff, ON)),
            BtSaveHost(slot) if !has_saved_host(slot) => Some((0xff, 0xff, 0x00, ON)),

            BtDeleteHost(slot) if has_saved_host(slot) => Some((0xff, 0, 0, ON)),
            BtToggleLegacyMode => match mode {
                BluetoothMode::Unknown => Some((0xff, 0, 0, ON)),
                BluetoothMode::Ble => Some((0, 0xff, 0, ON)),
                BluetoothMode::Legacy => Some((0xff, 0xff, 0, ON)),
            },
            Key(code) if connected_host == 1 && code == KeyCode::N1 => Some((0, 0xff, 0xff, ON)),
            Key(code) if connected_host == 2 && code == KeyCode::N2 => Some((0, 0xff, 0xff, ON)),
            Key(code) if connected_host == 3 && code == KeyCode::N3 => Some((0, 0xff, 0xff, ON)),
            Key(code) if connected_host == 4 && code == KeyCode::N4 => Some((0, 0xff, 0xff, ON)),
            Key(code) if code.is_modifier() => Some((0, 0xff, 0, ON)),
            Key(code) if KeyCode::PScreen <= code && code <= KeyCode::Up => {
                Some((0x1e, 0xee, 0xab, ON))
            }
            _ => None,
        }
    }
}
