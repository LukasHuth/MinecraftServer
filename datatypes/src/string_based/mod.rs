#[derive(Clone, Debug)]
/// A wrapper holding string
#[derive(PartialEq, Eq, Hash)]
pub struct String(std::string::String);
/// A wrapper holding a string that should be valid json
pub struct JSONTextComponent(String); // TODO: As JSON;
/// Represents a namespaced location in the form of `namespace:value`.
///
/// If the namespace is not provided, it defaults to `minecraft`, meaning `thing`
/// is equivalent to `minecraft:thing`. Custom content should always be in its own
/// namespace, not the default one.
///
/// Both the namespace and value can consist of lowercase alphanumeric characters
/// (a-z and 0-9), dot (.), dash (-), and underscore (_). Additionally, values can
/// use slash (/). The naming convention is lower_case_with_underscores.
///
/// For ease of determining whether a namespace or value is valid, here are regular
/// expressions for each:
///
/// - Namespace: `[a-z0-9.-_]`
/// - Value: `[a-z0-9.-_/]`
#[derive(Clone)]
#[derive(PartialEq, Eq, Hash)]
pub struct Identifier(String);
mod implementations;
mod important_functions;
