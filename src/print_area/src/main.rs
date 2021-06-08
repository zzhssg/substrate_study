pub trait  Get_area {
    fn get_area(&self) -> f32;
}

pub struct Circular {
    r:f32
}

pub struct Triangle {
    s:f32
}

pub struct Square {
    l:f32
}


impl Get_area for Circular {
    fn get_area(&self) -> f32 {
        let area = self.r * self.r * 3.1415926;
        area
    }
}

impl Get_area for Triangle {
    fn get_area(&self) -> f32 {
        let area = self.s * self.s * 0.43301270189222;
        area
    }
}

impl Get_area for Square {
    fn get_area(&self) -> f32 {
        let area = self.l * self.l ;
        area
    }
}

fn print_area<T:Get_area>(c:T) {
    let area = c.get_area();
    println!("图形面积为：{}",area)
}
fn main() {
    let c1=Circular {r:6.0};
    let c2=Triangle {s:10.0};
    let c3=Square{l:3.0};
    print_area(c1);
    print_area(c2);
    print_area(c3);

}
