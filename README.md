# mlang

This is a fun side project I started. I often have fun ideas for languages, and I decided for this project I'd learn how to make one. The premise of this language is that match statements are objects. I'll admit, match statements are nice and easy to use as a programmer, but I have no clue how they work under the hood. So right now, the only pattern matching that is done is type checking. And I haven't implemented tuples yet.

I was recommended to use Lex and Yacc, but I didn't want to install C/C++ tools, so I made it in rust. Because of this, it's not a compiled language - that would be a lot of work.

Anyway, onto the basics of the language:

## Basics:

The language follows roughly rust syntax, except for some notable exceptions.
- There are no semicolons
- As of right now, all variables are mutable
- Everything is a reference
- ifs, whiles, fors, and matches are completely different

Now that I think about it, I guess it doesn't follow rust syntax at all.

Here's some example code for various things:

### Instantiating and updating variables
This is pretty much what you expect
```
let num = 0
num = num + 1    // No += operator yet :(
```
### Match statements
```
let obj = // Something
obj
  | int i : 
    // code block
  | float f : // Alternatively, you can do it inline
  | int i ~ i % 2 == 0 : // Match guard
```
#### Match statements as objects
```
let is_even = | num :
  return num % 2 == 0
  
2 is_even // true
```
Because match statements are executed on the right, calls come to the right of the arguments. 
Something to note is that the last statement in a block is assumed to be the return, so you could also write `is_even` as:
```
let is_even = | num :
  num % 2 == 0
```

### If statements
```
let num = 0
|~ num % 2 == 0 : // Basically a match statement without a match pattern
|~ num % 3 == 0 : // Else if
|~ : // Else
```

### For statements
The dollar operator means to apply a match statement over each element in an iterator
```
0..10 $ | i :
  i print
```

### While statements
You can emulate a while statement by looping forever and breaking when a condition is met. There is no dedicated while statement

### Special operators
- `$` : Apply the match to each item in the iterator
- `#` : Filter the iterator by the predicate
- `@` : Map each item in the iterator according to the match
- `&&&` : return true if all items match the predicate
- `|||` : return true if any items match the predicate

Example:
```
// We want to print all the prime numbers below 100, squared
let is_prime = | int num :
  2..num &&& | factor :
    num % factor != 0

// We can either do it very explicitly:
2..100 $ | num ~ num is_prime : (num * num) print

// Or do it using iterators:
let squared = | i : i * i

2..100 # is_prime 
  @ squared 
  $ print
// (Expressions continue if the next line is indented)
```

It's quite a bit of syntax, for sure. It's a fun way to learn to make a language though.

Also, the name is \*temporary. Haven't thought of a better one yet. 
Now that I think of it, there's probably an existing language called mlang out there.
