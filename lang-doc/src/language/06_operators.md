# Operators

## Arithmetic operators

The following table shows which operators are implemented for which types.

| Left type | Operator | Right type | Example                             |
| --------- | :------: | ---------- | ----------------------------------- |
| number    |   `+`    | number     | `1 + 2` == `3`                      |
| string    |   `+`    | any        | `"text" + 3` == `"text3"`           |
| array     |   `+`    | array      | `[1, 2] + [3, 4]` == `[1, 2, 3, 4]` |
| array     |   `+`    | any        | `[1, 2] + 3` == `[1, 2, 3]`         |
| number    |   `-`    | number     | `3 - 4` == `-1`                     |
| number    |   `*`    | number     | `3 * -4` == `-12`                   |
| number    |   `/`    | number     | `2 / 4` == `0.5`                    |
| number    |   `%`    | number     | `7 % 3` == `1`                      |

<div class="warning">
<strong>Errors</strong>

All other operator/type combinations raise an error and cancel the order.

Furthermore, if the right value of `/` and `%` is `0`, an error is raised and the order gets canceled.

</div>

## Logical operators

As logical operators `and` and `or` can be used. For that, the left and the right operand are converted to their truth values and the operators are evaluated as follows.

| Left  | Operator | Right | Result |
| ----- | :------: | ----- | ------ |
| false |  `and`   | false | false  |
| false |  `and`   | true  | false  |
| true  |  `and`   | false | false  |
| true  |  `and`   | true  | true   |
| false |   `or`   | false | false  |
| false |   `or`   | true  | true   |
| true  |   `or`   | false | true   |
| true  |   `or`   | true  | true   |

<div class="warning">
<strong>Warning</strong>

Logical operators get _short-circuited_ evaluated, which means if the operator is `and` and the left value is _false_, the right expression is not evaluated at all. Vice versa if the operator is `or` and the left value is _true_, the right expression is not evaluated.

That means that the following expression evaluates to _true_ without canceling the order: `true or order::cancel()`.

</div>

## Relational operators

To compare values, two equality operators and four comparison operators are provided by the language.

### Equality

| Expression | Meaning                 |
| ---------- | ----------------------- |
| `a == b`   | `a` is equal to `b`     |
| `a != b`   | `a` is not equal to `b` |

`a` and `b` can be of any type, but the operators are type strict, which means that `2 == "2"` is _false_. Arrays and objects are compared recursively.

### Comparison

| Expression | Meaning                             |
| ---------- | ----------------------------------- |
| `a < b`    | `a` is less than `b`                |
| `a <= b`   | `a` is less than or equal to `b`    |
| `a > b`    | `a` is greater than `b`             |
| `a > b`    | `a` is greater than or equal to `b` |

Comparison operators should only be used for numbers. Because if either `a` or `b` is not a number, the value of the expression is always _false_.

## Unary operators

| Operator | Type   | Example                  |
| :------: | ------ | ------------------------ |
|   `-`    | number | `-3` == `-3`             |
|   `!`    | any    | `!{ k: "v" }` == `false` |

<div class="warning">
<strong>Error</strong>

All other operator/type combinations raise an error and cancel the order.

</div>

## Null coalescing operator

To set a default value if a value is _null_, the null coalescing operator `??` can be used. The default value can be of any type.

**Example:**

```wdl
1 ?? 2 == 3

null ?? "default" == "default"
```

## Precedence

1. Logical: `and`, and `or`
2. Relational: `==`, `!=`, `<`, `<=`, `>`, and `>=`
3. Additive: `+`, and `-`
4. Multiplicative: `*`, `/`, and `%`
5. Unary: `-`, and `!`, `<-`
6. Null coalescing: `??`
7. Offset, Member, Call: `[]`, `.`, and `()`
8. Values and variables: `23`, `"test"`, `var`, ...

Except unary operators, multiple operators on the same precedence level are evaluated left-associative. Unary operators are evaluated right-associative.

**Example:**

```wdl
1 + 2 + 3               ==   ((1 + 2) + 3)

1 + 2 * 3               ==   (1 + (2 * 3))

1 + 2 == 3 or -4 >= 6   ==   (((1 + 2) == 3) or ((-4) <= 6))

-4 ?? "default"         ==   (-(4 ?? "default"))

test()[2].key           ==   ((((test)())[2]).key)

-<-var ?? 5             ==   (-(<-(var ?? 5)))
```
