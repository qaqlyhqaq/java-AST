#![feature(str_as_str)]
#![feature(unboxed_closures)]
#![feature(fn_traits)]
#![feature(tuple_trait)]

mod code_block_parser;
mod ast;

use nom::branch::alt;
use nom::bytes::streaming::{tag,take_while1};
use nom::bytes::take_while;
use nom::combinator::{map, opt};
use nom::multi::many1;
use nom::sequence::{delimited,  preceded, terminated};
use nom::Parser;
use nom::{AsChar, IResult};
use crate::code_block_parser::find_class::FindClass;

fn main() {

    let source_code = include_str!("../assets/Simple.java");
    // let source_code = include_str!("../assets/MultiLine.java");

    //查找到class 标识
    let source_code = source_code.to_string();
    let (source_code, _) =
        FindClass::new("class")(&source_code);

    //解析class 结构体类
    let (source_code, _) =
        nom::bytes::streaming::tag::<&str, &str, nom::error::Error<&str>>("class")(source_code).unwrap();
    //判断是否有分隔符,语法检查
    let (source_code, _) =
        take_while1::<_, &str, nom::error::Error<&str>>(AsChar::is_space)(source_code).unwrap();
    //识别第一个标识,类名
    let (source_code, class_name) =
        take_while1::<_, &str, nom::error::Error<&str>>(AsChar::is_alphanum)(source_code).unwrap();
    println!("class_name:{}", class_name);
    //去除无效空行
    let (source_code, _) =
        take_while1::<_, &str, nom::error::Error<&str>>(AsChar::is_space)(source_code).unwrap();

    //提取正文内容
    //单行注释区分
    let until = nom::bytes::complete::take_until("\n");
    let preceded1 = preceded(
        nom::bytes::complete::take_until("//"),
        preceded(
            nom::bytes::complete::tag::<&str, &str, nom::error::Error<&str>>("//"),
            until,
        ),
    );
    let body_content_parser = preceded1;
    let body_content_parser = map(
        body_content_parser,
        |x| {
            return format!("查找到一条注释:{}",x).leak().as_str();
        }
    );
    //直接匹配结尾
    let body_content_parser_1 =nom::bytes::complete::take_until1::<&str, &str, nom::error::Error<&str>>("}");
    //匹配类属性声明字段
    //去除开头空白
    let take_while2 = nom::bytes::complete::take_while::<_, &str, nom::error::Error<_>>(|x2| {
        if x2.is_space() || x2.eq(&'\n') || x2.eq(&'\r'){
            return true;
        }
        return false;
    });

    //标记字段  public 或 private
    let visit_declare = alt((nom::bytes::complete::tag::<&str, &str, nom::error::Error<&str>>("public"), nom::bytes::complete::tag("private")));
    let class_body_content =
        (
        //属性注解声明
        opt(many1(
            preceded(take_while(AsChar::is_space),
                delimited(
                    nom::bytes::complete::tag("@"),
                    terminated(
                        (take_while1(AsChar::is_alphanum),
                                          opt(delimited(nom::bytes::complete::tag("("),opt(nom::bytes::complete::take_until(")")),nom::bytes::complete::tag(")")))),take_while(AsChar::is_space))
                    ,opt(alt((nom::bytes::complete::tag("\r\n"),nom::bytes::complete::tag("\n")))))
           )
        )
        ),
        //可见声明
        opt(delimited(opt(nom::bytes::complete::take_while1(AsChar::is_space)),visit_declare, nom::bytes::complete::take_while1(AsChar::is_space),)),
        //属性类型
        delimited(opt(nom::bytes::complete::take_while1(AsChar::is_space)),nom::bytes::complete::take_while1(AsChar::is_alphanum), nom::bytes::complete::take_while1(AsChar::is_space), ),
        //属性标识名称
        //分号
        terminated( nom::bytes::complete::take_while1(AsChar::is_alphanum),nom::bytes::complete::tag(";")),
    );
    
    let body_content_parser_2 = map(preceded(take_while2, class_body_content), |element| {
        return format!("查找到一属性值:{:?}",element).leak().as_str();
    });


    let preceded2 = preceded(take_while(|a|{return AsChar::is_space(a)||AsChar::is_newline(a);}), delimited(nom::bytes::complete::tag::<&str, &str, nom::error::Error<&str>>("/*"), nom::bytes::complete::take_until1("*/"),tag("*/")));
    let preceded2 = map(preceded2, |a|{
        return format!("解析多行注释:{}",a.trim()).leak().as_str();
    });


    let body_content_parser = nom::multi::many0(alt((preceded2,body_content_parser_2,body_content_parser, body_content_parser_1)));

    //去头尾{}  解析模式
    // let body_content_parser = take_until::<&str, &str, nom::error::Error<&str>>("}");
    let end_while = take_while(AsChar::is_space);
    // let end_while = rest;
    let res: IResult<&str, Vec<&str>, nom::error::Error<_>> = delimited(
        nom::bytes::streaming::tag("{"),
        terminated(
        body_content_parser,
        nom::bytes::streaming::tag("}")),
        opt(end_while),
    )
    .parse(source_code);
    // dbg!(&res);
    res.unwrap().1.iter().for_each(|x| println!("{}", x));
}
