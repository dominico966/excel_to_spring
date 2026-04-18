use std::path::Path;
use crate::interface_definition::InterfaceDefinition;

pub trait InterfaceDefinitionReader {
    fn read(&self, path: &Path) -> InterfaceDefinition;
}