# channel

## new

```wdl
function new(buffer: number) -> channel
```

**Example**

```wdl
let ch = channel::new(3);
```

## close

```wdl
function close(chan​nel: channel) -> void
```

**Example**

```wdl
let ch = channel::new(3);

channel::close(ch);
```
