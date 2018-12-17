use bluetooth::BluetoothMode;
use layout::Layout;

pub struct LedTheme {
    pub key_colors: [Option<(u8, u8, u8, u8)>; 70],
}

impl LedTheme {
    fn new() -> Self {
        LedTheme {
            key_colors: [None; 70],
        }
    }
    pub fn fill_payload(&self, payload: &mut [u8]) -> usize {
        let mut key_count = 1;
        for (index, color) in self
            .key_colors
            .iter()
            .enumerate()
            .filter(|&(_, c)| c.is_some())
        {
            let offset = 2 + key_count * 5;
            let (r, g, b, mode) = color.unwrap(); // color is Some
            payload[offset] = index as u8;
            payload[offset + 1] = r;
            payload[offset + 2] = g;
            payload[offset + 3] = b;
            payload[offset + 4] = mode;
            key_count += 1;
        }
        payload[1] = key_count as u8;

        2 + key_count * 5
    }
}

pub fn layout_to_theme(
    layout: &Layout,
    bt_saved_hosts: u8,
    bt_connected_host: u8,
    bt_mode: BluetoothMode,
) -> LedTheme {
    let mut theme = LedTheme::new();
    for (index, action) in layout.iter().enumerate() {
        theme.key_colors[index] = action.to_color(bt_saved_hosts, bt_connected_host, bt_mode);
    }
    theme
}
