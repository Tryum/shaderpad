// From https://github.com/glium/glium/blob/master/examples/tutorial-02.rs

#version 140
in vec2 pos;

void main() {
    gl_Position = vec4(pos, 0.0, 1.0);
}