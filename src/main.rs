use rand::Rng;
mod point;
mod object;

use std::{borrow::Borrow, f32::consts::PI, ops::Deref};

use object::*;
use point::*;
use raylib::prelude::*;

const BOID_VIEW_RADIUS: f32 = 50.0;
const HALF_PI: f32 = (PI / 2.0) as f32;
const TWO_PI: f32 = (2.0 * PI) as f32;

fn main() {
    boid_simulation();
}

fn test() {
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("Test")
        .fullscreen()
        .build();

    let mut screen_width = rl.get_render_width() as f32;
    let mut screen_height = unsafe { ffi::GetRenderHeight() } as f32;
    let mut half_screen_width = screen_width / 2.0;
    let mut half_screen_height = screen_height / 2.0;
    let mut mouse_pos = Point { x: rl.get_mouse_x() as f32 - half_screen_width, y: rl.get_mouse_y() as f32 - half_screen_height };
    

    const HALF_PI: f32 = (PI / 2.0) as f32;
    const TWO_PI: f32 = (2.0 * PI) as f32;

    struct Arrow {
        shape: Box<dyn Shape>,
        position: Point,
        direction: f32,
        velocity: f32,
        ang_velocity: f32,
    }
    
    impl Arrow {
        pub fn new(position: Point) -> Self {
            Self {
                shape: Box::new(
                    Triangle { 
                        points: [
                            Point { x: 80.0, y: 0.0 },
                            Point { x: -80.0, y: -60.0 },
                            Point { x: -80.0, y: 60.0 },
                        ],
                        color: Color::BLACK
                    }
                ),
                position,
                direction: 0.0,
                velocity: 0.0,
                ang_velocity: 0.0,
            }
        }
        pub fn add_direction(&mut self, delta_rad: f32) {
            self.direction += delta_rad;
            const TWO_PI: f32 = (2.0 * PI) as f32;
            while self.direction < 0.0 {
                self.direction += TWO_PI;
            }
            while self.direction > TWO_PI {
                self.direction -= TWO_PI;
            }
        }
    }

    impl Object for Arrow {
        fn position(&self) -> &Point {
            &self.position
        }
        fn position_mut(&mut self) -> &mut Point {
            &mut self.position
        }
        fn direction(&self) -> f32 {
            self.direction
        }
        fn shape(&self) -> &Box<dyn Shape> {
            &self.shape
        }
        fn shape_mut(&mut self) -> &mut Box<dyn Shape> {
            &mut self.shape
        }
        fn velocity(&self) -> f32 {
            self.velocity
        }
        fn velocity_mut(&mut self) -> &mut f32 {
            &mut self.velocity
        }
        fn ang_velocity(&self) -> f32 {
            self.ang_velocity
        }
        fn ang_velocity_mut(&mut self) -> &mut f32 {
            &mut self.ang_velocity
        }
        fn draw(&mut self, delta_time: f32, d: &mut RaylibDrawHandle<'_>) {
            self.shape.draw(&self.position, self.direction, d);
            self.position.add_to(&Point { x: self.direction.cos() * self.velocity * delta_time, y: self.direction.sin() * self.velocity * delta_time });
            self.add_direction(delta_time * self.ang_velocity);
        }
    }

    let mut arrow = Arrow::new(Point { x: half_screen_width, y: half_screen_height});

    rl.set_target_fps(60);
    while !rl.window_should_close() {
        if rl.is_window_resized() {
            screen_width = rl.get_render_width() as f32;
            screen_height = unsafe { ffi::GetRenderHeight() } as f32;
            half_screen_width = screen_width / 2.0;
            half_screen_height = screen_height / 2.0;
            arrow.position.set(half_screen_width, half_screen_height);
        }

        if !rl.is_window_fullscreen() {
            rl.toggle_fullscreen();
        }

        mouse_pos = Point { x: (rl.get_mouse_x() as f32 - half_screen_width), y: (rl.get_mouse_y() as f32 - half_screen_height) };
        let delta_time = rl.get_frame_time();

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);

        arrow.draw(delta_time, &mut d);

        let mouse_ang = mouse_pos.angle();
        println!("Mouse angle: {} deg", mouse_ang * 180.0 / PI as f32);
        println!("Arrow angle: {} deg", arrow.direction * 180.0 / PI as f32);
        let mut delta_ang = mouse_ang - arrow.direction;
        while delta_ang > PI as f32 { delta_ang -= TWO_PI; }
        while delta_ang < -PI as f32 { delta_ang += TWO_PI; }
        const ANG_VELOCITY: f32 = 1.0;
        if delta_ang > 0.05 {
            arrow.ang_velocity = ANG_VELOCITY;
        } else if delta_ang < -0.05 {
            arrow.ang_velocity = -ANG_VELOCITY;
        } else {
            arrow.ang_velocity = 0.0;
        }
    }
}
fn boid_simulation() {
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("Boids")
        .fullscreen()
        .build();
    
    let mut screen_width = rl.get_render_width() as f32;
    let mut screen_height = unsafe { ffi::GetRenderHeight() } as f32;

    const BOID_VELOCITY: f32 = 150.0 / 2.0;
    const BOID_ANG_VELOCITY: f32 = (PI as f32 + HALF_PI) / 2.0;
    const WALL_AVOIDANCE_COOLDOWN: f32 = 3.0;
    const TOTAL_BOIDS: usize = 600;

    let mut boids: Vec<Boid> = vec![];
    for i in 0..TOTAL_BOIDS {
        boids.push(Boid::new(Point {
            x: rand::thread_rng().gen_range(0..(screen_width as i32)) as f32,
            y: rand::thread_rng().gen_range(0..(screen_height as i32)) as f32,
        }));
        boids[i].set_direction(rand::thread_rng().gen_range(0..(TWO_PI as i32)) as f32);
        *boids[i].velocity_mut() = BOID_VELOCITY * rand::thread_rng().gen_range(95..105) as f32 / 100.0;
    }

    rl.set_target_fps(60);
    while !rl.window_should_close() {
        if rl.is_window_resized() {
            screen_width = rl.get_render_width() as f32;
            screen_height = unsafe { ffi::GetRenderHeight() } as f32;
        }

        if !rl.is_window_fullscreen() {
            rl.toggle_fullscreen();
        }

        let delta_time = rl.get_frame_time();

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);

        for i in 0..boids.len() {
            
            let boundary_correction_offset = get_boundary_correction_offset(boids[i].position(), boids[i].direction(), screen_width, screen_height);
            let correction_offset: Point = if boundary_correction_offset.magnitude() <= 0.1 {
                if boids[i].time_since_wall_avoidance >= WALL_AVOIDANCE_COOLDOWN {
                    let seperation_correction_offset = get_seperation_correction_offset(i, boids[i].position(), &boids);
                    let alignment_correction_offset = get_alignment_correction_offset(i, boids[i].position(), &boids);
                    let cohesion_correction_offset = get_cohesion_correction_offset(i, boids[i].position(), &boids);

                    seperation_correction_offset
                        .add(&alignment_correction_offset)
                        .add_to(&cohesion_correction_offset).clone()
                } else {
                    Point { x: 0.0, y: 0.0 }
                }
            } else {
                boids[i].time_since_wall_avoidance = 0.0;
                boundary_correction_offset
            };
            boids[i].time_since_wall_avoidance += delta_time;
            //println!("Time since wall avoidance: {}", boids[i].time_since_wall_avoidance);

            let boid = &mut boids[i];
            let correction = boid.position().add(&correction_offset);
            
            let correction_angle = correction_offset.angle();
            let mut needs_clamping = false;
            let mut angle_diff = correction_angle - boid.direction();
            if correction_offset.magnitude() != 0.0 && angle_diff.abs() >= HALF_PI / 2.0 {
                while angle_diff > PI as f32 { angle_diff -= TWO_PI; }
                while angle_diff < -PI as f32 { angle_diff += TWO_PI; }

                needs_clamping = angle_diff.abs() <= BOID_ANG_VELOCITY * delta_time;
                *boid.ang_velocity_mut() = if angle_diff > 0.0 { BOID_ANG_VELOCITY } else { -BOID_ANG_VELOCITY };
            } else {
                *boid.ang_velocity_mut() = 0.0;
            }
            
            boid.draw(delta_time, &mut d);

            if needs_clamping {
                let mut clamped_angle = correction_angle;
                if clamped_angle >= TWO_PI { clamped_angle -= TWO_PI; }
                boid.set_direction(clamped_angle);
            }
        }
    }
}

fn get_boundary_correction_offset(position: &Point, direction: f32, screen_width: f32, screen_height: f32) -> Point {
    const PADDING: f32 = BOID_VIEW_RADIUS;
    const LEFT_BOUND: f32 = PADDING;
    let right_bound: f32 = screen_width - PADDING;
    const UPPER_BOUND: f32 = PADDING;
    let bottom_bound: f32 = screen_height - PADDING;

    let facing_left= direction >= HALF_PI && direction <= 3.0 * HALF_PI || true;
    let facing_right = direction >= 3.0 * HALF_PI || direction <= HALF_PI || true;
    let facing_up = direction >= PI as f32 || true;
    let facing_down = direction >= 0.0 && direction <= PI as f32 || true;

    let mut boundary_correction_offset = Point { x: 0.0, y: 0.0 };
    if position.x < LEFT_BOUND && facing_left {
        boundary_correction_offset
            .add_to(
                Point { x: 1.0, y: -direction.sin() }
                    .scale_to(PADDING - position.x)
            );
    } else if position.x > right_bound && facing_right {
        boundary_correction_offset
            .add_to(
                Point { x: -1.0, y: -direction.sin() }
                    .scale_to(position.x - (screen_width - PADDING))
            );
    }
    if position.y < UPPER_BOUND && facing_up {
        boundary_correction_offset
            .add_to(
                Point { x: -direction.cos(), y: 1.0 }
                    .scale_to(PADDING - position.y)
            );
    } else if position.y > bottom_bound && facing_down  {
        boundary_correction_offset
            .add_to(
                Point { x: -direction.cos(), y: -1.0 }
                    .scale_to(position.y - (screen_height - PADDING))
            );
    }
    boundary_correction_offset.powf_to(1.0/10.0);
    boundary_correction_offset

}

fn get_seperation_correction_offset(self_index: usize, position: &Point, boids: &Vec<Boid>) -> Point {
    const MAX_DISTANCE: f32 = BOID_VIEW_RADIUS;

    let mut seperation_correction_offset = Point { x: 0.0, y: 0.0 };
    for (i, boid) in boids.iter().enumerate() {
        if i != self_index {
            let difference = position.sub(boid.position());
            let distance = difference.magnitude();
            if distance <= MAX_DISTANCE {
                seperation_correction_offset.add_to(&difference.scale((MAX_DISTANCE - distance) / MAX_DISTANCE));
            }
        }
    }
    seperation_correction_offset.normalize().scale_to(10.0);
    seperation_correction_offset
}

fn get_alignment_correction_offset(self_index: usize, position: &Point, boids: &Vec<Boid>) -> Point {
    const MAX_DISTANCE: f32 = BOID_VIEW_RADIUS;

    let mut average_direction = 0.0;
    for (i, boid) in boids.iter().enumerate() {
        if i != self_index {
            let difference = position.sub(boid.position());
            let distance = difference.magnitude();
            if distance <= MAX_DISTANCE {
                average_direction += boid.direction();
            }
        }
    }
    if average_direction == 0.0 { return Point { x: 0.0, y: 0.0 }}
    average_direction /= (boids.len() - 1) as f32;
    let mut alignment_correction_offset = Point { x: average_direction.cos(), y: average_direction.sin() };
    alignment_correction_offset.scale_to(7.5);

    alignment_correction_offset
}

fn get_cohesion_correction_offset(self_index: usize, position: &Point, boids: &Vec<Boid>) -> Point {
    const MAX_DISTANCE: f32 = BOID_VIEW_RADIUS;

    let mut average_position = Point { x: 0.0 , y: 0.0 };
    for (i, boid) in boids.iter().enumerate() {
        if i != self_index {
            let difference = position.sub(boid.position());
            let distance = difference.magnitude();
            if distance <= MAX_DISTANCE {
                average_position.add_to(boid.position());
            }
        }
    }
    average_position.scale_to(1.0/(boids.len() - 1) as f32);
    let mut cohesion_correction_offset = position.sub(&average_position);

    cohesion_correction_offset.normalize().scale_to(60.0);
    cohesion_correction_offset
}