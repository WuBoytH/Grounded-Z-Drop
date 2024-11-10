use super::*;
use crate::func_links;

unsafe extern "C" fn holywater_get_fighter_kind(item: &mut L2CAgent) -> L2CValue {
    let table = &mut *(((item as *const L2CAgent as u64) + 0xc8) as *mut L2CValue);
    let kind = (*table)["kind_"].get_i32();
    let fighter_kind = if kind == *ITEM_KIND_RICHTERHOLYWATER {
        *FIGHTER_KIND_RICHTER
    }
    else {
        *FIGHTER_KIND_SIMON
    };
    fighter_kind.into()
}

pub static mut SIMON_HOLYWATER_THROW : usize = 0x792300;
pub static mut RICHTER_HOLYWATER_THROW : usize = 0x757e20;

#[skyline::hook(replace = SIMON_HOLYWATER_THROW)]
unsafe extern "C" fn simon_holywater_throw(item: &mut L2CAgent) -> L2CValue {
    holywater_throw_internal(item)
}

#[skyline::hook(replace = RICHTER_HOLYWATER_THROW)]
unsafe extern "C" fn richter_holywater_throw(item: &mut L2CAgent) -> L2CValue {
    holywater_throw_internal(item)
}

unsafe extern "C" fn holywater_throw_internal(item: &mut L2CAgent) -> L2CValue {
    let mut speed_x = KineticModule::get_sum_speed_x(item.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let mut speed_y = KineticModule::get_sum_speed_y(item.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);

    let table = &mut *(((item as *const L2CAgent as u64) + 0x458) as *mut L2CValue);
    if !table["is_special_throw_"].get_bool() {
        if 1e-5 <= speed_x.abs() {
            let kind = holywater_get_fighter_kind(item).get_i32();
            let angle = func_links::HOLYWATER::THROW_ANGLE_SIDE(FighterKind(kind));
            let speed_length = sv_math::vec2_length(speed_x, speed_y);
            let rad = angle.to_radians();
            speed_x = speed_length.abs() * rad.cos() * speed_x.signum();
            speed_y = speed_length.abs() * rad.sin() * speed_y.signum();
        }
        holywater_throw_internal_internal(item, speed_x.into(), speed_y.into());
    }
    else {
        item.clear_lua_stack();
        func_links::KineticEnergyControl::enable(item.lua_state_agent);
        item.clear_lua_stack();
        func_links::KineticEnergyControlRot::enable(item.lua_state_agent);
        KineticModule::clear_speed_all(item.module_accessor);

        ItemKineticModuleImpl::it_ai_move(
            item.module_accessor,
            &Vector2f{x: speed_x, y: speed_y},
            &Vector2f{x: -1.0, y: -1.0},
            &Vector2f{x: 0.0, y: 0.0},
            &Vector2f{x: 0.0, y: 0.0},
            &Vector2f{x: 0.0, y: 0.0},
            false,
            false
        );

        item.clear_lua_stack();
        lua_args!(item, 0.0);
        func_links::KineticEnergyGravity::set_accel(item.lua_state_agent, 0.0);

        let kind = holywater_get_fighter_kind(item).get_i32();
        let rot_speed = func_links::HOLYWATER::ROT_SPEED(FighterKind(kind));
        let lr = PostureModule::lr(item.module_accessor);

        item.clear_lua_stack();
        lua_args!(item, 0.0, 0.0, rot_speed * lr);
        func_links::KineticEnergyControlRot::set_rotation(item.lua_state_agent, &Vector3f{x: 0.0, y: 0.0, z: rot_speed * lr});

        item.clear_lua_stack();
        lua_args!(item, 0.0, 0.0, 0.0);
        func_links::KineticEnergyRot::set_rotation(item.lua_state_agent, &Vector3f{x: 0.0, y: 0.0, z: 0.0});
    }
    table[0x11b6eba1e3 as u64].assign(&L2CValue::Bool(false));

    MotionModule::change_motion(
        item.module_accessor,
        Hash40::new("throw"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );

    0.into()
}

unsafe extern "C" fn holywater_throw_internal_internal(item: &mut L2CAgent, speed_x: L2CValue, speed_y: L2CValue) {
    item.clear_lua_stack();
    func_links::Item::reset_gravity_energy_brake(item.lua_state_agent);
    KineticModule::clear_speed_all(item.module_accessor);
    KineticModule::add_speed(item.module_accessor, &Vector3f{x: speed_x.get_f32(), y: speed_y.get_f32(), z: 0.0});

    item.clear_lua_stack();
    lua_args!(item, 0.0, 0.0, 0.0);
    func_links::KineticEnergyControlRot::set_rotation(item.lua_state_agent, &Vector3f{x: 0.0, y: 0.0, z: 0.0});

    let kind = holywater_get_fighter_kind(item).get_i32();
    let rot_speed = func_links::HOLYWATER::REFLECT_SHIELD_ROT_SPEED(FighterKind(kind));
    let lr = PostureModule::lr(item.module_accessor);
    item.clear_lua_stack();
    lua_args!(item, 0.0, 0.0, rot_speed * lr);
    func_links::KineticEnergyRot::set_rotation(item.lua_state_agent, &Vector3f{x: 0.0, y: 0.0, z: rot_speed * lr});
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "item" {
        unsafe {
            let base = (*info.module.ModuleObject).module_base as usize;
            SIMON_HOLYWATER_THROW += base;
            RICHTER_HOLYWATER_THROW += base;

            skyline::install_hooks!(
                simon_holywater_throw,
                richter_holywater_throw
            );
        }
    }
}

pub fn install() {
    let _ = skyline::nro::add_hook(nro_hook);
}