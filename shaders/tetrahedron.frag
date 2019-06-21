#version 450
layout(location = 0) in vec4 v_Position;
layout(location = 1) in vec3 v_Normal;
layout(location = 2) in vec3 f_Colour;

layout(location = 0) out vec4 o_Colour;

struct Light {
  mat4 projection;
  vec4 position;
  vec4 colour;
};

layout(set = 0, binding = 2) uniform Lights {
  Light u_Lights[2];
};

void main() {
  vec3 normal = normalize(v_Normal);
  vec3 ambient = vec3(0.05, 0.05, 0.05);

  vec3 colour = ambient;
  for(int i = 0; i < 2; ++i) {
    Light light = u_Lights[i];
    vec4 light_local = light.projection * v_Position;
    vec3 light_dir = normalize(light.position.xyz - v_Position.xyz);
    float diffuse = max(0.0, dot(normal, light_dir));
    colour += diffuse * light.colour.xyz;
  }
  
  o_Colour = vec4(colour, 1.0) * vec4(f_Colour, 1.0);
}
