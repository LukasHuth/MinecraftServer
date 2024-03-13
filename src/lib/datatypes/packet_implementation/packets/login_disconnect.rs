use crate::datatypes::datastructs::JSONTextComponent;

pub struct LoginDisconnect {
    reason: JSONTextComponent,
}
// sent as { "text": <reason> } ig (JSONTextComponent)
