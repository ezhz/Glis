
use std::{ops::Deref, rc::Rc};
use super::{bindings, traits};

// ------------------------------------------------------------

#[derive(Clone)]
pub struct FunctionPointers(Rc<bindings::Gl>);

impl FunctionPointers
{    
    pub fn load<F>(pointer_loader: F) -> Self
    where F: FnMut(&'static str) -> *const std::ffi::c_void
    {
        let pointers = bindings::Gl::load_with(pointer_loader);
        // This is enabled by default on some hardware,
        // disabling it for consistency
        unsafe{pointers.Disable(bindings::FRAMEBUFFER_SRGB)}
        Self(Rc::new(pointers))
    }
}

impl Deref for FunctionPointers
{
    type Target = bindings::Gl;
    fn deref(&self) -> &Self::Target
    { 
        &self.0
    }
}

impl FunctionPointers
{
    pub fn viewport
    (
        &self,
        origin: [bindings::GLint; 2],
        resolution: [bindings::GLsizei; 2]
    ) -> ()
    {
        unsafe
        {
            self.Viewport
            (
                origin[0],
                origin[1],
                resolution[0],
                resolution[1]
            )
        }
    }

    pub fn clear(&self, mask: bindings::GLbitfield) -> ()
    {
        unsafe{self.Clear(mask)}
    }

    pub fn draw_elements<const N: usize, D: traits::IndicesDataType>
    (
        &self,
        mode: bindings::GLenum,
        indices: &[D; N]
    ) -> ()
    {
        unsafe
        {
            self.DrawElements
            (
                mode,
                N as _,
                D::TYPE_ENUM,
                indices.as_ptr() as _
            )
        }   
    }

    pub fn active_texture(&self, unit: bindings::GLenum) -> ()
    {
        unsafe
        {
            self.ActiveTexture
            (
                bindings::TEXTURE0 + unit
            )
        }
    }

    pub fn bind_default_framebuffer(&self) -> ()
    {
        unsafe
        {
            self.BindFramebuffer
                (bindings::FRAMEBUFFER, 0)
        }
    }

    pub fn read_framebuffer<D>
    (
        &self,
        origin: [i32; 2],
        resolution: [u32; 2],
        channels: super::impls::ChannelCount
    ) -> super::error::OGLResult<Vec<D>>
    where
        D: super::traits::TextureComponentDataType
            + super::traits::Zero
            + Clone
    {
        self.clear_errors();
        let size =
        (
            resolution[0] * resolution[1]
            * u8::from(channels) as u32
        ) as usize;
        let mut data: Vec<D> = Vec::with_capacity(size);
        unsafe
        {
            self.ReadPixels
            (
                origin[0], origin[1],
                resolution[0] as _, resolution[1] as _,
                channels.into(),
                D::TYPE_ENUM,
                data.as_mut_ptr() as _
            );
            data.set_len(size)
        };
        self.get_error()?;
        Ok(data)
    }
}

