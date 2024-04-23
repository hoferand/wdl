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
