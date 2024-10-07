// From https://github.com/glium/glium/blob/master/examples/tutorial-02.rs

#version 140

out vec4 color;

uniform float iTime;
uniform vec3 iResolution;

void mainImage( out vec4 fragColor, in vec2 fragCoord )
{
    // Normalized pixel coordinates (from 0 to 1)
    vec2 uv = fragCoord/iResolution.xy;

    // Time varying pixel color
    vec3 col = 0.5 + 0.5*cos(iTime+uv.xyx+vec3(0,2,4));

    // Output to screen
    fragColor = vec4(col,1.0);
}

void main() {
    mainImage(color, gl_FragCoord.xy);
}

