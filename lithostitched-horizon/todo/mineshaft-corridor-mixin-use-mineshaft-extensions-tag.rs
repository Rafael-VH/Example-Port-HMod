// Not ported yet: the Java @WrapOperation(INVOKE) on MineShaftCorridor.placeDoubleLowerOrUpperSupport (MineshaftCorridorMixin.useMineshaftExtensionsTag).
// Why it's off: No Horizon function mapped for MineShaftCorridor.placeDoubleLowerOrUpperSupport (run `match`, or map it in a --mappings file).
// horizon-mixins kind: WRAP_OPERATION
// Note: No Horizon function known for MineShaftCorridor.placeDoubleLowerOrUpperSupport: run `match`.
// Note: The call it hooks (BlockState.is) isn't mapped to a Rust function yet.
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
// Decompiled Java, dev/worldgen/lithostitched/mixin/common/processor/MineshaftCorridorMixin.java:81:
// | @WrapOperation(
// |    method = "placeDoubleLowerOrUpperSupport",
// |    at = @At(value = "INVOKE", target = "Lnet/minecraft/world/level/block/state/BlockState;is(Ljava/lang/Object;)Z")
// | )
// | private boolean useMineshaftExtensionsTag(BlockState state, Object block, Operation<Boolean> operation) {
// |    return (Boolean)operation.call(new Object[]{state, block}) || state.is(LithostitchedBlockTags.SUPPORTS_MINESHAFT_EXTENSIONS);
// | }
