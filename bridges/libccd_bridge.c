/*
 * libccd_bridge.c — wraps ccd_t, CCD_INIT, and ccdMPRPenetration for Rust.
 *
 * Strategy: Rust passes plain double[3] callbacks. We define per-call
 * trampoline globals (thread-safe via stack locals in each bridge call)
 * that adapt between ccd_vec3_t* and double[3].
 *
 * MuJoCo builds libccd with CCD_DOUBLE=ON so ccd_real_t == double.
 */

#include "libccd_bridge.h"

/* Real libccd headers */
#include <ccd/ccd.h>
#include <ccd/vec3.h>

#include <string.h>  /* memcpy */

/* ------------------------------------------------------------------ */
/* Per-call context: stored in thread stack, set before calling libccd */
/* ------------------------------------------------------------------ */

typedef struct {
    c2rust_support_fn   support1;
    c2rust_support_fn   support2;
    c2rust_center_fn    center1;
    c2rust_center_fn    center2;
} bridge_ctx;

/* GCC/Clang thread_local to hold current call context */
static _Thread_local const bridge_ctx *current_ctx = NULL;

/* ------------------------------------------------------------------ */
/* libccd callback trampolines                                          */
/* ------------------------------------------------------------------ */

static void trampoline_support1(const void *obj, const ccd_vec3_t *dir, ccd_vec3_t *vec) {
    current_ctx->support1(obj,
        (const c2rust_ccd_real *)dir->v,
        (c2rust_ccd_real *)vec->v);
}

static void trampoline_support2(const void *obj, const ccd_vec3_t *dir, ccd_vec3_t *vec) {
    current_ctx->support2(obj,
        (const c2rust_ccd_real *)dir->v,
        (c2rust_ccd_real *)vec->v);
}

static void trampoline_center1(const void *obj, ccd_vec3_t *center) {
    current_ctx->center1(obj, (c2rust_ccd_real *)center->v);
}

static void trampoline_center2(const void *obj, ccd_vec3_t *center) {
    current_ctx->center2(obj, (c2rust_ccd_real *)center->v);
}

/* prism_firstdir: always return (0, 0, 1) */
static void trampoline_prism_firstdir(const void *o1, const void *o2, ccd_vec3_t *dir) {
    (void)o1; (void)o2;
    dir->v[0] = 0.0;
    dir->v[1] = 0.0;
    dir->v[2] = 1.0;
}

/* ------------------------------------------------------------------ */
/* Public bridge entry point                                            */
/* ------------------------------------------------------------------ */

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
) {
    /* Store context for trampolines */
    bridge_ctx ctx;
    ctx.support1 = support1;
    ctx.support2 = support2;
    ctx.center1  = center1;
    ctx.center2  = center2;
    current_ctx = &ctx;

    /* Build ccd_t on stack — exactly as MuJoCo's _libccd_wrapper does */
    ccd_t ccd;
    CCD_INIT(&ccd);
    ccd.mpr_tolerance  = mpr_tolerance;
    ccd.epa_tolerance  = epa_tolerance;
    ccd.max_iterations = max_iterations;
    ccd.support1 = trampoline_support1;
    ccd.support2 = trampoline_support2;
    ccd.center1  = trampoline_center1;
    ccd.center2  = trampoline_center2;
    if (use_prism_dir) {
        ccd.first_dir = trampoline_prism_firstdir;
    }
    /* else: CCD_INIT already set first_dir = ccdFirstDirDefault */

    ccd_real_t depth;
    ccd_vec3_t dir, pos;

    int ret = ccdMPRPenetration(obj1, obj2, &ccd, &depth, &dir, &pos);

    result->status = ret;
    result->depth  = (c2rust_ccd_real)depth;

    /* Check for degenerate zero direction — do this in C to use libccd epsilon */
    result->dir_is_origin = ccdVec3Eq(&dir, ccd_vec3_origin);

    memcpy(result->dir, dir.v, sizeof(result->dir));
    memcpy(result->pos, pos.v, sizeof(result->pos));

    current_ctx = NULL;
}
