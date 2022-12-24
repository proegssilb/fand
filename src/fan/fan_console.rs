use crate::parser::{Evaluator, Node};
use std::io::{self, Write};

use crate::fan::Fan;
use crate::util;

use anyhow::Result;

// Fan speed output on console
//
// Rather than actually controlling a fan, this
// prints out fan values to the console. Useful
// for debugging and verifying configurations.
pub struct ConsoleFan {
    name: String,
}

impl ConsoleFan {
    pub fn create(name_v: &str) -> Box<dyn Fan> {
        Box::new(ConsoleFan {
            name: name_v.to_string(),
        })
    }
}

impl Fan for ConsoleFan {
    fn set_enabled(&mut self, _: bool) -> Result<()> {
        Ok(())
    }

    fn set(&mut self, v: f64) -> Result<()> {
        let _res = writeln!(io::stdout(), "{}: {}", &self.name, &v.to_string())?;
        Ok(())
    }
}

///////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////

pub struct EvalConsoleFan;

impl EvalConsoleFan {
    pub fn new() -> EvalConsoleFan {
        EvalConsoleFan {}
    }
}

impl Evaluator<Box<dyn Fan>> for EvalConsoleFan {
    fn parse_nodes(&self, nodes: &[Node]) -> Result<Box<dyn Fan>> {
        Ok(ConsoleFan::create(util::get_text_node(
            "console-fan",
            nodes,
            0,
        )?))
    }
}
