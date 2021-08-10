//! This module provides custom events that are handled by the program.
//!
//! The numeric constants are generated using a recursive counting macro.

/// Consume the first argument in exchange of unit
macro_rules! unit {
    ($_t:tt $unit:expr) => {
        $unit
    };
}

/// Create events using provided identifiers
macro_rules! events {
    (
        $(#[$first_doc:meta])*
        $first_event:ident $(,)?
        $(
            $(#[$other_docs:meta])*
            $other_events:ident
        ),*
    ) => {
            $(#[$first_doc])*
            pub const $first_event: i32 = 1000 + <[()]>::len(&[$(unit!(($other_events) ())),*]) as i32;
            events!($(
                $(#[$other_docs])*
                $other_events
            ),*);
    };
    () => {};
}

events!(
    /// Hide the secondary pane (called by the Arrow and Coins buttons in the Focus Pane)
    MAIN_WINDOW_HIDE_THE_PANE,
    /// Show the secondary pane (called by the Arrow and Coins buttons in the Focus Pane)
    MAIN_WINDOW_SHOW_THE_PANE,
    /// Start the timer (called by the Timer button in the Focus Pane)
    START_TIMER,
    /// Tick (meaning, advance the timer by one second) (called by Timer in the Focus Pane)
    TICK,
    /// Stop the timer (called by the Timer button in the Focus Pane)
    STOP_TIMER,
    /// Do the callback of the OK button in the Rewards Edit window (called by the Coins, Reward
    /// widgets, and the OK button in that window)
    OK_BUTTON_DO_CALLBACK,
    /// Set the OK button in the Rewards Edit window to the 'Add a reward' mode (called by the
    /// Add Button in the Rewards' Menubar)
    OK_BUTTON_SET_TO_ADD,
    /// Set the OK button in the Rewards Edit window to the 'Edit the reward' mode (called by the
    /// Edit Button in the Rewards' Menubar)
    OK_BUTTON_SET_TO_EDIT,
    /// Show the Rewards Edit window for adding a new reward (called by the Add Button in the
    /// Rewards' Menubar)
    ADD_A_REWARD_OPEN,
    /// Start a chain of events to create a new reward. Start by sending the input of the
    /// Coins widget in the Rewards Edit window to its sibling, the Reward widget (called by
    /// the OK button in that window, but that signal may be inherited from the input widgets, too)
    ADD_A_REWARD_SEND_COINS,
    /// Combine the number of coins received from the Coins widget in the Rewards Edit window with
    /// input of the Reward widget, thus creating a reward, and send a result to the Rewards'
    /// List (called by the Coins widget as part of the chain of the events)
    ADD_A_REWARD_SEND_REWARD,
    /// Receive a reward from the Rewards Edit window (called by the Reward widget in the Rewards
    /// Edit window as part of the chain of the events)
    ADD_A_REWARD_RECEIVE,
    /// Start a chain of events to open an existing reward in the Rewards Edit window. Send the
    /// currently selected item from the List to the Rewards Edit window, so it's ready to get
    /// edited (called by the Edit Button in the Rewards' Menubar)
    EDIT_THE_REWARD_SEND_ITEM,
    /// Show the Rewards Edit window for editing an existing reward (called by the List as part of
    /// the chain of the events)
    EDIT_THE_REWARD_OPEN,
    /// Receive the price of the reward that's being edited (called by the Rewards Edit window as
    /// part of the chain of the events)
    EDIT_THE_REWARD_RECEIVE_COINS,
    /// Receive the text of the reward that's being edited (called by the Rewards Edit window as
    /// part of the chain of the events)
    EDIT_THE_REWARD_RECEIVE_REWARD,
    /// Start a chain of events to edit the existing (currently selected) reward. Start by sending
    /// the input of the Coins widget in the Rewards Edit window to its sibling, the Reward widget
    /// (called by the OK button in that window, but that signal may be inherited from the input
    /// widgets, too)
    EDIT_THE_REWARD_SEND_COINS,
    /// Combine the number of coins received from the Coins widget in the Rewards Edit window with
    /// input of the Reward widget, thus creating a reward, and send a result to the Rewards'
    /// List (called by the Coins widget as part of the chain of the events)
    EDIT_THE_REWARD_SEND_REWARD,
    /// Receive a reward from the Rewards Edit window (called by the Reward widget in the Rewards
    /// Edit window as part of the chain of the events)
    EDIT_THE_REWARD_RECEIVE,
    /// Delete the selected reward (called by the Delete Button in the Rewards' Menubar)
    DELETE_A_REWARD,
    /// Start a chain of events to spend coins on the selected reward. Start by sending the price
    /// (called by the Spend Button in the Rewards' Menubar)
    SPEND_COINS_SEND_PRICE,
    /// Receive the price of the selected item (called by the List)
    SPEND_COINS_RECEIVE_PRICE,
    /// Handle the Timer button's shortcut (called by the Main Window)
    TIMER_SHORTCUT,
    /// Handle the Rates button's shortcut (called by the Main Window)
    RATES_SHORTCUT,
    /// Handle the Rewards button's shortcut (called by the Main Window)
    REWARDS_SHORTCUT,
    /// Handle the Add Button's shortcut (called by the Main Window)
    ADD_BUTTON_SHORTCUT,
    /// Handle the Delete Button's shortcut (called by the Main Window)
    DELETE_BUTTON_SHORTCUT,
    /// Handle the Edit Button's shortcut (called by the Main Window)
    EDIT_BUTTON_SHORTCUT,
    /// Handle the Spend Button's shortcut (called by the Main Window)
    SPEND_BUTTON_SHORTCUT,
    /// Lock the Delete Button in the Rewards' Menubar (called by the List)
    LOCK_THE_DELETE_A_REWARD_BUTTON,
    /// Unlock the Delete Button in the Rewards' Menubar (called by the List)
    UNLOCK_THE_DELETE_A_REWARD_BUTTON,
    /// Lock the Edit Button in the Rewards' Menubar (called by the List)
    LOCK_THE_EDIT_A_REWARD_BUTTON,
    /// Unlock the Delete Button in the Rewards' Menubar (called by the List)
    UNLOCK_THE_EDIT_A_REWARD_BUTTON,
    /// Check the affordability (meaning, if the user has enough coins) of the reward. This leads
    /// to the Spend button in the Rewards' Menubar getting locked / unlocked (called by the List)
    CHECK_AFFORDABILITY_RECEIVE_PRICE,
    /// Lock the Spend Button in the Rewards' Menubar (called by the List)
    LOCK_THE_SPEND_BUTTON,
    /// Unlock the Spend Button in the Rewards' Menubar (called by the List)
    UNLOCK_THE_SPEND_BUTTON
);
