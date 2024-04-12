pub trait TEntidade {
    fn id(&self) -> i32;
    fn campos_model(&self) -> Vec<String>;
}