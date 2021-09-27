# version 450

layout(location = 0) in vec2 texCoordsIn;
layout(location = 0) out vec4 targetColor;

layout(set = 0, binding = 0) uniform texture2D t_diffuse;
layout(set = 0, binding = 1) uniform sampler s_diffuse;

void main() {
    targetColor = texture(sampler2D(t_diffuse, s_diffuse), texCoordsIn);
}