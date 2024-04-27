# Values

The language provides five data types analogous to JSON: _null_, _bool_, _number_, _string_, _array_, and _object_.

## Syntax

### Null

Null values represent the absence of values.

**Example:**

```wdl
null
```

### Bool

Bool values represent truth values; they can be either _true_ or _false_. Mostly used for conditions.

**Example:**

```wdl
false

true
```

### Number

Numbers are represented as IEEE 754 floating-point numbers with 64-bit precision.

**Example:**

```wdl
0

23

34.5

-45.6
```

### Strings

Strings can be used to save text; they fully support Unicode code points as UTF-8 encoded. Strings start and end with `"`. `'` and `` ` `` are not allowed. Furthermore, escaped characters can be used: `\\` to use a backslash `\`, `\n` to add a line break, and `\"` to use `"` inside strings. Multiline strings are also supported.

**Example:**

```wdl
"text"

"first line\nsecond line"

"multi
line
string"
```

### Array

Arrays are sequences of values with arbitrary length. The elements can be of any type. An array starts with `[` and ends with `]`, the elements are separated by a `,`.

**Example:**

```wdl
[1, 2, 3]

["first", 2, null]

[
    1,
    [1, 2],
    {
        key: "value",
        key2: 2
    }
]
```

### Object

Objects are collections of named values. They start with a `{` and end with a `}`, the key and the value are separated by a `:`, and the key can be either an identifier or a string. The key-value pairs are separated by a `,` like in arrays.

<div class="warning">
<strong>Warning</strong>

The key `wdl_type` is reserved for internal use. Using this key may lead to unintended behavior.

</div>

**Example:**

```wdl
{
    key: 23,
    "my key": [
        1,
        2,
    ],
    key3: {
        inner_key: "text".
        inner_key2: 34
    }
}
```

## Access Operators

### Offset operator

To access a specific element of a string, array, or object, the offset operator `[]` can be used.

| Type   | Example                        |
| ------ | ------------------------------ |
| string | `"text"[1 + 1] == "x"`         |
| array  | `[1, 2, 3][1] == 2`            |
| object | `{ "k 1": "v" }["k 1"] == "v"` |

<div class="warning">
<strong>Warning</strong>

Offsets on strings and arrays are 0-based. So the first element is at offset 0 and the second element is at offset 1.

</div>

### Member operator

The member operator is a special form of the offset operator used to access members of an object. It has the restriction that only values associated with an identifier can be accessed, not those associated with a string key.

**Example:**

```wdl
{
    key: "value",
    "key 1": 23
}.key == "value"
```

To access `"key 1"`, the offset operator has to be used.

## Truth value

Each of the above values can be used as a condition, which means each of these values has a truth value _true_/_false_.

| Value  | Truth value                              | Example                                                  |
| ------ | ---------------------------------------- | -------------------------------------------------------- |
| null   | _false_                                  | `null` == _false_                                        |
| bool   | _false_ if `false` <br> _true_ if `true` | `false` == _false_ <br> `true` == _true_                 |
| number | _false_ if `0` <br> _true_ otherwise     | `0` == _false_ <br> `34` == _true_ <br> `-0.5` == _true_ |
| string | _false_ if `""` <br> _true_ otherwise    | `""` == _false_ <br> `"text"` == _true_                  |
| array  | _false_ if `[]` <br> _true_ otherwise    | `[]` == _false_ <br> `[1, 2]` == _true_                  |
| object | _false_ if `{}` <br> _true_ otherwise    | `{}` == _false_ <br> `{ k: "v" }` == _true_              |
