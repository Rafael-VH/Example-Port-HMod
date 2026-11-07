# Horizon port report: Entity Texture Features

`entity_texture_features` 7.2.7 · entity_texture_features-7.2.7-26.3-fabric.jar · 259 classes

**Partly convertible**

| Status | Count | Meaning |
|---|---|---|
| auto | 1 | The Horizon hook or injection is known: the wiring can be generated. The Java body still has to become Rust. |
| content | 1 | Data and assets: horizon-port-pack carries these into a content mod (.hmod). |
| needs-mapping | 81 | The kind of change carries over, but the matching Horizon function isn't in the table yet. Run `match`. |
| needs-translation | 40 | No direct equivalent: a person (or an AI agent) has to rewrite this part. |
| unsupported | 0 | Horizon has nothing this can attach to yet. |

## Entry points

- `client`: traben.entity_texture_features.ETFInit
- `modmenu`: traben.entity_texture_features.config.ETFModMenu

## Mixins

| Injection | In | Horizon | Status | Note |
|---|---|---|---|---|
| @Inject(HEAD) on GameRenderer.render | `MixinGameRenderer.emf$injectCounter` | HEAD on hook `frame` | auto | Target mapping is approximate. |
| @Inject(RETURN) on Identifier.isValidPath | `MixinIdentifier.etf$illegalPathOverride` | RETURN | needs-mapping | Horizon's RETURN runs at every normal exit and passes the return value on. No Horizon function known for Identifier.isValidPath: run `match`. |
| @Inject(INVOKE) on LevelRenderer.submitEntities at Ljava/util/Iterator;next()Ljava/lang/Object; | `MixinLevelRenderer.emf$verify` | BEFORE_CALL | needs-mapping | Only whole statements, plain let initializers and block tails can be selected. No Horizon function known for LevelRenderer.submitEntities: run `match`. The call it hooks (Iterator.next) isn't mapped to a Rust function yet. |
| @Inject(INVOKE) on LevelRenderer.submitBlockEntities at Ljava/util/Iterator;next()Ljava/lang/Object; | `MixinLevelRenderer.emf$verify` | BEFORE_CALL | needs-mapping | Only whole statements, plain let initializers and block tails can be selected. No Horizon function known for LevelRenderer.submitBlockEntities: run `match`. The call it hooks (Iterator.next) isn't mapped to a Rust function yet. |
| @Inject(TAIL) on LevelRenderer.submitEntities | `MixinLevelRenderer.emf$verifyEnd` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for LevelRenderer.submitEntities: run `match`. |
| @Inject(TAIL) on LevelRenderer.submitBlockEntities | `MixinLevelRenderer.emf$verifyEnd` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for LevelRenderer.submitBlockEntities: run `match`. |
| @Inject(HEAD) on ModelPart.render | `MixinModelPart.etf$findOutIfInitialModelPart` | HEAD | needs-mapping | No Horizon function known for ModelPart.render: run `match`. |
| @Inject(RETURN) on ModelPart.render | `MixinModelPart.etf$doEmissiveIfInitialPart` | RETURN | needs-mapping | Horizon's RETURN runs at every normal exit and passes the return value on. No Horizon function known for ModelPart.render: run `match`. |
| @ModifyVariable(HEAD) on ModelPart.render | `MixinModelPart.etf$modify` | MODIFY_VARIABLE | needs-mapping | Selects a Rust local by name: pick it in the target function. No Horizon function known for ModelPart.render: run `match`. |
| mixin into RenderType with no injectors (interfaces or @Shadow only) | `MixinMultiPhase` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| @Inject(TAIL) on PackSelectionScreen.repositionElements | `MixinPackScreen.etf$etfButtonResize` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for PackSelectionScreen.repositionElements: run `match`. |
| @Inject(TAIL) on PackSelectionScreen.init | `MixinPackScreen.etf$etfButton` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for PackSelectionScreen.init: run `match`. |
| @Inject(HEAD) on SkinTextureDownloader.setNoAlpha | `MixinPlayerSkinTexture.etf$cancelling` | HEAD | needs-mapping | No Horizon function known for SkinTextureDownloader.setNoAlpha: run `match`. |
| @Inject(INVOKE) on SkinTextureDownloader.processLegacySkin at Lnet/minecraft/client/renderer/texture/SkinTextureDownloader;setNoAlpha(Lcom/mojang/blaze3d/platform/NativeImage;IIII)V | `MixinPlayerSkinTexture.etf$differentAlpha` | BEFORE_CALL on horizon_client_render::skins::process_legacy_skin | needs-mapping | Only whole statements, plain let initializers and block tails can be selected. The call it hooks (SkinTextureDownloader.setNoAlpha) isn't mapped to a Rust function yet. |
| @ModifyVariable(HEAD) on RenderTypes.entitySolid | `MixinRenderLayer.etf$mixinAllEntityLayers` | MODIFY_VARIABLE | needs-mapping | Selects a Rust local by name: pick it in the target function. No Horizon function known for RenderTypes.entitySolid: run `match`. |
| @ModifyVariable(HEAD) on RenderTypes.eyes | `MixinRenderLayer.etf$mixinAllEntityLayers` | MODIFY_VARIABLE | needs-mapping | Selects a Rust local by name: pick it in the target function. No Horizon function known for RenderTypes.eyes: run `match`. |
| @ModifyVariable(HEAD) on RenderTypes.energySwirl | `MixinRenderLayer.etf$mixinAllEntityLayers` | MODIFY_VARIABLE | needs-mapping | Selects a Rust local by name: pick it in the target function. No Horizon function known for RenderTypes.energySwirl: run `match`. |
| @ModifyVariable(HEAD) on RenderTypes.entitySmoothCutout | `MixinRenderLayer.etf$mixinAllEntityLayers` | MODIFY_VARIABLE | needs-mapping | Selects a Rust local by name: pick it in the target function. No Horizon function known for RenderTypes.entitySmoothCutout: run `match`. |
| @ModifyVariable(HEAD) on RenderTypes.itemEntityTranslucentCull | `MixinRenderLayer.etf$mixinAllEntityLayers` | MODIFY_VARIABLE | needs-mapping | Selects a Rust local by name: pick it in the target function. No Horizon function known for RenderTypes.itemEntityTranslucentCull: run `match`. |
| @ModifyVariable(HEAD) on RenderTypes.entityCutoutCull | `MixinRenderLayer.etf$mixinAllEntityLayers` | MODIFY_VARIABLE | needs-mapping | Selects a Rust local by name: pick it in the target function. No Horizon function known for RenderTypes.entityCutoutCull: run `match`. |
| @ModifyVariable(HEAD) on RenderTypes.entityCutout | `MixinRenderLayer.etf$mixinAllEntityLayers` | MODIFY_VARIABLE | needs-mapping | Selects a Rust local by name: pick it in the target function. No Horizon function known for RenderTypes.entityCutout: run `match`. |
| @ModifyVariable(HEAD) on RenderTypes.entityCutoutNoCullZOffset | `MixinRenderLayer.etf$mixinAllEntityLayers` | MODIFY_VARIABLE | needs-mapping | Selects a Rust local by name: pick it in the target function. No Horizon function known for RenderTypes.entityCutoutNoCullZOffset: run `match`. |
| @ModifyVariable(HEAD) on RenderTypes.entityDecal | `MixinRenderLayer.etf$mixinAllEntityLayers` | MODIFY_VARIABLE | needs-mapping | Selects a Rust local by name: pick it in the target function. No Horizon function known for RenderTypes.entityDecal: run `match`. |
| @ModifyVariable(HEAD) on RenderTypes.entityNoOutline | `MixinRenderLayer.etf$mixinAllEntityLayers` | MODIFY_VARIABLE | needs-mapping | Selects a Rust local by name: pick it in the target function. No Horizon function known for RenderTypes.entityNoOutline: run `match`. |
| @ModifyVariable(HEAD) on RenderTypes.entitySmoothCutout | `MixinRenderLayer.etf$mixinAllEntityLayers` | MODIFY_VARIABLE | needs-mapping | Selects a Rust local by name: pick it in the target function. No Horizon function known for RenderTypes.entitySmoothCutout: run `match`. |
| @ModifyVariable(HEAD) on RenderTypes.entityTranslucent | `MixinRenderLayer.etf$mixinAllEntityLayers` | MODIFY_VARIABLE | needs-mapping | Selects a Rust local by name: pick it in the target function. No Horizon function known for RenderTypes.entityTranslucent: run `match`. |
| @ModifyVariable(HEAD) on RenderTypes.entityTranslucentEmissive | `MixinRenderLayer.etf$mixinAllEntityLayers` | MODIFY_VARIABLE | needs-mapping | Selects a Rust local by name: pick it in the target function. No Horizon function known for RenderTypes.entityTranslucentEmissive: run `match`. |
| @ModifyVariable(HEAD) on RenderTypes.armorCutoutNoCull | `MixinRenderLayer.etf$mixinAllEntityLayers` | MODIFY_VARIABLE | needs-mapping | Selects a Rust local by name: pick it in the target function. No Horizon function known for RenderTypes.armorCutoutNoCull: run `match`. |
| @ModifyVariable(HEAD) on RenderTypes.entityShadow | `MixinRenderLayer.etf$mixinAllEntityLayers` | MODIFY_VARIABLE | needs-mapping | Selects a Rust local by name: pick it in the target function. No Horizon function known for RenderTypes.entityShadow: run `match`. |
| @ModifyVariable(HEAD) on RenderTypes.entityTranslucentCullItemTarget | `MixinRenderLayer.etf$mixinAllEntityLayers` | MODIFY_VARIABLE | needs-mapping | Selects a Rust local by name: pick it in the target function. No Horizon function known for RenderTypes.entityTranslucentCullItemTarget: run `match`. |
| @ModifyVariable(HEAD) on RenderTypes.entityCutoutZOffset | `MixinRenderLayer.etf$mixinAllEntityLayers` | MODIFY_VARIABLE | needs-mapping | Selects a Rust local by name: pick it in the target function. No Horizon function known for RenderTypes.entityCutoutZOffset: run `match`. |
| @ModifyVariable(HEAD) on RenderTypes.entityCutoutDissolve | `MixinRenderLayer.etf$mixinAllEntityLayers` | MODIFY_VARIABLE | needs-mapping | Selects a Rust local by name: pick it in the target function. No Horizon function known for RenderTypes.entityCutoutDissolve: run `match`. |
| @ModifyVariable(HEAD) on RenderTypes.entitySolidZOffsetForward | `MixinRenderLayer.etf$mixinAllEntityLayers` | MODIFY_VARIABLE | needs-mapping | Selects a Rust local by name: pick it in the target function. No Horizon function known for RenderTypes.entitySolidZOffsetForward: run `match`. |
| mixin into SpriteId with no injectors (interfaces or @Shadow only) | `MixinSpriteIdentifier` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| @Accessor on Tooltip.cachedTooltip | `TooltipAccessor.setCachedTooltip` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for Tooltip.cachedTooltip: run `match`. |
| @Inject(RETURN) on BaseSpawner.getOrCreateDisplayEntity | `MixinMobSpawnerLogic.etf$stabiliseMobSpawnerUUID` | RETURN | needs-mapping | Horizon's RETURN runs at every normal exit and passes the return value on. No Horizon function known for BaseSpawner.getOrCreateDisplayEntity: run `match`. |
| mixin into CancelTarget with no injectors (interfaces or @Shadow only) | `MixinSkullBlockEntityRenderer` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| @ModifyArg(INVOKE) on Blaze.aiStep at Lnet/minecraft/world/level/Level;addParticle(Lnet/minecraft/core/particles/ParticleOptions;DDDDDD)V | `MixinBlazeEntity.mixin` | MODIFY_ARG | needs-mapping | No Horizon function known for Blaze.aiStep: run `match`. The call it hooks (Level.addParticle) isn't mapped to a Rust function yet. |
| mixin into BlockEntity with no injectors (interfaces or @Shadow only) | `MixinBlockEntity` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| @Inject(HEAD) on BlockEntityRenderDispatcher.submit | `MixinBlockEntityRenderDispatcher.etf$grabContext` | HEAD | needs-mapping | No Horizon function known for BlockEntityRenderDispatcher.submit: run `match`. |
| @Inject(RETURN) on BlockEntityRenderDispatcher.submit | `MixinBlockEntityRenderDispatcher.etf$clearContext` | RETURN | needs-mapping | Horizon's RETURN runs at every normal exit and passes the return value on. No Horizon function known for BlockEntityRenderDispatcher.submit: run `match`. |
| @Inject(HEAD) on BlockEntityRenderState.extractBase | `MixinBlockEntityRenderState.etf$createRenderState` | HEAD | needs-mapping | No Horizon function known for BlockEntityRenderState.extractBase: run `match`. |
| @ModifyExpressionValue(INVOKE) on BlockEntityRenderState.extractBase at Lnet/minecraft/util/LightCoordsUtil;getLightCoords(Lnet/minecraft/world/level/BlockAndLightGetter;Lnet/minecraft/core/BlockPos;)I | `MixinBlockEntityRenderState.etf$vanillaLightOverride` | MODIFY_EXPRESSION_VALUE | needs-mapping | Selects an exact Rust expression: pick it in the target function. No Horizon function known for BlockEntityRenderState.extractBase: run `match`. |
| mixin into Entity with no injectors (interfaces or @Shadow only) | `MixinEntity` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| @Inject(HEAD) on EntityRenderDispatcher.submit | `MixinEntityRenderDispatcher.etf$grabContext` | HEAD | needs-mapping | No Horizon function known for EntityRenderDispatcher.submit: run `match`. |
| @Inject(RETURN) on EntityRenderDispatcher.submit | `MixinEntityRenderDispatcher.etf$clearContext` | RETURN | needs-mapping | Horizon's RETURN runs at every normal exit and passes the return value on. No Horizon function known for EntityRenderDispatcher.submit: run `match`. |
| mixin into EntityRenderState with no injectors (interfaces or @Shadow only) | `MixinEntityRenderState` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| @ModifyArg(INVOKE) on GlowSquid.aiStep at Lnet/minecraft/world/level/Level;addParticle(Lnet/minecraft/core/particles/ParticleOptions;DDDDDD)V | `MixinGlowSquidEntity.mixin` | MODIFY_ARG | needs-mapping | No Horizon function known for GlowSquid.aiStep: run `match`. The call it hooks (Level.addParticle) isn't mapped to a Rust function yet. |
| @WrapOperation(INVOKE) on Mob.addAdditionalSaveData at Lnet/minecraft/world/entity/Mob;writeLeashData(Lnet/minecraft/world/level/storage/ValueOutput;Lnet/minecraft/world/entity/Leashable$LeashData;)V | `MixinMob_LeashNBTFix.etf$catchLeashCrash` | WRAP_OPERATION | needs-mapping | No Horizon function known for Mob.addAdditionalSaveData: run `match`. The call it hooks (Mob.writeLeashData) isn't mapped to a Rust function yet. |
| @ModifyArg(INVOKE) on Phantom.tick at Lnet/minecraft/world/level/Level;addParticle(Lnet/minecraft/core/particles/ParticleOptions;DDDDDD)V | `MixinPhantomEntity.mixin` | MODIFY_ARG | needs-mapping | No Horizon function known for Phantom.tick: run `match`. The call it hooks (Level.addParticle) isn't mapped to a Rust function yet. |
| mixin into Avatar with no injectors (interfaces or @Shadow only) | `MixinPlayerEntity` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| @Inject(HEAD) on Player.interactOn | `MixinPlayerEntity_DebugClick.etf$injected` | HEAD | needs-mapping | No Horizon function known for Player.interactOn: run `match`. |
| mixin into SkullBlockEntity with no injectors (interfaces or @Shadow only) | `MixinSkullBlockEntity` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| mixin into CancelTarget with no injectors (interfaces or @Shadow only) | `MixinEndCrystalRenderer` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| @ModifyArg(INVOKE) on EnderDragonRenderer.submit at Lnet/minecraft/client/renderer/SubmitNodeCollector;submitModel(Lnet/minecraft/client/model/Model;Ljava/lang/Object;Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/rendertype/RenderType;III)V | `MixinEnderDragonEntityRenderer.etf$returnAlteredTexture2` | MODIFY_ARG | needs-mapping | No Horizon function known for EnderDragonRenderer.submit: run `match`. The call it hooks (SubmitNodeCollector.submitModel) isn't mapped to a Rust function yet. |
| @Inject(TAIL) on EntityRenderer.extractRenderState | `MixinEntityRenderer.etf$createRenderState` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for EntityRenderer.extractRenderState: run `match`. |
| @ModifyReturnValue(RETURN) on EntityRenderer.getPackedLightCoords | `MixinEntityRenderer.etf$vanillaLightOverrideCancel` | RETURN | needs-mapping | No Horizon function known for EntityRenderer.getPackedLightCoords: run `match`. |
| @Inject(HEAD) on EntityRenderer.submit | `MixinEntityRenderer.etf$protectPostRenderersLikeNametag` | HEAD | needs-mapping | No Horizon function known for EntityRenderer.submit: run `match`. |
| @Inject(TAIL) on EntityRenderer.submit | `MixinEntityRenderer.etf$revertForRenderersThatCallSuperFirst` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for EntityRenderer.submit: run `match`. |
| mixin into CancelTarget with no injectors (interfaces or @Shadow only) | `MixinIllusionerRenderer` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| @Inject(INVOKE) on LivingEntityRenderer.submit at Ljava/util/List;iterator()Ljava/util/Iterator; | `MixinLivingEntityRenderer.etf$markFeatures` | BEFORE_CALL | needs-mapping | Only whole statements, plain let initializers and block tails can be selected. No Horizon function known for LivingEntityRenderer.submit: run `match`. The call it hooks (List.iterator) isn't mapped to a Rust function yet. |
| @Inject(INVOKE) on LivingEntityRenderer.submit at Lcom/mojang/blaze3d/vertex/PoseStack;popPose()V | `MixinLivingEntityRenderer.etf$markFeaturesEnd` | BEFORE_CALL | needs-mapping | Only whole statements, plain let initializers and block tails can be selected. No Horizon function known for LivingEntityRenderer.submit: run `match`. The call it hooks (PoseStack.popPose) isn't mapped to a Rust function yet. |
| @Inject(INVOKE) on LivingEntityRenderer.submit at Ljava/util/List;iterator()Ljava/util/Iterator; | `MixinLivingEntityRenderer.emf$grabEntity` | BEFORE_CALL | needs-mapping | Only whole statements, plain let initializers and block tails can be selected. No Horizon function known for LivingEntityRenderer.submit: run `match`. The call it hooks (List.iterator) isn't mapped to a Rust function yet. |
| @Inject(INVOKE) on LivingEntityRenderer.submit at Ljava/util/Iterator;next()Ljava/lang/Object; | `MixinLivingEntityRenderer.emf$eachFeatureLoop` | BEFORE_CALL | needs-mapping | Only whole statements, plain let initializers and block tails can be selected. No Horizon function known for LivingEntityRenderer.submit: run `match`. The call it hooks (Iterator.next) isn't mapped to a Rust function yet. |
| @Inject(TAIL) on LivingEntityRenderer.submit | `MixinLivingEntityRenderer.emf$postRender` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for LivingEntityRenderer.submit: run `match`. |
| @Inject(INVOKE) on PaintingRenderer.submit at Lnet/minecraft/client/renderer/entity/PaintingRenderer;renderPainting(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;Lnet/minecraft/client/renderer/rendertype/RenderType;[IIILnet/minecraft/client/renderer/texture/TextureAtlasSprite;Lnet/minecraft/client/renderer/texture/TextureAtlasSprite;)V | `MixinPaintingEntityRenderer.etf$getSprites` | BEFORE_CALL | needs-mapping | Only whole statements, plain let initializers and block tails can be selected. No Horizon function known for PaintingRenderer.submit: run `match`. The call it hooks (PaintingRenderer.renderPainting) isn't mapped to a Rust function yet. |
| @Inject(TAIL) on AvatarRenderer.<init> | `MixinPlayerEntityRenderer.etf$addFeatures` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for AvatarRenderer.<init>: run `match`. |
| @Inject(TAIL) on AvatarRenderer.renderHand | `MixinPlayerEntityRenderer.etf$renderHandFeatures` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for AvatarRenderer.renderHand: run `match`. |
| @Inject(RETURN) on AvatarRenderer.getTextureLocation | `MixinPlayerEntityRenderer.etf$getTexture` | RETURN | needs-mapping | Horizon's RETURN runs at every normal exit and passes the return value on. No Horizon function known for AvatarRenderer.getTextureLocation: run `match`. |
| @Inject(HEAD) on WingsLayer.submit | `MixinElytraFeatureRenderer.etf$markPatchable` | HEAD | needs-mapping | No Horizon function known for WingsLayer.submit: run `match`. |
| @Inject(RETURN) on WingsLayer.submit | `MixinElytraFeatureRenderer.etf$markPatchableEnd` | RETURN | needs-mapping | Horizon's RETURN runs at every normal exit and passes the return value on. No Horizon function known for WingsLayer.submit: run `match`. |
| @ModifyExpressionValue(INVOKE) on EyesLayer.submit at Lnet/minecraft/client/renderer/entity/layers/EyesLayer;renderType()Lnet/minecraft/client/renderer/rendertype/RenderType; | `MixinEyeFeatureRenderer.etf$allowModifiableEyes` | MODIFY_EXPRESSION_VALUE | needs-mapping | Selects an exact Rust expression: pick it in the target function. No Horizon function known for EyesLayer.submit: run `match`. |
| @ModifyVariable(HEAD) on EyesLayer.submit | `MixinEyeFeatureRenderer.emf$markEyeLight` | MODIFY_VARIABLE | needs-mapping | Selects a Rust local by name: pick it in the target function. No Horizon function known for EyesLayer.submit: run `match`. |
| @Inject(HEAD) on MushroomCowMushroomLayer.submit | `MixinMooshroomMushroomFeatureRenderer.etf$injected` | HEAD | needs-mapping | No Horizon function known for MushroomCowMushroomLayer.submit: run `match`. |
| @Inject(HEAD) on MushroomCowMushroomLayer.submitMushroomBlock | `MixinMooshroomMushroomFeatureRenderer.etf$injected` | HEAD | needs-mapping | No Horizon function known for MushroomCowMushroomLayer.submitMushroomBlock: run `match`. |
| @Inject(INVOKE) on ParrotOnShoulderLayer.submitOnShoulder at Lnet/minecraft/client/renderer/SubmitNodeCollector;submitModel(Lnet/minecraft/client/model/Model;Ljava/lang/Object;Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/resources/Identifier;III)V | `MixinShoulderParrotFeatureRenderer.etf$modifySubmit` | BEFORE_CALL | needs-mapping | Only whole statements, plain let initializers and block tails can be selected. No Horizon function known for ParrotOnShoulderLayer.submitOnShoulder: run `match`. The call it hooks (SubmitNodeCollector.submitModel) isn't mapped to a Rust function yet. |
| @Inject(RETURN) on ParrotOnShoulderLayer.submitOnShoulder | `MixinShoulderParrotFeatureRenderer.etf$resetEntity` | RETURN | needs-mapping | Horizon's RETURN runs at every normal exit and passes the return value on. No Horizon function known for ParrotOnShoulderLayer.submitOnShoulder: run `match`. |
| @WrapOperation(INVOKE) on WardenModel.createTendrilsLayer at Lnet/minecraft/client/model/geom/builders/LayerDefinition;apply(Lnet/minecraft/client/model/geom/builders/MeshTransformer;)Lnet/minecraft/client/model/geom/builders/LayerDefinition; | `MixinWardenExtraTextureParts.etf$modifyParts1` | WRAP_OPERATION | needs-mapping | No Horizon function known for WardenModel.createTendrilsLayer: run `match`. The call it hooks (LayerDefinition.apply) isn't mapped to a Rust function yet. |
| @WrapOperation(INVOKE) on WardenModel.createHeartLayer at Lnet/minecraft/client/model/geom/builders/LayerDefinition;apply(Lnet/minecraft/client/model/geom/builders/MeshTransformer;)Lnet/minecraft/client/model/geom/builders/LayerDefinition; | `MixinWardenExtraTextureParts.etf$modifyParts1` | WRAP_OPERATION | needs-mapping | No Horizon function known for WardenModel.createHeartLayer: run `match`. The call it hooks (LayerDefinition.apply) isn't mapped to a Rust function yet. |
| @WrapOperation(INVOKE) on WardenModel.createBioluminescentLayer at Lnet/minecraft/client/model/geom/builders/LayerDefinition;apply(Lnet/minecraft/client/model/geom/builders/MeshTransformer;)Lnet/minecraft/client/model/geom/builders/LayerDefinition; | `MixinWardenExtraTextureParts.etf$modifyParts1` | WRAP_OPERATION | needs-mapping | No Horizon function known for WardenModel.createBioluminescentLayer: run `match`. The call it hooks (LayerDefinition.apply) isn't mapped to a Rust function yet. |
| @WrapOperation(INVOKE) on WardenModel.createPulsatingSpotsLayer at Lnet/minecraft/client/model/geom/builders/LayerDefinition;apply(Lnet/minecraft/client/model/geom/builders/MeshTransformer;)Lnet/minecraft/client/model/geom/builders/LayerDefinition; | `MixinWardenExtraTextureParts.etf$modifyParts1` | WRAP_OPERATION | needs-mapping | No Horizon function known for WardenModel.createPulsatingSpotsLayer: run `match`. The call it hooks (LayerDefinition.apply) isn't mapped to a Rust function yet. |
| mixin into CancelTarget with no injectors (interfaces or @Shadow only) | `MixinBatchableBufferSource` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| mixin into CancelTarget with no injectors (interfaces or @Shadow only) | `MixinFullyBufferedMultiBufferSource` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| mixin into CancelTarget with no injectors (interfaces or @Shadow only) | `MixinInnerWrappedRenderType` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| mixin into OuterWrappedRenderType with no injectors (interfaces or @Shadow only) | `MixinOuterWrappedRenderType` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| mixin into CancelTarget with no injectors (interfaces or @Shadow only) | `MixinFullyBufferedMultiBufferSource` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| mixin into CancelTarget with no injectors (interfaces or @Shadow only) | `MixinInnerWrappedRenderType` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| mixin into CancelTarget with no injectors (interfaces or @Shadow only) | `MixinOuterWrappedRenderType` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| mixin into CancelTarget with no injectors (interfaces or @Shadow only) | `MixinCustomizableModelPart` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| mixin into CancelTarget with no injectors (interfaces or @Shadow only) | `MixinModelPartSodium` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| mixin into CancelTarget with no injectors (interfaces or @Shadow only) | `MixinSodiumBufferBuilder` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| mixin into CancelTarget with no injectors (interfaces or @Shadow only) | `MixinSodiumWorldRenderer` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| @Inject(TAIL) on Minecraft.disconnect* | `MixinMinecraftClient.etf$injected` | RETURN | needs-translation | Horizon has no TAIL: RETURN runs at every exit, early ones too. Wildcard or unknown selector `disconnect*`. |
| @Inject(HEAD) on Minecraft.runTick | `MixinMinecraftClient.etf$recycleSubmitData` | HEAD | needs-mapping | No Horizon function known for Minecraft.runTick: run `match`. |
| @Inject(HEAD) on Minecraft.onResourceLoadFinished | `MixinResourceReload.etf$injected` | HEAD | needs-mapping | No Horizon function known for Minecraft.onResourceLoadFinished: run `match`. |
| @Inject(INVOKE) on PictureInPictureRenderer.prepare at Lnet/minecraft/client/renderer/feature/FeatureRenderDispatcher;renderAllFeatures(Lcom/mojang/renderpearl/api/commands/RenderPass;Lnet/minecraft/client/renderer/feature/FeatureRenderDispatcher$PreparedFrame;)V | `Mixin_GuiEntityRenderer.emf$initRender2` | BEFORE_CALL | needs-mapping | Only whole statements, plain let initializers and block tails can be selected. No Horizon function known for PictureInPictureRenderer.prepare: run `match`. The call it hooks (FeatureRenderDispatcher.renderAllFeatures) isn't mapped to a Rust function yet. |
| @Inject(TAIL) on PictureInPictureRenderer.prepare | `Mixin_GuiEntityRenderer.emf$endRender` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for PictureInPictureRenderer.prepare: run `match`. |
| mixin into CancelTarget with no injectors (interfaces or @Shadow only) | `Mixin_ModelPartRenderer` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| @Inject(HEAD) on ModelFeatureRenderer.prepareModel | `Mixin_ModelRenderer.emf$initRender` | HEAD | needs-mapping | No Horizon function known for ModelFeatureRenderer.prepareModel: run `match`. |
| @Inject(TAIL) on ModelFeatureRenderer.prepareModel | `Mixin_ModelRenderer.emf$endRender` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for ModelFeatureRenderer.prepareModel: run `match`. |
| @Inject(RETURN) on Submit.<init> | `Mixin_ModelSubmit_AddData.emf$initBackupState` | RETURN | needs-mapping | Horizon's RETURN runs at every normal exit and passes the return value on. No Horizon function known for Submit.<init>: run `match`. |
| @Inject(HEAD) on ChestSpecialRenderer.submit | `Mixin_ChestSpecialRenderer_PassState.emf$dummyState` | HEAD | needs-mapping | No Horizon function known for ChestSpecialRenderer.submit: run `match`. |
| @Inject(TAIL) on ChestSpecialRenderer.submit | `Mixin_ChestSpecialRenderer_PassState.emf$reset` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for ChestSpecialRenderer.submit: run `match`. |
| mixin into BufferBuilder with no injectors (interfaces or @Shadow only) | `MixinBufferBuilder` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| mixin into SheetedDecalTextureGenerator with no injectors (interfaces or @Shadow only) | `MixinSheetedDecalTextureGen` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| mixin into SpriteCoordinateExpander with no injectors (interfaces or @Shadow only) | `MixinSpriteCoordExpander` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| @ModifyVariable(HEAD) on RenderTypeFeatureRenderer.getVertexBuilder | `MixinVertexConsumerProvider$Immediate.etf$modifyRenderLayer` | MODIFY_VARIABLE | needs-mapping | Selects a Rust local by name: pick it in the target function. No Horizon function known for RenderTypeFeatureRenderer.getVertexBuilder: run `match`. |
| @Inject(RETURN) on RenderTypeFeatureRenderer.getVertexBuilder | `MixinVertexConsumerProvider$Immediate.etf$injectIntoGetBufferReturn` | RETURN | needs-mapping | Horizon's RETURN runs at every normal exit and passes the return value on. No Horizon function known for RenderTypeFeatureRenderer.getVertexBuilder: run `match`. |
| mixin into CancelTarget with no injectors (interfaces or @Shadow only) | `MixinVertexMultiConsumer$Double` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |

## Classes extending the game

| Class | In | Status | Note |
|---|---|---|---|
| extends Screen | `ETFConfigScreenSkinTool$ConfirmScreen` | needs-translation | No general screen hook; simple option pages can use the settings hook. |
| extends Plain | `ETFConfigScreenSkinToolPixelSelection$1` | needs-translation | No mapping for this superclass yet. |
| extends RenderLayer | `ETFPlayerFeatureRenderer` | needs-translation | No mapping for this superclass yet. |
| extends ImageButton | `MixinPackScreen$1` | needs-translation | No mapping for this superclass yet. |
| extends Entry | `TConfigEntryListWidget$TConfigEntryForList` | needs-translation | No mapping for this superclass yet. |
| extends AbstractSelectionList | `TConfigEntryListWidget` | needs-translation | No mapping for this superclass yet. |
| extends Screen | `TConfigScreen` | needs-translation | No general screen hook; simple option pages can use the settings hook. |
| extends Plain | `TConfigEntryBoolean$BooleanButtonWidget` | needs-translation | No mapping for this superclass yet. |
| extends Plain | `TConfigEntryCategory$CategoryButton` | needs-translation | No mapping for this superclass yet. |
| extends Plain | `TConfigEntryEnumButton$EnumButtonWidget` | needs-translation | No mapping for this superclass yet. |
| extends AbstractSliderButton | `TConfigEntryEnumSlider$EnumSliderWidget` | needs-translation | No mapping for this superclass yet. |
| extends AbstractSliderButton | `TConfigEntryInt$IntSliderWidget` | needs-translation | No mapping for this superclass yet. |

## Access wideners

| File | In | Status | Note |
|---|---|---|---|
| access widener (opens private game fields) | `entity_texture_features_16.accesswidener` | needs-translation | Native helpers already see the game's fields, once each field is mapped. |

## Content

| What | In | Status | Note |
|---|---|---|---|
| 29 asset files, 0 data files | `jar resources` | content | Recipes, loot, tags, models, textures, worldgen: horizon-port-pack carries these. |

## Game methods and fields used

22 of 558 are mapped to Horizon. Every Java body that uses an unmapped one needs it mapped before its Rust can be written.

<details><summary>Mapped</summary>

| Java | Horizon |
|---|---|
| `net/minecraft/client/Minecraft#font` | `(implicit)` |
| `net/minecraft/client/Minecraft#getInstance` | `self` |
| `net/minecraft/client/Minecraft#player` | `self.native_player()` |
| `net/minecraft/client/StringSplitter#splitLines` | horizon_client_render::font::Font::split_lines (suggested) |
| `net/minecraft/client/renderer/GameRenderer#render` | hook `frame` |
| `net/minecraft/client/renderer/texture/SkinTextureDownloader#processLegacySkin` | horizon_client_render::skins::process_legacy_skin (suggested) |
| `net/minecraft/core/registries/BuiltInRegistries#BLOCK` | content |
| `net/minecraft/network/chat/MutableComponent#withStyle` | horizon_client_core::text::Text::with_style (suggested) |
| `net/minecraft/util/Mth#ceil` | horizon_client_render::hud_layout::ceil (suggested) |
| `net/minecraft/util/Mth#clamp` | horizon_client_render::chat::clamp (suggested) |
| `net/minecraft/util/Mth#floor` | horizon_client_render::gfx::occlusion::floor (suggested) |
| `net/minecraft/world/entity/Entity#getX` | `self.native_player()?.x` |
| `net/minecraft/world/entity/Entity#getY` | `self.native_player()?.y` |
| `net/minecraft/world/entity/Entity#getZ` | `self.native_player()?.z` |
| `net/minecraft/world/entity/Entity#isOnFire` | horizon_client::mobs::Mob::is_on_fire (suggested) |
| `net/minecraft/world/entity/EntityType#create` | horizon_client::mobs::finalize::create (suggested) |
| `net/minecraft/world/entity/LivingEntity#hasEffect` | horizon_client::mobs::Seen::has_effect (suggested) |
| `net/minecraft/world/item/ItemStackTemplate#create` | horizon_client::crafting::Template::create (suggested) |
| `net/minecraft/world/level/Level#canSeeSky` | horizon_client::mobs::world::MobWorld<'_>::can_see_sky (suggested) |
| `net/minecraft/world/level/Level#getMaxLocalRawBrightness` | horizon_client::animate_tick::AnimateContext<'_>::max_local_raw_brightness (suggested) |
| `net/minecraft/world/level/Level#getMinY` | horizon_client::game::Dim::min_y (suggested) |
| `net/minecraft/world/level/Level#isClientSide` | horizon_world::level::is_client_side (suggested) |

</details>

<details><summary>Not mapped yet</summary>

| Java | Kind | Classes using it |
|---|---|---|
| `com/mojang/blaze3d/Blaze3D#openPath` | method | 1 |
| `com/mojang/blaze3d/platform/NativeImage#<init>` | method | 2 |
| `com/mojang/blaze3d/platform/NativeImage#close` | method | 3 |
| `com/mojang/blaze3d/platform/NativeImage#copyFrom` | method | 3 |
| `com/mojang/blaze3d/platform/NativeImage#fillRect` | method | 2 |
| `com/mojang/blaze3d/platform/NativeImage#getHeight` | method | 6 |
| `com/mojang/blaze3d/platform/NativeImage#getLuminanceOrAlpha` | method | 2 |
| `com/mojang/blaze3d/platform/NativeImage#getPixel` | method | 1 |
| `com/mojang/blaze3d/platform/NativeImage#getWidth` | method | 6 |
| `com/mojang/blaze3d/platform/NativeImage#read` | method | 2 |
| `com/mojang/blaze3d/platform/NativeImage#setPixel` | method | 1 |
| `com/mojang/blaze3d/platform/NativeImage#writeToFile` | method | 3 |
| `com/mojang/blaze3d/systems/RenderSystem#isOnRenderThread` | method | 2 |
| `com/mojang/blaze3d/vertex/BufferBuilder#building` | field | 1 |
| `com/mojang/blaze3d/vertex/PoseStack#popPose` | method | 2 |
| `com/mojang/blaze3d/vertex/PoseStack#pushPose` | method | 1 |
| `com/mojang/blaze3d/vertex/PoseStack#scale` | method | 1 |
| `java/util/Iterator#next` | method | 3 |
| `java/util/List#iterator` | method | 2 |
| `net/minecraft/ChatFormatting#BOLD` | field | 1 |
| `net/minecraft/ChatFormatting#RED` | field | 1 |
| `net/minecraft/ChatFormatting#YELLOW` | field | 1 |
| `net/minecraft/advancements/predicates/NbtPredicate#getEntityTagToCompare` | method | 1 |
| `net/minecraft/client/Minecraft#execute` | method | 1 |
| `net/minecraft/client/Minecraft#gameRenderer` | field | 1 |
| `net/minecraft/client/Minecraft#getConnection` | method | 1 |
| `net/minecraft/client/Minecraft#getLaunchedVersion` | method | 1 |
| `net/minecraft/client/Minecraft#getProxy` | method | 1 |
| `net/minecraft/client/Minecraft#getResourceManager` | method | 8 |
| `net/minecraft/client/Minecraft#getResourcePackDirectory` | method | 1 |
| `net/minecraft/client/Minecraft#getResourcePackRepository` | method | 1 |
| `net/minecraft/client/Minecraft#getSkinManager` | method | 2 |
| `net/minecraft/client/Minecraft#getTextureManager` | method | 4 |
| `net/minecraft/client/Minecraft#getUser` | method | 1 |
| `net/minecraft/client/Minecraft#getVanillaPackResources` | method | 1 |
| `net/minecraft/client/Minecraft#gui` | field | 1 |
| `net/minecraft/client/Minecraft#onResourceLoadFinished` | method | 1 |
| `net/minecraft/client/Minecraft#options` | field | 1 |
| `net/minecraft/client/Minecraft#reloadResourcePacks` | method | 1 |
| `net/minecraft/client/Minecraft#runTick` | method | 1 |
| `net/minecraft/client/Options#languageCode` | field | 1 |
| `net/minecraft/client/User#getAccessToken` | method | 1 |
| `net/minecraft/client/gui/Font#getSplitter` | method | 1 |
| `net/minecraft/client/gui/Gui#screen` | method | 1 |
| `net/minecraft/client/gui/Gui#setScreen` | method | 1 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#blit` | method | 4 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#centeredText` | method | 4 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#disableScissor` | method | 1 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#enableScissor` | method | 1 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#entity` | method | 1 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#text` | method | 3 |
| `net/minecraft/client/gui/components/AbstractSelectionList#<init>` | method | 1 |
| `net/minecraft/client/gui/components/AbstractSelectionList#extractListBackground` | method | 1 |
| `net/minecraft/client/gui/components/AbstractSelectionList#extractListSeparators` | method | 1 |
| `net/minecraft/client/gui/components/AbstractSelectionList#getRowWidth` | method | 1 |
| `net/minecraft/client/gui/components/AbstractSelectionList#scrollBarX` | method | 1 |
| `net/minecraft/client/gui/components/AbstractSelectionList$Entry#<init>` | method | 1 |
| `net/minecraft/client/gui/components/AbstractSelectionList$Entry#getContentBottom` | method | 1 |
| `net/minecraft/client/gui/components/AbstractSelectionList$Entry#getContentHeight` | method | 1 |
| `net/minecraft/client/gui/components/AbstractSelectionList$Entry#getContentRight` | method | 1 |
| `net/minecraft/client/gui/components/AbstractSelectionList$Entry#getContentWidth` | method | 1 |
| `net/minecraft/client/gui/components/AbstractSelectionList$Entry#getContentX` | method | 1 |
| `net/minecraft/client/gui/components/AbstractSelectionList$Entry#getContentXMiddle` | method | 1 |
| `net/minecraft/client/gui/components/AbstractSelectionList$Entry#getContentY` | method | 1 |
| `net/minecraft/client/gui/components/AbstractSelectionList$Entry#getContentYMiddle` | method | 1 |
| `net/minecraft/client/gui/components/AbstractSelectionList$Entry#getHeight` | method | 1 |
| `net/minecraft/client/gui/components/AbstractSelectionList$Entry#getRectangle` | method | 1 |
| `net/minecraft/client/gui/components/AbstractSelectionList$Entry#getWidth` | method | 1 |
| `net/minecraft/client/gui/components/AbstractSelectionList$Entry#getX` | method | 1 |
| `net/minecraft/client/gui/components/AbstractSelectionList$Entry#getY` | method | 1 |
| `net/minecraft/client/gui/components/AbstractSelectionList$Entry#isFocused` | method | 1 |
| `net/minecraft/client/gui/components/AbstractSelectionList$Entry#isMouseOver` | method | 1 |
| `net/minecraft/client/gui/components/AbstractSelectionList$Entry#setHeight` | method | 1 |
| `net/minecraft/client/gui/components/AbstractSelectionList$Entry#setWidth` | method | 1 |
| `net/minecraft/client/gui/components/AbstractSelectionList$Entry#setX` | method | 1 |
| `net/minecraft/client/gui/components/AbstractSelectionList$Entry#setY` | method | 1 |
| `net/minecraft/client/gui/components/AbstractSelectionList$Entry#visitWidgets` | method | 1 |
| `net/minecraft/client/gui/components/AbstractSliderButton#<init>` | method | 2 |
| `net/minecraft/client/gui/components/AbstractWidget#active` | field | 1 |
| `net/minecraft/client/gui/components/AbstractWidget#extractRenderState` | method | 1 |
| `net/minecraft/client/gui/components/AbstractWidget#isMouseOver` | method | 1 |
| `net/minecraft/client/gui/components/AbstractWidget#mouseClicked` | method | 1 |
| `net/minecraft/client/gui/components/AbstractWidget#mouseDragged` | method | 1 |
| `net/minecraft/client/gui/components/AbstractWidget#mouseReleased` | method | 1 |
| `net/minecraft/client/gui/components/AbstractWidget#setFocused` | method | 1 |
| `net/minecraft/client/gui/components/Button#DEFAULT_NARRATION` | field | 1 |
| `net/minecraft/client/gui/components/Button#active` | field | 3 |
| `net/minecraft/client/gui/components/Button#builder` | method | 7 |
| `net/minecraft/client/gui/components/Button#getHeight` | method | 1 |
| `net/minecraft/client/gui/components/Button#getWidth` | method | 1 |
| `net/minecraft/client/gui/components/Button#getX` | method | 2 |
| `net/minecraft/client/gui/components/Button#getY` | method | 2 |
| `net/minecraft/client/gui/components/Button#onPress` | method | 1 |
| `net/minecraft/client/gui/components/Button#setMessage` | method | 4 |
| `net/minecraft/client/gui/components/Button#setRectangle` | method | 2 |
| `net/minecraft/client/gui/components/Button$Builder#bounds` | method | 7 |
| `net/minecraft/client/gui/components/Button$Builder#build` | method | 7 |
| `net/minecraft/client/gui/components/Button$Builder#tooltip` | method | 4 |
| `net/minecraft/client/gui/components/Button$Plain#<init>` | method | 4 |
| `net/minecraft/client/gui/components/Button$Plain#getMessage` | method | 1 |
| `net/minecraft/client/gui/components/EditBox#<init>` | method | 1 |
| `net/minecraft/client/gui/components/EditBox#setHint` | method | 1 |
| `net/minecraft/client/gui/components/EditBox#setResponder` | method | 1 |
| `net/minecraft/client/gui/components/EditBox#setWidth` | method | 1 |
| `net/minecraft/client/gui/components/EditBox#setY` | method | 1 |
| `net/minecraft/client/gui/components/ImageButton#<init>` | method | 1 |
| `net/minecraft/client/gui/components/ImageButton#setX` | method | 1 |
| `net/minecraft/client/gui/components/ImageButton#setY` | method | 1 |
| `net/minecraft/client/gui/components/StringWidget#<init>` | method | 2 |
| `net/minecraft/client/gui/components/StringWidget#extractRenderState` | method | 1 |
| `net/minecraft/client/gui/components/StringWidget#setRectangle` | method | 2 |
| `net/minecraft/client/gui/components/Tooltip#cachedTooltip` | method | 1 |
| `net/minecraft/client/gui/components/Tooltip#create` | method | 5 |
| `net/minecraft/client/gui/components/WidgetSprites#<init>` | method | 1 |
| `net/minecraft/client/gui/render/pip/PictureInPictureRenderer#prepare` | method | 2 |
| `net/minecraft/client/gui/screens/Screen#<init>` | method | 3 |
| `net/minecraft/client/gui/screens/Screen#extractRenderState` | method | 2 |
| `net/minecraft/client/gui/screens/Screen#init` | method | 1 |
| `net/minecraft/client/gui/screens/Screen#width` | field | 1 |
| `net/minecraft/client/gui/screens/inventory/InventoryScreen#extractEntityInInventoryFollowsMouse` | method | 1 |
| `net/minecraft/client/gui/screens/inventory/InventoryScreen#extractRenderState` | method | 1 |
| `net/minecraft/client/gui/screens/packs/PackSelectionScreen#init` | method | 1 |
| `net/minecraft/client/gui/screens/packs/PackSelectionScreen#repositionElements` | method | 1 |
| `net/minecraft/client/input/MouseButtonEvent#<init>` | method | 1 |
| `net/minecraft/client/input/MouseButtonEvent#x` | method | 1 |
| `net/minecraft/client/input/MouseButtonEvent#y` | method | 1 |
| `net/minecraft/client/input/MouseButtonInfo#<init>` | method | 1 |
| `net/minecraft/client/model/EntityModel#<init>` | method | 1 |
| `net/minecraft/client/model/Model#renderToBuffer` | method | 1 |
| `net/minecraft/client/model/Model#renderType` | method | 1 |
| `net/minecraft/client/model/Model#root` | method | 1 |
| `net/minecraft/client/model/geom/ModelPart#getChild` | method | 1 |
| `net/minecraft/client/model/geom/ModelPart#getInitialPose` | method | 1 |
| `net/minecraft/client/model/geom/ModelPart#loadPose` | method | 1 |
| `net/minecraft/client/model/geom/ModelPart#render` | method | 4 |
| `net/minecraft/client/model/geom/ModelPart#translateAndRotate` | method | 1 |
| `net/minecraft/client/model/geom/ModelPart#xRot` | field | 2 |
| `net/minecraft/client/model/geom/ModelPart#y` | field | 1 |
| `net/minecraft/client/model/geom/ModelPart#yRot` | field | 1 |
| `net/minecraft/client/model/geom/ModelPart#z` | field | 1 |
| `net/minecraft/client/model/geom/PartPose#ZERO` | field | 2 |
| `net/minecraft/client/model/geom/PartPose#offset` | method | 1 |
| `net/minecraft/client/model/geom/builders/CubeDeformation#<init>` | method | 2 |
| `net/minecraft/client/model/geom/builders/CubeDeformation#NONE` | field | 1 |
| `net/minecraft/client/model/geom/builders/CubeDeformation#extend` | method | 1 |
| `net/minecraft/client/model/geom/builders/CubeListBuilder#addBox` | method | 3 |
| `net/minecraft/client/model/geom/builders/CubeListBuilder#create` | method | 2 |
| `net/minecraft/client/model/geom/builders/CubeListBuilder#texOffs` | method | 2 |
| `net/minecraft/client/model/geom/builders/LayerDefinition#apply` | method | 4 |
| `net/minecraft/client/model/geom/builders/LayerDefinition#bakeRoot` | method | 1 |
| `net/minecraft/client/model/geom/builders/MeshDefinition#<init>` | method | 2 |
| `net/minecraft/client/model/geom/builders/MeshDefinition#getRoot` | method | 2 |
| `net/minecraft/client/model/geom/builders/PartDefinition#addOrReplaceChild` | method | 2 |
| `net/minecraft/client/model/geom/builders/PartDefinition#bake` | method | 2 |
| `net/minecraft/client/model/geom/builders/PartDefinition#getChild` | method | 2 |
| `net/minecraft/client/model/monster/creeper/CreeperModel#createBodyLayer` | method | 1 |
| `net/minecraft/client/model/monster/warden/WardenModel#createBioluminescentLayer` | method | 1 |
| `net/minecraft/client/model/monster/warden/WardenModel#createHeartLayer` | method | 1 |
| `net/minecraft/client/model/monster/warden/WardenModel#createPulsatingSpotsLayer` | method | 1 |
| `net/minecraft/client/model/monster/warden/WardenModel#createTendrilsLayer` | method | 1 |
| `net/minecraft/client/model/player/PlayerModel#body` | field | 1 |
| `net/minecraft/client/model/player/PlayerModel#head` | field | 1 |
| `net/minecraft/client/model/player/PlayerModel#root` | method | 1 |
| `net/minecraft/client/multiplayer/ClientPacketListener#getPlayerInfo` | method | 1 |
| `net/minecraft/client/multiplayer/PlayerInfo#getProfile` | method | 1 |
| `net/minecraft/client/player/AbstractClientPlayer#getGameProfile` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#gameMode` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#getBoundingBox` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#getGameProfile` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#getSkin` | method | 2 |
| `net/minecraft/client/player/LocalPlayer#getUUID` | method | 4 |
| `net/minecraft/client/player/LocalPlayer#level` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#sendSystemMessage` | method | 1 |
| `net/minecraft/client/renderer/GameRenderer#gameRenderState` | method | 1 |
| `net/minecraft/client/renderer/LevelRenderer#submitBlockEntities` | method | 2 |
| `net/minecraft/client/renderer/LevelRenderer#submitEntities` | method | 2 |
| `net/minecraft/client/renderer/RenderPipelines#GUI_TEXTURED` | field | 4 |
| `net/minecraft/client/renderer/SubmitNodeCollector#submitCustomGeometry` | method | 1 |
| `net/minecraft/client/renderer/SubmitNodeCollector#submitModel` | method | 2 |
| `net/minecraft/client/renderer/SubmitNodeCollector#submitModelPart` | method | 2 |
| `net/minecraft/client/renderer/blockentity/BlockEntityRenderDispatcher#submit` | method | 2 |
| `net/minecraft/client/renderer/blockentity/BlockEntityRenderers#PROVIDERS` | field | 1 |
| `net/minecraft/client/renderer/blockentity/state/BlockEntityRenderState#extractBase` | method | 2 |
| `net/minecraft/client/renderer/entity/EnderDragonRenderer#submit` | method | 1 |
| `net/minecraft/client/renderer/entity/EntityRenderDispatcher#submit` | method | 2 |
| `net/minecraft/client/renderer/entity/EntityRenderer#<init>` | method | 3 |
| `net/minecraft/client/renderer/entity/EntityRenderer#extractRenderState` | method | 1 |
| `net/minecraft/client/renderer/entity/EntityRenderer#getPackedLightCoords` | method | 1 |
| `net/minecraft/client/renderer/entity/EntityRenderer#submit` | method | 2 |
| `net/minecraft/client/renderer/entity/LivingEntityRenderer#<init>` | method | 1 |
| `net/minecraft/client/renderer/entity/LivingEntityRenderer#submit` | method | 5 |
| `net/minecraft/client/renderer/entity/PaintingRenderer#renderPainting` | method | 1 |
| `net/minecraft/client/renderer/entity/PaintingRenderer#submit` | method | 1 |
| `net/minecraft/client/renderer/entity/layers/EyesLayer#submit` | method | 2 |
| `net/minecraft/client/renderer/entity/layers/MushroomCowMushroomLayer#submit` | method | 1 |
| `net/minecraft/client/renderer/entity/layers/MushroomCowMushroomLayer#submitMushroomBlock` | method | 1 |
| `net/minecraft/client/renderer/entity/layers/ParrotOnShoulderLayer#submitOnShoulder` | method | 2 |
| `net/minecraft/client/renderer/entity/layers/RenderLayer#<init>` | method | 2 |
| `net/minecraft/client/renderer/entity/layers/WingsLayer#submit` | method | 2 |
| `net/minecraft/client/renderer/entity/player/AvatarRenderer#<init>` | method | 1 |
| `net/minecraft/client/renderer/entity/player/AvatarRenderer#getTextureLocation` | method | 1 |
| `net/minecraft/client/renderer/entity/player/AvatarRenderer#renderHand` | method | 1 |
| `net/minecraft/client/renderer/entity/state/AvatarRenderState#showJacket` | field | 1 |
| `net/minecraft/client/renderer/entity/state/EntityRenderState#lightCoords` | field | 1 |
| `net/minecraft/client/renderer/entity/state/MushroomCowRenderState#variant` | field | 1 |
| `net/minecraft/client/renderer/entity/state/PaintingRenderState#direction` | field | 1 |
| `net/minecraft/client/renderer/feature/FeatureRenderDispatcher#renderAllFeatures` | method | 1 |
| `net/minecraft/client/renderer/feature/ModelFeatureRenderer#prepareModel` | method | 2 |
| `net/minecraft/client/renderer/feature/ModelFeatureRenderer$Submit#<init>` | method | 1 |
| `net/minecraft/client/renderer/feature/ModelFeatureRenderer$Submit#lightCoords` | method | 1 |
| `net/minecraft/client/renderer/feature/ModelFeatureRenderer$Submit#state` | method | 1 |
| `net/minecraft/client/renderer/feature/RenderTypeFeatureRenderer#getVertexBuilder` | method | 3 |
| `net/minecraft/client/renderer/rendertype/RenderSetup#textures` | field | 1 |
| `net/minecraft/client/renderer/rendertype/RenderSetup$TextureBinding#location` | method | 1 |
| `net/minecraft/client/renderer/rendertype/RenderType#isOutline` | method | 1 |
| `net/minecraft/client/renderer/rendertype/RenderTypes#armorCutoutNoCull` | method | 2 |
| `net/minecraft/client/renderer/rendertype/RenderTypes#armorCutoutNoCullGlint` | method | 1 |
| `net/minecraft/client/renderer/rendertype/RenderTypes#beaconBeam` | method | 2 |
| `net/minecraft/client/renderer/rendertype/RenderTypes#endGateway` | method | 1 |
| `net/minecraft/client/renderer/rendertype/RenderTypes#energySwirl` | method | 1 |
| `net/minecraft/client/renderer/rendertype/RenderTypes#entityCutout` | method | 2 |
| `net/minecraft/client/renderer/rendertype/RenderTypes#entityCutoutCull` | method | 1 |
| `net/minecraft/client/renderer/rendertype/RenderTypes#entityCutoutDissolve` | method | 1 |
| `net/minecraft/client/renderer/rendertype/RenderTypes#entityCutoutNoCullZOffset` | method | 1 |
| `net/minecraft/client/renderer/rendertype/RenderTypes#entityCutoutZOffset` | method | 1 |
| `net/minecraft/client/renderer/rendertype/RenderTypes#entityDecal` | method | 1 |
| `net/minecraft/client/renderer/rendertype/RenderTypes#entityNoOutline` | method | 1 |
| `net/minecraft/client/renderer/rendertype/RenderTypes#entityShadow` | method | 1 |
| `net/minecraft/client/renderer/rendertype/RenderTypes#entitySmoothCutout` | method | 2 |
| `net/minecraft/client/renderer/rendertype/RenderTypes#entitySolid` | method | 3 |
| `net/minecraft/client/renderer/rendertype/RenderTypes#entitySolidZOffsetForward` | method | 1 |
| `net/minecraft/client/renderer/rendertype/RenderTypes#entityTranslucent` | method | 5 |
| `net/minecraft/client/renderer/rendertype/RenderTypes#entityTranslucentCullItemTarget` | method | 1 |
| `net/minecraft/client/renderer/rendertype/RenderTypes#entityTranslucentEmissive` | method | 1 |
| `net/minecraft/client/renderer/rendertype/RenderTypes#eyes` | method | 3 |
| `net/minecraft/client/renderer/rendertype/RenderTypes#itemEntityTranslucentCull` | method | 1 |
| `net/minecraft/client/renderer/rendertype/RenderTypes#outline` | method | 1 |
| `net/minecraft/client/renderer/special/ChestSpecialRenderer#submit` | method | 2 |
| `net/minecraft/client/renderer/state/GameRenderState#levelRenderState` | field | 1 |
| `net/minecraft/client/renderer/state/gui/pip/GuiEntityRenderState#renderState` | method | 1 |
| `net/minecraft/client/renderer/state/level/LevelRenderState#skyRenderState` | field | 1 |
| `net/minecraft/client/renderer/state/level/SkyRenderState#moonPhase` | field | 1 |
| `net/minecraft/client/renderer/texture/DynamicTexture#<init>` | method | 1 |
| `net/minecraft/client/renderer/texture/DynamicTexture#getPixels` | method | 2 |
| `net/minecraft/client/renderer/texture/DynamicTexture#setPixels` | method | 1 |
| `net/minecraft/client/renderer/texture/OverlayTexture#NO_OVERLAY` | field | 4 |
| `net/minecraft/client/renderer/texture/SkinTextureDownloader#<init>` | method | 1 |
| `net/minecraft/client/renderer/texture/SkinTextureDownloader#downloadSkin` | method | 1 |
| `net/minecraft/client/renderer/texture/SkinTextureDownloader#setNoAlpha` | method | 2 |
| `net/minecraft/client/renderer/texture/SpriteContents#<init>` | method | 1 |
| `net/minecraft/client/renderer/texture/SpriteContents#close` | method | 1 |
| `net/minecraft/client/renderer/texture/SpriteContents#height` | method | 1 |
| `net/minecraft/client/renderer/texture/SpriteContents#name` | method | 1 |
| `net/minecraft/client/renderer/texture/SpriteContents#width` | method | 1 |
| `net/minecraft/client/renderer/texture/TextureAtlasSprite#<init>` | method | 1 |
| `net/minecraft/client/renderer/texture/TextureAtlasSprite#atlasLocation` | method | 1 |
| `net/minecraft/client/renderer/texture/TextureAtlasSprite#contents` | method | 1 |
| `net/minecraft/client/renderer/texture/TextureAtlasSprite#getU` | method | 1 |
| `net/minecraft/client/renderer/texture/TextureAtlasSprite#getU0` | method | 1 |
| `net/minecraft/client/renderer/texture/TextureAtlasSprite#getU1` | method | 1 |
| `net/minecraft/client/renderer/texture/TextureAtlasSprite#getV` | method | 1 |
| `net/minecraft/client/renderer/texture/TextureAtlasSprite#getV0` | method | 1 |
| `net/minecraft/client/renderer/texture/TextureAtlasSprite#getV1` | method | 1 |
| `net/minecraft/client/renderer/texture/TextureManager#getTexture` | method | 4 |
| `net/minecraft/client/renderer/texture/TextureManager#register` | method | 1 |
| `net/minecraft/client/renderer/texture/TextureManager#release` | method | 1 |
| `net/minecraft/client/resources/SkinManager#get` | method | 2 |
| `net/minecraft/client/resources/SkinManager#skinTextures` | field | 2 |
| `net/minecraft/client/resources/SkinManager$TextureCache#root` | field | 2 |
| `net/minecraft/client/resources/metadata/animation/AnimationMetadataSection#TYPE` | field | 1 |
| `net/minecraft/client/resources/metadata/animation/FrameSize#<init>` | method | 1 |
| `net/minecraft/client/resources/metadata/animation/FrameSize#height` | method | 1 |
| `net/minecraft/client/resources/metadata/animation/FrameSize#width` | method | 1 |
| `net/minecraft/core/BlockPos#<init>` | method | 1 |
| `net/minecraft/core/BlockPos#ZERO` | field | 1 |
| `net/minecraft/core/BlockPos#asLong` | method | 2 |
| `net/minecraft/core/BlockPos#below` | method | 1 |
| `net/minecraft/core/BlockPos#containing` | method | 1 |
| `net/minecraft/core/BlockPos#equals` | method | 1 |
| `net/minecraft/core/BlockPos#getX` | method | 1 |
| `net/minecraft/core/BlockPos#getY` | method | 1 |
| `net/minecraft/core/BlockPos#getZ` | method | 1 |
| `net/minecraft/core/BlockPos$MutableBlockPos#<init>` | method | 4 |
| `net/minecraft/core/BlockPos$MutableBlockPos#above` | method | 1 |
| `net/minecraft/core/BlockPos$MutableBlockPos#getY` | method | 4 |
| `net/minecraft/core/BlockPos$MutableBlockPos#move` | method | 4 |
| `net/minecraft/core/BlockPos$MutableBlockPos#set` | method | 4 |
| `net/minecraft/core/ClientAsset$DownloadedTexture#url` | method | 2 |
| `net/minecraft/core/ClientAsset$Texture#texturePath` | method | 2 |
| `net/minecraft/core/DefaultedRegistry#forEach` | method | 1 |
| `net/minecraft/core/DefaultedRegistry#getKey` | method | 1 |
| `net/minecraft/core/DefaultedRegistry#getResourceKey` | method | 1 |
| `net/minecraft/core/Direction#EAST` | field | 1 |
| `net/minecraft/core/Direction#NORTH` | field | 1 |
| `net/minecraft/core/Direction#SOUTH` | field | 1 |
| `net/minecraft/core/Direction#WEST` | field | 1 |
| `net/minecraft/core/Direction#get2DDataValue` | method | 1 |
| `net/minecraft/core/Holder#tags` | method | 1 |
| `net/minecraft/core/Holder#toString` | method | 1 |
| `net/minecraft/core/Holder#unwrapKey` | method | 2 |
| `net/minecraft/core/Holder#value` | method | 2 |
| `net/minecraft/core/Registry#getKey` | method | 1 |
| `net/minecraft/core/Registry#getResourceKey` | method | 1 |
| `net/minecraft/core/Vec3i#ZERO` | field | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#BLOCK_ENTITY_TYPE` | field | 2 |
| `net/minecraft/core/registries/BuiltInRegistries#ENTITY_TYPE` | field | 2 |
| `net/minecraft/nbt/CollectionTag#get` | method | 1 |
| `net/minecraft/nbt/CollectionTag#iterator` | method | 1 |
| `net/minecraft/nbt/CollectionTag#stream` | method | 1 |
| `net/minecraft/nbt/CompoundTag#<init>` | method | 1 |
| `net/minecraft/nbt/CompoundTag#contains` | method | 1 |
| `net/minecraft/nbt/CompoundTag#get` | method | 1 |
| `net/minecraft/nbt/CompoundTag#isEmpty` | method | 1 |
| `net/minecraft/nbt/CompoundTag#putString` | method | 1 |
| `net/minecraft/nbt/NumericTag#asNumber` | method | 1 |
| `net/minecraft/nbt/Tag#asString` | method | 1 |
| `net/minecraft/nbt/Tag#toString` | method | 1 |
| `net/minecraft/network/chat/CommonComponents#GUI_BACK` | field | 2 |
| `net/minecraft/network/chat/CommonComponents#GUI_CANCEL` | field | 1 |
| `net/minecraft/network/chat/CommonComponents#GUI_DONE` | field | 2 |
| `net/minecraft/network/chat/CommonComponents#GUI_NO` | field | 3 |
| `net/minecraft/network/chat/CommonComponents#GUI_YES` | field | 3 |
| `net/minecraft/network/chat/CommonComponents#OPTION_OFF` | field | 3 |
| `net/minecraft/network/chat/CommonComponents#OPTION_ON` | field | 2 |
| `net/minecraft/network/chat/CommonComponents#optionStatus` | method | 1 |
| `net/minecraft/network/chat/Component#getContents` | method | 1 |
| `net/minecraft/network/chat/Component#getString` | method | 24 |
| `net/minecraft/network/chat/Component#getVisualOrderText` | method | 2 |
| `net/minecraft/network/chat/Component#literal` | method | 1 |
| `net/minecraft/network/chat/Component#nullToEmpty` | method | 13 |
| `net/minecraft/network/chat/Component#translatable` | method | 4 |
| `net/minecraft/network/chat/FormattedText#getString` | method | 1 |
| `net/minecraft/network/chat/MutableComponent#create` | method | 1 |
| `net/minecraft/network/chat/Style#EMPTY` | field | 1 |
| `net/minecraft/network/chat/contents/PlainTextContents$LiteralContents#<init>` | method | 1 |
| `net/minecraft/network/chat/contents/PlainTextContents$LiteralContents#text` | method | 1 |
| `net/minecraft/resources/Identifier#equals` | method | 8 |
| `net/minecraft/resources/Identifier#fromNamespaceAndPath` | method | 1 |
| `net/minecraft/resources/Identifier#getNamespace` | method | 7 |
| `net/minecraft/resources/Identifier#getPath` | method | 9 |
| `net/minecraft/resources/Identifier#isValidPath` | method | 1 |
| `net/minecraft/resources/Identifier#parse` | method | 1 |
| `net/minecraft/resources/Identifier#toString` | method | 7 |
| `net/minecraft/resources/ResourceKey#identifier` | method | 2 |
| `net/minecraft/server/packs/PackResources#packId` | method | 1 |
| `net/minecraft/server/packs/VanillaPackResources#fullResources` | method | 1 |
| `net/minecraft/server/packs/repository/PackRepository#getSelectedIds` | method | 1 |
| `net/minecraft/server/packs/resources/Resource#<init>` | method | 1 |
| `net/minecraft/server/packs/resources/Resource#metadata` | method | 1 |
| `net/minecraft/server/packs/resources/Resource#open` | method | 2 |
| `net/minecraft/server/packs/resources/Resource#source` | method | 1 |
| `net/minecraft/server/packs/resources/Resource#sourcePackId` | method | 4 |
| `net/minecraft/server/packs/resources/ResourceManager#getResource` | method | 7 |
| `net/minecraft/server/packs/resources/ResourceManager#getResourceStack` | method | 1 |
| `net/minecraft/server/packs/resources/ResourceManager#listPacks` | method | 1 |
| `net/minecraft/server/packs/resources/ResourceMetadata#getSection` | method | 1 |
| `net/minecraft/server/packs/resources/ResourceMetadata#getTypedSections` | method | 1 |
| `net/minecraft/tags/ItemTags#LEG_ARMOR` | field | 1 |
| `net/minecraft/tags/ItemTags#WOOL_CARPETS` | field | 1 |
| `net/minecraft/tags/TagKey#location` | method | 1 |
| `net/minecraft/util/ARGB#color` | method | 1 |
| `net/minecraft/util/ARGB#fromABGR` | method | 1 |
| `net/minecraft/util/ARGB#toABGR` | method | 1 |
| `net/minecraft/util/FileUtil#createDirectoriesSafe` | method | 1 |
| `net/minecraft/util/LightCoordsUtil#getLightCoords` | method | 1 |
| `net/minecraft/util/Mth#isMultipleOf` | method | 1 |
| `net/minecraft/util/Mth#sqrt` | method | 1 |
| `net/minecraft/util/ProblemReporter#DISCARDING` | field | 1 |
| `net/minecraft/world/Difficulty#getId` | method | 2 |
| `net/minecraft/world/Nameable#getCustomName` | method | 1 |
| `net/minecraft/world/Nameable#hasCustomName` | method | 1 |
| `net/minecraft/world/effect/MobEffects#BLINDNESS` | field | 1 |
| `net/minecraft/world/entity/Entity#<init>` | method | 3 |
| `net/minecraft/world/entity/Entity#getLightProbePosition` | method | 1 |
| `net/minecraft/world/entity/Entity#getType` | method | 1 |
| `net/minecraft/world/entity/Entity#getUUID` | method | 1 |
| `net/minecraft/world/entity/Entity#getVehicle` | method | 1 |
| `net/minecraft/world/entity/Entity#isAlliedTo` | method | 1 |
| `net/minecraft/world/entity/Entity#level` | method | 1 |
| `net/minecraft/world/entity/Entity#setUUID` | method | 1 |
| `net/minecraft/world/entity/EntitySpawnReason#COMMAND` | field | 1 |
| `net/minecraft/world/entity/EntityType#getDescriptionId` | method | 3 |
| `net/minecraft/world/entity/EntityType#getKey` | method | 1 |
| `net/minecraft/world/entity/EntityTypes#PARROT` | field | 1 |
| `net/minecraft/world/entity/EntityTypes#PLAYER` | field | 1 |
| `net/minecraft/world/entity/EquipmentSlot#LEGS` | field | 1 |
| `net/minecraft/world/entity/EquipmentSlot#getIndex` | method | 1 |
| `net/minecraft/world/entity/EquipmentSlot#getType` | method | 1 |
| `net/minecraft/world/entity/EquipmentSlot$Type#ANIMAL_ARMOR` | field | 1 |
| `net/minecraft/world/entity/EquipmentSlot$Type#HAND` | field | 1 |
| `net/minecraft/world/entity/EquipmentSlot$Type#HUMANOID_ARMOR` | field | 1 |
| `net/minecraft/world/entity/Leashable$LeashData#delayedLeashInfo` | field | 1 |
| `net/minecraft/world/entity/Leashable$LeashData#leashHolder` | field | 1 |
| `net/minecraft/world/entity/LivingEntity#getBbHeight` | method | 2 |
| `net/minecraft/world/entity/LivingEntity#getBbWidth` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#getHealth` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#getMaxHealth` | method | 2 |
| `net/minecraft/world/entity/LivingEntity#getScale` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#getSpeed` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#getXRot` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#getYRot` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#hasPose` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#isBaby` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#isRemoved` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#lastEquipmentItems` | field | 1 |
| `net/minecraft/world/entity/LivingEntity#setXRot` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#setYRot` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#tickCount` | field | 1 |
| `net/minecraft/world/entity/LivingEntity#yBodyRot` | field | 1 |
| `net/minecraft/world/entity/LivingEntity#yHeadRot` | field | 1 |
| `net/minecraft/world/entity/LivingEntity#yHeadRotO` | field | 1 |
| `net/minecraft/world/entity/Mob#addAdditionalSaveData` | method | 1 |
| `net/minecraft/world/entity/Mob#writeLeashData` | method | 1 |
| `net/minecraft/world/entity/NeutralMob#isAngry` | method | 1 |
| `net/minecraft/world/entity/Pose#SLEEPING` | field | 1 |
| `net/minecraft/world/entity/Pose#STANDING` | field | 1 |
| `net/minecraft/world/entity/animal/cow/MushroomCow$Variant#BROWN` | field | 1 |
| `net/minecraft/world/entity/animal/cow/MushroomCow$Variant#RED` | field | 1 |
| `net/minecraft/world/entity/animal/cow/MushroomCow$Variant#equals` | method | 1 |
| `net/minecraft/world/entity/animal/equine/AbstractHorse#getJumpBoostPower` | method | 1 |
| `net/minecraft/world/entity/animal/equine/Llama#getBodyArmorItem` | method | 1 |
| `net/minecraft/world/entity/animal/equine/Llama#getInventoryColumns` | method | 1 |
| `net/minecraft/world/entity/animal/feline/Cat#getCollarColor` | method | 1 |
| `net/minecraft/world/entity/animal/fish/TropicalFish#getBaseColor` | method | 1 |
| `net/minecraft/world/entity/animal/goat/Goat#isScreamingGoat` | method | 1 |
| `net/minecraft/world/entity/animal/golem/IronGolem#isPlayerCreated` | method | 1 |
| `net/minecraft/world/entity/animal/panda/Panda#getHiddenGene` | method | 1 |
| `net/minecraft/world/entity/animal/panda/Panda$Gene#getSerializedName` | method | 1 |
| `net/minecraft/world/entity/animal/sheep/Sheep#getColor` | method | 1 |
| `net/minecraft/world/entity/animal/squid/GlowSquid#aiStep` | method | 1 |
| `net/minecraft/world/entity/animal/squid/Squid#<init>` | method | 1 |
| `net/minecraft/world/entity/animal/wolf/Wolf#getCollarColor` | method | 1 |
| `net/minecraft/world/entity/decoration/painting/Painting#getBlockX` | method | 1 |
| `net/minecraft/world/entity/decoration/painting/Painting#getBlockZ` | method | 1 |
| `net/minecraft/world/entity/decoration/painting/Painting#getDirection` | method | 1 |
| `net/minecraft/world/entity/decoration/painting/Painting#getVariant` | method | 1 |
| `net/minecraft/world/entity/decoration/painting/Painting#getX` | method | 1 |
| `net/minecraft/world/entity/decoration/painting/Painting#getY` | method | 1 |
| `net/minecraft/world/entity/decoration/painting/Painting#getZ` | method | 1 |
| `net/minecraft/world/entity/decoration/painting/Painting#level` | method | 1 |
| `net/minecraft/world/entity/decoration/painting/PaintingVariant#height` | method | 1 |
| `net/minecraft/world/entity/decoration/painting/PaintingVariant#width` | method | 1 |
| `net/minecraft/world/entity/monster/Blaze#aiStep` | method | 1 |
| `net/minecraft/world/entity/monster/Blaze#isOnFire` | method | 1 |
| `net/minecraft/world/entity/monster/Creeper#isPowered` | method | 1 |
| `net/minecraft/world/entity/monster/Enderman#isCreepy` | method | 1 |
| `net/minecraft/world/entity/monster/Guardian#getActiveAttackTarget` | method | 1 |
| `net/minecraft/world/entity/monster/Monster#<init>` | method | 1 |
| `net/minecraft/world/entity/monster/Phantom#getPhantomSize` | method | 1 |
| `net/minecraft/world/entity/monster/Phantom#tick` | method | 1 |
| `net/minecraft/world/entity/monster/Shulker#getColor` | method | 1 |
| `net/minecraft/world/entity/monster/cubemob/AbstractCubeMob#getSize` | method | 1 |
| `net/minecraft/world/entity/monster/illager/SpellcasterIllager#isCastingSpell` | method | 1 |
| `net/minecraft/world/entity/monster/illager/Vindicator#isAggressive` | method | 1 |
| `net/minecraft/world/entity/npc/villager/VillagerData#level` | method | 1 |
| `net/minecraft/world/entity/npc/villager/VillagerData#profession` | method | 1 |
| `net/minecraft/world/entity/npc/villager/VillagerDataHolder#getVillagerData` | method | 1 |
| `net/minecraft/world/entity/player/Inventory#getItem` | method | 1 |
| `net/minecraft/world/entity/player/Player#getInventory` | method | 1 |
| `net/minecraft/world/entity/player/Player#getName` | method | 1 |
| `net/minecraft/world/entity/player/Player#getUUID` | method | 1 |
| `net/minecraft/world/entity/player/Player#interactOn` | method | 1 |
| `net/minecraft/world/entity/player/Player#isCreative` | method | 1 |
| `net/minecraft/world/entity/player/Player#level` | method | 1 |
| `net/minecraft/world/entity/player/PlayerModelType#WIDE` | field | 1 |
| `net/minecraft/world/entity/player/PlayerSkin#body` | method | 2 |
| `net/minecraft/world/entity/player/PlayerSkin#model` | method | 1 |
| `net/minecraft/world/item/BlockItem#getBlock` | method | 1 |
| `net/minecraft/world/item/DyeColor#getName` | method | 1 |
| `net/minecraft/world/item/Item#toString` | method | 1 |
| `net/minecraft/world/item/ItemStack#getHoverName` | method | 1 |
| `net/minecraft/world/item/ItemStack#getItem` | method | 2 |
| `net/minecraft/world/item/ItemStack#is` | method | 2 |
| `net/minecraft/world/item/ItemStack#isEmpty` | method | 1 |
| `net/minecraft/world/item/component/ResolvableProfile#partialProfile` | method | 1 |
| `net/minecraft/world/level/BaseSpawner#getOrCreateDisplayEntity` | method | 1 |
| `net/minecraft/world/level/GameType#getId` | method | 1 |
| `net/minecraft/world/level/Level#addParticle` | method | 3 |
| `net/minecraft/world/level/Level#dimensionTypeRegistration` | method | 1 |
| `net/minecraft/world/level/Level#getBiome` | method | 3 |
| `net/minecraft/world/level/Level#getBlockState` | method | 5 |
| `net/minecraft/world/level/Level#getBrightness` | method | 1 |
| `net/minecraft/world/level/Level#getDifficulty` | method | 2 |
| `net/minecraft/world/level/Level#getEntities` | method | 1 |
| `net/minecraft/world/level/Level#getGameTime` | method | 2 |
| `net/minecraft/world/level/Level#getLevelData` | method | 1 |
| `net/minecraft/world/level/Level#getOverworldClockTime` | method | 1 |
| `net/minecraft/world/level/Level#getSeaLevel` | method | 1 |
| `net/minecraft/world/level/Level#isRaining` | method | 1 |
| `net/minecraft/world/level/Level#isThundering` | method | 1 |
| `net/minecraft/world/level/LightLayer#BLOCK` | field | 1 |
| `net/minecraft/world/level/LightLayer#SKY` | field | 1 |
| `net/minecraft/world/level/MoonPhase#index` | method | 1 |
| `net/minecraft/world/level/biome/Biome#getHeightAdjustedTemperature` | method | 1 |
| `net/minecraft/world/level/block/Block#defaultBlockState` | method | 6 |
| `net/minecraft/world/level/block/Blocks#CHEST` | field | 1 |
| `net/minecraft/world/level/block/Blocks#SPAWNER` | field | 5 |
| `net/minecraft/world/level/block/ShulkerBoxBlock#getColor` | method | 1 |
| `net/minecraft/world/level/block/SignBlock#type` | method | 1 |
| `net/minecraft/world/level/block/SkullBlock#ROTATION` | field | 1 |
| `net/minecraft/world/level/block/WoolCarpetBlock#getColor` | method | 1 |
| `net/minecraft/world/level/block/entity/BlockEntity#<init>` | method | 1 |
| `net/minecraft/world/level/block/entity/BlockEntity#getBlockPos` | method | 2 |
| `net/minecraft/world/level/block/entity/BlockEntity#getBlockState` | method | 2 |
| `net/minecraft/world/level/block/entity/BlockEntity#getLevel` | method | 1 |
| `net/minecraft/world/level/block/entity/BlockEntity#getType` | method | 2 |
| `net/minecraft/world/level/block/entity/ChestBlockEntity#<init>` | method | 1 |
| `net/minecraft/world/level/block/entity/DecoratedPotBlockEntity#getDecorations` | method | 1 |
| `net/minecraft/world/level/block/entity/PotDecorations#back` | method | 1 |
| `net/minecraft/world/level/block/entity/PotDecorations#front` | method | 1 |
| `net/minecraft/world/level/block/entity/PotDecorations#left` | method | 1 |
| `net/minecraft/world/level/block/entity/PotDecorations#right` | method | 1 |
| `net/minecraft/world/level/block/entity/ShulkerBoxBlockEntity#getBlockState` | method | 1 |
| `net/minecraft/world/level/block/entity/SignBlockEntity#getBlockState` | method | 1 |
| `net/minecraft/world/level/block/entity/SkullBlockEntity#getBlockState` | method | 1 |
| `net/minecraft/world/level/block/entity/SkullBlockEntity#getOwnerProfile` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#getBlock` | method | 2 |
| `net/minecraft/world/level/block/state/BlockState#getValue` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#getValues` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#hashCode` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#isAir` | method | 3 |
| `net/minecraft/world/level/block/state/BlockState#isSolidRender` | method | 2 |
| `net/minecraft/world/level/block/state/properties/Property#getName` | method | 1 |
| `net/minecraft/world/level/block/state/properties/Property$Value#property` | method | 1 |
| `net/minecraft/world/level/block/state/properties/Property$Value#valueName` | method | 1 |
| `net/minecraft/world/level/block/state/properties/WoodType#name` | method | 1 |
| `net/minecraft/world/level/dimension/BuiltinDimensionTypes#END` | field | 1 |
| `net/minecraft/world/level/dimension/BuiltinDimensionTypes#NETHER` | field | 1 |
| `net/minecraft/world/level/dimension/BuiltinDimensionTypes#OVERWORLD` | field | 1 |
| `net/minecraft/world/level/storage/LevelData#isHardcore` | method | 1 |
| `net/minecraft/world/level/storage/TagValueOutput#buildResult` | method | 1 |
| `net/minecraft/world/level/storage/TagValueOutput#createWithoutContext` | method | 1 |
| `net/minecraft/world/phys/AABB#inflate` | method | 1 |
| `net/minecraft/world/phys/Vec3#ZERO` | field | 1 |
| `net/minecraft/world/phys/Vec3#horizontalDistance` | method | 1 |
| `net/minecraft/world/scores/Team#getName` | method | 1 |

</details>

