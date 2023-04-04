# Kiban's grammar
## Types and values
### Unknown
Assigned automatically when type can't be inferred

### Reference
Reference to other type
`<identifier>`
> `let my_value: my_type = (true, false);`

### Boolean
`bool`: `true`/`false`
> `let my_bool: bool = true;`

### Integer
`int`: `<integer>` **(`int` size is `32` and is `signed`)**
> `let my_integer: int = 0;`

### Float
`float`: `<float>` **(`float` size is `32`)**
> `let my_float: float = 0.0;`

### Char
`char`: `'<char>'`
> `let my_string: char = '0';`

### String
`string`: `"<text>"`
> `let my_string: string = "This is my string";`

### Vector
`vec<<type>>`: `[<value>, ...]` **(All `value` must have the same type)**
> `let my_vector: vec<bool> = [true, false];`

### Tuple
`(<type>, ...)`: `(<value>, ...)`
>> `let my_tuple: (bool, string) = (true, "This is my tuple");`

### Function
`fn(<parameter_name>: <type>, ...) -> <return_type>`: `|<parameter_name>: <type>, ...| -> <return_type> { <statements> }`
> `let my_function: fn(my_parameter: bool) -> bool = |my_parameter: bool| -> bool { return my_parameter; };`

### Option
`option<<type>>`: `<value>`/`none`
> `let my_optional: option<bool> = none;`

## Global
### Declare imports
`import <name>;`
> `import my_module;`

### Declare types
`type <name> = <type>;`
> `type my_type = (bool, bool);`

### Declare constants
`const <name> = <value>;` 
> `const my_constant = "This is my constant";`

### Declare functions
`fn <name>(<parameter_name>: <type>, ...) -> <return_type> { <statements> }` **(You can declare entry functions by adding the keyword `entry` like so: `entry fn ...`)**
> `fn sum(a: int, b:int) -> int { return a + b; }`

## Expressions
### Reference
`<identifier>`
> `my_reference`

### Scope
`{ <statements> }`
> `{ let my_value = true; }`

### Unary
`<value>`
> `true`

### Binary
`<expression> <operator> <expression>`
> `1 + 1`

#### Operators
- Addition - `+`
- Substraction - `-`
- Multiplication - `*`
- Division - `/`
- Exponentiation - `^`
- Equal - `==`
- Not equal - `!=`
- And - `&&`
- Or - `||`
- Less - `<`
- Less or equal - `<=`
- More - `>`
- More or equal - `>=`

### Function call
`<expression>(<expression>, ...)`
> `my_function(true, 0)`

### Type function
`<expression>.<identifier>(<expression>, ...)`
> `[true].push(false)`

### Access
#### Vector
 `<expression>[<identifier>]`
> `[true, false][0]`
#### Tuple
 `<expression>.<identifier>`
> `(true, 0).1`

## Statements
### Expressions
Allows you to write an expression
### Declarations
- Specific type: `let <identifier>: <type> = <value>;`
    > `let my_value_1: int = 0;`
- Inferred type: `let <identifier> = <value>;`
    > `let my_value_2 = 1;`

### Assign
`<identifier> = <value>;`
> `my_value = "New value";`

### Condition
`if <expression> <statement> else <statement>` **(`else` clause isn't mandatory)**
> `if my_value_1 > my_value_2 my_function() else my_other_function()`

### Loop
`loop <expression> <statement>`
> `loop my_value_1 > my_value_2 my_function()`

### For/In
`for <identifier> in <expression> <statement>`
> `for i in my_vector my_other_function(i)`

### Continue
`continue;`

### Break
`break;`

### Return
`return <expression>;` **(`expression` isn't required)**
> `return 0;`

## Type methods
### Boolean
- `switch` -> `bool` - returns the opposite value

### Integer
- `to_string` -> `string`
- `to_float` -> `float`
- `abs` -> `int` - returns the absolute value of the given integer

### Float
- `to_string` -> `string`
- `to_int` -> `int`
- `abs` -> `float` - returns the absolute value of the given float

### String
- `to_int` -> `option<int>`
- `to_float` -> `option<float>`
- `to_vec` -> `vec<char>`

### Vector
- `len` -> `int` - returns the lenght of `vec`
- `push(value)` - pushes new element to vector
- `insert(position, value)` - insert element at position
- `remove(position)` - remove element at position

### Option
- `is_some` -> `bool` - check if has value
- `unwrap` -> `<option_type>`  - returns optional value. **Panics if it's none!**
