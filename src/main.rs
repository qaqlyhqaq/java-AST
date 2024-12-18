use nom::AsChar;
use nom::bytes::streaming::{tag, take_until, take_until1, take_while1};
use nom::character::streaming::char;


fn main() {

    let source_code = r#"
    //ignore commentary
    class Main {
        //声明语句
         private int a;
    }
    "#;

    //查找到main 标识
    let (source_code, discern) = take_until::<&str, &str, nom::error::Error<&str>>("class")(source_code).unwrap();
    //打印暂时无效上下文
    println!("{}", discern);
    //解析main 结构体类
    let (source_code, _) = tag::<&str, &str, nom::error::Error<&str>>("class")(source_code).unwrap();
    //判断是否有分隔符,语法检查
    let (source_code, _) = take_while1::<_, &str, nom::error::Error<&str>>(AsChar::is_space)(source_code).unwrap();
    //识别第一个标识,类名
    let (source_code, class_name) = take_while1::<_, &str, nom::error::Error<&str>>(AsChar::is_alphanum )(source_code).unwrap();
    println!("class_name:{}", class_name);
    //分隔符,提取
    let (source_code, _) = take_while1::<_, &str, nom::error::Error<&str>>(AsChar::is_space)(source_code).unwrap();
    println!("body:{}",source_code);
}
