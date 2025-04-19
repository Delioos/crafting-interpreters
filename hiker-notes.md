# Introduction
1. "There are at least six dsl used in the little system I cobbled together to write and publish this book. What are they?"
-> Java, C, C#, lua, Cpython, Ruby ?

2. Get a "hw" program written and running in java. Setup ide and debugger and shi
-> `cargo new crafting-interpreters`

3. Do the same thing for C. To get some practice with poiters, define a doubly linked list of heap-allocated strings.
Write functions to insert, find and delete items from it. Test them
-> src/chall1 (wip) (will also do it in rust)

# chap 5
## gameplan
interpret a simple language, then add features

first targets: Literals. Numbers, strings, Booleans, and nil.

Unary expressions. A prefix ! to perform a logical not, and - to negate a number.

Binary expressions. The infix arithmetic (+, -, *, /) and logic operators (==, !=, <, <=, >, >=) we know and love.

Parentheses. A pair of ( and ) wrapped around an expression.

That gives us enough syntax for expressions like:

`1 - (2 * 3) < 4 == false`

Using our handy dandy new notation, here’s a grammar for those:

``` 
expression     → literal
               | unary
               | binary
               | grouping ;

literal        → NUMBER | STRING | "true" | "false" | "nil" ;
grouping       → "(" expression ")" ;
unary          → ( "-" | "!" ) expression ;
binary         → expression operator expression ;
operator       → "==" | "!=" | "<" | "<=" | ">" | ">="
               | "+"  | "-"  | "*" | "/" ;
``` 

