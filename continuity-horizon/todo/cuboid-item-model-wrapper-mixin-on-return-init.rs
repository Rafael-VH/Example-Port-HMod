// Not ported yet: the Java @Inject(RETURN) on CuboidItemModelWrapper.<init> (CuboidItemModelWrapperMixin.onReturnInit).
// Why it's off: No Horizon function mapped for CuboidItemModelWrapper.<init> (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon's RETURN runs at every normal exit and passes the return value on.
// Note: No Horizon function known for CuboidItemModelWrapper.<init>: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   CuboidItemModelWrapper.<init> -> (not mapped yet)
//   CuboidItemModelWrapper.update -> (not mapped yet)
//   ItemStackRenderState.appendModelIdentityElement -> (not mapped yet)
//   ItemStackRenderState.setAnimated -> (not mapped yet)
//   LayerRenderState.emitter -> (not mapped yet)
//   LayerRenderState.setQuads -> (not mapped yet)
//
// Decompiled Java, me/pepperbell/continuity/client/mixin/CuboidItemModelWrapperMixin.java:40:
// | @Inject(
// |    method = "<init>(Ljava/util/List;Lnet/minecraft/client/resources/model/geometry/QuadCollection;Lnet/minecraft/client/renderer/item/ModelRenderProperties;Lorg/joml/Matrix4fc;)V",
// |    at = @At("RETURN")
// | )
// | private void onReturnInit(List<ItemTintSource> tints, QuadCollection quads, ModelRenderProperties properties, Matrix4fc transformation, CallbackInfo ci) {
// |    SpriteFinderGetter spriteFinderGetter = CuboidItemModelWrapperInitContext.SPRITE_FINDER_GETTER.get();
// |    this.emissiveGeometry = EmissiveItemGeometry.of(quads, this.itemQuads, spriteFinderGetter);
// | }
