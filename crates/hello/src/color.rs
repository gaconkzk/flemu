#[derive(Copy, Clone)]
pub enum Color {
  Red,
  Black,
  White,
  Grey,
  Green,
  Blue,
  Magenta,
  Yellow,
  Cyan,
}

impl Color {
  pub fn to_rgb(self) -> String {
    let (b1, b2, b3) = self.rgb();
    format!("rgb({}, {}, {})", &b1, &b2, &b3)
  }

  pub fn rgb(self) -> (u8, u8, u8) {
    match self {
      Color::Black => (34, 34, 34),
      Color::Red => (240, 10, 10),
      Color::White => (240, 240, 240),
      Color::Grey => (120, 120, 120),
      Color::Green => (10, 240, 10),
      Color::Blue => (10, 10, 240),
      Color::Magenta => (240, 10, 240),
      Color::Yellow => (240, 240, 10),
      // cyan
      _ => (10, 240, 240),
    }
  }
}
