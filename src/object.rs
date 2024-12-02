use crate::point::*;
use raylib::prelude::*;

macro_rules! map {
    ($array:expr, $map_value:expr, $target_type:ty) => {{
        let result = $array.iter().map($map_value).collect::<Vec<$target_type>>();
        result.try_into().unwrap()
    }};
}

pub trait Shape {
    fn points(&self) -> &[Point; 3];
    fn points_mut(&mut self) -> &mut [Point; 3];
    fn color(&self) -> &Color;
    fn color_mut(&mut self) -> &mut Color;
    fn draw(&self, offset: &Point, direction: f32, d: &mut RaylibDrawHandle);
}

pub struct Triangle {
    pub points: [Point; 3],
    pub color: Color
}

impl Shape for Triangle {
    fn points(&self) -> &[Point; 3] {
        &self.points
    }
    fn points_mut(&mut self) -> &mut [Point; 3] {
        &mut self.points
    }
    fn color(&self) -> &Color {
        &self.color
    }
    fn color_mut(&mut self) -> &mut Color {
        &mut self.color
    }
    fn draw(&self, offset: &Point, direction: f32, d: &mut RaylibDrawHandle) {
        let sin_val = direction.sin();
        let cos_val = direction.cos();
        let rotated_points: [Vector2; 3] = map!(self.points, |point| Vector2 { x: point.x * cos_val - point.y * sin_val, y: point.x * sin_val + point.y * cos_val }, Vector2);
        let points: [Vector2; 3] = map!(rotated_points, |point| Vector2 { x: offset.x + point.x, y: offset.y + point.y }, Vector2);
        d.draw_triangle(points[0], points[1], points[2], self.color);
    }
}

pub trait Object {
    fn position(&self) -> &Point;
    fn position_mut(&mut self) -> &mut Point;
    fn direction(&self) -> f32;
    fn shape(&self) -> &Box<dyn Shape>;
    fn shape_mut(&mut self) -> &mut Box<dyn Shape>;
    fn velocity(&self) -> f32;
    fn velocity_mut(&mut self) -> &mut f32;
    fn ang_velocity(&self) -> f32;
    fn ang_velocity_mut(&mut self) -> &mut f32;
    fn draw(&mut self, delta_time: f32, d: &mut RaylibDrawHandle<'_>);
}

pub struct Boid {
    shape: Box<dyn Shape>,
    position: Point,
    direction: f32,
    velocity: f32,
    ang_velocity: f32,
    pub time_since_wall_avoidance: f32,
}

impl Boid {
    pub fn new(position: Point) -> Self {
        Self {
            shape: Box::new(
                Triangle { 
                    points: [
                        Point { x: 4.0, y: 0.0 },
                        Point { x: -4.0, y: -3.0 },
                        Point { x: -4.0, y: 3.0 },
                    ],
                    color: Color::BLACK
                }
            ),
            position,
            direction: 0.0,
            velocity: 0.0,
            ang_velocity: 0.0,
            time_since_wall_avoidance: 1000.0,
        }
    }
    pub fn add_direction(&mut self, delta_rad: f32) {
        self.direction += delta_rad;
        const TWO_PI: f32 = (2.0 * PI) as f32;
        while self.direction < 0.0 {
            self.direction += TWO_PI;
        }
        while self.direction >= TWO_PI {
            self.direction -= TWO_PI;
        }
    }
    pub fn set_direction(&mut self, rad: f32) {
        self.direction = rad;
    }
}
impl Object for Boid {
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