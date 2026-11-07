// Not ported yet: the Java @Inject(HEAD) on LivingEntity.tickEffects (LivingEntityMixin.be_applyEffects).
// Why it's off: No Horizon function mapped for LivingEntity.tickEffects (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for LivingEntity.tickEffects: run `match`.
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
// Decompiled Java, org/betterx/betterend/mixin/common/LivingEntityMixin.java:49:
// | @Inject(method = "tickEffects", at = @At("HEAD"))
// | protected void be_applyEffects(CallbackInfo info) {
// |    if (!this.level().isClientSide()) {
// |       LivingEntity owner = LivingEntity.class.cast(this);
// |       if (CrystaliteArmor.hasFullSet(owner)) {
// |          CrystaliteArmor.applySetEffect(owner);
// |       }
// |
// |       for (EquipmentSlot slot : EquipmentSlot.values()) {
// |          if (slot.isArmor()) {
// |             ItemStack itemStack = this.getItemBySlot(slot);
// |             if (itemStack.getItem() instanceof MobEffectApplier) {
// |                ((MobEffectApplier)itemStack.getItem()).applyEffect(owner);
// |             }
// |          }
// |       }
// |    }
// | }
