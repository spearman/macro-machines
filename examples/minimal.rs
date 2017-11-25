#![feature(const_fn)]
#![feature(core_intrinsics)]

#[macro_use] extern crate unwrap;
#[macro_use] extern crate log;
extern crate simplelog;
extern crate colored;
extern crate escapade;

extern crate rs_utils;

#[macro_use] extern crate macro_machines;

def_machine!{
  machine M {
    STATES [
      state S ()
      state T ()
    ]
    EVENTS [
      event A <S> => <T>
    ]
    EXTENDED []
    initial_state: S
  }
}

pub const LOG_LEVEL_FILTER
  : simplelog::LogLevelFilter = simplelog::LogLevelFilter::Trace;

fn main () {
  use std::io::Write;
  use colored::Colorize;
  use macro_machines::*;
  let example_name = &rs_utils::process::EXE_FILE_NAME;
  println!("{}", format!("{} main...", **example_name)
    .green().bold());

  unwrap!{
    simplelog::TermLogger::init (
      LOG_LEVEL_FILTER,
      simplelog::Config::default())
  };

  M::report();

  let dotfile_name = format!("{}.dot", **example_name);
  let mut f = unwrap!{ std::fs::File::create (dotfile_name) };
  unwrap!{ f.write_all (M::dotfile().as_bytes()) };
  std::mem::drop (f);

  let mut m = M::initial();
  println!("m: {:?}", m);

  let e = EventId::A.into();
  unwrap!{ m.handle_event (e) };
  println!("m: {:?}", m);

  let e = EventId::A.into();
  assert_eq!(m.handle_event (e), Err (HandleEventException::WrongState));

  println!("{}", format!("...{} main", **example_name)
    .green().bold());
}
