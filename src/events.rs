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
    OK_BUTTON_SET_TO_ADD,
    ADD_A_REWARD_OPEN,
    ADD_A_REWARD_SEND_COINS,
    ADD_A_REWARD_SEND_REWARD,
    ADD_A_REWARD_RECEIVE,
    OK_BUTTON_SET_TO_EDIT,
    EDIT_A_REWARD_SEND_ITEM,
    EDIT_A_REWARD_OPEN,
    EDIT_A_REWARD_RECEIVE_COINS,
    EDIT_A_REWARD_RECEIVE_REWARD,
    EDIT_A_REWARD_SEND_COINS,
    EDIT_A_REWARD_SEND_REWARD,
    EDIT_A_REWARD_RECEIVE,
    DELETE_A_REWARD,
    SPEND_COINS_SEND_PRICE,
    SPEND_COINS_RECEIVE_PRICE,
    LOCK_THE_DELETE_A_REWARD_BUTTON,
    UNLOCK_THE_DELETE_A_REWARD_BUTTON,
    LOCK_THE_EDIT_A_REWARD_BUTTON,
    UNLOCK_THE_EDIT_A_REWARD_BUTTON,
    CHECK_AFFORDABILITY_RECEIVE_PRICE,
    LOCK_THE_SPEND_BUTTON,
    UNLOCK_THE_SPEND_BUTTON
);
