

pub mod render {

    pub trait Renderer {
        fn create(name: &str) -> impl Renderer;
    }
}