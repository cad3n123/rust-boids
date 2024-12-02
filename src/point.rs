use raylib::ffi::PI;

#[derive(Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32
}

impl Point {
    pub fn set(&mut self, x: f32, y: f32) -> &mut Point {
        self.x = x;
        self.y = y;

        self
    }

    pub fn add_to(&mut self, other: &Point) -> &mut Point {
        self.x += other.x;
        self.y += other.y;
        self
    }

    pub fn add(&self, other: &Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn sub_to(&mut self, other: &Point) -> &mut Point {
        self.x -= other.x;
        self.y -= other.y;
        self
    }

    pub fn sub(&self, other: &Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
    
    pub fn add_num_to(&mut self, other: f32) -> &mut Point {
        self.x += other;
        self.y += other;
        self
    }

    pub fn add_num(&self, other: f32) -> Point {
        Point {
            x: self.x + other,
            y: self.y + other,
        }
    }

    pub fn scale_to(&mut self, scalar: f32) -> &mut Point {
        self.x *= scalar;
        self.y *= scalar;
        self
    }

    pub fn scale(&self, scalar: f32) -> Point {
        Point {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }

    pub fn pow_to(&mut self, exponent: i32) -> &mut Point {
        self.x.powi(exponent);
        self.y.powi(exponent);
        self
    }

    pub fn pow(&self, exponent: i32) -> Point {
        Point {
            x: self.x.powi(exponent),
            y: self.y.powi(exponent),
        }
    }

    

    pub fn powf_to(&mut self, exponent: f32) -> &mut Point {
        self.x.powf(exponent);
        self.y.powf(exponent);
        self
    }

    pub fn powf(&self, exponent: f32) -> Point {
        Point {
            x: self.x.powf(exponent),
            y: self.y.powf(exponent),
        }
    }

    pub fn angle(&self) -> f32 {
        let mut angle = (self.y / self.x).atan();
        if self.x < 0.0 {
            angle = PI as f32 + angle;
        }
        const TWO_PI: f32 = (2.0 * PI) as f32;
        while angle < 0.0 {
            angle += TWO_PI;
        }
        while angle > TWO_PI {
            angle -= TWO_PI;
        }

        angle
    }

    pub fn magnitude(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn normalize(&mut self) -> &mut Point {
        let magnitude = self.magnitude();
        if magnitude == 0.0 { return self }
        self.scale_to(1.0/magnitude);
        self
    }
}