pub(crate) trait NamedEnum {
    fn get_name(&self) -> &'static str;
    fn from_name(name: &'static str) -> Self;
}
