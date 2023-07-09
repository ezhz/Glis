
use std::{path::*, env};
use super::
{
    watcher::*,
    window::*,
    timeline::*,
    canvas::*,
    glsl::*,
    picture::*,
    runtime::*
};
use winit::
{
    event_loop::*,
    dpi::*
};

// ------------------------------------------------------------

impl CodeAnnotations
{
    fn runtime_setup
    (
        &self,
        root: impl AsRef<Path>,
        pointers: &ogl::FunctionPointers
    ) -> anyhow::Result<RuntimeSetup>
    {
        let this = RuntimeSetup
        {
            resolution: self.resolution(),
            feedback: self.feedback(),
            timeline: Timeline::new(self.rate(), self.range()),
            textures:
            {
                let cwd = env::current_dir()?;
                env::set_current_dir(root)?;
                let mut textures = vec!();
                for (name, path) in self.texture_paths()
                {
                    let mut picture = Picture::open(path)?;
                    picture.flipv();
                    let picture: PictureData = picture.try_into()?;
                    let texture = match &picture.pixel_data
                    {
                        PixelData::EightBit(data) => NamedTexture::new
                        (
                            pointers,
                            ogl::Image::<u8>
                            {
                                data: Some(data),
                                resolution: picture.resolution,
                                channels: picture.channels.into()
                            },
                            &name
                        ),
                        PixelData::SixteenBit(data) => NamedTexture::new
                        (
                            pointers,
                            ogl::Image::<u16>
                            {
                                data: Some(data),
                                resolution: picture.resolution,
                                channels: picture.channels.into()
                            },
                            &name
                        ),
                        PixelData::ThirtyTwoBit(data) => NamedTexture::new
                        (
                            pointers,
                            ogl::Image::<f32>
                            {
                                data: Some(data),
                                resolution: picture.resolution,
                                channels: picture.channels.into()
                            },
                            &name
                        )
                    };
                    textures.push(texture)
                }
                env::set_current_dir(cwd)?;
                textures
            }
        };
        Ok(this)
    }
}

// ------------------------------------------------------------

fn init_window(event_loop: &EventLoop<()>) -> anyhow::Result<GLWindow>
{
    let window = Window::new
    (
        &event_loop, |builder| builder
            .with_visible(false)
            .with_title("")
            .with_maximized(false)
            .with_transparent(true)
            .with_window_icon(None)
            .with_decorations(false)
            .with_resizable(false)
    ).map
    (
        |window| GLWindow::new
        (
            window,
            Default::default()
        )
    )??;
    let pointers = window.pointers();
    unsafe
    {
        pointers.PixelStorei(ogl::UNPACK_ALIGNMENT, 1);
        pointers.PixelStorei(ogl::PACK_ALIGNMENT, 1);
        pointers.Disable(ogl::DEPTH_TEST)
    }
    Ok(window)
}

// ------------------------------------------------------------

pub struct App
{
    window: GLWindow,
    watcher: Option<CodeWatcher>,
    runtime: RuntimeState<f32> 
}

impl App
{
    pub fn new
    (
        event_loop: &EventLoop<()>,
        path: impl AsRef<Path>
    ) -> anyhow::Result<Self>
    {
        let window = init_window(event_loop)?;
        let runtime = RuntimeState::new(window.pointers())?;
        let mut this = Self
        {
            window,
            watcher: None,
            runtime
        };
        if let Ok(watcher) = CodeWatcher::new(path)
        {
            this.watcher = Some(watcher);
            this.restart()?
        }
        this.window.set_visible(true);
        this.window.set_size::<PhysicalSize<u32>>
            (this.runtime.resolution().into());
        Ok(this)
    }

    pub fn drag_window(&self)  -> Result<(), winit::error::ExternalError>
    {
        self.window.drag()
    }

    fn restart(&mut self) -> anyhow::Result<()>
    {
        let watcher = self.watcher.as_ref().unwrap();
        match watcher.code()
        {
            Ok(code) => match AnnotatedGLSL::new(&code)
            {
                Ok(code) => match watcher.filepath().parent()
                {
                    Some(root) => match code.annotations()
                        .runtime_setup(root, self.runtime.pointers())
                    {
                        Ok(setup) =>
                        {
                            self.runtime.restart(&code.code(), setup)?;
                            self.window.set_size::<PhysicalSize<u32>>
                                (self.runtime.resolution().into())
                        }
                        Err(error) => self.runtime.into_errored(&error)?
                    }
                    None => self.runtime.into_errored
                        (&"Could not get parent directory")?

                }
                Err(error) => self.runtime.into_errored(&error)?
            }
            Err(error) => self.runtime.into_errored(&error)?
        };
        Ok(())
    }

    pub fn refresh(&mut self) -> anyhow::Result<()>
    {
        if let Some(watcher) = &self.watcher
        {
            match watcher.refresh()
            {
                Ok(true) => self.restart()?,
                Ok(false) => {}
                Err(error) => self.runtime.into_errored(&error)?
            }
        } 
        if self.runtime.refresh()
        {
            self.window.context().swap_buffers()
        }
        Ok(())
    }
}

