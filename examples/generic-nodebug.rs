extern crate env_logger;
extern crate unwrap;
use unwrap::unwrap;

extern crate macro_machines;
use macro_machines::def_machine_nodefault;

def_machine_nodefault!{
  machine G <X> {
    STATES [
      state S ()
      state T ()
    ]
    EVENTS [
      event A <S> => <T> ()
    ]
    EXTENDED [
      x  : X,
      rx : std::sync::mpsc::Receiver <X> = std::sync::mpsc::channel().1
    ]
    initial_state: S
  }
}

struct Nodebug {
  _foo : u8
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

  G::<Nodebug>::report_sizes();
  G::<(Nodebug,Nodebug,Nodebug)>::report_sizes();

  let dotfile_name = format!("{}.dot", example_name);
  let mut f = unwrap!(std::fs::File::create (dotfile_name));
  unwrap!(f.write_all (G::<Nodebug>::dotfile_show_defaults().as_bytes()));
  drop (f);

  //let mut g = G::<std::sync::mpsc::Receiver <f64>>::initial();
  let mut g = G::<f64>::new (
    ExtendedState::new (Some (Default::default()), None).unwrap());
  println!("g state: {:?}", g.state().id());

  let e = EventParams::A{}.into();
  unwrap!(g.handle_event (e));
  println!("g state: {:?}", g.state().id());

  let e = EventParams::A{}.into();
  assert_eq!(g.handle_event (e), Err (HandleEventException::WrongState));

  println!("{}: ...main", example_name);
}
