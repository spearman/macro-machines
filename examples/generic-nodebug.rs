#![feature(const_fn)]
#![feature(core_intrinsics)]

#[macro_use] extern crate unwrap;
extern crate simplelog;

#[macro_use] extern crate macro_machines;

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

  G::<Nodebug>::report_sizes();
  G::<(Nodebug,Nodebug,Nodebug)>::report_sizes();

  let dotfile_name = format!("{}.dot", example_name);
  let mut f = unwrap!(std::fs::File::create (dotfile_name));
  unwrap!(f.write_all (G::<Nodebug>::dotfile_show_defaults().as_bytes()));
  drop (f);

  //let mut g = G::<std::sync::mpsc::Receiver <f64>>::initial();
  let mut g = G::<f64>::new (
    ExtendedState::new (Some (Default::default()), None
  ).unwrap());
  println!("g state: {:?}", g.state().id());

  let e = EventParams::A{}.into();
  unwrap!(g.handle_event (e));
  println!("g state: {:?}", g.state().id());

  let e = EventParams::A{}.into();
  assert_eq!(g.handle_event (e), Err (HandleEventException::WrongState));

  println!("{}", format!("...{} main", example_name));
}
