use crossbeam_channel::{self, Receiver, Sender};

macro_rules! channels_default_impl {
    (
        $(#[$attr:meta])*
        pub struct $name:ident {
        $(
            #[$field_doc:meta]
            pub $field_name:ident: $field_type:ty,
        )*
    }) => {
        $(#[$attr])*
        pub struct $name {
            $(
                #[$field_doc]
                pub $field_name: $field_type,
            )*
        }

        impl $name {
            /// Get the default set of the application's channels
            pub fn default() -> $name {
                $name {
                    $(
                        $field_name: Channel::new(crossbeam_channel::unbounded()),
                    )*
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct Channel<T> {
    pub s: Sender<T>,
    pub r: Receiver<T>,
}

impl<T> Channel<T> {
    fn new((s, r): (Sender<T>, Receiver<T>)) -> Channel<T> {
        Channel { s, r }
    }
}

channels_default_impl! {
    /// A struct providing access to the application's channels
    #[derive(Clone)]
    pub struct Channels {
        /// Channel 1: From Any Window to Main Window
        pub mw: Channel<i32>,
        /// Channel 2: From Main Window to Rewards Edit Window
        pub rewards_edit: Channel<i32>,
        /// Channel 3: Send Coins To Reward in the Rewards Edit Window
        pub rewards_send_coins: Channel<f64>,
        /// Channel 4: Send an Item from the Rewards Edit Window to Rewards
        pub rewards_send_item: Channel<String>,
        /// Channel 5: Receive an Item from the Rewards
        pub rewards_receive_item: Channel<String>,
        /// Channel 6: Receive Coins from an Item
        pub rewards_receive_coins: Channel<f64>,
        /// Channel 7: Receive a Reward from an Item
        pub rewards_receive_reward: Channel<String>,
    }
}
