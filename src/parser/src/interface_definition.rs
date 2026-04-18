use std::path::Path;
use tracing::instrument;
use crate::traits::InterfaceDefinitionReader;
use crate::types::InterfaceDefinitionFileType;

pub struct InterfaceDefinition {
    pub(crate) interface_id: String,
    pub(crate) requirement_id: String,
}

impl InterfaceDefinition {

    #[instrument]
    pub fn from(path : &Path, interface_definition_file_type: &InterfaceDefinitionFileType) -> Self {
        let path_str = path
            .to_str()
            .expect("Path should be valid UTF-8");



        Self {
            interface_id: String::from("logic test"),
            requirement_id: String::from("logic test"),
        }
    }
}

impl InterfaceDefinitionReader for InterfaceDefinitionFileType {

    #[instrument]
    fn read(&self, path: &Path) -> InterfaceDefinition {
        InterfaceDefinition::from(&path, &self)
    }
}