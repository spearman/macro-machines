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
        ) $({
          $(entry $entry:block)*
          $(exit  $exit:block)*
        })*)+
      ]
      EVENTS [
        $(event $event:ident <$source:tt> $(=> <$target:ident>)*
          ($($param_name:ident : $param_type:ty $(= $param_default:expr)*),*)
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

    $crate::def_machine!{
      @base
      machine $machine
        $(<$($type_var $(: { $($type_constraint),+ })*),+>)*
      {
        STATES [
          $(state $state ($($data_name : $data_type $(= $data_default)*),*)
          $({
            $(entry $entry)*
            $(exit  $exit)*
          })*)+
        ]
        EVENTS [
          $(event $event <$source> $(=> <$target>)*
            ($($param_name : $param_type $(= $param_default)*),*)
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
        $crate::log::trace!("{}::initial", stringify!($machine));
        let mut extended_state = ExtendedState::initial();
        let state = StateId::$initial.to_state (&mut extended_state);
        let mut initial = Self { state, extended_state };
        {
          $(#[allow(unused_variables)]
          let $self_reference = &mut initial;)*
          $($($initial_action)*)*
        }
        initial.state_entry();
        initial
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
            unsafe { std::intrinsics::type_name::<$type_var>() }));
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
            concat!(stringify!($ext_type), "::default()")
          }
        });
        )*
        _v
      }
      fn self_reference() -> &'static str {
        stringify!($($self_reference)*)
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
              concat!(stringify!($data_type), "::default()")
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
            = $crate::def_machine!(@expr_default $($data_default)*);
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
          $($ext_name: $crate::def_machine!(@expr_default $($ext_default)*)),*
        }
      }

      /// Creation method that allows overriding defaults.
      pub fn new ($($ext_name : Option <$ext_type>),*) -> Self {
        Self {
          $($ext_name: $ext_name.unwrap_or (
            $crate::def_machine!(@expr_default $($ext_default)*))
          ),*
        }
      }
    }

    impl <'event> Event <'event> {
      /// Construct an event with default parameters for the given ID
      #[inline]
      pub fn from_id (id : EventId) -> Self {
        let params = id.clone().into();
        Event { id, params }
      }
    }

    impl <'event> From <EventId> for EventParams <'event> {
      fn from (id : EventId) -> Self {
        match id {
          $(EventId::$event => EventParams::$event {
            $($param_name:
              $crate::def_machine!(@expr_default $($param_default)*)
            ),*
          }),+
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
    $(@ $self_reference:ident)*
    {
      STATES [
        $(state $state:ident (
          $($data_name:ident : $data_type:ty $(= $data_default:expr)*),*
        ) $({
          $(entry $entry:block)*
          $(exit  $exit:block)*
        })*)+
      ]
      EVENTS [
        $(event $event:ident <$source:tt> $(=> <$target:ident>)*
          ($($param_name:ident : $param_type:ty $(= $param_default:expr)*),*)
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

    $crate::def_machine!{
      machine $machine $(<$($type_var $(: { $($type_constraint),+ })*),+>)* {
        STATES [
          $(state $state ($($data_name : $data_type $(= $data_default)*),*)
          $({
            $(entry $entry)*
            $(exit  $exit)*
          })*)+
        ]
        EVENTS [
          $(event $event <$source> $(=> <$target>)*
            ($($param_name : $param_type $(= $param_default)*),*)
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

  //
  //  @impl_fn_handle_event
  //
  ( @impl_fn_handle_event
    machine $machine:ident {
      EVENTS [
        $(event $event:ident <$source:tt> $(=> <$target:ident>)*
          ($($param_name:ident),*)
          $({ $($state_data:ident),* } => $action:block)*
        )+
      ]
      EXTENDED [
        $($ext_name:ident : $ext_type:ty $(= $ext_default:expr)*),*
      ]
      $(self_reference: $self_reference:ident)*
    }

  ) => {

    pub fn handle_event (&mut self, mut _event : Event)
      -> Result <(), $crate::HandleEventException>
    {
      $crate::log::trace!("{}::handle_event: {:?}",
        stringify!($machine), _event.id);
      // if only one kind of transition exists the following match expression
      // will detect the other branch as "unreachable_code"
      #[allow(unreachable_code)]
      match _event.transition() {
        Transition::Universal (target_id) => {
          $crate::log::trace!("{}::handle_event: <<< Ok: \
            Universal ({:?} => {:?})",
            stringify!($machine), self.state.id, target_id);
          self.state_exit();
          { // event action
            // bring extended state variables into scope
            #[allow(unused_variables)]
            match &mut self.extended_state {
              &mut ExtendedState { $(ref mut $ext_name,)*.. } => {
                // map each event to an action
                match _event.params {
                  $(EventParams::$event { $(ref mut $param_name,)*.. } => {
                    // only expands universal actions, unreachable otherwise
                    $crate::def_machine!{
                      @event_action_universal
                      event $event <$source> $(=> <$target>)* $($action)*
                    }
                  })+
                  _ => unreachable!("unreachable phantom data variant")
                }
              }
            }
          }
          let state  = target_id.to_state (&mut self.extended_state);
          self.state = state;
          self.state_entry();
          Ok (())
        }
        Transition::Internal (source_id) => {
          if self.state.id == source_id {
            $crate::log::trace!("{}::handle_event: <<< Ok: Internal ({:?})",
              stringify!($machine), source_id);
            // bring extended state variables into scope
            #[allow(unused_variables)]
            match &mut self.extended_state {
              &mut ExtendedState { $(ref mut $ext_name,)*.. } => {
                // map each event to an action
                match _event.params {
                  $(EventParams::$event { $(ref mut $param_name,)*.. } => {
                    // for universal transitions there is no source state so
                    // this produces a wildcard pattern resulting in the
                    // last branch being unreachable
                    // bring local state variables into scope
                    #[allow(unreachable_patterns)]
                    match &mut self.state.data {
                      $crate::def_machine!{
                        @event_internal_state_pattern
                        $source { $($($state_data),*)* }
                      } => {
                        // only expands internal actions, unreachable otherwise
                        $crate::def_machine!{
                          @event_action_internal
                          event $event <$source> $(=> <$target>)* $($action)*
                        }
                      }
                      _ => unreachable!("current state should match event source")
                    }
                  })+
                  _ => unreachable!("unreachable phantom data variant")
                }
              }
            }
            Ok (())
          } else {
            $crate::log::trace!("{}::handle_event: <<< Err: \
              internal transition current state ({:?}) != state ({:?})",
                stringify!($machine), self.state.id, source_id);
            Err ($crate::HandleEventException::WrongState)
          }
        }
        Transition::External (source_id, target_id) => {
          if self.state.id == source_id {
            $crate::log::trace!("{}::handle_event: <<< Ok: \
              External ({:?} => {:?})",
              stringify!($machine), source_id, target_id);
            self.state_exit();
            { // event action
              // bring extended state variables into scope
              #[allow(unused_variables)]
              match &mut self.extended_state {
                &mut ExtendedState { $(ref mut $ext_name,)*.. } => {
                  // map each event to an action
                  match _event.params {
                    $(EventParams::$event { $(ref mut $param_name,)*.. } => {
                      // only expands external actions, unreachable otherwise
                      $crate::def_machine!{
                        @event_action_external
                        event $event <$source> $(=> <$target>)* $($action)*
                      }
                    })+
                    _ => unreachable!("unreachable phantom data variant")
                  }
                }
              }
            }
            let state  = target_id.to_state (&mut self.extended_state);
            self.state = state;
            self.state_entry();
            Ok (())
          } else {
            $crate::log::trace!("{}::handle_event: <<< Err: \
              external transition current state ({:?}) != source state ({:?})",
                stringify!($machine), self.state.id, source_id);
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
        ) $({
          $(entry $entry:block)*
          $(exit  $exit:block)*
        })*)+
      ]
      EVENTS [
        $(event $event:ident <$source:tt> $(=> <$target:ident>)*
          ($($param_name:ident : $param_type:ty $(= $param_default:expr)*),*)
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

    pub struct Event <'event> {
      id     : EventId,
      params : EventParams <'event>
    }

    #[derive(Clone, Debug, Eq, PartialEq,$crate::variant_count::VariantCount)]
    pub enum StateId {
      $($state),+
    }

    pub enum StateData {
      $($state {
        $($data_name : $data_type),*
      }),+
    }

    #[derive(Debug, Eq, PartialEq)]
    pub enum Transition {
      Internal  (StateId),
      External  (StateId, StateId),
      Universal (StateId)
    }

    #[derive(Clone, Debug, Eq, PartialEq, $crate::variant_count::VariantCount)]
    pub enum EventId {
      $($event),+
    }

    pub enum EventParams <'event> {
      $($event {
        $($param_name : $param_type),*
      },)+
      _PhantomData (::std::marker::PhantomData <&'event ()>)
    }

    impl $(<$($type_var),+>)* $machine $(<$($type_var),+>)* where
    $($(
      $($($type_var : $type_constraint),+)*
    ),+)*
    {
      pub fn report_sizes() where $($($type_var : 'static),+)* {
        let machine_name = stringify!($machine);
        let machine_type = unsafe { std::intrinsics::type_name::<Self>() };
        println!("{}::report_sizes...", machine_name);
        println!("  size of {}: {}", machine_type,
          std::mem::size_of::<Self>());
        println!("...{}::report_sizes", machine_name);
      }

      pub fn new (mut extended_state : ExtendedState $(<$($type_var),+>)*)
        -> Self
      {
        let state   = StateId::$initial.to_state (&mut extended_state);
        let mut new = Self { state, extended_state };
        {
          $(#[allow(unused_variables)]
          let $self_reference = &mut new;)*
          $($($initial_action)*)*
        }
        new.state_entry();
        new
      }

      #[allow(dead_code)]
      #[inline]
      pub fn state (&self) -> &State {
        &self.state
      }

      #[allow(dead_code)]
      #[inline]
      pub fn state_id (&self) -> StateId {
        self.state().id().clone()
      }

      #[allow(dead_code)]
      #[inline]
      pub fn state_data (&self) -> &StateData {
        self.state().data()
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

      $crate::def_machine!{
        @impl_fn_handle_event
        machine $machine {
          EVENTS [
            $(event $event <$source> $(=> <$target>)*
              ($($param_name),*)
              $({$($state_data),*} => $action)*
            )+
          ]
          EXTENDED [
            $($ext_name : $ext_type $(= $ext_default)*),*
          ]
          $(self_reference: $self_reference)*
        }
      }

      fn state_entry (&mut self) {
        // bring extended state variables into scope
        #[allow(unused_variables)]
        match &mut self.extended_state {
          &mut ExtendedState { $(ref mut $ext_name,)*.. } => {
            match self.state.data {
              // bring local state variables into scope
              $(StateData::$state { $(ref mut $data_name,)*.. } => {
                $($($entry)*)*
              })+
            }
          }
        }
      }
      fn state_exit (&mut self) {
        // bring extended state variables into scope
        #[allow(unused_variables)]
        match &mut self.extended_state {
          &mut ExtendedState { $(ref mut $ext_name,)*.. } => {
            match self.state.data {
              // bring local state variables into scope
              $(StateData::$state { $(ref mut $data_name,)*.. } => {
                $($($exit)*)*
              })+
            }
          }
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
        $crate::log::trace!("{}::drop", stringify!($machine));
        self.state_exit();
        let _state_id = self.state.id.clone();
        $(#[allow(unused_variables)]
        let $self_reference = &mut *self;)*
        $(
        if _state_id != StateId::$terminal {
          $crate::log::trace!("{}::drop failure: \
            current state ({:?}) != terminal state ({:?})",
              stringify!($machine), _state_id, StateId::$terminal);
          $($($terminate_failure)*)*
        } else {
          $($($terminate_success)*)*
        }
        )*
      }
    }

    impl State {
      #[inline]
      pub fn id (&self) -> &StateId {
        &self.id
      }

      #[inline]
      pub fn data (&self) -> &StateData {
        &self.data
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
      pub fn to_state $(<$($type_var),+>)* (self,
        extended_state : &mut ExtendedState$(<$($type_var),+>)*) -> State
      where
      $($(
        $($($type_var : $type_constraint),+)*
      ),+)*
      {
        // bring extended state variables into scope
        #[allow(unused_variables)]
        match extended_state {
          &mut ExtendedState { $(ref mut $ext_name,)*.. } => {
            match self {
              $(StateId::$state => {
                State {
                  id:   self,
                  data: StateData::$state {
                    $($data_name:
                      $crate::def_machine!(@expr_default $($data_default)*)
                    ),*
                  }
                }
              })+
            }
          }
        }
      }
    }

    impl EventId {
      pub fn transition (&self) -> Transition {
        match *self {
          $(
          EventId::$event =>
            $crate::def_machine!(@event_transition <$source> $(=> <$target>)*)
          ),+
        }
      }
    }

    impl <'event> EventParams <'event> {
      pub fn id (&self) -> EventId {
        match *self {
          $(EventParams::$event {..} => EventId::$event,)+
          _ => unreachable!("unreachable phantom data variant")
        }
      }
    }

    impl <'event> Event <'event> {
      #[inline]
      pub fn transition (&self) -> Transition {
        self.id.transition()
      }

      #[inline]
      pub fn id (&self) -> &EventId {
        &self.id
      }

      #[inline]
      pub fn params (&self) -> &EventParams {
        &self.params
      }
    }

    impl <'event> From <EventParams <'event>> for Event <'event> {
      fn from (params : EventParams <'event>) -> Self {
        let id = params.id();
        Event { id, params }
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
        ) $({
          $(entry $entry:block)*
          $(exit  $exit:block)*
        })*)+
      ]
      EVENTS [
        $(event $event:ident <$source:tt> $(=> <$target:ident>)*
          ($($param_name:ident : $param_type:ty $(= $param_default:expr)*),*)
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

    $crate::def_machine!{
      @base
      machine $machine
        $(<$($type_var $(: { $($type_constraint),+ })*),+>)*
      {
        STATES [
          $(state $state ($($data_name : $data_type $(= $data_default)*),*)
          $({
            $(entry $entry)*
            $(exit  $exit)*
          })*)+
        ]
        EVENTS [
          $(event $event <$source> $(=> <$target>)*
            ($($param_name : $param_type $(= $param_default)*),*)
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
            unsafe { std::intrinsics::type_name::<$type_var>() }));
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
      fn self_reference() -> &'static str {
        stringify!($($self_reference)*)
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
      /// This version does not evaluate expressions, only pretty prints them
      fn state_data_pretty_defaults() -> Vec <Vec <String>> {
        let mut v = Vec::new();
        $({
          let mut _w = Vec::new();
          $(
          _w.push (format!("{:#?}", stringify!($($data_default)*)));
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
                $crate::def_machine!(@expr_option $($ext_default)*)
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
    $(@ $self_reference:ident)*
    {
      STATES [
        $(state $state:ident (
          $($data_name:ident : $data_type:ty $(= $data_default:expr)*),*
        ) $({
          $(entry $entry:block)*
          $(exit  $exit:block)*
        })*)+
      ]
      EVENTS [
        $(event $event:ident <$source:tt> $(=> <$target:ident>)*
          ($($param_name:ident : $param_type:ty $(= $param_default:expr)*),*)
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

    $crate::def_machine_nodefault!{
      machine $machine $(<$($type_var $(: { $($type_constraint),+ })*),+>)* {
        STATES [
          $(state $state ($($data_name : $data_type $(= $data_default)*),*)
          $({
            $(entry $entry)*
            $(exit  $exit)*
          })*)+
        ]
        EVENTS [
          $(event $event <$source> $(=> <$target>)*
            ($($param_name : $param_type $(= $param_default)*),*)
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
        ) $({
          $(entry $entry:block)*
          $(exit  $exit:block)*
        })*)+
      ]
      EVENTS [
        $(event $event:ident <$source:tt> $(=> <$target:ident>)*
          ($($param_name:ident : $param_type:ty $(= $param_default:expr)*),*)
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

    $crate::def_machine_debug!{
      @base
      machine $machine
        $(<$($type_var $(: { $($type_constraint),+ })*),+>)*
      {
        STATES [
          $(state $state ($($data_name : $data_type $(= $data_default)*),*)
          $({
            $(entry $entry)*
            $(exit  $exit)*
          })*)+
        ]
        EVENTS [
          $(event $event <$source> $(=> <$target>)*
            ($($param_name : $param_type $(= $param_default)*),*)
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
      $type_var : std::fmt::Debug,
      $($($type_var : $type_constraint),+)*
    ),+)*
    {
      pub fn initial() -> Self {
        $crate::log::trace!("{}::initial", stringify!($machine));
        let mut extended_state = ExtendedState::initial();
        let state = StateId::$initial.to_state (&mut extended_state);
        let mut initial = Self { state, extended_state };
        {
          $(#[allow(unused_variables)]
          let $self_reference = &mut initial;)*
          $($($initial_action)*)*
        }
        initial.state_entry();
        initial
      }
    }

    impl $(<$($type_var),+>)* $crate::MachineDotfile
      for $machine $(<$($type_var),+>)*
    where
    $($(
      $type_var : std::fmt::Debug,
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
            unsafe { std::intrinsics::type_name::<$type_var>() }));
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
            concat!(stringify!($ext_type), "::default()")
          }
        });
        )*
        _v
      }
      fn self_reference() -> &'static str {
        stringify!($($self_reference)*)
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
              concat!(stringify!($data_type), "::default()")
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
            = $crate::def_machine_debug!(@expr_default $($data_default)*);
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
      $type_var : std::fmt::Debug,
      $($($type_var : $type_constraint),+)*
    ),+)*
    {
      pub fn initial() -> Self {
        Self {
          $($ext_name: $crate::def_machine_debug!(@expr_default $($ext_default)*)),*
        }
      }

      /// Creation method that allows overriding defaults.
      pub fn new ($($ext_name : Option <$ext_type>),*) -> Self {
        Self {
          $($ext_name: $ext_name.unwrap_or (
            $crate::def_machine_debug!(@expr_default $($ext_default)*))
          ),*
        }
      }
    }

    impl <'event> Event <'event> {
      /// Construct an event with default parameters for the given ID
      #[inline]
      pub fn from_id (id : EventId) -> Self {
        let params = id.clone().into();
        Event { id, params }
      }
    }

    impl <'event> From <EventId> for EventParams <'event> {
      fn from (id : EventId) -> Self {
        match id {
          $(EventId::$event => EventParams::$event {
            $($param_name:
              $crate::def_machine_debug!(@expr_default $($param_default)*)
            ),*
          }),+
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
    $(@ $self_reference:ident)*
    {
      STATES [
        $(state $state:ident (
          $($data_name:ident : $data_type:ty $(= $data_default:expr)*),*
        ) $({
          $(entry $entry:block)*
          $(exit  $exit:block)*
        })*)+
      ]
      EVENTS [
        $(event $event:ident <$source:tt> $(=> <$target:ident>)*
          ($($param_name:ident : $param_type:ty $(= $param_default:expr)*),*)
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

    $crate::def_machine_debug!{
      machine $machine $(<$($type_var $(: { $($type_constraint),+ })*),+>)* {
        STATES [
          $(state $state ($($data_name : $data_type $(= $data_default)*),*)
          $({
            $(entry $entry)*
            $(exit  $exit)*
          })*)+
        ]
        EVENTS [
          $(event $event <$source> $(=> <$target>)*
            ($($param_name : $param_type $(= $param_default)*),*)
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

  //
  //  @impl_fn_handle_event
  //
  ( @impl_fn_handle_event
    machine $machine:ident {
      EVENTS [
        $(event $event:ident <$source:tt> $(=> <$target:ident>)*
          ($($param_name:ident),*)
          $({ $($state_data:ident),* } => $action:block)*
        )+
      ]
      EXTENDED [
        $($ext_name:ident : $ext_type:ty $(= $ext_default:expr)*),*
      ]
      $(self_reference: $self_reference:ident)*
    }

  ) => {

    pub fn handle_event (&mut self, mut _event : Event)
      -> Result <(), $crate::HandleEventException>
    {
      $crate::log::trace!("{}::handle_event: {:?}", stringify!($machine), _event.id);
      // if only one kind of transition exists the following match expression
      // will detect the other branch as "unreachable_code"
      #[allow(unreachable_code)]
      match _event.transition() {
        Transition::Universal (target_id) => {
          $crate::log::trace!("{}::handle_event: <<< Ok: \
            Universal ({:?} => {:?})",
            stringify!($machine), self.state.id, target_id);
          self.state_exit();
          { // event action
            // bring extended state variables into scope
            #[allow(unused_variables)]
            match &mut self.extended_state {
              &mut ExtendedState { $(ref mut $ext_name,)*.. } => {
                // map each event to an action
                match _event.params {
                  $(EventParams::$event { $(ref mut $param_name,)*.. } => {
                    // only expands universal actions, unreachable otherwise
                    $crate::def_machine_debug!{
                      @event_action_universal
                      event $event <$source> $(=> <$target>)* $($action)*
                    }
                  })+
                  _ => unreachable!("unreachable phantom data variant")
                }
              }
            }
          }
          let state  = target_id.to_state (&mut self.extended_state);
          self.state = state;
          self.state_entry();
          Ok (())
        }
        Transition::Internal (source_id) => {
          if self.state.id == source_id {
            $crate::log::trace!("{}::handle_event: <<< Ok: Internal ({:?})",
              stringify!($machine), source_id);
            // bring extended state variables into scope
            #[allow(unused_variables)]
            match &mut self.extended_state {
              &mut ExtendedState { $(ref mut $ext_name,)*.. } => {
                // map each event to an action
                match _event.params {
                  $(EventParams::$event { $(ref mut $param_name,)*.. } => {
                    // for universal transitions there is no source state so
                    // this produces a wildcard pattern resulting in the
                    // last branch being unreachable
                    // bring local state variables into scope
                    #[allow(unreachable_patterns)]
                    match &mut self.state.data {
                      $crate::def_machine_debug!{
                        @event_internal_state_pattern
                        $source { $($($state_data),*)* }
                      } => {
                        // only expands internal actions, unreachable otherwise
                        $crate::def_machine_debug!{
                          @event_action_internal
                          event $event <$source> $(=> <$target>)* $($action)*
                        }
                      }
                      _ => unreachable!("current state should match event source")
                    }
                  })+
                  _ => unreachable!("unreachable phantom data variant")
                }
              }
            }
            Ok (())
          } else {
            $crate::log::trace!("{}::handle_event: <<< Err: \
              internal transition current state ({:?}) != state ({:?})",
                stringify!($machine), self.state.id, source_id);
            Err ($crate::HandleEventException::WrongState)
          }
        }
        Transition::External (source_id, target_id) => {
          if self.state.id == source_id {
            $crate::log::trace!("{}::handle_event: <<< Ok: \
              External ({:?} => {:?})",
              stringify!($machine), source_id, target_id);
            self.state_exit();
            { // event action
              // bring extended state variables into scope
              #[allow(unused_variables)]
              match &mut self.extended_state {
                &mut ExtendedState { $(ref mut $ext_name,)*.. } => {
                  // map each event to an action
                  match _event.params {
                    $(EventParams::$event { $(ref mut $param_name,)*.. } => {
                      // only expands external actions, unreachable otherwise
                      $crate::def_machine_debug!{
                        @event_action_external
                        event $event <$source> $(=> <$target>)* $($action)*
                      }
                    })+
                    _ => unreachable!("unreachable phantom data variant")
                  }
                }
              }
            }
            let state  = target_id.to_state (&mut self.extended_state);
            self.state = state;
            self.state_entry();
            Ok (())
          } else {
            $crate::log::trace!("{}::handle_event: <<< Err: \
              external transition current state ({:?}) != source state ({:?})",
                stringify!($machine), self.state.id, source_id);
            Err ($crate::HandleEventException::WrongState)
          }
        }
      }
    } // end handle event

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
        ) $({
          $(entry $entry:block)*
          $(exit  $exit:block)*
        })*)+
      ]
      EVENTS [
        $(event $event:ident <$source:tt> $(=> <$target:ident>)*
          ($($param_name:ident : $param_type:ty $(= $param_default:expr)*),*)
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

    #[derive(Debug)]
    pub struct Event <'event> {
      id     : EventId,
      params : EventParams <'event>
    }

    #[derive(Clone, Debug, Eq, PartialEq, $crate::variant_count::VariantCount)]
    pub enum StateId {
      $($state),+
    }

    #[derive(Debug)]
    pub enum StateData {
      $($state {
        $($data_name : $data_type),*
      }),+
    }

    #[derive(Debug, Eq, PartialEq)]
    pub enum Transition {
      Internal  (StateId),
      External  (StateId, StateId),
      Universal (StateId)
    }

    #[derive(Clone, Debug, Eq, PartialEq, $crate::variant_count::VariantCount)]
    pub enum EventId {
      $($event),+
    }

    #[derive(Debug)]
    pub enum EventParams <'event> {
      $($event {
        $($param_name : $param_type),*
      },)+
      _PhantomData (::std::marker::PhantomData <&'event ()>)
    }

    impl $(<$($type_var),+>)* $machine $(<$($type_var),+>)* where
    $($(
      $type_var : std::fmt::Debug,
      $($($type_var : $type_constraint),+)*
    ),+)*
    {
      pub fn report_sizes() where $($($type_var : 'static),+)* {
        let machine_name = stringify!($machine);
        let machine_type = unsafe { std::intrinsics::type_name::<Self>() };
        println!("{} report sizes...", machine_name);
        println!("  size of {}: {}", machine_type, std::mem::size_of::<Self>());
        println!("...{} report sizes", machine_name);
      }

      pub fn new (mut extended_state : ExtendedState $(<$($type_var),+>)*)
        -> Self
      {
        let state   = StateId::$initial.to_state (&mut extended_state);
        let mut new = Self { state, extended_state };
        {
          $(#[allow(unused_variables)]
          let $self_reference = &mut new;)*
          $($($initial_action)*)*
        }
        new.state_entry();
        new
      }

      #[allow(dead_code)]
      #[inline]
      pub fn state (&self) -> &State {
        &self.state
      }

      #[allow(dead_code)]
      #[inline]
      pub fn state_id (&self) -> StateId {
        self.state().id().clone()
      }

      #[allow(dead_code)]
      #[inline]
      pub fn state_data (&self) -> &StateData {
        self.state().data()
      }

      #[allow(dead_code)]
      #[inline]
      pub fn extended_state (&self) -> &ExtendedState $(<$($type_var),+>)* {
        &self.extended_state
      }

      $crate::def_machine_debug!{
        @impl_fn_handle_event
        machine $machine {
          EVENTS [
            $(event $event <$source> $(=> <$target>)*
              ($($param_name),*)
              $({$($state_data),*} => $action)*
            )+
          ]
          EXTENDED [
            $($ext_name : $ext_type $(= $ext_default)*),*
          ]
          $(self_reference: $self_reference)*
        }
      }

      fn state_entry (&mut self) {
        // bring extended state variables into scope
        #[allow(unused_variables)]
        match &mut self.extended_state {
          &mut ExtendedState { $(ref mut $ext_name,)*.. } => {
            match self.state.data {
              // bring local state variables into scope
              $(StateData::$state { $(ref mut $data_name,)*.. } => {
                $($($entry)*)*
              })+
            }
          }
        }
      }
      fn state_exit (&mut self) {
        // bring extended state variables into scope
        #[allow(unused_variables)]
        match &mut self.extended_state {
          &mut ExtendedState { $(ref mut $ext_name,)*.. } => {
            match self.state.data {
              // bring local state variables into scope
              $(StateData::$state { $(ref mut $data_name,)*.. } => {
                $($($exit)*)*
              })+
            }
          }
        }
      }

    } // end impl $machine

    impl $(<$($type_var),+>)* AsRef <ExtendedState $(<$($type_var),+>)*>
      for $machine $(<$($type_var),+>)*
    where
    $($(
      $type_var : std::fmt::Debug,
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
      $type_var : std::fmt::Debug,
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
      $type_var : std::fmt::Debug,
      $($($type_var : $type_constraint),+)*
    ),+)*
    {
      fn drop (&mut self) {
        $crate::log::trace!("{}::drop", stringify!($machine));
        self.state_exit();
        let _state_id = self.state.id.clone();
        $(#[allow(unused_variables)]
        let $self_reference = &mut *self;)*
        $(
        if _state_id != StateId::$terminal {
          $crate::log::trace!("{}::drop failure: \
            current state ({:?}) != terminal state ({:?})",
              stringify!($machine), _state_id, StateId::$terminal);
          $($($terminate_failure)*)*
        } else {
          $($($terminate_success)*)*
        }
        )*
      }
    }

    impl State {
      #[inline]
      pub fn id (&self) -> &StateId {
        &self.id
      }

      #[inline]
      pub fn data (&self) -> &StateData {
        &self.data
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
      pub fn to_state $(<$($type_var),+>)* (self,
        extended_state : &mut ExtendedState$(<$($type_var),+>)*) -> State
      where
      $($(
        $type_var : std::fmt::Debug,
        $($($type_var : $type_constraint),+)*
      ),+)*
      {
        // bring extended state variables into scope
        #[allow(unused_variables)]
        match extended_state {
          &mut ExtendedState { $(ref mut $ext_name,)*.. } => {
            match self {
              $(StateId::$state => {
                State {
                  id:   self,
                  data: StateData::$state {
                    $($data_name:
                      $crate::def_machine_debug!(@expr_default $($data_default)*)
                    ),*
                  }
                }
              })+
            }
          }
        }
      }
    }

    impl EventId {
      pub fn transition (&self) -> Transition {
        match *self {
          $(
          EventId::$event =>
            $crate::def_machine_debug!(@event_transition <$source> $(=> <$target>)*)
          ),+
        }
      }
    }

    impl <'event> EventParams <'event> {
      pub fn id (&self) -> EventId {
        match *self {
          $(EventParams::$event {..} => EventId::$event,)+
          _ => unreachable!("unreachable phantom data variant")
        }
      }
    }

    impl <'event> Event <'event> {
      #[inline]
      pub fn transition (&self) -> Transition {
        self.id.transition()
      }

      #[inline]
      pub fn id (&self) -> &EventId {
        &self.id
      }

      #[inline]
      pub fn params (&self) -> &EventParams {
        &self.params
      }
    }

    impl <'event> From <EventParams <'event>> for Event <'event> {
      fn from (params : EventParams <'event>) -> Self {
        let id = params.id();
        Event { id, params }
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
        ) $({
          $(entry $entry:block)*
          $(exit  $exit:block)*
        })*)+
      ]
      EVENTS [
        $(event $event:ident <$source:tt> $(=> <$target:ident>)*
          ($($param_name:ident : $param_type:ty $(=> $param_default:expr)*),*)
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

    $crate::def_machine_debug!{
      @base
      machine $machine
        $(<$($type_var $(: { $($type_constraint),+ })*),+>)*
      {
        STATES [
          $(state $state ($($data_name : $data_type $(= $data_default)*),*)
          $({
            $(entry $entry)*
            $(exit  $exit)*
          })*)+
        ]
        EVENTS [
          $(event $event <$source> $(=> <$target>)*
            ($($param_name : $param_type $(=> $param_default)*),*)
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
      $type_var : std::fmt::Debug,
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
            unsafe { std::intrinsics::type_name::<$type_var>() }));
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
      fn self_reference() -> &'static str {
        stringify!($($self_reference)*)
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
      /// This version does not evaluate expressions, only pretty prints them
      fn state_data_pretty_defaults() -> Vec <Vec <String>> {
        let mut v = Vec::new();
        $({
          let mut _w = Vec::new();
          $(
          _w.push (format!("{:#?}", stringify!($($data_default)*)));
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
                $crate::def_machine_debug!(@expr_option $($ext_default)*)
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
    $(@ $self_reference:ident)*
    {
      STATES [
        $(state $state:ident (
          $($data_name:ident : $data_type:ty $(= $data_default:expr)*),*
        ) $({
          $(entry $entry:block)*
          $(exit  $exit:block)*
        })*)+
      ]
      EVENTS [
        $(event $event:ident <$source:tt> $(=> <$target:ident>)*
          ($($param_name:ident : $param_type:ty $(= $param_default:expr)*),*)
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

    $crate::def_machine_nodefault_debug!{
      machine $machine $(<$($type_var $(: { $($type_constraint),+ })*),+>)* {
        STATES [
          $(state $state ($($data_name : $data_type $(= $data_default)*),*)
          $({
            $(entry $entry)*
            $(exit  $exit)*
          })*)+
        ]
        EVENTS [
          $(event $event <$source> $(=> <$target>)*
            ($($param_name : $param_type $(= $param_default)*),*)
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
