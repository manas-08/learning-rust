fn main() {
    let l = 5.0;
    let w = 10.0;
    println!("Area of rectangle: {}", area_rectangle(l, w));
}


fn area_rectangle(length: f64, width: f64) -> f64 {
    length * width
}