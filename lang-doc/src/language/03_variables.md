# Variables

Variables are dynamically typed and can be used to store [values](./01_values.md), [functions](./03_functions.md), and [channels](./07_channels.md) of any type for later reuse. The values of variables can be changed through an assignment operation. The language supports both _global_ and _local_ variables.

## Global

Global variables can be used everywhere in the program, and their value can be set at the workflow start. To use a global variable, it needs to be declared in the outermost scope, for example, above the actions block.

**Example:**

```wdl
global my_variable = 23;

actions {
    my_variable = my_variable + 2;

    // This raises an error because `global`
    // is not allowed inside any scope.
    global another_var = 3;
}
```

## Local

Local variables can only be used within the scope in which they are declared. A local variable can be declared using the `let` keyword. They cannot be declared in the outermost scope.

**Example:**

```wdl
// This raises an error because `let`
// is not allowed on the global scope.
let var1 = 23;

actions {
    let var2 = 3;

    if true {
        var2 = var2 + 2;

        let var3 = 45;

        var3 = 23;
    } // <- `var3` gets deleted at this `}`

    var2 = 78;

    // This raise an error because `var3`
    // is not declared inside this scope.
    var3 = 56;
} // <- `var2` gets deleted at this `{`
```
