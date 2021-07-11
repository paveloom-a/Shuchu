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
        /// Channel 1: Nothing Yet
        pub nothing: Channel<i32>,
    }
}
