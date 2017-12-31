#version 330

in FragData {
       vec3 position;
       vec4 color;
} frag;

out vec4 outColor;

void main() {
    outColor = frag.color;
}
