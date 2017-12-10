# `State machine macros with logging and graphviz dotfile output.macro-machines`

> State machine macros with logging and graphviz dotfile output.

## Usage

Using the macros defined by this library requires the additional external
dependencies:

```toml
log = "0.3.*"
escapade = "0.0.*"
```

and the following directives:

```rust
#![feature(const_fn)]
#![feature(core_intrinsics)]

extern crate escapade;
#[macro_use] extern crate log;
#[macro_use] extern crate macro_machines;
```

Define and use a minimal state machine:

```rust
def_machine_debug!{
  machine M {
    STATES [
      state S ()
      state T ()
    ]
    EVENTS [
      event A <S> => <T>
    ]
    EXTENDED []
    initial_state: S
  }
}

fn main () {
  use macro_machines::*;

  let mut m = M::initial();
  let e = EventId::A.into();
  m.handle_event (e).unwrap();
  let e = EventId::A.into();
  assert_eq!(m.handle_event (e), Err (HandleEventException::WrongState));
}
```

Generate a dotfile and write to file:

```rust
  use std::io::Write;
  let mut f = std::fs::File::create ("minimal.dot").unwrap();
  f.write_all (M::dotfile().as_bytes()).unwrap();
  drop (f);
```

Converted to `png` with `$ dot -Tpng minimal.dot > minimal.png`:

![](minimal.png)
