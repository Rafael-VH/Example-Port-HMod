// Not ported yet: the Java @ModifyReturnValue(RETURN) on NetherFossilPiece.makeSettings (NetherFossilPieceMixin.addShipwreckProcessor).
// Why it's off: No Horizon function mapped for NetherFossilPiece.makeSettings (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: No Horizon function known for NetherFossilPiece.makeSettings: run `match`.
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
// Decompiled Java, dev/worldgen/lithostitched/mixin/common/processor/NetherFossilPieceMixin.java:24:
// | @ModifyReturnValue(method = "makeSettings", at = @At("RETURN"))
// | private static StructurePlaceSettings addShipwreckProcessor(StructurePlaceSettings settings) {
// |    return Lithostitched.breaksSeedParity() ? settings.addProcessor(UnboundReferenceProcessor.of("nether_fossil")) : settings;
// | }
