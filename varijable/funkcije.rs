fn main(){
    let x : u8 = 5;
    let y: u8 = 5;
    plus(x, y);
   let minuss =  minus(x, y);
   println!("{minuss}");
}
fn minus(x : u8, y: u8) -> u8{
   x - y
}
fn plus(x: u8, y: u8){
    let r : u8 = x + y;
    println!("{r}");
}
