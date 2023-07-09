
use super::{bindings::*, pointers::*};

// ------------------------------------------------------------

pub trait IndicesDataType
{
    const TYPE_ENUM: GLenum;
}

impl IndicesDataType for GLubyte
{
    const TYPE_ENUM: GLenum = UNSIGNED_BYTE;
}

impl IndicesDataType for GLushort
{
    const TYPE_ENUM: GLenum = UNSIGNED_SHORT;
}

impl IndicesDataType for GLuint
{
    const TYPE_ENUM: GLenum = UNSIGNED_INT;
}

// ------------------------------------------------------------

pub trait ToUniform
{
    fn to_uniform(&self, pointers: &FunctionPointers, location: GLint) -> ();
}

impl ToUniform for GLfloat
{
    fn to_uniform(&self, pointers: &FunctionPointers, location: GLint) -> ()
    {
        unsafe{pointers.Uniform1fv(location, 1, self)}
    }
}

impl ToUniform for [GLfloat; 2]
{
    fn to_uniform(&self, pointers: &FunctionPointers, location: GLint) -> ()
    {
        unsafe{pointers.Uniform2fv(location, 1, self.as_ptr())}
    }
}

impl ToUniform for [GLfloat; 3]
{
    fn to_uniform(&self, pointers: &FunctionPointers, location: GLint) -> ()
    {
        unsafe{pointers.Uniform3fv(location, 1, self.as_ptr())}
    }
}

impl ToUniform for [GLfloat; 4]
{
    fn to_uniform(&self, pointers: &FunctionPointers, location: GLint) -> ()
    {
        unsafe{pointers.Uniform4fv(location, 1, self.as_ptr())}
    }
}

impl ToUniform for [GLfloat; 9]
{
    fn to_uniform(&self, pointers: &FunctionPointers, location: GLint) -> ()
    {
        unsafe{pointers.UniformMatrix3fv(location, 1, FALSE, self.as_ptr())}
    }
}

impl ToUniform for [GLfloat; 16]
{
    fn to_uniform(&self, pointers: &FunctionPointers, location: GLint) -> ()
    {
        unsafe{pointers.UniformMatrix4fv(location, 1, FALSE, self.as_ptr())}
    }
}

impl ToUniform for GLint
{
    fn to_uniform(&self, pointers: &FunctionPointers, location: GLint) -> ()
    {
        unsafe{pointers.Uniform1iv(location, 1, self)}
    }
}

impl ToUniform for [GLint; 2]
{
    fn to_uniform(&self, pointers: &FunctionPointers, location: GLint) -> ()
    {
        unsafe{pointers.Uniform2iv(location, 1, self.as_ptr())}
    }
}

impl ToUniform for [GLint; 3]
{
    fn to_uniform(&self, pointers: &FunctionPointers, location: GLint) -> ()
    {
        unsafe{pointers.Uniform3iv(location, 1, self.as_ptr())}
    }
}

impl ToUniform for [GLint; 4]
{
    fn to_uniform(&self, pointers: &FunctionPointers, location: GLint) -> ()
    {
        unsafe{pointers.Uniform4iv(location, 1, self.as_ptr())}
    }
}

impl ToUniform for GLuint
{
    fn to_uniform(&self, pointers: &FunctionPointers, location: GLint) -> ()
    {
        unsafe{pointers.Uniform1uiv(location, 1, self)}
    }
}

impl ToUniform for [GLuint; 2]
{
    fn to_uniform(&self, pointers: &FunctionPointers, location: GLint) -> ()
    {
        unsafe{pointers.Uniform2uiv(location, 1, self.as_ptr())}
    }
}

impl ToUniform for [GLuint; 3]
{
    fn to_uniform(&self, pointers: &FunctionPointers, location: GLint) -> ()
    {
        unsafe{pointers.Uniform3uiv(location, 1, self.as_ptr())}
    }
}

impl ToUniform for [GLuint; 4]
{
    fn to_uniform(&self, pointers: &FunctionPointers, location: GLint) -> ()
    {
        unsafe{pointers.Uniform4uiv(location, 1, self.as_ptr())}
    }
}

// ------------------------------------------------------------

pub trait Zero
{
    const ZERO: Self;
}

impl Zero for GLubyte
{
    const ZERO: Self = 0;
}

impl Zero for GLbyte
{
    const ZERO: Self = 0;
}

impl Zero for GLushort
{
    const ZERO: Self = 0;
}

impl Zero for GLshort
{
    const ZERO: Self = 0;
}

impl Zero for GLuint
{
    const ZERO: Self = 0;
}

impl Zero for GLint
{
    const ZERO: Self = 0;
}

impl Zero for GLfloat
{
    const ZERO: Self = 0.0;
}

// ------------------------------------------------------------

pub trait AttributeComponentDataType
{
    const TYPE_ENUM: GLenum;
}

impl AttributeComponentDataType for GLubyte
{
    const TYPE_ENUM: GLenum = UNSIGNED_BYTE;
}

impl AttributeComponentDataType for GLbyte
{
    const TYPE_ENUM: GLenum = BYTE;
}

impl AttributeComponentDataType for GLushort
{
    const TYPE_ENUM: GLenum = UNSIGNED_SHORT;
}

impl AttributeComponentDataType for GLshort
{
    const TYPE_ENUM: GLenum = SHORT;
}

impl AttributeComponentDataType for GLuint
{
    const TYPE_ENUM: GLenum = UNSIGNED_INT;
}

impl AttributeComponentDataType for GLint
{
    const TYPE_ENUM: GLenum = INT;
}

impl AttributeComponentDataType for GLfloat
{
    const TYPE_ENUM: GLenum = FLOAT;
}

impl AttributeComponentDataType for GLdouble
{
    const TYPE_ENUM: GLenum = DOUBLE;
}

// ------------------------------------------------------------

pub trait TextureComponentDataType
{
    const TYPE_ENUM: GLenum;
}

impl TextureComponentDataType for GLubyte
{
    const TYPE_ENUM: GLenum = UNSIGNED_BYTE;
}

impl TextureComponentDataType for GLbyte
{
    const TYPE_ENUM: GLenum = BYTE;
}

impl TextureComponentDataType for GLushort
{
    const TYPE_ENUM: GLenum = UNSIGNED_SHORT;
}

impl TextureComponentDataType for GLshort
{
    const TYPE_ENUM: GLenum = SHORT;
}

impl TextureComponentDataType for GLuint
{
    const TYPE_ENUM: GLenum = UNSIGNED_INT;
}

impl TextureComponentDataType for GLint
{
    const TYPE_ENUM: GLenum = INT;
}

impl TextureComponentDataType for GLfloat
{
    const TYPE_ENUM: GLenum = FLOAT;
}

