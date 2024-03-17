use super::Particle;

fn add(u: &Vec<f32>, v: &Vec<f32>) -> Vec<f32> {
    u.into_iter().zip(v).map(|(u, v)| u + v).collect()
}

fn scale(a: f32, v: &Vec<f32>) -> Vec<f32> {
    v.into_iter().map(|c| a * c).collect()
}

pub fn step(p: &Particle, h: f32) -> Particle {
    let x = add(&p.position, &scale(h, &p.velocity));
    let v = p.velocity.clone();
    Particle {
        position: x,
        velocity: v,
    }
}
