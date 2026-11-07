// Not ported yet: the Java @ModifyArg(INVOKE) on LivingEntity.dealDefaultKnockback (LivingEntityMixin.be_increaseKnockback).
// Why it's off: No Horizon function mapped for LivingEntity.dealDefaultKnockback (run `match`, or map it in a --mappings file).
// horizon-mixins kind: MODIFY_ARG
// Note: No Horizon function known for LivingEntity.dealDefaultKnockback: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Holder.is -> (not mapped yet)
//   DamageSource.getEntity -> (not mapped yet)
//   MobEffectInstance.getEffect -> (not mapped yet)
//   MobEffects.BLINDNESS -> (not mapped yet)
//   Entity.<init> -> (not mapped yet)
//   EquipmentSlot.MAINHAND -> (not mapped yet)
//   EquipmentSlot.isArmor -> (not mapped yet)
//   EquipmentSlot.values -> (not mapped yet)
//   LivingEntity.canBeAffected -> (not mapped yet)
//   LivingEntity.createLivingAttributes -> (not mapped yet)
//   LivingEntity.dealDefaultKnockback -> (not mapped yet)
//   LivingEntity.getMainHandItem -> (not mapped yet)
//   LivingEntity.hurtServer -> (not mapped yet)
//   LivingEntity.knockback -> horizon_client::mobs::Mob::knockback (suggested, high)
//   LivingEntity.tickEffects -> (not mapped yet)
//   AttributeMap.getValue -> (not mapped yet)
//   AttributeModifier.amount -> (not mapped yet)
//   Attributes.ATTACK_KNOCKBACK -> (not mapped yet)
//   ItemStack.forEachModifier -> (not mapped yet)
//   ItemStack.getItem -> (not mapped yet)
//   Level.isClientSide -> horizon_world::level::is_client_side (suggested, high)
//
// Decompiled Java, org/betterx/betterend/mixin/common/LivingEntityMixin.java:84:
// | @ModifyArg(
// |    method = "dealDefaultKnockback",
// |    at = @At(value = "INVOKE", target = "Lnet/minecraft/world/entity/LivingEntity;knockback(DDDLnet/minecraft/world/damagesource/DamageSource;F)V"),
// |    index = 0
// | )
// | private double be_increaseKnockback(double value) {
// |    if (this.be_lastAttacker != null && this.be_lastAttacker instanceof LivingEntity attacker) {
// |       value += this.be_getKnockback(attacker.getMainHandItem());
// |    }
// |
// |    return value;
// | }
