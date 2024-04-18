#![deny(missing_docs)]
//! This crate provides structs for every entity in Minecraft
//!
//! # Example
//! 
//! ```rust
//! use entmet_lib::entities::entity_types::EntityEnum;
//! use entmet_lib::entities::entity::Entity;
//! use entmet_lib::entities::entity::Cow;
//!
//! let cow = Cow::default();
//! let default_entity = Entity::default();
//! let test_entity_enum = EntityEnum::Cow(cow.clone());
//! assert!(matches!(test_entity_enum, EntityEnum::Cow(_)));
//! assert_eq!(default_entity.silent, cow.silent);
//! assert_eq!(default_entity.pose, cow.pose);
//! assert_eq!(default_entity.custom_name, cow.custom_name);
//! ```

/// Module that contains all the entity structs
pub mod entities;
/// Module that contains all neccessary datastructs
pub mod datatypes;
