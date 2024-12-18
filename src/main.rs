use nom::bytes::streaming::{tag, take_until, take_while1};
use nom::AsChar;
use nom::branch::alt;
use nom::bytes::{tag_no_case, take_while};
use nom::combinator::rest;
use nom::sequence::{delimited, terminated};
use nom::Parser;

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
    println!("body:{}",&source_code);
    //提取正文内容,去头尾{}  解析模式
    let body_content_parser = take_until::<&str, &str, nom::error::Error<&str>>("}");
    let take_while2 = take_while(AsChar::is_space);
    let res = delimited(tag("{"), terminated(body_content_parser, tag("}")),take_while2).parse(source_code);
    dbg!(&res);
    println!("parse content:{}", res.unwrap().1);

}
