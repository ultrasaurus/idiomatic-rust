/*
 * cargo +nightly run --example into
 */

#![feature(float_to_from_bytes)]

#[derive(Debug)]
enum TypeMarker {
    NUMBER = 0x00,  // f64
    // BOOLEAN: u8 = 0x01, // bool
    // STRING: u8 = 0x02,  // UTF-8 string 
}

#[derive(Debug)]
struct Value {
    t: TypeMarker,
    bytes: [u8; 8]    
}

impl From<f64> for Value {
    fn from(v: f64) -> Self {
        Value {
            t: TypeMarker::NUMBER,
            bytes: v.to_be_bytes()
        }
    }
}

fn main() {
  let num = 4.0;
  println!("num = {:?}", num);

  let v1 = Value::from(4.0);
  println!("Value::from(4.0) = {:?}", v1);

  let v2:Value = num.into();
  println!("num.into() = {:?}", v2);

}
