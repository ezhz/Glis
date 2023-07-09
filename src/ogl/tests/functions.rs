
pub use ogl::*;

// ------------------------------------------------------------

pub fn init() ->
(
    winit::window::Window,
    raw_gl_context::GlContext,
    FunctionPointers
)
{    
    let event_loop = winit::event_loop::EventLoop::new();
    let window = winit::window::WindowBuilder
        ::new().build(&event_loop).unwrap();
    let context = raw_gl_context::GlContext::create
    (
        &window,
        raw_gl_context::GlConfig
        {
            version: (3, 1),
            profile: raw_gl_context::Profile::Core,
            ..Default::default()
        }
    ).unwrap();
    context.make_current();
    let pointers = FunctionPointers
        ::load(|s| context.get_proc_address(s));
    (window, context, pointers)
}

// ------------------------------------------------------------

pub fn vertex_shader_100(pointers: &FunctionPointers) -> Shader
{
    Shader::compile
    (
        pointers, 
        VERTEX_SHADER,
        "
        #version 100
        void main()
        {
            gl_Position = vec4(1.0);
        }
        \0"
    ).unwrap()
}

// ------------------------------------------------------------

pub fn fragment_shader_100(pointers: &FunctionPointers) -> Shader
{
   Shader::compile
   (
       pointers, 
       FRAGMENT_SHADER,
       "
       #version 100
       precision mediump float;
       void main()
       {
           gl_FragColor = vec4(1.0);
       }
       \0"
   ).unwrap()
}

// ------------------------------------------------------------

pub fn program_100(pointers: &FunctionPointers) -> Program
{
    Program::link
    (
       pointers,
       &[
           &vertex_shader_100(pointers),
           &fragment_shader_100(pointers)
       ]
    ).unwrap()
}

// ------------------------------------------------------------

pub fn quad
(
    pointers: &FunctionPointers,
    fragment_source: &str
) ->
(
    Program,
    VertexArrayObject,
    VertexBufferObject,
    [u32; 6]
)
{
    let program = Program::link
    (
       pointers,
       &[
           &Shader::compile
           (
               &pointers,
               VERTEX_SHADER,
               &"
               #version 100
               attribute vec2 corners;
               void main()
               {
                   gl_Position = vec4(corners, 0.0, 1.0);
               }
               \0"
           ).unwrap(),
           &Shader::compile
           (
               &pointers,
               FRAGMENT_SHADER,
               fragment_source
           ).unwrap()
       ]
    ).unwrap();
    let vao = VertexArrayObject::new(pointers);
    vao.bind();
    let corners = VertexBufferObject::new(pointers);
    corners.bind();
    corners.fill
    (
        &[
            -1.0, -1.0,
            -1.0,  1.0,
             1.0,  1.0,
             1.0, -1.0
        ]
    );
    vao.attach_buffer::<f64>
    (
        program.location
            ("corners", LocationOf::Attribute)
            .unwrap() as _,
        2
    ).unwrap();
    let indices = [0, 1, 2, 0, 3, 2];
    (program, vao, corners, indices)
}

// ------------------------------------------------------------

pub fn color_quad
(
    pointers: &FunctionPointers,
    color: (f32, f32, f32, f32)
) -> 
(
    Program,
    VertexArrayObject,
    VertexBufferObject,
    [u32; 6]
)
{
    quad
    (
        pointers,
        &format!
        (
             "
             #version 460
             out vec4 color;
             void main()
             {{
                 color = vec4{color:?};
             }}
             \0"
         )
    )
}

// ------------------------------------------------------------

pub fn texture_8bit
(
    pointers: &FunctionPointers,
    data: Option<&Vec<u8>>
) -> Texture
{
    let texture = Texture::new(pointers);
    texture.bind();
    texture.setup
    (
        None,
        InterpolationType::Linear,
        InterpolationType::Linear,
        None
    );
    texture.fill
    (
        Image
        {
            data,
            resolution: [1; 2],
            channels: ChannelCount::Four
        },
        false
    );
    texture
}

