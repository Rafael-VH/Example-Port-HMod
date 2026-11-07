# Horizon port report: Farmer's Delight

`farmersdelight` 26.3-3.6.30+refabricated · FarmersDelight-26.3-3.6.30+refabricated.jar · 368 classes

**Partly convertible**

| Status | Count | Meaning |
|---|---|---|
| auto | 3 | The Horizon hook or injection is known: the wiring can be generated. The Java body still has to become Rust. |
| content | 3 | Data and assets: horizon-port-pack carries these into a content mod (.hmod). |
| needs-mapping | 50 | The kind of change carries over, but the matching Horizon function isn't in the table yet. Run `match`. |
| needs-translation | 81 | No direct equivalent: a person (or an AI agent) has to rewrite this part. |
| unsupported | 12 | Horizon has nothing this can attach to yet. |

## Entry points

- `main`: vectorwing.farmersdelight.FarmersDelight
- `client`: vectorwing.farmersdelight.client.FarmersDelightClient
- `fabric-datagen`: vectorwing.farmersdelight.data.DataGenerators
- `jei_mod_plugin`: vectorwing.farmersdelight.integration.jei.JEIPlugin
- `rei_client`: vectorwing.farmersdelight.integration.rei.client.ClientREIPlugin
- `rei_common`: vectorwing.farmersdelight.integration.rei.CommonREIPlugin
- `modmenu`: vectorwing.farmersdelight.integration.modmenu.ModMenuCompat
- `rrv_client`: vectorwing.farmersdelight.integration.rrv.RRVPlugin

## Fabric events

| Event | In | Horizon | Status | Note |
|---|---|---|---|---|
| ServerPlayNetworking.send | `FarmersDelight` | – | unsupported | Custom packets aren't supported. |
| ClientPlayNetworking.send | `FarmersDelightClient` | – | unsupported | Custom packets aren't supported. |
| UseBlockCallback.EVENT | `CuttingBoardBlock` | – | unsupported | No interaction hook yet: needs a raw target in Horizon's game code. |
| PayloadTypeRegistry.clientboundPlay | `ModNetworking` | – | unsupported | Horizon speaks the vanilla protocol; custom packets aren't supported. |
| PayloadTypeRegistry.register | `ModNetworking` | – | unsupported | Horizon speaks the vanilla protocol; custom packets aren't supported. |
| PayloadTypeRegistry.serverboundPlay | `ModNetworking` | – | unsupported | Horizon speaks the vanilla protocol; custom packets aren't supported. |
| ServerPlayNetworking.registerGlobalReceiver | `ModNetworking` | – | unsupported | Custom packets aren't supported. |
| ClientPlayNetworking.registerGlobalReceiver | `ModNetworking` | – | unsupported | Custom packets aren't supported. |
| FabricBlockEntityTypeBuilder.create | `ModBlockEntityTypes` | – | unsupported | Block entities aren't supported in Horizon mods yet. |
| FabricBlockEntityTypeBuilder.build | `ModBlockEntityTypes` | – | unsupported | Block entities aren't supported in Horizon mods yet. |

## Mixins

| Injection | In | Horizon | Status | Note |
|---|---|---|---|---|
| @Inject(HEAD) on CampfireBlock.isSmokeSource | `CampfireBaleMixin.isFDSmokeSource` | HEAD | needs-mapping | No Horizon function known for CampfireBlock.isSmokeSource: run `match`. |
| @Inject(INVOKE) on DispenserBlock.dispenseFrom at Lnet/minecraft/world/level/block/DispenserBlock;getDispenseMethod(Lnet/minecraft/world/level/Level;Lnet/minecraft/world/item/ItemStack;)Lnet/minecraft/core/dispenser/DispenseItemBehavior; | `CuttingBoardDispenserMixin.onCuttingBoardDispenseFromInject` | BEFORE_CALL on horizon_client::game::machines::Game<'_>::dispense_from | needs-mapping | Only whole statements, plain let initializers and block tails can be selected. The call it hooks (DispenserBlock.getDispenseMethod) isn't mapped to a Rust function yet. |
| @ModifyExpressionValue(FIELD) on LevelRenderer.submitBlockDestroyAnimation at Lnet/minecraft/world/level/block/RenderShape;MODEL:Lnet/minecraft/world/level/block/RenderShape; | `HideBlockBreakProgressMixin.hideBlockDamage` | MODIFY_EXPRESSION_VALUE | needs-mapping | Selects an exact Rust expression: pick it in the target function. No Horizon function known for LevelRenderer.submitBlockDestroyAnimation: run `match`. |
| @Inject(HEAD) on Player.canEat | `NourishmentAlwaysEatMixin.alwaysEatUnderNourishmentEffect` | HEAD | needs-mapping | No Horizon function known for Player.canEat: run `match`. |
| @Inject(TAIL) on Item.useOn | `PlacePumpkinPieMixin.usePumpkinPie` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for Item.useOn: run `match`. |
| @Inject(HEAD) on FenceBlock.connectsTo | `RopeFenceConnectionMixin.denyConnectionToRopeFenceGate` | HEAD | needs-mapping | No Horizon function known for FenceBlock.connectsTo: run `match`. |
| @Inject(HEAD) on FenceBlock.isSameFence | `RopeFenceConnectionMixin.denyConnectionToRopeFence` | HEAD | needs-mapping | No Horizon function known for FenceBlock.isSameFence: run `match`. |
| @WrapOperation(INVOKE) on SecondaryPoiSensor.doTick at Lcom/google/common/collect/ImmutableSet;contains(Ljava/lang/Object;)Z | `VillagersTargetRichSoilMixin.detectModdedFarmland` | WRAP_OPERATION | needs-mapping | No Horizon function known for SecondaryPoiSensor.doTick: run `match`. The call it hooks (ImmutableSet.contains) isn't mapped to a Rust function yet. |
| @Inject(HEAD) on ItemStackComponentizationFix.fixItemStack | `ItemStackComponentizationFixMixin.fixCustomStacks` | HEAD | needs-mapping | No Horizon function known for ItemStackComponentizationFix.fixItemStack: run `match`. |
| @Inject(RETURN) on V1460.registerBlockEntities | `V1460Mixin.registerFarmersDelightBlockEntities` | RETURN | needs-mapping | Horizon's RETURN runs at every normal exit and passes the return value on. No Horizon function known for V1460.registerBlockEntities: run `match`. |
| @ModifyArg(INVOKE) on V3818_3.lambda$registerTypes$0 at Lcom/mojang/datafixers/DSL;optionalFieldsLazy(Ljava/util/Map;)Lcom/mojang/datafixers/types/templates/TypeTemplate; | `V3818_3Mixin.addFDCustomComponents` | MODIFY_ARG | needs-mapping | No Horizon function known for V3818_3.lambda$registerTypes$0: run `match`. The call it hooks (DSL.optionalFieldsLazy) isn't mapped to a Rust function yet. |
| @Accessor on AbstractRecipeBookScreen.recipeBookComponent | `AbstractRecipeBookScreenAccessor.fdrf$getRecipeBookComponent` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for AbstractRecipeBookScreen.recipeBookComponent: run `match`. |
| @Accessor on CuboidItemModelWrapper.properties | `CuboidItemModelWrapperAccessor.fdrf$getProperties` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for CuboidItemModelWrapper.properties: run `match`. |
| @Accessor on CuboidItemModelWrapper.properties | `CuboidItemModelWrapperAccessor.fdrf$setProperties` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for CuboidItemModelWrapper.properties: run `match`. |
| @ModifyExpressionValue(INVOKE) on FenceGateBlock.useWithoutItem at Lnet/minecraft/world/level/block/state/properties/WoodType;fenceGateOpen()Lnet/minecraft/sounds/SoundEvent; | `FenceGateBlockMixin.fdrf$modifyUseWithoutItemOpenSound` | MODIFY_EXPRESSION_VALUE | needs-mapping | Selects an exact Rust expression: pick it in the target function. No Horizon function known for FenceGateBlock.useWithoutItem: run `match`. |
| @ModifyExpressionValue(INVOKE) on FenceGateBlock.useWithoutItem at Lnet/minecraft/world/level/block/state/properties/WoodType;fenceGateClose()Lnet/minecraft/sounds/SoundEvent; | `FenceGateBlockMixin.fdrf$modifyUseWithoutItemCloseSound` | MODIFY_EXPRESSION_VALUE | needs-mapping | Selects an exact Rust expression: pick it in the target function. No Horizon function known for FenceGateBlock.useWithoutItem: run `match`. |
| @ModifyExpressionValue(INVOKE) on FenceGateBlock.onExplosionHit at Lnet/minecraft/world/level/block/state/properties/WoodType;fenceGateOpen()Lnet/minecraft/sounds/SoundEvent; | `FenceGateBlockMixin.fdrf$modifyOnExplosionHitOpenSound` | MODIFY_EXPRESSION_VALUE | needs-mapping | Selects an exact Rust expression: pick it in the target function. No Horizon function known for FenceGateBlock.onExplosionHit: run `match`. |
| @ModifyExpressionValue(INVOKE) on FenceGateBlock.useWithoutItem at Lnet/minecraft/world/level/block/state/properties/WoodType;fenceGateClose()Lnet/minecraft/sounds/SoundEvent; | `FenceGateBlockMixin.fdrf$modifyOnExplosionHitCloseSound` | MODIFY_EXPRESSION_VALUE | needs-mapping | Selects an exact Rust expression: pick it in the target function. No Horizon function known for FenceGateBlock.useWithoutItem: run `match`. |
| @ModifyExpressionValue(INVOKE) on FenceGateBlock.neighborChanged at Lnet/minecraft/world/level/block/state/properties/WoodType;fenceGateOpen()Lnet/minecraft/sounds/SoundEvent; | `FenceGateBlockMixin.fdrf$modifyNeighborChangedOpenSound` | MODIFY_EXPRESSION_VALUE | needs-mapping | Selects an exact Rust expression: pick it in the target function. No Horizon function known for FenceGateBlock.neighborChanged: run `match`. |
| @ModifyExpressionValue(INVOKE) on FenceGateBlock.neighborChanged at Lnet/minecraft/world/level/block/state/properties/WoodType;fenceGateClose()Lnet/minecraft/sounds/SoundEvent; | `FenceGateBlockMixin.fdrf$modifyNeighborChangedCloseSound` | MODIFY_EXPRESSION_VALUE | needs-mapping | Selects an exact Rust expression: pick it in the target function. No Horizon function known for FenceGateBlock.neighborChanged: run `match`. |
| @Accessor on FoodData.exhaustionLevel | `FoodDataAccessor.fdrf$getExhaustionLevel` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for FoodData.exhaustionLevel: run `match`. |
| mixin into GameRules with no injectors (interfaces or @Shadow only) | `GameRulesTypeAccessor` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| @Invoker on GhostSlots.setInput | `GhostSlotsInvoker.fdrf$setInput` | INVOKE | needs-mapping | A [[bridge]] INVOKE of the Rust method. No Horizon function known for GhostSlots.setInput: run `match`. |
| @Invoker on GhostSlots.setResult | `GhostSlotsInvoker.fdrf$setResult` | INVOKE | needs-mapping | A [[bridge]] INVOKE of the Rust method. No Horizon function known for GhostSlots.setResult: run `match`. |
| @Accessor on GuiGraphicsExtractor.guiRenderState | `GuiGraphicsExtractorAccessor.fdrf$getGuiRenderState` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for GuiGraphicsExtractor.guiRenderState: run `match`. |
| @Accessor on GuiGraphicsExtractor.scissorStack | `GuiGraphicsExtractorAccessor.fdrf$getScissorStack` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for GuiGraphicsExtractor.scissorStack: run `match`. |
| @Inject(TAIL) on HangingSignEditScreen.<init> | `HangingSignEditScreenMixin.fdrf$fixSignTextures` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for HangingSignEditScreen.<init>: run `match`. |
| @Inject(HEAD) on Hud.extractHotbarAndDecorations | `HudMixin.farmersdelightrefabricated$captureDeltaTracker` | HEAD | needs-mapping | No Horizon function known for Hud.extractHotbarAndDecorations: run `match`. |
| @Inject(TAIL) on Hud.extractHotbarAndDecorations | `HudMixin.farmersdelightrefabricated$clearDeltaTracker` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for Hud.extractHotbarAndDecorations: run `match`. |
| @Inject(TAIL) on Hud.extractHearts | `HudMixin.farmersdelightrefabricated$renderHearts` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for Hud.extractHearts: run `match`. |
| @Inject(TAIL) on Hud.extractFood | `HudMixin.farmersdelightrefabricated$renderNourishment` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for Hud.extractFood: run `match`. |
| @Inject(TAIL) on HumanoidMobRenderer.extractHumanoidRenderState | `HumanoidMobRendererMixin.fdrf$extractSkilletEntityRenderState` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for HumanoidMobRenderer.extractHumanoidRenderState: run `match`. |
| @Inject(FIELD) on HumanoidModel.setupAnim at Lnet/minecraft/client/renderer/entity/state/HumanoidRenderState;useItemHand:Lnet/minecraft/world/InteractionHand; | `HumanoidModelMixin.farmersdelightrefabricated$setupSkilletThirdPersonAnim` | – | needs-translation | FIELD, NEW, JUMP and CONSTANT points have no horizon-mixins counterpart yet. |
| mixin into HumanoidRenderState with no injectors (interfaces or @Shadow only) | `HumanoidRenderStateMixin` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| @Inject(HEAD) on ItemStack.finishUsingItem | `ItemStackMixin.fdrf$onItemUseFinished` | HEAD | needs-mapping | No Horizon function known for ItemStack.finishUsingItem: run `match`. |
| @ModifyExpressionValue(INVOKE) on FenceGateBlock.<init> at Lnet/minecraft/world/level/block/state/properties/WoodType;fenceGateOpen()Lnet/minecraft/sounds/SoundEvent; | `LaunchpadFenceGateBlockMixin.fdrf$modifyOpenSound` | MODIFY_EXPRESSION_VALUE | needs-mapping | Selects an exact Rust expression: pick it in the target function. No Horizon function known for FenceGateBlock.<init>: run `match`. |
| @ModifyExpressionValue(INVOKE) on FenceGateBlock.<init> at Lnet/minecraft/world/level/block/state/properties/WoodType;fenceGateClose()Lnet/minecraft/sounds/SoundEvent; | `LaunchpadFenceGateBlockMixin.fdrf$modifyCloseSound` | MODIFY_EXPRESSION_VALUE | needs-mapping | Selects an exact Rust expression: pick it in the target function. No Horizon function known for FenceGateBlock.<init>: run `match`. |
| @WrapOperation(INVOKE) on Level.removeBlock at Lnet/minecraft/world/level/Level;setBlock(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;I)Z | `LevelMixin.fdrf$setBlock` | WRAP_OPERATION on horizon_world::level::update::Sim<'_, L>::remove_block | auto |  |
| @ModifyVariable(HEAD) on LivingEntity.hurtServer | `LivingEntityMixin.handleBackstabbingDamage` | MODIFY_VARIABLE | needs-mapping | Selects a Rust local by name: pick it in the target function. No Horizon function known for LivingEntity.hurtServer: run `match`. |
| @ModifyVariable(HEAD) on LivingEntity.knockback | `LivingEntityMixin.handleKnifeKnockback` | MODIFY_VARIABLE on horizon_client::mobs::Mob::knockback | auto | Selects a Rust local by name: pick it in the target function. |
| @Inject(HEAD) on MinecraftServer.onGameRuleChanged | `MinecraftServerMixin.fdrf$syncNaturalRegenerationGameRule` | HEAD | needs-mapping | No Horizon function known for MinecraftServer.onGameRuleChanged: run `match`. |
| @Inject(INVOKE) on Player.attack at Lnet/minecraft/world/entity/player/Player;getAttackStrengthScale(F)F | `PlayerMixin.captureAttackStrengthScale` | BEFORE_CALL on horizon_client::game::Game<'_>::attack | auto | Only whole statements, plain let initializers and block tails can be selected. |
| @Inject(HEAD) on Player.hurtServer | `PlayerMixin.handleSkilletAttackSound` | HEAD | needs-mapping | No Horizon function known for Player.hurtServer: run `match`. |
| @ModifyExpressionValue(INVOKE) on RecipeBookSettings.<clinit> at Lcom/mojang/serialization/codecs/RecordCodecBuilder;mapCodec(Ljava/util/function/Function;)Lcom/mojang/serialization/MapCodec; | `RecipeBookSettingsMixin.fdrf$modifyCodec` | MODIFY_EXPRESSION_VALUE | needs-mapping | Selects an exact Rust expression: pick it in the target function. No Horizon function known for RecipeBookSettings.<clinit>: run `match`. |
| @ModifyReturnValue(RETURN) on RecipeBookSettings.copy | `RecipeBookSettingsMixin.fdrf$copyCookingBook` | RETURN | needs-mapping | No Horizon function known for RecipeBookSettings.copy: run `match`. |
| @Inject(TAIL) on RecipeBookSettings.replaceFrom | `RecipeBookSettingsMixin.fdrf$replaceFromCookingBook` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for RecipeBookSettings.replaceFrom: run `match`. |
| @Inject(HEAD) on RecipeBookSettings.getSettings | `RecipeBookSettingsMixin.fdrf$getCookingBookSettings` | HEAD | needs-mapping | No Horizon function known for RecipeBookSettings.getSettings: run `match`. |
| @Inject(HEAD) on RecipeBookSettings.updateSettings | `RecipeBookSettingsMixin.fdrf$updateCookingBookSettings` | HEAD | needs-mapping | No Horizon function known for RecipeBookSettings.updateSettings: run `match`. |
| @Invoker on TypeSettings.codec | `RecipeBookSettingsTypeSettingsInvoker.fdrf$invokeCodec` | INVOKE | needs-mapping | A [[bridge]] INVOKE of the Rust method. No Horizon function known for TypeSettings.codec: run `match`. |
| mixin into RecipeBookType with no injectors (interfaces or @Shadow only) | `RecipeBookTypeMixin` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| @Accessor on RecipeManager.RECIPE_PROPERTY_SETS | `RecipeManagerMixin.fdrf$setRECIPE_PROPERTY_SETS` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for RecipeManager.RECIPE_PROPERTY_SETS: run `match`. |
| @Inject(TAIL) on RecipeManager.<clinit> | `RecipeManagerMixin.fdrf$modifyRecipePropertySets` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for RecipeManager.<clinit>: run `match`. |
| mixin into SearchRecipeBookCategory with no injectors (interfaces or @Shadow only) | `SearchRecipeBookCategoryMixin` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| @Inject(INVOKE) on ServerRecipeBook.sendInitialRecipeBook at Lnet/minecraft/server/network/ServerGamePacketListenerImpl;send(Lnet/minecraft/network/protocol/Packet;)V | `ServerRecipeBookMixin.fdrf$sendInitialCategoryRecipeBook` | BEFORE_CALL | needs-mapping | Only whole statements, plain let initializers and block tails can be selected. No Horizon function known for ServerRecipeBook.sendInitialRecipeBook: run `match`. The call it hooks (ServerGamePacketListenerImpl.send) isn't mapped to a Rust function yet. |
| @ModifyExpressionValue(FIELD) on SignBlockEntity.<init> at Lnet/minecraft/world/level/block/entity/SignText;EMPTY:Lnet/minecraft/world/level/block/entity/SignText; | `SignBlockEntityMixin.fdrf$fixSignTextColor` | MODIFY_EXPRESSION_VALUE | needs-mapping | Selects an exact Rust expression: pick it in the target function. No Horizon function known for SignBlockEntity.<init>: run `match`. |
| @ModifyExpressionValue(FIELD) on SignBlockEntity.loadAdditional at Lnet/minecraft/world/level/block/entity/SignText;EMPTY:Lnet/minecraft/world/level/block/entity/SignText; | `SignBlockEntityMixin.fdrf$fixSignTextColor` | MODIFY_EXPRESSION_VALUE | needs-mapping | Selects an exact Rust expression: pick it in the target function. No Horizon function known for SignBlockEntity.loadAdditional: run `match`. |
| @ModifyExpressionValue(FIELD) on SignBlockEntity.applyImplicitComponents at Lnet/minecraft/world/level/block/entity/SignText;EMPTY:Lnet/minecraft/world/level/block/entity/SignText; | `SignBlockEntityMixin.fdrf$fixSignTextColor` | MODIFY_EXPRESSION_VALUE | needs-mapping | Selects an exact Rust expression: pick it in the target function. No Horizon function known for SignBlockEntity.applyImplicitComponents: run `match`. |
| @Inject(TAIL) on SignEditScreen.<init> | `SignEditScreenMixin.fdrf$fixSignTextures` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for SignEditScreen.<init>: run `match`. |

## Classes extending the game

| Class | In | Status | Note |
|---|---|---|---|
| extends RecipeBookComponent | `CookingPotRecipeBookComponent` | needs-translation | No mapping for this superclass yet. |
| extends AbstractRecipeBookScreen | `CookingPotScreen` | needs-translation | No mapping for this superclass yet. |
| extends SingleQuadParticle | `SparkleParticle` | needs-translation | No mapping for this superclass yet. |
| extends SingleQuadParticle | `StarParticle` | needs-translation | No mapping for this superclass yet. |
| extends SingleQuadParticle | `SteamParticle` | needs-translation | No mapping for this superclass yet. |
| extends BlockEntityRenderState | `CuttingBoardRenderState` | needs-translation | No mapping for this superclass yet. |
| extends BlockEntityRenderState | `SkilletRenderState` | needs-translation | No mapping for this superclass yet. |
| extends BlockEntityRenderState | `StoveRenderState` | needs-translation | No mapping for this superclass yet. |
| extends SimpleCriterionTrigger | `CuttingBoardTrigger` | needs-translation | No mapping for this superclass yet. |
| extends BaseEntityBlock | `AbstractStoveBlock` | needs-translation | No mapping for this superclass yet. |
| extends BaseEntityBlock | `BasketBlock` | needs-translation | No mapping for this superclass yet. |
| extends VegetationBlock | `BuddingBushBlock` | needs-translation | No mapping for this superclass yet. |
| extends CropBlock | `CabbageBlock` | needs-translation | No mapping for this superclass yet. |
| extends BaseEntityBlock | `CabinetBlock` | needs-translation | No mapping for this superclass yet. |
| extends Block | `CanvasRugBlock` | needs-translation | Overrides getShape, propagatesSkylightDown: the content ports, that behaviour needs native code. |
| extends CeilingHangingSignBlock | `CeilingHangingCanvasSignBlock` | needs-translation | No mapping for this superclass yet. |
| extends BaseEntityBlock | `CookingPotBlock` | needs-translation | No mapping for this superclass yet. |
| extends BaseEntityBlock | `CuttingBoardBlock` | needs-translation | No mapping for this superclass yet. |
| extends Block | `FeastBlock` | needs-translation | Overrides getServingsProperty, getMaxServings, getServingItem, getShape, useItemOn, takeServing, getStateForPlacement, updateShape, canSurvive, createBlockStateDefinition, getAnalogOutputSignal, hasAnalogOutputSignal, isPathfindable: the content ports, that behaviour needs native code. |
| extends VegetationBlock | `MushroomColonyBlock` | needs-translation | No mapping for this superclass yet. |
| extends CropBlock | `OnionBlock` | needs-translation | No mapping for this superclass yet. |
| extends Block | `OrganicCompostBlock` | needs-translation | Overrides isRandomlyTicking, createBlockStateDefinition, getMaxCompostingStage, randomTick, hasAnalogOutputSignal, getAnalogOutputSignal, animateTick: the content ports, that behaviour needs native code. |
| extends Block | `PieBlock` | needs-translation | Overrides getPieSliceItem, getMaxBites, getShape, buildShapes, getStateForPlacement, useItemOn, useWithoutItem, consumeBite, cutSlice, updateShape, canSurvive, createBlockStateDefinition, getAnalogOutputSignal, hasAnalogOutputSignal, isPathfindable: the content ports, that behaviour needs native code. |
| extends Block | `RiceBaleBlock` | needs-translation | Overrides fallOn, getStateForPlacement, getFireSpreadSpeed, getFlammability, createBlockStateDefinition, rotate, mirror: the content ports, that behaviour needs native code. |
| extends VegetationBlock | `RiceBlock` | needs-translation | No mapping for this superclass yet. |
| extends CropBlock | `RicePaniclesBlock` | needs-translation | No mapping for this superclass yet. |
| extends Block | `RichSoilBlock` | needs-translation | Overrides init, randomTick, tryBoostingPlantsAboveAndBelow, boostPlant, convertMushroomToColony: the content ports, that behaviour needs native code. |
| extends FarmlandBlock | `RichSoilFarmlandBlock` | needs-translation | No mapping for this superclass yet. |
| extends IronBarsBlock | `RopeBlock` | needs-translation | No mapping for this superclass yet. |
| extends CrossCollisionBlock | `RopeFenceBlock` | needs-translation | No mapping for this superclass yet. |
| extends FenceGateBlock | `RopeFenceGateBlock` | needs-translation | No mapping for this superclass yet. |
| extends Block | `SafetyNetBlock` | needs-translation | Overrides createBlockStateDefinition, getStateForPlacement, updateShape, getFluidState, getShape, fallOn: the content ports, that behaviour needs native code. |
| extends VegetationBlock | `SandyShrubBlock` | needs-translation | No mapping for this superclass yet. |
| extends BaseEntityBlock | `SkilletBlock` | needs-translation | No mapping for this superclass yet. |
| extends StandingSignBlock | `StandingCanvasSignBlock` | needs-translation | No mapping for this superclass yet. |
| extends HayBlock | `StrawBaleBlock` | needs-translation | No mapping for this superclass yet. |
| extends Block | `TatamiBlock` | needs-translation | Overrides getStateForPlacement, setPlacedBy, updateShape, createBlockStateDefinition, rotate, mirror: the content ports, that behaviour needs native code. |
| extends HorizontalDirectionalBlock | `TatamiHalfMatBlock` | needs-translation | No mapping for this superclass yet. |
| extends HorizontalDirectionalBlock | `TatamiMatBlock` | needs-translation | No mapping for this superclass yet. |
| extends CropBlock | `TomatoBlock` | needs-translation | No mapping for this superclass yet. |
| extends WallSignBlock | `WallCanvasSignBlock` | needs-translation | No mapping for this superclass yet. |
| extends WallHangingSignBlock | `WallHangingCanvasSignBlock` | needs-translation | No mapping for this superclass yet. |
| extends FlowerBlock | `WildCropBlock` | needs-translation | No mapping for this superclass yet. |
| extends DoublePlantBlock | `WildRiceBlock` | needs-translation | No mapping for this superclass yet. |
| extends BlockEntity | `AbstractStoveBlockEntity` | unsupported | Block entities aren't supported in Horizon mods yet. |
| extends RandomizableContainerBlockEntity | `BasketBlockEntity` | needs-translation | No mapping for this superclass yet. |
| extends ContainerOpenersCounter | `CabinetBlockEntity$1` | needs-translation | No mapping for this superclass yet. |
| extends RandomizableContainerBlockEntity | `CabinetBlockEntity` | needs-translation | No mapping for this superclass yet. |
| extends BlockEntity | `SyncedBlockEntity` | unsupported | Block entities aren't supported in Horizon mods yet. |
| extends RecipeBookMenu | `CookingPotMenu` | needs-translation | No mapping for this superclass yet. |
| extends OptionalDispenseItemBehavior | `CuttingBoardDispenseBehavior` | needs-translation | No mapping for this superclass yet. |
| extends CustomRecipe | `DoughRecipe` | needs-translation | No mapping for this superclass yet. |
| extends CustomRecipe | `FoodServingRecipe` | needs-translation | No mapping for this superclass yet. |
| extends MobEffect | `ComfortEffect` | needs-translation | No mapping for this superclass yet. |
| extends MobEffect | `NourishmentEffect` | needs-translation | No mapping for this superclass yet. |
| extends ThrowableItemProjectile | `RottenTomatoEntity` | needs-translation | No mapping for this superclass yet. |
| extends Item | `ConsumableItem` | needs-translation | Overrides finishUsingItem, affectConsumer, appendHoverText: the content ports, that behaviour needs native code. |
| extends BlockItem | `CookingPotItem` | needs-translation | Overrides isBarVisible, getBarWidth, getBarColor, getTooltipImage, getServingCount: the content ports, that behaviour needs native code. |
| extends BlockItem | `FuelBlockItem` | needs-translation | Overrides getBurnTime: the content ports, that behaviour needs native code. |
| extends Item | `FuelItem` | needs-translation | Overrides getBurnTime: the content ports, that behaviour needs native code. |
| extends Item | `HorseFeedItem` | needs-translation | Overrides init, appendHoverText, interactLivingEntity: the content ports, that behaviour needs native code. |
| extends Item | `KnifeItem` | needs-translation | Overrides init, createAttributes, canBeEnchantedWith, useOn: the content ports, that behaviour needs native code. |
| extends BlockItem | `MushroomColonyItem` | needs-translation | Overrides getPlacementState: the content ports, that behaviour needs native code. |
| extends BlockItem | `PlaceableItem` | needs-translation | Overrides appendHoverText: the content ports, that behaviour needs native code. |
| extends BlockItem | `RiceItem` | needs-translation | Overrides useOn: the content ports, that behaviour needs native code. |
| extends BlockItem | `RopeItem` | needs-translation | Overrides updatePlacementContext, mustSurvive: the content ports, that behaviour needs native code. |
| extends Item | `RottenTomatoItem` | needs-translation | Overrides use, asProjectile: the content ports, that behaviour needs native code. |
| extends BlockItem | `SkilletItem` | needs-translation | Overrides createAttributes, allowComponentsUpdateAnimation, compareComponents, postHurtEnemy, isPlayerNearHeatSource, appendHoverText, getUseDuration, use, onUseTick, releaseUsing, finishUsingItem, getBarWidth, getClientPlayerHack, getBarColor, isBarVisible, getCookingRecipe, mineBlock, place, canBeEnchantedWith: the content ports, that behaviour needs native code. |
| extends LootItemConditionalFunction | `CopySkilletFunction` | needs-translation | No mapping for this superclass yet. |
| extends LootItemConditionalFunction | `SmokerCookFunction` | needs-translation | No mapping for this superclass yet. |
| extends BlockItem | `ModItems$1` | needs-translation | Overrides registerBlocks: the content ports, that behaviour needs native code. |
| extends BlockItem | `ModItems$2` | needs-translation | Overrides appendHoverText: the content ports, that behaviour needs native code. |
| extends RecipeProvider | `Recipes$1` | needs-translation | No mapping for this superclass yet. |
| extends Slot | `ItemHandlerSlot` | needs-translation | No mapping for this superclass yet. |
| extends Entry | `FabricConfigListScreen$ConfigButton` | needs-translation | No mapping for this superclass yet. |
| extends ContainerObjectSelectionList | `FabricConfigListScreen$ConfigList` | needs-translation | No mapping for this superclass yet. |
| extends Screen | `FabricConfigListScreen` | needs-translation | No general screen hook; simple option pages can use the settings hook. |

## Access wideners

| File | In | Status | Note |
|---|---|---|---|
| access widener (opens private game fields) | `farmersdelight.classtweaker` | needs-translation | Native helpers already see the game's fields, once each field is mapped. |

## Content

| What | In | Status | Note |
|---|---|---|---|
| registers blocks or items in code | `ModEffects` | content | Registration and properties: horizon-port-pack carries them as a content mod. |
| registers blocks or items in code | `RegUtils` | content | Registration and properties: horizon-port-pack carries them as a content mod. |
| 1506 asset files, 955 data files | `jar resources` | content | Recipes, loot, tags, models, textures, worldgen: horizon-port-pack carries these. |

## Game methods and fields used

83 of 2063 are mapped to Horizon. Every Java body that uses an unmapped one needs it mapped before its Rust can be written.

<details><summary>Mapped</summary>

| Java | Horizon |
|---|---|
| `com/mojang/blaze3d/vertex/PoseStack#rotateDegrees` | horizon_client_render::camera::rotate_degrees (suggested) |
| `net/minecraft/client/Minecraft#font` | `(implicit)` |
| `net/minecraft/client/Minecraft#getInstance` | `self` |
| `net/minecraft/client/Minecraft#player` | `self.native_player()` |
| `net/minecraft/client/gui/Font#width` | `self.native_text_width(text)` |
| `net/minecraft/client/renderer/item/ItemStackRenderState#submit` | horizon_client::item_render::ItemRenderState::submit (suggested) |
| `net/minecraft/core/Direction#getOpposite` | horizon_client::mobs::shulker::opposite (suggested) |
| `net/minecraft/core/Registry#register` | content |
| `net/minecraft/core/Registry#registerForHolder` | content |
| `net/minecraft/core/registries/BuiltInRegistries#BLOCK` | content |
| `net/minecraft/core/registries/BuiltInRegistries#ITEM` | content |
| `net/minecraft/network/chat/MutableComponent#append` | horizon_client::game::dedicated::append (suggested) |
| `net/minecraft/network/chat/MutableComponent#withStyle` | horizon_client_core::text::Text::with_style (suggested) |
| `net/minecraft/network/codec/ByteBufCodecs#list` | horizon_protocol::item::skip::list (suggested) |
| `net/minecraft/network/codec/ByteBufCodecs#optional` | horizon_protocol::item::skip::optional (suggested) |
| `net/minecraft/server/level/ServerLevel#destroyBlock` | horizon_world::level::update::Sim<'_, L>::destroy_block (suggested) |
| `net/minecraft/server/level/ServerLevel#getRawBrightness` | horizon_world::level::query::Sim<'_, L>::raw_brightness (suggested) |
| `net/minecraft/server/level/ServerLevel#hasChunksAt` | horizon_client::raid::has_chunks_at (suggested) |
| `net/minecraft/server/level/ServerLevel#isClientSide` | horizon_world::level::is_client_side (suggested) |
| `net/minecraft/server/level/ServerLevel#isRainingAt` | horizon_world::level::weather::Behaviours::is_raining_at (suggested) |
| `net/minecraft/server/level/ServerLevel#sendParticles` | horizon_client::game::fishing::Game<'_>::send_particles (suggested) |
| `net/minecraft/server/level/ServerLevel#setBlock` | horizon_world::level::Behaviours::set_block (suggested) |
| `net/minecraft/util/ARGB#colorFromFloat` | horizon_client_render::hud_layout::color_from_float (suggested) |
| `net/minecraft/util/ARGB#opaque` | horizon_client_render::level_loading::opaque (suggested) |
| `net/minecraft/util/Mth#ceil` | horizon_client_render::hud_layout::ceil (suggested) |
| `net/minecraft/util/Mth#clamp` | horizon_client_render::chat::clamp (suggested) |
| `net/minecraft/util/Mth#floor` | horizon_client_render::gfx::occlusion::floor (suggested) |
| `net/minecraft/util/Mth#frac` | horizon_client_render::border_render::frac (suggested) |
| `net/minecraft/util/Mth#nextInt` | horizon_world::worldgen::structure::ocean_ruin::next_int (suggested) |
| `net/minecraft/world/Containers#dropItemStack` | horizon_world::level::removal::drop_item_stack (suggested) |
| `net/minecraft/world/effect/MobEffectUtil#formatDuration` | horizon_client_render::brewing::format_duration (suggested) |
| `net/minecraft/world/entity/Entity#getX` | `self.native_player()?.x` |
| `net/minecraft/world/entity/Entity#getY` | `self.native_player()?.y` |
| `net/minecraft/world/entity/Entity#getZ` | `self.native_player()?.z` |
| `net/minecraft/world/entity/Entity#hurt` | horizon_world::living_fall::hurt (suggested) |
| `net/minecraft/world/entity/LivingEntity#extinguishFire` | horizon_client::mobs::Mob::extinguish_fire (suggested) |
| `net/minecraft/world/entity/LivingEntity#getViewVector` | horizon_client::mobs::goals::Mob::view_vector (suggested) |
| `net/minecraft/world/entity/LivingEntity#hasEffect` | horizon_client::mobs::Seen::has_effect (suggested) |
| `net/minecraft/world/entity/LivingEntity#knockback` | horizon_client::mobs::Mob::knockback (suggested) |
| `net/minecraft/world/entity/ai/attributes/AttributeInstance#getValue` | horizon_client::mobs::finalize::value (suggested) |
| `net/minecraft/world/entity/player/Inventory#add` | horizon_client::cauldron::add (suggested) |
| `net/minecraft/world/entity/player/Player#attack` | horizon_client::game::Game<'_>::attack (suggested) |
| `net/minecraft/world/entity/player/Player#causeFoodExhaustion` | horizon_client::hunger::PlayerMotion::cause_food_exhaustion (suggested) |
| `net/minecraft/world/entity/player/Player#getAttackStrengthScale` | horizon_client::game::Game<'_>::attack_strength_scale (suggested) |
| `net/minecraft/world/entity/player/Player#hasEffect` | horizon_client::mobs::PlayerView::has_effect (suggested) |
| `net/minecraft/world/entity/player/Player#isOnFire` | horizon_client::mobs::Mob::is_on_fire (suggested) |
| `net/minecraft/world/entity/player/StackedItemContents#accountStack` | horizon_client::menu::place::Stacked::account_stack (suggested) |
| `net/minecraft/world/item/BlockItem#place` | horizon_world::level::place::Sim<'_, L>::place (suggested) |
| `net/minecraft/world/item/Item#<init>` | content |
| `net/minecraft/world/item/Item$Properties#durability` | content |
| `net/minecraft/world/item/Item$Properties#stacksTo` | content |
| `net/minecraft/world/item/ItemStack#getCount` | horizon_world::loot::LootStack::get_count (suggested) |
| `net/minecraft/world/item/ItemStack#getRarity` | horizon_client::item_tooltip::rarity (suggested) |
| `net/minecraft/world/item/ItemStack#hurtAndBreak` | horizon_client::game::dispenser::Game<'_>::hurt_and_break (suggested) |
| `net/minecraft/world/item/ItemStack#isSameItemSameComponents` | horizon_items::item_data::ItemStack::same_item_same_components (suggested) |
| `net/minecraft/world/item/ItemStack#matches` | horizon_client_render::vault::matches (suggested) |
| `net/minecraft/world/item/ItemStack#shrink` | horizon_client::game::dispenser::shrink (suggested) |
| `net/minecraft/world/item/ItemStackTemplate#create` | horizon_client::crafting::Template::create (suggested) |
| `net/minecraft/world/level/Level#destroyBlock` | horizon_world::level::update::Sim<'_, L>::destroy_block (suggested) |
| `net/minecraft/world/level/Level#getMaxY` | horizon_client::game::Dim::max_y (suggested) |
| `net/minecraft/world/level/Level#getMinY` | horizon_client::game::Dim::min_y (suggested) |
| `net/minecraft/world/level/Level#isClientSide` | horizon_world::level::is_client_side (suggested) |
| `net/minecraft/world/level/Level#removeBlock` | horizon_world::level::update::Sim<'_, L>::remove_block (suggested) |
| `net/minecraft/world/level/Level#setBlock` | horizon_world::level::Behaviours::set_block (suggested) |
| `net/minecraft/world/level/Level#updateNeighborsAt` | horizon_world::level::Behaviours::update_neighbors_at (suggested) |
| `net/minecraft/world/level/Level#updateNeighbourForOutputSignal` | horizon_world::level::update::Sim<'_, L>::update_neighbour_for_output_signal (suggested) |
| `net/minecraft/world/level/WorldGenLevel#getBiome` | horizon_world::worldgen::structure::legacy::biome (suggested) |
| `net/minecraft/world/level/block/Block#<init>` | content |
| `net/minecraft/world/level/block/Block#canSupportCenter` | horizon_world::level::query::Sim<'_, L>::can_support_center (suggested) |
| `net/minecraft/world/level/block/Block#canSupportRigidBlock` | horizon_world::level::query::Sim<'_, L>::can_support_rigid_block (suggested) |
| `net/minecraft/world/level/block/Block#getStateForPlacement` | horizon_world::level::place::Sim<'_, L>::state_for_placement (suggested) |
| `net/minecraft/world/level/block/Block#setPlacedBy` | horizon_world::level::place::Sim<'_, L>::set_placed_by (suggested) |
| `net/minecraft/world/level/block/BonemealableBlock#performBonemeal` | horizon_world::level::bonemeal::Sim<'_, L>::perform_bonemeal (suggested) |
| `net/minecraft/world/level/block/DispenserBlock#dispenseFrom` | horizon_client::game::machines::Game<'_>::dispense_from (suggested) |
| `net/minecraft/world/level/block/VegetationBlock#mayPlaceOn` | horizon_world::level::survive::Sim<'_, L>::may_place_on (suggested) |
| `net/minecraft/world/level/block/state/BlockState#canBeReplaced` | horizon_world::level::place::Sim<'_, L>::can_be_replaced (suggested) |
| `net/minecraft/world/level/block/state/BlockState#canSurvive` | horizon_world::level::survive::Sim<'_, L>::can_survive (suggested) |
| `net/minecraft/world/level/levelgen/structure/templatesystem/StructureTemplate#load` | horizon_world::worldgen::structure::template::StructureTemplate::from_nbt (suggested) |
| `net/minecraft/world/phys/AABB#intersects` | horizon_core::geometry::Aabb::intersects_corners (suggested) |
| `net/minecraft/world/phys/Vec3#normalize` | horizon_client::mobs::creaking::normalize (suggested) |
| `net/minecraft/world/phys/shapes/Shapes#joinIsNotEmpty` | horizon_core::geometry::shape::join::VoxelShape::join_is_not_empty (suggested) |
| `net/minecraft/world/phys/shapes/Shapes#joinUnoptimized` | horizon_core::geometry::shape::join::VoxelShape::join_unoptimized (suggested) |
| `net/minecraft/world/phys/shapes/VoxelShape#optimize` | horizon_core::geometry::shape::create::VoxelShape::optimized (suggested) |

</details>

<details><summary>Not mapped yet</summary>

| Java | Kind | Classes using it |
|---|---|---|
| `com/google/common/collect/ImmutableSet#contains` | method | 1 |
| `com/mojang/blaze3d/vertex/PoseStack#popPose` | method | 4 |
| `com/mojang/blaze3d/vertex/PoseStack#pushPose` | method | 4 |
| `com/mojang/blaze3d/vertex/PoseStack#scale` | method | 4 |
| `com/mojang/blaze3d/vertex/PoseStack#translate` | method | 5 |
| `com/mojang/datafixers/DSL#optionalFieldsLazy` | method | 1 |
| `net/minecraft/ChatFormatting#BLUE` | field | 2 |
| `net/minecraft/ChatFormatting#DARK_GRAY` | field | 1 |
| `net/minecraft/ChatFormatting#DARK_PURPLE` | field | 1 |
| `net/minecraft/ChatFormatting#GOLD` | field | 3 |
| `net/minecraft/ChatFormatting#GRAY` | field | 3 |
| `net/minecraft/ChatFormatting#ITALIC` | field | 1 |
| `net/minecraft/ChatFormatting#RED` | field | 1 |
| `net/minecraft/CrashReport#addCategory` | method | 1 |
| `net/minecraft/CrashReport#forThrowable` | method | 1 |
| `net/minecraft/CrashReportCategory#setDetail` | method | 2 |
| `net/minecraft/ReportedException#<init>` | method | 1 |
| `net/minecraft/SharedConstants#getCurrentVersion` | method | 1 |
| `net/minecraft/WorldVersion#dataVersion` | method | 1 |
| `net/minecraft/advancements/Advancement$Builder#addCriterion` | method | 2 |
| `net/minecraft/advancements/Advancement$Builder#advancement` | method | 1 |
| `net/minecraft/advancements/Advancement$Builder#build` | method | 1 |
| `net/minecraft/advancements/Advancement$Builder#display` | method | 1 |
| `net/minecraft/advancements/Advancement$Builder#parent` | method | 1 |
| `net/minecraft/advancements/Advancement$Builder#requirements` | method | 2 |
| `net/minecraft/advancements/Advancement$Builder#rewards` | method | 2 |
| `net/minecraft/advancements/Advancement$Builder#rootDisplay` | method | 1 |
| `net/minecraft/advancements/Advancement$Builder#save` | method | 1 |
| `net/minecraft/advancements/AdvancementRequirements$Strategy#OR` | field | 2 |
| `net/minecraft/advancements/AdvancementRewards$Builder#experience` | method | 1 |
| `net/minecraft/advancements/AdvancementRewards$Builder#recipe` | method | 1 |
| `net/minecraft/advancements/AdvancementType#CHALLENGE` | field | 1 |
| `net/minecraft/advancements/AdvancementType#GOAL` | field | 1 |
| `net/minecraft/advancements/AdvancementType#TASK` | field | 1 |
| `net/minecraft/advancements/predicates/BlockPredicate$Builder#block` | method | 2 |
| `net/minecraft/advancements/predicates/BlockPredicate$Builder#of` | method | 2 |
| `net/minecraft/advancements/predicates/BlockPredicate$Builder#setProperties` | method | 2 |
| `net/minecraft/advancements/predicates/DamagePredicate$Builder#build` | method | 1 |
| `net/minecraft/advancements/predicates/DamagePredicate$Builder#damageInstance` | method | 1 |
| `net/minecraft/advancements/predicates/DamagePredicate$Builder#type` | method | 1 |
| `net/minecraft/advancements/predicates/DamageSourcePredicate$Builder#damageType` | method | 1 |
| `net/minecraft/advancements/predicates/DamageSourcePredicate$Builder#direct` | method | 1 |
| `net/minecraft/advancements/predicates/DamageSourcePredicate$Builder#tag` | method | 1 |
| `net/minecraft/advancements/predicates/DataComponentMatchers$Builder#build` | method | 1 |
| `net/minecraft/advancements/predicates/DataComponentMatchers$Builder#components` | method | 1 |
| `net/minecraft/advancements/predicates/DataComponentMatchers$Builder#partial` | method | 1 |
| `net/minecraft/advancements/predicates/EnchantmentPredicate#<init>` | method | 1 |
| `net/minecraft/advancements/predicates/ItemPredicate$Builder#build` | method | 1 |
| `net/minecraft/advancements/predicates/ItemPredicate$Builder#item` | method | 4 |
| `net/minecraft/advancements/predicates/ItemPredicate$Builder#of` | method | 4 |
| `net/minecraft/advancements/predicates/ItemPredicate$Builder#withComponents` | method | 1 |
| `net/minecraft/advancements/predicates/LocationPredicate$Builder#location` | method | 2 |
| `net/minecraft/advancements/predicates/LocationPredicate$Builder#setBlock` | method | 2 |
| `net/minecraft/advancements/predicates/MinMaxBounds$Ints#ANY` | field | 1 |
| `net/minecraft/advancements/predicates/MobEffectsPredicate$Builder#and` | method | 1 |
| `net/minecraft/advancements/predicates/MobEffectsPredicate$Builder#effects` | method | 1 |
| `net/minecraft/advancements/predicates/StatePropertiesPredicate$Builder#hasProperty` | method | 5 |
| `net/minecraft/advancements/predicates/StatePropertiesPredicate$Builder#properties` | method | 3 |
| `net/minecraft/advancements/predicates/TagPredicate#is` | method | 1 |
| `net/minecraft/advancements/predicates/entity/EntityEquipmentPredicate$Builder#equipment` | method | 1 |
| `net/minecraft/advancements/predicates/entity/EntityEquipmentPredicate$Builder#mainhand` | method | 1 |
| `net/minecraft/advancements/predicates/entity/EntityFlagsPredicate$Builder#flags` | method | 1 |
| `net/minecraft/advancements/predicates/entity/EntityFlagsPredicate$Builder#setOnFire` | method | 1 |
| `net/minecraft/advancements/predicates/entity/EntityPredicate$Builder#build` | method | 1 |
| `net/minecraft/advancements/predicates/entity/EntityPredicate$Builder#entity` | method | 2 |
| `net/minecraft/advancements/predicates/entity/EntityPredicate$Builder#equipment` | method | 1 |
| `net/minecraft/advancements/predicates/entity/EntityPredicate$Builder#flags` | method | 1 |
| `net/minecraft/advancements/predicates/entity/EntityPredicate$Builder#of` | method | 2 |
| `net/minecraft/advancements/triggers/ConsumeItemTrigger#trigger` | method | 2 |
| `net/minecraft/advancements/triggers/ConsumeItemTrigger$TriggerInstance#usedItem` | method | 1 |
| `net/minecraft/advancements/triggers/CriteriaTriggers#CONSUME_ITEM` | field | 2 |
| `net/minecraft/advancements/triggers/CriteriaTriggers#DEFAULT_BLOCK_USE` | field | 1 |
| `net/minecraft/advancements/triggers/DefaultBlockInteractionTrigger#createCriterion` | method | 1 |
| `net/minecraft/advancements/triggers/DefaultBlockInteractionTrigger$TriggerInstance#<init>` | method | 1 |
| `net/minecraft/advancements/triggers/EffectsChangedTrigger$TriggerInstance#hasEffects` | method | 1 |
| `net/minecraft/advancements/triggers/InventoryChangeTrigger$TriggerInstance#hasItems` | method | 6 |
| `net/minecraft/advancements/triggers/ItemUsedOnLocationTrigger$TriggerInstance#placedBlock` | method | 1 |
| `net/minecraft/advancements/triggers/PlayerHurtEntityTrigger$TriggerInstance#playerHurtEntity` | method | 1 |
| `net/minecraft/advancements/triggers/RecipeUnlockedTrigger#unlocked` | method | 1 |
| `net/minecraft/advancements/triggers/SimpleCriterionTrigger#<init>` | method | 1 |
| `net/minecraft/client/ClientRecipeBook#setFiltering` | method | 1 |
| `net/minecraft/client/ClientRecipeBook#setOpen` | method | 1 |
| `net/minecraft/client/DeltaTracker#getGameTimeDeltaPartialTick` | method | 2 |
| `net/minecraft/client/Minecraft#execute` | method | 2 |
| `net/minecraft/client/Minecraft#gameMode` | field | 1 |
| `net/minecraft/client/Minecraft#getDeltaTracker` | method | 2 |
| `net/minecraft/client/Minecraft#gui` | field | 4 |
| `net/minecraft/client/Minecraft#level` | field | 4 |
| `net/minecraft/client/data/models/BlockModelGenerators#ROTATION_HORIZONTAL_FACING_ALT` | field | 1 |
| `net/minecraft/client/data/models/BlockModelGenerators#blockStateOutput` | field | 1 |
| `net/minecraft/client/data/models/BlockModelGenerators#createHangingSign` | method | 1 |
| `net/minecraft/client/data/models/BlockModelGenerators#createSign` | method | 1 |
| `net/minecraft/client/data/models/BlockModelGenerators#modelOutput` | field | 1 |
| `net/minecraft/client/data/models/BlockModelGenerators#plainVariant` | method | 1 |
| `net/minecraft/client/data/models/BlockModelGenerators#registerSimpleFlatItemModel` | method | 1 |
| `net/minecraft/client/data/models/blockstates/MultiVariantGenerator#dispatch` | method | 1 |
| `net/minecraft/client/data/models/blockstates/MultiVariantGenerator#with` | method | 1 |
| `net/minecraft/client/data/models/model/ModelLocationUtils#getModelLocation` | method | 1 |
| `net/minecraft/client/data/models/model/ModelTemplate#create` | method | 2 |
| `net/minecraft/client/data/models/model/ModelTemplates#ATTACHED_HANGING_SIGN_ROT_0` | field | 1 |
| `net/minecraft/client/data/models/model/ModelTemplates#ATTACHED_HANGING_SIGN_ROT_1` | field | 1 |
| `net/minecraft/client/data/models/model/ModelTemplates#ATTACHED_HANGING_SIGN_ROT_2` | field | 1 |
| `net/minecraft/client/data/models/model/ModelTemplates#ATTACHED_HANGING_SIGN_ROT_3` | field | 1 |
| `net/minecraft/client/data/models/model/ModelTemplates#HANGING_SIGN_ROT_0` | field | 1 |
| `net/minecraft/client/data/models/model/ModelTemplates#HANGING_SIGN_ROT_1` | field | 1 |
| `net/minecraft/client/data/models/model/ModelTemplates#HANGING_SIGN_ROT_2` | field | 1 |
| `net/minecraft/client/data/models/model/ModelTemplates#HANGING_SIGN_ROT_3` | field | 1 |
| `net/minecraft/client/data/models/model/ModelTemplates#SIGN_ROT_0` | field | 1 |
| `net/minecraft/client/data/models/model/ModelTemplates#SIGN_ROT_1` | field | 1 |
| `net/minecraft/client/data/models/model/ModelTemplates#SIGN_ROT_2` | field | 1 |
| `net/minecraft/client/data/models/model/ModelTemplates#SIGN_ROT_3` | field | 1 |
| `net/minecraft/client/data/models/model/ModelTemplates#WALL_HANGING_SIGN` | field | 1 |
| `net/minecraft/client/data/models/model/ModelTemplates#WALL_SIGN` | field | 1 |
| `net/minecraft/client/data/models/model/TextureMapping#<init>` | method | 1 |
| `net/minecraft/client/data/models/model/TextureMapping#getBlockTexture` | method | 1 |
| `net/minecraft/client/data/models/model/TextureMapping#put` | method | 1 |
| `net/minecraft/client/data/models/model/TextureSlot#ALL` | field | 1 |
| `net/minecraft/client/data/models/model/TextureSlot#PARTICLE` | field | 1 |
| `net/minecraft/client/gui/Gui#hud` | field | 2 |
| `net/minecraft/client/gui/Gui#setScreen` | method | 2 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#blit` | method | 3 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#centeredText` | method | 1 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#fakeItem` | method | 1 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#guiHeight` | method | 1 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#guiRenderState` | method | 1 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#guiWidth` | method | 1 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#item` | method | 2 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#scissorStack` | method | 1 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#setTooltipForNextFrame` | method | 5 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#textWithWordWrap` | method | 2 |
| `net/minecraft/client/gui/Hud#extractFood` | method | 1 |
| `net/minecraft/client/gui/Hud#extractHearts` | method | 1 |
| `net/minecraft/client/gui/Hud#extractHotbarAndDecorations` | method | 2 |
| `net/minecraft/client/gui/Hud#getGuiTicks` | method | 2 |
| `net/minecraft/client/gui/Hud#isHidden` | method | 1 |
| `net/minecraft/client/gui/components/AbstractWidget#extractRenderState` | method | 1 |
| `net/minecraft/client/gui/components/AbstractWidget#setY` | method | 1 |
| `net/minecraft/client/gui/components/Button#builder` | method | 2 |
| `net/minecraft/client/gui/components/Button$Builder#bounds` | method | 2 |
| `net/minecraft/client/gui/components/Button$Builder#build` | method | 2 |
| `net/minecraft/client/gui/components/ContainerObjectSelectionList#<init>` | method | 1 |
| `net/minecraft/client/gui/components/ContainerObjectSelectionList#scrollBarX` | method | 1 |
| `net/minecraft/client/gui/components/ContainerObjectSelectionList$Entry#<init>` | method | 1 |
| `net/minecraft/client/gui/components/WidgetSprites#<init>` | method | 1 |
| `net/minecraft/client/gui/navigation/ScreenPosition#<init>` | method | 1 |
| `net/minecraft/client/gui/screens/MenuScreens#register` | method | 1 |
| `net/minecraft/client/gui/screens/Screen#<init>` | method | 1 |
| `net/minecraft/client/gui/screens/Screen#extractRenderState` | method | 1 |
| `net/minecraft/client/gui/screens/Screen#mouseClicked` | method | 1 |
| `net/minecraft/client/gui/screens/inventory/AbstractRecipeBookScreen#<init>` | method | 1 |
| `net/minecraft/client/gui/screens/inventory/AbstractRecipeBookScreen#extractBackground` | method | 1 |
| `net/minecraft/client/gui/screens/inventory/AbstractRecipeBookScreen#extractTooltip` | method | 1 |
| `net/minecraft/client/gui/screens/inventory/AbstractRecipeBookScreen#init` | method | 1 |
| `net/minecraft/client/gui/screens/inventory/AbstractRecipeBookScreen#recipeBookComponent` | method | 1 |
| `net/minecraft/client/gui/screens/inventory/HangingSignEditScreen#<init>` | method | 1 |
| `net/minecraft/client/gui/screens/inventory/SignEditScreen#<init>` | method | 1 |
| `net/minecraft/client/gui/screens/recipebook/GhostSlots#setInput` | method | 1 |
| `net/minecraft/client/gui/screens/recipebook/GhostSlots#setResult` | method | 1 |
| `net/minecraft/client/gui/screens/recipebook/RecipeBookComponent#<init>` | method | 1 |
| `net/minecraft/client/gui/screens/recipebook/RecipeBookComponent#isVisible` | method | 1 |
| `net/minecraft/client/gui/screens/recipebook/RecipeBookComponent#toggleVisibility` | method | 1 |
| `net/minecraft/client/gui/screens/recipebook/RecipeBookComponent$TabInfo#<init>` | method | 3 |
| `net/minecraft/client/gui/screens/recipebook/RecipeCollection#selectRecipes` | method | 1 |
| `net/minecraft/client/gui/screens/recipebook/SearchRecipeBookCategory#FARMERSDELIGHT_COOKING` | field | 1 |
| `net/minecraft/client/input/MouseButtonEvent#x` | method | 1 |
| `net/minecraft/client/input/MouseButtonEvent#y` | method | 1 |
| `net/minecraft/client/model/HumanoidModel#leftArm` | field | 1 |
| `net/minecraft/client/model/HumanoidModel#rightArm` | field | 1 |
| `net/minecraft/client/model/HumanoidModel#setupAnim` | method | 1 |
| `net/minecraft/client/model/geom/ModelPart#xRot` | field | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#getGameTime` | method | 2 |
| `net/minecraft/client/multiplayer/ClientLevel#recipeAccess` | method | 1 |
| `net/minecraft/client/multiplayer/MultiPlayerGameMode#canHurtPlayer` | method | 1 |
| `net/minecraft/client/particle/SingleQuadParticle#<init>` | method | 3 |
| `net/minecraft/client/particle/SingleQuadParticle$Layer#OPAQUE` | field | 2 |
| `net/minecraft/client/particle/SingleQuadParticle$Layer#TRANSLUCENT` | field | 1 |
| `net/minecraft/client/particle/SpriteSet#first` | method | 1 |
| `net/minecraft/client/particle/SpriteSet#get` | method | 2 |
| `net/minecraft/client/player/LocalPlayer#getRecipeBook` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#getUseItem` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#isSpectator` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#isUsingItem` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#level` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#tickCount` | field | 1 |
| `net/minecraft/client/renderer/LevelRenderer#submitBlockDestroyAnimation` | method | 1 |
| `net/minecraft/client/renderer/RenderPipelines#GUI_TEXTURED` | field | 4 |
| `net/minecraft/client/renderer/blockentity/BlockEntityRenderer#extractRenderState` | method | 3 |
| `net/minecraft/client/renderer/blockentity/BlockEntityRendererProvider$Context#itemModelResolver` | method | 3 |
| `net/minecraft/client/renderer/blockentity/BlockEntityRenderers#register` | method | 1 |
| `net/minecraft/client/renderer/blockentity/state/BlockEntityRenderState#<init>` | method | 3 |
| `net/minecraft/client/renderer/entity/EntityRenderers#register` | method | 1 |
| `net/minecraft/client/renderer/entity/HumanoidMobRenderer#extractHumanoidRenderState` | method | 1 |
| `net/minecraft/client/renderer/entity/ThrownItemRenderer#<init>` | method | 1 |
| `net/minecraft/client/renderer/entity/state/HumanoidRenderState#mainArm` | field | 1 |
| `net/minecraft/client/renderer/item/CuboidItemModelWrapper#properties` | method | 2 |
| `net/minecraft/client/renderer/item/CuboidItemModelWrapper#update` | method | 1 |
| `net/minecraft/client/renderer/item/CuboidItemModelWrapper$Unbaked#MAP_CODEC` | field | 1 |
| `net/minecraft/client/renderer/item/CuboidItemModelWrapper$Unbaked#bake` | method | 1 |
| `net/minecraft/client/renderer/item/CuboidItemModelWrapper$Unbaked#resolveDependencies` | method | 1 |
| `net/minecraft/client/renderer/item/ItemModelResolver#updateForTopItem` | method | 4 |
| `net/minecraft/client/renderer/item/ItemModels#ID_MAPPER` | field | 1 |
| `net/minecraft/client/renderer/item/ItemStackRenderState#<init>` | method | 4 |
| `net/minecraft/client/renderer/item/ItemStackRenderState#appendModelIdentityElement` | method | 1 |
| `net/minecraft/client/renderer/item/ItemStackRenderState#isEmpty` | method | 2 |
| `net/minecraft/client/renderer/item/ItemStackRenderState#newLayer` | method | 1 |
| `net/minecraft/client/renderer/item/ItemStackRenderState#usesBlockLight` | method | 1 |
| `net/minecraft/client/renderer/item/ItemStackRenderState$LayerRenderState#setupSpecialModel` | method | 1 |
| `net/minecraft/client/renderer/item/ModelRenderProperties#<init>` | method | 1 |
| `net/minecraft/client/renderer/item/ModelRenderProperties#applyToLayer` | method | 1 |
| `net/minecraft/client/renderer/item/ModelRenderProperties#particleMaterial` | method | 1 |
| `net/minecraft/client/renderer/item/ModelRenderProperties#usesBlockLight` | method | 1 |
| `net/minecraft/client/renderer/item/properties/conditional/ConditionalItemModelProperties#ID_MAPPER` | field | 1 |
| `net/minecraft/client/renderer/state/level/BlockBreakingRenderState#blockState` | method | 1 |
| `net/minecraft/client/renderer/texture/OverlayTexture#NO_OVERLAY` | field | 3 |
| `net/minecraft/client/resources/model/cuboid/ItemTransforms#NO_TRANSFORMS` | field | 1 |
| `net/minecraft/core/BlockPos#<init>` | method | 1 |
| `net/minecraft/core/BlockPos#STREAM_CODEC` | field | 2 |
| `net/minecraft/core/BlockPos#above` | method | 11 |
| `net/minecraft/core/BlockPos#asLong` | method | 3 |
| `net/minecraft/core/BlockPos#below` | method | 18 |
| `net/minecraft/core/BlockPos#betweenClosed` | method | 4 |
| `net/minecraft/core/BlockPos#east` | method | 4 |
| `net/minecraft/core/BlockPos#getX` | method | 19 |
| `net/minecraft/core/BlockPos#getY` | method | 20 |
| `net/minecraft/core/BlockPos#getZ` | method | 19 |
| `net/minecraft/core/BlockPos#mutable` | method | 3 |
| `net/minecraft/core/BlockPos#north` | method | 4 |
| `net/minecraft/core/BlockPos#offset` | method | 8 |
| `net/minecraft/core/BlockPos#relative` | method | 5 |
| `net/minecraft/core/BlockPos#south` | method | 4 |
| `net/minecraft/core/BlockPos#west` | method | 4 |
| `net/minecraft/core/BlockPos$MutableBlockPos#getY` | method | 2 |
| `net/minecraft/core/BlockPos$MutableBlockPos#move` | method | 3 |
| `net/minecraft/core/DefaultedRegistry#addAlias` | method | 1 |
| `net/minecraft/core/DefaultedRegistry#filterFeatures` | method | 1 |
| `net/minecraft/core/DefaultedRegistry#get` | method | 1 |
| `net/minecraft/core/DefaultedRegistry#getKey` | method | 3 |
| `net/minecraft/core/DefaultedRegistry#getValue` | method | 6 |
| `net/minecraft/core/DefaultedRegistry#stream` | method | 1 |
| `net/minecraft/core/Direction#DOWN` | field | 11 |
| `net/minecraft/core/Direction#EAST` | field | 4 |
| `net/minecraft/core/Direction#NORTH` | field | 11 |
| `net/minecraft/core/Direction#SOUTH` | field | 4 |
| `net/minecraft/core/Direction#UP` | field | 11 |
| `net/minecraft/core/Direction#WEST` | field | 4 |
| `net/minecraft/core/Direction#equals` | method | 5 |
| `net/minecraft/core/Direction#get2DDataValue` | method | 4 |
| `net/minecraft/core/Direction#get3DDataValue` | method | 1 |
| `net/minecraft/core/Direction#getAxis` | method | 9 |
| `net/minecraft/core/Direction#getClockWise` | method | 4 |
| `net/minecraft/core/Direction#getCounterClockWise` | method | 2 |
| `net/minecraft/core/Direction#getStepX` | method | 7 |
| `net/minecraft/core/Direction#getStepZ` | method | 7 |
| `net/minecraft/core/Direction#getUnitVec3i` | method | 1 |
| `net/minecraft/core/Direction#toYRot` | method | 3 |
| `net/minecraft/core/Direction$Axis#X` | field | 3 |
| `net/minecraft/core/Direction$Axis#Y` | field | 4 |
| `net/minecraft/core/Direction$Axis#Z` | field | 2 |
| `net/minecraft/core/Direction$Axis#equals` | method | 3 |
| `net/minecraft/core/Direction$Axis#getPlane` | method | 1 |
| `net/minecraft/core/Direction$Axis#isHorizontal` | method | 1 |
| `net/minecraft/core/Direction$Plane#HORIZONTAL` | field | 1 |
| `net/minecraft/core/Holder#direct` | method | 1 |
| `net/minecraft/core/Holder#is` | method | 9 |
| `net/minecraft/core/Holder#isBound` | method | 1 |
| `net/minecraft/core/Holder#value` | method | 18 |
| `net/minecraft/core/Holder$Reference#value` | method | 3 |
| `net/minecraft/core/HolderGetter#getOrThrow` | method | 7 |
| `net/minecraft/core/HolderLookup#getOrThrow` | method | 2 |
| `net/minecraft/core/HolderLookup$Provider#getOrThrow` | method | 2 |
| `net/minecraft/core/HolderLookup$Provider#lookupOrThrow` | method | 11 |
| `net/minecraft/core/HolderLookup$RegistryLookup#get` | method | 1 |
| `net/minecraft/core/HolderLookup$RegistryLookup#getOrThrow` | method | 2 |
| `net/minecraft/core/HolderSet#iterator` | method | 1 |
| `net/minecraft/core/HolderSet$Named#forEach` | method | 1 |
| `net/minecraft/core/NonNullList#add` | method | 3 |
| `net/minecraft/core/NonNullList#clear` | method | 1 |
| `net/minecraft/core/NonNullList#create` | method | 2 |
| `net/minecraft/core/NonNullList#createWithCapacity` | method | 1 |
| `net/minecraft/core/NonNullList#get` | method | 2 |
| `net/minecraft/core/NonNullList#iterator` | method | 1 |
| `net/minecraft/core/NonNullList#of` | method | 1 |
| `net/minecraft/core/NonNullList#set` | method | 4 |
| `net/minecraft/core/NonNullList#size` | method | 4 |
| `net/minecraft/core/NonNullList#subList` | method | 1 |
| `net/minecraft/core/NonNullList#withSize` | method | 5 |
| `net/minecraft/core/Position#x` | method | 1 |
| `net/minecraft/core/Position#y` | method | 1 |
| `net/minecraft/core/Position#z` | method | 1 |
| `net/minecraft/core/Registry#addAlias` | method | 1 |
| `net/minecraft/core/Registry#get` | method | 1 |
| `net/minecraft/core/Registry#getOptional` | method | 1 |
| `net/minecraft/core/Registry#getOrThrow` | method | 2 |
| `net/minecraft/core/Registry#getValue` | method | 1 |
| `net/minecraft/core/Registry#iterator` | method | 1 |
| `net/minecraft/core/Registry#wrapAsHolder` | method | 2 |
| `net/minecraft/core/RegistryAccess#get` | method | 1 |
| `net/minecraft/core/RegistryAccess#lookupOrThrow` | method | 1 |
| `net/minecraft/core/RegistryAccess$Frozen#lookupOrThrow` | method | 1 |
| `net/minecraft/core/RegistrySetBuilder#add` | method | 1 |
| `net/minecraft/core/Vec3i#getX` | method | 1 |
| `net/minecraft/core/Vec3i#getY` | method | 1 |
| `net/minecraft/core/Vec3i#getZ` | method | 1 |
| `net/minecraft/core/component/DataComponentGetter#get` | method | 1 |
| `net/minecraft/core/component/DataComponentGetter#getOrDefault` | method | 2 |
| `net/minecraft/core/component/DataComponentGetter#getTyped` | method | 1 |
| `net/minecraft/core/component/DataComponentMap$Builder#getOrCreate` | method | 1 |
| `net/minecraft/core/component/DataComponentMap$Builder#set` | method | 3 |
| `net/minecraft/core/component/DataComponentPatch#builder` | method | 1 |
| `net/minecraft/core/component/DataComponentPatch$Builder#build` | method | 1 |
| `net/minecraft/core/component/DataComponentPatch$Builder#set` | method | 1 |
| `net/minecraft/core/component/DataComponentType#builder` | method | 1 |
| `net/minecraft/core/component/DataComponentType$Builder#build` | method | 1 |
| `net/minecraft/core/component/DataComponentType$Builder#cacheEncoding` | method | 1 |
| `net/minecraft/core/component/DataComponentType$Builder#networkSynchronized` | method | 1 |
| `net/minecraft/core/component/DataComponentType$Builder#persistent` | method | 1 |
| `net/minecraft/core/component/DataComponents#COMPOSTABLE` | field | 1 |
| `net/minecraft/core/component/DataComponents#CONSUMABLE` | field | 5 |
| `net/minecraft/core/component/DataComponents#COOKING_FUEL` | field | 2 |
| `net/minecraft/core/component/DataComponents#CUSTOM_NAME` | field | 2 |
| `net/minecraft/core/component/DataComponents#ENCHANTMENTS` | field | 1 |
| `net/minecraft/core/component/DataComponents#FOOD` | field | 2 |
| `net/minecraft/core/component/DataComponents#ITEM_NAME` | field | 1 |
| `net/minecraft/core/component/DataComponents#MAX_STACK_SIZE` | field | 1 |
| `net/minecraft/core/component/DataComponents#TOOL` | field | 3 |
| `net/minecraft/core/component/DataComponents#VILLAGER_FOOD` | field | 1 |
| `net/minecraft/core/component/DataComponents#WEAPON` | field | 1 |
| `net/minecraft/core/component/predicates/DataComponentPredicates#ENCHANTMENTS` | field | 1 |
| `net/minecraft/core/component/predicates/EnchantmentsPredicate#enchantments` | method | 1 |
| `net/minecraft/core/dispenser/BlockSource#level` | method | 1 |
| `net/minecraft/core/dispenser/BlockSource#pos` | method | 1 |
| `net/minecraft/core/dispenser/BlockSource#state` | method | 1 |
| `net/minecraft/core/dispenser/DispenseItemBehavior#dispense` | method | 1 |
| `net/minecraft/core/dispenser/OptionalDispenseItemBehavior#<init>` | method | 1 |
| `net/minecraft/core/particles/BlockParticleOption#<init>` | method | 3 |
| `net/minecraft/core/particles/ItemParticleOption#<init>` | method | 3 |
| `net/minecraft/core/particles/ParticleTypes#BLOCK` | field | 3 |
| `net/minecraft/core/particles/ParticleTypes#BUBBLE_POP` | field | 1 |
| `net/minecraft/core/particles/ParticleTypes#ENCHANTED_HIT` | field | 1 |
| `net/minecraft/core/particles/ParticleTypes#FLAME` | field | 1 |
| `net/minecraft/core/particles/ParticleTypes#ITEM` | field | 3 |
| `net/minecraft/core/particles/ParticleTypes#MYCELIUM` | field | 1 |
| `net/minecraft/core/particles/ParticleTypes#SMOKE` | field | 2 |
| `net/minecraft/core/registries/BuiltInRegistries#BLOCK_ENTITY_TYPE` | field | 2 |
| `net/minecraft/core/registries/BuiltInRegistries#CONSUME_EFFECT_TYPE` | field | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#CREATIVE_MODE_TAB` | field | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#DATA_COMPONENT_TYPE` | field | 2 |
| `net/minecraft/core/registries/BuiltInRegistries#ENCHANTMENT_EFFECT_COMPONENT_TYPE` | field | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#ENTITY_TYPE` | field | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#FEATURE_TYPE` | field | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#LOOT_CONDITION_TYPE` | field | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#LOOT_FUNCTION_TYPE` | field | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#MENU` | field | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#MOB_EFFECT` | field | 2 |
| `net/minecraft/core/registries/BuiltInRegistries#PARTICLE_TYPE` | field | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#PLACEMENT_MODIFIER_TYPE` | field | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#RECIPE_BOOK_CATEGORY` | field | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#RECIPE_DISPLAY` | field | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#RECIPE_SERIALIZER` | field | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#RECIPE_TYPE` | field | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#SOUND_EVENT` | field | 2 |
| `net/minecraft/core/registries/BuiltInRegistries#TRIGGER_TYPES` | field | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#acquireBootstrapRegistrationLookup` | method | 1 |
| `net/minecraft/core/registries/Registries#BIOME` | field | 3 |
| `net/minecraft/core/registries/Registries#BLOCK` | field | 9 |
| `net/minecraft/core/registries/Registries#DAMAGE_TYPE` | field | 4 |
| `net/minecraft/core/registries/Registries#ENCHANTMENT` | field | 8 |
| `net/minecraft/core/registries/Registries#ENTITY_TYPE` | field | 5 |
| `net/minecraft/core/registries/Registries#FEATURE` | field | 2 |
| `net/minecraft/core/registries/Registries#ITEM` | field | 14 |
| `net/minecraft/core/registries/Registries#LOOT_TABLE` | field | 3 |
| `net/minecraft/core/registries/Registries#MOB_EFFECT` | field | 4 |
| `net/minecraft/core/registries/Registries#PLACED_FEATURE` | field | 4 |
| `net/minecraft/core/registries/Registries#PROCESSOR_LIST` | field | 1 |
| `net/minecraft/core/registries/Registries#RECIPE` | field | 7 |
| `net/minecraft/core/registries/Registries#STRUCTURE` | field | 1 |
| `net/minecraft/core/registries/Registries#TEMPLATE_POOL` | field | 1 |
| `net/minecraft/core/registries/Registries#VILLAGER_TRADE` | field | 2 |
| `net/minecraft/data/CachedOutput#writeIfNeeded` | method | 1 |
| `net/minecraft/data/PackOutput$PathProvider#file` | method | 1 |
| `net/minecraft/data/loot/BlockLootSubProvider#createNameableBlockEntityTable` | method | 1 |
| `net/minecraft/data/recipes/RecipeBuilder#getDefaultRecipeId` | method | 1 |
| `net/minecraft/data/recipes/RecipeCategory#BUILDING_BLOCKS` | field | 1 |
| `net/minecraft/data/recipes/RecipeCategory#COMBAT` | field | 1 |
| `net/minecraft/data/recipes/RecipeCategory#DECORATIONS` | field | 1 |
| `net/minecraft/data/recipes/RecipeCategory#FOOD` | field | 2 |
| `net/minecraft/data/recipes/RecipeCategory#MISC` | field | 2 |
| `net/minecraft/data/recipes/RecipeCategory#TOOLS` | field | 1 |
| `net/minecraft/data/recipes/RecipeOutput#accept` | method | 2 |
| `net/minecraft/data/recipes/RecipeOutput#advancement` | method | 1 |
| `net/minecraft/data/recipes/RecipeProvider#<init>` | method | 1 |
| `net/minecraft/data/recipes/ShapedRecipeBuilder#define` | method | 2 |
| `net/minecraft/data/recipes/ShapedRecipeBuilder#group` | method | 1 |
| `net/minecraft/data/recipes/ShapedRecipeBuilder#pattern` | method | 1 |
| `net/minecraft/data/recipes/ShapedRecipeBuilder#save` | method | 2 |
| `net/minecraft/data/recipes/ShapedRecipeBuilder#shaped` | method | 2 |
| `net/minecraft/data/recipes/ShapedRecipeBuilder#unlockedBy` | method | 1 |
| `net/minecraft/data/recipes/ShapelessRecipeBuilder#group` | method | 1 |
| `net/minecraft/data/recipes/ShapelessRecipeBuilder#requires` | method | 3 |
| `net/minecraft/data/recipes/ShapelessRecipeBuilder#save` | method | 3 |
| `net/minecraft/data/recipes/ShapelessRecipeBuilder#shapeless` | method | 2 |
| `net/minecraft/data/recipes/ShapelessRecipeBuilder#unlockedBy` | method | 1 |
| `net/minecraft/data/recipes/SimpleCookingRecipeBuilder#blasting` | method | 1 |
| `net/minecraft/data/recipes/SimpleCookingRecipeBuilder#campfireCooking` | method | 1 |
| `net/minecraft/data/recipes/SimpleCookingRecipeBuilder#save` | method | 3 |
| `net/minecraft/data/recipes/SimpleCookingRecipeBuilder#smelting` | method | 1 |
| `net/minecraft/data/recipes/SimpleCookingRecipeBuilder#smoking` | method | 1 |
| `net/minecraft/data/recipes/SimpleCookingRecipeBuilder#unlockedBy` | method | 1 |
| `net/minecraft/data/recipes/SmithingTransformRecipeBuilder#save` | method | 1 |
| `net/minecraft/data/recipes/SmithingTransformRecipeBuilder#smithing` | method | 1 |
| `net/minecraft/data/recipes/SmithingTransformRecipeBuilder#unlocks` | method | 1 |
| `net/minecraft/data/recipes/SpecialRecipeBuilder#save` | method | 2 |
| `net/minecraft/data/recipes/SpecialRecipeBuilder#special` | method | 1 |
| `net/minecraft/data/tags/BlockItemTagAppender#add` | method | 4 |
| `net/minecraft/data/tags/BlockItemTagAppender#addAll` | method | 2 |
| `net/minecraft/data/tags/BlockItemTagAppender#addOptionalTag` | method | 2 |
| `net/minecraft/data/tags/BlockItemTagAppender#addTag` | method | 1 |
| `net/minecraft/data/tags/BlockItemTagAppender#forceAddTag` | method | 2 |
| `net/minecraft/data/tags/TagAppender#add` | method | 4 |
| `net/minecraft/data/tags/TagAppender#addOptional` | method | 1 |
| `net/minecraft/data/worldgen/BootstrapContext#lookup` | method | 2 |
| `net/minecraft/data/worldgen/BootstrapContext#register` | method | 3 |
| `net/minecraft/nbt/CompoundTag#<init>` | method | 1 |
| `net/minecraft/nbt/CompoundTag#equals` | method | 1 |
| `net/minecraft/nbt/CompoundTag#getIntOr` | method | 1 |
| `net/minecraft/nbt/NbtAccounter#unlimitedHeap` | method | 1 |
| `net/minecraft/nbt/NbtIo#readCompressed` | method | 1 |
| `net/minecraft/nbt/NbtIo#writeCompressed` | method | 1 |
| `net/minecraft/network/FriendlyByteBuf#readBoolean` | method | 2 |
| `net/minecraft/network/RegistryFriendlyByteBuf#readFloat` | method | 1 |
| `net/minecraft/network/RegistryFriendlyByteBuf#readUtf` | method | 2 |
| `net/minecraft/network/RegistryFriendlyByteBuf#readVarInt` | method | 1 |
| `net/minecraft/network/RegistryFriendlyByteBuf#writeBoolean` | method | 2 |
| `net/minecraft/network/RegistryFriendlyByteBuf#writeFloat` | method | 1 |
| `net/minecraft/network/RegistryFriendlyByteBuf#writeUtf` | method | 2 |
| `net/minecraft/network/RegistryFriendlyByteBuf#writeVarInt` | method | 1 |
| `net/minecraft/network/chat/ClickEvent$OpenUrl#<init>` | method | 1 |
| `net/minecraft/network/chat/CommonComponents#EMPTY` | field | 1 |
| `net/minecraft/network/chat/CommonComponents#GUI_DONE` | field | 1 |
| `net/minecraft/network/chat/Component#empty` | method | 1 |
| `net/minecraft/network/chat/Component#getVisualOrderText` | method | 1 |
| `net/minecraft/network/chat/Component#literal` | method | 9 |
| `net/minecraft/network/chat/Component#nullToEmpty` | method | 1 |
| `net/minecraft/network/chat/Component#translatable` | method | 22 |
| `net/minecraft/network/chat/Component#translatableWithFallback` | method | 1 |
| `net/minecraft/network/chat/ComponentSerialization#CODEC` | field | 1 |
| `net/minecraft/network/chat/MutableComponent#getString` | method | 1 |
| `net/minecraft/network/chat/MutableComponent#setStyle` | method | 1 |
| `net/minecraft/network/chat/Style#EMPTY` | field | 1 |
| `net/minecraft/network/chat/Style#withColor` | method | 1 |
| `net/minecraft/network/chat/TextColor#GRAY` | field | 1 |
| `net/minecraft/network/chat/TextColor#fromLegacyFormat` | method | 1 |
| `net/minecraft/network/chat/TextColor#getValue` | method | 1 |
| `net/minecraft/network/codec/ByteBufCodecs#BOOL` | field | 2 |
| `net/minecraft/network/codec/ByteBufCodecs#FLOAT` | field | 6 |
| `net/minecraft/network/codec/ByteBufCodecs#INT` | field | 4 |
| `net/minecraft/network/codec/ByteBufCodecs#VAR_LONG` | field | 1 |
| `net/minecraft/network/codec/ByteBufCodecs#fromCodec` | method | 1 |
| `net/minecraft/network/codec/ByteBufCodecs#idMapper` | method | 1 |
| `net/minecraft/network/codec/StreamCodec#apply` | method | 6 |
| `net/minecraft/network/codec/StreamCodec#cast` | method | 1 |
| `net/minecraft/network/codec/StreamCodec#composite` | method | 9 |
| `net/minecraft/network/codec/StreamCodec#decode` | method | 2 |
| `net/minecraft/network/codec/StreamCodec#encode` | method | 2 |
| `net/minecraft/network/codec/StreamCodec#map` | method | 2 |
| `net/minecraft/network/codec/StreamCodec#of` | method | 4 |
| `net/minecraft/network/codec/StreamCodec#unit` | method | 5 |
| `net/minecraft/network/protocol/common/custom/CustomPacketPayload$Type#<init>` | method | 4 |
| `net/minecraft/network/protocol/game/ClientboundBlockEntityDataPacket#create` | method | 2 |
| `net/minecraft/recipebook/ServerPlaceRecipe#placeRecipe` | method | 1 |
| `net/minecraft/references/BlockIds#BIG_DRIPLEAF_STEM` | field | 1 |
| `net/minecraft/references/BlockIds#CANDLE_CAKE` | field | 1 |
| `net/minecraft/references/BlockIds#CAVE_VINES_PLANT` | field | 1 |
| `net/minecraft/references/BlockIds#DYED_CANDLE_CAKE` | field | 1 |
| `net/minecraft/references/BlockIds#LAVA` | field | 1 |
| `net/minecraft/references/BlockIds#LAVA_CAULDRON` | field | 1 |
| `net/minecraft/references/BlockIds#TWISTING_VINES_PLANT` | field | 1 |
| `net/minecraft/references/BlockItemId#block` | method | 2 |
| `net/minecraft/references/BlockItemId#create` | method | 1 |
| `net/minecraft/references/BlockItemId#item` | method | 2 |
| `net/minecraft/references/BlockItemIds#BIG_DRIPLEAF` | field | 1 |
| `net/minecraft/references/BlockItemIds#BROWN_MUSHROOM` | field | 1 |
| `net/minecraft/references/BlockItemIds#CACTUS` | field | 1 |
| `net/minecraft/references/BlockItemIds#CAKE` | field | 2 |
| `net/minecraft/references/BlockItemIds#CARVED_PUMPKIN` | field | 1 |
| `net/minecraft/references/BlockItemIds#COBWEB` | field | 1 |
| `net/minecraft/references/BlockItemIds#CRIMSON_NYLIUM` | field | 1 |
| `net/minecraft/references/BlockItemIds#FERN` | field | 1 |
| `net/minecraft/references/BlockItemIds#GLOW_BERRY_CROP` | field | 1 |
| `net/minecraft/references/BlockItemIds#GRASS_BLOCK` | field | 1 |
| `net/minecraft/references/BlockItemIds#HOPPER` | field | 1 |
| `net/minecraft/references/BlockItemIds#JACK_O_LANTERN` | field | 1 |
| `net/minecraft/references/BlockItemIds#MAGMA_BLOCK` | field | 1 |
| `net/minecraft/references/BlockItemIds#MELON` | field | 1 |
| `net/minecraft/references/BlockItemIds#MOSS_BLOCK` | field | 1 |
| `net/minecraft/references/BlockItemIds#MYCELIUM` | field | 1 |
| `net/minecraft/references/BlockItemIds#PINK_PETALS` | field | 1 |
| `net/minecraft/references/BlockItemIds#PODZOL` | field | 1 |
| `net/minecraft/references/BlockItemIds#PUMPKIN` | field | 1 |
| `net/minecraft/references/BlockItemIds#RED_MUSHROOM` | field | 1 |
| `net/minecraft/references/BlockItemIds#SHELF_MUSHROOM` | field | 1 |
| `net/minecraft/references/BlockItemIds#SHORT_GRASS` | field | 1 |
| `net/minecraft/references/BlockItemIds#TWISTING_VINES` | field | 1 |
| `net/minecraft/references/BlockItemIds#WARPED_NYLIUM` | field | 1 |
| `net/minecraft/references/BlockItemIds#WILDFLOWERS` | field | 1 |
| `net/minecraft/references/ItemIds#BEEF` | field | 1 |
| `net/minecraft/references/ItemIds#BEETROOT_SOUP` | field | 1 |
| `net/minecraft/references/ItemIds#BOWL` | field | 1 |
| `net/minecraft/references/ItemIds#BUCKET` | field | 1 |
| `net/minecraft/references/ItemIds#CHICKEN` | field | 1 |
| `net/minecraft/references/ItemIds#COD` | field | 1 |
| `net/minecraft/references/ItemIds#COOKED_BEEF` | field | 1 |
| `net/minecraft/references/ItemIds#COOKED_CHICKEN` | field | 1 |
| `net/minecraft/references/ItemIds#COOKED_COD` | field | 1 |
| `net/minecraft/references/ItemIds#COOKED_MUTTON` | field | 1 |
| `net/minecraft/references/ItemIds#COOKED_PORKCHOP` | field | 1 |
| `net/minecraft/references/ItemIds#COOKED_SALMON` | field | 1 |
| `net/minecraft/references/ItemIds#COOKIE` | field | 1 |
| `net/minecraft/references/ItemIds#FLINT` | field | 1 |
| `net/minecraft/references/ItemIds#GLASS_BOTTLE` | field | 1 |
| `net/minecraft/references/ItemIds#MUSHROOM_STEW` | field | 1 |
| `net/minecraft/references/ItemIds#MUTTON` | field | 1 |
| `net/minecraft/references/ItemIds#PORKCHOP` | field | 1 |
| `net/minecraft/references/ItemIds#PUMPKIN_PIE` | field | 1 |
| `net/minecraft/references/ItemIds#RABBIT_STEW` | field | 1 |
| `net/minecraft/references/ItemIds#SALMON` | field | 1 |
| `net/minecraft/references/ItemIds#SPYGLASS` | field | 1 |
| `net/minecraft/references/ItemIds#TRIDENT` | field | 1 |
| `net/minecraft/references/ItemIds#WHEAT` | field | 1 |
| `net/minecraft/resources/Identifier#CODEC` | field | 3 |
| `net/minecraft/resources/Identifier#STREAM_CODEC` | field | 2 |
| `net/minecraft/resources/Identifier#equals` | method | 2 |
| `net/minecraft/resources/Identifier#fromNamespaceAndPath` | method | 35 |
| `net/minecraft/resources/Identifier#getNamespace` | method | 6 |
| `net/minecraft/resources/Identifier#getPath` | method | 7 |
| `net/minecraft/resources/Identifier#parse` | method | 10 |
| `net/minecraft/resources/Identifier#toDebugFileName` | method | 1 |
| `net/minecraft/resources/Identifier#toString` | method | 3 |
| `net/minecraft/resources/Identifier#tryParse` | method | 2 |
| `net/minecraft/resources/Identifier#withDefaultNamespace` | method | 3 |
| `net/minecraft/resources/Identifier#withPrefix` | method | 2 |
| `net/minecraft/resources/ResourceKey#create` | method | 21 |
| `net/minecraft/resources/ResourceKey#identifier` | method | 7 |
| `net/minecraft/server/MinecraftServer#onGameRuleChanged` | method | 1 |
| `net/minecraft/server/MinecraftServer#registryAccess` | method | 1 |
| `net/minecraft/server/level/ChunkMap#getPlayers` | method | 1 |
| `net/minecraft/server/level/ServerChunkCache#chunkMap` | field | 1 |
| `net/minecraft/server/level/ServerChunkCache#getGenerator` | method | 1 |
| `net/minecraft/server/level/ServerLevel#enabledFeatures` | method | 1 |
| `net/minecraft/server/level/ServerLevel#gameEvent` | method | 1 |
| `net/minecraft/server/level/ServerLevel#getBlockEntity` | method | 1 |
| `net/minecraft/server/level/ServerLevel#getBlockState` | method | 10 |
| `net/minecraft/server/level/ServerLevel#getBrightness` | method | 1 |
| `net/minecraft/server/level/ServerLevel#getChunkAt` | method | 1 |
| `net/minecraft/server/level/ServerLevel#getChunkSource` | method | 2 |
| `net/minecraft/server/level/ServerLevel#getGameRules` | method | 3 |
| `net/minecraft/server/level/ServerLevel#getGameTime` | method | 1 |
| `net/minecraft/server/level/ServerLevel#getRandom` | method | 5 |
| `net/minecraft/server/level/ServerLevel#isEmptyBlock` | method | 2 |
| `net/minecraft/server/level/ServerLevel#levelEvent` | method | 1 |
| `net/minecraft/server/level/ServerLevel#playSound` | method | 1 |
| `net/minecraft/server/level/ServerLevel#recipeAccess` | method | 3 |
| `net/minecraft/server/level/ServerLevel#registryAccess` | method | 1 |
| `net/minecraft/server/level/ServerLevel#sendBlockUpdated` | method | 1 |
| `net/minecraft/server/level/ServerLevel#setBlockAndUpdate` | method | 6 |
| `net/minecraft/server/level/ServerPlayer#getUseItem` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#level` | method | 2 |
| `net/minecraft/server/network/ServerGamePacketListenerImpl#player` | field | 1 |
| `net/minecraft/server/network/ServerGamePacketListenerImpl#send` | method | 1 |
| `net/minecraft/server/packs/PackLocationInfo#<init>` | method | 1 |
| `net/minecraft/server/packs/PackResources#getResource` | method | 1 |
| `net/minecraft/server/packs/PackType#SERVER_DATA` | field | 1 |
| `net/minecraft/server/packs/PathPackResources#<init>` | method | 1 |
| `net/minecraft/server/packs/repository/PackSource#BUILT_IN` | field | 1 |
| `net/minecraft/server/packs/resources/Resource#<init>` | method | 1 |
| `net/minecraft/server/packs/resources/Resource#open` | method | 1 |
| `net/minecraft/server/players/PlayerList#getPlayers` | method | 1 |
| `net/minecraft/sounds/SoundEvent#CODEC` | field | 1 |
| `net/minecraft/sounds/SoundEvent#STREAM_CODEC` | field | 1 |
| `net/minecraft/sounds/SoundEvent#createVariableRangeEvent` | method | 1 |
| `net/minecraft/sounds/SoundEvent#location` | method | 1 |
| `net/minecraft/sounds/SoundEvents#ARMOR_EQUIP_GENERIC` | field | 3 |
| `net/minecraft/sounds/SoundEvents#AXE_STRIP` | field | 1 |
| `net/minecraft/sounds/SoundEvents#BARREL_CLOSE` | field | 1 |
| `net/minecraft/sounds/SoundEvents#BARREL_OPEN` | field | 1 |
| `net/minecraft/sounds/SoundEvents#CAMPFIRE_CRACKLE` | field | 1 |
| `net/minecraft/sounds/SoundEvents#FIRECHARGE_USE` | field | 1 |
| `net/minecraft/sounds/SoundEvents#FLINTANDSTEEL_USE` | field | 1 |
| `net/minecraft/sounds/SoundEvents#GENERIC_EAT` | field | 2 |
| `net/minecraft/sounds/SoundEvents#GENERIC_EXTINGUISH_FIRE` | field | 1 |
| `net/minecraft/sounds/SoundEvents#HORSE_EAT` | field | 1 |
| `net/minecraft/sounds/SoundEvents#LANTERN_PLACE` | field | 3 |
| `net/minecraft/sounds/SoundEvents#LEAD_TIED` | field | 1 |
| `net/minecraft/sounds/SoundEvents#LEAD_UNTIED` | field | 1 |
| `net/minecraft/sounds/SoundEvents#METAL_BREAK` | field | 1 |
| `net/minecraft/sounds/SoundEvents#PLAYER_ATTACK_CRIT` | field | 1 |
| `net/minecraft/sounds/SoundEvents#PUMPKIN_CARVE` | field | 1 |
| `net/minecraft/sounds/SoundEvents#SHEEP_SHEAR` | field | 2 |
| `net/minecraft/sounds/SoundEvents#SLIME_ATTACK` | field | 1 |
| `net/minecraft/sounds/SoundEvents#SNOWBALL_THROW` | field | 1 |
| `net/minecraft/sounds/SoundEvents#SWEET_BERRY_BUSH_PICK_BERRIES` | field | 1 |
| `net/minecraft/sounds/SoundEvents#WOOD_BREAK` | field | 2 |
| `net/minecraft/sounds/SoundEvents#WOOD_HIT` | field | 1 |
| `net/minecraft/sounds/SoundEvents#WOOD_PLACE` | field | 2 |
| `net/minecraft/sounds/SoundEvents#WOOL_BREAK` | field | 3 |
| `net/minecraft/sounds/SoundSource#BLOCKS` | field | 14 |
| `net/minecraft/sounds/SoundSource#NEUTRAL` | field | 1 |
| `net/minecraft/sounds/SoundSource#PLAYERS` | field | 6 |
| `net/minecraft/stats/RecipeBook#<init>` | method | 1 |
| `net/minecraft/stats/RecipeBookSettings#<clinit>` | method | 1 |
| `net/minecraft/stats/RecipeBookSettings#copy` | method | 1 |
| `net/minecraft/stats/RecipeBookSettings#getSettings` | method | 1 |
| `net/minecraft/stats/RecipeBookSettings#isFiltering` | method | 1 |
| `net/minecraft/stats/RecipeBookSettings#isOpen` | method | 1 |
| `net/minecraft/stats/RecipeBookSettings#replaceFrom` | method | 1 |
| `net/minecraft/stats/RecipeBookSettings#updateSettings` | method | 1 |
| `net/minecraft/stats/RecipeBookSettings$TypeSettings#DEFAULT` | field | 1 |
| `net/minecraft/stats/RecipeBookSettings$TypeSettings#codec` | method | 1 |
| `net/minecraft/stats/ServerRecipeBook#sendInitialRecipeBook` | method | 1 |
| `net/minecraft/stats/StatType#get` | method | 4 |
| `net/minecraft/stats/Stats#ITEM_USED` | field | 4 |
| `net/minecraft/tags/BlockItemTagId#item` | method | 1 |
| `net/minecraft/tags/BlockItemTags#SMALL_FLOWERS` | field | 1 |
| `net/minecraft/tags/BlockTags#BLOCKS_MOTION_NO_LEAVES` | field | 1 |
| `net/minecraft/tags/BlockTags#CAMPFIRES` | field | 1 |
| `net/minecraft/tags/BlockTags#CANDLE_CAKES` | field | 1 |
| `net/minecraft/tags/BlockTags#CEILING_HANGING_SIGNS` | field | 1 |
| `net/minecraft/tags/BlockTags#CLIMBABLE` | field | 1 |
| `net/minecraft/tags/BlockTags#COMBINATION_STEP_SOUND_BLOCKS` | field | 1 |
| `net/minecraft/tags/BlockTags#CROPS` | field | 1 |
| `net/minecraft/tags/BlockTags#DIRT` | field | 1 |
| `net/minecraft/tags/BlockTags#FENCES` | field | 1 |
| `net/minecraft/tags/BlockTags#FENCE_GATES` | field | 1 |
| `net/minecraft/tags/BlockTags#FIRE` | field | 1 |
| `net/minecraft/tags/BlockTags#GROWS_CROPS` | field | 2 |
| `net/minecraft/tags/BlockTags#INCORRECT_FOR_WOODEN_TOOL` | field | 1 |
| `net/minecraft/tags/BlockTags#MAINTAINS_FARMLAND` | field | 1 |
| `net/minecraft/tags/BlockTags#MINEABLE_WITH_AXE` | field | 1 |
| `net/minecraft/tags/BlockTags#MINEABLE_WITH_HOE` | field | 1 |
| `net/minecraft/tags/BlockTags#MINEABLE_WITH_PICKAXE` | field | 1 |
| `net/minecraft/tags/BlockTags#MINEABLE_WITH_SHOVEL` | field | 1 |
| `net/minecraft/tags/BlockTags#OVERRIDES_MUSHROOM_LIGHT_REQUIREMENT` | field | 2 |
| `net/minecraft/tags/BlockTags#REPLACEABLE` | field | 1 |
| `net/minecraft/tags/BlockTags#REPLACEABLE_BY_TREES` | field | 1 |
| `net/minecraft/tags/BlockTags#SAND` | field | 2 |
| `net/minecraft/tags/BlockTags#SMALL_FLOWERS` | field | 2 |
| `net/minecraft/tags/BlockTags#STANDING_SIGNS` | field | 1 |
| `net/minecraft/tags/BlockTags#SUBSTRATE_OVERWORLD` | field | 2 |
| `net/minecraft/tags/BlockTags#SUPPORTS_AZALEA` | field | 1 |
| `net/minecraft/tags/BlockTags#SUPPORTS_BAMBOO` | field | 1 |
| `net/minecraft/tags/BlockTags#SUPPORTS_BIG_DRIPLEAF` | field | 1 |
| `net/minecraft/tags/BlockTags#SUPPORTS_CROPS` | field | 4 |
| `net/minecraft/tags/BlockTags#SUPPORTS_SMALL_DRIPLEAF` | field | 1 |
| `net/minecraft/tags/BlockTags#SUPPORTS_SUGAR_CANE` | field | 1 |
| `net/minecraft/tags/BlockTags#SUPPORTS_VEGETATION` | field | 1 |
| `net/minecraft/tags/BlockTags#SUPPORT_OVERRIDE_CACTUS_FLOWER` | field | 1 |
| `net/minecraft/tags/BlockTags#SWORD_INSTANTLY_MINES` | field | 1 |
| `net/minecraft/tags/BlockTags#WALLS` | field | 2 |
| `net/minecraft/tags/BlockTags#WALL_HANGING_SIGNS` | field | 1 |
| `net/minecraft/tags/BlockTags#WALL_SIGNS` | field | 1 |
| `net/minecraft/tags/BlockTags#WASHED_AWAY_BY_FLUIDS` | field | 1 |
| `net/minecraft/tags/BlockTags#WOOL` | field | 1 |
| `net/minecraft/tags/BlockTags#WOOL_CARPETS` | field | 1 |
| `net/minecraft/tags/BlockTags#WOOL_SLABS` | field | 1 |
| `net/minecraft/tags/BlockTags#WOOL_STAIRS` | field | 1 |
| `net/minecraft/tags/DamageTypeTags#BURN_FROM_STEPPING` | field | 1 |
| `net/minecraft/tags/DamageTypeTags#IS_FIRE` | field | 1 |
| `net/minecraft/tags/DamageTypeTags#IS_PROJECTILE` | field | 1 |
| `net/minecraft/tags/DamageTypeTags#NO_KNOCKBACK` | field | 1 |
| `net/minecraft/tags/DamageTypeTags#PANIC_ENVIRONMENTAL_CAUSES` | field | 1 |
| `net/minecraft/tags/EnchantmentTags#NON_TREASURE` | field | 1 |
| `net/minecraft/tags/EntityTypeTags#RAIDERS` | field | 1 |
| `net/minecraft/tags/FluidTags#WATER` | field | 5 |
| `net/minecraft/tags/ItemTags#AXES` | field | 2 |
| `net/minecraft/tags/ItemTags#BREAKS_DECORATED_POTS` | field | 1 |
| `net/minecraft/tags/ItemTags#CAT_FOOD` | field | 1 |
| `net/minecraft/tags/ItemTags#CHICKEN_FOOD` | field | 1 |
| `net/minecraft/tags/ItemTags#DURABILITY_ENCHANTABLE` | field | 1 |
| `net/minecraft/tags/ItemTags#EGGS` | field | 3 |
| `net/minecraft/tags/ItemTags#FIRE_ASPECT_ENCHANTABLE` | field | 1 |
| `net/minecraft/tags/ItemTags#HANGING_SIGNS` | field | 1 |
| `net/minecraft/tags/ItemTags#HOES` | field | 3 |
| `net/minecraft/tags/ItemTags#HORSE_TEMPT_ITEMS` | field | 1 |
| `net/minecraft/tags/ItemTags#LOGS` | field | 1 |
| `net/minecraft/tags/ItemTags#MEAT` | field | 1 |
| `net/minecraft/tags/ItemTags#MELEE_WEAPON_ENCHANTABLE` | field | 1 |
| `net/minecraft/tags/ItemTags#MINING_ENCHANTABLE` | field | 1 |
| `net/minecraft/tags/ItemTags#MINING_LOOT_ENCHANTABLE` | field | 1 |
| `net/minecraft/tags/ItemTags#MUSHROOMS` | field | 1 |
| `net/minecraft/tags/ItemTags#PARROT_FOOD` | field | 1 |
| `net/minecraft/tags/ItemTags#PICKAXES` | field | 2 |
| `net/minecraft/tags/ItemTags#PIGLIN_LOVED` | field | 1 |
| `net/minecraft/tags/ItemTags#PIG_FOOD` | field | 1 |
| `net/minecraft/tags/ItemTags#PLANKS` | field | 1 |
| `net/minecraft/tags/ItemTags#RABBIT_FOOD` | field | 1 |
| `net/minecraft/tags/ItemTags#SHARP_WEAPON_ENCHANTABLE` | field | 1 |
| `net/minecraft/tags/ItemTags#SHOVELS` | field | 1 |
| `net/minecraft/tags/ItemTags#SIGNS` | field | 1 |
| `net/minecraft/tags/ItemTags#SWORDS` | field | 1 |
| `net/minecraft/tags/ItemTags#VILLAGER_PLANTABLE_SEEDS` | field | 1 |
| `net/minecraft/tags/ItemTags#WEAPON_ENCHANTABLE` | field | 1 |
| `net/minecraft/tags/TagBuilder#addOptionalElement` | method | 2 |
| `net/minecraft/tags/TagKey#codec` | method | 2 |
| `net/minecraft/tags/TagKey#create` | method | 5 |
| `net/minecraft/tags/TagKey#streamCodec` | method | 1 |
| `net/minecraft/tags/VillagerTradeTags#FARMER_LEVEL_1` | field | 1 |
| `net/minecraft/tags/VillagerTradeTags#FARMER_LEVEL_2` | field | 1 |
| `net/minecraft/tags/VillagerTradeTags#WANDERING_TRADER_COMMON` | field | 1 |
| `net/minecraft/util/ByIdMap#continuous` | method | 1 |
| `net/minecraft/util/ByIdMap$OutOfBoundsStrategy#ZERO` | field | 1 |
| `net/minecraft/util/ExtraCodecs$LateBoundIdMapper#put` | method | 1 |
| `net/minecraft/util/Mth#sin` | method | 2 |
| `net/minecraft/util/Prediction#PREDICTED` | field | 3 |
| `net/minecraft/util/Prediction#SERVER_ONLY` | field | 2 |
| `net/minecraft/util/ProblemReporter$ScopedCollector#<init>` | method | 2 |
| `net/minecraft/util/ProblemReporter$ScopedCollector#close` | method | 2 |
| `net/minecraft/util/RandomSource#nextBoolean` | method | 3 |
| `net/minecraft/util/RandomSource#nextDouble` | method | 5 |
| `net/minecraft/util/RandomSource#nextFloat` | method | 20 |
| `net/minecraft/util/RandomSource#nextGaussian` | method | 1 |
| `net/minecraft/util/RandomSource#nextInt` | method | 12 |
| `net/minecraft/util/StringRepresentable#fromEnum` | method | 1 |
| `net/minecraft/util/datafix/DataFixTypes#STRUCTURE` | field | 1 |
| `net/minecraft/util/datafix/DataFixTypes#updateToCurrentVersion` | method | 1 |
| `net/minecraft/util/datafix/DataFixers#getDataFixer` | method | 1 |
| `net/minecraft/util/datafix/fixes/ItemStackComponentizationFix#fixItemStack` | method | 1 |
| `net/minecraft/util/datafix/fixes/ItemStackComponentizationFix$ItemStackData#fixSubTag` | method | 1 |
| `net/minecraft/util/datafix/fixes/ItemStackComponentizationFix$ItemStackData#is` | method | 1 |
| `net/minecraft/util/datafix/fixes/ItemStackComponentizationFix$ItemStackData#setComponent` | method | 1 |
| `net/minecraft/util/datafix/fixes/References#ITEM_STACK` | field | 2 |
| `net/minecraft/util/datafix/schemas/V1460#registerBlockEntities` | method | 1 |
| `net/minecraft/util/datafix/schemas/V3818_3#lambda$registerTypes$0` | method | 1 |
| `net/minecraft/world/Container#canPlaceItem` | method | 1 |
| `net/minecraft/world/Container#equals` | method | 1 |
| `net/minecraft/world/Container#getContainerSize` | method | 1 |
| `net/minecraft/world/Container#getItem` | method | 1 |
| `net/minecraft/world/Container#getMaxStackSize` | method | 1 |
| `net/minecraft/world/Container#removeItem` | method | 1 |
| `net/minecraft/world/Container#setChanged` | method | 1 |
| `net/minecraft/world/Container#setItem` | method | 1 |
| `net/minecraft/world/ContainerHelper#loadAllItems` | method | 3 |
| `net/minecraft/world/ContainerHelper#removeItem` | method | 1 |
| `net/minecraft/world/ContainerHelper#saveAllItems` | method | 3 |
| `net/minecraft/world/Containers#dropContents` | method | 1 |
| `net/minecraft/world/Containers#updateNeighboursAfterDestroy` | method | 3 |
| `net/minecraft/world/InteractionHand#MAIN_HAND` | field | 5 |
| `net/minecraft/world/InteractionHand#OFF_HAND` | field | 1 |
| `net/minecraft/world/InteractionHand#asEquipmentSlot` | method | 3 |
| `net/minecraft/world/InteractionHand#equals` | method | 1 |
| `net/minecraft/world/InteractionResult#CONSUME` | field | 10 |
| `net/minecraft/world/InteractionResult#FAIL` | field | 1 |
| `net/minecraft/world/InteractionResult#PASS` | field | 12 |
| `net/minecraft/world/InteractionResult#SUCCESS` | field | 20 |
| `net/minecraft/world/InteractionResult#SUCCESS_SERVER` | field | 2 |
| `net/minecraft/world/InteractionResult#TRY_WITH_EMPTY_HAND` | field | 4 |
| `net/minecraft/world/InteractionResult#consumesAction` | method | 4 |
| `net/minecraft/world/InteractionResult#equals` | method | 1 |
| `net/minecraft/world/SimpleContainer#<init>` | method | 1 |
| `net/minecraft/world/TickRateManager#tickrate` | method | 1 |
| `net/minecraft/world/damagesource/DamageEffects#BURNING` | field | 1 |
| `net/minecraft/world/damagesource/DamageScaling#WHEN_CAUSED_BY_LIVING_NON_PLAYER` | field | 1 |
| `net/minecraft/world/damagesource/DamageSource#<init>` | method | 1 |
| `net/minecraft/world/damagesource/DamageSource#getDirectEntity` | method | 1 |
| `net/minecraft/world/damagesource/DamageSource#getEntity` | method | 1 |
| `net/minecraft/world/damagesource/DamageSource#getSourcePosition` | method | 1 |
| `net/minecraft/world/damagesource/DamageSources#fall` | method | 3 |
| `net/minecraft/world/damagesource/DamageSources#thrown` | method | 1 |
| `net/minecraft/world/damagesource/DamageType#<init>` | method | 1 |
| `net/minecraft/world/effect/MobEffect#<init>` | method | 2 |
| `net/minecraft/world/effect/MobEffect#createModifiers` | method | 1 |
| `net/minecraft/world/effect/MobEffect#getCategory` | method | 5 |
| `net/minecraft/world/effect/MobEffectCategory#BENEFICIAL` | field | 2 |
| `net/minecraft/world/effect/MobEffectCategory#HARMFUL` | field | 1 |
| `net/minecraft/world/effect/MobEffectCategory#equals` | method | 1 |
| `net/minecraft/world/effect/MobEffectCategory#getTooltipFormatting` | method | 4 |
| `net/minecraft/world/effect/MobEffectInstance#<init>` | method | 8 |
| `net/minecraft/world/effect/MobEffectInstance#getAmplifier` | method | 3 |
| `net/minecraft/world/effect/MobEffectInstance#getDescriptionId` | method | 4 |
| `net/minecraft/world/effect/MobEffectInstance#getDuration` | method | 4 |
| `net/minecraft/world/effect/MobEffectInstance#getEffect` | method | 5 |
| `net/minecraft/world/effect/MobEffects#ABSORPTION` | field | 1 |
| `net/minecraft/world/effect/MobEffects#FIRE_RESISTANCE` | field | 1 |
| `net/minecraft/world/effect/MobEffects#GLOWING` | field | 1 |
| `net/minecraft/world/effect/MobEffects#HUNGER` | field | 1 |
| `net/minecraft/world/effect/MobEffects#JUMP_BOOST` | field | 2 |
| `net/minecraft/world/effect/MobEffects#MINING_FATIGUE` | field | 1 |
| `net/minecraft/world/effect/MobEffects#NAUSEA` | field | 2 |
| `net/minecraft/world/effect/MobEffects#POISON` | field | 1 |
| `net/minecraft/world/effect/MobEffects#REGENERATION` | field | 4 |
| `net/minecraft/world/effect/MobEffects#RESISTANCE` | field | 1 |
| `net/minecraft/world/effect/MobEffects#SPEED` | field | 3 |
| `net/minecraft/world/effect/MobEffects#STRENGTH` | field | 2 |
| `net/minecraft/world/effect/MobEffects#WATER_BREATHING` | field | 1 |
| `net/minecraft/world/entity/Entity#<init>` | method | 1 |
| `net/minecraft/world/entity/Entity#blockPosition` | method | 2 |
| `net/minecraft/world/entity/Entity#causeFallDamage` | method | 3 |
| `net/minecraft/world/entity/Entity#damageSources` | method | 1 |
| `net/minecraft/world/entity/Entity#getBoundingBox` | method | 1 |
| `net/minecraft/world/entity/Entity#is` | method | 2 |
| `net/minecraft/world/entity/Entity#isSteppingCarefully` | method | 1 |
| `net/minecraft/world/entity/Entity#isSuppressingBounce` | method | 1 |
| `net/minecraft/world/entity/Entity#level` | method | 1 |
| `net/minecraft/world/entity/EntitySelector#ENTITY_STILL_ALIVE` | field | 1 |
| `net/minecraft/world/entity/EntityType$Builder#build` | method | 1 |
| `net/minecraft/world/entity/EntityType$Builder#clientTrackingRange` | method | 1 |
| `net/minecraft/world/entity/EntityType$Builder#of` | method | 1 |
| `net/minecraft/world/entity/EntityType$Builder#sized` | method | 1 |
| `net/minecraft/world/entity/EntityType$Builder#updateInterval` | method | 1 |
| `net/minecraft/world/entity/EntityTypeIds#COW` | field | 1 |
| `net/minecraft/world/entity/EntityTypeIds#DONKEY` | field | 1 |
| `net/minecraft/world/entity/EntityTypeIds#HORSE` | field | 1 |
| `net/minecraft/world/entity/EntityTypeIds#LLAMA` | field | 1 |
| `net/minecraft/world/entity/EntityTypeIds#MOOSHROOM` | field | 1 |
| `net/minecraft/world/entity/EntityTypeIds#MULE` | field | 1 |
| `net/minecraft/world/entity/EntityTypeIds#SKELETON_HORSE` | field | 1 |
| `net/minecraft/world/entity/EntityTypeIds#TRADER_LLAMA` | field | 1 |
| `net/minecraft/world/entity/EntityTypeIds#WOLF` | field | 1 |
| `net/minecraft/world/entity/EntityTypeIds#ZOMBIE_HORSE` | field | 1 |
| `net/minecraft/world/entity/EquipmentSlot#MAINHAND` | field | 3 |
| `net/minecraft/world/entity/EquipmentSlot#OFFHAND` | field | 1 |
| `net/minecraft/world/entity/EquipmentSlotGroup#MAINHAND` | field | 3 |
| `net/minecraft/world/entity/ExperienceOrb#award` | method | 1 |
| `net/minecraft/world/entity/HumanoidArm#LEFT` | field | 1 |
| `net/minecraft/world/entity/LivingEntity#<init>` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#addEffect` | method | 2 |
| `net/minecraft/world/entity/LivingEntity#getActiveEffects` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#getEffect` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#getHealth` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#getItemInHand` | method | 2 |
| `net/minecraft/world/entity/LivingEntity#getKillCredit` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#getLootTable` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#getMainHandItem` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#getMaxHealth` | method | 3 |
| `net/minecraft/world/entity/LivingEntity#getRandom` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#getRandomX` | method | 2 |
| `net/minecraft/world/entity/LivingEntity#getRandomY` | method | 2 |
| `net/minecraft/world/entity/LivingEntity#getRandomZ` | method | 2 |
| `net/minecraft/world/entity/LivingEntity#getSoundSource` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#getUseItem` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#getUsedItemHand` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#getWeaponItem` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#heal` | method | 2 |
| `net/minecraft/world/entity/LivingEntity#hurtServer` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#isAlive` | method | 2 |
| `net/minecraft/world/entity/LivingEntity#isShiftKeyDown` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#level` | method | 4 |
| `net/minecraft/world/entity/LivingEntity#position` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#removeEffect` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#setHealth` | method | 2 |
| `net/minecraft/world/entity/LivingEntity#typeHolder` | method | 1 |
| `net/minecraft/world/entity/MobCategory#MISC` | field | 1 |
| `net/minecraft/world/entity/TamableAnimal#isTame` | method | 1 |
| `net/minecraft/world/entity/ai/attributes/Attribute#getDescriptionId` | method | 1 |
| `net/minecraft/world/entity/ai/attributes/AttributeModifier#<init>` | method | 2 |
| `net/minecraft/world/entity/ai/attributes/AttributeModifier#amount` | method | 1 |
| `net/minecraft/world/entity/ai/attributes/AttributeModifier#operation` | method | 1 |
| `net/minecraft/world/entity/ai/attributes/AttributeModifier$Operation#ADD_MULTIPLIED_BASE` | field | 1 |
| `net/minecraft/world/entity/ai/attributes/AttributeModifier$Operation#ADD_MULTIPLIED_TOTAL` | field | 1 |
| `net/minecraft/world/entity/ai/attributes/AttributeModifier$Operation#ADD_VALUE` | field | 2 |
| `net/minecraft/world/entity/ai/attributes/AttributeModifier$Operation#id` | method | 1 |
| `net/minecraft/world/entity/ai/attributes/Attributes#ATTACK_DAMAGE` | field | 2 |
| `net/minecraft/world/entity/ai/attributes/Attributes#ATTACK_KNOCKBACK` | field | 1 |
| `net/minecraft/world/entity/ai/attributes/Attributes#ATTACK_SPEED` | field | 2 |
| `net/minecraft/world/entity/ai/attributes/Attributes#MAX_HEALTH` | field | 1 |
| `net/minecraft/world/entity/ai/sensing/SecondaryPoiSensor#doTick` | method | 1 |
| `net/minecraft/world/entity/animal/equine/AbstractHorse#isTamed` | method | 1 |
| `net/minecraft/world/entity/animal/equine/Horse#isAlive` | method | 1 |
| `net/minecraft/world/entity/animal/equine/Horse#isTamed` | method | 1 |
| `net/minecraft/world/entity/animal/wolf/Wolf#isAlive` | method | 1 |
| `net/minecraft/world/entity/animal/wolf/Wolf#isTame` | method | 1 |
| `net/minecraft/world/entity/item/ItemEntity#<init>` | method | 2 |
| `net/minecraft/world/entity/item/ItemEntity#discard` | method | 1 |
| `net/minecraft/world/entity/item/ItemEntity#getItem` | method | 1 |
| `net/minecraft/world/entity/item/ItemEntity#setDeltaMovement` | method | 2 |
| `net/minecraft/world/entity/item/ItemEntity#setItem` | method | 1 |
| `net/minecraft/world/entity/npc/villager/Villager#getVillagerData` | method | 1 |
| `net/minecraft/world/entity/npc/villager/VillagerData#profession` | method | 1 |
| `net/minecraft/world/entity/npc/villager/VillagerProfession#FARMER` | field | 1 |
| `net/minecraft/world/entity/player/Abilities#instabuild` | field | 7 |
| `net/minecraft/world/entity/player/Abilities#mayBuild` | field | 1 |
| `net/minecraft/world/entity/player/Inventory#placeItemBackInInventory` | method | 1 |
| `net/minecraft/world/entity/player/Inventory#player` | field | 1 |
| `net/minecraft/world/entity/player/Player#addItem` | method | 1 |
| `net/minecraft/world/entity/player/Player#awardRecipes` | method | 1 |
| `net/minecraft/world/entity/player/Player#awardStat` | method | 4 |
| `net/minecraft/world/entity/player/Player#blockPosition` | method | 1 |
| `net/minecraft/world/entity/player/Player#canEat` | method | 2 |
| `net/minecraft/world/entity/player/Player#containerMenu` | field | 1 |
| `net/minecraft/world/entity/player/Player#drop` | method | 4 |
| `net/minecraft/world/entity/player/Player#getAbilities` | method | 7 |
| `net/minecraft/world/entity/player/Player#getAbsorptionAmount` | method | 1 |
| `net/minecraft/world/entity/player/Player#getAttribute` | method | 1 |
| `net/minecraft/world/entity/player/Player#getDirection` | method | 2 |
| `net/minecraft/world/entity/player/Player#getEffect` | method | 2 |
| `net/minecraft/world/entity/player/Player#getFoodData` | method | 4 |
| `net/minecraft/world/entity/player/Player#getHealth` | method | 1 |
| `net/minecraft/world/entity/player/Player#getInventory` | method | 6 |
| `net/minecraft/world/entity/player/Player#getItemInHand` | method | 9 |
| `net/minecraft/world/entity/player/Player#getMainHandItem` | method | 2 |
| `net/minecraft/world/entity/player/Player#getUseItemRemainingTicks` | method | 1 |
| `net/minecraft/world/entity/player/Player#getX` | method | 1 |
| `net/minecraft/world/entity/player/Player#getXRot` | method | 1 |
| `net/minecraft/world/entity/player/Player#getY` | method | 1 |
| `net/minecraft/world/entity/player/Player#getYRot` | method | 1 |
| `net/minecraft/world/entity/player/Player#getZ` | method | 1 |
| `net/minecraft/world/entity/player/Player#hurtServer` | method | 1 |
| `net/minecraft/world/entity/player/Player#isCreative` | method | 5 |
| `net/minecraft/world/entity/player/Player#isHurt` | method | 3 |
| `net/minecraft/world/entity/player/Player#isSecondaryUseActive` | method | 3 |
| `net/minecraft/world/entity/player/Player#isShiftKeyDown` | method | 3 |
| `net/minecraft/world/entity/player/Player#isSpectator` | method | 5 |
| `net/minecraft/world/entity/player/Player#isUnderWater` | method | 1 |
| `net/minecraft/world/entity/player/Player#level` | method | 4 |
| `net/minecraft/world/entity/player/Player#openMenu` | method | 3 |
| `net/minecraft/world/entity/player/Player#position` | method | 1 |
| `net/minecraft/world/entity/player/Player#sendOverlayMessage` | method | 5 |
| `net/minecraft/world/entity/player/Player#setItemInHand` | method | 2 |
| `net/minecraft/world/entity/player/Player#setItemSlot` | method | 2 |
| `net/minecraft/world/entity/player/Player#startUsingItem` | method | 1 |
| `net/minecraft/world/entity/player/StackedItemContents#<init>` | method | 1 |
| `net/minecraft/world/entity/player/StackedItemContents#accountSimpleStack` | method | 1 |
| `net/minecraft/world/entity/player/StackedItemContents#canCraft` | method | 1 |
| `net/minecraft/world/entity/projectile/throwableitemprojectile/ThrowableItemProjectile#<init>` | method | 3 |
| `net/minecraft/world/entity/projectile/throwableitemprojectile/ThrowableItemProjectile#onHit` | method | 1 |
| `net/minecraft/world/entity/projectile/throwableitemprojectile/ThrowableItemProjectile#onHitEntity` | method | 1 |
| `net/minecraft/world/flag/FeatureFlags#DEFAULT_FLAGS` | field | 1 |
| `net/minecraft/world/food/FoodData#exhaustionLevel` | method | 1 |
| `net/minecraft/world/food/FoodData#getFoodLevel` | method | 3 |
| `net/minecraft/world/food/FoodData#getSaturationLevel` | method | 3 |
| `net/minecraft/world/food/FoodProperties#<init>` | method | 1 |
| `net/minecraft/world/food/FoodProperties$Builder#<init>` | method | 1 |
| `net/minecraft/world/food/FoodProperties$Builder#alwaysEdible` | method | 1 |
| `net/minecraft/world/food/FoodProperties$Builder#build` | method | 1 |
| `net/minecraft/world/food/FoodProperties$Builder#nutrition` | method | 1 |
| `net/minecraft/world/food/FoodProperties$Builder#saturationModifier` | method | 1 |
| `net/minecraft/world/food/VillagerFood#<init>` | method | 1 |
| `net/minecraft/world/inventory/AbstractContainerMenu#getRedstoneSignalFromBlockEntity` | method | 2 |
| `net/minecraft/world/inventory/ChestMenu#getContainer` | method | 1 |
| `net/minecraft/world/inventory/ChestMenu#threeRows` | method | 2 |
| `net/minecraft/world/inventory/ContainerData#get` | method | 1 |
| `net/minecraft/world/inventory/ContainerLevelAccess#create` | method | 1 |
| `net/minecraft/world/inventory/RecipeBookMenu#<init>` | method | 1 |
| `net/minecraft/world/inventory/RecipeBookType#FARMERSDELIGHT_COOKING` | field | 1 |
| `net/minecraft/world/inventory/SimpleContainerData#<init>` | method | 1 |
| `net/minecraft/world/inventory/Slot#<init>` | method | 2 |
| `net/minecraft/world/inventory/Slot#getItem` | method | 3 |
| `net/minecraft/world/inventory/Slot#hasItem` | method | 2 |
| `net/minecraft/world/inventory/Slot#index` | field | 2 |
| `net/minecraft/world/inventory/Slot#onTake` | method | 1 |
| `net/minecraft/world/inventory/Slot#set` | method | 2 |
| `net/minecraft/world/inventory/Slot#setChanged` | method | 1 |
| `net/minecraft/world/inventory/tooltip/TooltipComponent#getClass` | method | 1 |
| `net/minecraft/world/item/BlockItem#<init>` | method | 10 |
| `net/minecraft/world/item/BlockItem#allowComponentsUpdateAnimation` | method | 1 |
| `net/minecraft/world/item/BlockItem#appendHoverText` | method | 1 |
| `net/minecraft/world/item/BlockItem#canBeEnchantedWith` | method | 1 |
| `net/minecraft/world/item/BlockItem#getBarColor` | method | 1 |
| `net/minecraft/world/item/BlockItem#getBarWidth` | method | 1 |
| `net/minecraft/world/item/BlockItem#getBlock` | method | 1 |
| `net/minecraft/world/item/BlockItem#isBarVisible` | method | 1 |
| `net/minecraft/world/item/BlockItem#registerBlocks` | method | 1 |
| `net/minecraft/world/item/BlockItem#releaseUsing` | method | 1 |
| `net/minecraft/world/item/BoneMealItem#addGrowthParticles` | method | 1 |
| `net/minecraft/world/item/CreativeModeTab$Builder#build` | method | 1 |
| `net/minecraft/world/item/CreativeModeTab$Builder#displayItems` | method | 1 |
| `net/minecraft/world/item/CreativeModeTab$Builder#icon` | method | 1 |
| `net/minecraft/world/item/CreativeModeTab$Builder#title` | method | 1 |
| `net/minecraft/world/item/CreativeModeTab$Output#accept` | method | 1 |
| `net/minecraft/world/item/DoubleHighBlockItem#<init>` | method | 1 |
| `net/minecraft/world/item/DyeColor#BLACK` | field | 1 |
| `net/minecraft/world/item/DyeColor#BLUE` | field | 1 |
| `net/minecraft/world/item/DyeColor#BROWN` | field | 1 |
| `net/minecraft/world/item/DyeColor#CYAN` | field | 1 |
| `net/minecraft/world/item/DyeColor#GRAY` | field | 1 |
| `net/minecraft/world/item/DyeColor#GREEN` | field | 1 |
| `net/minecraft/world/item/DyeColor#LIGHT_BLUE` | field | 1 |
| `net/minecraft/world/item/DyeColor#LIGHT_GRAY` | field | 1 |
| `net/minecraft/world/item/DyeColor#LIME` | field | 1 |
| `net/minecraft/world/item/DyeColor#MAGENTA` | field | 1 |
| `net/minecraft/world/item/DyeColor#ORANGE` | field | 1 |
| `net/minecraft/world/item/DyeColor#PINK` | field | 1 |
| `net/minecraft/world/item/DyeColor#PURPLE` | field | 1 |
| `net/minecraft/world/item/DyeColor#RED` | field | 1 |
| `net/minecraft/world/item/DyeColor#WHITE` | field | 2 |
| `net/minecraft/world/item/DyeColor#YELLOW` | field | 1 |
| `net/minecraft/world/item/DyeColor#getName` | method | 3 |
| `net/minecraft/world/item/HangingSignItem#<init>` | method | 1 |
| `net/minecraft/world/item/Item#builtInRegistryHolder` | method | 2 |
| `net/minecraft/world/item/Item#canBeEnchantedWith` | method | 1 |
| `net/minecraft/world/item/Item#finishUsingItem` | method | 1 |
| `net/minecraft/world/item/Item#getDefaultInstance` | method | 5 |
| `net/minecraft/world/item/Item#getDescriptionId` | method | 1 |
| `net/minecraft/world/item/Item#getId` | method | 2 |
| `net/minecraft/world/item/Item#useOn` | method | 1 |
| `net/minecraft/world/item/Item$Properties#<init>` | method | 1 |
| `net/minecraft/world/item/Item$Properties#attributes` | method | 1 |
| `net/minecraft/world/item/Item$Properties#component` | method | 3 |
| `net/minecraft/world/item/Item$Properties#compostable` | method | 1 |
| `net/minecraft/world/item/Item$Properties#craftRemainder` | method | 1 |
| `net/minecraft/world/item/Item$Properties#enchantable` | method | 1 |
| `net/minecraft/world/item/Item$Properties#fireResistant` | method | 1 |
| `net/minecraft/world/item/Item$Properties#food` | method | 2 |
| `net/minecraft/world/item/Item$Properties#repairable` | method | 1 |
| `net/minecraft/world/item/Item$Properties#setId` | method | 1 |
| `net/minecraft/world/item/Item$Properties#useBlockDescriptionPrefix` | method | 1 |
| `net/minecraft/world/item/Item$TooltipContext#tickRate` | method | 3 |
| `net/minecraft/world/item/ItemDisplayContext#FIXED` | field | 4 |
| `net/minecraft/world/item/ItemDisplayContext#GUI` | field | 1 |
| `net/minecraft/world/item/ItemDisplayContext#NONE` | field | 1 |
| `net/minecraft/world/item/ItemDisplayContext#firstPerson` | method | 1 |
| `net/minecraft/world/item/ItemInstance#hashCode` | method | 1 |
| `net/minecraft/world/item/ItemInstance#is` | method | 1 |
| `net/minecraft/world/item/ItemInstance#typeHolder` | method | 1 |
| `net/minecraft/world/item/ItemStack#<init>` | method | 29 |
| `net/minecraft/world/item/ItemStack#CODEC` | field | 2 |
| `net/minecraft/world/item/ItemStack#EMPTY` | field | 21 |
| `net/minecraft/world/item/ItemStack#OPTIONAL_CODEC` | field | 2 |
| `net/minecraft/world/item/ItemStack#OPTIONAL_STREAM_CODEC` | field | 1 |
| `net/minecraft/world/item/ItemStack#applyComponents` | method | 1 |
| `net/minecraft/world/item/ItemStack#copy` | method | 21 |
| `net/minecraft/world/item/ItemStack#copyWithCount` | method | 1 |
| `net/minecraft/world/item/ItemStack#finishUsingItem` | method | 1 |
| `net/minecraft/world/item/ItemStack#get` | method | 5 |
| `net/minecraft/world/item/ItemStack#getAllOfType` | method | 1 |
| `net/minecraft/world/item/ItemStack#getCraftingRemainder` | method | 7 |
| `net/minecraft/world/item/ItemStack#getDamageValue` | method | 2 |
| `net/minecraft/world/item/ItemStack#getHoverName` | method | 2 |
| `net/minecraft/world/item/ItemStack#getItem` | method | 20 |
| `net/minecraft/world/item/ItemStack#getMaxStackSize` | method | 14 |
| `net/minecraft/world/item/ItemStack#getOrDefault` | method | 5 |
| `net/minecraft/world/item/ItemStack#grow` | method | 4 |
| `net/minecraft/world/item/ItemStack#has` | method | 5 |
| `net/minecraft/world/item/ItemStack#hashItemAndComponents` | method | 1 |
| `net/minecraft/world/item/ItemStack#is` | method | 20 |
| `net/minecraft/world/item/ItemStack#isEmpty` | method | 30 |
| `net/minecraft/world/item/ItemStack#isItemEnabled` | method | 1 |
| `net/minecraft/world/item/ItemStack#isSameItem` | method | 2 |
| `net/minecraft/world/item/ItemStack#onCraftedBy` | method | 1 |
| `net/minecraft/world/item/ItemStack#remove` | method | 1 |
| `net/minecraft/world/item/ItemStack#set` | method | 3 |
| `net/minecraft/world/item/ItemStack#setCount` | method | 9 |
| `net/minecraft/world/item/ItemStack#split` | method | 4 |
| `net/minecraft/world/item/ItemStack#typeHolder` | method | 1 |
| `net/minecraft/world/item/ItemStackTemplate#<init>` | method | 10 |
| `net/minecraft/world/item/ItemStackTemplate#CODEC` | field | 2 |
| `net/minecraft/world/item/ItemStackTemplate#STREAM_CODEC` | field | 2 |
| `net/minecraft/world/item/ItemStackTemplate#components` | method | 1 |
| `net/minecraft/world/item/ItemStackTemplate#count` | method | 2 |
| `net/minecraft/world/item/ItemStackTemplate#equals` | method | 1 |
| `net/minecraft/world/item/ItemStackTemplate#hashCode` | method | 1 |
| `net/minecraft/world/item/ItemStackTemplate#item` | method | 3 |
| `net/minecraft/world/item/Items#ACACIA_BOAT` | field | 1 |
| `net/minecraft/world/item/Items#ACACIA_BUTTON` | field | 1 |
| `net/minecraft/world/item/Items#ACACIA_CHEST_BOAT` | field | 1 |
| `net/minecraft/world/item/Items#ACACIA_DOOR` | field | 1 |
| `net/minecraft/world/item/Items#ACACIA_FENCE` | field | 1 |
| `net/minecraft/world/item/Items#ACACIA_FENCE_GATE` | field | 1 |
| `net/minecraft/world/item/Items#ACACIA_HANGING_SIGN` | field | 1 |
| `net/minecraft/world/item/Items#ACACIA_LOG` | field | 1 |
| `net/minecraft/world/item/Items#ACACIA_PLANKS` | field | 1 |
| `net/minecraft/world/item/Items#ACACIA_PRESSURE_PLATE` | field | 1 |
| `net/minecraft/world/item/Items#ACACIA_SIGN` | field | 1 |
| `net/minecraft/world/item/Items#ACACIA_SLAB` | field | 1 |
| `net/minecraft/world/item/Items#ACACIA_TRAPDOOR` | field | 2 |
| `net/minecraft/world/item/Items#ACACIA_WOOD` | field | 1 |
| `net/minecraft/world/item/Items#ALLIUM` | field | 2 |
| `net/minecraft/world/item/Items#AMETHYST_BLOCK` | field | 1 |
| `net/minecraft/world/item/Items#AMETHYST_SHARD` | field | 1 |
| `net/minecraft/world/item/Items#APPLE` | field | 2 |
| `net/minecraft/world/item/Items#AXOLOTL_BUCKET` | field | 1 |
| `net/minecraft/world/item/Items#AZURE_BLUET` | field | 1 |
| `net/minecraft/world/item/Items#BAKED_POTATO` | field | 1 |
| `net/minecraft/world/item/Items#BAMBOO` | field | 2 |
| `net/minecraft/world/item/Items#BAMBOO_BLOCK` | field | 1 |
| `net/minecraft/world/item/Items#BAMBOO_BUTTON` | field | 1 |
| `net/minecraft/world/item/Items#BAMBOO_CHEST_RAFT` | field | 1 |
| `net/minecraft/world/item/Items#BAMBOO_DOOR` | field | 1 |
| `net/minecraft/world/item/Items#BAMBOO_FENCE` | field | 1 |
| `net/minecraft/world/item/Items#BAMBOO_FENCE_GATE` | field | 1 |
| `net/minecraft/world/item/Items#BAMBOO_HANGING_SIGN` | field | 1 |
| `net/minecraft/world/item/Items#BAMBOO_PLANKS` | field | 1 |
| `net/minecraft/world/item/Items#BAMBOO_PRESSURE_PLATE` | field | 1 |
| `net/minecraft/world/item/Items#BAMBOO_RAFT` | field | 1 |
| `net/minecraft/world/item/Items#BAMBOO_SIGN` | field | 1 |
| `net/minecraft/world/item/Items#BAMBOO_SLAB` | field | 1 |
| `net/minecraft/world/item/Items#BAMBOO_TRAPDOOR` | field | 2 |
| `net/minecraft/world/item/Items#BEEF` | field | 2 |
| `net/minecraft/world/item/Items#BEETROOT` | field | 5 |
| `net/minecraft/world/item/Items#BEETROOT_SEEDS` | field | 2 |
| `net/minecraft/world/item/Items#BEETROOT_SOUP` | field | 3 |
| `net/minecraft/world/item/Items#BIRCH_BOAT` | field | 1 |
| `net/minecraft/world/item/Items#BIRCH_BUTTON` | field | 1 |
| `net/minecraft/world/item/Items#BIRCH_CHEST_BOAT` | field | 1 |
| `net/minecraft/world/item/Items#BIRCH_DOOR` | field | 1 |
| `net/minecraft/world/item/Items#BIRCH_FENCE` | field | 1 |
| `net/minecraft/world/item/Items#BIRCH_FENCE_GATE` | field | 1 |
| `net/minecraft/world/item/Items#BIRCH_HANGING_SIGN` | field | 1 |
| `net/minecraft/world/item/Items#BIRCH_LOG` | field | 1 |
| `net/minecraft/world/item/Items#BIRCH_PLANKS` | field | 1 |
| `net/minecraft/world/item/Items#BIRCH_PRESSURE_PLATE` | field | 1 |
| `net/minecraft/world/item/Items#BIRCH_SIGN` | field | 1 |
| `net/minecraft/world/item/Items#BIRCH_SLAB` | field | 1 |
| `net/minecraft/world/item/Items#BIRCH_TRAPDOOR` | field | 2 |
| `net/minecraft/world/item/Items#BIRCH_WOOD` | field | 1 |
| `net/minecraft/world/item/Items#BLUE_ORCHID` | field | 1 |
| `net/minecraft/world/item/Items#BONE` | field | 3 |
| `net/minecraft/world/item/Items#BONE_MEAL` | field | 5 |
| `net/minecraft/world/item/Items#BOOK` | field | 1 |
| `net/minecraft/world/item/Items#BOWL` | field | 5 |
| `net/minecraft/world/item/Items#BREAD` | field | 2 |
| `net/minecraft/world/item/Items#BRICK` | field | 2 |
| `net/minecraft/world/item/Items#BRICKS` | field | 1 |
| `net/minecraft/world/item/Items#BROWN_MUSHROOM` | field | 5 |
| `net/minecraft/world/item/Items#BUCKET` | field | 2 |
| `net/minecraft/world/item/Items#CACTUS_FLOWER` | field | 1 |
| `net/minecraft/world/item/Items#CAKE` | field | 2 |
| `net/minecraft/world/item/Items#CARROT` | field | 6 |
| `net/minecraft/world/item/Items#CHERRY_BOAT` | field | 1 |
| `net/minecraft/world/item/Items#CHERRY_BUTTON` | field | 1 |
| `net/minecraft/world/item/Items#CHERRY_CHEST_BOAT` | field | 1 |
| `net/minecraft/world/item/Items#CHERRY_DOOR` | field | 1 |
| `net/minecraft/world/item/Items#CHERRY_FENCE` | field | 1 |
| `net/minecraft/world/item/Items#CHERRY_FENCE_GATE` | field | 1 |
| `net/minecraft/world/item/Items#CHERRY_HANGING_SIGN` | field | 1 |
| `net/minecraft/world/item/Items#CHERRY_LOG` | field | 1 |
| `net/minecraft/world/item/Items#CHERRY_PLANKS` | field | 1 |
| `net/minecraft/world/item/Items#CHERRY_PRESSURE_PLATE` | field | 1 |
| `net/minecraft/world/item/Items#CHERRY_SIGN` | field | 1 |
| `net/minecraft/world/item/Items#CHERRY_SLAB` | field | 1 |
| `net/minecraft/world/item/Items#CHERRY_TRAPDOOR` | field | 2 |
| `net/minecraft/world/item/Items#CHERRY_WOOD` | field | 1 |
| `net/minecraft/world/item/Items#CHEST` | field | 1 |
| `net/minecraft/world/item/Items#CHEST_MINECART` | field | 1 |
| `net/minecraft/world/item/Items#CHICKEN` | field | 2 |
| `net/minecraft/world/item/Items#CLAY` | field | 1 |
| `net/minecraft/world/item/Items#CLAY_BALL` | field | 1 |
| `net/minecraft/world/item/Items#CLOSED_EYEBLOSSOM` | field | 1 |
| `net/minecraft/world/item/Items#COBBLED_DEEPSLATE` | field | 1 |
| `net/minecraft/world/item/Items#COBBLESTONE` | field | 1 |
| `net/minecraft/world/item/Items#COCOA_BEANS` | field | 2 |
| `net/minecraft/world/item/Items#COD` | field | 2 |
| `net/minecraft/world/item/Items#COD_BUCKET` | field | 1 |
| `net/minecraft/world/item/Items#COOKED_BEEF` | field | 1 |
| `net/minecraft/world/item/Items#COOKED_CHICKEN` | field | 2 |
| `net/minecraft/world/item/Items#COOKED_COD` | field | 1 |
| `net/minecraft/world/item/Items#COOKED_MUTTON` | field | 2 |
| `net/minecraft/world/item/Items#COOKED_PORKCHOP` | field | 1 |
| `net/minecraft/world/item/Items#COOKED_SALMON` | field | 1 |
| `net/minecraft/world/item/Items#COPPER_NUGGET` | field | 1 |
| `net/minecraft/world/item/Items#CORNFLOWER` | field | 1 |
| `net/minecraft/world/item/Items#CRIMSON_BUTTON` | field | 1 |
| `net/minecraft/world/item/Items#CRIMSON_DOOR` | field | 1 |
| `net/minecraft/world/item/Items#CRIMSON_FENCE` | field | 1 |
| `net/minecraft/world/item/Items#CRIMSON_FENCE_GATE` | field | 1 |
| `net/minecraft/world/item/Items#CRIMSON_FUNGUS` | field | 1 |
| `net/minecraft/world/item/Items#CRIMSON_HANGING_SIGN` | field | 1 |
| `net/minecraft/world/item/Items#CRIMSON_HYPHAE` | field | 1 |
| `net/minecraft/world/item/Items#CRIMSON_PLANKS` | field | 1 |
| `net/minecraft/world/item/Items#CRIMSON_PRESSURE_PLATE` | field | 1 |
| `net/minecraft/world/item/Items#CRIMSON_SIGN` | field | 1 |
| `net/minecraft/world/item/Items#CRIMSON_SLAB` | field | 1 |
| `net/minecraft/world/item/Items#CRIMSON_STEM` | field | 1 |
| `net/minecraft/world/item/Items#CRIMSON_TRAPDOOR` | field | 2 |
| `net/minecraft/world/item/Items#DANDELION` | field | 1 |
| `net/minecraft/world/item/Items#DARK_OAK_BOAT` | field | 1 |
| `net/minecraft/world/item/Items#DARK_OAK_BUTTON` | field | 1 |
| `net/minecraft/world/item/Items#DARK_OAK_CHEST_BOAT` | field | 1 |
| `net/minecraft/world/item/Items#DARK_OAK_DOOR` | field | 1 |
| `net/minecraft/world/item/Items#DARK_OAK_FENCE` | field | 1 |
| `net/minecraft/world/item/Items#DARK_OAK_FENCE_GATE` | field | 1 |
| `net/minecraft/world/item/Items#DARK_OAK_HANGING_SIGN` | field | 1 |
| `net/minecraft/world/item/Items#DARK_OAK_LOG` | field | 1 |
| `net/minecraft/world/item/Items#DARK_OAK_PLANKS` | field | 1 |
| `net/minecraft/world/item/Items#DARK_OAK_PRESSURE_PLATE` | field | 1 |
| `net/minecraft/world/item/Items#DARK_OAK_SIGN` | field | 1 |
| `net/minecraft/world/item/Items#DARK_OAK_SLAB` | field | 1 |
| `net/minecraft/world/item/Items#DARK_OAK_TRAPDOOR` | field | 2 |
| `net/minecraft/world/item/Items#DARK_OAK_WOOD` | field | 1 |
| `net/minecraft/world/item/Items#DEEPSLATE` | field | 1 |
| `net/minecraft/world/item/Items#DIAMOND` | field | 1 |
| `net/minecraft/world/item/Items#DIRT` | field | 1 |
| `net/minecraft/world/item/Items#DRIED_KELP` | field | 2 |
| `net/minecraft/world/item/Items#DYE` | field | 1 |
| `net/minecraft/world/item/Items#EGG` | field | 1 |
| `net/minecraft/world/item/Items#EMERALD` | field | 1 |
| `net/minecraft/world/item/Items#EXPERIENCE_BOTTLE` | field | 1 |
| `net/minecraft/world/item/Items#FEATHER` | field | 1 |
| `net/minecraft/world/item/Items#FLINT` | field | 2 |
| `net/minecraft/world/item/Items#FURNACE` | field | 1 |
| `net/minecraft/world/item/Items#FURNACE_MINECART` | field | 1 |
| `net/minecraft/world/item/Items#GLASS_BOTTLE` | field | 3 |
| `net/minecraft/world/item/Items#GLOW_BERRIES` | field | 2 |
| `net/minecraft/world/item/Items#GLOW_LICHEN` | field | 1 |
| `net/minecraft/world/item/Items#GOLDEN_CARROT` | field | 1 |
| `net/minecraft/world/item/Items#GOLDEN_DANDELION` | field | 1 |
| `net/minecraft/world/item/Items#GOLD_INGOT` | field | 1 |
| `net/minecraft/world/item/Items#GOLD_NUGGET` | field | 1 |
| `net/minecraft/world/item/Items#GRAVEL` | field | 1 |
| `net/minecraft/world/item/Items#HANGING_ROOTS` | field | 1 |
| `net/minecraft/world/item/Items#HAY_BLOCK` | field | 1 |
| `net/minecraft/world/item/Items#HONEY_BOTTLE` | field | 1 |
| `net/minecraft/world/item/Items#HOPPER` | field | 1 |
| `net/minecraft/world/item/Items#HOPPER_MINECART` | field | 1 |
| `net/minecraft/world/item/Items#ICE` | field | 1 |
| `net/minecraft/world/item/Items#INK_SAC` | field | 2 |
| `net/minecraft/world/item/Items#IRON_CHAIN` | field | 1 |
| `net/minecraft/world/item/Items#IRON_INGOT` | field | 1 |
| `net/minecraft/world/item/Items#IRON_NUGGET` | field | 2 |
| `net/minecraft/world/item/Items#JUNGLE_BOAT` | field | 1 |
| `net/minecraft/world/item/Items#JUNGLE_BUTTON` | field | 1 |
| `net/minecraft/world/item/Items#JUNGLE_CHEST_BOAT` | field | 1 |
| `net/minecraft/world/item/Items#JUNGLE_DOOR` | field | 1 |
| `net/minecraft/world/item/Items#JUNGLE_FENCE` | field | 1 |
| `net/minecraft/world/item/Items#JUNGLE_FENCE_GATE` | field | 1 |
| `net/minecraft/world/item/Items#JUNGLE_HANGING_SIGN` | field | 1 |
| `net/minecraft/world/item/Items#JUNGLE_LOG` | field | 1 |
| `net/minecraft/world/item/Items#JUNGLE_PLANKS` | field | 1 |
| `net/minecraft/world/item/Items#JUNGLE_PRESSURE_PLATE` | field | 1 |
| `net/minecraft/world/item/Items#JUNGLE_SIGN` | field | 1 |
| `net/minecraft/world/item/Items#JUNGLE_SLAB` | field | 1 |
| `net/minecraft/world/item/Items#JUNGLE_TRAPDOOR` | field | 2 |
| `net/minecraft/world/item/Items#JUNGLE_WOOD` | field | 1 |
| `net/minecraft/world/item/Items#LEAD` | field | 2 |
| `net/minecraft/world/item/Items#LEATHER` | field | 2 |
| `net/minecraft/world/item/Items#LEATHER_BOOTS` | field | 1 |
| `net/minecraft/world/item/Items#LEATHER_CHESTPLATE` | field | 1 |
| `net/minecraft/world/item/Items#LEATHER_HELMET` | field | 1 |
| `net/minecraft/world/item/Items#LEATHER_HORSE_ARMOR` | field | 1 |
| `net/minecraft/world/item/Items#LEATHER_LEGGINGS` | field | 1 |
| `net/minecraft/world/item/Items#LILY_OF_THE_VALLEY` | field | 1 |
| `net/minecraft/world/item/Items#LINGERING_POTION` | field | 1 |
| `net/minecraft/world/item/Items#MANGROVE_BOAT` | field | 1 |
| `net/minecraft/world/item/Items#MANGROVE_BUTTON` | field | 1 |
| `net/minecraft/world/item/Items#MANGROVE_CHEST_BOAT` | field | 1 |
| `net/minecraft/world/item/Items#MANGROVE_DOOR` | field | 1 |
| `net/minecraft/world/item/Items#MANGROVE_FENCE` | field | 1 |
| `net/minecraft/world/item/Items#MANGROVE_FENCE_GATE` | field | 1 |
| `net/minecraft/world/item/Items#MANGROVE_HANGING_SIGN` | field | 1 |
| `net/minecraft/world/item/Items#MANGROVE_LOG` | field | 1 |
| `net/minecraft/world/item/Items#MANGROVE_PLANKS` | field | 1 |
| `net/minecraft/world/item/Items#MANGROVE_PRESSURE_PLATE` | field | 1 |
| `net/minecraft/world/item/Items#MANGROVE_SIGN` | field | 1 |
| `net/minecraft/world/item/Items#MANGROVE_SLAB` | field | 1 |
| `net/minecraft/world/item/Items#MANGROVE_TRAPDOOR` | field | 2 |
| `net/minecraft/world/item/Items#MANGROVE_WOOD` | field | 1 |
| `net/minecraft/world/item/Items#MELON` | field | 1 |
| `net/minecraft/world/item/Items#MELON_SLICE` | field | 2 |
| `net/minecraft/world/item/Items#MILK_BUCKET` | field | 2 |
| `net/minecraft/world/item/Items#MINECART` | field | 1 |
| `net/minecraft/world/item/Items#MUD` | field | 1 |
| `net/minecraft/world/item/Items#MUSHROOM_STEW` | field | 3 |
| `net/minecraft/world/item/Items#MUTTON` | field | 2 |
| `net/minecraft/world/item/Items#MYCELIUM` | field | 1 |
| `net/minecraft/world/item/Items#NETHERITE_INGOT` | field | 1 |
| `net/minecraft/world/item/Items#NETHERITE_UPGRADE_SMITHING_TEMPLATE` | field | 1 |
| `net/minecraft/world/item/Items#NETHER_BRICK` | field | 1 |
| `net/minecraft/world/item/Items#NETHER_BRICKS` | field | 1 |
| `net/minecraft/world/item/Items#OAK_BOAT` | field | 1 |
| `net/minecraft/world/item/Items#OAK_BUTTON` | field | 1 |
| `net/minecraft/world/item/Items#OAK_CHEST_BOAT` | field | 1 |
| `net/minecraft/world/item/Items#OAK_DOOR` | field | 1 |
| `net/minecraft/world/item/Items#OAK_FENCE` | field | 1 |
| `net/minecraft/world/item/Items#OAK_FENCE_GATE` | field | 1 |
| `net/minecraft/world/item/Items#OAK_HANGING_SIGN` | field | 1 |
| `net/minecraft/world/item/Items#OAK_LOG` | field | 1 |
| `net/minecraft/world/item/Items#OAK_PLANKS` | field | 1 |
| `net/minecraft/world/item/Items#OAK_PRESSURE_PLATE` | field | 1 |
| `net/minecraft/world/item/Items#OAK_SIGN` | field | 1 |
| `net/minecraft/world/item/Items#OAK_SLAB` | field | 1 |
| `net/minecraft/world/item/Items#OAK_TRAPDOOR` | field | 2 |
| `net/minecraft/world/item/Items#OAK_WOOD` | field | 1 |
| `net/minecraft/world/item/Items#OPEN_EYEBLOSSOM` | field | 1 |
| `net/minecraft/world/item/Items#ORANGE_TULIP` | field | 1 |
| `net/minecraft/world/item/Items#OXEYE_DAISY` | field | 1 |
| `net/minecraft/world/item/Items#PACKED_MUD` | field | 1 |
| `net/minecraft/world/item/Items#PAINTING` | field | 1 |
| `net/minecraft/world/item/Items#PALE_OAK_BOAT` | field | 1 |
| `net/minecraft/world/item/Items#PALE_OAK_BUTTON` | field | 1 |
| `net/minecraft/world/item/Items#PALE_OAK_CHEST_BOAT` | field | 1 |
| `net/minecraft/world/item/Items#PALE_OAK_DOOR` | field | 1 |
| `net/minecraft/world/item/Items#PALE_OAK_FENCE` | field | 1 |
| `net/minecraft/world/item/Items#PALE_OAK_FENCE_GATE` | field | 1 |
| `net/minecraft/world/item/Items#PALE_OAK_HANGING_SIGN` | field | 1 |
| `net/minecraft/world/item/Items#PALE_OAK_LOG` | field | 1 |
| `net/minecraft/world/item/Items#PALE_OAK_PLANKS` | field | 1 |
| `net/minecraft/world/item/Items#PALE_OAK_PRESSURE_PLATE` | field | 1 |
| `net/minecraft/world/item/Items#PALE_OAK_SIGN` | field | 1 |
| `net/minecraft/world/item/Items#PALE_OAK_SLAB` | field | 1 |
| `net/minecraft/world/item/Items#PALE_OAK_TRAPDOOR` | field | 2 |
| `net/minecraft/world/item/Items#PALE_OAK_WOOD` | field | 1 |
| `net/minecraft/world/item/Items#PAPER` | field | 1 |
| `net/minecraft/world/item/Items#PINK_TULIP` | field | 1 |
| `net/minecraft/world/item/Items#PODZOL` | field | 1 |
| `net/minecraft/world/item/Items#POPLAR_BOAT` | field | 1 |
| `net/minecraft/world/item/Items#POPLAR_BUTTON` | field | 1 |
| `net/minecraft/world/item/Items#POPLAR_CHEST_BOAT` | field | 1 |
| `net/minecraft/world/item/Items#POPLAR_DOOR` | field | 1 |
| `net/minecraft/world/item/Items#POPLAR_FENCE` | field | 1 |
| `net/minecraft/world/item/Items#POPLAR_FENCE_GATE` | field | 1 |
| `net/minecraft/world/item/Items#POPLAR_HANGING_SIGN` | field | 1 |
| `net/minecraft/world/item/Items#POPLAR_LOG` | field | 1 |
| `net/minecraft/world/item/Items#POPLAR_PLANKS` | field | 1 |
| `net/minecraft/world/item/Items#POPLAR_PRESSURE_PLATE` | field | 1 |
| `net/minecraft/world/item/Items#POPLAR_SIGN` | field | 1 |
| `net/minecraft/world/item/Items#POPLAR_SLAB` | field | 1 |
| `net/minecraft/world/item/Items#POPLAR_TRAPDOOR` | field | 2 |
| `net/minecraft/world/item/Items#POPLAR_WOOD` | field | 1 |
| `net/minecraft/world/item/Items#POPPY` | field | 1 |
| `net/minecraft/world/item/Items#PORKCHOP` | field | 2 |
| `net/minecraft/world/item/Items#POTATO` | field | 6 |
| `net/minecraft/world/item/Items#POTION` | field | 1 |
| `net/minecraft/world/item/Items#POWDER_SNOW_BUCKET` | field | 1 |
| `net/minecraft/world/item/Items#PUFFERFISH_BUCKET` | field | 1 |
| `net/minecraft/world/item/Items#PUMPKIN` | field | 3 |
| `net/minecraft/world/item/Items#PUMPKIN_PIE` | field | 4 |
| `net/minecraft/world/item/Items#PUMPKIN_SEEDS` | field | 2 |
| `net/minecraft/world/item/Items#QUARTZ` | field | 1 |
| `net/minecraft/world/item/Items#QUARTZ_BLOCK` | field | 1 |
| `net/minecraft/world/item/Items#RABBIT` | field | 1 |
| `net/minecraft/world/item/Items#RABBIT_HIDE` | field | 1 |
| `net/minecraft/world/item/Items#RABBIT_STEW` | field | 4 |
| `net/minecraft/world/item/Items#RED_MUSHROOM` | field | 5 |
| `net/minecraft/world/item/Items#RED_TULIP` | field | 1 |
| `net/minecraft/world/item/Items#ROTTEN_FLESH` | field | 2 |
| `net/minecraft/world/item/Items#SADDLE` | field | 1 |
| `net/minecraft/world/item/Items#SALMON` | field | 3 |
| `net/minecraft/world/item/Items#SALMON_BUCKET` | field | 1 |
| `net/minecraft/world/item/Items#SCAFFOLDING` | field | 1 |
| `net/minecraft/world/item/Items#SHELF_MUSHROOM` | field | 1 |
| `net/minecraft/world/item/Items#SHULKER_SHELL` | field | 1 |
| `net/minecraft/world/item/Items#SPLASH_POTION` | field | 1 |
| `net/minecraft/world/item/Items#SPRUCE_BOAT` | field | 1 |
| `net/minecraft/world/item/Items#SPRUCE_BUTTON` | field | 1 |
| `net/minecraft/world/item/Items#SPRUCE_CHEST_BOAT` | field | 1 |
| `net/minecraft/world/item/Items#SPRUCE_DOOR` | field | 1 |
| `net/minecraft/world/item/Items#SPRUCE_FENCE` | field | 1 |
| `net/minecraft/world/item/Items#SPRUCE_FENCE_GATE` | field | 1 |
| `net/minecraft/world/item/Items#SPRUCE_HANGING_SIGN` | field | 1 |
| `net/minecraft/world/item/Items#SPRUCE_LOG` | field | 1 |
| `net/minecraft/world/item/Items#SPRUCE_PLANKS` | field | 1 |
| `net/minecraft/world/item/Items#SPRUCE_PRESSURE_PLATE` | field | 1 |
| `net/minecraft/world/item/Items#SPRUCE_SIGN` | field | 1 |
| `net/minecraft/world/item/Items#SPRUCE_SLAB` | field | 1 |
| `net/minecraft/world/item/Items#SPRUCE_TRAPDOOR` | field | 2 |
| `net/minecraft/world/item/Items#SPRUCE_WOOD` | field | 1 |
| `net/minecraft/world/item/Items#STICK` | field | 2 |
| `net/minecraft/world/item/Items#STONE` | field | 1 |
| `net/minecraft/world/item/Items#STRAW_BED` | field | 1 |
| `net/minecraft/world/item/Items#STRING` | field | 1 |
| `net/minecraft/world/item/Items#STRIPPED_ACACIA_LOG` | field | 1 |
| `net/minecraft/world/item/Items#STRIPPED_ACACIA_WOOD` | field | 1 |
| `net/minecraft/world/item/Items#STRIPPED_BAMBOO_BLOCK` | field | 1 |
| `net/minecraft/world/item/Items#STRIPPED_BIRCH_LOG` | field | 1 |
| `net/minecraft/world/item/Items#STRIPPED_BIRCH_WOOD` | field | 1 |
| `net/minecraft/world/item/Items#STRIPPED_CHERRY_LOG` | field | 1 |
| `net/minecraft/world/item/Items#STRIPPED_CHERRY_WOOD` | field | 1 |
| `net/minecraft/world/item/Items#STRIPPED_CRIMSON_HYPHAE` | field | 1 |
| `net/minecraft/world/item/Items#STRIPPED_CRIMSON_STEM` | field | 1 |
| `net/minecraft/world/item/Items#STRIPPED_DARK_OAK_LOG` | field | 1 |
| `net/minecraft/world/item/Items#STRIPPED_DARK_OAK_WOOD` | field | 1 |
| `net/minecraft/world/item/Items#STRIPPED_JUNGLE_LOG` | field | 1 |
| `net/minecraft/world/item/Items#STRIPPED_JUNGLE_WOOD` | field | 1 |
| `net/minecraft/world/item/Items#STRIPPED_MANGROVE_LOG` | field | 1 |
| `net/minecraft/world/item/Items#STRIPPED_MANGROVE_WOOD` | field | 1 |
| `net/minecraft/world/item/Items#STRIPPED_OAK_LOG` | field | 1 |
| `net/minecraft/world/item/Items#STRIPPED_OAK_WOOD` | field | 1 |
| `net/minecraft/world/item/Items#STRIPPED_PALE_OAK_LOG` | field | 1 |
| `net/minecraft/world/item/Items#STRIPPED_PALE_OAK_WOOD` | field | 1 |
| `net/minecraft/world/item/Items#STRIPPED_POPLAR_LOG` | field | 1 |
| `net/minecraft/world/item/Items#STRIPPED_POPLAR_WOOD` | field | 1 |
| `net/minecraft/world/item/Items#STRIPPED_SPRUCE_LOG` | field | 1 |
| `net/minecraft/world/item/Items#STRIPPED_SPRUCE_WOOD` | field | 1 |
| `net/minecraft/world/item/Items#STRIPPED_WARPED_HYPHAE` | field | 1 |
| `net/minecraft/world/item/Items#STRIPPED_WARPED_STEM` | field | 1 |
| `net/minecraft/world/item/Items#SUGAR` | field | 2 |
| `net/minecraft/world/item/Items#SUSPICIOUS_STEW` | field | 1 |
| `net/minecraft/world/item/Items#SWEET_BERRIES` | field | 1 |
| `net/minecraft/world/item/Items#TNT` | field | 1 |
| `net/minecraft/world/item/Items#TNT_MINECART` | field | 1 |
| `net/minecraft/world/item/Items#TORCHFLOWER` | field | 1 |
| `net/minecraft/world/item/Items#TROPICAL_FISH` | field | 1 |
| `net/minecraft/world/item/Items#TROPICAL_FISH_BUCKET` | field | 1 |
| `net/minecraft/world/item/Items#WARPED_BUTTON` | field | 1 |
| `net/minecraft/world/item/Items#WARPED_DOOR` | field | 1 |
| `net/minecraft/world/item/Items#WARPED_FENCE` | field | 1 |
| `net/minecraft/world/item/Items#WARPED_FENCE_GATE` | field | 1 |
| `net/minecraft/world/item/Items#WARPED_FUNGUS` | field | 1 |
| `net/minecraft/world/item/Items#WARPED_HANGING_SIGN` | field | 1 |
| `net/minecraft/world/item/Items#WARPED_HYPHAE` | field | 1 |
| `net/minecraft/world/item/Items#WARPED_PLANKS` | field | 1 |
| `net/minecraft/world/item/Items#WARPED_PRESSURE_PLATE` | field | 1 |
| `net/minecraft/world/item/Items#WARPED_SIGN` | field | 1 |
| `net/minecraft/world/item/Items#WARPED_SLAB` | field | 1 |
| `net/minecraft/world/item/Items#WARPED_STEM` | field | 1 |
| `net/minecraft/world/item/Items#WARPED_TRAPDOOR` | field | 2 |
| `net/minecraft/world/item/Items#WATER_BUCKET` | field | 2 |
| `net/minecraft/world/item/Items#WHEAT` | field | 5 |
| `net/minecraft/world/item/Items#WHITE_TULIP` | field | 1 |
| `net/minecraft/world/item/Items#WITHER_ROSE` | field | 1 |
| `net/minecraft/world/item/Items#WOODEN_SHOVEL` | field | 1 |
| `net/minecraft/world/item/LeadItem#bindPlayerMobs` | method | 1 |
| `net/minecraft/world/item/Rarity#color` | method | 1 |
| `net/minecraft/world/item/StandingAndWallBlockItem#<init>` | method | 1 |
| `net/minecraft/world/item/ToolMaterial#<init>` | method | 1 |
| `net/minecraft/world/item/ToolMaterial#COPPER` | field | 1 |
| `net/minecraft/world/item/ToolMaterial#DIAMOND` | field | 1 |
| `net/minecraft/world/item/ToolMaterial#GOLD` | field | 1 |
| `net/minecraft/world/item/ToolMaterial#IRON` | field | 2 |
| `net/minecraft/world/item/ToolMaterial#NETHERITE` | field | 1 |
| `net/minecraft/world/item/ToolMaterial#attackDamageBonus` | method | 2 |
| `net/minecraft/world/item/ToolMaterial#durability` | method | 2 |
| `net/minecraft/world/item/ToolMaterial#enchantmentValue` | method | 1 |
| `net/minecraft/world/item/ToolMaterial#incorrectBlocksForDrops` | method | 1 |
| `net/minecraft/world/item/ToolMaterial#repairItems` | method | 1 |
| `net/minecraft/world/item/ToolMaterial#speed` | method | 1 |
| `net/minecraft/world/item/component/Compostable#<init>` | method | 1 |
| `net/minecraft/world/item/component/Consumable#<init>` | method | 1 |
| `net/minecraft/world/item/component/Consumable#animation` | method | 1 |
| `net/minecraft/world/item/component/Consumable#consumeSeconds` | method | 1 |
| `net/minecraft/world/item/component/Consumable#hasConsumeParticles` | method | 1 |
| `net/minecraft/world/item/component/Consumable#onConsumeEffects` | method | 3 |
| `net/minecraft/world/item/component/Consumable#sound` | method | 1 |
| `net/minecraft/world/item/component/Consumable$Builder#build` | method | 1 |
| `net/minecraft/world/item/component/Consumable$Builder#consumeSeconds` | method | 1 |
| `net/minecraft/world/item/component/Consumable$Builder#onConsume` | method | 1 |
| `net/minecraft/world/item/component/ConsumableListener#onConsume` | method | 1 |
| `net/minecraft/world/item/component/Consumables#DEFAULT_DRINK` | field | 1 |
| `net/minecraft/world/item/component/Consumables#DEFAULT_FOOD` | field | 2 |
| `net/minecraft/world/item/component/Consumables#defaultDrink` | method | 1 |
| `net/minecraft/world/item/component/Consumables#defaultFood` | method | 1 |
| `net/minecraft/world/item/component/CookingFuel#<init>` | method | 2 |
| `net/minecraft/world/item/component/ItemAttributeModifiers#ATTRIBUTE_MODIFIER_FORMAT` | field | 1 |
| `net/minecraft/world/item/component/ItemAttributeModifiers#builder` | method | 2 |
| `net/minecraft/world/item/component/ItemAttributeModifiers$Builder#add` | method | 2 |
| `net/minecraft/world/item/component/ItemAttributeModifiers$Builder#build` | method | 2 |
| `net/minecraft/world/item/component/Tool#<init>` | method | 1 |
| `net/minecraft/world/item/component/Tool$Rule#deniesDrops` | method | 1 |
| `net/minecraft/world/item/component/Tool$Rule#minesAndDrops` | method | 1 |
| `net/minecraft/world/item/component/Tool$Rule#overrideSpeed` | method | 1 |
| `net/minecraft/world/item/component/Weapon#<init>` | method | 1 |
| `net/minecraft/world/item/consume_effects/ApplyStatusEffectsConsumeEffect#<init>` | method | 3 |
| `net/minecraft/world/item/consume_effects/ApplyStatusEffectsConsumeEffect#effects` | method | 2 |
| `net/minecraft/world/item/consume_effects/ConsumeEffect#apply` | method | 1 |
| `net/minecraft/world/item/consume_effects/ConsumeEffect$Type#<init>` | method | 1 |
| `net/minecraft/world/item/context/BlockPlaceContext#<init>` | method | 2 |
| `net/minecraft/world/item/context/BlockPlaceContext#at` | method | 1 |
| `net/minecraft/world/item/context/BlockPlaceContext#getClickedFace` | method | 5 |
| `net/minecraft/world/item/context/BlockPlaceContext#getClickedPos` | method | 13 |
| `net/minecraft/world/item/context/BlockPlaceContext#getHorizontalDirection` | method | 9 |
| `net/minecraft/world/item/context/BlockPlaceContext#getItemInHand` | method | 1 |
| `net/minecraft/world/item/context/BlockPlaceContext#getLevel` | method | 13 |
| `net/minecraft/world/item/context/BlockPlaceContext#getNearestLookingDirection` | method | 1 |
| `net/minecraft/world/item/context/BlockPlaceContext#getPlayer` | method | 2 |
| `net/minecraft/world/item/context/BlockPlaceContext#isSecondaryUseActive` | method | 1 |
| `net/minecraft/world/item/context/UseOnContext#getClickedFace` | method | 2 |
| `net/minecraft/world/item/context/UseOnContext#getClickedPos` | method | 2 |
| `net/minecraft/world/item/context/UseOnContext#getHand` | method | 2 |
| `net/minecraft/world/item/context/UseOnContext#getItemInHand` | method | 2 |
| `net/minecraft/world/item/context/UseOnContext#getLevel` | method | 2 |
| `net/minecraft/world/item/context/UseOnContext#getPlayer` | method | 3 |
| `net/minecraft/world/item/crafting/AbstractCookingRecipe#assemble` | method | 1 |
| `net/minecraft/world/item/crafting/AbstractCookingRecipe#cookingTime` | method | 1 |
| `net/minecraft/world/item/crafting/CampfireCookingRecipe#assemble` | method | 2 |
| `net/minecraft/world/item/crafting/CampfireCookingRecipe#cookingTime` | method | 2 |
| `net/minecraft/world/item/crafting/CookingBookCategory#FOOD` | field | 1 |
| `net/minecraft/world/item/crafting/CookingBookCategory#MISC` | field | 1 |
| `net/minecraft/world/item/crafting/CraftingBookCategory#MISC` | field | 2 |
| `net/minecraft/world/item/crafting/CraftingInput#getItem` | method | 2 |
| `net/minecraft/world/item/crafting/CraftingInput#size` | method | 2 |
| `net/minecraft/world/item/crafting/CraftingRecipe$CraftingBookInfo#<init>` | method | 2 |
| `net/minecraft/world/item/crafting/CustomRecipe#<init>` | method | 2 |
| `net/minecraft/world/item/crafting/Ingredient#CODEC` | field | 2 |
| `net/minecraft/world/item/crafting/Ingredient#CONTENTS_STREAM_CODEC` | field | 2 |
| `net/minecraft/world/item/crafting/Ingredient#display` | method | 1 |
| `net/minecraft/world/item/crafting/Ingredient#equals` | method | 1 |
| `net/minecraft/world/item/crafting/Ingredient#items` | method | 1 |
| `net/minecraft/world/item/crafting/Ingredient#of` | method | 15 |
| `net/minecraft/world/item/crafting/Ingredient#test` | method | 2 |
| `net/minecraft/world/item/crafting/PlacementInfo#NOT_PLACEABLE` | field | 1 |
| `net/minecraft/world/item/crafting/PlacementInfo#create` | method | 1 |
| `net/minecraft/world/item/crafting/Recipe#KEY_CODEC` | field | 1 |
| `net/minecraft/world/item/crafting/Recipe$CommonInfo#<init>` | method | 2 |
| `net/minecraft/world/item/crafting/RecipeAccess#getSynchronizedRecipes` | method | 1 |
| `net/minecraft/world/item/crafting/RecipeAccess#propertySet` | method | 3 |
| `net/minecraft/world/item/crafting/RecipeBookCategory#<init>` | method | 1 |
| `net/minecraft/world/item/crafting/RecipeHolder#<init>` | method | 2 |
| `net/minecraft/world/item/crafting/RecipeHolder#id` | method | 4 |
| `net/minecraft/world/item/crafting/RecipeHolder#value` | method | 12 |
| `net/minecraft/world/item/crafting/RecipeManager#<clinit>` | method | 1 |
| `net/minecraft/world/item/crafting/RecipeManager#RECIPE_PROPERTY_SETS` | method | 1 |
| `net/minecraft/world/item/crafting/RecipeManager#byKey` | method | 1 |
| `net/minecraft/world/item/crafting/RecipeManager#createCheck` | method | 4 |
| `net/minecraft/world/item/crafting/RecipeManager#getRecipeFor` | method | 2 |
| `net/minecraft/world/item/crafting/RecipeManager$CachedCheck#getRecipeFor` | method | 4 |
| `net/minecraft/world/item/crafting/RecipePropertySet#CAMPFIRE_INPUT` | field | 2 |
| `net/minecraft/world/item/crafting/RecipePropertySet#TYPE_KEY` | field | 1 |
| `net/minecraft/world/item/crafting/RecipePropertySet#test` | method | 3 |
| `net/minecraft/world/item/crafting/RecipeSerializer#<init>` | method | 3 |
| `net/minecraft/world/item/crafting/RecipeType#CAMPFIRE_COOKING` | field | 3 |
| `net/minecraft/world/item/crafting/RecipeType#CRAFTING` | field | 1 |
| `net/minecraft/world/item/crafting/RecipeType#SMOKING` | field | 1 |
| `net/minecraft/world/item/crafting/ShapelessRecipe#<init>` | method | 2 |
| `net/minecraft/world/item/crafting/SingleRecipeInput#<init>` | method | 4 |
| `net/minecraft/world/item/crafting/SmokingRecipe#assemble` | method | 1 |
| `net/minecraft/world/item/crafting/display/RecipeDisplay#isEnabled` | method | 1 |
| `net/minecraft/world/item/crafting/display/RecipeDisplay#result` | method | 1 |
| `net/minecraft/world/item/crafting/display/RecipeDisplay$Type#<init>` | method | 1 |
| `net/minecraft/world/item/crafting/display/RecipeDisplayId#<init>` | method | 1 |
| `net/minecraft/world/item/crafting/display/RecipeDisplayId#STREAM_CODEC` | field | 1 |
| `net/minecraft/world/item/crafting/display/RecipeDisplayId#index` | method | 1 |
| `net/minecraft/world/item/crafting/display/SlotDisplay#CODEC` | field | 1 |
| `net/minecraft/world/item/crafting/display/SlotDisplay#STREAM_CODEC` | field | 1 |
| `net/minecraft/world/item/crafting/display/SlotDisplay#isEnabled` | method | 1 |
| `net/minecraft/world/item/crafting/display/SlotDisplay$ItemSlotDisplay#<init>` | method | 1 |
| `net/minecraft/world/item/crafting/display/SlotDisplay$ItemStackSlotDisplay#<init>` | method | 1 |
| `net/minecraft/world/item/enchantment/ConditionalEffect#codec` | method | 1 |
| `net/minecraft/world/item/enchantment/Enchantment#definition` | method | 1 |
| `net/minecraft/world/item/enchantment/Enchantment#dynamicCost` | method | 1 |
| `net/minecraft/world/item/enchantment/Enchantment#enchantment` | method | 1 |
| `net/minecraft/world/item/enchantment/Enchantment#modifyDamageFilteredValue` | method | 1 |
| `net/minecraft/world/item/enchantment/Enchantment$Builder#build` | method | 1 |
| `net/minecraft/world/item/enchantment/Enchantment$Builder#withEffect` | method | 1 |
| `net/minecraft/world/item/enchantment/EnchantmentHelper#getItemEnchantmentLevel` | method | 2 |
| `net/minecraft/world/item/enchantment/EnchantmentHelper#runIterationOnItem` | method | 1 |
| `net/minecraft/world/item/enchantment/Enchantments#FIRE_ASPECT` | field | 2 |
| `net/minecraft/world/item/enchantment/Enchantments#FORTUNE` | field | 2 |
| `net/minecraft/world/item/enchantment/Enchantments#SILK_TOUCH` | field | 1 |
| `net/minecraft/world/item/enchantment/Enchantments#SWEEPING_EDGE` | field | 2 |
| `net/minecraft/world/item/enchantment/ItemEnchantments#EMPTY` | field | 1 |
| `net/minecraft/world/item/enchantment/ItemEnchantments#getLevel` | method | 1 |
| `net/minecraft/world/item/enchantment/LevelBasedValue#perLevel` | method | 1 |
| `net/minecraft/world/item/enchantment/effects/EnchantmentValueEffect#CODEC` | field | 1 |
| `net/minecraft/world/item/enchantment/effects/MultiplyValue#<init>` | method | 1 |
| `net/minecraft/world/item/trading/TradeCost#<init>` | method | 1 |
| `net/minecraft/world/item/trading/VillagerTrade#builder` | method | 1 |
| `net/minecraft/world/item/trading/VillagerTrade$Builder#build` | method | 1 |
| `net/minecraft/world/level/BlockGetter#getBlockState` | method | 2 |
| `net/minecraft/world/level/ItemLike#asItem` | method | 4 |
| `net/minecraft/world/level/Level#addFreshEntity` | method | 3 |
| `net/minecraft/world/level/Level#addParticle` | method | 10 |
| `net/minecraft/world/level/Level#broadcastEntityEvent` | method | 1 |
| `net/minecraft/world/level/Level#damageSources` | method | 2 |
| `net/minecraft/world/level/Level#gameEvent` | method | 2 |
| `net/minecraft/world/level/Level#getBlockEntity` | method | 9 |
| `net/minecraft/world/level/Level#getBlockState` | method | 11 |
| `net/minecraft/world/level/Level#getEntitiesOfClass` | method | 1 |
| `net/minecraft/world/level/Level#getFluidState` | method | 8 |
| `net/minecraft/world/level/Level#getGameTime` | method | 1 |
| `net/minecraft/world/level/Level#getRandom` | method | 14 |
| `net/minecraft/world/level/Level#hasNeighborSignal` | method | 1 |
| `net/minecraft/world/level/Level#holderLookup` | method | 1 |
| `net/minecraft/world/level/Level#levelEvent` | method | 2 |
| `net/minecraft/world/level/Level#playLocalSound` | method | 3 |
| `net/minecraft/world/level/Level#playSound` | method | 18 |
| `net/minecraft/world/level/Level#recipeAccess` | method | 3 |
| `net/minecraft/world/level/Level#registryAccess` | method | 3 |
| `net/minecraft/world/level/Level#sendBlockUpdated` | method | 2 |
| `net/minecraft/world/level/Level#setBlockAndUpdate` | method | 3 |
| `net/minecraft/world/level/Level#tickRateManager` | method | 1 |
| `net/minecraft/world/level/LevelAccessor#destroyBlock` | method | 1 |
| `net/minecraft/world/level/LevelAccessor#gameEvent` | method | 1 |
| `net/minecraft/world/level/LevelAccessor#getBlockEntity` | method | 1 |
| `net/minecraft/world/level/LevelAccessor#getBlockState` | method | 2 |
| `net/minecraft/world/level/LevelAccessor#getFluidState` | method | 1 |
| `net/minecraft/world/level/LevelAccessor#isClientSide` | method | 2 |
| `net/minecraft/world/level/LevelAccessor#playSound` | method | 1 |
| `net/minecraft/world/level/LevelAccessor#setBlock` | method | 3 |
| `net/minecraft/world/level/LevelReader#canSeeSky` | method | 3 |
| `net/minecraft/world/level/LevelReader#getBlockEntity` | method | 2 |
| `net/minecraft/world/level/LevelReader#getBlockState` | method | 12 |
| `net/minecraft/world/level/LevelReader#getFluidState` | method | 3 |
| `net/minecraft/world/level/LevelReader#getRawBrightness` | method | 4 |
| `net/minecraft/world/level/LevelReader#isEmptyBlock` | method | 2 |
| `net/minecraft/world/level/LightLayer#SKY` | field | 1 |
| `net/minecraft/world/level/ScheduledTickAccess#scheduleTick` | method | 10 |
| `net/minecraft/world/level/WorldGenLevel#getBlockState` | method | 1 |
| `net/minecraft/world/level/biome/Biome#getBaseTemperature` | method | 1 |
| `net/minecraft/world/level/block/BaseEntityBlock#<init>` | method | 6 |
| `net/minecraft/world/level/block/BaseEntityBlock#createBlockStateDefinition` | method | 4 |
| `net/minecraft/world/level/block/BaseEntityBlock#createTickerHelper` | method | 1 |
| `net/minecraft/world/level/block/BaseEntityBlock#getCloneItemStack` | method | 2 |
| `net/minecraft/world/level/block/BaseEntityBlock#getFluidState` | method | 4 |
| `net/minecraft/world/level/block/BaseEntityBlock#neighborChanged` | method | 1 |
| `net/minecraft/world/level/block/BaseEntityBlock#stepOn` | method | 1 |
| `net/minecraft/world/level/block/BaseEntityBlock#updateShape` | method | 2 |
| `net/minecraft/world/level/block/BaseEntityBlock#useItemOn` | method | 1 |
| `net/minecraft/world/level/block/BedBlock#PART` | field | 1 |
| `net/minecraft/world/level/block/BellBlock#FACING` | field | 1 |
| `net/minecraft/world/level/block/BellBlock#attemptToRing` | method | 1 |
| `net/minecraft/world/level/block/Block#animateTick` | method | 1 |
| `net/minecraft/world/level/block/Block#asItem` | method | 1 |
| `net/minecraft/world/level/block/Block#box` | method | 28 |
| `net/minecraft/world/level/block/Block#createBlockStateDefinition` | method | 1 |
| `net/minecraft/world/level/block/Block#defaultBlockState` | method | 17 |
| `net/minecraft/world/level/block/Block#dropResources` | method | 1 |
| `net/minecraft/world/level/block/Block#fallOn` | method | 1 |
| `net/minecraft/world/level/block/Block#getFluidState` | method | 1 |
| `net/minecraft/world/level/block/Block#getId` | method | 1 |
| `net/minecraft/world/level/block/Block#getLootTable` | method | 1 |
| `net/minecraft/world/level/block/Block#updateShape` | method | 4 |
| `net/minecraft/world/level/block/Blocks#AIR` | field | 8 |
| `net/minecraft/world/level/block/Blocks#BARREL` | field | 1 |
| `net/minecraft/world/level/block/Blocks#BEETROOTS` | field | 1 |
| `net/minecraft/world/level/block/Blocks#BELL` | field | 1 |
| `net/minecraft/world/level/block/Blocks#BRICKS` | field | 2 |
| `net/minecraft/world/level/block/Blocks#BROWN_MUSHROOM` | field | 4 |
| `net/minecraft/world/level/block/Blocks#CAKE` | field | 3 |
| `net/minecraft/world/level/block/Blocks#CAMPFIRE` | field | 2 |
| `net/minecraft/world/level/block/Blocks#CARPET` | field | 1 |
| `net/minecraft/world/level/block/Blocks#CARROTS` | field | 1 |
| `net/minecraft/world/level/block/Blocks#CARVED_PUMPKIN` | field | 1 |
| `net/minecraft/world/level/block/Blocks#CAVE_VINES` | field | 1 |
| `net/minecraft/world/level/block/Blocks#CHORUS_FLOWER` | field | 1 |
| `net/minecraft/world/level/block/Blocks#COCOA` | field | 1 |
| `net/minecraft/world/level/block/Blocks#DIRT` | field | 1 |
| `net/minecraft/world/level/block/Blocks#FARMLAND` | field | 1 |
| `net/minecraft/world/level/block/Blocks#HAY_BLOCK` | field | 1 |
| `net/minecraft/world/level/block/Blocks#KELP` | field | 1 |
| `net/minecraft/world/level/block/Blocks#MELON` | field | 1 |
| `net/minecraft/world/level/block/Blocks#MELON_STEM` | field | 1 |
| `net/minecraft/world/level/block/Blocks#NETHER_WART` | field | 1 |
| `net/minecraft/world/level/block/Blocks#OAK_FENCE` | field | 1 |
| `net/minecraft/world/level/block/Blocks#OAK_PLANKS` | field | 1 |
| `net/minecraft/world/level/block/Blocks#POTATOES` | field | 2 |
| `net/minecraft/world/level/block/Blocks#PUMPKIN` | field | 3 |
| `net/minecraft/world/level/block/Blocks#PUMPKIN_STEM` | field | 1 |
| `net/minecraft/world/level/block/Blocks#RED_MUSHROOM` | field | 4 |
| `net/minecraft/world/level/block/Blocks#SHELF_MUSHROOM` | field | 1 |
| `net/minecraft/world/level/block/Blocks#SOUL_CAMPFIRE` | field | 1 |
| `net/minecraft/world/level/block/Blocks#SPRUCE_HANGING_SIGN` | field | 1 |
| `net/minecraft/world/level/block/Blocks#SPRUCE_SIGN` | field | 1 |
| `net/minecraft/world/level/block/Blocks#SUGAR_CANE` | field | 1 |
| `net/minecraft/world/level/block/Blocks#SWEET_BERRY_BUSH` | field | 1 |
| `net/minecraft/world/level/block/Blocks#TALL_GRASS` | field | 1 |
| `net/minecraft/world/level/block/Blocks#WATER` | field | 1 |
| `net/minecraft/world/level/block/Blocks#WHEAT` | field | 4 |
| `net/minecraft/world/level/block/Blocks#WOOL` | field | 1 |
| `net/minecraft/world/level/block/BonemealSource#INTERACTION` | field | 2 |
| `net/minecraft/world/level/block/BonemealableBlock#isValidBonemealTarget` | method | 3 |
| `net/minecraft/world/level/block/CakeBlock#BITES` | field | 3 |
| `net/minecraft/world/level/block/CampfireBlock#isSmokeSource` | method | 1 |
| `net/minecraft/world/level/block/CarvedPumpkinBlock#FACING` | field | 1 |
| `net/minecraft/world/level/block/CeilingHangingSignBlock#<init>` | method | 1 |
| `net/minecraft/world/level/block/ColorCollection#black` | method | 3 |
| `net/minecraft/world/level/block/ColorCollection#blue` | method | 3 |
| `net/minecraft/world/level/block/ColorCollection#brown` | method | 2 |
| `net/minecraft/world/level/block/ColorCollection#create` | method | 2 |
| `net/minecraft/world/level/block/ColorCollection#cyan` | method | 2 |
| `net/minecraft/world/level/block/ColorCollection#gray` | method | 3 |
| `net/minecraft/world/level/block/ColorCollection#green` | method | 3 |
| `net/minecraft/world/level/block/ColorCollection#lightBlue` | method | 3 |
| `net/minecraft/world/level/block/ColorCollection#lightGray` | method | 3 |
| `net/minecraft/world/level/block/ColorCollection#lime` | method | 3 |
| `net/minecraft/world/level/block/ColorCollection#magenta` | method | 3 |
| `net/minecraft/world/level/block/ColorCollection#map` | method | 4 |
| `net/minecraft/world/level/block/ColorCollection#orange` | method | 3 |
| `net/minecraft/world/level/block/ColorCollection#pink` | method | 3 |
| `net/minecraft/world/level/block/ColorCollection#prefixWithColor` | method | 2 |
| `net/minecraft/world/level/block/ColorCollection#purple` | method | 3 |
| `net/minecraft/world/level/block/ColorCollection#red` | method | 3 |
| `net/minecraft/world/level/block/ColorCollection#white` | method | 3 |
| `net/minecraft/world/level/block/ColorCollection#yellow` | method | 3 |
| `net/minecraft/world/level/block/CropBlock#<init>` | method | 4 |
| `net/minecraft/world/level/block/CropBlock#AGE` | field | 2 |
| `net/minecraft/world/level/block/CropBlock#canSurvive` | method | 1 |
| `net/minecraft/world/level/block/CropBlock#getBonemealAgeIncrease` | method | 2 |
| `net/minecraft/world/level/block/CropBlock#playerDestroy` | method | 1 |
| `net/minecraft/world/level/block/CropBlock#updateShape` | method | 1 |
| `net/minecraft/world/level/block/CropBlock#useItemOn` | method | 1 |
| `net/minecraft/world/level/block/CropBlock#useWithoutItem` | method | 1 |
| `net/minecraft/world/level/block/CrossCollisionBlock#<init>` | method | 1 |
| `net/minecraft/world/level/block/CrossCollisionBlock#EAST` | field | 1 |
| `net/minecraft/world/level/block/CrossCollisionBlock#NORTH` | field | 1 |
| `net/minecraft/world/level/block/CrossCollisionBlock#PROPERTY_BY_DIRECTION` | field | 1 |
| `net/minecraft/world/level/block/CrossCollisionBlock#SOUTH` | field | 1 |
| `net/minecraft/world/level/block/CrossCollisionBlock#WATERLOGGED` | field | 1 |
| `net/minecraft/world/level/block/CrossCollisionBlock#WEST` | field | 1 |
| `net/minecraft/world/level/block/CrossCollisionBlock#getStateForPlacement` | method | 1 |
| `net/minecraft/world/level/block/CrossCollisionBlock#updateShape` | method | 1 |
| `net/minecraft/world/level/block/DispenserBlock#DISPENSER_REGISTRY` | field | 1 |
| `net/minecraft/world/level/block/DispenserBlock#FACING` | field | 2 |
| `net/minecraft/world/level/block/DispenserBlock#getDispenseMethod` | method | 1 |
| `net/minecraft/world/level/block/DispenserBlock#registerBehavior` | method | 1 |
| `net/minecraft/world/level/block/DispenserBlock#registerProjectileBehavior` | method | 1 |
| `net/minecraft/world/level/block/DoublePlantBlock#<init>` | method | 1 |
| `net/minecraft/world/level/block/DoublePlantBlock#HALF` | field | 2 |
| `net/minecraft/world/level/block/DoublePlantBlock#canSurvive` | method | 1 |
| `net/minecraft/world/level/block/DoublePlantBlock#getStateForPlacement` | method | 1 |
| `net/minecraft/world/level/block/DoublePlantBlock#placeAt` | method | 1 |
| `net/minecraft/world/level/block/DoublePlantBlock#updateShape` | method | 1 |
| `net/minecraft/world/level/block/FarmlandBlock#<init>` | method | 1 |
| `net/minecraft/world/level/block/FarmlandBlock#MOISTURE` | field | 1 |
| `net/minecraft/world/level/block/FarmlandBlock#canSurvive` | method | 1 |
| `net/minecraft/world/level/block/FarmlandBlock#getStateForPlacement` | method | 1 |
| `net/minecraft/world/level/block/FenceBlock#connectsTo` | method | 1 |
| `net/minecraft/world/level/block/FenceBlock#isSameFence` | method | 1 |
| `net/minecraft/world/level/block/FenceGateBlock#<init>` | method | 3 |
| `net/minecraft/world/level/block/FenceGateBlock#connectsToDirection` | method | 1 |
| `net/minecraft/world/level/block/FenceGateBlock#neighborChanged` | method | 2 |
| `net/minecraft/world/level/block/FenceGateBlock#onExplosionHit` | method | 1 |
| `net/minecraft/world/level/block/FenceGateBlock#updateShape` | method | 1 |
| `net/minecraft/world/level/block/FenceGateBlock#useWithoutItem` | method | 3 |
| `net/minecraft/world/level/block/FlowerBlock#<init>` | method | 1 |
| `net/minecraft/world/level/block/HayBlock#<init>` | method | 1 |
| `net/minecraft/world/level/block/HorizontalDirectionalBlock#<init>` | method | 2 |
| `net/minecraft/world/level/block/HorizontalDirectionalBlock#FACING` | field | 4 |
| `net/minecraft/world/level/block/HorizontalDirectionalBlock#playerWillDestroy` | method | 1 |
| `net/minecraft/world/level/block/HorizontalDirectionalBlock#setPlacedBy` | method | 1 |
| `net/minecraft/world/level/block/HorizontalDirectionalBlock#updateShape` | method | 2 |
| `net/minecraft/world/level/block/IronBarsBlock#<init>` | method | 1 |
| `net/minecraft/world/level/block/IronBarsBlock#updateShape` | method | 1 |
| `net/minecraft/world/level/block/IronBarsBlock#useItemOn` | method | 1 |
| `net/minecraft/world/level/block/Mirror#getRotation` | method | 6 |
| `net/minecraft/world/level/block/RenderShape#MODEL` | field | 7 |
| `net/minecraft/world/level/block/Rotation#rotate` | method | 6 |
| `net/minecraft/world/level/block/SoundType#BAMBOO_WOOD` | field | 1 |
| `net/minecraft/world/level/block/SoundType#CHERRY_WOOD` | field | 1 |
| `net/minecraft/world/level/block/SoundType#CROP` | field | 1 |
| `net/minecraft/world/level/block/SoundType#GRASS` | field | 1 |
| `net/minecraft/world/level/block/SoundType#LANTERN` | field | 1 |
| `net/minecraft/world/level/block/SoundType#NETHER_WOOD` | field | 1 |
| `net/minecraft/world/level/block/SoundType#STRAW_BED` | field | 1 |
| `net/minecraft/world/level/block/SoundType#WOOD` | field | 1 |
| `net/minecraft/world/level/block/SoundType#WOOL` | field | 1 |
| `net/minecraft/world/level/block/SoundType#getBreakSound` | method | 2 |
| `net/minecraft/world/level/block/StandingSignBlock#<init>` | method | 1 |
| `net/minecraft/world/level/block/StrawBedBlock#<init>` | method | 1 |
| `net/minecraft/world/level/block/VegetationBlock#<init>` | method | 4 |
| `net/minecraft/world/level/block/VegetationBlock#canSurvive` | method | 2 |
| `net/minecraft/world/level/block/VegetationBlock#createBlockStateDefinition` | method | 1 |
| `net/minecraft/world/level/block/VegetationBlock#entityInside` | method | 1 |
| `net/minecraft/world/level/block/VegetationBlock#getStateForPlacement` | method | 1 |
| `net/minecraft/world/level/block/VegetationBlock#tick` | method | 1 |
| `net/minecraft/world/level/block/VegetationBlock#updateShape` | method | 1 |
| `net/minecraft/world/level/block/VegetationBlock#useItemOn` | method | 1 |
| `net/minecraft/world/level/block/WallHangingSignBlock#<init>` | method | 1 |
| `net/minecraft/world/level/block/WallSignBlock#<init>` | method | 1 |
| `net/minecraft/world/level/block/entity/BlockEntity#<init>` | method | 3 |
| `net/minecraft/world/level/block/entity/BlockEntity#loadAdditional` | method | 1 |
| `net/minecraft/world/level/block/entity/BlockEntity#preRemoveSideEffects` | method | 1 |
| `net/minecraft/world/level/block/entity/BlockEntity#saveAdditional` | method | 1 |
| `net/minecraft/world/level/block/entity/BlockEntity#setChanged` | method | 1 |
| `net/minecraft/world/level/block/entity/BlockEntityType#addValidBlock` | method | 1 |
| `net/minecraft/world/level/block/entity/BlockEntityType#create` | method | 4 |
| `net/minecraft/world/level/block/entity/BlockEntityTypes#HANGING_SIGN` | field | 1 |
| `net/minecraft/world/level/block/entity/BlockEntityTypes#SIGN` | field | 1 |
| `net/minecraft/world/level/block/entity/ContainerOpenersCounter#<init>` | method | 1 |
| `net/minecraft/world/level/block/entity/ContainerOpenersCounter#decrementOpeners` | method | 1 |
| `net/minecraft/world/level/block/entity/ContainerOpenersCounter#incrementOpeners` | method | 1 |
| `net/minecraft/world/level/block/entity/ContainerOpenersCounter#recheckOpeners` | method | 1 |
| `net/minecraft/world/level/block/entity/DispenserBlockEntity#setItem` | method | 1 |
| `net/minecraft/world/level/block/entity/RandomizableContainerBlockEntity#<init>` | method | 2 |
| `net/minecraft/world/level/block/entity/RandomizableContainerBlockEntity#loadAdditional` | method | 2 |
| `net/minecraft/world/level/block/entity/RandomizableContainerBlockEntity#saveAdditional` | method | 2 |
| `net/minecraft/world/level/block/entity/SignBlockEntity#<init>` | method | 1 |
| `net/minecraft/world/level/block/entity/SignBlockEntity#applyImplicitComponents` | method | 1 |
| `net/minecraft/world/level/block/entity/SignBlockEntity#getBlockState` | method | 2 |
| `net/minecraft/world/level/block/entity/SignBlockEntity#loadAdditional` | method | 1 |
| `net/minecraft/world/level/block/entity/SignText#asMutable` | method | 1 |
| `net/minecraft/world/level/block/entity/SignText$Mutable#asImmutable` | method | 1 |
| `net/minecraft/world/level/block/entity/SignText$Mutable#setColor` | method | 1 |
| `net/minecraft/world/level/block/state/BlockBehaviour$Properties#blockIdOrThrow` | method | 1 |
| `net/minecraft/world/level/block/state/BlockBehaviour$Properties#bounceRestitution` | method | 1 |
| `net/minecraft/world/level/block/state/BlockBehaviour$Properties#forceSolidOn` | method | 1 |
| `net/minecraft/world/level/block/state/BlockBehaviour$Properties#ignitedByLava` | method | 1 |
| `net/minecraft/world/level/block/state/BlockBehaviour$Properties#lightLevel` | method | 1 |
| `net/minecraft/world/level/block/state/BlockBehaviour$Properties#mapColor` | method | 1 |
| `net/minecraft/world/level/block/state/BlockBehaviour$Properties#noCollision` | method | 1 |
| `net/minecraft/world/level/block/state/BlockBehaviour$Properties#noOcclusion` | method | 1 |
| `net/minecraft/world/level/block/state/BlockBehaviour$Properties#of` | method | 1 |
| `net/minecraft/world/level/block/state/BlockBehaviour$Properties#ofFullCopy` | method | 1 |
| `net/minecraft/world/level/block/state/BlockBehaviour$Properties#overrideLootTable` | method | 1 |
| `net/minecraft/world/level/block/state/BlockBehaviour$Properties#pushReaction` | method | 1 |
| `net/minecraft/world/level/block/state/BlockBehaviour$Properties#randomTicks` | method | 1 |
| `net/minecraft/world/level/block/state/BlockBehaviour$Properties#setId` | method | 1 |
| `net/minecraft/world/level/block/state/BlockBehaviour$Properties#sound` | method | 1 |
| `net/minecraft/world/level/block/state/BlockBehaviour$Properties#strength` | method | 2 |
| `net/minecraft/world/level/block/state/BlockState#getBlock` | method | 24 |
| `net/minecraft/world/level/block/state/BlockState#getDestroySpeed` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#getFluidState` | method | 2 |
| `net/minecraft/world/level/block/state/BlockState#getOptionalValue` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#getShape` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#getSoundType` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#getValue` | method | 46 |
| `net/minecraft/world/level/block/state/BlockState#getValueOrElse` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#hasProperty` | method | 2 |
| `net/minecraft/world/level/block/state/BlockState#is` | method | 36 |
| `net/minecraft/world/level/block/state/BlockState#isAir` | method | 3 |
| `net/minecraft/world/level/block/state/BlockState#isFaceSturdy` | method | 2 |
| `net/minecraft/world/level/block/state/BlockState#isSolidRender` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#rotate` | method | 6 |
| `net/minecraft/world/level/block/state/BlockState#setValue` | method | 31 |
| `net/minecraft/world/level/block/state/BlockState#updateNeighbourShapes` | method | 2 |
| `net/minecraft/world/level/block/state/StateDefinition#any` | method | 17 |
| `net/minecraft/world/level/block/state/StateDefinition$Builder#add` | method | 24 |
| `net/minecraft/world/level/block/state/properties/BedPart#FOOT` | field | 1 |
| `net/minecraft/world/level/block/state/properties/BedPart#HEAD` | field | 2 |
| `net/minecraft/world/level/block/state/properties/BlockStateProperties#AGE_3` | field | 4 |
| `net/minecraft/world/level/block/state/properties/BlockStateProperties#BED_PART` | field | 1 |
| `net/minecraft/world/level/block/state/properties/BlockStateProperties#ENABLED` | field | 2 |
| `net/minecraft/world/level/block/state/properties/BlockStateProperties#FACING` | field | 3 |
| `net/minecraft/world/level/block/state/properties/BlockStateProperties#HORIZONTAL_FACING` | field | 6 |
| `net/minecraft/world/level/block/state/properties/BlockStateProperties#LIT` | field | 3 |
| `net/minecraft/world/level/block/state/properties/BlockStateProperties#MOISTURE` | field | 1 |
| `net/minecraft/world/level/block/state/properties/BlockStateProperties#OPEN` | field | 1 |
| `net/minecraft/world/level/block/state/properties/BlockStateProperties#WATERLOGGED` | field | 6 |
| `net/minecraft/world/level/block/state/properties/BooleanProperty#create` | method | 5 |
| `net/minecraft/world/level/block/state/properties/DoubleBlockHalf#LOWER` | field | 3 |
| `net/minecraft/world/level/block/state/properties/DoubleBlockHalf#UPPER` | field | 2 |
| `net/minecraft/world/level/block/state/properties/EnumProperty#create` | method | 1 |
| `net/minecraft/world/level/block/state/properties/IntegerProperty#create` | method | 5 |
| `net/minecraft/world/level/block/state/properties/IntegerProperty#getPossibleValues` | method | 1 |
| `net/minecraft/world/level/block/state/properties/WoodType#ACACIA` | field | 1 |
| `net/minecraft/world/level/block/state/properties/WoodType#BAMBOO` | field | 1 |
| `net/minecraft/world/level/block/state/properties/WoodType#BIRCH` | field | 1 |
| `net/minecraft/world/level/block/state/properties/WoodType#CHERRY` | field | 1 |
| `net/minecraft/world/level/block/state/properties/WoodType#CRIMSON` | field | 1 |
| `net/minecraft/world/level/block/state/properties/WoodType#DARK_OAK` | field | 1 |
| `net/minecraft/world/level/block/state/properties/WoodType#JUNGLE` | field | 1 |
| `net/minecraft/world/level/block/state/properties/WoodType#MANGROVE` | field | 1 |
| `net/minecraft/world/level/block/state/properties/WoodType#OAK` | field | 2 |
| `net/minecraft/world/level/block/state/properties/WoodType#PALE_OAK` | field | 1 |
| `net/minecraft/world/level/block/state/properties/WoodType#POPLAR` | field | 1 |
| `net/minecraft/world/level/block/state/properties/WoodType#SPRUCE` | field | 5 |
| `net/minecraft/world/level/block/state/properties/WoodType#WARPED` | field | 1 |
| `net/minecraft/world/level/block/state/properties/WoodType#name` | method | 1 |
| `net/minecraft/world/level/chunk/LevelChunk#getPos` | method | 1 |
| `net/minecraft/world/level/gameevent/GameEvent#BLOCK_CHANGE` | field | 3 |
| `net/minecraft/world/level/gameevent/GameEvent$Context#of` | method | 3 |
| `net/minecraft/world/level/gamerules/GameRules#MOB_GRIEFING` | field | 1 |
| `net/minecraft/world/level/gamerules/GameRules#NATURAL_HEALTH_REGENERATION` | field | 3 |
| `net/minecraft/world/level/gamerules/GameRules#get` | method | 3 |
| `net/minecraft/world/level/levelgen/GenerationStep$Decoration#VEGETAL_DECORATION` | field | 1 |
| `net/minecraft/world/level/levelgen/placement/PlacedFeature#LIST_CODEC` | field | 1 |
| `net/minecraft/world/level/levelgen/placement/PlacedFeature#place` | method | 2 |
| `net/minecraft/world/level/levelgen/placement/PlacementContext#getLevel` | method | 1 |
| `net/minecraft/world/level/levelgen/placement/PlacementFilter#modify` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/pools/SinglePoolElement#single` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/pools/StructureTemplatePool#rawTemplates` | field | 1 |
| `net/minecraft/world/level/levelgen/structure/pools/StructureTemplatePool#templates` | field | 1 |
| `net/minecraft/world/level/levelgen/structure/pools/StructureTemplatePool$Projection#RIGID` | field | 1 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/AlwaysTrueTest#INSTANCE` | field | 1 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/ProcessorRule#<init>` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/RandomBlockMatchTest#<init>` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/RuleProcessor#<init>` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/StructureProcessorList#list` | method | 2 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/StructureTemplate#<init>` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/StructureTemplate#save` | method | 1 |
| `net/minecraft/world/level/material/FlowingFluid#getSource` | method | 7 |
| `net/minecraft/world/level/material/FlowingFluid#getTickDelay` | method | 9 |
| `net/minecraft/world/level/material/Fluid#defaultFluidState` | method | 1 |
| `net/minecraft/world/level/material/FluidState#getAmount` | method | 2 |
| `net/minecraft/world/level/material/FluidState#getType` | method | 7 |
| `net/minecraft/world/level/material/FluidState#is` | method | 5 |
| `net/minecraft/world/level/material/FluidState#isEmpty` | method | 1 |
| `net/minecraft/world/level/material/Fluids#EMPTY` | field | 1 |
| `net/minecraft/world/level/material/Fluids#WATER` | field | 9 |
| `net/minecraft/world/level/material/MapColor#COLOR_YELLOW` | field | 1 |
| `net/minecraft/world/level/material/MapColor#METAL` | field | 1 |
| `net/minecraft/world/level/material/MapColor#PLANT` | field | 1 |
| `net/minecraft/world/level/material/MapColor#WOOD` | field | 1 |
| `net/minecraft/world/level/material/PushReaction#POPPED` | field | 1 |
| `net/minecraft/world/level/material/PushReaction#PUSH_PULL` | field | 1 |
| `net/minecraft/world/level/pathfinder/PathType#FIRE` | field | 1 |
| `net/minecraft/world/level/storage/DataVersion#version` | method | 1 |
| `net/minecraft/world/level/storage/TagValueOutput#buildResult` | method | 2 |
| `net/minecraft/world/level/storage/TagValueOutput#createWithContext` | method | 2 |
| `net/minecraft/world/level/storage/ValueInput#childOrEmpty` | method | 4 |
| `net/minecraft/world/level/storage/ValueInput#getBooleanOr` | method | 1 |
| `net/minecraft/world/level/storage/ValueInput#getIntArray` | method | 1 |
| `net/minecraft/world/level/storage/ValueInput#getIntOr` | method | 3 |
| `net/minecraft/world/level/storage/ValueInput#lookup` | method | 1 |
| `net/minecraft/world/level/storage/ValueInput#read` | method | 2 |
| `net/minecraft/world/level/storage/ValueOutput#child` | method | 4 |
| `net/minecraft/world/level/storage/ValueOutput#putBoolean` | method | 1 |
| `net/minecraft/world/level/storage/ValueOutput#putInt` | method | 3 |
| `net/minecraft/world/level/storage/ValueOutput#putIntArray` | method | 1 |
| `net/minecraft/world/level/storage/ValueOutput#store` | method | 1 |
| `net/minecraft/world/level/storage/ValueOutput#storeNullable` | method | 2 |
| `net/minecraft/world/level/storage/loot/BuiltInLootTables#ABANDONED_MINESHAFT` | field | 1 |
| `net/minecraft/world/level/storage/loot/BuiltInLootTables#BASTION_HOGLIN_STABLE` | field | 1 |
| `net/minecraft/world/level/storage/loot/BuiltInLootTables#BASTION_TREASURE` | field | 1 |
| `net/minecraft/world/level/storage/loot/BuiltInLootTables#END_CITY_TREASURE` | field | 1 |
| `net/minecraft/world/level/storage/loot/BuiltInLootTables#PILLAGER_OUTPOST` | field | 1 |
| `net/minecraft/world/level/storage/loot/BuiltInLootTables#RUINED_PORTAL` | field | 1 |
| `net/minecraft/world/level/storage/loot/BuiltInLootTables#SHIPWRECK_SUPPLY` | field | 1 |
| `net/minecraft/world/level/storage/loot/BuiltInLootTables#SIMPLE_DUNGEON` | field | 1 |
| `net/minecraft/world/level/storage/loot/BuiltInLootTables#VILLAGE_BUTCHER` | field | 1 |
| `net/minecraft/world/level/storage/loot/BuiltInLootTables#VILLAGE_DESERT_HOUSE` | field | 1 |
| `net/minecraft/world/level/storage/loot/BuiltInLootTables#VILLAGE_PLAINS_HOUSE` | field | 1 |
| `net/minecraft/world/level/storage/loot/BuiltInLootTables#VILLAGE_SAVANNA_HOUSE` | field | 1 |
| `net/minecraft/world/level/storage/loot/BuiltInLootTables#VILLAGE_SNOWY_HOUSE` | field | 1 |
| `net/minecraft/world/level/storage/loot/BuiltInLootTables#VILLAGE_TAIGA_HOUSE` | field | 1 |
| `net/minecraft/world/level/storage/loot/LootContext#getLevel` | method | 1 |
| `net/minecraft/world/level/storage/loot/LootContext#getOptional` | method | 4 |
| `net/minecraft/world/level/storage/loot/LootContext$EntityTarget#ATTACKER` | field | 1 |
| `net/minecraft/world/level/storage/loot/LootContext$EntityTarget#THIS` | field | 1 |
| `net/minecraft/world/level/storage/loot/LootPool#lootPool` | method | 3 |
| `net/minecraft/world/level/storage/loot/LootPool$Builder#add` | method | 4 |
| `net/minecraft/world/level/storage/loot/LootPool$Builder#apply` | method | 1 |
| `net/minecraft/world/level/storage/loot/LootPool$Builder#build` | method | 1 |
| `net/minecraft/world/level/storage/loot/LootPool$Builder#setRolls` | method | 2 |
| `net/minecraft/world/level/storage/loot/LootPool$Builder#when` | method | 3 |
| `net/minecraft/world/level/storage/loot/LootTable#lootTable` | method | 2 |
| `net/minecraft/world/level/storage/loot/LootTable$Builder#modifyPools` | method | 1 |
| `net/minecraft/world/level/storage/loot/LootTable$Builder#pool` | method | 1 |
| `net/minecraft/world/level/storage/loot/LootTable$Builder#withPool` | method | 3 |
| `net/minecraft/world/level/storage/loot/entries/AlternativesEntry#alternatives` | method | 1 |
| `net/minecraft/world/level/storage/loot/entries/EmptyLootItem#emptyItem` | method | 1 |
| `net/minecraft/world/level/storage/loot/entries/LootItem#lootTableItem` | method | 3 |
| `net/minecraft/world/level/storage/loot/entries/NestedLootTable#lootTableReference` | method | 1 |
| `net/minecraft/world/level/storage/loot/entries/UniformContainerBase$Builder#apply` | method | 3 |
| `net/minecraft/world/level/storage/loot/entries/UniformContainerBase$Builder#build` | method | 1 |
| `net/minecraft/world/level/storage/loot/entries/UniformContainerBase$Builder#otherwise` | method | 1 |
| `net/minecraft/world/level/storage/loot/entries/UniformContainerBase$Builder#setWeight` | method | 1 |
| `net/minecraft/world/level/storage/loot/entries/UniformContainerBase$Builder#when` | method | 2 |
| `net/minecraft/world/level/storage/loot/functions/ApplyBonusCount#addBonusBinomialDistributionCount` | method | 1 |
| `net/minecraft/world/level/storage/loot/functions/ApplyBonusCount#addUniformBonusCount` | method | 1 |
| `net/minecraft/world/level/storage/loot/functions/ApplyExplosionDecay#explosionDecay` | method | 1 |
| `net/minecraft/world/level/storage/loot/functions/CopyComponentsFunction#copyComponentsFromBlockEntity` | method | 1 |
| `net/minecraft/world/level/storage/loot/functions/CopyComponentsFunction$Builder#include` | method | 1 |
| `net/minecraft/world/level/storage/loot/functions/EnchantRandomlyFunction#randomApplicableEnchantment` | method | 1 |
| `net/minecraft/world/level/storage/loot/functions/EnchantWithLevelsFunction#enchantWithLevels` | method | 1 |
| `net/minecraft/world/level/storage/loot/functions/LootItemConditionalFunction#<init>` | method | 2 |
| `net/minecraft/world/level/storage/loot/functions/SetItemCountFunction#setCount` | method | 3 |
| `net/minecraft/world/level/storage/loot/functions/SetItemDamageFunction#setDamage` | method | 1 |
| `net/minecraft/world/level/storage/loot/parameters/LootContextParamSets#CHEST` | field | 1 |
| `net/minecraft/world/level/storage/loot/parameters/LootContextParams#ATTACKING_ENTITY` | field | 1 |
| `net/minecraft/world/level/storage/loot/parameters/LootContextParams#BLOCK_ENTITY` | field | 2 |
| `net/minecraft/world/level/storage/loot/parameters/LootContextParams#BLOCK_STATE` | field | 2 |
| `net/minecraft/world/level/storage/loot/parameters/LootContextParams#THIS_ENTITY` | field | 1 |
| `net/minecraft/world/level/storage/loot/parameters/LootContextParams#TOOL` | field | 2 |
| `net/minecraft/world/level/storage/loot/predicates/AllOfCondition$Builder#and` | method | 1 |
| `net/minecraft/world/level/storage/loot/predicates/AnyOfCondition$Builder#build` | method | 1 |
| `net/minecraft/world/level/storage/loot/predicates/ExplosionCondition#survivesExplosion` | method | 1 |
| `net/minecraft/world/level/storage/loot/predicates/LocationCheck#checkLocation` | method | 2 |
| `net/minecraft/world/level/storage/loot/predicates/LootItemCondition#CODEC` | field | 1 |
| `net/minecraft/world/level/storage/loot/predicates/LootItemCondition$Builder#and` | method | 1 |
| `net/minecraft/world/level/storage/loot/predicates/LootItemCondition$Builder#build` | method | 1 |
| `net/minecraft/world/level/storage/loot/predicates/LootItemCondition$Builder#invert` | method | 2 |
| `net/minecraft/world/level/storage/loot/predicates/LootItemCondition$Builder#or` | method | 1 |
| `net/minecraft/world/level/storage/loot/predicates/LootItemEntityPropertyCondition#hasProperties` | method | 1 |
| `net/minecraft/world/level/storage/loot/predicates/LootItemRandomChanceCondition#randomChance` | method | 2 |
| `net/minecraft/world/level/storage/loot/predicates/LootItemRandomChanceWithEnchantedBonusCondition#randomChanceAndLootingBoost` | method | 1 |
| `net/minecraft/world/level/storage/loot/predicates/MatchBlock#blockMatches` | method | 2 |
| `net/minecraft/world/level/storage/loot/predicates/MatchTool#toolMatches` | method | 2 |
| `net/minecraft/world/level/storage/loot/providers/number/floats/ContextFloatProviders#COOKING_DEFAULT_SPEED_MULTIPLIER` | field | 2 |
| `net/minecraft/world/level/storage/loot/providers/number/floats/ContextFloatProviders#between` | method | 1 |
| `net/minecraft/world/level/storage/loot/providers/number/floats/ResolvableFloat#fromKey` | method | 2 |
| `net/minecraft/world/level/storage/loot/providers/number/ints/ContextIntProviders#COMPOSTABLE_ALWAYS_ADD_ONE` | field | 1 |
| `net/minecraft/world/level/storage/loot/providers/number/ints/ContextIntProviders#COMPOSTABLE_LOW` | field | 1 |
| `net/minecraft/world/level/storage/loot/providers/number/ints/ContextIntProviders#COMPOSTABLE_LOW_MEDIUM` | field | 1 |
| `net/minecraft/world/level/storage/loot/providers/number/ints/ContextIntProviders#COMPOSTABLE_MEDIUM` | field | 1 |
| `net/minecraft/world/level/storage/loot/providers/number/ints/ContextIntProviders#COMPOSTABLE_MEDIUM_HIGH` | field | 2 |
| `net/minecraft/world/level/storage/loot/providers/number/ints/ContextIntProviders#between` | method | 2 |
| `net/minecraft/world/level/storage/loot/providers/number/ints/ContextIntProviders#exactly` | method | 3 |
| `net/minecraft/world/level/storage/loot/providers/number/ints/ResolvableInt$Constant#<init>` | method | 2 |
| `net/minecraft/world/phys/AABB#move` | method | 2 |
| `net/minecraft/world/phys/BlockHitResult#getBlockPos` | method | 2 |
| `net/minecraft/world/phys/EntityHitResult#getEntity` | method | 1 |
| `net/minecraft/world/phys/Vec2#<init>` | method | 2 |
| `net/minecraft/world/phys/Vec2#x` | field | 2 |
| `net/minecraft/world/phys/Vec2#y` | field | 2 |
| `net/minecraft/world/phys/Vec3#<init>` | method | 2 |
| `net/minecraft/world/phys/Vec3#atCenterOf` | method | 3 |
| `net/minecraft/world/phys/Vec3#dot` | method | 1 |
| `net/minecraft/world/phys/Vec3#subtract` | method | 1 |
| `net/minecraft/world/phys/Vec3#x` | method | 4 |
| `net/minecraft/world/phys/Vec3#y` | method | 3 |
| `net/minecraft/world/phys/Vec3#z` | method | 4 |
| `net/minecraft/world/phys/shapes/BooleanOp#AND` | field | 1 |
| `net/minecraft/world/phys/shapes/BooleanOp#ONLY_FIRST` | field | 2 |
| `net/minecraft/world/phys/shapes/BooleanOp#OR` | field | 3 |
| `net/minecraft/world/phys/shapes/Shapes#block` | method | 1 |
| `net/minecraft/world/phys/shapes/Shapes#empty` | method | 2 |
| `net/minecraft/world/phys/shapes/Shapes#join` | method | 3 |
| `net/minecraft/world/phys/shapes/Shapes#or` | method | 4 |
| `net/minecraft/world/phys/shapes/VoxelShape#bounds` | method | 1 |
| `net/minecraft/world/phys/shapes/VoxelShape#forAllBoxes` | method | 1 |
| `net/minecraft/world/phys/shapes/VoxelShape#toAabbs` | method | 1 |

</details>

