use api::plugin::Plugin;
use libloading::{Library, Symbol};
use std::sync::Arc;

pub struct PluginManager {
    pub plugins: Vec<Arc<dyn Plugin>>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
        }
    }

    pub fn load_plugin(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let lib = Box::leak(Box::new(unsafe { Library::new(path)? }));

        let create_plugin: Symbol<extern "C" fn() -> Box<dyn Plugin>> =
            unsafe { lib.get(b"create_plugin")? };

        let plugin: Box<dyn Plugin> = create_plugin();

        self.plugins.push(Arc::from(plugin));

        Ok(())
    }

    pub fn load_from_directory(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let dir = std::fs::read_dir(path)?;

        for entry in dir {
            let entry = entry?;

            if !entry.file_type()?.is_file() {
                continue;
            }

            let path = entry.path();
            let path = path.to_str().unwrap();

            self.load_plugin(path)?;
        }

        Ok(())
    }
}
