//! This module contains all data structures needed to represent Commands
//! Source: <https://wiki.vg/Command_Data>

use std::{collections::HashMap, rc::Rc};

use binary_utils::{DataReader, DataWriter};
use datatypes::{
    Array, Boolean, Byte, Double, Float, Identifier, ImportantFunctions, Int, Long, VarInt,
};

/// A Node of a command
#[derive(PartialEq, Eq, Hash)]
pub enum CommandNode {
    /// The root of all commands
    Root {
        /// The children
        children: Vec<Rc<CommandNode>>,
    },
    /// A literal, for example "ban" or "execute"
    Literal {
        /// The children
        children: Vec<Rc<CommandNode>>,
        /// Whether the command is executeable at this point or not
        executeable: bool,
        /// The node this one is redirecting to
        redirect_node: Option<Rc<CommandNode>>,
        /// The name of the node
        name: String,
    },
    /// An Argument, which defines an accepted datatype
    Argument {
        /// The children
        children: Vec<Rc<CommandNode>>,
        /// Whether the command is executeable at this point or not
        executeable: bool,
        /// The node this one is redirecting to
        redirect_node: Option<Rc<CommandNode>>,
        /// The name of the node
        name: String,
        /// Data of the argument
        parser_data: CommandArgumentParser,
        ///
        suggestion_type: Option<Identifier>,
    },
}
/// A value of a Argument
#[allow(missing_docs)]
#[derive(PartialEq, Eq, Hash, Clone)]
pub enum CommandArgumentParser {
    Bool,
    Float {
        min: Option<Float>,
        max: Option<Float>,
    },
    Double {
        min: Option<Double>,
        max: Option<Double>,
    },
    Integer {
        min: Option<Int>,
        max: Option<Int>,
    },
    Long {
        min: Option<Long>,
        max: Option<Long>,
    },
    String {
        greed: StringGreed,
    },
    Entity {
        single: bool,
        only_players: bool,
    },
    GameProfile,
    BlockPos,
    ColumnPos,
    Vec3,
    Vec2,
    BlockState,
    BlockPredicate,
    ItemStack,
    ItemPredicate,
    Color,
    Component,
    Style,
    Message,
    Nbt,
    NbtTag,
    NbtPath,
    Objective,
    ObjectiveCriteria,
    Operation,
    Particle,
    Angle,
    Rotation,
    ScoreboardSlot,
    ScoreHolder {
        multiple: Boolean,
    },
    Swizzle,
    Team,
    ItemSlot,
    ResourceLocation,
    Function,
    EntityAnchor,
    IntRange,
    FloatRange,
    Dimension,
    Gamemode,
    Time {
        min: Int,
    },
    ResourceOrTag {
        registry: Identifier,
    },
    ResourceOrTagKey {
        registry: Identifier,
    },
    Resource {
        registry: Identifier,
    },
    ResourceKey {
        registry: Identifier,
    },
    TemplateMirror,
    TemplateRotation,
    Heightmap,
    Uuid,
}
impl CommandArgumentParser {
    fn get_id(&self) -> usize {
        0
    }
}
impl DataWriter for CommandArgumentParser {
    async fn write(
        &self,
        writer: &mut (impl tokio::io::AsyncWrite + Unpin),
    ) -> binary_utils::Result<()> {
        VarInt::new(self.get_id() as i32).write(writer).await?;
        match self {
            Self::Float { min, max } => {
                if let Some(min) = min {
                    min.write(writer).await?
                }
                if let Some(max) = max {
                    max.write(writer).await?
                }
            }
            Self::Double { min, max } => {
                if let Some(min) = min {
                    min.write(writer).await?
                }
                if let Some(max) = max {
                    max.write(writer).await?
                }
            }
            Self::Integer { min, max } => {
                if let Some(min) = min {
                    min.write(writer).await?
                }
                if let Some(max) = max {
                    max.write(writer).await?
                }
            }
            Self::Long { min, max } => {
                if let Some(min) = min {
                    min.write(writer).await?
                }
                if let Some(max) = max {
                    max.write(writer).await?
                }
            }
            Self::String { greed } => {
                let value = match greed {
                    StringGreed::SingleWord => 0,
                    StringGreed::QuoteablePhrase => 1,
                    StringGreed::GreedyPhrase => 2,
                };
                VarInt::new(value).write(writer).await?;
            }
            Self::Entity {
                single,
                only_players,
            } => {
                let value =
                    if *single { 0x01 } else { 0x00 } | if *only_players { 0x02 } else { 0x00 };
                VarInt::new(value).write(writer).await?;
            }
            Self::ScoreHolder { multiple } => multiple.write(writer).await?,
            Self::Time { min } => min.write(writer).await?,
            Self::ResourceOrTag { registry } => registry.write(writer).await?,
            Self::ResourceOrTagKey { registry } => registry.write(writer).await?,
            Self::Resource { registry } => registry.write(writer).await?,
            Self::ResourceKey { registry } => registry.write(writer).await?,
            _ => (),
        }
        Ok(())
    }
}
#[allow(missing_docs)]
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum StringGreed {
    SingleWord,
    QuoteablePhrase,
    GreedyPhrase,
}
#[allow(missing_docs)]
pub mod node_format_data {
    pub enum NodeFlags {
        Root,
        Literal,
        Argument,
        IsExecuteable,
        HasRedirect,
        HasSuggestionType,
    }
}
#[derive(Clone)]
/// A node that represents a part of a command
pub struct CommandListNode {
    /// Some flags of the node
    pub flags: Byte,
    /// The amount of children
    pub children_count: VarInt,
    /// A list of the indices of all children
    pub children: Array<VarInt>,
    /// A possible redirect to another node
    pub redirect_node: Option<VarInt>,
    /// A name, only used on arguments and literals
    pub name: Option<datatypes::String>,
    /// The type of the literal
    pub parser: Option<CommandArgumentParser>,
    /// a possible suggestion
    pub suggestion_type: Option<Identifier>,
}
impl CommandNode {
    /// converts a root element to a sendable list
    pub fn to_list(&self) -> Result<(Vec<CommandListNode>, usize), ()> {
        match self {
            Self::Root { children } => {
                let mut list = Vec::new();
                let mut map = HashMap::new();
                let mut count = 0;
                map.insert(self, 0);
                for child in children {
                    child.get_id_and_generate_children(&mut list, &mut map, &mut count)?;
                }
                Ok((list, 0))
            }
            _ => Err(()),
        }
    }
    fn get_id_and_generate_children<'a>(
        &'a self,
        list: &mut Vec<CommandListNode>,
        map: &mut HashMap<&'a CommandNode, usize>,
        count: &mut usize,
    ) -> Result<usize, ()> {
        if map.contains_key(self) {
            return map.get(self).ok_or(()).map(|&e| e);
        }
        match self {
            Self::Root { .. } => Err(()),
            Self::Literal {
                children,
                executeable,
                redirect_node,
                name,
            } => {
                *count += 1;
                let this_id = *count;
                map.insert(self, this_id);
                let mut children_list = Vec::new();
                for child in children {
                    children_list.push(child.get_id_and_generate_children(list, map, count)?);
                }
                let children_count = VarInt::new(children_list.len() as i32);
                let children = Array::new(
                    children_list
                        .into_iter()
                        .map(|e| VarInt::new(e as i32))
                        .collect(),
                );
                let flags = 0x3
                    | if *executeable { 0x4 } else { 0 }
                    | if let Some(_) = redirect_node { 0x8 } else { 0 };
                let flags = Byte::new(flags);
                let name = Some(datatypes::String::new(name.to_owned()));
                let redirect_node = if let Some(node) = redirect_node {
                    Some(VarInt::new(
                        node.get_id_and_generate_children(list, map, count)? as i32,
                    ))
                } else {
                    None
                };
                let instance = CommandListNode {
                    children,
                    children_count,
                    flags,
                    name,
                    parser: None,
                    redirect_node,
                    suggestion_type: None,
                };
                if list.len() <= this_id {
                    // Will be overwritten in a later iteration
                    list.resize(this_id + 1, instance.clone());
                }
                list[this_id] = instance;
                Ok(this_id)
            }
            Self::Argument {
                children,
                executeable,
                redirect_node,
                name,
                parser_data,
                suggestion_type,
            } => {
                *count += 1;
                let this_id = *count;
                map.insert(self, this_id);
                let mut children_list = Vec::new();
                for child in children {
                    children_list.push(child.get_id_and_generate_children(list, map, count)?);
                }
                let children_count = VarInt::new(children_list.len() as i32);
                let children = Array::new(
                    children_list
                        .into_iter()
                        .map(|e| VarInt::new(e as i32))
                        .collect(),
                );
                let flags = 0x3
                    | if *executeable { 0x4 } else { 0 }
                    | if let Some(_) = redirect_node { 0x8 } else { 0 }
                    | if let Some(_) = suggestion_type {
                        0x10
                    } else {
                        0
                    };
                let flags = Byte::new(flags);
                let name = Some(datatypes::String::new(name.to_owned()));
                let redirect_node = if let Some(node) = redirect_node {
                    Some(VarInt::new(
                        node.get_id_and_generate_children(list, map, count)? as i32,
                    ))
                } else {
                    None
                };
                let instance = CommandListNode {
                    children,
                    children_count,
                    flags,
                    name,
                    parser: Some(parser_data.to_owned()),
                    redirect_node,
                    suggestion_type: suggestion_type.to_owned(),
                };
                if list.len() <= this_id {
                    // Will be overwritten in a later iteration
                    list.resize(this_id + 1, instance.clone());
                }
                list[this_id] = instance;
                Ok(this_id)
            }
        }
    }
}
impl DataReader for CommandListNode {
    async fn read(_reader: &mut (impl tokio::io::AsyncRead + Unpin)) -> binary_utils::Result<Self> {
        unreachable!("This struct should never be read")
    }
}
impl DataWriter for CommandListNode {
    async fn write(
        &self,
        writer: &mut (impl tokio::io::AsyncWrite + Unpin),
    ) -> binary_utils::Result<()> {
        self.flags.write(writer).await?;
        self.children_count.write(writer).await?;
        self.children.write(writer).await?;
        if let Some(ref redirect_node) = self.redirect_node {
            redirect_node.write(writer).await?
        }
        if let Some(ref name) = self.name {
            name.write(writer).await?
        }
        if let Some(ref parser) = self.parser {
            parser.write(writer).await?
        }
        if let Some(ref suggestion_type) = self.suggestion_type {
            suggestion_type.write(writer).await?
        }
        Ok(())
    }
}
