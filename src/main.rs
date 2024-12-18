use nom::Parser;
use nom::branch::alt;
use nom::bytes::streaming::{tag, take_until, take_while1};
use nom::bytes::{tag_no_case, take_while};
use nom::combinator::{map, map_opt, opt, rest};
use nom::sequence::{delimited, preceded, terminated, tuple};
use nom::{AsChar, IResult};

fn main() {
    let source_code = r#"
    //ignore commentary
    class Main {
        //注释字段
        //注释字段
        //注释字段
        //注释字段
        private int a;
    }
    花括号以后
    "#;

    //查找到main 标识
    let (source_code, discern) =
        take_until::<&str, &str, nom::error::Error<&str>>("class")(source_code).unwrap();
    //打印暂时无效上下文
    println!("{}", discern);
    //解析main 结构体类
    let (source_code, _) =
        tag::<&str, &str, nom::error::Error<&str>>("class")(source_code).unwrap();
    //判断是否有分隔符,语法检查
    let (source_code, _) =
        take_while1::<_, &str, nom::error::Error<&str>>(AsChar::is_space)(source_code).unwrap();
    //识别第一个标识,类名
    let (source_code, class_name) =
        take_while1::<_, &str, nom::error::Error<&str>>(AsChar::is_alphanum)(source_code).unwrap();
    println!("class_name:{}", class_name);
    //分隔符,提取
    let (source_code, _) =
        take_while1::<_, &str, nom::error::Error<&str>>(AsChar::is_space)(source_code).unwrap();
    println!("body:{}", &source_code);

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
    //直接匹配结尾
    let body_content_parser_1 =nom::bytes::complete::take_until1::<&str, &str, nom::error::Error<&str>>("}");
    //匹配类属性声明字段
    //去除开头空白
    let take_while2 = nom::bytes::complete::take_while::<_, &str, nom::error::Error<_>>(|x2| {
        if x2.is_space() || x2.eq(&'\n'){
            return true;
        }
        return false;
    });
    //标记字段  public 或 private
    let visit_declare = alt((nom::bytes::complete::tag::<&str, &str, nom::error::Error<&str>>("public"), nom::bytes::complete::tag("private")));
    let x = tuple((
        //可见声明
        terminated(visit_declare, nom::bytes::complete::take_while1(AsChar::is_space), ),
        //属性类型
        terminated(nom::bytes::complete::take_while1(AsChar::is_alphanum), nom::bytes::complete::take_while1(AsChar::is_space), ),
        //属性标识名称
        nom::bytes::complete::take_while1(AsChar::is_alphanum),
        //分号
        nom::bytes::complete::tag(";"))
    );
    let body_content_parser_2 = map(preceded(take_while2, x),|x1| {return "查找到一属性值";});


    // nom::multi::many0(body_content_parser_2);

    let body_content_parser = nom::multi::many0(alt((body_content_parser_2,body_content_parser, body_content_parser_1)));

    //去头尾{}  解析模式
    // let body_content_parser = take_until::<&str, &str, nom::error::Error<&str>>("}");
    let end_while = take_while(AsChar::is_space);
    // let end_while = rest;
    let res: IResult<&str, Vec<&str>, nom::error::Error<_>> = delimited(
        tag("{"),
        terminated(
        body_content_parser,
        tag("}")),
        end_while,
    )
    .parse(source_code);
    dbg!(&res);
    // println!("parse content:{}", res.unwrap().1);
}
#[cfg(test)]
mod tests {
    use nom::Parser;
    use nom::branch::alt;
    use nom::character::complete::{alpha1, digit1};
    use nom::multi::many1;

    #[test]
    fn it_works() {
        let mut choice = many1(alt((alpha1::<&str, nom::error::Error<_>>, digit1)));
        let choice1 = choice.parse("1a1a1");
        dbg!(&choice1);
    }
}
