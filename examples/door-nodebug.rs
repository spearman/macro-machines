use env_logger;
use macro_machines::def_machine;

def_machine! {
  Door (open_count : u64) @ door {
    STATES [
      state Closed (knock_count : u64)
      state Opened ()
    ]
    EVENTS [
      event Knock <Closed> () { knock_count } => { *knock_count += 1; }
      event Open  <Closed> => <Opened> ()  {} => { *open_count += 1; }
      event Close <Opened> => <Closed> ()
    ]
    initial_state:  Closed {
      initial_action: {
        println!("hello");
        println!("open_count: {:?}", door.as_ref().open_count);
      }
    }
    terminal_state: Closed {
      terminate_success: {
        println!("open_count: {:?}", door.as_ref().open_count);
        println!("goodbye")
      }
      terminate_failure: {
        panic!("door was left: {:?}", door.state().id())
      }
    }
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

  Door::report_sizes();

  let dotfile_name = format!("{example_name}.dot");
  let mut f = std::fs::File::create (dotfile_name).unwrap();
  f.write_all (Door::dotfile_hide_actions().as_bytes()).unwrap();
  drop (f);

  let dotfile_name = format!("{example_name}-show-defaults.dot");
  let mut f = std::fs::File::create (dotfile_name).unwrap();
  f.write_all (Door::dotfile_show_defaults().as_bytes()).unwrap();
  drop (f);

  let mut door = Door::initial();
  door.handle_event (EventId::Knock.into()).unwrap();
  door.handle_event (EventId::Open.into()).unwrap();
  door.handle_event (EventId::Close.into()).unwrap();

  println!("{example_name}: ...main");
}
