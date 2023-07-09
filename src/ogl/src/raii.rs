
use std::ops::Deref;
use super::{bindings::*, pointers::*};

// ------------------------------------------------------------

pub struct VertexBufferObject
{ 
    pointers: FunctionPointers,
    handle: GLuint
}

impl Deref for VertexBufferObject
{
    type Target = GLuint;
    fn deref(&self) -> &Self::Target
    {
        &self.handle
    }
}

impl VertexBufferObject
{
    pub fn new(pointers: &FunctionPointers) -> Self
    {
        let mut handle = 0;
        unsafe{pointers.GenBuffers(1, &mut handle)}
        Self{pointers: pointers.clone(), handle}
    }

    pub fn pointers(&self) -> &FunctionPointers
    {
        &self.pointers
    }
    
    pub fn bind(&self) -> ()
    {
        unsafe{self.pointers.BindBuffer(ARRAY_BUFFER, **self)}
    }
}

impl Drop for VertexBufferObject
{
    fn drop(&mut self) -> ()
    {
        unsafe{self.pointers.DeleteBuffers(1, &**self)}
    }
}

// ------------------------------------------------------------

pub struct VertexArrayObject
{ 
    pointers: FunctionPointers,
    handle: GLuint
}

impl Deref for VertexArrayObject
{
    type Target = GLuint;
    fn deref(&self) -> &Self::Target
    {
        &self.handle
    }
}

impl VertexArrayObject
{
    pub fn new(pointers: &FunctionPointers) -> Self
    {
        let mut handle = 0;
        unsafe{pointers.GenVertexArrays(1, &mut handle)}
        Self{pointers: pointers.clone(), handle}
    }

    pub fn pointers(&self) -> &FunctionPointers
    {
        &self.pointers
    }

    pub fn bind(&self) -> ()
    {
        unsafe{self.pointers.BindVertexArray(**self)}
    }
}

impl Drop for VertexArrayObject
{
    fn drop(&mut self) -> ()
    {
        unsafe{self.pointers.DeleteVertexArrays(1, &**self)}
    }
}

// ------------------------------------------------------------

pub struct Shader
{
    pointers: FunctionPointers,
    handle: GLuint
}

impl Deref for Shader
{
    type Target = GLuint;
    fn deref(&self) -> &Self::Target
    {
        &self.handle
    }
}

impl Shader
{
    pub fn new
    (
        pointers: &FunctionPointers,
        kind: GLenum
    ) -> Self
    {
        Self
        {
            pointers: pointers.clone(),
            handle: unsafe{pointers.CreateShader(kind)}
        }
    }

    pub fn pointers(&self) -> &FunctionPointers
    {
        &self.pointers
    }
}

impl Drop for Shader
{
    fn drop(&mut self) -> ()
    {
        unsafe{self.pointers.DeleteShader(**self)}
    }
}

// ------------------------------------------------------------

pub struct Program
{
    pointers: FunctionPointers,
    handle: GLuint
}

impl Deref for Program
{
    type Target = GLuint;
    fn deref(&self) -> &Self::Target
    {
        &self.handle
    }
}

impl Program
{
    pub fn new(pointers: &FunctionPointers) -> Self
    {
        Self
        {
            pointers: pointers.clone(), 
            handle: unsafe{pointers.CreateProgram()}
        }
    }

    pub fn pointers(&self) -> &FunctionPointers
    {
        &self.pointers
    }
    
    pub fn r#use(&self) -> ()
    {
        unsafe{self.pointers.UseProgram(**self)}
    }
}

impl Drop for Program
{
    fn drop(&mut self) -> ()
    {
        unsafe{self.pointers.DeleteProgram(**self)}
    }
}

// ------------------------------------------------------------

pub struct Texture
{
    pointers: FunctionPointers,
    handle: GLuint
}

impl Deref for Texture
{
    type Target = GLuint;
    fn deref(&self) -> &Self::Target
    {
        &self.handle
    }
}

impl Texture
{
    pub fn new(pointers: &FunctionPointers) -> Self
    {
        let mut handle = 0;
        unsafe{pointers.GenTextures(1, &mut handle)}
        Self{pointers: pointers.clone(), handle}
    }

    pub fn pointers(&self) -> &FunctionPointers
    {
        &self.pointers
    }
    
    pub fn bind(&self) -> ()
    {
        unsafe{self.pointers.BindTexture(TEXTURE_2D, **self)}
    }
}

impl Drop for Texture
{
    fn drop(&mut self) -> ()
    {
        unsafe{self.pointers.DeleteTextures(1, &**self)}
    }
}

// ------------------------------------------------------------

pub struct FramebufferObject
{
    pointers: FunctionPointers,
    handle: GLuint
}

impl Deref for FramebufferObject
{
    type Target = GLuint;
    fn deref(&self) -> &Self::Target
    {
        &self.handle
    }
}

impl FramebufferObject
{
    pub fn new(pointers: &FunctionPointers) -> Self
    {
        let mut handle = 0;
        unsafe{pointers.GenFramebuffers(1, &mut handle)}
        Self{pointers: pointers.clone(), handle}
    }

    pub fn pointers(&self) -> &FunctionPointers
    {
        &self.pointers
    }

    pub fn bind(&self) -> ()
    {
        unsafe{self.pointers.BindFramebuffer(FRAMEBUFFER, **self)}
    }
}

impl Drop for FramebufferObject
{
    fn drop(&mut self) -> ()
    {
        unsafe{self.pointers.DeleteFramebuffers(1, &**self)}
    }
}

