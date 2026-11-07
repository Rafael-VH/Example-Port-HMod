// Not ported yet: the Java @Inject(HEAD) on Player.canEat (NourishmentAlwaysEatMixin.alwaysEatUnderNourishmentEffect).
// Why it's off: No Horizon function mapped for Player.canEat (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for Player.canEat: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Player.canEat -> (not mapped yet)
//   Player.hasEffect -> horizon_client::mobs::PlayerView::has_effect (suggested, high)
//
// Decompiled Java, vectorwing/farmersdelight/common/mixin/NourishmentAlwaysEatMixin.java:12:
// | @Inject(method = "canEat", at = @At("HEAD"), cancellable = true)
// | private void alwaysEatUnderNourishmentEffect(boolean canAlwaysEat, CallbackInfoReturnable<Boolean> cir) {
// |    if (((Player)this).hasEffect(ModEffects.NOURISHMENT)) {
// |       cir.setReturnValue(true);
// |    }
// | }
