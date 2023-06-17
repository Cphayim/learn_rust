use anyhow::anyhow;
use polars::lazy::dsl::Expr;
use sqlparser::ast::{
    BinaryOperator as SqlBinaryOperator, Expr as SqlExpr, Offset as SqlOffset, OrderByExpr, Select,
    SelectItem, SetExpr, Statement, TableWithJoins, Value as SqlValue,
};

/// 解析出来的 SQL
pub struct Sql<'a> {
    pub(crate) selection: Vec<Expr>,
    pub(crate) condition: Option<Expr>,
    pub(crate) source: &'a str,
    pub(crate) order_by: Vec<(String, bool)>,
    pub(crate) offset: Option<i64>,
    pub(crate) limit: Option<usize>,
}

// 因为 Rust trait 的孤儿原则，我们如果要想对已有的类型实现已有的 trait,
// 需要简单包装一下

pub struct Expression(pub(crate) Box<SqlExpr>);
pub struct Operation(pub(crate) SqlBinaryOperator);
pub struct Projection<'a>(pub(crate) &'a SelectItem);
pub struct Source<'a>(pub(crate) &'a [TableWithJoins]);
pub struct Order<'a>(pub(crate) &'a OrderByExpr);
pub struct Offset<'a>(pub(crate) &'a SqlOffset);
pub struct Limit<'a>(pub(crate) &'a SqlExpr);
pub struct Value(pub(crate) SqlValue);

// /// 把 SqlParser 解析出来的 Statement 转换成我们需要的结构
// impl<'a> TryFrom<&'a Statement> for Sql<'a> {
//     type Error = anyhow::Error;

//     fn try_from(sql: &'a Statement) -> Result<Self, Self::Error> {
//         match sql {
//             Statement::Query(q) => {
//                 let offset = q.offset.as_ref();
//                 let limit = q.limit.as_ref();
//                 let orders = &q.order_by;

//                 let Select {
//                     from: table_with_joins,
//                     selection: where_clause,
//                     projection,
//                     group_by: _,
//                     ..
//                 } = match &q.body.as_ref() {
//                     SetExpr::Select(statement) => statement.as_ref(),
//                     _ => return Err(anyhow!("We only support Select Query at the moment")),
//                 };

//                 let source = Source(table_with_joins).try_into()?;

//                 let condition = match where_clause {
//                     Some(expr) => Some(Expression(Box::new(expr.to_owned())).try_into()?),
//                     None => None,
//                 };

//                 Ok(Sql {
//                     selection: (),
//                     condition,
//                     source: (),
//                     order_by: (),
//                     offset: (),
//                     limit: (),
//                 })
//             }
//             _ => Err(anyhow!("We only support Query at the moment")),
//         }
//     }
// }

// /// 把 SqlParser 的 Expr 转换成 DataFrame 的 Expr
// impl TryFrom<Expression> for Expr {
//     type Error = anyhow::Error;

//     fn try_from(expr: Expression) -> Result<Self, Self::Error> {
//         match *expr.0 {
//             SqlExpr::BinaryOp { left, op, right } => Ok(Expr::BinaryExpr {
//                 left: Box::new(Expression(left).try_into()?),
//                 op: Operation(op).try_into()?,
//                 right: Box::new(Expression(right).try_into()?),
//             }),
//             SqlExpr::Wildcard => Ok(Self::Wildcard),
//             SqlExpr::IsNull(expr) => OK(Self::Is)
//             v => Err(anyhow!("expr {:#?} is not supported", v)),
//         }
//     }
// }
