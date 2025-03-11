/* 
Create an enum called Shape and provide the values of "triangle square, pentagon, octagon".  Then create a method for this enum that returns the number of corners each shape has based on the type of shape.
Example:
triangle.corners() will return 3
square.corners() will return 4
*/

enum Shape {
    Triangle,
    Square,
    Pentagon,
    Octagon,
}

impl Shape {
    fn corners(&self) -> u32 {
        match self
        {
           Shape::Triangle => 3,
           Shape::Square => 4,
           Shape::Pentagon => 5,
           Shape::Octagon => 8,
        }
    }
}

fn main() {
    let triangle = Shape::Triangle;
    let square = Shape::Square;
    let pentagon = Shape::Pentagon;
    let octagon = Shape::Octagon;
    println!("{}", triangle.corners());
    println!("{}", square.corners());
    println!("{}", pentagon.corners());
    println!("{}", octagon.corners());
}
