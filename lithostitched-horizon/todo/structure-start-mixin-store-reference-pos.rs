// Not ported yet: the Java @Inject(INVOKE) on StructureStart.placeInChunk (StructureStartMixin.storeReferencePos).
// Why it's off: `horizon_world::worldgen::structure::start::place_in_chunk` is generic: horizon-mixins rejects generic targets.
// horizon-mixins kind: BEFORE_CALL on `horizon_world::worldgen::structure::start::place_in_chunk`
// Note: Only whole statements, plain let initializers and block tails can be selected.
// Note: The call it hooks (StructurePiece.postProcess) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   StructurePiece.postProcess -> (not mapped yet)
//   StructureStart.placeInChunk -> horizon_world::worldgen::structure::start::place_in_chunk (suggested, high)
//
// Decompiled Java, dev/worldgen/lithostitched/mixin/common/processor/StructureStartMixin.java:21:
// | @Inject(
// |    method = "placeInChunk",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/world/level/levelgen/structure/StructurePiece;postProcess(Lnet/minecraft/world/level/WorldGenLevel;Lnet/minecraft/world/level/StructureManager;Lnet/minecraft/world/level/chunk/ChunkGenerator;Lnet/minecraft/util/RandomSource;Lnet/minecraft/world/level/levelgen/structure/BoundingBox;Lnet/minecraft/world/level/ChunkPos;Lnet/minecraft/core/BlockPos;)V"
// |    )
// | )
// | private void storeReferencePos(
// |    WorldGenLevel level,
// |    StructureManager structureManager,
// |    ChunkGenerator generator,
// |    RandomSource random,
// |    BoundingBox chunkBB,
// |    ChunkPos chunkPos,
// |    CallbackInfo ci,
// |    @Local(ordinal = 1) BlockPos referencePos,
// |    @Local(ordinal = 0) StructurePiece piece
// | ) {
// |    ((ReferencePosDuck)piece).lithostitched$setReferencePos(referencePos);
// | }
