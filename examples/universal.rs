#![feature(const_fn)]
#![feature(core_intrinsics)]

#[macro_use] extern crate unwrap;
#[macro_use] extern crate log;
extern crate simplelog;

#[macro_use] extern crate macro_machines;

def_machine_debug!{
  machine M {
    STATES [
      state R ()
      state S ()
      state T ()
    ]
    EVENTS [
      event ToR <*> => <R>
      event ToT <*> => <T>
      event ToS <T> => <S>
    ]
    EXTENDED []
    initial_state: S
  }
}

pub const LOG_LEVEL_FILTER : simplelog::LevelFilter
  = simplelog::LevelFilter::Trace;

fn main () {
  use std::io::Write;
  use macro_machines::{HandleEventException, MachineDotfile};
  let example_name = std::path::PathBuf::from (std::env::args().next().unwrap())
    .file_name().unwrap().to_str().unwrap().to_string();
  println!("{}", format!("{} main...", example_name));

  unwrap!(
    simplelog::TermLogger::init (LOG_LEVEL_FILTER, simplelog::Config::default())
  );

  M::report();

  let dotfile_name = format!("{}.dot", example_name);
  let mut f = unwrap!(std::fs::File::create (dotfile_name));
  unwrap!(f.write_all (M::dotfile().as_bytes()));
  std::mem::drop (f);

  let mut m = M::initial();
  println!("m: {:?}", m);

  let e = EventId::ToR.into();
  unwrap!(m.handle_event (e));
  println!("m: {:?}", m);

  let e = EventId::ToS.into();
  assert_eq!(m.handle_event (e), Err (HandleEventException::WrongState));

  println!("{}", format!("...{} main", example_name));
}
