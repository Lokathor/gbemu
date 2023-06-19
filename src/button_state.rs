use bitfrob::{u8_get_bit, u8_with_bit};

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct ButtonState(u8);
impl Default for ButtonState {
  fn default() -> Self {
    Self::new()
  }
}
impl ButtonState {
  pub const fn new() -> Self {
    Self(u8::MAX)
  }
  pub const fn a(self) -> bool {
    !u8_get_bit(0, self.0)
  }
  pub const fn b(self) -> bool {
    !u8_get_bit(1, self.0)
  }
  pub const fn select(self) -> bool {
    !u8_get_bit(2, self.0)
  }
  pub const fn start(self) -> bool {
    !u8_get_bit(3, self.0)
  }
  pub const fn right(self) -> bool {
    !u8_get_bit(4, self.0)
  }
  pub const fn left(self) -> bool {
    !u8_get_bit(5, self.0)
  }
  pub const fn up(self) -> bool {
    !u8_get_bit(6, self.0)
  }
  pub const fn down(self) -> bool {
    !u8_get_bit(7, self.0)
  }
  pub const fn with_a(self, pushed: bool) -> Self {
    Self(u8_with_bit(0, self.0, !pushed))
  }
  pub const fn with_b(self, pushed: bool) -> Self {
    Self(u8_with_bit(1, self.0, !pushed))
  }
  pub const fn with_select(self, pushed: bool) -> Self {
    Self(u8_with_bit(2, self.0, !pushed))
  }
  pub const fn with_start(self, pushed: bool) -> Self {
    Self(u8_with_bit(3, self.0, !pushed))
  }
  pub const fn with_right(self, pushed: bool) -> Self {
    Self(u8_with_bit(4, self.0, !pushed))
  }
  pub const fn with_left(self, pushed: bool) -> Self {
    Self(u8_with_bit(5, self.0, !pushed))
  }
  pub const fn with_up(self, pushed: bool) -> Self {
    Self(u8_with_bit(6, self.0, !pushed))
  }
  pub const fn with_down(self, pushed: bool) -> Self {
    Self(u8_with_bit(7, self.0, !pushed))
  }
  /// Converts the emulated button state into a `JOYP` value based on the two
  /// selection bits.
  pub const fn to_joyp(self, action: bool, direction: bool) -> u8 {
    let mut out = 0b1111;
    // llvm plz be smarter than me and make this bit math fast
    if action {
      out &= self.0 >> 4;
    }
    if direction {
      out &= self.0;
    }
    if !action {
      out |= 1 << 5;
    }
    if !direction {
      out |= 1 << 4;
    }
    out
  }
}
impl core::fmt::Debug for ButtonState {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut x = f.debug_set();
    if self.a() {
      x.entry(&"a");
    }
    if self.b() {
      x.entry(&"b");
    }
    if self.select() {
      x.entry(&"select");
    }
    if self.start() {
      x.entry(&"start");
    }
    if self.right() {
      x.entry(&"right");
    }
    if self.left() {
      x.entry(&"left");
    }
    if self.up() {
      x.entry(&"up");
    }
    if self.down() {
      x.entry(&"down");
    }
    x.finish()
  }
}

#[test]
fn test_button_state() {
  let buttons = ButtonState::default();
  assert!(!buttons.a());
  assert!(!buttons.b());
  assert!(!buttons.select());
  assert!(!buttons.start());
  assert!(!buttons.right());
  assert!(!buttons.left());
  assert!(!buttons.up());
  assert!(!buttons.down());

  let buttons = ButtonState::default().with_a(true);
  assert!(buttons.a());
  let buttons = ButtonState::default().with_b(true);
  assert!(buttons.b());
  let buttons = ButtonState::default().with_select(true);
  assert!(buttons.select());
  let buttons = ButtonState::default().with_start(true);
  assert!(buttons.start());
  let buttons = ButtonState::default().with_right(true);
  assert!(buttons.right());
  let buttons = ButtonState::default().with_left(true);
  assert!(buttons.left());
  let buttons = ButtonState::default().with_up(true);
  assert!(buttons.up());
  let buttons = ButtonState::default().with_down(true);
  assert!(buttons.down());
}
