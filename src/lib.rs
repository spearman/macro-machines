#[derive(Debug,PartialEq)]
pub enum HandleEventException {
  WrongState
}

#[macro_export]
macro_rules! def_machine {
  //
  //  main implementation rule
  //
  ( machine $machine:ident {
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

    #[derive(Clone,Debug,PartialEq)]
    pub struct $machine {
      state : State,
      $($data_name : $data_type,)*
    }

    #[derive(Clone,Debug,PartialEq)]
    pub struct State {
      id : StateId
    }

    #[derive(Clone,Debug,PartialEq)]
    pub struct Event {
      id : EventId
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

    impl $machine {
      pub fn new() -> Self {
        trace!("{}::new", stringify!($machine));
        let mut _new = $machine {
          state: State::initial(),
          $($data_name:
            def_machine!{ @impl_default_expr $($data_default)* }
          ),*
        };
        $(let $self_reference = _new;)*
        $($($initial_action)*)*
        $(let _new = $self_reference;)*
        _new
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
              $(let $self_reference = self;)*
              match event.id {
                $(EventId::$event => $($action)*)+
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
    }

    impl Drop for $machine {
      fn drop (&mut self) {
        trace!("{}::drop", stringify!($machine));
        $(let $self_reference = self;)*
        $(
        if $self_reference.state.id != StateId::$terminal {
          trace!("<<< current state ({:?}) != terminal state ({:?})",
            $self_reference.state.id, StateId::$terminal);
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

  //
  //  @impl_default_expr: override default
  //
  ( @impl_default_expr $default:expr ) => { $default };

  //
  //  @impl_default_expr: use default
  //
  ( @impl_default_expr ) => { Default::default() };

  //
  //  alternate syntax
  //
  (
    $machine:ident
    $(<$($self_reference:ident)*>)*
    $((
      $($data_name:ident : $data_type:ty $(= $data_default:expr)*),*
    ))*
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
      machine $machine {
        STATES [
          $(state $state {})+
        ]
        EVENTS [
          $(event $event <$source> => <$target> $(
            $action
          )*)+
        ]
        DATA [
          $($($data_name : $data_type $(= $data_default)*),*)*
        ]
        $($(self_reference: $self_reference)*)*
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

} // end def_machine!
