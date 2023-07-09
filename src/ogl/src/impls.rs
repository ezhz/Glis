
use std::ffi::*;
use super::{bindings::*, pointers::*, raii, error::*, traits::*};

// ------------------------------------------------------------

impl raii::VertexBufferObject
{
    pub fn fill<T>(&self, data: &[T]) -> ()
    {
        let pointers = self.pointers();
        unsafe
        {
            pointers.BufferData
            (
                ARRAY_BUFFER,
                (data.len() * std::mem::size_of::<T>()) as _,
                data.as_ptr() as _,
                STATIC_DRAW
            )
        }
    }
}

// ------------------------------------------------------------

impl raii::VertexArrayObject
{
    pub fn attach_buffer<T: AttributeComponentDataType>
    (
        &self,
        location: GLuint,
        components: GLint
    ) -> OGLResult<()>
    {
        assert!((1..5).contains(&components));
        let pointers = self.pointers();
        pointers.clear_errors();
        unsafe
        {
            pointers.EnableVertexAttribArray(location);
            pointers.VertexAttribPointer
            (
                location,
                components,
                T::TYPE_ENUM,
                FALSE, 
                0, 
                0 as _
            )
        }
        pointers.get_error()
    }
}

// ------------------------------------------------------------

impl raii::Shader
{
    pub fn compile
    (
        pointers: &FunctionPointers,
        kind: GLenum,
        code: &str,
    ) -> OGLResult<Self>
    {
        let mut success = 0;
        let shader = Self::new(pointers, kind); 
        unsafe
        {
            pointers.ShaderSource
            (
                *shader, 
                1, 
                [code].as_ptr() as _,
                0 as _
            );
            pointers.CompileShader(*shader);
            pointers.GetShaderiv
            (
                *shader,
                COMPILE_STATUS,
                &mut success
            )
        }
        match success as GLboolean
        {
            TRUE => Ok(shader),
            _ =>
            {
                let mut log_len: GLint = 0;
                unsafe
                {
                    pointers.GetShaderiv
                    (
                        *shader,
                        INFO_LOG_LENGTH,
                        &mut log_len
                    )
                }
                let error_message = match log_len
                {
                    0 => String::from("Unknown shader compilation error"),
                    _ =>
                    {
                        let mut log = vec![0u8; (log_len - 1) as usize];
                        unsafe
                        {
                            pointers.GetShaderInfoLog
                            (
                                *shader, 
                                log_len,
                                0 as _,
                                log.as_mut_ptr() as _
                            )
                        }
                        String::from_utf8(log).unwrap()
                    }
                };
                Err(OGLError::ShaderCompilation(error_message))
            }
        }
    }
}

// ------------------------------------------------------------

pub enum LocationOf
{
    Attribute,
    Uniform
}

// ------------------------------------------------------------

impl raii::Program
{
    pub fn link
    (
        pointers: &FunctionPointers,
        shaders: &[&raii::Shader]
    ) -> OGLResult<Self>
    {
        let program = Self::new(pointers);
        let mut success = 0;
        unsafe
        {
            shaders.iter().for_each
            (
                |shader| pointers.AttachShader(*program, ***shader)
            );
            pointers.LinkProgram(*program);
            shaders.iter().for_each
            (
                |shader| pointers.DetachShader(*program, ***shader)
            );
            pointers.GetProgramiv(*program, LINK_STATUS, &mut success)
        }
        match success as GLboolean
        {
            TRUE => Ok(program),
            _ =>
            {
                let mut log_len: GLint = 0;
                unsafe
                {
                    pointers.GetProgramiv
                    (
                        *program, 
                        INFO_LOG_LENGTH,
                        &mut log_len
                    )
                }
                let error_message = match log_len
                {
                    0 => String::from("Unknown program linking error"),
                    _ =>
                    {
                        let mut log = vec![0u8; (log_len - 1) as usize];
                        unsafe
                        {
                            pointers.GetProgramInfoLog
                            (
                                *program, 
                                log_len, 
                                0 as _,
                                log.as_mut_ptr() as _
                            )
                        }
                        String::from_utf8(log).unwrap()
                    }
                };
                Err(OGLError::ProgramLinking(error_message))
            }
        }
    }

    pub fn location(&self, name: &str, location: LocationOf) -> OGLResult<GLint>
    {
        let cname = CString::new((name).clone()).unwrap();
        unsafe
        {
            match location
            {
                LocationOf::Attribute => match self.pointers()
                    .GetAttribLocation(**self, cname.as_ptr())
                {
                    -1 => Err
                    (
                        OGLError::AttributeNotFound(name.to_string())
                    ),
                    location => Ok(location)
                }
                LocationOf::Uniform => match self.pointers()
                    .GetUniformLocation(**self, cname.as_ptr())
                {
                    -1 => Err
                    (
                        OGLError::UniformNotFound(name.to_string())
                    ),
                    location => Ok(location)
                }
            }
        }
    }
}

// ------------------------------------------------------------

#[derive(Clone, Copy)]
pub enum ChannelCount
{
    One,
    Two,
    Three,
    Four
}

impl From<ChannelCount> for GLenum
{
    fn from(count: ChannelCount) -> Self
    {
        match count
        {
            ChannelCount::One => RED,
            ChannelCount::Two => RG,
            ChannelCount::Three => RGB,
            ChannelCount::Four => RGBA
        }
    }
}

impl From<ChannelCount> for u8
{
    fn from(count: ChannelCount) -> Self
    {
        match count
        {
            ChannelCount::One => 1,
            ChannelCount::Two => 2,
            ChannelCount::Three => 3,
            ChannelCount::Four => 4
        }
    }
}

// ------------------------------------------------------------

pub struct Image<'data, D>
{
    pub data: Option<&'data Vec<D>>,
    pub resolution: [u32; 2],
    pub channels: ChannelCount
}

// ------------------------------------------------------------

#[non_exhaustive]
#[derive(Clone, Copy)]
pub enum WrapMode
{
    Repeat,
    MirroredRepeat,
    ClampToEdge
}

// ------------------------------------------------------------

#[derive(Clone, Copy)]
pub enum InterpolationType
{
    Nearest,
    Linear
}

// ------------------------------------------------------------

impl raii::Texture
{
    pub fn setup
    (
        &self,
        wrap_mode: Option<WrapMode>,
        minification_filter: InterpolationType,
        magnification_filter: InterpolationType,
        mimap_filter: Option<InterpolationType> // **
    ) -> ()
    {
        use {InterpolationType::*, WrapMode::*};
        let pointers = self.pointers();
        unsafe
        {
            if let Some(wrap_mode) = wrap_mode
            {
                let wrap_mode = match wrap_mode
                {
                    Repeat => REPEAT,
                    MirroredRepeat => MIRRORED_REPEAT,
                    ClampToEdge => CLAMP_TO_EDGE
                };
                pointers.TexParameteri(TEXTURE_2D, TEXTURE_WRAP_S, wrap_mode as _);
                pointers.TexParameteri(TEXTURE_2D, TEXTURE_WRAP_T, wrap_mode as _);
            };
            pointers.TexParameteri
            (
                TEXTURE_2D, 
                TEXTURE_MIN_FILTER,
                match mimap_filter
                {
                    Some(mimap_filter) => match (minification_filter, mimap_filter)
                    {
                        (Nearest, Nearest) => NEAREST_MIPMAP_NEAREST,
                        (Nearest, Linear) => NEAREST_MIPMAP_LINEAR,
                        (Linear, Nearest) => LINEAR_MIPMAP_NEAREST,
                        (Linear, Linear) => LINEAR_MIPMAP_LINEAR
                    }
                    None => match minification_filter
                    {
                        Nearest => NEAREST,
                        Linear => LINEAR
                    }
                } as _
            );
            pointers.TexParameteri
            (
                TEXTURE_2D, 
                TEXTURE_MAG_FILTER, 
                match magnification_filter
                {
                    Linear => LINEAR,
                    Nearest => NEAREST
                } as _
            )
        }
    }

    pub fn fill<D: TextureComponentDataType>
    (
        &self,
        image: Image<D>,
        mipmap: bool // **
    ) -> ()
    {
        let pointers = self.pointers();
        unsafe
        {
            pointers.TexImage2D
            (
                TEXTURE_2D,
                0,
                match D::TYPE_ENUM
                {
                    UNSIGNED_BYTE | BYTE => RGBA8,
                    UNSIGNED_SHORT | SHORT => RGBA16,
                    UNSIGNED_INT | INT | FLOAT => RGBA32F,
                    _ => unreachable!()
                } as _,
                image.resolution[0] as _,
                image.resolution[1] as _,
                0,
                image.channels.into(),
                D::TYPE_ENUM,
                match image.data
                {
                    Some(data) => data.as_ptr() as _,
                    None => 0 as _
                }
            );
            if mipmap
            {
                pointers.GenerateMipmap(TEXTURE_2D)
            }
        }
    }
}

// ------------------------------------------------------------

impl raii::FramebufferObject
{
    pub fn attach_color(&self, texture: &raii::Texture) -> OGLResult<()>
    {
        let pointers = self.pointers();
        pointers.clear_errors();
        unsafe
        {
            pointers.FramebufferTexture2D
            (
                FRAMEBUFFER,
                COLOR_ATTACHMENT0,
                TEXTURE_2D,
                **texture, 
                0
            )
        }
        match unsafe{pointers.CheckFramebufferStatus(FRAMEBUFFER)}
        {
            FRAMEBUFFER_COMPLETE => Ok(()),
            0 => Err(pointers.get_error().unwrap_err()),
            status @ _ => Err
            (
                OGLError::FramebufferCreation(status)
            )
        }
    }
}

