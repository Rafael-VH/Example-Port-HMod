// Not ported yet: the Java @WrapOperation(NEW) on Unbaked.bake (CuboidItemModelWrapperUnbakedMixin.wrapModelCtor).
// Why it's off: No Horizon function mapped for Unbaked.bake (run `match`, or map it in a --mappings file).
// horizon-mixins kind: WRAP_OPERATION
// Note: No Horizon function known for Unbaked.bake: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Unbaked.bake -> (not mapped yet)
//   BakingContext.blockModelBaker -> (not mapped yet)
//   ModelBaker.materials -> (not mapped yet)
//
// Decompiled Java, me/pepperbell/continuity/client/mixin/CuboidItemModelWrapperUnbakedMixin.java:19:
// | @WrapOperation(
// |    method = "bake(Lnet/minecraft/client/renderer/item/ItemModel$BakingContext;Lorg/joml/Matrix4fc;)Lnet/minecraft/client/renderer/item/ItemModel;",
// |    at = @At(
// |       value = "NEW",
// |       target = "(Ljava/util/List;Lnet/minecraft/client/resources/model/geometry/QuadCollection;Lnet/minecraft/client/renderer/item/ModelRenderProperties;Lorg/joml/Matrix4fc;)Lnet/minecraft/client/renderer/item/CuboidItemModelWrapper;"
// |    )
// | )
// | private CuboidItemModelWrapper wrapModelCtor(
// |    List<ItemTintSource> tints,
// |    QuadCollection quads,
// |    ModelRenderProperties properties,
// |    Matrix4fc transformation,
// |    Operation<CuboidItemModelWrapper> original,
// |    BakingContext context,
// |    Matrix4fc transformation1
// | ) {
// |    CuboidItemModelWrapperInitContext.SPRITE_FINDER_GETTER.set(context.blockModelBaker().materials());
// |    CuboidItemModelWrapper model = (CuboidItemModelWrapper)original.call(new Object[]{tints, quads, properties, transformation});
// |    CuboidItemModelWrapperInitContext.SPRITE_FINDER_GETTER.remove();
// |    return model;
// | }
