pub use smash::{
    lua2cpp::*,
    hash40,
    app::lua_bind::*,
    lib::lua_const::*
};

mod item;

pub fn install() {
    item::install();
}