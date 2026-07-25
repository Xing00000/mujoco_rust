/*
 * user_vfs_bridge.cc — source-overlay bridge for mj_addFileVFS / mj_addBufferVFS.
 *
 * Strategy (plan R6 / B3, method 1):
 *   Include the original user_vfs.cc source so that BufferProvider and its
 *   Mount template are compiled into THIS translation unit.  Then declare
 *   extern "C" bridge symbols that call the local (same-TU) definitions,
 *   not the ones in libmujoco.a.
 *
 * Why not just link libmujoco? Because that would be a delegation to the
 * upstream binary, not a proper translation.  This TU owns the implementation.
 *
 * Build requirements (handled by build.rs):
 *   - Same -I, -D, and -std flags as the original user_vfs.cc compilation.
 *   - Do NOT also link libmujoco.a (would cause duplicate symbol errors for
 *     mj_addFileVFS etc.).  Instead, link only the subset of mujoco needed:
 *     libmujoco_incomplete.a or the individual object files.
 *   - The mujoco_EXPORTS define must be set so MJAPI expands correctly.
 */

/* Pull in the full user_vfs.cc implementation including anonymous-namespace
 * BufferProvider, FilePath, VFS::Upcast, hash, etc. */
#include "user/user_vfs.cc"  // NOLINT(build/include) — intentional source overlay

/* Stable C ABI bridge symbols for Rust */
extern "C" {

int c2rust_mj_addFileVFS_bridge(mjVFS* vfs, const char* directory, const char* filename) {
    /* Calls the local mj_addFileVFS defined in the included user_vfs.cc,
     * not the one in libmujoco.a.  BufferProvider::Mount runs here. */
    return mj_addFileVFS(vfs, directory, filename);
}

int c2rust_mj_addBufferVFS_bridge(mjVFS* vfs, const char* name,
                                   const void* buffer, int nbuffer) {
    /* Calls the local mj_addBufferVFS.  Buffer is copied by BufferProvider. */
    return mj_addBufferVFS(vfs, name, buffer, nbuffer);
}

} /* extern "C" */
