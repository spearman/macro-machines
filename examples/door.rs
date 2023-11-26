extern crate env_logger;
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

fn main () {
  use std::io::Write;
  use macro_machines::MachineDotfile;

  let example_name = std::env::current_exe().unwrap().file_name().unwrap()
    .to_str().unwrap().to_string();
  println!("{}: main...", example_name);

  env_logger::Builder::new()
    .filter_level (log::LevelFilter::Trace)
    .parse_default_env()
    .init();

  Door::report_sizes();

  let dotfile_name = format!("{}.dot", example_name);
  let mut f = unwrap!(std::fs::File::create (dotfile_name));
  unwrap!(f.write_all (Door::dotfile().as_bytes()));
  drop (f);

  let dotfile_name = format!("{}-show-defaults.dot", example_name);
  let mut f = unwrap!(std::fs::File::create (dotfile_name));
  unwrap!(f.write_all (Door::dotfile_show_defaults().as_bytes()));
  drop (f);

  let mut door = Door::initial();
  println!("door: {:?}", door);

  unwrap!(door.handle_event (EventId::Knock.into()));
  println!("door: {:?}", door);

  unwrap!(door.handle_event (EventId::Open.into()));
  println!("door: {:?}", door);

  unwrap!(door.handle_event (EventId::Close.into()));
  println!("door: {:?}", door);

  println!("{}: ...main", example_name);
}
