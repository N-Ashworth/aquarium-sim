#version 330 core

layout (location = 0) in vec3 aPos;

uniform vec2 position;
uniform float direction;

uniform vec2 cam_scale;

void main() {
    //rotate by direction then offset by position
    float theta = atan(aPos.y, aPos.x) - direction;
    float mag = length(aPos.xy);

    vec2 pos = vec2(cos(theta), sin(theta)) * mag + position;
    pos /= cam_scale;
    gl_Position = vec4(pos, 0.0, 1.0);
}