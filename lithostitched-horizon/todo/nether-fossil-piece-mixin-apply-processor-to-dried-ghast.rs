// Not ported yet: the Java @WrapOperation(INVOKE) on NetherFossilPiece.placeDriedGhast (NetherFossilPieceMixin.applyProcessorToDriedGhast).
// Why it's off: No Horizon function mapped for NetherFossilPiece.placeDriedGhast (run `match`, or map it in a --mappings file).
// horizon-mixins kind: WRAP_OPERATION
// Note: No Horizon function known for NetherFossilPiece.placeDriedGhast: run `match`.
// Note: The call it hooks (WorldGenLevel.setBlock) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   BlockPos.<init> -> (not mapped yet)
//   WorldGenLevel.setBlock -> (not mapped yet)
//   BoundingBox.minX -> (not mapped yet)
//   BoundingBox.minY -> (not mapped yet)
//   BoundingBox.minZ -> (not mapped yet)
//   NetherFossilPiece.makeSettings -> (not mapped yet)
//   NetherFossilPiece.placeDriedGhast -> (not mapped yet)
//   StructurePlaceSettings.addProcessor -> (not mapped yet)
//
// Decompiled Java, dev/worldgen/lithostitched/mixin/common/processor/NetherFossilPieceMixin.java:29:
// | @WrapOperation(
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/world/level/WorldGenLevel;setBlock(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;I)Z"
// |    ),
// |    method = "placeDriedGhast"
// | )
// | private static boolean applyProcessorToDriedGhast(
// |    WorldGenLevel level, BlockPos pos, BlockState state, int flags, Operation<Boolean> operation, @Local(ordinal = 0, argsOnly = true) BoundingBox fossilBB
// | ) {
// |    Optional<StructureProcessorList> processorList = NonTemplateProcessorHelper.getProcessor(level, LithostitchedProcessorLists.NETHER_FOSSIL);
// |    if (processorList.isEmpty()) {
// |       return (Boolean)operation.call(new Object[]{level, pos, state, flags});
// |    }
// |
// |    BlockState processedState = NonTemplateProcessorHelper.processBlock(
// |       new BlockPos(fossilBB.minX(), fossilBB.minY(), fossilBB.minZ()), level, pos, state, processorList.get()
// |    );
// |    return processedState != null ? (Boolean)operation.call(new Object[]{level, pos, processedState, flags}) : false;
// | }
