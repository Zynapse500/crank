#version 330

in FragData {
       vec3 position;
       vec4 color;
       vec2 texCoord;
} frag;

uniform sampler2D tex0;

out vec4 outColor;


void main() {
    outColor = frag.color * texture(tex0, frag.texCoord);

    if (outColor.a == 0) {
        // Don't render fully transparent pixels to the depth buffer
        gl_FragDepth = 1;
    } else {
        gl_FragDepth = frag.position.z;
    }
}
