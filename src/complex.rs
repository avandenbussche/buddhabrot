use std::fmt;

pub struct Complex(pub f64, pub f64);

impl Complex {

    pub fn squared(&mut self) {
        let old_r = self.0;
        let old_i = self.1;
        self.0 = old_r * old_r - old_i * old_i;
        self.1 = 2.0 * old_r * old_i;
    }

    pub fn add(&mut self, c: Complex) {
        self.0 += c.0;
        self.1 += c.1;
    }
    
    pub fn abs(&self) -> f64 {
        (self.0 * self.0 + self.1 * self.1).sqrt()
    }

    pub fn abs_squared(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1
    }

}

impl fmt::Display for Complex {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} + {}i", self.0, self.1)
    }

}