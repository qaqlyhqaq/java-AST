/*
闭包实现案例,声明
 */
use std::marker;

struct Closure {
    s : String,
    t : String,
}

/*
闭包结构实现,函数调用
 */
impl FnOnce<()> for Closure
{

    type Output = String;

    extern "rust-call" fn call_once(self, args: ()) -> Self::Output {
        unreachable!("不可达 FnOnce");
    }
}

impl FnMut<()>  for Closure {

    extern "rust-call" fn call_mut(&mut self, args: ()) -> Self::Output {
        println!("FnMut");
        let x = self.s.clone() + &*self.t;
        x
    }
}


/*
    let (source_code, _) =
        nom::bytes::complete::take_until::<&str, &str, nom::error::Error<&str>>("class")(source_code).unwrap();
 */





#[cfg(test)]
mod tests{
    use crate::code_block_parser::find_class::Closure;

    #[test]
    fn it_works() {
        let mut closure = Closure { s: "hello_".to_string(), t: "world".to_string() };
        let string = closure();
        assert_eq!(string, "hello_world".to_string());
    }
}


#[derive(Debug,Clone)]
struct  FindClass<> {
    class_name : String,

}

impl<'a>  FnOnce<(&'a String,)> for FindClass<>
{

    type Output = String;

    extern "rust-call" fn call_once(self, args: (&'a String,)) -> Self::Output {
        let mut until = nom::bytes::complete::take_until::<&str, &str, nom::error::Error<&str>>(self.class_name.as_str());
        until(args.0.as_str()).unwrap().1.to_string()
    }

}