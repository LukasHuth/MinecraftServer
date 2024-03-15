use crate::datatypes::datastructs::JSONTextComponent;

pub struct LoginDisconnect {
    pub reason: JSONTextComponent,
}
// sent as { "text": <reason> } ig (JSONTextComponent)
