use crate::ast::ast::JavaAST;

pub struct Package{
    //包名
    name: String,
    //包路径
    path: String,
    //包下内容
    context:Vec<JavaAST>,
}