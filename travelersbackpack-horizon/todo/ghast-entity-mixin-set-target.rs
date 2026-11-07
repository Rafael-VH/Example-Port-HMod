// Not ported yet: the Java @Inject(HEAD) on Mob.setTarget (GhastEntityMixin.setTarget).
// Why it's off: No Horizon function mapped for Mob.setTarget (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for Mob.setTarget: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   LivingEntity.<init> -> (not mapped yet)
//   Mob.setTarget -> (not mapped yet)
//
// Decompiled Java, com/tiviacz/travelersbackpack/mixin/abilities/GhastEntityMixin.java:21:
// | @Inject(at = @At("HEAD"), method = "setTarget", cancellable = true)
// | public void setTarget(LivingEntity target, CallbackInfo ci) {
// |    if (TravelersBackpackConfig.serverSpec.isLoaded()
// |       && (Boolean)TravelersBackpackConfig.SERVER.backpackAbilities.enableBackpackAbilities.get()
// |       && this instanceof Ghast ghast) {
// |       BackpackAbilities.ghastAbility(ghast, target, ci);
// |    }
// | }
