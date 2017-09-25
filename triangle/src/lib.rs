pub struct Triangle(i32, i32, i32);

impl Triangle {
    pub fn build(sides: [i32; 3]) -> Result<Triangle, &'static str> {
        let mut sides = sides.to_owned();
        sides.sort();

        if sides.contains(&0) {
            Err("Triangle with zero length side is illegal")
        } else if sides[0] + sides[1] < sides[2] ||
                  sides[0] + sides[2] < sides[1] ||
                  sides[1] + sides[2] < sides[0] {
            Err("Triangle inequality violated")
        } else {
            Ok(Triangle(sides[0], sides[1], sides[2]))
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.0 == self.1 && self.1 == self.2
    }

    pub fn is_isosceles(&self) -> bool {
        self.0 == self.1 || self.1 == self.2
    }

    pub fn is_scalene(&self) -> bool {
        self.0 != self.1 && self.1 != self.2
    }
}
