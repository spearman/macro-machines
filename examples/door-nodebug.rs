#![feature(const_fn)]
#![feature(core_intrinsics)]

#[macro_use] extern crate unwrap;
#[macro_use] extern crate log;
extern crate simplelog;

#[macro_use] extern crate macro_machines;

def_machine! {
  Door (open_count : u64) where let _door = self {
    STATES [
      state Closed (knock_count : u64, code : u64 = 12345)
      state Opened ()
    ]
    EVENTS [
      event Knock <Closed> { knock_count } => {
        *knock_count += 1;
      }
      event Open  <Closed> => <Opened> {} => {
        _door.as_mut().open_count += 1;
      }
      event Close <Opened> => <Closed>
    ]
    initial_state:  Closed {
      initial_action: {
        println!("hello");
        println!("open_count: {:?}", _door.as_ref().open_count);
      }
    }
    terminal_state: Closed {
      terminate_success: {
        println!("open_count: {:?}", _door.as_ref().open_count);
        println!("goodbye")
      }
      terminate_failure: { panic!("door was left: {:?}", _door.state().id()) }
    }
  }
}

pub const LOG_LEVEL_FILTER : simplelog::LevelFilter
  = simplelog::LevelFilter::Trace;

fn main () {
  use std::io::Write;
  use macro_machines::MachineDotfile;
  let example_name = std::path::PathBuf::from (std::env::args().next().unwrap())
    .file_name().unwrap().to_str().unwrap().to_string();
  println!("{}", format!("{} main...", example_name));

  unwrap!{
    simplelog::TermLogger::init (
      LOG_LEVEL_FILTER,
      simplelog::Config::default())
  };

  Door::report();

  let dotfile_name = format!("{}.dot", example_name);
  let mut f = unwrap!{ std::fs::File::create (dotfile_name) };
  unwrap!{ f.write_all (Door::dotfile().as_bytes()) };
  std::mem::drop (f);

  let dotfile_name = format!("{}-hide-defaults.dot", example_name);
  let mut f = unwrap!{ std::fs::File::create (dotfile_name) };
  unwrap!{ f.write_all (Door::dotfile_hide_defaults().as_bytes()) };
  std::mem::drop (f);

  let mut door = Door::initial();
  let e = EventId::Knock.into();
  unwrap!(door.handle_event (e));
  let e = EventId::Open.into();
  unwrap!(door.handle_event (e));
  let e = EventId::Close.into();
  unwrap!(door.handle_event (e));

  println!("{}", format!("...{} main", example_name));
}
