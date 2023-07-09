
use std::
{
    marker::PhantomData,
    ops::Index
};
use ogl::*;
use super::timeline::*;

// ------------------------------------------------------------

struct QuadVertices(VertexBufferObject);

impl QuadVertices
{
    fn new(pointers: &FunctionPointers) -> OGLResult<Self>
    {
        let vbo = VertexBufferObject::new(pointers);
        vbo.bind();
        vbo.fill
        (
            &[
                -1.0, -1.0,
                -1.0,  1.0,
                 1.0,  1.0,
                 1.0, -1.0
            ]
        );
        Ok(Self(vbo))
    }

    fn pointers(&self) -> &FunctionPointers
    {
        self.0.pointers()
    }

    fn bind(&self) -> ()
    {
        self.0.bind()
    }
}

// ------------------------------------------------------------

struct QuadProgram
{
    program: Program,
    vao: VertexArrayObject
}

impl QuadProgram
{
    fn new
    (
        vertices: &QuadVertices,
        fragment_code: &str
    ) -> OGLResult<Self>
    {
        let pointers = vertices.pointers();
        let program = Program::link
        (
           pointers,
           &[
                &Shader::compile
                (
                    &pointers,
                    VERTEX_SHADER,
                    "
                    #version 100
                    attribute vec2 corners;
                    varying vec2 st;
                    void main()
                    {
                        gl_Position = vec4(corners, 0.0, 1.0);
                        st = corners * 0.5 + 0.5;
                    }
                    \0"
                )?,
                &Shader::compile
                (
                    &pointers,
                    FRAGMENT_SHADER,
                    &format!("{fragment_code}\0"),
                )?
           ]
        )?;
        let location = program.location
        (
            &"corners",
            LocationOf::Attribute
        )?;
        let vao = VertexArrayObject::new(pointers);
        vao.bind();
        vertices.bind();
        vao.attach_buffer::<f64>(location as _, 2)?;
        Ok(Self{program, vao})
    }

    fn pointers(&self) -> &FunctionPointers
    {
        self.program.pointers()
    }

    fn use_program(&self) -> ()
    {
        self.program.r#use()
    }

    fn bind_vao(&self) -> ()
    {
        self.vao.bind()
    }
    
    fn set_uniform<D>(&self, name: &str, data: D) -> OGLResult<()>
    where D: ToUniform
    {
        let location = self.program.location
        (
            name,
            LocationOf::Uniform
        )?;
        data.to_uniform
        (
            self.pointers(),
            location
        );
        Ok(())
    }

    fn draw(&self) -> ()
    {
        self.pointers().draw_elements
        (
            TRIANGLES,
            &[0u8, 1, 2, 0, 3, 2]
        )
    }
}

// ------------------------------------------------------------

struct BlitterProgram(QuadProgram);

impl BlitterProgram
{
    fn new(vbo: &QuadVertices, unit: GLint) -> OGLResult<Self>
    {    
        let quad = QuadProgram::new
        (
            vbo,
            "
            #version 330 core
            in vec2 st;
            out vec4 color;
            uniform sampler2D image;
            void main()
            {
                color = texture
                (
                    image,
                    vec2(st.x, st.y)
                );
            }
            "
        )?;
        quad.use_program();
        quad.set_uniform("image", unit)?;
        Ok(Self(quad))
    }

    fn use_program(&self) -> ()
    {
        self.0.use_program()
    }

    fn bind_vao(&self) -> ()
    {
        self.0.bind_vao()
    }

    fn blit(&self) -> ()
    {
        self.0.draw()
    }
}

// ------------------------------------------------------------

struct ColorBuffer<D>
{
    framebuffer: FramebufferObject,
    texture: Texture,
    data: PhantomData<D>
}

impl<D> ColorBuffer<D>
{
    const MIMAP_FILTER: Option<InterpolationType>
        = Some(InterpolationType::Nearest);

    fn new
    (
        pointers: &FunctionPointers,
        resolution: [u32; 2]
    ) -> OGLResult<Self>
    where D: TextureComponentDataType
    {
        let framebuffer = FramebufferObject::new(pointers);
        framebuffer.bind();
        let texture = Texture::new(pointers);
        texture.bind();
        texture.setup
        (
            None,
            InterpolationType::Nearest,
            InterpolationType::Nearest,
            Self::MIMAP_FILTER
        );
        let image = Image::<D>
        {
            data: None,
            resolution,
            channels: ChannelCount::Four
        };
        texture.fill(image, Self::MIMAP_FILTER.is_some());
        framebuffer.attach_color(&texture)?;
        let this = Self
        {
            framebuffer,
            texture,
            data: PhantomData 
        };
        Ok(this)
    }

    fn pointers(&self) -> &FunctionPointers
    {
        self.framebuffer.pointers()
    }

    fn bind_framebuffer(&self) -> ()
    {
        self.framebuffer.bind()
    }

    fn bind_texture(&self) -> ()
    {
        self.texture.bind()
    }
}

// ------------------------------------------------------------

struct ColorBuffers<D, const N: usize>
{
    list: [ColorBuffer<D>; N],
    cursor: usize
}

impl<D, const N: usize> Index<usize> for ColorBuffers<D, N>
{
    type Output = ColorBuffer<D>;
    fn index(&self, index: usize) -> &Self::Output
    {
        &self.list[index]
    }
}

impl<D, const N: usize> ColorBuffers<D, N>
{
    fn new
    (
        pointers: &FunctionPointers,
        resolution: [u32; 2]
    ) -> OGLResult<Self>
    where D: TextureComponentDataType
    {
        assert_ne!(N, 0);
        let mut list = vec!();
        for _ in 0..N
        {
            list.push
            (
                ColorBuffer::<D>::new
                (
                    pointers,
                    resolution
                )?
            )
        }
        let this = Self
        {
            list: list.try_into()
                .ok().unwrap(),
            cursor: 0
        };
        Ok(this)
    }

    fn pointers(&self) -> &FunctionPointers
    {
        self[0].pointers()
    }

    fn cursor(&self) -> usize
    {
        self.cursor
    }

    fn next(&mut self) -> ()
    {
        self.cursor = (self.cursor + 1) % N
    }

    fn reset(&mut self) -> ()
    {
        for buffer in &self.list
        {
            buffer.bind_framebuffer();
            self.pointers().clear(COLOR_BUFFER_BIT)
        }
        self.cursor = 0
    }
}

// ------------------------------------------------------------

pub struct NamedTexture
{
    texture: Texture,
    name: String
}

impl NamedTexture
{
    const MIMAP_FILTER: Option<InterpolationType>
        = None;

    pub fn new<D>
    (
        pointers: &FunctionPointers,
        image: Image<D>,
        name: &str
    ) -> Self
    where D: TextureComponentDataType
    {
        let texture = Texture::new(pointers);
        texture.bind();
        texture.setup
        (
            None,
            InterpolationType::Nearest,
            InterpolationType::Nearest,
            Self::MIMAP_FILTER
        );
        texture.fill(image, Self::MIMAP_FILTER.is_some());
        Self
        {
            texture,
            name: name.to_string()
        }
    }

    fn pointers(&self) -> &FunctionPointers
    {
        self.texture.pointers()
    }

    fn bind_texture(&self) -> ()
    {
        self.texture.bind()
    }

    fn name(&self) -> &str
    {
        &self.name
    }
}

// ------------------------------------------------------------

struct Sampler
{
    texture: NamedTexture,
    unit: GLenum
}

impl Sampler
{
    fn new(texture: NamedTexture, unit: GLenum) -> Self
    {
        Self{texture, unit}
    }

    fn pointers(&self) -> &FunctionPointers
    {
        self.texture.pointers()
    }

    fn activate_unit(&self) -> ()
    {
        self.pointers().active_texture(self.unit)
    }

    fn bind_texture(&self) -> ()
    {
        self.texture.bind_texture()
    }

    fn unit(&self) -> u32
    {
        self.unit
    }
}

// ------------------------------------------------------------

#[derive(Clone, Copy)]
struct Viewport
{
    origin: [i32; 2],
    resolution: [u32; 2]
}

impl Viewport
{
    fn new<O, R>(origin: O, resolution: R) -> Self
    where
        O: Into<[i32; 2]>,
        R: Into<[u32; 2]>
    {
        Self
        {
            origin: origin.into(),
            resolution: resolution.into()
        }
    }

    fn set(&self, pointers: &FunctionPointers) -> ()
    {
        pointers.viewport
        (
            self.origin,
            [
                self.resolution[0] as _,
                self.resolution[1] as _
            ]
        )
    }
}

// ------------------------------------------------------------

struct SimpleCanvas<D>
{
    programs: (QuadProgram, BlitterProgram),
    samplers: Vec<Sampler>,
    colorbuffer: ColorBuffer<D>,
    resolution: [u32; 2]
}

impl<D> SimpleCanvas<D>
{
    fn new
    (
        vertices: &QuadVertices,
        code: &str,
        textures: Vec<NamedTexture>,
        resolution: [u32; 2]
    ) -> OGLResult<Self>
    where D: TextureComponentDataType
    {
        let main = QuadProgram::new(&vertices, code)?;
        main.use_program();
        let mut samplers = vec!();
        for (index, textures) in textures.into_iter().enumerate()
        {
            main.set_uniform(textures.name(), index as GLint)?;
            samplers.push(Sampler::new(textures, index as _))
        }
        let blitter = BlitterProgram::new(&vertices, 0)?;
        let colorbuffer = ColorBuffer::new
        (
            vertices.pointers(),
            resolution
        )?;
        let this = Self
        {
            programs: (main, blitter),
            samplers,
            colorbuffer,
            resolution
        };
        Ok(this)
    }

    fn pointers(&self) -> &FunctionPointers
    {
        self.programs.0.pointers()
    }

    fn resolution(&self) -> [u32; 2]
    {
        self.resolution
    }

    fn set_uniform<U>(&self, name: &str, data: U) -> OGLResult<()>
    where U: ToUniform
    {
        let (main, _) = &self.programs;
        main.use_program();
        main.set_uniform(name, data)
    }

    fn render(&mut self) -> ()
    {
        let pointers = self.pointers();
        for sampler in &self.samplers
        {
            sampler.activate_unit();
            sampler.bind_texture()
        }
        self.colorbuffer.bind_framebuffer();
        self.pointers().clear(COLOR_BUFFER_BIT);
        let (main, _) = &self.programs;
        main.use_program();
        main.bind_vao();
        Viewport::new
        (
            [0, 0],
            self.resolution
        ).set(pointers);
        main.draw()
    }

    fn blit
    (
        &mut self,
        origin: [i32; 2],
        clear: GLbitfield
    ) -> ()
    {
        let pointers = self.pointers();
        pointers.bind_default_framebuffer();
        pointers.clear(clear);
        Viewport::new
        (
            origin,
            self.resolution
        ).set(pointers);
        pointers.active_texture(0);
        self.colorbuffer.bind_texture();
        let (_, blitter) = &self.programs;
        blitter.use_program();
        blitter.bind_vao();
        blitter.blit()
    }
}

// ------------------------------------------------------------

struct FeedbackCanvas<D>
{
    programs: (QuadProgram, BlitterProgram),
    samplers: Vec<Sampler>,
    colorbuffers: ColorBuffers<D, 2>,
    resolution: [u32; 2]
}

impl<D> FeedbackCanvas<D>
{
    fn new
    (
        vertices: &QuadVertices,
        code: &str,
        textures: Vec<NamedTexture>,
        resolution: [u32; 2]
    ) -> OGLResult<Self>
    where D: TextureComponentDataType
    {
        let main = QuadProgram::new(&vertices, code)?;
        main.use_program();
        main.set_uniform("previous", 0 as GLint)?;
        let mut samplers = vec!();
        for (index, texture) in textures.into_iter().enumerate()
        {
            let unit = index + 1;
            main.set_uniform(texture.name(), unit as GLint)?;
            samplers.push(Sampler::new(texture, unit as _))
        }
        let blitter = BlitterProgram::new(&vertices, 0)?;
        let colorbuffers = ColorBuffers::new
        (
            vertices.pointers(),
            resolution
        )?;
        let this = Self
        {
            programs: (main, blitter),
            samplers,
            colorbuffers,
            resolution
        };
        Ok(this)
    }

    fn pointers(&self) -> &FunctionPointers
    {
        self.programs.0.pointers()
    }

    fn reset(&mut self) -> ()
    {
        self.colorbuffers.reset()
    }

    fn resolution(&self) -> [u32; 2]
    {
        self.resolution
    }

    fn set_uniform<U>(&self, name: &str, data: U) -> OGLResult<()>
    where U: ToUniform
    {
        let (main, _) = &self.programs;
        main.use_program();
        main.set_uniform(name, data)
    }

    fn render(&mut self) -> ()
    {
        self.colorbuffers.next();
        for sampler in &self.samplers
        {
            assert_ne!(sampler.unit(), 0);
            sampler.activate_unit();
            sampler.bind_texture()
        }
        let pointers = self.pointers();
        let cursor = self.colorbuffers.cursor();
        self.colorbuffers[cursor].bind_framebuffer();
        self.pointers().clear(COLOR_BUFFER_BIT);
        pointers.active_texture(0);
        self.colorbuffers[1 - cursor].bind_texture();
        let (main, _) = &self.programs;
        main.use_program();
        main.bind_vao();
        Viewport::new
        (
            [0, 0],
            self.resolution
        ).set(pointers);
        main.draw()
    }

    fn blit
    (
        &mut self,
        origin: [i32; 2],
        clear: GLbitfield
    ) -> ()
    {
        let pointers = self.pointers();
        pointers.bind_default_framebuffer();
        pointers.clear(clear);
        Viewport::new
        (
            origin,
            self.resolution
        ).set(pointers);
        pointers.active_texture(0);
        let cursor = self.colorbuffers.cursor();
        self.colorbuffers[cursor].bind_texture();
        let (_, blitter) = &self.programs;
        blitter.use_program();
        blitter.bind_vao();
        blitter.blit()
    }
}

// ------------------------------------------------------------

enum CanvasKind<D>
{
    Simple(SimpleCanvas<D>),
    Feedback(FeedbackCanvas<D>)
}

impl<D> From<SimpleCanvas<D>> for CanvasKind<D>
{
    fn from(simple: SimpleCanvas<D>) -> Self
    {
        Self::Simple(simple)
    }
}

impl<D> From<FeedbackCanvas<D>> for CanvasKind<D>
{
    fn from(feedback: FeedbackCanvas<D>) -> Self
    {
        Self::Feedback(feedback)
    }
}

impl<D> CanvasKind<D>
{
    fn new
    (
        vertices: &QuadVertices,
        code: &str,
        textures: Vec<NamedTexture>,
        resolution: [u32; 2],
        feedback: bool
    ) -> OGLResult<Self>
    where D: TextureComponentDataType
    {
        match feedback
        {
            true => FeedbackCanvas
                ::new(vertices, code, textures, resolution)
                .map(Into::into),
            false => SimpleCanvas
                ::new(vertices, code, textures, resolution)
                .map(Into::into)
        }
    }

    fn resolution(&self) -> [u32; 2]
    {
        match self
        {
            Self::Simple(simple) =>
                simple.resolution(),
            Self::Feedback(feedback) =>
                feedback.resolution()
        }
    }

    fn set_uniform<U>(&self, name: &str, data: U) -> OGLResult<()>
    where U: ToUniform
    {
        match self
        {
            Self::Simple(simple) => simple
                .set_uniform(name, data),
            Self::Feedback(feedback) => feedback
                .set_uniform(name, data)
        }
    }

    fn render(&mut self) -> ()
    {
        match self
        {
            Self::Simple(simple) =>
                simple.render(),
            Self::Feedback(feedback) =>
                feedback.render()
        }
    }

    fn blit
    (
        &mut self,
        origin: [i32; 2],
        clear: GLbitfield
    ) -> ()
    {
        match self
        {
            Self::Simple(simple) =>
                simple.blit(origin, clear),
            Self::Feedback(feedback) =>
                feedback.blit(origin, clear)
        }
    }
}

// ------------------------------------------------------------

struct Canvas<D>
{
    #[allow(unused)]
    vertices: QuadVertices,
    kind: CanvasKind<D>
}

impl<D> Canvas<D>
{
    fn new
    (
        pointers: &FunctionPointers,
        code: &str,
        textures: Vec<NamedTexture>,
        resolution: [u32; 2],
        feedback: bool
    ) -> OGLResult<Self>
    where D: TextureComponentDataType
    {
        let vertices = QuadVertices::new(pointers)?;
        let kind = CanvasKind::new
        (
            &vertices,
            code,
            textures,
            resolution,
            feedback
        )?;
        Ok(Self{vertices, kind})
    }

    fn pointers(&self) -> &FunctionPointers
    {
        self.vertices.pointers()
    }

    fn resolution(&self) -> [u32; 2]
    {
        self.kind.resolution()
    }

    fn set_uniform<U>(&self, name: &str, data: U) -> OGLResult<()>
    where U: ToUniform
    {
        self.kind.set_uniform(name, data)
    }

    fn render(&mut self) -> ()
    {
        self.kind.render()
    }

    fn blit(&mut self, origin: [i32; 2], clear: GLbitfield) -> ()
    {
        self.kind.blit(origin, clear)
    }
}

// ------------------------------------------------------------

pub struct CanvasPlayer<D>
{
    timeline: Timeline,
    canvas: Canvas<D>
}

impl<D> CanvasPlayer<D>
{
    pub fn new
    (
        pointers: &FunctionPointers,
        timeline: Timeline,
        code: &str,
        textures: Vec<NamedTexture>,
        resolution: [u32; 2],
        feedback: bool
    ) -> OGLResult<Self>
    where D: TextureComponentDataType
    {
        let canvas = Canvas::new
        (
            pointers,
            code,
            textures,
            resolution,
            feedback
        )?;
        Ok(Self{timeline, canvas})
    }

    pub fn pointers(&self) -> &FunctionPointers
    {
        self.canvas.pointers()
    }

    pub fn refresh(&mut self) -> bool
    {
        match self.timeline.next()
        {
            Some(frame) =>
            {
                if frame == 0
                {
                    if let CanvasKind::Feedback(feedback)
                        = &mut self.canvas.kind
                    {
                        feedback.reset()
                    }
                }
                let time = self.timeline.time();
                let _ = self.canvas.set_uniform("time", time);
                let _ = self.canvas.set_uniform("frame", frame as GLint);
                self.canvas.render();
                self.canvas.blit([0, 0], COLOR_BUFFER_BIT);
                true
            }
            None => false
        }
    }

    pub fn resolution(&self) -> [u32; 2]
    {
        self.canvas.resolution()
    }
}

