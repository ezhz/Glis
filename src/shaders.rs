
pub const ERROR_SHADER: &'static str =
"
#version 120
in vec2 st;
uniform float time;
void main()
{
    gl_FragColor = vec4
    (
        cos(st.x * 20.0 + time) + 
        sin(st.y * 20.0 + time),
        0.0, .5, 1.0
    );
}
";

