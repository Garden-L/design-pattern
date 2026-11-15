// TODO: adapt a second external API without exposing its concrete type.
pub trait AppReader { fn read(&self) -> Result<String, String>; }
