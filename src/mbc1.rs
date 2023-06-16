use crate::{cpu::MemoryBus, ERAM_BANK_SIZE, ROM_BANK_SIZE};

pub struct MBC1 {
  rom_banks: Vec<[u8; ROM_BANK_SIZE]>,
  ram_banks: Vec<[u8; ERAM_BANK_SIZE]>,
  low5_bits: u8,
  upper2_bits: u8,
  ram_active: bool,
  advanced_banking: bool,
  // these are cached values that the `update_indexes` method regenerates.
  rom_0_index: usize,
  rom_x_index: usize,
  ram_index: usize,
}
impl MBC1 {
  pub fn new(raw_rom: &[u8], raw_ram: Option<&[u8]>) -> Result<Self, ()> {
    // check minimum length
    if raw_rom.len() < 0x150 {
      return Err(());
    }
    // check for correct cart type byte.
    match raw_rom[0x147] {
      1 | 2 | 3 => (),
      _ => return Err(()),
    }
    // determine bank counts
    let rom_bank_count = (1 << raw_rom[0x148]).max(2);
    if rom_bank_count == 0 {
      return Err(());
    }
    let ram_bank_count = match raw_rom[0x149] {
      // technically this is "no ram" but we fudge it a bit and always give at
      // least 1 ram bank so that computations elsewhere don't have to worry
      // about the len=0 case.
      0 => 1,
      // this is supposed to be "unused", but sometimes meant half a bank of
      // ram, so whatever we'll just give 1 bank.
      1 => 1,
      // these are normal values
      2 => 1,
      3 => 4,
      // 5 is a legal value for some mappers, but not for MBC1. Any other values
      // are not expected.
      _ => return Err(()),
    };
    // copy input data to banks
    let mut rom_banks = vec![[0u8; ROM_BANK_SIZE]; rom_bank_count];
    rom_banks
      .iter_mut()
      .zip(raw_rom.chunks(ROM_BANK_SIZE))
      .for_each(|(b, r)| b[..r.len()].copy_from_slice(r));
    let mut ram_banks = vec![[0u8; ERAM_BANK_SIZE]; ram_bank_count];
    ram_banks
      .iter_mut()
      .zip(raw_ram.unwrap_or(&[]).chunks(ERAM_BANK_SIZE))
      .for_each(|(b, r)| b[..r.len()].copy_from_slice(r));
    // complete
    Ok(Self {
      rom_banks,
      ram_banks,
      low5_bits: 0,
      upper2_bits: 0,
      ram_active: false,
      advanced_banking: false,
      rom_0_index: 0,
      rom_x_index: 1,
      ram_index: 0,
    })
  }

  fn update_indexes(&mut self) {
    self.rom_x_index = {
      let b5 = self.low5_bits.max(1);
      let b2 = if self.rom_banks.len() > 0b11111 { self.upper2_bits } else { 0 };
      let full_index = b2 << 5 | b5;
      let wrapped_index = full_index % (self.rom_banks.len() as u8);
      usize::from(wrapped_index)
    };
    //
    if self.advanced_banking {
      self.rom_0_index = {
        let full_index = self.upper2_bits << 5;
        let wrapped_index = full_index % (self.rom_banks.len() as u8);
        usize::from(wrapped_index)
      };
      self.ram_index = {
        let full_index = self.upper2_bits;
        let wrapped_index = full_index % (self.ram_banks.len() as u8);
        usize::from(wrapped_index)
      };
    } else {
      self.rom_0_index = 0;
      self.ram_index = 0;
    }
  }
}
impl MemoryBus for MBC1 {
  #[inline]
  fn read(&self, address: u16) -> u8 {
    match address {
      0x0000..=0x3FFF => {
        let bank = &self.rom_banks[self.rom_0_index];
        let index = usize::from(address);
        bank[index]
      }
      0x4000..=0x7FFF => {
        let bank = &self.rom_banks[self.rom_x_index];
        let index = usize::from(address) - 0x4000;
        bank[index]
      }
      0xA000..=0xBFFF if self.ram_active => {
        let bank = &self.ram_banks[self.ram_index];
        let index = usize::from(address) - 0xA000;
        bank[index]
      }
      _ => 0xFF,
    }
  }
  #[inline]
  fn write(&mut self, address: u16, byte: u8) {
    match address {
      0x0000..=0x1FFF => self.ram_active = (byte & 0xF) == 0xA,
      0x2000..=0x3FFF => {
        self.low5_bits = byte & 0b11111;
        self.update_indexes();
      }
      0x4000..=0x5FFF => {
        self.upper2_bits = byte & 0b11;
        self.update_indexes();
      }
      0x6000..=0x7FFF => {
        self.advanced_banking = (byte & 0b1) != 0;
        self.update_indexes();
      }
      0xA000..=0xBFFF if self.ram_active => {
        let bank = &mut self.ram_banks[self.ram_index];
        let index = usize::from(address) - 0xA000;
        bank[index] = byte;
      }
      _ => (),
    }
  }
}
