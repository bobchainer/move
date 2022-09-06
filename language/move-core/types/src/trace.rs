use crate::gas_algebra::InternalGas;
use crate::value::MoveTypeLayout;
use crate::vm_status::VMStatus;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum CallType {
    Call,
    CallGeneric,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CallTrace {
    pub depth: u32,
    pub call_type: CallType,
    pub module_address: Option<String>,
    pub function: String,
    pub ty_args: Vec<MoveTypeLayout>,
    pub args_types: Vec<MoveTypeLayout>,
    pub args_values: Vec<Vec<u8>>,
    pub gas_used: InternalGas,
    pub err: Option<VMStatus>,
}
