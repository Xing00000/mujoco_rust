/*
 * user_vfs_bridge.cc — C ABI bridge for mj_addFileVFS and mj_addBufferVFS.
 *
 * Strategy (plan R6 / B3): compile this TU with the same flags as
 * user_vfs.cc so it can include the internal headers and call
 * BufferProvider::Mount directly.
 *
 * The anonymous-namespace BufferProvider class and its Mount template live
 * in user_vfs.cc.  We cannot include that TU directly, but we can
 * re-include the same internal headers and use the public C API that the
 * real user_vfs.cc exposes — specifically, just delegate to the real
 * mj_addFileVFS / mj_addBufferVFS via their public header declarations.
 *
 * Since libmujoco.a already contains the compiled user_vfs.cc object,
 * the simplest correct bridge is to just forward to the symbols that are
 * already in libmujoco.a.
 */

#include <mujoco/mujoco.h>  /* declares mj_addFileVFS, mj_addBufferVFS */

extern "C" {

int c2rust_mj_addFileVFS_bridge(mjVFS* vfs, const char* directory, const char* filename) {
    return mj_addFileVFS(vfs, directory, filename);
}

int c2rust_mj_addBufferVFS_bridge(mjVFS* vfs, const char* name,
                                   const void* buffer, int nbuffer) {
    return mj_addBufferVFS(vfs, name, buffer, nbuffer);
}

} /* extern "C" */
