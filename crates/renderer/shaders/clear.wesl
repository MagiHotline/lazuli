struct State {
    color: vec4f,
    depth: f32,
}

var<push_constant> state: State;

var<private> POSITIONS: array<vec2f, 4> = array<vec2f, 4>(
    vec2f(-1.0, 1.0),
    vec2f(-1.0, -1.0),
    vec2f(1.0, 1.0),
    vec2f(1.0, -1.0),
);

struct VertexOutput {
    @builtin(position) clip: vec4f,
}

@vertex
fn vs_main(
    @builtin(vertex_index) index: u32,
) -> VertexOutput {
    return VertexOutput(vec4f(POSITIONS[index], 0.0, 1.0));
}

struct FragmentOutput {
    @location(0) color: vec4f,
    @builtin(frag_depth) depth: f32,
}

@fragment
fn fs_main(in: VertexOutput) -> FragmentOutput {
    return FragmentOutput(state.color, state.depth);
}
