pub use {
    smash::{
        phx::*,
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue, L2CAgent}
    },
    smash_script::*
};

mod holywater;

pub fn install() {
    holywater::install();
}