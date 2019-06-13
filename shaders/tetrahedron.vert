#version 450
layout(location = 0) in vec3 in_position;
layout(location = 1) in vec3 in_colour;
layout(location = 0) out vec3 out_colour;

layout(set = 0, binding = 0) uniform Projection {
  mat4 u_Camera;
};

layout(set = 0, binding = 1) uniform Translate {
  mat4 u_Rotation;
};

void main() {
  //gl_Position = u_Camera * vec4(in_position, 1.0);
  gl_Position = u_Camera * (u_Rotation * vec4(in_position, 1.0));
  gl_Position.z = 0.5 * (gl_Position.z + gl_Position.w);
  out_colour = in_colour;
}
