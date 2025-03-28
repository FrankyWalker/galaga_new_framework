use raw_window_handle::{HasWindowHandle, HasDisplayHandle};

pub trait WindowHandle: HasWindowHandle + HasDisplayHandle + Send + Sync {}
impl<T: HasWindowHandle + HasDisplayHandle + Send + Sync> WindowHandle for T {}

pub trait WindowApp: std::fmt::Debug {
    fn new(window: Box<dyn WindowHandle>) -> impl std::future::Future<Output = Self> where Self: Sized;
    fn prepare(&mut self, width: u32, height: u32, scale_factor: f32) -> impl std::future::Future<Output = ()>;
    fn render(&mut self) -> impl std::future::Future<Output = ()>;
}
