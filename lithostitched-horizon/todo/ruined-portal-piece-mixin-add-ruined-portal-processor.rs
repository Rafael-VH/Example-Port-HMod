// Not ported yet: the Java @ModifyReturnValue(RETURN) on RuinedPortalPiece.makeSettings (RuinedPortalPieceMixin.addRuinedPortalProcessor).
// Why it's off: No Horizon function mapped for RuinedPortalPiece.makeSettings (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: No Horizon function known for RuinedPortalPiece.makeSettings: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   LevelAccessor.setBlockAndUpdate -> (not mapped yet)
//   BlockState.is -> (not mapped yet)
//   StructurePiece.<init> -> (not mapped yet)
//   RuinedPortalPiece.makeSettings -> (not mapped yet)
//   RuinedPortalPiece.maybeAddLeavesAbove -> (not mapped yet)
//   RuinedPortalPiece.maybeAddVines -> (not mapped yet)
//   RuinedPortalPiece.placeNetherrackOrMagma -> (not mapped yet)
//   StructurePlaceSettings.addProcessor -> (not mapped yet)
//
// Decompiled Java, dev/worldgen/lithostitched/mixin/common/processor/RuinedPortalPieceMixin.java:28:
// | @ModifyReturnValue(
// |    method = "makeSettings(Lnet/minecraft/core/HolderLookup$Provider;Lnet/minecraft/world/level/block/Mirror;Lnet/minecraft/world/level/block/Rotation;Lnet/minecraft/world/level/levelgen/structure/structures/RuinedPortalPiece$VerticalPlacement;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/levelgen/structure/structures/RuinedPortalPiece$Properties;)Lnet/minecraft/world/level/levelgen/structure/templatesystem/StructurePlaceSettings;",
// |    at = @At("RETURN")
// | )
// | private static StructurePlaceSettings addRuinedPortalProcessor(StructurePlaceSettings settings) {
// |    return Lithostitched.breaksSeedParity() ? settings.addProcessor(UnboundReferenceProcessor.of("ruined_portal")) : settings;
// | }
