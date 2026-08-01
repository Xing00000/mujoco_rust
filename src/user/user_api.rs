//! Port of: user/user_api.cc
//! IR hash: 9343293228317031
//! CODEGEN: source paths, owners, and callable names are locked.

use crate::types::*;

/// C: mjs_getTimer (user/user_api.cc:515)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_getTimer(s: *mut mjSpec) -> *const f64 {
    if s.is_null() {
        return std::ptr::null();
    }
    // SAFETY: s is a valid mjSpec pointer (checked above).
    // element points to the containing mjCModel (C++ static_cast pattern).
    unsafe {
        let model_c: *mut mjCModel = (*s).element as *mut mjCModel;
        (*model_c).timer.as_ptr()
    }
}

/// C: mjs_setToMotor (user/user_api.h:175)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_setToMotor(actuator: *mut mjsActuator) -> *const i8 {
    // SAFETY: caller guarantees actuator is a valid pointer to mjsActuator.
    unsafe {
        // unit gain
        (*actuator).gainprm[0] = 1.0;

        // implied parameters
        std::ptr::write(std::ptr::addr_of_mut!((*actuator).dyntype) as *mut i32, 0);   // mjDYN_NONE
        std::ptr::write(std::ptr::addr_of_mut!((*actuator).gaintype) as *mut i32, 0);  // mjGAIN_FIXED
        std::ptr::write(std::ptr::addr_of_mut!((*actuator).biastype) as *mut i32, 0);  // mjBIAS_NONE
    }
    b"\0".as_ptr() as *const i8
}

/// C: mjs_setToPosition (user/user_api.h:178)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_setToPosition(actuator: *mut mjsActuator, kp: f64, kv: *mut f64, dampratio: *mut f64, timeconst: *mut f64, inheritrange: f64) -> *const i8 {
    // SAFETY: caller guarantees actuator is valid. kv/dampratio/timeconst may be null.
    unsafe {
        (*actuator).gainprm[0] = kp;
        (*actuator).biasprm[1] = -kp;

        // set biasprm[2]; negative: regular damping, positive: dampratio
        if !dampratio.is_null() && !kv.is_null() {
            return b"kv and dampratio cannot both be defined\0".as_ptr() as *const i8;
        }

        if !kv.is_null() {
            if *kv < 0.0 {
                return b"kv cannot be negative\0".as_ptr() as *const i8;
            }
            (*actuator).biasprm[2] = -(*kv);
        }
        if !dampratio.is_null() {
            if *dampratio < 0.0 {
                return b"dampratio cannot be negative\0".as_ptr() as *const i8;
            }
            (*actuator).biasprm[2] = *dampratio;
        }
        if !timeconst.is_null() {
            if *timeconst < 0.0 {
                return b"timeconst cannot be negative\0".as_ptr() as *const i8;
            }
            (*actuator).dynprm[0] = *timeconst;
            let dynval: i32 = if *timeconst == 0.0 { 0 } else { 3 }; // mjDYN_NONE : mjDYN_FILTEREXACT
            std::ptr::write(std::ptr::addr_of_mut!((*actuator).dyntype) as *mut i32, dynval);
        }
        (*actuator).inheritrange = inheritrange;

        if inheritrange > 0.0 {
            if (*actuator).ctrlrange[0] != 0.0 || (*actuator).ctrlrange[1] != 0.0 {
                return b"ctrlrange and inheritrange cannot both be defined\0".as_ptr() as *const i8;
            }
        }

        std::ptr::write(std::ptr::addr_of_mut!((*actuator).gaintype) as *mut i32, 0);  // mjGAIN_FIXED
        std::ptr::write(std::ptr::addr_of_mut!((*actuator).biastype) as *mut i32, 1);  // mjBIAS_AFFINE
    }
    b"\0".as_ptr() as *const i8
}

/// C: mjs_setToIntVelocity (user/user_api.h:182)
/// Calls: cxx:_mjs_setToPosition
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_setToIntVelocity(actuator: *mut mjsActuator, kp: f64, kv: *mut f64, dampratio: *mut f64, timeconst: *mut f64, inheritrange: f64) -> *const i8 {
    // SAFETY: caller guarantees actuator is valid. kv/dampratio/timeconst may be null.
    unsafe {
        let err = mjs_setToPosition(actuator, kp, kv, dampratio, timeconst, inheritrange);
        // mjs_setToPosition returns "" on success, error string on failure
        // but we ignore its return and always continue (matches C behavior)
        let _ = err;

        std::ptr::write(std::ptr::addr_of_mut!((*actuator).dyntype) as *mut i32, 1);  // mjDYN_INTEGRATOR
        (*actuator).actlimited = 1;

        if inheritrange > 0.0 {
            if (*actuator).actrange[0] != 0.0 || (*actuator).actrange[1] != 0.0 {
                return b"actrange and inheritrange cannot both be defined\0".as_ptr() as *const i8;
            }
        }
    }
    b"\0".as_ptr() as *const i8
}

/// C: mjs_setToCylinder (user/user_api.h:192)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_setToCylinder(actuator: *mut mjsActuator, timeconst: f64, bias: f64, area: f64, diameter: f64) -> *const i8 {
    // SAFETY: caller guarantees actuator is valid.
    unsafe {
        (*actuator).dynprm[0] = timeconst;
        (*actuator).biasprm[0] = bias;
        (*actuator).gainprm[0] = area;
        if diameter >= 0.0 {
            (*actuator).gainprm[0] = std::f64::consts::PI / 4.0 * diameter * diameter;
        }
        std::ptr::write(std::ptr::addr_of_mut!((*actuator).dyntype) as *mut i32, 2);   // mjDYN_FILTER
        std::ptr::write(std::ptr::addr_of_mut!((*actuator).gaintype) as *mut i32, 0);  // mjGAIN_FIXED
        std::ptr::write(std::ptr::addr_of_mut!((*actuator).biastype) as *mut i32, 1);  // mjBIAS_AFFINE
    }
    b"\0".as_ptr() as *const i8
}

/// C: mjs_setToMuscle (user/user_api.h:196)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_setToMuscle(actuator: *mut mjsActuator, timeconst: *mut f64, tausmooth: f64, range: *mut f64, force: f64, scale: f64, lmin: f64, lmax: f64, vmax: f64, fpmax: f64, fvmax: f64) -> *const i8 {
    // SAFETY: caller guarantees actuator is valid. timeconst and range are non-null arrays.
    unsafe {
        // set muscle defaults if same as global defaults
        if (*actuator).dynprm[0] == 1.0 { (*actuator).dynprm[0] = 0.01; }    // tau act
        if (*actuator).dynprm[1] == 0.0 { (*actuator).dynprm[1] = 0.04; }    // tau deact
        if (*actuator).gainprm[0] == 1.0 { (*actuator).gainprm[0] = 0.75; }  // range[0]
        if (*actuator).gainprm[1] == 0.0 { (*actuator).gainprm[1] = 1.05; }  // range[1]
        if (*actuator).gainprm[2] == 0.0 { (*actuator).gainprm[2] = -1.0; }  // force
        if (*actuator).gainprm[3] == 0.0 { (*actuator).gainprm[3] = 200.0; } // scale
        if (*actuator).gainprm[4] == 0.0 { (*actuator).gainprm[4] = 0.5; }   // lmin
        if (*actuator).gainprm[5] == 0.0 { (*actuator).gainprm[5] = 1.6; }   // lmax
        if (*actuator).gainprm[6] == 0.0 { (*actuator).gainprm[6] = 1.5; }   // vmax
        if (*actuator).gainprm[7] == 0.0 { (*actuator).gainprm[7] = 1.3; }   // fpmax
        if (*actuator).gainprm[8] == 0.0 { (*actuator).gainprm[8] = 1.2; }   // fvmax

        if tausmooth < 0.0 {
            return b"muscle tausmooth cannot be negative\0".as_ptr() as *const i8;
        }

        (*actuator).dynprm[2] = tausmooth;
        if *timeconst.add(0) >= 0.0 { (*actuator).dynprm[0] = *timeconst.add(0); }
        if *timeconst.add(1) >= 0.0 { (*actuator).dynprm[1] = *timeconst.add(1); }
        if *range.add(0) >= 0.0 { (*actuator).gainprm[0] = *range.add(0); }
        if *range.add(1) >= 0.0 { (*actuator).gainprm[1] = *range.add(1); }
        if force >= 0.0 { (*actuator).gainprm[2] = force; }
        if scale >= 0.0 { (*actuator).gainprm[3] = scale; }
        if lmin >= 0.0 { (*actuator).gainprm[4] = lmin; }
        if lmax >= 0.0 { (*actuator).gainprm[5] = lmax; }
        if vmax >= 0.0 { (*actuator).gainprm[6] = vmax; }
        if fpmax >= 0.0 { (*actuator).gainprm[7] = fpmax; }
        if fvmax >= 0.0 { (*actuator).gainprm[8] = fvmax; }

        // biasprm = gainprm
        let mut n: i32 = 0;
        while n < 9 {
            (*actuator).biasprm[n as usize] = (*actuator).gainprm[n as usize];
            n += 1;
        }

        std::ptr::write(std::ptr::addr_of_mut!((*actuator).dyntype) as *mut i32, 4);   // mjDYN_MUSCLE
        std::ptr::write(std::ptr::addr_of_mut!((*actuator).gaintype) as *mut i32, 2);  // mjGAIN_MUSCLE
        std::ptr::write(std::ptr::addr_of_mut!((*actuator).biastype) as *mut i32, 2);  // mjBIAS_MUSCLE
    }
    b"\0".as_ptr() as *const i8
}

/// C: mjs_setToAdhesion (user/user_api.h:201)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_setToAdhesion(actuator: *mut mjsActuator, gain: f64) -> *const i8 {
    // SAFETY: caller guarantees actuator is valid.
    unsafe {
        (*actuator).gainprm[0] = gain;
        (*actuator).ctrllimited = 1;
        std::ptr::write(std::ptr::addr_of_mut!((*actuator).gaintype) as *mut i32, 0);  // mjGAIN_FIXED
        std::ptr::write(std::ptr::addr_of_mut!((*actuator).biastype) as *mut i32, 0);  // mjBIAS_NONE

        if gain < 0.0 {
            return b"adhesion gain cannot be negative\0".as_ptr() as *const i8;
        }
        if (*actuator).ctrlrange[0] < 0.0 || (*actuator).ctrlrange[1] < 0.0 {
            return b"adhesion control range cannot be negative\0".as_ptr() as *const i8;
        }
    }
    b"\0".as_ptr() as *const i8
}

/// C: mjs_setToDCMotor (user/user_api.h:204)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_setToDCMotor(actuator: *mut mjsActuator, motorconst: *mut f64, resistance: f64, nominal: *mut f64, saturation: *mut f64, inductance: *mut f64, cogging: *mut f64, controller: *mut f64, thermal: *mut f64, lugre: *mut f64, input_mode: i32) -> *const i8 {
    // SAFETY: all pointer dereferences follow the C original's null-check pattern.
    // Caller guarantees actuator is valid; optional params are null-checked before access.
    unsafe {
        let mut R = resistance;
        let Kt: f64 = if !motorconst.is_null() { *motorconst.add(0) } else { 0.0 };
        let mut Ke: f64 = if !motorconst.is_null() { *motorconst.add(1) } else { 0.0 };
        let vn: f64 = if !nominal.is_null() { *nominal.add(0) } else { 0.0 };
        let tau0: f64 = if !nominal.is_null() { *nominal.add(1) } else { 0.0 };
        let omega0: f64 = if !nominal.is_null() { *nominal.add(2) } else { 0.0 };

        // derive Ke from nominal: omega0 = vn*Ke / (Ke^2 + R*B)
        if vn > 0.0 && Ke <= 0.0 && omega0 > 0.0 {
            // viscous damping (linear)
            let B = (*actuator).damping[0];

            if B > 0.0 && R > 0.0 {
                // R known: solve quadratic Ke^2*omega0 - Ke*vn + R*B*omega0 = 0
                let disc = vn * vn - 4.0 * R * B * omega0 * omega0;
                Ke = if disc > 0.0 { (vn + f64::sqrt(disc)) / (2.0 * omega0) } else { vn / omega0 };
            } else if B > 0.0 && tau0 > 0.0 {
                // R from nominal (tau0 = Ke*vn/R, so R = Ke*vn/tau0)
                let Ke_exact = vn / omega0 - vn * B / tau0;
                Ke = if Ke_exact > 0.0 { Ke_exact } else { vn / omega0 };
            } else {
                // B = 0 or insufficient data for B-correction: omega0 = vn/Ke
                Ke = vn / omega0;
            }
        }

        // resolve effective motor constant K from [Kt, Ke]
        let K: f64 = if Kt > 0.0 && Ke > 0.0 {
            f64::sqrt(Kt * Ke)
        } else if Kt > 0.0 {
            Kt
        } else {
            Ke
        };

        // derive R from nominal: tau0 = K*vn/R
        if R == 0.0 && vn > 0.0 && tau0 > 0.0 && K > 0.0 {
            R = K * vn / tau0;
        }

        if K <= 0.0 {
            return b"DC motor: motor constant K must be positive\0".as_ptr() as *const i8;
        }
        if R <= 0.0 {
            return b"DC motor: resistance R must be positive\0".as_ptr() as *const i8;
        }

        // set types
        std::ptr::write(std::ptr::addr_of_mut!((*actuator).dyntype) as *mut i32, mjtDyn_mjDYN_DCMOTOR as i32);
        std::ptr::write(std::ptr::addr_of_mut!((*actuator).gaintype) as *mut i32, mjtGain_mjGAIN_DCMOTOR as i32);
        std::ptr::write(std::ptr::addr_of_mut!((*actuator).biastype) as *mut i32, mjtBias_mjBIAS_DCMOTOR as i32);

        // gainprm: [R, K, alpha, T0]
        (*actuator).gainprm[0] = R;
        (*actuator).gainprm[1] = K;

        // controller parameters: gainprm[4:6] for kp, ki, kd
        (*actuator).gainprm[4] = if !controller.is_null() { *controller.add(0) } else { 0.0 };
        (*actuator).gainprm[5] = if !controller.is_null() { *controller.add(1) } else { 0.0 };
        (*actuator).gainprm[6] = if !controller.is_null() { *controller.add(2) } else { 0.0 };

        // controller parameters: dynprm[7,8] for slewmax, Imax
        (*actuator).dynprm[7] = if !controller.is_null() { *controller.add(3) } else { 0.0 };
        (*actuator).dynprm[8] = if !controller.is_null() { *controller.add(4) } else { 0.0 };

        // controller parameters: gainprm[7] for v_max
        if !controller.is_null() && *controller.add(5) > 0.0 {
            (*actuator).gainprm[7] = *controller.add(5);
        }

        // saturation -> forcerange
        if !saturation.is_null() && (*saturation.add(0) > 0.0 || *saturation.add(1) > 0.0) {
            let mut tau_max = *saturation.add(0);
            if tau_max == 0.0 && *saturation.add(1) > 0.0 {
                tau_max = K * *saturation.add(1); // tau_max = K * i_max
            }
            (*actuator).forcerange[0] = -tau_max;
            (*actuator).forcerange[1] = tau_max;
            (*actuator).forcelimited = 1;
        }

        // saturation: [tau_max, i_max, (di/dt)_max]
        if !saturation.is_null() && *saturation.add(2) > 0.0 {
            (*actuator).dynprm[1] = *saturation.add(2);
        }

        // cogging: [amplitude, periodicity, phase] -> biasprm[0:3]
        (*actuator).biasprm[0] = if !cogging.is_null() { *cogging.add(0) } else { 0.0 };
        (*actuator).biasprm[1] = if !cogging.is_null() { *cogging.add(1) } else { 0.0 };
        (*actuator).biasprm[2] = if !cogging.is_null() { *cogging.add(2) } else { 0.0 };

        // count activation variables: slot order is slew, integral, temperature, bristle, current
        let mut actdim: i32 = 0;

        // inductance: [L, te]
        if !inductance.is_null() && *inductance.add(0) < 0.0 {
            return b"DC motor: inductance must be non-negative\0".as_ptr() as *const i8;
        }
        if !inductance.is_null() && *inductance.add(1) < 0.0 {
            return b"DC motor: electrical time constant must be non-negative\0".as_ptr() as *const i8;
        }
        let te: f64 = if !inductance.is_null() && *inductance.add(0) > 0.0 {
            *inductance.add(0) / R
        } else if !inductance.is_null() {
            *inductance.add(1)
        } else {
            0.0
        };
        (*actuator).dynprm[0] = te;
        if te > 0.0 {
            actdim += 1;
        }

        // controller states: slew rate limiting
        if !controller.is_null() && *controller.add(3) > 0.0 {
            actdim += 1;
        }

        // controller states: integral
        if !controller.is_null() && *controller.add(1) > 0.0 {
            actdim += 1;
        }

        // thermal -> temperature activation
        if !thermal.is_null() && (*thermal.add(0) > 0.0 || *thermal.add(1) > 0.0 || *thermal.add(2) > 0.0) {
            let mut RT = *thermal.add(0);    // thermal resistance
            let mut C = *thermal.add(1);     // thermal capacitance
            let mut tth = *thermal.add(2);   // thermal time constant
            let alpha = *thermal.add(3);     // temperature coefficient
            let T0 = *thermal.add(4);        // reference temperature
            let Ta = *thermal.add(5);        // ambient temperature

            if tth > 0.0 && RT > 0.0 && C == 0.0 {
                C = tth / RT;
            } else if tth > 0.0 && C > 0.0 && RT == 0.0 {
                RT = tth / C;
            } else if tth == 0.0 && RT > 0.0 && C > 0.0 {
                tth = RT * C;
            }

            if RT <= 0.0 {
                return b"DC motor: thermal resistance must be positive\0".as_ptr() as *const i8;
            }
            if C <= 0.0 {
                return b"DC motor: thermal capacitance must be positive\0".as_ptr() as *const i8;
            }

            (*actuator).dynprm[2] = RT;
            (*actuator).dynprm[3] = C;
            (*actuator).dynprm[4] = Ta;
            (*actuator).gainprm[2] = alpha;
            (*actuator).gainprm[3] = T0;
            actdim += 1;
        }

        // lugre: {stiffness, damping, coulomb, static, stribeck}
        if !lugre.is_null() && *lugre.add(0) > 0.0 {
            (*actuator).dynprm[5] = *lugre.add(0);    // stiffness -> sigma0
            (*actuator).dynprm[6] = *lugre.add(1);    // damping   -> sigma1
            (*actuator).biasprm[3] = *lugre.add(2);   // coulomb   -> tau_c
            (*actuator).biasprm[4] = *lugre.add(3);   // static    -> tau_s
            (*actuator).biasprm[5] = *lugre.add(4);   // stribeck  -> omega_s
            actdim += 1;
        }

        // set input mode and activation dimension
        (*actuator).gainprm[8] = input_mode as f64;
        (*actuator).actdim = actdim;

        // enforce actlimited = 0; homogeneous bounds are invalid across DC motor states
        (*actuator).actlimited = 0;

        // DC motor always uses actearly
        (*actuator).actearly = 1;

        b"\0".as_ptr() as *const i8
    }
}

/// C: mjs_getSpec (user/user_api.h:233)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_getSpec(element: *const mjsElement) -> *mut mjSpec {
    // SAFETY: element is a valid pointer to mjsElement within a mjCBase-derived object.
    // Accesses mjCBase.model->spec, matching C++ static_cast<const mjCBase*>(element)->model->spec.
    unsafe {
        let base = element as *const mjCBase;
        let model = (*base).model;
        &mut (*model).spec as *mut mjSpec
    }
}

/// C: mjs_getFrame (user/user_api.h:255)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_getFrame(element: *const mjsElement) -> *mut mjsFrame {
    // SAFETY: element is a valid mjsElement pointer embedded in a mjCBase-derived object.
    // The cast mirrors C++ static_cast<const mjCBase*>(element).
    unsafe {
        let elemtype = *(element as *const i32);
        match elemtype {
            1 | 100 | 3 | 5 | 6 | 7 | 8 => {
                // mjOBJ_BODY=1, mjOBJ_FRAME=100, mjOBJ_JOINT=3, mjOBJ_GEOM=5,
                // mjOBJ_SITE=6, mjOBJ_CAMERA=7, mjOBJ_LIGHT=8
                let base = element as *const mjCBase;
                let frame = (*base).frame;
                if frame.is_null() {
                    std::ptr::null_mut()
                } else {
                    &mut (*frame).spec as *mut mjsFrame
                }
            }
            _ => std::ptr::null_mut(),
        }
    }
}

/// C: mjs_getId (user/user_api.h:270)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_getId(element: *const mjsElement) -> i32 {
    // SAFETY: element is a valid mjsElement pointer embedded in a mjCBase-derived object.
    unsafe {
        if element.is_null() {
            return -1;
        }
        (*(element as *const mjCBase)).id
    }
}

/// C: mjs_asBody (user/user_api.h:301)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_asBody(element: *mut mjsElement) -> *mut mjsBody {
    // SAFETY: element is a valid mjsElement pointer (caller contract).
    // static_cast<mjCBody*>(element)->spec mirrors the C++ cast.
    unsafe {
        if element.is_null() {
            return std::ptr::null_mut();
        }
        // elemtype is first 4 bytes of the [u8; 8] field (i32 in C)
        let elemtype = *(element as *const i32);
        if elemtype == 1 {
            &mut (*(element as *mut crate::types::mjCBody)).spec as *mut crate::types::mjsBody
        } else {
            std::ptr::null_mut()
        }
    }
}

/// C: mjs_asGeom (user/user_api.h:304)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_asGeom(element: *mut mjsElement) -> *mut mjsGeom {
    // SAFETY: element is a valid mjsElement pointer (caller contract).
    // static_cast<mjCGeom*>(element)->spec mirrors the C++ cast.
    unsafe {
        if element.is_null() {
            return std::ptr::null_mut();
        }
        // elemtype is first 4 bytes of the [u8; 8] field (i32 in C)
        let elemtype = *(element as *const i32);
        if elemtype == 5 {
            &mut (*(element as *mut crate::types::mjCGeom)).spec as *mut crate::types::mjsGeom
        } else {
            std::ptr::null_mut()
        }
    }
}

/// C: mjs_asJoint (user/user_api.h:307)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_asJoint(element: *mut mjsElement) -> *mut mjsJoint {
    // SAFETY: element is a valid mjsElement pointer (caller contract).
    // static_cast<mjCJoint*>(element)->spec mirrors the C++ cast.
    unsafe {
        if element.is_null() {
            return std::ptr::null_mut();
        }
        // elemtype is first 4 bytes of the [u8; 8] field (i32 in C)
        let elemtype = *(element as *const i32);
        if elemtype == 3 {
            &mut (*(element as *mut crate::types::mjCJoint)).spec as *mut crate::types::mjsJoint
        } else {
            std::ptr::null_mut()
        }
    }
}

/// C: mjs_asSite (user/user_api.h:310)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_asSite(element: *mut mjsElement) -> *mut mjsSite {
    // SAFETY: element is a valid mjsElement pointer (caller contract).
    // static_cast<mjCSite*>(element)->spec mirrors the C++ cast.
    unsafe {
        if element.is_null() {
            return std::ptr::null_mut();
        }
        // elemtype is first 4 bytes of the [u8; 8] field (i32 in C)
        let elemtype = *(element as *const i32);
        if elemtype == 6 {
            &mut (*(element as *mut crate::types::mjCSite)).spec as *mut crate::types::mjsSite
        } else {
            std::ptr::null_mut()
        }
    }
}

/// C: mjs_asCamera (user/user_api.h:313)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_asCamera(element: *mut mjsElement) -> *mut mjsCamera {
    // SAFETY: element is a valid mjsElement pointer (caller contract).
    // static_cast<mjCCamera*>(element)->spec mirrors the C++ cast.
    unsafe {
        if element.is_null() {
            return std::ptr::null_mut();
        }
        // elemtype is first 4 bytes of the [u8; 8] field (i32 in C)
        let elemtype = *(element as *const i32);
        if elemtype == 7 {
            &mut (*(element as *mut crate::types::mjCCamera)).spec as *mut crate::types::mjsCamera
        } else {
            std::ptr::null_mut()
        }
    }
}

/// C: mjs_asLight (user/user_api.h:316)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_asLight(element: *mut mjsElement) -> *mut mjsLight {
    // SAFETY: element is a valid mjsElement pointer (caller contract).
    // static_cast<mjCLight*>(element)->spec mirrors the C++ cast.
    unsafe {
        if element.is_null() {
            return std::ptr::null_mut();
        }
        // elemtype is first 4 bytes of the [u8; 8] field (i32 in C)
        let elemtype = *(element as *const i32);
        if elemtype == 8 {
            &mut (*(element as *mut crate::types::mjCLight)).spec as *mut crate::types::mjsLight
        } else {
            std::ptr::null_mut()
        }
    }
}

/// C: mjs_asFrame (user/user_api.h:319)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_asFrame(element: *mut mjsElement) -> *mut mjsFrame {
    // SAFETY: element is a valid mjsElement pointer (caller contract).
    // static_cast<mjCFrame*>(element)->spec mirrors the C++ cast.
    unsafe {
        if element.is_null() {
            return std::ptr::null_mut();
        }
        // elemtype is first 4 bytes of the [u8; 8] field (i32 in C)
        let elemtype = *(element as *const i32);
        if elemtype == 100 {
            &mut (*(element as *mut crate::types::mjCFrame)).spec as *mut crate::types::mjsFrame
        } else {
            std::ptr::null_mut()
        }
    }
}

/// C: mjs_asActuator (user/user_api.h:322)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_asActuator(element: *mut mjsElement) -> *mut mjsActuator {
    // SAFETY: element is a valid mjsElement pointer (caller contract).
    // static_cast<mjCActuator*>(element)->spec mirrors the C++ cast.
    unsafe {
        if element.is_null() {
            return std::ptr::null_mut();
        }
        // elemtype is first 4 bytes of the [u8; 8] field (i32 in C)
        let elemtype = *(element as *const i32);
        if elemtype == 19 {
            &mut (*(element as *mut crate::types::mjCActuator)).spec as *mut crate::types::mjsActuator
        } else {
            std::ptr::null_mut()
        }
    }
}

/// C: mjs_asSensor (user/user_api.h:325)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_asSensor(element: *mut mjsElement) -> *mut mjsSensor {
    // SAFETY: element is a valid mjsElement pointer (caller contract).
    // static_cast<mjCSensor*>(element)->spec mirrors the C++ cast.
    unsafe {
        if element.is_null() {
            return std::ptr::null_mut();
        }
        // elemtype is first 4 bytes of the [u8; 8] field (i32 in C)
        let elemtype = *(element as *const i32);
        if elemtype == 20 {
            &mut (*(element as *mut crate::types::mjCSensor)).spec as *mut crate::types::mjsSensor
        } else {
            std::ptr::null_mut()
        }
    }
}

/// C: mjs_asFlex (user/user_api.h:328)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_asFlex(element: *mut mjsElement) -> *mut mjsFlex {
    // SAFETY: element is a valid mjsElement pointer (caller contract).
    // static_cast<mjCFlex*>(element)->spec mirrors the C++ cast.
    unsafe {
        if element.is_null() {
            return std::ptr::null_mut();
        }
        // elemtype is first 4 bytes of the [u8; 8] field (i32 in C)
        let elemtype = *(element as *const i32);
        if elemtype == 9 { // mjOBJ_FLEX
            &mut (*(element as *mut crate::types::mjCFlex)).spec as *mut crate::types::mjsFlex
        } else {
            std::ptr::null_mut()
        }
    }
}

/// C: mjs_asPair (user/user_api.h:331)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_asPair(element: *mut mjsElement) -> *mut mjsPair {
    // SAFETY: element is a valid mjsElement pointer (caller contract).
    // static_cast<mjCPair*>(element)->spec mirrors the C++ cast.
    unsafe {
        if element.is_null() {
            return std::ptr::null_mut();
        }
        // elemtype is first 4 bytes of the [u8; 8] field (i32 in C)
        let elemtype = *(element as *const i32);
        if elemtype == 15 {
            &mut (*(element as *mut crate::types::mjCPair)).spec as *mut crate::types::mjsPair
        } else {
            std::ptr::null_mut()
        }
    }
}

/// C: mjs_asEquality (user/user_api.h:334)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_asEquality(element: *mut mjsElement) -> *mut mjsEquality {
    // SAFETY: element is a valid mjsElement pointer (caller contract).
    // static_cast<mjCEquality*>(element)->spec mirrors the C++ cast.
    unsafe {
        if element.is_null() {
            return std::ptr::null_mut();
        }
        // elemtype is first 4 bytes of the [u8; 8] field (i32 in C)
        let elemtype = *(element as *const i32);
        if elemtype == 17 {
            &mut (*(element as *mut crate::types::mjCEquality)).spec as *mut crate::types::mjsEquality
        } else {
            std::ptr::null_mut()
        }
    }
}

/// C: mjs_asExclude (user/user_api.h:337)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_asExclude(element: *mut mjsElement) -> *mut mjsExclude {
    // SAFETY: element is a valid mjsElement pointer (caller contract).
    // static_cast<mjCExclude*>(element)->spec mirrors the C++ cast.
    unsafe {
        if element.is_null() {
            return std::ptr::null_mut();
        }
        // elemtype is first 4 bytes of the [u8; 8] field (i32 in C)
        let elemtype = *(element as *const i32);
        if elemtype == 16 {
            &mut (*(element as *mut crate::types::mjCBodyPair)).spec as *mut crate::types::mjsExclude
        } else {
            std::ptr::null_mut()
        }
    }
}

/// C: mjs_asTendon (user/user_api.h:340)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_asTendon(element: *mut mjsElement) -> *mut mjsTendon {
    // SAFETY: element is a valid mjsElement pointer (caller contract).
    // static_cast<mjCTendon*>(element)->spec mirrors the C++ cast.
    unsafe {
        if element.is_null() {
            return std::ptr::null_mut();
        }
        // elemtype is first 4 bytes of the [u8; 8] field (i32 in C)
        let elemtype = *(element as *const i32);
        if elemtype == 18 {
            &mut (*(element as *mut crate::types::mjCTendon)).spec as *mut crate::types::mjsTendon
        } else {
            std::ptr::null_mut()
        }
    }
}

/// C: mjs_asNumeric (user/user_api.h:343)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_asNumeric(element: *mut mjsElement) -> *mut mjsNumeric {
    // SAFETY: element is a valid mjsElement pointer (caller contract).
    // static_cast<mjCNumeric*>(element)->spec mirrors the C++ cast.
    unsafe {
        if element.is_null() {
            return std::ptr::null_mut();
        }
        // elemtype is first 4 bytes of the [u8; 8] field (i32 in C)
        let elemtype = *(element as *const i32);
        if elemtype == 21 {
            &mut (*(element as *mut crate::types::mjCNumeric)).spec as *mut crate::types::mjsNumeric
        } else {
            std::ptr::null_mut()
        }
    }
}

/// C: mjs_asText (user/user_api.h:346)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_asText(element: *mut mjsElement) -> *mut mjsText {
    // SAFETY: element is a valid mjsElement pointer (caller contract).
    // static_cast<mjCText*>(element)->spec mirrors the C++ cast.
    unsafe {
        if element.is_null() {
            return std::ptr::null_mut();
        }
        // elemtype is first 4 bytes of the [u8; 8] field (i32 in C)
        let elemtype = *(element as *const i32);
        if elemtype == 22 {
            &mut (*(element as *mut crate::types::mjCText)).spec as *mut crate::types::mjsText
        } else {
            std::ptr::null_mut()
        }
    }
}

/// C: mjs_asTuple (user/user_api.h:349)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_asTuple(element: *mut mjsElement) -> *mut mjsTuple {
    // SAFETY: element is a valid mjsElement pointer (caller contract).
    // static_cast<mjCTuple*>(element)->spec mirrors the C++ cast.
    unsafe {
        if element.is_null() {
            return std::ptr::null_mut();
        }
        // elemtype is first 4 bytes of the [u8; 8] field (i32 in C)
        let elemtype = *(element as *const i32);
        if elemtype == 23 {
            &mut (*(element as *mut crate::types::mjCTuple)).spec as *mut crate::types::mjsTuple
        } else {
            std::ptr::null_mut()
        }
    }
}

/// C: mjs_asKey (user/user_api.h:352)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_asKey(element: *mut mjsElement) -> *mut mjsKey {
    // SAFETY: element is a valid mjsElement pointer (caller contract).
    // static_cast<mjCKey*>(element)->spec mirrors the C++ cast.
    unsafe {
        if element.is_null() {
            return std::ptr::null_mut();
        }
        // elemtype is first 4 bytes of the [u8; 8] field (i32 in C)
        let elemtype = *(element as *const i32);
        if elemtype == 24 {
            &mut (*(element as *mut crate::types::mjCKey)).spec as *mut crate::types::mjsKey
        } else {
            std::ptr::null_mut()
        }
    }
}

/// C: mjs_asMesh (user/user_api.h:355)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_asMesh(element: *mut mjsElement) -> *mut mjsMesh {
    // SAFETY: element is a valid mjsElement pointer (caller contract).
    // static_cast<mjCMesh*>(element)->spec mirrors the C++ cast.
    unsafe {
        if element.is_null() {
            return std::ptr::null_mut();
        }
        // elemtype is first 4 bytes of the [u8; 8] field (i32 in C)
        let elemtype = *(element as *const i32);
        if elemtype == 10 {
            &mut (*(element as *mut crate::types::mjCMesh)).spec as *mut crate::types::mjsMesh
        } else {
            std::ptr::null_mut()
        }
    }
}

/// C: mjs_asHField (user/user_api.h:358)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_asHField(element: *mut mjsElement) -> *mut mjsHField {
    // SAFETY: element is a valid mjsElement pointer (caller contract).
    // static_cast<mjCHField*>(element)->spec mirrors the C++ cast.
    unsafe {
        if element.is_null() {
            return std::ptr::null_mut();
        }
        // elemtype is first 4 bytes of the [u8; 8] field (i32 in C)
        let elemtype = *(element as *const i32);
        if elemtype == 12 {
            &mut (*(element as *mut crate::types::mjCHField)).spec as *mut crate::types::mjsHField
        } else {
            std::ptr::null_mut()
        }
    }
}

/// C: mjs_asSkin (user/user_api.h:361)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_asSkin(element: *mut mjsElement) -> *mut mjsSkin {
    // SAFETY: element is a valid mjsElement pointer (caller contract).
    // static_cast<mjCSkin*>(element)->spec mirrors the C++ cast.
    unsafe {
        if element.is_null() {
            return std::ptr::null_mut();
        }
        // elemtype is first 4 bytes of the [u8; 8] field (i32 in C)
        let elemtype = *(element as *const i32);
        if elemtype == 11 {
            &mut (*(element as *mut crate::types::mjCSkin)).spec as *mut crate::types::mjsSkin
        } else {
            std::ptr::null_mut()
        }
    }
}

/// C: mjs_asTexture (user/user_api.h:364)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_asTexture(element: *mut mjsElement) -> *mut mjsTexture {
    // SAFETY: element is a valid mjsElement pointer (caller contract).
    // static_cast<mjCTexture*>(element)->spec mirrors the C++ cast.
    unsafe {
        if element.is_null() {
            return std::ptr::null_mut();
        }
        // elemtype is first 4 bytes of the [u8; 8] field (i32 in C)
        let elemtype = *(element as *const i32);
        if elemtype == 13 {
            &mut (*(element as *mut crate::types::mjCTexture)).spec as *mut crate::types::mjsTexture
        } else {
            std::ptr::null_mut()
        }
    }
}

/// C: mjs_asMaterial (user/user_api.h:367)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_asMaterial(element: *mut mjsElement) -> *mut mjsMaterial {
    // SAFETY: element is a valid mjsElement pointer (caller contract).
    // static_cast<mjCMaterial*>(element)->spec mirrors the C++ cast.
    unsafe {
        if element.is_null() {
            return std::ptr::null_mut();
        }
        // elemtype is first 4 bytes of the [u8; 8] field (i32 in C)
        let elemtype = *(element as *const i32);
        if elemtype == 14 {
            &mut (*(element as *mut crate::types::mjCMaterial)).spec as *mut crate::types::mjsMaterial
        } else {
            std::ptr::null_mut()
        }
    }
}

/// C: mjs_asPlugin (user/user_api.h:370)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_asPlugin(element: *mut mjsElement) -> *mut mjsPlugin {
    // SAFETY: element is a valid mjsElement pointer (caller contract).
    // static_cast<mjCPlugin*>(element)->spec mirrors the C++ cast.
    unsafe {
        if element.is_null() {
            return std::ptr::null_mut();
        }
        // elemtype is first 4 bytes of the [u8; 8] field (i32 in C)
        let elemtype = *(element as *const i32);
        if elemtype == 25 {
            &mut (*(element as *mut crate::types::mjCPlugin)).spec as *mut crate::types::mjsPlugin
        } else {
            std::ptr::null_mut()
        }
    }
}

/// C: mjs_getName (user/user_api.h:415)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_getName(element: *mut mjsElement) -> *mut mjString {
    // SAFETY: element is a valid mjsElement pointer. Cast mirrors C++ static_cast.
    unsafe {
        let elemtype = *(element as *const i32);
        if elemtype == 101 {
            // mjOBJ_DEFAULT: cast to mjCDef, return &name
            &mut (*(element as *mut mjCDef)).name as *mut std__string as *mut mjString
        } else {
            // all others: cast to mjCBase, return &name
            &mut (*(element as *mut mjCBase)).name as *mut std__string as *mut mjString
        }
    }
}

/// C: mjs_getPluginAttributes (user/user_api.h:429)
#[allow(unused_variables, non_snake_case)]
pub fn mjs_getPluginAttributes(plugin: *const mjsPlugin) -> *const () {
    // SAFETY: plugin is valid; plugin->element points to a mjCPlugin object.
    // Returns &pluginC->config_attribs (pointer to the config_attribs field).
    unsafe {
        let pluginC = (*plugin).element as *const mjCPlugin;
        &(*pluginC).config_attribs as *const _ as *const ()
    }
}

