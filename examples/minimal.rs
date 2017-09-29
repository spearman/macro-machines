#![feature(const_fn)]

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
      state S {}
      state T {}
    ]
    EVENTS [
      event StoT <S> => <T> {}
    ]
    DATA []
    initial_state: S
  }
}

pub const LOG_LEVEL_FILTER
  : simplelog::LogLevelFilter = simplelog::LogLevelFilter::Trace;

fn main () {
  use colored::Colorize;
  use macro_machines::*;
  println!("{}", format!("{} main...", *rs_utils::process::FILE_NAME)
    .green().bold());

  unwrap!{
    simplelog::TermLogger::init (
      LOG_LEVEL_FILTER,
      simplelog::Config::default())
  };

  let mut m = M::new();
  println!("m: {:?}", m);

  let e = EventId::StoT.into();
  unwrap!{ m.handle_event (e) };
  println!("m: {:?}", m);

  let e = EventId::StoT.into();
  assert_eq!(m.handle_event (e), Err (HandleEventException::WrongState));

  println!("{}", format!("...{} main", *rs_utils::process::FILE_NAME)
    .green().bold());
}
