/*
 * libccd_bridge.h — narrow C bridge between Rust and libccd.
 *
 * Purpose: wrap ccd_t, CCD_INIT, ccdMPRPenetration, and ccdVec3Eq so
 * Rust never touches the libccd struct layout directly. The bridge owns
 * all stack allocations; Rust passes callbacks and scalar config.
 *
 * All symbols prefixed c2rust_ to avoid collisions with the real library.
 */
#ifndef C2RUST_LIBCCD_BRIDGE_H
#define C2RUST_LIBCCD_BRIDGE_H

#ifdef __cplusplus
extern "C" {
#endif

/* ccd_real_t is double (MuJoCo builds libccd with CCD_DOUBLE=ON) */
typedef double c2rust_ccd_real;

/* Result written by c2rust_ccd_mpr_penetration */
typedef struct {
    int          status;        /* 0 = penetration found, non-zero = no penetration */
    int          dir_is_origin; /* 1 = dir equals ccd_vec3_origin (degenerate case) */
    c2rust_ccd_real depth;
    c2rust_ccd_real dir[3];
    c2rust_ccd_real pos[3];
} c2rust_ccd_result;

/*
 * Callback types matching libccd's ccd_support_fn and ccd_center_fn.
 * These must match the libccd signatures exactly.
 */
typedef void (*c2rust_support_fn)(const void *obj, const c2rust_ccd_real dir[3],
                                  c2rust_ccd_real vec[3]);
typedef void (*c2rust_center_fn)(const void *obj, c2rust_ccd_real center[3]);
typedef void (*c2rust_first_dir_fn)(const void *o1, const void *o2,
                                    c2rust_ccd_real dir[3]);

/*
 * c2rust_ccd_mpr_penetration — call ccdMPRPenetration via bridge.
 *
 * Parameters:
 *   obj1, obj2       — passed through to support/center callbacks
 *   mpr_tolerance    — m->opt.ccd_tolerance
 *   epa_tolerance    — m->opt.ccd_tolerance  (same as mpr in MuJoCo)
 *   max_iterations   — m->opt.ccd_iterations
 *   support1/2       — support callbacks (same fn used for both objs)
 *   center1/2        — center callbacks  (same fn used for both objs)
 *   use_prism_dir    — 1 → first_dir = prism_firstdir (hfield), 0 → default
 *   result           — output, filled on success
 */
void c2rust_ccd_mpr_penetration(
    const void        *obj1,
    const void        *obj2,
    c2rust_ccd_real    mpr_tolerance,
    c2rust_ccd_real    epa_tolerance,
    unsigned long      max_iterations,
    c2rust_support_fn  support1,
    c2rust_support_fn  support2,
    c2rust_center_fn   center1,
    c2rust_center_fn   center2,
    int                use_prism_dir,
    c2rust_ccd_result *result
);

#ifdef __cplusplus
}
#endif

#endif /* C2RUST_LIBCCD_BRIDGE_H */
