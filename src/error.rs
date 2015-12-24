#[derive(Debug)]
pub enum Error {
    WindowCreationError(::glutin::CreationError),
    OpenGLContextError(::glutin::ContextError),
}
