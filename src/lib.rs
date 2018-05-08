//! State machine macros with logging and graphviz DOT file generation.
//!
//! [Repository](https://github.com/spearman/macro-machines)
//!
//! An example that shows a number of features of the macro syntax is a `Door`
//! state machine with:
//!
//! - two *states*: `Closed` (with *state-local variable* `knock_count`) and
//!   `Open`
//! - three *events*: one *internal event* `Knock` (with *action* on the
//!   `Closed` state) and two *external events* `Open` (wth associated action) and
//!   `Close` (without any action)
//! - an `open_count` *extended state variable* -- this variable is initialized
//!   once and is independent of the current machine state
//!
//! ```text
//! def_machine_debug! {
//!   Door (open_count : u64) @ door {
//!     STATES [
//!       state Closed (knock_count : u64)
//!       state Opened ()
//!     ]
//!     EVENTS [
//!       event Knock <Closed> () { knock_count } => { *knock_count += 1; }
//!       event Open  <Closed> => <Opened> ()  {} => { *open_count += 1; }
//!       event Close <Opened> => <Closed> ()
//!     ]
//!     initial_state:  Closed {
//!       initial_action: {
//!         println!("hello");
//!         println!("open_count: {:?}", door.as_ref().open_count);
//!       }
//!     }
//!     terminal_state: Closed {
//!       terminate_success: {
//!         println!("open_count: {:?}", door.as_ref().open_count);
//!         println!("goodbye")
//!       }
//!       terminate_failure: {
//!         panic!("door was left: {:?}", door.state())
//!       }
//!     }
//!   }
//! }
//! ```
//!
//! To optionally make the state machine accessible in initial and terminal
//! action blocks, the macro implementation requires an identifier `door` be
//! introduced here following the `@` symbol. The variable is then brought into
//! scope as an alias for a mutable self-reference in initial and terminal
//! action blocks.
//!
//! In event actions, mutable references to extended state variables will
//! implicitly be brought into scope of the associated action block, however
//! local state variables need to be explicitly listed in the LHS brace of the
//! action construct to be accessible (e.g. the `knock_count` local state
//! variable in the `Knock` event action of the current example).
//!
//! The `Door::dotfile()` function will generate a '.dot' file string that can
//! be saved and rendered as a PNG with layout generated by graphviz `dot` tool:
//!
//! ```text
//! $ dot -Tpng door.dot > door.png
//! ```
//!
//! ![](https://raw.githubusercontent.com/spearman/macro-machines/master/door.png)

#![feature(macro_reexport)]

extern crate marksman_escape;

#[macro_reexport(log, trace, debug, info, warn, error)]
extern crate log;

#[macro_use] mod macro_def;

/// Methods for DOT file creation
// TODO: if we had a proper Machine trait with associated state and event ID
// types, some of this would be redundant
pub trait MachineDotfile {
  // required
  fn name()                       -> &'static str;
  fn type_vars()                  -> Vec <String>;
  fn extended_state_names()       -> Vec <&'static str>;
  fn extended_state_types()       -> Vec <&'static str>;
  fn extended_state_defaults()    -> Vec <&'static str>;
  fn self_reference()             -> &'static str;
  fn states()                     -> Vec <&'static str>;
  fn state_data_names()           -> Vec <Vec <&'static str>>;
  fn state_data_types()           -> Vec <Vec <&'static str>>;
  fn state_data_defaults()        -> Vec <Vec <&'static str>>;
  fn state_data_pretty_defaults() -> Vec <Vec <String>>;
  fn state_initial()              -> &'static str;
  fn state_terminal()             -> &'static str;
  fn events()                     -> Vec <&'static str>;
  fn event_sources()              -> Vec <&'static str>;
  fn event_targets()              -> Vec <&'static str>;
  fn event_actions()              -> Vec <&'static str>;
  // provided: these are intended to be called by the user
  /// Generate a DOT file for the state machine that hides default expressions
  /// for state fields and extended state fields
  fn dotfile() -> String where Self : Sized {
    machine_dotfile::<Self> (true, false)
  }
  /// Generate a DOT file for the state machine that shows default expressions
  /// for state fields and extended state fields
  fn dotfile_show_defaults() -> String where Self : Sized {
    machine_dotfile::<Self> (false, false)
  }
  /// Generate a DOT file for the state machine that pretty prints the *values*
  /// of default expressions for state fields and extended state fields.
  ///
  /// &#9888; Calling this this function evaluates default expressions and
  /// pretty prints the resulting values at runtime.
  fn dotfile_pretty_defaults() -> String where Self : Sized {
    machine_dotfile::<Self> (false, true)
  }
}

/// Describes an exceptional result when attempting to handle an event.
///
/// Currently the only exception is the '`WrongState`' exception.
#[derive(Debug,PartialEq)]
pub enum HandleEventException {
  WrongState
}

//
//  private functions
//

/// Private DOT file creation function
fn machine_dotfile <M : MachineDotfile>
  (hide_defaults : bool, pretty_defaults : bool) -> String
{
  let mut s = String::new();
  //
  // begin graph
  //
  s.push_str (
    "digraph {\n  \
       rankdir=LR\n  \
       node [shape=record, style=rounded, fontname=\"Sans Bold\"]\n  \
       edge [fontname=\"Sans\"]\n");

    //
  { // begin subgraph
    //
  s.push_str (format!(
    "  subgraph cluster_{} {{\n", M::name()).as_str());
  let title_string = {
    let mut s = String::new();
    s.push_str (M::name());
    if !M::type_vars().is_empty() {
      s.push_str ("<");
      let type_vars = M::type_vars();
      for string in type_vars {
        s.push_str (string.as_str());
        s.push_str (",");
      }
      assert_eq!(s.pop(), Some (','));
      s.push_str (">");
    }
    s
  };
  s.push_str (format!("    label=<{}", escape (title_string)).as_str());

  //  extended state
  let mut mono_font           = false;
  let extended_state_names    = M::extended_state_names();
  let extended_state_types    = M::extended_state_types();
  let extended_state_defaults = M::extended_state_defaults();
  debug_assert_eq!(extended_state_names.len(), extended_state_types.len());
  debug_assert_eq!(extended_state_types.len(), extended_state_defaults.len());

  if !extended_state_names.is_empty() {
    s.push_str ("<FONT FACE=\"Mono\"><BR/><BR/>\n");
    mono_font = true;
    //  for each extended state field, print a line
    // TODO: we are manually aligning the columns of the field name and field
    // type, is there a better way ? (record node, html table, format width?)
    debug_assert!(mono_font);

    let mut extended_string = String::new();
    let separator = ",<BR ALIGN=\"LEFT\"/>\n";

    let longest_fieldname = extended_state_names.iter().fold (
      0, |longest, ref fieldname| std::cmp::max (longest, fieldname.len())
    );

    let longest_typename = extended_state_types.iter().fold (
      0, |longest, ref typename| std::cmp::max (longest, typename.len())
    );

    for (i,f) in extended_state_names.iter().enumerate() {
      let spacer1 : String = std::iter::repeat (' ')
        .take (longest_fieldname - f.len())
        .collect();
      let spacer2 : String = std::iter::repeat (' ')
        .take (longest_typename - extended_state_types[i].len())
        .collect();

      if !hide_defaults && !extended_state_defaults[i].is_empty() {
        extended_string.push_str (escape (format!(
          "{}{} : {}{} = {}",
          f, spacer1, extended_state_types[i], spacer2, extended_state_defaults[i]
        )).as_str());
      } else {
        extended_string.push_str (escape (format!(
          "{}{} : {}", f, spacer1, extended_state_types[i]
        )).as_str());
      }
      extended_string.push_str (format!("{}", separator).as_str());
    }

    let len = extended_string.len();
    extended_string.truncate (len - separator.len());
    s.push_str (format!("{}", extended_string).as_str());
  } // end extended state

  s.push_str ("<BR ALIGN=\"LEFT\"/>");
  let self_reference = M::self_reference();
  if !self_reference.is_empty() && mono_font {
    s.push_str (format!("@ {}<BR ALIGN=\"CENTER\"/>", self_reference).as_str());
  }
  if !extended_state_names.is_empty() {
    s.push_str ("\n      ");
  }

  // extended state transitions
  // TODO

  if mono_font {
    s.push_str ("</FONT><BR/>");
  }
  s.push_str (">\
    \n    shape=record\
    \n    style=rounded\
    \n    fontname=\"Sans Bold Italic\"\n");
  } // end begin subgraph

  //
  // nodes (states)
  //
  // initial node
  s.push_str (
    "    INITIAL [label=\"\", shape=circle, width=0.2, \
           style=filled, fillcolor=black]\n");
  // states
  let state_data_names    = M::state_data_names();
  let state_data_types    = M::state_data_types();
  let state_data_defaults : Vec <Vec <String>> = if !pretty_defaults {
    M::state_data_defaults().into_iter().map (
      |v| v.into_iter().map (str::to_string).collect()
    ).collect()
  } else {
    let pretty_defaults = M::state_data_pretty_defaults();
    pretty_defaults.into_iter().map (
      |v| v.into_iter().map (|pretty_newline| {
        let mut pretty_br = String::new();
        let separator = "<BR ALIGN=\"LEFT\"/>\n";
        for line in pretty_newline.lines() {
          pretty_br.push_str (escape (line.to_string()).as_str());
          pretty_br.push_str (separator);
        }
        let len = pretty_br.len();
        pretty_br.truncate (len - separator.len());
        pretty_br
      }).collect()
    ).collect()
  };
  debug_assert_eq!(state_data_names.len(), state_data_types.len());
  debug_assert_eq!(state_data_types.len(), state_data_defaults.len());

  // for each state: node
  for (i, state) in M::states().iter().enumerate() {
    let mut mono_font       = false;
    let state_data_names    = &state_data_names[i];
    let state_data_types    = &state_data_types[i];
    let state_data_defaults = &state_data_defaults[i];
    debug_assert_eq!(state_data_names.len(), state_data_types.len());
    debug_assert_eq!(state_data_types.len(), state_data_defaults.len());
    s.push_str (format!("    {} [label=<<B>{}</B>", state, state).as_str());
    // NOTE: within the mono font block leading whitespace in the source
    // is counted as part of the layout so we don't indent these lines
    if !state_data_names.is_empty() {
      if !mono_font {
        s.push_str ("|<FONT FACE=\"Mono\"><BR/>\n");
        mono_font = true;
      }
      let mut data_string = String::new();
      let separator = ",<BR ALIGN=\"LEFT\"/>\n";
      let longest_fieldname = state_data_names.iter().fold (
        0, |longest, ref fieldname| std::cmp::max (longest, fieldname.len())
      );
      let longest_typename = state_data_types.iter().fold (
        0, |longest, ref typename| std::cmp::max (longest, typename.len())
      );
      for (i,f) in state_data_names.iter().enumerate() {
        let spacer1 : String = std::iter::repeat (' ')
          .take(longest_fieldname - f.len())
          .collect();
        let spacer2 : String = std::iter::repeat (' ')
          .take(longest_typename - state_data_types[i].len())
          .collect();
        if !hide_defaults && !state_data_defaults[i].is_empty() {
          data_string.push_str (escape (format!(
            "{}{} : {}{} = {}",
            f, spacer1, state_data_types[i], spacer2, state_data_defaults[i]
          )).as_str());
        } else {
          data_string.push_str (escape (format!(
            "{}{} : {}", f, spacer1, state_data_types[i]
          )).as_str());
        }
        data_string.push_str (format!("{}", separator).as_str());
      }
      let len = data_string.len();
      data_string.truncate (len - separator.len());
      s.push_str (format!("{}", data_string).as_str());
    }

    /*
    if s.chars().last().unwrap() == '>' {
      let len = s.len();
      s.truncate (len-5);
    } else {
      s.push_str ("</FONT>");
    }
    */

    // state guards
    // TODO

    if mono_font {
      s.push_str ("<BR ALIGN=\"LEFT\"/></FONT>");
    }
    s.push_str (">]\n");
  } // end for each state: node
  // end nodes (states)

  //
  // transitions (events)
  //
  // initial transition edge
  // TODO: show initial action
  s.push_str (format!(
    "    INITIAL -> {}\n", M::state_initial()).as_str());
  let event_sources = M::event_sources();
  let event_targets = M::event_targets();
  let event_actions = M::event_actions();
  let mut universal = false;
  // for each event: transition edge
  for (i, event) in M::events().into_iter().enumerate() {
    let source = event_sources[i];
    let mut target = event_targets[i];
    let action = event_actions[i];
    if target.is_empty() {  // internal transition source == target
      target = source;
    }

    if source == "*" {
      universal = true;
    }
    s.push_str (format!(
      "    \"{}\" -> \"{}\" [label=<<FONT FACE=\"Sans Italic\">{}</FONT>",
      source, target, event
    ).as_str());

    let mut mono_font = false;
    // params
    // TODO
    // guards
    // TODO

    if !action.is_empty() {
      match action {
        // don't render empty actions
        "{}" | "{ }" => {}
        _ => {
          if !mono_font {
            s.push_str ("<FONT FACE=\"Mono\"><BR/>");
            mono_font = true;
          }
          // TODO: different formatting if params or guards were present
          //action = "  ".to_string() + action.as_str();
          s.push_str (format!("{}", escape (action.to_string())).as_str());
        }
      }
    }

    if mono_font {
      s.push_str ("</FONT>");
    }
    s.push_str (">]\n");
  } // end for each event: transition edge

  if universal {
    for state in M::states() {
      s.push_str (format!(
        "    {} -> \"*\" [style=dashed, color=gray]", state).as_str());
    }
  }

  // terminal transition: node + edge
  // TODO: show terminal action(s)
  let state_terminal = M::state_terminal();
  if !state_terminal.is_empty() {
    s.push_str (
      "    TERMINAL [label=\"\", shape=doublecircle, width=0.2,\
     \n      style=filled, fillcolor=black]\n");
    s.push_str (format!(
      "    {} -> TERMINAL\n", state_terminal).as_str());
  }
  // end transitions

  //
  //  end graph
  //
  s.push_str (
    "  }\n\
    }");
  s
} // end fn machine_dotfile

/// Escape HTML special characters
#[inline]
fn escape (s : String) -> String {
  use marksman_escape::Escape;
  String::from_utf8 (Escape::new (s.bytes()).collect()).unwrap()
}
