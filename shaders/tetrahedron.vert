#version 450
layout(location = 0) in vec3 in_position;
layout(location = 1) in vec3 in_colour;
layout(location = 0) out vec3 out_colour;

layout(set = 0, binding = 0) uniform Locals {
  mat4 u_Transform;
};

void main() {
  //gl_Position = vec4(in_position, 1.0);
  gl_Position = u_Transform * vec4(in_position, 1.0);
  gl_Position.z = 0.5 * (gl_Position.z + gl_Position.w);
  out_colour = in_colour;
}
