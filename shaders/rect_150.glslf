#version 150 core

in vec4 v_Color;
out vec4 Target0; // the RenderTarget represented in rust

void main() {
    Target0 = v_Color;
}