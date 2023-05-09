- [ ] Allow the following code to work without parentheses around the map:
```
0..100 
  # is_prime 
  @ (| p : p * p) // <= These parentheses should not be necessary
  $ print
```
- [ ] Find a better way to represent multiple if statements without parentheses:
```
0..101 $ | num :
  let out = "";
  (|~ num % 3 == 0:
    out = out + "fizz") // <= These parentheses should not be necessary. Find another way to represent this.
  (|~ num % 5 == 0:
    out = out + "buzz")
  (|~: out = num)
  out print
```
- [ ] Force mutable variables to be declared with `mut`:
```
let mut x = 0
x++
let y = 0
y++ // <= This should be an error
```
- [ ] Replace `#`, `@`, and `$` with keywords:
```
0..100 
  keep is_prime 
  map | p : p * p
  each print
```
- [ ] OR make filters, maps, and matches identical. One problem with this is that we can no longer have lazy iterators:
```
0..100
  $ is_prime            // returns None is false, and the input if true. This isn't ideal, but it means the iterator can skip the None values.
  $ | p : p * p         // Regular map
  $ print               // map that returns none. (print has no return value)
```
- [ ] Allow the creation of types. Syntax pending:
```
type Person:
  pub mut first_name: str
  pub mut last_name: str
  pub id: int
  pub mut favorite_color: str
  pub mut age: int

// Types only have static methods and variables (non-static methods would be equivalent to a function that takes a person as the first argument)
impl Person:
  mut ID_COUNTER: int = 0

  new = | first_name, last_name, favorite_color :
    Person:
      first_name
      last_name
      id: ID_COUNTER++
      favorite_color
      age: 0

// Create a new person
let person = ("John", "Doe", "blue") Person::new

// OR use explicit syntax (Can only be used if all fields are public)
let person2 = Person:
  first_name: "Jane"
  last_name: "Doe"
  id: 1
  favorite_color: "pink"
  age: 0

// Access a field
person.first_name print
```