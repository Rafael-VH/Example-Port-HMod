// Not ported yet: the Java @Inject(TAIL) on ArmadilloModel.setupAnim (MixinArmadilloModel.emf$assertLayerFactory).
// Why it's off: No Horizon function mapped for ArmadilloModel.setupAnim (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon has no TAIL: RETURN runs at every exit, early ones too.
// Note: No Horizon function known for ArmadilloModel.setupAnim: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   EntityModel.<init> -> (not mapped yet)
//   ArmadilloModel.setupAnim -> (not mapped yet)
//   ModelPart.skipDraw -> (not mapped yet)
//   ModelPart.visible -> (not mapped yet)
//
// Decompiled Java, traben/entity_model_features/mixin/mixins/rendering/model/MixinArmadilloModel.java:25:
// | @Inject(method = "setupAnim(Lnet/minecraft/client/renderer/entity/state/ArmadilloRenderState;)V", at = @At("TAIL"))
// | private void emf$assertLayerFactory(CallbackInfo ci) {
// |    if (this.body instanceof EMFModelPartVanilla emfPart) {
// |       boolean set = !this.body.skipDraw;
// |
// |       for (ModelPart sub : emfPart.getAllEMFCustomChildren()) {
// |          sub.visible = set;
// |       }
// |    }
// | }
