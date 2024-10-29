Have explicit types. The compiler will not infer types, types need to be defined at variable creation time, for passing argument, return value from a function, and all places where values need to be defined. The developer needs to explicitly define the types everywhere.

### Code blocks

You can end an expression with a semi-colon or not. Its this way because the parser checks for a newline character and uses that as to determine the end of an expression. It is only superseded by {curly brackets}.

The parser parses {} as one expression here is an example.

```rust
let foo: u8 = {
	let bar: u8 = 1
	return bar + 3
}
```
Inside the brackets we define a variable bar with the value 1. Then we return the value of the expression bar + 3. Which binds 4 to foo. Bar can only be reference inside the curly brackets and gets dropped at the end of the brackets. Making curly brackets have their own sperate scope.

### Variable definitions

`let` 
Creates an immutable variable. Cannot be redefine or modified in any way.
```rust
// Basic let expressions
let foo: u8 = 5
let bar: [1; u8] = [10]
let person: Person = Person(name: "Jack")
// Changing value of immutable variable not allowed
// This will not compile
foo = 8
bar[0] = 1
person.name = "Jane"
```

`let!`
Create a mutable variable. Able to be redefined and modified.
```rust
// Basic let! expressions
let! foo: u8 = 5
let! bar: [1; u8] = [10]
let! person: Person = Person(name: "Jack")
// Returns an error as `baz` is not modified
let! baz: u8 = 3
// Changing value of mutable variable is allowed
// This will complile
foo = 8
bar[0] = 1
person.name = "Jane"
```

`const`
Creates an immutable global variable. Cannot be redefine or modified in any way.
```rust
// Basic constant expression
const FOO: u8 = 10
// Unable to parse this expression
// Throws a parsing error
let bar: u8 = 5
func main() u8 {
	// Able to access FOO
	return FOO
}
func returnBar() u8 { 
	// Unable to access bar
	return bar
}
```

###  Type Checking

Type checking is going to be required during variable declaration. Not having types declared will result in a compilation error and the program not compiling.

### Writing to files and stdout

```rust
// Prints to stdout
write(stdout, "Hello World!")

// Prints to stderr
write(seterr, "Hello World!")

// Opens a file
let! file = file.open();

// Writes to a file
write(file, "Writing to this file.")

// You have to close the file
file.close()
```
