# Introduction

Workflow Definition Language (WDL) is a domain-specific language tailored for mobile robotics. Its semantics are based on the pi-calculus, introducing interesting features such as [concurrent execution](language/07_control_structures.md#concurrency) and synchronization [channels](language/05_channels.md). Moreover, it supports well-known concepts from other languages like [JSON compatible values](language/02_values.md), [global and local variables](language/03_variables.md), [functions](language/04_functions.md), [operators](language/06_operators.md), and [control structures](language/07_control_structures.md) like `if-else` statements and `while` loops.

In addition to its core functionality, WDL provides a comprehensive [standard library](standard_library.md). This library offers functions to perform physical [actions](standard_library/modules/action.md) like pickup and drop actions, accessing the web through [HTTP](standard_library/modules/http.md) calls, using [regex](standard_library/modules/regex.md) for searching, finding, and replacing strings, and many more.

## Example

The following is a simple example of how a station-to-station workflow in WDL would look:

```wdl
global source = "mySource";
global destination = "myDestination";

actions {
    action::pickup(
        target: {
            stations: [
                source
            ]
        },
        events: {
            no_station_left: order::cancel
        }
    );

    action::drop(
        target: {
            stations: [
                destination
            ]
        }
    );
}
```
