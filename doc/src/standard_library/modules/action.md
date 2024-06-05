# action

## pickup

```wdl
function pickup(target: Target, events?: Events) -> void
```

**Example**

```wdl
action::pickup(
    target: {
        stations: [
            "myStation"
        ]
    },
    events: {
        no_station_left: order::cancel
    }
)
```

## drop

```wdl
function drop(target: Target, events?: Events) -> void
```

**Example**

```wdl
action::drop(
    target: {
        stationareas: [
            "myArea"
        ],
        not: {
            stations: [
                "myStation"
            ]
        }
    }
)
```

## drive

```wdl
function drive(target: Target, events?: Events) -> void
```

**Example**

```wdl
action::drive(
    target: {
        stations: [
            "myStation"
        ]
    },
    events: {
        no_station_left: log::warn
    }
)
```
