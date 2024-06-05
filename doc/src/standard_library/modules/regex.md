# regex

## match

```wdl
function match(regex: string, haystack: string) -> bool
```

**Example**

```wdl
regex::match("\\d", "abc123") // returns `true`

regex::match("\\d", "abc") // returns `false`
```

## find

```wdl
function find(regex: string, haystack: string) -> [string]
```

**Example**

```wdl
regex::find("a\\d", "a a3 c a45") // returns `["a3", "a4"]`
```

## replace

```wdl
function replace(regex: string, haystack: string, replace: string) -> string
```

**Example**

```wdl
regex::replace("a\\d", "a a3 c a45", "b") // returns `"a b c b5"`
```
