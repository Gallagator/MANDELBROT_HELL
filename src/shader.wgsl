// Vertex shader

struct CameraUniform {
    top_left: vec2<f32>,
    scale: f32,
    n_iter: i32,
}

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.color = model.color;
    out.clip_position = vec4<f32>(model.position, 1.0);
    return out;
}

// Fragment shader
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let scaled_clip = in.clip_position * camera.scale;
    let position = vec2<f32>(camera.top_left.x + scaled_clip.x, camera.top_left.y - scaled_clip.y);
    return in_mandelbrot(position);
}

fn in_mandelbrot(c: vec2<f32>) -> vec4<f32> {
    var z = c;
    var i: i32 = 0;
    while(dot(z,z) < 4.0 && i < camera.n_iter) {
        z = complex_mult(z, z) + c;
        i += 1;
    }
    
    if(i == camera.n_iter) {
        return vec4(0.0,0.0,0.0,1.0);
    } else {
        var n = (f32(i) + 1.0 - log(log2(dot(z,z)))) / 100.0 * 255.0;
        var r = sin(0.1 * n);
        var g = sin(0.1235 * n + 1.23);
        var b = sin(0.12 * n + 0.23);
        return vec4(r,g,b,1.0);
    }
}

fn complex_mult(p1: vec2<f32>, p2: vec2<f32>) -> vec2<f32> {
    return vec2<f32>(p1.x * p2.x - p1.y * p2.y, p1.y * p2.x + p1.x * p2.y);
}