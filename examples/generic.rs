#![feature(const_fn)]
#![feature(core_intrinsics)]

#[macro_use] extern crate unwrap;
#[macro_use] extern crate log;
extern crate simplelog;

#[macro_use] extern crate macro_machines;

def_machine_nodefault_debug!{
  machine G <X> {
    STATES [
      state S ()
      state T ()
    ]
    EVENTS [
      event A <S> => <T>
    ]
    EXTENDED [
      x  : X,
      rx : std::sync::mpsc::Receiver <X> = std::sync::mpsc::channel().1
    ]
    initial_state: S
  }
}

pub const LOG_LEVEL_FILTER : simplelog::LevelFilter
  = simplelog::LevelFilter::Trace;

fn main () {
  use std::io::Write;
  use macro_machines::*;
  let example_name = std::path::PathBuf::from (std::env::args().next().unwrap())
    .file_name().unwrap().to_str().unwrap().to_string();
  println!("{}", format!("{} main...", example_name));

  unwrap!(
    simplelog::TermLogger::init (LOG_LEVEL_FILTER, simplelog::Config::default())
  );

  G::<u8>::report();
  G::<f64>::report();
  G::<(f64,f64,f64)>::report();

  let dotfile_name = format!("{}.dot", example_name);
  let mut f = unwrap!(std::fs::File::create (dotfile_name));
  unwrap!(f.write_all (G::<f64>::dotfile().as_bytes()));
  std::mem::drop (f);

  //let mut g = G::<std::sync::mpsc::Receiver <f64>>::initial();
  let mut g = G::<f64>::new (
    ExtendedState::new (Some (Default::default()), None
  ).unwrap());
  println!("g: {:?}", g);

  let e = EventId::A.into();
  unwrap!(g.handle_event (e));
  println!("g: {:?}", g);

  let e = EventId::A.into();
  assert_eq!(g.handle_event (e), Err (HandleEventException::WrongState));

  println!("{}", format!("...{} main", example_name));
}
