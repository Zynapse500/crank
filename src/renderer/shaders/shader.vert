#version 330

in vec3 position;
in vec4 color;

out FragData {
    vec3 position;
    vec4 color;
} frag;

void main() {
    gl_Position = vec4(position, 1);

    frag.position = position;
    frag.color = color;
}
