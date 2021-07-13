/// Consume the first argument in exchange of unit
macro_rules! unit {
    ($_t:tt $unit:expr) => {
        $unit
    };
}

/// Create events using provided identifiers
macro_rules! events {
    ( $first_event:ident $(,)? $($other_events:ident),* ) => {
            pub const $first_event: i32 = 1000 + <[()]>::len(&[$(unit!(($other_events) ())),*]) as i32;
            events!($($other_events),*);
    };
    () => {};
}

events!(START_TIMER, TICK, STOP_TIMER);
