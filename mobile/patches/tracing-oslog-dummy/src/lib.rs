//! Dummy tracing-oslog implementation for iOS simulator builds
//! This avoids the bindgen issues with the real tracing-oslog crate.

use tracing_core::{Event, Subscriber};

/// Dummy OsLogger that does nothing
pub struct OsLogger;

impl OsLogger {
    pub fn new() -> Self {
        OsLogger
    }
    
    pub fn with_subsystem(self, _subsystem: &str) -> Self {
        self
    }
    
    pub fn with_category(self, _category: &str) -> Self {
        self
    }
}

impl Default for OsLogger {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "subscriber")]
pub mod subscriber {
    use super::*;
    use tracing_subscriber::layer::{Context, Layer};
    
    /// Dummy layer implementation
    pub struct OsLayer;
    
    impl OsLayer {
        pub fn new() -> Self {
            OsLayer
        }
    }
    
    impl Default for OsLayer {
        fn default() -> Self {
            Self::new()
        }
    }
    
    impl<S> Layer<S> for OsLayer
    where
        S: Subscriber,
    {
        fn on_event(&self, _event: &Event<'_>, _ctx: Context<'_, S>) {
            // Do nothing - this is a dummy implementation
        }
    }
}

/// Initialize oslog - dummy implementation that does nothing
pub fn init() {
    // Do nothing
}

/// Create a default oslog layer - dummy implementation
#[cfg(feature = "subscriber")]
pub fn layer() -> subscriber::OsLayer {
    subscriber::OsLayer::new()
}
