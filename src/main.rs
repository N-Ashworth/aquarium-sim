use glint::{Window, Shader, Mesh};
use tobj::Model;

fn load_obj(obj_file: &str) -> Vec<Model> {
    let (models, _) = tobj::load_obj(
        obj_file,
        &tobj::LoadOptions {
            triangulate: true,
            ..Default::default()
        },
    ).expect("Failed to load OBJ file");

    models
}

struct Boid {
    position: (f32, f32),
    direction: f32,
}

fn main() {
    //WINDOWS AND MESHES
    let mut app = Window::new(600, 600, "Aquarium Simulation");

    let shader = Shader::new(r"src\shader.vert", r"src\shader.frag");
    shader.bind();

    let fish_obj = load_obj("fish.obj")[0].clone();
    let fish_mesh = Mesh::new(fish_obj.mesh.positions, fish_obj.mesh.indices, 3);

    let cam_scale = 10.0;

    //GLOBAL VARIABLES
    let mut boids: Vec<Boid> = vec![
        Boid {
            position: (0.0, 0.0),
            direction: 0.0,
        }
    ];

    let mut last_time = 0.0;

    while app.running() {
        //DELTATIME
        let current_time = app.time();
        let delta_time = current_time - last_time;

        app.poll_events();

        //SIMULATION
        for b in &mut boids {
            b.direction += delta_time;
            b.position.1 += delta_time * 0.1;
        }

        //RENDERING
        app.clear_with_color(0.1, 0.1, 0.1);

        shader.set_vec2("cam_scale", [app.width as f32 / 600.0 * cam_scale, app.height as f32 / 600.0 * cam_scale]);

        for b in &boids {
            shader.set_vec2("position", [b.position.0, b.position.1]);
            shader.set_float("direction", b.direction);

            fish_mesh.draw();
        }

        app.swap_buffers();

        last_time = current_time;
    }
}