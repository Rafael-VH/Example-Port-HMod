// Not ported yet: the Java @WrapOperation(INVOKE) on Beardifier.forStructuresInChunk (BeardifierMixin.overrideTerrainAdaptation).
// Why it's off: No Horizon function mapped for Beardifier.forStructuresInChunk (run `match`, or map it in a --mappings file).
// horizon-mixins kind: WRAP_OPERATION
// Note: No Horizon function known for Beardifier.forStructuresInChunk: run `match`.
// Note: The call it hooks (List.add) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   List.add -> (not mapped yet)
//   Beardifier.forStructuresInChunk -> (not mapped yet)
//   Rigid.<init> -> (not mapped yet)
//   PoolElementStructurePiece.getBoundingBox -> (not mapped yet)
//   PoolElementStructurePiece.getElement -> (not mapped yet)
//   PoolElementStructurePiece.getGroundLevelDelta -> (not mapped yet)
//
// Decompiled Java, dev/worldgen/lithostitched/mixin/common/BeardifierMixin.java:19:
// | @WrapOperation(
// |    method = "forStructuresInChunk(Lnet/minecraft/world/level/StructureManager;Lnet/minecraft/world/level/ChunkPos;)Lnet/minecraft/world/level/levelgen/Beardifier;",
// |    at = @At(value = "INVOKE", target = "Ljava/util/List;add(Ljava/lang/Object;)Z", ordinal = 0)
// | )
// | private static boolean overrideTerrainAdaptation(
// |    List<Rigid> list, Object rigid, Operation<Boolean> operation, @Local(ordinal = 0) StructurePiece structurePiece
// | ) {
// |    PoolElementStructurePiece piece = (PoolElementStructurePiece)structurePiece;
// |    if (piece.getElement() instanceof DelegatingPoolElement delegating) {
// |       Optional<TerrainAdjustment> terrainAdaptation = delegating.config().overrideTerrainAdaptation();
// |       if (terrainAdaptation.isPresent()) {
// |          return (Boolean)operation.call(new Object[]{list, new Rigid(piece.getBoundingBox(), terrainAdaptation.get(), piece.getGroundLevelDelta())});
// |       }
// |    }
// |
// |    return (Boolean)operation.call(new Object[]{list, rigid});
// | }
