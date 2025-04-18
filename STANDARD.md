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

### Words

This is the quintessential concept in Forth. In Forest, they are implemented
as macros. Essentially, when a word is invoked, the contents of the word are
pushed onto the instruction stack, and the program executes by popping
instructions off of that stack and interpreting the instructions.

To declare a variable word, do:
`: <name of word> <content of words> ;`
Note that the spaces after `:` and before `;` are mandatory.

To declare a constant word, do:
`:: <name of word> <content of words> ;`
Note that the spaces after `:` and before `;` are mandatory.
Reassigning to a constant word will throw a runtime error.

### Variables

Variable definition takes the topmost value on the stack and assigns it to a
name. Variables and words cannot share the same namespace, but defining a
variable or a word will override the other.

To declare a variable, do:
`-> <name of variable>`
This will consume the topmost value of the stack.

To declare a constant, do:
`=> <name of variable>`
This will consume the topmost value of the stack.

### Constants


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
  - Also note that spaces are not allowed, instead, \s has to be used. <!--TODO: fix this-->
- tables
  - ... <table> ...
  - e.g.: ... {} ...
  - Note that tables can have any value (other than nil) as a key,
    and all values are valid (including nil).
  - Note that table literals are not allowed, you have to declare an empty
    table then populate it with `assoc`. <!--TODO: fix this-->
    Duplicate entries are not allowed.
  - Note that table declarations must have an even number of values in them.

All values can be compared with =.
All values can be duplicated with dup, and dropped with drop.
All top 2 values can be swapped with swap.

## ints

Ints support the basic arithmetic operations like + - * /.
Note that / is floor division.
- str -> casts an int to a string

## strings

Strings can be appended to each other by using the <> function.
Indexing into a string uses the same syntax as indexing into a table.
Strings are zero indexed. (no u lua)
Strings can be printed using print.

## tables

These are essentially hashmaps.
To access the value stored at a key, do:
`<map> <key> get`
To associate a value with a key, do:
`<map> <key> <value> assoc`
To get the keys in a map, do:
`<map> keys`
This pushes an table of keys onto the stack, not consuming the map.
`<map> vals`
This pushes an table of keys onto the stack, not consuming the map.
`<map> splat`
This pushes the values stored in a table onto the stack, consuming the map.

Using str on a table simply returns a string of all the values in a table,
wrapped in "{" and "}".

## Conditions
All conditions take the form of:
`if ... ifend ...`
Note that there doesn't necessarily have to be anything in `if ifend`.
If the top-most value on the stack is not nil, then the code in `if` and `ifend`
is executed, else, it is not.

The `&` `|` and `!` functions do what you would expect them to.

## Loops
There is only one kind of loop in Forest, being the `[ ]` loop.
Nested `[ ]` loops are allowed, and loops are terminated by `]`.
The `break` keyword can be used in a loop to skip until the matching `]`.

