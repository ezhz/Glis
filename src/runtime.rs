
use super::{canvas::*, timeline::*};

// ------------------------------------------------------------

pub struct RuntimeSetup
{
    pub resolution: [u32; 2],
    pub feedback: bool,
    pub timeline: Timeline,
    pub textures: Vec<NamedTexture>
}

// ------------------------------------------------------------

pub struct RuntimeErrored<D>(CanvasPlayer<D>);

impl<D> RuntimeErrored<D>
{
    pub fn new<E>
    (
        pointers: &ogl::FunctionPointers,
        error: &E
    )-> ogl::OGLResult<Self>
    where
        D: ogl::TextureComponentDataType,
        E: std::fmt::Display
    {
        eprintln!("\n{error}\n");
        let inner = CanvasPlayer::new
        (
            pointers,
            Timeline::default(),
            super::shaders::ERROR_SHADER,
            vec!(),
            [500; 2],
            false
        )?;
        Ok(Self(inner))
    }

    fn pointers(&self) -> &ogl::FunctionPointers
    {
        self.0.pointers()
    }

    fn refresh(&mut self) -> bool
    {
        self.0.refresh()
    }

    fn resolution(&self) -> [u32; 2]
    {
        self.0.resolution()
    }
}

// ------------------------------------------------------------

pub enum RuntimeState<D>
{
    Running(CanvasPlayer<D>),
    Errored(RuntimeErrored<D>)
}

impl<D> From<CanvasPlayer<D>> for RuntimeState<D>
{
    fn from(state: CanvasPlayer<D>) -> Self
    {
        Self::Running(state)
    }
}

impl<D> From<RuntimeErrored<D>> for RuntimeState<D>
{
    fn from(state: RuntimeErrored<D>) -> Self
    {
        Self::Errored(state)
    }
}

impl<D> RuntimeState<D>
{
    pub fn new(pointers: &ogl::FunctionPointers) // **
        -> ogl::OGLResult<Self>
    where D: ogl::TextureComponentDataType
    {
        RuntimeErrored::new(pointers, &"")
            .map(Into::into)
    }

    pub fn restart
    (
        &mut self,
        code: &str,
        setup: RuntimeSetup
    ) -> ogl::OGLResult<()>
    where D: ogl::TextureComponentDataType
    {
        match CanvasPlayer::<D>::new
        (
            self.pointers(),
            setup.timeline,
            code,
            setup.textures,
            setup.resolution,
            setup.feedback
        )
        {
            Ok(canvas) => Ok(*self = canvas.into()),
            Err(error) => self.into_errored(&error)
        }
    }

    pub fn pointers(&self) -> &ogl::FunctionPointers
    {
        match self
        {
            Self::Running(running) => running.pointers(),
            Self::Errored(errored) => errored.pointers()
        }
    }

    pub fn into_errored<E>(&mut self, error: &E) -> ogl::OGLResult<()>
    where
        D: ogl::TextureComponentDataType,
        E: std::fmt::Display
    {
        RuntimeErrored::new(self.pointers(), error)
            .map(|e| *self = e.into())
    }

    pub fn refresh(&mut self) -> bool
    {
        match self
        {
            Self::Running(running) => running.refresh(),
            Self::Errored(errored) => errored.refresh()
        }
    }
    
    pub fn resolution(&self) -> [u32; 2]
    {
        match self
        {
            Self::Running(running) => running.resolution(),
            Self::Errored(errored) => errored.resolution()
        }
    }
}

