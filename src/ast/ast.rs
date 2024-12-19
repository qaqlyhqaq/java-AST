/*
java AST 定义树结构
 */
use crate::ast::abstract_class::AbstractClass;
use crate::ast::annotation::Annotation;
use crate::ast::class::Class;
use crate::ast::interface::Interface;
use crate::ast::package::Package;

pub enum JavaAST {
    //类
    Class(Class),
    //包
    Package(Package),
    //接口
    Interface(Interface),
    //抽象类
    AbstractClass(AbstractClass),
    //注解类
    Annotation(Annotation),
}