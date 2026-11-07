// Not ported yet: the Java @Inject(INVOKE) on PaintingRenderer.submit (MixinPaintingEntityRenderer.etf$getSprites).
// Why it's off: No Horizon function mapped for PaintingRenderer.submit (run `match`, or map it in a --mappings file).
// horizon-mixins kind: BEFORE_CALL
// Note: Only whole statements, plain let initializers and block tails can be selected.
// Note: No Horizon function known for PaintingRenderer.submit: run `match`.
// Note: The call it hooks (PaintingRenderer.renderPainting) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   SubmitNodeCollector.submitCustomGeometry -> (not mapped yet)
//   EntityRenderer.<init> -> (not mapped yet)
//   PaintingRenderer.renderPainting -> (not mapped yet)
//   PaintingRenderer.submit -> (not mapped yet)
//   PaintingRenderState.direction -> (not mapped yet)
//   RenderTypes.entitySolid -> (not mapped yet)
//   RenderTypes.entityTranslucent -> (not mapped yet)
//   SpriteContents.name -> (not mapped yet)
//   TextureAtlasSprite.atlasLocation -> (not mapped yet)
//   TextureAtlasSprite.contents -> (not mapped yet)
//   TextureAtlasSprite.getU -> (not mapped yet)
//   TextureAtlasSprite.getU0 -> (not mapped yet)
//   TextureAtlasSprite.getU1 -> (not mapped yet)
//   TextureAtlasSprite.getV -> (not mapped yet)
//   TextureAtlasSprite.getV0 -> (not mapped yet)
//   TextureAtlasSprite.getV1 -> (not mapped yet)
//   BlockPos.<init> -> (not mapped yet)
//   Direction.EAST -> (not mapped yet)
//   Direction.NORTH -> (not mapped yet)
//   Direction.SOUTH -> (not mapped yet)
//   Direction.WEST -> (not mapped yet)
//   Direction.get2DDataValue -> (not mapped yet)
//   Holder.value -> (not mapped yet)
//   Identifier.getNamespace -> (not mapped yet)
//   Identifier.getPath -> (not mapped yet)
//   LightCoordsUtil.getLightCoords -> (not mapped yet)
//   Mth.floor -> horizon_client_render::gfx::occlusion::floor (suggested, high)
//   Painting.getBlockX -> (not mapped yet)
//   Painting.getBlockZ -> (not mapped yet)
//   Painting.getDirection -> (not mapped yet)
//   Painting.getVariant -> (not mapped yet)
//   Painting.getX -> (not mapped yet)
//   Painting.getY -> (not mapped yet)
//   Painting.getZ -> (not mapped yet)
//   Painting.level -> (not mapped yet)
//   PaintingVariant.height -> (not mapped yet)
//   PaintingVariant.width -> (not mapped yet)
//
// Decompiled Java, traben/entity_texture_features/mixin/mixins/entity/renderer/MixinPaintingEntityRenderer.java:59:
// | @Inject(
// |    method = "submit(Lnet/minecraft/client/renderer/entity/state/PaintingRenderState;Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;Lnet/minecraft/client/renderer/state/level/CameraRenderState;)V",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/client/renderer/entity/PaintingRenderer;renderPainting(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;Lnet/minecraft/client/renderer/rendertype/RenderType;[IIILnet/minecraft/client/renderer/texture/TextureAtlasSprite;Lnet/minecraft/client/renderer/texture/TextureAtlasSprite;)V"
// |    )
// | )
// | private void etf$getSprites(
// |    PaintingRenderState paintingRenderState,
// |    PoseStack matrixStack,
// |    SubmitNodeCollector submitNodeCollector,
// |    CameraRenderState cameraRenderState,
// |    CallbackInfo ci,
// |    @Local(ordinal = 0) TextureAtlasSprite paintingSprite,
// |    @Local(ordinal = 1) TextureAtlasSprite backSprite
// | ) {
// |    ETFEntityRenderState etfEntity = ((HoldsETFRenderState)paintingRenderState).etf$getState();
// |    if (etfEntity != null && etfEntity.entity() instanceof Painting paintingEntity) {
// |       float var27 = paintingRenderState.direction.get2DDataValue() * 90;
// |
// |       try {
// |          Identifier paintingId = paintingSprite.contents().name();
// |          String paintingFileName = paintingId.getPath();
// |          Identifier paintingTexture = ETFUtils2.res(paintingId.getNamespace(), "textures/painting/" + paintingFileName + ".png");
// |          boolean aztec = "aztec".equals(paintingFileName);
// |          if (aztec) {
// |             ETFState.onlyRandomizeViaPropertiesFiles = true;
// |          }
// |
// |          ETFTexture frontTexture = ETFManager.getInstance().getETFTextureVariant(paintingTexture, etfEntity);
// |          ETFSprite etf$Sprite = frontTexture.getPaintingSprite(paintingSprite, paintingTexture);
// |          if (aztec) {
// |             ETFState.onlyRandomizeViaPropertiesFiles = false;
// |          }
// |
// |          ETFTexture backTexture = ETFManager.getInstance().getETFTextureVariant(etf$BACK_SPRITE_ID, etfEntity);
// |          ETFSprite etf$BackSprite = backTexture.getPaintingSprite(backSprite, etf$BACK_SPRITE_ID);
// |          if (etf$Sprite.isETFAltered || etf$Sprite.isEmissive() || etf$BackSprite.isETFAltered || etf$BackSprite.isEmissive()) {
// |             PaintingVariant paintingVariant = (PaintingVariant)paintingEntity.getVariant().value();
// |             int width = paintingVariant.width();
// |             int height = paintingVariant.height();
// |             ETFState.pushRenderLayerModifyState(false);
// |             RenderType type = RenderTypes.entitySolid(etf$Sprite.getSpriteVariant().atlasLocation());
// |             RenderType back = RenderTypes.entitySolid(etf$BackSprite.getSpriteVariant().atlasLocation());
// |             RenderType typeE = RenderTypes.entityTranslucent(etf$Sprite.getEmissive().atlasLocation());
// |             RenderType backE = RenderTypes.entityTranslucent(etf$BackSprite.getEmissive().atlasLocation());
// |             ETFState.popRenderLayerModifyState();
// |             submitNodeCollector.submitCustomGeometry(
// |                matrixStack,
// |                type,
// |                (pose, vertexConsumer) -> this.etf$renderETFPaintingFront(
// |                   pose, vertexConsumer, paintingEntity, width, height, etf$Sprite.getSpriteVariant(), false
// |                )
// |             );
// |             submitNodeCollector.submitCustomGeometry(
// |                matrixStack,
// |                back,
// |                (pose, vertexConsumer) -> this.etf$renderETFPaintingBack(
// |                   pose, vertexConsumer, paintingEntity, width, height, etf$BackSprite.getSpriteVariant(), false
// |                )
// |             );
// |             if (etf$Sprite.isEmissive()) {
// |                submitNodeCollector.submitCustomGeometry(
// |                   matrixStack,
// |                   typeE,
// |                   (pose, vertexConsumer) -> this.etf$renderETFPaintingFront(
// |                      pose, vertexConsumer, paintingEntity, width, height, etf$Sprite.getSpriteVariant(), true
// |                   )
// |                );
// |             }
// |
// |             if (etf$BackSprite.isEmissive()) {
// |                submitNodeCollector.submitCustomGeometry(
// |                   matrixStack,
// |                   backE,
// |                   (pose, vertexConsumer) -> this.etf$renderETFPaintingFront(
// |                      pose, vertexConsumer, paintingEntity, width, height, etf$BackSprite.getSpriteVariant(), true
// |                   )
// |                );
// |             }
// |          }
// |       } catch (Exception var26) {
// |       }
// |    }
// | }
