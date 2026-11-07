// Not ported yet: the Java @ModifyVariable(HEAD) on LivingEntity.hurtServer (LivingEntityMixin.handleBackstabbingDamage).
// Why it's off: No Horizon function mapped for LivingEntity.hurtServer (run `match`, or map it in a --mappings file).
// horizon-mixins kind: MODIFY_VARIABLE
// Note: Selects a Rust local by name: pick it in the target function.
// Note: No Horizon function known for LivingEntity.hurtServer: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Entity.<init> -> (not mapped yet)
//   LivingEntity.hurtServer -> (not mapped yet)
//   LivingEntity.knockback -> horizon_client::mobs::Mob::knockback (suggested, high)
//
// Decompiled Java, vectorwing/farmersdelight/common/mixin/refabricated/LivingEntityMixin.java:22:
// | @ModifyVariable(method = "hurtServer", at = @At("HEAD"), argsOnly = true)
// | private float handleBackstabbingDamage(float original, ServerLevel serverLevel, DamageSource source) {
// |    if (original > 0.0F) {
// |       SkilletItem.SkilletEvents.playSkilletAttackSound((LivingEntity)this, source);
// |       return BackstabbingEnchantment.BackstabbingEvent.onKnifeBackstab((LivingEntity)this, source, original);
// |    } else {
// |       return original;
// |    }
// | }
