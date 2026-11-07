// Not ported yet: the Java @ModifyVariable(HEAD) on StructurePiece.<init> (StructurePieceMixin.tectonic$lowerOceanMonuments).
// Why it's off: No Horizon function mapped for StructurePiece.<init> (run `match`, or map it in a --mappings file).
// horizon-mixins kind: MODIFY_VARIABLE
// Note: Selects a Rust local by name: pick it in the target function.
// Note: No Horizon function known for StructurePiece.<init>: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   BoundingBox.moved -> (not mapped yet)
//   StructurePiece.<init> -> (not mapped yet)
//   StructurePieceType.OCEAN_MONUMENT_BUILDING -> (not mapped yet)
//
// Decompiled Java, dev/worldgen/tectonic/mixin/StructurePieceMixin.java:14:
// | @ModifyVariable(
// |    method = "<init>(Lnet/minecraft/world/level/levelgen/structure/pieces/StructurePieceType;ILnet/minecraft/world/level/levelgen/structure/BoundingBox;)V",
// |    at = @At("HEAD"),
// |    ordinal = 0,
// |    argsOnly = true
// | )
// | private static BoundingBox tectonic$lowerOceanMonuments(BoundingBox boundingBox, StructurePieceType type) {
// |    return type == StructurePieceType.OCEAN_MONUMENT_BUILDING && ((ConfigState)Tectonic.CONFIG.getState()).general.modEnabled
// |       ? boundingBox.moved(0, ((ConfigState)Tectonic.CONFIG.getState()).oceans.monumentOffset, 0)
// |       : boundingBox;
// | }
