//! =====================================================================================
//!
//!       Filename:  util/mod.rs
//!
//!    Description:  Various utils, like aliases.
//!
//!        Version:  1.0
//!        Created:  13/06/16 22:43:05
//!       Revision:  none
//!       Compiler:  rust
//!
//!         Author:  Anicka Burova
//!
//! =====================================================================================

use collision::{Aabb2};
use cgmath::{Point2,Vector2};

pub type Point = Point2<i32>;
pub type Offset = Vector2<i32>;
pub type Bounds = Aabb2<i32>;
