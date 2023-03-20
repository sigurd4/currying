A crate for currying anything implementing FnOnce.

Arguments can be passed one at a time, yielding a new something implementing FnOnce (and possibly FnMut and Fn) which can be called with one less argument.