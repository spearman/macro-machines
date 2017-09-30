#![feature(const_fn)]
#![feature(core_intrinsics)]

#[macro_use] extern crate unwrap;
#[macro_use] extern crate log;
extern crate simplelog;
extern crate colored;
extern crate escapade;

extern crate rs_utils;

#[macro_use] extern crate macro_machines;

def_machine! {
  Door (knock_count : u64) where self = _door {
    STATES [
      state Closed {}
      state Opened {}
    ]
    EVENTS [
      event Knock <Closed> => <Closed> {
        _door.knock_count += 1;
      }
      event Open  <Closed> => <Opened> {}
      event Close <Opened> => <Closed> {}
    ]
    initial_state:  Closed {
      initial_action: {
        println!("hello");
        println!("knock_count: {:?}", _door.knock_count);
      }
    }
    terminal_state: Closed {
      terminate_failure: { panic!("door was left opened") }
      terminate_success: {
        println!("knock_count: {:?}", _door.knock_count);
        println!("goodbye")
      }
    }
  }
}

pub const LOG_LEVEL_FILTER
  : simplelog::LogLevelFilter = simplelog::LogLevelFilter::Trace;

fn main () {
  use std::io::Write;
  use colored::Colorize;
  let example_name = &rs_utils::process::FILE_NAME;
  println!("{}", format!("{} main...", **example_name)
    .green().bold());

  unwrap!{
    simplelog::TermLogger::init (
      LOG_LEVEL_FILTER,
      simplelog::Config::default())
  };

  Door::report();

  let mut f = unwrap!{ std::fs::File::create (format!("{}.dot", **example_name)) };
  unwrap!{ f.write_all (Door::dotfile().as_bytes()) };
  std::mem::drop (f);

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

  println!("{}", format!("...{} main", **example_name)
    .green().bold());
}
