use plugin_system::{DocumentPlugin, PluginRegistry};
use std::{cell::RefCell, rc::Rc};
use structural::proxy::MemoryStore;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let loads = Rc::new(RefCell::new(0));
    let mut registry = PluginRegistry::new();
    registry.register("docs", DocumentPlugin::new(MemoryStore::new(loads.clone())))?;
    println!("{}", registry.run("docs", "guide")?);
    println!("{}", registry.run("docs", "guide")?);
    println!("loads={}", loads.borrow());
    Ok(())
}
