// Not ported yet: the Java @Inject(INVOKE) on BlendingData.getHeightAtXZ (BlendingDataMixin.checkTerralithSurfaceBlocks).
// Why it's off: No Horizon function mapped for BlendingData.getHeightAtXZ (run `match`, or map it in a --mappings file).
// horizon-mixins kind: BEFORE_CALL
// Note: Only whole statements, plain let initializers and block tails can be selected.
// Note: No Horizon function known for BlendingData.getHeightAtXZ: run `match`.
// Note: The call it hooks (MutableBlockPos.move) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   MutableBlockPos.getY -> (not mapped yet)
//   MutableBlockPos.move -> (not mapped yet)
//   BlockState.is -> (not mapped yet)
//   ChunkAccess.getBlockState -> (not mapped yet)
//   BlendingData.getHeightAtXZ -> (not mapped yet)
//
// Decompiled Java, dev/worldgen/tectonic/mixin/BlendingDataMixin.java:15:
// | @Inject(
// |    method = "getHeightAtXZ",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/core/BlockPos$MutableBlockPos;move(Lnet/minecraft/core/Direction;)Lnet/minecraft/core/BlockPos$MutableBlockPos;"
// |    ),
// |    locals = LocalCapture.CAPTURE_FAILHARD,
// |    cancellable = true
// | )
// | private void checkTerralithSurfaceBlocks(ChunkAccess access, int x, int z, CallbackInfoReturnable<Integer> cir, int k, int l, MutableBlockPos pos) {
// |    if (access.getBlockState(pos).is(TectonicTags.TERRALITH_SURFACE, b -> true)) {
// |       cir.setReturnValue(pos.getY());
// |    }
// | }
