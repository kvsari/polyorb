#version 450
layout(location = 0) in vec4 v_Position;
layout(location = 1) in vec3 v_Normal;
layout(location = 2) in vec3 f_Colour;

layout(location = 0) out vec4 o_Colour;
void main() {
  o_Colour = vec4(f_Colour, 1.0);
}
