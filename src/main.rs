fn factorial_vector(x : u8) -> Vec<usize> {
    assert!(x != 0, "For this function, do not accept argument equal to zero");
    //let mut ans = Vec::with_capacity(x as usize);
    //let ans = (1..(x + 1)).filter(|x| x % 2 == 0).map(|x| x.pow(2)).collect::<Vec<_>>();
    let mut p = 1usize;
    (1..(x + 1)).map(|i| { p += i; p }).collect::<Vec<usize>>()
}
fn produce_combinations(x : u8) -> Vec<Vec<u8>> {
    let v = factorial_vector(x);

}

struct BooleanLogic {
    variables : Vec<bool>,
    names : Hashmap<
}
struct BooleanLogic {
    fn new() -> BooleanLogic {
        BooleanLogic {
            values : Vec::new(),
            names : Hashmap::new(),
        }
    }
    fn with_capacity(x : usize) -> BooleanLogic {
        BooleanLogic {
            values : Vec::with_capacity(x),
            names : Hashmap::with_capacity(x),
        }
    }
}
fn main() {
    {
        let george = BooleanLogic::new();
        let p = Proposition::from_bool(true);
        let q = Proposition::from_bool(false);
        let r = Proposition::Symbol()
    }
}
struct Proposition {
    Symbol(bool)
}
enum BinaryOperator {
    False,
    Nor,
    Xq,
    NotP,
    MaterialNonImplication,
    NotQ,
    Xor,
    Nand,
    And,
    Xnor,
    Q,
    IfThen,
    P,
    ThenIf,
    Or,
    True,
}
enum BooleanExpression {
    B(BinaryOperator),
    U(UnaryOperator),
    V(Boolean),
}
/*
trait InVector {
    fn 
}
impl InVector for BinaryOperator {
    fn append(&self, )
}

*/
enum UnaryOperator {
    Not,
    Identity,
    False,
    True,
}
impl BooleanExpression {
    fn is_valid(s : Vec<InVector>) -> bool {
        let mut expected = 1usize;
        use crate::InVector::*;
        for i in x {
            match i {
                B => {
                    match expected.checked_add(1) {
                        Some(a) => {
                            
                        },
                        None => {

                        },
                    }
                },
                U => {

                },
                V => {
                    expected.check_minus()
                },
            }
        }
        return (expected == 0usize);
    }
}
impl BinaryOperator {
    fn calculate(x : BinaryOpeator, p : bool, q : bool) -> bool {
        match 
    }
}
fn main() {
    use crate::InVector::*;
    use crate::BinaryOperator::*;
    use crate::UnaryOperator::*;
    let expression : Vec<InVector> = vec![];
    assert!(InVector::is_valid(expression));
    let expression : Vec<InVector> = vec![V(true)];
    assert!(InVector::is_valid(expression));
    let expression : Vec<InVector> = vec![V(false)];
    assert!(InVector::is_valid(expression));
    let expression : Vec<InVector> = vec![B(And), U(Identity), V(true), U(Not), V(false)];
    assert!(InVector::is_valid(expression));
    let expression : Vec<InVector> = vec![U(False), B(And), V(true), V(false)];
    assert!(InVector::is_valid(expression));
    println!("Hello, world!");
}
enum Boolean {
    T,
    C,
}
impl Boolean {
    
}
enum Connectives {
    A
}
struct Proposition {
    truth_value : Boolean,
    expression : BooleanExpression,
}
#[derive(Eq, PartialEq)]
struct Pixel(u8, u8, u8);
fn mut_erase_bits(x : mut &u8, y : u8) -> Result<(), ()> {
    if y < 8 {
        x = (x << y) >> y;
    }
    return Err(());
}
fn erase_bits(x : u8, y : u8) -> u8 {
    if y < 8 {
        return ((x << y) >> y);
    }
    panic!();
}
struct RunLength(Vec<(u8, Pixel)>);
impl RunLength {
    fn with_capacity(x : usize) -> RunLength {
        RunLength(Vec::with_capacity(x));
    }
}
impl Pixel {
    fn new(r : u8, g : u8, b : u8) -> Pixel {
        Pixel()
    }
    fn eq(&self, y : &Pixel) -> bool {
        (self.0 == y.0 && self.1 == y.1 && self.2 == y.2)
    }
    fn approx_eq(x : &Pixel, y : &Pixel) -> bool {
        x_approx = Pixel::new(erase_bits(x.0, 1), erase_bits(x.1, 1), erase_bits(x.2, 1));
        y_approx = Pixel::new(erase_bits(x.0, 1), erase_bits(x.1, 1), erase_bits(x.2, 1));
        x_approx.eq(&y_approx)
    }
    fn encode_run_length(x : Vec<Pixel>) -> Vec<(Pixel, u8)> {
        assert!(!x.is_empty());
        let mut last_pixel = x[0];
        let mut count = 0u8;
        let mut ans : RunLength = RunLength::with_capacity(x.len());
        for current_pixel in x.iter() {
            if last_pixel.eq(&current_pixel) {
                count = count.checked_add(1);
            }
            else {
                ans.append((current_pixel, count));
                last_pixel = current_pixel;
                count = 1;
            }
        }
        ans.append((current_pixel, count));
    }
}
enum SimplifiedColor {
    Blue,
    Green,
    Indigo,
    Orange,
    Pink,
    Purple,
    Red,
    Teal,
    Yellow,
    White,
    Black,
}
impl SimplifiedColors {
    fn to_rgb_light(x : &SimplifiedColor) -> Pixel {
        match x {
            White => {
                Pixel::new(255, 255, 255),
            },
            Black => {
                Pixel::new(0, 0, 0),
            },
            Blue => {
                Pixel::new(0, 122, 255),
            },
            Green => {
                Pixel::new(52, 199, 89),
            },
            Indigo => {
                Pixel::new(88, 86, 214),
            },
            Orange => {
                Pixel::new(255, 149, 0),
            },
            Pink => {
                Pixel::new(255, 45, 85),
            },
            Purple => {
                Pixel::new(175, 82, 222),
            },
            Red => {
                Pixel::new(255, 59, 48),
            },
            Teal => {
                Pixel::new(90, 200, 250),
            },
            Yellow => {
                Pixel::new(255, 204, 0),
            },
        }
    }
    fn to_rgb_dark(x : &SimplifiedColor) -> Pixel {
        match x {
            White => {
                Pixel::new(255, 255, 255),
            },
            Black => {
                Pixel::new(0, 0, 0),
            },
            Blue => {
                Pixel::new(10, 132, 255),
            },
            Green => {
                Pixel::new(48, 209, 230),
            },
            Indigo => {
                Pixel::new(94, 92, 230),
            },
            Orange => {
                Pixel::new(255, 159, 10),
            },
            Pink => {
                Pixel::new(255, 55, 95),
            },
            Purple => {
                Pixel::new(191, 90, 242),
            },
            Red => {
                Pixel::new(255, 69, 58),
            },
            Teal => {
                Pixel::new(100, 210, 255),
            },
            Yellow => {
                Pixel::new(255, 214, 10),
            },
        }
    }
}
struct TwoDSpace {

}
impl TwoDSpace {
    fn 
}
fn main() {
    
}
