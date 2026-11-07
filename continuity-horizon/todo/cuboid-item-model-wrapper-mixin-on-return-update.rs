// Not ported yet: the Java @Inject(RETURN) on CuboidItemModelWrapper.update (CuboidItemModelWrapperMixin.onReturnUpdate).
// Why it's off: No Horizon function mapped for CuboidItemModelWrapper.update (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon's RETURN runs at every normal exit and passes the return value on.
// Note: No Horizon function known for CuboidItemModelWrapper.update: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   CuboidItemModelWrapper.<init> -> (not mapped yet)
//   CuboidItemModelWrapper.update -> (not mapped yet)
//   ItemStackRenderState.appendModelIdentityElement -> (not mapped yet)
//   ItemStackRenderState.setAnimated -> (not mapped yet)
//   LayerRenderState.emitter -> (not mapped yet)
//   LayerRenderState.setQuads -> (not mapped yet)
//
// Decompiled Java, me/pepperbell/continuity/client/mixin/CuboidItemModelWrapperMixin.java:49:
// | @Inject(
// |    method = "update(Lnet/minecraft/client/renderer/item/ItemStackRenderState;Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/client/renderer/item/ItemModelResolver;Lnet/minecraft/world/item/ItemDisplayContext;Lnet/minecraft/client/multiplayer/ClientLevel;Lnet/minecraft/world/entity/ItemOwner;I)V",
// |    at = @At("RETURN")
// | )
// | private void onReturnUpdate(
// |    ItemStackRenderState output,
// |    ItemStack item,
// |    ItemModelResolver resolver,
// |    ItemDisplayContext displayContext,
// |    @Nullable ClientLevel level,
// |    @Nullable ItemOwner owner,
// |    int seed,
// |    CallbackInfo ci,
// |    @Local LayerRenderState layer
// | ) {
// |    if (this.emissiveGeometry != null && (Boolean)ContinuityConfig.INSTANCE.emissiveTextures.get()) {
// |       if (this.emissiveGeometry.baseAndEmissiveItemQuads() != null) {
// |          layer.setQuads(this.emissiveGeometry.baseAndEmissiveItemQuads());
// |       } else if (this.emissiveGeometry.emissiveMesh() != null) {
// |          this.emissiveGeometry.emissiveMesh().outputTo(layer.emitter());
// |       }
// |
// |       output.appendModelIdentityElement(EmissiveItemGeometry.EMISSIVE_GEOMETRY_MARKER);
// |       if (this.emissiveGeometry.emissiveAnimated()) {
// |          output.setAnimated();
// |       }
// |    }
// | }
