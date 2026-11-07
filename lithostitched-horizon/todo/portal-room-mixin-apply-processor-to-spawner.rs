// Not ported yet: the Java @WrapOperation(INVOKE) on PortalRoom.postProcess (PortalRoomMixin.applyProcessorToSpawner).
// Why it's off: No Horizon function mapped for PortalRoom.postProcess (run `match`, or map it in a --mappings file).
// horizon-mixins kind: WRAP_OPERATION
// Note: No Horizon function known for PortalRoom.postProcess: run `match`.
// Note: The call it hooks (WorldGenLevel.setBlock) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   WorldGenLevel.setBlock -> (not mapped yet)
//   StructurePiece.<init> -> (not mapped yet)
//   PortalRoom.postProcess -> (not mapped yet)
//
// Decompiled Java, dev/worldgen/lithostitched/mixin/common/processor/PortalRoomMixin.java:21:
// | @WrapOperation(
// |    method = "postProcess",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/world/level/WorldGenLevel;setBlock(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;I)Z"
// |    )
// | )
// | private boolean applyProcessorToSpawner(WorldGenLevel level, BlockPos pos, BlockState state, int flags, Operation<Boolean> operation) {
// |    return NonTemplateProcessorHelper.getProcessorAndApply(this, this.boundingBox, level, pos, state, flags, operation);
// | }
