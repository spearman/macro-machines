#![feature(const_fn)]

#[macro_use] extern crate unwrap;
#[macro_use] extern crate log;
extern crate simplelog;
extern crate colored;
extern crate escapade;

extern crate rs_utils;

#[macro_use] extern crate macro_machines;

def_machine! {
  Door (knock_count : u64) {
    STATES [
      state Closed {}
      state Opened {}
    ]
    EVENTS [
      event Knock <Closed> => <Closed> {}
      event Open  <Closed> => <Opened> {}
      event Close <Opened> => <Closed> {}
    ]
    initial_state:  Closed {
      initial_action: { println!("hello") }
    }
    terminal_state: Closed {
      terminate_failure: { panic!("door was left opened") }
      terminate_success: {
        println!("knock_count: {:?}", knock_count);
        println!("goodbye")
      }
    }
  }
}

pub const LOG_LEVEL_FILTER
  : simplelog::LogLevelFilter = simplelog::LogLevelFilter::Trace;

fn main () {
  use colored::Colorize;
  println!("{}", format!("{} main...", *rs_utils::process::FILE_NAME)
    .green().bold());

  unwrap!{
    simplelog::TermLogger::init (
      LOG_LEVEL_FILTER,
      simplelog::Config::default())
  };

  let mut door = Door::new();
  println!("door: {:?}", door);

  let e = EventId::Knock.into();
  unwrap!(door.handle_event (e));
  println!("door: {:?}", door);

  let e = EventId::Open.into();
  unwrap!(door.handle_event (e));
  println!("door: {:?}", door);

  let e = EventId::Close.into();
  unwrap!(door.handle_event (e));
  println!("door: {:?}", door);

  println!("{}", format!("...{} main", *rs_utils::process::FILE_NAME)
    .green().bold());
}
