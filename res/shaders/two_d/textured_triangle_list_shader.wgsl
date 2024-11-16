struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
}
struct InstanceInput {
    @location(2) model_matrix_0: vec4<f32>,
    @location(3) model_matrix_1: vec4<f32>,
    @location(4) model_matrix_2: vec4<f32>,
    @location(5) model_matrix_3: vec4<f32>,
    @location(6) tint: vec4<f32>,
    @location(7) highlight: vec4<f32>,
}

struct Camera {
    view_projection: mat4x4<f32>,
}
@group(1) @binding(0)
var<uniform> camera: Camera;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) tint: vec4<f32>,
    @location(2) highlight: vec4<f32>,
}

@vertex
fn vs_main(
    model: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    let model_matrix = mat4x4<f32>(
        instance.model_matrix_0,
        instance.model_matrix_1,
        instance.model_matrix_2,
        instance.model_matrix_3
    );

    let world_position = model_matrix * vec4<f32>(model.position, 1.0);
    let perspective_position = camera.view_projection * world_position;
    let z = (perspective_position.z + 1.0) / 2.0;

    var out: VertexOutput;
    
    out.clip_position = vec4<f32>(perspective_position.xy, z, 1.0);
    out.tex_coords = model.tex_coords;
    out.tint = instance.tint;
    out.highlight = instance.highlight;
    
    return out;
}

@group(0) @binding(0)
var obj_texture: texture_2d<f32>;
@group(0) @binding(1)
var obj_sampler: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let object_color = textureSample(obj_texture, obj_sampler, in.tex_coords);
    return (vec4<f32>(object_color) * in.tint + in.highlight);
}