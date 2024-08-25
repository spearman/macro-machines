use env_logger;
use macro_machines::def_machine_debug;

def_machine_debug!{
  machine M {
    STATES [
      state S ()
      state T (sum : u64)
    ]
    EVENTS [
      event A   <S> => <T> ()
      event Foo <T> (add : u64) { sum } => { *sum += add }
    ]
    EXTENDED []
    initial_state: S
  }
}

fn main () {
  use std::io::Write;
  use macro_machines::MachineDotfile;

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

  m.handle_event (EventId::A.into()).unwrap();
  println!("m: {m:?}");

  let e = EventParams::Foo { add: 5 }.into();
  m.handle_event (e).unwrap();
  println!("m: {m:?}");

  println!("{example_name}: ...main");
}
