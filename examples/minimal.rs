use env_logger;
use macro_machines::def_machine_debug;

def_machine_debug!{
  machine M {
    STATES [
      state S ()
      state T ()
    ]
    EVENTS [
      event A <S> => <T> ()
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
  println!("{example_name}: main...");

  env_logger::Builder::new()
    .filter_level (log::LevelFilter::Trace)
    .parse_default_env()
    .init();

  M::report_sizes();

  let dotfile_name = format!("{example_name}.dot");
  let mut f = std::fs::File::create (dotfile_name).unwrap();
  f.write_all (M::dotfile_show_defaults().as_bytes()).unwrap();
  drop (f);

  let mut m = M::initial();
  println!("m: {m:?}");

  let e = Event::from (EventId::A);
  m.handle_event (e).unwrap();
  println!("m: {m:?}");

  let e = Event::from (EventId::A);
  assert_eq!(m.handle_event (e), Err (HandleEventException::WrongState));

  println!("{example_name}: ...main");
}
