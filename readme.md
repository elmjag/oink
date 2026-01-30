# Oink RustPython native module

An example implementation of a native (built-in) module for [RustPython](https://github.com/RustPython/RustPython/).
This crate extends the RustPython interpreter with a custom `oink` python module.
The `oink` module is written in Rust and uses the RustPython extension API.

## `oink` python module

`oink` python module provides `reg_cb()` and `call_cb()` functins.
The `reg_cb()` allows you to register a callback.
The `call_cb()` will invoke the registered callback.
See the [cb_example.py](cb_example.py) for a demonstration.

## `oink` implementation

The `oink` is implemented in the [src/oink.rs](src/oink.rs) file.

It demonstrates usage of RustPython extension API.
It contains an example on:

* using a macro to declare python module
* using a macro to declare python functions
* defining function arguments and return type
* raising exceptions
* invoking python callables

## Building

Build the crate with `cargo build`.
This will create the `oink` binary.
The `oink` binary is the Python interperator,
with the `oink` native module included.

The `oink` crate pulls in RustPython crates as dependencies.
It injects it's custom `oink` python module on interpreter start-up.
See the RustPython [documentation](https://docs.rs/rustpython/latest/rustpython/) for more details.

## Using

Use `oink` binary as the python interperator.
For example, running `oink` without arguments will give you a REPL session.
In that session the `oink` python module will be available.

    $ oink
    Welcome to the magnificent Rust Python 0.4.0 interpreter 😱 🖖
    RustPython 3.14.0
    Type "help", "copyright", "credits" or "license" for more information.
    >>>>> import oink
    >>>>> print(f"{oink=}\n{oink.reg_cb}")
    oink=<module 'oink' (built-in)>
    <builtin_function_or_method object at 0x638e4d0714b0>
