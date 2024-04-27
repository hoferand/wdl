# Functions

Functions can be used to reuse common code at multiple places within a workflow.

## Declaration

Before a function can be used, it has to be declared in the global scope using the `function` keyword. A function can take an arbitrary number of input variables and can return at most one value using the `return` keyword.

**Example:**

```wdl
actions {
    // This raises an error because function
    // declarations are only allowed at global scope.
    function sub(left, right) {
        return left - right;
    }
}

function sum(left, right) {
    return left + right;
}
```

## Usage

After declaring a function, it can be used through a function call. The return value can be used directly in an expression or can be assigned to a variable. Input variables can be either placed in the correct order or can be named. Positional and named arguments can be used in the same call, but first all positional arguments have to be placed, and after that, the named ones can be placed.

**Example:**

```wdl
actions {
    log::info(sub(5, 2)); // logs the value `3`

    let my_sum = sub(right: 4, left: 9); // my_sum now has the value `5`

    sub(7, right: 2);

    // This raises an error because after the
    // named argument `left`, is the positional argument `2`.
    sub(left: 7, 2);

    // Calling a function with too few or
    // too many input variables raises an error.
    sub(1);
    sub(1, 2, 3);
}

function sub(left, right) {
    return left - right;
}
```
