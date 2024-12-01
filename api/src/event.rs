use std::sync::Arc;

/**
 * Event Handler
 * Modified from: https://github.com/serenity-rs/serenity/blob/d6b9b287d4f72aca511f22163c89ff70264ac3f0/src/client/context.rs
 */
use crate::context::Context;
use async_trait::async_trait;

macro_rules! event_handler {
    ( $(
        $( #[doc = $doc:literal] )*
        $( #[deprecated = $deprecated:literal] )?
        $( #[cfg(feature = $feature:literal)] )?
        $variant_name:ident { $( $arg_name:ident: $arg_type:ty ),* } => async fn $method_name:ident(&self $(, $context:ident: Context)?);
    )* ) => {
        /// The core trait for handling events by serenity.
        #[async_trait]
        pub trait EventHandler: Send + Sync {
            $(
                $( #[doc = $doc] )*
                $( #[cfg(feature = $feature)] )?
                $( #[deprecated = $deprecated] )?
                async fn $method_name(&self, $($context: Context,)? $( $arg_name: $arg_type ),*) {
                    // Suppress unused argument warnings
                    drop(( $($context,)? $($arg_name),* ))
                }
            )*
        }

        /// This enum stores every possible event that an [`EventHandler`] can receive.
        #[non_exhaustive]
        #[allow(clippy::large_enum_variant)] // TODO: do some boxing to fix this
        #[derive(Clone, Debug)]
        pub enum FullEvent {
            $(
                $( #[doc = $doc] )*
                $( #[cfg(feature = $feature)] )?
                $( #[deprecated = $deprecated] )?
                $variant_name {
                    $( $arg_name: $arg_type ),*
                },
            )*
        }

        impl FullEvent {
            /// Returns the name of this event as a snake case string
            ///
            /// ```rust,no_run
            /// # use serenity::client::{Context, FullEvent};
            /// # fn foo_(ctx: Context, event: FullEvent) {
            /// if let FullEvent::Message { .. } = &event {
            ///     assert_eq!(event.snake_case_name(), "message");
            /// }
            /// # }
            /// ```
            #[must_use]
            pub fn snake_case_name(&self) -> &'static str {
                #[allow(deprecated)]
                match self {
                    $(
                        $( #[cfg(feature = $feature)] )?
                        Self::$variant_name { .. } => stringify!($method_name),
                    )*
                }
            }

            /// Runs the given [`EventHandler`]'s code for this event.
            pub async fn dispatch(self, ctx: Context, handler: &dyn EventHandler) {
                #[allow(deprecated)]
                match self {
                    $(
                        $( #[cfg(feature = $feature)] )?
                        Self::$variant_name { $( $arg_name ),* } => {
                            $( let $context = ctx; )?
                            handler.$method_name( $($context,)? $( $arg_name ),* ).await;
                        }
                    )*
                }
            }
        }
    };
}

event_handler! {
    Test { message: String } => async fn test_event(&self, ctx: Context);
}

pub struct EventManager {
    pub handlers: Vec<Arc<dyn EventHandler>>,
}

impl EventManager {
    pub fn new() -> Self {
        Self {
            handlers: Vec::new(),
        }
    }

    pub fn event_handler(&mut self, event_handler: impl EventHandler + 'static) {
        self.handlers.push(Arc::new(event_handler));
    }

    pub async fn dispatch(&self, event: FullEvent, ctx: Context) {
        for handler in &self.handlers {
            event.clone().dispatch(ctx.clone(), handler.as_ref()).await;
        }
    }
}
