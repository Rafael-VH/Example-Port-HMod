// Not ported yet: the Java @WrapOperation(INVOKE) on RuinedPortalPiece.maybeAddVines (RuinedPortalPieceMixin.applyProcessorToExtraBits).
// Why it's off: No Horizon function mapped for RuinedPortalPiece.maybeAddVines (run `match`, or map it in a --mappings file).
// horizon-mixins kind: WRAP_OPERATION
// Note: No Horizon function known for RuinedPortalPiece.maybeAddVines: run `match`.
// Note: The call it hooks (LevelAccessor.setBlockAndUpdate) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   LevelAccessor.setBlockAndUpdate -> (not mapped yet)
//   BlockState.is -> (not mapped yet)
//   StructurePiece.<init> -> (not mapped yet)
//   RuinedPortalPiece.makeSettings -> (not mapped yet)
//   RuinedPortalPiece.maybeAddLeavesAbove -> (not mapped yet)
//   RuinedPortalPiece.maybeAddVines -> (not mapped yet)
//   RuinedPortalPiece.placeNetherrackOrMagma -> (not mapped yet)
//   StructurePlaceSettings.addProcessor -> (not mapped yet)
//
// Decompiled Java, dev/worldgen/lithostitched/mixin/common/processor/RuinedPortalPieceMixin.java:36:
// | @WrapOperation(
// |    method = {"maybeAddVines", "maybeAddLeavesAbove", "placeNetherrackOrMagma"},
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/world/level/LevelAccessor;setBlockAndUpdate(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)Z"
// |    )
// | )
// | private boolean applyProcessorToExtraBits(LevelAccessor level, BlockPos pos, BlockState state, Operation<Boolean> original) {
// |    Optional<StructureProcessorList> processors = NonTemplateProcessorHelper.getProcessor(this, level);
// |    if (processors.isEmpty()) {
// |       return (Boolean)original.call(new Object[]{level, pos, state});
// |    }
// |
// |    BlockState processedState = NonTemplateProcessorHelper.processBlock(pos, level, pos, state, processors.get());
// |    return processedState != null ? (Boolean)original.call(new Object[]{level, pos, processedState}) : false;
// | }
