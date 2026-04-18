#[derive(Debug)]
pub enum InterfaceDefinitionFileType {
    XLSX,
}

impl InterfaceDefinitionFileType {
    pub fn extension(path: &str) -> Option<Self> {
        match path.to_ascii_lowercase() {
            s if s.ends_with("xlsx") => Some(Self::XLSX),
            _ => None
        }
    }
}