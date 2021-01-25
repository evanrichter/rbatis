use std::borrow::BorrowMut;

use serde_json::{json, Value};

use crate::interpreter::sql::ast::RbatisAST;
use crate::interpreter::sql::node::node::do_child_nodes;
use crate::interpreter::sql::node::node_type::NodeType;
use rexpr::ast::Node;
use rexpr::runtime::RExprRuntime;

#[derive(Clone, Debug)]
pub struct WhenNode {
    pub childs: Vec<NodeType>,
    pub test: String,
    pub test_fn: Node,
}

impl WhenNode {
    pub fn from(
        runtime: &RExprRuntime,
        source: &str,
        express: &str,
        childs: Vec<NodeType>,
    ) -> Result<Self, crate::core::Error> {
        let express = express[Self::name().len()..].trim();
        return Ok(WhenNode {
            childs,
            test: express.to_string(),
            test_fn: runtime.parse(express)?,
        });
    }
}

impl RbatisAST for WhenNode {
    fn name() -> &'static str {
        "when"
    }
    fn eval(
        &self,
        convert: &dyn crate::interpreter::sql::StringConvert,
        env: &mut Value,
        engine: &RExprRuntime,
        arg_array: &mut Vec<Value>,
        arg_sql: &mut String,
    ) -> Result<serde_json::Value, crate::core::Error> {
        let result = self.test_fn.eval(env)?;
        if !result.is_boolean() {
            return Result::Err(crate::core::Error::from(
                "[rbatis] test:'".to_owned() + self.test.as_str() + "' is not return bool!",
            ));
        }
        let is_ok = result.as_bool().unwrap_or(false);
        if is_ok {
            do_child_nodes(convert, &self.childs, env, engine, arg_array, arg_sql)?;
        }
        return Ok(result);
    }
}
