// Not ported yet: the Java @ModifyArgs(INVOKE) on Library.init (LibraryMixin.modifyContext).
// Why it's off: No Horizon function mapped for Library.init (run `match`, or map it in a --mappings file).
// horizon-mixins kind: MODIFY_ARGS
// Note: No Horizon function known for Library.init: run `match`.
// Note: The call it hooks (ALC10.alcCreateContext) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Library.init -> (not mapped yet)
//   ALC10.alcCreateContext -> (not mapped yet)
//
// Decompiled Java, com/sonicether/soundphysics/mixin/LibraryMixin.java:15:
// | @ModifyArgs(method = "init", at = @At(value = "INVOKE", target = "Lorg/lwjgl/openal/ALC10;alcCreateContext(JLjava/nio/IntBuffer;)J"))
// | private void modifyContext(Args args) {
// |    int[] original = this.toArray(((IntBuffer)args.get(1)).duplicate());
// |    MemoryStack stack = MemoryStack.stackPush();
// |
// |    try {
// |       IntBuffer buffer = stack.mallocInt(original.length + 3);
// |       buffer.put(original, 0, original.length - 1);
// |       buffer.put(131075).put(4).put(0);
// |       args.set(1, buffer.put(0).flip());
// |    } catch (Throwable var7) {
// |       if (stack != null) {
// |          try {
// |             stack.close();
// |          } catch (Throwable var6) {
// |             var7.addSuppressed(var6);
// |          }
// |       }
// |
// |       throw var7;
// |    }
// |
// |    if (stack != null) {
// |       stack.close();
// |    }
// | }
