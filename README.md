# `macro-machines`

> State machine macros with logging and graphviz dotfile generation

[Documentation](https://spearman.github.io/macro-machines/macro_machines/)

## Usage

The macros provided by this library expand to definitions using `const fn`s and
some intrinsics to help generate dotfiles, so these features must be enabled in
the crate root:

```rust
#![feature(const_fn)]
#![feature(core_intrinsics)]

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
      event A <S> => <T> ()
    ]
    EXTENDED []
    initial_state: S
  }
}

fn main () {
  use macro_machines::HandleEventException;

  let mut m = M::initial();
  let e = Event::from_id (EventId::A);
  m.handle_event (e).unwrap();
  let e = Event::from_id (EventId::A);
  assert_eq!(m.handle_event (e), Err (HandleEventException::WrongState));
}
```

Generate a dotfile and write to file:

```rust
  use std::io::Write;
  use macro_machines::MachineDotfile;
  let mut f = std::fs::File::create ("minimal.dot").unwrap();
  f.write_all (M::dotfile().as_bytes()).unwrap();
  drop (f);
```

Rendered as PNG with `$ dot -Tpng minimal.dot > minimal.png`:

![](minimal.png)

For examples of more complex state machines, see the `./examples/` directory.


## Dependencies

![](dependencies.png)
