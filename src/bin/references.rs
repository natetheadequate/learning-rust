use std::any::Any;



fn requires_ref(msg : &String)->String{
    return msg.clone()+" world"
}

trait AccessPrivateField {
    fn get_private_field(self: &Self)-> Any
}

impl AccessPrivateField for Vec<i32> {
    fn get_private_field(self: &Self)-> Any {
        self.buf
    }
}
pub fn main(){
    let msg = String::from("hello");
    let mut v: Vec<i32> = vec![1, 2, 3];
    *v.buf.capacity()
    println!("{}",requires_ref(&msg));
}