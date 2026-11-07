// Not ported yet: the Java @Inject(RETURN) on SinglePoolElement.getShuffledJigsawBlocks (SinglePoolElementMixin.graveyard$applyPoolAliases).
// Why it's off: No Horizon function mapped for SinglePoolElement.getShuffledJigsawBlocks (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon's RETURN runs at every normal exit and passes the return value on.
// Note: No Horizon function known for SinglePoolElement.getShuffledJigsawBlocks: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Holder.isBound -> (not mapped yet)
//   Holder.value -> (not mapped yet)
//   SinglePoolElement.getSettings -> (not mapped yet)
//   SinglePoolElement.getShuffledJigsawBlocks -> (not mapped yet)
//   LiquidSettings.IGNORE_WATERLOGGING -> (not mapped yet)
//   StructurePlaceSettings.setLiquidSettings -> (not mapped yet)
//   StructureProcessorList.list -> (not mapped yet)
//
// Decompiled Java, com/lion/graveyard/mixin/SinglePoolElementMixin.java:32:
// | @Inject(method = "getShuffledJigsawBlocks", at = @At("RETURN"), cancellable = true)
// | private void graveyard$applyPoolAliases(
// |    StructureTemplateManager structureTemplateManager,
// |    BlockPos pos,
// |    Rotation rotation,
// |    RandomSource random,
// |    CallbackInfoReturnable<List<JigsawBlockInfo>> callback
// | ) {
// |    if (this.processors.isBound()) {
// |       for (StructureProcessor processor : ((StructureProcessorList)this.processors.value()).list()) {
// |          if (processor instanceof PoolAliasProcessor poolAliases) {
// |             callback.setReturnValue(poolAliases.apply((List<JigsawBlockInfo>)callback.getReturnValue()));
// |             return;
// |          }
// |       }
// |    }
// | }
