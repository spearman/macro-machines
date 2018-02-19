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
        $(event $event:ident <$source:tt> $(=> <$target:ident>)*
          $({ $($state_data:ident),* } => $action:block)*
        )+
      ]
      EXTENDED [
        $($ext_name:ident : $ext_type:ty $(= $ext_default:expr)*),*
      ]
      $(self_reference: $self_reference:ident)*
      initial_state: $initial:ident $({
        $(initial_action: $initial_action:block)*
      })*
      $(terminal_state: $terminal:ident $({
        $(terminate_success: $terminate_success:block)*
        $(terminate_failure: $terminate_failure:block)*
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
        $(self_reference: $self_reference)*
        initial_state: $initial $({
          $(initial_action: $initial_action)*
        })*
        $(terminal_state: $terminal $({
          $(terminate_success: $terminate_success)*
          $(terminate_failure: $terminate_failure)*
        })*)*
      }
    }

    impl $(<$($type_var),+>)* $machine $(<$($type_var),+>)* where
    $($(
      $($($type_var : $type_constraint),+)*
    ),+)*
    {
      pub fn initial() -> Self {
        trace!("{}::initial", stringify!($machine));
        let mut _new = Self {
          state:          State::initial(),
          extended_state: ExtendedState::initial()
        };
        $(let $self_reference = _new;)*
        $($($initial_action)*)*
        $(_new = $self_reference;)*
        _new
      }
    }

    impl $(<$($type_var),+>)* $crate::MachineDotfile
      for $machine $(<$($type_var),+>)*
    where
    $($(
      $($($type_var : $type_constraint),+)*
    ),+)*
    {
      fn name() -> &'static str {
        stringify!($machine)
      }
      fn type_vars() -> Vec <String> {
        let mut _v = Vec::new();
        $($(
        _v.push (format!(
          "{} = {}", stringify!($type_var),
            unsafe { ::std::intrinsics::type_name::<$type_var>() }));
        )+)*
        _v
      }
      fn extended_state_names() -> Vec <&'static str> {
        let mut _v = Vec::new();
        $(
        _v.push (stringify!($ext_name));
        )*
        _v
      }
      fn extended_state_types() -> Vec <&'static str> {
        let mut _v = Vec::new();
        $(
        _v.push (stringify!($ext_type));
        )*
        _v
      }
      fn extended_state_defaults() -> Vec <&'static str> {
        let mut _v = Vec::new();
        $(
        _v.push ({
          let default_expr = stringify!($($ext_default)*);
          if !default_expr.is_empty() {
            default_expr
          } else {
            "Default::default()"  // TODO: make this $ext_type::default() ?
          }
        });
        )*
        _v
      }
      fn states() -> Vec <&'static str> {
        let mut v = Vec::new();
        $(
        v.push (stringify!($state));
        )+
        v
      }
      fn state_initial() -> &'static str {
        stringify!($initial)
      }
      fn state_terminal() -> &'static str {
        stringify!($($terminal)*)
      }
      fn state_data_names() -> Vec <Vec <&'static str>> {
        let mut v = Vec::new();
        $({
          let mut _w = Vec::new();
          $(_w.push (stringify!($data_name));)*
          v.push (_w);
        })+
        v
      }
      fn state_data_types() -> Vec <Vec <&'static str>> {
        let mut v = Vec::new();
        $({
          let mut _w = Vec::new();
          $(_w.push (stringify!($data_type));)*
          v.push (_w);
        })+
        v
      }
      fn state_data_defaults() -> Vec <Vec <&'static str>> {
        let mut v = Vec::new();
        $({
          let mut _w = Vec::new();
          $(
          _w.push ({
            let default_expr = stringify!($($data_default)*);
            if !default_expr.is_empty() {
              default_expr
            } else {
              "Default::default()"  // TODO: make this $data_type::default() ?
            }
          });
          )*
          v.push (_w);
        })+
        v
      }
      /// &#9888; This function creates default values for each state data field
      /// and creates a pretty printed string of the value
      fn state_data_pretty_defaults() -> Vec <Vec <String>> {
        let mut v = Vec::new();
        $({
          let mut _w = Vec::new();
          $(
          let default_val : $data_type
            = def_machine!(@expr_default $($data_default)*);
          _w.push (format!("{:#?}", default_val));
          )*
          v.push (_w);
        })+
        v
      }
      fn events() -> Vec <&'static str> {
        let mut v = Vec::new();
        $(
        v.push (stringify!($event));
        )+
        v
      }
      fn event_sources() -> Vec <&'static str> {
        let mut v = Vec::new();
        $(
        v.push (stringify!($source));
        )+
        v
      }
      fn event_targets() -> Vec <&'static str> {
        let mut v = Vec::new();
        $(
        v.push (stringify!($($target)*));
        )+
        v
      }
      fn event_actions() -> Vec <&'static str> {
        let mut v = Vec::new();
        $(
        v.push (stringify!($($action)*));
        )+
        v
      }
    } // end impl MachineDotfile

    impl $(<$($type_var),+>)* ExtendedState $(<$($type_var),+>)* where
    $($(
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
      $(where let $self_reference:ident = self)*
    {
      STATES [
        $(state $state:ident (
          $($data_name:ident : $data_type:ty $(= $data_default:expr)*),*
        ))+
      ]
      EVENTS [
        $(event $event:ident <$source:tt> $(=> <$target:ident>)*
          $({ $($state_data:ident),* } => $action:block)*
        )+
      ]
      initial_state: $initial:ident $({
        $(initial_action: $initial_action:block)*
      })*
      $(terminal_state: $terminal:ident $({
        $(terminate_success: $terminate_success:block)*
        $(terminate_failure: $terminate_failure:block)*
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
        $(self_reference: $self_reference)*
        initial_state: $initial $({
          $(initial_action: $initial_action)*
        })*
        $(terminal_state: $terminal $({
          $(terminate_success: $terminate_success)*
          $(terminate_failure: $terminate_failure)*
        })*)*
      }
    }

  };
  //  end alternate syntax

  //
  //  @impl_fn_handle_event
  //
  ( @impl_fn_handle_event
    machine $machine:ident $(where let $self_reference:ident = self)* {
      STATES [
        $(state $state:ident (
          $($data_name:ident : $data_type:ty $(= $data_default:expr)*),*
        ))+
      ]
      EVENTS [
        $(event $event:ident <$source:tt> $(=> <$target:ident>)*
          $({ $($state_data:ident),* } => $action:block)*
        )+
      ]
      EXTENDED [
        $($ext_name:ident : $ext_type:ty $(= $ext_default:expr)*),*
      ]
    }

  ) => {

    pub fn handle_event (&mut self, event : Event)
      -> Result <(), $crate::HandleEventException>
    {
      trace!("{}::handle_event: {:?}", stringify!($machine), event.id);
      // if only one kind of transition exists the following match expression
      // will detect the other branch as "unreachable_code"
      #[allow(unreachable_code)]
      match event.transition() {

        Transition::Universal (target_id) => {
          trace!("<<< Ok: {:?} => {:?}", self.state.id, target_id);
          {
            $(let $self_reference = &mut*self;)*
            match event.id {
              $(EventId::$event => {
                // only expands universal actions, unreachable otherwise
                def_machine!{
                  @event_action_universal
                  event $event <$source> $(=> <$target>)* $($action)*
                }
              })+
            }
          }
          self.state = target_id.into();
          Ok (())
        }

        Transition::Internal (source_id) => {
          if self.state.id == source_id {
            // bring extended state variables into scope
            #[allow(unused_variables)]
            match &mut self.extended_state {
              &mut ExtendedState { $(ref mut $ext_name,)*.. } => {
                // map each event to an action
                match event.id {
                  $(EventId::$event => {
                    // for universal transitions there is no source state so
                    // this produces a wildcard pattern resulting in the
                    // last branch being unreachable
                    // bring local state variables into scope
                    #[allow(unreachable_patterns)]
                    match &mut self.state.data {
                      def_machine!{
                        @event_internal_state_pattern
                        $source { $($($state_data),*)* }
                      } => {
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
                self.state.id, source_id);
            Err ($crate::HandleEventException::WrongState)
          }
        }

        Transition::External (source_id, target_id) => {
          if self.state.id == source_id {
            trace!("<<< Ok: {:?} => {:?}", source_id, target_id);
            {
              $(let $self_reference = &mut*self;)*
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
            self.state = target_id.into();
            Ok (())
          } else {
            trace!("<<< Err: external transition: \
              current state ({:?}) != source state ({:?})",
                self.state.id, source_id);
            Err ($crate::HandleEventException::WrongState)
          }
        }

      } // end match transition
    } // end fn handle_event

  };  // end @impl_fn_handle_event

  //
  //  @event_internal_state_pattern
  //
  ( @event_internal_state_pattern
    $source:ident { $($state_data:ident),* }
  ) => {
    &mut StateData::$source {$(ref mut $state_data,)*..}
  };

  //
  //  @event_internal_state_pattern: not an internal event
  //
  ( @event_internal_state_pattern
    * { $($state_data:ident),* }
  ) => {
    _
  };

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
  //  @event_action_external: not an external event
  //
  ( @event_action_external
    event $event:ident <*> => <$target:ident> $($action:block)*
  ) => { unreachable!("not an external event") };

  //
  //  @event_action_universal
  //
  ( @event_action_universal
    event $event:ident <*> => <$target:ident> $($action:block)*
  ) => {
    $($action)*
  };

  //
  //  @event_action_universal: not an universal event
  //
  ( @event_action_universal
    event $event:ident <$source:ident> $(=> <$target:ident>)* $($action:block)*
  ) => { unreachable!("not an universal event") };

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
    event $event:ident <$source:tt> => <$target:ident> $($action:block)*
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
  //  @event_transition: universal
  //
  ( @event_transition <*> => <$target:ident> ) => {
    Transition::Universal (StateId::$target)
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
  //  @base implementation rule
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
        $(event $event:ident <$source:tt> $(=> <$target:ident>)*
          $({ $($state_data:ident),* } => $action:block)*
        )+
      ]
      EXTENDED [
        $($ext_name:ident : $ext_type:ty $(= $ext_default:expr)*),*
      ]
      $(self_reference: $self_reference:ident)*
      initial_state: $initial:ident $({
        $(initial_action: $initial_action:block)*
      })*
      $(terminal_state: $terminal:ident $({
        $(terminate_success: $terminate_success:block)*
        $(terminate_failure: $terminate_failure:block)*
      })*)*
    }

  ) => {

    pub struct $machine $(<$($type_var),+>)* where
    $($(
      $($($type_var : $type_constraint),+)*
    ),+)*
    {
      state          : State,
      extended_state : ExtendedState $(<$($type_var),+>)*
    }

    pub struct State {
      id   : StateId,
      data : StateData
    }

    pub struct ExtendedState $(<$($type_var),+>)* where
    $($(
      $($($type_var : $type_constraint),+)*
    ),+)*
    {
      $(pub $ext_name : $ext_type),*
    }

    #[derive(Clone,PartialEq)]
    pub struct Event {
      id : EventId
    }

    #[derive(Clone,Debug,PartialEq)]
    pub enum StateId {
      $($state),+
    }

    pub enum StateData {
      $($state {
        $($data_name : $data_type),*
      }),+
    }

    #[derive(Debug,PartialEq)]
    pub enum Transition {
      Internal  (StateId),
      External  (StateId, StateId),
      Universal (StateId)
    }

    #[derive(Clone,Debug,PartialEq)]
    pub enum EventId {
      $($event),+
    }

    impl $(<$($type_var),+>)* $machine $(<$($type_var),+>)* where
    $($(
      $($($type_var : $type_constraint),+)*
    ),+)*
    {
      pub fn report() where $($($type_var : 'static),+)* {
        let machine_name = stringify!($machine);
        let machine_type = unsafe { ::std::intrinsics::type_name::<Self>() };
        println!("{} report...", machine_name);
        println!("  size of {}: {}", machine_type, ::std::mem::size_of::<Self>());
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

      #[allow(dead_code)]
      #[inline]
      pub fn extended_state_mut (&mut self)
        -> &mut ExtendedState $(<$($type_var),+>)*
      {
        &mut self.extended_state
      }

      def_machine!{
        @impl_fn_handle_event
        machine $machine $(where let $self_reference = self)* {
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

    impl $(<$($type_var),+>)* AsRef <ExtendedState $(<$($type_var),+>)*>
      for $machine $(<$($type_var),+>)*
    where
    $($(
      $($($type_var : $type_constraint),+)*
    ),+)*
    {
      #[inline]
      fn as_ref (&self) -> &ExtendedState $(<$($type_var),+>)* {
        &self.extended_state
      }
    }

    impl $(<$($type_var),+>)* AsMut <ExtendedState $(<$($type_var),+>)*>
      for $machine $(<$($type_var),+>)*
    where
    $($(
      $($($type_var : $type_constraint),+)*
    ),+)*
    {
      #[inline]
      fn as_mut (&mut self) -> &mut ExtendedState $(<$($type_var),+>)* {
        &mut self.extended_state
      }
    }

    impl $(<$($type_var),+>)* Drop for $machine $(<$($type_var),+>)* where
    $($(
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
        }
        )*
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
        $(event $event:ident <$source:tt> $(=> <$target:ident>)*
          $({ $($state_data:ident),* } => $action:block)*
        )+
      ]
      EXTENDED [
        $($ext_name:ident : $ext_type:ty $(= $ext_default:expr)*),*
      ]
      $(self_reference: $self_reference:ident)*
      initial_state: $initial:ident $({
        $(initial_action: $initial_action:block)*
      })*
      $(terminal_state: $terminal:ident $({
        $(terminate_success: $terminate_success:block)*
        $(terminate_failure: $terminate_failure:block)*
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
        $(self_reference: $self_reference)*
        initial_state: $initial $({
          $(initial_action: $initial_action)*
        })*
        $(terminal_state: $terminal $({
          $(terminate_success: $terminate_success)*
          $(terminate_failure: $terminate_failure)*
        })*)*
      }
    }

    impl $(<$($type_var),+>)* $crate::MachineDotfile
      for $machine $(<$($type_var),+>)*
    where
    $($(
      $($($type_var : $type_constraint),+)*
    ),+)*
    {
      fn name() -> &'static str {
        stringify!($machine)
      }
      fn type_vars() -> Vec <String> {
        let mut _v = Vec::new();
        $($(
        _v.push (format!(
          "{} = {}", stringify!($type_var),
            unsafe { ::std::intrinsics::type_name::<$type_var>() }));
        )+)*
        _v
      }
      fn extended_state_names() -> Vec <&'static str> {
        let mut _v = Vec::new();
        $(
        _v.push (stringify!($ext_name));
        )*
        _v
      }
      fn extended_state_types() -> Vec <&'static str> {
        let mut _v = Vec::new();
        $(
        _v.push (stringify!($ext_type));
        )*
        _v
      }
      fn extended_state_defaults() -> Vec <&'static str> {
        let mut _v = Vec::new();
        $(
        _v.push (stringify!($($ext_default)*));
        )*
        _v
      }
      fn states() -> Vec <&'static str> {
        let mut v = Vec::new();
        $(
        v.push (stringify!($state));
        )+
        v
      }
      fn state_initial() -> &'static str {
        stringify!($initial)
      }
      fn state_terminal() -> &'static str {
        stringify!($($terminal)*)
      }
      fn state_data_names() -> Vec <Vec <&'static str>> {
        let mut v = Vec::new();
        $({
          let mut _w = Vec::new();
          $(_w.push (stringify!($data_name));)*
          v.push (_w);
        })+
        v
      }
      fn state_data_types() -> Vec <Vec <&'static str>> {
        let mut v = Vec::new();
        $({
          let mut _w = Vec::new();
          $(_w.push (stringify!($data_type));)*
          v.push (_w);
        })+
        v
      }
      fn state_data_defaults() -> Vec <Vec <&'static str>> {
        let mut v = Vec::new();
        $({
          let mut _w = Vec::new();
          $(_w.push (stringify!($($data_default)*));)*
          v.push (_w);
        })+
        v
      }
      /// &#9888; This function creates default values for each state data field
      /// and creates a pretty printed string of the value
      fn state_data_pretty_defaults() -> Vec <Vec <String>> {
        let mut v = Vec::new();
        $({
          let mut _w = Vec::new();
          $(
          let default_val : $data_type
            = def_machine!(@expr_default $($data_default)*);
          _w.push (format!("{:#?}", default_val));
          )*
          v.push (_w);
        })+
        v
      }
      fn events() -> Vec <&'static str> {
        let mut v = Vec::new();
        $(
        v.push (stringify!($event));
        )+
        v
      }
      fn event_sources() -> Vec <&'static str> {
        let mut v = Vec::new();
        $(
        v.push (stringify!($source));
        )+
        v
      }
      fn event_targets() -> Vec <&'static str> {
        let mut v = Vec::new();
        $(
        v.push (stringify!($($target)*));
        )+
        v
      }
      fn event_actions() -> Vec <&'static str> {
        let mut v = Vec::new();
        $(
        v.push (stringify!($($action)*));
        )+
        v
      }
    } // end impl MachineDotfile

    impl $(<$($type_var),+>)* ExtendedState $(<$($type_var),+>)* where
    $($(
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
      $(where let $self_reference:ident = self)*
    {
      STATES [
        $(state $state:ident (
          $($data_name:ident : $data_type:ty $(= $data_default:expr)*),*
        ))+
      ]
      EVENTS [
        $(event $event:ident <$source:tt> $(=> <$target:ident>)*
          $({ $($state_data:ident),* } => $action:block)*
        )+
      ]
      initial_state: $initial:ident $({
        $(initial_action: $initial_action:block)*
      })*
      $(terminal_state: $terminal:ident $({
        $(terminate_success: $terminate_success:block)*
        $(terminate_failure: $terminate_failure:block)*
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
        $(self_reference: $self_reference)*
        initial_state: $initial $({
          $(initial_action: $initial_action)*
        })*
        $(terminal_state: $terminal $({
          $(terminate_success: $terminate_success)*
          $(terminate_failure: $terminate_failure)*
        })*)*
      }
    }

  };

} // end def_machine_nodefault!

/// State machines with a default `initial` state and deriving `Debug`.
///
/// For each extended state field, either the type must implement `Default`, or
/// else a default expression is provided following `=`.
///
/// For a state machine that requires runtime initialization, see
/// `def_machine_nodefault_debug!`.

#[macro_export]
macro_rules! def_machine_debug {
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
        $(event $event:ident <$source:tt> $(=> <$target:ident>)*
          $({ $($state_data:ident),* } => $action:block)*
        )+
      ]
      EXTENDED [
        $($ext_name:ident : $ext_type:ty $(= $ext_default:expr)*),*
      ]
      $(self_reference: $self_reference:ident)*
      initial_state: $initial:ident $({
        $(initial_action: $initial_action:block)*
      })*
      $(terminal_state: $terminal:ident $({
        $(terminate_success: $terminate_success:block)*
        $(terminate_failure: $terminate_failure:block)*
      })*)*
    }

  ) => {

    def_machine_debug!{
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
        $(self_reference: $self_reference)*
        initial_state: $initial $({
          $(initial_action: $initial_action)*
        })*
        $(terminal_state: $terminal $({
          $(terminate_success: $terminate_success)*
          $(terminate_failure: $terminate_failure)*
        })*)*
      }
    }

    impl $(<$($type_var),+>)* $machine $(<$($type_var),+>)* where
    $($(
      $type_var : ::std::fmt::Debug,
      $($($type_var : $type_constraint),+)*
    ),+)*
    {
      pub fn initial() -> Self {
        trace!("{}::initial", stringify!($machine));
        let mut _new = Self {
          state:          State::initial(),
          extended_state: ExtendedState::initial()
        };
        $(let $self_reference = _new;)*
        $($($initial_action)*)*
        $(_new = $self_reference;)*
        _new
      }
    }

    impl $(<$($type_var),+>)* $crate::MachineDotfile
      for $machine $(<$($type_var),+>)*
    where
    $($(
      $type_var : ::std::fmt::Debug,
      $($($type_var : $type_constraint),+)*
    ),+)*
    {
      fn name() -> &'static str {
        stringify!($machine)
      }
      fn type_vars() -> Vec <String> {
        let mut _v = Vec::new();
        $($(
        _v.push (format!(
          "{} = {}", stringify!($type_var),
            unsafe { ::std::intrinsics::type_name::<$type_var>() }));
        )+)*
        _v
      }
      fn extended_state_names() -> Vec <&'static str> {
        let mut _v = Vec::new();
        $(
        _v.push (stringify!($ext_name));
        )*
        _v
      }
      fn extended_state_types() -> Vec <&'static str> {
        let mut _v = Vec::new();
        $(
        _v.push (stringify!($ext_type));
        )*
        _v
      }
      fn extended_state_defaults() -> Vec <&'static str> {
        let mut _v = Vec::new();
        $(
        _v.push ({
          let default_expr = stringify!($($ext_default)*);
          if !default_expr.is_empty() {
            default_expr
          } else {
            "Default::default()"  // TODO: make this $ext_type::default() ?
          }
        });
        )*
        _v
      }
      fn states() -> Vec <&'static str> {
        let mut v = Vec::new();
        $(
        v.push (stringify!($state));
        )+
        v
      }
      fn state_initial() -> &'static str {
        stringify!($initial)
      }
      fn state_terminal() -> &'static str {
        stringify!($($terminal)*)
      }
      fn state_data_names() -> Vec <Vec <&'static str>> {
        let mut v = Vec::new();
        $({
          let mut _w = Vec::new();
          $(_w.push (stringify!($data_name));)*
          v.push (_w);
        })+
        v
      }
      fn state_data_types() -> Vec <Vec <&'static str>> {
        let mut v = Vec::new();
        $({
          let mut _w = Vec::new();
          $(_w.push (stringify!($data_type));)*
          v.push (_w);
        })+
        v
      }
      fn state_data_defaults() -> Vec <Vec <&'static str>> {
        let mut v = Vec::new();
        $({
          let mut _w = Vec::new();
          $(
          _w.push ({
            let default_expr = stringify!($($data_default)*);
            if !default_expr.is_empty() {
              default_expr
            } else {
              "Default::default()"  // TODO: make this $data_type::default() ?
            }
          });
          )*
          v.push (_w);
        })+
        v
      }
      /// &#9888; This function creates default values for each state data field
      /// and creates a pretty printed string of the value
      fn state_data_pretty_defaults() -> Vec <Vec <String>> {
        let mut v = Vec::new();
        $({
          let mut _w = Vec::new();
          $(
          let default_val : $data_type
            = def_machine_debug!(@expr_default $($data_default)*);
          _w.push (format!("{:#?}", default_val));
          )*
          v.push (_w);
        })+
        v
      }
      fn events() -> Vec <&'static str> {
        let mut v = Vec::new();
        $(
        v.push (stringify!($event));
        )+
        v
      }
      fn event_sources() -> Vec <&'static str> {
        let mut v = Vec::new();
        $(
        v.push (stringify!($source));
        )+
        v
      }
      fn event_targets() -> Vec <&'static str> {
        let mut v = Vec::new();
        $(
        v.push (stringify!($($target)*));
        )+
        v
      }
      fn event_actions() -> Vec <&'static str> {
        let mut v = Vec::new();
        $(
        v.push (stringify!($($action)*));
        )+
        v
      }
    } // end impl MachineDotfile

    impl $(<$($type_var),+>)* ExtendedState $(<$($type_var),+>)* where
    $($(
      $type_var : ::std::fmt::Debug,
      $($($type_var : $type_constraint),+)*
    ),+)*
    {
      pub fn initial() -> Self {
        Self {
          $($ext_name: def_machine_debug!(@expr_default $($ext_default)*)),*
        }
      }

      /// Creation method that allows overriding defaults.
      pub fn new ($($ext_name : Option <$ext_type>),*) -> Self {
        Self {
          $($ext_name: $ext_name.unwrap_or (
            def_machine_debug!(@expr_default $($ext_default)*))
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
      $(where let $self_reference:ident = self)*
    {
      STATES [
        $(state $state:ident (
          $($data_name:ident : $data_type:ty $(= $data_default:expr)*),*
        ))+
      ]
      EVENTS [
        $(event $event:ident <$source:tt> $(=> <$target:ident>)*
          $({ $($state_data:ident),* } => $action:block)*
        )+
      ]
      initial_state: $initial:ident $({
        $(initial_action: $initial_action:block)*
      })*
      $(terminal_state: $terminal:ident $({
        $(terminate_success: $terminate_success:block)*
        $(terminate_failure: $terminate_failure:block)*
      })*)*
    }

  ) => {

    def_machine_debug!{
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
        $(self_reference: $self_reference)*
        initial_state: $initial $({
          $(initial_action: $initial_action)*
        })*
        $(terminal_state: $terminal $({
          $(terminate_success: $terminate_success)*
          $(terminate_failure: $terminate_failure)*
        })*)*
      }
    }

  };
  //  end alternate syntax

  //
  //  @impl_fn_handle_event
  //
  ( @impl_fn_handle_event
    machine $machine:ident $(where let $self_reference:ident = self)* {
      STATES [
        $(state $state:ident (
          $($data_name:ident : $data_type:ty $(= $data_default:expr)*),*
        ))+
      ]
      EVENTS [
        $(event $event:ident <$source:tt> $(=> <$target:ident>)*
          $({ $($state_data:ident),* } => $action:block)*
        )+
      ]
      EXTENDED [
        $($ext_name:ident : $ext_type:ty $(= $ext_default:expr)*),*
      ]
    }

  ) => {

    pub fn handle_event (&mut self, event : Event)
      -> Result <(), $crate::HandleEventException>
    {
      trace!("{}::handle_event: {:?}", stringify!($machine), event);
      // if only one kind of transition exists the following match expression
      // will detect the other branch as "unreachable_code"
      #[allow(unreachable_code)]
      match event.transition() {

        Transition::Universal (target_id) => {
          trace!("<<< Ok: {:?} => {:?}", self.state.id, target_id);
          {
            $(let $self_reference = &mut*self;)*
            match event.id {
              $(EventId::$event => {
                // only expands universal actions, unreachable otherwise
                def_machine_debug!{
                  @event_action_universal
                  event $event <$source> $(=> <$target>)* $($action)*
                }
              })+
            }
          }
          self.state = target_id.into();
          Ok (())
        }

        Transition::Internal (state_id) => {
          if self.state.id == state_id {
            // bring extended state variables into scope
            #[allow(unused_variables)]
            match &mut self.extended_state {
              &mut ExtendedState { $(ref mut $ext_name,)*.. } => {
                // map each event to an action
                match event.id {
                  $(EventId::$event => {
                    // for universal transitions there is no source state so
                    // this produces a wildcard pattern resulting in the
                    // last branch being unreachable
                    // bring local state variables into scope
                    #[allow(unreachable_patterns)]
                    match &mut self.state.data {
                      def_machine_debug!{
                        @event_internal_state_pattern
                        $source { $($($state_data),*)* }
                      } => {
                        // only expands internal actions, unreachable otherwise
                        def_machine_debug!{
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
            Err ($crate::HandleEventException::WrongState)
          }
        }

        Transition::External (source_id, target_id) => {
          if self.state.id == source_id {
            trace!("<<< Ok: {:?} => {:?}", source_id, target_id);
            {
              $(let $self_reference = &mut*self;)*
              match event.id {
                $(EventId::$event => {
                  // only expands external actions, unreachable otherwise
                  def_machine_debug!{
                    @event_action_external
                    event $event <$source> $(=> <$target>)* $($action)*
                  }
                })+
              }
            }
            self.state = target_id.into();
            Ok (())
          } else {
            trace!("<<< Err: external transition: \
              current state ({:?}) != source state ({:?})",
                self.state.id, source_id);
            Err ($crate::HandleEventException::WrongState)
          }
        }

      }
    }

  };  // end @impl_fn_handle_event

  //
  //  @event_internal_state_pattern
  //
  ( @event_internal_state_pattern
    $source:ident { $($state_data:ident),* }
  ) => {
    &mut StateData::$source {$(ref mut $state_data,)*..}
  };

  //
  //  @event_internal_state_pattern: not an internal event
  //
  ( @event_internal_state_pattern
    * { $($state_data:ident),* }
  ) => {
    _
  };

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
    event $event:ident <$source:tt> $($action:block)*
  ) => { unreachable!("not an external event") };

  //
  //  @event_action_external: not an external event
  //
  ( @event_action_external
    event $event:ident <*> => <$target:ident> $($action:block)*
  ) => { unreachable!("not an external event") };

  //
  //  @event_action_universal
  //
  ( @event_action_universal
    event $event:ident <*> => <$target:ident> $($action:block)*
  ) => {
    $($action)*
  };

  //
  //  @event_action_universal: not an universal event
  //
  ( @event_action_universal
    event $event:ident <$source:ident> $(=> <$target:ident>)* $($action:block)*
  ) => { unreachable!("not an universal event") };

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
    event $event:ident <$source:tt> => <$target:ident> $($action:block)*
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
  //  @event_transition: universal
  //
  ( @event_transition <*> => <$target:ident> ) => {
    Transition::Universal (StateId::$target)
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
  //  @base implementation rule
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
        $(event $event:ident <$source:tt> $(=> <$target:ident>)*
          $({ $($state_data:ident),* } => $action:block)*
        )+
      ]
      EXTENDED [
        $($ext_name:ident : $ext_type:ty $(= $ext_default:expr)*),*
      ]
      $(self_reference: $self_reference:ident)*
      initial_state: $initial:ident $({
        $(initial_action: $initial_action:block)*
      })*
      $(terminal_state: $terminal:ident $({
        $(terminate_success: $terminate_success:block)*
        $(terminate_failure: $terminate_failure:block)*
      })*)*
    }

  ) => {

    #[derive(Debug)]
    pub struct $machine $(<$($type_var),+>)* where
    $($(
      $type_var : ::std::fmt::Debug,
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
      $type_var : ::std::fmt::Debug,
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
      Internal  (StateId),
      External  (StateId, StateId),
      Universal (StateId)
    }

    #[derive(Clone,Debug,PartialEq)]
    pub enum EventId {
      $($event),+
    }

    impl $(<$($type_var),+>)* $machine $(<$($type_var),+>)* where
    $($(
      $type_var : ::std::fmt::Debug,
      $($($type_var : $type_constraint),+)*
    ),+)*
    {
      pub fn report() where $($($type_var : 'static),+)* {
        let machine_name = stringify!($machine);
        let machine_type = unsafe { ::std::intrinsics::type_name::<Self>() };
        println!("{} report...", machine_name);
        println!("  size of {}: {}", machine_type, ::std::mem::size_of::<Self>());
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

      def_machine_debug!{
        @impl_fn_handle_event
        machine $machine $(where let $self_reference = self)* {
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

    impl $(<$($type_var),+>)* AsRef <ExtendedState $(<$($type_var),+>)*>
      for $machine $(<$($type_var),+>)*
    where
    $($(
      $type_var : ::std::fmt::Debug,
      $($($type_var : $type_constraint),+)*
    ),+)*
    {
      #[inline]
      fn as_ref (&self) -> &ExtendedState $(<$($type_var),+>)* {
        &self.extended_state
      }
    }

    impl $(<$($type_var),+>)* AsMut <ExtendedState $(<$($type_var),+>)*>
      for $machine $(<$($type_var),+>)*
    where
    $($(
      $type_var : ::std::fmt::Debug,
      $($($type_var : $type_constraint),+)*
    ),+)*
    {
      #[inline]
      fn as_mut (&mut self) -> &mut ExtendedState $(<$($type_var),+>)* {
        &mut self.extended_state
      }
    }

    impl $(<$($type_var),+>)* Drop for $machine $(<$($type_var),+>)* where
    $($(
      $type_var : ::std::fmt::Debug,
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
        }
        )*
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
            $($data_name: def_machine_debug!(@expr_default $($data_default)*)),*
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
            def_machine_debug!(@event_transition <$source> $(=> <$target>)*)
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

} // end def_machine_debug!

/// State machine that requires runtime initialization and deriving `Debug`.
#[macro_export]
macro_rules! def_machine_nodefault_debug {
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
        $(event $event:ident <$source:tt> $(=> <$target:ident>)*
          $({ $($state_data:ident),* } => $action:block)*
        )+
      ]
      EXTENDED [
        $($ext_name:ident : $ext_type:ty $(= $ext_default:expr)*),*
      ]
      $(self_reference: $self_reference:ident)*
      initial_state: $initial:ident $({
        $(initial_action: $initial_action:block)*
      })*
      $(terminal_state: $terminal:ident $({
        $(terminate_success: $terminate_success:block)*
        $(terminate_failure: $terminate_failure:block)*
      })*)*
    }

  ) => {

    def_machine_debug!{
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
        $(self_reference: $self_reference)*
        initial_state: $initial $({
          $(initial_action: $initial_action)*
        })*
        $(terminal_state: $terminal $({
          $(terminate_success: $terminate_success)*
          $(terminate_failure: $terminate_failure)*
        })*)*
      }
    }

    impl $(<$($type_var),+>)* $crate::MachineDotfile
      for $machine $(<$($type_var),+>)*
    where
    $($(
      $type_var : ::std::fmt::Debug,
      $($($type_var : $type_constraint),+)*
    ),+)*
    {
      fn name() -> &'static str {
        stringify!($machine)
      }
      fn type_vars() -> Vec <String> {
        let mut _v = Vec::new();
        $($(
        _v.push (format!(
          "{} = {}", stringify!($type_var),
            unsafe { ::std::intrinsics::type_name::<$type_var>() }));
        )+)*
        _v
      }
      fn extended_state_names() -> Vec <&'static str> {
        let mut _v = Vec::new();
        $(
        _v.push (stringify!($ext_name));
        )*
        _v
      }
      fn extended_state_types() -> Vec <&'static str> {
        let mut _v = Vec::new();
        $(
        _v.push (stringify!($ext_type));
        )*
        _v
      }
      fn extended_state_defaults() -> Vec <&'static str> {
        let mut _v = Vec::new();
        $(
        _v.push (stringify!($($ext_default)*));
        )*
        _v
      }
      fn states() -> Vec <&'static str> {
        let mut v = Vec::new();
        $(
        v.push (stringify!($state));
        )+
        v
      }
      fn state_initial() -> &'static str {
        stringify!($initial)
      }
      fn state_terminal() -> &'static str {
        stringify!($($terminal)*)
      }
      fn state_data_names() -> Vec <Vec <&'static str>> {
        let mut v = Vec::new();
        $({
          let mut _w = Vec::new();
          $(_w.push (stringify!($data_name));)*
          v.push (_w);
        })+
        v
      }
      fn state_data_types() -> Vec <Vec <&'static str>> {
        let mut v = Vec::new();
        $({
          let mut _w = Vec::new();
          $(_w.push (stringify!($data_type));)*
          v.push (_w);
        })+
        v
      }
      fn state_data_defaults() -> Vec <Vec <&'static str>> {
        let mut v = Vec::new();
        $({
          let mut _w = Vec::new();
          $(_w.push (stringify!($($data_default)*));)*
          v.push (_w);
        })+
        v
      }
      /// &#9888; This function creates default values for each state data field
      /// and creates a pretty printed string of the value
      fn state_data_pretty_defaults() -> Vec <Vec <String>> {
        let mut v = Vec::new();
        $({
          let mut _w = Vec::new();
          $(
          let default_val : $data_type
            = def_machine_debug!(@expr_default $($data_default)*);
          _w.push (format!("{:#?}", default_val));
          )*
          v.push (_w);
        })+
        v
      }

      fn events() -> Vec <&'static str> {
        let mut v = Vec::new();
        $(
        v.push (stringify!($event));
        )+
        v
      }
      fn event_sources() -> Vec <&'static str> {
        let mut v = Vec::new();
        $(
        v.push (stringify!($source));
        )+
        v
      }
      fn event_targets() -> Vec <&'static str> {
        let mut v = Vec::new();
        $(
        v.push (stringify!($($target)*));
        )+
        v
      }
      fn event_actions() -> Vec <&'static str> {
        let mut v = Vec::new();
        $(
        v.push (stringify!($($action)*));
        )+
        v
      }
    }

    impl $(<$($type_var),+>)* ExtendedState $(<$($type_var),+>)* where
    $($(
      $type_var : ::std::fmt::Debug,
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
                def_machine_debug!(@expr_option $($ext_default)*)
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
      $(where let $self_reference:ident = self)*
    {
      STATES [
        $(state $state:ident (
          $($data_name:ident : $data_type:ty $(= $data_default:expr)*),*
        ))+
      ]
      EVENTS [
        $(event $event:ident <$source:tt> $(=> <$target:ident>)*
          $({ $($state_data:ident),* } => $action:block)*
        )+
      ]
      initial_state: $initial:ident $({
        $(initial_action: $initial_action:block)*
      })*
      $(terminal_state: $terminal:ident $({
        $(terminate_success: $terminate_success:block)*
        $(terminate_failure: $terminate_failure:block)*
      })*)*
    }

  ) => {

    def_machine_nodefault_debug!{
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
        $(self_reference: $self_reference)*
        initial_state: $initial $({
          $(initial_action: $initial_action)*
        })*
        $(terminal_state: $terminal $({
          $(terminate_success: $terminate_success)*
          $(terminate_failure: $terminate_failure)*
        })*)*
      }
    }

  };

} // end def_machine_nodefault_debug!
