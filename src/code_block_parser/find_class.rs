use nom::{IResult, Input, OutputMode, PResult};

#[derive(Debug,Clone)]
pub struct  FindClass<> {

    class_name : String,

}

impl<'a>  FnOnce<(&'a str,)> for FindClass<>
{

    // type Output = (&'a str,&'a str);
    type Output =IResult<&'a str, &'a str, nom::Err<(&'a str,)>>;

    extern "rust-call" fn call_once(self, args: (&'a str,)) -> Self::Output {
        let mut until = nom::bytes::complete::take_until::<&str, &str, nom::error::Error<&str>>(self.class_name.as_str());
        let x:(&str,&str) = until(args.0.as_str()).unwrap();
        Ok(x)
    }

}


impl<'a>  FnMut<(&'a str,)> for FindClass<> {

    extern "rust-call" fn call_mut(&mut self, args: (&'a str,)) -> Self::Output {
        let mut until = nom::bytes::complete::take_until::<&str, &str, nom::error::Error<&str>>(self.class_name.as_str());
        let x:(&str,&str) = until(args.0.as_str()).unwrap();
        Ok(x)
    }
}

// pub trait Fn<Args: Tuple>: FnMut<Args> {
//     /// Performs the call operation.
//     #[unstable(feature = "fn_traits", issue = "29625")]
//     extern "rust-call" fn call(&self, args: Args) -> Self::Output;
// }

impl<'a>  Fn<(&'a str,)> for FindClass<> {
    extern "rust-call" fn call(&self, args: (&'a str,)) -> Self::Output {
        let mut until = nom::bytes::complete::take_until::<&str, &str, nom::error::Error<&str>>(self.class_name.as_str());
        let x: (&str, &str) = until(args.0.as_str()).unwrap();
        Ok(x)
    }

}

// impl<'a> nom::Parser<&'a str> for FindClass {
//     type Output = ();
//     type Error = ();
//
//     fn process<OM: OutputMode>(&mut self, input: &'a str) -> PResult<OM, &'a str, Self::Output, Self::Error> {
//         todo!()
//     }
// }


impl FindClass{
    pub fn new(class_name : &str) -> FindClass{
        FindClass{class_name:class_name.to_string()}
    }
}



#[cfg(test)]
mod test_find_class{
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::Parser;
    use crate::code_block_parser::find_class::{ FindClass};

    /*
    功能单元测试
     */
    #[test]
    fn it_works() {
        let mut find_class = FindClass::new("lyh");
        let source_code = "start_lyh_end";
        let result = find_class(source_code);
        assert_eq!(result, Ok(("lyh_end","start_")));
    }

    #[test]
    fn is_nom_use(){
        let mut choice = alt((tag("class"),tag::<&str, &str, nom::error::Error<&str>>("asdf")));
        // let mut choice = alt((FindClass::new("class"),tag("class"),tag::<&str, &str, nom::error::Error<&str>>("asdf")));


        let choice1 = choice.parse("classasdf");
        assert!(choice1.is_ok());
    }
}