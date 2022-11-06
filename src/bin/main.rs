use raytracer_rs::{camera::Camera, renderer::Renderer, vec3::Vec3, world::World};

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;
const FOV: f32 = 90.0;
const NUM_SAMPLES: u32 = 100;
const MAX_BOUNCES: u32 = 50;

fn main() {
    let camera = Camera::new(
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::Z,
        Vec3::Y,
        FOV,
        HEIGHT,
        WIDTH,
    );

    let world = World::make_metallic_example();

    let renderer = Renderer::new(NUM_SAMPLES, MAX_BOUNCES);
    let img = renderer.render(&camera, &world, WIDTH, HEIGHT);
    img.save("out.png").unwrap();
}
