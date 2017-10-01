#[derive(Debug,PartialEq)]
pub enum HandleEventException {
  WrongState
}

/// State machines with a default `initial` state.
///
/// For each extended 'data' field, either the type must implement `Default`,
/// or else a default expression is provided following `=`.
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
        $(state $state:ident {})+
      ]
      EVENTS [
        $(event $event:ident <$source:ident> => <$target:ident>
          $($action:block)*
        )+
      ]
      DATA [
        $($data_name:ident : $data_type:ty $(= $data_default:expr)*),*
      ]
      $(self_reference: $self_reference:ident)*
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
          $(state $state {})+
        ]
        EVENTS [
          $(event $event <$source> => <$target> $($action)*)+
        ]
        DATA [
          $($data_name : $data_type $(= $data_default)*),*
        ]
        $(self_reference: $self_reference)*
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
          state: State::initial(),
          data:  Data::initial()
        };
        {
          $(let $self_reference = &mut _new;)*
          $($($initial_action)*)*
        }
        _new
      }
    }

    impl $(<$($type_var),+>)* Data $(<$($type_var),+>)* where
    $($(
      $type_var : std::fmt::Debug,
      $($($type_var : $type_constraint),+)*
    ),+)*
    {
      pub fn initial() -> Self {
        Self {
          $($data_name: def_machine!(@impl_expr_default $($data_default)*)),*
        }
      }

      /// Creation method that allows overriding defaults.
      pub fn new ($($data_name : Option <$data_type>),*) -> Self {
        Self {
          $($data_name: $data_name.unwrap_or (
            def_machine!(@impl_expr_default $($data_default)*))
          ),*
        }
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
      $data_name:ident : $data_type:ty $(= $data_default:expr)*
    ),*))*
    $(where self = $self_reference:ident)*
    {
      STATES [
        $(state $state:ident {})+
      ]
      EVENTS [
        $(event $event:ident <$source:ident> => <$target:ident>
          $($action:block)*
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
          $(state $state {})+
        ]
        EVENTS [
          $(event $event <$source> => <$target> $($action)*)+
        ]
        DATA [
          $($($data_name : $data_type $(= $data_default)*),*)*
        ]
        $(self_reference: $self_reference)*
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

  //
  //  @impl_expr_default: override default
  //
  ( @impl_expr_default $default:expr ) => { $default };

  //
  //  @impl_expr_default: use default
  //
  ( @impl_expr_default ) => { Default::default() };

  //
  //  @impl_expr_nodefault: override default
  //
  ( @impl_expr_nodefault $default:expr ) => { $default };

  //
  //  @impl_expr_nodefault: no default
  //
  ( @impl_expr_nodefault ) => { return None };

  //
  //  @impl_fn_dotfile
  //
  ( @impl_fn_dotfile
    machine $machine:ident $(<$($type_var:ident),+>)* {
      STATES [
        $(state $state:ident {})+
      ]
      EVENTS [
        $(event $event:ident <$source:ident> => <$target:ident>
          $($action:block)*
        )+
      ]
      EVENTS_TT $events_tt:tt
      DATA [
        $($data_name:ident : $data_type:ty $(= $data_default:expr)*),*
      ]
      $(self_reference: $self_reference:ident)*
      initial_state: $initial:ident $({
        $(initial_action: $initial_action:block)*
      })*
      $(terminal_state: $terminal:ident $({
        $(terminate_failure: $terminate_failure:block)*
        $(terminate_success: $terminate_success:block)*
      })*)*
    }

  ) => {

    pub fn dotfile() -> String {
      let mut s = String::new();
      // begin graph
      s.push_str (def_machine!(@fn_dotfile_begin).as_str());

      // begin subgraph
      s.push_str (def_machine!(
        @fn_dotfile_subgraph_begin
        machine $machine $(<$($type_var),+>)* {
          EVENTS [ $(event $event <$source> => <$target> $($action)*)+ ]
          DATA   [ $($data_name : $data_type $(= $data_default)*),* ]
        }
      ).as_str());

      // nodes
      s.push_str (def_machine!(@fn_dotfile_node_initial).as_str());
      $(
      s.push_str (def_machine!(
        @fn_dotfile_node
        state $state {}
        EVENTS $events_tt
      ).as_str());
      )+

      // transitions
      s.push_str (
        def_machine!(@fn_dotfile_transition_initial $initial
      ).as_str());
      $(
      s.push_str (def_machine!(
        @fn_dotfile_transition
        event $event <$source> => <$target> $($action)*
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
        $(event $event:ident <$source:ident> => <$target:ident>
          $($action:block)*
        )+
      ]
      DATA [
        $($data_name:ident : $data_type:ty $(= $data_default:expr)*),*
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
    s.push_str (format!("    label=<{}<FONT FACE=\"Mono\"><BR/><BR/>",
      title_string.escape().into_inner()).as_str());

    let mut _data_fields = std::vec::Vec::<String>::new();
    let mut _data_types = std::vec::Vec::<String>::new();

    $({
      _data_fields.push (stringify!($data_name).to_string());
      _data_types.push (stringify!($data_type).to_string());
    })*

    debug_assert_eq!(_data_fields.len(), _data_types.len());

    //
    //  for each data field, print a line
    //
    // TODO: we are manually aligning the columns of the field name and field
    // type, is there a better way ? (record node, html table, format width?)
    if !_data_types.is_empty() {
      s.push_str ("\n      ");

      let mut data_string = String::new();
      let separator = ",<BR ALIGN=\"LEFT\"/>\n      ";

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

      /*
      let longest_typename = data_types.iter().fold (0,
        |longest, ref typename| {
          let len = typename.len();
          if longest < len {
            len
          } else {
            longest
          }
        }
      );
      */

      for (i,f) in _data_fields.iter().enumerate() {
        use escapade::Escapable;

        let spacer1 : String = std::iter::repeat (' ').take(
          longest_fieldname - f.len()
        ).collect();

        data_string.push_str (
          format!("{}{} : {}",
            f, spacer1, _data_types[i])
          .escape().into_inner().as_str()
        );
        data_string.push_str (format!("{}", separator).as_str());
      }

      let len = data_string.len();
      data_string.truncate (len - separator.len());
      s.push_str (format!("{}", data_string).as_str());
    }

    // TODO
    /*
    // internal state transitions
    let mut _data_once = false;
    $({
      match EventId::$event.transition() {

        Transition::Internal => {
          if !_data_once {
            s.push_str (
              "<BR ALIGN=\"LEFT\"/></FONT>\
               <BR ALIGN=\"LEFT\"/>\
               <FONT FACE=\"Sans Italic\">");
            _data_once = true;
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
              @impl_expr_default $($param_default)*
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
    if !_data_types.is_empty() {
      s.push_str ("\n      ");
    }
    s.push_str ( "</FONT><BR/>>\n    \
      shape=record\n    \
      style=rounded\n    \
      fontname=\"Sans Bold Italic\"\n");
    s
  }}; // end @fn_dotfile_subgraph_begin

  //
  //  @fn_dotfile_node
  //
  ( @fn_dotfile_node
    state $state:ident {}
    EVENTS [
      $(event $event:ident <$source:ident> => <$target:ident>
        $($action:block)*
      )+
    ]

  ) => {{
    let mut s = String::new();

    s.push_str (format!(
      "    {:?} [label=<<B>{:?}</B>",
      StateId::$state, StateId::$state).as_str());

    // TODO: state variables
    /*
    let mut mono_font = false;
    let mut var_fields = std::vec::Vec::<String>::new();
    let mut var_types = std::vec::Vec::<String>::new();
    let mut var_defaults = std::vec::Vec::<String>::new();
    // the following 3 lines are to avoid unused_mut warnings
    var_fields.clear();
    var_types.clear();
    var_defaults.clear();

    $({
      if !mono_font {
        s.push_str ("|<FONT FACE=\"Mono\"><BR/>");
        mono_font = true;
      }
      var_fields.push (stringify!($var_name).to_string());
      var_types.push (stringify!($var_type).to_string());
      let default_val : $var_type = def_machine!{
        @impl_expr_default $($var_default)*
      };
      var_defaults.push (format!("{:?}", default_val));
    })*

    debug_assert_eq!(var_fields.len(), var_types.len());
    debug_assert_eq!(var_types.len(),  var_defaults.len());

    //
    //  for each data field, print a line
    //
    // TODO: we are manually aligning the columns of the field
    // name, field type, and default values, is there a better
    // way ? (record node, html table, format width?)
    if !var_types.is_empty() {
      debug_assert!(!var_defaults.is_empty());

      let mut var_string = String::new();
      let separator = ",<BR ALIGN=\"LEFT\"/>";

      let longest_fieldname = var_fields.iter().fold (0,
        |longest, ref fieldname| {
          let len = fieldname.len();
          if longest < len {
            len
          } else {
            longest
          }
        }
      );

      let longest_typename = var_types.iter().fold (0,
        |longest, ref typename| {
          let len = typename.len();
          if longest < len {
            len
          } else {
            longest
          }
        }
      );

      for (i,f) in var_fields.iter().enumerate() {
        use escapade::Escapable;

        let spacer1 : String = std::iter::repeat (' ').take(
          longest_fieldname - f.len()
        ).collect();
        let spacer2 : String = std::iter::repeat (' ').take(
          longest_typename - var_types[i].len()
        ).collect();

        var_string.push_str (
          format!("{}{} : {}{} = {}",
            f, spacer1, var_types[i], spacer2,
            var_defaults[i])
          .escape().into_inner().as_str()
        );
        var_string.push_str (format!("{}", separator).as_str());
      }

      let len = var_string.len();
      var_string.truncate (len - separator.len());
      s.push_str (format!("{}", var_string).as_str());
    }
    */

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
              @impl_expr_default $($param_default)*
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
    s.push_str (">]\n");
    s
  }};  // end @fn_dotfile_node

  //
  //  @fn_dotfile_node_initial
  //
  ( @fn_dotfile_node_initial ) => {{
    let mut s = String::new();
    s.push_str (
      "    INITIAL [label=\"\", shape=circle, width=0.2,\n      \
           style=filled, fillcolor=black]\n");
    s
  }};

  //
  //  @fn_dotfile_node_terminal
  //
  ( @fn_dotfile_node_terminal ) => {{
    let mut s = String::new();
    s.push_str (
      "    TERMINAL [label=\"\", shape=doublecircle, \
           width=0.2,\n      style=filled, fillcolor=black]\n");
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

    let mut mono_font = false;
    // TODO: params
    /*
    let mut open_params = false;
    $({
      use escapade::Escapable;
      if !open_params {
        if !mono_font {
          s.push_str ("<FONT FACE=\"Mono\">");
          mono_font = true;
        }
        s.push_str ("(".as_str());
        open_params = true;
      }
      let default_val : $param_type = def_machine!{
        @impl_expr_default $($param_default)*
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
      if !mono_font {
        s.push_str ("<FONT FACE=\"Mono\">");
        mono_font = true;
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
        if !mono_font {
          s.push_str ("<FONT FACE=\"Mono\"><BR/>");
          mono_font = true;
        }
        // TODO: different formatting if params or guards were present
        //action = "  ".to_string() + action.as_str();
        let action = action.escape().into_inner();
        s.push_str (format!("{}", action).as_str());
      }
    }
    )*

    if mono_font {
      s.push_str ("</FONT>");
    }
    s.push_str (">]\n");
    s
  }};  // end @fn_dotfile_transition: external

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
        $(state $state:ident {})+
      ]
      EVENTS [
        $(event $event:ident <$source:ident> => <$target:ident>
          $($action:block)*
        )+
      ]
      DATA [
        $($data_name:ident : $data_type:ty $(= $data_default:expr)*),*
      ]
      $(self_reference: $self_reference:ident)*
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
      state : State,
      data  : Data $(<$($type_var),+>)*
    }

    #[derive(Clone,Debug,PartialEq)]
    pub struct State {
      id : StateId
    }

    #[derive(Clone,Debug,PartialEq)]
    pub struct Event {
      id : EventId
    }

    #[derive(Debug)]
    pub struct Data $(<$($type_var),+>)* where
    $($(
      $type_var : std::fmt::Debug,
      $($($type_var : $type_constraint),+)*
    ),+)*
    {
      $(pub $data_name : $data_type),*
    }

    #[derive(Clone,Debug,PartialEq)]
    pub enum StateId {
      $($state),+
    }

    #[derive(Debug,PartialEq)]
    pub enum Transition {
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

      pub fn new (data : Data $(<$($type_var),+>)*) -> Self {
        Self {
          state: State::initial(),
          data
        }
      }

      #[allow(dead_code)]
      #[inline]
      pub fn state (&self) -> &State {
        &self.state
      }

      #[allow(dead_code)]
      #[inline]
      pub fn data (&self) -> &Data $(<$($type_var),+>)* {
        &self.data
      }

      pub fn handle_event (&mut self, event : Event)
        -> Result <(), macro_machines::HandleEventException>
      {
        trace!("{}::handle_event: {:?}", stringify!($machine), event);
        match event.transition() {
          Transition::External (source, target) => {
            if self.state.id == source {
              trace!("<<< Ok: {:?} => {:?}", source, target);
              self.state.id = target;
              {
                $(let $self_reference = self;)*
                match event.id {
                  $(EventId::$event => $($action)*)+
                }
              }
              Ok (())
            } else {
              trace!("<<< Err: current state ({:?}) != source state ({:?})",
                self.state.id, source);
              Err (macro_machines::HandleEventException::WrongState)
            }
          }
        }
      }

      def_machine!{
        @impl_fn_dotfile
        machine $machine $(<$($type_var),+>)* {
          STATES    [ $(state $state {})+ ]
          EVENTS    [ $(event $event <$source> => <$target> $($action)*)+ ]
          EVENTS_TT [ $(event $event <$source> => <$target> $($action)*)+ ]
          DATA      [ $($data_name : $data_type $(= $data_default)*),* ]
          initial_state:  $initial
          $(terminal_state: $terminal $({
            $(terminate_failure: $terminate_failure)*
          })*)*
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
        $(let $self_reference = self;)*
        $(
        if _state_id != StateId::$terminal {
          trace!("<<< current state ({:?}) != terminal state ({:?})",
            _state_id, StateId::$terminal);
          $($($terminate_failure)*)*
        } else {
          $($($terminate_success)*)*
        })*
      }
    }

    impl State {
      pub const fn initial() -> Self {
        State {
          id: StateId::initial()
        }
      }

      #[inline]
      pub fn id (&self) -> &StateId {
        &self.id
      }
    }

    impl StateId {
      pub const fn initial() -> Self {
        StateId::$initial
      }
      $(
      pub const fn terminal() -> Self {
        StateId::$terminal
      }
      )*
    }

    impl From <StateId> for State {
      fn from (id : StateId) -> Self {
        State {
          id: id
        }
      }
    }

    impl EventId {
      pub fn transition (&self) -> Transition {
        match self {
          $(
          &EventId::$event =>
            Transition::External (StateId::$source, StateId::$target)
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

  };
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
        $(state $state:ident {})+
      ]
      EVENTS [
        $(event $event:ident <$source:ident> => <$target:ident>
          $($action:block)*
        )+
      ]
      DATA [
        $($data_name:ident : $data_type:ty $(= $data_default:expr)*),*
      ]
      $(self_reference: $self_reference:ident)*
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
          $(state $state {})+
        ]
        EVENTS [
          $(event $event <$source> => <$target> $($action)*)+
        ]
        DATA [
          $($data_name : $data_type $(= $data_default)*),*
        ]
        $(self_reference: $self_reference)*
        initial_state: $initial $({
          $(initial_action: $initial_action)*
        })*
        $(terminal_state: $terminal $({
          $(terminate_failure: $terminate_failure)*
          $(terminate_success: $terminate_success)*
        })*)*
      }
    }

    impl $(<$($type_var),+>)* Data $(<$($type_var),+>)* where
    $($(
      $type_var : std::fmt::Debug,
      $($($type_var : $type_constraint),+)*
    ),+)*
    {
      /// Creation method that allows overriding defaults. If a field does not
      /// have a default specified it is a required argument.
      // TODO: indicate which arguments are missing in case of failure
      // TODO: make required arguments not Option types
      pub fn new ($($data_name : Option <$data_type>),*) -> Option <Self> {
        Some (Self {
          $($data_name: {
            if let Some ($data_name) = $data_name {
              $data_name
            } else {
              def_machine!(@impl_expr_nodefault $($data_default)*)
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
      $data_name:ident : $data_type:ty $(= $data_default:expr)*
    ),*))*
    $(where self = $self_reference:ident)*
    {
      STATES [
        $(state $state:ident {})+
      ]
      EVENTS [
        $(event $event:ident <$source:ident> => <$target:ident>
          $($action:block)*
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
          $(state $state {})+
        ]
        EVENTS [
          $(event $event <$source> => <$target> $($action)*)+
        ]
        DATA [
          $($($data_name : $data_type $(= $data_default)*),*)*
        ]
        $(self_reference: $self_reference)*
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
