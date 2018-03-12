use crate::action::Action;
use crate::action::Action::*;
use crate::keycodes::KeyCode::*;
use crate::keymatrix::{COLUMNS, ROWS};

/*
  ,-----------------------------------------------------------------------------.
  |Esc   |  1|   2|   3|   4|   5|   6|   7|   8|   9|   0|   -|   = |   Backsp |
  |-----------------------------------------------------------------------------|
  |Tab    |  Q  |  W  |  E  |  R  |  T  |  Y  |  U  |  I|   O|  P|  [|  ]|  \ ] |
  |-----------------------------------------------------------------------------|
  |Caps         |    A|    S|    D|    F|   G|  H|  J|  K|  L|  ;|  '|   #|Enter|
  |-----------------------------------------------------------------------------|
  |Shift      |    Z|     X|    C|     V|  B|  N|  M|  ,|  .|  /|     Shift     |
  |-----------------------------------------------------------------------------|
  |Ctrl |Meta | Alt |               Space                |Alt | Fn  | Anne |Ctrl|
  `-----------------------------------------------------------------------------'
*/

pub type Layout = [Action; COLUMNS * ROWS];

pub const LAYERS: [Layout; 4] = [BASE, FN, FN2, BT];

pub const LAYER_FN: u8 = 1;
pub const LAYER_BT: u8 = 3;

// activate by indexing into LAYERS
const FN_M: Action = LayerMomentary(LAYER_FN);
const BT_M: Action = LayerMomentary(LAYER_BT);
const __: Action = Transparent;
const LED_NT: Action = LedNextTheme;
const LED_NB: Action = LedNextBrightness;
const LED_NAS: Action = LedNextAnimationSpeed;
const BT_ON: Action = LayerOn(LAYER_BT);

pub const BASE: Layout = layout![
Escape  N1 N2  N3    N4    N5    N6    N7    N8    N9    N0   LBracket   RBracket          BSpace
  Tab  Quote     Comma     Dot     P     Y     F     G     C     R     L    Slash   Equal   BSlash
 LCtrl A     O     E     U     I     D     H     T     N    S   Minus           No Enter
 LShift        SColon     Q     J     K     X     B     M    W   V   Z     No No RShift
  FN_M   LMeta   LAlt    No No       Space   No No No No   RMeta    FN_M      BT_M   Grave
];

pub const FN: Layout = layout![
  Grave   F1    F2    F3    F4    F5    F6    F7    F8    F9         F10   F11   F12  Delete
  __ PgUp  Numlock Kp8   KpPlus  LED_NB LED_NAS  LED_NT   Up  LedToggle __   __   __  PScreen
  __     Home   Kp4  Kp2  Kp6   Insert     Home   Left  Down      Right  End   __     No __
  __    PgDown  KpSlash  KpAsterisk KpMinus KpDot   BT_ON    __     __     __ __       No No __
  __  __      __      No No        Reset      No No No No   __          __       __     __
];

pub const FN2: Layout = layout![
    LedOff LedOn LED_NT LED_NAS LED_NB __ __ __ __ __ __ __ __ __
    __     __    __     __      __     __ __ __ __ __ __ __ __ __
    __     __    __     __      __     __ __ __ __ __ __ __ No __
    __     __    __     __      __     __ __ __ __ __ __ __ __ __
    __     __    __     No      No     __ No No No No __ __ __ __
];

#[rustfmt::skip]
pub const BT: Layout = layout![
    LayerOff(LAYER_BT) BtConnectHost(1) BtConnectHost(2) BtConnectHost(3) BtConnectHost(4) UsbToggle __ __ __ __ BtToggleLegacyMode BtOff BtBroadcast BtOn
    BtHostListQuery BtSaveHost(1) BtSaveHost(2) BtSaveHost(3) BtSaveHost(4) __ __ __ __ __ __ __ __ __
    __ BtDeleteHost(1) BtDeleteHost(2) BtDeleteHost(3) BtDeleteHost(4) __ __ __ __ __ __ __ No __
    __ __ __ __ __ LayerToggle(LAYER_BT) LayerOff(LAYER_BT) __ __ __ __ __ __ __
    __ __ __ No No __ No No No No __ __ __ __
];
