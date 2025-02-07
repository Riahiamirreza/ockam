use crate::{Action, Expr, ResourceType};
use minicbor::{Decode, Encode};

#[derive(Debug, Decode, Encode, PartialEq, Eq)]
#[rustfmt::skip]
#[cbor(map)]
pub struct ResourceTypePolicy {
    #[n(1)] pub resource_type: ResourceType,
    #[n(2)] pub action: Action,
    #[n(3)] pub expression: Expr,
}

impl ResourceTypePolicy {
    pub fn new(resource_type: ResourceType, action: Action, expression: Expr) -> Self {
        ResourceTypePolicy {
            resource_type,
            action,
            expression,
        }
    }
}
