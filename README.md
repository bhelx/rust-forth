## Forth

An attempt at a stripped down version of [Forth](https://en.wikipedia.org/wiki/Forth_(programming_language)) implemented in Rust.
This was a weekend project and the first full program I've written in Rust, so I wouldn't take anything here as good advice.

### Using

First build:

```bash
cargo build
```

Start a REPL and do fizz buzz:

```bash
$ ./target/debug/forth
: fizz?  3 mod 0 = dup if ." Fizz" then ;
: buzz?  5 mod 0 = dup if ." Buzz" then ;
: fizz-buzz?  dup fizz? swap buzz? or invert ;
: do-fizz-buzz do i fizz-buzz? if i . then loop ;
20 1 do-fizz-buzz
1
2
Fizz
4
Buzz
Fizz
7
8
Fizz
Buzz
11
Fizz
13
14
Fizz
Buzz
16
17
Fizz
19
```
