use borsh::{BorshDeserialize, BorshSerialize};

use crate::types::{ConstantCurve, FixedCurve, LinearCurve};

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub enum CurveParams {
    Constant { data: ConstantCurve },
    Fixed { data: FixedCurve },
    Linear { data: LinearCurve },
}
