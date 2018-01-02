#version 330

in vec3 position;
in vec4 color;
in vec2 texCoord;

uniform vec2 translation = vec2(0, 0);
uniform vec2 scale = vec2(1, 1);
uniform uint layers = uint(1);

out FragData {
    vec3 position;
    vec4 color;
    vec2 texCoord;
} frag;

void main() {
    vec2 newPosition = (position.xy + translation) * scale;
    float z = 0.99 - 1.98 * position.z / float(layers);

    gl_Position = vec4(newPosition, z, 1);

    frag.position = position;
    frag.color = color;
    frag.texCoord = texCoord;
}
