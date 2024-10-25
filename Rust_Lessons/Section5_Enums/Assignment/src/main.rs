enum Shape
{
    triangle,
    square,
    pentagon,
    octagon,
}

impl Shape
{
    fn corners(self) -> &'static i32
    {
        match self
        {
            Shape::triangle => &3,
            Shape::square => &4,
            Shape::pentagon => &5,
            Shape::octagon => &8,
        }
    }
}

fn main() {

    let Triangle = Shape::triangle;
    let Square = Shape::square;
    let Pentagon = Shape::pentagon;
    let Octagon = Shape::octagon;
    println!("{}", Triangle.corners());
    println!("{}", Square.corners());
    println!("{}", Pentagon.corners());
    println!("{}", Octagon.corners());
}
