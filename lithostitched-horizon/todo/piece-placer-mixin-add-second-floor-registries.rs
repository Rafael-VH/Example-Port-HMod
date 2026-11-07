// Not ported yet: the Java @WrapOperation(NEW) on MansionPiecePlacer.createMansion (PiecePlacerMixin.addSecondFloorRegistries).
// Why it's off: No Horizon function mapped for MansionPiecePlacer.createMansion (run `match`, or map it in a --mappings file).
// horizon-mixins kind: WRAP_OPERATION
// Note: No Horizon function known for MansionPiecePlacer.createMansion: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   MansionPiecePlacer.createMansion -> (not mapped yet)
//
// Decompiled Java, dev/worldgen/lithostitched/mixin/common/template/mansion/PiecePlacerMixin.java:38:
// | @WrapOperation(
// |    method = "createMansion",
// |    at = @At(value = "NEW", target = "net/minecraft/world/level/levelgen/structure/structures/WoodlandMansionPieces$SecondFloorRoomCollection")
// | )
// | private SecondFloorRoomCollection addSecondFloorRegistries(Operation<SecondFloorRoomCollection> operation) {
// |    SecondFloorRoomCollection collection = (SecondFloorRoomCollection)operation.call(new Object[0]);
// |    ((RegistryHolder)collection).setRegistries(this.registries);
// |    return collection;
// | }
