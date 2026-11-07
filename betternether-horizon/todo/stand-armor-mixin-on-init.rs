// Not ported yet: the Java @Inject(RETURN) on ArmorStandRenderer.<init>* (StandArmorMixin.onInit).
// Why it's off: No Horizon function mapped for ArmorStandRenderer.<init>* (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon's RETURN runs at every normal exit and passes the return value on.
// Note: Wildcard or unknown selector `<init>*`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ModelLayers.PLAYER_SLIM_ARMOR -> (not mapped yet)
//   ArmorStandArmorModel.<init> -> (not mapped yet)
//   ArmorModelSet.bake -> (not mapped yet)
//   Context.getEquipmentRenderer -> (not mapped yet)
//   Context.getModelSet -> (not mapped yet)
//   LivingEntityRenderer.<init> -> (not mapped yet)
//   HumanoidArmorLayer.<init> -> (not mapped yet)
//
// Decompiled Java, org/betterx/betternether/mixin/client/StandArmorMixin.java:28:
// | @Inject(method = "<init>*", at = @At("RETURN"))
// | private void onInit(Context ctx, CallbackInfo info) {
// |    if ((Boolean)Configs.CLIENT.thinArmor.get()) {
// |       for (RenderLayer<ArmorStandRenderState, ArmorStandArmorModel> feature : this.layers) {
// |          if (feature instanceof HumanoidArmorLayer) {
// |             this.layers.remove(feature);
// |             break;
// |          }
// |       }
// |
// |       this.layers
// |          .add(
// |             0,
// |             new HumanoidArmorLayer(
// |                this, ArmorModelSet.bake(ModelLayers.PLAYER_SLIM_ARMOR, ctx.getModelSet(), ArmorStandArmorModel::new), ctx.getEquipmentRenderer()
// |             )
// |          );
// |    }
// | }
