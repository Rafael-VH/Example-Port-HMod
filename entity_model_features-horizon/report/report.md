# Horizon port report: Entity Model Features

`entity_model_features` 3.3.12 · entity_model_features-3.3.12-26.3-fabric.jar · 281 classes

**Partly convertible**

| Status | Count | Meaning |
|---|---|---|
| auto | 1 | The Horizon hook or injection is known: the wiring can be generated. The Java body still has to become Rust. |
| content | 1 | Data and assets: horizon-port-pack carries these into a content mod (.hmod). |
| needs-mapping | 96 | The kind of change carries over, but the matching Horizon function isn't in the table yet. Run `match`. |
| needs-translation | 15 | No direct equivalent: a person (or an AI agent) has to rewrite this part. |
| unsupported | 0 | Horizon has nothing this can attach to yet. |

## Entry points

- `client`: traben.entity_model_features.EMFInit
- `modmenu`: traben.entity_model_features.config.EMFModMenu
- `ebe_v1`: traben.entity_model_features.mod_compat.EBEConfigModifier

## Mixins

| Injection | In | Horizon | Status | Note |
|---|---|---|---|---|
| mixin into BlockEntity with no injectors (interfaces or @Shadow only) | `MixinBlockEntity` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| @Inject(RETURN) on BlockEntityRenderers.createEntityRenderers | `MixinBlockEntityRendererFactories.emf$clearMarker` | RETURN | needs-mapping | Horizon's RETURN runs at every normal exit and passes the return value on. No Horizon function known for BlockEntityRenderers.createEntityRenderers: run `match`. |
| @ModifyArg(INVOKE) on BlockEntityRenderers.createEntityRenderers at Ljava/util/Map;forEach(Ljava/util/function/BiConsumer;)V | `MixinBlockEntityRendererFactories.setEmf$Model` | MODIFY_ARG | needs-mapping | No Horizon function known for BlockEntityRenderers.createEntityRenderers: run `match`. The call it hooks (Map.forEach) isn't mapped to a Rust function yet. |
| mixin into CancelTarget with no injectors (interfaces or @Shadow only) | `MixinCapeModels` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| mixin into Entity with no injectors (interfaces or @Shadow only) | `MixinEntity` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| @Inject(RETURN) on EntityModelSet.bakeLayer | `MixinEntityModelLoader.emf$injectModelLoad` | RETURN | needs-mapping | Horizon's RETURN runs at every normal exit and passes the return value on. No Horizon function known for EntityModelSet.bakeLayer: run `match`. |
| @ModifyExpressionValue(INVOKE) on EntityRenderer.extractShadow at Lnet/minecraft/client/renderer/entity/EntityRenderer;getShadowRadius(Lnet/minecraft/client/renderer/entity/state/EntityRenderState;)F | `MixinEntityRenderer.radius` | MODIFY_EXPRESSION_VALUE | needs-mapping | Selects an exact Rust expression: pick it in the target function. No Horizon function known for EntityRenderer.extractShadow: run `match`. |
| @ModifyExpressionValue(INVOKE) on EntityRenderer.extractShadow at Lnet/minecraft/client/renderer/entity/EntityRenderer;getShadowStrength(Lnet/minecraft/client/renderer/entity/state/EntityRenderState;)F | `MixinEntityRenderer.strength` | MODIFY_EXPRESSION_VALUE | needs-mapping | Selects an exact Rust expression: pick it in the target function. No Horizon function known for EntityRenderer.extractShadow: run `match`. |
| @Inject(HEAD) on GameRenderer.render | `MixinGameRenderer.emf$injectCounter` | HEAD on hook `frame` | auto | Target mapping is approximate. |
| @Inject(TAIL) on Model.<init> | `MixinModel.emf$discoverEMFModel` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for Model.<init>: run `match`. |
| @Inject(HEAD) on Model.renderType | `MixinModel.emf$discoverEMFModel` | HEAD | needs-mapping | No Horizon function known for Model.renderType: run `match`. |
| @Inject(TAIL) on Cube.<init> | `MixinModelPart$Cuboid.emf$injectAnnouncerCube` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for Cube.<init>: run `match`. |
| @Inject(HEAD) on ModelPart.render | `MixinModelPart.emf$injectAnnouncerPart` | HEAD | needs-mapping | No Horizon function known for ModelPart.render: run `match`. |
| @Inject(RETURN) on PartDefinition.bake | `MixinModelPartData.emf$injectAnnouncerData` | RETURN | needs-mapping | Horizon's RETURN runs at every normal exit and passes the return value on. No Horizon function known for PartDefinition.bake: run `match`. |
| @ModifyExpressionValue(INVOKE) on ParrotOnShoulderLayer.<init> at Lnet/minecraft/client/model/geom/EntityModelSet;bakeLayer(Lnet/minecraft/client/model/geom/ModelLayerLocation;)Lnet/minecraft/client/model/geom/ModelPart; | `MixinParrotEntityModel.emf$injectParrotShoulderLayer` | MODIFY_EXPRESSION_VALUE | needs-mapping | Selects an exact Rust expression: pick it in the target function. No Horizon function known for ParrotOnShoulderLayer.<init>: run `match`. |
| @Inject(HEAD) on ParrotOnShoulderLayer.submitOnShoulder | `MixinParrotEntityModel.emf$parrot1` | HEAD | needs-mapping | No Horizon function known for ParrotOnShoulderLayer.submitOnShoulder: run `match`. |
| @Inject(TAIL) on ParrotOnShoulderLayer.submitOnShoulder | `MixinParrotEntityModel.emf$parrot2` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for ParrotOnShoulderLayer.submitOnShoulder: run `match`. |
| @Inject(HEAD) on Player.interactOn | `MixinPlayerEntity.emf$injected` | HEAD | needs-mapping | No Horizon function known for Player.interactOn: run `match`. |
| @Inject(HEAD) on Minecraft.onResourceLoadFinished | `MixinResourceReloadEnd.emf$reloadFinish` | HEAD | needs-mapping | No Horizon function known for Minecraft.onResourceLoadFinished: run `match`. |
| @Inject(HEAD) on Minecraft.reloadResourcePacks | `MixinResourceReloadStart.emf$reloadStart` | HEAD | needs-mapping | No Horizon function known for Minecraft.reloadResourcePacks: run `match`. |
| @Accessor on AgeableMobRenderer.babyModel | `AgeableMobRendererAccessor.getBabyModel` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for AgeableMobRenderer.babyModel: run `match`. |
| @Accessor on Cube.polygons | `CuboidAccessor.setPolygons` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for Cube.polygons: run `match`. |
| @Accessor on Cube.minX | `CuboidAccessor.setMinX` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for Cube.minX: run `match`. |
| @Accessor on Cube.minY | `CuboidAccessor.setMinY` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for Cube.minY: run `match`. |
| @Accessor on Cube.minZ | `CuboidAccessor.setMinZ` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for Cube.minZ: run `match`. |
| @Accessor on Cube.maxX | `CuboidAccessor.setMaxX` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for Cube.maxX: run `match`. |
| @Accessor on Cube.maxY | `CuboidAccessor.setMaxY` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for Cube.maxY: run `match`. |
| @Accessor on Cube.maxZ | `CuboidAccessor.setMaxZ` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for Cube.maxZ: run `match`. |
| @Accessor on Minecraft.deltaTracker | `MinecraftClientAccessor.getDeltaTracker` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for Minecraft.deltaTracker: run `match`. |
| @Inject(HEAD) on GuiRenderer.render | `Mixin_GuiEntityTester.etf$beforeRenderToTexture` | HEAD | needs-mapping | No Horizon function known for GuiRenderer.render: run `match`. |
| @Inject(TAIL) on GuiRenderer.render | `Mixin_GuiEntityTester.etf$afterRenderToTexture` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for GuiRenderer.render: run `match`. |
| @Inject(HEAD) on BabyModelTransform.apply | `MixinBabyModelTransform.emf$cancel` | HEAD | needs-mapping | No Horizon function known for BabyModelTransform.apply: run `match`. |
| @Inject(RETURN) on EntityModelSet.vanilla | `MixinEntityModelSet.emf$unModifiedRoots` | RETURN | needs-mapping | Horizon's RETURN runs at every normal exit and passes the return value on. No Horizon function known for EntityModelSet.vanilla: run `match`. |
| @Inject(HEAD) on MeshTransformer.lambda$scaling$0 | `MixinMeshTransformer.emf$cancel` | HEAD | needs-mapping | No Horizon function known for MeshTransformer.lambda$scaling$0: run `match`. |
| @Inject(HEAD) on BannerRenderer.submitPatterns | `MixinBannerRenderer.emf$injected` | HEAD | needs-mapping | No Horizon function known for BannerRenderer.submitPatterns: run `match`. |
| @Inject(HEAD) on LayerRenderState.submit | `MixinBlockEntityWithoutLevelRenderer_skull.emf$setRenderFactory` | HEAD | needs-mapping | No Horizon function known for LayerRenderState.submit: run `match`. |
| @Inject(INVOKE) on EnderDragonRenderer.submit at Lnet/minecraft/client/renderer/SubmitNodeCollector;submitModel(Lnet/minecraft/client/model/Model;Ljava/lang/Object;Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/rendertype/RenderType;III)V | `MixinDragonRenderer.emf$allowMultiPartRender4` | BEFORE_CALL | needs-mapping | Only whole statements, plain let initializers and block tails can be selected. No Horizon function known for EnderDragonRenderer.submit: run `match`. The call it hooks (SubmitNodeCollector.submitModel) isn't mapped to a Rust function yet. |
| @Inject(RETURN) on EntityRenderDispatcher.submit | `MixinEntityRenderDispatcher.emf$endOfRender` | RETURN | needs-mapping | Horizon's RETURN runs at every normal exit and passes the return value on. No Horizon function known for EntityRenderDispatcher.submit: run `match`. |
| @Inject(INVOKE) on EntityRenderDispatcher.submit at Lnet/minecraft/client/renderer/entity/EntityRenderer;submit(Lnet/minecraft/client/renderer/entity/state/EntityRenderState;Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;Lnet/minecraft/client/renderer/state/level/CameraRenderState;)V | `MixinEntityRenderDispatcher.postSubmitTweaks` | AFTER_CALL | needs-mapping | No Horizon function known for EntityRenderDispatcher.submit: run `match`. The call it hooks (EntityRenderer.submit) isn't mapped to a Rust function yet. |
| @Inject(INVOKE) on EntityRenderDispatcher.submit at Lnet/minecraft/client/renderer/SubmitNodeCollector;submitShadow(Lcom/mojang/blaze3d/vertex/PoseStack;FLjava/util/List;)V | `MixinEntityRenderDispatcher.preShadow` | BEFORE_CALL | needs-mapping | Only whole statements, plain let initializers and block tails can be selected. No Horizon function known for EntityRenderDispatcher.submit: run `match`. The call it hooks (SubmitNodeCollector.submitShadow) isn't mapped to a Rust function yet. |
| @Inject(INVOKE) on EntityRenderDispatcher.submit at Lnet/minecraft/client/renderer/SubmitNodeCollector;submitShadow(Lcom/mojang/blaze3d/vertex/PoseStack;FLjava/util/List;)V | `MixinEntityRenderDispatcher.postShadow` | AFTER_CALL | needs-mapping | No Horizon function known for EntityRenderDispatcher.submit: run `match`. The call it hooks (SubmitNodeCollector.submitShadow) isn't mapped to a Rust function yet. |
| @Inject(HEAD) on EntityRenderers.lambda$createEntityRenderers$0 | `MixinEntityRenderers.emf$locateTransient` | HEAD | needs-mapping | No Horizon function known for EntityRenderers.lambda$createEntityRenderers$0: run `match`. |
| @Inject(TAIL) on EntityRenderers.lambda$createEntityRenderers$0 | `MixinEntityRenderers.emf$reset` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for EntityRenderers.lambda$createEntityRenderers$0: run `match`. |
| mixin into ItemFrameRenderer with no injectors (interfaces or @Shadow only) | `MixinItemFrameEntityRenderer` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| @ModifyExpressionValue(INVOKE) on EntityRenderer.extractRenderState at Lnet/minecraft/world/entity/Leashable;getLeashHolder()Lnet/minecraft/world/entity/Entity; | `MixinLeashOffsets.getLeashHolder` | MODIFY_EXPRESSION_VALUE | needs-mapping | Selects an exact Rust expression: pick it in the target function. No Horizon function known for EntityRenderer.extractRenderState: run `match`. |
| @ModifyExpressionValue(INVOKE) on EntityRenderer.extractRenderState at Lnet/minecraft/world/entity/Leashable;getLeashOffset(F)Lnet/minecraft/world/phys/Vec3; | `MixinLeashOffsets.emf$leash` | MODIFY_EXPRESSION_VALUE | needs-mapping | Selects an exact Rust expression: pick it in the target function. No Horizon function known for EntityRenderer.extractRenderState: run `match`. |
| @ModifyExpressionValue(INVOKE) on EntityRenderer.extractRenderState at Lnet/minecraft/world/entity/Entity;getRopeHoldPosition(F)Lnet/minecraft/world/phys/Vec3; | `MixinLeashOffsets.emf$leashHold` | MODIFY_EXPRESSION_VALUE | needs-mapping | Selects an exact Rust expression: pick it in the target function. No Horizon function known for EntityRenderer.extractRenderState: run `match`. |
| @ModifyExpressionValue(INVOKE) on EntityRenderer.extractRenderState at Lnet/minecraft/world/entity/Leashable;getQuadLeashOffsets()[Lnet/minecraft/world/phys/Vec3; | `MixinLeashOffsets.emf$leashQuad` | MODIFY_EXPRESSION_VALUE | needs-mapping | Selects an exact Rust expression: pick it in the target function. No Horizon function known for EntityRenderer.extractRenderState: run `match`. |
| @ModifyExpressionValue(INVOKE) on EntityRenderer.extractRenderState at Lnet/minecraft/world/entity/Entity;getQuadLeashHolderOffsets()[Lnet/minecraft/world/phys/Vec3; | `MixinLeashOffsets.emf$leashQuadHolder` | MODIFY_EXPRESSION_VALUE | needs-mapping | Selects an exact Rust expression: pick it in the target function. No Horizon function known for EntityRenderer.extractRenderState: run `match`. |
| @Inject(INVOKE) on LivingEntityRenderer.submit at Lnet/minecraft/client/model/EntityModel;setupAnim(Ljava/lang/Object;)V | `MixinLivingEntityRenderer.falseAnimation` | AFTER_CALL | needs-mapping | No Horizon function known for LivingEntityRenderer.submit: run `match`. The call it hooks (EntityModel.setupAnim) isn't mapped to a Rust function yet. |
| @ModifyExpressionValue(INVOKE) on LivingEntityRenderer.getRenderType at Lnet/minecraft/client/renderer/entity/LivingEntityRenderer;getTextureLocation(Lnet/minecraft/client/renderer/entity/state/LivingEntityRenderState;)Lnet/minecraft/resources/Identifier; | `MixinLivingEntityRenderer.emf$getTextureRedirect` | MODIFY_EXPRESSION_VALUE | needs-mapping | Selects an exact Rust expression: pick it in the target function. No Horizon function known for LivingEntityRenderer.getRenderType: run `match`. |
| @Inject(INVOKE) on LivingEntityRenderer.submit at Ljava/util/List;iterator()Ljava/util/Iterator; | `MixinLivingEntityRenderer.emf$grabEntity` | BEFORE_CALL | needs-mapping | Only whole statements, plain let initializers and block tails can be selected. No Horizon function known for LivingEntityRenderer.submit: run `match`. The call it hooks (List.iterator) isn't mapped to a Rust function yet. |
| @Inject(INVOKE) on LivingEntityRenderer.submit at Ljava/util/Iterator;next()Ljava/lang/Object; | `MixinLivingEntityRenderer.emf$eachFeatureLoop` | BEFORE_CALL | needs-mapping | Only whole statements, plain let initializers and block tails can be selected. No Horizon function known for LivingEntityRenderer.submit: run `match`. The call it hooks (Iterator.next) isn't mapped to a Rust function yet. |
| @Inject(HEAD) on LivingEntityRenderer.submit | `MixinLivingEntityRenderer.emf$preRender` | HEAD | needs-mapping | No Horizon function known for LivingEntityRenderer.submit: run `match`. |
| @Inject(TAIL) on LivingEntityRenderer.submit | `MixinLivingEntityRenderer.emf$postRender` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for LivingEntityRenderer.submit: run `match`. |
| @Inject(HEAD) on AvatarRenderer.renderHand | `MixinPlayerEntityRenderer.emf$setHandAnimState` | HEAD | needs-mapping | No Horizon function known for AvatarRenderer.renderHand: run `match`. |
| @Inject(INVOKE) on AvatarRenderer.renderHand at Lnet/minecraft/client/renderer/SubmitNodeCollector;submitModelPart(Lnet/minecraft/client/model/geom/ModelPart;Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/rendertype/RenderType;IILnet/minecraft/client/renderer/texture/UvMapping;)V | `MixinPlayerEntityRenderer.emf$setHandAnims` | BEFORE_CALL | needs-mapping | Only whole statements, plain let initializers and block tails can be selected. No Horizon function known for AvatarRenderer.renderHand: run `match`. The call it hooks (SubmitNodeCollector.submitModelPart) isn't mapped to a Rust function yet. |
| @Inject(RETURN) on AvatarRenderer.renderHand | `MixinPlayerEntityRenderer.emf$unsetHand` | RETURN | needs-mapping | Horizon's RETURN runs at every normal exit and passes the return value on. No Horizon function known for AvatarRenderer.renderHand: run `match`. |
| @Inject(RETURN) on LoadedBlockModels.bake | `Mixin_SpecialModelRenderers_SetCurrentSpecifiedModel.emf$clearMarker` | RETURN | needs-mapping | Horizon's RETURN runs at every normal exit and passes the return value on. No Horizon function known for LoadedBlockModels.bake: run `match`. |
| @Inject(HEAD) on LoadedBlockModels.lambda$bake$1 | `Mixin_SpecialModelRenderers_SetCurrentSpecifiedModel.setEmf$Model` | HEAD | needs-mapping | No Horizon function known for LoadedBlockModels.lambda$bake$1: run `match`. |
| mixin into FirstPersonHandsAndItemsRenderer with no injectors (interfaces or @Shadow only) | `Mixin_WrapFirstPersonHandRender` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| mixin into CancelTarget with no injectors (interfaces or @Shadow only) | `MixinArrowRenderer` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| mixin into CancelTarget with no injectors (interfaces or @Shadow only) | `MixinProjectileEntityRenderer` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| mixin into CancelTarget with no injectors (interfaces or @Shadow only) | `MixinSpectralArrowRenderer` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| @Inject(HEAD) on StuckInBodyLayer.submit | `MixinStuckArrowsFeatureRenderer.emf$start` | HEAD | needs-mapping | No Horizon function known for StuckInBodyLayer.submit: run `match`. |
| @Inject(RETURN) on StuckInBodyLayer.submit | `MixinStuckArrowsFeatureRenderer.emf$end` | RETURN | needs-mapping | Horizon's RETURN runs at every normal exit and passes the return value on. No Horizon function known for StuckInBodyLayer.submit: run `match`. |
| @Inject(INVOKE) on SulfurCubeInnerLayer.submit at Lcom/mojang/blaze3d/vertex/PoseStack;rotateDegrees(Lcom/mojang/math/Axis;F)V | `MixinSulfurCubeInnerLayer_BlockOffset.offsetBlock` | BEFORE_CALL | needs-mapping | Only whole statements, plain let initializers and block tails can be selected. No Horizon function known for SulfurCubeInnerLayer.submit: run `match`. |
| @Inject(INVOKE) on DolphinCarryingItemLayer.submit at Lcom/mojang/blaze3d/vertex/PoseStack;translate(FFF)V | `Mixin_Dolphin_BlockOffset.offsetBlock` | BEFORE_CALL | needs-mapping | Only whole statements, plain let initializers and block tails can be selected. No Horizon function known for DolphinCarryingItemLayer.submit: run `match`. The call it hooks (PoseStack.translate) isn't mapped to a Rust function yet. |
| @Inject(INVOKE) on CarriedBlockLayer.submit at Lcom/mojang/blaze3d/vertex/PoseStack;rotateDegrees(Lcom/mojang/math/Axis;F)V | `Mixin_Enderman_BlockOffset.offsetBlock` | BEFORE_CALL | needs-mapping | Only whole statements, plain let initializers and block tails can be selected. No Horizon function known for CarriedBlockLayer.submit: run `match`. |
| @WrapWithCondition(INVOKE) on FoxHeldItemLayer.submit at Lcom/mojang/blaze3d/vertex/PoseStack;translate(FFF)V | `Mixin_Fox_BlockOffset.offsetBlock` | WRAP_OPERATION | needs-mapping | Call `original` only when the condition holds. No Horizon function known for FoxHeldItemLayer.submit: run `match`. The call it hooks (PoseStack.translate) isn't mapped to a Rust function yet. |
| @WrapWithCondition(INVOKE) on FoxHeldItemLayer.submit at Lcom/mojang/blaze3d/vertex/PoseStack;rotate(Lcom/mojang/math/Axis;F)V | `Mixin_Fox_BlockOffset.offsetBlock3` | WRAP_OPERATION | needs-mapping | Call `original` only when the condition holds. No Horizon function known for FoxHeldItemLayer.submit: run `match`. The call it hooks (PoseStack.rotate) isn't mapped to a Rust function yet. |
| @WrapWithCondition(INVOKE) on FoxHeldItemLayer.submit at Lcom/mojang/blaze3d/vertex/PoseStack;rotateDegrees(Lcom/mojang/math/Axis;F)V | `Mixin_Fox_BlockOffset.offsetBlock4` | WRAP_OPERATION | needs-mapping | Call `original` only when the condition holds. No Horizon function known for FoxHeldItemLayer.submit: run `match`. |
| @WrapWithCondition(INVOKE) on FoxHeldItemLayer.submit at Lcom/mojang/blaze3d/vertex/PoseStack;rotateDegrees(Lcom/mojang/math/Axis;F)V | `Mixin_Fox_BlockOffset.offsetBlock5` | WRAP_OPERATION | needs-mapping | Call `original` only when the condition holds. No Horizon function known for FoxHeldItemLayer.submit: run `match`. |
| @WrapWithCondition(INVOKE) on CustomHeadLayer.submit at Lnet/minecraft/client/model/geom/ModelPart;translateAndRotate(Lcom/mojang/blaze3d/vertex/PoseStack;)V | `Mixin_Head_BlockOffset.offsetBlock` | WRAP_OPERATION | needs-mapping | Call `original` only when the condition holds. No Horizon function known for CustomHeadLayer.submit: run `match`. The call it hooks (ModelPart.translateAndRotate) isn't mapped to a Rust function yet. |
| @WrapWithCondition(INVOKE) on CustomHeadLayer.submit at Lnet/minecraft/client/model/HeadedModel;translateToHead(Lcom/mojang/blaze3d/vertex/PoseStack;)V | `Mixin_Head_BlockOffset.cancel` | WRAP_OPERATION | needs-mapping | Call `original` only when the condition holds. No Horizon function known for CustomHeadLayer.submit: run `match`. The call it hooks (HeadedModel.translateToHead) isn't mapped to a Rust function yet. |
| @Inject(INVOKE) on PandaHoldsItemLayer.submit at Lcom/mojang/blaze3d/vertex/PoseStack;translate(FFF)V | `Mixin_Panda_BlockOffset.offsetBlock` | BEFORE_CALL | needs-mapping | Only whole statements, plain let initializers and block tails can be selected. No Horizon function known for PandaHoldsItemLayer.submit: run `match`. The call it hooks (PoseStack.translate) isn't mapped to a Rust function yet. |
| @WrapWithCondition(INVOKE) on ParrotOnShoulderLayer.submitOnShoulder at Lcom/mojang/blaze3d/vertex/PoseStack;translate(FFF)V | `Mixin_Parrot_ShoulderOffset.offsetBlock` | WRAP_OPERATION | needs-mapping | Call `original` only when the condition holds. No Horizon function known for ParrotOnShoulderLayer.submitOnShoulder: run `match`. The call it hooks (PoseStack.translate) isn't mapped to a Rust function yet. |
| @WrapWithCondition(INVOKE) on CrossedArmsItemLayer.applyTranslation at Lnet/minecraft/client/model/VillagerLikeModel;translateToArms(Lnet/minecraft/client/renderer/entity/state/EntityRenderState;Lcom/mojang/blaze3d/vertex/PoseStack;)V | `Mixin_Villager_BlockOffset.offsetBlock` | WRAP_OPERATION | needs-mapping | Call `original` only when the condition holds. No Horizon function known for CrossedArmsItemLayer.applyTranslation: run `match`. The call it hooks (VillagerLikeModel.translateToArms) isn't mapped to a Rust function yet. |
| @WrapWithCondition(INVOKE) on WitchItemLayer.applyTranslation at Lnet/minecraft/client/model/geom/ModelPart;translateAndRotate(Lcom/mojang/blaze3d/vertex/PoseStack;)V | `Mixin_Witch_BlockOffset.offsetBlock` | WRAP_OPERATION | needs-mapping | Call `original` only when the condition holds. No Horizon function known for WitchItemLayer.applyTranslation: run `match`. The call it hooks (ModelPart.translateAndRotate) isn't mapped to a Rust function yet. |
| @WrapWithCondition(INVOKE) on WitchItemLayer.applyTranslation at Lnet/minecraft/client/model/geom/ModelPart;translateAndRotate(Lcom/mojang/blaze3d/vertex/PoseStack;)V | `Mixin_Witch_BlockOffset.cancel` | WRAP_OPERATION | needs-mapping | Call `original` only when the condition holds. No Horizon function known for WitchItemLayer.applyTranslation: run `match`. The call it hooks (ModelPart.translateAndRotate) isn't mapped to a Rust function yet. |
| @WrapWithCondition(INVOKE) on WitchItemLayer.applyTranslation at Lnet/minecraft/client/model/monster/witch/WitchModel;translateToHead(Lcom/mojang/blaze3d/vertex/PoseStack;)V | `Mixin_Witch_BlockOffset.cancel2` | WRAP_OPERATION | needs-mapping | Call `original` only when the condition holds. No Horizon function known for WitchItemLayer.applyTranslation: run `match`. The call it hooks (WitchModel.translateToHead) isn't mapped to a Rust function yet. |
| @Inject(RETURN) on CreeperPowerLayer.getTextureLocation | `MixinCreeperChargeFeatureRenderer.emf$getTextureRedirect` | RETURN | needs-mapping | Horizon's RETURN runs at every normal exit and passes the return value on. No Horizon function known for CreeperPowerLayer.getTextureLocation: run `match`. |
| @Inject(HEAD) on CustomHeadLayer.submit | `MixinHeadFeatureRenderer.emf$setHand` | HEAD | needs-mapping | No Horizon function known for CustomHeadLayer.submit: run `match`. |
| @Inject(TAIL) on CustomHeadLayer.submit | `MixinHeadFeatureRenderer.emf$unsetHand` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for CustomHeadLayer.submit: run `match`. |
| @Inject(HEAD) on ItemInHandLayer.submitArmWithItem | `MixinHeldItemFeatureRenderer.emf$setHand` | HEAD | needs-mapping | No Horizon function known for ItemInHandLayer.submitArmWithItem: run `match`. |
| @Inject(INVOKE) on ItemInHandLayer.submitArmWithItem at Lnet/minecraft/client/model/ArmedModel;translateToHand(Lnet/minecraft/client/renderer/entity/state/EntityRenderState;Lnet/minecraft/world/entity/HumanoidArm;Lcom/mojang/blaze3d/vertex/PoseStack;)V | `MixinHeldItemFeatureRenderer.emf$transform` | BEFORE_CALL | needs-mapping | Only whole statements, plain let initializers and block tails can be selected. No Horizon function known for ItemInHandLayer.submitArmWithItem: run `match`. The call it hooks (ArmedModel.translateToHand) isn't mapped to a Rust function yet. |
| @Inject(INVOKE) on ItemInHandLayer.submitArmWithItem at Lnet/minecraft/client/model/ArmedModel;translateToHand(Lnet/minecraft/client/renderer/entity/state/EntityRenderState;Lnet/minecraft/world/entity/HumanoidArm;Lcom/mojang/blaze3d/vertex/PoseStack;)V | `MixinHeldItemFeatureRenderer.emf$transform2` | AFTER_CALL | needs-mapping | No Horizon function known for ItemInHandLayer.submitArmWithItem: run `match`. The call it hooks (ArmedModel.translateToHand) isn't mapped to a Rust function yet. |
| @Inject(TAIL) on ItemInHandLayer.submitArmWithItem | `MixinHeldItemFeatureRenderer.emf$unsetHand` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for ItemInHandLayer.submitArmWithItem: run `match`. |
| mixin into CancelTarget with no injectors (interfaces or @Shadow only) | `MixinHumanoidArmorLayer` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| @Inject(HEAD) on SlimeOuterLayer.submit | `MixinSlimeOverlayFeatureRenderer.emf$setLayerForOverrides` | HEAD | needs-mapping | No Horizon function known for SlimeOuterLayer.submit: run `match`. |
| @Inject(TAIL) on WolfCollarLayer.<init> | `MixinWolfCollarFeatureRenderer.setEmf$Model` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for WolfCollarLayer.<init>: run `match`. |
| @Inject(TAIL) on ArmadilloModel.setupAnim | `MixinArmadilloModel.emf$assertLayerFactory` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for ArmadilloModel.setupAnim: run `match`. |
| mixin into CancelTarget with no injectors (interfaces or @Shadow only) | `MixinDragonModel` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| @Inject(HEAD) on VillagerModel.setupAnim | `MixinVillagerModel.emf$assertLayerFactory` | HEAD | needs-mapping | No Horizon function known for VillagerModel.setupAnim: run `match`. |
| @Inject(HEAD) on WolfModel.setupAnim | `MixinWolfEntityModel.smf$setAngles` | HEAD | needs-mapping | No Horizon function known for WolfModel.setupAnim: run `match`. |
| @Inject(TAIL) on ChickenRenderer.<init> | `Mixin_ChickenRenderer_WarmModel.emf$createWarmModels` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for ChickenRenderer.<init>: run `match`. |
| @Inject(INVOKE) on ChickenRenderer.submit at Lnet/minecraft/client/renderer/entity/MobRenderer;submit(Lnet/minecraft/client/renderer/entity/state/LivingEntityRenderState;Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;Lnet/minecraft/client/renderer/state/level/CameraRenderState;)V | `Mixin_ChickenRenderer_WarmModel.emf$warmModel` | BEFORE_CALL | needs-mapping | Only whole statements, plain let initializers and block tails can be selected. No Horizon function known for ChickenRenderer.submit: run `match`. The call it hooks (MobRenderer.submit) isn't mapped to a Rust function yet. |
| @Inject(TAIL) on DragonFireballRenderer.<init> | `Mixin_EnderDragonFireball_Model.emf$createModel` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for DragonFireballRenderer.<init>: run `match`. |
| @Inject(HEAD) on DragonFireballRenderer.submit | `Mixin_EnderDragonFireball_Model.emf$renderModel` | HEAD | needs-mapping | No Horizon function known for DragonFireballRenderer.submit: run `match`. |
| @Inject(TAIL) on PigRenderer.<init> | `Mixin_PigRenderer_WarmModel.emf$createWarmModels` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for PigRenderer.<init>: run `match`. |
| @Inject(INVOKE) on PigRenderer.submit at Lnet/minecraft/client/renderer/entity/MobRenderer;submit(Lnet/minecraft/client/renderer/entity/state/LivingEntityRenderState;Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;Lnet/minecraft/client/renderer/state/level/CameraRenderState;)V | `Mixin_PigRenderer_WarmModel.emf$warmModel` | BEFORE_CALL | needs-mapping | Only whole statements, plain let initializers and block tails can be selected. No Horizon function known for PigRenderer.submit: run `match`. The call it hooks (MobRenderer.submit) isn't mapped to a Rust function yet. |
| @Inject(TAIL) on PlayerModel.setupAnim | `Mixin_PlayerModel_ReApplyPoseToExtraCallsMadeByMods.emf$initRender` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for PlayerModel.setupAnim: run `match`. |
| @ModifyExpressionValue(FIELD) on FlameFeatureRenderer.prepare at Lnet/minecraft/client/renderer/entity/state/EntityRenderState;boundingBoxWidth:F | `MixinFlameFeatureRenderer.width` | MODIFY_EXPRESSION_VALUE | needs-mapping | Selects an exact Rust expression: pick it in the target function. No Horizon function known for FlameFeatureRenderer.prepare: run `match`. |
| @ModifyExpressionValue(FIELD) on FlameFeatureRenderer.prepare at Lnet/minecraft/client/renderer/entity/state/EntityRenderState;boundingBoxHeight:F | `MixinFlameFeatureRenderer.height` | MODIFY_EXPRESSION_VALUE | needs-mapping | Selects an exact Rust expression: pick it in the target function. No Horizon function known for FlameFeatureRenderer.prepare: run `match`. |
| @Inject(HEAD) on HumanoidArmorLayer.submit | `Mixin_HumanoidArmorLayer_AddBaseModelPoseRef.emf$initRender` | HEAD | needs-mapping | No Horizon function known for HumanoidArmorLayer.submit: run `match`. |
| @Inject(TAIL) on HumanoidArmorLayer.submit | `Mixin_HumanoidArmorLayer_AddBaseModelPoseRef.emf$endRender` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for HumanoidArmorLayer.submit: run `match`. |
| mixin into CancelTarget with no injectors (interfaces or @Shadow only) | `Mixin_ModelPartRenderer` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| @Inject(INVOKE) on ModelFeatureRenderer.prepareModel at Lnet/minecraft/client/model/Model;setupAnim(Ljava/lang/Object;)V | `Mixin_ModelRenderer.emf$animate` | AFTER_CALL | needs-mapping | No Horizon function known for ModelFeatureRenderer.prepareModel: run `match`. The call it hooks (Model.setupAnim) isn't mapped to a Rust function yet. |

## Classes extending the game

| Class | In | Status | Note |
|---|---|---|---|
| extends EntityModel | `Mixin_EnderDragonFireball_Model$1` | needs-translation | No mapping for this superclass yet. |
| extends ModelPart | `EMFModelPart` | needs-translation | No mapping for this superclass yet. |
| extends Cube | `EMFModelPartCustom$EMFCube` | needs-translation | No mapping for this superclass yet. |

## Access wideners

| File | In | Status | Note |
|---|---|---|---|
| access widener (opens private game fields) | `entity_model_features_16.accesswidener` | needs-translation | Native helpers already see the game's fields, once each field is mapped. |

## Content

| What | In | Status | Note |
|---|---|---|---|
| 15 asset files, 0 data files | `jar resources` | content | Recipes, loot, tags, models, textures, worldgen: horizon-port-pack carries these. |

## Game methods and fields used

20 of 451 are mapped to Horizon. Every Java body that uses an unmapped one needs it mapped before its Rust can be written.

<details><summary>Mapped</summary>

| Java | Horizon |
|---|---|
| `com/mojang/blaze3d/vertex/PoseStack#rotateDegrees` | horizon_client_render::camera::rotate_degrees (suggested) |
| `com/mojang/blaze3d/vertex/PoseStack$Pose#transformNormal` | horizon_client::item_render::transform_normal (suggested) |
| `net/minecraft/client/Minecraft#getInstance` | `self` |
| `net/minecraft/client/Minecraft#player` | `self.native_player()` |
| `net/minecraft/client/renderer/GameRenderer#render` | hook `frame` |
| `net/minecraft/core/registries/BuiltInRegistries#BLOCK` | content |
| `net/minecraft/network/chat/MutableComponent#withStyle` | horizon_client_core::text::Text::with_style (suggested) |
| `net/minecraft/util/Mth#atan2` | horizon_core::geometry::trig::atan2 (suggested) |
| `net/minecraft/util/Mth#ceil` | horizon_client_render::hud_layout::ceil (suggested) |
| `net/minecraft/util/Mth#clamp` | horizon_client_render::chat::clamp (suggested) |
| `net/minecraft/util/Mth#floor` | horizon_client_render::gfx::occlusion::floor (suggested) |
| `net/minecraft/util/Mth#frac` | horizon_client_render::border_render::frac (suggested) |
| `net/minecraft/util/Mth#lerp` | horizon_world::stepped_interpolation::lerp (suggested) |
| `net/minecraft/util/Mth#rotLerp` | horizon_world::stepped_interpolation::rot_lerp (suggested) |
| `net/minecraft/util/Mth#wrapDegrees` | horizon_world::stepped_interpolation::wrap_degrees (suggested) |
| `net/minecraft/world/entity/Entity#getYRot` | `self.native_player()?.yaw` |
| `net/minecraft/world/entity/LivingEntity#onClimbable` | horizon_client::mobs::ai::on_climbable (suggested) |
| `net/minecraft/world/level/Level#getMaxY` | horizon_client::game::Dim::max_y (suggested) |
| `net/minecraft/world/level/Level#getMinY` | horizon_client::game::Dim::min_y (suggested) |
| `net/minecraft/world/level/Level#isClientSide` | horizon_world::level::is_client_side (suggested) |

</details>

<details><summary>Not mapped yet</summary>

| Java | Kind | Classes using it |
|---|---|---|
| `com/mojang/blaze3d/vertex/BufferBuilder#building` | field | 1 |
| `com/mojang/blaze3d/vertex/PoseStack#<init>` | method | 1 |
| `com/mojang/blaze3d/vertex/PoseStack#last` | method | 3 |
| `com/mojang/blaze3d/vertex/PoseStack#popPose` | method | 3 |
| `com/mojang/blaze3d/vertex/PoseStack#pushPose` | method | 3 |
| `com/mojang/blaze3d/vertex/PoseStack#rotate` | method | 1 |
| `com/mojang/blaze3d/vertex/PoseStack#translate` | method | 11 |
| `com/mojang/blaze3d/vertex/PoseStack$Pose#pose` | method | 2 |
| `com/mojang/blaze3d/vertex/PoseStack$Pose#translate` | method | 1 |
| `com/mojang/blaze3d/vertex/VertexConsumer#addVertex` | method | 2 |
| `com/mojang/blaze3d/vertex/VertexConsumer#setColor` | method | 1 |
| `com/mojang/blaze3d/vertex/VertexConsumer#setLineWidth` | method | 1 |
| `com/mojang/blaze3d/vertex/VertexConsumer#setNormal` | method | 1 |
| `java/util/Iterator#next` | method | 1 |
| `java/util/List#iterator` | method | 1 |
| `java/util/Map#forEach` | method | 1 |
| `net/minecraft/ChatFormatting#YELLOW` | field | 1 |
| `net/minecraft/client/CameraType#FIRST_PERSON` | field | 1 |
| `net/minecraft/client/CameraType#THIRD_PERSON_BACK` | field | 1 |
| `net/minecraft/client/CameraType#THIRD_PERSON_FRONT` | field | 1 |
| `net/minecraft/client/CameraType#isFirstPerson` | method | 1 |
| `net/minecraft/client/DeltaTracker$Timer#getGameTimeDeltaPartialTick` | method | 1 |
| `net/minecraft/client/DeltaTracker$Timer#getGameTimeDeltaTicks` | method | 1 |
| `net/minecraft/client/Minecraft#crosshairPickEntity` | field | 1 |
| `net/minecraft/client/Minecraft#deltaTracker` | method | 1 |
| `net/minecraft/client/Minecraft#gameRenderer` | field | 2 |
| `net/minecraft/client/Minecraft#getCameraEntity` | method | 1 |
| `net/minecraft/client/Minecraft#getEntityModels` | method | 3 |
| `net/minecraft/client/Minecraft#getEntityRenderDispatcher` | method | 1 |
| `net/minecraft/client/Minecraft#getFps` | method | 1 |
| `net/minecraft/client/Minecraft#getResourceManager` | method | 5 |
| `net/minecraft/client/Minecraft#gui` | field | 1 |
| `net/minecraft/client/Minecraft#isPaused` | method | 5 |
| `net/minecraft/client/Minecraft#level` | field | 1 |
| `net/minecraft/client/Minecraft#onResourceLoadFinished` | method | 1 |
| `net/minecraft/client/Minecraft#options` | field | 2 |
| `net/minecraft/client/Minecraft#reloadResourcePacks` | method | 1 |
| `net/minecraft/client/Options#getCameraType` | method | 2 |
| `net/minecraft/client/gui/Gui#toastManager` | method | 1 |
| `net/minecraft/client/gui/components/Button#active` | field | 1 |
| `net/minecraft/client/gui/components/Button#setMessage` | method | 1 |
| `net/minecraft/client/gui/components/toasts/SystemToast#add` | method | 1 |
| `net/minecraft/client/gui/components/toasts/SystemToast$SystemToastId#PERIODIC_NOTIFICATION` | field | 1 |
| `net/minecraft/client/gui/render/GuiRenderer#render` | method | 2 |
| `net/minecraft/client/model/AdultAndBabyModelPair#<init>` | method | 2 |
| `net/minecraft/client/model/AdultAndBabyModelPair#getModel` | method | 2 |
| `net/minecraft/client/model/ArmedModel#translateToHand` | method | 2 |
| `net/minecraft/client/model/BabyModelTransform#apply` | method | 1 |
| `net/minecraft/client/model/EntityModel#<init>` | method | 2 |
| `net/minecraft/client/model/EntityModel#setupAnim` | method | 1 |
| `net/minecraft/client/model/HeadedModel#translateToHead` | method | 1 |
| `net/minecraft/client/model/HumanoidModel#body` | field | 1 |
| `net/minecraft/client/model/HumanoidModel#head` | field | 1 |
| `net/minecraft/client/model/HumanoidModel#leftArm` | field | 1 |
| `net/minecraft/client/model/HumanoidModel#leftLeg` | field | 1 |
| `net/minecraft/client/model/HumanoidModel#rightArm` | field | 1 |
| `net/minecraft/client/model/HumanoidModel#rightLeg` | field | 1 |
| `net/minecraft/client/model/HumanoidModel#root` | method | 1 |
| `net/minecraft/client/model/Model#<init>` | method | 1 |
| `net/minecraft/client/model/Model#renderType` | method | 3 |
| `net/minecraft/client/model/Model#root` | method | 3 |
| `net/minecraft/client/model/Model#setupAnim` | method | 1 |
| `net/minecraft/client/model/VillagerLikeModel#translateToArms` | method | 1 |
| `net/minecraft/client/model/animal/armadillo/ArmadilloModel#setupAnim` | method | 1 |
| `net/minecraft/client/model/animal/chicken/AdultChickenModel#<init>` | method | 1 |
| `net/minecraft/client/model/animal/chicken/AdultChickenModel#createBodyLayer` | method | 1 |
| `net/minecraft/client/model/animal/chicken/BabyChickenModel#<init>` | method | 1 |
| `net/minecraft/client/model/animal/chicken/BabyChickenModel#createBodyLayer` | method | 1 |
| `net/minecraft/client/model/animal/parrot/ParrotModel#createBodyLayer` | method | 1 |
| `net/minecraft/client/model/animal/pig/BabyPigModel#<init>` | method | 1 |
| `net/minecraft/client/model/animal/pig/BabyPigModel#createBodyLayer` | method | 1 |
| `net/minecraft/client/model/animal/pig/PigModel#<init>` | method | 1 |
| `net/minecraft/client/model/animal/pig/PigModel#createBodyLayer` | method | 1 |
| `net/minecraft/client/model/animal/wolf/AdultWolfModel#<init>` | method | 1 |
| `net/minecraft/client/model/animal/wolf/AdultWolfModel#createBodyLayer` | method | 1 |
| `net/minecraft/client/model/animal/wolf/BabyWolfModel#<init>` | method | 1 |
| `net/minecraft/client/model/animal/wolf/BabyWolfModel#createBodyLayer` | method | 1 |
| `net/minecraft/client/model/animal/wolf/WolfModel#setupAnim` | method | 2 |
| `net/minecraft/client/model/geom/EntityModelSet#bakeLayer` | method | 1 |
| `net/minecraft/client/model/geom/EntityModelSet#roots` | field | 2 |
| `net/minecraft/client/model/geom/EntityModelSet#vanilla` | method | 1 |
| `net/minecraft/client/model/geom/LayerDefinitions#createRoots` | method | 1 |
| `net/minecraft/client/model/geom/ModelLayerLocation#<init>` | method | 5 |
| `net/minecraft/client/model/geom/ModelLayerLocation#layer` | method | 1 |
| `net/minecraft/client/model/geom/ModelLayerLocation#model` | method | 1 |
| `net/minecraft/client/model/geom/ModelLayerLocation#toString` | method | 1 |
| `net/minecraft/client/model/geom/ModelLayers#WOLF` | field | 1 |
| `net/minecraft/client/model/geom/ModelPart#<init>` | method | 3 |
| `net/minecraft/client/model/geom/ModelPart#children` | field | 6 |
| `net/minecraft/client/model/geom/ModelPart#cubes` | field | 5 |
| `net/minecraft/client/model/geom/ModelPart#getChild` | method | 2 |
| `net/minecraft/client/model/geom/ModelPart#getInitialPose` | method | 2 |
| `net/minecraft/client/model/geom/ModelPart#getRandomCube` | method | 1 |
| `net/minecraft/client/model/geom/ModelPart#hasChild` | method | 1 |
| `net/minecraft/client/model/geom/ModelPart#render` | method | 2 |
| `net/minecraft/client/model/geom/ModelPart#setInitialPose` | method | 1 |
| `net/minecraft/client/model/geom/ModelPart#skipDraw` | field | 6 |
| `net/minecraft/client/model/geom/ModelPart#translateAndRotate` | method | 5 |
| `net/minecraft/client/model/geom/ModelPart#visible` | field | 6 |
| `net/minecraft/client/model/geom/ModelPart#x` | field | 4 |
| `net/minecraft/client/model/geom/ModelPart#xRot` | field | 4 |
| `net/minecraft/client/model/geom/ModelPart#xScale` | field | 4 |
| `net/minecraft/client/model/geom/ModelPart#y` | field | 4 |
| `net/minecraft/client/model/geom/ModelPart#yRot` | field | 4 |
| `net/minecraft/client/model/geom/ModelPart#yScale` | field | 4 |
| `net/minecraft/client/model/geom/ModelPart#z` | field | 4 |
| `net/minecraft/client/model/geom/ModelPart#zRot` | field | 4 |
| `net/minecraft/client/model/geom/ModelPart#zScale` | field | 4 |
| `net/minecraft/client/model/geom/ModelPart$Cube#<init>` | method | 3 |
| `net/minecraft/client/model/geom/ModelPart$Cube#compile` | method | 1 |
| `net/minecraft/client/model/geom/ModelPart$Cube#maxX` | field | 5 |
| `net/minecraft/client/model/geom/ModelPart$Cube#maxY` | field | 5 |
| `net/minecraft/client/model/geom/ModelPart$Cube#maxZ` | field | 5 |
| `net/minecraft/client/model/geom/ModelPart$Cube#minX` | field | 5 |
| `net/minecraft/client/model/geom/ModelPart$Cube#minY` | field | 5 |
| `net/minecraft/client/model/geom/ModelPart$Cube#minZ` | field | 5 |
| `net/minecraft/client/model/geom/ModelPart$Cube#polygons` | method | 1 |
| `net/minecraft/client/model/geom/ModelPart$Polygon#<init>` | method | 1 |
| `net/minecraft/client/model/geom/ModelPart$Polygon#normal` | method | 1 |
| `net/minecraft/client/model/geom/ModelPart$Polygon#vertices` | method | 1 |
| `net/minecraft/client/model/geom/ModelPart$Vertex#<init>` | method | 1 |
| `net/minecraft/client/model/geom/ModelPart$Vertex#u` | method | 1 |
| `net/minecraft/client/model/geom/ModelPart$Vertex#v` | method | 1 |
| `net/minecraft/client/model/geom/ModelPart$Vertex#x` | method | 1 |
| `net/minecraft/client/model/geom/ModelPart$Vertex#y` | method | 1 |
| `net/minecraft/client/model/geom/ModelPart$Vertex#z` | method | 1 |
| `net/minecraft/client/model/geom/PartPose#<init>` | method | 1 |
| `net/minecraft/client/model/geom/PartPose#x` | method | 1 |
| `net/minecraft/client/model/geom/PartPose#xRot` | method | 2 |
| `net/minecraft/client/model/geom/PartPose#xScale` | method | 1 |
| `net/minecraft/client/model/geom/PartPose#y` | method | 1 |
| `net/minecraft/client/model/geom/PartPose#yRot` | method | 2 |
| `net/minecraft/client/model/geom/PartPose#yScale` | method | 1 |
| `net/minecraft/client/model/geom/PartPose#z` | method | 1 |
| `net/minecraft/client/model/geom/PartPose#zRot` | method | 2 |
| `net/minecraft/client/model/geom/PartPose#zScale` | method | 1 |
| `net/minecraft/client/model/geom/builders/CubeDeformation#NONE` | field | 2 |
| `net/minecraft/client/model/geom/builders/LayerDefinition#bakeRoot` | method | 7 |
| `net/minecraft/client/model/geom/builders/MeshDefinition#<init>` | method | 1 |
| `net/minecraft/client/model/geom/builders/MeshDefinition#getRoot` | method | 2 |
| `net/minecraft/client/model/geom/builders/MeshTransformer#lambda$scaling$0` | method | 1 |
| `net/minecraft/client/model/geom/builders/PartDefinition#bake` | method | 3 |
| `net/minecraft/client/model/monster/witch/WitchModel#translateToHead` | method | 1 |
| `net/minecraft/client/model/npc/VillagerModel#setupAnim` | method | 1 |
| `net/minecraft/client/model/player/PlayerModel#setupAnim` | method | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#tickRateManager` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#blockInteractionRange` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#blockPosition` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#getX` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#getXRot` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#getY` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#getYRot` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#getZ` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#isScoping` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#sendSystemMessage` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#xRotO` | field | 1 |
| `net/minecraft/client/player/LocalPlayer#xo` | field | 1 |
| `net/minecraft/client/player/LocalPlayer#yRotO` | field | 1 |
| `net/minecraft/client/player/LocalPlayer#yo` | field | 1 |
| `net/minecraft/client/player/LocalPlayer#zo` | field | 1 |
| `net/minecraft/client/renderer/GameRenderer#gameRenderState` | method | 1 |
| `net/minecraft/client/renderer/GameRenderer#renderBuffers` | method | 1 |
| `net/minecraft/client/renderer/RenderBuffers#stagedVertexBuffer` | method | 1 |
| `net/minecraft/client/renderer/SpriteCoordinateExpander#delegate` | field | 1 |
| `net/minecraft/client/renderer/StagedVertexBuffer#appendDraw` | method | 1 |
| `net/minecraft/client/renderer/StagedVertexBuffer#getVertexBuilder` | method | 1 |
| `net/minecraft/client/renderer/SubmitNodeCollector#submitModel` | method | 2 |
| `net/minecraft/client/renderer/SubmitNodeCollector#submitModelPart` | method | 1 |
| `net/minecraft/client/renderer/SubmitNodeCollector#submitShadow` | method | 2 |
| `net/minecraft/client/renderer/block/LoadedBlockModels#bake` | method | 1 |
| `net/minecraft/client/renderer/block/LoadedBlockModels#lambda$bake$1` | method | 1 |
| `net/minecraft/client/renderer/blockentity/BannerRenderer#submitPatterns` | method | 1 |
| `net/minecraft/client/renderer/blockentity/BlockEntityRenderers#PROVIDERS` | field | 1 |
| `net/minecraft/client/renderer/blockentity/BlockEntityRenderers#createEntityRenderers` | method | 2 |
| `net/minecraft/client/renderer/entity/AgeableMobRenderer#babyModel` | method | 1 |
| `net/minecraft/client/renderer/entity/ChickenRenderer#<init>` | method | 1 |
| `net/minecraft/client/renderer/entity/ChickenRenderer#submit` | method | 1 |
| `net/minecraft/client/renderer/entity/DragonFireballRenderer#<init>` | method | 1 |
| `net/minecraft/client/renderer/entity/DragonFireballRenderer#submit` | method | 1 |
| `net/minecraft/client/renderer/entity/EnderDragonRenderer#submit` | method | 1 |
| `net/minecraft/client/renderer/entity/EntityRenderDispatcher#getRenderer` | method | 1 |
| `net/minecraft/client/renderer/entity/EntityRenderDispatcher#submit` | method | 4 |
| `net/minecraft/client/renderer/entity/EntityRenderer#<init>` | method | 1 |
| `net/minecraft/client/renderer/entity/EntityRenderer#createRenderState` | method | 1 |
| `net/minecraft/client/renderer/entity/EntityRenderer#extractRenderState` | method | 6 |
| `net/minecraft/client/renderer/entity/EntityRenderer#extractShadow` | method | 3 |
| `net/minecraft/client/renderer/entity/EntityRenderer#submit` | method | 1 |
| `net/minecraft/client/renderer/entity/EntityRenderers#lambda$createEntityRenderers$0` | method | 2 |
| `net/minecraft/client/renderer/entity/LivingEntityRenderer#<init>` | method | 1 |
| `net/minecraft/client/renderer/entity/LivingEntityRenderer#getRenderType` | method | 1 |
| `net/minecraft/client/renderer/entity/LivingEntityRenderer#isUpsideDownName` | method | 1 |
| `net/minecraft/client/renderer/entity/LivingEntityRenderer#submit` | method | 5 |
| `net/minecraft/client/renderer/entity/MobRenderer#<init>` | method | 2 |
| `net/minecraft/client/renderer/entity/MobRenderer#submit` | method | 2 |
| `net/minecraft/client/renderer/entity/PigRenderer#<init>` | method | 1 |
| `net/minecraft/client/renderer/entity/PigRenderer#submit` | method | 1 |
| `net/minecraft/client/renderer/entity/RenderLayerParent#getModel` | method | 1 |
| `net/minecraft/client/renderer/entity/layers/CarriedBlockLayer#submit` | method | 1 |
| `net/minecraft/client/renderer/entity/layers/CreeperPowerLayer#getTextureLocation` | method | 1 |
| `net/minecraft/client/renderer/entity/layers/CrossedArmsItemLayer#<init>` | method | 1 |
| `net/minecraft/client/renderer/entity/layers/CrossedArmsItemLayer#applyTranslation` | method | 1 |
| `net/minecraft/client/renderer/entity/layers/CustomHeadLayer#submit` | method | 4 |
| `net/minecraft/client/renderer/entity/layers/DolphinCarryingItemLayer#submit` | method | 1 |
| `net/minecraft/client/renderer/entity/layers/FoxHeldItemLayer#submit` | method | 4 |
| `net/minecraft/client/renderer/entity/layers/HumanoidArmorLayer#submit` | method | 2 |
| `net/minecraft/client/renderer/entity/layers/ItemInHandLayer#submitArmWithItem` | method | 4 |
| `net/minecraft/client/renderer/entity/layers/PandaHoldsItemLayer#submit` | method | 1 |
| `net/minecraft/client/renderer/entity/layers/ParrotOnShoulderLayer#<init>` | method | 1 |
| `net/minecraft/client/renderer/entity/layers/ParrotOnShoulderLayer#submitOnShoulder` | method | 3 |
| `net/minecraft/client/renderer/entity/layers/RenderLayer#<init>` | method | 9 |
| `net/minecraft/client/renderer/entity/layers/RenderLayer#getParentModel` | method | 1 |
| `net/minecraft/client/renderer/entity/layers/SlimeOuterLayer#submit` | method | 1 |
| `net/minecraft/client/renderer/entity/layers/StuckInBodyLayer#submit` | method | 2 |
| `net/minecraft/client/renderer/entity/layers/StuckInBodyLayer$PlacementStyle#ON_SURFACE` | field | 1 |
| `net/minecraft/client/renderer/entity/layers/SulfurCubeInnerLayer#submit` | method | 1 |
| `net/minecraft/client/renderer/entity/layers/WitchItemLayer#applyTranslation` | method | 3 |
| `net/minecraft/client/renderer/entity/layers/WolfCollarLayer#<init>` | method | 1 |
| `net/minecraft/client/renderer/entity/player/AvatarRenderer#renderHand` | method | 3 |
| `net/minecraft/client/renderer/entity/state/AvatarRenderState#id` | field | 1 |
| `net/minecraft/client/renderer/entity/state/AvatarRenderState#outlineColor` | field | 1 |
| `net/minecraft/client/renderer/entity/state/EntityRenderState#lightCoords` | field | 1 |
| `net/minecraft/client/renderer/entity/state/EntityRenderState#x` | field | 1 |
| `net/minecraft/client/renderer/entity/state/EntityRenderState#z` | field | 1 |
| `net/minecraft/client/renderer/entity/state/LivingEntityRenderState#walkAnimationPos` | field | 1 |
| `net/minecraft/client/renderer/entity/state/LivingEntityRenderState#walkAnimationSpeed` | field | 1 |
| `net/minecraft/client/renderer/entity/state/LivingEntityRenderState#xRot` | field | 1 |
| `net/minecraft/client/renderer/entity/state/LivingEntityRenderState#yRot` | field | 1 |
| `net/minecraft/client/renderer/entity/state/SulfurCubeRenderState#isBaby` | field | 1 |
| `net/minecraft/client/renderer/feature/FlameFeatureRenderer#prepare` | method | 2 |
| `net/minecraft/client/renderer/feature/ModelFeatureRenderer#prepareModel` | method | 1 |
| `net/minecraft/client/renderer/feature/ModelFeatureRenderer$Submit#model` | method | 3 |
| `net/minecraft/client/renderer/feature/ModelFeatureRenderer$Submit#state` | method | 1 |
| `net/minecraft/client/renderer/item/ItemStackRenderState$LayerRenderState#submit` | method | 1 |
| `net/minecraft/client/renderer/rendertype/RenderType#format` | method | 1 |
| `net/minecraft/client/renderer/rendertype/RenderTypes#endGateway` | method | 1 |
| `net/minecraft/client/renderer/rendertype/RenderTypes#entityCutout` | method | 1 |
| `net/minecraft/client/renderer/rendertype/RenderTypes#entityCutoutCull` | method | 1 |
| `net/minecraft/client/renderer/rendertype/RenderTypes#entityCutoutZOffset` | method | 1 |
| `net/minecraft/client/renderer/rendertype/RenderTypes#entitySolid` | method | 1 |
| `net/minecraft/client/renderer/rendertype/RenderTypes#entityTranslucent` | method | 2 |
| `net/minecraft/client/renderer/rendertype/RenderTypes#lines` | method | 1 |
| `net/minecraft/client/renderer/rendertype/RenderTypes#outline` | method | 1 |
| `net/minecraft/client/renderer/state/GameRenderState#levelRenderState` | field | 1 |
| `net/minecraft/client/renderer/state/level/CameraRenderState#hudFov` | field | 1 |
| `net/minecraft/client/renderer/state/level/LevelRenderState#cameraRenderState` | field | 1 |
| `net/minecraft/client/renderer/texture/MissingTextureAtlasSprite#getLocation` | method | 1 |
| `net/minecraft/client/renderer/texture/OverlayTexture#NO_OVERLAY` | field | 1 |
| `net/minecraft/core/BlockPos#above` | method | 1 |
| `net/minecraft/core/BlockPos#below` | method | 1 |
| `net/minecraft/core/BlockPos#equals` | method | 1 |
| `net/minecraft/core/BlockPos#getX` | method | 2 |
| `net/minecraft/core/BlockPos#getY` | method | 3 |
| `net/minecraft/core/BlockPos#getZ` | method | 2 |
| `net/minecraft/core/BlockPos#mutable` | method | 1 |
| `net/minecraft/core/BlockPos$MutableBlockPos#getY` | method | 1 |
| `net/minecraft/core/BlockPos$MutableBlockPos#move` | method | 1 |
| `net/minecraft/core/DefaultedRegistry#forEach` | method | 1 |
| `net/minecraft/core/DefaultedRegistry#wrapAsHolder` | method | 1 |
| `net/minecraft/core/Direction#DOWN` | field | 2 |
| `net/minecraft/core/Direction#EAST` | field | 1 |
| `net/minecraft/core/Direction#NORTH` | field | 1 |
| `net/minecraft/core/Direction#SOUTH` | field | 1 |
| `net/minecraft/core/Direction#UP` | field | 1 |
| `net/minecraft/core/Direction#WEST` | field | 1 |
| `net/minecraft/core/Direction#values` | method | 2 |
| `net/minecraft/core/Holder#getRegisteredName` | method | 2 |
| `net/minecraft/core/Holder#unwrapKey` | method | 2 |
| `net/minecraft/core/Holder$Reference#is` | method | 1 |
| `net/minecraft/core/Holder$Reference#unwrapKey` | method | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#ENTITY_TYPE` | field | 1 |
| `net/minecraft/nbt/CompoundTag#contains` | method | 1 |
| `net/minecraft/network/chat/Component#getString` | method | 1 |
| `net/minecraft/network/chat/Component#literal` | method | 1 |
| `net/minecraft/network/chat/Component#nullToEmpty` | method | 1 |
| `net/minecraft/network/chat/Component#translatable` | method | 6 |
| `net/minecraft/network/chat/MutableComponent#create` | method | 1 |
| `net/minecraft/network/chat/MutableComponent#getString` | method | 4 |
| `net/minecraft/network/chat/contents/PlainTextContents$LiteralContents#<init>` | method | 1 |
| `net/minecraft/resources/Identifier#equals` | method | 3 |
| `net/minecraft/resources/Identifier#fromNamespaceAndPath` | method | 1 |
| `net/minecraft/resources/Identifier#getNamespace` | method | 3 |
| `net/minecraft/resources/Identifier#getPath` | method | 3 |
| `net/minecraft/resources/Identifier#isValidPath` | method | 1 |
| `net/minecraft/resources/Identifier#parse` | method | 1 |
| `net/minecraft/resources/Identifier#toString` | method | 3 |
| `net/minecraft/resources/Identifier#tryParse` | method | 1 |
| `net/minecraft/resources/ResourceKey#identifier` | method | 3 |
| `net/minecraft/server/packs/PackResources#packId` | method | 1 |
| `net/minecraft/server/packs/resources/Resource#open` | method | 2 |
| `net/minecraft/server/packs/resources/Resource#sourcePackId` | method | 1 |
| `net/minecraft/server/packs/resources/ResourceManager#getResource` | method | 4 |
| `net/minecraft/server/packs/resources/ResourceManager#listPacks` | method | 1 |
| `net/minecraft/server/packs/resources/ResourceManager#listResources` | method | 1 |
| `net/minecraft/tags/EntityTypeTags#BOAT` | field | 1 |
| `net/minecraft/util/ARGB#alpha` | method | 1 |
| `net/minecraft/util/ARGB#color` | method | 1 |
| `net/minecraft/util/ARGB#green` | method | 1 |
| `net/minecraft/util/Mth#abs` | method | 1 |
| `net/minecraft/util/Mth#catmullrom` | method | 3 |
| `net/minecraft/util/Mth#cos` | method | 1 |
| `net/minecraft/util/Mth#degreesDifferenceAbs` | method | 1 |
| `net/minecraft/util/Mth#sin` | method | 2 |
| `net/minecraft/util/Mth#sqrt` | method | 2 |
| `net/minecraft/util/RandomSource#create` | method | 1 |
| `net/minecraft/util/RandomSource#nextFloat` | method | 1 |
| `net/minecraft/util/RandomSource#nextInt` | method | 1 |
| `net/minecraft/world/InteractionHand#MAIN_HAND` | field | 1 |
| `net/minecraft/world/InteractionHand#OFF_HAND` | field | 1 |
| `net/minecraft/world/TickRateManager#isEntityFrozen` | method | 1 |
| `net/minecraft/world/TickRateManager#isFrozen` | method | 1 |
| `net/minecraft/world/entity/Entity#equals` | method | 1 |
| `net/minecraft/world/entity/Entity#getBbHeight` | method | 1 |
| `net/minecraft/world/entity/Entity#getBbWidth` | method | 1 |
| `net/minecraft/world/entity/Entity#getType` | method | 1 |
| `net/minecraft/world/entity/Entity#getUUID` | method | 1 |
| `net/minecraft/world/entity/Entity#isSwimming` | method | 1 |
| `net/minecraft/world/entity/Entity#isVisuallyCrawling` | method | 1 |
| `net/minecraft/world/entity/Entity#pick` | method | 1 |
| `net/minecraft/world/entity/Entity#yRot` | field | 1 |
| `net/minecraft/world/entity/Entity#yRotO` | field | 1 |
| `net/minecraft/world/entity/EntityType#builtInRegistryHolder` | method | 1 |
| `net/minecraft/world/entity/EntityType#getDescriptionId` | method | 2 |
| `net/minecraft/world/entity/EntityType#toString` | method | 2 |
| `net/minecraft/world/entity/EntityTypes#BLAZE` | field | 1 |
| `net/minecraft/world/entity/EntityTypes#BREEZE_WIND_CHARGE` | field | 1 |
| `net/minecraft/world/entity/EntityTypes#CAMEL_HUSK` | field | 1 |
| `net/minecraft/world/entity/EntityTypes#HORSE` | field | 1 |
| `net/minecraft/world/entity/EntityTypes#SPECTRAL_ARROW` | field | 1 |
| `net/minecraft/world/entity/EntityTypes#VILLAGER` | field | 1 |
| `net/minecraft/world/entity/HumanoidArm#LEFT` | field | 1 |
| `net/minecraft/world/entity/HumanoidArm#RIGHT` | field | 2 |
| `net/minecraft/world/entity/LivingEntity#deathTime` | field | 1 |
| `net/minecraft/world/entity/LivingEntity#getCustomName` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#getHealth` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#getItemInHand` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#getMainArm` | method | 2 |
| `net/minecraft/world/entity/LivingEntity#getMaxHealth` | method | 2 |
| `net/minecraft/world/entity/LivingEntity#getSwingAnimation` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#getUsedItemHand` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#getVehicle` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#getXRot` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#hurtTime` | field | 1 |
| `net/minecraft/world/entity/LivingEntity#isBaby` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#isBlocking` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#isFallFlying` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#isPassenger` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#isUsingItem` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#jumping` | field | 1 |
| `net/minecraft/world/entity/LivingEntity#level` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#walkAnimation` | field | 1 |
| `net/minecraft/world/entity/LivingEntity#xRotO` | field | 1 |
| `net/minecraft/world/entity/LivingEntity#yBodyRot` | field | 1 |
| `net/minecraft/world/entity/LivingEntity#yBodyRotO` | field | 1 |
| `net/minecraft/world/entity/LivingEntity#yHeadRot` | field | 1 |
| `net/minecraft/world/entity/LivingEntity#yHeadRotO` | field | 1 |
| `net/minecraft/world/entity/Mob#isAggressive` | method | 1 |
| `net/minecraft/world/entity/NeutralMob#getPersistentAngerEndTime` | method | 1 |
| `net/minecraft/world/entity/NeutralMob#isAngry` | method | 1 |
| `net/minecraft/world/entity/TamableAnimal#isInSittingPose` | method | 1 |
| `net/minecraft/world/entity/TamableAnimal#isTame` | method | 1 |
| `net/minecraft/world/entity/Targeting#getTarget` | method | 1 |
| `net/minecraft/world/entity/WalkAnimationState#position` | method | 1 |
| `net/minecraft/world/entity/WalkAnimationState#speed` | method | 1 |
| `net/minecraft/world/entity/animal/camel/Camel#isCamelSitting` | method | 1 |
| `net/minecraft/world/entity/animal/chicken/Chicken#getVariant` | method | 1 |
| `net/minecraft/world/entity/animal/chicken/Chicken#isBaby` | method | 1 |
| `net/minecraft/world/entity/animal/feline/Cat#isInSittingPose` | method | 1 |
| `net/minecraft/world/entity/animal/fox/Fox#isSitting` | method | 1 |
| `net/minecraft/world/entity/animal/parrot/Parrot#isInSittingPose` | method | 1 |
| `net/minecraft/world/entity/animal/pig/Pig#getVariant` | method | 1 |
| `net/minecraft/world/entity/animal/pig/Pig#isBaby` | method | 1 |
| `net/minecraft/world/entity/animal/wolf/Wolf#isInSittingPose` | method | 1 |
| `net/minecraft/world/entity/monster/Blaze#isOnFire` | method | 1 |
| `net/minecraft/world/entity/monster/Enderman#isCreepy` | method | 1 |
| `net/minecraft/world/entity/monster/Guardian#getActiveAttackTarget` | method | 1 |
| `net/minecraft/world/entity/monster/Vex#isCharging` | method | 1 |
| `net/minecraft/world/entity/monster/illager/SpellcasterIllager#isCastingSpell` | method | 1 |
| `net/minecraft/world/entity/monster/illager/Vindicator#isAggressive` | method | 1 |
| `net/minecraft/world/entity/player/Player#interactOn` | method | 1 |
| `net/minecraft/world/entity/player/Player#isLocalPlayer` | method | 2 |
| `net/minecraft/world/entity/projectile/Projectile#isInWall` | method | 1 |
| `net/minecraft/world/entity/projectile/arrow/AbstractArrow#isInGround` | method | 1 |
| `net/minecraft/world/entity/vehicle/boat/AbstractBoat#getRowingTime` | method | 1 |
| `net/minecraft/world/item/ItemStack#isEmpty` | method | 1 |
| `net/minecraft/world/level/Level#dimensionTypeRegistration` | method | 1 |
| `net/minecraft/world/level/Level#getBlockState` | method | 2 |
| `net/minecraft/world/level/Level#getFluidState` | method | 1 |
| `net/minecraft/world/level/Level#getGameTime` | method | 1 |
| `net/minecraft/world/level/Level#getOverworldClockTime` | method | 1 |
| `net/minecraft/world/level/block/Blocks#CHEST` | field | 1 |
| `net/minecraft/world/level/block/Blocks#ENCHANTING_TABLE` | field | 1 |
| `net/minecraft/world/level/block/Blocks#ENDER_CHEST` | field | 1 |
| `net/minecraft/world/level/block/Blocks#LECTERN` | field | 1 |
| `net/minecraft/world/level/block/Blocks#TRAPPED_CHEST` | field | 1 |
| `net/minecraft/world/level/block/entity/BlockEntity#getType` | method | 1 |
| `net/minecraft/world/level/block/entity/BlockEntityType#builtInRegistryHolder` | method | 1 |
| `net/minecraft/world/level/block/entity/BlockEntityTypes#BELL` | field | 1 |
| `net/minecraft/world/level/block/entity/BlockEntityTypes#CHEST` | field | 1 |
| `net/minecraft/world/level/block/entity/BlockEntityTypes#DECORATED_POT` | field | 1 |
| `net/minecraft/world/level/block/entity/BlockEntityTypes#ENCHANTING_TABLE` | field | 1 |
| `net/minecraft/world/level/block/entity/BlockEntityTypes#ENDER_CHEST` | field | 1 |
| `net/minecraft/world/level/block/entity/BlockEntityTypes#LECTERN` | field | 1 |
| `net/minecraft/world/level/block/entity/BlockEntityTypes#SHULKER_BOX` | field | 1 |
| `net/minecraft/world/level/block/entity/BlockEntityTypes#SIGN` | field | 1 |
| `net/minecraft/world/level/block/entity/BlockEntityTypes#TRAPPED_CHEST` | field | 1 |
| `net/minecraft/world/level/block/state/BlockState#entityCanStandOn` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#getBlock` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#getFluidState` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#is` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#isAir` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#toString` | method | 1 |
| `net/minecraft/world/level/dimension/BuiltinDimensionTypes#END` | field | 1 |
| `net/minecraft/world/level/dimension/BuiltinDimensionTypes#NETHER` | field | 1 |
| `net/minecraft/world/level/material/FluidState#isEmpty` | method | 2 |
| `net/minecraft/world/phys/AABB#<init>` | method | 2 |
| `net/minecraft/world/phys/AABB#inflate` | method | 2 |
| `net/minecraft/world/phys/AABB#maxX` | field | 1 |
| `net/minecraft/world/phys/AABB#maxY` | field | 1 |
| `net/minecraft/world/phys/AABB#maxZ` | field | 1 |
| `net/minecraft/world/phys/AABB#minX` | field | 1 |
| `net/minecraft/world/phys/AABB#minY` | field | 1 |
| `net/minecraft/world/phys/AABB#minZ` | field | 1 |
| `net/minecraft/world/phys/BlockHitResult#getBlockPos` | method | 1 |
| `net/minecraft/world/phys/HitResult#getType` | method | 1 |
| `net/minecraft/world/phys/HitResult$Type#BLOCK` | field | 1 |
| `net/minecraft/world/phys/Vec3#<init>` | method | 1 |
| `net/minecraft/world/phys/Vec3#add` | method | 1 |
| `net/minecraft/world/phys/Vec3#equals` | method | 1 |
| `net/minecraft/world/phys/Vec3#subtract` | method | 1 |
| `net/minecraft/world/phys/Vec3#x` | field | 1 |
| `net/minecraft/world/phys/Vec3#z` | field | 1 |

</details>

