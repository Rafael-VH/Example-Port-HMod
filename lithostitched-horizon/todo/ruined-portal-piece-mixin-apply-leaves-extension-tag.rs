// Not ported yet: the Java @WrapOperation(INVOKE) on RuinedPortalPiece.maybeAddLeavesAbove (RuinedPortalPieceMixin.applyLeavesExtensionTag).
// Why it's off: No Horizon function mapped for RuinedPortalPiece.maybeAddLeavesAbove (run `match`, or map it in a --mappings file).
// horizon-mixins kind: WRAP_OPERATION
// Note: No Horizon function known for RuinedPortalPiece.maybeAddLeavesAbove: run `match`.
// Note: The call it hooks (BlockState.is) isn't mapped to a Rust function yet.
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
// Decompiled Java, dev/worldgen/lithostitched/mixin/common/processor/RuinedPortalPieceMixin.java:53:
// | @WrapOperation(
// |    method = "maybeAddLeavesAbove",
// |    at = @At(value = "INVOKE", target = "Lnet/minecraft/world/level/block/state/BlockState;is(Ljava/lang/Object;)Z")
// | )
// | private boolean applyLeavesExtensionTag(BlockState state, Object object, Operation<Boolean> operation) {
// |    return (Boolean)operation.call(new Object[]{state, object}) || state.is(LithostitchedBlockTags.SUPPORTS_RUINED_PORTAL_LEAVES);
// | }
