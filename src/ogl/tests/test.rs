
mod functions; use functions::*;

// ------------------------------------------------------------

fn program_compiles() -> ()
{
    #[allow(unused_variables)]
    let (window, context, pointers) = init();
    fragment_shader_100(&pointers);
    vertex_shader_100(&pointers);
}

// ------------------------------------------------------------

fn program_does_not_compile() -> ()
{
    #[allow(unused_variables)]
    let (_, context, pointers) = init();
    assert!
    (
        Shader::compile
        (
            &pointers, 
            FRAGMENT_SHADER,
            &"nil\0"
        ).map(|_|()).unwrap_err()
        .to_string()
        .contains("C0000")
    )
}

// ------------------------------------------------------------

fn program_links() -> ()
{
    #[allow(unused_variables)]
    let (_, context, pointers) = init();
    program_100(&pointers);
}

// ------------------------------------------------------------

fn program_does_not_link() -> ()
{
    #[allow(unused_variables)]
    let (_, context, pointers) = init();
    assert!
    (
        Program::link
        (
            &pointers, 
            &[
                &Shader::compile
                (
                    &pointers,
                    VERTEX_SHADER,
                    &"\0"
                ).unwrap(),
                &fragment_shader_100(&pointers)
            ]
        ).map(|_|()).unwrap_err()
        .to_string()
        .contains("C5145")
    )
}

// ------------------------------------------------------------

fn render_pixel() -> ()
{
    #[allow(unused_variables)]
    let (window, context, pointers) = init();
    #[allow(unused_variables)]
    let (program, vao, corners, indices) = color_quad
    (
        &pointers,
        (0.5, 0.5, 0.5, 0.5)
    );
    program.r#use();
    let origin = [0, 0];
    let resolution = [1, 1];
    pointers.viewport(origin, resolution);
    pointers.draw_elements(TRIANGLES, &indices);
    let output_pixel = pointers.read_framebuffer::<u8>
    (
        origin,
        [resolution[0] as _, resolution[1] as _],
        ChannelCount::One
    ).unwrap();
    assert_eq!(output_pixel, [127])
}

// ------------------------------------------------------------

fn write_read_texture() -> ()
{
    #[allow(unused_variables)]
    let (_, context, pointers) = init();
    let input_pixel: Vec<u8> = vec![123, 111, 66, 211];
    #[allow(unused_variables)]
    let texture = texture_8bit(&pointers, Some(&input_pixel));
    #[allow(unused_variables)]
    let (program, vao, corners, indices) = quad
    (
        &pointers,
        &"
        #version 460
        out vec4 color;
        uniform sampler2D t;
        void main()
        {
            color = texture(t, vec2(0.5));
        }
        \0"
    );
    let (origin, resolution) = ([0, 0], [1, 1]);
    program.r#use();
    pointers.viewport(origin, resolution);
    pointers.draw_elements(TRIANGLES, &indices);
    let output_pixel = pointers.read_framebuffer::<u8>
    (
        origin,
        [resolution[0] as _, resolution[1] as _],
        ChannelCount::Four
    ).unwrap();
    assert_eq!(input_pixel, output_pixel)
}

// ------------------------------------------------------------

fn write_read_multiple_texture() -> ()
{
    #[allow(unused_variables)]
    let (_, context, pointers) = init();
    let sampler_names = ["t", "u"];
    #[allow(unused_variables)]
    let (program, vao, corners, indices) = quad
    (
        &pointers,
        &"
        #version 460
        out vec4 color;
        uniform sampler2D t;
        uniform sampler2D u;
        void main()
        {
            color = vec4
            (
                texture(t, vec2(0.5)).x,
                texture(u, vec2(0.5)).x,
                0,
                1
            );
        }
        \0"
    );
    program.r#use();
    let step = 51;
    let mut textures = vec![];
    for i in 1..=2
    {
        pointers.active_texture(i as _);
        let texture = texture_8bit
        (
            &pointers, 
            Some(&vec![step * i; 4])
        );
        let location = program.location
        (
            &sampler_names[(i as usize) - 1],
            LocationOf::Uniform
        ).unwrap();
        (i as GLint).to_uniform(&pointers, location);
        textures.push(texture)
    }
    let expected_value = vec![step, step * 2];
    let (origin, resolution) = ([0, 0], [1, 1]);
    pointers.viewport(origin, resolution);
    pointers.draw_elements(TRIANGLES, &indices);
    let output_value = pointers.read_framebuffer
    (
        origin,
        [resolution[0] as _, resolution[1] as _],
        ChannelCount::Two
    ).unwrap();
    assert_eq!(expected_value, output_value)
}

// ------------------------------------------------------------

fn location_found() -> ()
{
    #[allow(unused_variables)]
    let (_, context, pointers) = init();
    let (program, ..) = quad
    (
        &pointers,
        &"
        #version 100
        precision mediump float;
        uniform int u;
        void main()
        {
            gl_FragColor = vec4(u);
        }
        \0"
    );
    program.location
        ("corners", LocationOf::Attribute).unwrap();
    program.location
        ("u", LocationOf::Uniform).unwrap();
}

// ------------------------------------------------------------

fn location_not_found() -> ()
{
    #[allow(unused_variables)]
    let (_, context, pointers) = init();
    let program = program_100(&pointers);
    program.location
        ("nil", LocationOf::Attribute).unwrap_err();
    program.location
        (&"nil", LocationOf::Uniform).unwrap_err();
}

// ------------------------------------------------------------

fn write_read_uniform() -> ()
{
    #[allow(unused_variables)]
    let (_, context, pointers) = init();
    let input_values: [f32; 3] = [1.0, 2.0, 3.0];
    let mut output_value: [f32; 3] = [0.0, 0.0, 0.0];
    let (program, ..) = quad
    (
        &pointers,
        &"
        #version 100
        precision mediump float;
        uniform vec3 u;
        void main()
        {
            gl_FragColor = vec4(u, 1.0);
        }
        \0"
    );
    let location = program.location
        (&"u", LocationOf::Uniform).unwrap();
    program.r#use();
    input_values.to_uniform(&pointers, location);
    unsafe // **
    {
        pointers.GetUniformfv
        (
            *program, 
            location, 
            (&mut output_value) as _
        )
    }    
    assert_eq!(output_value, input_values)
}

// ------------------------------------------------------------

fn render_to_framebuffer() -> ()
{
    #[allow(unused_variables)]
    let (_, context, pointers) = init();
    #[allow(unused_variables)]
    let (program, vao, corners, indices) = color_quad
    (
        &pointers,
        (0.0, 0.2, 0.4, 0.6)
    );
    let texture = texture_8bit(&pointers, None);
    let fbo = FramebufferObject::new(&pointers);
    fbo.bind();
    fbo.attach_color(&texture).unwrap();
    fbo.bind();
    program.r#use();
    let (origin, resolution) = ([0, 0], [1, 1]);
    pointers.viewport(origin, resolution);
    pointers.draw_elements(TRIANGLES, &indices);
    texture.bind();
    let output_pixel = pointers.read_framebuffer::<u8>
    (
        origin,
        [resolution[0] as _, resolution[1] as _],
        ChannelCount::Four
    ).unwrap();
    assert_eq!(vec![0, 51, 102, 153], output_pixel)
}

// ------------------------------------------------------------

fn framebuffer_ping_pong() -> ()
{
    #[allow(unused_variables)]
    let (_, context, pointers) = init();
    let step = 0.2;
    #[allow(unused_variables)]
    let (program, vao, corners, indices) = quad
    (
        &pointers,
        &format!
        (
            "
            #version 130
            uniform sampler2D previous;
            void main()
            {{
                gl_FragColor = vec4({step})
                    + texture(previous, vec2(0.5));
            }}
            \0"
        )
    );
    let mut fbos = vec![];
    for _ in 0..2
    {
        let texture = texture_8bit(&pointers, None);
        let fbo = FramebufferObject::new(&pointers);
        fbo.bind();
        fbo.attach_color(&texture).unwrap();
        fbos.push((texture, fbo))
    }
    let frames = 4;
    let expected_output = (step * frames as f32 * 255.0) as u8;
    program.r#use();
    let (origin, resolution) = ([0, 0], [1, 1]);
    pointers.viewport(origin, resolution);
    for frame in 0..frames
    {
        let index = frame % 2;
        fbos[index].0.bind();
        fbos[1 - index].1.bind();
        pointers.draw_elements(TRIANGLES, &indices)
    }
    fbos[frames % 2].0.bind();
    let output_pixel = pointers.read_framebuffer::<u8>
    (
        origin,
        [resolution[0] as _, resolution[1] as _],
        ChannelCount::One
    ).unwrap();
    assert_eq!(expected_output, output_pixel[0])
}

// ------------------------------------------------------------

fn main() -> ()
{
    program_compiles();
    program_does_not_compile();
    program_links();
    program_does_not_link();
    render_pixel();
    write_read_texture();
    write_read_multiple_texture();
    location_not_found();
    location_found();
    write_read_uniform();
    render_to_framebuffer();
    framebuffer_ping_pong()
}

