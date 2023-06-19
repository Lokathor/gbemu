/*

https://youtu.be/HyzD8pNlpwI?t=1756


| Addr | Name | Use |
|:-:|:-|:-|
| FF40|  LCDC | |
| FF41|  STAT | |
| FF42|  SCY | scroll Y |
| FF43|  SCX | scroll X |
| FF44|  LY | |
| FF45|  LYC | |
| FF46|  DMA | |
| FF47|  BGP |  bg-palette |
| FF48|  OBP0 | object palette 1 |
| FF49|  OBP1 | object palette 2 |
| FF4A|  WY | window y |
| FF4B|  WX | window x |

*/

use pixel_formats::r8g8b8a8_Srgb;

use crate::system::{LCD, LCD_HEIGHT, LCD_WIDTH};

#[derive(Debug, Clone, Default)]
pub struct PPU {
  next_y: u8,
  next_x: u8,
  mode: PpuMode,
  scanline_ticks: u16,
}
impl PPU {
  /// time spend scanning OAM
  const SCAN_TIME: u16 = 80;
  /// scan + draw + blank time always adds up to this many
  const DOTS_HORIZONTAL: u16 = 456;
  /// current line is always in `0..=LINES_VERTICAL`
  const LINES_VERTICAL: u8 = 153;

  pub fn t_cycle(&mut self, lcd: &mut LCD) -> PpuMode {
    match self.mode {
      PpuMode::Scan => {
        self.scanline_ticks += 1;
        if self.scanline_ticks == Self::SCAN_TIME {
          self.mode = PpuMode::Draw;
        }
      }
      PpuMode::Draw => {
        self.scanline_ticks += 1;
        let i = ((self.next_y as usize) * (LCD_WIDTH as usize)) + (self.next_x as usize);
        lcd[i].r ^= 0xFF;
        lcd[i].g ^= 0xF0;
        lcd[i].b ^= 0x0F;
        self.next_x += 1;
        if self.next_x >= LCD_WIDTH {
          self.mode = PpuMode::HBlank;
        }
      }
      PpuMode::HBlank => {
        self.scanline_ticks += 1;
        if self.scanline_ticks == Self::DOTS_HORIZONTAL {
          self.next_x = 0;
          self.scanline_ticks = 0;
          self.next_y += 1;
          if self.next_y >= LCD_HEIGHT {
            self.mode = PpuMode::VBlank;
          } else {
            self.mode = PpuMode::Scan;
          }
        }
      }
      PpuMode::VBlank => {
        self.scanline_ticks += 1;
        if self.scanline_ticks == Self::DOTS_HORIZONTAL {
          self.next_y += 1;
          self.scanline_ticks = 0;
          if self.next_y >= Self::LINES_VERTICAL {
            self.next_y = 0;
            self.mode = PpuMode::Scan;
          }
        }
      }
    }
    self.mode
  }

  pub fn current_scanline(&self) -> u8 {
    self.next_y
  }
  pub fn mode(&self) -> PpuMode {
    self.mode
  }
}

#[derive(Debug, Clone, Copy, Default, bytemuck::Zeroable, bytemuck::Pod)]
#[repr(C)]
pub struct OamEntry {
  pub y: u8,
  pub x: u8,
  pub tile: u8,
  pub flags: u8,
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(u8)]
pub enum PpuMode {
  HBlank = 0,
  VBlank = 1,
  #[default]
  Scan = 2,
  Draw = 3,
}
