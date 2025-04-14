# Forest standard

This is the standard for the Forest programming language.

## What is Forest

Forest is a Forth, which, like Lisp, is a family of programming languages.
Forths aren't used that much nowadays, but they are very simple.
I am doing this project as a means of learning Rust and learning about
programming programming languages, so I do not expect the languages to be
perfect or good.

### What is a Forth

Forths are a family of stack-based programming languages.
It uses a stack as its primary data structure, and instructions are also
piled onto each other like in a stack.

## Language standard

### Types

Values are pushed onto the stack, and these values have types.
Forest will only support nil, ints, strings and tables, with characters
being strings with length 1, and tables representing all other types (like in
lua).
All non-nil values are interpreted as true by boolean operators.

### Value Syntax

To push values onto the stack, simply write the value as a token.

- nil
  - ... nil ...
- ints
  - ... <int> ...
  - e.g.: ... 69 ...
  - Note that ints are i64s.
- strings
  - ... <string> ...
  - e.g.: ... "Hello world!" ...
  - Note that escape charatcers are allowed, and function normally.
- tables
  - ... <table> ...
  - e.g.: ... {"nice" 69 "everything" 42} ...
  - Note that tables can have any value (other than nil) as a key,
    and all values are valid (including nil).
    Duplicate entries are not allowed.
  - Note that table declarations must have an even number of values in them.

## ints

Ints support the basic arithmetic operations like + - * /.
Note that / is floor division.
- str -> casts an int to a string

## strings

Strings can be appended to each other by using the <> function.
Indexing into a string uses the same syntax as indexing into a table.
Strings are zero indexed. (no u lua)

## tables

These are essentially hashmaps.
To access the value stored at a key, do:
`<map> <key> get`
To associate a value with a key, do:
`<map> <key> <value> assoc`
To get the keys in a map, do:
`<map> keys`
This pushes an table of keys onto the stack, not consuming the map.

Using str on a table simply returns a string of all the values in a table,
wrapped in "{" and "}".

## Conditions
All conditions take the form of:
`if ... end ...`
Note that there doesn't necessarily have to be anything in `if end`.
If the top-most value on the stack is not nil, then the code in `if` and `end`
is executed, else, it is not.

The `and` `or` and `not` functions do what you would expect them to.

## Loops
There is only one kind of loop in Forest, being the `loop end` loop.
Nested `loop` loops are allowed, and loops are terminated by `end`.
The `break` keyword can be used in a loop to skip until the matching `end`.

