use beryllium::{
  events::Event,
  init::InitFlags,
  video::{CreateWinArgs, RendererFlags},
  Sdl,
};
use gbemu::{button_state::ButtonState, mbc1::MBC1, system::System};

fn main() {
  let rom = include_bytes!("../../tests/blargg/01-special.gb");
  let mbc1 = MBC1::new(rom, None).unwrap();
  let mut system = System::from_cart(Box::new(mbc1));

  let sdl = Sdl::init(InitFlags::VIDEO);
  let win = sdl
    .create_renderer_window(
      CreateWinArgs { title: "GbEmu", width: 160 * 2, height: 144 * 2, ..Default::default() },
      RendererFlags::default(),
    )
    .unwrap();

  let button_state = ButtonState::new();

  loop {
    while let Some((event, _)) = sdl.poll_events() {
      if matches!(event, Event::Quit) {
        return;
      }
    }
    // update and simulate
    system.set_button_state(button_state);
    system.m_cycle();
    // change the title
    let title = format!("LY: {}", system.mmio().ly());
    win.set_title(&title);
    // draw
    let surface = sdl.create_surface_from(system.lcd(), 160, 144).unwrap();
    let t = win.create_texture_from_surface(&surface).unwrap();
    win.clear().unwrap();
    win.copy(&t, [0, 0, 160, 144], [0, 0, 160 * 2, 144 * 2]).unwrap();
    win.present();
  }
}
