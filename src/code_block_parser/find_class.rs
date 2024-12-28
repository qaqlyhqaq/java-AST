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

    type Output = (&'a str,&'a str);

    extern "rust-call" fn call_once(self, args: (&'a String,)) -> (&str,&str) {
        let mut until = nom::bytes::complete::take_until::<&str, &str, nom::error::Error<&str>>(self.class_name.as_str());
        let x:(&str,&str) = until(args.0.as_str()).unwrap();
        x
    }

}

impl FindClass{
    pub fn new(class_name : &str) -> FindClass{
        FindClass{class_name:class_name.to_string()}
    }
}


#[cfg(test)]
mod test_find_class{
    use crate::code_block_parser::find_class::{ FindClass};

    /*
    功能单元测试
     */
    #[test]
    fn it_works() {
        let mut find_class = FindClass::new("lyh");
        let source_code = "start_lyh_end".to_string();
        let result = find_class(&source_code);
        assert_eq!(result, ("lyh_end","start_"));
    }
}