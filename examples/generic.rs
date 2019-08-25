#![feature(const_fn)]
#![feature(core_intrinsics)]

extern crate simplelog;
extern crate unwrap;
use unwrap::unwrap;

extern crate macro_machines;
use macro_machines::def_machine_nodefault_debug;

def_machine_nodefault_debug!{
  G <X> (
    x   : X,
    rx  : std::sync::mpsc::Receiver <X> = std::sync::mpsc::channel().1,
    foo : u64
  ) @ g {
    STATES [
      state S ()
      state T (t : u64 = *foo)
    ]
    EVENTS [
      event A <S> => <T> ()
    ]
    initial_state: S {
      initial_action: { println!("initial G: {:?}", g) }
    }
  }
}

fn main () {
  use std::io::Write;
  use macro_machines::{HandleEventException, MachineDotfile};

  let example_name = std::env::current_exe().unwrap().file_name().unwrap()
    .to_str().unwrap().to_string();
  println!("{}: main...", example_name);

  simplelog::TermLogger::init (
    simplelog::LevelFilter::Trace,
    simplelog::ConfigBuilder::new()
      .set_target_level (simplelog::LevelFilter::Error)
      .set_thread_level (simplelog::LevelFilter::Off)
      .build(),
    simplelog::TerminalMode::Stdout
  ).unwrap();

  G::<u8>::report_sizes();
  G::<f64>::report_sizes();
  G::<(f64,f64,f64)>::report_sizes();

  let dotfile_name = format!("{}.dot", example_name);
  let mut f = unwrap!(std::fs::File::create (dotfile_name));
  unwrap!(f.write_all (G::<f64>::dotfile_show_defaults().as_bytes()));
  drop (f);

  //let mut g = G::<std::sync::mpsc::Receiver <f64>>::initial();
  let mut g = G::<f64>::new (
    ExtendedState::new (Some (Default::default()), None, Some (10)).unwrap());
  println!("g: {:?}", g);

  let e = EventParams::A{}.into();
  unwrap!(g.handle_event (e));
  println!("g: {:?}", g);

  let e = EventParams::A{}.into();
  assert_eq!(g.handle_event (e), Err (HandleEventException::WrongState));

  println!("{}: ...main", example_name);
}
