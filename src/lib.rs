#[derive(Debug,PartialEq)]
pub enum HandleEventException {
  WrongState
}

/// State machines with a default `initial` state.
///
/// For each extended state field, either the type must implement `Default`, or
/// else a default expression is provided following `=`.
///
/// For a state machine that requires runtime initialization, see
/// `def_machine_nodefault!`.

#[macro_export]
macro_rules! def_machine {
  //
  //  main interface
  //
  ( machine $machine:ident
      $(<$(
        $type_var:ident $(: { $($type_constraint:path),+ })*
      ),+>)*
    {
      STATES [
        $(state $state:ident (
          $($data_name:ident : $data_type:ty $(= $data_default:expr)*),*
        ))+
      ]
      EVENTS [
        $(event $event:ident <$source:ident> $(=> <$target:ident>)*
          $({ $($state_data:ident),* } => $action:block)*
        )+
      ]
      EXTENDED [
        $($ext_name:ident : $ext_type:ty $(= $ext_default:expr)*),*
      ]
      $(state_reference: $state_reference:ident)*
      initial_state: $initial:ident $({
        $(initial_action: $initial_action:block)*
      })*
      $(terminal_state: $terminal:ident $({
        $(terminate_failure: $terminate_failure:block)*
        $(terminate_success: $terminate_success:block)*
      })*)*
    }

  ) => {

    def_machine!{
      @base
      machine $machine
        $(<$($type_var $(: { $($type_constraint),+ })*),+>)*
      {
        STATES [
          $(state $state ($($data_name : $data_type $(= $data_default)*),*))+
        ]
        EVENTS [
          $(event $event <$source> $(=> <$target>)*
            $({$($state_data),*} => $action)*
          )+
        ]
        EXTENDED [
          $($ext_name : $ext_type $(= $ext_default)*),*
        ]
        $(state_reference: $state_reference)*
        initial_state: $initial $({
          $(initial_action: $initial_action)*
        })*
        $(terminal_state: $terminal $({
          $(terminate_failure: $terminate_failure)*
          $(terminate_success: $terminate_success)*
        })*)*
      }
    }

    impl $(<$($type_var),+>)* $machine $(<$($type_var),+>)* where
    $($(
      $type_var : std::fmt::Debug,
      $($($type_var : $type_constraint),+)*
    ),+)*
    {
      pub fn initial() -> Self {
        trace!("{}::initial", stringify!($machine));
        let mut _new = Self {
          state:          State::initial(),
          extended_state: ExtendedState::initial()
        };
        #[allow(unused_variables)]
        match &mut _new.extended_state {
          &mut ExtendedState { $(ref mut $ext_name,)*.. } => {
            $($($initial_action)*)*
          }
        }
        _new
      }

      def_machine!{
        @impl_fn_dotfile
        machine $machine $(<$($type_var),+>)* {
          STATES    [
            $(state $state ($($data_name : $data_type $(= $data_default)*),*))+
          ]
          EVENTS    [ $(event $event <$source> $(=> <$target>)* $($action)*)+ ]
          EVENTS_TT [ $(event $event <$source> $(=> <$target>)* $($action)*)+ ]
          EXTENDED      [ $($ext_name : $ext_type $(= $ext_default)*),* ]
          initial_state:  $initial
          $(terminal_state: $terminal $({
            $(terminate_failure: $terminate_failure)*
          })*)*
        }
      }

    }

    impl $(<$($type_var),+>)* ExtendedState $(<$($type_var),+>)* where
    $($(
      $type_var : std::fmt::Debug,
      $($($type_var : $type_constraint),+)*
    ),+)*
    {
      pub fn initial() -> Self {
        Self {
          $($ext_name: def_machine!(@expr_default $($ext_default)*)),*
        }
      }

      /// Creation method that allows overriding defaults.
      pub fn new ($($ext_name : Option <$ext_type>),*) -> Self {
        Self {
          $($ext_name: $ext_name.unwrap_or (
            def_machine!(@expr_default $($ext_default)*))
          ),*
        }
      }
    }
  };
  //  end main interface

  //
  //  alternate syntax
  //
  ( $machine:ident
    $(<$(
      $type_var:ident $(: { $($type_constraint:path),+ })*
    ),+>)*
    $(($(
      $ext_name:ident : $ext_type:ty $(= $ext_default:expr)*
    ),*))*
      $(where let $state_reference:ident = &mut self.state)*
    {
      STATES [
        $(state $state:ident (
          $($data_name:ident : $data_type:ty $(= $data_default:expr)*),*
        ))+
      ]
      EVENTS [
        $(event $event:ident <$source:ident> $(=> <$target:ident>)*
          $({ $($state_data:ident),* } => $action:block)*
        )+
      ]
      initial_state: $initial:ident $({
        $(initial_action: $initial_action:block)*
      })*
      $(terminal_state: $terminal:ident $({
        $(terminate_failure: $terminate_failure:block)*
        $(terminate_success: $terminate_success:block)*
      })*)*
    }

  ) => {

    def_machine!{
      machine $machine $(<$($type_var $(: { $($type_constraint),+ })*),+>)* {
        STATES [
          $(state $state ($($data_name : $data_type $(= $data_default)*),*))+
        ]
        EVENTS [
          $(event $event <$source> $(=> <$target>)*
            $({$($state_data),*} => $action)*
          )+
        ]
        EXTENDED [
          $($($ext_name : $ext_type $(= $ext_default)*),*)*
        ]
        $(state_reference: $state_reference)*
        initial_state: $initial $({
          $(initial_action: $initial_action)*
        })*
        $(terminal_state: $terminal $({
          $(terminate_failure: $terminate_failure)*
          $(terminate_success: $terminate_success)*
        })*)*
      }
    }

  };
  //  end alternate syntax

  //
  //  @impl_fn_handle_event
  //
  ( @impl_fn_handle_event
    machine $machine:ident {
      STATES [
        $(state $state:ident (
          $($data_name:ident : $data_type:ty $(= $data_default:expr)*),*
        ))+
      ]
      EVENTS [
        $(event $event:ident <$source:ident> $(=> <$target:ident>)*
          $({ $($state_data:ident),* } => $action:block)*
        )+
      ]
      EXTENDED [
        $($ext_name:ident : $ext_type:ty $(= $ext_default:expr)*),*
      ]
    }

  ) => {

    pub fn handle_event (&mut self, event : Event)
      -> Result <(), macro_machines::HandleEventException>
    {
      trace!("{}::handle_event: {:?}", stringify!($machine), event);
      // if only one kind of transition exists the following match expression
      // will detect the other branch as "unreachable_code"
      #[allow(unreachable_code)]
      match event.transition() {
        Transition::Internal (state_id) => {
          if self.state.id == state_id {
            // bring extended state variables into scope
            #[allow(unused_variables)]
            match &mut self.extended_state {
              &mut ExtendedState { $(ref mut $ext_name,)*.. } => {
                // map each event to an action
                match event.id {
                  $(EventId::$event => {
                    // bring local state variables into scope
                    match &mut self.state.data {
                      &mut StateData::$source {$($(ref mut $state_data,)*)*..}
                        => {
                          // only expands internal actions, unreachable otherwise
                          def_machine!{
                            @event_action_internal
                            event $event <$source> $(=> <$target>)* $($action)*
                          }
                        }
                      _ => unreachable!("current state should match event source")
                    }
                  })+
                }
              }
            }
            Ok (())
          } else {
            trace!("<<< Err: internal transition: \
              current state ({:?}) != state ({:?})",
                self.state.id, state_id);
            Err (macro_machines::HandleEventException::WrongState)
          }
        }
        Transition::External (source_id, target_id) => {
          if self.state.id == source_id {
            trace!("<<< Ok: {:?} => {:?}", source_id, target_id);
            // bring extended state variables into scope
            #[allow(unused_variables)]
            match &mut self.extended_state {
              &mut ExtendedState { $(ref mut $ext_name,)*.. } => {
                match event.id {
                  $(EventId::$event => {
                    // only expands external actions, unreachable otherwise
                    def_machine!{
                      @event_action_external
                      event $event <$source> $(=> <$target>)* $($action)*
                    }
                  })+
                }
              }
            }
            self.state = target_id.into();
            Ok (())
          } else {
            trace!("<<< Err: external transition: \
              current state ({:?}) != source state ({:?})",
                self.state.id, source_id);
            Err (macro_machines::HandleEventException::WrongState)
          }
        }
      }
    }

  };  // end @impl_fn_handle_event

  //
  //  @event_action_external
  //
  ( @event_action_external
    event $event:ident <$source:ident> => <$target:ident> $($action:block)*
  ) => {
    $($action)*
  };

  //
  //  @event_action_external: not an external event
  //
  ( @event_action_external
    event $event:ident <$source:ident> $($action:block)*
  ) => { unreachable!("not an external event") };

  //
  //  @event_action_internal
  //
  ( @event_action_internal
    event $event:ident <$source:ident> $($action:block)*
  ) => {
    $($action)*
  };

  //
  //  @event_action_internal: not an internal event
  //
  ( @event_action_internal
    event $event:ident <$source:ident> => <$target:ident> $($action:block)*
  ) => { unreachable!("not an internal event") };

  //
  //  @event_transition: external
  //
  ( @event_transition <$source:ident> => <$target:ident> ) => {
    Transition::External (StateId::$source, StateId::$target)
  };

  //
  //  @event_transition: internal
  //
  ( @event_transition <$source:ident> ) => {
    Transition::Internal (StateId::$source)
  };

  //
  //  @expr_default: override default
  //
  ( @expr_default $default:expr ) => { $default };

  //
  //  @expr_default: use default
  //
  ( @expr_default ) => { Default::default() };

  //
  //  @expr_option: Some
  //
  ( @expr_option $default:expr ) => { Some ($default) };

  //
  //  @expr_option: None
  //
  ( @expr_option ) => { None };

  //
  //  @impl_fn_dotfile
  //
  ( @impl_fn_dotfile
    machine $machine:ident $(<$($type_var:ident),+>)* {
      STATES [
        $(state $state:ident (
          $($data_name:ident : $data_type:ty $(= $data_default:expr)*),*
        ))+
      ]
      EVENTS [
        $(event $event:ident <$source:ident> $(=> <$target:ident>)*
          $($action:block)*
        )+
      ]
      EVENTS_TT $events_tt:tt
      EXTENDED [
        $($ext_name:ident : $ext_type:ty $(= $ext_default:expr)*),*
      ]
      initial_state: $initial:ident $({
        $(initial_action: $initial_action:block)*
      })*
      $(terminal_state: $terminal:ident $({
        $(terminate_failure: $terminate_failure:block)*
        $(terminate_success: $terminate_success:block)*
      })*)*
    }

  ) => {

    #[inline]
    pub fn dotfile() -> String {
      $machine$(::<$($type_var),+>)*::_dotfile (false, false)
    }

    #[inline]
    pub fn dotfile_hide_defaults() -> String {
      $machine$(::<$($type_var),+>)*::_dotfile (true, false)
    }

    #[inline]
    pub fn dotfile_pretty_defaults() -> String {
      $machine$(::<$($type_var),+>)*::_dotfile (false, true)
    }

    fn _dotfile (
      hide_defaults   : bool,
      pretty_defaults : bool
    ) -> String {
      let mut s = String::new();
      // begin graph
      s.push_str (def_machine!(@fn_dotfile_begin).as_str());

      // begin subgraph
      s.push_str (def_machine!(
        @fn_dotfile_subgraph_begin
        machine $machine $(<$($type_var),+>)* {
          EVENTS [ $(event $event <$source> $(=> <$target>)* $($action)*)+ ]
          EXTENDED   [ $($ext_name : $ext_type $(= $ext_default)*),* ]
        }
      ).as_str());

      // nodes
      s.push_str (def_machine!(@fn_dotfile_node_initial).as_str());
      if !hide_defaults {
        if !pretty_defaults {
          $(
          s.push_str (def_machine!(
            @fn_dotfile_node
            state $state ($($data_name : $data_type $(= $data_default)*),*)
            EVENTS $events_tt
          ).as_str());
          )+
        } else {
          $(
          s.push_str (def_machine!(
            @fn_dotfile_node_pretty_defaults
            state $state ($($data_name : $data_type $(= $data_default)*),*)
            EVENTS $events_tt
          ).as_str());
          )+
        }
      } else {
        $(
        s.push_str (def_machine!(
          @fn_dotfile_node_hide_defaults
          state $state ($($data_name : $data_type $(= $data_default)*),*)
          EVENTS $events_tt
        ).as_str());
        )+
      }

      // transitions
      s.push_str (
        def_machine!(@fn_dotfile_transition_initial $initial
      ).as_str());
      $(
      s.push_str (def_machine!(
        @fn_dotfile_transition
        event $event <$source> $(=> <$target>)* $($action)*
      ).as_str());
      )+

      // terminal
      $(
      s.push_str (def_machine!(@fn_dotfile_node_terminal).as_str());
      s.push_str (def_machine!(
        @fn_dotfile_transition_terminal $terminal
      ).as_str());
      )*

      //  end graph
      s.push_str (def_machine!(@fn_dotfile_end).as_str());
      s
    } // end fn dotfile

  };  // end @impl_fn_dotfile

  //
  //  @impl_fn_dotfile_nodefault
  //
  ( @impl_fn_dotfile_nodefault
    machine $machine:ident $(<$($type_var:ident),+>)* {
      STATES [
        $(state $state:ident (
          $($data_name:ident : $data_type:ty $(= $data_default:expr)*),*
        ))+
      ]
      EVENTS [
        $(event $event:ident <$source:ident> $(=> <$target:ident>)*
          $($action:block)*
        )+
      ]
      EVENTS_TT $events_tt:tt
      EXTENDED [
        $($ext_name:ident : $ext_type:ty $(= $ext_default:expr)*),*
      ]
      initial_state: $initial:ident $({
        $(initial_action: $initial_action:block)*
      })*
      $(terminal_state: $terminal:ident $({
        $(terminate_failure: $terminate_failure:block)*
        $(terminate_success: $terminate_success:block)*
      })*)*
    }

  ) => {

    #[inline]
    pub fn dotfile() -> String {
      $machine$(::<$($type_var),+>)*::_dotfile (false, false)
    }

    #[inline]
    pub fn dotfile_hide_defaults() -> String {
      $machine$(::<$($type_var),+>)*::_dotfile (true, false)
    }

    #[inline]
    pub fn dotfile_pretty_defaults() -> String {
      $machine$(::<$($type_var),+>)*::_dotfile (false, true)
    }

    fn _dotfile (
      hide_defaults   : bool,
      pretty_defaults : bool
    ) -> String {
      let mut s = String::new();
      // begin graph
      s.push_str (def_machine!(@fn_dotfile_begin).as_str());

      // begin subgraph
      s.push_str (def_machine!(
        @fn_dotfile_subgraph_begin_nodefault
        machine $machine $(<$($type_var),+>)* {
          EVENTS [ $(event $event <$source> $(=> <$target>)* $($action)*)+ ]
          EXTENDED   [ $($ext_name : $ext_type $(= $ext_default)*),* ]
        }
      ).as_str());

      // nodes
      s.push_str (def_machine!(@fn_dotfile_node_initial).as_str());
      if !hide_defaults {
        if !pretty_defaults {
          $(
          s.push_str (def_machine!(
            @fn_dotfile_node
            state $state ($($data_name : $data_type $(= $data_default)*),*)
            EVENTS $events_tt
          ).as_str());
          )+
        } else {
          $(
          s.push_str (def_machine!(
            @fn_dotfile_node_pretty_defaults
            state $state ($($data_name : $data_type $(= $data_default)*),*)
            EVENTS $events_tt
          ).as_str());
          )+
        }
      } else {
        $(
        s.push_str (def_machine!(
          @fn_dotfile_node_hide_defaults
          state $state ($($data_name : $data_type $(= $data_default)*),*)
          EVENTS $events_tt
        ).as_str());
        )+
      }

      // transitions
      s.push_str (
        def_machine!(@fn_dotfile_transition_initial $initial
      ).as_str());
      $(
      s.push_str (def_machine!(
        @fn_dotfile_transition
        event $event <$source> $(=> <$target>)* $($action)*
      ).as_str());
      )+

      // terminal
      $(
      s.push_str (def_machine!(@fn_dotfile_node_terminal).as_str());
      s.push_str (def_machine!(
        @fn_dotfile_transition_terminal $terminal
      ).as_str());
      )*

      //  end graph
      s.push_str (def_machine!(@fn_dotfile_end).as_str());
      s
    } // end fn dotfile

  };  // end @impl_fn_dotfile

  //
  //  @fn_dotfile_begin
  //
  ( @fn_dotfile_begin ) => {{
    let mut s = String::new();
    s.push_str (
      "digraph {\n  \
         rankdir=LR\n  \
         node [shape=record, style=rounded, \
          fontname=\"Sans Bold\"]\n  \
         edge [fontname=\"Sans\"]\n");
    s
  }};

  //
  //  @fn_dotfile_subgraph_begin
  //
  ( @fn_dotfile_subgraph_begin
    machine $machine:ident $(<$($type_var:ident),+>)* {
      EVENTS [
        $(event $event:ident <$source:ident> $(=> <$target:ident>)*
          $($action:block)*
        )+
      ]
      EXTENDED [
        $($ext_name:ident : $ext_type:ty $(= $ext_default:expr)*),*
      ]
    }

  ) => {{
    use escapade::Escapable;
    let mut s = String::new();
    s.push_str (format!(
      "  subgraph cluster_{} {{\n", stringify!($machine)).as_str());
    let title_string = {
      let mut s = String::new();
      s.push_str (stringify!($machine));
      $(
      s.push_str (format!("<{}>",
        stringify!($($type_var),+)).as_str());
      )*
      s
    };
    s.push_str (
      format!("    label=<{}", title_string.escape().into_inner()).as_str());

    let mut _mono_font         = false;
    let mut _extended_fields   = Vec::<String>::new();
    let mut _extended_types    = Vec::<String>::new();
    let mut _extended_defaults = Vec::<String>::new();

    // NOTE: within the mono font block leading whitespace in the source
    // is counted as part of the layout so we don't indent these lines
    $({
      if !_mono_font {
        s.push_str ("<FONT FACE=\"Mono\"><BR/><BR/>\n");
        _mono_font = true;
      }
      _extended_fields.push (stringify!($ext_name).to_string());
      _extended_types.push (stringify!($ext_type).to_string());
      let default_val : $ext_type
        = def_machine!(@expr_default $($ext_default)*);
      _extended_defaults.push (format!("{:?}", default_val));
    })*

    debug_assert_eq!(_extended_fields.len(), _extended_types.len());
    debug_assert_eq!(_extended_types.len(), _extended_defaults.len());

    //
    //  for each extended state field, print a line
    //
    // TODO: we are manually aligning the columns of the field name and field
    // type, is there a better way ? (record node, html table, format width?)
    if !_extended_types.is_empty() {
      debug_assert!(_mono_font);
      debug_assert!(!_extended_defaults.is_empty());

      let mut extended_string = String::new();
      let separator = ",<BR ALIGN=\"LEFT\"/>\n";

      let longest_fieldname = _extended_fields.iter().fold (0,
        |longest, ref fieldname| {
          let len = fieldname.len();
          if longest < len {
            len
          } else {
            longest
          }
        }
      );

      let longest_typename = _extended_types.iter().fold (0,
        |longest, ref typename| {
          let len = typename.len();
          if longest < len {
            len
          } else {
            longest
          }
        }
      );

      for (i,f) in _extended_fields.iter().enumerate() {
        use escapade::Escapable;

        let spacer1 : String = std::iter::repeat (' ')
          .take(longest_fieldname - f.len())
          .collect();
        let spacer2 : String = std::iter::repeat (' ')
          .take (longest_typename - _extended_types[i].len())
          .collect();

        extended_string.push_str (
          format!("{}{} : {}{} = {}",
            f, spacer1, _extended_types[i], spacer2, _extended_defaults[i])
          .escape().into_inner().as_str()
        );
        extended_string.push_str (format!("{}", separator).as_str());
      }

      let len = extended_string.len();
      extended_string.truncate (len - separator.len());
      s.push_str (format!("{}", extended_string).as_str());
    }

    // TODO
    /*
    // internal state transitions
    let mut _extended_once = false;
    $({
      match EventId::$event.transition() {

        Transition::Internal => {
          if !_extended_once {
            s.push_str (
              "<BR ALIGN=\"LEFT\"/></FONT>\
               <BR ALIGN=\"LEFT\"/>\
               <FONT FACE=\"Sans Italic\">");
            _extended_once = true;
          } else {
            s.push_str ("<BR ALIGN=\"LEFT\"/></FONT>");
            s.push_str ("<BR ALIGN=\"LEFT\"/>\
              <FONT FACE=\"Sans Italic\">");
          }
          s.push_str (format!("{} </FONT><FONT FACE=\"Mono\">(",
            stringify!($event)).as_str());

          $(
          {
            use escapade::Escapable;
            let default_val : $param_type = def_machine!{
              @expr_default $($param_default)*
            };
            s.push_str (
              format!("{} : {} = {}, ",
                stringify!($param_name),
                stringify!($param_type).escape().into_inner(),
                format!("{:?}", default_val).escape().into_inner()
            ).as_str());
          }
          )*

          if unwrap!{ s.chars().last() } != '(' {
            debug_assert_eq!(unwrap!{ s.chars().last() }, ' ');
            let len = s.len();
            s.truncate (len-2);
          }
          s.push_str (")<BR ALIGN=\"LEFT\"/>");

          $(
          let mut guard = stringify!($guard_expr).to_string();
          if guard.as_str() != "true" {
            use escapade::Escapable;
            guard = "  [ ".to_string() + guard.as_str();
            guard.push (' ');
            guard.push (']');
            let guard = guard.escape().into_inner();
            s.push_str (format!("{}<BR ALIGN=\"LEFT\"/>", guard).as_str());
          }
          )*

          $(
          let mut action = stringify!($action_expr).to_string();
          if action.as_str() != "()" {
            use escapade::Escapable;
            if unwrap!{ action.chars().next() } != '{' {
              debug_assert!(unwrap!{ action.chars().last() } != '}');
              action = "  { ".to_string() + action.as_str();
              action.push_str (" }");
            } else {
              debug_assert_eq!(unwrap!{ action.chars().last() }, '}');
              action = "  ".to_string() + action.as_str();
            }
            let action = action.escape().into_inner();
            s.push_str (format!("{}", action).as_str());
          }
          )*

        }
        _ => ()

      }

    })+
    // end internal state transitions
    */

    s.push_str ("<BR ALIGN=\"LEFT\"/>");
    if !_extended_types.is_empty() {
      s.push_str ("\n      ");
    }
    if _mono_font {
      s.push_str ("</FONT><BR/>");
    }
    s.push_str (">\
      \n    shape=record\
      \n    style=rounded\
      \n    fontname=\"Sans Bold Italic\"\n");
    s
  }}; // end @fn_dotfile_subgraph_begin

  //
  //  @fn_dotfile_subgraph_begin_nodefault
  //
  //  Expressions without a provided default are replaced with an empty string
  //  instead of a `Default::default()` instance.
  ( @fn_dotfile_subgraph_begin_nodefault
    machine $machine:ident $(<$($type_var:ident),+>)* {
      EVENTS [
        $(event $event:ident <$source:ident> $(=> <$target:ident>)*
          $($action:block)*
        )+
      ]
      EXTENDED [
        $($ext_name:ident : $ext_type:ty $(= $ext_default:expr)*),*
      ]
    }

  ) => {{
    use escapade::Escapable;
    let mut s = String::new();
    s.push_str (format!(
      "  subgraph cluster_{} {{\n", stringify!($machine)).as_str());
    let title_string = {
      let mut s = String::new();
      s.push_str (stringify!($machine));
      $(
      s.push_str (format!("<{}>",
        stringify!($($type_var),+)).as_str());
      )*
      s
    };
    s.push_str (
      format!("    label=<{}", title_string.escape().into_inner()).as_str());

    let mut _mono_font         = false;
    let mut _extended_fields   = Vec::<String>::new();
    let mut _extended_types    = Vec::<String>::new();
    let mut _extended_defaults = Vec::<String>::new();

    // NOTE: within the mono font block leading whitespace in the source
    // is counted as part of the layout so we don't indent these lines
    $({
      if !_mono_font {
        s.push_str ("<FONT FACE=\"Mono\"><BR/><BR/>\n");
        _mono_font = true;
      }
      _extended_fields.push (stringify!($ext_name).to_string());
      _extended_types.push (stringify!($ext_type).to_string());
      let default_val : Option <$ext_type>
        = def_machine!(@expr_option $($ext_default)*);
      if let Some (default_val) = default_val {
        _extended_defaults.push (format!("{:?}", default_val));
      } else {
        _extended_defaults.push (String::new());
      }
    })*

    debug_assert_eq!(_extended_fields.len(), _extended_types.len());
    debug_assert_eq!(_extended_types.len(), _extended_defaults.len());

    //
    //  for each extended state field, print a line
    //
    // TODO: we are manually aligning the columns of the field name and field
    // type, is there a better way ? (record node, html table, format width?)
    if !_extended_types.is_empty() {
      debug_assert!(!_extended_defaults.is_empty());

      s.push_str ("\n      ");

      let mut extended_string = String::new();
      let separator = ",<BR ALIGN=\"LEFT\"/>\n      ";

      let longest_fieldname = _extended_fields.iter().fold (0,
        |longest, ref fieldname| {
          let len = fieldname.len();
          if longest < len {
            len
          } else {
            longest
          }
        }
      );

      let longest_typename = _extended_types.iter().fold (0,
        |longest, ref typename| {
          let len = typename.len();
          if longest < len {
            len
          } else {
            longest
          }
        }
      );

      for (i,f) in _extended_fields.iter().enumerate() {
        use escapade::Escapable;

        let spacer1 : String = std::iter::repeat (' ')
          .take(longest_fieldname - f.len())
          .collect();
        let _spacer2 : String = std::iter::repeat (' ')
          .take (longest_typename - _extended_types[i].len())
          .collect();

        if _extended_defaults[i].is_empty() {
          extended_string.push_str (
            format!("{}{} : {}", f, spacer1, _extended_types[i])
              .escape().into_inner().as_str()
          );
        } else {
          extended_string.push_str (
            format!("{}{} : {}{} = {}",
              f, spacer1, _extended_types[i], _spacer2, _extended_defaults[i])
            .escape().into_inner().as_str()
          );
        }
        extended_string.push_str (format!("{}", separator).as_str());
      }

      let len = extended_string.len();
      extended_string.truncate (len - separator.len());
      s.push_str (format!("{}", extended_string).as_str());
    }

    // TODO
    /*
    // internal state transitions
    let mut _extended_once = false;
    $({
      match EventId::$event.transition() {

        Transition::Internal => {
          if !_extended_once {
            s.push_str (
              "<BR ALIGN=\"LEFT\"/></FONT>\
               <BR ALIGN=\"LEFT\"/>\
               <FONT FACE=\"Sans Italic\">");
            _extended_once = true;
          } else {
            s.push_str ("<BR ALIGN=\"LEFT\"/></FONT>");
            s.push_str ("<BR ALIGN=\"LEFT\"/>\
              <FONT FACE=\"Sans Italic\">");
          }
          s.push_str (format!("{} </FONT><FONT FACE=\"Mono\">(",
            stringify!($event)).as_str());

          $(
          {
            use escapade::Escapable;
            let default_val : $param_type = def_machine!{
              @expr_default $($param_default)*
            };
            s.push_str (
              format!("{} : {} = {}, ",
                stringify!($param_name),
                stringify!($param_type).escape().into_inner(),
                format!("{:?}", default_val).escape().into_inner()
            ).as_str());
          }
          )*

          if unwrap!{ s.chars().last() } != '(' {
            debug_assert_eq!(unwrap!{ s.chars().last() }, ' ');
            let len = s.len();
            s.truncate (len-2);
          }
          s.push_str (")<BR ALIGN=\"LEFT\"/>");

          $(
          let mut guard = stringify!($guard_expr).to_string();
          if guard.as_str() != "true" {
            use escapade::Escapable;
            guard = "  [ ".to_string() + guard.as_str();
            guard.push (' ');
            guard.push (']');
            let guard = guard.escape().into_inner();
            s.push_str (format!("{}<BR ALIGN=\"LEFT\"/>", guard).as_str());
          }
          )*

          $(
          let mut action = stringify!($action_expr).to_string();
          if action.as_str() != "()" {
            use escapade::Escapable;
            if unwrap!{ action.chars().next() } != '{' {
              debug_assert!(unwrap!{ action.chars().last() } != '}');
              action = "  { ".to_string() + action.as_str();
              action.push_str (" }");
            } else {
              debug_assert_eq!(unwrap!{ action.chars().last() }, '}');
              action = "  ".to_string() + action.as_str();
            }
            let action = action.escape().into_inner();
            s.push_str (format!("{}", action).as_str());
          }
          )*

        }
        _ => ()

      }

    })+
    // end internal state transitions
    */

    s.push_str ("<BR ALIGN=\"LEFT\"/>");
    if !_extended_types.is_empty() {
      s.push_str ("\n      ");
    }
    if _mono_font {
      s.push_str ("</FONT><BR/>");
    }
    s.push_str ( ">\
      \n    shape=record\
      \n    style=rounded\
      \n    fontname=\"Sans Bold Italic\"\n");
    s
  }}; // end @fn_dotfile_subgraph_begin

  //
  //  @fn_dotfile_node
  //
  ( @fn_dotfile_node
    state $state:ident (
      $($data_name:ident : $data_type:ty $(= $data_default:expr)*),*
    )
    EVENTS [
      $(event $event:ident <$source:ident> $(=> <$target:ident>)*
        $($action:block)*
      )+
    ]

  ) => {{
    let mut s = String::new();

    s.push_str (format!(
      "    {:?} [label=<<B>{:?}</B>",
      StateId::$state, StateId::$state).as_str());

    let mut _mono_font     = false;
    let mut _data_fields   = Vec::<String>::new();
    let mut _data_types    = Vec::<String>::new();
    let mut _data_defaults = Vec::<String>::new();

    // NOTE: within the mono font block leading whitespace in the source
    // is counted as part of the layout so we don't indent these lines
    $({
      if !_mono_font {
        s.push_str ("|<FONT FACE=\"Mono\"><BR/>\n");
        _mono_font = true;
      }
      _data_fields.push (stringify!($data_name).to_string());
      _data_types.push (stringify!($data_type).to_string());
      let default_val : $data_type
        = def_machine!(@expr_default $($data_default)*);
      _data_defaults.push (format!("{:?}", default_val));
    })*

    debug_assert_eq!(_data_fields.len(), _data_types.len());
    debug_assert_eq!(_data_types.len(),  _data_defaults.len());

    //
    //  for each data field, print a line
    //
    // TODO: we are manually aligning the columns of the field
    // name, field type, and default values, is there a better
    // way ? (record node, html table, format width?)
    if !_data_types.is_empty() {
      debug_assert!(_mono_font);
      debug_assert!(!_data_defaults.is_empty());

      let mut data_string = String::new();
      let separator = ",<BR ALIGN=\"LEFT\"/>\n";

      let longest_fieldname = _data_fields.iter().fold (0,
        |longest, ref fieldname| {
          let len = fieldname.len();
          if longest < len {
            len
          } else {
            longest
          }
        }
      );

      let longest_typename = _data_types.iter().fold (0,
        |longest, ref typename| {
          let len = typename.len();
          if longest < len {
            len
          } else {
            longest
          }
        }
      );

      for (i,f) in _data_fields.iter().enumerate() {
        use escapade::Escapable;

        let spacer1 : String = std::iter::repeat (' ')
          .take(longest_fieldname - f.len())
          .collect();
        let spacer2 : String = std::iter::repeat (' ')
          .take(longest_typename - _data_types[i].len())
          .collect();

        data_string.push_str (
          format!("{}{} : {}{} = {}",
            f, spacer1, _data_types[i], spacer2, _data_defaults[i]
          ).escape().into_inner().as_str()
        );
        data_string.push_str (format!("{}", separator).as_str());
      }

      let len = data_string.len();
      data_string.truncate (len - separator.len());
      s.push_str (format!("{}", data_string).as_str());
    }

    /*
    if unwrap!{ s.chars().last() } == '>' {
      let len = s.len();
      s.truncate (len-5);
    } else {
      s.push_str ("</FONT>");
    }
    */

    // TODO: state guards
    /*
    $({
      use escapade::Escapable;
      if !mono_font {
        s.push_str ("|<FONT FACE=\"Mono\">");
        mono_font = true;
      }
      s.push_str ("<BR ALIGN=\"LEFT\"/>");
      let mut entry_string = "entry [".to_string();
      $(
      entry_string.push_str (
        format!(" {} ", stringify!($entry_guard)).as_str());
      )*
      entry_string.push_str ("] {");
      $(
      entry_string.push_str (
        format!(" {} ", stringify!($entry_action)).as_str());
      )*
      entry_string.push_str ("}");
      s.push_str (entry_string.escape().into_inner().as_str());
    })*

    $({
      use escapade::Escapable;
      if !mono_font {
        s.push_str ("|<FONT FACE=\"Mono\">");
        mono_font = true;
      }
      s.push_str ("<BR ALIGN=\"LEFT\"/>");
      let mut exit_string = "exit  [".to_string();
      $(
      exit_string.push_str (
        format!(" {} ", stringify!($exit_guard)).as_str());
      )*
      exit_string.push_str ("] {");
      $(
      exit_string.push_str (
        format!(" {} ", stringify!($exit_action)).as_str());
      )*
      exit_string.push_str ("}");
      s.push_str (exit_string.escape().into_inner().as_str());
    })*

    // internal transitions
    let mut _internal_once = false;
    $({
      if !mono_font {
        s.push_str ("|<FONT FACE=\"Mono\">");
        mono_font = true;
      }
      match EventId::$event.transition() {

        Transition::Internal (StateId::$state) => {
          if !_internal_once {
            s.push_str (
              "<BR ALIGN=\"LEFT\"/></FONT>|\
               <BR ALIGN=\"LEFT\"/>\
               <FONT FACE=\"Sans Italic\">");
            _internal_once = true;
          } else {
            s.push_str ("<BR ALIGN=\"LEFT\"/></FONT>");
            s.push_str ("<BR ALIGN=\"LEFT\"/>\
              <FONT FACE=\"Sans Italic\">");
          }
          s.push_str (format!("{} </FONT><FONT FACE=\"Mono\">(",
            stringify!($event)).as_str());

          $({
            use escapade::Escapable;
            let default_val : $param_type = def_machine!{
              @expr_default $($param_default)*
            };
            s.push_str (
              format!("{} : {} = {}, ",
                stringify!($param_name),
                stringify!($param_type).escape().into_inner(),
                format!("{:?}", default_val).escape().into_inner()
            ).as_str());
          })*

          if unwrap!{ s.chars().last() } != '(' {
            debug_assert_eq!(unwrap!{ s.chars().last() }, ' ');
            let len = s.len();
            s.truncate (len-2);
          }
          s.push_str (")<BR ALIGN=\"LEFT\"/>");

          $(
          let mut guard = stringify!($guard_expr).to_string();
          if guard.as_str() != "true" {
            use escapade::Escapable;
            guard = "  [ ".to_string() + guard.as_str();
            guard.push (' ');
            guard.push (']');
            let guard = guard.escape().into_inner();
            s.push_str (format!("{}<BR ALIGN=\"LEFT\"/>", guard).as_str());
          }
          )*

          $(
          let mut action = stringify!($action_expr).to_string();
          if action.as_str() != "()" {
            use escapade::Escapable;
            if unwrap!{ action.chars().next() } != '{' {
              debug_assert!(unwrap!{ action.chars().last() } != '}');
              action = "  { ".to_string() + action.as_str();
              action.push_str (" }");
            } else {
              debug_assert_eq!(unwrap!{ action.chars().last() }, '}');
              action = "  ".to_string() + action.as_str();
            }
            let action = action.escape().into_inner();
            s.push_str (format!("{}", action).as_str());
          }
          )*

        },
        _ => ()

      }

    })+
    // end internal transitions
    if mono_font {
      s.push_str ("<BR ALIGN=\"LEFT\"/></FONT>");
    }
    */
    if _mono_font {
      s.push_str ("</FONT><BR/>");
    }
    s.push_str (">]\n");
    s
  }};  // end @fn_dotfile_node

  //
  //  @fn_dotfile_node_pretty_defaults
  //
  ( @fn_dotfile_node_pretty_defaults
    state $state:ident (
      $($data_name:ident : $data_type:ty $(= $data_default:expr)*),*
    )
    EVENTS [
      $(event $event:ident <$source:ident> $(=> <$target:ident>)*
        $($action:block)*
      )+
    ]

  ) => {{
    let mut s = String::new();

    s.push_str (format!(
      "    {:?} [label=<<B>{:?}</B>",
      StateId::$state, StateId::$state).as_str());

    let mut _mono_font     = false;
    let mut _data_fields   = Vec::<String>::new();
    let mut _data_types    = Vec::<String>::new();
    let mut _data_defaults = Vec::<String>::new();

    // NOTE: within the mono font block leading whitespace in the source
    // is counted as part of the layout so we don't indent these lines
    $({
      if !_mono_font {
        s.push_str ("|<FONT FACE=\"Mono\"><BR/>\n");
        _mono_font = true;
      }
      _data_fields.push (stringify!($data_name).to_string());
      _data_types.push (stringify!($data_type).to_string());
      let default_val : $data_type
        = def_machine!(@expr_default $($data_default)*);
      let pretty_br = {
        use escapade::Escapable;
        let pretty_newline = format!("{:#?}", default_val);
        let mut pretty_br = String::new();
        let separator = "<BR ALIGN=\"LEFT\"/>\n";
        for line in pretty_newline.lines() {
          pretty_br.push_str (line.escape().into_inner().as_str());
          pretty_br.push_str (separator);
        }
        let len = pretty_br.len();
        pretty_br.truncate (len - separator.len());
        pretty_br
      };
      _data_defaults.push (pretty_br);
    })*

    debug_assert_eq!(_data_fields.len(), _data_types.len());
    debug_assert_eq!(_data_types.len(),  _data_defaults.len());

    //
    //  for each data field, print a line
    //
    // TODO: we are manually aligning the columns of the field
    // name, field type, and default values, is there a better
    // way ? (record node, html table, format width?)
    if !_data_types.is_empty() {
      debug_assert!(_mono_font);
      debug_assert!(!_data_defaults.is_empty());

      let mut data_string = String::new();
      let separator = ",<BR ALIGN=\"LEFT\"/>\n";

      let longest_fieldname = _data_fields.iter().fold (0,
        |longest, ref fieldname| {
          let len = fieldname.len();
          if longest < len {
            len
          } else {
            longest
          }
        }
      );

      let longest_typename = _data_types.iter().fold (0,
        |longest, ref typename| {
          let len = typename.len();
          if longest < len {
            len
          } else {
            longest
          }
        }
      );

      for (i,f) in _data_fields.iter().enumerate() {
        use escapade::Escapable;

        let spacer1 : String = std::iter::repeat (' ')
          .take(longest_fieldname - f.len())
          .collect();
        let spacer2 : String = std::iter::repeat (' ')
          .take(longest_typename - _data_types[i].len())
          .collect();

        data_string.push_str (
          format!("{}{} : {}{} = ",
            f, spacer1, _data_types[i], spacer2
          ).escape().into_inner().as_str()
        );
        data_string.push_str (_data_defaults[i].as_str());
        data_string.push_str (format!("{}", separator).as_str());
      }

      let len = data_string.len();
      data_string.truncate (len - separator.len());
      s.push_str (format!("{}", data_string).as_str());
    }

    /*
    if unwrap!{ s.chars().last() } == '>' {
      let len = s.len();
      s.truncate (len-5);
    } else {
      s.push_str ("</FONT>");
    }
    */

    // TODO: state guards
    /*
    $({
      use escapade::Escapable;
      if !mono_font {
        s.push_str ("|<FONT FACE=\"Mono\">");
        mono_font = true;
      }
      s.push_str ("<BR ALIGN=\"LEFT\"/>");
      let mut entry_string = "entry [".to_string();
      $(
      entry_string.push_str (
        format!(" {} ", stringify!($entry_guard)).as_str());
      )*
      entry_string.push_str ("] {");
      $(
      entry_string.push_str (
        format!(" {} ", stringify!($entry_action)).as_str());
      )*
      entry_string.push_str ("}");
      s.push_str (entry_string.escape().into_inner().as_str());
    })*

    $({
      use escapade::Escapable;
      if !mono_font {
        s.push_str ("|<FONT FACE=\"Mono\">");
        mono_font = true;
      }
      s.push_str ("<BR ALIGN=\"LEFT\"/>");
      let mut exit_string = "exit  [".to_string();
      $(
      exit_string.push_str (
        format!(" {} ", stringify!($exit_guard)).as_str());
      )*
      exit_string.push_str ("] {");
      $(
      exit_string.push_str (
        format!(" {} ", stringify!($exit_action)).as_str());
      )*
      exit_string.push_str ("}");
      s.push_str (exit_string.escape().into_inner().as_str());
    })*

    // internal transitions
    let mut _internal_once = false;
    $({
      if !mono_font {
        s.push_str ("|<FONT FACE=\"Mono\">");
        mono_font = true;
      }
      match EventId::$event.transition() {

        Transition::Internal (StateId::$state) => {
          if !_internal_once {
            s.push_str (
              "<BR ALIGN=\"LEFT\"/></FONT>|\
               <BR ALIGN=\"LEFT\"/>\
               <FONT FACE=\"Sans Italic\">");
            _internal_once = true;
          } else {
            s.push_str ("<BR ALIGN=\"LEFT\"/></FONT>");
            s.push_str ("<BR ALIGN=\"LEFT\"/>\
              <FONT FACE=\"Sans Italic\">");
          }
          s.push_str (format!("{} </FONT><FONT FACE=\"Mono\">(",
            stringify!($event)).as_str());

          $({
            use escapade::Escapable;
            let default_val : $param_type = def_machine!{
              @expr_default $($param_default)*
            };
            s.push_str (
              format!("{} : {} = {}, ",
                stringify!($param_name),
                stringify!($param_type).escape().into_inner(),
                format!("{:?}", default_val).escape().into_inner()
            ).as_str());
          })*

          if unwrap!{ s.chars().last() } != '(' {
            debug_assert_eq!(unwrap!{ s.chars().last() }, ' ');
            let len = s.len();
            s.truncate (len-2);
          }
          s.push_str (")<BR ALIGN=\"LEFT\"/>");

          $(
          let mut guard = stringify!($guard_expr).to_string();
          if guard.as_str() != "true" {
            use escapade::Escapable;
            guard = "  [ ".to_string() + guard.as_str();
            guard.push (' ');
            guard.push (']');
            let guard = guard.escape().into_inner();
            s.push_str (format!("{}<BR ALIGN=\"LEFT\"/>", guard).as_str());
          }
          )*

          $(
          let mut action = stringify!($action_expr).to_string();
          if action.as_str() != "()" {
            use escapade::Escapable;
            if unwrap!{ action.chars().next() } != '{' {
              debug_assert!(unwrap!{ action.chars().last() } != '}');
              action = "  { ".to_string() + action.as_str();
              action.push_str (" }");
            } else {
              debug_assert_eq!(unwrap!{ action.chars().last() }, '}');
              action = "  ".to_string() + action.as_str();
            }
            let action = action.escape().into_inner();
            s.push_str (format!("{}", action).as_str());
          }
          )*

        },
        _ => ()

      }

    })+
    // end internal transitions
    if mono_font {
      s.push_str ("<BR ALIGN=\"LEFT\"/></FONT>");
    }
    */
    if _mono_font {
      s.push_str ("</FONT><BR/>");
    }
    s.push_str (">]\n");
    s
  }};  // end @fn_dotfile_node_pretty_defaults

  //
  //  @fn_dotfile_node_hide_defaults
  //
  ( @fn_dotfile_node_hide_defaults
    state $state:ident (
      $($data_name:ident : $data_type:ty $(= $data_default:expr)*),*
    )
    EVENTS [
      $(event $event:ident <$source:ident> $(=> <$target:ident>)*
        $($action:block)*
      )+
    ]

  ) => {{
    let mut s = String::new();

    s.push_str (format!(
      "    {:?} [label=<<B>{:?}</B>",
      StateId::$state, StateId::$state).as_str());

    let mut _mono_font     = false;
    let mut _data_fields   = Vec::<String>::new();
    let mut _data_types    = Vec::<String>::new();

    // NOTE: within the mono font block leading whitespace in the source
    // is counted as part of the layout so we don't indent these lines
    $({
      if !_mono_font {
        s.push_str ("|<FONT FACE=\"Mono\"><BR/>\n");
        _mono_font = true;
      }
      _data_fields.push (stringify!($data_name).to_string());
      _data_types.push (stringify!($data_type).to_string());
    })*

    debug_assert_eq!(_data_fields.len(), _data_types.len());

    //
    //  for each data field, print a line
    //
    // TODO: we are manually aligning the columns of the field name, field
    // type, is there a better way ? (record node, html table, format width?)
    if !_data_types.is_empty() {
      debug_assert!(_mono_font);

      let mut data_string = String::new();
      let separator = ",<BR ALIGN=\"LEFT\"/>\n";

      let longest_fieldname = _data_fields.iter().fold (0,
        |longest, ref fieldname| {
          let len = fieldname.len();
          if longest < len {
            len
          } else {
            longest
          }
        }
      );

      for (i,f) in _data_fields.iter().enumerate() {
        use escapade::Escapable;

        let spacer1 : String = std::iter::repeat (' ')
          .take(longest_fieldname - f.len())
          .collect();

        data_string.push_str (
          format!("{}{} : {}", f, spacer1, _data_types[i])
            .escape().into_inner().as_str()
        );
        data_string.push_str (format!("{}", separator).as_str());
      }

      let len = data_string.len();
      data_string.truncate (len - separator.len());
      s.push_str (format!("{}", data_string).as_str());
    }

    if _mono_font {
      s.push_str ("</FONT><BR ALIGN=\"LEFT\"/>");
    }
    s.push_str (">]\n");
    s
  }};  // end @fn_dotfile_node_hide_defaults

  //
  //  @fn_dotfile_node_initial
  //
  ( @fn_dotfile_node_initial ) => {{
    let mut s = String::new();
    s.push_str (
      "    INITIAL [label=\"\", shape=circle, width=0.2,\
      \n      style=filled, fillcolor=black]\n");
    s
  }};

  //
  //  @fn_dotfile_node_terminal
  //
  ( @fn_dotfile_node_terminal ) => {{
    let mut s = String::new();
    s.push_str (
      "    TERMINAL [label=\"\", shape=doublecircle, width=0.2,\
      \n      style=filled, fillcolor=black]\n");
    s
  }};

  //
  //  @fn_dotfile_transition: external
  //
  ( @fn_dotfile_transition
    event $event:ident <$source:ident> => <$target:ident> $($action:block)*

  ) => {{
    let mut s = String::new();
    s.push_str (format!(
      "    {:?} -> {:?} [label=<<FONT FACE=\"Sans Italic\">{:?}\
           </FONT>",
      StateId::$source, StateId::$target, EventId::$event).as_str());

    let mut _mono_font = false;
    // TODO: params
    /*
    let mut open_params = false;
    $({
      use escapade::Escapable;
      if !open_params {
        if !_mono_font {
          s.push_str ("<FONT FACE=\"Mono\">");
          _mono_font = true;
        }
        s.push_str ("(".as_str());
        open_params = true;
      }
      let default_val : $param_type = def_machine!{
        @expr_default $($param_default)*
      };
      s.push_str (
        format!("{} : {} = {}, ",
          stringify!($param_name),
          stringify!($param_type).escape().into_inner(),
          format!("{:?}", default_val).escape().into_inner()
      ).as_str());
    })*

    if unwrap!{ s.chars().last() } != '(' {
      debug_assert_eq!(unwrap!{ s.chars().last() }, ' ');
      let len = s.len();
      s.truncate (len-2);
    }
    if open_params {
      s.push_str (")<BR ALIGN=\"LEFT\"/>");
    }
    */

    // TODO: guards
    /*
    $(
      if !_mono_font {
        s.push_str ("<FONT FACE=\"Mono\">");
        _mono_font = true;
      }
      let mut guard = stringify!($guard_expr).to_string();
      if guard.as_str() != "true" {
        use escapade::Escapable;
        guard = "  [ ".to_string() + guard.as_str();
        guard.push (' ');
        guard.push (']');
        let guard = guard.escape().into_inner();
        s.push_str (format!("{}<BR ALIGN=\"LEFT\"/>", guard).as_str());
      }
    )*
    */

    $(
    let /*mut*/ action = stringify!($action).to_string();
    match action.as_str() {
      // don't render empty actions
      "{}" | "{ }" => {}
      _ => {
        use escapade::Escapable;
        if !_mono_font {
          s.push_str ("<FONT FACE=\"Mono\"><BR/>");
          _mono_font = true;
        }
        // TODO: different formatting if params or guards were present
        //action = "  ".to_string() + action.as_str();
        let action = action.escape().into_inner();
        s.push_str (format!("{}", action).as_str());
      }
    }
    )*

    if _mono_font {
      s.push_str ("</FONT>");
    }
    s.push_str (">]\n");
    s
  }};  // end @fn_dotfile_transition: external

  //
  //  @fn_dotfile_transition: internal
  //
  ( @fn_dotfile_transition
    event $event:ident <$source:ident> $($action:block)*
  ) => {{ /* do not draw edge */ String::new() }};

  // TODO:
  /*
  //
  //  @fn_dotfile_transition: internal
  //
  ( @fn_dotfile_transition
    $event:ident <$source:ident>
      ($($param_name:ident : $param_type:ty $(= $param_default:expr)*),*)
      [ $($guard_expr:expr)* ] { $($action_expr:expr)* }

  ) => { /*do not draw edge*/ String::new() };
  */

  //
  //  @fn_dotfile_transition_initial
  //
  ( @fn_dotfile_transition_initial $initial:ident ) => {{
    let mut s = String::new();
    s.push_str (format!(
      "    INITIAL -> {:?}\n", StateId::$initial).as_str());
    s
  }};

  //
  //  @fn_dotfile_transition_terminal
  //
  ( @fn_dotfile_transition_terminal $terminal:ident ) => {{
    let mut s = String::new();
    s.push_str (format!(
      "    {:?} -> TERMINAL\n", StateId::$terminal).as_str());
    s
  }};

  //
  //  @fn_dotfile_end
  //
  ( @fn_dotfile_end ) => {{
    let mut s = String::new();
    s.push_str (
      "  }\n\
      }");
    s
  }};

  //
  //  base implementation rule
  //
  ( @base
    machine $machine:ident
      $(<$(
        $type_var:ident $(: { $($type_constraint:path),+ })*
      ),+>)*
    {
      STATES [
        $(state $state:ident (
          $($data_name:ident : $data_type:ty $(= $data_default:expr)*),*
        ))+
      ]
      EVENTS [
        $(event $event:ident <$source:ident> $(=> <$target:ident>)*
          $({ $($state_data:ident),* } => $action:block)*
        )+
      ]
      EXTENDED [
        $($ext_name:ident : $ext_type:ty $(= $ext_default:expr)*),*
      ]
      $(state_reference: $state_reference:ident)*
      initial_state: $initial:ident $({
        $(initial_action: $initial_action:block)*
      })*
      $(terminal_state: $terminal:ident $({
        $(terminate_failure: $terminate_failure:block)*
        $(terminate_success: $terminate_success:block)*
      })*)*
    }

  ) => {

    #[derive(Debug)]
    pub struct $machine $(<$($type_var),+>)* where
    $($(
      $type_var : std::fmt::Debug,
      $($($type_var : $type_constraint),+)*
    ),+)*
    {
      state          : State,
      extended_state : ExtendedState $(<$($type_var),+>)*
    }

    #[derive(Debug)]
    pub struct State {
      id   : StateId,
      data : StateData
    }

    #[derive(Debug)]
    pub struct ExtendedState $(<$($type_var),+>)* where
    $($(
      $type_var : std::fmt::Debug,
      $($($type_var : $type_constraint),+)*
    ),+)*
    {
      $(pub $ext_name : $ext_type),*
    }

    #[derive(Clone,Debug,PartialEq)]
    pub struct Event {
      id : EventId
    }

    #[derive(Clone,Debug,PartialEq)]
    pub enum StateId {
      $($state),+
    }

    #[derive(Debug)]
    pub enum StateData {
      $($state {
        $($data_name : $data_type),*
      }),+
    }

    #[derive(Debug,PartialEq)]
    pub enum Transition {
      Internal (StateId),
      External (StateId, StateId)
    }

    #[derive(Clone,Debug,PartialEq)]
    pub enum EventId {
      $($event),+
    }

    impl $(<$($type_var),+>)* $machine $(<$($type_var),+>)* where
    $($(
      $type_var : std::fmt::Debug,
      $($($type_var : $type_constraint),+)*
    ),+)*
    {
      pub fn report() where $($($type_var : 'static),+)* {
        let machine_name = stringify!($machine);
        let machine_type = unsafe { std::intrinsics::type_name::<Self>() };
        println!("{} report...", machine_name);
        println!("size of {}: {}", machine_type, std::mem::size_of::<Self>());
        println!("...{} report", machine_name);
      }

      pub fn new (extended_state : ExtendedState $(<$($type_var),+>)*) -> Self {
        Self {
          state: State::initial(),
          extended_state
        }
      }

      #[allow(dead_code)]
      #[inline]
      pub fn state (&self) -> &State {
        &self.state
      }

      #[allow(dead_code)]
      #[inline]
      pub fn extended_state (&self) -> &ExtendedState $(<$($type_var),+>)* {
        &self.extended_state
      }

      def_machine!{
        @impl_fn_handle_event
        machine $machine {
          STATES [
            $(state $state ($($data_name : $data_type $(= $data_default)*),*))+
          ]
          EVENTS [
            $(event $event <$source> $(=> <$target>)*
              $({$($state_data),*} => $action)*
            )+
          ]
          EXTENDED [
            $($ext_name : $ext_type $(= $ext_default)*),*
          ]
        }
      }

    } // end impl $machine

    impl $(<$($type_var),+>)* Drop for $machine $(<$($type_var),+>)* where
    $($(
      $type_var : std::fmt::Debug,
      $($($type_var : $type_constraint),+)*
    ),+)*
    {
      fn drop (&mut self) {
        trace!("{}::drop", stringify!($machine));
        let _state_id = self.state.id.clone();
        #[allow(unused_variables)]
        match &mut self.extended_state {
          &mut ExtendedState { $(ref mut $ext_name,)*.. } => {
            $(let $state_reference = &mut self.state;)*
            $(
            if _state_id != StateId::$terminal {
              trace!("<<< current state ({:?}) != terminal state ({:?})",
                _state_id, StateId::$terminal);
              $($($terminate_failure)*)*
            } else {
              $($($terminate_success)*)*
            }
            )*
          }
        }
      }
    }

    impl State {
      #[inline]
      pub fn initial() -> Self {
        StateId::initial().into()
      }

      #[inline]
      pub fn id (&self) -> &StateId {
        &self.id
      }

      #[inline]
      pub fn data (&self) -> &StateData {
        &self.data
      }
    }

    impl From <StateId> for State {
      fn from (id : StateId) -> Self {
        State {
          id:   id.clone(),
          data: id.into()
        }
      }
    }

    impl StateData {
      #[inline]
      pub fn initial() -> Self {
        StateId::initial().into()
      }

      $(
      #[inline]
      pub fn terminal() -> Self {
        // we use the metavariable here to take advantage of the
        // zero-or-one repetition
        StateId::$terminal.into()
      }
      )*
    }

    impl From <StateId> for StateData {
      fn from (id : StateId) -> Self {
        match id {
          $(StateId::$state => StateData::$state {
            $($data_name: def_machine!(@expr_default $($data_default)*)),*
          }),+
        }
      }
    }

    impl StateId {
      #[inline]
      pub const fn initial() -> Self {
        StateId::$initial
      }
      $(
      #[inline]
      pub const fn terminal() -> Self {
        StateId::$terminal
      }
      )*
    }

    impl EventId {
      pub fn transition (&self) -> Transition {
        match *self {
          $(
          EventId::$event =>
            def_machine!(@event_transition <$source> $(=> <$target>)*)
          ),+
        }
      }
    }

    impl Event {
      pub fn transition (&self) -> Transition {
        self.id.transition()
      }
    }

    impl From <EventId> for Event {
      fn from (id : EventId) -> Self {
        Event {
          id: id
        }
      }
    }

  };  // end @base

} // end def_machine!

/// State machine that requires runtime initialization.
#[macro_export]
macro_rules! def_machine_nodefault {
  //
  //  main implementation rule
  //
  ( machine $machine:ident
      $(<$(
        $type_var:ident $(: { $($type_constraint:path),+ })*
      ),+>)*
    {
      STATES [
        $(state $state:ident (
          $($data_name:ident : $data_type:ty $(= $data_default:expr)*),*
        ))+
      ]
      EVENTS [
        $(event $event:ident <$source:ident> $(=> <$target:ident>)*
          $({ $($state_data:ident),* } => $action:block)*
        )+
      ]
      EXTENDED [
        $($ext_name:ident : $ext_type:ty $(= $ext_default:expr)*),*
      ]
      $(state_reference: $state_reference:ident)*
      initial_state: $initial:ident $({
        $(initial_action: $initial_action:block)*
      })*
      $(terminal_state: $terminal:ident $({
        $(terminate_failure: $terminate_failure:block)*
        $(terminate_success: $terminate_success:block)*
      })*)*
    }

  ) => {

    def_machine!{
      @base
      machine $machine
        $(<$($type_var $(: { $($type_constraint),+ })*),+>)*
      {
        STATES [
          $(state $state ($($data_name : $data_type $(= $data_default)*),*))+
        ]
        EVENTS [
          $(event $event <$source> $(=> <$target>)*
            $({$($state_data),*} => $action)*
          )+
        ]
        EXTENDED [
          $($ext_name : $ext_type $(= $ext_default)*),*
        ]
        $(state_reference: $state_reference)*
        initial_state: $initial $({
          $(initial_action: $initial_action)*
        })*
        $(terminal_state: $terminal $({
          $(terminate_failure: $terminate_failure)*
          $(terminate_success: $terminate_success)*
        })*)*
      }
    }

    impl $(<$($type_var),+>)* $machine $(<$($type_var),+>)* where
    $($(
      $type_var : std::fmt::Debug,
      $($($type_var : $type_constraint),+)*
    ),+)*
    {
      def_machine!{
        @impl_fn_dotfile_nodefault
        machine $machine $(<$($type_var),+>)* {
          STATES    [
            $(state $state ($($data_name : $data_type = $($data_default)*),*))+
          ]
          EVENTS    [ $(event $event <$source> $(=> <$target>)* $($action)*)+ ]
          EVENTS_TT [ $(event $event <$source> $(=> <$target>)* $($action)*)+ ]
          EXTENDED      [ $($ext_name : $ext_type $(= $ext_default)*),* ]
          initial_state:  $initial
          $(terminal_state: $terminal $({
            $(terminate_failure: $terminate_failure)*
          })*)*
        }
      }
    }

    impl $(<$($type_var),+>)* ExtendedState $(<$($type_var),+>)* where
    $($(
      $type_var : std::fmt::Debug,
      $($($type_var : $type_constraint),+)*
    ),+)*
    {
      /// Creation method that allows overriding defaults. If a field does not
      /// have a default specified it is a required argument.
      // TODO: indicate which arguments are missing in case of failure
      // TODO: make required arguments non-Option types?
      pub fn new ($($ext_name : Option <$ext_type>),*) -> Option <Self> {
        Some (Self {
          $($ext_name: {
            if let Some ($ext_name) = $ext_name {
              $ext_name
            } else {
              if let Some (default) =
                def_machine!(@expr_option $($ext_default)*)
              {
                default
              } else {
                return None
              }
            }
          }),*
        })
      }
    }
  };

  //
  //  alternate syntax
  //
  ( $machine:ident
    $(<$(
      $type_var:ident $(: { $($type_constraint:path),+ })*
    ),+>)*
    $(($(
      $ext_name:ident : $ext_type:ty $(= $ext_default:expr)*
    ),*))*
      $(where let $state_reference:ident = &mut self.state)*
    {
      STATES [
        $(state $state:ident (
          $($data_name:ident : $data_type:ty $(= $data_default:expr)*),*
        ))+
      ]
      EVENTS [
        $(event $event:ident <$source:ident> $(=> <$target:ident>)*
          $({ $($state_data:ident),* } => $action:block)*
        )+
      ]
      initial_state: $initial:ident $({
        $(initial_action: $initial_action:block)*
      })*
      $(terminal_state: $terminal:ident $({
        $(terminate_failure: $terminate_failure:block)*
        $(terminate_success: $terminate_success:block)*
      })*)*
    }

  ) => {

    def_machine_nodefault!{
      machine $machine $(<$($type_var $(: { $($type_constraint),+ })*),+>)* {
        STATES [
          $(state $state ($($data_name : $data_type $(= $data_default)*),*))+
        ]
        EVENTS [
          $(event $event <$source> $(=> <$target>)*
            $({$($state_data),*} => $action)*
          )+
        ]
        EXTENDED [
          $($($ext_name : $ext_type $(= $ext_default)*),*)*
        ]
        $(state_reference: $state_reference)*
        initial_state: $initial $({
          $(initial_action: $initial_action)*
        })*
        $(terminal_state: $terminal $({
          $(terminate_failure: $terminate_failure)*
          $(terminate_success: $terminate_success)*
        })*)*
      }
    }

  };

} // end def_machine_nodefault!
