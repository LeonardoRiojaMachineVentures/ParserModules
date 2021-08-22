a = [1, 5]
b = [2]
c = [4, 7]

print(a)
print(b)
print(c)

x = a + b
print(x)
y = x + c
print(y)
w = b + c
print(w)
z = a + w
print(y)
print(z)

'''

let r Real
: GreaterThanEq(r 0)

struct Sphere
    points [Real 3]


fn Sphere(r Real) [Real 3]
    : GreaterThanEq(r 0)
    let p [Real 3]
    : Eq(Powi(r 2) p.dot(p))
    return p

fn main() {
    let a : [u8] = [1, 5];
    let b : [u8] = [2];
    let c : [u8] = [4, 7];

    let mut video = Video::for_yt([2560, 1600], 60).unwrap();
    let mut video = Video::default();
    let a_t = Function::time_2d(["1280", "1600/6"]);
    let b_t = Function::time_2d(["1280", "3*1600/6"]);
    let c_t = Function::time_2d(["1280", "5*1600/6"]);
    let mut a_svg = Svg::rigid_body(&a).with_position_from_center(a_t).with_rotation(Function::rotation_null_2d());
    let mut b_svg = Svg::rigid_body(&b).with_position_from_center(b_t).with_rotation(Function::rotation_null_2d());
    let mut c_svg = Svg::rigid_body(&c).with_position_from_center(c_t).with_rotation(Function::rotation_null_2d());
    
    video.add([[&a_svg, 3], [&b_svg, 3], [&c_svg, 3]]);
    video.animate_for_frames(300);
    let a_t = Function::time_2d(["1280", "1600/6"]);
    let b_t = Function::time_2d(["1280", "3*1600/6"]);
    let c_t = Function::time_2d(["1280", "5*1600/6"]);
    
    a_svg.with_position()
}
let a [Natural] = [1 5]
let b [Natural] = [2]
let c [Natural] = [4 7]
Append(a b)

let d = svg(a)
let e = svg(b)
let f = svg(c)

let g = Video::new()


'''