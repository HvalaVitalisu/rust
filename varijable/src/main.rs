fn main() {
    let x = 5;
    let x = x * 5;
    println!("x = {x}");

    let spaces = "    ";
    //mut daje err
    let spaces = spaces.len();
    
    println!("duzina = {spaces}");

    let x: f64 = 0.1 + 0.2;

    println!("x = {x}");

    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("z = {z}");
    println!("hc = {heart_eyed_cat}");

   // let tup: (i32, i128, u8) = (700, 500000, 1);
   let tup = (500, 54.1, 1);
   let (x, _y, _z) = tup;

    println!("x = {x}");
}
