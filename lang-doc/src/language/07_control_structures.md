# Control Structures

To skip and repeat actions, and perform tasks in the background, the language provides control structures.

## Conditional Branches

The `if-else` structure allows selecting specific statements based on conditions.

**Example:**

```wdl
actions {
    let var = 3;
    if var < 0 {
        // ...
    } else if var > 10 {
        // ...
    } else {
        // ...
    }
}
```

## Loops

To repeat statements, the `while` loop can be used. The statements inside the loop body are executed as long as the condition holds.

**Example:**

```wdl
actions {
    let var = 5;
    while var >= 0 {
        var = var - 1;
        // ...
    }
}
```

### Continue

To skip the current loop iteration, `continue` statements can be used.

**Example:**

```wdl
actions {
    let arr = [1, 101, 2, 3, 66, 4];
    let sum = 0;
    let idx = 0;
    while idx < 6 {
        if arr[idx] > 10 {
            continue;
        }
        sum = sum + arr[idx];
        idx = idx + 1;
    }
    // now sum has the value `10`
}
```

### Break

To cancel the loop execution, `break` statements can be used.

**Example:**

```wdl
actions {
    let arr = [1, 2, 3, 66, 4];
    let sum = 0;
    let idx = 0;
    while idx < 5 {
        if arr[idx] > 10 {
            break;
        }
        sum = sum + arr[idx];
        idx = idx + 1;
    }
    // now sum has the value `6`
}
```

## Concurrency

To run time-consuming tasks in the background, the `spawn` operator can be used. Calling the spawn operator moves the task on the right side to the background and returns a channel on which the result of the function can be received.

**Example:**

```wdl
actions {
    let res_ch = spawn http::get("example.org/get-target");
    // do something else
    let result = <-res_ch; // this blocks until the response arrives
    // now `result` holds the HTTP response
}
```
