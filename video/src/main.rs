enum Compression {
    RunLengthVertical,
    RunLengthHorizontal,
    Grid,
}
struct Pixel {
    r : u8,
    g : u8,
    b : u8,
}
struct Frame {
    compression : Compression,
    pixels : Ve<Pixel>,
}
struct Changes {
    position : 
}
struct Video {

    first_frame : Frame,
    changes : Vec<Changes>,
}

/*
type U8(x &Natural)
    : &Natural::less_than_eq(x 255)
    (x)
struct Pixel
    field r U8
    field g U8
    field b U8
impl Pixel
    new(r let U8 g let U8 b let U8) (Pixel)
        return(Pixel
            r
            g
            b
        )
struct Video
    let frames_quantity &Natural
    let width &Natural
    let height &Natural
    : &Natural::greater_than(width 0)
    : &Natural::greater_than(height 0)
    let size &Natural::multiply(width height)
    let frame [(U8 U8 U8) size]
    field video [frame frames_quantity]
    
let x Pixel::new()
*/
fn main() {
    
}
