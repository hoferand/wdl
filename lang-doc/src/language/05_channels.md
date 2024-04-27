# Channels

In addition to variables that hold values permanently, channels can be used as synchronized queues that hold a value only until it is consumed. If the channel is empty, a read operation blocks the execution of the workflow until a value is sent over the channel. If the buffer is full, a send operation blocks until there is space for another element in the buffer.

## Creation

To create a new channel, the [`channel::new`](../standard_library/modules/channel.md#new) function from the standard library can be used.

**Example:**

```wdl
actions {
    // This creates a channel with a buffer for 3 values.
    let ch = channel::new(3);
}
```

## Send

Before a value can be received from a channel, a value has to be sent over this channel. For that, the `<-` operator can be used.

```wdl
actions {
    let ch = channel::new(3);

    ch <- 4;
    ch <- 5;

    // `ch` holds the values 4 and 5
}
```

## Receive

To receive an already sent value from a channel, the `<-` operator can be used as a unary operator.

```wdl
actions {
    let ch = channel::new(3);

    ch <- 4;
    ch <- 5;

    logs::info(<-ch); // logs `4`
    logs::info(<-ch); // logs `5`
    logs::info(<-ch); // blocks until another value is sent along `ch`
}
```
