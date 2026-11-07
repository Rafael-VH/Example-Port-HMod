# Java translated to Rust

Translated 97 of 961 methods (1 more is done another way in Horizon) and 52 of 95 static fields to Rust; 0 of 1 Fabric event handlers. It's in `runtime.rs`, under `pub mod java`; code that calls the game is in `game_side.rs`.

Each method is translated whole or not at all. A method that can't be (it uses Minecraft, a lambda, null...) is left out with its reason, and so is every method that calls it.

## Why methods were left out

| Cause | Methods | For example |
|---|---|---|
| Calls or uses other code that isn't translated | 603 | `EMFConfig.getRenderModeFor(1)`: its class isn't translated (it extends `TConfig` (traben.tconfig)) |
| Other Java this translator doesn't handle yet | 186 | `EMFConfig.reprintLoadingErrors(1)`: it uses `Exception` |
| Uses Minecraft classes (needs Horizon's game API) | 38 | `EMF.getConfigScreen(1)`: it uses Minecraft's `Screen` |
| Uses other libraries | 26 | `EMF.config(0)`: it uses `TConfigHandler` (traben.tconfig) |
| Calls the game where Horizon has no helper yet | 5 | `ASMHelper.notPausedNotValidation(0)`: it calls Minecraft's Minecraft.isPaused (no Horizon helper for it yet) |
| Uses Java collections other than List (Map, Set...) | 3 | `EMF.randomQuip(0)`: it creates `Random` (java.util) |
| Shares one object between names (Java references) | 2 | `EMFAnimationHandler.addAnimLineData(1)`: it stores a second reference to an object (Rust would need it copied or shared) |
| Done another way in Horizon | 1 | `EMFModMenu.getModConfigScreenFactory(0)`: done another way in Horizon: the mod's page in Options > Mods (its settings buttons) |

## Fabric event handlers

| Handler | Hook | Result |
|---|---|---|
| traben/entity_model_features/EMFInit.java:9 | `init` | not translated: it calls EMF.init, which isn't translated |

## Every class


### `traben/entity_model_features/models/animation/math/EMFMath.java`: couldn't be read: expected `;` but found `>` (line 58)


### `traben/entity_model_features/models/parts/EMFModelPartWithState.java`: couldn't be read: expected `;` but found `.` (line 17)


### `EMFConfig`: it extends `TConfig` (traben.tconfig)

- [ ] `EMFConfig.getRenderModeFor(1)`: its class isn't translated (it extends `TConfig` (traben.tconfig)) (traben/entity_model_features/config/EMFConfig.java:105)
- [ ] `EMFConfig.getRenderModeFor(1)`: its class isn't translated (it extends `TConfig` (traben.tconfig)) (traben/entity_model_features/config/EMFConfig.java:109)
- [ ] `EMFConfig.getPhysicsModModeFor(1)`: its class isn't translated (it extends `TConfig` (traben.tconfig)) (traben/entity_model_features/config/EMFConfig.java:116)
- [ ] `EMFConfig.getVanillaHologramModeFor(1)`: its class isn't translated (it extends `TConfig` (traben.tconfig)) (traben/entity_model_features/config/EMFConfig.java:128)
- [ ] `EMFConfig.getTypeString(1)`: it uses EMFEntity, which isn't translated (it's an interface) (traben/entity_model_features/config/EMFConfig.java:133)
- [ ] `EMFConfig.isModelDisabled(1)`: its class isn't translated (it extends `TConfig` (traben.tconfig)) (traben/entity_model_features/config/EMFConfig.java:142)
- [ ] `EMFConfig.getGUIOptions(0)`: its class isn't translated (it extends `TConfig` (traben.tconfig)) (traben/entity_model_features/config/EMFConfig.java:146)
- [ ] `EMFConfig.reprintLoadingErrors(1)`: it uses `Exception` (traben/entity_model_features/config/EMFConfig.java:386)
- [ ] `EMFConfig.getMathInfo(0)`: its class isn't translated (it extends `TConfig` (traben.tconfig)) (traben/entity_model_features/config/EMFConfig.java:404)
- [ ] `EMFConfig.getModelSettings(0)`: its class isn't translated (it extends `TConfig` (traben.tconfig)) (traben/entity_model_features/config/EMFConfig.java:441)
- [ ] `EMFConfig.getExport(2)`: its class isn't translated (it extends `TConfig` (traben.tconfig)) (traben/entity_model_features/config/EMFConfig.java:510)
- [ ] `EMFConfig.getmappings(1)`: its class isn't translated (it extends `TConfig` (traben.tconfig)) (traben/entity_model_features/config/EMFConfig.java:547)
- [ ] `EMFConfig.getEntitySettings(0)`: its class isn't translated (it extends `TConfig` (traben.tconfig)) (traben/entity_model_features/config/EMFConfig.java:571)
- [ ] `EMFConfig.addEntityConfigs(2)`: its class isn't translated (it extends `TConfig` (traben.tconfig)) (traben/entity_model_features/config/EMFConfig.java:594)
- [ ] `EMFConfig.getModIcon(0)`: its class isn't translated (it extends `TConfig` (traben.tconfig)) (traben/entity_model_features/config/EMFConfig.java:609)

### `EMFConfig.CustomEntityModelSupportMode`: its constants carry values

- [ ] `EMFConfig.CustomEntityModelSupportMode.CustomEntityModelSupportMode(1)`: its class isn't translated (its constants carry values) (traben/entity_model_features/config/EMFConfig.java:621)
- [ ] `EMFConfig.CustomEntityModelSupportMode.toString(0)`: its class isn't translated (its constants carry values) (traben/entity_model_features/config/EMFConfig.java:625)
- [ ] `EMFConfig.CustomEntityModelSupportMode.allowsKnown(0)`: its class isn't translated (its constants carry values) (traben/entity_model_features/config/EMFConfig.java:630)
- [ ] `EMFConfig.CustomEntityModelSupportMode.allowsUnknowns(0)`: its class isn't translated (its constants carry values) (traben/entity_model_features/config/EMFConfig.java:634)
- [ ] `EMFConfig.CustomEntityModelSupportMode.isOn(0)`: its class isn't translated (its constants carry values) (traben/entity_model_features/config/EMFConfig.java:638)
- [ ] `EMFConfig.CustomEntityModelSupportMode.allowsAll(0)`: its class isn't translated (its constants carry values) (traben/entity_model_features/config/EMFConfig.java:642)

### `EMFConfig.ModelPrintMode`: its constants carry values

- [ ] `EMFConfig.ModelPrintMode.ModelPrintMode(1)`: its class isn't translated (its constants carry values) (traben/entity_model_features/config/EMFConfig.java:657)
- [ ] `EMFConfig.ModelPrintMode.doesJems(0)`: its class isn't translated (its constants carry values) (traben/entity_model_features/config/EMFConfig.java:661)
- [ ] `EMFConfig.ModelPrintMode.doesAll(0)`: its class isn't translated (its constants carry values) (traben/entity_model_features/config/EMFConfig.java:665)
- [ ] `EMFConfig.ModelPrintMode.doesLog(0)`: its class isn't translated (its constants carry values) (traben/entity_model_features/config/EMFConfig.java:669)
- [ ] `EMFConfig.ModelPrintMode.toString(0)`: its class isn't translated (its constants carry values) (traben/entity_model_features/config/EMFConfig.java:673)

### `EMFConfig.ModelRootRenderer`: it implements `Renderable` (traben.tconfig.gui.TConfigScreenList)

- [ ] `EMFConfig.ModelRootRenderer.ModelRootRenderer(1)`: its class isn't translated (it implements `Renderable` (traben.tconfig.gui.TConfigScreenList)) (traben/entity_model_features/config/EMFConfig.java:684)
- [ ] `EMFConfig.ModelRootRenderer.canRender(0)`: its class isn't translated (it implements `Renderable` (traben.tconfig.gui.TConfigScreenList)) (traben/entity_model_features/config/EMFConfig.java:688)
- [ ] `EMFConfig.ModelRootRenderer.render(3)`: its class isn't translated (it implements `Renderable` (traben.tconfig.gui.TConfigScreenList)) (traben/entity_model_features/config/EMFConfig.java:702)
- [ ] `EMFConfig.ModelRootRenderer.renderBoxes(3)`: its class isn't translated (it implements `Renderable` (traben.tconfig.gui.TConfigScreenList)) (traben/entity_model_features/config/EMFConfig.java:705)

### `EMFConfig.PhysicsModCompatChoice`


### `EMFConfig.RenderModeChoice`: its constants carry values

- [ ] `EMFConfig.RenderModeChoice.RenderModeChoice(1)`: its class isn't translated (its constants carry values) (traben/entity_model_features/config/EMFConfig.java:743)
- [ ] `EMFConfig.RenderModeChoice.toString(0)`: its class isn't translated (its constants carry values) (traben/entity_model_features/config/EMFConfig.java:747)
- [ ] `EMFConfig.RenderModeChoice.allowsRegularDraw(0)`: its class isn't translated (its constants carry values) (traben/entity_model_features/config/EMFConfig.java:752)

### `EMFConfig.VanillaModelRenderMode`


### `EMFModMenu`: it implements `ModMenuApi` (com.terraformersmc.modmenu.api)

- [ ] `EMFModMenu.getModConfigScreenFactory(0)`: done another way in Horizon: the mod's page in Options > Mods (its settings buttons) (traben/entity_model_features/config/EMFModMenu.java:11)

### `EMF`

- [x] `EMF.MOD_ID` (traben/entity_model_features/EMF.java:38)
- [x] `EMF.forgeHadLoadingError` (traben/entity_model_features/EMF.java:39)
- [ ] `EMF.testedForge`: it calls `ETF` (traben.entity_texture_features).isForge (traben/entity_model_features/EMF.java:40)
- [x] `EMF.tempDisableModelModifications` (traben/entity_model_features/EMF.java:41)
- [x] `EMF.isLoadingPhase` (traben/entity_model_features/EMF.java:42)
- [ ] `EMF.configHandler`: it uses `TConfigHandler` (traben.tconfig) (traben/entity_model_features/EMF.java:43)
- [ ] `EMF.config(0)`: it uses `TConfigHandler` (traben.tconfig) (traben/entity_model_features/EMF.java:45)
- [ ] `EMF.init(0)`: its body couldn't be read: an anonymous class (traben/entity_model_features/EMF.java:54)
- [ ] `EMF.getConfigScreen(1)`: it uses Minecraft's `Screen` (traben/entity_model_features/EMF.java:151)
- [ ] `EMF.getConfigScreen(2)`: it uses Minecraft's `Screen` (traben/entity_model_features/EMF.java:155)
- [ ] `EMF.testForForgeLoadingError(0)`: it uses EMF.testedForge, which isn't translated (it calls `ETF` (traben.entity_texture_features).isForge) (traben/entity_model_features/EMF.java:159)
- [ ] `EMF.randomQuip(0)`: it creates `Random` (java.util) (traben/entity_model_features/EMF.java:181)

### `EMFAnimationApi`: it's an interface

- [x] `EMFAnimationApi.getApiVersion(0)` (traben/entity_model_features/EMFAnimationApi.java:40)
- [ ] `EMFAnimationApi.getCurrentEntity(0)`: it uses EMFEntity, which isn't translated (it's an interface) (traben/entity_model_features/EMFAnimationApi.java:44)
- [ ] `EMFAnimationApi.registerSingletonAnimationVariable(4)`: it's overloaded with the same number of parameters (traben/entity_model_features/EMFAnimationApi.java:50)
- [ ] `EMFAnimationApi.registerSingletonAnimationVariable(4)`: it's overloaded with the same number of parameters (traben/entity_model_features/EMFAnimationApi.java:61)
- [ ] `EMFAnimationApi.registerSingletonAnimationVariable(4)`: it's overloaded with the same number of parameters (traben/entity_model_features/EMFAnimationApi.java:68)
- [ ] `EMFAnimationApi.registerUniqueAnimationVariableFactory(3)`: it calls VariableRegistry.getInstance, which isn't translated (traben/entity_model_features/EMFAnimationApi.java:79)
- [ ] `EMFAnimationApi.registerCustomFunctionFromStaticMethod(5)`: it uses `Method` (java.lang.reflect) (traben/entity_model_features/EMFAnimationApi.java:88)
- [ ] `EMFAnimationApi.registerCustomFunctionFactory(4)`: it uses MethodFactory, which isn't translated (it's an interface) (traben/entity_model_features/EMFAnimationApi.java:99)
- [ ] `EMFAnimationApi.emfEntityOf(1)`: it uses EMFEntity, which isn't translated (it's an interface) (traben/entity_model_features/EMFAnimationApi.java:110)
- [ ] `EMFAnimationApi.emfEntityOf(1)`: it uses EMFEntity, which isn't translated (it's an interface) (traben/entity_model_features/EMFAnimationApi.java:114)
- [x] `EMFAnimationApi.registerAnimationHook(1)` (traben/entity_model_features/EMFAnimationApi.java:118)
- [ ] `EMFAnimationApi.animateModelForEntity(3)`: it's overloaded with the same number of parameters (traben/entity_model_features/EMFAnimationApi.java:127)
- [ ] `EMFAnimationApi.animateModelForEntity(3)`: it's overloaded with the same number of parameters (traben/entity_model_features/EMFAnimationApi.java:133)
- [ ] `EMFAnimationApi.animateModelForState(3)`: it's overloaded with the same number of parameters (traben/entity_model_features/EMFAnimationApi.java:142)
- [ ] `EMFAnimationApi.animateModelForState(3)`: it's overloaded with the same number of parameters (traben/entity_model_features/EMFAnimationApi.java:146)
- [ ] `EMFAnimationApi.registerPauseCondition(1)`: it uses EMFEntity, which isn't translated (it's an interface) (traben/entity_model_features/EMFAnimationApi.java:168)
- [ ] `EMFAnimationApi.pauseAllCustomAnimationsForEntity(1)`: it uses EMFEntity, which isn't translated (it's an interface) (traben/entity_model_features/EMFAnimationApi.java:177)
- [ ] `EMFAnimationApi.resumeAllCustomAnimationsForEntity(1)`: it uses EMFEntity, which isn't translated (it's an interface) (traben/entity_model_features/EMFAnimationApi.java:186)
- [ ] `EMFAnimationApi.pauseCustomAnimationsForThesePartsOfEntity(2)`: it uses EMFEntity, which isn't translated (it's an interface) (traben/entity_model_features/EMFAnimationApi.java:196)
- [ ] `EMFAnimationApi.registerVanillaModelCondition(1)`: it uses EMFEntity, which isn't translated (it's an interface) (traben/entity_model_features/EMFAnimationApi.java:205)
- [ ] `EMFAnimationApi.lockEntityToVanillaModel(1)`: it uses EMFEntity, which isn't translated (it's an interface) (traben/entity_model_features/EMFAnimationApi.java:214)
- [ ] `EMFAnimationApi.unlockEntityToVanillaModel(1)`: it uses EMFEntity, which isn't translated (it's an interface) (traben/entity_model_features/EMFAnimationApi.java:223)
- [ ] `EMFAnimationApi.getCurrentEMFVariantOfModel(1)`: it uses Minecraft's `EntityModel` (traben/entity_model_features/EMFAnimationApi.java:232)
- [ ] `EMFAnimationApi.isModelAnimatedByEMF(1)`: it uses Minecraft's `EntityModel` (traben/entity_model_features/EMFAnimationApi.java:236)
- [ ] `EMFAnimationApi.isModelCustomizedByEMF(1)`: it uses Minecraft's `EntityModel` (traben/entity_model_features/EMFAnimationApi.java:240)
- [ ] `EMFAnimationApi.isModelPartCustomToEMF(1)`: it uses Minecraft's `ModelPart` (traben/entity_model_features/EMFAnimationApi.java:244)
- [ ] `EMFAnimationApi.isModelPartAnimatedByEMF(1)`: it uses Minecraft's `ModelPart` (traben/entity_model_features/EMFAnimationApi.java:248)
- [x] `EMFAnimationApi.registerAnimationFunction(4)` (traben/entity_model_features/EMFAnimationApi.java:252)
- [x] `EMFAnimationApi.registerAnimationBiFunction(4)` (traben/entity_model_features/EMFAnimationApi.java:257)
- [ ] `EMFAnimationApi.registerAnimationTriFunction(4)`: it uses `TriFunction` (org.apache.commons.lang3.function) (traben/entity_model_features/EMFAnimationApi.java:264)
- [x] `EMFAnimationApi.registerAnimationMultiFunction(4)` (traben/entity_model_features/EMFAnimationApi.java:271)
- [ ] `EMFAnimationApi.registerSingletonAnimationVariable(3)`: it's overloaded with the same number of parameters (traben/entity_model_features/EMFAnimationApi.java:278)
- [ ] `EMFAnimationApi.registerSingletonAnimationVariable(3)`: it's overloaded with the same number of parameters (traben/entity_model_features/EMFAnimationApi.java:284)
- [ ] `EMFAnimationApi.registerSingletonAnimationVariable(3)`: it's overloaded with the same number of parameters (traben/entity_model_features/EMFAnimationApi.java:289)
- [x] `EMFAnimationApi.registerAnimationFunction(3)` (traben/entity_model_features/EMFAnimationApi.java:295)
- [x] `EMFAnimationApi.registerAnimationBiFunction(3)` (traben/entity_model_features/EMFAnimationApi.java:300)
- [ ] `EMFAnimationApi.registerAnimationTriFunction(3)`: it uses `TriFunction` (org.apache.commons.lang3.function) (traben/entity_model_features/EMFAnimationApi.java:305)
- [x] `EMFAnimationApi.registerAnimationMultiFunction(3)` (traben/entity_model_features/EMFAnimationApi.java:310)
- [ ] `EMFAnimationApi.registerCustomFunctionFactory(3)`: it uses MethodFactory, which isn't translated (it's an interface) (traben/entity_model_features/EMFAnimationApi.java:315)
- [ ] `EMFAnimationApi.dependencyException(0)`: it uses `Exception` (traben/entity_model_features/EMFAnimationApi.java:320)
- [ ] `EMFAnimationApi.paramFail(1)`: it uses `Exception` (traben/entity_model_features/EMFAnimationApi.java:324)

### `EMFAnimationApi.EMFAnimationHook`

- [ ] `EMFAnimationApi.EMFAnimationHook.onAnimationStart(2)`: it uses AnimationContext, which isn't translated (it uses EMFEntityRenderState, which isn't translated (it's an interface)) (traben/entity_model_features/EMFAnimationApi.java:330)
- [ ] `EMFAnimationApi.EMFAnimationHook.onAnimationEnd(2)`: it uses AnimationContext, which isn't translated (it uses EMFEntityRenderState, which isn't translated (it's an interface)) (traben/entity_model_features/EMFAnimationApi.java:334)
- [ ] `EMFAnimationApi.EMFAnimationHook.onBipedPoseCopyStart(3)`: it uses Minecraft's `HumanoidModel` (traben/entity_model_features/EMFAnimationApi.java:337)
- [ ] `EMFAnimationApi.EMFAnimationHook.onBipedPoseCopyEnd(3)`: it uses Minecraft's `HumanoidModel` (traben/entity_model_features/EMFAnimationApi.java:341)

### `EMFAnimationApi.EMFAnimationHook.AnimationContext`: it uses EMFEntityRenderState, which isn't translated (it's an interface)

- [ ] `EMFAnimationApi.EMFAnimationHook.AnimationContext.AnimationContext(5)`: its class isn't translated (it uses EMFEntityRenderState, which isn't translated (it's an interface)) (traben/entity_model_features/EMFAnimationApi.java:344)
- [ ] `EMFAnimationApi.EMFAnimationHook.AnimationContext.activeState(0)`: its class isn't translated (it uses EMFEntityRenderState, which isn't translated (it's an interface)) (traben/entity_model_features/EMFAnimationApi.java:344)
- [ ] `EMFAnimationApi.EMFAnimationHook.AnimationContext.animatingModelRoot(0)`: its class isn't translated (it uses EMFEntityRenderState, which isn't translated (it's an interface)) (traben/entity_model_features/EMFAnimationApi.java:344)
- [ ] `EMFAnimationApi.EMFAnimationHook.AnimationContext.error(0)`: its class isn't translated (it uses EMFEntityRenderState, which isn't translated (it's an interface)) (traben/entity_model_features/EMFAnimationApi.java:344)
- [ ] `EMFAnimationApi.EMFAnimationHook.AnimationContext.partsRequestedToPauseThisAnimation(0)`: its class isn't translated (it uses EMFEntityRenderState, which isn't translated (it's an interface)) (traben/entity_model_features/EMFAnimationApi.java:344)
- [ ] `EMFAnimationApi.EMFAnimationHook.AnimationContext.animationHandler(0)`: its class isn't translated (it uses EMFEntityRenderState, which isn't translated (it's an interface)) (traben/entity_model_features/EMFAnimationApi.java:344)

### `EMFException`: it extends `Exception`

- [ ] `EMFException.EMFException(1)`: its class isn't translated (it extends `Exception`) (traben/entity_model_features/EMFException.java:6)
- [ ] `EMFException.record(0)`: its class isn't translated (it extends `Exception`) (traben/entity_model_features/EMFException.java:10)
- [ ] `EMFException.recordException(1)`: it's generic (traben/entity_model_features/EMFException.java:14)
- [ ] `EMFException.toString(0)`: its class isn't translated (it extends `Exception`) (traben/entity_model_features/EMFException.java:19)

### `EMFInit`: it implements Fabric's `ClientModInitializer`

- [ ] `EMFInit.onInitializeClient(0)`: its class isn't translated (it implements Fabric's `ClientModInitializer`) (traben/entity_model_features/EMFInit.java:9)

### `EMFManager`: it uses `UUIDInteger` (traben.entity_texture_features.utils.ETFLruCache)

- [ ] `EMFManager.EBETypes`: it uses Minecraft's `BlockEntityType` (traben/entity_model_features/EMFManager.java:66)
- [ ] `EMFManager.lastCreatedRootModelPart`: it uses EMFModelPartRoot, which isn't translated (it extends `EMFModelPartVanilla`) (traben/entity_model_features/EMFManager.java:82)
- [ ] `EMFManager.self`: it uses EMFManager, which isn't translated (it uses `UUIDInteger` (traben.entity_texture_features.utils.ETFLruCache)) (traben/entity_model_features/EMFManager.java:83)
- [x] `EMFManager.managerInstanceHash` (traben/entity_model_features/EMFManager.java:107)
- [ ] `EMFManager.receiveException(1)`: its class isn't translated (it uses `UUIDInteger` (traben.entity_texture_features.utils.ETFLruCache)) (traben/entity_model_features/EMFManager.java:110)
- [ ] `EMFManager.EMFManager(0)`: its class isn't translated (it uses `UUIDInteger` (traben.entity_texture_features.utils.ETFLruCache)) (traben/entity_model_features/EMFManager.java:116)
- [x] `EMFManager.getManagerInstanceHash(0)` (traben/entity_model_features/EMFManager.java:127)
- [ ] `EMFManager.getInstance(0)`: it uses EMFManager, which isn't translated (it uses `UUIDInteger` (traben.entity_texture_features.utils.ETFLruCache)) (traben/entity_model_features/EMFManager.java:131)
- [ ] `EMFManager.resetInstance(0)`: it calls EMFUtils.log, which isn't translated (traben/entity_model_features/EMFManager.java:139)
- [ ] `EMFManager.getJemDataWithDirectory(2)`: it uses EMFJemData, which isn't translated (it uses `LinkedList` (java.util)) (traben/entity_model_features/EMFManager.java:150)
- [ ] `EMFManager.getModelFromHierarchicalId(2)`: it uses EMFModelPart, which isn't translated (it extends Minecraft's `ModelPart`) (traben/entity_model_features/EMFManager.java:204)
- [ ] `EMFManager.handleBoats(2)`: it uses EMFModel_ID, which isn't translated (it implements `Comparable`) (traben/entity_model_features/EMFManager.java:237)
- [ ] `EMFManager.wasEBEModified(0)`: its class isn't translated (it uses `UUIDInteger` (traben.entity_texture_features.utils.ETFLruCache)) (traben/entity_model_features/EMFManager.java:254)
- [ ] `EMFManager.getResourcePackList(0)`: its class isn't translated (it uses `UUIDInteger` (traben.entity_texture_features.utils.ETFLruCache)) (traben/entity_model_features/EMFManager.java:258)
- [ ] `EMFManager.modifyEBEIfRequired(0)`: its class isn't translated (it uses `UUIDInteger` (traben.entity_texture_features.utils.ETFLruCache)) (traben/entity_model_features/EMFManager.java:268)
- [ ] `EMFManager.injectIntoModelRootGetter(2)`: its class isn't translated (it uses `UUIDInteger` (traben.entity_texture_features.utils.ETFLruCache)) (traben/entity_model_features/EMFManager.java:285)
- [ ] `EMFManager.reloadEnd(0)`: its class isn't translated (it uses `UUIDInteger` (traben.entity_texture_features.utils.ETFLruCache)) (traben/entity_model_features/EMFManager.java:804)
- [ ] `EMFManager.getArmorParts(0)`: its class isn't translated (it uses `UUIDInteger` (traben.entity_texture_features.utils.ETFLruCache)) (traben/entity_model_features/EMFManager.java:819)
- [ ] `EMFManager.getJemAndContext(3)`: its class isn't translated (it uses `UUIDInteger` (traben.entity_texture_features.utils.ETFLruCache)) (traben/entity_model_features/EMFManager.java:828)
- [ ] `EMFManager.getDoubleChest(4)`: its class isn't translated (it uses `UUIDInteger` (traben.entity_texture_features.utils.ETFLruCache)) (traben/entity_model_features/EMFManager.java:851)
- [ ] `EMFManager.setupAnimationsFromJemToModel(3)`: its class isn't translated (it uses `UUIDInteger` (traben.entity_texture_features.utils.ETFLruCache)) (traben/entity_model_features/EMFManager.java:871)
- [ ] `AgeableMobRendererAccessor`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_model_features/EMFManager.java:10)
- [ ] `CuboidAccessor`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_model_features/EMFManager.java:10)
- [ ] `MinecraftClientAccessor`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_model_features/EMFManager.java:9)
- [ ] `Mixin_GuiEntityTester`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_model_features/EMFManager.java:11)
- [ ] `MixinBabyModelTransform`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_model_features/EMFManager.java:12)
- [ ] `MixinEntityModelSet`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_model_features/EMFManager.java:18)
- [ ] `MixinMeshTransformer`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_model_features/EMFManager.java:12)
- [ ] `MixinBlockEntity`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_model_features/EMFManager.java:19)
- [ ] `MixinBlockEntityRendererFactories`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_model_features/EMFManager.java:26)
- [ ] `MixinCapeModels`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_model_features/EMFManager.java:7)
- [ ] `MixinEntity`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_model_features/EMFManager.java:20)
- [ ] `MixinEntityModelLoader`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_model_features/EMFManager.java:14)
- [ ] `MixinEntityRenderer`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_model_features/EMFManager.java:11)
- [ ] `MixinGameRenderer`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_model_features/EMFManager.java:11)
- [ ] `MixinModel`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_model_features/EMFManager.java:22)
- [ ] `MixinModelPart$Cuboid`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_model_features/EMFManager.java:13)
- [ ] `MixinModelPart`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_model_features/EMFManager.java:20)
- [ ] `MixinModelPartData`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_model_features/EMFManager.java:12)
- [ ] `MixinParrotEntityModel`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_model_features/EMFManager.java:20)
- [ ] `MixinPlayerEntity`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_model_features/EMFManager.java:17)
- [ ] `MixinResourceReloadEnd`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_model_features/EMFManager.java:12)
- [ ] `MixinResourceReloadStart`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_model_features/EMFManager.java:12)
- [ ] `MixinArrowRenderer`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_model_features/EMFManager.java:7)
- [ ] `MixinProjectileEntityRenderer`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_model_features/EMFManager.java:7)
- [ ] `MixinSpectralArrowRenderer`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_model_features/EMFManager.java:7)
- [ ] `MixinStuckArrowsFeatureRenderer`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_model_features/EMFManager.java:32)
- [ ] `MixinSulfurCubeInnerLayer_BlockOffset`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_model_features/EMFManager.java:20)
- [ ] `Mixin_Dolphin_BlockOffset`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_model_features/EMFManager.java:17)
- [ ] `Mixin_Enderman_BlockOffset`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_model_features/EMFManager.java:17)
- [ ] `Mixin_Fox_BlockOffset`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_model_features/EMFManager.java:19)
- [ ] `Mixin_Head_BlockOffset`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_model_features/EMFManager.java:19)
- [ ] `Mixin_Panda_BlockOffset`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_model_features/EMFManager.java:17)
- [ ] `Mixin_Parrot_ShoulderOffset`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_model_features/EMFManager.java:19)
- [ ] `Mixin_Villager_BlockOffset`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_model_features/EMFManager.java:17)
- [ ] `Mixin_Witch_BlockOffset`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_model_features/EMFManager.java:20)
- [ ] `MixinCreeperChargeFeatureRenderer`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_model_features/EMFManager.java:16)
- [ ] `MixinHeadFeatureRenderer`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_model_features/EMFManager.java:11)
- [ ] `MixinHeldItemFeatureRenderer`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_model_features/EMFManager.java:17)
- [ ] `MixinHumanoidArmorLayer`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_model_features/EMFManager.java:7)
- [ ] `MixinSlimeOverlayFeatureRenderer`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_model_features/EMFManager.java:12)
- [ ] `MixinWolfCollarFeatureRenderer`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_model_features/EMFManager.java:29)
- [ ] `MixinBannerRenderer`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_model_features/EMFManager.java:11)
- [ ] `MixinBlockEntityWithoutLevelRenderer_skull`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_model_features/EMFManager.java:17)
- [ ] `MixinDragonRenderer`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_model_features/EMFManager.java:11)
- [ ] `MixinEntityRenderDispatcher`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_model_features/EMFManager.java:19)
- [ ] `MixinEntityRenderers`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_model_features/EMFManager.java:17)
- [ ] `MixinItemFrameEntityRenderer`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_model_features/EMFManager.java:7)
- [ ] `MixinLeashOffsets`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_model_features/EMFManager.java:19)
- [ ] `MixinLivingEntityRenderer`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_model_features/EMFManager.java:32)
- [ ] `MixinPlayerEntityRenderer`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_model_features/EMFManager.java:21)
- [ ] `Mixin_SpecialModelRenderers_SetCurrentSpecifiedModel`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_model_features/EMFManager.java:29)
- [ ] `Mixin_WrapFirstPersonHandRender`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_model_features/EMFManager.java:18)
- [ ] `MixinArmadilloModel`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_model_features/EMFManager.java:16)
- [ ] `MixinDragonModel`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_model_features/EMFManager.java:7)
- [ ] `MixinVillagerModel`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_model_features/EMFManager.java:12)
- [ ] `MixinWolfEntityModel`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_model_features/EMFManager.java:14)
- [ ] `Mixin_ChickenRenderer_WarmModel`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_model_features/EMFManager.java:27)
- [ ] `Mixin_EnderDragonFireball_Model`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_model_features/EMFManager.java:29)
- [ ] `Mixin_PigRenderer_WarmModel`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_model_features/EMFManager.java:27)
- [ ] `Mixin_PlayerModel_ReApplyPoseToExtraCallsMadeByMods`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_model_features/EMFManager.java:13)
- [ ] `MixinFlameFeatureRenderer`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_model_features/EMFManager.java:13)
- [ ] `Mixin_HumanoidArmorLayer_AddBaseModelPoseRef`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_model_features/EMFManager.java:14)
- [ ] `Mixin_ModelPartRenderer`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_model_features/EMFManager.java:7)
- [ ] `Mixin_ModelRenderer`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_model_features/EMFManager.java:18)

### `Plugin`: it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)

- [ ] `Plugin.onLoad(1)`: its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)) (traben/entity_model_features/mixin/Plugin.java:11)
- [ ] `Plugin.getRefMapperConfig(0)`: its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)) (traben/entity_model_features/mixin/Plugin.java:15)
- [ ] `Plugin.shouldApplyMixin(2)`: its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)) (traben/entity_model_features/mixin/Plugin.java:19)
- [ ] `Plugin.acceptTargets(2)`: its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)) (traben/entity_model_features/mixin/Plugin.java:23)
- [ ] `Plugin.getMixins(0)`: its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)) (traben/entity_model_features/mixin/Plugin.java:26)
- [ ] `Plugin.preApply(4)`: its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)) (traben/entity_model_features/mixin/Plugin.java:30)
- [ ] `Plugin.postApply(4)`: its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)) (traben/entity_model_features/mixin/Plugin.java:33)

### `AnimSetupContext`: it implements `AutoCloseable`

- [ ] `AnimSetupContext.AnimSetupContext(3)`: its class isn't translated (it implements `AutoCloseable`) (traben/entity_model_features/models/animation/AnimSetupContext.java:17)
- [ ] `AnimSetupContext.getModelFromHierarchicalId(1)`: its class isn't translated (it implements `AutoCloseable`) (traben/entity_model_features/models/animation/AnimSetupContext.java:23)
- [ ] `AnimSetupContext.close(0)`: its class isn't translated (it implements `AutoCloseable`) (traben/entity_model_features/models/animation/AnimSetupContext.java:34)

### `EMFAnimation`


### `EMFAnimationHandler`: it uses AnimLineData, which isn't translated (it uses EMFModelPart, which isn't translated (it extends Minecraft's `ModelPart`))

- [ ] `EMFAnimationHandler.EMFAnimationHandler(2)`: its class isn't translated (it uses AnimLineData, which isn't translated (it uses EMFModelPart, which isn't translated (it extends Minecraft's `ModelPart`))) (traben/entity_model_features/models/animation/EMFAnimationHandler.java:15)
- [ ] `EMFAnimationHandler.addAnimLineData(1)`: it stores a second reference to an object (Rust would need it copied or shared) (traben/entity_model_features/models/animation/EMFAnimationHandler.java:20)
- [ ] `EMFAnimationHandler.lines(0)`: its class isn't translated (it uses AnimLineData, which isn't translated (it uses EMFModelPart, which isn't translated (it extends Minecraft's `ModelPart`))) (traben/entity_model_features/models/animation/EMFAnimationHandler.java:25)
- [ ] `EMFAnimationHandler.getLine(1)`: it stores a second reference to an object (Rust would need it copied or shared) (traben/entity_model_features/models/animation/EMFAnimationHandler.java:29)
- [ ] `EMFAnimationHandler.animate(1)`: it uses Minecraft's `ModelPart` (traben/entity_model_features/models/animation/EMFAnimationHandler.java:39)
- [ ] `EMFAnimationHandler.animateInner(1)`: it has no body (traben/entity_model_features/models/animation/EMFAnimationHandler.java:50)
- [ ] `EMFAnimationHandler.finishAndValidate(0)`: it has no body (traben/entity_model_features/models/animation/EMFAnimationHandler.java:52)
- [ ] `EMFAnimationHandler.toString(0)`: it calls EMFAnimationHandler.getClass, which isn't in the mod's source (traben/entity_model_features/models/animation/EMFAnimationHandler.java:54)

### `EMFAnimationHandler.AnimLineData`: it uses EMFModelPart, which isn't translated (it extends Minecraft's `ModelPart`)

- [ ] `EMFAnimationHandler.AnimLineData.AnimLineData(4)`: its class isn't translated (it uses EMFModelPart, which isn't translated (it extends Minecraft's `ModelPart`)) (traben/entity_model_features/models/animation/EMFAnimationHandler.java:72)
- [ ] `EMFAnimationHandler.AnimLineData.hashCode(0)`: its class isn't translated (it uses EMFModelPart, which isn't translated (it extends Minecraft's `ModelPart`)) (traben/entity_model_features/models/animation/EMFAnimationHandler.java:83)
- [ ] `EMFAnimationHandler.AnimLineData.toString(0)`: its class isn't translated (it uses EMFModelPart, which isn't translated (it extends Minecraft's `ModelPart`)) (traben/entity_model_features/models/animation/EMFAnimationHandler.java:88)

### `EMFAttachment`: it uses Minecraft's `Pose`

- [ ] `EMFAttachment.EMFAttachment(4)`: its class isn't translated (it uses Minecraft's `Pose`) (traben/entity_model_features/models/animation/EMFAttachment.java:14)
- [ ] `EMFAttachment.translate(1)`: its class isn't translated (it uses Minecraft's `Pose`) (traben/entity_model_features/models/animation/EMFAttachment.java:21)

### `EMFAttachment.Type`: its constants carry values

- [ ] `EMFAttachment.Type.NON_HANDS`: it uses Type, which isn't translated (its constants carry values) (traben/entity_model_features/models/animation/EMFAttachment.java:60)
- [ ] `EMFAttachment.Type.Type(1)`: its class isn't translated (its constants carry values) (traben/entity_model_features/models/animation/EMFAttachment.java:64)
- [ ] `EMFAttachment.Type.translateForType(2)`: its class isn't translated (its constants carry values) (traben/entity_model_features/models/animation/EMFAttachment.java:68)
- [ ] `EMFAttachment.Type.isHumanoidHand(0)`: its class isn't translated (its constants carry values) (traben/entity_model_features/models/animation/EMFAttachment.java:72)
- [ ] `EMFAttachment.Type.of(1)`: it uses Type, which isn't translated (its constants carry values) (traben/entity_model_features/models/animation/EMFAttachment.java:76)

### `ASMAnimationHandler`: it extends `EMFAnimationHandler` (traben.entity_model_features.models.animation)

- [ ] `ASMAnimationHandler.ASMAnimationHandler(3)`: its class isn't translated (it extends `EMFAnimationHandler` (traben.entity_model_features.models.animation)) (traben/entity_model_features/models/animation/math/asm/ASMAnimationHandler.java:36)
- [ ] `ASMAnimationHandler.complete(2)`: its class isn't translated (it extends `EMFAnimationHandler` (traben.entity_model_features.models.animation)) (traben/entity_model_features/models/animation/math/asm/ASMAnimationHandler.java:42)
- [ ] `ASMAnimationHandler.animateInner(1)`: its class isn't translated (it extends `EMFAnimationHandler` (traben.entity_model_features.models.animation)) (traben/entity_model_features/models/animation/math/asm/ASMAnimationHandler.java:48)
- [ ] `ASMAnimationHandler.canLODPause(1)`: its class isn't translated (it extends `EMFAnimationHandler` (traben.entity_model_features.models.animation)) (traben/entity_model_features/models/animation/math/asm/ASMAnimationHandler.java:58)
- [ ] `ASMAnimationHandler.animateInnerLOD(1)`: its class isn't translated (it extends `EMFAnimationHandler` (traben.entity_model_features.models.animation)) (traben/entity_model_features/models/animation/math/asm/ASMAnimationHandler.java:62)
- [ ] `ASMAnimationHandler.animateInnerNoLOD(1)`: its class isn't translated (it extends `EMFAnimationHandler` (traben.entity_model_features.models.animation)) (traben/entity_model_features/models/animation/math/asm/ASMAnimationHandler.java:69)
- [ ] `ASMAnimationHandler.finishAndValidate(0)`: its class isn't translated (it extends `EMFAnimationHandler` (traben.entity_model_features.models.animation)) (traben/entity_model_features/models/animation/math/asm/ASMAnimationHandler.java:86)
- [ ] `ASMAnimationHandler.buildFloatConsumers(0)`: its class isn't translated (it extends `EMFAnimationHandler` (traben.entity_model_features.models.animation)) (traben/entity_model_features/models/animation/math/asm/ASMAnimationHandler.java:102)
- [ ] `ASMAnimationHandler.buildBoolConsumers(0)`: its class isn't translated (it extends `EMFAnimationHandler` (traben.entity_model_features.models.animation)) (traben/entity_model_features/models/animation/math/asm/ASMAnimationHandler.java:159)
- [ ] `ASMAnimationHandler.buildVarSupplier(2)`: its class isn't translated (it extends `EMFAnimationHandler` (traben.entity_model_features.models.animation)) (traben/entity_model_features/models/animation/math/asm/ASMAnimationHandler.java:209)
- [ ] `ASMAnimationHandler.asmLog(3)`: it uses ASMVariableHandler, which isn't translated (it uses `Stack` (java.util)) (traben/entity_model_features/models/animation/math/asm/ASMAnimationHandler.java:254)

### `ASMAnimationHandler.AnimVars`

- [x] `ASMAnimationHandler.AnimVars.AnimVars(2)` (traben/entity_model_features/models/animation/math/asm/ASMAnimationHandler.java:271)
- [x] `ASMAnimationHandler.AnimVars.floats(0)` (traben/entity_model_features/models/animation/math/asm/ASMAnimationHandler.java:271)
- [x] `ASMAnimationHandler.AnimVars.bools(0)` (traben/entity_model_features/models/animation/math/asm/ASMAnimationHandler.java:271)

### `ASMAnimationHandler.BoolConsumerAsm`: it's an interface

- [ ] `ASMAnimationHandler.BoolConsumerAsm.accept(2)`: it has no body (traben/entity_model_features/models/animation/math/asm/ASMAnimationHandler.java:275)

### `ASMAnimationHandler.FloatConsumerAsm`: it's an interface

- [ ] `ASMAnimationHandler.FloatConsumerAsm.accept(2)`: it has no body (traben/entity_model_features/models/animation/math/asm/ASMAnimationHandler.java:279)

### `ASMAnimationHandler.VarConsumer`: it's an interface

- [ ] `ASMAnimationHandler.VarConsumer.accept(2)`: it has no body (traben/entity_model_features/models/animation/math/asm/ASMAnimationHandler.java:283)

### `ASMHelper`

- [ ] `ASMHelper.visitHelperFunctionASM(2)`: it uses `MethodVisitor` (org.objectweb.asm) (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:20)
- [ ] `ASMHelper.getHelperMethod(1)`: it uses `Method` (java.lang.reflect) (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:24)
- [ ] `ASMHelper.getHelperMethodCompiler(1)`: it uses ASMVisitable, which isn't translated (it's an interface) (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:33)
- [ ] `ASMHelper.getStaticMethod(2)`: it uses `Method` (java.lang.reflect) (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:49)
- [ ] `ASMHelper.visitStaticFunctionASM(3)`: it uses `MethodVisitor` (org.objectweb.asm) (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:58)
- [ ] `ASMHelper.visitStaticFunctionASM(2)`: it uses `MethodVisitor` (org.objectweb.asm) (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:63)
- [ ] `ASMHelper.asmIfZeroOrGreater(3)`: it uses `MethodVisitor` (org.objectweb.asm) (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:68)
- [ ] `ASMHelper.asmIf(3)`: it uses `MethodVisitor` (org.objectweb.asm) (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:82)
- [x] `ASMHelper.sin(1)` (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:93)
- [x] `ASMHelper.cos(1)` (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:97)
- [x] `ASMHelper.asin(1)` (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:101)
- [x] `ASMHelper.acos(1)` (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:105)
- [x] `ASMHelper.tan(1)` (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:109)
- [x] `ASMHelper.atan(1)` (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:113)
- [x] `ASMHelper.abs(1)` (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:117)
- [x] `ASMHelper.floor(1)` (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:121)
- [x] `ASMHelper.ceil(1)` (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:125)
- [x] `ASMHelper.round(1)` (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:129)
- [ ] `ASMHelper.log(1)`: it calls EMFManager.getInstance, which isn't translated (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:133)
- [x] `ASMHelper._log(1)` (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:137)
- [ ] `ASMHelper.log_ASM(2)`: it uses `MethodVisitor` (org.objectweb.asm) (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:141)
- [x] `ASMHelper.exp(1)` (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:148)
- [x] `ASMHelper.torad(1)` (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:152)
- [ ] `ASMHelper.torad_ASM(2)`: it uses `MethodVisitor` (org.objectweb.asm) (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:156)
- [x] `ASMHelper.todeg(1)` (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:161)
- [ ] `ASMHelper.todeg_ASM(2)`: it uses `MethodVisitor` (org.objectweb.asm) (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:165)
- [x] `ASMHelper.frac(1)` (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:170)
- [x] `ASMHelper.signum(1)` (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:174)
- [ ] `ASMHelper.sqrt(1)`: it calls EMFManager.getInstance, which isn't translated (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:178)
- [x] `ASMHelper._sqrt(1)` (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:182)
- [ ] `ASMHelper.sqrt_ASM(2)`: it uses `MethodVisitor` (org.objectweb.asm) (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:186)
- [x] `ASMHelper.fmod(2)` (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:193)
- [x] `ASMHelper.pow(2)` (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:197)
- [x] `ASMHelper.atan2(2)` (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:201)
- [x] `ASMHelper.clamp(3)` (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:205)
- [x] `ASMHelper.lerp(3)` (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:209)
- [x] `ASMHelper.wrapdeg(1)` (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:213)
- [x] `ASMHelper.wraprad(1)` (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:217)
- [ ] `ASMHelper.degdiff(2)`: it calls Minecraft's `Mth`.degreesDifferenceAbs (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:221)
- [ ] `ASMHelper.raddiff(2)`: it calls Minecraft's `Mth`.degreesDifferenceAbs (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:225)
- [ ] `ASMHelper.catmullrom(5)`: it calls Minecraft's `Mth`.catmullrom (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:229)
- [x] `ASMHelper.between(3)` (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:233)
- [x] `ASMHelper.equals(3)` (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:237)
- [ ] `ASMHelper.notPausedNotValidation(0)`: it calls Minecraft's Minecraft.isPaused (no Horizon helper for it yet) (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:241)
- [x] `ASMHelper.quadraticBezier(4)` (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:245)
- [x] `ASMHelper.cubicBezier(5)` (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:250)
- [x] `ASMHelper.hermiteInterpolation(5)` (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:257)
- [x] `ASMHelper.easeInQuad(3)` (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:267)
- [x] `ASMHelper.easeOutQuad(3)` (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:272)
- [x] `ASMHelper.easeInOutQuad(3)` (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:277)
- [x] `ASMHelper.easeInCubic(3)` (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:283)
- [ ] `ASMHelper.easeOutCubic(3)`: `--` inside an expression (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:288)
- [ ] `ASMHelper.easeInOutCubic(3)`: `--` inside an expression (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:293)
- [x] `ASMHelper.easeInQuart(3)` (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:299)
- [ ] `ASMHelper.easeOutQuart(3)`: `--` inside an expression (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:304)
- [ ] `ASMHelper.easeInOutQuart(3)`: `--` inside an expression (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:309)
- [x] `ASMHelper.easeInQuint(3)` (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:315)
- [ ] `ASMHelper.easeOutQuint(3)`: `--` inside an expression (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:320)
- [ ] `ASMHelper.easeInOutQuint(3)`: `--` inside an expression (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:325)
- [x] `ASMHelper.easeInSine(3)` (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:331)
- [x] `ASMHelper.easeOutSine(3)` (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:336)
- [x] `ASMHelper.easeInOutSine(3)` (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:341)
- [x] `ASMHelper.easeInExpo(3)` (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:346)
- [x] `ASMHelper.easeOutExpo(3)` (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:351)
- [ ] `ASMHelper.easeInOutExpo(3)`: `--` inside an expression (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:356)
- [x] `ASMHelper.easeInCirc(3)` (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:362)
- [x] `ASMHelper.easeOutCirc(3)` (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:367)
- [x] `ASMHelper.easeInOutCirc(3)` (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:373)
- [ ] `ASMHelper.easeInElastic(3)`: `--` inside an expression (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:384)
- [x] `ASMHelper.easeOutElastic(3)` (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:389)
- [ ] `ASMHelper.easeInOutElastic(3)`: `--` inside an expression (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:394)
- [ ] `ASMHelper.easeInBounce(3)`: it calls ASMHelper.easeOutBounce, which isn't translated (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:402)
- [ ] `ASMHelper.easeOutBounce(3)`: an assignment inside an expression (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:407)
- [ ] `ASMHelper.easeInOutBounce(3)`: it calls ASMHelper.easeInBounce, which isn't translated (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:424)
- [x] `ASMHelper.easeInBack(3)` (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:429)
- [ ] `ASMHelper.easeOutBack(3)`: `--` inside an expression (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:434)
- [ ] `ASMHelper.easeInOutBack(3)`: `--` inside an expression (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:439)
- [ ] `ASMHelper.functionName(2)`: it uses `Class` (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:445)
- [ ] `ASMHelper.fromMethod(2)`: it uses `Class` (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:454)
- [ ] `ASMHelper.methodDescriptor(1)`: it uses `Method` (java.lang.reflect) (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:458)
- [ ] `ASMHelper.typeDescriptor(1)`: it uses `Class` (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:471)

### `ASMHelper.ThrowingRunnable`: it's an interface

- [ ] `ASMHelper.ThrowingRunnable.run(0)`: it has no body (traben/entity_model_features/models/animation/math/asm/ASMHelper.java:500)

### `ASMParser`

- [x] `ASMParser.id` (traben/entity_model_features/models/animation/math/asm/ASMParser.java:32)
- [x] `ASMParser.CHUNK_INSNS` (traben/entity_model_features/models/animation/math/asm/ASMParser.java:33)
- [x] `ASMParser.P_BASTORE_FLOAT_TO_INT` (traben/entity_model_features/models/animation/math/asm/ASMParser.java:34)
- [x] `ASMParser.P_FASTORE_INT_TO_FLOAT` (traben/entity_model_features/models/animation/math/asm/ASMParser.java:35)
- [x] `ASMParser.P_FLOAT_TO_INT` (traben/entity_model_features/models/animation/math/asm/ASMParser.java:36)
- [x] `ASMParser.P_INT_TO_FLOAT` (traben/entity_model_features/models/animation/math/asm/ASMParser.java:37)
- [ ] `ASMParser.getOrNull(2)`: it uses EMFAnimationHandler, which isn't translated (it uses AnimLineData, which isn't translated (it uses EMFModelPart, which isn't translated (it extends Minecraft's `ModelPart`))) (traben/entity_model_features/models/animation/math/asm/ASMParser.java:39)
- [ ] `ASMParser.splitAndGetOrNull(4)`: it uses ASMAnimationHandler, which isn't translated (it extends `EMFAnimationHandler` (traben.entity_model_features.models.animation)) (traben/entity_model_features/models/animation/math/asm/ASMParser.java:52)
- [ ] `ASMParser.compileOrNull(2)`: its body couldn't be read: an assert (traben/entity_model_features/models/animation/math/asm/ASMParser.java:113)
- [ ] `ASMParser.setupClass(1)`: it uses `ClassWriter` (org.objectweb.asm) (traben/entity_model_features/models/animation/math/asm/ASMParser.java:177)
- [ ] `ASMParser.handleParseException(2)`: it uses AnimLineData, which isn't translated (it uses EMFModelPart, which isn't translated (it extends Minecraft's `ModelPart`)) (traben/entity_model_features/models/animation/math/asm/ASMParser.java:192)

### `ASMParser.ASMExecutor`: it's an interface

- [ ] `ASMParser.ASMExecutor.execute(2)`: it has no body (traben/entity_model_features/models/animation/math/asm/ASMParser.java:244)

### `ASMParser.NeedToSplitAnimationsException`: it extends `EMFException` (traben.entity_model_features)

- [ ] `ASMParser.NeedToSplitAnimationsException.NeedToSplitAnimationsException(0)`: its class isn't translated (it extends `EMFException` (traben.entity_model_features)) (traben/entity_model_features/models/animation/math/asm/ASMParser.java:248)

### `ASMVariableHandler`: it uses `Stack` (java.util)

- [ ] `ASMVariableHandler.getLocalVarIndex(0)`: its class isn't translated (it uses `Stack` (java.util)) (traben/entity_model_features/models/animation/math/asm/ASMVariableHandler.java:24)
- [ ] `ASMVariableHandler.popLocalVarIndex(1)`: its class isn't translated (it uses `Stack` (java.util)) (traben/entity_model_features/models/animation/math/asm/ASMVariableHandler.java:28)
- [ ] `ASMVariableHandler.getFloatVarList(0)`: its class isn't translated (it uses `Stack` (java.util)) (traben/entity_model_features/models/animation/math/asm/ASMVariableHandler.java:36)
- [ ] `ASMVariableHandler.getBoolVarList(0)`: its class isn't translated (it uses `Stack` (java.util)) (traben/entity_model_features/models/animation/math/asm/ASMVariableHandler.java:40)
- [ ] `ASMVariableHandler.scope(1)`: its class isn't translated (it uses `Stack` (java.util)) (traben/entity_model_features/models/animation/math/asm/ASMVariableHandler.java:44)
- [ ] `ASMVariableHandler.scopeFloat(0)`: its class isn't translated (it uses `Stack` (java.util)) (traben/entity_model_features/models/animation/math/asm/ASMVariableHandler.java:48)
- [ ] `ASMVariableHandler.scopeBool(0)`: its class isn't translated (it uses `Stack` (java.util)) (traben/entity_model_features/models/animation/math/asm/ASMVariableHandler.java:52)
- [ ] `ASMVariableHandler.scopePop(0)`: its class isn't translated (it uses `Stack` (java.util)) (traben/entity_model_features/models/animation/math/asm/ASMVariableHandler.java:56)
- [ ] `ASMVariableHandler.verifyEndOfParse(0)`: its class isn't translated (it uses `Stack` (java.util)) (traben/entity_model_features/models/animation/math/asm/ASMVariableHandler.java:60)
- [ ] `ASMVariableHandler.isScopeBool(0)`: its class isn't translated (it uses `Stack` (java.util)) (traben/entity_model_features/models/animation/math/asm/ASMVariableHandler.java:76)
- [ ] `ASMVariableHandler.isScopeFloat(0)`: its class isn't translated (it uses `Stack` (java.util)) (traben/entity_model_features/models/animation/math/asm/ASMVariableHandler.java:80)
- [ ] `ASMVariableHandler.isReadVarName(1)`: its class isn't translated (it uses `Stack` (java.util)) (traben/entity_model_features/models/animation/math/asm/ASMVariableHandler.java:84)
- [ ] `ASMVariableHandler.isWriteVarName(1)`: its class isn't translated (it uses `Stack` (java.util)) (traben/entity_model_features/models/animation/math/asm/ASMVariableHandler.java:88)
- [ ] `ASMVariableHandler.getAndAssignVarIndex(2)`: its class isn't translated (it uses `Stack` (java.util)) (traben/entity_model_features/models/animation/math/asm/ASMVariableHandler.java:92)
- [ ] `ASMVariableHandler.asmVisitFrameCounter(1)`: its class isn't translated (it uses `Stack` (java.util)) (traben/entity_model_features/models/animation/math/asm/ASMVariableHandler.java:118)
- [ ] `ASMVariableHandler.asmVisitVar(2)`: its class isn't translated (it uses `Stack` (java.util)) (traben/entity_model_features/models/animation/math/asm/ASMVariableHandler.java:124)
- [ ] `ASMVariableHandler.asmStoreVar(2)`: its class isn't translated (it uses `Stack` (java.util)) (traben/entity_model_features/models/animation/math/asm/ASMVariableHandler.java:132)
- [ ] `ASMVariableHandler.asmInvertBoolean(1)`: its class isn't translated (it uses `Stack` (java.util)) (traben/entity_model_features/models/animation/math/asm/ASMVariableHandler.java:143)
- [ ] `ASMVariableHandler.asmNegateFloat(1)`: its class isn't translated (it uses `Stack` (java.util)) (traben/entity_model_features/models/animation/math/asm/ASMVariableHandler.java:148)

### `ASMVisitable`: it's an interface

- [ ] `ASMVisitable.asmVisit(2)`: it has no body (traben/entity_model_features/models/animation/math/asm/ASMVisitable.java:7)

### `MultiASMAnimationHandler`: it extends `EMFAnimationHandler` (traben.entity_model_features.models.animation)

- [ ] `MultiASMAnimationHandler.MultiASMAnimationHandler(3)`: its class isn't translated (it extends `EMFAnimationHandler` (traben.entity_model_features.models.animation)) (traben/entity_model_features/models/animation/math/asm/MultiASMAnimationHandler.java:18)
- [ ] `MultiASMAnimationHandler.animateInner(1)`: its class isn't translated (it extends `EMFAnimationHandler` (traben.entity_model_features.models.animation)) (traben/entity_model_features/models/animation/math/asm/MultiASMAnimationHandler.java:23)
- [ ] `MultiASMAnimationHandler.finishAndValidate(0)`: its class isn't translated (it extends `EMFAnimationHandler` (traben.entity_model_features.models.animation)) (traben/entity_model_features/models/animation/math/asm/MultiASMAnimationHandler.java:58)

### `EMFMathException`: it extends `EMFException` (traben.entity_model_features)

- [ ] `EMFMathException.EMFMathException(1)`: its class isn't translated (it extends `EMFException` (traben.entity_model_features)) (traben/entity_model_features/models/animation/math/EMFMathException.java:6)

### `MathBinaryExpressionComponent`: it extends `MathValue`

- [ ] `MathBinaryExpressionComponent.MathBinaryExpressionComponent(3)`: its class isn't translated (it extends `MathValue`) (traben/entity_model_features/models/animation/math/expression_tree/MathBinaryExpressionComponent.java:13)
- [ ] `MathBinaryExpressionComponent.getOptimizedExpression(3)`: it uses MathComponent, which isn't translated (it's an interface) (traben/entity_model_features/models/animation/math/expression_tree/MathBinaryExpressionComponent.java:19)
- [ ] `MathBinaryExpressionComponent.asmVisit(2)`: its class isn't translated (it extends `MathValue`) (traben/entity_model_features/models/animation/math/expression_tree/MathBinaryExpressionComponent.java:30)
- [ ] `MathBinaryExpressionComponent.getResultSupplier(0)`: its class isn't translated (it extends `MathValue`) (traben/entity_model_features/models/animation/math/expression_tree/MathBinaryExpressionComponent.java:65)
- [ ] `MathBinaryExpressionComponent.getResult(0)`: its class isn't translated (it extends `MathValue`) (traben/entity_model_features/models/animation/math/expression_tree/MathBinaryExpressionComponent.java:70)
- [ ] `MathBinaryExpressionComponent.toString(0)`: its class isn't translated (it extends `MathValue`) (traben/entity_model_features/models/animation/math/expression_tree/MathBinaryExpressionComponent.java:76)

### `MathComponent`: it's an interface

- [ ] `MathComponent.getResult(0)`: it has no body (traben/entity_model_features/models/animation/math/expression_tree/MathComponent.java:9)
- [ ] `MathComponent.isConstant(0)`: its class isn't translated (it's an interface) (traben/entity_model_features/models/animation/math/expression_tree/MathComponent.java:11)
- [ ] `MathComponent.asmVisit(2)`: it has no body (traben/entity_model_features/models/animation/math/expression_tree/MathComponent.java:15)
- [ ] `MathComponent.toConstant(0)`: its class isn't translated (it's an interface) (traben/entity_model_features/models/animation/math/expression_tree/MathComponent.java:18)
- [ ] `MathComponent.negativeDelegate(1)`: its body couldn't be read: an anonymous class (traben/entity_model_features/models/animation/math/expression_tree/MathComponent.java:22)
- [ ] `MathComponent.invertedBooleanDelegate(1)`: its body couldn't be read: an anonymous class (traben/entity_model_features/models/animation/math/expression_tree/MathComponent.java:42)
- [ ] `MathComponent.validateBooleanDelegate(1)`: its body couldn't be read: an anonymous class (traben/entity_model_features/models/animation/math/expression_tree/MathComponent.java:62)

### `MathConstant`: it extends `MathValue`

- [ ] `MathConstant.ZERO_CONST`: it uses MathConstant, which isn't translated (it extends `MathValue`) (traben/entity_model_features/models/animation/math/expression_tree/MathConstant.java:7)
- [ ] `MathConstant.FALSE_CONST`: it uses MathConstant, which isn't translated (it extends `MathValue`) (traben/entity_model_features/models/animation/math/expression_tree/MathConstant.java:8)
- [ ] `MathConstant.MathConstant(2)`: its class isn't translated (it extends `MathValue`) (traben/entity_model_features/models/animation/math/expression_tree/MathConstant.java:11)
- [ ] `MathConstant.MathConstant(1)`: its class isn't translated (it extends `MathValue`) (traben/entity_model_features/models/animation/math/expression_tree/MathConstant.java:15)
- [ ] `MathConstant.getResultSupplier(0)`: its class isn't translated (it extends `MathValue`) (traben/entity_model_features/models/animation/math/expression_tree/MathConstant.java:19)
- [ ] `MathConstant.isConstant(0)`: its class isn't translated (it extends `MathValue`) (traben/entity_model_features/models/animation/math/expression_tree/MathConstant.java:24)
- [ ] `MathConstant.toString(0)`: its class isn't translated (it extends `MathValue`) (traben/entity_model_features/models/animation/math/expression_tree/MathConstant.java:29)
- [ ] `MathConstant.getResult(0)`: its class isn't translated (it extends `MathValue`) (traben/entity_model_features/models/animation/math/expression_tree/MathConstant.java:34)
- [ ] `MathConstant.asmVisit(2)`: its class isn't translated (it extends `MathValue`) (traben/entity_model_features/models/animation/math/expression_tree/MathConstant.java:39)

### `MathExpressionParser`: it uses AnimSetupContext, which isn't translated (it implements `AutoCloseable`)

- [ ] `MathExpressionParser.NULL_EXPRESSION`: its value couldn't be read: an anonymous class (traben/entity_model_features/models/animation/math/expression_tree/MathExpressionParser.java:14)
- [ ] `MathExpressionParser.BOOLEAN_COMPARATOR_ACTIONS`: it uses MathOperator, which isn't translated (its constants carry values) (traben/entity_model_features/models/animation/math/expression_tree/MathExpressionParser.java:21)
- [ ] `MathExpressionParser.BOOLEAN_LOGICAL_ACTIONS`: it uses MathOperator, which isn't translated (its constants carry values) (traben/entity_model_features/models/animation/math/expression_tree/MathExpressionParser.java:29)
- [ ] `MathExpressionParser.MULTIPLICATION_ACTIONS`: it uses MathOperator, which isn't translated (its constants carry values) (traben/entity_model_features/models/animation/math/expression_tree/MathExpressionParser.java:30)
- [ ] `MathExpressionParser.ADDITION_ACTIONS`: it uses MathOperator, which isn't translated (its constants carry values) (traben/entity_model_features/models/animation/math/expression_tree/MathExpressionParser.java:31)
- [ ] `MathExpressionParser.MathExpressionParser(4)`: its class isn't translated (it uses AnimSetupContext, which isn't translated (it implements `AutoCloseable`)) (traben/entity_model_features/models/animation/math/expression_tree/MathExpressionParser.java:41)
- [ ] `MathExpressionParser.getOptimizedExpression(3)`: it uses MathComponent, which isn't translated (it's an interface) (traben/entity_model_features/models/animation/math/expression_tree/MathExpressionParser.java:143)
- [ ] `MathExpressionParser.getOptimizedExpression(4)`: it uses MathComponent, which isn't translated (it's an interface) (traben/entity_model_features/models/animation/math/expression_tree/MathExpressionParser.java:147)
- [ ] `MathExpressionParser.readBracketContents(1)`: an Iterator passed as a parameter (traben/entity_model_features/models/animation/math/expression_tree/MathExpressionParser.java:163)
- [ ] `MathExpressionParser.readLastSingleBooleanAction(3)`: its class isn't translated (it uses AnimSetupContext, which isn't translated (it implements `AutoCloseable`)) (traben/entity_model_features/models/animation/math/expression_tree/MathExpressionParser.java:185)
- [ ] `MathExpressionParser.readDoubleBooleanAction(3)`: its class isn't translated (it uses AnimSetupContext, which isn't translated (it implements `AutoCloseable`)) (traben/entity_model_features/models/animation/math/expression_tree/MathExpressionParser.java:205)
- [ ] `MathExpressionParser.readMethodOrBrackets(2)`: its class isn't translated (it uses AnimSetupContext, which isn't translated (it implements `AutoCloseable`)) (traben/entity_model_features/models/animation/math/expression_tree/MathExpressionParser.java:227)
- [ ] `MathExpressionParser.readVariableOrConstant(1)`: its class isn't translated (it uses AnimSetupContext, which isn't translated (it implements `AutoCloseable`)) (traben/entity_model_features/models/animation/math/expression_tree/MathExpressionParser.java:246)
- [ ] `MathExpressionParser.validateAndOptimize(0)`: its class isn't translated (it uses AnimSetupContext, which isn't translated (it implements `AutoCloseable`)) (traben/entity_model_features/models/animation/math/expression_tree/MathExpressionParser.java:287)
- [ ] `MathExpressionParser.getNegativeNext(0)`: its class isn't translated (it uses AnimSetupContext, which isn't translated (it implements `AutoCloseable`)) (traben/entity_model_features/models/animation/math/expression_tree/MathExpressionParser.java:299)
- [ ] `MathExpressionParser.validateCalculationAndOptimize(0)`: its class isn't translated (it uses AnimSetupContext, which isn't translated (it implements `AutoCloseable`)) (traben/entity_model_features/models/animation/math/expression_tree/MathExpressionParser.java:305)
- [ ] `MathExpressionParser.optimiseTheseActionsIntoBinaryComponents(2)`: its class isn't translated (it uses AnimSetupContext, which isn't translated (it implements `AutoCloseable`)) (traben/entity_model_features/models/animation/math/expression_tree/MathExpressionParser.java:367)
- [ ] `MathExpressionParser.toString(0)`: its class isn't translated (it uses AnimSetupContext, which isn't translated (it implements `AutoCloseable`)) (traben/entity_model_features/models/animation/math/expression_tree/MathExpressionParser.java:393)

### `MathExpressionParser.CalculationList`: it extends `ArrayList` (java.util)

- [ ] `MathExpressionParser.CalculationList.CalculationList(1)`: its class isn't translated (it extends `ArrayList` (java.util)) (traben/entity_model_features/models/animation/math/expression_tree/MathExpressionParser.java:407)
- [ ] `MathExpressionParser.CalculationList.CalculationList(0)`: its class isn't translated (it extends `ArrayList` (java.util)) (traben/entity_model_features/models/animation/math/expression_tree/MathExpressionParser.java:411)
- [ ] `MathExpressionParser.CalculationList.getLast(0)`: its class isn't translated (it extends `ArrayList` (java.util)) (traben/entity_model_features/models/animation/math/expression_tree/MathExpressionParser.java:414)

### `MathExpressionParser.RollingReader`

- [x] `MathExpressionParser.RollingReader.clear(0)` (traben/entity_model_features/models/animation/math/expression_tree/MathExpressionParser.java:422)
- [x] `MathExpressionParser.RollingReader.write(1)` (traben/entity_model_features/models/animation/math/expression_tree/MathExpressionParser.java:426)
- [x] `MathExpressionParser.RollingReader.read(0)` (traben/entity_model_features/models/animation/math/expression_tree/MathExpressionParser.java:430)
- [x] `MathExpressionParser.RollingReader.toString(0)` (traben/entity_model_features/models/animation/math/expression_tree/MathExpressionParser.java:436)
- [x] `MathExpressionParser.RollingReader.isEmpty(0)` (traben/entity_model_features/models/animation/math/expression_tree/MathExpressionParser.java:441)

### `MathMethod`: it extends `MathValue`

- [ ] `MathMethod.MathMethod(3)`: its class isn't translated (it extends `MathValue`) (traben/entity_model_features/models/animation/math/expression_tree/MathMethod.java:28)
- [ ] `MathMethod.MathMethod(4)`: its class isn't translated (it extends `MathValue`) (traben/entity_model_features/models/animation/math/expression_tree/MathMethod.java:32)
- [ ] `MathMethod.MathMethod(5)`: its class isn't translated (it extends `MathValue`) (traben/entity_model_features/models/animation/math/expression_tree/MathMethod.java:36)
- [ ] `MathMethod.parseArg(3)`: its class isn't translated (it extends `MathValue`) (traben/entity_model_features/models/animation/math/expression_tree/MathMethod.java:58)
- [ ] `MathMethod.parseAllArgs(2)`: its class isn't translated (it extends `MathValue`) (traben/entity_model_features/models/animation/math/expression_tree/MathMethod.java:76)
- [ ] `MathMethod.of(4)`: it uses MathMethod, which isn't translated (it extends `MathValue`) (traben/entity_model_features/models/animation/math/expression_tree/MathMethod.java:90)
- [ ] `MathMethod.getArgsList(1)`: Stream.iterator (traben/entity_model_features/models/animation/math/expression_tree/MathMethod.java:106)
- [ ] `MathMethod.getOptimizedExpression(4)`: it uses MathComponent, which isn't translated (it's an interface) (traben/entity_model_features/models/animation/math/expression_tree/MathMethod.java:142)
- [ ] `MathMethod.setOptimizedAlternativeToThis(1)`: its class isn't translated (it extends `MathValue`) (traben/entity_model_features/models/animation/math/expression_tree/MathMethod.java:152)
- [ ] `MathMethod.canOptimizeForConstantArgs(0)`: its class isn't translated (it extends `MathValue`) (traben/entity_model_features/models/animation/math/expression_tree/MathMethod.java:156)
- [ ] `MathMethod.setSupplierAndOptimize(1)`: its class isn't translated (it extends `MathValue`) (traben/entity_model_features/models/animation/math/expression_tree/MathMethod.java:160)
- [ ] `MathMethod.setSupplierAndOptimize(2)`: its class isn't translated (it extends `MathValue`) (traben/entity_model_features/models/animation/math/expression_tree/MathMethod.java:164)
- [ ] `MathMethod.setSupplierAndOptimize(2)`: its class isn't translated (it extends `MathValue`) (traben/entity_model_features/models/animation/math/expression_tree/MathMethod.java:169)
- [ ] `MathMethod.invertSupplierBoolean(0)`: its class isn't translated (it extends `MathValue`) (traben/entity_model_features/models/animation/math/expression_tree/MathMethod.java:174)
- [ ] `MathMethod.setOptimizedIfPossible(2)`: its class isn't translated (it extends `MathValue`) (traben/entity_model_features/models/animation/math/expression_tree/MathMethod.java:184)
- [ ] `MathMethod.getResultSupplier(0)`: its class isn't translated (it extends `MathValue`) (traben/entity_model_features/models/animation/math/expression_tree/MathMethod.java:196)
- [ ] `MathMethod.asmVisit(2)`: its class isn't translated (it extends `MathValue`) (traben/entity_model_features/models/animation/math/expression_tree/MathMethod.java:201)
- [ ] `MathMethod.asmVisitInner(2)`: it has no body (traben/entity_model_features/models/animation/math/expression_tree/MathMethod.java:213)
- [ ] `MathMethod.isRawStringArg(1)`: its class isn't translated (it extends `MathValue`) (traben/entity_model_features/models/animation/math/expression_tree/MathMethod.java:215)
- [ ] `MathMethod.hasCorrectArgCount(1)`: it has no body (traben/entity_model_features/models/animation/math/expression_tree/MathMethod.java:219)

### `MathOperator`: its constants carry values

- [ ] `MathOperator.getAction(1)`: it uses MathOperator, which isn't translated (its constants carry values) (traben/entity_model_features/models/animation/math/expression_tree/MathOperator.java:186)
- [ ] `MathOperator.execute(2)`: its class isn't translated (its constants carry values) (traben/entity_model_features/models/animation/math/expression_tree/MathOperator.java:201)
- [ ] `MathOperator.isConstant(0)`: its class isn't translated (its constants carry values) (traben/entity_model_features/models/animation/math/expression_tree/MathOperator.java:206)
- [ ] `MathOperator.getResult(0)`: its class isn't translated (its constants carry values) (traben/entity_model_features/models/animation/math/expression_tree/MathOperator.java:211)
- [ ] `MathOperator.isScopeBool(0)`: its class isn't translated (its constants carry values) (traben/entity_model_features/models/animation/math/expression_tree/MathOperator.java:217)
- [ ] `MathOperator.isEqualsType(0)`: its class isn't translated (its constants carry values) (traben/entity_model_features/models/animation/math/expression_tree/MathOperator.java:221)
- [ ] `MathOperator.asmVisit(2)`: its class isn't translated (its constants carry values) (traben/entity_model_features/models/animation/math/expression_tree/MathOperator.java:225)
- [ ] `MathOperator.asmCompare(2)`: it uses `MethodVisitor` (org.objectweb.asm) (traben/entity_model_features/models/animation/math/expression_tree/MathOperator.java:230)

### `MathValue`: it implements `MathComponent`

- [x] `MathValue.TRUE` (traben/entity_model_features/models/animation/math/expression_tree/MathValue.java:6)
- [x] `MathValue.FALSE` (traben/entity_model_features/models/animation/math/expression_tree/MathValue.java:7)
- [ ] `MathValue.MathValue(1)`: its class isn't translated (it implements `MathComponent`) (traben/entity_model_features/models/animation/math/expression_tree/MathValue.java:10)
- [ ] `MathValue.MathValue(0)`: its class isn't translated (it implements `MathComponent`) (traben/entity_model_features/models/animation/math/expression_tree/MathValue.java:14)
- [ ] `MathValue.fromBoolean(1)`: it's overloaded with the same number of parameters (traben/entity_model_features/models/animation/math/expression_tree/MathValue.java:18)
- [x] `MathValue.toBoolean(1)` (traben/entity_model_features/models/animation/math/expression_tree/MathValue.java:22)
- [x] `MathValue.validateBoolean(1)` (traben/entity_model_features/models/animation/math/expression_tree/MathValue.java:32)
- [ ] `MathValue.invertBoolean(1)`: it's overloaded with the same number of parameters (traben/entity_model_features/models/animation/math/expression_tree/MathValue.java:37)
- [ ] `MathValue.invertBoolean(1)`: it's overloaded with the same number of parameters (traben/entity_model_features/models/animation/math/expression_tree/MathValue.java:41)
- [ ] `MathValue.invertBoolean(1)`: it's overloaded with the same number of parameters (traben/entity_model_features/models/animation/math/expression_tree/MathValue.java:45)
- [ ] `MathValue.fromBoolean(1)`: it's overloaded with the same number of parameters (traben/entity_model_features/models/animation/math/expression_tree/MathValue.java:49)
- [ ] `MathValue.invertBoolean(1)`: it's overloaded with the same number of parameters (traben/entity_model_features/models/animation/math/expression_tree/MathValue.java:53)
- [x] `MathValue.isBoolean(1)` (traben/entity_model_features/models/animation/math/expression_tree/MathValue.java:57)
- [ ] `MathValue.getResultSupplier(0)`: it has no body (traben/entity_model_features/models/animation/math/expression_tree/MathValue.java:61)
- [ ] `MathValue.getResult(0)`: its class isn't translated (it implements `MathComponent`) (traben/entity_model_features/models/animation/math/expression_tree/MathValue.java:63)

### `MathValue.ResultSupplier`: it's an interface

- [ ] `MathValue.ResultSupplier.get(0)`: it has no body (traben/entity_model_features/models/animation/math/expression_tree/MathValue.java:69)

### `MathVariable`: it extends `MathValue`

- [ ] `MathVariable.MathVariable(3)`: its class isn't translated (it extends `MathValue`) (traben/entity_model_features/models/animation/math/expression_tree/MathVariable.java:12)
- [ ] `MathVariable.MathVariable(2)`: its class isn't translated (it extends `MathValue`) (traben/entity_model_features/models/animation/math/expression_tree/MathVariable.java:18)
- [ ] `MathVariable.getOptimizedVariable(3)`: it uses MathComponent, which isn't translated (it's an interface) (traben/entity_model_features/models/animation/math/expression_tree/MathVariable.java:23)
- [ ] `MathVariable.getResultSupplier(0)`: its class isn't translated (it extends `MathValue`) (traben/entity_model_features/models/animation/math/expression_tree/MathVariable.java:29)
- [ ] `MathVariable.toString(0)`: its class isn't translated (it extends `MathValue`) (traben/entity_model_features/models/animation/math/expression_tree/MathVariable.java:34)
- [ ] `MathVariable.asmVisit(2)`: its class isn't translated (it extends `MathValue`) (traben/entity_model_features/models/animation/math/expression_tree/MathVariable.java:39)

### `OldEMFAnimationHandler`: it extends `EMFAnimationHandler` (traben.entity_model_features.models.animation)

- [ ] `OldEMFAnimationHandler.OldEMFAnimationHandler(1)`: its class isn't translated (it extends `EMFAnimationHandler` (traben.entity_model_features.models.animation)) (traben/entity_model_features/models/animation/math/expression_tree/OldEMFAnimationHandler.java:29)
- [ ] `OldEMFAnimationHandler.addParsedLine(2)`: its class isn't translated (it extends `EMFAnimationHandler` (traben.entity_model_features.models.animation)) (traben/entity_model_features/models/animation/math/expression_tree/OldEMFAnimationHandler.java:33)
- [ ] `OldEMFAnimationHandler.getLastResultGetter(1)`: its class isn't translated (it extends `EMFAnimationHandler` (traben.entity_model_features.models.animation)) (traben/entity_model_features/models/animation/math/expression_tree/OldEMFAnimationHandler.java:38)
- [ ] `OldEMFAnimationHandler.prevResultsOfEntity(0)`: its class isn't translated (it extends `EMFAnimationHandler` (traben.entity_model_features.models.animation)) (traben/entity_model_features/models/animation/math/expression_tree/OldEMFAnimationHandler.java:42)
- [ ] `OldEMFAnimationHandler.lastResult(1)`: its class isn't translated (it extends `EMFAnimationHandler` (traben.entity_model_features.models.animation)) (traben/entity_model_features/models/animation/math/expression_tree/OldEMFAnimationHandler.java:47)
- [ ] `OldEMFAnimationHandler.finishAndValidate(0)`: its class isn't translated (it extends `EMFAnimationHandler` (traben.entity_model_features.models.animation)) (traben/entity_model_features/models/animation/math/expression_tree/OldEMFAnimationHandler.java:57)
- [ ] `OldEMFAnimationHandler.animateInner(1)`: its class isn't translated (it extends `EMFAnimationHandler` (traben.entity_model_features.models.animation)) (traben/entity_model_features/models/animation/math/expression_tree/OldEMFAnimationHandler.java:96)

### `CatchMethod`: it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)

- [ ] `CatchMethod.CatchMethod(3)`: its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)) (traben/entity_model_features/models/animation/math/methods/emf/CatchMethod.java:14)
- [ ] `CatchMethod.canOptimizeForConstantArgs(0)`: its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)) (traben/entity_model_features/models/animation/math/methods/emf/CatchMethod.java:47)
- [ ] `CatchMethod.asmVisitInner(2)`: its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)) (traben/entity_model_features/models/animation/math/methods/emf/CatchMethod.java:52)
- [ ] `CatchMethod.hasCorrectArgCount(1)`: its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)) (traben/entity_model_features/models/animation/math/methods/emf/CatchMethod.java:70)

### `IfBMethod`: it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)

- [ ] `IfBMethod.IfBMethod(3)`: its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)) (traben/entity_model_features/models/animation/math/methods/emf/IfBMethod.java:16)
- [ ] `IfBMethod.hasCorrectArgCount(1)`: its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)) (traben/entity_model_features/models/animation/math/methods/emf/IfBMethod.java:67)
- [ ] `IfBMethod.asmVisitInner(2)`: its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)) (traben/entity_model_features/models/animation/math/methods/emf/IfBMethod.java:72)
- [ ] `IfBMethod.deepIf(3)`: its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)) (traben/entity_model_features/models/animation/math/methods/emf/IfBMethod.java:86)

### `KeyframeLoopMethod`: it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)

- [ ] `KeyframeLoopMethod.KeyframeLoopMethod(3)`: its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)) (traben/entity_model_features/models/animation/math/methods/emf/KeyframeLoopMethod.java:16)
- [ ] `KeyframeLoopMethod.keyFrameLoopStatic(2)`: it calls Minecraft's `Mth`.catmullrom (traben/entity_model_features/models/animation/math/methods/emf/KeyframeLoopMethod.java:50)
- [ ] `KeyframeLoopMethod.asmVisitInner(2)`: its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)) (traben/entity_model_features/models/animation/math/methods/emf/KeyframeLoopMethod.java:61)
- [ ] `KeyframeLoopMethod.hasCorrectArgCount(1)`: its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)) (traben/entity_model_features/models/animation/math/methods/emf/KeyframeLoopMethod.java:80)

### `KeyframeMethod`: it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)

- [ ] `KeyframeMethod.KeyframeMethod(3)`: its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)) (traben/entity_model_features/models/animation/math/methods/emf/KeyframeMethod.java:16)
- [ ] `KeyframeMethod.keyFrameStatic(2)`: it calls Minecraft's `Mth`.catmullrom (traben/entity_model_features/models/animation/math/methods/emf/KeyframeMethod.java:64)
- [ ] `KeyframeMethod.asmVisitInner(2)`: its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)) (traben/entity_model_features/models/animation/math/methods/emf/KeyframeMethod.java:83)
- [ ] `KeyframeMethod.hasCorrectArgCount(1)`: its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)) (traben/entity_model_features/models/animation/math/methods/emf/KeyframeMethod.java:102)

### `NBTMethod`: it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)

- [ ] `NBTMethod.CACHE`: it uses `NBTProperty` (traben.entity_texture_features.features.property_reading.properties.optifine_properties) (traben/entity_model_features/models/animation/math/methods/emf/NBTMethod.java:20)
- [ ] `NBTMethod.NULL`: it uses `NBTProperty` (traben.entity_texture_features.features.property_reading.properties.optifine_properties) (traben/entity_model_features/models/animation/math/methods/emf/NBTMethod.java:21)
- [ ] `NBTMethod.NBTMethod(3)`: its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)) (traben/entity_model_features/models/animation/math/methods/emf/NBTMethod.java:23)
- [ ] `NBTMethod.nullz(0)`: it uses `NBTProperty` (traben.entity_texture_features.features.property_reading.properties.optifine_properties) (traben/entity_model_features/models/animation/math/methods/emf/NBTMethod.java:40)
- [ ] `NBTMethod.nbtMethodStatic(2)`: it uses `NBTProperty` (traben.entity_texture_features.features.property_reading.properties.optifine_properties) (traben/entity_model_features/models/animation/math/methods/emf/NBTMethod.java:46)
- [ ] `NBTMethod.asmVisitInner(2)`: its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)) (traben/entity_model_features/models/animation/math/methods/emf/NBTMethod.java:61)
- [ ] `NBTMethod.canOptimizeForConstantArgs(0)`: its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)) (traben/entity_model_features/models/animation/math/methods/emf/NBTMethod.java:68)
- [ ] `NBTMethod.hasCorrectArgCount(1)`: its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)) (traben/entity_model_features/models/animation/math/methods/emf/NBTMethod.java:73)
- [ ] `NBTMethod.isRawStringArg(1)`: its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)) (traben/entity_model_features/models/animation/math/methods/emf/NBTMethod.java:78)

### `RandomBMethod`: it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)

- [ ] `RandomBMethod.RandomBMethod(3)`: its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)) (traben/entity_model_features/models/animation/math/methods/emf/RandomBMethod.java:17)
- [ ] `RandomBMethod.nextValue(1)`: it calls RandomMethod.nextValue, which isn't translated (traben/entity_model_features/models/animation/math/methods/emf/RandomBMethod.java:28)
- [ ] `RandomBMethod.nextValueBasic(0)`: it calls RandomMethod.nextValueBasic, which isn't translated (traben/entity_model_features/models/animation/math/methods/emf/RandomBMethod.java:32)
- [ ] `RandomBMethod.asmVisitInner(2)`: its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)) (traben/entity_model_features/models/animation/math/methods/emf/RandomBMethod.java:36)
- [ ] `RandomBMethod.hasCorrectArgCount(1)`: its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)) (traben/entity_model_features/models/animation/math/methods/emf/RandomBMethod.java:48)

### `MethodRegistry`: it uses MethodFactory, which isn't translated (it's an interface)

- [ ] `MethodRegistry.INSTANCE`: it uses MethodRegistry, which isn't translated (it uses MethodFactory, which isn't translated (it's an interface)) (traben/entity_model_features/models/animation/math/methods/MethodRegistry.java:31)
- [ ] `MethodRegistry.MethodRegistry(0)`: its class isn't translated (it uses MethodFactory, which isn't translated (it's an interface)) (traben/entity_model_features/models/animation/math/methods/MethodRegistry.java:35)
- [x] `MethodRegistry.emfTranslationKey(1)` (traben/entity_model_features/models/animation/math/methods/MethodRegistry.java:123)
- [ ] `MethodRegistry.getInstance(0)`: it uses MethodRegistry, which isn't translated (it uses MethodFactory, which isn't translated (it's an interface)) (traben/entity_model_features/models/animation/math/methods/MethodRegistry.java:127)
- [ ] `MethodRegistry.getMethodExplanationTranslationKeys(0)`: its class isn't translated (it uses MethodFactory, which isn't translated (it's an interface)) (traben/entity_model_features/models/animation/math/methods/MethodRegistry.java:131)
- [ ] `MethodRegistry.registerHelperMethodFactory(1)`: its class isn't translated (it uses MethodFactory, which isn't translated (it's an interface)) (traben/entity_model_features/models/animation/math/methods/MethodRegistry.java:135)
- [ ] `MethodRegistry.registerHelperMethodFactory(2)`: its class isn't translated (it uses MethodFactory, which isn't translated (it's an interface)) (traben/entity_model_features/models/animation/math/methods/MethodRegistry.java:141)
- [ ] `MethodRegistry.registerSimpleMethodFactory(4)`: its class isn't translated (it uses MethodFactory, which isn't translated (it's an interface)) (traben/entity_model_features/models/animation/math/methods/MethodRegistry.java:147)
- [ ] `MethodRegistry.registerSimpleMethodFactory(3)`: its class isn't translated (it uses MethodFactory, which isn't translated (it's an interface)) (traben/entity_model_features/models/animation/math/methods/MethodRegistry.java:151)
- [ ] `MethodRegistry.registerSimpleMethodFactory(3)`: its class isn't translated (it uses MethodFactory, which isn't translated (it's an interface)) (traben/entity_model_features/models/animation/math/methods/MethodRegistry.java:156)
- [ ] `MethodRegistry.registerSimpleMethodFactory(3)`: its class isn't translated (it uses MethodFactory, which isn't translated (it's an interface)) (traben/entity_model_features/models/animation/math/methods/MethodRegistry.java:161)
- [ ] `MethodRegistry.registerSimpleMultiMethodFactory(3)`: its class isn't translated (it uses MethodFactory, which isn't translated (it's an interface)) (traben/entity_model_features/models/animation/math/methods/MethodRegistry.java:166)
- [ ] `MethodRegistry.register(3)`: its class isn't translated (it uses MethodFactory, which isn't translated (it's an interface)) (traben/entity_model_features/models/animation/math/methods/MethodRegistry.java:171)
- [ ] `MethodRegistry.registerAndWrapMethodFactory(2)`: its class isn't translated (it uses MethodFactory, which isn't translated (it's an interface)) (traben/entity_model_features/models/animation/math/methods/MethodRegistry.java:176)
- [ ] `MethodRegistry.registerAndWrapMethodFactory(3)`: its class isn't translated (it uses MethodFactory, which isn't translated (it's an interface)) (traben/entity_model_features/models/animation/math/methods/MethodRegistry.java:180)
- [ ] `MethodRegistry.containsMethod(1)`: its class isn't translated (it uses MethodFactory, which isn't translated (it's an interface)) (traben/entity_model_features/models/animation/math/methods/MethodRegistry.java:190)
- [ ] `MethodRegistry.getMethodFactory(1)`: its class isn't translated (it uses MethodFactory, which isn't translated (it's an interface)) (traben/entity_model_features/models/animation/math/methods/MethodRegistry.java:194)

### `MethodRegistry.MethodFactory`: it's an interface

- [ ] `MethodRegistry.MethodFactory.getMethod(3)`: it has no body (traben/entity_model_features/models/animation/math/methods/MethodRegistry.java:199)

### `IfMethod`: it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)

- [ ] `IfMethod.IfMethod(3)`: its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)) (traben/entity_model_features/models/animation/math/methods/optifine/IfMethod.java:16)
- [ ] `IfMethod.hasCorrectArgCount(1)`: its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)) (traben/entity_model_features/models/animation/math/methods/optifine/IfMethod.java:60)
- [ ] `IfMethod.asmVisitInner(2)`: its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)) (traben/entity_model_features/models/animation/math/methods/optifine/IfMethod.java:65)
- [ ] `IfMethod.deepIf(3)`: its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)) (traben/entity_model_features/models/animation/math/methods/optifine/IfMethod.java:84)

### `InMethod`: it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)

- [ ] `InMethod.InMethod(3)`: its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)) (traben/entity_model_features/models/animation/math/methods/optifine/InMethod.java:14)
- [ ] `InMethod.asmVisitInner(2)`: its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)) (traben/entity_model_features/models/animation/math/methods/optifine/InMethod.java:32)
- [ ] `InMethod.hasCorrectArgCount(1)`: its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)) (traben/entity_model_features/models/animation/math/methods/optifine/InMethod.java:57)

### `MaxMethod`: it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)

- [ ] `MaxMethod.MaxMethod(3)`: its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)) (traben/entity_model_features/models/animation/math/methods/optifine/MaxMethod.java:13)
- [ ] `MaxMethod.hasCorrectArgCount(1)`: its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)) (traben/entity_model_features/models/animation/math/methods/optifine/MaxMethod.java:32)
- [ ] `MaxMethod.asmVisitInner(2)`: its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)) (traben/entity_model_features/models/animation/math/methods/optifine/MaxMethod.java:37)

### `MinMethod`: it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)

- [ ] `MinMethod.MinMethod(3)`: its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)) (traben/entity_model_features/models/animation/math/methods/optifine/MinMethod.java:13)
- [ ] `MinMethod.asmVisitInner(2)`: its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)) (traben/entity_model_features/models/animation/math/methods/optifine/MinMethod.java:32)
- [ ] `MinMethod.hasCorrectArgCount(1)`: its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)) (traben/entity_model_features/models/animation/math/methods/optifine/MinMethod.java:48)

### `PrintBMethod`: it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)

- [ ] `PrintBMethod.PrintBMethod(3)`: its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)) (traben/entity_model_features/models/animation/math/methods/optifine/PrintBMethod.java:20)
- [ ] `PrintBMethod.getPrintCount(0)`: its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)) (traben/entity_model_features/models/animation/math/methods/optifine/PrintBMethod.java:48)
- [ ] `PrintBMethod.printStaticOne(2)`: it calls Minecraft's Minecraft.isPaused (no Horizon helper for it yet) (traben/entity_model_features/models/animation/math/methods/optifine/PrintBMethod.java:53)
- [ ] `PrintBMethod.printStatic(4)`: it calls Minecraft's Minecraft.isPaused (no Horizon helper for it yet) (traben/entity_model_features/models/animation/math/methods/optifine/PrintBMethod.java:61)
- [ ] `PrintBMethod.canOptimizeForConstantArgs(0)`: its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)) (traben/entity_model_features/models/animation/math/methods/optifine/PrintBMethod.java:69)
- [ ] `PrintBMethod.asmVisitInner(2)`: its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)) (traben/entity_model_features/models/animation/math/methods/optifine/PrintBMethod.java:74)
- [ ] `PrintBMethod.hasCorrectArgCount(1)`: its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)) (traben/entity_model_features/models/animation/math/methods/optifine/PrintBMethod.java:95)
- [ ] `PrintBMethod.isRawStringArg(1)`: its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)) (traben/entity_model_features/models/animation/math/methods/optifine/PrintBMethod.java:100)

### `PrintMethod`: it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)

- [ ] `PrintMethod.PrintMethod(3)`: its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)) (traben/entity_model_features/models/animation/math/methods/optifine/PrintMethod.java:18)
- [ ] `PrintMethod.getPrintCount(0)`: its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)) (traben/entity_model_features/models/animation/math/methods/optifine/PrintMethod.java:46)
- [ ] `PrintMethod.printStaticOne(2)`: it calls Minecraft's Minecraft.isPaused (no Horizon helper for it yet) (traben/entity_model_features/models/animation/math/methods/optifine/PrintMethod.java:51)
- [ ] `PrintMethod.printStatic(4)`: it calls Minecraft's Minecraft.isPaused (no Horizon helper for it yet) (traben/entity_model_features/models/animation/math/methods/optifine/PrintMethod.java:59)
- [ ] `PrintMethod.canOptimizeForConstantArgs(0)`: its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)) (traben/entity_model_features/models/animation/math/methods/optifine/PrintMethod.java:67)
- [ ] `PrintMethod.asmVisitInner(2)`: its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)) (traben/entity_model_features/models/animation/math/methods/optifine/PrintMethod.java:72)
- [ ] `PrintMethod.hasCorrectArgCount(1)`: its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)) (traben/entity_model_features/models/animation/math/methods/optifine/PrintMethod.java:90)
- [ ] `PrintMethod.isRawStringArg(1)`: its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)) (traben/entity_model_features/models/animation/math/methods/optifine/PrintMethod.java:95)

### `RandomMethod`: it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)

- [ ] `RandomMethod.RandomMethod(3)`: its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)) (traben/entity_model_features/models/animation/math/methods/optifine/RandomMethod.java:15)
- [ ] `RandomMethod.nextValue(1)`: it calls `Float`.floatToIntBits (traben/entity_model_features/models/animation/math/methods/optifine/RandomMethod.java:26)
- [ ] `RandomMethod.nextValueBasic(0)`: Math.random (no random numbers without a crate) (traben/entity_model_features/models/animation/math/methods/optifine/RandomMethod.java:31)
- [x] `RandomMethod.optifineIntHash(1)` (traben/entity_model_features/models/animation/math/methods/optifine/RandomMethod.java:35)
- [ ] `RandomMethod.canOptimizeForConstantArgs(0)`: its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)) (traben/entity_model_features/models/animation/math/methods/optifine/RandomMethod.java:43)
- [ ] `RandomMethod.asmVisitInner(2)`: its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)) (traben/entity_model_features/models/animation/math/methods/optifine/RandomMethod.java:48)
- [ ] `RandomMethod.hasCorrectArgCount(1)`: its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)) (traben/entity_model_features/models/animation/math/methods/optifine/RandomMethod.java:60)

### `SimpleMethod`: it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)

- [ ] `SimpleMethod.SimpleMethod(5)`: its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)) (traben/entity_model_features/models/animation/math/methods/SimpleMethod.java:24)
- [ ] `SimpleMethod.makeFactory(3)`: it uses MethodFactory, which isn't translated (it's an interface) (traben/entity_model_features/models/animation/math/methods/SimpleMethod.java:55)
- [ ] `SimpleMethod.asmVisitInner(2)`: its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)) (traben/entity_model_features/models/animation/math/methods/SimpleMethod.java:75)
- [ ] `SimpleMethod.getParameterCount(1)`: it uses `Method` (java.lang.reflect) (traben/entity_model_features/models/animation/math/methods/SimpleMethod.java:102)
- [ ] `SimpleMethod.getStringArgs(1)`: it uses `Method` (java.lang.reflect) (traben/entity_model_features/models/animation/math/methods/SimpleMethod.java:106)
- [ ] `SimpleMethod.hasCorrectArgCount(1)`: its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)) (traben/entity_model_features/models/animation/math/methods/SimpleMethod.java:118)

### `BiFunctionMethods`: it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)

- [ ] `BiFunctionMethods.BiFunctionMethods(4)`: its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)) (traben/entity_model_features/models/animation/math/methods/simple/BiFunctionMethods.java:14)
- [ ] `BiFunctionMethods.makeFactory(2)`: it uses MethodFactory, which isn't translated (it's an interface) (traben/entity_model_features/models/animation/math/methods/simple/BiFunctionMethods.java:19)
- [ ] `BiFunctionMethods.asmVisitInner(2)`: its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)) (traben/entity_model_features/models/animation/math/methods/simple/BiFunctionMethods.java:24)
- [ ] `BiFunctionMethods.hasCorrectArgCount(1)`: its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)) (traben/entity_model_features/models/animation/math/methods/simple/BiFunctionMethods.java:29)

### `FunctionMethods`: it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)

- [ ] `FunctionMethods.FunctionMethods(4)`: its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)) (traben/entity_model_features/models/animation/math/methods/simple/FunctionMethods.java:14)
- [ ] `FunctionMethods.makeFactory(2)`: it uses MethodFactory, which isn't translated (it's an interface) (traben/entity_model_features/models/animation/math/methods/simple/FunctionMethods.java:19)
- [ ] `FunctionMethods.asmVisitInner(2)`: its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)) (traben/entity_model_features/models/animation/math/methods/simple/FunctionMethods.java:24)
- [ ] `FunctionMethods.hasCorrectArgCount(1)`: its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)) (traben/entity_model_features/models/animation/math/methods/simple/FunctionMethods.java:29)

### `MultiFunctionMethods`: it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)

- [ ] `MultiFunctionMethods.MultiFunctionMethods(4)`: its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)) (traben/entity_model_features/models/animation/math/methods/simple/MultiFunctionMethods.java:14)
- [ ] `MultiFunctionMethods.makeFactory(2)`: it uses MethodFactory, which isn't translated (it's an interface) (traben/entity_model_features/models/animation/math/methods/simple/MultiFunctionMethods.java:18)
- [ ] `MultiFunctionMethods.asmVisitInner(2)`: its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)) (traben/entity_model_features/models/animation/math/methods/simple/MultiFunctionMethods.java:23)
- [ ] `MultiFunctionMethods.hasCorrectArgCount(1)`: its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)) (traben/entity_model_features/models/animation/math/methods/simple/MultiFunctionMethods.java:28)

### `TriFunctionMethods`: it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)

- [ ] `TriFunctionMethods.TriFunctionMethods(4)`: its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)) (traben/entity_model_features/models/animation/math/methods/simple/TriFunctionMethods.java:14)
- [ ] `TriFunctionMethods.makeFactory(2)`: it uses MethodFactory, which isn't translated (it's an interface) (traben/entity_model_features/models/animation/math/methods/simple/TriFunctionMethods.java:19)
- [ ] `TriFunctionMethods.asmVisitInner(2)`: its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)) (traben/entity_model_features/models/animation/math/methods/simple/TriFunctionMethods.java:24)
- [ ] `TriFunctionMethods.hasCorrectArgCount(1)`: its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)) (traben/entity_model_features/models/animation/math/methods/simple/TriFunctionMethods.java:29)

### `EMFModelOrRenderVariable`: its constants carry values

- [ ] `EMFModelOrRenderVariable.getRenderVariable(1)`: it uses EMFModelOrRenderVariable, which isn't translated (its constants carry values) (traben/entity_model_features/models/animation/math/variables/EMFModelOrRenderVariable.java:945)
- [ ] `EMFModelOrRenderVariable.get(1)`: it uses EMFModelOrRenderVariable, which isn't translated (its constants carry values) (traben/entity_model_features/models/animation/math/variables/EMFModelOrRenderVariable.java:995)
- [ ] `EMFModelOrRenderVariable.isRenderVariable(0)`: its class isn't translated (its constants carry values) (traben/entity_model_features/models/animation/math/variables/EMFModelOrRenderVariable.java:1017)
- [ ] `EMFModelOrRenderVariable.isBoolean(0)`: its class isn't translated (its constants carry values) (traben/entity_model_features/models/animation/math/variables/EMFModelOrRenderVariable.java:1021)
- [ ] `EMFModelOrRenderVariable.getValue(1)`: it has no body (traben/entity_model_features/models/animation/math/variables/EMFModelOrRenderVariable.java:1025)
- [ ] `EMFModelOrRenderVariable.getValue(0)`: its class isn't translated (its constants carry values) (traben/entity_model_features/models/animation/math/variables/EMFModelOrRenderVariable.java:1027)
- [ ] `EMFModelOrRenderVariable.setValue(2)`: it has no body (traben/entity_model_features/models/animation/math/variables/EMFModelOrRenderVariable.java:1031)

### `GlobalVariableFactory`: it extends `UniqueVariableFactory`

- [x] `GlobalVariableFactory.globalVariables` (traben/entity_model_features/models/animation/math/variables/factories/GlobalVariableFactory.java:11)
- [x] `GlobalVariableFactory.clear(0)` (traben/entity_model_features/models/animation/math/variables/factories/GlobalVariableFactory.java:13)
- [x] `GlobalVariableFactory.setGlobalVariable(2)` (traben/entity_model_features/models/animation/math/variables/factories/GlobalVariableFactory.java:17)
- [x] `GlobalVariableFactory.getGlobalVariable(1)` (traben/entity_model_features/models/animation/math/variables/factories/GlobalVariableFactory.java:21)
- [ ] `GlobalVariableFactory.getSupplierOrNull(2)`: its class isn't translated (it extends `UniqueVariableFactory`) (traben/entity_model_features/models/animation/math/variables/factories/GlobalVariableFactory.java:25)
- [ ] `GlobalVariableFactory.getASMBoolSupplierOrNull(2)`: its class isn't translated (it extends `UniqueVariableFactory`) (traben/entity_model_features/models/animation/math/variables/factories/GlobalVariableFactory.java:30)
- [ ] `GlobalVariableFactory.getASMFloatSupplierOrNull(2)`: its class isn't translated (it extends `UniqueVariableFactory`) (traben/entity_model_features/models/animation/math/variables/factories/GlobalVariableFactory.java:36)
- [ ] `GlobalVariableFactory.createsThisVariable(1)`: its class isn't translated (it extends `UniqueVariableFactory`) (traben/entity_model_features/models/animation/math/variables/factories/GlobalVariableFactory.java:42)
- [ ] `GlobalVariableFactory.getExplanationTranslationKey(0)`: its class isn't translated (it extends `UniqueVariableFactory`) (traben/entity_model_features/models/animation/math/variables/factories/GlobalVariableFactory.java:47)
- [ ] `GlobalVariableFactory.getTitleTranslationKey(0)`: its class isn't translated (it extends `UniqueVariableFactory`) (traben/entity_model_features/models/animation/math/variables/factories/GlobalVariableFactory.java:53)

### `ModelPartVariableFactory`: it extends `UniqueVariableFactory`

- [ ] `ModelPartVariableFactory.getSupplierOrNull(2)`: its class isn't translated (it extends `UniqueVariableFactory`) (traben/entity_model_features/models/animation/math/variables/factories/ModelPartVariableFactory.java:15)
- [ ] `ModelPartVariableFactory.getASMBoolSupplierOrNull(2)`: its class isn't translated (it extends `UniqueVariableFactory`) (traben/entity_model_features/models/animation/math/variables/factories/ModelPartVariableFactory.java:56)
- [ ] `ModelPartVariableFactory.getASMFloatSupplierOrNull(2)`: its class isn't translated (it extends `UniqueVariableFactory`) (traben/entity_model_features/models/animation/math/variables/factories/ModelPartVariableFactory.java:89)
- [ ] `ModelPartVariableFactory.createsThisVariable(1)`: its class isn't translated (it extends `UniqueVariableFactory`) (traben/entity_model_features/models/animation/math/variables/factories/ModelPartVariableFactory.java:129)
- [ ] `ModelPartVariableFactory.getExplanationTranslationKey(0)`: its class isn't translated (it extends `UniqueVariableFactory`) (traben/entity_model_features/models/animation/math/variables/factories/ModelPartVariableFactory.java:148)
- [ ] `ModelPartVariableFactory.getTitleTranslationKey(0)`: its class isn't translated (it extends `UniqueVariableFactory`) (traben/entity_model_features/models/animation/math/variables/factories/ModelPartVariableFactory.java:154)

### `ModelVariableFactory`: it extends `UniqueVariableFactory`

- [ ] `ModelVariableFactory.getSupplierOrNull(2)`: its class isn't translated (it extends `UniqueVariableFactory`) (traben/entity_model_features/models/animation/math/variables/factories/ModelVariableFactory.java:9)
- [ ] `ModelVariableFactory.createsThisVariable(1)`: its class isn't translated (it extends `UniqueVariableFactory`) (traben/entity_model_features/models/animation/math/variables/factories/ModelVariableFactory.java:16)
- [ ] `ModelVariableFactory.getExplanationTranslationKey(0)`: its class isn't translated (it extends `UniqueVariableFactory`) (traben/entity_model_features/models/animation/math/variables/factories/ModelVariableFactory.java:21)
- [ ] `ModelVariableFactory.getTitleTranslationKey(0)`: its class isn't translated (it extends `UniqueVariableFactory`) (traben/entity_model_features/models/animation/math/variables/factories/ModelVariableFactory.java:27)

### `RenderVariableFactory`: it extends `UniqueVariableFactory`

- [ ] `RenderVariableFactory.getSupplierOrNull(2)`: its class isn't translated (it extends `UniqueVariableFactory`) (traben/entity_model_features/models/animation/math/variables/factories/RenderVariableFactory.java:10)
- [ ] `RenderVariableFactory.createsThisVariable(1)`: its class isn't translated (it extends `UniqueVariableFactory`) (traben/entity_model_features/models/animation/math/variables/factories/RenderVariableFactory.java:29)
- [ ] `RenderVariableFactory.getExplanationTranslationKey(0)`: its class isn't translated (it extends `UniqueVariableFactory`) (traben/entity_model_features/models/animation/math/variables/factories/RenderVariableFactory.java:34)
- [ ] `RenderVariableFactory.getTitleTranslationKey(0)`: its class isn't translated (it extends `UniqueVariableFactory`) (traben/entity_model_features/models/animation/math/variables/factories/RenderVariableFactory.java:40)

### `UniqueVariableFactory`

- [ ] `UniqueVariableFactory.equals(1)`: it uses `Object` (traben/entity_model_features/models/animation/math/variables/factories/UniqueVariableFactory.java:11)
- [ ] `UniqueVariableFactory.getSupplierOrNull(2)`: it has no body (traben/entity_model_features/models/animation/math/variables/factories/UniqueVariableFactory.java:16)
- [ ] `UniqueVariableFactory.getASMBoolSupplierOrNull(2)`: it uses AnimSetupContext, which isn't translated (it implements `AutoCloseable`) (traben/entity_model_features/models/animation/math/variables/factories/UniqueVariableFactory.java:19)
- [ ] `UniqueVariableFactory.getASMFloatSupplierOrNull(2)`: it uses ResultSupplier, which isn't translated (it's an interface) (traben/entity_model_features/models/animation/math/variables/factories/UniqueVariableFactory.java:25)
- [ ] `UniqueVariableFactory.createsThisVariable(1)`: it has no body (traben/entity_model_features/models/animation/math/variables/factories/UniqueVariableFactory.java:30)
- [ ] `UniqueVariableFactory.getExplanationTranslationKey(0)`: it has no body (traben/entity_model_features/models/animation/math/variables/factories/UniqueVariableFactory.java:32)
- [ ] `UniqueVariableFactory.getTitleTranslationKey(0)`: it has no body (traben/entity_model_features/models/animation/math/variables/factories/UniqueVariableFactory.java:35)
- [ ] `UniqueVariableFactory.printing(0)`: it calls EMF.config, which isn't translated (traben/entity_model_features/models/animation/math/variables/factories/UniqueVariableFactory.java:38)

### `VariableRegistry`: it uses MathComponent, which isn't translated (it's an interface)

- [ ] `VariableRegistry.INSTANCE`: it uses VariableRegistry, which isn't translated (it uses MathComponent, which isn't translated (it's an interface)) (traben/entity_model_features/models/animation/math/variables/VariableRegistry.java:33)
- [ ] `VariableRegistry.getASMVarBoolOrDefault(2)`: its class isn't translated (it uses MathComponent, which isn't translated (it's an interface)) (traben/entity_model_features/models/animation/math/variables/VariableRegistry.java:40)
- [ ] `VariableRegistry.getASMVarFloatOrDefault(2)`: its class isn't translated (it uses MathComponent, which isn't translated (it's an interface)) (traben/entity_model_features/models/animation/math/variables/VariableRegistry.java:58)
- [ ] `VariableRegistry.VariableRegistry(0)`: its class isn't translated (it uses MathComponent, which isn't translated (it's an interface)) (traben/entity_model_features/models/animation/math/variables/VariableRegistry.java:76)
- [x] `VariableRegistry.emfTranslationKey(1)` (traben/entity_model_features/models/animation/math/variables/VariableRegistry.java:204)
- [ ] `VariableRegistry.getInstance(0)`: it uses VariableRegistry, which isn't translated (it uses MathComponent, which isn't translated (it's an interface)) (traben/entity_model_features/models/animation/math/variables/VariableRegistry.java:208)
- [ ] `VariableRegistry.getSingletonVariableExplanationTranslationKeys(0)`: its class isn't translated (it uses MathComponent, which isn't translated (it's an interface)) (traben/entity_model_features/models/animation/math/variables/VariableRegistry.java:212)
- [ ] `VariableRegistry.getUniqueVariableFactories(0)`: its class isn't translated (it uses MathComponent, which isn't translated (it's an interface)) (traben/entity_model_features/models/animation/math/variables/VariableRegistry.java:216)
- [ ] `VariableRegistry.registerContextVariable(1)`: its class isn't translated (it uses MathComponent, which isn't translated (it's an interface)) (traben/entity_model_features/models/animation/math/variables/VariableRegistry.java:220)
- [ ] `VariableRegistry.registerSimpleFloatVariable(2)`: its class isn't translated (it uses MathComponent, which isn't translated (it's an interface)) (traben/entity_model_features/models/animation/math/variables/VariableRegistry.java:230)
- [ ] `VariableRegistry.registerSimpleFloatVariable(3)`: its class isn't translated (it uses MathComponent, which isn't translated (it's an interface)) (traben/entity_model_features/models/animation/math/variables/VariableRegistry.java:234)
- [ ] `VariableRegistry.registerSimpleSpellingErrorWarning(1)`: its class isn't translated (it uses MathComponent, which isn't translated (it's an interface)) (traben/entity_model_features/models/animation/math/variables/VariableRegistry.java:245)
- [ ] `VariableRegistry.registerSimpleBoolVariable(2)`: its class isn't translated (it uses MathComponent, which isn't translated (it's an interface)) (traben/entity_model_features/models/animation/math/variables/VariableRegistry.java:263)
- [ ] `VariableRegistry.registerSimpleBoolVariable(3)`: its class isn't translated (it uses MathComponent, which isn't translated (it's an interface)) (traben/entity_model_features/models/animation/math/variables/VariableRegistry.java:267)
- [ ] `VariableRegistry.getVariable(3)`: its class isn't translated (it uses MathComponent, which isn't translated (it's an interface)) (traben/entity_model_features/models/animation/math/variables/VariableRegistry.java:280)
- [ ] `VariableRegistry.printing(0)`: its class isn't translated (it uses MathComponent, which isn't translated (it's an interface)) (traben/entity_model_features/models/animation/math/variables/VariableRegistry.java:329)

### `EMFBipedPose`

- [ ] `EMFBipedPose.EMFBipedPose(1)`: it uses Minecraft's `HumanoidModel` (traben/entity_model_features/models/animation/state/EMFBipedPose.java:17)
- [ ] `EMFBipedPose.applyTo(1)`: it uses Minecraft's `HumanoidModel` (traben/entity_model_features/models/animation/state/EMFBipedPose.java:27)

### `EMFBipedPose.PosePart`

- [ ] `EMFBipedPose.PosePart.PosePart(1)`: it uses Minecraft's `ModelPart` (traben/entity_model_features/models/animation/state/EMFBipedPose.java:62)
- [ ] `EMFBipedPose.PosePart.applyTo(1)`: it uses Minecraft's `ModelPart` (traben/entity_model_features/models/animation/state/EMFBipedPose.java:74)

### `EMFEntityRenderState`: it's an interface

- [ ] `EMFEntityRenderState.emfEntity(0)`: it has no body (traben/entity_model_features/models/animation/state/EMFEntityRenderState.java:36)
- [ ] `EMFEntityRenderState.isManualPlayerState(0)`: it has no body (traben/entity_model_features/models/animation/state/EMFEntityRenderState.java:39)
- [ ] `EMFEntityRenderState.setManualPlayerState(1)`: it has no body (traben/entity_model_features/models/animation/state/EMFEntityRenderState.java:41)
- [ ] `EMFEntityRenderState.manualPlayerState(0)`: it uses EMFEntityRenderState, which isn't translated (it's an interface) (traben/entity_model_features/models/animation/state/EMFEntityRenderState.java:43)
- [ ] `EMFEntityRenderState.prevX(0)`: it has no body (traben/entity_model_features/models/animation/state/EMFEntityRenderState.java:70)
- [ ] `EMFEntityRenderState.x(0)`: it has no body (traben/entity_model_features/models/animation/state/EMFEntityRenderState.java:72)
- [ ] `EMFEntityRenderState.prevY(0)`: it has no body (traben/entity_model_features/models/animation/state/EMFEntityRenderState.java:74)
- [ ] `EMFEntityRenderState.y(0)`: it has no body (traben/entity_model_features/models/animation/state/EMFEntityRenderState.java:76)
- [ ] `EMFEntityRenderState.prevZ(0)`: it has no body (traben/entity_model_features/models/animation/state/EMFEntityRenderState.java:78)
- [ ] `EMFEntityRenderState.z(0)`: it has no body (traben/entity_model_features/models/animation/state/EMFEntityRenderState.java:80)
- [ ] `EMFEntityRenderState.prevPitch(0)`: it has no body (traben/entity_model_features/models/animation/state/EMFEntityRenderState.java:82)
- [ ] `EMFEntityRenderState.pitch(0)`: it has no body (traben/entity_model_features/models/animation/state/EMFEntityRenderState.java:84)
- [ ] `EMFEntityRenderState.isTouchingWater(0)`: it has no body (traben/entity_model_features/models/animation/state/EMFEntityRenderState.java:86)
- [ ] `EMFEntityRenderState.isOnFire(0)`: it has no body (traben/entity_model_features/models/animation/state/EMFEntityRenderState.java:88)
- [ ] `EMFEntityRenderState.hasVehicle(0)`: it has no body (traben/entity_model_features/models/animation/state/EMFEntityRenderState.java:90)
- [ ] `EMFEntityRenderState.isOnGround(0)`: it has no body (traben/entity_model_features/models/animation/state/EMFEntityRenderState.java:92)
- [ ] `EMFEntityRenderState.isAlive(0)`: it has no body (traben/entity_model_features/models/animation/state/EMFEntityRenderState.java:94)
- [ ] `EMFEntityRenderState.isGlowing(0)`: it has no body (traben/entity_model_features/models/animation/state/EMFEntityRenderState.java:96)
- [ ] `EMFEntityRenderState.isInLava(0)`: it has no body (traben/entity_model_features/models/animation/state/EMFEntityRenderState.java:98)
- [ ] `EMFEntityRenderState.isInvisible(0)`: it has no body (traben/entity_model_features/models/animation/state/EMFEntityRenderState.java:100)
- [ ] `EMFEntityRenderState.hasPassengers(0)`: it has no body (traben/entity_model_features/models/animation/state/EMFEntityRenderState.java:102)
- [ ] `EMFEntityRenderState.isSneaking(0)`: it has no body (traben/entity_model_features/models/animation/state/EMFEntityRenderState.java:104)
- [ ] `EMFEntityRenderState.isSprinting(0)`: it has no body (traben/entity_model_features/models/animation/state/EMFEntityRenderState.java:106)
- [ ] `EMFEntityRenderState.isWet(0)`: it has no body (traben/entity_model_features/models/animation/state/EMFEntityRenderState.java:108)
- [ ] `EMFEntityRenderState.age(0)`: it has no body (traben/entity_model_features/models/animation/state/EMFEntityRenderState.java:110)
- [ ] `EMFEntityRenderState.yaw(0)`: it has no body (traben/entity_model_features/models/animation/state/EMFEntityRenderState.java:112)
- [ ] `EMFEntityRenderState.emfVelocity(0)`: it has no body (traben/entity_model_features/models/animation/state/EMFEntityRenderState.java:114)
- [ ] `EMFEntityRenderState.typeString(0)`: it has no body (traben/entity_model_features/models/animation/state/EMFEntityRenderState.java:116)
- [ ] `EMFEntityRenderState.variableMap(0)`: it has no body (traben/entity_model_features/models/animation/state/EMFEntityRenderState.java:118)
- [ ] `EMFEntityRenderState.layerFactory(0)`: it has no body (traben/entity_model_features/models/animation/state/EMFEntityRenderState.java:120)
- [ ] `EMFEntityRenderState.setLayerFactory(1)`: it has no body (traben/entity_model_features/models/animation/state/EMFEntityRenderState.java:122)
- [ ] `EMFEntityRenderState.setBipedPose(1)`: it has no body (traben/entity_model_features/models/animation/state/EMFEntityRenderState.java:124)
- [ ] `EMFEntityRenderState.getBipedPose(0)`: it has no body (traben/entity_model_features/models/animation/state/EMFEntityRenderState.java:126)
- [ ] `EMFEntityRenderState.isFirstPersonHand(0)`: it has no body (traben/entity_model_features/models/animation/state/EMFEntityRenderState.java:129)
- [ ] `EMFEntityRenderState.setIsFirstPersonHand(1)`: it has no body (traben/entity_model_features/models/animation/state/EMFEntityRenderState.java:131)
- [ ] `EMFEntityRenderState.shadowSize(0)`: it has no body (traben/entity_model_features/models/animation/state/EMFEntityRenderState.java:133)
- [ ] `EMFEntityRenderState.setShadowSize(1)`: it has no body (traben/entity_model_features/models/animation/state/EMFEntityRenderState.java:135)
- [ ] `EMFEntityRenderState.shadowOpacity(0)`: it has no body (traben/entity_model_features/models/animation/state/EMFEntityRenderState.java:137)
- [ ] `EMFEntityRenderState.setShadowOpacity(1)`: it has no body (traben/entity_model_features/models/animation/state/EMFEntityRenderState.java:139)
- [ ] `EMFEntityRenderState.shadowX(0)`: it has no body (traben/entity_model_features/models/animation/state/EMFEntityRenderState.java:141)
- [ ] `EMFEntityRenderState.setShadowX(1)`: it has no body (traben/entity_model_features/models/animation/state/EMFEntityRenderState.java:143)
- [ ] `EMFEntityRenderState.shadowZ(0)`: it has no body (traben/entity_model_features/models/animation/state/EMFEntityRenderState.java:145)
- [ ] `EMFEntityRenderState.setShadowZ(1)`: it has no body (traben/entity_model_features/models/animation/state/EMFEntityRenderState.java:147)
- [ ] `EMFEntityRenderState.limbAngle(0)`: it has no body (traben/entity_model_features/models/animation/state/EMFEntityRenderState.java:149)
- [ ] `EMFEntityRenderState.setLimbAngle(1)`: it has no body (traben/entity_model_features/models/animation/state/EMFEntityRenderState.java:151)
- [ ] `EMFEntityRenderState.limbDistance(0)`: it has no body (traben/entity_model_features/models/animation/state/EMFEntityRenderState.java:153)
- [ ] `EMFEntityRenderState.setLimbDistance(1)`: it has no body (traben/entity_model_features/models/animation/state/EMFEntityRenderState.java:155)
- [ ] `EMFEntityRenderState.headYaw(0)`: it has no body (traben/entity_model_features/models/animation/state/EMFEntityRenderState.java:157)
- [ ] `EMFEntityRenderState.setHeadYaw(1)`: it has no body (traben/entity_model_features/models/animation/state/EMFEntityRenderState.java:159)
- [ ] `EMFEntityRenderState.headPitch(0)`: it has no body (traben/entity_model_features/models/animation/state/EMFEntityRenderState.java:161)
- [ ] `EMFEntityRenderState.setHeadPitch(1)`: it has no body (traben/entity_model_features/models/animation/state/EMFEntityRenderState.java:163)
- [ ] `EMFEntityRenderState.needsToModifyShadow(0)`: its class isn't translated (it's an interface) (traben/entity_model_features/models/animation/state/EMFEntityRenderState.java:165)
- [ ] `EMFEntityRenderState.fireX(0)`: it has no body (traben/entity_model_features/models/animation/state/EMFEntityRenderState.java:169)
- [ ] `EMFEntityRenderState.setFireX(1)`: it has no body (traben/entity_model_features/models/animation/state/EMFEntityRenderState.java:171)
- [ ] `EMFEntityRenderState.fireY(0)`: it has no body (traben/entity_model_features/models/animation/state/EMFEntityRenderState.java:173)
- [ ] `EMFEntityRenderState.setFireY(1)`: it has no body (traben/entity_model_features/models/animation/state/EMFEntityRenderState.java:175)
- [ ] `EMFEntityRenderState.fireZ(0)`: it has no body (traben/entity_model_features/models/animation/state/EMFEntityRenderState.java:177)
- [ ] `EMFEntityRenderState.setFireZ(1)`: it has no body (traben/entity_model_features/models/animation/state/EMFEntityRenderState.java:179)
- [ ] `EMFEntityRenderState.fireHeight(0)`: it has no body (traben/entity_model_features/models/animation/state/EMFEntityRenderState.java:181)
- [ ] `EMFEntityRenderState.setFireHeight(1)`: it has no body (traben/entity_model_features/models/animation/state/EMFEntityRenderState.java:183)
- [ ] `EMFEntityRenderState.fireScale(0)`: it has no body (traben/entity_model_features/models/animation/state/EMFEntityRenderState.java:185)
- [ ] `EMFEntityRenderState.setFireScale(1)`: it has no body (traben/entity_model_features/models/animation/state/EMFEntityRenderState.java:187)
- [ ] `EMFEntityRenderState.needsToModifyFire(0)`: its class isn't translated (it's an interface) (traben/entity_model_features/models/animation/state/EMFEntityRenderState.java:189)
- [ ] `EMFEntityRenderState.skipModelVariate(0)`: it has no body (traben/entity_model_features/models/animation/state/EMFEntityRenderState.java:197)
- [ ] `EMFEntityRenderState.setSkipModelVariate(1)`: it has no body (traben/entity_model_features/models/animation/state/EMFEntityRenderState.java:199)
- [ ] `EMFEntityRenderState.isSubmit(0)`: it has no body (traben/entity_model_features/models/animation/state/EMFEntityRenderState.java:201)
- [ ] `EMFEntityRenderState.setSubmit(1)`: it has no body (traben/entity_model_features/models/animation/state/EMFEntityRenderState.java:203)
- [ ] `EMFEntityRenderState.getEMFStateSnapshot(0)`: it has no body (traben/entity_model_features/models/animation/state/EMFEntityRenderState.java:205)
- [ ] `EMFEntityRenderState.setEMFStateSnapshot(1)`: it has no body (traben/entity_model_features/models/animation/state/EMFEntityRenderState.java:207)
- [ ] `EMFEntityRenderState.preSubmitActivate(2)`: its class isn't translated (it's an interface) (traben/entity_model_features/models/animation/state/EMFEntityRenderState.java:209)
- [ ] `EMFEntityRenderState.activate(1)`: its class isn't translated (it's an interface) (traben/entity_model_features/models/animation/state/EMFEntityRenderState.java:255)
- [ ] `EMFEntityRenderState.deactivate(1)`: its class isn't translated (it's an interface) (traben/entity_model_features/models/animation/state/EMFEntityRenderState.java:324)
- [ ] `EMFEntityRenderState.from(1)`: it uses EMFEntityRenderState, which isn't translated (it's an interface) (traben/entity_model_features/models/animation/state/EMFEntityRenderState.java:344)

### `EMFEntityRenderStateViaReference`: it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)

- [ ] `EMFEntityRenderStateViaReference.EMFEntityRenderStateViaReference(1)`: its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)) (traben/entity_model_features/models/animation/state/EMFEntityRenderStateViaReference.java:35)
- [ ] `EMFEntityRenderStateViaReference.emfEntity(0)`: its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)) (traben/entity_model_features/models/animation/state/EMFEntityRenderStateViaReference.java:40)
- [ ] `EMFEntityRenderStateViaReference.isManualPlayerState(0)`: its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)) (traben/entity_model_features/models/animation/state/EMFEntityRenderStateViaReference.java:46)
- [ ] `EMFEntityRenderStateViaReference.setManualPlayerState(1)`: its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)) (traben/entity_model_features/models/animation/state/EMFEntityRenderStateViaReference.java:51)
- [ ] `EMFEntityRenderStateViaReference.prevX(0)`: its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)) (traben/entity_model_features/models/animation/state/EMFEntityRenderStateViaReference.java:56)
- [ ] `EMFEntityRenderStateViaReference.x(0)`: its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)) (traben/entity_model_features/models/animation/state/EMFEntityRenderStateViaReference.java:61)
- [ ] `EMFEntityRenderStateViaReference.prevY(0)`: its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)) (traben/entity_model_features/models/animation/state/EMFEntityRenderStateViaReference.java:66)
- [ ] `EMFEntityRenderStateViaReference.y(0)`: its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)) (traben/entity_model_features/models/animation/state/EMFEntityRenderStateViaReference.java:71)
- [ ] `EMFEntityRenderStateViaReference.prevZ(0)`: its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)) (traben/entity_model_features/models/animation/state/EMFEntityRenderStateViaReference.java:76)
- [ ] `EMFEntityRenderStateViaReference.z(0)`: its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)) (traben/entity_model_features/models/animation/state/EMFEntityRenderStateViaReference.java:81)
- [ ] `EMFEntityRenderStateViaReference.prevPitch(0)`: its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)) (traben/entity_model_features/models/animation/state/EMFEntityRenderStateViaReference.java:86)
- [ ] `EMFEntityRenderStateViaReference.pitch(0)`: its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)) (traben/entity_model_features/models/animation/state/EMFEntityRenderStateViaReference.java:91)
- [ ] `EMFEntityRenderStateViaReference.isTouchingWater(0)`: its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)) (traben/entity_model_features/models/animation/state/EMFEntityRenderStateViaReference.java:96)
- [ ] `EMFEntityRenderStateViaReference.isOnFire(0)`: its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)) (traben/entity_model_features/models/animation/state/EMFEntityRenderStateViaReference.java:101)
- [ ] `EMFEntityRenderStateViaReference.hasVehicle(0)`: its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)) (traben/entity_model_features/models/animation/state/EMFEntityRenderStateViaReference.java:106)
- [ ] `EMFEntityRenderStateViaReference.isOnGround(0)`: its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)) (traben/entity_model_features/models/animation/state/EMFEntityRenderStateViaReference.java:111)
- [ ] `EMFEntityRenderStateViaReference.isAlive(0)`: its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)) (traben/entity_model_features/models/animation/state/EMFEntityRenderStateViaReference.java:116)
- [ ] `EMFEntityRenderStateViaReference.isGlowing(0)`: its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)) (traben/entity_model_features/models/animation/state/EMFEntityRenderStateViaReference.java:121)
- [ ] `EMFEntityRenderStateViaReference.isInLava(0)`: its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)) (traben/entity_model_features/models/animation/state/EMFEntityRenderStateViaReference.java:126)
- [ ] `EMFEntityRenderStateViaReference.isInvisible(0)`: its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)) (traben/entity_model_features/models/animation/state/EMFEntityRenderStateViaReference.java:131)
- [ ] `EMFEntityRenderStateViaReference.hasPassengers(0)`: its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)) (traben/entity_model_features/models/animation/state/EMFEntityRenderStateViaReference.java:136)
- [ ] `EMFEntityRenderStateViaReference.isSneaking(0)`: its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)) (traben/entity_model_features/models/animation/state/EMFEntityRenderStateViaReference.java:141)
- [ ] `EMFEntityRenderStateViaReference.isSprinting(0)`: its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)) (traben/entity_model_features/models/animation/state/EMFEntityRenderStateViaReference.java:146)
- [ ] `EMFEntityRenderStateViaReference.isWet(0)`: its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)) (traben/entity_model_features/models/animation/state/EMFEntityRenderStateViaReference.java:151)
- [ ] `EMFEntityRenderStateViaReference.age(0)`: its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)) (traben/entity_model_features/models/animation/state/EMFEntityRenderStateViaReference.java:156)
- [ ] `EMFEntityRenderStateViaReference.yaw(0)`: its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)) (traben/entity_model_features/models/animation/state/EMFEntityRenderStateViaReference.java:161)
- [ ] `EMFEntityRenderStateViaReference.emfVelocity(0)`: its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)) (traben/entity_model_features/models/animation/state/EMFEntityRenderStateViaReference.java:166)
- [ ] `EMFEntityRenderStateViaReference.typeString(0)`: its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)) (traben/entity_model_features/models/animation/state/EMFEntityRenderStateViaReference.java:171)
- [ ] `EMFEntityRenderStateViaReference.variableMap(0)`: its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)) (traben/entity_model_features/models/animation/state/EMFEntityRenderStateViaReference.java:176)
- [ ] `EMFEntityRenderStateViaReference.layerFactory(0)`: its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)) (traben/entity_model_features/models/animation/state/EMFEntityRenderStateViaReference.java:181)
- [ ] `EMFEntityRenderStateViaReference.setLayerFactory(1)`: its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)) (traben/entity_model_features/models/animation/state/EMFEntityRenderStateViaReference.java:186)
- [ ] `EMFEntityRenderStateViaReference.setBipedPose(1)`: its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)) (traben/entity_model_features/models/animation/state/EMFEntityRenderStateViaReference.java:193)
- [ ] `EMFEntityRenderStateViaReference.getBipedPose(0)`: its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)) (traben/entity_model_features/models/animation/state/EMFEntityRenderStateViaReference.java:198)
- [ ] `EMFEntityRenderStateViaReference.isFirstPersonHand(0)`: its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)) (traben/entity_model_features/models/animation/state/EMFEntityRenderStateViaReference.java:203)
- [ ] `EMFEntityRenderStateViaReference.setIsFirstPersonHand(1)`: its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)) (traben/entity_model_features/models/animation/state/EMFEntityRenderStateViaReference.java:208)
- [ ] `EMFEntityRenderStateViaReference.skipModelVariate(0)`: its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)) (traben/entity_model_features/models/animation/state/EMFEntityRenderStateViaReference.java:213)
- [ ] `EMFEntityRenderStateViaReference.setSkipModelVariate(1)`: its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)) (traben/entity_model_features/models/animation/state/EMFEntityRenderStateViaReference.java:218)
- [ ] `EMFEntityRenderStateViaReference.isSubmit(0)`: its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)) (traben/entity_model_features/models/animation/state/EMFEntityRenderStateViaReference.java:223)
- [ ] `EMFEntityRenderStateViaReference.setSubmit(1)`: its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)) (traben/entity_model_features/models/animation/state/EMFEntityRenderStateViaReference.java:228)
- [ ] `EMFEntityRenderStateViaReference.getEMFStateSnapshot(0)`: its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)) (traben/entity_model_features/models/animation/state/EMFEntityRenderStateViaReference.java:233)
- [ ] `EMFEntityRenderStateViaReference.setEMFStateSnapshot(1)`: its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)) (traben/entity_model_features/models/animation/state/EMFEntityRenderStateViaReference.java:238)
- [ ] `EMFEntityRenderStateViaReference.shadowSize(0)`: its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)) (traben/entity_model_features/models/animation/state/EMFEntityRenderStateViaReference.java:243)
- [ ] `EMFEntityRenderStateViaReference.setShadowSize(1)`: its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)) (traben/entity_model_features/models/animation/state/EMFEntityRenderStateViaReference.java:248)
- [ ] `EMFEntityRenderStateViaReference.shadowOpacity(0)`: its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)) (traben/entity_model_features/models/animation/state/EMFEntityRenderStateViaReference.java:253)
- [ ] `EMFEntityRenderStateViaReference.setShadowOpacity(1)`: its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)) (traben/entity_model_features/models/animation/state/EMFEntityRenderStateViaReference.java:258)
- [ ] `EMFEntityRenderStateViaReference.shadowX(0)`: its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)) (traben/entity_model_features/models/animation/state/EMFEntityRenderStateViaReference.java:263)
- [ ] `EMFEntityRenderStateViaReference.setShadowX(1)`: its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)) (traben/entity_model_features/models/animation/state/EMFEntityRenderStateViaReference.java:268)
- [ ] `EMFEntityRenderStateViaReference.shadowZ(0)`: its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)) (traben/entity_model_features/models/animation/state/EMFEntityRenderStateViaReference.java:273)
- [ ] `EMFEntityRenderStateViaReference.setShadowZ(1)`: its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)) (traben/entity_model_features/models/animation/state/EMFEntityRenderStateViaReference.java:278)
- [ ] `EMFEntityRenderStateViaReference.limbAngle(0)`: its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)) (traben/entity_model_features/models/animation/state/EMFEntityRenderStateViaReference.java:283)
- [ ] `EMFEntityRenderStateViaReference.setLimbAngle(1)`: its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)) (traben/entity_model_features/models/animation/state/EMFEntityRenderStateViaReference.java:288)
- [ ] `EMFEntityRenderStateViaReference.limbDistance(0)`: its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)) (traben/entity_model_features/models/animation/state/EMFEntityRenderStateViaReference.java:293)
- [ ] `EMFEntityRenderStateViaReference.setLimbDistance(1)`: its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)) (traben/entity_model_features/models/animation/state/EMFEntityRenderStateViaReference.java:298)
- [ ] `EMFEntityRenderStateViaReference.headYaw(0)`: its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)) (traben/entity_model_features/models/animation/state/EMFEntityRenderStateViaReference.java:303)
- [ ] `EMFEntityRenderStateViaReference.setHeadYaw(1)`: its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)) (traben/entity_model_features/models/animation/state/EMFEntityRenderStateViaReference.java:308)
- [ ] `EMFEntityRenderStateViaReference.headPitch(0)`: its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)) (traben/entity_model_features/models/animation/state/EMFEntityRenderStateViaReference.java:313)
- [ ] `EMFEntityRenderStateViaReference.setHeadPitch(1)`: its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)) (traben/entity_model_features/models/animation/state/EMFEntityRenderStateViaReference.java:318)
- [ ] `EMFEntityRenderStateViaReference.fireX(0)`: its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)) (traben/entity_model_features/models/animation/state/EMFEntityRenderStateViaReference.java:323)
- [ ] `EMFEntityRenderStateViaReference.setFireX(1)`: its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)) (traben/entity_model_features/models/animation/state/EMFEntityRenderStateViaReference.java:328)
- [ ] `EMFEntityRenderStateViaReference.fireY(0)`: its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)) (traben/entity_model_features/models/animation/state/EMFEntityRenderStateViaReference.java:333)
- [ ] `EMFEntityRenderStateViaReference.setFireY(1)`: its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)) (traben/entity_model_features/models/animation/state/EMFEntityRenderStateViaReference.java:338)
- [ ] `EMFEntityRenderStateViaReference.fireZ(0)`: its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)) (traben/entity_model_features/models/animation/state/EMFEntityRenderStateViaReference.java:343)
- [ ] `EMFEntityRenderStateViaReference.setFireZ(1)`: its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)) (traben/entity_model_features/models/animation/state/EMFEntityRenderStateViaReference.java:348)
- [ ] `EMFEntityRenderStateViaReference.fireScale(0)`: its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)) (traben/entity_model_features/models/animation/state/EMFEntityRenderStateViaReference.java:353)
- [ ] `EMFEntityRenderStateViaReference.setFireScale(1)`: its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)) (traben/entity_model_features/models/animation/state/EMFEntityRenderStateViaReference.java:358)
- [ ] `EMFEntityRenderStateViaReference.fireHeight(0)`: its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)) (traben/entity_model_features/models/animation/state/EMFEntityRenderStateViaReference.java:363)
- [ ] `EMFEntityRenderStateViaReference.setFireHeight(1)`: its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)) (traben/entity_model_features/models/animation/state/EMFEntityRenderStateViaReference.java:368)
- [ ] `EMFEntityRenderStateViaReference.toString(0)`: its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)) (traben/entity_model_features/models/animation/state/EMFEntityRenderStateViaReference.java:373)

### `EMFState`

- [x] `EMFState.animationHooks` (traben/entity_model_features/models/animation/state/EMFState.java:36)
- [x] `EMFState.frameCounter` (traben/entity_model_features/models/animation/state/EMFState.java:37)
- [x] `EMFState.isLayerPhase` (traben/entity_model_features/models/animation/state/EMFState.java:38)
- [x] `EMFState.isMainPhase` (traben/entity_model_features/models/animation/state/EMFState.java:39)
- [x] `EMFState.isInShoulderMethod` (traben/entity_model_features/models/animation/state/EMFState.java:40)
- [x] `EMFState.isLeftShoulder` (traben/entity_model_features/models/animation/state/EMFState.java:41)
- [x] `EMFState.isInGui` (traben/entity_model_features/models/animation/state/EMFState.java:42)
- [x] `EMFState.isInGroundOverride` (traben/entity_model_features/models/animation/state/EMFState.java:43)
- [x] `EMFState.isOnHead` (traben/entity_model_features/models/animation/state/EMFState.java:44)
- [x] `EMFState.isInHand` (traben/entity_model_features/models/animation/state/EMFState.java:45)
- [x] `EMFState.isInLeftHand` (traben/entity_model_features/models/animation/state/EMFState.java:47)
- [x] `EMFState.isInHandItemLayerTransform` (traben/entity_model_features/models/animation/state/EMFState.java:48)
- [x] `EMFState.hasDoneArmOverride` (traben/entity_model_features/models/animation/state/EMFState.java:50)
- [x] `EMFState.isInItemFrame` (traben/entity_model_features/models/animation/state/EMFState.java:51)
- [x] `EMFState.modelVariationIgnoresVisibility` (traben/entity_model_features/models/animation/state/EMFState.java:52)
- [ ] `EMFState.SNAPSHOTS`: a EMFState.EMFStateStaticSnapshot with no value (null in Java) (traben/entity_model_features/models/animation/state/EMFState.java:53)
- [x] `EMFState.announceModels` (traben/entity_model_features/models/animation/state/EMFState.java:54)
- [ ] `EMFState.entitiesToForceVanillaModel`: it uses `UUID` (java.util) (traben/entity_model_features/models/animation/state/EMFState.java:55)
- [ ] `EMFState.forceVanillaModelListeners`: it uses EMFEntity, which isn't translated (it's an interface) (traben/entity_model_features/models/animation/state/EMFState.java:56)
- [ ] `EMFState.state(0)`: it uses EMFEntityRenderState, which isn't translated (it's an interface) (traben/entity_model_features/models/animation/state/EMFState.java:58)
- [ ] `EMFState.emfEntity(0)`: it uses EMFEntity, which isn't translated (it's an interface) (traben/entity_model_features/models/animation/state/EMFState.java:63)
- [ ] `EMFState.clear(0)`: it calls `ETFState` (traben.entity_texture_features.features.state).clear (traben/entity_model_features/models/animation/state/EMFState.java:69)
- [ ] `EMFState.incFrameCount(0)`: it calls IrisShadowPassDetection.getInstance, which isn't translated (traben/entity_model_features/models/animation/state/EMFState.java:75)
- [x] `EMFState.getFrameCounter(0)` (traben/entity_model_features/models/animation/state/EMFState.java:83)
- [x] `EMFState.clearFrameStatics(0)` (traben/entity_model_features/models/animation/state/EMFState.java:87)
- [ ] `EMFState.captureStatics(0)`: it uses EMFState.SNAPSHOTS, which isn't translated (a EMFState.EMFStateStaticSnapshot with no value (null in Java)) (traben/entity_model_features/models/animation/state/EMFState.java:103)
- [ ] `EMFState.anounceModels(1)`: it uses EMFEntityRenderState, which isn't translated (it's an interface) (traben/entity_model_features/models/animation/state/EMFState.java:120)
- [x] `EMFState.entryAndValue(2)` (traben/entity_model_features/models/animation/state/EMFState.java:198)
- [ ] `EMFState.isEntityForcedToVanillaModel(1)`: it uses EMFEntityRenderState, which isn't translated (it's an interface) (traben/entity_model_features/models/animation/state/EMFState.java:202)
- [ ] `EMFState.getLayerFromRecentFactoryOrETFOverrideOrTranslucent(1)`: it uses Minecraft's `RenderType` (traben/entity_model_features/models/animation/state/EMFState.java:237)

### `EMFState.EMFStateStaticSnapshot`

- [x] `EMFState.EMFStateStaticSnapshot.EMFStateStaticSnapshot(7)` (traben/entity_model_features/models/animation/state/EMFState.java:257)
- [ ] `EMFState.EMFStateStaticSnapshot.isDefault(0)`: `this` used as a value (traben/entity_model_features/models/animation/state/EMFState.java:266)
- [x] `EMFState.EMFStateStaticSnapshot.restoreStatics(0)` (traben/entity_model_features/models/animation/state/EMFState.java:270)
- [x] `EMFState.EMFStateStaticSnapshot.isInShoulderMethod(0)` (traben/entity_model_features/models/animation/state/EMFState.java:257)
- [x] `EMFState.EMFStateStaticSnapshot.isLeftShoulder(0)` (traben/entity_model_features/models/animation/state/EMFState.java:257)
- [x] `EMFState.EMFStateStaticSnapshot.isInGui(0)` (traben/entity_model_features/models/animation/state/EMFState.java:257)
- [x] `EMFState.EMFStateStaticSnapshot.isInGroundOverride(0)` (traben/entity_model_features/models/animation/state/EMFState.java:257)
- [x] `EMFState.EMFStateStaticSnapshot.isOnHead(0)` (traben/entity_model_features/models/animation/state/EMFState.java:257)
- [x] `EMFState.EMFStateStaticSnapshot.isInHand(0)` (traben/entity_model_features/models/animation/state/EMFState.java:257)
- [x] `EMFState.EMFStateStaticSnapshot.isInItemFrame(0)` (traben/entity_model_features/models/animation/state/EMFState.java:257)

### `EMFModelMappings`

- [x] `EMFModelMappings.UNKNOWN_MODEL_MAP_CACHE` (traben/entity_model_features/models/EMFModelMappings.java:39)
- [x] `EMFModelMappings.OPTIFINE_MODEL_MAP_CACHE` (traben/entity_model_features/models/EMFModelMappings.java:40)
- [ ] `EMFModelMappings.genericNonPlayerBiped`: it calls EMFModelMappings.partMapping, which isn't translated (traben/entity_model_features/models/EMFModelMappings.java:41)
- [ ] `EMFModelMappings.initOptifineMappings(0)`: its body couldn't be read: `var` (traben/entity_model_features/models/EMFModelMappings.java:51)
- [ ] `EMFModelMappings.partMapping(1)`: Minecraft's entry kept in a variable or field (traben/entity_model_features/models/EMFModelMappings.java:1258)
- [ ] `EMFModelMappings.partMapping(2)`: Minecraft's entry kept in a variable or field (traben/entity_model_features/models/EMFModelMappings.java:1263)
- [ ] `EMFModelMappings.isKnownMapping(1)`: it uses EMFModel_ID, which isn't translated (it implements `Comparable`) (traben/entity_model_features/models/EMFModelMappings.java:1268)
- [ ] `EMFModelMappings.getMapOf(2)`: it uses EMFModel_ID, which isn't translated (it implements `Comparable`) (traben/entity_model_features/models/EMFModelMappings.java:1272)
- [ ] `EMFModelMappings.getMapOf(5)`: it uses EMFModel_ID, which isn't translated (it implements `Comparable`) (traben/entity_model_features/models/EMFModelMappings.java:1276)
- [ ] `EMFModelMappings.exportKnown(5)`: it uses EMFModel_ID, which isn't translated (it implements `Comparable`) (traben/entity_model_features/models/EMFModelMappings.java:1298)
- [x] `EMFModelMappings.getKnownMap(1)` (traben/entity_model_features/models/EMFModelMappings.java:1316)
- [ ] `EMFModelMappings.exploreProvidedEntityModelAndExportIfNeeded(5)`: it uses Minecraft's `ModelPart` (traben/entity_model_features/models/EMFModelMappings.java:1321)
- [ ] `EMFModelMappings.printModel(3)`: it calls `ETF` (traben.entity_texture_features).getConfigDirectory (traben/entity_model_features/models/EMFModelMappings.java:1416)
- [ ] `EMFModelMappings.addArrayProperty(3)`: it's overloaded with the same number of parameters (traben/entity_model_features/models/EMFModelMappings.java:1434)
- [ ] `EMFModelMappings.addArrayProperty(3)`: it's overloaded with the same number of parameters (traben/entity_model_features/models/EMFModelMappings.java:1444)
- [ ] `EMFModelMappings.partPrinter(5)`: it uses PartAndOffsets, which isn't translated (it uses Minecraft's `ModelPart`) (traben/entity_model_features/models/EMFModelMappings.java:1454)
- [ ] `EMFModelMappings.getChildByName(5)`: it uses PartAndOffsets, which isn't translated (it uses Minecraft's `ModelPart`) (traben/entity_model_features/models/EMFModelMappings.java:1561)
- [ ] `EMFModelMappings.mapThisAndChildren(5)`: it uses Minecraft's `ModelPart` (traben/entity_model_features/models/EMFModelMappings.java:1577)

### `EMFModelMappings.OptifineMapper`

- [x] `EMFModelMappings.OptifineMapper.exceptions` (traben/entity_model_features/models/EMFModelMappings.java:1667)
- [x] `EMFModelMappings.OptifineMapper.OptifineMapper(1)` (traben/entity_model_features/models/EMFModelMappings.java:1669)
- [x] `EMFModelMappings.OptifineMapper.models(1)` (traben/entity_model_features/models/EMFModelMappings.java:1673)
- [ ] `EMFModelMappings.OptifineMapper.parts(1)`: it calls EMFUtils.logError, which isn't translated (traben/entity_model_features/models/EMFModelMappings.java:1679)

### `EMFModelMappings.PartAndOffsets`: it uses Minecraft's `ModelPart`

- [ ] `EMFModelMappings.PartAndOffsets.PartAndOffsets(4)`: its class isn't translated (it uses Minecraft's `ModelPart`) (traben/entity_model_features/models/EMFModelMappings.java:1688)
- [ ] `EMFModelMappings.PartAndOffsets.part(0)`: its class isn't translated (it uses Minecraft's `ModelPart`) (traben/entity_model_features/models/EMFModelMappings.java:1688)
- [ ] `EMFModelMappings.PartAndOffsets.x(0)`: its class isn't translated (it uses Minecraft's `ModelPart`) (traben/entity_model_features/models/EMFModelMappings.java:1688)
- [ ] `EMFModelMappings.PartAndOffsets.y(0)`: its class isn't translated (it uses Minecraft's `ModelPart`) (traben/entity_model_features/models/EMFModelMappings.java:1688)
- [ ] `EMFModelMappings.PartAndOffsets.z(0)`: its class isn't translated (it uses Minecraft's `ModelPart`) (traben/entity_model_features/models/EMFModelMappings.java:1688)

### `EMFModel_ID`: it implements `Comparable`

- [ ] `EMFModel_ID.setCacheID(1)`: its class isn't translated (it implements `Comparable`) (traben/entity_model_features/models/EMFModel_ID.java:22)
- [ ] `EMFModel_ID.resetCacheID(0)`: its class isn't translated (it implements `Comparable`) (traben/entity_model_features/models/EMFModel_ID.java:27)
- [ ] `EMFModel_ID.getCacheID(0)`: its class isn't translated (it implements `Comparable`) (traben/entity_model_features/models/EMFModel_ID.java:31)
- [ ] `EMFModel_ID.EMFModel_ID(1)`: its class isn't translated (it implements `Comparable`) (traben/entity_model_features/models/EMFModel_ID.java:36)
- [ ] `EMFModel_ID.EMFModel_ID(2)`: its class isn't translated (it implements `Comparable`) (traben/entity_model_features/models/EMFModel_ID.java:40)
- [ ] `EMFModel_ID.EMFModel_ID(3)`: its class isn't translated (it implements `Comparable`) (traben/entity_model_features/models/EMFModel_ID.java:44)
- [ ] `EMFModel_ID.setFileName(1)`: its class isn't translated (it implements `Comparable`) (traben/entity_model_features/models/EMFModel_ID.java:51)
- [ ] `EMFModel_ID.equals(1)`: its class isn't translated (it implements `Comparable`) (traben/entity_model_features/models/EMFModel_ID.java:68)
- [ ] `EMFModel_ID.hashCode(0)`: its class isn't translated (it implements `Comparable`) (traben/entity_model_features/models/EMFModel_ID.java:78)
- [ ] `EMFModel_ID.toString(0)`: its class isn't translated (it implements `Comparable`) (traben/entity_model_features/models/EMFModel_ID.java:83)
- [ ] `EMFModel_ID.setBoth(1)`: its class isn't translated (it implements `Comparable`) (traben/entity_model_features/models/EMFModel_ID.java:88)
- [ ] `EMFModel_ID.areBothSame(0)`: its class isn't translated (it implements `Comparable`) (traben/entity_model_features/models/EMFModel_ID.java:94)
- [ ] `EMFModel_ID.setBoth(2)`: its class isn't translated (it implements `Comparable`) (traben/entity_model_features/models/EMFModel_ID.java:98)
- [ ] `EMFModel_ID.getfileName(0)`: its class isn't translated (it implements `Comparable`) (traben/entity_model_features/models/EMFModel_ID.java:104)
- [ ] `EMFModel_ID.hasFallbackModels(0)`: its class isn't translated (it implements `Comparable`) (traben/entity_model_features/models/EMFModel_ID.java:108)
- [ ] `EMFModel_ID.forEachFallback(1)`: its class isn't translated (it implements `Comparable`) (traben/entity_model_features/models/EMFModel_ID.java:112)
- [ ] `EMFModel_ID.getNextFallbackModel(1)`: its class isn't translated (it implements `Comparable`) (traben/entity_model_features/models/EMFModel_ID.java:126)
- [ ] `EMFModel_ID.addFallbackModel(2)`: its class isn't translated (it implements `Comparable`) (traben/entity_model_features/models/EMFModel_ID.java:138)
- [ ] `EMFModel_ID.addFallbackModel(3)`: its class isn't translated (it implements `Comparable`) (traben/entity_model_features/models/EMFModel_ID.java:142)
- [ ] `EMFModel_ID.pushNewMainModelAddingOldAsFallback(1)`: its class isn't translated (it implements `Comparable`) (traben/entity_model_features/models/EMFModel_ID.java:162)
- [ ] `EMFModel_ID.pushNewMainModelAndMapIdAddingOldAsFallback(1)`: its class isn't translated (it implements `Comparable`) (traben/entity_model_features/models/EMFModel_ID.java:168)
- [ ] `EMFModel_ID.pushNewMainModelAndMapIdAddingOldAsFallback(2)`: its class isn't translated (it implements `Comparable`) (traben/entity_model_features/models/EMFModel_ID.java:172)
- [ ] `EMFModel_ID.addFallbackModel(1)`: its class isn't translated (it implements `Comparable`) (traben/entity_model_features/models/EMFModel_ID.java:179)
- [ ] `EMFModel_ID.setMapIdAndAddFallbackModel(1)`: its class isn't translated (it implements `Comparable`) (traben/entity_model_features/models/EMFModel_ID.java:183)
- [ ] `EMFModel_ID.setMapIdAndAddFallbackModel(2)`: its class isn't translated (it implements `Comparable`) (traben/entity_model_features/models/EMFModel_ID.java:187)
- [ ] `EMFModel_ID.getNamespace(0)`: its class isn't translated (it implements `Comparable`) (traben/entity_model_features/models/EMFModel_ID.java:203)
- [ ] `EMFModel_ID.finishAndPrepAutomatedFallbacks(0)`: its class isn't translated (it implements `Comparable`) (traben/entity_model_features/models/EMFModel_ID.java:207)
- [ ] `EMFModel_ID.assertNamespaceAndCreateDeprecatedModdedFileName(2)`: its class isn't translated (it implements `Comparable`) (traben/entity_model_features/models/EMFModel_ID.java:236)
- [ ] `EMFModel_ID.getMapId(0)`: its class isn't translated (it implements `Comparable`) (traben/entity_model_features/models/EMFModel_ID.java:242)
- [ ] `EMFModel_ID.compareTo(1)`: its class isn't translated (it implements `Comparable`) (traben/entity_model_features/models/EMFModel_ID.java:246)
- [ ] `EMFModel_ID.getDisplayFileName(0)`: its class isn't translated (it implements `Comparable`) (traben/entity_model_features/models/EMFModel_ID.java:250)

### `EMFModel_ID.FallbackModel`

- [x] `EMFModel_ID.FallbackModel.FallbackModel(2)` (traben/entity_model_features/models/EMFModel_ID.java:254)
- [x] `EMFModel_ID.FallbackModel.namespace(0)` (traben/entity_model_features/models/EMFModel_ID.java:254)
- [x] `EMFModel_ID.FallbackModel.fileName(0)` (traben/entity_model_features/models/EMFModel_ID.java:254)

### `EMFPartialArmor`

- [x] `EMFPartialArmor.hasCustom(0)` (traben/entity_model_features/models/EMFPartialArmor.java:6)

### `IEMFModel`: it's an interface

- [ ] `IEMFModel.emf$isEMFModel(0)`: it has no body (traben/entity_model_features/models/IEMFModel.java:6)
- [ ] `IEMFModel.emf$getEMFRootModel(0)`: it has no body (traben/entity_model_features/models/IEMFModel.java:8)

### `IEMFModelNameContainer`: it's an interface

- [ ] `IEMFModelNameContainer.emf$insertKnownMappings(1)`: it has no body (traben/entity_model_features/models/IEMFModelNameContainer.java:4)

### `EMFBoxData`

- [ ] `EMFBoxData.prepare(3)`: a catch block that uses its exception (traben/entity_model_features/models/jem_objects/EMFBoxData.java:27)
- [ ] `EMFBoxData.validateUV(2)`: it calls `Arrays` (java.util).toString (traben/entity_model_features/models/jem_objects/EMFBoxData.java:72)
- [x] `EMFBoxData.checkAndFixUVLegacyDirections(0)` (traben/entity_model_features/models/jem_objects/EMFBoxData.java:100)
- [ ] `EMFBoxData.toString(0)`: it calls `Arrays` (java.util).toString (traben/entity_model_features/models/jem_objects/EMFBoxData.java:118)

### `EMFJemData`: it uses `LinkedList` (java.util)

- [x] `EMFJemData.WHITESPACE` (traben/entity_model_features/models/jem_objects/EMFJemData.java:39)
- [x] `EMFJemData.THIS` (traben/entity_model_features/models/jem_objects/EMFJemData.java:40)
- [x] `EMFJemData.PART` (traben/entity_model_features/models/jem_objects/EMFJemData.java:41)
- [ ] `EMFJemData.getAllTopLevelAnimationsByVanillaPartName(0)`: its class isn't translated (it uses `LinkedList` (java.util)) (traben/entity_model_features/models/jem_objects/EMFJemData.java:43)
- [ ] `EMFJemData.getMobModelIDInfo(0)`: its class isn't translated (it uses `LinkedList` (java.util)) (traben/entity_model_features/models/jem_objects/EMFJemData.java:47)
- [ ] `EMFJemData.getCustomTexture(0)`: its class isn't translated (it uses `LinkedList` (java.util)) (traben/entity_model_features/models/jem_objects/EMFJemData.java:51)
- [ ] `EMFJemData.validateJemTexture(1)`: its class isn't translated (it uses `LinkedList` (java.util)) (traben/entity_model_features/models/jem_objects/EMFJemData.java:55)
- [ ] `EMFJemData.validateJemTexture(2)`: its class isn't translated (it uses `LinkedList` (java.util)) (traben/entity_model_features/models/jem_objects/EMFJemData.java:60)
- [ ] `EMFJemData.validateResourcePathAndExists(2)`: its class isn't translated (it uses `LinkedList` (java.util)) (traben/entity_model_features/models/jem_objects/EMFJemData.java:70)
- [ ] `EMFJemData.prepare(2)`: its class isn't translated (it uses `LinkedList` (java.util)) (traben/entity_model_features/models/jem_objects/EMFJemData.java:105)
- [ ] `EMFJemData.processAnimAndKeyString(2)`: it uses EMFPartData, which isn't translated (it holds an object of its own class (a linked structure)) (traben/entity_model_features/models/jem_objects/EMFJemData.java:210)
- [ ] `EMFJemData.toString(0)`: its class isn't translated (it uses `LinkedList` (java.util)) (traben/entity_model_features/models/jem_objects/EMFJemData.java:217)

### `EMFPartData`: it holds an object of its own class (a linked structure)

- [ ] `EMFPartData.getCustomTexture(0)`: its class isn't translated (it holds an object of its own class (a linked structure)) (traben/entity_model_features/models/jem_objects/EMFPartData.java:41)
- [ ] `EMFPartData.equals(1)`: its class isn't translated (it holds an object of its own class (a linked structure)) (traben/entity_model_features/models/jem_objects/EMFPartData.java:45)
- [ ] `EMFPartData.hashCode(0)`: its class isn't translated (it holds an object of its own class (a linked structure)) (traben/entity_model_features/models/jem_objects/EMFPartData.java:73)
- [ ] `EMFPartData.copyFrom(1)`: its class isn't translated (it holds an object of its own class (a linked structure)) (traben/entity_model_features/models/jem_objects/EMFPartData.java:96)
- [ ] `EMFPartData.getAttachments(0)`: its class isn't translated (it holds an object of its own class (a linked structure)) (traben/entity_model_features/models/jem_objects/EMFPartData.java:150)
- [ ] `EMFPartData.prepare(2)`: its class isn't translated (it holds an object of its own class (a linked structure)) (traben/entity_model_features/models/jem_objects/EMFPartData.java:176)
- [ ] `EMFPartData.toString(0)`: its class isn't translated (it holds an object of its own class (a linked structure)) (traben/entity_model_features/models/jem_objects/EMFPartData.java:262)

### `EMFModelPart`: it extends Minecraft's `ModelPart`

- [ ] `EMFModelPart.getRoot(0)`: its class isn't translated (it extends Minecraft's `ModelPart`) (traben/entity_model_features/models/parts/EMFModelPart.java:49)
- [ ] `EMFModelPart.EMFModelPart(3)`: its class isn't translated (it extends Minecraft's `ModelPart`) (traben/entity_model_features/models/parts/EMFModelPart.java:53)
- [ ] `EMFModelPart.getAttachmentPositioner(1)`: its class isn't translated (it extends Minecraft's `ModelPart`) (traben/entity_model_features/models/parts/EMFModelPart.java:60)
- [ ] `EMFModelPart.render(5)`: its class isn't translated (it extends Minecraft's `ModelPart`) (traben/entity_model_features/models/parts/EMFModelPart.java:77)
- [ ] `EMFModelPart.lines(0)`: its class isn't translated (it extends Minecraft's `ModelPart`) (traben/entity_model_features/models/parts/EMFModelPart.java:131)
- [ ] `EMFModelPart.renderDebugTinted(5)`: its class isn't translated (it extends Minecraft's `ModelPart`) (traben/entity_model_features/models/parts/EMFModelPart.java:135)
- [ ] `EMFModelPart.renderWithTextureOverride(5)`: its body couldn't be read: an anonymous class (traben/entity_model_features/models/parts/EMFModelPart.java:141)
- [ ] `EMFModelPart.renderTextureOverrideWithoutReset(5)`: its class isn't translated (it extends Minecraft's `ModelPart`) (traben/entity_model_features/models/parts/EMFModelPart.java:184)
- [ ] `EMFModelPart.renderLikeVanilla(5)`: its class isn't translated (it extends Minecraft's `ModelPart`) (traben/entity_model_features/models/parts/EMFModelPart.java:200)
- [ ] `EMFModelPart.testForBuildingException(1)`: its class isn't translated (it extends Minecraft's `ModelPart`) (traben/entity_model_features/models/parts/EMFModelPart.java:216)
- [ ] `EMFModelPart.renderLikeETF(5)`: its class isn't translated (it extends Minecraft's `ModelPart`) (traben/entity_model_features/models/parts/EMFModelPart.java:235)
- [ ] `EMFModelPart.renderBoxes(3)`: its class isn't translated (it extends Minecraft's `ModelPart`) (traben/entity_model_features/models/parts/EMFModelPart.java:263)
- [ ] `EMFModelPart.debugBoxColor(0)`: it has no body (traben/entity_model_features/models/parts/EMFModelPart.java:291)
- [ ] `EMFModelPart.compile(5)`: its class isn't translated (it extends Minecraft's `ModelPart`) (traben/entity_model_features/models/parts/EMFModelPart.java:293)
- [ ] `EMFModelPart.simplePrintChildren(1)`: its class isn't translated (it extends Minecraft's `ModelPart`) (traben/entity_model_features/models/parts/EMFModelPart.java:303)
- [ ] `EMFModelPart.toStringShort(0)`: its class isn't translated (it extends Minecraft's `ModelPart`) (traben/entity_model_features/models/parts/EMFModelPart.java:318)
- [ ] `EMFModelPart.toString(0)`: its class isn't translated (it extends Minecraft's `ModelPart`) (traben/entity_model_features/models/parts/EMFModelPart.java:322)
- [ ] `EMFModelPart.getVanillaModelPartsOfCurrentState(0)`: its class isn't translated (it extends Minecraft's `ModelPart`) (traben/entity_model_features/models/parts/EMFModelPart.java:326)
- [ ] `EMFModelPart.getAllChildPartsAsAnimationMap(3)`: its class isn't translated (it extends Minecraft's `ModelPart`) (traben/entity_model_features/models/parts/EMFModelPart.java:356)

### `EMFModelPartCustom`: it extends `EMFModelPart`

- [ ] `EMFModelPartCustom.EMFModelPartCustom(5)`: its class isn't translated (it extends `EMFModelPart`) (traben/entity_model_features/models/parts/EMFModelPartCustom.java:38)
- [ ] `EMFModelPartCustom.resetPose(0)`: its class isn't translated (it extends `EMFModelPart`) (traben/entity_model_features/models/parts/EMFModelPartCustom.java:62)
- [ ] `EMFModelPartCustom.getCuboidsFromData(1)`: it uses Minecraft's `Cube` (traben/entity_model_features/models/parts/EMFModelPartCustom.java:69)
- [ ] `EMFModelPartCustom.getChildrenFromData(3)`: it uses Minecraft's `ModelPart` (traben/entity_model_features/models/parts/EMFModelPartCustom.java:73)
- [ ] `EMFModelPartCustom.createCuboidsFromBoxDataV3(1)`: it uses Minecraft's `Cube` (traben/entity_model_features/models/parts/EMFModelPartCustom.java:84)
- [ ] `EMFModelPartCustom.toString(0)`: its class isn't translated (it extends `EMFModelPart`) (traben/entity_model_features/models/parts/EMFModelPartCustom.java:144)
- [ ] `EMFModelPartCustom.toStringShort(0)`: its class isn't translated (it extends `EMFModelPart`) (traben/entity_model_features/models/parts/EMFModelPartCustom.java:149)
- [ ] `EMFModelPartCustom.getAttachmentPositioner(1)`: its class isn't translated (it extends `EMFModelPart`) (traben/entity_model_features/models/parts/EMFModelPartCustom.java:154)
- [ ] `EMFModelPartCustom.debugBoxColor(0)`: its class isn't translated (it extends `EMFModelPart`) (traben/entity_model_features/models/parts/EMFModelPartCustom.java:171)
- [ ] `EMFModelPartCustom.renderWithTextureOverride(5)`: its class isn't translated (it extends `EMFModelPart`) (traben/entity_model_features/models/parts/EMFModelPartCustom.java:176)

### `EMFModelPartCustom.EMFCube`: it extends Minecraft's `Cube`

- [ ] `EMFModelPartCustom.EMFCube.EMFCube(16)`: its body couldn't be read: an anonymous class (traben/entity_model_features/models/parts/EMFModelPartCustom.java:184)
- [ ] `EMFModelPartCustom.EMFCube.EMFCube(20)`: its body couldn't be read: an anonymous class (traben/entity_model_features/models/parts/EMFModelPartCustom.java:394)
- [ ] `EMFModelPartCustom.EMFCube.compile(5)`: its class isn't translated (it extends Minecraft's `Cube`) (traben/entity_model_features/models/parts/EMFModelPartCustom.java:571)

### `EMFModelPartRoot`: it extends `EMFModelPartVanilla`

- [ ] `EMFModelPartRoot.getPositionerForAttachment(1)`: its class isn't translated (it extends `EMFModelPartVanilla`) (traben/entity_model_features/models/parts/EMFModelPartRoot.java:64)
- [ ] `EMFModelPartRoot.getRoot(0)`: its class isn't translated (it extends `EMFModelPartVanilla`) (traben/entity_model_features/models/parts/EMFModelPartRoot.java:75)
- [ ] `EMFModelPartRoot.EMFModelPartRoot(5)`: its class isn't translated (it extends `EMFModelPartVanilla`) (traben/entity_model_features/models/parts/EMFModelPartRoot.java:80)
- [ ] `EMFModelPartRoot.debugBoxColor(0)`: its class isn't translated (it extends `EMFModelPartVanilla`) (traben/entity_model_features/models/parts/EMFModelPartRoot.java:95)
- [ ] `EMFModelPartRoot.getAllVanillaPartsEMF(0)`: its class isn't translated (it extends `EMFModelPartVanilla`) (traben/entity_model_features/models/parts/EMFModelPartRoot.java:100)
- [ ] `EMFModelPartRoot.getAllVanillaPartsByNameEMF(0)`: its class isn't translated (it extends `EMFModelPartVanilla`) (traben/entity_model_features/models/parts/EMFModelPartRoot.java:104)
- [ ] `EMFModelPartRoot.oneTimeRunnable(0)`: its class isn't translated (it extends `EMFModelPartVanilla`) (traben/entity_model_features/models/parts/EMFModelPartRoot.java:108)
- [ ] `EMFModelPartRoot.registerModelRunnableWithEntityTypeContext(0)`: its class isn't translated (it extends `EMFModelPartVanilla`) (traben/entity_model_features/models/parts/EMFModelPartRoot.java:112)
- [ ] `EMFModelPartRoot.doVariantCheck(1)`: its class isn't translated (it extends `EMFModelPartVanilla`) (traben/entity_model_features/models/parts/EMFModelPartRoot.java:133)
- [ ] `EMFModelPartRoot.checkIfShouldExpireEntity(2)`: its class isn't translated (it extends `EMFModelPartVanilla`) (traben/entity_model_features/models/parts/EMFModelPartRoot.java:151)
- [ ] `EMFModelPartRoot.addAndSetVariantOfJem(2)`: its class isn't translated (it extends `EMFModelPartVanilla`) (traben/entity_model_features/models/parts/EMFModelPartRoot.java:169)
- [ ] `EMFModelPartRoot.discoverAndInitVariants(1)`: its class isn't translated (it extends `EMFModelPartVanilla`) (traben/entity_model_features/models/parts/EMFModelPartRoot.java:236)
- [ ] `EMFModelPartRoot.setVariant1ToVanilla0(0)`: its class isn't translated (it extends `EMFModelPartVanilla`) (traben/entity_model_features/models/parts/EMFModelPartRoot.java:327)
- [ ] `EMFModelPartRoot.hasAnimation(0)`: its class isn't translated (it extends `EMFModelPartVanilla`) (traben/entity_model_features/models/parts/EMFModelPartRoot.java:332)
- [ ] `EMFModelPartRoot.setVariantStateTo(1)`: its class isn't translated (it extends `EMFModelPartVanilla`) (traben/entity_model_features/models/parts/EMFModelPartRoot.java:336)
- [ ] `EMFModelPartRoot.animate(0)`: its class isn't translated (it extends `EMFModelPartVanilla`) (traben/entity_model_features/models/parts/EMFModelPartRoot.java:342)
- [ ] `EMFModelPartRoot.animateNoPause(0)`: its class isn't translated (it extends `EMFModelPartVanilla`) (traben/entity_model_features/models/parts/EMFModelPartRoot.java:349)
- [ ] `EMFModelPartRoot.receiveAnimationHandler(2)`: its class isn't translated (it extends `EMFModelPartVanilla`) (traben/entity_model_features/models/parts/EMFModelPartRoot.java:356)
- [ ] `EMFModelPartRoot.getTopLevelJemTexture(0)`: its class isn't translated (it extends `EMFModelPartVanilla`) (traben/entity_model_features/models/parts/EMFModelPartRoot.java:408)
- [ ] `EMFModelPartRoot.resetVanillaPartsToDefaults(0)`: its class isn't translated (it extends `EMFModelPartVanilla`) (traben/entity_model_features/models/parts/EMFModelPartRoot.java:430)
- [ ] `EMFModelPartRoot.toString(0)`: its class isn't translated (it extends `EMFModelPartVanilla`) (traben/entity_model_features/models/parts/EMFModelPartRoot.java:435)
- [ ] `EMFModelPartRoot.toStringShort(0)`: its class isn't translated (it extends `EMFModelPartVanilla`) (traben/entity_model_features/models/parts/EMFModelPartRoot.java:440)

### `EMFModelPartVanilla`: it extends `EMFModelPartWithState`

- [ ] `EMFModelPartVanilla.EMFModelPartVanilla(5)`: its class isn't translated (it extends `EMFModelPartWithState`) (traben/entity_model_features/models/parts/EMFModelPartVanilla.java:25)
- [ ] `EMFModelPartVanilla.getAllEMFCustomChildren(0)`: its class isn't translated (it extends `EMFModelPartWithState`) (traben/entity_model_features/models/parts/EMFModelPartVanilla.java:48)
- [ ] `EMFModelPartVanilla.translateAndRotate(1)`: its class isn't translated (it extends `EMFModelPartWithState`) (traben/entity_model_features/models/parts/EMFModelPartVanilla.java:52)
- [ ] `EMFModelPartVanilla.debugBoxColor(0)`: its class isn't translated (it extends `EMFModelPartWithState`) (traben/entity_model_features/models/parts/EMFModelPartVanilla.java:87)
- [ ] `EMFModelPartVanilla.render(5)`: its class isn't translated (it extends `EMFModelPartWithState`) (traben/entity_model_features/models/parts/EMFModelPartVanilla.java:92)
- [ ] `EMFModelPartVanilla.setHideInTheseStates(1)`: its class isn't translated (it extends `EMFModelPartWithState`) (traben/entity_model_features/models/parts/EMFModelPartVanilla.java:99)
- [ ] `EMFModelPartVanilla.toString(0)`: its class isn't translated (it extends `EMFModelPartWithState`) (traben/entity_model_features/models/parts/EMFModelPartVanilla.java:108)
- [ ] `EMFModelPartVanilla.toStringShort(0)`: its class isn't translated (it extends `EMFModelPartWithState`) (traben/entity_model_features/models/parts/EMFModelPartVanilla.java:113)

### `EBEConfigModifier`: it implements `BiConsumer` (java.util.function)

- [x] `EBEConfigModifier.chestsDisabled` (traben/entity_model_features/mod_compat/EBEConfigModifier.java:13)
- [x] `EBEConfigModifier.shulkerBoxesDisabled` (traben/entity_model_features/mod_compat/EBEConfigModifier.java:14)
- [x] `EBEConfigModifier.bellsDisabled` (traben/entity_model_features/mod_compat/EBEConfigModifier.java:15)
- [x] `EBEConfigModifier.bedsDisabled` (traben/entity_model_features/mod_compat/EBEConfigModifier.java:16)
- [x] `EBEConfigModifier.signsDisabled` (traben/entity_model_features/mod_compat/EBEConfigModifier.java:17)
- [x] `EBEConfigModifier.decoratedPotsDisabled` (traben/entity_model_features/mod_compat/EBEConfigModifier.java:18)
- [ ] `EBEConfigModifier.ebeReloader`: a function kept in a variable or field (only parameters can be) (traben/entity_model_features/mod_compat/EBEConfigModifier.java:19)
- [ ] `EBEConfigModifier.modifyEBEConfig(1)`: it uses EBEConfigModifier.ebeReloader, which isn't translated (a function kept in a variable or field (only parameters can be)) (traben/entity_model_features/mod_compat/EBEConfigModifier.java:21)
- [ ] `EBEConfigModifier.accept(2)`: its class isn't translated (it implements `BiConsumer` (java.util.function)) (traben/entity_model_features/mod_compat/EBEConfigModifier.java:35)
- [ ] `EBEConfigModifier.accept(1)`: its class isn't translated (it implements `BiConsumer` (java.util.function)) (traben/entity_model_features/mod_compat/EBEConfigModifier.java:81)

### `IrisShadowPassDetection`

- [x] `IrisShadowPassDetection.instance` (traben/entity_model_features/mod_compat/IrisShadowPassDetection.java:9)
- [ ] `IrisShadowPassDetection.inShadowPass(0)`: it has no body (traben/entity_model_features/mod_compat/IrisShadowPassDetection.java:11)
- [ ] `IrisShadowPassDetection.getInstance(0)`: its body couldn't be read: an anonymous class (traben/entity_model_features/mod_compat/IrisShadowPassDetection.java:13)

### `IrisShadowPassDetection.IrisShadowPassDetectionImpl`: it extends `IrisShadowPassDetection`

- [ ] `IrisShadowPassDetection.IrisShadowPassDetectionImpl.IrisShadowPassDetectionImpl(0)`: its class isn't translated (it extends `IrisShadowPassDetection`) (traben/entity_model_features/mod_compat/IrisShadowPassDetection.java:32)
- [ ] `IrisShadowPassDetection.IrisShadowPassDetectionImpl.inShadowPass(0)`: its class isn't translated (it extends `IrisShadowPassDetection`) (traben/entity_model_features/mod_compat/IrisShadowPassDetection.java:40)

### `PALCompat`

- [ ] `PALCompat.shouldPauseEntityAnim(1)`: it uses EMFEntity, which isn't translated (it's an interface) (traben/entity_model_features/mod_compat/PALCompat.java:8)

### `EntityVariableBooleanProperty`: it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)

- [ ] `EntityVariableBooleanProperty.EntityVariableBooleanProperty(2)`: its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)) (traben/entity_model_features/propeties/EntityVariableBooleanProperty.java:17)
- [ ] `EntityVariableBooleanProperty.getPropertyOrNull(2)`: it uses EntityVariableBooleanProperty, which isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)) (traben/entity_model_features/propeties/EntityVariableBooleanProperty.java:35)
- [ ] `EntityVariableBooleanProperty.testEntityInternal(1)`: its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)) (traben/entity_model_features/propeties/EntityVariableBooleanProperty.java:43)
- [ ] `EntityVariableBooleanProperty.getPropertyIds(0)`: its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)) (traben/entity_model_features/propeties/EntityVariableBooleanProperty.java:58)
- [ ] `EntityVariableBooleanProperty.getPrintableRuleInfo(0)`: its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)) (traben/entity_model_features/propeties/EntityVariableBooleanProperty.java:63)

### `EntityVariableFloatProperty`: it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)

- [ ] `EntityVariableFloatProperty.EntityVariableFloatProperty(2)`: its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)) (traben/entity_model_features/propeties/EntityVariableFloatProperty.java:19)
- [ ] `EntityVariableFloatProperty.getPropertyOrNull(2)`: it uses EntityVariableFloatProperty, which isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)) (traben/entity_model_features/propeties/EntityVariableFloatProperty.java:43)
- [ ] `EntityVariableFloatProperty.testEntityInternal(1)`: its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)) (traben/entity_model_features/propeties/EntityVariableFloatProperty.java:51)
- [ ] `EntityVariableFloatProperty.getPropertyIds(0)`: its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)) (traben/entity_model_features/propeties/EntityVariableFloatProperty.java:61)
- [ ] `EntityVariableFloatProperty.getPrintableRuleInfo(0)`: its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)) (traben/entity_model_features/propeties/EntityVariableFloatProperty.java:66)

### `EntityVariableFloatProperty.InnerTester`: it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)

- [ ] `EntityVariableFloatProperty.InnerTester.InnerTester(3)`: its class isn't translated (it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_model_features/propeties/EntityVariableFloatProperty.java:74)
- [ ] `EntityVariableFloatProperty.InnerTester.getRangeValueFromEntity(1)`: its class isn't translated (it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_model_features/propeties/EntityVariableFloatProperty.java:80)
- [ ] `EntityVariableFloatProperty.InnerTester.getPropertyIds(0)`: its class isn't translated (it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_model_features/propeties/EntityVariableFloatProperty.java:85)

### `GlobalVariableBooleanProperty`: it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)

- [ ] `GlobalVariableBooleanProperty.GlobalVariableBooleanProperty(2)`: its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)) (traben/entity_model_features/propeties/GlobalVariableBooleanProperty.java:17)
- [ ] `GlobalVariableBooleanProperty.getPropertyOrNull(2)`: it uses GlobalVariableBooleanProperty, which isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)) (traben/entity_model_features/propeties/GlobalVariableBooleanProperty.java:35)
- [ ] `GlobalVariableBooleanProperty.testEntityInternal(1)`: its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)) (traben/entity_model_features/propeties/GlobalVariableBooleanProperty.java:43)
- [ ] `GlobalVariableBooleanProperty.getPropertyIds(0)`: its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)) (traben/entity_model_features/propeties/GlobalVariableBooleanProperty.java:54)
- [ ] `GlobalVariableBooleanProperty.getPrintableRuleInfo(0)`: its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)) (traben/entity_model_features/propeties/GlobalVariableBooleanProperty.java:59)

### `GlobalVariableFloatProperty`: it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)

- [ ] `GlobalVariableFloatProperty.GlobalVariableFloatProperty(2)`: its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)) (traben/entity_model_features/propeties/GlobalVariableFloatProperty.java:20)
- [ ] `GlobalVariableFloatProperty.getPropertyOrNull(2)`: it uses GlobalVariableFloatProperty, which isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)) (traben/entity_model_features/propeties/GlobalVariableFloatProperty.java:44)
- [ ] `GlobalVariableFloatProperty.testEntityInternal(1)`: its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)) (traben/entity_model_features/propeties/GlobalVariableFloatProperty.java:52)
- [ ] `GlobalVariableFloatProperty.getPropertyIds(0)`: its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)) (traben/entity_model_features/propeties/GlobalVariableFloatProperty.java:62)
- [ ] `GlobalVariableFloatProperty.getPrintableRuleInfo(0)`: its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)) (traben/entity_model_features/propeties/GlobalVariableFloatProperty.java:67)

### `GlobalVariableFloatProperty.InnerTester`: it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)

- [ ] `GlobalVariableFloatProperty.InnerTester.InnerTester(3)`: its class isn't translated (it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_model_features/propeties/GlobalVariableFloatProperty.java:75)
- [ ] `GlobalVariableFloatProperty.InnerTester.getRangeValueFromEntity(1)`: its class isn't translated (it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_model_features/propeties/GlobalVariableFloatProperty.java:81)
- [ ] `GlobalVariableFloatProperty.InnerTester.getPropertyIds(0)`: its class isn't translated (it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_model_features/propeties/GlobalVariableFloatProperty.java:86)

### `ModelRuleIndexProperty`: it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)

- [ ] `ModelRuleIndexProperty.ModelRuleIndexProperty(2)`: its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_model_features/propeties/ModelRuleIndexProperty.java:11)
- [ ] `ModelRuleIndexProperty.getPropertyOrNull(2)`: it uses ModelRuleIndexProperty, which isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_model_features/propeties/ModelRuleIndexProperty.java:15)
- [ ] `ModelRuleIndexProperty.getPropertyIds(0)`: its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_model_features/propeties/ModelRuleIndexProperty.java:23)
- [ ] `ModelRuleIndexProperty.getValueFromEntity(1)`: its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_model_features/propeties/ModelRuleIndexProperty.java:28)

### `ModelSuffixProperty`: it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)

- [ ] `ModelSuffixProperty.ModelSuffixProperty(2)`: its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_model_features/propeties/ModelSuffixProperty.java:11)
- [ ] `ModelSuffixProperty.getPropertyOrNull(2)`: it uses ModelSuffixProperty, which isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_model_features/propeties/ModelSuffixProperty.java:15)
- [ ] `ModelSuffixProperty.getPropertyIds(0)`: its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_model_features/propeties/ModelSuffixProperty.java:23)
- [ ] `ModelSuffixProperty.getValueFromEntity(1)`: its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_model_features/propeties/ModelSuffixProperty.java:28)

### `EMFAnimationPauseHandler`

- [ ] `EMFAnimationPauseHandler.entitiesPausedParts`: it uses `UUID` (java.util) (traben/entity_model_features/utils/EMFAnimationPauseHandler.java:19)
- [ ] `EMFAnimationPauseHandler.entitiesPaused`: it uses `UUID` (java.util) (traben/entity_model_features/utils/EMFAnimationPauseHandler.java:20)
- [ ] `EMFAnimationPauseHandler.pauseListeners`: it uses EMFEntity, which isn't translated (it's an interface) (traben/entity_model_features/utils/EMFAnimationPauseHandler.java:21)
- [x] `EMFAnimationPauseHandler.checkedIfIEmotePlayerExists` (traben/entity_model_features/utils/EMFAnimationPauseHandler.java:22)
- [ ] `EMFAnimationPauseHandler.iEmotePlayerEntityType`: it uses `Class` (traben/entity_model_features/utils/EMFAnimationPauseHandler.java:23)
- [ ] `EMFAnimationPauseHandler.isPlayingEmoteMethod`: it uses `Method` (java.lang.reflect) (traben/entity_model_features/utils/EMFAnimationPauseHandler.java:24)
- [x] `EMFAnimationPauseHandler.skipEssential` (traben/entity_model_features/utils/EMFAnimationPauseHandler.java:25)
- [ ] `EMFAnimationPauseHandler.essentialIsPoseModifiedMethod`: it uses `Method` (java.lang.reflect) (traben/entity_model_features/utils/EMFAnimationPauseHandler.java:26)
- [ ] `EMFAnimationPauseHandler.shouldAnimationsPause(1)`: it uses EMFEntityRenderState, which isn't translated (it's an interface) (traben/entity_model_features/utils/EMFAnimationPauseHandler.java:28)
- [ ] `EMFAnimationPauseHandler.isPlayerEmoting_KosmX_mod(1)`: it uses EMFEntity, which isn't translated (it's an interface) (traben/entity_model_features/utils/EMFAnimationPauseHandler.java:56)
- [ ] `EMFAnimationPauseHandler.getIEmotePlayerEntityType(0)`: it uses `Class` (traben/entity_model_features/utils/EMFAnimationPauseHandler.java:73)
- [ ] `EMFAnimationPauseHandler.getIsPlayingEmoteMethod(0)`: it uses `Method` (java.lang.reflect) (traben/entity_model_features/utils/EMFAnimationPauseHandler.java:94)
- [ ] `EMFAnimationPauseHandler.isPlayerEmoting_Essential(1)`: it uses EMFEntity, which isn't translated (it's an interface) (traben/entity_model_features/utils/EMFAnimationPauseHandler.java:114)
- [ ] `EMFAnimationPauseHandler.getEntityPartsAnimPaused(0)`: it uses Minecraft's `ModelPart` (traben/entity_model_features/utils/EMFAnimationPauseHandler.java:139)

### `EMFDirectoryHandler`: it uses EMFDirectory, which isn't translated (its constants carry values)

- [ ] `EMFDirectoryHandler.EMFDirectoryHandler(4)`: its class isn't translated (it uses EMFDirectory, which isn't translated (its constants carry values)) (traben/entity_model_features/utils/EMFDirectoryHandler.java:22)
- [ ] `EMFDirectoryHandler.basic(1)`: it uses EMFDirectoryHandler, which isn't translated (it uses EMFDirectory, which isn't translated (its constants carry values)) (traben/entity_model_features/utils/EMFDirectoryHandler.java:77)
- [ ] `EMFDirectoryHandler.getDirectoryManagerOrNull(4)`: it uses EMFDirectoryHandler, which isn't translated (it uses EMFDirectory, which isn't translated (its constants carry values)) (traben/entity_model_features/utils/EMFDirectoryHandler.java:81)
- [ ] `EMFDirectoryHandler.getFileNameWithType(0)`: its class isn't translated (it uses EMFDirectory, which isn't translated (its constants carry values)) (traben/entity_model_features/utils/EMFDirectoryHandler.java:111)
- [ ] `EMFDirectoryHandler.getRelativeDirectoryLocationNoValidation(1)`: its class isn't translated (it uses EMFDirectory, which isn't translated (its constants carry values)) (traben/entity_model_features/utils/EMFDirectoryHandler.java:115)
- [ ] `EMFDirectoryHandler.packIndex(0)`: its class isn't translated (it uses EMFDirectory, which isn't translated (its constants carry values)) (traben/entity_model_features/utils/EMFDirectoryHandler.java:119)
- [ ] `EMFDirectoryHandler.validForThisBase(1)`: its class isn't translated (it uses EMFDirectory, which isn't translated (its constants carry values)) (traben/entity_model_features/utils/EMFDirectoryHandler.java:123)
- [ ] `EMFDirectoryHandler.foundModel(0)`: its class isn't translated (it uses EMFDirectory, which isn't translated (its constants carry values)) (traben/entity_model_features/utils/EMFDirectoryHandler.java:127)
- [ ] `EMFDirectoryHandler.getResourceOrNull(3)`: its class isn't translated (it uses EMFDirectory, which isn't translated (its constants carry values)) (traben/entity_model_features/utils/EMFDirectoryHandler.java:131)
- [ ] `EMFDirectoryHandler.getPackId(1)`: its class isn't translated (it uses EMFDirectory, which isn't translated (its constants carry values)) (traben/entity_model_features/utils/EMFDirectoryHandler.java:151)
- [ ] `EMFDirectoryHandler.getPackIndex(2)`: its class isn't translated (it uses EMFDirectory, which isn't translated (its constants carry values)) (traben/entity_model_features/utils/EMFDirectoryHandler.java:155)
- [ ] `EMFDirectoryHandler.getFinalFileLocation(0)`: its class isn't translated (it uses EMFDirectory, which isn't translated (its constants carry values)) (traben/entity_model_features/utils/EMFDirectoryHandler.java:159)
- [ ] `EMFDirectoryHandler.getRelativeFilePossiblyEMFOverridden(1)`: its class isn't translated (it uses EMFDirectory, which isn't translated (its constants carry values)) (traben/entity_model_features/utils/EMFDirectoryHandler.java:163)
- [ ] `EMFDirectoryHandler.toString(0)`: its class isn't translated (it uses EMFDirectory, which isn't translated (its constants carry values)) (traben/entity_model_features/utils/EMFDirectoryHandler.java:177)

### `EMFDirectoryHandler.EMFDirectory`: its constants carry values

- [ ] `EMFDirectoryHandler.EMFDirectory.getAsDirectory(2)`: it has no body (traben/entity_model_features/utils/EMFDirectoryHandler.java:211)
- [ ] `EMFDirectoryHandler.EMFDirectory.fallback(0)`: its class isn't translated (its constants carry values) (traben/entity_model_features/utils/EMFDirectoryHandler.java:213)
- [ ] `EMFDirectoryHandler.EMFDirectory.override(0)`: its class isn't translated (its constants carry values) (traben/entity_model_features/utils/EMFDirectoryHandler.java:222)

### `EMFEntity`: it's an interface

- [ ] `EMFEntity.emf$prevX(0)`: it has no body (traben/entity_model_features/utils/EMFEntity.java:8)
- [ ] `EMFEntity.emf$getX(0)`: it has no body (traben/entity_model_features/utils/EMFEntity.java:10)
- [ ] `EMFEntity.emf$prevY(0)`: it has no body (traben/entity_model_features/utils/EMFEntity.java:12)
- [ ] `EMFEntity.emf$getY(0)`: it has no body (traben/entity_model_features/utils/EMFEntity.java:14)
- [ ] `EMFEntity.emf$prevZ(0)`: it has no body (traben/entity_model_features/utils/EMFEntity.java:16)
- [ ] `EMFEntity.emf$getZ(0)`: it has no body (traben/entity_model_features/utils/EMFEntity.java:18)
- [ ] `EMFEntity.emf$prevPitch(0)`: it has no body (traben/entity_model_features/utils/EMFEntity.java:20)
- [ ] `EMFEntity.emf$getPitch(0)`: it has no body (traben/entity_model_features/utils/EMFEntity.java:22)
- [ ] `EMFEntity.emf$isTouchingWater(0)`: it has no body (traben/entity_model_features/utils/EMFEntity.java:24)
- [ ] `EMFEntity.emf$isOnFire(0)`: it has no body (traben/entity_model_features/utils/EMFEntity.java:26)
- [ ] `EMFEntity.emf$hasVehicle(0)`: it has no body (traben/entity_model_features/utils/EMFEntity.java:28)
- [ ] `EMFEntity.emf$isOnGround(0)`: it has no body (traben/entity_model_features/utils/EMFEntity.java:30)
- [ ] `EMFEntity.emf$isAlive(0)`: it has no body (traben/entity_model_features/utils/EMFEntity.java:32)
- [ ] `EMFEntity.emf$isGlowing(0)`: it has no body (traben/entity_model_features/utils/EMFEntity.java:34)
- [ ] `EMFEntity.emf$isInLava(0)`: it has no body (traben/entity_model_features/utils/EMFEntity.java:36)
- [ ] `EMFEntity.emf$isInvisible(0)`: it has no body (traben/entity_model_features/utils/EMFEntity.java:38)
- [ ] `EMFEntity.emf$hasPassengers(0)`: it has no body (traben/entity_model_features/utils/EMFEntity.java:40)
- [ ] `EMFEntity.emf$isSneaking(0)`: it has no body (traben/entity_model_features/utils/EMFEntity.java:42)
- [ ] `EMFEntity.emf$isSprinting(0)`: it has no body (traben/entity_model_features/utils/EMFEntity.java:44)
- [ ] `EMFEntity.emf$isWet(0)`: it has no body (traben/entity_model_features/utils/EMFEntity.java:46)
- [ ] `EMFEntity.emf$age(0)`: it has no body (traben/entity_model_features/utils/EMFEntity.java:48)
- [ ] `EMFEntity.emf$getYaw(0)`: it has no body (traben/entity_model_features/utils/EMFEntity.java:50)
- [ ] `EMFEntity.emf$getVelocity(0)`: it has no body (traben/entity_model_features/utils/EMFEntity.java:52)
- [ ] `EMFEntity.emf$getTypeString(0)`: it has no body (traben/entity_model_features/utils/EMFEntity.java:54)
- [ ] `EMFEntity.emf$getVariableMap(0)`: it has no body (traben/entity_model_features/utils/EMFEntity.java:56)

### `EMFLODHandler`

- [x] `EMFLODHandler.lodEntityTimers` (traben/entity_model_features/utils/EMFLODHandler.java:21)
- [x] `EMFLODHandler.lodFrameSkipping` (traben/entity_model_features/utils/EMFLODHandler.java:22)
- [x] `EMFLODHandler.lastFOV` (traben/entity_model_features/utils/EMFLODHandler.java:24)
- [x] `EMFLODHandler.setNullLodFrameSkipping(0)` (traben/entity_model_features/utils/EMFLODHandler.java:26)
- [ ] `EMFLODHandler.getLODFactorOfEntity(1)`: it uses EMFEntityRenderState, which isn't translated (it's an interface) (traben/entity_model_features/utils/EMFLODHandler.java:30)
- [ ] `EMFLODHandler.isLODSkippingThisFrame(1)`: it uses `ETF` (traben.entity_texture_features).IRIS_DETECTED (traben/entity_model_features/utils/EMFLODHandler.java:64)
- [ ] `EMFLODHandler.distanceOfEntityFrom(2)`: it uses Minecraft's `BlockPos` (traben/entity_model_features/utils/EMFLODHandler.java:108)
- [ ] `EMFLODHandler.emfState(0)`: it uses EMFEntityRenderState, which isn't translated (it's an interface) (traben/entity_model_features/utils/EMFLODHandler.java:116)
- [ ] `EMFLODHandler.emfEntity(0)`: it uses EMFEntity, which isn't translated (it's an interface) (traben/entity_model_features/utils/EMFLODHandler.java:121)

### `EMFResourceCaching`

- [x] `EMFResourceCaching.RESOURCE_EXISTENCE_CACHE` (traben/entity_model_features/utils/EMFResourceCaching.java:11)
- [x] `EMFResourceCaching.isPopulated` (traben/entity_model_features/utils/EMFResourceCaching.java:12)
- [x] `EMFResourceCaching.clearCache(0)` (traben/entity_model_features/utils/EMFResourceCaching.java:14)
- [ ] `EMFResourceCaching.populateCacheIfNeeded(1)`: it uses Minecraft's `ResourceManager` (traben/entity_model_features/utils/EMFResourceCaching.java:19)
- [ ] `EMFResourceCaching.scanPrefix(2)`: it uses Minecraft's `ResourceManager` (traben/entity_model_features/utils/EMFResourceCaching.java:27)
- [ ] `EMFResourceCaching.resourceExists(2)`: it uses Minecraft's `ResourceManager` (traben/entity_model_features/utils/EMFResourceCaching.java:40)

### `EMFUtils`

- [x] `EMFUtils.MOD_ID_SHORT` (traben/entity_model_features/utils/EMFUtils.java:35)
- [x] `EMFUtils.LOGGER` (traben/entity_model_features/utils/EMFUtils.java:36)
- [x] `EMFUtils.res(1)` (traben/entity_model_features/utils/EMFUtils.java:38)
- [x] `EMFUtils.res(2)` (traben/entity_model_features/utils/EMFUtils.java:43)
- [ ] `EMFUtils.getArrowOrNull(1)`: it uses EMFModelPartRoot, which isn't translated (it extends `EMFModelPartVanilla`) (traben/entity_model_features/utils/EMFUtils.java:48)
- [ ] `EMFUtils.renderLineBox(7)`: it uses Minecraft's `Pose` (traben/entity_model_features/utils/EMFUtils.java:60)
- [ ] `EMFUtils.renderLineBox(15)`: it uses Minecraft's `Pose` (traben/entity_model_features/utils/EMFUtils.java:64)
- [x] `EMFUtils.overrideMessage(3)` (traben/entity_model_features/utils/EMFUtils.java:114)
- [ ] `EMFUtils.log(1)`: it calls EMFUtils.log, which isn't translated (traben/entity_model_features/utils/EMFUtils.java:127)
- [ ] `EMFUtils.log(2)`: it calls EMFUtils.log, which isn't translated (traben/entity_model_features/utils/EMFUtils.java:131)
- [ ] `EMFUtils.log(3)`: it calls Minecraft's `Component`.nullToEmpty (traben/entity_model_features/utils/EMFUtils.java:135)
- [ ] `EMFUtils.chat(1)`: it creates Minecraft's `LiteralContents` (traben/entity_model_features/utils/EMFUtils.java:148)
- [ ] `EMFUtils.logWarn(1)`: it calls EMFUtils.logWarn, which isn't translated (traben/entity_model_features/utils/EMFUtils.java:155)
- [ ] `EMFUtils.logWarn(2)`: it calls Minecraft's `Component`.nullToEmpty (traben/entity_model_features/utils/EMFUtils.java:159)
- [ ] `EMFUtils.logError(1)`: it calls EMFUtils.logError, which isn't translated (traben/entity_model_features/utils/EMFUtils.java:172)
- [ ] `EMFUtils.logError(2)`: it calls Minecraft's `Component`.nullToEmpty (traben/entity_model_features/utils/EMFUtils.java:176)
- [ ] `EMFUtils.readModelPart(1)`: it uses EMFPartData, which isn't translated (it holds an object of its own class (a linked structure)) (traben/entity_model_features/utils/EMFUtils.java:189)
- [x] `EMFUtils.getIdUnique(2)` (traben/entity_model_features/utils/EMFUtils.java:232)

### `IEMFCuboidDataSupplier`: it's an interface

- [ ] `IEMFCuboidDataSupplier.emf$getTextureUV(0)`: it has no body (traben/entity_model_features/utils/IEMFCuboidDataSupplier.java:4)
- [ ] `IEMFCuboidDataSupplier.emf$getTextureXY(0)`: it has no body (traben/entity_model_features/utils/IEMFCuboidDataSupplier.java:6)
- [ ] `IEMFCuboidDataSupplier.emf$getSizeAdd(0)`: it has no body (traben/entity_model_features/utils/IEMFCuboidDataSupplier.java:8)
- [ ] `IEMFCuboidDataSupplier.emf$getMirror(0)`: it has no body (traben/entity_model_features/utils/IEMFCuboidDataSupplier.java:10)

### `IEMFCustomModelHolder`: it's an interface

- [ ] `IEMFCustomModelHolder.emf$hasModel(0)`: its class isn't translated (it's an interface) (traben/entity_model_features/utils/IEMFCustomModelHolder.java:6)
- [ ] `IEMFCustomModelHolder.emf$getModel(0)`: it has no body (traben/entity_model_features/utils/IEMFCustomModelHolder.java:10)
- [ ] `IEMFCustomModelHolder.emf$setModel(1)`: it has no body (traben/entity_model_features/utils/IEMFCustomModelHolder.java:12)

### `IEMFTextureSizeSupplier`: it's an interface

- [ ] `IEMFTextureSizeSupplier.emf$getTextureSize(0)`: it has no body (traben/entity_model_features/utils/IEMFTextureSizeSupplier.java:4)
- [ ] `IEMFTextureSizeSupplier.emf$setTextureSize(1)`: it has no body (traben/entity_model_features/utils/IEMFTextureSizeSupplier.java:6)

### `IEMFUnmodifiedLayerRootGetter`: it's an interface

- [ ] `IEMFUnmodifiedLayerRootGetter.emf$getUnmodifiedRoots(0)`: it has no body (traben/entity_model_features/utils/IEMFUnmodifiedLayerRootGetter.java:8)
- [ ] `IEMFUnmodifiedLayerRootGetter.emf$setUnmodifiedRoots(1)`: it has no body (traben/entity_model_features/utils/IEMFUnmodifiedLayerRootGetter.java:10)

### `IEMFWolfCollarHolder`: it's an interface

- [ ] `IEMFWolfCollarHolder.emf$hasCollarModel(0)`: its class isn't translated (it's an interface) (traben/entity_model_features/utils/IEMFWolfCollarHolder.java:6)
- [ ] `IEMFWolfCollarHolder.emf$getCollarModel(0)`: it has no body (traben/entity_model_features/utils/IEMFWolfCollarHolder.java:10)
- [ ] `IEMFWolfCollarHolder.emf$setCollarModel(1)`: it has no body (traben/entity_model_features/utils/IEMFWolfCollarHolder.java:12)

### `UEntityTypes`

- [ ] `UEntityTypes.CHEST`: it uses Minecraft's `BlockEntityType` (traben/entity_model_features/utils/UEntityTypes.java:9)
- [ ] `UEntityTypes.TRAPPED_CHEST`: it uses Minecraft's `BlockEntityType` (traben/entity_model_features/utils/UEntityTypes.java:10)
- [ ] `UEntityTypes.ENDER_CHEST`: it uses Minecraft's `BlockEntityType` (traben/entity_model_features/utils/UEntityTypes.java:11)
- [ ] `UEntityTypes.SHULKER_BOX`: it uses Minecraft's `BlockEntityType` (traben/entity_model_features/utils/UEntityTypes.java:12)
- [ ] `UEntityTypes.BELL`: it uses Minecraft's `BlockEntityType` (traben/entity_model_features/utils/UEntityTypes.java:13)
- [ ] `UEntityTypes.SIGN`: it uses Minecraft's `BlockEntityType` (traben/entity_model_features/utils/UEntityTypes.java:14)
- [ ] `UEntityTypes.DECORATED_POT`: it uses Minecraft's `BlockEntityType` (traben/entity_model_features/utils/UEntityTypes.java:15)
- [ ] `UEntityTypes.ENCHANTING_TABLE`: it uses Minecraft's `BlockEntityType` (traben/entity_model_features/utils/UEntityTypes.java:16)
- [ ] `UEntityTypes.LECTERN`: it uses Minecraft's `BlockEntityType` (traben/entity_model_features/utils/UEntityTypes.java:17)
- [ ] `UEntityTypes.SPECTRAL_ARROW`: it uses Minecraft's `EntityType` (traben/entity_model_features/utils/UEntityTypes.java:18)
- [ ] `UEntityTypes.BREEZE_WIND_CHARGE`: it uses Minecraft's `EntityType` (traben/entity_model_features/utils/UEntityTypes.java:19)
- [ ] `UEntityTypes.VILLAGER`: it uses Minecraft's `EntityType` (traben/entity_model_features/utils/UEntityTypes.java:20)
- [ ] `UEntityTypes.HORSE`: it uses Minecraft's `EntityType` (traben/entity_model_features/utils/UEntityTypes.java:21)
- [ ] `UEntityTypes.BLAZE`: it uses Minecraft's `EntityType` (traben/entity_model_features/utils/UEntityTypes.java:22)
- [ ] `UEntityTypes.CAMEL_HUSK`: it uses Minecraft's `EntityType` (traben/entity_model_features/utils/UEntityTypes.java:23)
