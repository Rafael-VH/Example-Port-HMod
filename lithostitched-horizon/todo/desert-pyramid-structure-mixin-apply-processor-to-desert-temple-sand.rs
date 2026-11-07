// Not ported yet: the Java @WrapOperation(INVOKE) on DesertPyramidStructure.afterPlace (DesertPyramidStructureMixin.applyProcessorToDesertTempleSand).
// Why it's off: `horizon_world::worldgen::structure::temple::<DesertPyramidPiece as LegacyPiece>::after_place` has a `?` in its body: horizon-mixins doesn't rewrite such bodies with WRAP_OPERATION. A HEAD hook on it, or a WRAP_OPERATION on a function it calls, may work instead.
// horizon-mixins kind: WRAP_OPERATION on `horizon_world::worldgen::structure::temple::<DesertPyramidPiece as LegacyPiece>::after_place`
// Note: The call it hooks (WorldGenLevel.setBlock) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   WorldGenLevel.setBlock -> (not mapped yet)
//   BoundingBox.getCenter -> (not mapped yet)
//   DesertPyramidStructure.afterPlace -> horizon_world::worldgen::structure::temple::<DesertPyramidPiece as LegacyPiece>::after_place (suggested, high)
//   DesertPyramidStructure.placeSuspiciousSand -> (not mapped yet)
//
// Decompiled Java, dev/worldgen/lithostitched/mixin/common/processor/DesertPyramidStructureMixin.java:26:
// | )
// | private static boolean applyProcessorToDesertTempleSand(
// |    WorldGenLevel level, BlockPos pos, BlockState state, int flags, Operation<Boolean> operation, @Local(ordinal = 0, argsOnly = true) BoundingBox chunkBB
// | ) {
// |    Optional<StructureProcessorList> processorList = NonTemplateProcessorHelper.getProcessor(level, LithostitchedProcessorLists.DESERT_TEMPLE);
// |    if (processorList.isEmpty()) {
// |       return (Boolean)operation.call(new Object[]{level, pos, state, flags});
// |    }
// |
// |    BlockState processedState = NonTemplateProcessorHelper.processBlock(chunkBB.getCenter(), level, pos, state, processorList.get());
// |    return processedState != null ? (Boolean)operation.call(new Object[]{level, pos, processedState, flags}) : false;
// | }
