use async_trait::async_trait;

#[async_trait]
pub trait Plugin: Send + Sync {
    async fn name(&self) -> String;
    async fn execute(&self, input: &str) -> String;
}
#[macro_export]
macro_rules! register_plugin {
    ($plugin_type:ty) => {
        #[no_mangle]
        pub extern "C" fn create_plugin() -> Box<dyn $crate::Plugin> {
            Box::new(<$plugin_type>::new())
        }
    };
}
