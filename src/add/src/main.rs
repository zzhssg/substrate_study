
fn add(num:&[u32]) -> Option<u32>{
    let mut count = 0;
    for i in num.iter() {
        if u32::MAX - i < count {
            return None
        }
        count = count + i ;
    }

    Some(count)

}
fn main() {
    let mut x =[u32::MAX,1];
    let mut x2 =[u32::MAX,0];

    let y = add(&x);
    let y2 = add(&x2);


    println!("{:?}",y);
    println!("{:?}",y2);

}
