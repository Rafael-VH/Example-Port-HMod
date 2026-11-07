// Not ported yet: the Java @Inject(INVOKE) on Player.attack (PlayerMixin.captureAttackStrengthScale).
// Why it's off: `horizon_client::game::Game<'_>::attack` has a closure in its body: horizon-mixins doesn't rewrite such bodies with BEFORE_CALL. A HEAD hook on it, or a BEFORE_CALL on a function it calls, may work instead.
// horizon-mixins kind: BEFORE_CALL on `horizon_client::game::Game<'_>::attack`
// Note: Only whole statements, plain let initializers and block tails can be selected.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   LivingEntity.<init> -> (not mapped yet)
//   Player.attack -> horizon_client::game::Game<'_>::attack (suggested, high)
//   Player.getAttackStrengthScale -> horizon_client::game::Game<'_>::attack_strength_scale (suggested, high)
//   Player.hurtServer -> (not mapped yet)
//
// Decompiled Java, vectorwing/farmersdelight/common/mixin/refabricated/PlayerMixin.java:27:
// | @Inject(method = "attack", at = @At(value = "INVOKE", target = "Lnet/minecraft/world/entity/player/Player;getAttackStrengthScale(F)F"))
// | private void captureAttackStrengthScale(Entity target, CallbackInfo ci) {
// |    SkilletItem.SkilletEvents.attackPower = this.getAttackStrengthScale(0.0F);
// | }
