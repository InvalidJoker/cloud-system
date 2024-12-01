use async_trait::async_trait;

use crate::event::EventManager;

#[async_trait]
pub trait Plugin: Send + Sync {
    async fn name(&self) -> String;
    async fn startup(&self, manager: &mut EventManager);
}
#[macro_export]
macro_rules! register_plugin {
    ($plugin_type:ty) => {
        #[no_mangle]
        pub extern "C" fn create_plugin() -> Box<dyn $crate::plugin::Plugin> {
            Box::new(<$plugin_type>::new())
        }
    };
}
