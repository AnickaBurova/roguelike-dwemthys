//! =====================================================================================
//!
//!       Filename:  component/mod.rs
//!
//!    Description:  Component trait to update actors.
//!
//!        Version:  1.0
//!        Created:  13/06/16 22:43:05
//!       Revision:  none
//!       Compiler:  rust
//!
//!         Author:  Anicka Burova
//!
//! =====================================================================================

use game::Game;
use world::World;
use actor::Actor;

pub trait Component{
    fn update(&mut self, &mut Actor, &Game, &mut World);
}
