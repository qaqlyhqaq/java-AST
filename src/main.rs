use nom::AsChar;
use nom::bytes::streaming::{tag, take_until1, take_while1};
use nom::character::streaming::char;


fn main() {

    let source_code = r#"
    //ignore commentary
    class Main {}
    "#;

    //查找到main 标识
    let (source_code, discern) = take_until1::<&str, &str, nom::error::Error<&str>>("class")(source_code).unwrap();
    println!("{}", discern);
    //解析main 结构体类
    let (source_code, _) = tag::<&str, &str, nom::error::Error<&str>>("class")(source_code).unwrap();
    //判断是否有分隔符,语法检查
    let (input, _) = take_while1::<_, &str, nom::error::Error<&str>>(AsChar::is_space)(source_code).unwrap();

    println!("input:{}",input);
}
