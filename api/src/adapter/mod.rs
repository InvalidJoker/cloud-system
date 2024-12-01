use async_trait::async_trait;

#[async_trait]
pub trait Adapter: Send + Sync {
    async fn name(&self) -> String;
    async fn create_server(&self, name: &str) -> String;
}
#[macro_export]
macro_rules! register_adapter {
    ($adapter_type:ty) => {
        #[no_mangle]
        pub extern "C" fn create_adapter() -> Box<dyn Adapter> {
            Box::new(<$adapter_type>::new())
        }
    };
}
