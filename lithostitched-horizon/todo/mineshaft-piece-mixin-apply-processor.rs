// Not ported yet: the Java @WrapOperation(INVOKE) on MineShaftPiece.setPlanksBlock (MineshaftPieceMixin.applyProcessor).
// Why it's off: No Horizon function mapped for MineShaftPiece.setPlanksBlock (run `match`, or map it in a --mappings file).
// horizon-mixins kind: WRAP_OPERATION
// Note: No Horizon function known for MineShaftPiece.setPlanksBlock: run `match`.
// Note: The call it hooks (WorldGenLevel.setBlock) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   WorldGenLevel.setBlock -> (not mapped yet)
//   StructurePiece.<init> -> (not mapped yet)
//   MineShaftPiece.setPlanksBlock -> (not mapped yet)
//
// Decompiled Java, dev/worldgen/lithostitched/mixin/common/processor/MineshaftPieceMixin.java:22:
// | @WrapOperation(
// |    method = "setPlanksBlock",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/world/level/WorldGenLevel;setBlock(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;I)Z"
// |    )
// | )
// | private boolean applyProcessor(
// |    WorldGenLevel level,
// |    BlockPos pos,
// |    BlockState state,
// |    int flags,
// |    Operation<Boolean> operation,
// |    @Local(ordinal = 0) int x,
// |    @Local(ordinal = 1) int y,
// |    @Local(ordinal = 2) int z
// | ) {
// |    return NonTemplateProcessorHelper.getProcessorAndApply(this, this.boundingBox, level, pos, state, flags, operation, x, y, z);
// | }
