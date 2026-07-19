//! Port of: engine/engine_name.c
//! IR hash: adc2f24e872d94f7
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: _getnumadr (engine/engine_name.c:28)
/// Calls: FilePath::size, mjCActuator::act
#[allow(unused_variables, non_snake_case)]
pub fn getnumadr(m: *const mjModel, r#type: u32, padr: *mut *mut i32, mapadr: *mut i32) -> i32 {
    const MJ_LOAD_MULTIPLE: i64 = 2;
    // mjOBJ enum values
    const BODY: u32 = 1; const XBODY: u32 = 2; const JOINT: u32 = 3;
    const GEOM: u32 = 5; const SITE: u32 = 6; const CAMERA: u32 = 7;
    const LIGHT: u32 = 8; const FLEX: u32 = 9; const MESH: u32 = 10;
    const SKIN: u32 = 11; const HFIELD: u32 = 12; const TEXTURE: u32 = 13;
    const MATERIAL: u32 = 14; const PAIR: u32 = 15; const EXCLUDE: u32 = 16;
    const EQUALITY: u32 = 17; const TENDON: u32 = 18; const ACTUATOR: u32 = 19;
    const SENSOR: u32 = 20; const NUMERIC: u32 = 21; const TEXT: u32 = 22;
    const TUPLE: u32 = 23; const KEY: u32 = 24; const PLUGIN: u32 = 25;

    // SAFETY: caller guarantees m, padr, mapadr valid
    unsafe {
        let mut num: i32 = -1;
        *mapadr = (*m).nnames_map as i32;
        let t = r#type;

        // The C uses switch fallthrough: entering at type X subtracts all subsequent sizes.
        // We simulate this with sequential checks: subtract if t <= threshold.

        // BODY/XBODY
        if t <= BODY || t == XBODY {
            *mapadr -= (MJ_LOAD_MULTIPLE * (*m).nbody) as i32;
            if num < 0 && (t == BODY || t == XBODY) { *padr = (*m).name_bodyadr; num = (*m).nbody as i32; }
        }
        // JOINT
        if t <= JOINT {
            *mapadr -= (MJ_LOAD_MULTIPLE * (*m).njnt) as i32;
            if num < 0 && t == JOINT { *padr = (*m).name_jntadr; num = (*m).njnt as i32; }
        }
        // GEOM
        if t <= GEOM {
            *mapadr -= (MJ_LOAD_MULTIPLE * (*m).ngeom) as i32;
            if num < 0 && t == GEOM { *padr = (*m).name_geomadr; num = (*m).ngeom as i32; }
        }
        // SITE
        if t <= SITE {
            *mapadr -= (MJ_LOAD_MULTIPLE * (*m).nsite) as i32;
            if num < 0 && t == SITE { *padr = (*m).name_siteadr; num = (*m).nsite as i32; }
        }
        // CAMERA
        if t <= CAMERA {
            *mapadr -= (MJ_LOAD_MULTIPLE * (*m).ncam) as i32;
            if num < 0 && t == CAMERA { *padr = (*m).name_camadr; num = (*m).ncam as i32; }
        }
        // LIGHT
        if t <= LIGHT {
            *mapadr -= (MJ_LOAD_MULTIPLE * (*m).nlight) as i32;
            if num < 0 && t == LIGHT { *padr = (*m).name_lightadr; num = (*m).nlight as i32; }
        }
        // FLEX
        if t <= FLEX {
            *mapadr -= (MJ_LOAD_MULTIPLE * (*m).nflex) as i32;
            if num < 0 && t == FLEX { *padr = (*m).name_flexadr; num = (*m).nflex as i32; }
        }
        // MESH
        if t <= MESH {
            *mapadr -= (MJ_LOAD_MULTIPLE * (*m).nmesh) as i32;
            if num < 0 && t == MESH { *padr = (*m).name_meshadr; num = (*m).nmesh as i32; }
        }
        // SKIN
        if t <= SKIN {
            *mapadr -= (MJ_LOAD_MULTIPLE * (*m).nskin) as i32;
            if num < 0 && t == SKIN { *padr = (*m).name_skinadr; num = (*m).nskin as i32; }
        }
        // HFIELD
        if t <= HFIELD {
            *mapadr -= (MJ_LOAD_MULTIPLE * (*m).nhfield) as i32;
            if num < 0 && t == HFIELD { *padr = (*m).name_hfieldadr; num = (*m).nhfield as i32; }
        }
        // TEXTURE
        if t <= TEXTURE {
            *mapadr -= (MJ_LOAD_MULTIPLE * (*m).ntex) as i32;
            if num < 0 && t == TEXTURE { *padr = (*m).name_texadr; num = (*m).ntex as i32; }
        }
        // MATERIAL
        if t <= MATERIAL {
            *mapadr -= (MJ_LOAD_MULTIPLE * (*m).nmat) as i32;
            if num < 0 && t == MATERIAL { *padr = (*m).name_matadr; num = (*m).nmat as i32; }
        }
        // PAIR
        if t <= PAIR {
            *mapadr -= (MJ_LOAD_MULTIPLE * (*m).npair) as i32;
            if num < 0 && t == PAIR { *padr = (*m).name_pairadr; num = (*m).npair as i32; }
        }
        // EXCLUDE
        if t <= EXCLUDE {
            *mapadr -= (MJ_LOAD_MULTIPLE * (*m).nexclude) as i32;
            if num < 0 && t == EXCLUDE { *padr = (*m).name_excludeadr; num = (*m).nexclude as i32; }
        }
        // EQUALITY
        if t <= EQUALITY {
            *mapadr -= (MJ_LOAD_MULTIPLE * (*m).neq) as i32;
            if num < 0 && t == EQUALITY { *padr = (*m).name_eqadr; num = (*m).neq as i32; }
        }
        // TENDON
        if t <= TENDON {
            *mapadr -= (MJ_LOAD_MULTIPLE * (*m).ntendon) as i32;
            if num < 0 && t == TENDON { *padr = (*m).name_tendonadr; num = (*m).ntendon as i32; }
        }
        // ACTUATOR
        if t <= ACTUATOR {
            *mapadr -= (MJ_LOAD_MULTIPLE * (*m).nu) as i32;
            if num < 0 && t == ACTUATOR { *padr = (*m).name_actuatoradr; num = (*m).nu as i32; }
        }
        // SENSOR
        if t <= SENSOR {
            *mapadr -= (MJ_LOAD_MULTIPLE * (*m).nsensor) as i32;
            if num < 0 && t == SENSOR { *padr = (*m).name_sensoradr; num = (*m).nsensor as i32; }
        }
        // NUMERIC
        if t <= NUMERIC {
            *mapadr -= (MJ_LOAD_MULTIPLE * (*m).nnumeric) as i32;
            if num < 0 && t == NUMERIC { *padr = (*m).name_numericadr; num = (*m).nnumeric as i32; }
        }
        // TEXT
        if t <= TEXT {
            *mapadr -= (MJ_LOAD_MULTIPLE * (*m).ntext) as i32;
            if num < 0 && t == TEXT { *padr = (*m).name_textadr; num = (*m).ntext as i32; }
        }
        // TUPLE
        if t <= TUPLE {
            *mapadr -= (MJ_LOAD_MULTIPLE * (*m).ntuple) as i32;
            if num < 0 && t == TUPLE { *padr = (*m).name_tupleadr; num = (*m).ntuple as i32; }
        }
        // KEY
        if t <= KEY {
            *mapadr -= (MJ_LOAD_MULTIPLE * (*m).nkey) as i32;
            if num < 0 && t == KEY { *padr = (*m).name_keyadr; num = (*m).nkey as i32; }
        }
        // PLUGIN
        if t <= PLUGIN {
            *mapadr -= (MJ_LOAD_MULTIPLE * (*m).nplugin) as i32;
            if num < 0 && t == PLUGIN { *padr = (*m).name_pluginadr; num = (*m).nplugin as i32; }
        }

        // default: if nothing matched
        if num < 0 {
            *padr = std::ptr::null_mut();
            num = 0;
        }

        num
    }
}

/// C: mj_hashString (engine/engine_name.h:30)
#[allow(unused_variables, non_snake_case)]
pub fn mj_hash_string(s: *const i8, n: u64) -> u64 {
    let mut h: u64 = 5381;
    let mut ptr = s;
    // SAFETY: s is a null-terminated C string, we read bytes until null
    unsafe {
        loop {
            let c = *ptr as u8;
            if c == 0 {
                break;
            }
            h = ((h << 5) + h) ^ (c as u64);
            ptr = ptr.add(1);
        }
    }
    h % n
}

/// C: mj_name2id (engine/engine_name.h:33)
/// Calls: _getnumadr, mj_hashString
#[allow(unused_variables, non_snake_case)]
pub fn mj_name2id(m: *const mjModel, r#type: i32, name: *const i8) -> i32 {
    todo!() // mj_name2id
}

/// C: mj_id2name (engine/engine_name.h:36)
/// Calls: _getnumadr
#[allow(unused_variables, non_snake_case)]
pub fn mj_id2name(m: *const mjModel, r#type: i32, id: i32) -> *const i8 {
    // SAFETY: caller guarantees m is valid
    unsafe {
        let mut mapadr: i32 = 0;
        let mut adr: *mut i32 = std::ptr::null_mut();

        // get number of objects and name addresses
        let num = getnumadr(m, r#type as u32, &mut adr, &mut mapadr);

        // id is in [0, num) and the found name is not the empty string
        if id >= 0 && id < num && *(*m).names.add(*adr.add(id as usize) as usize) != 0 {
            return (*m).names.add(*adr.add(id as usize) as usize);
        }

        std::ptr::null()
    }
}

