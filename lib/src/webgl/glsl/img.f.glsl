varying highp vec2 vTexPos;
varying highp vec2 vTexIndex;

void main(void) {
  highp vec4 color = texture2D(vTexIndex, vec2(vTexPos.s, vTexPos.t));
  gl_FragColor = color;
}
