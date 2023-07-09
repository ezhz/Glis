
use std::fmt;
use super::{bindings::*, pointers::*};

// ------------------------------------------------------------

#[derive(Debug)]
pub enum OGLError
{
    ShaderCompilation(String),
    ProgramLinking(String),
    AttributeCreate(String),
    AttributeNotFound(String),
    UniformNotFound(String),
    FramebufferCreation(GLenum),
    GL(GLenum)
}

impl std::error::Error for OGLError {}

impl fmt::Display for OGLError
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        match self
        {
            Self::ShaderCompilation(log) => write!(formatter, "{log}"),
            Self::ProgramLinking(log) => write!(formatter, "{log}"),
            Self::AttributeCreate(description) => write!(formatter, "{description}"),
            Self::AttributeNotFound(name) => write!
            (
                formatter,
                "Named attribute variable `{name}` is not an active attribute in the specified
                program object or name starts with the reserved prefix `gl_`"
            ),
            Self::UniformNotFound(name) => write!
            (
                formatter, 
                "Uniform `{name}` does not correspond to an active uniform variable in program or
                name is associated with a named uniform block"
            ),
            Self::FramebufferCreation(status) => write!
            (
                formatter,
                "{}",
                match *status
                {
                    FRAMEBUFFER_UNDEFINED => 
                        "Target is the default framebuffer,
                        but the default framebuffer does not exist",
                    FRAMEBUFFER_INCOMPLETE_ATTACHMENT => 
                        "Some of the framebuffer attachment points are framebuffer incomplete",
                    FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT => 
                        "Framebuffer has to have at least one image attached to it",
                    FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER => 
                        "The value of GL_FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE is GL_NONE for any color
                        attachment point(s) named by GL_DRAW_BUFFERi",
                    FRAMEBUFFER_INCOMPLETE_READ_BUFFER => 
                        "GL_READ_BUFFER is not GL_NONE and the value of
                        GL_FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE is GL_NONE for the color attachment
                        point named by GL_READ_BUFFER",
                    FRAMEBUFFER_UNSUPPORTED => 
                        "The combination of internal formats of the attached images violates an
                        implementation-dependent set of restrictions",
                    FRAMEBUFFER_INCOMPLETE_MULTISAMPLE => 
                        "#1 The value of GL_RENDERBUFFER_SAMPLES is not the same for all attached
                        renderbuffers; or the value of GL_TEXTURE_SAMPLES is the not same for all
                        attached textures; or the attached images are a mix of renderbuffers and
                        textures, the value of GL_RENDERBUFFER_SAMPLES does not match the value of
                        GL_TEXTURE_SAMPLES.
                        #2 The value of GL_TEXTURE_FIXED_SAMPLE_LOCATIONS is not the same for all
                        #attached textures; or the attached images are a mix of renderbuffers and
                        #textures, the value of GL_TEXTURE_FIXED_SAMPLE_LOCATIONS is not GL_TRUE
                        #for all attached textures",
                    FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS => 
                        "Any framebuffer attachment is layered, and any populated attachment is not
                        layered, or all populated color attachments are not from textures of the
                        same target",
                    _ => "Unknown framebuffer error"
                }
            ),
            Self::GL(flag) => write!
            (
                formatter,
                "{}",
                match *flag
                {
                    INVALID_ENUM => "An unacceptable value is specified for an enumerated argument",
                    INVALID_VALUE => "A numeric argument is out of range.",
                    INVALID_OPERATION => "The specified operation is not allowed in the current state",
                    INVALID_FRAMEBUFFER_OPERATION => "The framebuffer object is not complete",
                    OUT_OF_MEMORY => "There is not enough memory left to execute the command",
                    NO_ERROR => "Conflicting error reports",
                    _ => "Unknown OpenGL error"
                }
            )
        }
    }
}

// ------------------------------------------------------------

pub type OGLResult<T, E = OGLError> = std::result::Result<T, E>;

// ------------------------------------------------------------

impl FunctionPointers
{
    pub fn get_error(&self) -> OGLResult<()>
    {    
        match unsafe{self.GetError()}
        {
            NO_ERROR => Ok(()),
            flag @ _ => Err(OGLError::GL(flag))
        }
    }

    pub fn clear_errors(&self) -> ()
    {
        let _ = self.get_error();
    }
}

