extern crate env_logger;
extern crate unwrap;
use unwrap::unwrap;

extern crate macro_machines;
use macro_machines::def_machine_debug;

def_machine_debug!{
  machine M {
    STATES [
      state R ()
      state S ()
      state T ()
    ]
    EVENTS [
      event ToR <*> => <R> ()
      event ToT <*> => <T> ()
      event ToS <T> => <S> ()
    ]
    EXTENDED []
    initial_state: S
  }
}

fn main () {
  use std::io::Write;
  use macro_machines::{HandleEventException, MachineDotfile};

  let example_name = std::env::current_exe().unwrap().file_name().unwrap()
    .to_str().unwrap().to_string();
  println!("{}: main...", example_name);

  env_logger::Builder::new()
    .filter_level (log::LevelFilter::Trace)
    .parse_default_env()
    .init();

  M::report_sizes();

  let dotfile_name = format!("{}.dot", example_name);
  let mut f = unwrap!(std::fs::File::create (dotfile_name));
  unwrap!(f.write_all (M::dotfile_show_defaults().as_bytes()));
  drop (f);

  let mut m = M::initial();
  println!("m: {:?}", m);

  unwrap!(m.handle_event (EventId::ToR.into()));
  println!("m: {:?}", m);

  assert_eq!(m.handle_event (EventId::ToS.into()),
    Err (HandleEventException::WrongState));

  println!("{}: ...main", example_name);
}
