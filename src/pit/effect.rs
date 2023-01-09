use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smashline::*;
use smash_script::*;
use smash::lua2cpp::L2CAgentBase;

#[acmd_script(//GlideStart
    agent = "pit", 
    script = "effect_glidestart", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn pit_glidestartgfx(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("pit_sword"), Hash40::new("swordr2"), -0, 0, 0, 0, 0, 0, 1, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("pit_sword"), Hash40::new("swordl"), -0, 0, 0, 0, 0, 0, 1, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("pit_fly_miracle_start"), Hash40::new("top"), 0, 7, 0, 0, 0, 0, 1, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_aura_light"), Hash40::new("top"), 0.0, 0, 0, 0, 0, 0, 5.8, true);
        macros::LAST_EFFECT_SET_COLOR(fighter, /*R*/ 2.0, /*G*/ 0.9, /*B*/ 0.04);
    }
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("pit_ikaros_wing_flare"), Hash40::new("s_wingl1"), -3, 0, 1, 0, 0, 0, 1, false);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("pit_ikaros_wing_flare"), Hash40::new("s_wingr1"), -3, 0, -1, 0, 0, 0, 1, false);
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("pit_fly_miracle_start"), true, true);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("pit_sword"), false, false);
    }
}

#[acmd_script(//GlideWingGFX
    agent = "pit", 
    script = "effect_glidewing", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn pit_glidegfx(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_aura_light"), false, false);
    }
}

#[acmd_script(//GlideAttack
    agent = "pit", 
    script = "effect_glideattack", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn pit_glideattackgfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("pit_sword"), Hash40::new("swordr2"), -0, 0, 0, 0, 90, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_pit_sword1"), Hash40::new("tex_pit_sword2"), 3, Hash40::new("swordr1"), 0.0, 0.0, -0.2, Hash40::new("swordr1"), 0.0, 10.4, -1.2, true, Hash40::new("pit_sword"), Hash40::new("swordr1"), 0.0, 0.0, 0.0, 0.0, 90.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_pit_sword1"), Hash40::new("tex_pit_sword2"), 4, Hash40::new("swordl"), 0.0, -0.9, -0.2, Hash40::new("swordl"), 0.0, -11.0, -1.2, true, Hash40::new("pit_sword"), Hash40::new("swordl"), 0.0, -0.9, 0.0, 180.0, 90.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        //macros::EFFECT_FOLLOW(fighter, Hash40::new("pit_atk_wind"), Hash40::new("top"), -7, 6, 2.3, -12, -42, 48, 1, true);
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("pit_sword"), true, true);
        //macros::EFFECT_OFF_KIND(fighter, Hash40::new("pit_atk_wind"), true, true);
        macros::AFTER_IMAGE_OFF(fighter, 3);
        macros::AFTER_IMAGE_OFF(fighter, 4);
    }
}  

#[acmd_script(//GlideLanding
    agent = "pit", 
    script = "effect_glidelanding", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn pit_glidelandinggfx(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("pit_ikaros_wing_flare"), true, true);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("pit_fly_miracle_start"), true, true);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("pit_sword"), false, false);
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script(//GlideEnd
    agent = "pit", 
    script = "effect_glideend", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn pit_glideendgfx(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("pit_ikaros_wing_flare"), true, true);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("pit_fly_miracle_start"), true, true);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("pit_sword"), false, false);
    }
}   

pub fn install() {
    smashline::install_acmd_scripts!(
        pit_glidestartgfx,
        pit_glidegfx,
        pit_glideattackgfx,
        pit_glidelandinggfx,
        pit_glideendgfx
    );
}