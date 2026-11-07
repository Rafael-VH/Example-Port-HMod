// Not ported yet: the Java @Inject(HEAD) on Enderman.isBeingStaredBy (EndermanEntityMixin.isPlayerStaring).
// Why it's off: No Horizon function mapped for Enderman.isBeingStaredBy (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for Enderman.isBeingStaredBy: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Enderman.isBeingStaredBy -> (not mapped yet)
//   Monster.<init> -> (not mapped yet)
//
// Decompiled Java, com/tiviacz/travelersbackpack/mixin/abilities/EndermanEntityMixin.java:20:
// | @Inject(at = @At("HEAD"), method = "isBeingStaredBy", cancellable = true)
// | public void isPlayerStaring(Player player, CallbackInfoReturnable<Boolean> cir) {
// |    BackpackAbilities.pumpkinAbility(player, cir);
// | }
