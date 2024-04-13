pub trait TEntidade {
    fn id(&self) -> i32;
    fn campos_model() -> Vec<(String, String)>;
}