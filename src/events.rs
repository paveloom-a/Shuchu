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

events!(
    START_TIMER,
    TICK,
    STOP_TIMER,
    ADD_A_REWARD_OPEN,
    ADD_A_REWARD_SEND_COINS,
    ADD_A_REWARD_SEND_REWARD,
    ADD_A_REWARD_RECEIVE,
    ADD_A_REWARD_RESET_COINS,
    ADD_A_REWARD_RESET_REWARD
);
