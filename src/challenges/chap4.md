# _The lexical grammars of Python and Haskell are not regauler. What does that mean, and where aren't they?_
-> they inlclud the number of space as lexemes to processs indentation

# _Aside from separating tokens - distiringuishing `print foo` from `printfoo` - spaces aren't used for much in most languages. However, in a couple of dark corners, a space does affect hos code is parsed in CoffeeScript, Ruby, and the C preprocessor. Where and what effect odes it have in each of those languages?_ 

- C preprocessor: 
closest answer I got https://stackoverflow.com/questions/37796947/spaces-inserted-by-the-c-preprocessor
- CoffeeScript: 
```
The reason for this is that parentheses for a function call (invocation) are optional. I find this a constant confusion in my own code and have a general policy of always including parentheses to make it clear.

In coffee script, if you leave out the parentheses it assumes that the argument list goes to the end of the line. Your first example coffee script is actually the same as this:

nums = [1..10].map((i) -> i*2)
where the first argument of the call to map is a function (i)->i*2

If you remove the space between the map and the (i) then coffee script implies parentheses around the rest of the line. Your second example coffee script is actually the same as this:

nums = [1..10].map(i)(-> i*2)
Here you can see that map is being called with i as the only argument and then coffee script is expecting the map(i) call to return a function which is then being called passing ->i*2 or to be more explicit ()->i*2 as an argument.

Given that coffee script is designed to remove the potential coding hazards of javascript, I think that it would have been much safer if they had not included this implied parenthesis.```
- Ruby:
```
a.count - 1 # you are subtracting 
a.count -1  # you are doing a.count(-1)

```

# _Our scanner here, like most, discards comments and whitespace since those aren't needed by the parser. Why might you want to write a scanner that does not discard those? What would it be useful for?_
First use case would be to lint comments (e.g. : Starts with a capital letter or something like that, also split them on multiple lines to make it readable)
Second usefull feature would be to use them as built in documentation (like javadoc or docs.rs). What I mean by that is we could leverage the existing scanner instead of reparsing everything and build a standalone tool with almost every same lib

# Add support to Lox's scanner for Cstyle /* ... */ lock comments. make sure to handle newlines in them. Consider allowing them to nest. Is adding support for nest more work than you expected? Why?_ 

// before starting : I think nesting is an overhead becausse it requires to count the number of arms to avoid eixtiing the comment too early ye. Said like that it doesn't super duper hard but we'll see
