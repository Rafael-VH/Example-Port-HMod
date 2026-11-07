// Not ported yet: the Java @Inject(HEAD) on Player.hurtServer (PlayerMixin.handleSkilletAttackSound).
// Why it's off: No Horizon function mapped for Player.hurtServer (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for Player.hurtServer: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   LivingEntity.<init> -> (not mapped yet)
//   Player.attack -> horizon_client::game::Game<'_>::attack (suggested, high)
//   Player.getAttackStrengthScale -> horizon_client::game::Game<'_>::attack_strength_scale (suggested, high)
//   Player.hurtServer -> (not mapped yet)
//
// Decompiled Java, vectorwing/farmersdelight/common/mixin/refabricated/PlayerMixin.java:32:
// | @Inject(method = "hurtServer", at = @At("HEAD"))
// | private void handleSkilletAttackSound(ServerLevel level, DamageSource source, float amount, CallbackInfoReturnable<Boolean> cir) {
// |    SkilletItem.SkilletEvents.playSkilletAttackSound(this, source);
// | }
