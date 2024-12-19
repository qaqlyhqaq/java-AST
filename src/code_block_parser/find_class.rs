struct Closure {
    s : String,
    t : String,
}

impl FnOnce<()> for Closure
{

    type Output = String;

    extern "rust-call" fn call_once(self, args: ()) -> Self::Output {
        let x = self.s + &*self.t;
        x
    }
}


#[cfg(test)]
mod tests{
    use crate::code_block_parser::find_class::Closure;

    #[test]
    fn it_works() {
        let closure = Closure { s: "hello".to_string(), t: "world".to_string() };
        let string = closure();
        assert_eq!(string, "helloworld".to_string());
    }
}