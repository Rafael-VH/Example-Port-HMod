// Not ported yet: the Java @WrapOperation(INVOKE) on MineShaftCorridor.fillPillarDownOrChainUp (MineshaftCorridorMixin.applyProcessorToChains).
// Why it's off: No Horizon function mapped for MineShaftCorridor.fillPillarDownOrChainUp (run `match`, or map it in a --mappings file).
// horizon-mixins kind: WRAP_OPERATION
// Note: No Horizon function known for MineShaftCorridor.fillPillarDownOrChainUp: run `match`.
// Note: The call it hooks (MineShaftCorridor.fillColumnBetween) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   MutableBlockPos.setY -> (not mapped yet)
//   WorldGenLevel.setBlock -> (not mapped yet)
//   BlockState.is -> (not mapped yet)
//   StructurePiece.<init> -> (not mapped yet)
//   MineShaftCorridor.fillColumnBetween -> (not mapped yet)
//   MineShaftCorridor.fillColumnDown -> (not mapped yet)
//   MineShaftCorridor.fillPillarDownOrChainUp -> (not mapped yet)
//   MineShaftCorridor.placeDoubleLowerOrUpperSupport -> (not mapped yet)
//   MineShaftCorridor.postProcess -> (not mapped yet)
//
// Decompiled Java, dev/worldgen/lithostitched/mixin/common/processor/MineshaftCorridorMixin.java:57:
// | @WrapOperation(
// |    method = "fillPillarDownOrChainUp",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/world/level/levelgen/structure/structures/MineshaftPieces$MineShaftCorridor;fillColumnBetween(Lnet/minecraft/world/level/WorldGenLevel;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/core/BlockPos$MutableBlockPos;II)V"
// |    )
// | )
// | private void applyProcessorToChains(
// |    WorldGenLevel level, BlockState pillarState, MutableBlockPos pos, int bottomInclusive, int topExclusive, Operation<Void> operation
// | ) {
// |    Optional<StructureProcessorList> processorList = NonTemplateProcessorHelper.getProcessor(this, level);
// |    if (processorList.isEmpty()) {
// |       operation.call(new Object[]{level, pillarState, pos, bottomInclusive, topExclusive});
// |    } else {
// |       for (int pillarY = bottomInclusive; pillarY < topExclusive; pillarY++) {
// |          pos.setY(pillarY);
// |          BlockState processedState = NonTemplateProcessorHelper.processBlock(this, this.boundingBox, level, pos, pillarState, processorList.get());
// |          if (processedState != null) {
// |             level.setBlock(pos.setY(pillarY), processedState, 2);
// |          }
// |       }
// |    }
// | }
