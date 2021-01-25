use crate::interpreter::sql::ast::RbatisAST;
use crate::interpreter::sql::node::bind_node::BindNode;
use crate::interpreter::sql::node::choose_node::ChooseNode;
use crate::interpreter::sql::node::foreach_node::ForEachNode;
use crate::interpreter::sql::node::if_node::IfNode;
use crate::interpreter::sql::node::node_type::NodeType::NWhen;
use crate::interpreter::sql::node::otherwise_node::OtherwiseNode;
use crate::interpreter::sql::node::set_node::SetNode;
use crate::interpreter::sql::node::string_node::StringNode;
use crate::interpreter::sql::node::trim_node::TrimNode;
use crate::interpreter::sql::node::when_node::WhenNode;
use crate::interpreter::sql::node::where_node::WhereNode;
use rexpr::runtime::RExprRuntime;
use serde_json::{json, Value};
use std::collections::HashMap;

use super::node_type::NodeType;

//执行子所有节点
pub(crate) fn do_child_nodes(
    convert: &dyn crate::interpreter::sql::StringConvert,
    child_nodes: &Vec<NodeType>,
    env: &mut Value,
    engine: &RExprRuntime,
    arg_array: &mut Vec<Value>,
    arg_sql: &mut String,
) -> Result<serde_json::Value, crate::core::Error> {
    for item in child_nodes {
        item.eval(convert, env, engine, arg_array, arg_sql)?;
    }
    return Result::Ok(serde_json::Value::Null);
}
