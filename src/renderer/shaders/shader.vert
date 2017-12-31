#version 330

in vec3 position;
in vec4 color;

uniform vec2 translation = vec2(0, 0);
uniform vec2 scale = vec2(1, 1);

out FragData {
    vec3 position;
    vec4 color;
} frag;

void main() {
    vec2 newPosition = (position.xy + translation) * scale;

    gl_Position = vec4(newPosition, position.z, 1);

    frag.position = position;
    frag.color = color;
}
