// Not ported yet: the Java @Inject(HEAD) on IOWorker.isOldChunk (IOWorkerMixin.tectonic$needsBlending).
// Why it's off: No Horizon function mapped for IOWorker.isOldChunk (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for IOWorker.isOldChunk: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   CompoundTag.getIntOr -> (not mapped yet)
//   IOWorker.isOldChunk -> (not mapped yet)
//
// Decompiled Java, dev/worldgen/tectonic/mixin/IOWorkerMixin.java:13:
// | @Inject(method = "isOldChunk", at = @At("HEAD"), cancellable = true)
// | private void tectonic$needsBlending(CompoundTag nbt, CallbackInfoReturnable<Boolean> cir) {
// |    int version = nbt.getIntOr(Tectonic.BLENDING_KEY, 0);
// |    if (version != Tectonic.BLENDING_VERSION) {
// |       cir.setReturnValue(true);
// |    }
// | }
