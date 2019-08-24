#![feature(const_fn)]
#![feature(core_intrinsics)]

extern crate simplelog;
extern crate unwrap;
use unwrap::unwrap;

extern crate macro_machines;
use macro_machines::def_machine_debug;

def_machine_debug! {
  Door (open_count : u64) @ door {
    STATES [
      state Closed (knock_count : u64) {
        exit { println!("final knock count: {}", knock_count); }
      }
      state Opened () {
        entry { println!("open count: {}", open_count); }
      }
    ]
    EVENTS [
      event Knock <Closed> () { knock_count } => { *knock_count += 1; }
      event Open  <Closed> => <Opened> ()  {} => { *open_count += 1; }
      event Close <Opened> => <Closed> ()
    ]
    initial_state:  Closed {
      initial_action: { println!("hello"); }
    }
    terminal_state: Closed {
      terminate_success: { println!("goodbye") }
      terminate_failure: {
        panic!("door was left: {:?}", door.state())
      }
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

  unwrap!(simplelog::TermLogger::init (LOG_LEVEL_FILTER,
    simplelog::Config {
      thread: None,
      target: Some (simplelog::Level::Error),
      .. simplelog::Config::default()
    },
    simplelog::TerminalMode::Stdout));


  Door::report_sizes();

  let dotfile_name = format!("{}.dot", example_name);
  let mut f = unwrap!{ std::fs::File::create (dotfile_name) };
  unwrap!(f.write_all (Door::dotfile().as_bytes()));
  drop (f);

  let dotfile_name = format!("{}-show-defaults.dot", example_name);
  let mut f = unwrap!(std::fs::File::create (dotfile_name));
  unwrap!(f.write_all (Door::dotfile_show_defaults().as_bytes()));
  drop (f);

  let mut door = Door::initial();
  println!("door: {:?}", door);

  let e = Event::from_id (EventId::Knock);
  unwrap!(door.handle_event (e));
  println!("door: {:?}", door);

  let e = Event::from_id (EventId::Open);
  unwrap!(door.handle_event (e));
  println!("door: {:?}", door);

  let e = Event::from_id (EventId::Close);
  unwrap!(door.handle_event (e));
  println!("door: {:?}", door);

  log::trace!("foo");
  log::debug!("foo");
  log::info!("foo");
  log::warn!("foo");
  log::error!("foo");

  println!("{}", format!("...{} main", example_name));
}
