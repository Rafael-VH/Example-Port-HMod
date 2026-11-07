# Horizon port report: Better Nether

`betternether` 26.302.1 · better-nether-26.302.1.jar · 612 classes

**Partly convertible**

| Status | Count | Meaning |
|---|---|---|
| auto | 1 | The Horizon hook or injection is known: the wiring can be generated. The Java body still has to become Rust. |
| content | 9 | Data and assets: horizon-port-pack carries these into a content mod (.hmod). |
| needs-mapping | 33 | The kind of change carries over, but the matching Horizon function isn't in the table yet. Run `match`. |
| needs-translation | 128 | No direct equivalent: a person (or an AI agent) has to rewrite this part. |
| unsupported | 4 | Horizon has nothing this can attach to yet. |

## Entry points

- `main`: org.betterx.betternether.BetterNether
- `client`: org.betterx.betternether.client.BetterNetherClient
- `wover.blocks`: org.betterx.betternether.registry.NetherClassification
- `wover.block.definitions`: org.betterx.datagen.betternether.blocks.NetherBlockDefinitions
- `fabric-datagen`: org.betterx.datagen.betternether.BetterNetherDatagen
- `emi_disabled`: org.betterx.betternether.integrations.emi.EMIPlugin
- `jei_mod_plugin`: org.betterx.betternether.integrations.jei.JeiPlugin

## Fabric events

| Event | In | Horizon | Status | Note |
|---|---|---|---|---|
| CommandRegistrationCallback.EVENT | `CommandRegistry` | hook `command` | needs-translation | Server commands. Horizon's command hook is client-side. |
| FabricBlockEntityTypeBuilder.create | `BlockEntitiesRegistry` | – | unsupported | Block entities aren't supported in Horizon mods yet. |
| FabricBlockEntityTypeBuilder.build | `BlockEntitiesRegistry` | – | unsupported | Block entities aren't supported in Horizon mods yet. |
| EntityRendererRegistry.register | `EntityRenderRegistry` | – | unsupported | Custom entity renderers aren't supported. |

## Mixins

| Injection | In | Horizon | Status | Note |
|---|---|---|---|---|
| @Inject(HEAD) on LavaFluid.animateTick | `LavaFluidMixin.displayTick` | HEAD | needs-mapping | No Horizon function known for LavaFluid.animateTick: run `match`. |
| @Inject(TAIL) on Minecraft.<init>* | `MinecraftClientMixin.onInit` | RETURN | needs-translation | Horizon has no TAIL: RETURN runs at every exit, early ones too. Wildcard or unknown selector `<init>*`. |
| @ModifyArg(INVOKE) on AvatarRenderer.<init> at Lnet/minecraft/client/renderer/entity/ArmorModelSet;bake(Lnet/minecraft/client/renderer/entity/ArmorModelSet;Lnet/minecraft/client/model/geom/EntityModelSet;Ljava/util/function/Function;)Lnet/minecraft/client/renderer/entity/ArmorModelSet; | `PlayerArmorMixin.bn_useThinArmorLayers` | MODIFY_ARG | needs-mapping | No Horizon function known for AvatarRenderer.<init>: run `match`. The call it hooks (ArmorModelSet.bake) isn't mapped to a Rust function yet. |
| @Inject(RETURN) on ArmorStandRenderer.<init>* | `StandArmorMixin.onInit` | RETURN | needs-translation | Horizon's RETURN runs at every normal exit and passes the return value on. Wildcard or unknown selector `<init>*`. |
| @Accessor on LayerDefinition.mesh | `TexturedModelDataMixin.getMesh` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for LayerDefinition.mesh: run `match`. |
| @Inject(RETURN) on AbstractFurnaceBlockEntity.getBurnDuration | `AbstractFurnaceBlockEntityMixin.betternether$creditCrystalBurn` | RETURN | needs-mapping | Horizon's RETURN runs at every normal exit and passes the return value on. No Horizon function known for AbstractFurnaceBlockEntity.getBurnDuration: run `match`. |
| @Inject(RETURN) on AbstractFurnaceBlockEntity.getTotalCookTime | `AbstractFurnaceBlockEntityMixin.betternether$getTotalCookTime` | RETURN | needs-mapping | Horizon's RETURN runs at every normal exit and passes the return value on. No Horizon function known for AbstractFurnaceBlockEntity.getTotalCookTime: run `match`. |
| @Inject(HEAD) on BlockAgeProcessor.processBlock | `BlockAgeProcessorMixin.bn_processBlock` | HEAD | needs-mapping | No Horizon function known for BlockAgeProcessor.processBlock: run `match`. |
| @Accessor on BlockBehaviour.properties | `BlockBehaviourAccessor.getProperties` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for BlockBehaviour.properties: run `match`. |
| @Inject(HEAD) on Block.dropResources | `BlockBehaviourMixin.bn_getDrops` | HEAD | needs-mapping | No Horizon function known for Block.dropResources: run `match`. |
| @Accessor on Properties.offsetFunction | `BlockBehaviourPropertiesAccessor.betternether$getOffsetFunction` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for Properties.offsetFunction: run `match`. |
| @Accessor on Properties.offsetFunction | `BlockBehaviourPropertiesAccessor.betternether$setOffsetFunction` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for Properties.offsetFunction: run `match`. |
| @ModifyArg(INVOKE) on CopperGolemAi.initIdleActivity at Lnet/minecraft/world/entity/ai/behavior/TransportItemsBetweenContainers;<init>(FLjava/util/function/Predicate;Ljava/util/function/Predicate;IILjava/util/Map;Ljava/util/function/Consumer;Ljava/util/function/Predicate;)V | `CopperGolemAiMixin.bn_acceptWoodenChests` | MODIFY_ARG | needs-mapping | No Horizon function known for CopperGolemAi.initIdleActivity: run `match`. The call it hooks (TransportItemsBetweenContainers.<init>) isn't mapped to a Rust function yet. |
| @Inject(HEAD) on CraftingMenu.stillValid | `CraftingScreenHandlerMixin.canUse` | HEAD | needs-mapping | No Horizon function known for CraftingMenu.stillValid: run `match`. |
| @Inject(HEAD) on DefaultAttributes.getSupplier | `DefaultAttributeRegistryMixin.getAttribute` | HEAD | needs-mapping | No Horizon function known for DefaultAttributes.getSupplier: run `match`. |
| @Inject(HEAD) on DefaultAttributes.hasSupplier | `DefaultAttributeRegistryMixin.hasDefinition` | HEAD | needs-mapping | No Horizon function known for DefaultAttributes.hasSupplier: run `match`. |
| @Inject(HEAD) on DeltaFeature.isClear | `DeltaFeatureMixin.checkBlock` | HEAD | needs-mapping | No Horizon function known for DeltaFeature.isClear: run `match`. |
| @Inject(HEAD) on Bootstrap.run | `FlatLevelGeneratorPresetsMixin.bcl_run` | HEAD | needs-mapping | No Horizon function known for Bootstrap.run: run `match`. |
| @Inject(TAIL) on LightningRodBlock.onLightningStrike | `LightningRodBlockMixin.bn_onLightningStrike` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for LightningRodBlock.onLightningStrike: run `match`. |
| @Inject(INVOKE) on LiquidBlock.shouldSpreadLiquid at Lnet/minecraft/world/level/Level;setBlockAndUpdate(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)Z | `LiquidBlockMixin.bn_shouldSpreadLiquid` | BEFORE_CALL on horizon_world::level::fluids::Sim<'_, L>::should_spread_liquid | needs-mapping | Only whole statements, plain let initializers and block tails can be selected. The call it hooks (Level.setBlockAndUpdate) isn't mapped to a Rust function yet. |
| @Inject(HEAD) on MapItem.update | `MapMixin.bn_update` | HEAD | needs-mapping | No Horizon function known for MapItem.update: run `match`. |
| @Inject(HEAD) on MappedRegistry.freeze | `MappedRegistryMixin.bn_debug` | HEAD | needs-mapping | No Horizon function known for MappedRegistry.freeze: run `match`. |
| @Inject(RETURN) on MushroomBlock.canSurvive | `MushroomMixin.canStay` | RETURN | needs-mapping | Horizon's RETURN runs at every normal exit and passes the return value on. No Horizon function known for MushroomBlock.canSurvive: run `match`. |
| @Inject(HEAD) on MushroomBlock.performBonemeal | `MushroomMixin.growStructure` | HEAD | needs-mapping | No Horizon function known for MushroomBlock.performBonemeal: run `match`. |
| @Inject(RETURN) on NetherWartBlock.mayPlaceOn | `NetherWartMixin.canStay` | RETURN | needs-mapping | Horizon's RETURN runs at every normal exit and passes the return value on. No Horizon function known for NetherWartBlock.mayPlaceOn: run `match`. |
| @Inject(HEAD) on NetherWartBlock.randomTick | `NetherWartMixin.bn_randomTick` | HEAD | needs-mapping | No Horizon function known for NetherWartBlock.randomTick: run `match`. |
| @Inject(HEAD) on ServerPlayer.findRespawnPositionAndUseSpawnBlock | `PlayerEntityMixin.bn_statueRespawn` | HEAD | needs-mapping | No Horizon function known for ServerPlayer.findRespawnPositionAndUseSpawnBlock: run `match`. |
| @Inject(RETURN) on PortalShape.findAnyShape | `PortalShapeMixin.bn_findAnyShape` | RETURN | needs-mapping | Horizon's RETURN runs at every normal exit and passes the return value on. No Horizon function known for PortalShape.findAnyShape: run `match`. |
| @Inject(HEAD) on PortalShape.isValid | `PortalShapeMixin.bn_isValid` | HEAD | needs-mapping | No Horizon function known for PortalShape.isValid: run `match`. |
| @Inject(HEAD) on PortalShape.isComplete | `PortalShapeMixin.bn_isComplete` | HEAD | needs-mapping | No Horizon function known for PortalShape.isComplete: run `match`. |
| @Inject(HEAD) on PortalShape.createPortalBlocks | `PortalShapeMixin.bn_createPortalBlocks` | HEAD on horizon_world::level::portal::Sim<'_, L>::create_portal_blocks | auto |  |
| @WrapOperation(INVOKE) on Player.createAttributes at Lnet/minecraft/world/entity/LivingEntity;createLivingAttributes()Lnet/minecraft/world/entity/ai/attributes/AttributeSupplier$Builder; | `PlayerMixin.wover_test_createAttributes` | WRAP_OPERATION | needs-mapping | No Horizon function known for Player.createAttributes: run `match`. The call it hooks (LivingEntity.createLivingAttributes) isn't mapped to a Rust function yet. |
| @WrapOperation(INVOKE) on Player.getDestroySpeed at Lnet/minecraft/world/item/ItemStack;getDestroySpeed(Lnet/minecraft/world/level/block/state/BlockState;)F | `PlayerMixin.wover_test_getDestroySpeed` | WRAP_OPERATION | needs-mapping | No Horizon function known for Player.getDestroySpeed: run `match`. The call it hooks (ItemStack.getDestroySpeed) isn't mapped to a Rust function yet. |
| @Inject(HEAD) on PiglinAi.isWearingSafeArmor | `PiglinAiMixin.bn_isWearingSafeArmor` | HEAD | needs-mapping | No Horizon function known for PiglinAi.isWearingSafeArmor: run `match`. |
| @Inject(HEAD) on PiglinBrute.populateDefaultEquipmentSlots | `PiglinBruteMixin.bn_populateDefaultEquipmentSlots` | HEAD | needs-mapping | No Horizon function known for PiglinBrute.populateDefaultEquipmentSlots: run `match`. |
| @Inject(HEAD) on Piglin.populateDefaultEquipmentSlots | `PiglinMixin.bn_populateDefaultEquipmentSlots` | HEAD | needs-mapping | No Horizon function known for Piglin.populateDefaultEquipmentSlots: run `match`. |

## Classes extending the game

| Class | In | Status | Note |
|---|---|---|---|
| extends SimpleCriterionTrigger | `ConvertByLightningTrigger` | needs-translation | No mapping for this superclass yet. |
| extends BaseContainerBlockEntity | `BNBrewingStandBlockEntity` | needs-translation | No mapping for this superclass yet. |
| extends RandomizableContainerBlockEntity | `BlockEntityChestOfDrawers` | needs-translation | No mapping for this superclass yet. |
| extends AbstractFurnaceBlockEntity | `BlockEntityForge` | needs-translation | No mapping for this superclass yet. |
| extends AbstractFurnaceBlockEntity | `BlockEntityFurnace` | needs-translation | No mapping for this superclass yet. |
| extends BrewingStandBlock | `BNBrewingStand` | needs-translation | No mapping for this superclass yet. |
| extends TintedParticleLeavesBlock | `BNLeaves` | needs-translation | No mapping for this superclass yet. |
| extends Block | `BNObsidianBase` | needs-translation | Overrides neighborChanged: the content ports, that behaviour needs native code. |
| extends IronBarsBlock | `BNPane` | needs-translation | No mapping for this superclass yet. |
| extends RotatedPillarBlock | `BNPillar` | needs-translation | No mapping for this superclass yet. |
| extends DoorBlock | `BNWoodlikeDoor` | needs-translation | No mapping for this superclass yet. |
| extends Block | `BaseBlockCommonSapling` | needs-translation | Overrides getShape, updateShape, isValidBonemealTarget, isBonemealSuccess, canGrowTerrain, grow, performBonemeal, randomTick: the content ports, that behaviour needs native code. |
| extends Block | `BaseBlockMold` | needs-translation | Overrides getShadeBrightness, updateShape, randomTick: the content ports, that behaviour needs native code. |
| extends Block | `BlockAnchorTreeVine` | needs-translation | Overrides getLuminance, getOffsetType, createBlockStateDefinition, getShape, getShadeBrightness, propagatesSkylightDown, updateShape, getCloneItemStack: the content ports, that behaviour needs native code. |
| extends Block | `BlockBNPot` | needs-translation | Overrides useShapeForLightOcclusion, getShape, useItemOn: the content ports, that behaviour needs native code. |
| extends Block | `BlockBlackBush` | needs-translation | Overrides getShape, canSurvive, updateShape, isValidBonemealTarget, isBonemealSuccess, performBonemeal: the content ports, that behaviour needs native code. |
| extends Block | `BlockBoneMushroom` | needs-translation | Overrides createBlockStateDefinition, getShape, getShadeBrightness, canSurvive, rotate, mirror, updateShape, randomTick, getStateForPlacement: the content ports, that behaviour needs native code. |
| extends Block | `BlockBrownLargeMushroom` | needs-translation | Overrides getCloneItemStack, createBlockStateDefinition, getShape, rotate, mirror, getRotationIndex, rotOffset, updateShape, getStateIfSame: the content ports, that behaviour needs native code. |
| extends BaseEntityBlock | `BlockChestOfDrawers` | needs-translation | No mapping for this superclass yet. |
| extends Block | `BlockCincinnasitPillar` | needs-translation | Overrides createBlockStateDefinition, updateShape: the content ports, that behaviour needs native code. |
| extends AnvilBlock | `BlockCincinnasiteAnvil` | needs-translation | No mapping for this superclass yet. |
| extends AbstractFurnaceBlock | `BlockCincinnasiteForge` | needs-translation | No mapping for this superclass yet. |
| extends Block | `BlockCincinnasiteFrame` | needs-translation | Overrides getShadeBrightness, propagatesSkylightDown, skipRendering: the content ports, that behaviour needs native code. |
| extends Block | `BlockCincinnasitePedestal` | needs-translation | Overrides getShape: the content ports, that behaviour needs native code. |
| extends Block | `BlockCommonPlant` | needs-translation | Overrides getMaxAge, getAgeProperty, createBlockStateDefinition, canSurvive, updateShape, isValidBonemealTarget, isBonemealSuccess, canGrowTerrain, grow, performBonemeal, randomTick: the content ports, that behaviour needs native code. |
| extends Block | `BlockEyeBase` | needs-translation | Overrides allowsSpawning, updateShape, getCloneItemStack: the content ports, that behaviour needs native code. |
| extends Block | `BlockEyeSeed` | needs-translation | Overrides getShape, isValidBonemealTarget, canGrowRandomly, isBonemealSuccess, grow, performBonemeal, canSurvive, updateShape, randomTick: the content ports, that behaviour needs native code. |
| extends Block | `BlockFarmland` | content | A plain block becomes a content-mod block copying a vanilla template. Overridden behaviour methods need native code. |
| extends Block | `BlockFireBowl` | needs-translation | Overrides getLuminance, createBlockStateDefinition, getShape, getOcclusionShape, useItemOn, stepOn, animateTick: the content ports, that behaviour needs native code. |
| extends Block | `BlockGeyser` | needs-translation | Overrides getShape, animateTick, stepOn, canSurvive, updateShape: the content ports, that behaviour needs native code. |
| extends HugeMushroomBlock | `BlockGiantLucis` | needs-translation | No mapping for this superclass yet. |
| extends Block | `BlockGiantMold` | needs-translation | Overrides createBlockStateDefinition, getShape, getCloneItemStack, updateShape: the content ports, that behaviour needs native code. |
| extends Block | `BlockGiantMoldSapling` | needs-translation | Overrides getShape, canSurvive, updateShape, isValidBonemealTarget, canGrowRandomly, isBonemealSuccess, grow, performBonemeal, randomTick: the content ports, that behaviour needs native code. |
| extends Block | `BlockGloomsculkCrystal` | needs-translation | Overrides createBlockStateDefinition, getShape, getStateForPlacement, canSurvive, updateShape: the content ports, that behaviour needs native code. |
| extends Block | `BlockGloomsculkGeode` | needs-translation | Overrides randomTick, playerDestroy: the content ports, that behaviour needs native code. |
| extends Block | `BlockGloomwispVine` | needs-translation | Overrides stateAwareOffset, createBlockStateDefinition, getStateForPlacement, getShadeBrightness, getShape, updateShape, shapeAt, useItemOn, entityInside, shedExperience, drain, tick, base, head, canSurvive, animateTick, randomTick: the content ports, that behaviour needs native code. |
| extends Block | `BlockJellyfishMushroom` | needs-translation | Overrides createBlockStateDefinition, getShape, getCloneItemStack, getShadeBrightness, propagatesSkylightDown, updateShape, fallOn, stepOn: the content ports, that behaviour needs native code. |
| extends Block | `BlockJellyfishMushroomSapling` | needs-translation | Overrides getShape, canSurvive, updateShape, isValidBonemealTarget, canGrowRandomly, isBonemealSuccess, grow, performBonemeal, randomTick: the content ports, that behaviour needs native code. |
| extends Block | `BlockLucisMushroom` | needs-translation | Overrides createBlockStateDefinition, getShape, rotate, mirror, getCloneItemStack: the content ports, that behaviour needs native code. |
| extends Block | `BlockLucisSpore` | needs-translation | Overrides createBlockStateDefinition, getShape, isValidBonemealTarget, canGrowRandomly, isBonemealSuccess, grow, performBonemeal, updateShape, randomTick, rotate, mirror, canSurvive, getStateForPlacement: the content ports, that behaviour needs native code. |
| extends Block | `BlockLumabusSeed` | needs-translation | Overrides getShape, isValidBonemealTarget, canGrowRandomly, isBonemealSuccess, grow, performBonemeal, canSurvive, updateShape, randomTick: the content ports, that behaviour needs native code. |
| extends RotatedPillarBlock | `BlockMoltenGloomwoodLog` | needs-translation | No mapping for this superclass yet. |
| extends Block | `BlockMushroomFir` | needs-translation | Overrides createBlockStateDefinition, getShape, getCloneItemStack, canSurvive, updateShape, tick: the content ports, that behaviour needs native code. |
| extends Block | `BlockNetherCactus` | needs-translation | Overrides createBlockStateDefinition, getShape, updateShape, canSurvive, randomTick, entityInside: the content ports, that behaviour needs native code. |
| extends AbstractFurnaceBlock | `BlockNetherFurnace` | needs-translation | No mapping for this superclass yet. |
| extends Block | `BlockNetherMycelium` | needs-translation | Overrides setVegetationFeature, createBlockStateDefinition, animateTick, isValidBonemealTarget, getHostBlock, getCoverFeature: the content ports, that behaviour needs native code. |
| extends Block | `BlockNetherReed` | needs-translation | Overrides createBlockStateDefinition, getShadeBrightness, updateShape, canSurvive, randomTick: the content ports, that behaviour needs native code. |
| extends Block | `BlockObsidianGlass` | needs-translation | Overrides getShadeBrightness, propagatesSkylightDown, skipRendering: the content ports, that behaviour needs native code. |
| extends DropExperienceBlock | `BlockOre` | needs-translation | No mapping for this superclass yet. |
| extends Block | `BlockPottedPlant` | needs-translation | Overrides getLuminance, getShape, propagatesSkylightDown, getCollisionShape, getBlockSupportShape, createBlockStateDefinition, canSurvive, updateShape, getPlant: the content ports, that behaviour needs native code. |
| extends Block | `BlockRedLargeMushroom` | needs-translation | Overrides createBlockStateDefinition, getShape, getCloneItemStack, updateShape: the content ports, that behaviour needs native code. |
| extends Block | `BlockRubeusCone` | needs-translation | Overrides getShape, canSurvive, updateShape: the content ports, that behaviour needs native code. |
| extends TintedParticleLeavesBlock | `BlockRubeusLeaves` | needs-translation | No mapping for this superclass yet. |
| extends Block | `BlockSmallLantern` | needs-translation | Overrides createBlockStateDefinition, getShape, canSurvive, rotate, mirror, updateShape, getStateForPlacement: the content ports, that behaviour needs native code. |
| extends Block | `BlockSmoker` | needs-translation | Overrides createBlockStateDefinition, animateTick, getShape, updateShape, canSurvive: the content ports, that behaviour needs native code. |
| extends Block | `BlockSoulLily` | needs-translation | Overrides createBlockStateDefinition, getShape, canGrow, randomTick, isAirSides, rotate, mirror, getRotationIndex, rotOffset, getCloneItemStack, canSurvive, updateShape: the content ports, that behaviour needs native code. |
| extends Block | `BlockSoulSandstone` | needs-translation | Overrides createBlockStateDefinition, updateShape, getStateForPlacement: the content ports, that behaviour needs native code. |
| extends Block | `BlockSoulVein` | needs-translation | Overrides getShape, canSurvive, updateShape, isValidBonemealTarget, isBonemealSuccess, performBonemeal, setPlacedBy: the content ports, that behaviour needs native code. |
| extends Block | `BlockStalactite` | needs-translation | Overrides createBlockStateDefinition, getShape, setPlacedBy, destroy, canSurvive, canPlace: the content ports, that behaviour needs native code. |
| extends Block | `BlockStalagnate` | needs-translation | Overrides getShape, getCollisionShape, createBlockStateDefinition, getCloneItemStack: the content ports, that behaviour needs native code. |
| extends Block | `BlockStalagnateBowl` | needs-translation | Overrides createBlockStateDefinition, getShape, canSurvive, updateShape: the content ports, that behaviour needs native code. |
| extends Block | `BlockStatueRespawner` | needs-translation | Overrides createBlockStateDefinition, getStateForPlacement, getShape, getOcclusionShape, useWithoutItem, getHelpMessage, canSurvive, setPlacedBy, updateShape, rotate, mirror, playerWillDestroy: the content ports, that behaviour needs native code. |
| extends Block | `BlockStem` | needs-translation | Overrides getShape, getCollisionShape, rotate, createBlockStateDefinition, getStateForPlacement: the content ports, that behaviour needs native code. |
| extends Block | `BlockTerrain` | needs-translation | Overrides setVegetationFeature, isValidBonemealTarget, getHostBlock, getCoverFeature: the content ports, that behaviour needs native code. |
| extends Block | `BlockVeinedSand` | needs-translation | Overrides updateShape, getCloneItemStack: the content ports, that behaviour needs native code. |
| extends Block | `BlockWartRoots` | content | A plain block becomes a content-mod block copying a vanilla template. Overridden behaviour methods need native code. |
| extends Block | `BlockWartSeed` | needs-translation | Overrides createBlockStateDefinition, getShape, getStateForPlacement, canSurvive, isValidBonemealTarget, isBonemealSuccess, performBonemeal, rotate, mirror, updateShape: the content ports, that behaviour needs native code. |
| extends Block | `BlockWhisperingGourdLantern` | needs-translation | Overrides createBlockStateDefinition, rotate, mirror, getStateForPlacement: the content ports, that behaviour needs native code. |
| extends Block | `BlockWillowBranch` | needs-translation | Overrides getLuminance, getShadeBrightness, createBlockStateDefinition, getShape, updateShape, getCloneItemStack: the content ports, that behaviour needs native code. |
| extends Block | `BlockWillowTorch` | needs-translation | Overrides createBlockStateDefinition, getShape, canSurvive, rotate, mirror, updateShape, getStateForPlacement: the content ports, that behaviour needs native code. |
| extends Block | `BlockWillowTrunk` | needs-translation | Overrides createBlockStateDefinition, getShape: the content ports, that behaviour needs native code. |
| extends RedStoneOreBlock | `RedstoneOreBlock` | needs-translation | No mapping for this superclass yet. |
| extends RotatedPillarBlock | `RubeusBark` | needs-translation | No mapping for this superclass yet. |
| extends RotatedPillarBlock | `RubeusLog` | needs-translation | No mapping for this superclass yet. |
| extends FlyingPathNavigation | `EntityFirefly$1` | needs-translation | No mapping for this superclass yet. |
| extends LookControl | `EntityFirefly$FreflyLookControl` | needs-translation | No mapping for this superclass yet. |
| extends Goal | `EntityFirefly$MoveRandomGoal` | needs-translation | No mapping for this superclass yet. |
| extends Goal | `EntityFirefly$MoveToFlowersGoal` | needs-translation | No mapping for this superclass yet. |
| extends Goal | `EntityFirefly$SittingGoal` | needs-translation | No mapping for this superclass yet. |
| extends Goal | `EntityFirefly$WanderAroundGoal` | needs-translation | No mapping for this superclass yet. |
| extends FlyingPathNavigation | `EntityFlyingPig$1` | needs-translation | No mapping for this superclass yet. |
| extends Goal | `EntityFlyingPig$FindFoodGoal` | needs-translation | No mapping for this superclass yet. |
| extends Goal | `EntityFlyingPig$RoostingGoal` | needs-translation | No mapping for this superclass yet. |
| extends Goal | `EntityFlyingPig$SittingGoal` | needs-translation | No mapping for this superclass yet. |
| extends Goal | `EntityFlyingPig$WanderAroundGoal` | needs-translation | No mapping for this superclass yet. |
| extends Skeleton | `EntityJungleSkeleton` | needs-translation | No mapping for this superclass yet. |
| extends Monster | `EntityNaga` | needs-translation | No mapping for this superclass yet. |
| extends Mob | `EntityNagaProjectile` | unsupported | New entities aren't supported. |
| extends LookControl | `EntitySkull$SkullLookControl` | needs-translation | No mapping for this superclass yet. |
| extends Monster | `EntitySkull` | needs-translation | No mapping for this superclass yet. |
| extends EntityModel | `ModelEntityFirefly` | needs-translation | No mapping for this superclass yet. |
| extends EntityModel | `ModelEntityFlyingPig` | needs-translation | No mapping for this superclass yet. |
| extends EntityModel | `ModelEntityHydrogenJellyfish` | needs-translation | No mapping for this superclass yet. |
| extends SkeletonModel | `ModelJungleSkeleton` | needs-translation | No mapping for this superclass yet. |
| extends EntityModel | `ModelNaga` | needs-translation | No mapping for this superclass yet. |
| extends EntityModel | `ModelSkull` | needs-translation | No mapping for this superclass yet. |
| extends RenderLayer | `FireflyGlowFeatureRenderer` | needs-translation | No mapping for this superclass yet. |
| extends LivingEntityRenderState | `FireflyRenderState` | needs-translation | No mapping for this superclass yet. |
| extends LivingEntityRenderState | `FlyingPigRenderState` | needs-translation | No mapping for this superclass yet. |
| extends LivingEntityRenderState | `HydrogenJellyfishRenderState` | needs-translation | No mapping for this superclass yet. |
| extends SkeletonRenderState | `JungleSkeletonRenderState` | needs-translation | No mapping for this superclass yet. |
| extends LivingEntityRenderState | `NagaRenderState` | needs-translation | No mapping for this superclass yet. |
| extends MobRenderer | `RenderFirefly` | needs-translation | No mapping for this superclass yet. |
| extends MobRenderer | `RenderFlyingPig` | needs-translation | No mapping for this superclass yet. |
| extends MobRenderer | `RenderHydrogenJellyfish` | needs-translation | No mapping for this superclass yet. |
| extends AbstractSkeletonRenderer | `RenderJungleSkeleton` | needs-translation | No mapping for this superclass yet. |
| extends MobRenderer | `RenderNaga` | needs-translation | No mapping for this superclass yet. |
| extends EntityRenderer | `RenderNagaProjectile` | needs-translation | No mapping for this superclass yet. |
| extends EyesLayer | `RenderSkull$GlowFeatureRenderer` | needs-translation | No mapping for this superclass yet. |
| extends MobRenderer | `RenderSkull` | needs-translation | No mapping for this superclass yet. |
| extends LivingEntityRenderState | `SkullRenderState` | needs-translation | No mapping for this superclass yet. |
| extends Item | `ItemBlackApple` | needs-translation | Overrides finishUsingItem: the content ports, that behaviour needs native code. |
| extends Item | `ItemBowlFood` | needs-translation | Overrides useOn, finishUsingItem: the content ports, that behaviour needs native code. |
| extends Item | `ItemBowlMedicine` | needs-translation | Overrides finishUsingItem: the content ports, that behaviour needs native code. |
| extends Item | `NetherArmor` | needs-translation | Overrides setupItemStack: the content ports, that behaviour needs native code. |
| extends Item | `NetherAxe` | needs-translation | Overrides setupItemStack: the content ports, that behaviour needs native code. |
| extends Item | `NetherHoe` | needs-translation | Overrides setupItemStack: the content ports, that behaviour needs native code. |
| extends Item | `NetherPickaxe` | needs-translation | Overrides setupItemStack: the content ports, that behaviour needs native code. |
| extends Item | `NetherShovel` | needs-translation | Overrides setupItemStack: the content ports, that behaviour needs native code. |
| extends Item | `NetherSpear` | needs-translation | Overrides setupItemStack: the content ports, that behaviour needs native code. |
| extends Item | `NetherSword` | needs-translation | Overrides setupItemStack: the content ports, that behaviour needs native code. |
| extends DripHangParticle | `BNDripHangParticle` | needs-translation | No mapping for this superclass yet. |
| extends DripLandParticle | `BNDripLandParticle` | needs-translation | No mapping for this superclass yet. |
| extends FallAndLandParticle | `BNFallAndLandParticle` | needs-translation | No mapping for this superclass yet. |
| extends Structure | `CityStructure` | needs-translation | No mapping for this superclass yet. |
| extends Structure | `MegaLavaLakeStructure` | needs-translation | No mapping for this superclass yet. |
| extends StructurePiece | `CustomPiece` | needs-translation | No mapping for this superclass yet. |

## Access wideners

| File | In | Status | Note |
|---|---|---|---|
| access widener (opens private game fields) | `betternether.accesswidener` | needs-translation | Native helpers already see the game's fields, once each field is mapped. |

## Content

| What | In | Status | Note |
|---|---|---|---|
| registers blocks or items in code | `BNCriterion` | content | Registration and properties: horizon-port-pack carries them as a content mod. |
| registers blocks or items in code | `BlockEntitiesRegistry` | content | Registration and properties: horizon-port-pack carries them as a content mod. |
| registers blocks or items in code | `NetherBiomes` | content | Registration and properties: horizon-port-pack carries them as a content mod. |
| registers blocks or items in code | `NetherEntities` | content | Registration and properties: horizon-port-pack carries them as a content mod. |
| registers blocks or items in code | `NetherFeatures` | content | Registration and properties: horizon-port-pack carries them as a content mod. |
| registers blocks or items in code | `SoundsRegistry` | content | Registration and properties: horizon-port-pack carries them as a content mod. |
| 5448 asset files, 3520 data files | `jar resources` | content | Recipes, loot, tags, models, textures, worldgen: horizon-port-pack carries these. |

## Game methods and fields used

61 of 1584 are mapped to Horizon. Every Java body that uses an unmapped one needs it mapped before its Rust can be written.

<details><summary>Mapped</summary>

| Java | Horizon |
|---|---|
| `net/minecraft/client/Minecraft#getInstance` | `self` |
| `net/minecraft/core/Direction#getOpposite` | horizon_client::mobs::shulker::opposite (suggested) |
| `net/minecraft/core/Registry#register` | content |
| `net/minecraft/core/Registry#registerForHolder` | content |
| `net/minecraft/core/registries/BuiltInRegistries#BLOCK` | content |
| `net/minecraft/core/registries/BuiltInRegistries#ITEM` | content |
| `net/minecraft/network/chat/MutableComponent#append` | horizon_client::game::dedicated::append (suggested) |
| `net/minecraft/server/level/ServerLevel#destroyBlock` | horizon_world::level::update::Sim<'_, L>::destroy_block (suggested) |
| `net/minecraft/server/level/ServerLevel#getMaxY` | horizon_client::game::Dim::max_y (suggested) |
| `net/minecraft/server/level/ServerLevel#getMinY` | horizon_client::game::Dim::min_y (suggested) |
| `net/minecraft/server/level/ServerLevel#scheduleTick` | horizon_world::level::Behaviours::schedule_tick (suggested) |
| `net/minecraft/server/level/ServerLevel#sendParticles` | horizon_client::game::fishing::Game<'_>::send_particles (suggested) |
| `net/minecraft/server/level/ServerLevel#setBlock` | horizon_world::level::Behaviours::set_block (suggested) |
| `net/minecraft/server/network/ServerGamePacketListenerImpl#teleport` | horizon_client::lan::hub::Hub::teleport (suggested) |
| `net/minecraft/util/Mth#atan2` | horizon_core::geometry::trig::atan2 (suggested) |
| `net/minecraft/util/Mth#ceil` | horizon_client_render::hud_layout::ceil (suggested) |
| `net/minecraft/util/Mth#clamp` | horizon_client_render::chat::clamp (suggested) |
| `net/minecraft/util/Mth#floor` | horizon_client_render::gfx::occlusion::floor (suggested) |
| `net/minecraft/util/Mth#frac` | horizon_client_render::border_render::frac (suggested) |
| `net/minecraft/util/Mth#lerp` | horizon_world::stepped_interpolation::lerp (suggested) |
| `net/minecraft/util/Mth#wrapDegrees` | horizon_world::stepped_interpolation::wrap_degrees (suggested) |
| `net/minecraft/world/Containers#dropItemStack` | horizon_world::level::removal::drop_item_stack (suggested) |
| `net/minecraft/world/entity/Entity#getX` | `self.native_player()?.x` |
| `net/minecraft/world/entity/Entity#getZ` | `self.native_player()?.z` |
| `net/minecraft/world/entity/Entity#hurt` | horizon_world::living_fall::hurt (suggested) |
| `net/minecraft/world/entity/Entity#igniteForSeconds` | horizon_client::arrows::ArrowState::ignite_for_seconds (suggested) |
| `net/minecraft/world/entity/EntityType#create` | horizon_client::mobs::finalize::create (suggested) |
| `net/minecraft/world/entity/LivingEntity#hasEffect` | horizon_client::mobs::Seen::has_effect (suggested) |
| `net/minecraft/world/entity/LivingEntity#hurt` | horizon_client::mobs::Mob::hurt (suggested) |
| `net/minecraft/world/entity/player/Player#hasEffect` | horizon_client::mobs::PlayerView::has_effect (suggested) |
| `net/minecraft/world/entity/player/Player#hurt` | horizon_client::mobs::Mob::hurt (suggested) |
| `net/minecraft/world/entity/player/Player#igniteForSeconds` | horizon_client::arrows::ArrowState::ignite_for_seconds (suggested) |
| `net/minecraft/world/item/Item#<init>` | content |
| `net/minecraft/world/item/Item$Properties#durability` | content |
| `net/minecraft/world/item/Item$Properties#stacksTo` | content |
| `net/minecraft/world/item/ItemStack#getCount` | horizon_world::loot::LootStack::get_count (suggested) |
| `net/minecraft/world/item/ItemStack#hurtAndBreak` | horizon_client::game::dispenser::Game<'_>::hurt_and_break (suggested) |
| `net/minecraft/world/item/ItemStack#shrink` | horizon_client::game::dispenser::shrink (suggested) |
| `net/minecraft/world/item/ItemStackTemplate#create` | horizon_client::crafting::Template::create (suggested) |
| `net/minecraft/world/level/Level#getMinY` | horizon_client::game::Dim::min_y (suggested) |
| `net/minecraft/world/level/Level#isClientSide` | horizon_world::level::is_client_side (suggested) |
| `net/minecraft/world/level/Level#isUnobstructed` | horizon_world::level::is_unobstructed (suggested) |
| `net/minecraft/world/level/Level#setBlock` | horizon_world::level::Behaviours::set_block (suggested) |
| `net/minecraft/world/level/block/Block#<init>` | content |
| `net/minecraft/world/level/block/Block#canSupportCenter` | horizon_world::level::query::Sim<'_, L>::can_support_center (suggested) |
| `net/minecraft/world/level/block/Block#getStateForPlacement` | horizon_world::level::place::Sim<'_, L>::state_for_placement (suggested) |
| `net/minecraft/world/level/block/Block#popResource` | horizon_client::game::spill::GameLevel<'_>::pop_resource (suggested) |
| `net/minecraft/world/level/block/LiquidBlock#shouldSpreadLiquid` | horizon_world::level::fluids::Sim<'_, L>::should_spread_liquid (suggested) |
| `net/minecraft/world/level/block/state/BlockState#canBeReplaced` | horizon_world::level::place::Sim<'_, L>::can_be_replaced (suggested) |
| `net/minecraft/world/level/block/state/BlockState#canOcclude` | horizon_world::worldgen::placement::context::can_occlude (suggested) |
| `net/minecraft/world/level/block/state/BlockState#canSurvive` | horizon_world::level::survive::Sim<'_, L>::can_survive (suggested) |
| `net/minecraft/world/level/block/state/BlockState#getCollisionShape` | horizon_client_render::collisions::<SceneCollisions<'_, '_> as PistonCollisionWorld>::collision_shape (suggested) |
| `net/minecraft/world/level/block/state/BlockState#getOffset` | horizon_client_render::mesher::MeshTables::offset (suggested) |
| `net/minecraft/world/level/chunk/ChunkGenerator#getBaseColumn` | horizon_world::worldgen::generator::Pipeline<R>::base_column (suggested) |
| `net/minecraft/world/level/chunk/ChunkGenerator#getBaseHeight` | horizon_world::worldgen::generator::Pipeline<R>::base_height (suggested) |
| `net/minecraft/world/level/levelgen/structure/templatesystem/StructureTemplate#filterBlocks` | horizon_world::worldgen::structure::legacy::filter_blocks (suggested) |
| `net/minecraft/world/level/levelgen/structure/templatesystem/StructureTemplate#placeInWorld` | horizon_world::worldgen::structure::processor::place_in_world (suggested) |
| `net/minecraft/world/level/portal/PortalShape#createPortalBlocks` | horizon_world::level::portal::Sim<'_, L>::create_portal_blocks (suggested) |
| `net/minecraft/world/phys/AABB#intersects` | horizon_core::geometry::Aabb::intersects_corners (suggested) |
| `net/minecraft/world/phys/Vec3#normalize` | horizon_client::mobs::creaking::normalize (suggested) |
| `net/minecraft/world/phys/Vec3#yRot` | horizon_client::mobs::raider::y_rot (suggested) |

</details>

<details><summary>Not mapped yet</summary>

| Java | Kind | Classes using it |
|---|---|---|
| `com/mojang/blaze3d/vertex/PoseStack#<init>` | method | 1 |
| `com/mojang/blaze3d/vertex/PoseStack#last` | method | 1 |
| `com/mojang/blaze3d/vertex/PoseStack#mulPose` | method | 1 |
| `com/mojang/blaze3d/vertex/PoseStack#popPose` | method | 2 |
| `com/mojang/blaze3d/vertex/PoseStack#pushPose` | method | 2 |
| `com/mojang/blaze3d/vertex/PoseStack#rotate` | method | 2 |
| `com/mojang/blaze3d/vertex/PoseStack#scale` | method | 1 |
| `com/mojang/blaze3d/vertex/PoseStack#translate` | method | 1 |
| `com/mojang/blaze3d/vertex/PoseStack$Pose#normal` | method | 1 |
| `com/mojang/blaze3d/vertex/PoseStack$Pose#pose` | method | 1 |
| `com/mojang/blaze3d/vertex/VertexConsumer#addVertex` | method | 2 |
| `com/mojang/blaze3d/vertex/VertexConsumer#setColor` | method | 2 |
| `com/mojang/blaze3d/vertex/VertexConsumer#setLight` | method | 2 |
| `com/mojang/blaze3d/vertex/VertexConsumer#setNormal` | method | 2 |
| `com/mojang/blaze3d/vertex/VertexConsumer#setOverlay` | method | 2 |
| `com/mojang/blaze3d/vertex/VertexConsumer#setUv` | method | 2 |
| `net/minecraft/ChatFormatting#DARK_GREEN` | field | 1 |
| `net/minecraft/ChatFormatting#RED` | field | 1 |
| `net/minecraft/advancements/predicates/ItemPredicate#CODEC` | field | 1 |
| `net/minecraft/advancements/predicates/ItemPredicate#test` | method | 1 |
| `net/minecraft/advancements/predicates/ItemPredicate$Builder#build` | method | 1 |
| `net/minecraft/advancements/predicates/ItemPredicate$Builder#item` | method | 2 |
| `net/minecraft/advancements/predicates/ItemPredicate$Builder#of` | method | 2 |
| `net/minecraft/advancements/predicates/LocationPredicate$Builder#location` | method | 1 |
| `net/minecraft/advancements/predicates/StatePropertiesPredicate$Builder#hasProperty` | method | 3 |
| `net/minecraft/advancements/predicates/StatePropertiesPredicate$Builder#properties` | method | 1 |
| `net/minecraft/advancements/triggers/ChangeDimensionTrigger$TriggerInstance#changedDimensionTo` | method | 1 |
| `net/minecraft/advancements/triggers/PlayerTrigger#<init>` | method | 1 |
| `net/minecraft/advancements/triggers/PlayerTrigger#createCriterion` | method | 1 |
| `net/minecraft/advancements/triggers/PlayerTrigger#trigger` | method | 4 |
| `net/minecraft/advancements/triggers/PlayerTrigger$TriggerInstance#<init>` | method | 1 |
| `net/minecraft/advancements/triggers/PlayerTrigger$TriggerInstance#located` | method | 1 |
| `net/minecraft/advancements/triggers/SimpleCriterionTrigger#<init>` | method | 1 |
| `net/minecraft/advancements/triggers/SimpleCriterionTrigger$SimpleInstance#validate` | method | 1 |
| `net/minecraft/client/Camera#rotation` | method | 1 |
| `net/minecraft/client/Minecraft#getEntityRenderDispatcher` | method | 1 |
| `net/minecraft/client/data/models/BlockModelGenerators#X_ROT_180` | field | 1 |
| `net/minecraft/client/data/models/BlockModelGenerators#createSimpleBlock` | method | 1 |
| `net/minecraft/client/data/models/BlockModelGenerators#createSlab` | method | 1 |
| `net/minecraft/client/data/models/BlockModelGenerators#modelOutput` | field | 2 |
| `net/minecraft/client/data/models/BlockModelGenerators#plainModel` | method | 2 |
| `net/minecraft/client/data/models/BlockModelGenerators#plainVariant` | method | 2 |
| `net/minecraft/client/data/models/ItemModelGenerators#TRIM_MATERIAL_MODELS` | field | 1 |
| `net/minecraft/client/data/models/ItemModelGenerators#generateFlatItem` | method | 1 |
| `net/minecraft/client/data/models/ItemModelGenerators#generateLayeredItem` | method | 1 |
| `net/minecraft/client/data/models/ItemModelGenerators#itemModelOutput` | field | 1 |
| `net/minecraft/client/data/models/ItemModelGenerators#modelOutput` | field | 1 |
| `net/minecraft/client/data/models/ItemModelGenerators#prefixForSlotTrim` | method | 1 |
| `net/minecraft/client/data/models/ItemModelGenerators$TrimMaterialData#materialKey` | method | 1 |
| `net/minecraft/client/data/models/ItemModelGenerators$TrimMaterialData#palette` | method | 1 |
| `net/minecraft/client/data/models/ItemModelOutput#accept` | method | 1 |
| `net/minecraft/client/data/models/MultiVariant#<init>` | method | 1 |
| `net/minecraft/client/data/models/MultiVariant#with` | method | 1 |
| `net/minecraft/client/data/models/blockstates/MultiVariantGenerator#dispatch` | method | 2 |
| `net/minecraft/client/data/models/blockstates/MultiVariantGenerator$Empty#with` | method | 1 |
| `net/minecraft/client/data/models/blockstates/PropertyDispatch#initial` | method | 1 |
| `net/minecraft/client/data/models/blockstates/PropertyDispatch$C1#select` | method | 1 |
| `net/minecraft/client/data/models/model/ItemModelUtils#plainModel` | method | 1 |
| `net/minecraft/client/data/models/model/ItemModelUtils#select` | method | 1 |
| `net/minecraft/client/data/models/model/ItemModelUtils#when` | method | 1 |
| `net/minecraft/client/data/models/model/ModelLocationUtils#getModelLocation` | method | 6 |
| `net/minecraft/client/data/models/model/ModelTemplate#<init>` | method | 2 |
| `net/minecraft/client/data/models/model/ModelTemplate#create` | method | 6 |
| `net/minecraft/client/data/models/model/ModelTemplate#createWithSuffix` | method | 1 |
| `net/minecraft/client/data/models/model/ModelTemplates#CROSS` | field | 1 |
| `net/minecraft/client/data/models/model/ModelTemplates#CUBE_ALL` | field | 1 |
| `net/minecraft/client/data/models/model/ModelTemplates#CUBE_BOTTOM_TOP` | field | 1 |
| `net/minecraft/client/data/models/model/ModelTemplates#CUBE_COLUMN` | field | 1 |
| `net/minecraft/client/data/models/model/ModelTemplates#FLAT_ITEM` | field | 2 |
| `net/minecraft/client/data/models/model/ModelTemplates#SLAB_BOTTOM` | field | 1 |
| `net/minecraft/client/data/models/model/ModelTemplates#SLAB_TOP` | field | 1 |
| `net/minecraft/client/data/models/model/TextureMapping#<init>` | method | 2 |
| `net/minecraft/client/data/models/model/TextureMapping#cross` | method | 1 |
| `net/minecraft/client/data/models/model/TextureMapping#cube` | method | 1 |
| `net/minecraft/client/data/models/model/TextureMapping#getBlockTexture` | method | 2 |
| `net/minecraft/client/data/models/model/TextureMapping#getItemTexture` | method | 1 |
| `net/minecraft/client/data/models/model/TextureMapping#layer0` | method | 2 |
| `net/minecraft/client/data/models/model/TextureMapping#put` | method | 2 |
| `net/minecraft/client/data/models/model/TextureSlot#ALL` | field | 2 |
| `net/minecraft/client/data/models/model/TextureSlot#BOTTOM` | field | 2 |
| `net/minecraft/client/data/models/model/TextureSlot#CROSS` | field | 2 |
| `net/minecraft/client/data/models/model/TextureSlot#END` | field | 1 |
| `net/minecraft/client/data/models/model/TextureSlot#PARTICLE` | field | 1 |
| `net/minecraft/client/data/models/model/TextureSlot#SIDE` | field | 1 |
| `net/minecraft/client/data/models/model/TextureSlot#TEXTURE` | field | 2 |
| `net/minecraft/client/data/models/model/TextureSlot#TOP` | field | 2 |
| `net/minecraft/client/data/models/model/TexturedModel#CUBE` | field | 1 |
| `net/minecraft/client/data/models/model/TexturedModel#getTemplate` | method | 1 |
| `net/minecraft/client/data/models/model/TexturedModel$Provider#get` | method | 1 |
| `net/minecraft/client/model/EntityModel#<init>` | method | 5 |
| `net/minecraft/client/model/EntityModel#setupAnim` | method | 5 |
| `net/minecraft/client/model/geom/ModelLayerLocation#<init>` | method | 1 |
| `net/minecraft/client/model/geom/ModelLayers#PLAYER_SLIM_ARMOR` | field | 2 |
| `net/minecraft/client/model/geom/ModelLayers#SKELETON_ARMOR` | field | 1 |
| `net/minecraft/client/model/geom/ModelPart#getChild` | method | 5 |
| `net/minecraft/client/model/geom/ModelPart#loadPose` | method | 1 |
| `net/minecraft/client/model/geom/ModelPart#render` | method | 1 |
| `net/minecraft/client/model/geom/ModelPart#setPos` | method | 2 |
| `net/minecraft/client/model/geom/ModelPart#storePose` | method | 1 |
| `net/minecraft/client/model/geom/ModelPart#visible` | field | 2 |
| `net/minecraft/client/model/geom/ModelPart#xRot` | field | 4 |
| `net/minecraft/client/model/geom/ModelPart#yRot` | field | 3 |
| `net/minecraft/client/model/geom/ModelPart#zRot` | field | 2 |
| `net/minecraft/client/model/geom/PartPose#ZERO` | field | 1 |
| `net/minecraft/client/model/geom/PartPose#offset` | method | 5 |
| `net/minecraft/client/model/geom/PartPose#offsetAndRotation` | method | 3 |
| `net/minecraft/client/model/geom/builders/CubeDeformation#<init>` | method | 1 |
| `net/minecraft/client/model/geom/builders/CubeListBuilder#addBox` | method | 7 |
| `net/minecraft/client/model/geom/builders/CubeListBuilder#create` | method | 6 |
| `net/minecraft/client/model/geom/builders/CubeListBuilder#mirror` | method | 1 |
| `net/minecraft/client/model/geom/builders/CubeListBuilder#texOffs` | method | 6 |
| `net/minecraft/client/model/geom/builders/LayerDefinition#create` | method | 5 |
| `net/minecraft/client/model/geom/builders/LayerDefinition#mesh` | method | 1 |
| `net/minecraft/client/model/geom/builders/MeshDefinition#<init>` | method | 5 |
| `net/minecraft/client/model/geom/builders/MeshDefinition#getRoot` | method | 6 |
| `net/minecraft/client/model/geom/builders/PartDefinition#addOrReplaceChild` | method | 6 |
| `net/minecraft/client/model/geom/builders/PartDefinition#getChild` | method | 1 |
| `net/minecraft/client/model/monster/skeleton/SkeletonModel#<init>` | method | 1 |
| `net/minecraft/client/model/monster/skeleton/SkeletonModel#createBodyLayer` | method | 1 |
| `net/minecraft/client/model/object/armorstand/ArmorStandArmorModel#<init>` | method | 1 |
| `net/minecraft/client/particle/DripParticle$DripHangParticle#<init>` | method | 1 |
| `net/minecraft/client/particle/DripParticle$DripLandParticle#<init>` | method | 1 |
| `net/minecraft/client/particle/DripParticle$FallAndLandParticle#<init>` | method | 1 |
| `net/minecraft/client/particle/SpriteSet#get` | method | 9 |
| `net/minecraft/client/renderer/RenderPipelines#ENTITY_NO_LIGHTMAP_SNIPPET` | field | 1 |
| `net/minecraft/client/renderer/RenderPipelines#EYES_SNIPPET` | field | 1 |
| `net/minecraft/client/renderer/SubmitNodeCollector#submitCustomGeometry` | method | 2 |
| `net/minecraft/client/renderer/block/dispatch/Variant#withYRot` | method | 1 |
| `net/minecraft/client/renderer/entity/AbstractSkeletonRenderer#<init>` | method | 1 |
| `net/minecraft/client/renderer/entity/ArmorModelSet#bake` | method | 2 |
| `net/minecraft/client/renderer/entity/EntityRenderDispatcher#camera` | field | 1 |
| `net/minecraft/client/renderer/entity/EntityRenderer#<init>` | method | 1 |
| `net/minecraft/client/renderer/entity/EntityRenderer#submit` | method | 1 |
| `net/minecraft/client/renderer/entity/EntityRendererProvider$Context#bakeLayer` | method | 6 |
| `net/minecraft/client/renderer/entity/EntityRendererProvider$Context#getEquipmentRenderer` | method | 1 |
| `net/minecraft/client/renderer/entity/EntityRendererProvider$Context#getModelSet` | method | 1 |
| `net/minecraft/client/renderer/entity/LivingEntityRenderer#<init>` | method | 1 |
| `net/minecraft/client/renderer/entity/MobRenderer#<init>` | method | 5 |
| `net/minecraft/client/renderer/entity/MobRenderer#extractRenderState` | method | 4 |
| `net/minecraft/client/renderer/entity/MobRenderer#getBlockLightLevel` | method | 1 |
| `net/minecraft/client/renderer/entity/layers/EyesLayer#<init>` | method | 1 |
| `net/minecraft/client/renderer/entity/layers/HumanoidArmorLayer#<init>` | method | 1 |
| `net/minecraft/client/renderer/entity/layers/RenderLayer#<init>` | method | 1 |
| `net/minecraft/client/renderer/entity/player/AvatarRenderer#<init>` | method | 1 |
| `net/minecraft/client/renderer/entity/state/EntityRenderState#<init>` | method | 1 |
| `net/minecraft/client/renderer/entity/state/EntityRenderState#lightCoords` | field | 1 |
| `net/minecraft/client/renderer/entity/state/LivingEntityRenderState#<init>` | method | 5 |
| `net/minecraft/client/renderer/entity/state/SkeletonRenderState#<init>` | method | 1 |
| `net/minecraft/client/renderer/item/properties/select/TrimMaterialProperty#<init>` | method | 1 |
| `net/minecraft/client/renderer/rendertype/RenderSetup#builder` | method | 1 |
| `net/minecraft/client/renderer/rendertype/RenderSetup$OutlineProperty#NONE` | field | 1 |
| `net/minecraft/client/renderer/rendertype/RenderSetup$RenderSetupBuilder#createRenderSetup` | method | 1 |
| `net/minecraft/client/renderer/rendertype/RenderSetup$RenderSetupBuilder#setOutline` | method | 1 |
| `net/minecraft/client/renderer/rendertype/RenderSetup$RenderSetupBuilder#sortOnUpload` | method | 1 |
| `net/minecraft/client/renderer/rendertype/RenderSetup$RenderSetupBuilder#withTexture` | method | 1 |
| `net/minecraft/client/renderer/rendertype/RenderType#create` | method | 1 |
| `net/minecraft/client/renderer/rendertype/RenderTypes#entityCutout` | method | 1 |
| `net/minecraft/client/renderer/rendertype/RenderTypes#entityTranslucent` | method | 1 |
| `net/minecraft/client/renderer/state/level/CameraRenderState#orientation` | field | 1 |
| `net/minecraft/client/renderer/texture/OverlayTexture#NO_OVERLAY` | field | 2 |
| `net/minecraft/client/resources/model/sprite/Material#<init>` | method | 4 |
| `net/minecraft/client/resources/model/sprite/Material#sprite` | method | 2 |
| `net/minecraft/commands/CommandSourceStack#getLevel` | method | 2 |
| `net/minecraft/commands/CommandSourceStack#getPlayerOrException` | method | 1 |
| `net/minecraft/commands/CommandSourceStack#getPosition` | method | 2 |
| `net/minecraft/commands/CommandSourceStack#sendFailure` | method | 1 |
| `net/minecraft/commands/CommandSourceStack#sendSuccess` | method | 1 |
| `net/minecraft/commands/Commands#LEVEL_GAMEMASTERS` | field | 1 |
| `net/minecraft/commands/Commands#LEVEL_OWNERS` | field | 1 |
| `net/minecraft/commands/Commands#argument` | method | 2 |
| `net/minecraft/commands/Commands#hasPermission` | method | 2 |
| `net/minecraft/commands/Commands#literal` | method | 2 |
| `net/minecraft/commands/arguments/ResourceOrTagKeyArgument#getResourceOrTagKey` | method | 1 |
| `net/minecraft/commands/arguments/ResourceOrTagKeyArgument#resourceOrTagKey` | method | 1 |
| `net/minecraft/commands/arguments/ResourceOrTagKeyArgument$Result#asPrintable` | method | 1 |
| `net/minecraft/core/BlockPos#<init>` | method | 18 |
| `net/minecraft/core/BlockPos#CODEC` | field | 4 |
| `net/minecraft/core/BlockPos#ZERO` | field | 5 |
| `net/minecraft/core/BlockPos#above` | method | 55 |
| `net/minecraft/core/BlockPos#below` | method | 77 |
| `net/minecraft/core/BlockPos#distSqr` | method | 1 |
| `net/minecraft/core/BlockPos#east` | method | 17 |
| `net/minecraft/core/BlockPos#getX` | method | 64 |
| `net/minecraft/core/BlockPos#getY` | method | 58 |
| `net/minecraft/core/BlockPos#getZ` | method | 64 |
| `net/minecraft/core/BlockPos#immutable` | method | 5 |
| `net/minecraft/core/BlockPos#mutable` | method | 2 |
| `net/minecraft/core/BlockPos#north` | method | 17 |
| `net/minecraft/core/BlockPos#offset` | method | 9 |
| `net/minecraft/core/BlockPos#relative` | method | 26 |
| `net/minecraft/core/BlockPos#rotate` | method | 2 |
| `net/minecraft/core/BlockPos#south` | method | 17 |
| `net/minecraft/core/BlockPos#west` | method | 17 |
| `net/minecraft/core/BlockPos$MutableBlockPos#<init>` | method | 17 |
| `net/minecraft/core/BlockPos$MutableBlockPos#above` | method | 5 |
| `net/minecraft/core/BlockPos$MutableBlockPos#below` | method | 11 |
| `net/minecraft/core/BlockPos$MutableBlockPos#distManhattan` | method | 1 |
| `net/minecraft/core/BlockPos$MutableBlockPos#east` | method | 1 |
| `net/minecraft/core/BlockPos$MutableBlockPos#getX` | method | 6 |
| `net/minecraft/core/BlockPos$MutableBlockPos#getY` | method | 12 |
| `net/minecraft/core/BlockPos$MutableBlockPos#getZ` | method | 6 |
| `net/minecraft/core/BlockPos$MutableBlockPos#immutable` | method | 5 |
| `net/minecraft/core/BlockPos$MutableBlockPos#move` | method | 3 |
| `net/minecraft/core/BlockPos$MutableBlockPos#north` | method | 1 |
| `net/minecraft/core/BlockPos$MutableBlockPos#offset` | method | 1 |
| `net/minecraft/core/BlockPos$MutableBlockPos#relative` | method | 1 |
| `net/minecraft/core/BlockPos$MutableBlockPos#set` | method | 23 |
| `net/minecraft/core/BlockPos$MutableBlockPos#setWithOffset` | method | 3 |
| `net/minecraft/core/BlockPos$MutableBlockPos#setX` | method | 15 |
| `net/minecraft/core/BlockPos$MutableBlockPos#setY` | method | 21 |
| `net/minecraft/core/BlockPos$MutableBlockPos#setZ` | method | 15 |
| `net/minecraft/core/BlockPos$MutableBlockPos#south` | method | 1 |
| `net/minecraft/core/BlockPos$MutableBlockPos#west` | method | 1 |
| `net/minecraft/core/DefaultedRegistry#containsKey` | method | 1 |
| `net/minecraft/core/DefaultedRegistry#getKey` | method | 9 |
| `net/minecraft/core/DefaultedRegistry#getOptional` | method | 1 |
| `net/minecraft/core/DefaultedRegistry#getValue` | method | 1 |
| `net/minecraft/core/Direction#DOWN` | field | 29 |
| `net/minecraft/core/Direction#EAST` | field | 21 |
| `net/minecraft/core/Direction#NORTH` | field | 22 |
| `net/minecraft/core/Direction#SOUTH` | field | 21 |
| `net/minecraft/core/Direction#UP` | field | 35 |
| `net/minecraft/core/Direction#WEST` | field | 21 |
| `net/minecraft/core/Direction#fromAxisAndDirection` | method | 1 |
| `net/minecraft/core/Direction#getAxis` | method | 5 |
| `net/minecraft/core/Direction#getClockWise` | method | 2 |
| `net/minecraft/core/Direction#getCounterClockWise` | method | 3 |
| `net/minecraft/core/Direction#getRandom` | method | 3 |
| `net/minecraft/core/Direction#getStepX` | method | 7 |
| `net/minecraft/core/Direction#getStepY` | method | 4 |
| `net/minecraft/core/Direction#getStepZ` | method | 7 |
| `net/minecraft/core/Direction#getUnitVec3i` | method | 1 |
| `net/minecraft/core/Direction#ordinal` | method | 8 |
| `net/minecraft/core/Direction#values` | method | 8 |
| `net/minecraft/core/Direction$Axis#X` | field | 7 |
| `net/minecraft/core/Direction$Axis#Y` | field | 6 |
| `net/minecraft/core/Direction$Axis#Z` | field | 7 |
| `net/minecraft/core/Direction$Axis#equals` | method | 1 |
| `net/minecraft/core/Direction$Axis#isHorizontal` | method | 2 |
| `net/minecraft/core/Direction$Axis#ordinal` | method | 2 |
| `net/minecraft/core/Direction$Axis#values` | method | 1 |
| `net/minecraft/core/Direction$AxisDirection#NEGATIVE` | field | 1 |
| `net/minecraft/core/Direction$AxisDirection#POSITIVE` | field | 1 |
| `net/minecraft/core/Direction$Plane#HORIZONTAL` | field | 2 |
| `net/minecraft/core/Direction$Plane#getRandomDirection` | method | 1 |
| `net/minecraft/core/Direction$Plane#stream` | method | 1 |
| `net/minecraft/core/Holder#is` | method | 1 |
| `net/minecraft/core/Holder#unwrapKey` | method | 2 |
| `net/minecraft/core/Holder#value` | method | 13 |
| `net/minecraft/core/Holder$Reference#is` | method | 1 |
| `net/minecraft/core/Holder$Reference#value` | method | 1 |
| `net/minecraft/core/HolderGetter#get` | method | 1 |
| `net/minecraft/core/HolderGetter#getOrThrow` | method | 4 |
| `net/minecraft/core/HolderLookup$Provider#lookup` | method | 1 |
| `net/minecraft/core/HolderLookup$Provider#lookupOrThrow` | method | 3 |
| `net/minecraft/core/HolderLookup$RegistryLookup#get` | method | 2 |
| `net/minecraft/core/HolderLookup$RegistryLookup#getOrThrow` | method | 1 |
| `net/minecraft/core/HolderLookup$RegistryLookup#listElementIds` | method | 1 |
| `net/minecraft/core/HolderSet#contains` | method | 1 |
| `net/minecraft/core/HolderSet#empty` | method | 1 |
| `net/minecraft/core/HolderSet#get` | method | 1 |
| `net/minecraft/core/HolderSet#size` | method | 1 |
| `net/minecraft/core/HolderSet$Named#stream` | method | 1 |
| `net/minecraft/core/MappedRegistry#freeze` | method | 1 |
| `net/minecraft/core/NonNullList#clear` | method | 1 |
| `net/minecraft/core/NonNullList#get` | method | 1 |
| `net/minecraft/core/NonNullList#iterator` | method | 2 |
| `net/minecraft/core/NonNullList#set` | method | 1 |
| `net/minecraft/core/NonNullList#size` | method | 1 |
| `net/minecraft/core/NonNullList#withSize` | method | 2 |
| `net/minecraft/core/Registry#get` | method | 2 |
| `net/minecraft/core/Registry#getResourceKey` | method | 1 |
| `net/minecraft/core/Registry#getValue` | method | 1 |
| `net/minecraft/core/RegistryAccess#lookupOrThrow` | method | 5 |
| `net/minecraft/core/SectionPos#blockToSectionCoord` | method | 2 |
| `net/minecraft/core/Vec3i#<init>` | method | 2 |
| `net/minecraft/core/Vec3i#getX` | method | 4 |
| `net/minecraft/core/Vec3i#getY` | method | 4 |
| `net/minecraft/core/Vec3i#getZ` | method | 4 |
| `net/minecraft/core/component/DataComponentPatch#builder` | method | 1 |
| `net/minecraft/core/component/DataComponentPatch$Builder#build` | method | 1 |
| `net/minecraft/core/component/DataComponentPatch$Builder#set` | method | 1 |
| `net/minecraft/core/component/DataComponents#FOOD` | field | 1 |
| `net/minecraft/core/component/DataComponents#POTION_CONTENTS` | field | 1 |
| `net/minecraft/core/component/DataComponents#TOOL` | field | 1 |
| `net/minecraft/core/component/predicates/PotionsPredicate#ofPotion` | method | 1 |
| `net/minecraft/core/particles/BlockParticleOption#<init>` | method | 2 |
| `net/minecraft/core/particles/ColorParticleOption#create` | method | 2 |
| `net/minecraft/core/particles/DustParticleOptions#<init>` | method | 1 |
| `net/minecraft/core/particles/ItemParticleOption#<init>` | method | 1 |
| `net/minecraft/core/particles/ParticleTypes#ASH` | field | 3 |
| `net/minecraft/core/particles/ParticleTypes#BLOCK` | field | 1 |
| `net/minecraft/core/particles/ParticleTypes#CRIMSON_SPORE` | field | 3 |
| `net/minecraft/core/particles/ParticleTypes#DRIPPING_LAVA` | field | 1 |
| `net/minecraft/core/particles/ParticleTypes#DRIPPING_WATER` | field | 2 |
| `net/minecraft/core/particles/ParticleTypes#ENTITY_EFFECT` | field | 2 |
| `net/minecraft/core/particles/ParticleTypes#EXPLOSION` | field | 1 |
| `net/minecraft/core/particles/ParticleTypes#FALLING_DUST` | field | 1 |
| `net/minecraft/core/particles/ParticleTypes#FLAME` | field | 4 |
| `net/minecraft/core/particles/ParticleTypes#ITEM` | field | 1 |
| `net/minecraft/core/particles/ParticleTypes#LARGE_SMOKE` | field | 5 |
| `net/minecraft/core/particles/ParticleTypes#LAVA` | field | 3 |
| `net/minecraft/core/particles/ParticleTypes#MYCELIUM` | field | 4 |
| `net/minecraft/core/particles/ParticleTypes#PORTAL` | field | 2 |
| `net/minecraft/core/particles/ParticleTypes#SCULK_SOUL` | field | 1 |
| `net/minecraft/core/particles/ParticleTypes#SMALL_FLAME` | field | 2 |
| `net/minecraft/core/particles/ParticleTypes#SMOKE` | field | 5 |
| `net/minecraft/core/particles/ParticleTypes#WARPED_SPORE` | field | 3 |
| `net/minecraft/core/particles/ParticleTypes#WHITE_ASH` | field | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#BLOCK_ENTITY_TYPE` | field | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#ENTITY_TYPE` | field | 2 |
| `net/minecraft/core/registries/BuiltInRegistries#PLACEMENT_MODIFIER_TYPE` | field | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#SOUND_EVENT` | field | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#TRIGGER_TYPES` | field | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#acquireBootstrapRegistrationLookup` | method | 1 |
| `net/minecraft/core/registries/Registries#BIOME` | field | 4 |
| `net/minecraft/core/registries/Registries#BLOCK` | field | 1 |
| `net/minecraft/core/registries/Registries#DAMAGE_TYPE` | field | 1 |
| `net/minecraft/core/registries/Registries#ENCHANTMENT` | field | 4 |
| `net/minecraft/core/registries/Registries#ENTITY_TYPE` | field | 1 |
| `net/minecraft/core/registries/Registries#FLAT_LEVEL_GENERATOR_PRESET` | field | 1 |
| `net/minecraft/core/registries/Registries#ITEM` | field | 2 |
| `net/minecraft/core/registries/Registries#LOOT_TABLE` | field | 1 |
| `net/minecraft/core/registries/Registries#RECIPE` | field | 1 |
| `net/minecraft/core/registries/Registries#STRUCTURE` | field | 4 |
| `net/minecraft/core/registries/Registries#STRUCTURE_SET` | field | 1 |
| `net/minecraft/data/recipes/RecipeCategory#BREWING` | field | 2 |
| `net/minecraft/data/recipes/RecipeCategory#BUILDING_BLOCKS` | field | 4 |
| `net/minecraft/data/recipes/RecipeCategory#COMBAT` | field | 2 |
| `net/minecraft/data/recipes/RecipeCategory#DECORATIONS` | field | 5 |
| `net/minecraft/data/recipes/RecipeCategory#FOOD` | field | 1 |
| `net/minecraft/data/recipes/RecipeCategory#MISC` | field | 2 |
| `net/minecraft/data/recipes/RecipeCategory#REDSTONE` | field | 3 |
| `net/minecraft/data/recipes/RecipeCategory#TOOLS` | field | 3 |
| `net/minecraft/data/recipes/RecipeCategory#TRANSPORTATION` | field | 3 |
| `net/minecraft/data/recipes/RecipeCategory#valueOf` | method | 1 |
| `net/minecraft/data/recipes/RecipeOutput#accept` | method | 1 |
| `net/minecraft/data/worldgen/BootstrapContext#lookup` | method | 2 |
| `net/minecraft/data/worldgen/BootstrapContext#register` | method | 1 |
| `net/minecraft/data/worldgen/Carvers#NETHER_CAVE` | field | 1 |
| `net/minecraft/data/worldgen/placement/MiscOverworldPlacements#SPRING_LAVA` | field | 1 |
| `net/minecraft/data/worldgen/placement/NetherPlacements#GLOWSTONE` | field | 1 |
| `net/minecraft/data/worldgen/placement/NetherPlacements#GLOWSTONE_EXTRA` | field | 1 |
| `net/minecraft/data/worldgen/placement/NetherPlacements#PATCH_FIRE` | field | 1 |
| `net/minecraft/data/worldgen/placement/NetherPlacements#PATCH_SOUL_FIRE` | field | 1 |
| `net/minecraft/data/worldgen/placement/NetherPlacements#SPRING_CLOSED` | field | 1 |
| `net/minecraft/data/worldgen/placement/NetherPlacements#SPRING_OPEN` | field | 1 |
| `net/minecraft/data/worldgen/placement/OrePlacements#ORE_MAGMA` | field | 1 |
| `net/minecraft/data/worldgen/placement/PlacementUtils#RANGE_10_10` | field | 1 |
| `net/minecraft/data/worldgen/placement/VegetationPlacements#BROWN_MUSHROOM_NETHER` | field | 1 |
| `net/minecraft/data/worldgen/placement/VegetationPlacements#RED_MUSHROOM_NETHER` | field | 1 |
| `net/minecraft/nbt/CompoundTag#<init>` | method | 4 |
| `net/minecraft/nbt/CompoundTag#get` | method | 1 |
| `net/minecraft/nbt/CompoundTag#getCompound` | method | 3 |
| `net/minecraft/nbt/CompoundTag#getIntOr` | method | 5 |
| `net/minecraft/nbt/CompoundTag#getList` | method | 2 |
| `net/minecraft/nbt/CompoundTag#getString` | method | 1 |
| `net/minecraft/nbt/CompoundTag#getStringOr` | method | 4 |
| `net/minecraft/nbt/CompoundTag#isEmpty` | method | 1 |
| `net/minecraft/nbt/CompoundTag#put` | method | 4 |
| `net/minecraft/nbt/CompoundTag#putBoolean` | method | 1 |
| `net/minecraft/nbt/CompoundTag#putInt` | method | 6 |
| `net/minecraft/nbt/CompoundTag#putShort` | method | 1 |
| `net/minecraft/nbt/CompoundTag#putString` | method | 5 |
| `net/minecraft/nbt/CompoundTag#read` | method | 4 |
| `net/minecraft/nbt/CompoundTag#remove` | method | 2 |
| `net/minecraft/nbt/CompoundTag#store` | method | 4 |
| `net/minecraft/nbt/FloatTag#valueOf` | method | 2 |
| `net/minecraft/nbt/ListTag#<init>` | method | 1 |
| `net/minecraft/nbt/ListTag#add` | method | 1 |
| `net/minecraft/nbt/ListTag#forEach` | method | 3 |
| `net/minecraft/nbt/ListTag#get` | method | 1 |
| `net/minecraft/nbt/ListTag#set` | method | 1 |
| `net/minecraft/nbt/ListTag#size` | method | 1 |
| `net/minecraft/nbt/NumericTag#floatValue` | method | 1 |
| `net/minecraft/nbt/NumericTag#intValue` | method | 1 |
| `net/minecraft/network/chat/Component#literal` | method | 3 |
| `net/minecraft/network/chat/Component#translatable` | method | 7 |
| `net/minecraft/network/chat/MutableComponent#setStyle` | method | 1 |
| `net/minecraft/network/chat/Style#EMPTY` | field | 1 |
| `net/minecraft/network/chat/Style#withColor` | method | 1 |
| `net/minecraft/network/protocol/game/ClientboundLevelParticlesPacket#<init>` | method | 1 |
| `net/minecraft/network/syncher/EntityDataSerializers#BYTE` | field | 1 |
| `net/minecraft/network/syncher/EntityDataSerializers#INT` | field | 1 |
| `net/minecraft/network/syncher/SynchedEntityData#defineId` | method | 2 |
| `net/minecraft/network/syncher/SynchedEntityData#get` | method | 2 |
| `net/minecraft/network/syncher/SynchedEntityData#set` | method | 2 |
| `net/minecraft/network/syncher/SynchedEntityData$Builder#define` | method | 2 |
| `net/minecraft/resources/Identifier#CODEC` | field | 3 |
| `net/minecraft/resources/Identifier#equals` | method | 3 |
| `net/minecraft/resources/Identifier#fromNamespaceAndPath` | method | 2 |
| `net/minecraft/resources/Identifier#getNamespace` | method | 4 |
| `net/minecraft/resources/Identifier#getPath` | method | 11 |
| `net/minecraft/resources/Identifier#toString` | method | 3 |
| `net/minecraft/resources/Identifier#withDefaultNamespace` | method | 6 |
| `net/minecraft/resources/Identifier#withSuffix` | method | 2 |
| `net/minecraft/resources/ResourceKey#create` | method | 5 |
| `net/minecraft/resources/ResourceKey#identifier` | method | 7 |
| `net/minecraft/resources/ResourceKey#toString` | method | 1 |
| `net/minecraft/server/MinecraftServer#getGameRules` | method | 1 |
| `net/minecraft/server/MinecraftServer#getLevel` | method | 1 |
| `net/minecraft/server/commands/LocateCommand#getHolders` | method | 1 |
| `net/minecraft/server/commands/LocateCommand#showLocateResult` | method | 1 |
| `net/minecraft/server/level/ServerChunkCache#getGenerator` | method | 2 |
| `net/minecraft/server/level/ServerLevel#addFreshEntity` | method | 1 |
| `net/minecraft/server/level/ServerLevel#findClosestBiome3d` | method | 1 |
| `net/minecraft/server/level/ServerLevel#getBlockEntity` | method | 1 |
| `net/minecraft/server/level/ServerLevel#getBlockState` | method | 12 |
| `net/minecraft/server/level/ServerLevel#getChunkSource` | method | 2 |
| `net/minecraft/server/level/ServerLevel#getGameRules` | method | 5 |
| `net/minecraft/server/level/ServerLevel#getNearestPlayer` | method | 1 |
| `net/minecraft/server/level/ServerLevel#getRandom` | method | 2 |
| `net/minecraft/server/level/ServerLevel#getServer` | method | 1 |
| `net/minecraft/server/level/ServerLevel#isEmptyBlock` | method | 4 |
| `net/minecraft/server/level/ServerLevel#recipeAccess` | method | 1 |
| `net/minecraft/server/level/ServerLevel#registryAccess` | method | 1 |
| `net/minecraft/server/level/ServerLevel#setBlockAndUpdate` | method | 5 |
| `net/minecraft/server/level/ServerPlayer#connection` | field | 2 |
| `net/minecraft/server/level/ServerPlayer#distanceToSqr` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#findRespawnPositionAndUseSpawnBlock` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#getRespawnConfig` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#level` | method | 2 |
| `net/minecraft/server/level/ServerPlayer#setRespawnPosition` | method | 1 |
| `net/minecraft/server/level/ServerPlayer$RespawnConfig#<init>` | method | 1 |
| `net/minecraft/server/level/ServerPlayer$RespawnConfig#respawnData` | method | 1 |
| `net/minecraft/server/level/WorldGenRegion#hasChunk` | method | 1 |
| `net/minecraft/server/network/ServerGamePacketListenerImpl#send` | method | 1 |
| `net/minecraft/sounds/SoundEvent#createVariableRangeEvent` | method | 1 |
| `net/minecraft/sounds/SoundEvents#AMBIENT_BASALT_DELTAS_ADDITIONS` | field | 1 |
| `net/minecraft/sounds/SoundEvents#AMBIENT_BASALT_DELTAS_LOOP` | field | 1 |
| `net/minecraft/sounds/SoundEvents#AMBIENT_BASALT_DELTAS_MOOD` | field | 1 |
| `net/minecraft/sounds/SoundEvents#AMBIENT_CRIMSON_FOREST_ADDITIONS` | field | 17 |
| `net/minecraft/sounds/SoundEvents#AMBIENT_CRIMSON_FOREST_LOOP` | field | 12 |
| `net/minecraft/sounds/SoundEvents#AMBIENT_CRIMSON_FOREST_MOOD` | field | 17 |
| `net/minecraft/sounds/SoundEvents#AMBIENT_NETHER_WASTES_ADDITIONS` | field | 3 |
| `net/minecraft/sounds/SoundEvents#AMBIENT_NETHER_WASTES_LOOP` | field | 2 |
| `net/minecraft/sounds/SoundEvents#AMBIENT_NETHER_WASTES_MOOD` | field | 3 |
| `net/minecraft/sounds/SoundEvents#AMBIENT_SOUL_SAND_VALLEY_ADDITIONS` | field | 1 |
| `net/minecraft/sounds/SoundEvents#AMBIENT_SOUL_SAND_VALLEY_LOOP` | field | 1 |
| `net/minecraft/sounds/SoundEvents#AMBIENT_SOUL_SAND_VALLEY_MOOD` | field | 1 |
| `net/minecraft/sounds/SoundEvents#AMBIENT_WARPED_FOREST_ADDITIONS` | field | 1 |
| `net/minecraft/sounds/SoundEvents#AMBIENT_WARPED_FOREST_LOOP` | field | 1 |
| `net/minecraft/sounds/SoundEvents#AMBIENT_WARPED_FOREST_MOOD` | field | 1 |
| `net/minecraft/sounds/SoundEvents#ARMOR_EQUIP_DIAMOND` | field | 1 |
| `net/minecraft/sounds/SoundEvents#ARMOR_EQUIP_GOLD` | field | 1 |
| `net/minecraft/sounds/SoundEvents#ARMOR_EQUIP_NETHERITE` | field | 1 |
| `net/minecraft/sounds/SoundEvents#BARREL_OPEN` | field | 1 |
| `net/minecraft/sounds/SoundEvents#CROP_PLANTED` | field | 1 |
| `net/minecraft/sounds/SoundEvents#FIRE_AMBIENT` | field | 2 |
| `net/minecraft/sounds/SoundEvents#FIRE_EXTINGUISH` | field | 1 |
| `net/minecraft/sounds/SoundEvents#FLINTANDSTEEL_USE` | field | 1 |
| `net/minecraft/sounds/SoundEvents#FURNACE_FIRE_CRACKLE` | field | 3 |
| `net/minecraft/sounds/SoundEvents#LAVA_POP` | field | 1 |
| `net/minecraft/sounds/SoundEvents#MUSIC_BIOME_BASALT_DELTAS` | field | 1 |
| `net/minecraft/sounds/SoundEvents#MUSIC_BIOME_CRIMSON_FOREST` | field | 11 |
| `net/minecraft/sounds/SoundEvents#MUSIC_BIOME_NETHER_WASTES` | field | 3 |
| `net/minecraft/sounds/SoundEvents#MUSIC_BIOME_SOUL_SAND_VALLEY` | field | 1 |
| `net/minecraft/sounds/SoundEvents#MUSIC_BIOME_WARPED_FOREST` | field | 1 |
| `net/minecraft/sounds/SoundEvents#NETHERRACK_BREAK` | field | 1 |
| `net/minecraft/sounds/SoundEvents#NETHERRACK_FALL` | field | 1 |
| `net/minecraft/sounds/SoundEvents#NETHERRACK_HIT` | field | 1 |
| `net/minecraft/sounds/SoundEvents#NETHERRACK_PLACE` | field | 1 |
| `net/minecraft/sounds/SoundEvents#PIG_SOUNDS` | field | 1 |
| `net/minecraft/sounds/SoundEvents#SHEEP_SHEAR` | field | 2 |
| `net/minecraft/sounds/SoundEvents#SKELETON_AMBIENT` | field | 1 |
| `net/minecraft/sounds/SoundEvents#SKELETON_DEATH` | field | 2 |
| `net/minecraft/sounds/SoundEvents#SKELETON_HURT` | field | 2 |
| `net/minecraft/sounds/SoundEvents#SLIME_ATTACK` | field | 1 |
| `net/minecraft/sounds/SoundEvents#TOTEM_USE` | field | 1 |
| `net/minecraft/sounds/SoundEvents#WART_BLOCK_STEP` | field | 1 |
| `net/minecraft/sounds/SoundEvents#WOOD_PLACE` | field | 1 |
| `net/minecraft/sounds/SoundSource#BLOCKS` | field | 10 |
| `net/minecraft/stats/StatType#get` | method | 1 |
| `net/minecraft/stats/Stats#INTERACT_WITH_BREWINGSTAND` | field | 1 |
| `net/minecraft/stats/Stats#INTERACT_WITH_FURNACE` | field | 2 |
| `net/minecraft/stats/Stats#ITEM_USED` | field | 1 |
| `net/minecraft/tags/BiomeTags#HAS_BASTION_REMNANT` | field | 10 |
| `net/minecraft/tags/BiomeTags#HAS_NETHER_FORTRESS` | field | 13 |
| `net/minecraft/tags/BiomeTags#HAS_NETHER_FOSSIL` | field | 2 |
| `net/minecraft/tags/BiomeTags#HAS_RUINED_PORTAL_NETHER` | field | 1 |
| `net/minecraft/tags/BlockTags#ANVIL` | field | 1 |
| `net/minecraft/tags/BlockTags#BEACON_BASE_BLOCKS` | field | 1 |
| `net/minecraft/tags/BlockTags#DIRT` | field | 1 |
| `net/minecraft/tags/BlockTags#FIRE` | field | 1 |
| `net/minecraft/tags/BlockTags#FLOWER_POTS` | field | 1 |
| `net/minecraft/tags/BlockTags#GROWS_CROPS` | field | 1 |
| `net/minecraft/tags/BlockTags#INCORRECT_FOR_DIAMOND_TOOL` | field | 1 |
| `net/minecraft/tags/BlockTags#INCORRECT_FOR_IRON_TOOL` | field | 1 |
| `net/minecraft/tags/BlockTags#INCORRECT_FOR_NETHERITE_TOOL` | field | 1 |
| `net/minecraft/tags/BlockTags#LEAVES` | field | 1 |
| `net/minecraft/tags/BlockTags#LIGHTNING_RODS` | field | 1 |
| `net/minecraft/tags/BlockTags#LOGS` | field | 2 |
| `net/minecraft/tags/BlockTags#MINEABLE_WITH_AXE` | field | 1 |
| `net/minecraft/tags/BlockTags#MINEABLE_WITH_HOE` | field | 1 |
| `net/minecraft/tags/BlockTags#MINEABLE_WITH_PICKAXE` | field | 4 |
| `net/minecraft/tags/BlockTags#MINEABLE_WITH_SHOVEL` | field | 1 |
| `net/minecraft/tags/BlockTags#NEEDS_DIAMOND_TOOL` | field | 1 |
| `net/minecraft/tags/BlockTags#NEEDS_IRON_TOOL` | field | 1 |
| `net/minecraft/tags/BlockTags#NYLIUM` | field | 4 |
| `net/minecraft/tags/BlockTags#PLANKS` | field | 1 |
| `net/minecraft/tags/BlockTags#PREVENTS_NEARBY_LEAF_DECAY` | field | 1 |
| `net/minecraft/tags/BlockTags#REPLACEABLE` | field | 1 |
| `net/minecraft/tags/BlockTags#REPLACEABLE_BY_MUSHROOMS` | field | 1 |
| `net/minecraft/tags/BlockTags#REPLACEABLE_BY_TREES` | field | 1 |
| `net/minecraft/tags/BlockTags#SAND` | field | 2 |
| `net/minecraft/tags/BlockTags#SOUL_FIRE_BASE_BLOCKS` | field | 2 |
| `net/minecraft/tags/BlockTags#SOUL_SPEED_BLOCKS` | field | 2 |
| `net/minecraft/tags/BlockTags#SUPPORTS_CROPS` | field | 1 |
| `net/minecraft/tags/BlockTags#SWORD_EFFICIENT` | field | 2 |
| `net/minecraft/tags/BlockTags#WASHED_AWAY_BY_FLUIDS` | field | 2 |
| `net/minecraft/tags/BlockTags#WOOL` | field | 1 |
| `net/minecraft/tags/DamageTypeTags#BYPASSES_INVULNERABILITY` | field | 1 |
| `net/minecraft/tags/EnchantmentTags#CURSE` | field | 1 |
| `net/minecraft/tags/EnchantmentTags#DOUBLE_TRADE_PRICE` | field | 1 |
| `net/minecraft/tags/EntityTypeTags#ARTHROPOD` | field | 1 |
| `net/minecraft/tags/EntityTypeTags#UNDEAD` | field | 1 |
| `net/minecraft/tags/FluidTags#LAVA` | field | 1 |
| `net/minecraft/tags/ItemTags#MINING_ENCHANTABLE` | field | 1 |
| `net/minecraft/tags/ItemTags#PICKAXES` | field | 1 |
| `net/minecraft/tags/ItemTags#PLANKS` | field | 2 |
| `net/minecraft/tags/TagKey#create` | method | 2 |
| `net/minecraft/tags/TagKey#location` | method | 1 |
| `net/minecraft/util/Mth#cos` | method | 1 |
| `net/minecraft/util/Mth#sin` | method | 1 |
| `net/minecraft/util/Mth#square` | method | 1 |
| `net/minecraft/util/RandomSource#create` | method | 3 |
| `net/minecraft/util/RandomSource#nextBoolean` | method | 14 |
| `net/minecraft/util/RandomSource#nextDouble` | method | 28 |
| `net/minecraft/util/RandomSource#nextFloat` | method | 23 |
| `net/minecraft/util/RandomSource#nextGaussian` | method | 6 |
| `net/minecraft/util/RandomSource#nextInt` | method | 75 |
| `net/minecraft/util/RandomSource#nextLong` | method | 4 |
| `net/minecraft/util/Util#TICKER` | field | 1 |
| `net/minecraft/util/Util#memoize` | method | 1 |
| `net/minecraft/util/random/Weighted#<init>` | method | 2 |
| `net/minecraft/util/random/WeightedList#builder` | method | 1 |
| `net/minecraft/util/random/WeightedList#of` | method | 1 |
| `net/minecraft/util/random/WeightedList$Builder#add` | method | 1 |
| `net/minecraft/util/valueproviders/BiasedToBottomInt#of` | method | 3 |
| `net/minecraft/util/valueproviders/ClampedNormalInt#of` | method | 3 |
| `net/minecraft/util/valueproviders/ConstantFloat#of` | method | 1 |
| `net/minecraft/util/valueproviders/ConstantInt#of` | method | 1 |
| `net/minecraft/util/valueproviders/UniformFloat#of` | method | 2 |
| `net/minecraft/util/valueproviders/UniformInt#of` | method | 3 |
| `net/minecraft/world/ContainerHelper#loadAllItems` | method | 2 |
| `net/minecraft/world/ContainerHelper#removeItem` | method | 1 |
| `net/minecraft/world/ContainerHelper#saveAllItems` | method | 2 |
| `net/minecraft/world/ContainerHelper#takeItem` | method | 1 |
| `net/minecraft/world/Containers#dropContents` | method | 1 |
| `net/minecraft/world/Difficulty#PEACEFUL` | field | 3 |
| `net/minecraft/world/DifficultyInstance#getSpecialMultiplier` | method | 1 |
| `net/minecraft/world/InteractionResult#CONSUME` | field | 1 |
| `net/minecraft/world/InteractionResult#FAIL` | field | 2 |
| `net/minecraft/world/InteractionResult#SUCCESS` | field | 7 |
| `net/minecraft/world/damagesource/DamageSource#getDirectEntity` | method | 1 |
| `net/minecraft/world/damagesource/DamageSource#is` | method | 2 |
| `net/minecraft/world/damagesource/DamageSources#cactus` | method | 3 |
| `net/minecraft/world/damagesource/DamageSources#fall` | method | 1 |
| `net/minecraft/world/damagesource/DamageSources#fellOutOfWorld` | method | 1 |
| `net/minecraft/world/damagesource/DamageSources#generic` | method | 2 |
| `net/minecraft/world/damagesource/DamageSources#hotFloor` | method | 1 |
| `net/minecraft/world/damagesource/DamageSources#inFire` | method | 1 |
| `net/minecraft/world/damagesource/DamageSources#mobAttack` | method | 1 |
| `net/minecraft/world/damagesource/DamageTypes#INDIRECT_MAGIC` | field | 1 |
| `net/minecraft/world/damagesource/DamageTypes#WITHER` | field | 1 |
| `net/minecraft/world/effect/MobEffectInstance#<init>` | method | 4 |
| `net/minecraft/world/effect/MobEffectInstance#getEffect` | method | 1 |
| `net/minecraft/world/effect/MobEffects#POISON` | field | 1 |
| `net/minecraft/world/effect/MobEffects#REGENERATION` | field | 2 |
| `net/minecraft/world/effect/MobEffects#WITHER` | field | 2 |
| `net/minecraft/world/entity/ContainerUser#getLivingEntity` | method | 1 |
| `net/minecraft/world/entity/Entity#causeFallDamage` | method | 1 |
| `net/minecraft/world/entity/Entity#fireImmune` | method | 2 |
| `net/minecraft/world/entity/Entity#getBbHeight` | method | 1 |
| `net/minecraft/world/entity/Entity#getBoundingBox` | method | 1 |
| `net/minecraft/world/entity/Entity#getDeltaMovement` | method | 1 |
| `net/minecraft/world/entity/Entity#getKnownMovement` | method | 1 |
| `net/minecraft/world/entity/Entity#getType` | method | 1 |
| `net/minecraft/world/entity/Entity#isAlive` | method | 1 |
| `net/minecraft/world/entity/Entity#isClientAuthoritative` | method | 1 |
| `net/minecraft/world/entity/Entity#isSteppingCarefully` | method | 1 |
| `net/minecraft/world/entity/Entity#isSuppressingBounce` | method | 1 |
| `net/minecraft/world/entity/Entity#oldPosition` | method | 1 |
| `net/minecraft/world/entity/Entity#position` | method | 2 |
| `net/minecraft/world/entity/Entity#setDeltaMovement` | method | 1 |
| `net/minecraft/world/entity/EntityDimensions#fixed` | method | 1 |
| `net/minecraft/world/entity/EntitySpawnReason#BREEDING` | field | 2 |
| `net/minecraft/world/entity/EntitySpawnReason#MOB_SUMMONED` | field | 1 |
| `net/minecraft/world/entity/EntityType$Builder#build` | method | 1 |
| `net/minecraft/world/entity/EntityType$Builder#dimensions` | field | 1 |
| `net/minecraft/world/entity/EntityType$Builder#fireImmune` | method | 1 |
| `net/minecraft/world/entity/EntityType$Builder#noSummon` | method | 1 |
| `net/minecraft/world/entity/EntityType$Builder#of` | method | 1 |
| `net/minecraft/world/entity/EntityType$Builder#sized` | method | 1 |
| `net/minecraft/world/entity/EntityTypes#ENDERMAN` | field | 1 |
| `net/minecraft/world/entity/EntityTypes#GHAST` | field | 1 |
| `net/minecraft/world/entity/EntityTypes#HOGLIN` | field | 1 |
| `net/minecraft/world/entity/EntityTypes#MAGMA_CUBE` | field | 1 |
| `net/minecraft/world/entity/EntityTypes#PIGLIN` | field | 1 |
| `net/minecraft/world/entity/EntityTypes#PIGLIN_BRUTE` | field | 1 |
| `net/minecraft/world/entity/EntityTypes#STRIDER` | field | 1 |
| `net/minecraft/world/entity/EntityTypes#WITHER_SKELETON` | field | 1 |
| `net/minecraft/world/entity/EntityTypes#ZOMBIFIED_PIGLIN` | field | 1 |
| `net/minecraft/world/entity/EquipmentSlot#CHEST` | field | 2 |
| `net/minecraft/world/entity/EquipmentSlot#FEET` | field | 2 |
| `net/minecraft/world/entity/EquipmentSlot#HEAD` | field | 4 |
| `net/minecraft/world/entity/EquipmentSlot#LEGS` | field | 2 |
| `net/minecraft/world/entity/EquipmentSlot#MAINHAND` | field | 1 |
| `net/minecraft/world/entity/EquipmentSlot#OFFHAND` | field | 1 |
| `net/minecraft/world/entity/EquipmentSlotGroup#ANY` | field | 1 |
| `net/minecraft/world/entity/EquipmentSlotGroup#ARMOR` | field | 1 |
| `net/minecraft/world/entity/EquipmentSlotGroup#MAINHAND` | field | 1 |
| `net/minecraft/world/entity/EquipmentSlotGroup#iterator` | method | 1 |
| `net/minecraft/world/entity/ExperienceOrb#<init>` | method | 1 |
| `net/minecraft/world/entity/ExperienceOrb#award` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#addEffect` | method | 3 |
| `net/minecraft/world/entity/LivingEntity#createLivingAttributes` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#damageSources` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#getAttributeValue` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#getItemBySlot` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#isSpectator` | method | 1 |
| `net/minecraft/world/entity/Mob#<init>` | method | 1 |
| `net/minecraft/world/entity/Mob#addAdditionalSaveData` | method | 1 |
| `net/minecraft/world/entity/Mob#createMobAttributes` | method | 6 |
| `net/minecraft/world/entity/Mob#readAdditionalSaveData` | method | 1 |
| `net/minecraft/world/entity/Mob#tick` | method | 1 |
| `net/minecraft/world/entity/MobCategory#AMBIENT` | field | 1 |
| `net/minecraft/world/entity/MobCategory#MISC` | field | 1 |
| `net/minecraft/world/entity/MobCategory#MONSTER` | field | 1 |
| `net/minecraft/world/entity/MoverType#SELF` | field | 1 |
| `net/minecraft/world/entity/ai/attributes/AttributeInstance#setBaseValue` | method | 1 |
| `net/minecraft/world/entity/ai/attributes/AttributeModifier$Operation#ADD_VALUE` | field | 1 |
| `net/minecraft/world/entity/ai/attributes/AttributeSupplier$Builder#add` | method | 7 |
| `net/minecraft/world/entity/ai/attributes/AttributeSupplier$Builder#build` | method | 1 |
| `net/minecraft/world/entity/ai/attributes/Attributes#ARMOR` | field | 2 |
| `net/minecraft/world/entity/ai/attributes/Attributes#ATTACK_DAMAGE` | field | 4 |
| `net/minecraft/world/entity/ai/attributes/Attributes#FLYING_SPEED` | field | 4 |
| `net/minecraft/world/entity/ai/attributes/Attributes#FOLLOW_RANGE` | field | 4 |
| `net/minecraft/world/entity/ai/attributes/Attributes#MAX_HEALTH` | field | 5 |
| `net/minecraft/world/entity/ai/attributes/Attributes#MOVEMENT_SPEED` | field | 5 |
| `net/minecraft/world/entity/ai/attributes/Attributes#SCALE` | field | 1 |
| `net/minecraft/world/entity/ai/attributes/DefaultAttributes#getSupplier` | method | 1 |
| `net/minecraft/world/entity/ai/attributes/DefaultAttributes#hasSupplier` | method | 1 |
| `net/minecraft/world/entity/ai/attributes/RangedAttribute#<init>` | method | 1 |
| `net/minecraft/world/entity/ai/attributes/RangedAttribute#setSyncable` | method | 1 |
| `net/minecraft/world/entity/ai/behavior/TransportItemsBetweenContainers#<init>` | method | 1 |
| `net/minecraft/world/entity/ai/control/FlyingMoveControl#<init>` | method | 3 |
| `net/minecraft/world/entity/ai/control/LookControl#<init>` | method | 2 |
| `net/minecraft/world/entity/ai/goal/BreedGoal#<init>` | method | 2 |
| `net/minecraft/world/entity/ai/goal/FloatGoal#<init>` | method | 1 |
| `net/minecraft/world/entity/ai/goal/FollowParentGoal#<init>` | method | 1 |
| `net/minecraft/world/entity/ai/goal/Goal#<init>` | method | 8 |
| `net/minecraft/world/entity/ai/goal/Goal#start` | method | 8 |
| `net/minecraft/world/entity/ai/goal/Goal#stop` | method | 6 |
| `net/minecraft/world/entity/ai/goal/Goal#tick` | method | 5 |
| `net/minecraft/world/entity/ai/goal/Goal$Flag#MOVE` | field | 3 |
| `net/minecraft/world/entity/ai/goal/GoalSelector#addGoal` | method | 3 |
| `net/minecraft/world/entity/ai/goal/LookAtPlayerGoal#<init>` | method | 2 |
| `net/minecraft/world/entity/ai/goal/RandomLookAroundGoal#<init>` | method | 2 |
| `net/minecraft/world/entity/ai/goal/RangedAttackGoal#<init>` | method | 1 |
| `net/minecraft/world/entity/ai/goal/WaterAvoidingRandomStrollGoal#<init>` | method | 1 |
| `net/minecraft/world/entity/ai/goal/target/NearestAttackableTargetGoal#<init>` | method | 2 |
| `net/minecraft/world/entity/ai/navigation/FlyingPathNavigation#<init>` | method | 2 |
| `net/minecraft/world/entity/ai/navigation/FlyingPathNavigation#setCanFloat` | method | 2 |
| `net/minecraft/world/entity/ai/navigation/FlyingPathNavigation#setCanOpenDoors` | method | 2 |
| `net/minecraft/world/entity/ai/navigation/FlyingPathNavigation#tick` | method | 1 |
| `net/minecraft/world/entity/ai/navigation/PathNavigation#createPath` | method | 5 |
| `net/minecraft/world/entity/ai/navigation/PathNavigation#isDone` | method | 6 |
| `net/minecraft/world/entity/ai/navigation/PathNavigation#isInProgress` | method | 5 |
| `net/minecraft/world/entity/ai/navigation/PathNavigation#moveTo` | method | 5 |
| `net/minecraft/world/entity/ai/navigation/PathNavigation#stop` | method | 1 |
| `net/minecraft/world/entity/ai/util/HoverRandomPos#getPos` | method | 2 |
| `net/minecraft/world/entity/animal/golem/CopperGolemAi#initIdleActivity` | method | 1 |
| `net/minecraft/world/entity/animal/pig/PigSoundVariant#adultSounds` | method | 1 |
| `net/minecraft/world/entity/animal/pig/PigSoundVariant$PigSoundSet#ambientSound` | method | 1 |
| `net/minecraft/world/entity/animal/pig/PigSoundVariant$PigSoundSet#deathSound` | method | 1 |
| `net/minecraft/world/entity/animal/pig/PigSoundVariant$PigSoundSet#hurtSound` | method | 1 |
| `net/minecraft/world/entity/animal/pig/PigSoundVariants$SoundSet#CLASSIC` | field | 1 |
| `net/minecraft/world/entity/item/ItemEntity#blockPosition` | method | 1 |
| `net/minecraft/world/entity/item/ItemEntity#discard` | method | 1 |
| `net/minecraft/world/entity/item/ItemEntity#distanceTo` | method | 1 |
| `net/minecraft/world/entity/item/ItemEntity#getItem` | method | 1 |
| `net/minecraft/world/entity/item/ItemEntity#getX` | method | 1 |
| `net/minecraft/world/entity/item/ItemEntity#getY` | method | 1 |
| `net/minecraft/world/entity/item/ItemEntity#getZ` | method | 1 |
| `net/minecraft/world/entity/item/ItemEntity#isAlive` | method | 1 |
| `net/minecraft/world/entity/monster/Monster#<init>` | method | 2 |
| `net/minecraft/world/entity/monster/Monster#canBeAffected` | method | 1 |
| `net/minecraft/world/entity/monster/Monster#checkDespawn` | method | 2 |
| `net/minecraft/world/entity/monster/Monster#tick` | method | 1 |
| `net/minecraft/world/entity/monster/piglin/Piglin#isAdult` | method | 1 |
| `net/minecraft/world/entity/monster/piglin/Piglin#populateDefaultEquipmentSlots` | method | 1 |
| `net/minecraft/world/entity/monster/piglin/PiglinAi#isWearingSafeArmor` | method | 1 |
| `net/minecraft/world/entity/monster/piglin/PiglinBrute#populateDefaultEquipmentSlots` | method | 1 |
| `net/minecraft/world/entity/monster/piglin/PiglinBrute#setItemSlot` | method | 1 |
| `net/minecraft/world/entity/monster/skeleton/Skeleton#<init>` | method | 1 |
| `net/minecraft/world/entity/monster/skeleton/Skeleton#checkDespawn` | method | 1 |
| `net/minecraft/world/entity/monster/skeleton/Skeleton#finalizeSpawn` | method | 1 |
| `net/minecraft/world/entity/monster/skeleton/Skeleton#populateDefaultEquipmentSlots` | method | 1 |
| `net/minecraft/world/entity/player/Player#addEffect` | method | 1 |
| `net/minecraft/world/entity/player/Player#addItem` | method | 2 |
| `net/minecraft/world/entity/player/Player#awardStat` | method | 4 |
| `net/minecraft/world/entity/player/Player#createAttributes` | method | 1 |
| `net/minecraft/world/entity/player/Player#damageSources` | method | 1 |
| `net/minecraft/world/entity/player/Player#distanceToSqr` | method | 1 |
| `net/minecraft/world/entity/player/Player#getBbHeight` | method | 1 |
| `net/minecraft/world/entity/player/Player#getDestroySpeed` | method | 1 |
| `net/minecraft/world/entity/player/Player#getMainHandItem` | method | 4 |
| `net/minecraft/world/entity/player/Player#getYHeadRot` | method | 1 |
| `net/minecraft/world/entity/player/Player#hurtOrSimulate` | method | 1 |
| `net/minecraft/world/entity/player/Player#isCreative` | method | 5 |
| `net/minecraft/world/entity/player/Player#isShiftKeyDown` | method | 1 |
| `net/minecraft/world/entity/player/Player#isSpectator` | method | 1 |
| `net/minecraft/world/entity/player/Player#openMenu` | method | 4 |
| `net/minecraft/world/entity/player/Player#playSound` | method | 1 |
| `net/minecraft/world/entity/player/Player#position` | method | 1 |
| `net/minecraft/world/entity/player/Player#registryAccess` | method | 1 |
| `net/minecraft/world/entity/player/Player#sendOverlayMessage` | method | 1 |
| `net/minecraft/world/entity/projectile/ProjectileUtil#getHitResultOnMoveVector` | method | 1 |
| `net/minecraft/world/food/FoodProperties$Builder#<init>` | method | 1 |
| `net/minecraft/world/food/FoodProperties$Builder#build` | method | 1 |
| `net/minecraft/world/food/FoodProperties$Builder#nutrition` | method | 1 |
| `net/minecraft/world/food/FoodProperties$Builder#saturationModifier` | method | 1 |
| `net/minecraft/world/food/Foods#APPLE` | field | 2 |
| `net/minecraft/world/food/Foods#COOKED_CHICKEN` | field | 1 |
| `net/minecraft/world/food/Foods#MUSHROOM_STEW` | field | 1 |
| `net/minecraft/world/inventory/BrewingStandMenu#<init>` | method | 1 |
| `net/minecraft/world/inventory/ChestMenu#threeRows` | method | 1 |
| `net/minecraft/world/inventory/ContainerLevelAccess#evaluate` | method | 1 |
| `net/minecraft/world/inventory/CraftingMenu#stillValid` | method | 1 |
| `net/minecraft/world/inventory/FurnaceMenu#<init>` | method | 2 |
| `net/minecraft/world/item/BlockItem#<init>` | method | 1 |
| `net/minecraft/world/item/BlockItem#getBlock` | method | 3 |
| `net/minecraft/world/item/DyeColor#WHITE` | field | 1 |
| `net/minecraft/world/item/DyeColor#byName` | method | 1 |
| `net/minecraft/world/item/DyeColor#getSerializedName` | method | 1 |
| `net/minecraft/world/item/Item#finishUsingItem` | method | 3 |
| `net/minecraft/world/item/Item#getCraftingRemainder` | method | 1 |
| `net/minecraft/world/item/Item#useOn` | method | 1 |
| `net/minecraft/world/item/Item$Properties#<init>` | method | 1 |
| `net/minecraft/world/item/Item$Properties#axe` | method | 1 |
| `net/minecraft/world/item/Item$Properties#component` | method | 1 |
| `net/minecraft/world/item/Item$Properties#fireResistant` | method | 1 |
| `net/minecraft/world/item/Item$Properties#food` | method | 2 |
| `net/minecraft/world/item/Item$Properties#hoe` | method | 1 |
| `net/minecraft/world/item/Item$Properties#shovel` | method | 1 |
| `net/minecraft/world/item/ItemStack#<init>` | method | 32 |
| `net/minecraft/world/item/ItemStack#EMPTY` | field | 2 |
| `net/minecraft/world/item/ItemStack#get` | method | 1 |
| `net/minecraft/world/item/ItemStack#getDestroySpeed` | method | 1 |
| `net/minecraft/world/item/ItemStack#getHoverName` | method | 1 |
| `net/minecraft/world/item/ItemStack#getItem` | method | 10 |
| `net/minecraft/world/item/ItemStack#is` | method | 2 |
| `net/minecraft/world/item/ItemStack#isEmpty` | method | 4 |
| `net/minecraft/world/item/ItemStackTemplate#<init>` | method | 1 |
| `net/minecraft/world/item/Items#ACTIVATOR_RAIL` | field | 1 |
| `net/minecraft/world/item/Items#AIR` | field | 4 |
| `net/minecraft/world/item/Items#ANCIENT_DEBRIS` | field | 1 |
| `net/minecraft/world/item/Items#ARROW` | field | 1 |
| `net/minecraft/world/item/Items#BLAZE_POWDER` | field | 1 |
| `net/minecraft/world/item/Items#BLAZE_ROD` | field | 1 |
| `net/minecraft/world/item/Items#BONE` | field | 2 |
| `net/minecraft/world/item/Items#BONE_BLOCK` | field | 1 |
| `net/minecraft/world/item/Items#BONE_MEAL` | field | 1 |
| `net/minecraft/world/item/Items#BOOK` | field | 1 |
| `net/minecraft/world/item/Items#BOW` | field | 1 |
| `net/minecraft/world/item/Items#BROWN_MUSHROOM` | field | 4 |
| `net/minecraft/world/item/Items#BUCKET` | field | 1 |
| `net/minecraft/world/item/Items#COAL` | field | 1 |
| `net/minecraft/world/item/Items#COBBLESTONE` | field | 2 |
| `net/minecraft/world/item/Items#COMPASS` | field | 1 |
| `net/minecraft/world/item/Items#DETECTOR_RAIL` | field | 1 |
| `net/minecraft/world/item/Items#DIAMOND` | field | 5 |
| `net/minecraft/world/item/Items#DIAMOND_PICKAXE` | field | 1 |
| `net/minecraft/world/item/Items#DYE` | field | 3 |
| `net/minecraft/world/item/Items#EXPERIENCE_BOTTLE` | field | 1 |
| `net/minecraft/world/item/Items#FEATHER` | field | 1 |
| `net/minecraft/world/item/Items#FLINT_AND_STEEL` | field | 2 |
| `net/minecraft/world/item/Items#GHAST_TEAR` | field | 1 |
| `net/minecraft/world/item/Items#GLASS_BOTTLE` | field | 3 |
| `net/minecraft/world/item/Items#GLOWSTONE` | field | 2 |
| `net/minecraft/world/item/Items#GLOWSTONE_DUST` | field | 3 |
| `net/minecraft/world/item/Items#GOLDEN_APPLE` | field | 1 |
| `net/minecraft/world/item/Items#GOLD_INGOT` | field | 2 |
| `net/minecraft/world/item/Items#IRON_INGOT` | field | 1 |
| `net/minecraft/world/item/Items#LAPIS_LAZULI` | field | 1 |
| `net/minecraft/world/item/Items#LINGERING_POTION` | field | 1 |
| `net/minecraft/world/item/Items#MAGMA_CREAM` | field | 1 |
| `net/minecraft/world/item/Items#MINECART` | field | 1 |
| `net/minecraft/world/item/Items#NETHERITE_INGOT` | field | 1 |
| `net/minecraft/world/item/Items#NETHERITE_PICKAXE` | field | 1 |
| `net/minecraft/world/item/Items#NETHERITE_SCRAP` | field | 1 |
| `net/minecraft/world/item/Items#NETHERITE_UPGRADE_SMITHING_TEMPLATE` | field | 1 |
| `net/minecraft/world/item/Items#NETHERRACK` | field | 1 |
| `net/minecraft/world/item/Items#NETHER_BRICK` | field | 2 |
| `net/minecraft/world/item/Items#NETHER_BRICKS` | field | 1 |
| `net/minecraft/world/item/Items#NETHER_BRICK_SLAB` | field | 1 |
| `net/minecraft/world/item/Items#NETHER_WART` | field | 3 |
| `net/minecraft/world/item/Items#OBSIDIAN` | field | 1 |
| `net/minecraft/world/item/Items#PAPER` | field | 1 |
| `net/minecraft/world/item/Items#PISTON` | field | 1 |
| `net/minecraft/world/item/Items#PORKCHOP` | field | 1 |
| `net/minecraft/world/item/Items#POTION` | field | 2 |
| `net/minecraft/world/item/Items#QUARTZ` | field | 2 |
| `net/minecraft/world/item/Items#RAIL` | field | 1 |
| `net/minecraft/world/item/Items#REDSTONE` | field | 4 |
| `net/minecraft/world/item/Items#REDSTONE_TORCH` | field | 1 |
| `net/minecraft/world/item/Items#RED_MUSHROOM` | field | 4 |
| `net/minecraft/world/item/Items#SHIELD` | field | 4 |
| `net/minecraft/world/item/Items#SHULKER_SHELL` | field | 1 |
| `net/minecraft/world/item/Items#SLIME_BALL` | field | 1 |
| `net/minecraft/world/item/Items#SOUL_SAND` | field | 1 |
| `net/minecraft/world/item/Items#SPECTRAL_ARROW` | field | 1 |
| `net/minecraft/world/item/Items#SPLASH_POTION` | field | 1 |
| `net/minecraft/world/item/Items#STICK` | field | 3 |
| `net/minecraft/world/item/Items#STONE` | field | 1 |
| `net/minecraft/world/item/Items#STONE_PRESSURE_PLATE` | field | 1 |
| `net/minecraft/world/item/Items#STONE_SWORD` | field | 1 |
| `net/minecraft/world/item/Items#STRING` | field | 1 |
| `net/minecraft/world/item/Items#SUGAR` | field | 1 |
| `net/minecraft/world/item/Items#TORCH` | field | 1 |
| `net/minecraft/world/item/Items#WOODEN_SWORD` | field | 1 |
| `net/minecraft/world/item/MapItem#update` | method | 1 |
| `net/minecraft/world/item/ShearsItem#<init>` | method | 1 |
| `net/minecraft/world/item/ShearsItem#createToolProperties` | method | 1 |
| `net/minecraft/world/item/ToolMaterial#<init>` | method | 1 |
| `net/minecraft/world/item/ToolMaterial#durability` | method | 1 |
| `net/minecraft/world/item/ToolMaterial#repairItems` | method | 1 |
| `net/minecraft/world/item/ToolMaterial#speed` | method | 1 |
| `net/minecraft/world/item/alchemy/PotionContents#<init>` | method | 1 |
| `net/minecraft/world/item/alchemy/Potions#AWKWARD` | field | 1 |
| `net/minecraft/world/item/alchemy/Potions#HEALING` | field | 1 |
| `net/minecraft/world/item/alchemy/Potions#WATER` | field | 1 |
| `net/minecraft/world/item/component/Tool#<init>` | method | 1 |
| `net/minecraft/world/item/component/Tool#canDestroyBlocksInCreative` | method | 1 |
| `net/minecraft/world/item/component/Tool#damagePerBlock` | method | 1 |
| `net/minecraft/world/item/component/Tool#defaultMiningSpeed` | method | 1 |
| `net/minecraft/world/item/component/Tool#rules` | method | 1 |
| `net/minecraft/world/item/component/Tool$Rule#minesAndDrops` | method | 1 |
| `net/minecraft/world/item/consume_effects/ApplyStatusEffectsConsumeEffect#<init>` | method | 1 |
| `net/minecraft/world/item/context/BlockPlaceContext#getClickedFace` | method | 4 |
| `net/minecraft/world/item/context/BlockPlaceContext#getClickedPos` | method | 7 |
| `net/minecraft/world/item/context/BlockPlaceContext#getHorizontalDirection` | method | 4 |
| `net/minecraft/world/item/context/BlockPlaceContext#getLevel` | method | 7 |
| `net/minecraft/world/item/context/BlockPlaceContext#getNearestLookingDirections` | method | 5 |
| `net/minecraft/world/item/context/BlockPlaceContext#isSecondaryUseActive` | method | 1 |
| `net/minecraft/world/item/context/UseOnContext#getClickedFace` | method | 1 |
| `net/minecraft/world/item/context/UseOnContext#getClickedPos` | method | 1 |
| `net/minecraft/world/item/context/UseOnContext#getLevel` | method | 1 |
| `net/minecraft/world/item/context/UseOnContext#getPlayer` | method | 1 |
| `net/minecraft/world/item/crafting/BlastingRecipe#assemble` | method | 1 |
| `net/minecraft/world/item/crafting/BlastingRecipe#experience` | method | 1 |
| `net/minecraft/world/item/crafting/BlastingRecipe#input` | method | 1 |
| `net/minecraft/world/item/crafting/BrewingInput#<init>` | method | 1 |
| `net/minecraft/world/item/crafting/BrewingRecipe#<init>` | method | 1 |
| `net/minecraft/world/item/crafting/BrewingRecipe#assemble` | method | 1 |
| `net/minecraft/world/item/crafting/Ingredient#items` | method | 1 |
| `net/minecraft/world/item/crafting/PotionIngredient#of` | method | 2 |
| `net/minecraft/world/item/crafting/RecipeAccess#propertySet` | method | 1 |
| `net/minecraft/world/item/crafting/RecipeHolder#value` | method | 2 |
| `net/minecraft/world/item/crafting/RecipeManager#createCheck` | method | 1 |
| `net/minecraft/world/item/crafting/RecipeManager#getRecipes` | method | 1 |
| `net/minecraft/world/item/crafting/RecipeManager$CachedCheck#getRecipeFor` | method | 1 |
| `net/minecraft/world/item/crafting/RecipePropertySet#BREWING_REAGENTS` | field | 1 |
| `net/minecraft/world/item/crafting/RecipePropertySet#test` | method | 1 |
| `net/minecraft/world/item/crafting/RecipeType#BREWING` | field | 1 |
| `net/minecraft/world/item/crafting/RecipeType#SMELTING` | field | 2 |
| `net/minecraft/world/item/crafting/SingleRecipeInput#<init>` | method | 1 |
| `net/minecraft/world/item/enchantment/Enchantment#definition` | method | 2 |
| `net/minecraft/world/item/enchantment/Enchantment#dynamicCost` | method | 1 |
| `net/minecraft/world/item/enchantment/Enchantment#enchantment` | method | 1 |
| `net/minecraft/world/item/enchantment/Enchantment#getMaxLevel` | method | 1 |
| `net/minecraft/world/item/enchantment/Enchantment$Builder#withEffect` | method | 3 |
| `net/minecraft/world/item/enchantment/EnchantmentEffectComponents#ATTRIBUTES` | field | 1 |
| `net/minecraft/world/item/enchantment/EnchantmentEffectComponents#KNOCKBACK` | field | 1 |
| `net/minecraft/world/item/enchantment/EnchantmentEffectComponents#POST_ATTACK` | field | 1 |
| `net/minecraft/world/item/enchantment/EnchantmentHelper#doPostAttackEffects` | method | 1 |
| `net/minecraft/world/item/enchantment/EnchantmentHelper#getItemEnchantmentLevel` | method | 1 |
| `net/minecraft/world/item/enchantment/EnchantmentTarget#ATTACKER` | field | 1 |
| `net/minecraft/world/item/enchantment/EnchantmentTarget#VICTIM` | field | 1 |
| `net/minecraft/world/item/enchantment/Enchantments#EFFICIENCY` | field | 1 |
| `net/minecraft/world/item/enchantment/Enchantments#FEATHER_FALLING` | field | 1 |
| `net/minecraft/world/item/enchantment/Enchantments#FIRE_PROTECTION` | field | 1 |
| `net/minecraft/world/item/enchantment/Enchantments#LOOTING` | field | 1 |
| `net/minecraft/world/item/enchantment/Enchantments#MENDING` | field | 2 |
| `net/minecraft/world/item/enchantment/Enchantments#PROTECTION` | field | 1 |
| `net/minecraft/world/item/enchantment/Enchantments#SHARPNESS` | field | 1 |
| `net/minecraft/world/item/enchantment/Enchantments#SILK_TOUCH` | field | 3 |
| `net/minecraft/world/item/enchantment/Enchantments#THORNS` | field | 1 |
| `net/minecraft/world/item/enchantment/Enchantments#UNBREAKING` | field | 1 |
| `net/minecraft/world/item/enchantment/LevelBasedValue#constant` | method | 1 |
| `net/minecraft/world/item/enchantment/LevelBasedValue#perLevel` | method | 2 |
| `net/minecraft/world/item/enchantment/LevelBasedValue$LevelsSquared#<init>` | method | 1 |
| `net/minecraft/world/item/enchantment/LevelBasedValue$Lookup#<init>` | method | 1 |
| `net/minecraft/world/item/enchantment/effects/AddValue#<init>` | method | 1 |
| `net/minecraft/world/item/enchantment/effects/AllOf#entityEffects` | method | 1 |
| `net/minecraft/world/item/enchantment/effects/ApplyEntityImpulse#<init>` | method | 1 |
| `net/minecraft/world/item/enchantment/effects/ChangeItemDamage#<init>` | method | 1 |
| `net/minecraft/world/item/enchantment/effects/DamageEntity#<init>` | method | 1 |
| `net/minecraft/world/item/enchantment/effects/EnchantmentAttributeEffect#<init>` | method | 1 |
| `net/minecraft/world/item/enchantment/effects/Ignite#<init>` | method | 1 |
| `net/minecraft/world/item/equipment/ArmorMaterial#repairIngredient` | method | 1 |
| `net/minecraft/world/item/equipment/trim/TrimMaterials$Palette#suffix` | method | 1 |
| `net/minecraft/world/level/BlockGetter#getBlockState` | method | 3 |
| `net/minecraft/world/level/ChunkPos#getBlockX` | method | 1 |
| `net/minecraft/world/level/ChunkPos#getBlockZ` | method | 1 |
| `net/minecraft/world/level/ChunkPos#getMiddleBlockX` | method | 1 |
| `net/minecraft/world/level/ChunkPos#getMiddleBlockZ` | method | 1 |
| `net/minecraft/world/level/ChunkPos#unpack` | method | 1 |
| `net/minecraft/world/level/ChunkPos#x` | method | 3 |
| `net/minecraft/world/level/ChunkPos#z` | method | 3 |
| `net/minecraft/world/level/ItemLike#asItem` | method | 1 |
| `net/minecraft/world/level/Level#NETHER` | field | 2 |
| `net/minecraft/world/level/Level#addFreshEntity` | method | 1 |
| `net/minecraft/world/level/Level#addParticle` | method | 23 |
| `net/minecraft/world/level/Level#damageSources` | method | 7 |
| `net/minecraft/world/level/Level#dimension` | method | 2 |
| `net/minecraft/world/level/Level#dimensionType` | method | 1 |
| `net/minecraft/world/level/Level#explode` | method | 1 |
| `net/minecraft/world/level/Level#gameEvent` | method | 1 |
| `net/minecraft/world/level/Level#getBlockEntity` | method | 5 |
| `net/minecraft/world/level/Level#getBlockState` | method | 34 |
| `net/minecraft/world/level/Level#getChunk` | method | 1 |
| `net/minecraft/world/level/Level#getDifficulty` | method | 3 |
| `net/minecraft/world/level/Level#getEntitiesOfClass` | method | 3 |
| `net/minecraft/world/level/Level#getFluidState` | method | 3 |
| `net/minecraft/world/level/Level#getNearestPlayer` | method | 1 |
| `net/minecraft/world/level/Level#getRandom` | method | 5 |
| `net/minecraft/world/level/Level#getServer` | method | 1 |
| `net/minecraft/world/level/Level#isEmptyBlock` | method | 7 |
| `net/minecraft/world/level/Level#levelEvent` | method | 1 |
| `net/minecraft/world/level/Level#playLocalSound` | method | 7 |
| `net/minecraft/world/level/Level#playSound` | method | 4 |
| `net/minecraft/world/level/Level#players` | method | 1 |
| `net/minecraft/world/level/Level#recipeAccess` | method | 1 |
| `net/minecraft/world/level/Level#setBlockAndUpdate` | method | 7 |
| `net/minecraft/world/level/Level$ExplosionInteraction#MOB` | field | 1 |
| `net/minecraft/world/level/LevelAccessor#destroyBlock` | method | 1 |
| `net/minecraft/world/level/LevelAccessor#dimensionType` | method | 2 |
| `net/minecraft/world/level/LevelAccessor#getBlockState` | method | 15 |
| `net/minecraft/world/level/LevelAccessor#getChunk` | method | 1 |
| `net/minecraft/world/level/LevelAccessor#getDifficulty` | method | 2 |
| `net/minecraft/world/level/LevelAccessor#getEntitiesOfClass` | method | 2 |
| `net/minecraft/world/level/LevelAccessor#getMaxLocalRawBrightness` | method | 2 |
| `net/minecraft/world/level/LevelAccessor#getMaxY` | method | 1 |
| `net/minecraft/world/level/LevelAccessor#getMinY` | method | 1 |
| `net/minecraft/world/level/LevelAccessor#isEmptyBlock` | method | 5 |
| `net/minecraft/world/level/LevelAccessor#isOutsideBuildHeight` | method | 1 |
| `net/minecraft/world/level/LevelAccessor#setBlock` | method | 3 |
| `net/minecraft/world/level/LevelReader#getBlockState` | method | 49 |
| `net/minecraft/world/level/LevelReader#isEmptyBlock` | method | 1 |
| `net/minecraft/world/level/NoiseColumn#getBlock` | method | 1 |
| `net/minecraft/world/level/ScheduledTickAccess#scheduleTick` | method | 1 |
| `net/minecraft/world/level/ServerLevelAccessor#getBiome` | method | 1 |
| `net/minecraft/world/level/ServerLevelAccessor#getBlockState` | method | 18 |
| `net/minecraft/world/level/ServerLevelAccessor#getChunk` | method | 1 |
| `net/minecraft/world/level/ServerLevelAccessor#getLevel` | method | 2 |
| `net/minecraft/world/level/ServerLevelAccessor#getMaxY` | method | 1 |
| `net/minecraft/world/level/ServerLevelAccessor#getMinY` | method | 4 |
| `net/minecraft/world/level/ServerLevelAccessor#getRandom` | method | 2 |
| `net/minecraft/world/level/ServerLevelAccessor#isEmptyBlock` | method | 17 |
| `net/minecraft/world/level/ServerLevelAccessor#setBlock` | method | 4 |
| `net/minecraft/world/level/StructureManager#startsForStructure` | method | 1 |
| `net/minecraft/world/level/WorldGenLevel#dimensionType` | method | 1 |
| `net/minecraft/world/level/WorldGenLevel#getBlockState` | method | 3 |
| `net/minecraft/world/level/WorldGenLevel#getChunk` | method | 1 |
| `net/minecraft/world/level/WorldGenLevel#isEmptyBlock` | method | 1 |
| `net/minecraft/world/level/WorldGenLevel#registryAccess` | method | 1 |
| `net/minecraft/world/level/biome/Biomes#BASALT_DELTAS` | field | 2 |
| `net/minecraft/world/level/biome/Biomes#CRIMSON_FOREST` | field | 3 |
| `net/minecraft/world/level/biome/Biomes#NETHER_WASTES` | field | 2 |
| `net/minecraft/world/level/biome/Biomes#SOUL_SAND_VALLEY` | field | 2 |
| `net/minecraft/world/level/biome/Biomes#WARPED_FOREST` | field | 2 |
| `net/minecraft/world/level/block/AbstractFurnaceBlock#<init>` | method | 2 |
| `net/minecraft/world/level/block/AnvilBlock#<init>` | method | 1 |
| `net/minecraft/world/level/block/BaseEntityBlock#<init>` | method | 1 |
| `net/minecraft/world/level/block/Block#animateTick` | method | 1 |
| `net/minecraft/world/level/block/Block#asItem` | method | 7 |
| `net/minecraft/world/level/block/Block#box` | method | 10 |
| `net/minecraft/world/level/block/Block#canSurvive` | method | 1 |
| `net/minecraft/world/level/block/Block#defaultBlockState` | method | 86 |
| `net/minecraft/world/level/block/Block#defaultMapColor` | method | 2 |
| `net/minecraft/world/level/block/Block#destroy` | method | 1 |
| `net/minecraft/world/level/block/Block#dropResources` | method | 1 |
| `net/minecraft/world/level/block/Block#fallOn` | method | 1 |
| `net/minecraft/world/level/block/Block#getDrops` | method | 1 |
| `net/minecraft/world/level/block/Block#getFriction` | method | 1 |
| `net/minecraft/world/level/block/Block#getShape` | method | 1 |
| `net/minecraft/world/level/block/Block#getSoundType` | method | 1 |
| `net/minecraft/world/level/block/Block#playerDestroy` | method | 1 |
| `net/minecraft/world/level/block/Block#playerWillDestroy` | method | 1 |
| `net/minecraft/world/level/block/Block#randomTick` | method | 10 |
| `net/minecraft/world/level/block/Block#skipRendering` | method | 2 |
| `net/minecraft/world/level/block/Block#stepOn` | method | 2 |
| `net/minecraft/world/level/block/Block#useItemOn` | method | 1 |
| `net/minecraft/world/level/block/Blocks#AIR` | field | 47 |
| `net/minecraft/world/level/block/Blocks#ANCIENT_DEBRIS` | field | 3 |
| `net/minecraft/world/level/block/Blocks#BASALT` | field | 7 |
| `net/minecraft/world/level/block/Blocks#BEDROCK` | field | 4 |
| `net/minecraft/world/level/block/Blocks#BLACKSTONE` | field | 4 |
| `net/minecraft/world/level/block/Blocks#BONE_BLOCK` | field | 3 |
| `net/minecraft/world/level/block/Blocks#BOOKSHELF` | field | 1 |
| `net/minecraft/world/level/block/Blocks#BROWN_MUSHROOM` | field | 2 |
| `net/minecraft/world/level/block/Blocks#CARVED_PUMPKIN` | field | 1 |
| `net/minecraft/world/level/block/Blocks#CAULDRON` | field | 2 |
| `net/minecraft/world/level/block/Blocks#CONCRETE` | field | 2 |
| `net/minecraft/world/level/block/Blocks#CRIMSON_FUNGUS` | field | 1 |
| `net/minecraft/world/level/block/Blocks#CRIMSON_HYPHAE` | field | 1 |
| `net/minecraft/world/level/block/Blocks#CRIMSON_NYLIUM` | field | 3 |
| `net/minecraft/world/level/block/Blocks#CRIMSON_PLANKS` | field | 2 |
| `net/minecraft/world/level/block/Blocks#CRIMSON_ROOTS` | field | 1 |
| `net/minecraft/world/level/block/Blocks#CRIMSON_STEM` | field | 1 |
| `net/minecraft/world/level/block/Blocks#CRYING_OBSIDIAN` | field | 4 |
| `net/minecraft/world/level/block/Blocks#DEEPSLATE` | field | 1 |
| `net/minecraft/world/level/block/Blocks#GLAZED_TERRACOTTA` | field | 1 |
| `net/minecraft/world/level/block/Blocks#GLOWSTONE` | field | 4 |
| `net/minecraft/world/level/block/Blocks#GOLD_BLOCK` | field | 1 |
| `net/minecraft/world/level/block/Blocks#GRAVEL` | field | 2 |
| `net/minecraft/world/level/block/Blocks#HOPPER` | field | 1 |
| `net/minecraft/world/level/block/Blocks#JACK_O_LANTERN` | field | 1 |
| `net/minecraft/world/level/block/Blocks#LANTERN` | field | 2 |
| `net/minecraft/world/level/block/Blocks#LAPIS_BLOCK` | field | 1 |
| `net/minecraft/world/level/block/Blocks#LAVA` | field | 4 |
| `net/minecraft/world/level/block/Blocks#LECTERN` | field | 1 |
| `net/minecraft/world/level/block/Blocks#MAGMA_BLOCK` | field | 6 |
| `net/minecraft/world/level/block/Blocks#MOSS_BLOCK` | field | 1 |
| `net/minecraft/world/level/block/Blocks#NETHERITE_BLOCK` | field | 2 |
| `net/minecraft/world/level/block/Blocks#NETHERRACK` | field | 15 |
| `net/minecraft/world/level/block/Blocks#NETHER_BRICKS` | field | 7 |
| `net/minecraft/world/level/block/Blocks#NETHER_BRICK_FENCE` | field | 1 |
| `net/minecraft/world/level/block/Blocks#NETHER_BRICK_SLAB` | field | 1 |
| `net/minecraft/world/level/block/Blocks#NETHER_BRICK_STAIRS` | field | 1 |
| `net/minecraft/world/level/block/Blocks#NETHER_PORTAL` | field | 1 |
| `net/minecraft/world/level/block/Blocks#NETHER_WART` | field | 4 |
| `net/minecraft/world/level/block/Blocks#NETHER_WART_BLOCK` | field | 8 |
| `net/minecraft/world/level/block/Blocks#OBSIDIAN` | field | 9 |
| `net/minecraft/world/level/block/Blocks#PISTON` | field | 1 |
| `net/minecraft/world/level/block/Blocks#POLISHED_BASALT` | field | 1 |
| `net/minecraft/world/level/block/Blocks#RAIL` | field | 1 |
| `net/minecraft/world/level/block/Blocks#RED_MUSHROOM` | field | 2 |
| `net/minecraft/world/level/block/Blocks#RED_MUSHROOM_BLOCK` | field | 1 |
| `net/minecraft/world/level/block/Blocks#RED_SAND` | field | 2 |
| `net/minecraft/world/level/block/Blocks#SAND` | field | 2 |
| `net/minecraft/world/level/block/Blocks#SANDSTONE` | field | 1 |
| `net/minecraft/world/level/block/Blocks#SCULK` | field | 8 |
| `net/minecraft/world/level/block/Blocks#SCULK_CATALYST` | field | 2 |
| `net/minecraft/world/level/block/Blocks#SCULK_SENSOR` | field | 1 |
| `net/minecraft/world/level/block/Blocks#SCULK_SHRIEKER` | field | 1 |
| `net/minecraft/world/level/block/Blocks#SCULK_VEIN` | field | 2 |
| `net/minecraft/world/level/block/Blocks#SHULKER_BOX` | field | 1 |
| `net/minecraft/world/level/block/Blocks#SKELETON_SKULL` | field | 1 |
| `net/minecraft/world/level/block/Blocks#SMITHING_TABLE` | field | 1 |
| `net/minecraft/world/level/block/Blocks#SMOOTH_BASALT` | field | 1 |
| `net/minecraft/world/level/block/Blocks#SOUL_SAND` | field | 14 |
| `net/minecraft/world/level/block/Blocks#SOUL_SOIL` | field | 5 |
| `net/minecraft/world/level/block/Blocks#STAINED_GLASS` | field | 1 |
| `net/minecraft/world/level/block/Blocks#STONECUTTER` | field | 1 |
| `net/minecraft/world/level/block/Blocks#STRUCTURE_BLOCK` | field | 1 |
| `net/minecraft/world/level/block/Blocks#TORCH` | field | 1 |
| `net/minecraft/world/level/block/Blocks#TWISTING_VINES` | field | 1 |
| `net/minecraft/world/level/block/Blocks#TWISTING_VINES_PLANT` | field | 1 |
| `net/minecraft/world/level/block/Blocks#WALL_TORCH` | field | 1 |
| `net/minecraft/world/level/block/Blocks#WARPED_FUNGUS` | field | 1 |
| `net/minecraft/world/level/block/Blocks#WARPED_HYPHAE` | field | 1 |
| `net/minecraft/world/level/block/Blocks#WARPED_NYLIUM` | field | 3 |
| `net/minecraft/world/level/block/Blocks#WARPED_PLANKS` | field | 2 |
| `net/minecraft/world/level/block/Blocks#WARPED_ROOTS` | field | 1 |
| `net/minecraft/world/level/block/Blocks#WARPED_STEM` | field | 1 |
| `net/minecraft/world/level/block/Blocks#WITHER_SKELETON_SKULL` | field | 2 |
| `net/minecraft/world/level/block/Blocks#WOOL` | field | 3 |
| `net/minecraft/world/level/block/BrewingStandBlock#<init>` | method | 1 |
| `net/minecraft/world/level/block/BrewingStandBlock#HAS_BOTTLE` | field | 1 |
| `net/minecraft/world/level/block/BrewingStandBlock#affectNeighborsAfterRemoval` | method | 1 |
| `net/minecraft/world/level/block/ButtonBlock#<init>` | method | 1 |
| `net/minecraft/world/level/block/ColorCollection#black` | method | 6 |
| `net/minecraft/world/level/block/ColorCollection#blue` | method | 3 |
| `net/minecraft/world/level/block/ColorCollection#brown` | method | 2 |
| `net/minecraft/world/level/block/ColorCollection#cyan` | method | 2 |
| `net/minecraft/world/level/block/ColorCollection#gray` | method | 2 |
| `net/minecraft/world/level/block/ColorCollection#green` | method | 3 |
| `net/minecraft/world/level/block/ColorCollection#lightBlue` | method | 3 |
| `net/minecraft/world/level/block/ColorCollection#lightGray` | method | 1 |
| `net/minecraft/world/level/block/ColorCollection#lime` | method | 3 |
| `net/minecraft/world/level/block/ColorCollection#magenta` | method | 2 |
| `net/minecraft/world/level/block/ColorCollection#orange` | method | 4 |
| `net/minecraft/world/level/block/ColorCollection#pink` | method | 3 |
| `net/minecraft/world/level/block/ColorCollection#purple` | method | 2 |
| `net/minecraft/world/level/block/ColorCollection#red` | method | 7 |
| `net/minecraft/world/level/block/ColorCollection#white` | method | 3 |
| `net/minecraft/world/level/block/ColorCollection#yellow` | method | 4 |
| `net/minecraft/world/level/block/DirectionalBlock#FACING` | field | 1 |
| `net/minecraft/world/level/block/DoorBlock#<init>` | method | 1 |
| `net/minecraft/world/level/block/DoorBlock#FACING` | field | 1 |
| `net/minecraft/world/level/block/DoorBlock#HALF` | field | 2 |
| `net/minecraft/world/level/block/DoorBlock#HINGE` | field | 1 |
| `net/minecraft/world/level/block/DoorBlock#OPEN` | field | 1 |
| `net/minecraft/world/level/block/DoorBlock#POWERED` | field | 1 |
| `net/minecraft/world/level/block/DropExperienceBlock#<init>` | method | 1 |
| `net/minecraft/world/level/block/FenceBlock#EAST` | field | 1 |
| `net/minecraft/world/level/block/FenceBlock#NORTH` | field | 1 |
| `net/minecraft/world/level/block/FenceBlock#SOUTH` | field | 1 |
| `net/minecraft/world/level/block/FenceBlock#WATERLOGGED` | field | 1 |
| `net/minecraft/world/level/block/FenceBlock#WEST` | field | 1 |
| `net/minecraft/world/level/block/FenceGateBlock#FACING` | field | 1 |
| `net/minecraft/world/level/block/FenceGateBlock#IN_WALL` | field | 1 |
| `net/minecraft/world/level/block/FenceGateBlock#OPEN` | field | 1 |
| `net/minecraft/world/level/block/FenceGateBlock#POWERED` | field | 1 |
| `net/minecraft/world/level/block/HorizontalDirectionalBlock#FACING` | field | 9 |
| `net/minecraft/world/level/block/HugeMushroomBlock#<init>` | method | 1 |
| `net/minecraft/world/level/block/HugeMushroomBlock#DOWN` | field | 1 |
| `net/minecraft/world/level/block/HugeMushroomBlock#EAST` | field | 3 |
| `net/minecraft/world/level/block/HugeMushroomBlock#NORTH` | field | 3 |
| `net/minecraft/world/level/block/HugeMushroomBlock#SOUTH` | field | 3 |
| `net/minecraft/world/level/block/HugeMushroomBlock#UP` | field | 1 |
| `net/minecraft/world/level/block/HugeMushroomBlock#WEST` | field | 3 |
| `net/minecraft/world/level/block/IronBarsBlock#<init>` | method | 1 |
| `net/minecraft/world/level/block/IronBarsBlock#EAST` | field | 1 |
| `net/minecraft/world/level/block/IronBarsBlock#NORTH` | field | 1 |
| `net/minecraft/world/level/block/IronBarsBlock#SOUTH` | field | 1 |
| `net/minecraft/world/level/block/IronBarsBlock#WATERLOGGED` | field | 1 |
| `net/minecraft/world/level/block/IronBarsBlock#WEST` | field | 1 |
| `net/minecraft/world/level/block/IronBarsBlock#skipRendering` | method | 1 |
| `net/minecraft/world/level/block/LanternBlock#HANGING` | field | 2 |
| `net/minecraft/world/level/block/LayeredCauldronBlock#LEVEL` | field | 1 |
| `net/minecraft/world/level/block/LeavesBlock#DISTANCE` | field | 4 |
| `net/minecraft/world/level/block/LeavesBlock#PERSISTENT` | field | 6 |
| `net/minecraft/world/level/block/LightningRodBlock#POWERED` | field | 1 |
| `net/minecraft/world/level/block/LightningRodBlock#onLightningStrike` | method | 1 |
| `net/minecraft/world/level/block/Mirror#FRONT_BACK` | field | 3 |
| `net/minecraft/world/level/block/Mirror#LEFT_RIGHT` | field | 3 |
| `net/minecraft/world/level/block/Mirror#NONE` | field | 1 |
| `net/minecraft/world/level/block/Mirror#getRotation` | method | 1 |
| `net/minecraft/world/level/block/Mirror#ordinal` | method | 1 |
| `net/minecraft/world/level/block/Mirror#values` | method | 1 |
| `net/minecraft/world/level/block/MushroomBlock#canSurvive` | method | 1 |
| `net/minecraft/world/level/block/MushroomBlock#performBonemeal` | method | 1 |
| `net/minecraft/world/level/block/NetherPortalBlock#AXIS` | field | 1 |
| `net/minecraft/world/level/block/NetherWartBlock#AGE` | field | 2 |
| `net/minecraft/world/level/block/NetherWartBlock#mayPlaceOn` | method | 1 |
| `net/minecraft/world/level/block/NetherWartBlock#randomTick` | method | 1 |
| `net/minecraft/world/level/block/RedStoneOreBlock#<init>` | method | 1 |
| `net/minecraft/world/level/block/RenderShape#MODEL` | field | 1 |
| `net/minecraft/world/level/block/RotatedPillarBlock#<init>` | method | 4 |
| `net/minecraft/world/level/block/RotatedPillarBlock#AXIS` | field | 1 |
| `net/minecraft/world/level/block/RotatedPillarBlock#createBlockStateDefinition` | method | 2 |
| `net/minecraft/world/level/block/RotatedPillarBlock#defaultBlockState` | method | 1 |
| `net/minecraft/world/level/block/Rotation#CLOCKWISE_180` | field | 4 |
| `net/minecraft/world/level/block/Rotation#CLOCKWISE_90` | field | 5 |
| `net/minecraft/world/level/block/Rotation#COUNTERCLOCKWISE_90` | field | 4 |
| `net/minecraft/world/level/block/Rotation#NONE` | field | 3 |
| `net/minecraft/world/level/block/Rotation#ordinal` | method | 5 |
| `net/minecraft/world/level/block/Rotation#rotate` | method | 1 |
| `net/minecraft/world/level/block/Rotation#values` | method | 4 |
| `net/minecraft/world/level/block/SculkShriekerBlock#CAN_SUMMON` | field | 1 |
| `net/minecraft/world/level/block/SlabBlock#TYPE` | field | 1 |
| `net/minecraft/world/level/block/SlabBlock#WATERLOGGED` | field | 1 |
| `net/minecraft/world/level/block/SlabBlock#defaultBlockState` | method | 1 |
| `net/minecraft/world/level/block/SoundType#<init>` | method | 1 |
| `net/minecraft/world/level/block/SoundType#AMETHYST_CLUSTER` | field | 1 |
| `net/minecraft/world/level/block/SoundType#CROP` | field | 2 |
| `net/minecraft/world/level/block/SoundType#FUNGUS` | field | 1 |
| `net/minecraft/world/level/block/SoundType#IRON` | field | 2 |
| `net/minecraft/world/level/block/SoundType#NETHER_ORE` | field | 1 |
| `net/minecraft/world/level/block/SoundType#NETHER_WOOD` | field | 4 |
| `net/minecraft/world/level/block/SoundType#SAND` | field | 1 |
| `net/minecraft/world/level/block/SoundType#SLIME_BLOCK` | field | 1 |
| `net/minecraft/world/level/block/SoundType#STEM` | field | 2 |
| `net/minecraft/world/level/block/SoundType#VINE` | field | 1 |
| `net/minecraft/world/level/block/SoundType#WART_BLOCK` | field | 1 |
| `net/minecraft/world/level/block/SoundType#WOOD` | field | 3 |
| `net/minecraft/world/level/block/SoundType#WOOL` | field | 1 |
| `net/minecraft/world/level/block/SoundType#getBreakSound` | method | 1 |
| `net/minecraft/world/level/block/StairBlock#FACING` | field | 1 |
| `net/minecraft/world/level/block/StairBlock#HALF` | field | 1 |
| `net/minecraft/world/level/block/StairBlock#SHAPE` | field | 1 |
| `net/minecraft/world/level/block/StairBlock#WATERLOGGED` | field | 1 |
| `net/minecraft/world/level/block/TintedParticleLeavesBlock#<init>` | method | 2 |
| `net/minecraft/world/level/block/TrapDoorBlock#FACING` | field | 1 |
| `net/minecraft/world/level/block/TrapDoorBlock#HALF` | field | 1 |
| `net/minecraft/world/level/block/TrapDoorBlock#OPEN` | field | 1 |
| `net/minecraft/world/level/block/TrapDoorBlock#POWERED` | field | 1 |
| `net/minecraft/world/level/block/TrapDoorBlock#WATERLOGGED` | field | 1 |
| `net/minecraft/world/level/block/VegetationBlock#<init>` | method | 1 |
| `net/minecraft/world/level/block/VegetationBlock#randomTick` | method | 1 |
| `net/minecraft/world/level/block/WallBlock#EAST` | field | 1 |
| `net/minecraft/world/level/block/WallBlock#NORTH` | field | 1 |
| `net/minecraft/world/level/block/WallBlock#SOUTH` | field | 1 |
| `net/minecraft/world/level/block/WallBlock#UP` | field | 1 |
| `net/minecraft/world/level/block/WallBlock#WEST` | field | 1 |
| `net/minecraft/world/level/block/WallBlock#defaultBlockState` | method | 1 |
| `net/minecraft/world/level/block/WallTorchBlock#FACING` | field | 1 |
| `net/minecraft/world/level/block/entity/AbstractFurnaceBlockEntity#<init>` | method | 2 |
| `net/minecraft/world/level/block/entity/AbstractFurnaceBlockEntity#awardUsedRecipesAndPopExperience` | method | 1 |
| `net/minecraft/world/level/block/entity/AbstractFurnaceBlockEntity#getBurnDuration` | method | 2 |
| `net/minecraft/world/level/block/entity/AbstractFurnaceBlockEntity#getTotalCookTime` | method | 1 |
| `net/minecraft/world/level/block/entity/BaseContainerBlockEntity#<init>` | method | 1 |
| `net/minecraft/world/level/block/entity/BaseContainerBlockEntity#loadAdditional` | method | 1 |
| `net/minecraft/world/level/block/entity/BaseContainerBlockEntity#saveAdditional` | method | 1 |
| `net/minecraft/world/level/block/entity/BlockEntity#getBlockPos` | method | 1 |
| `net/minecraft/world/level/block/entity/RandomizableContainerBlockEntity#<init>` | method | 1 |
| `net/minecraft/world/level/block/entity/RandomizableContainerBlockEntity#loadAdditional` | method | 1 |
| `net/minecraft/world/level/block/entity/RandomizableContainerBlockEntity#saveAdditional` | method | 1 |
| `net/minecraft/world/level/block/state/BlockBehaviour#properties` | method | 1 |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase#isAir` | method | 1 |
| `net/minecraft/world/level/block/state/BlockBehaviour$OffsetFunction#evaluate` | method | 1 |
| `net/minecraft/world/level/block/state/BlockBehaviour$OffsetType#NONE` | field | 7 |
| `net/minecraft/world/level/block/state/BlockBehaviour$OffsetType#XZ` | field | 3 |
| `net/minecraft/world/level/block/state/BlockBehaviour$Properties#bounceRestitution` | method | 1 |
| `net/minecraft/world/level/block/state/BlockBehaviour$Properties#lightLevel` | method | 5 |
| `net/minecraft/world/level/block/state/BlockBehaviour$Properties#mapColor` | method | 2 |
| `net/minecraft/world/level/block/state/BlockBehaviour$Properties#noOcclusion` | method | 3 |
| `net/minecraft/world/level/block/state/BlockBehaviour$Properties#ofFullCopy` | method | 6 |
| `net/minecraft/world/level/block/state/BlockBehaviour$Properties#offsetFunction` | method | 2 |
| `net/minecraft/world/level/block/state/BlockBehaviour$Properties#requiresCorrectToolForDrops` | method | 4 |
| `net/minecraft/world/level/block/state/BlockBehaviour$Properties#sound` | method | 2 |
| `net/minecraft/world/level/block/state/BlockBehaviour$Properties#strength` | method | 2 |
| `net/minecraft/world/level/block/state/BlockState#getBlock` | method | 58 |
| `net/minecraft/world/level/block/state/BlockState#getFluidState` | method | 3 |
| `net/minecraft/world/level/block/state/BlockState#getLightEmission` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#getMapColor` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#getOptionalValue` | method | 9 |
| `net/minecraft/world/level/block/state/BlockState#getValue` | method | 52 |
| `net/minecraft/world/level/block/state/BlockState#hasProperty` | method | 7 |
| `net/minecraft/world/level/block/state/BlockState#is` | method | 29 |
| `net/minecraft/world/level/block/state/BlockState#isAir` | method | 21 |
| `net/minecraft/world/level/block/state/BlockState#isCollisionShapeFullBlock` | method | 4 |
| `net/minecraft/world/level/block/state/BlockState#isFaceSturdy` | method | 10 |
| `net/minecraft/world/level/block/state/BlockState#isSolid` | method | 2 |
| `net/minecraft/world/level/block/state/BlockState#liquid` | method | 2 |
| `net/minecraft/world/level/block/state/BlockState#rotate` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#setValue` | method | 72 |
| `net/minecraft/world/level/block/state/BlockState#spawnAfterBreak` | method | 1 |
| `net/minecraft/world/level/block/state/StateDefinition#any` | method | 26 |
| `net/minecraft/world/level/block/state/StateDefinition$Builder#add` | method | 39 |
| `net/minecraft/world/level/block/state/properties/BlockSetType#CRIMSON` | field | 2 |
| `net/minecraft/world/level/block/state/properties/BlockSetType#IRON` | field | 1 |
| `net/minecraft/world/level/block/state/properties/BlockSetType#OAK` | field | 1 |
| `net/minecraft/world/level/block/state/properties/BlockStateProperties#AXIS` | field | 2 |
| `net/minecraft/world/level/block/state/properties/BlockStateProperties#DISTANCE` | field | 4 |
| `net/minecraft/world/level/block/state/properties/BlockStateProperties#FACING` | field | 5 |
| `net/minecraft/world/level/block/state/properties/BlockStateProperties#HANGING` | field | 1 |
| `net/minecraft/world/level/block/state/properties/BlockStateProperties#LIT` | field | 3 |
| `net/minecraft/world/level/block/state/properties/BlockStateProperties#PERSISTENT` | field | 2 |
| `net/minecraft/world/level/block/state/properties/BooleanProperty#create` | method | 7 |
| `net/minecraft/world/level/block/state/properties/DoubleBlockHalf#UPPER` | field | 1 |
| `net/minecraft/world/level/block/state/properties/EnumProperty#create` | method | 4 |
| `net/minecraft/world/level/block/state/properties/EnumProperty#getPossibleValues` | method | 4 |
| `net/minecraft/world/level/block/state/properties/NoteBlockInstrument#BASEDRUM` | field | 2 |
| `net/minecraft/world/level/block/state/properties/NoteBlockInstrument#HARP` | field | 1 |
| `net/minecraft/world/level/block/state/properties/NoteBlockInstrument#IRON_XYLOPHONE` | field | 1 |
| `net/minecraft/world/level/block/state/properties/WoodType#CRIMSON` | field | 2 |
| `net/minecraft/world/level/block/state/properties/WoodType#OAK` | field | 1 |
| `net/minecraft/world/level/block/state/properties/WoodType#setType` | method | 1 |
| `net/minecraft/world/level/chunk/ChunkAccess#getAllReferences` | method | 1 |
| `net/minecraft/world/level/chunk/ChunkAccess#getStartForStructure` | method | 1 |
| `net/minecraft/world/level/chunk/ChunkAccess#markPosForPostProcessing` | method | 2 |
| `net/minecraft/world/level/chunk/ChunkGenerator#getGenDepth` | method | 6 |
| `net/minecraft/world/level/chunk/ChunkGenerator#getMinY` | method | 1 |
| `net/minecraft/world/level/chunk/ChunkGenerator#getSeaLevel` | method | 1 |
| `net/minecraft/world/level/chunk/LevelChunk#getBlockState` | method | 1 |
| `net/minecraft/world/level/chunk/LevelChunk#getHeight` | method | 1 |
| `net/minecraft/world/level/chunk/LevelChunk#isEmpty` | method | 1 |
| `net/minecraft/world/level/chunk/status/ChunkStatus#STRUCTURE_REFERENCES` | field | 1 |
| `net/minecraft/world/level/chunk/status/ChunkStatus#STRUCTURE_STARTS` | field | 1 |
| `net/minecraft/world/level/dimension/DimensionType#hasCeiling` | method | 2 |
| `net/minecraft/world/level/dimension/DimensionType#minY` | method | 2 |
| `net/minecraft/world/level/dimension/LevelStem#NETHER` | field | 1 |
| `net/minecraft/world/level/dimension/LevelStem#OVERWORLD` | field | 1 |
| `net/minecraft/world/level/gameevent/GameEvent#SHEAR` | field | 1 |
| `net/minecraft/world/level/gamerules/GameRuleCategory#DROPS` | field | 1 |
| `net/minecraft/world/level/gamerules/GameRuleCategory#MISC` | field | 1 |
| `net/minecraft/world/level/gamerules/GameRuleCategory#UPDATES` | field | 1 |
| `net/minecraft/world/level/gamerules/GameRules#BLOCK_DROPS` | field | 1 |
| `net/minecraft/world/level/gamerules/GameRules#ENTITY_DROPS` | field | 1 |
| `net/minecraft/world/level/gamerules/GameRules#get` | method | 6 |
| `net/minecraft/world/level/levelgen/GenerationStep$Decoration#FLUID_SPRINGS` | field | 1 |
| `net/minecraft/world/level/levelgen/GenerationStep$Decoration#LAKES` | field | 1 |
| `net/minecraft/world/level/levelgen/GenerationStep$Decoration#RAW_GENERATION` | field | 1 |
| `net/minecraft/world/level/levelgen/GenerationStep$Decoration#SURFACE_STRUCTURES` | field | 2 |
| `net/minecraft/world/level/levelgen/GenerationStep$Decoration#UNDERGROUND_DECORATION` | field | 2 |
| `net/minecraft/world/level/levelgen/GenerationStep$Decoration#VEGETAL_DECORATION` | field | 7 |
| `net/minecraft/world/level/levelgen/Heightmap$Types#MOTION_BLOCKING` | field | 1 |
| `net/minecraft/world/level/levelgen/Heightmap$Types#MOTION_BLOCKING_NO_LEAVES` | field | 1 |
| `net/minecraft/world/level/levelgen/Heightmap$Types#WORLD_SURFACE` | field | 2 |
| `net/minecraft/world/level/levelgen/Heightmap$Types#WORLD_SURFACE_WG` | field | 1 |
| `net/minecraft/world/level/levelgen/Heightmap$Types#isOpaque` | method | 1 |
| `net/minecraft/world/level/levelgen/LegacyRandomSource#<init>` | method | 6 |
| `net/minecraft/world/level/levelgen/Noises#NETHERRACK` | field | 4 |
| `net/minecraft/world/level/levelgen/Noises#NETHER_STATE_SELECTOR` | field | 1 |
| `net/minecraft/world/level/levelgen/Noises#NETHER_WART` | field | 1 |
| `net/minecraft/world/level/levelgen/VerticalAnchor#aboveBottom` | method | 2 |
| `net/minecraft/world/level/levelgen/VerticalAnchor#belowTop` | method | 2 |
| `net/minecraft/world/level/levelgen/VerticalAnchor#bottom` | method | 1 |
| `net/minecraft/world/level/levelgen/VerticalAnchor#top` | method | 2 |
| `net/minecraft/world/level/levelgen/WorldgenRandom#<init>` | method | 2 |
| `net/minecraft/world/level/levelgen/WorldgenRandom#nextInt` | method | 1 |
| `net/minecraft/world/level/levelgen/WorldgenRandom#setLargeFeatureSeed` | method | 1 |
| `net/minecraft/world/level/levelgen/blockpredicates/BlockPredicate#alwaysTrue` | method | 2 |
| `net/minecraft/world/level/levelgen/blockpredicates/BlockPredicate#anyOf` | method | 4 |
| `net/minecraft/world/level/levelgen/blockpredicates/BlockPredicate#matchesBlocks` | method | 4 |
| `net/minecraft/world/level/levelgen/blockpredicates/BlockPredicate#matchesTag` | method | 6 |
| `net/minecraft/world/level/levelgen/blockpredicates/BlockPredicate#solid` | method | 1 |
| `net/minecraft/world/level/levelgen/feature/DeltaFeature#isClear` | method | 1 |
| `net/minecraft/world/level/levelgen/feature/MultifaceGrowthFeature#<init>` | method | 1 |
| `net/minecraft/world/level/levelgen/feature/stateproviders/WeightedStateProvider#<init>` | method | 1 |
| `net/minecraft/world/level/levelgen/flat/FlatLayerInfo#<init>` | method | 1 |
| `net/minecraft/world/level/levelgen/flat/FlatLevelGeneratorPresets$Bootstrap#run` | method | 1 |
| `net/minecraft/world/level/levelgen/material/MaterialRules#hole` | method | 1 |
| `net/minecraft/world/level/levelgen/material/MaterialRules#ifTrue` | method | 14 |
| `net/minecraft/world/level/levelgen/material/MaterialRules#noiseCondition2d` | method | 1 |
| `net/minecraft/world/level/levelgen/material/MaterialRules#not` | method | 1 |
| `net/minecraft/world/level/levelgen/material/MaterialRules#sequence` | method | 11 |
| `net/minecraft/world/level/levelgen/material/MaterialRules#state` | method | 16 |
| `net/minecraft/world/level/levelgen/material/MaterialRules#stoneDepthCheck` | method | 3 |
| `net/minecraft/world/level/levelgen/material/condition/VerticalGradientCondition#<init>` | method | 1 |
| `net/minecraft/world/level/levelgen/placement/CaveSurface#CEILING` | field | 2 |
| `net/minecraft/world/level/levelgen/placement/CaveSurface#FLOOR` | field | 2 |
| `net/minecraft/world/level/levelgen/placement/HeightRangePlacement#triangle` | method | 1 |
| `net/minecraft/world/level/levelgen/placement/HeightRangePlacement#uniform` | method | 1 |
| `net/minecraft/world/level/levelgen/placement/PlacedFeature#placement` | method | 1 |
| `net/minecraft/world/level/levelgen/placement/PlacementContext#<init>` | method | 1 |
| `net/minecraft/world/level/levelgen/placement/PlacementContext#getLevel` | method | 1 |
| `net/minecraft/world/level/levelgen/placement/PlacementModifier#getClass` | method | 1 |
| `net/minecraft/world/level/levelgen/placement/PlacementModifier#modify` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/BoundingBox#<init>` | method | 7 |
| `net/minecraft/world/level/levelgen/structure/BoundingBox#encapsulate` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/BoundingBox#intersects` | method | 3 |
| `net/minecraft/world/level/levelgen/structure/BoundingBox#isInside` | method | 5 |
| `net/minecraft/world/level/levelgen/structure/BoundingBox#maxX` | method | 8 |
| `net/minecraft/world/level/levelgen/structure/BoundingBox#maxY` | method | 4 |
| `net/minecraft/world/level/levelgen/structure/BoundingBox#maxZ` | method | 8 |
| `net/minecraft/world/level/levelgen/structure/BoundingBox#minX` | method | 9 |
| `net/minecraft/world/level/levelgen/structure/BoundingBox#minY` | method | 3 |
| `net/minecraft/world/level/levelgen/structure/BoundingBox#minZ` | method | 9 |
| `net/minecraft/world/level/levelgen/structure/BoundingBox#toString` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/BuiltinStructureSets#NETHER_COMPLEXES` | field | 1 |
| `net/minecraft/world/level/levelgen/structure/Structure#<init>` | method | 2 |
| `net/minecraft/world/level/levelgen/structure/Structure#biomes` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/Structure#findValidGenerationPoint` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/Structure$GenerationContext#<init>` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/Structure$GenerationContext#biomeSource` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/Structure$GenerationContext#chunkGenerator` | method | 3 |
| `net/minecraft/world/level/levelgen/structure/Structure$GenerationContext#chunkPos` | method | 2 |
| `net/minecraft/world/level/levelgen/structure/Structure$GenerationContext#climateSampler` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/Structure$GenerationContext#heightAccessor` | method | 3 |
| `net/minecraft/world/level/levelgen/structure/Structure$GenerationContext#random` | method | 2 |
| `net/minecraft/world/level/levelgen/structure/Structure$GenerationContext#randomState` | method | 3 |
| `net/minecraft/world/level/levelgen/structure/Structure$GenerationContext#registryAccess` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/Structure$GenerationContext#seed` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/Structure$GenerationContext#structureTemplateManager` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/Structure$GenerationStub#<init>` | method | 3 |
| `net/minecraft/world/level/levelgen/structure/Structure$GenerationStub#getPiecesBuilder` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/StructurePiece#<init>` | method | 2 |
| `net/minecraft/world/level/levelgen/structure/StructurePiece#getBoundingBox` | method | 2 |
| `net/minecraft/world/level/levelgen/structure/StructureSet#placement` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/StructureSet#structures` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/StructureSet$StructureSelectionEntry#structure` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/StructureSet$StructureSelectionEntry#weight` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/StructureStart#getPieces` | method | 2 |
| `net/minecraft/world/level/levelgen/structure/StructureStart#isValid` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/pieces/PiecesContainer#pieces` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/pieces/StructurePiecesBuilder#<init>` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/pieces/StructurePiecesBuilder#addPiece` | method | 2 |
| `net/minecraft/world/level/levelgen/structure/pieces/StructurePiecesBuilder#build` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/pieces/StructurePiecesBuilder#getBoundingBox` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/placement/RandomSpreadStructurePlacement#applyAdditionalChunkRestrictions` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/placement/RandomSpreadStructurePlacement#getPotentialStructureChunk` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/placement/RandomSpreadStructurePlacement#spacing` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/BlockAgeProcessor#processBlock` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/StructurePlaceSettings#<init>` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/StructurePlaceSettings#addProcessor` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/StructurePlaceSettings#getRandom` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/StructurePlaceSettings#setBoundingBox` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/StructurePlaceSettings#setMirror` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/StructurePlaceSettings#setRotation` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/StructureTemplate#getBoundingBox` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/StructureTemplate#getSize` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/StructureTemplate$StructureBlockInfo#<init>` | method | 2 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/StructureTemplate$StructureBlockInfo#nbt` | method | 2 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/StructureTemplate$StructureBlockInfo#pos` | method | 4 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/StructureTemplate$StructureBlockInfo#state` | method | 2 |
| `net/minecraft/world/level/levelgen/synth/PerlinNoise#<init>` | method | 1 |
| `net/minecraft/world/level/levelgen/synth/PerlinNoise#get` | method | 1 |
| `net/minecraft/world/level/material/FluidState#is` | method | 1 |
| `net/minecraft/world/level/material/FluidState#isEmpty` | method | 5 |
| `net/minecraft/world/level/material/FluidState#isSource` | method | 1 |
| `net/minecraft/world/level/material/Fluids#EMPTY` | field | 9 |
| `net/minecraft/world/level/material/LavaFluid#animateTick` | method | 1 |
| `net/minecraft/world/level/material/MapColor#COLOR_BLACK` | field | 2 |
| `net/minecraft/world/level/material/MapColor#COLOR_BLUE` | field | 6 |
| `net/minecraft/world/level/material/MapColor#COLOR_BROWN` | field | 5 |
| `net/minecraft/world/level/material/MapColor#COLOR_CYAN` | field | 5 |
| `net/minecraft/world/level/material/MapColor#COLOR_GRAY` | field | 4 |
| `net/minecraft/world/level/material/MapColor#COLOR_GREEN` | field | 4 |
| `net/minecraft/world/level/material/MapColor#COLOR_LIGHT_BLUE` | field | 2 |
| `net/minecraft/world/level/material/MapColor#COLOR_LIGHT_GRAY` | field | 2 |
| `net/minecraft/world/level/material/MapColor#COLOR_LIGHT_GREEN` | field | 3 |
| `net/minecraft/world/level/material/MapColor#COLOR_MAGENTA` | field | 1 |
| `net/minecraft/world/level/material/MapColor#COLOR_ORANGE` | field | 5 |
| `net/minecraft/world/level/material/MapColor#COLOR_PINK` | field | 3 |
| `net/minecraft/world/level/material/MapColor#COLOR_PURPLE` | field | 1 |
| `net/minecraft/world/level/material/MapColor#COLOR_RED` | field | 5 |
| `net/minecraft/world/level/material/MapColor#COLOR_YELLOW` | field | 3 |
| `net/minecraft/world/level/material/MapColor#CRIMSON_STEM` | field | 1 |
| `net/minecraft/world/level/material/MapColor#DIAMOND` | field | 1 |
| `net/minecraft/world/level/material/MapColor#NONE` | field | 1 |
| `net/minecraft/world/level/material/MapColor#TERRACOTTA_GRAY` | field | 1 |
| `net/minecraft/world/level/material/MapColor#TERRACOTTA_LIGHT_GREEN` | field | 2 |
| `net/minecraft/world/level/material/MapColor#TERRACOTTA_ORANGE` | field | 3 |
| `net/minecraft/world/level/material/MapColor#TERRACOTTA_RED` | field | 3 |
| `net/minecraft/world/level/material/MapColor#TERRACOTTA_WHITE` | field | 4 |
| `net/minecraft/world/level/material/MapColor#WARPED_STEM` | field | 1 |
| `net/minecraft/world/level/material/MapColor#WATER` | field | 1 |
| `net/minecraft/world/level/material/MapColor#getPackedId` | method | 1 |
| `net/minecraft/world/level/material/MapColor$Brightness#HIGH` | field | 1 |
| `net/minecraft/world/level/material/MapColor$Brightness#LOW` | field | 1 |
| `net/minecraft/world/level/material/MapColor$Brightness#NORMAL` | field | 1 |
| `net/minecraft/world/level/material/PushReaction#IMMOVEABLE` | field | 1 |
| `net/minecraft/world/level/material/PushReaction#POPPED` | field | 2 |
| `net/minecraft/world/level/material/PushReaction#PUSH_PULL` | field | 1 |
| `net/minecraft/world/level/pathfinder/PathType#FIRE_IN_NEIGHBOR` | field | 2 |
| `net/minecraft/world/level/pathfinder/PathType#LAVA` | field | 3 |
| `net/minecraft/world/level/pathfinder/PathType#WATER` | field | 3 |
| `net/minecraft/world/level/portal/PortalShape#findAnyShape` | method | 1 |
| `net/minecraft/world/level/portal/PortalShape#isComplete` | method | 1 |
| `net/minecraft/world/level/portal/PortalShape#isValid` | method | 1 |
| `net/minecraft/world/level/portal/TeleportTransition#<init>` | method | 1 |
| `net/minecraft/world/level/saveddata/maps/MapItemSavedData#centerX` | field | 1 |
| `net/minecraft/world/level/saveddata/maps/MapItemSavedData#centerZ` | field | 1 |
| `net/minecraft/world/level/saveddata/maps/MapItemSavedData#checkBanners` | method | 1 |
| `net/minecraft/world/level/saveddata/maps/MapItemSavedData#dimension` | field | 1 |
| `net/minecraft/world/level/saveddata/maps/MapItemSavedData#getHoldingPlayer` | method | 1 |
| `net/minecraft/world/level/saveddata/maps/MapItemSavedData#scale` | field | 1 |
| `net/minecraft/world/level/saveddata/maps/MapItemSavedData#updateColor` | method | 1 |
| `net/minecraft/world/level/saveddata/maps/MapItemSavedData$HoldingPlayer#step` | field | 1 |
| `net/minecraft/world/level/storage/LevelData$RespawnData#dimension` | method | 1 |
| `net/minecraft/world/level/storage/LevelData$RespawnData#of` | method | 1 |
| `net/minecraft/world/level/storage/LevelData$RespawnData#pos` | method | 1 |
| `net/minecraft/world/level/storage/LevelData$RespawnData#yaw` | method | 1 |
| `net/minecraft/world/level/storage/ValueInput#getByteOr` | method | 2 |
| `net/minecraft/world/level/storage/ValueInput#getFloatOr` | method | 1 |
| `net/minecraft/world/level/storage/ValueInput#getInt` | method | 1 |
| `net/minecraft/world/level/storage/ValueInput#getIntOr` | method | 1 |
| `net/minecraft/world/level/storage/ValueInput#getShortOr` | method | 1 |
| `net/minecraft/world/level/storage/ValueOutput#putByte` | method | 2 |
| `net/minecraft/world/level/storage/ValueOutput#putInt` | method | 2 |
| `net/minecraft/world/level/storage/ValueOutput#putShort` | method | 1 |
| `net/minecraft/world/level/storage/loot/BuiltInLootTables#BASTION_BRIDGE` | field | 1 |
| `net/minecraft/world/level/storage/loot/BuiltInLootTables#BASTION_HOGLIN_STABLE` | field | 1 |
| `net/minecraft/world/level/storage/loot/BuiltInLootTables#BASTION_OTHER` | field | 1 |
| `net/minecraft/world/level/storage/loot/BuiltInLootTables#BASTION_TREASURE` | field | 1 |
| `net/minecraft/world/level/storage/loot/BuiltInLootTables#NETHER_BRIDGE` | field | 1 |
| `net/minecraft/world/level/storage/loot/BuiltInLootTables#PIGLIN_BARTERING` | field | 1 |
| `net/minecraft/world/level/storage/loot/BuiltInLootTables#RUINED_PORTAL` | field | 1 |
| `net/minecraft/world/level/storage/loot/LootParams$Builder#getParameter` | method | 1 |
| `net/minecraft/world/level/storage/loot/LootPool#lootPool` | method | 4 |
| `net/minecraft/world/level/storage/loot/LootPool$Builder#add` | method | 4 |
| `net/minecraft/world/level/storage/loot/LootPool$Builder#apply` | method | 1 |
| `net/minecraft/world/level/storage/loot/LootPool$Builder#setRolls` | method | 4 |
| `net/minecraft/world/level/storage/loot/LootPool$Builder#when` | method | 2 |
| `net/minecraft/world/level/storage/loot/LootTable#lootTable` | method | 3 |
| `net/minecraft/world/level/storage/loot/LootTable$Builder#withPool` | method | 3 |
| `net/minecraft/world/level/storage/loot/entries/AlternativesEntry#alternatives` | method | 1 |
| `net/minecraft/world/level/storage/loot/entries/AlternativesEntry$Builder#otherwise` | method | 1 |
| `net/minecraft/world/level/storage/loot/entries/EmptyLootItem#emptyItem` | method | 1 |
| `net/minecraft/world/level/storage/loot/entries/LootItem#lootTableItem` | method | 4 |
| `net/minecraft/world/level/storage/loot/entries/UniformContainerBase$Builder#apply` | method | 4 |
| `net/minecraft/world/level/storage/loot/entries/UniformContainerBase$Builder#otherwise` | method | 1 |
| `net/minecraft/world/level/storage/loot/entries/UniformContainerBase$Builder#setWeight` | method | 2 |
| `net/minecraft/world/level/storage/loot/entries/UniformContainerBase$Builder#when` | method | 2 |
| `net/minecraft/world/level/storage/loot/functions/EnchantRandomlyFunction$Builder#<init>` | method | 1 |
| `net/minecraft/world/level/storage/loot/functions/EnchantRandomlyFunction$Builder#withEnchantment` | method | 1 |
| `net/minecraft/world/level/storage/loot/functions/EnchantedCountIncreaseFunction#lootingMultiplier` | method | 1 |
| `net/minecraft/world/level/storage/loot/functions/SetItemCountFunction#setCount` | method | 4 |
| `net/minecraft/world/level/storage/loot/parameters/LootContextParamSets#CHEST` | field | 1 |
| `net/minecraft/world/level/storage/loot/parameters/LootContextParamSets#ENTITY` | field | 1 |
| `net/minecraft/world/level/storage/loot/parameters/LootContextParams#BLOCK_ENTITY` | field | 1 |
| `net/minecraft/world/level/storage/loot/predicates/BonusLevelTableCondition#bonusLevelFlatChance` | method | 1 |
| `net/minecraft/world/level/storage/loot/predicates/ExplosionCondition#survivesExplosion` | method | 1 |
| `net/minecraft/world/level/storage/loot/predicates/LootItemCondition$Builder#and` | method | 1 |
| `net/minecraft/world/level/storage/loot/predicates/LootItemCondition$Builder#or` | method | 1 |
| `net/minecraft/world/level/storage/loot/predicates/LootItemKilledByPlayerCondition#killedByPlayer` | method | 1 |
| `net/minecraft/world/level/storage/loot/predicates/LootItemRandomChanceCondition#randomChance` | method | 3 |
| `net/minecraft/world/level/storage/loot/predicates/LootItemRandomChanceWithEnchantedBonusCondition#randomChanceAndLootingBoost` | method | 1 |
| `net/minecraft/world/level/storage/loot/predicates/MatchBlock#blockMatches` | method | 1 |
| `net/minecraft/world/level/storage/loot/predicates/MatchTool#toolMatches` | method | 1 |
| `net/minecraft/world/level/storage/loot/providers/number/floats/ContextFloatProviders#between` | method | 1 |
| `net/minecraft/world/level/storage/loot/providers/number/floats/ContextFloatProviders#forEnchantmentLevel` | method | 1 |
| `net/minecraft/world/level/storage/loot/providers/number/ints/ContextIntProviders#between` | method | 4 |
| `net/minecraft/world/level/storage/loot/providers/number/ints/ContextIntProviders#exactly` | method | 4 |
| `net/minecraft/world/phys/AABB#<init>` | method | 6 |
| `net/minecraft/world/phys/AABB#deflate` | method | 1 |
| `net/minecraft/world/phys/AABB#inflate` | method | 5 |
| `net/minecraft/world/phys/AABB#move` | method | 1 |
| `net/minecraft/world/phys/BlockHitResult#getDirection` | method | 2 |
| `net/minecraft/world/phys/EntityHitResult#getEntity` | method | 1 |
| `net/minecraft/world/phys/HitResult#getType` | method | 1 |
| `net/minecraft/world/phys/HitResult$Type#BLOCK` | field | 1 |
| `net/minecraft/world/phys/HitResult$Type#ENTITY` | field | 1 |
| `net/minecraft/world/phys/HitResult$Type#MISS` | field | 1 |
| `net/minecraft/world/phys/Vec3#<init>` | method | 5 |
| `net/minecraft/world/phys/Vec3#ZERO` | field | 2 |
| `net/minecraft/world/phys/Vec3#add` | method | 6 |
| `net/minecraft/world/phys/Vec3#atCenterOf` | method | 1 |
| `net/minecraft/world/phys/Vec3#atLowerCornerOf` | method | 1 |
| `net/minecraft/world/phys/Vec3#horizontalDistanceSqr` | method | 1 |
| `net/minecraft/world/phys/Vec3#lengthSqr` | method | 1 |
| `net/minecraft/world/phys/Vec3#multiply` | method | 1 |
| `net/minecraft/world/phys/Vec3#reverse` | method | 1 |
| `net/minecraft/world/phys/Vec3#scale` | method | 3 |
| `net/minecraft/world/phys/Vec3#subtract` | method | 3 |
| `net/minecraft/world/phys/Vec3#x` | field | 21 |
| `net/minecraft/world/phys/Vec3#xRot` | method | 1 |
| `net/minecraft/world/phys/Vec3#y` | field | 19 |
| `net/minecraft/world/phys/Vec3#z` | field | 21 |
| `net/minecraft/world/phys/Vec3#zRot` | method | 1 |
| `net/minecraft/world/phys/shapes/CollisionContext#of` | method | 1 |
| `net/minecraft/world/phys/shapes/Shapes#block` | method | 1 |
| `net/minecraft/world/phys/shapes/Shapes#box` | method | 3 |
| `net/minecraft/world/phys/shapes/Shapes#empty` | method | 3 |
| `net/minecraft/world/phys/shapes/Shapes#or` | method | 3 |
| `net/minecraft/world/phys/shapes/VoxelShape#isEmpty` | method | 1 |
| `net/minecraft/world/phys/shapes/VoxelShape#move` | method | 9 |

</details>

