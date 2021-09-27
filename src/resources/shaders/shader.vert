# version 450

layout(location = 0) in vec3 positionIn;
layout(location = 1) in vec3 normalIn;
layout(location = 2) in vec3 tangentIn;
layout(location = 3) in vec2 texCoordsIn;

layout(set = 1, binding = 0)
uniform Uniforms {
    mat4 viewProj;
};

layout(location = 0) out vec2 texCoordsOut;

void main() {
    texCoordsOut = texCoordsIn;
    gl_Position = viewProj * vec4(positionIn, 1.0);
}