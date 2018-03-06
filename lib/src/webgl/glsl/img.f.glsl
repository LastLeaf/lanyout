varying highp vec2 vTexPos;
uniform sampler2D uSampler;

void main(void) {
  highp vec4 color = texture2D(uSampler, vec2(vTexPos.s, vTexPos.t));
  gl_FragColor = color;
}
