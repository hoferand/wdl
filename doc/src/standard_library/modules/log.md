# log

<div class="warning">
<strong>Message format</strong>

All messages (<code>msg</code>) are serialized to strings and truncated to 100 characters if longer.

The string representation of messages may change with any release; thus, it should not be utilized as input for other programs.

</div>

## info

```wdl
function info(msg: any) -> void
```

**Example**

```wdl
log::info("Some useful information!")
```

## warn

```wdl
function warn(msg: any) -> void
```

**Example**

```wdl
log::warn({ key: "val" })
```

## error

```wdl
function error(msg: any) -> void
```

**Example**

```wdl
log::error(get_error_msg())
```
