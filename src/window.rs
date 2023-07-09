
use std::ops::*;
use winit::{window::*, event_loop::*, dpi::*};

// ------------------------------------------------------------

pub use raw_gl_context::Profile as GLProfile;

// ------------------------------------------------------------

pub struct Window(winit::window::Window);

impl Window
{
    pub fn new
    (
        event_loop: &EventLoop<()>,
        builder: impl Fn(WindowBuilder) -> WindowBuilder
    ) -> Result
    <
        Self,
        winit::error::OsError
    >
    {
        let window = builder(WindowBuilder::new())
            .build(event_loop)?;
        Ok(Self(window))
    }
    
    pub fn set_visible(&self, visible: bool) -> ()
    {
        self.0.set_visible(visible)
    }

    pub fn set_size<S: Into<Size>>(&mut self, size: S) -> ()
    {
        self.0.set_inner_size(size)
    }
    
    pub fn drag(&self) -> Result<(), winit::error::ExternalError>
    {
        self.0.drag_window()
    }
}

// ------------------------------------------------------------

#[derive(Debug)]
pub struct GLWindowCreateError(raw_gl_context::GlError);

impl std::fmt::Display for GLWindowCreateError
{
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>)
        -> std::fmt::Result
    {
        write!(formatter, "{:?}", self.0)   
    }
}

impl std::error::Error for GLWindowCreateError {}

// ------------------------------------------------------------

pub struct GLContextConfiguration
{
    pub version: [u8; 2],
    pub profile: GLProfile,
    pub srgb: bool,
    pub double_buffer: bool,
    pub vsync: bool
}

impl Default for GLContextConfiguration
{
    fn default() -> Self
    {
        Self
        {
            version: [3, 2],
            profile: GLProfile::Core,
            srgb: true,
            double_buffer: true,
            vsync: false
        }
    }
}

impl From<GLContextConfiguration> for raw_gl_context::GlConfig
{
    fn from(configuration: GLContextConfiguration) -> Self
    {
        let [major, minor] = configuration.version;
        Self
        {
            version: (major, minor),
            profile: configuration.profile,
            srgb: configuration.srgb,
            double_buffer: configuration.double_buffer,
            vsync: configuration.vsync,
            ..Default::default()
        }
    }
}

// ------------------------------------------------------------

pub struct GLWindow
{
    inner: Window,
    context: raw_gl_context::GlContext,
    pointers: ogl::FunctionPointers
}

impl Deref for GLWindow
{
    type Target = Window;
    fn deref(&self) -> &Self::Target
    {
        &self.inner
    }
}

impl DerefMut for GLWindow
{
    fn deref_mut(&mut self) -> &mut Self::Target
    {
        &mut self.inner    
    }
}

impl GLWindow
{
    pub fn new
    (
        window: Window,
        configuration: GLContextConfiguration
    ) -> Result<Self, GLWindowCreateError>
    {
        let context = raw_gl_context::GlContext::create
        (
            &window.0,
            configuration.into()
        ).map_err(GLWindowCreateError)?;
        context.make_current();
        let pointers = ogl::FunctionPointers
            ::load(|s| context.get_proc_address(s));
        Ok(Self{inner: window, context, pointers})
    }
    
    pub fn context(&self) -> &raw_gl_context::GlContext
    {
        &self.context
    }

    pub fn pointers(&self) -> &ogl::FunctionPointers
    {
        &self.pointers
    }
}

