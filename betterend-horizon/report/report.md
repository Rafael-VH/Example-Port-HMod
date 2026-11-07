# Horizon port report: Better End

`betterend` 26.302.1 · better-end-26.302.1.jar · 743 classes

**Partly convertible**

| Status | Count | Meaning |
|---|---|---|
| auto | 5 | The Horizon hook or injection is known: the wiring can be generated. The Java body still has to become Rust. |
| content | 14 | Data and assets: horizon-port-pack carries these into a content mod (.hmod). |
| needs-mapping | 41 | The kind of change carries over, but the matching Horizon function isn't in the table yet. Run `match`. |
| needs-translation | 126 | No direct equivalent: a person (or an AI agent) has to rewrite this part. |
| unsupported | 6 | Horizon has nothing this can attach to yet. |

## Entry points

- `main`: org.betterx.betterend.BetterEnd
- `wover.blocks`: org.betterx.betterend.registry.EndClassification
- `wover.block.definitions`: org.betterx.datagen.betterend.blocks.EndBlockDefinitions
- `client`: org.betterx.betterend.client.BetterEndClient
- `fabric-datagen`: org.betterx.datagen.betterend.BetterEndDatagen
- `rei_client_diabled`: org.betterx.betterend.integration.rei.REIPlugin
- `emi_diabled`: org.betterx.betterend.integration.emi.EMIPlugin
- `jei_mod_plugin`: org.betterx.betterend.integration.jei.JeiPlugin

## Fabric events

| Event | In | Horizon | Status | Note |
|---|---|---|---|---|
| ClientLifecycleEvents.CLIENT_STARTED | `BetterEndSkyRenderer` | hook `frame` | auto | CLIENT_STARTED: the Fabric API layer runs it on the window's first frame. CLIENT_STOPPING has no hook in Horizon a46ce68 (c9d474d has native_world_lifecycle). |
| CommandRegistrationCallback.EVENT | `CommandRegistry` | hook `command` | needs-translation | Server commands. Horizon's command hook is client-side. |
| FabricBlockEntityTypeBuilder.build | `EndBlockEntities` | – | unsupported | Block entities aren't supported in Horizon mods yet. |
| FabricBlockEntityTypeBuilder.create | `EndBlockEntities` | – | unsupported | Block entities aren't supported in Horizon mods yet. |
| EntityRendererRegistry.register | `EndEntitiesRenders` | – | unsupported | Custom entity renderers aren't supported. |

## Mixins

| Injection | In | Horizon | Status | Note |
|---|---|---|---|---|
| @Accessor on AbstractSoundInstance.volume | `AbstractSoundInstanceAccessor.setVolume` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for AbstractSoundInstance.volume: run `match`. |
| mixin into ArmorStandRenderer with no injectors (interfaces or @Shadow only) | `ArmorStandRendererMixin` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| @Inject(RETURN) on BiomeColors.getAverageWaterColor | `BiomeColorsMixin.be_getWaterColor` | RETURN | needs-mapping | Horizon's RETURN runs at every normal exit and passes the return value on. No Horizon function known for BiomeColors.getAverageWaterColor: run `match`. |
| @Inject(RETURN) on BlockStateModelLoader.lambda$loadBlockStates$0 | `BlockStateModelLoaderMixin.be_switchModelOnLoad` | RETURN | needs-mapping | Horizon's RETURN runs at every normal exit and passes the return value on. No Horizon function known for BlockStateModelLoader.lambda$loadBlockStates$0: run `match`. |
| @Inject(HEAD) on CapeLayer.submit | `CapeLayerMixin.be_checkCustomElytra` | HEAD | needs-mapping | No Horizon function known for CapeLayer.submit: run `match`. |
| @Inject(RETURN) on GameRenderer.bobHurt | `GameRendererMixin.be_wobbleForInfusionHint` | RETURN on horizon_client_render::camera::bob_hurt | auto | Horizon's RETURN runs at every normal exit and passes the return value on. |
| mixin into HumanoidMobRenderer with no injectors (interfaces or @Shadow only) | `HumanoidMobRendererMixin` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| @Inject(RETURN) on ItemStack.getTooltipLines | `ItemStackMixin.be_getTooltip` | RETURN | needs-mapping | Horizon's RETURN runs at every normal exit and passes the return value on. No Horizon function known for ItemStack.getTooltipLines: run `match`. |
| @Inject(HEAD) on LevelRenderer.executeSolid | `LevelRendererMixin.be_captureSolidRenderPass` | HEAD | needs-mapping | No Horizon function known for LevelRenderer.executeSolid: run `match`. |
| @Inject(RETURN) on LevelRenderer.executeSolid | `LevelRendererMixin.be_releaseSolidRenderPass` | RETURN | needs-mapping | Horizon's RETURN runs at every normal exit and passes the return value on. No Horizon function known for LevelRenderer.executeSolid: run `match`. |
| @Inject(HEAD) on Minecraft.getSituationalMusic | `MinecraftClientMixin.be_getEndMusic` | HEAD | needs-mapping | No Horizon function known for Minecraft.getSituationalMusic: run `match`. |
| @Inject(HEAD) on MusicManager.tick | `MusicTrackerMixin.be_onTick` | HEAD on horizon_client::music::MusicManager::tick | auto |  |
| mixin into AvatarRenderer with no injectors (interfaces or @Shadow only) | `PlayerRendererMixin` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| @Inject(HEAD) on ChorusFlowerBlock.canSurvive | `ChorusFlowerBlockMixin.be_canSurvive` | HEAD | needs-mapping | No Horizon function known for ChorusFlowerBlock.canSurvive: run `match`. |
| @Inject(HEAD) on ChorusFlowerBlock.randomTick | `ChorusFlowerBlockMixin.be_randomTick` | HEAD | needs-mapping | No Horizon function known for ChorusFlowerBlock.randomTick: run `match`. |
| @Inject(HEAD) on ChorusFlowerBlock.placeDeadFlower | `ChorusFlowerBlockMixin.be_placeDeadFlower` | HEAD | needs-mapping | No Horizon function known for ChorusFlowerBlock.placeDeadFlower: run `match`. |
| @Inject(RETURN) on ChorusPlantBlock.getStateForPlacement | `ChorusPlantBlockMixin.be_getStateForPlacement` | RETURN | needs-mapping | Horizon's RETURN runs at every normal exit and passes the return value on. No Horizon function known for ChorusPlantBlock.getStateForPlacement: run `match`. |
| @Inject(RETURN) on ChorusPlantBlock.getStateWithConnections | `ChorusPlantBlockMixin.be_getStateForPlacement` | RETURN | needs-mapping | Horizon's RETURN runs at every normal exit and passes the return value on. No Horizon function known for ChorusPlantBlock.getStateWithConnections: run `match`. |
| @Inject(HEAD) on ChorusPlantBlock.canSurvive | `ChorusPlantBlockMixin.be_canSurvive` | HEAD | needs-mapping | No Horizon function known for ChorusPlantBlock.canSurvive: run `match`. |
| @Inject(RETURN) on ChorusPlantBlock.updateShape | `ChorusPlantBlockMixin.be_updateShape` | RETURN | needs-mapping | Horizon's RETURN runs at every normal exit and passes the return value on. No Horizon function known for ChorusPlantBlock.updateShape: run `match`. |
| @Inject(HEAD) on ChorusPlantFeature.place | `ChorusPlantFeatureMixin.be_place` | HEAD | needs-mapping | No Horizon function known for ChorusPlantFeature.place: run `match`. |
| @Inject(HEAD) on CraftingMenu.stillValid | `CraftingMenuMixin.be_stillValid` | HEAD | needs-mapping | No Horizon function known for CraftingMenu.stillValid: run `match`. |
| @Inject(HEAD) on EndCityStructure.findGenerationPoint | `EndCityFeatureMixin.be_findGenerationPoint` | HEAD | needs-mapping | No Horizon function known for EndCityStructure.findGenerationPoint: run `match`. |
| @Inject(HEAD) on EnderDragonFight.tryRespawn | `EndDragonFightMixin.be_tryRespawnDragon` | HEAD on horizon_client::game::dragon::Game<'_>::try_respawn | auto |  |
| @Inject(HEAD) on EndPlatformFeature.createEndPlatform | `EndPlatformFeatureMixin.be_createEndSpawnPlatform` | HEAD | needs-mapping | No Horizon function known for EndPlatformFeature.createEndPlatform: run `match`. |
| @Inject(HEAD) on EndPodiumFeature.place | `EndPodiumFeatureMixin.be_place` | HEAD | needs-mapping | No Horizon function known for EndPodiumFeature.place: run `match`. |
| @ModifyVariable(HEAD) on EndPodiumFeature.place | `EndPodiumFeatureMixin.be_setPosOnGround` | MODIFY_VARIABLE | needs-mapping | Selects a Rust local by name: pick it in the target function. No Horizon function known for EndPodiumFeature.place: run `match`. |
| @WrapOperation(INVOKE) on EndPortalBlock.getPortalDestination at Lnet/minecraft/world/phys/Vec3;atBottomCenterOf(Lnet/minecraft/core/Vec3i;)Lnet/minecraft/world/phys/Vec3; | `EndPortalBlockMixin.be_changeSpawnInEnd` | WRAP_OPERATION | needs-mapping | No Horizon function known for EndPortalBlock.getPortalDestination: run `match`. The call it hooks (Vec3.atBottomCenterOf) isn't mapped to a Rust function yet. |
| @Inject(HEAD) on EndSpike.getHeight | `EndSpikeMixin.be_getSpikeHeight` | HEAD | needs-mapping | No Horizon function known for EndSpike.getHeight: run `match`. |
| @Inject(HEAD) on Enderman.isBeingStaredBy | `EnderManMixin.be_isLookingAtMe` | HEAD | needs-mapping | No Horizon function known for Enderman.isBeingStaredBy: run `match`. |
| @Inject(HEAD) on ServerLevel.getRespawnData | `LevelMixin.be_getRespawnData` | HEAD | needs-mapping | No Horizon function known for ServerLevel.getRespawnData: run `match`. |
| @Inject(RETURN) on LivingEntity.createLivingAttributes | `LivingEntityMixin.be_addLivingAttributes` | RETURN | needs-mapping | Horizon's RETURN runs at every normal exit and passes the return value on. No Horizon function known for LivingEntity.createLivingAttributes: run `match`. |
| @Inject(HEAD) on LivingEntity.tickEffects | `LivingEntityMixin.be_applyEffects` | HEAD | needs-mapping | No Horizon function known for LivingEntity.tickEffects: run `match`. |
| @Inject(HEAD) on LivingEntity.canBeAffected | `LivingEntityMixin.be_canBeAffected` | HEAD | needs-mapping | No Horizon function known for LivingEntity.canBeAffected: run `match`. |
| @Inject(HEAD) on LivingEntity.hurtServer | `LivingEntityMixin.be_hurt` | HEAD | needs-mapping | No Horizon function known for LivingEntity.hurtServer: run `match`. |
| @ModifyArg(INVOKE) on LivingEntity.dealDefaultKnockback at Lnet/minecraft/world/entity/LivingEntity;knockback(DDDLnet/minecraft/world/damagesource/DamageSource;F)V | `LivingEntityMixin.be_increaseKnockback` | MODIFY_ARG | needs-mapping | No Horizon function known for LivingEntity.dealDefaultKnockback: run `match`. |
| @Accessor on NoiseBasedChunkGenerator.settings | `NoiseBasedChunkGeneratorAccessor.be_getSettings` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for NoiseBasedChunkGenerator.settings: run `match`. |
| @Inject(HEAD) on NoiseBasedChunkGenerator.getBaseHeight | `NoiseBasedChunkGeneratorHeightMixin.be_getBaseHeight` | HEAD | needs-mapping | No Horizon function known for NoiseBasedChunkGenerator.getBaseHeight: run `match`. |
| @Inject(HEAD) on NoiseBasedChunkGenerator.getBaseColumn | `NoiseBasedChunkGeneratorHeightMixin.be_getBaseColumn` | HEAD | needs-mapping | No Horizon function known for NoiseBasedChunkGenerator.getBaseColumn: run `match`. |
| @ModifyExpressionValue(INVOKE) on NoiseBasedChunkGenerator.doFill at Lnet/minecraft/world/level/levelgen/NoiseRouter;finalDensity()Lnet/minecraft/world/level/levelgen/densityfunction/DensityFunction; | `NoiseBasedChunkGeneratorTerrainMixin.be_islandTerrain` | MODIFY_EXPRESSION_VALUE | needs-mapping | Selects an exact Rust expression: pick it in the target function. No Horizon function known for NoiseBasedChunkGenerator.doFill: run `match`. |
| @ModifyExpressionValue(INVOKE) on NoiseBasedChunkGenerator.iterateNoiseColumn at Lnet/minecraft/world/level/levelgen/NoiseRouter;finalDensity()Lnet/minecraft/world/level/levelgen/densityfunction/DensityFunction; | `NoiseBasedChunkGeneratorTerrainMixin.be_islandTerrain` | MODIFY_EXPRESSION_VALUE | needs-mapping | Selects an exact Rust expression: pick it in the target function. No Horizon function known for NoiseBasedChunkGenerator.iterateNoiseColumn: run `match`. |
| mixin into NoiseGeneratorSettings with no injectors (interfaces or @Shadow only) | `NoiseGeneratorSettingsMixin` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| @Inject(INVOKE) on PlayerAdvancements.award at Lnet/minecraft/advancements/AdvancementRewards;grant(Lnet/minecraft/server/level/ServerPlayer;)V | `PlayerAdvancementsMixin.be_award` | AFTER_CALL on horizon_client::game::advancements::Game<'_>::award | needs-mapping | The call it hooks (AdvancementRewards.grant) isn't mapped to a Rust function yet. |
| @Inject(HEAD) on ServerPlayer.findRespawnPositionAndUseSpawnBlock | `PlayerMixin.be_findRespawnAndUseSpawnBlock` | HEAD | needs-mapping | No Horizon function known for ServerPlayer.findRespawnPositionAndUseSpawnBlock: run `match`. |
| @Redirect(INVOKE) on ServerLevel.<init>* at Lnet/minecraft/world/level/dimension/DimensionType;hasEnderDragonFight()Z | `ServerLevelMixin.be_hasEnderDragonFight` | WRAP_OPERATION | needs-translation | A redirect is a WRAP_OPERATION that doesn't call `original`. Wildcard or unknown selector `<init>*`. The call it hooks (DimensionType.hasEnderDragonFight) isn't mapped to a Rust function yet. |
| @Inject(TAIL) on ServerLevel.<init>* | `ServerLevelMixin.be_onServerWorldInit` | RETURN | needs-translation | Horizon has no TAIL: RETURN runs at every exit, early ones too. Wildcard or unknown selector `<init>*`. |
| @ModifyArg(INVOKE) on ServerLevel.tickPrecipitation at Lnet/minecraft/server/level/ServerLevel;setBlockAndUpdate(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)Z | `ServerLevelMixin.be_modifyTickState` | MODIFY_ARG | needs-mapping | No Horizon function known for ServerLevel.tickPrecipitation: run `match`. The call it hooks (ServerLevel.setBlockAndUpdate) isn't mapped to a Rust function yet. |
| @Inject(HEAD) on ServerPlayerGameMode.destroyBlock | `ServerPlayerGameModeMixin.be_resonanceDestroyBlock` | HEAD | needs-mapping | No Horizon function known for ServerPlayerGameMode.destroyBlock: run `match`. |
| mixin into Slime with no injectors (interfaces or @Shadow only) | `SlimeMixin` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| @Inject(HEAD) on EndSpikeFeature.place | `SpikeFeatureMixin.be_place` | HEAD | needs-mapping | No Horizon function known for EndSpikeFeature.place: run `match`. |
| @Inject(HEAD) on EndSpikeFeature.placeSpike | `SpikeFeatureMixin.be_placeSpike` | HEAD | needs-mapping | No Horizon function known for EndSpikeFeature.placeSpike: run `match`. |
| @Inject(HEAD) on WorldGenRegion.ensureCanWrite | `WorldGenRegionMixin.be_alterBlockCheck` | HEAD on horizon_world::worldgen::placement::context::ensure_can_write | auto |  |
| mixin into Entity with no injectors (interfaces or @Shadow only) | `EntityMixin` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |

## Classes extending the game

| Class | In | Status | Note |
|---|---|---|---|
| extends Block | `AeterniumBlock` | content | A plain block becomes a content-mod block copying a vanilla template. Overridden behaviour methods need native code. |
| extends Block | `AmaranitaCapBlock` | content | A plain block becomes a content-mod block copying a vanilla template. Overridden behaviour methods need native code. |
| extends Block | `AmaranitaHymenophoreBlock` | content | A plain block becomes a content-mod block copying a vanilla template. Overridden behaviour methods need native code. |
| extends RotatedPillarBlock | `AmaranitaStemBlock` | needs-translation | No mapping for this superclass yet. |
| extends Block | `AmberBlock` | content | A plain block becomes a content-mod block copying a vanilla template. Overridden behaviour methods need native code. |
| extends Block | `AncientEmeraldIceBlock` | needs-translation | Overrides randomTick, makeParticles, stepOn: the content ports, that behaviour needs native code. |
| extends TransparentBlock | `AuroraCrystalBlock` | needs-translation | No mapping for this superclass yet. |
| extends Block | `BlueVineLanternBlock` | needs-translation | Overrides canSurvive, updateShape, createBlockStateDefinition, provideBlockModel: the content ports, that behaviour needs native code. |
| extends Block | `BrimstoneBlock` | needs-translation | Overrides createBlockStateDefinition, setPlacedBy, destroy, updateChunks, randomTick: the content ports, that behaviour needs native code. |
| extends RotatedPillarBlock | `BuddingSmaragdantCrystalBlock` | needs-translation | No mapping for this superclass yet. |
| extends Block | `CavePumpkinBlock` | needs-translation | Overrides createBlockStateDefinition, getShape, buildLoot: the content ports, that behaviour needs native code. |
| extends Block | `CharcoalBlock` | needs-translation | Overrides getFuelTime: the content ports, that behaviour needs native code. |
| extends MultifaceSpreadeableBlock | `CrystalMossCoverBlock` | needs-translation | No mapping for this superclass yet. |
| extends HalfTransparentBlock | `EmeraldIceBlock` | needs-translation | No mapping for this superclass yet. |
| extends Block | `EndLotusLeafBlock` | needs-translation | Overrides canSurvive, createBlockStateDefinition, getShape, rotate, mirror, getCloneItemStack: the content ports, that behaviour needs native code. |
| extends Block | `EndLotusStemBlock` | needs-translation | Overrides getShape, createBlockStateDefinition, getFluidState, getStateForPlacement, rotate, mirror, updateShape: the content ports, that behaviour needs native code. |
| extends PathBlock | `EndPathBlock` | needs-translation | No mapping for this superclass yet. |
| extends NetherPortalBlock | `EndPortalBlock` | needs-translation | No mapping for this superclass yet. |
| extends Block | `EnderBlock` | needs-translation | Overrides getColor: the content ports, that behaviour needs native code. |
| extends FallingBlock | `EndstoneDustBlock` | needs-translation | No mapping for this superclass yet. |
| extends Block | `FilaluxLanternBlock` | content | A plain block becomes a content-mod block copying a vanilla template. Overridden behaviour methods need native code. |
| extends Block | `FlowerPotBlock` | needs-translation | Overrides createBlockStateDefinition, newBlockEntity, getDrops, useItemOn, findByBlock, getShape, getCollisionShape, buildModel: the content ports, that behaviour needs native code. |
| extends Block | `GlowingHymenophoreBlock` | needs-translation | Overrides provideUnshadedCubeModel: the content ports, that behaviour needs native code. |
| extends Block | `GlowingPillarLuminophorBlock` | needs-translation | Overrides canSurvive, updateShape, createBlockStateDefinition: the content ports, that behaviour needs native code. |
| extends Block | `HelixTreeLeavesBlock` | needs-translation | Overrides createBlockStateDefinition, getStateForPlacement: the content ports, that behaviour needs native code. |
| extends Block | `HydraluxPetalBlock` | needs-translation | Overrides fallOn: the content ports, that behaviour needs native code. |
| extends Block | `HydrothermalVentBlock` | needs-translation | Overrides createBlockStateDefinition, getShape, canPlaceLiquid, placeLiquid, canSurvive, updateShape, getStateForPlacement, getFluidState, newBlockEntity, tick, setPlacedBy, animateTick, getTicker: the content ports, that behaviour needs native code. |
| extends SlimeBlock | `JellyshroomCapBlock` | needs-translation | No mapping for this superclass yet. |
| extends Block | `LumecornBlock` | needs-translation | Overrides createBlockStateDefinition, getShape, canSurvive, updateShape, buildLoot, shapeIs, getCloneItemStack: the content ports, that behaviour needs native code. |
| extends Block | `MengerSpongeBlock` | needs-translation | Overrides onPlace, updateShape, absorbWater, getShape: the content ports, that behaviour needs native code. |
| extends Block | `MengerSpongeWetBlock` | needs-translation | Overrides onPlace, animateTick, playerWillDestroy, getFluidState, getShape: the content ports, that behaviour needs native code. |
| extends RotatedPillarBlock | `MossyDragonBoneBlock` | needs-translation | No mapping for this superclass yet. |
| extends Block | `MossyGlowshroomCapBlock` | needs-translation | Overrides getStateForPlacement, createBlockStateDefinition: the content ports, that behaviour needs native code. |
| extends Block | `MossyObsidian` | needs-translation | Overrides randomTick, canSurvive: the content ports, that behaviour needs native code. |
| extends Block | `NeonCactusPlantBlock` | needs-translation | Overrides createBlockStateDefinition, getStateForPlacement, rotate, mirror, getFluidState, updateShape, tick, getShape, canSurvive, randomTick, growPlant, growPlant, growIteration, getSideDirection, isPathfindable, entityInside, getLength, getHorizontal, mutateStem: the content ports, that behaviour needs native code. |
| extends Block | `RespawnObeliskBlock` | needs-translation | Overrides getShape, createBlockStateDefinition, canSurvive, setPlacedBy, updateShape, playerWillDestroy, buildLoot, useItemOn: the content ports, that behaviour needs native code. |
| extends Block | `RunedFlavolite` | needs-translation | Overrides createBlockStateDefinition, dropFromExplosion: the content ports, that behaviour needs native code. |
| extends Block | `SilkMothHiveBlock` | needs-translation | Overrides createBlockStateDefinition, getStateForPlacement, rotate, mirror, randomTick, useItemOn: the content ports, that behaviour needs native code. |
| extends Block | `SilkMothNestBlock` | needs-translation | Overrides createBlockStateDefinition, getShape, getStateForPlacement, updateShape, rotate, mirror, buildLoot, playerWillDestroy, randomTick, useItemOn: the content ports, that behaviour needs native code. |
| extends Block | `UmbrellaTreeClusterBlock` | needs-translation | Overrides createBlockStateDefinition, useWithoutItem, useItemOn: the content ports, that behaviour needs native code. |
| extends Block | `UmbrellaTreeClusterEmptyBlock` | needs-translation | Overrides createBlockStateDefinition, randomTick: the content ports, that behaviour needs native code. |
| extends SlimeBlock | `UmbrellaTreeMembraneBlock` | needs-translation | No mapping for this superclass yet. |
| extends Block | `VentBubbleColumnBlock` | needs-translation | Overrides pickupBlock, getRenderShape, canSurvive, getShape, updateShape, animateTick, entityInside, canPlaceLiquid, placeLiquid, getFluidState, getPickupSound: the content ports, that behaviour needs native code. |
| extends Block | `EndLanternBlock` | needs-translation | Overrides createBlockStateDefinition, getStateForPlacement, canSurvive, updateShape, canPlaceLiquid, placeLiquid, getFluidState: the content ports, that behaviour needs native code. |
| extends Block | `PedestalBlock` | needs-translation | Overrides getHeight, useItemOn, setPlacedBy, destroy, checkRitual, getStateForPlacement, updateShape, getUpdatedState, getDrops, moveStoredStack, moveStoredStack, dropStoredStack, getDropPos, defaultShape, isPlaceable, getShape, createBlockStateDefinition, newBlockEntity, hasUniqueEntity, hasAnalogOutputSignal, getAnalogOutputSignal, provideBlockModel, provideBlockModel, createTextureMapping: the content ports, that behaviour needs native code. |
| extends BlockEntity | `BlockEntityHydrothermalVent` | unsupported | Block entities aren't supported in Horizon mods yet. |
| extends BaseContainerBlockEntity | `EndStoneSmelterBlockEntity` | needs-translation | No mapping for this superclass yet. |
| extends BlockEntity | `FlowerPotBlockEntity` | unsupported | Block entities aren't supported in Horizon mods yet. |
| extends BlockEntity | `PedestalBlockEntity` | unsupported | Block entities aren't supported in Horizon mods yet. |
| extends RecipeBookMenu | `EndStoneSmelterMenu` | needs-translation | No mapping for this superclass yet. |
| extends AbstractContainerScreen | `EndStoneSmelterScreen` | needs-translation | No mapping for this superclass yet. |
| extends AbstractScrollArea | `InfusionRecipeScreen$RecipeList` | needs-translation | No mapping for this superclass yet. |
| extends Screen | `InfusionRecipeScreen` | needs-translation | No general screen hook; simple option pages can use the settings hook. |
| extends Slot | `SmelterFuelSlot` | needs-translation | No mapping for this superclass yet. |
| extends Slot | `SmelterOutputSlot` | needs-translation | No mapping for this superclass yet. |
| extends BlockEntityRenderState | `FlowerPotRenderState` | needs-translation | No mapping for this superclass yet. |
| extends BlockEntityRenderState | `PedestalRenderState` | needs-translation | No mapping for this superclass yet. |
| extends MobEffect | `EndVeilEffect` | needs-translation | No mapping for this superclass yet. |
| extends AbstractSchoolingFish | `CubozoaEntity` | needs-translation | No mapping for this superclass yet. |
| extends FlyingPathNavigation | `DragonflyEntity$1` | needs-translation | No mapping for this superclass yet. |
| extends LookControl | `DragonflyEntity$DragonflyLookControl` | needs-translation | No mapping for this superclass yet. |
| extends Goal | `DragonflyEntity$WanderAroundGoal` | needs-translation | No mapping for this superclass yet. |
| extends AbstractSchoolingFish | `EndFishEntity` | needs-translation | No mapping for this superclass yet. |
| extends MoveControl | `EndSlimeEntity$EndSlimeMoveControl` | needs-translation | No mapping for this superclass yet. |
| extends Goal | `EndSlimeEntity$FaceTowardTargetGoal` | needs-translation | No mapping for this superclass yet. |
| extends Goal | `EndSlimeEntity$MoveGoal` | needs-translation | No mapping for this superclass yet. |
| extends Goal | `EndSlimeEntity$RandomLookGoal` | needs-translation | No mapping for this superclass yet. |
| extends Goal | `EndSlimeEntity$SwimmingGoal` | needs-translation | No mapping for this superclass yet. |
| extends Slime | `EndSlimeEntity` | needs-translation | No mapping for this superclass yet. |
| extends MeleeAttackGoal | `ShadowWalkerEntity$AttackGoal` | needs-translation | No mapping for this superclass yet. |
| extends Monster | `ShadowWalkerEntity` | needs-translation | No mapping for this superclass yet. |
| extends FlyingPathNavigation | `SilkMothEntity$1` | needs-translation | No mapping for this superclass yet. |
| extends LookControl | `SilkMothEntity$MothLookControl` | needs-translation | No mapping for this superclass yet. |
| extends Goal | `SilkMothEntity$ReturnToHiveGoal` | needs-translation | No mapping for this superclass yet. |
| extends Goal | `SilkMothEntity$WanderAroundGoal` | needs-translation | No mapping for this superclass yet. |
| extends Animal | `SilkMothEntity` | needs-translation | No mapping for this superclass yet. |
| extends EntityModel | `CubozoaEntityModel` | needs-translation | No mapping for this superclass yet. |
| extends EntityModel | `DragonflyEntityModel` | needs-translation | No mapping for this superclass yet. |
| extends EntityModel | `EndFishEntityModel` | needs-translation | No mapping for this superclass yet. |
| extends EntityModel | `EndSlimeEntityModel` | needs-translation | No mapping for this superclass yet. |
| extends EntityModel | `SilkMothEntityModel` | needs-translation | No mapping for this superclass yet. |
| extends EyesLayer | `RendererEntityCubozoa$1` | needs-translation | No mapping for this superclass yet. |
| extends MobRenderer | `RendererEntityCubozoa` | needs-translation | No mapping for this superclass yet. |
| extends EyesLayer | `RendererEntityDragonfly$1` | needs-translation | No mapping for this superclass yet. |
| extends MobRenderer | `RendererEntityDragonfly` | needs-translation | No mapping for this superclass yet. |
| extends EyesLayer | `RendererEntityEndFish$1` | needs-translation | No mapping for this superclass yet. |
| extends MobRenderer | `RendererEntityEndFish` | needs-translation | No mapping for this superclass yet. |
| extends EyesLayer | `RendererEntityEndSlime$1` | needs-translation | No mapping for this superclass yet. |
| extends RenderLayer | `RendererEntityEndSlime$OverlayFeatureRenderer` | needs-translation | No mapping for this superclass yet. |
| extends MobRenderer | `RendererEntityEndSlime` | needs-translation | No mapping for this superclass yet. |
| extends HumanoidMobRenderer | `RendererEntityShadowWalker` | needs-translation | No mapping for this superclass yet. |
| extends MobRenderer | `SilkMothEntityRenderer` | needs-translation | No mapping for this superclass yet. |
| extends LivingEntityRenderState | `CubozoaRenderState` | needs-translation | No mapping for this superclass yet. |
| extends LivingEntityRenderState | `DragonflyRenderState` | needs-translation | No mapping for this superclass yet. |
| extends LivingEntityRenderState | `EndFishRenderState` | needs-translation | No mapping for this superclass yet. |
| extends SlimeRenderState | `EndSlimeRenderState` | needs-translation | No mapping for this superclass yet. |
| extends HumanoidRenderState | `ShadowWalkerRenderState` | needs-translation | No mapping for this superclass yet. |
| extends LivingEntityRenderState | `SilkMothRenderState` | needs-translation | No mapping for this superclass yet. |
| extends Item | `ArmoredElytra` | needs-translation | Overrides getMovementFactor, getModelTexture: the content ports, that behaviour needs native code. |
| extends Item | `CrystaliteArmor` | needs-translation | Overrides hasFullSet, applySetEffect: the content ports, that behaviour needs native code. |
| extends Item | `EndArmorItem` | needs-translation | Overrides armorBoostId, toughnessBoostId, knockbackBoostId, createDefaultEndArmorSettings, createDefaultEndArmorSettings: the content ports, that behaviour needs native code. |
| extends Attribute | `EndAttribute` | needs-translation | No mapping for this superclass yet. |
| extends MobBucketItem | `EndBucketItem` | needs-translation | No mapping for this superclass yet. |
| extends Item | `GuideBookItem` | needs-translation | Overrides register, use, appendHoverText, ensureStaticallyLoaded: the content ports, that behaviour needs native code. |
| extends HumanoidModel | `CrystaliteBootsModel` | needs-translation | No mapping for this superclass yet. |
| extends HumanoidModel | `CrystaliteChestplateModel` | needs-translation | No mapping for this superclass yet. |
| extends HumanoidModel | `CrystaliteHelmetModel` | needs-translation | No mapping for this superclass yet. |
| extends HumanoidModel | `CrystaliteLeggingsModel` | needs-translation | No mapping for this superclass yet. |
| extends SimpleAnimatedParticle | `FireflyParticle` | needs-translation | No mapping for this superclass yet. |
| extends SingleQuadParticle | `InfusionMistParticle` | needs-translation | No mapping for this superclass yet. |
| extends SingleQuadParticle | `InfusionParticle` | needs-translation | No mapping for this superclass yet. |
| extends ParticleType | `InfusionParticleType` | needs-translation | No mapping for this superclass yet. |
| extends SimpleAnimatedParticle | `ParticleBlackSpore` | needs-translation | No mapping for this superclass yet. |
| extends SingleQuadParticle | `ParticleGeyser` | needs-translation | No mapping for this superclass yet. |
| extends SimpleAnimatedParticle | `ParticleGlowingSphere` | needs-translation | No mapping for this superclass yet. |
| extends SimpleAnimatedParticle | `ParticleJungleSpore` | needs-translation | No mapping for this superclass yet. |
| extends SingleQuadParticle | `ParticleSnowflake` | needs-translation | No mapping for this superclass yet. |
| extends SingleQuadParticle | `ParticleSulphur` | needs-translation | No mapping for this superclass yet. |
| extends SingleQuadParticle | `ParticleTenaneaPetal` | needs-translation | No mapping for this superclass yet. |
| extends SimpleAnimatedParticle | `PaticlePortalSphere` | needs-translation | No mapping for this superclass yet. |
| extends SimpleAnimatedParticle | `SmaragdantParticle` | needs-translation | No mapping for this superclass yet. |
| extends VegetationPatchFeature | `CaveVegetationPatchFeature` | needs-translation | No mapping for this superclass yet. |
| extends Structure | `FeatureBaseStructure` | needs-translation | No mapping for this superclass yet. |
| extends StructurePiece | `BasePiece` | needs-translation | No mapping for this superclass yet. |

## Access wideners

| File | In | Status | Note |
|---|---|---|---|
| access widener (opens private game fields) | `betterend.accesswidener` | needs-translation | Native helpers already see the game's fields, once each field is mapped. |

## Content

| What | In | Status | Note |
|---|---|---|---|
| registers blocks or items in code | `BECriteria` | content | Registration and properties: horizon-port-pack carries them as a content mod. |
| registers blocks or items in code | `EndStatusEffects` | content | Registration and properties: horizon-port-pack carries them as a content mod. |
| registers blocks or items in code | `EndAttributes` | content | Registration and properties: horizon-port-pack carries them as a content mod. |
| registers blocks or items in code | `EndBlockEntities` | content | Registration and properties: horizon-port-pack carries them as a content mod. |
| registers blocks or items in code | `EndCarvers` | content | Registration and properties: horizon-port-pack carries them as a content mod. |
| registers blocks or items in code | `EndEntities` | content | Registration and properties: horizon-port-pack carries them as a content mod. |
| registers blocks or items in code | `EndMenuTypes` | content | Registration and properties: horizon-port-pack carries them as a content mod. |
| registers blocks or items in code | `EndSounds` | content | Registration and properties: horizon-port-pack carries them as a content mod. |
| 6724 asset files, 4317 data files | `jar resources` | content | Recipes, loot, tags, models, textures, worldgen: horizon-port-pack carries these. |

## Game methods and fields used

83 of 1936 are mapped to Horizon. Every Java body that uses an unmapped one needs it mapped before its Rust can be written.

<details><summary>Mapped</summary>

| Java | Horizon |
|---|---|
| `net/minecraft/client/Minecraft#font` | `(implicit)` |
| `net/minecraft/client/Minecraft#getInstance` | `self` |
| `net/minecraft/client/Minecraft#player` | `self.native_player()` |
| `net/minecraft/client/gui/Font#width` | `self.native_text_width(text)` |
| `net/minecraft/client/gui/GuiGraphicsExtractor#blitSprite` | horizon_client_render::ui::blit_sprite (suggested) |
| `net/minecraft/client/gui/GuiGraphicsExtractor#itemDecorations` | horizon_client_render::hud_overlays::item_decorations (suggested) |
| `net/minecraft/client/renderer/GameRenderer#bobHurt` | horizon_client_render::camera::bob_hurt (suggested) |
| `net/minecraft/client/renderer/item/ItemStackRenderState#submit` | horizon_client::item_render::ItemRenderState::submit (suggested) |
| `net/minecraft/client/sounds/MusicManager#tick` | horizon_client::music::MusicManager::tick (suggested) |
| `net/minecraft/client/sounds/SoundManager#updateCategoryVolume` | horizon_client_render::sound_engine::update_category_volume (suggested) |
| `net/minecraft/core/BlockPos#spiralAround` | horizon_world::worldgen::generator::spiral_around (suggested) |
| `net/minecraft/core/Direction#getOpposite` | horizon_client::mobs::shulker::opposite (suggested) |
| `net/minecraft/core/Registry#register` | content |
| `net/minecraft/core/Registry#registerForHolder` | content |
| `net/minecraft/core/registries/BuiltInRegistries#BLOCK` | content |
| `net/minecraft/core/registries/BuiltInRegistries#ITEM` | content |
| `net/minecraft/network/chat/MutableComponent#append` | horizon_client::game::dedicated::append (suggested) |
| `net/minecraft/network/chat/MutableComponent#withStyle` | horizon_client_core::text::Text::with_style (suggested) |
| `net/minecraft/server/PlayerAdvancements#award` | horizon_client::game::advancements::Game<'_>::award (suggested) |
| `net/minecraft/server/level/ServerLevel#destroyBlock` | horizon_world::level::update::Sim<'_, L>::destroy_block (suggested) |
| `net/minecraft/server/level/ServerLevel#getHeight` | horizon_world::level::feature_world::<LevelFeatureWorld<'_, '_, L> as PlacementContext>::height (suggested) |
| `net/minecraft/server/level/ServerLevel#getMaxY` | horizon_client::game::Dim::max_y (suggested) |
| `net/minecraft/server/level/ServerLevel#getMinY` | horizon_client::game::Dim::min_y (suggested) |
| `net/minecraft/server/level/ServerLevel#removeBlock` | horizon_world::level::update::Sim<'_, L>::remove_block (suggested) |
| `net/minecraft/server/level/ServerLevel#scheduleTick` | horizon_world::level::Behaviours::schedule_tick (suggested) |
| `net/minecraft/server/level/ServerLevel#sendParticles` | horizon_client::game::fishing::Game<'_>::send_particles (suggested) |
| `net/minecraft/server/level/ServerLevel#setBlock` | horizon_world::level::Behaviours::set_block (suggested) |
| `net/minecraft/server/level/ServerPlayer#blockActionRestricted` | horizon_client::first_person::block_action_restricted (suggested) |
| `net/minecraft/server/level/ServerPlayer#canUseGameMasterBlocks` | horizon_client::game::command_blocks::Game<'_>::can_use_game_master_blocks (suggested) |
| `net/minecraft/server/level/ServerPlayer#triggerRecipeCrafted` | horizon_client::game::advancements::Game<'_>::trigger_recipe_crafted (suggested) |
| `net/minecraft/server/level/WorldGenRegion#ensureCanWrite` | horizon_world::worldgen::placement::context::ensure_can_write (suggested) |
| `net/minecraft/server/network/ServerGamePacketListenerImpl#teleport` | horizon_client::lan::hub::Hub::teleport (suggested) |
| `net/minecraft/util/ARGB#colorFromFloat` | horizon_client_render::hud_layout::color_from_float (suggested) |
| `net/minecraft/util/BlockUtil#getLargestRectangleAround` | horizon_world::level::portal_travel::largest_rectangle_around (suggested) |
| `net/minecraft/util/Mth#clamp` | horizon_client_render::chat::clamp (suggested) |
| `net/minecraft/util/Mth#floor` | horizon_client_render::gfx::occlusion::floor (suggested) |
| `net/minecraft/util/Mth#frac` | horizon_client_render::border_render::frac (suggested) |
| `net/minecraft/util/Mth#lerp` | horizon_world::stepped_interpolation::lerp (suggested) |
| `net/minecraft/util/Mth#nextInt` | horizon_world::worldgen::structure::ocean_ruin::next_int (suggested) |
| `net/minecraft/world/Container#stillValid` | horizon_client::lan::menu::Slots::still_valid (suggested) |
| `net/minecraft/world/attribute/BackgroundMusic#select` | horizon_client::music::BackgroundMusic::select (suggested) |
| `net/minecraft/world/entity/Entity#getX` | `self.native_player()?.x` |
| `net/minecraft/world/entity/Entity#getXRot` | `self.native_player()?.pitch` |
| `net/minecraft/world/entity/Entity#getY` | `self.native_player()?.y` |
| `net/minecraft/world/entity/Entity#getYRot` | `self.native_player()?.yaw` |
| `net/minecraft/world/entity/Entity#getZ` | `self.native_player()?.z` |
| `net/minecraft/world/entity/Entity#hurt` | horizon_world::living_fall::hurt (suggested) |
| `net/minecraft/world/entity/EntityType#create` | horizon_client::mobs::finalize::create (suggested) |
| `net/minecraft/world/entity/LivingEntity#hasEffect` | horizon_client::mobs::Seen::has_effect (suggested) |
| `net/minecraft/world/entity/LivingEntity#knockback` | horizon_client::mobs::Mob::knockback (suggested) |
| `net/minecraft/world/entity/LivingEntity#moveRelative` | horizon_client::mobs::living::Mob::move_relative (suggested) |
| `net/minecraft/world/entity/Mob#onGround` | horizon_client::game::minecarts::Minecart::on_ground (suggested) |
| `net/minecraft/world/entity/Mob#setSpeed` | horizon_client::mobs::control::Mob::set_speed (suggested) |
| `net/minecraft/world/entity/player/Player#hasEffect` | horizon_client::mobs::PlayerView::has_effect (suggested) |
| `net/minecraft/world/entity/player/StackedItemContents#accountStack` | horizon_client::menu::place::Stacked::account_stack (suggested) |
| `net/minecraft/world/inventory/Slot#remove` | horizon_client::menu::Menu::remove (suggested) |
| `net/minecraft/world/item/Item#<init>` | content |
| `net/minecraft/world/item/Item$Properties#durability` | content |
| `net/minecraft/world/item/ItemStack#getCount` | horizon_world::loot::LootStack::get_count (suggested) |
| `net/minecraft/world/item/ItemStack#isSameItemSameComponents` | horizon_items::item_data::ItemStack::same_item_same_components (suggested) |
| `net/minecraft/world/item/ItemStack#shrink` | horizon_client::game::dispenser::shrink (suggested) |
| `net/minecraft/world/item/ItemStackTemplate#create` | horizon_client::crafting::Template::create (suggested) |
| `net/minecraft/world/level/Level#getHeight` | horizon_world::level::feature_world::<LevelFeatureWorld<'_, '_, L> as PlacementContext>::height (suggested) |
| `net/minecraft/world/level/Level#isClientSide` | horizon_world::level::is_client_side (suggested) |
| `net/minecraft/world/level/Level#removeBlock` | horizon_world::level::update::Sim<'_, L>::remove_block (suggested) |
| `net/minecraft/world/level/Level#setBlock` | horizon_world::level::Behaviours::set_block (suggested) |
| `net/minecraft/world/level/Level#updateNeighborsAt` | horizon_world::level::Behaviours::update_neighbors_at (suggested) |
| `net/minecraft/world/level/WorldGenLevel#getBiome` | horizon_world::worldgen::structure::legacy::biome (suggested) |
| `net/minecraft/world/level/WorldGenLevel#getSeed` | horizon_world::worldgen::structure::legacy::seed (suggested) |
| `net/minecraft/world/level/block/Block#<init>` | content |
| `net/minecraft/world/level/block/Block#popResource` | horizon_client::game::spill::GameLevel<'_>::pop_resource (suggested) |
| `net/minecraft/world/level/block/Block#setPlacedBy` | horizon_world::level::place::Sim<'_, L>::set_placed_by (suggested) |
| `net/minecraft/world/level/block/BucketPickup#pickupBlock` | horizon_world::level::fluids::Sim<'_, L>::pickup_block (suggested) |
| `net/minecraft/world/level/block/Rotation#getRandom` | horizon_world::worldgen::structure::Rotation::random (suggested) |
| `net/minecraft/world/level/block/state/BlockState#canBeReplaced` | horizon_world::level::place::Sim<'_, L>::can_be_replaced (suggested) |
| `net/minecraft/world/level/block/state/BlockState#canOcclude` | horizon_world::worldgen::placement::context::can_occlude (suggested) |
| `net/minecraft/world/level/block/state/BlockState#canSurvive` | horizon_world::level::survive::Sim<'_, L>::can_survive (suggested) |
| `net/minecraft/world/level/block/state/BlockState#getOffset` | horizon_client_render::mesher::MeshTables::offset (suggested) |
| `net/minecraft/world/level/block/state/BlockState#updateShape` | horizon_world::level::shapes::Sim<'_, L>::update_shape (suggested) |
| `net/minecraft/world/level/chunk/ChunkGenerator#getBaseHeight` | horizon_world::worldgen::generator::Pipeline<R>::base_height (suggested) |
| `net/minecraft/world/level/dimension/end/EnderDragonFight#tryRespawn` | horizon_client::game::dragon::Game<'_>::try_respawn (suggested) |
| `net/minecraft/world/level/levelgen/structure/templatesystem/StructureTemplate#placeInWorld` | horizon_world::worldgen::structure::processor::place_in_world (suggested) |
| `net/minecraft/world/level/levelgen/structure/templatesystem/StructureTemplate#transform` | horizon_world::worldgen::structure::template::StructureTemplate::transform_vec (suggested) |

</details>

<details><summary>Not mapped yet</summary>

| Java | Kind | Classes using it |
|---|---|---|
| `com/mojang/blaze3d/platform/Window#getAppropriateLineWidth` | method | 1 |
| `com/mojang/blaze3d/systems/RenderSystem#getCompiledPipeline` | method | 1 |
| `com/mojang/blaze3d/systems/RenderSystem#getDevice` | method | 1 |
| `com/mojang/blaze3d/systems/RenderSystem#getDynamicUniforms` | method | 1 |
| `com/mojang/blaze3d/systems/RenderSystem#getSequentialBuffer` | method | 1 |
| `com/mojang/blaze3d/systems/RenderSystem$AutoStorageIndexBuffer#getBuffer` | method | 1 |
| `com/mojang/blaze3d/systems/RenderSystem$AutoStorageIndexBuffer#type` | method | 1 |
| `com/mojang/blaze3d/vertex/BufferBuilder#<init>` | method | 1 |
| `com/mojang/blaze3d/vertex/BufferBuilder#addVertex` | method | 1 |
| `com/mojang/blaze3d/vertex/BufferBuilder#buildOrThrow` | method | 1 |
| `com/mojang/blaze3d/vertex/ByteBufferBuilder#close` | method | 1 |
| `com/mojang/blaze3d/vertex/ByteBufferBuilder#exactlySized` | method | 1 |
| `com/mojang/blaze3d/vertex/DefaultVertexFormat#POSITION` | field | 1 |
| `com/mojang/blaze3d/vertex/DefaultVertexFormat#POSITION_TEX_COLOR` | field | 1 |
| `com/mojang/blaze3d/vertex/MeshData#close` | method | 1 |
| `com/mojang/blaze3d/vertex/MeshData#drawState` | method | 1 |
| `com/mojang/blaze3d/vertex/MeshData#vertexBuffer` | method | 1 |
| `com/mojang/blaze3d/vertex/MeshData$DrawState#indexCount` | method | 1 |
| `com/mojang/blaze3d/vertex/PoseStack#<init>` | method | 6 |
| `com/mojang/blaze3d/vertex/PoseStack#last` | method | 2 |
| `com/mojang/blaze3d/vertex/PoseStack#mulPose` | method | 5 |
| `com/mojang/blaze3d/vertex/PoseStack#popPose` | method | 6 |
| `com/mojang/blaze3d/vertex/PoseStack#pushPose` | method | 6 |
| `com/mojang/blaze3d/vertex/PoseStack#rotate` | method | 6 |
| `com/mojang/blaze3d/vertex/PoseStack#scale` | method | 5 |
| `com/mojang/blaze3d/vertex/PoseStack#translate` | method | 6 |
| `com/mojang/blaze3d/vertex/PoseStack$Pose#pose` | method | 7 |
| `com/mojang/blaze3d/vertex/PoseStack$Pose#set` | method | 1 |
| `com/mojang/blaze3d/vertex/VertexConsumer#addVertex` | method | 2 |
| `com/mojang/blaze3d/vertex/VertexConsumer#setColor` | method | 3 |
| `com/mojang/blaze3d/vertex/VertexConsumer#setLight` | method | 2 |
| `com/mojang/blaze3d/vertex/VertexConsumer#setNormal` | method | 2 |
| `com/mojang/blaze3d/vertex/VertexConsumer#setOverlay` | method | 2 |
| `com/mojang/blaze3d/vertex/VertexConsumer#setUv` | method | 3 |
| `net/minecraft/ChatFormatting#BLUE` | field | 1 |
| `net/minecraft/ChatFormatting#DARK_AQUA` | field | 1 |
| `net/minecraft/ChatFormatting#DARK_GRAY` | field | 2 |
| `net/minecraft/ChatFormatting#DARK_GREEN` | field | 1 |
| `net/minecraft/ChatFormatting#DARK_PURPLE` | field | 1 |
| `net/minecraft/ChatFormatting#ITALIC` | field | 3 |
| `net/minecraft/ChatFormatting#RED` | field | 1 |
| `net/minecraft/advancements/AdvancementHolder#id` | method | 1 |
| `net/minecraft/advancements/AdvancementRequirements$Strategy#AND` | field | 1 |
| `net/minecraft/advancements/AdvancementRequirements$Strategy#OR` | field | 1 |
| `net/minecraft/advancements/AdvancementRewards#grant` | method | 1 |
| `net/minecraft/advancements/AdvancementType#CHALLENGE` | field | 1 |
| `net/minecraft/advancements/AdvancementType#GOAL` | field | 1 |
| `net/minecraft/advancements/AdvancementType#TASK` | field | 1 |
| `net/minecraft/advancements/predicates/LocationPredicate$Builder#location` | method | 3 |
| `net/minecraft/advancements/predicates/LocationPredicate$Builder#setBiomes` | method | 1 |
| `net/minecraft/advancements/predicates/LocationPredicate$Builder#setDimension` | method | 1 |
| `net/minecraft/advancements/predicates/StatePropertiesPredicate$Builder#hasProperty` | method | 9 |
| `net/minecraft/advancements/predicates/StatePropertiesPredicate$Builder#properties` | method | 9 |
| `net/minecraft/advancements/triggers/ChangeDimensionTrigger$TriggerInstance#changedDimensionTo` | method | 1 |
| `net/minecraft/advancements/triggers/PlayerTrigger#<init>` | method | 1 |
| `net/minecraft/advancements/triggers/PlayerTrigger#createCriterion` | method | 1 |
| `net/minecraft/advancements/triggers/PlayerTrigger#trigger` | method | 3 |
| `net/minecraft/advancements/triggers/PlayerTrigger$TriggerInstance#<init>` | method | 1 |
| `net/minecraft/advancements/triggers/PlayerTrigger$TriggerInstance#located` | method | 1 |
| `net/minecraft/client/DeltaTracker#getGameTimeDeltaPartialTick` | method | 1 |
| `net/minecraft/client/DeltaTracker#getRealtimeDeltaTicks` | method | 1 |
| `net/minecraft/client/Minecraft#getDeltaTracker` | method | 2 |
| `net/minecraft/client/Minecraft#getModelManager` | method | 2 |
| `net/minecraft/client/Minecraft#getSituationalMusic` | method | 2 |
| `net/minecraft/client/Minecraft#getSoundManager` | method | 1 |
| `net/minecraft/client/Minecraft#getTextureManager` | method | 1 |
| `net/minecraft/client/Minecraft#getWindow` | method | 1 |
| `net/minecraft/client/Minecraft#gui` | field | 1 |
| `net/minecraft/client/Minecraft#level` | field | 8 |
| `net/minecraft/client/Minecraft#options` | field | 1 |
| `net/minecraft/client/OptionInstance#get` | method | 1 |
| `net/minecraft/client/Options#screenEffectScale` | method | 1 |
| `net/minecraft/client/color/block/BlockTintSource#colorInWorld` | method | 1 |
| `net/minecraft/client/data/models/BlockModelGenerators#Y_ROT_180` | field | 1 |
| `net/minecraft/client/data/models/BlockModelGenerators#Y_ROT_270` | field | 1 |
| `net/minecraft/client/data/models/BlockModelGenerators#Y_ROT_90` | field | 1 |
| `net/minecraft/client/data/models/BlockModelGenerators#createSimpleBlock` | method | 1 |
| `net/minecraft/client/data/models/BlockModelGenerators#modelOutput` | field | 4 |
| `net/minecraft/client/data/models/BlockModelGenerators#plainModel` | method | 4 |
| `net/minecraft/client/data/models/BlockModelGenerators#plainVariant` | method | 6 |
| `net/minecraft/client/data/models/ItemModelGenerators#generateFlatItem` | method | 1 |
| `net/minecraft/client/data/models/ItemModelGenerators#itemModelOutput` | field | 2 |
| `net/minecraft/client/data/models/ItemModelOutput#accept` | method | 2 |
| `net/minecraft/client/data/models/MultiVariant#<init>` | method | 3 |
| `net/minecraft/client/data/models/blockstates/MultiVariantGenerator#dispatch` | method | 9 |
| `net/minecraft/client/data/models/blockstates/MultiVariantGenerator#with` | method | 4 |
| `net/minecraft/client/data/models/blockstates/MultiVariantGenerator$Empty#with` | method | 4 |
| `net/minecraft/client/data/models/blockstates/PropertyDispatch#initial` | method | 4 |
| `net/minecraft/client/data/models/blockstates/PropertyDispatch#modify` | method | 4 |
| `net/minecraft/client/data/models/blockstates/PropertyDispatch$C1#select` | method | 7 |
| `net/minecraft/client/data/models/blockstates/PropertyDispatch$C2#select` | method | 1 |
| `net/minecraft/client/data/models/model/ItemModelUtils#plainModel` | method | 2 |
| `net/minecraft/client/data/models/model/ModelLocationUtils#getModelLocation` | method | 2 |
| `net/minecraft/client/data/models/model/ModelTemplate#<init>` | method | 1 |
| `net/minecraft/client/data/models/model/ModelTemplate#create` | method | 7 |
| `net/minecraft/client/data/models/model/ModelTemplate#createWithSuffix` | method | 6 |
| `net/minecraft/client/data/models/model/ModelTemplates#CUBE` | field | 1 |
| `net/minecraft/client/data/models/model/ModelTemplates#CUBE_ALL` | field | 2 |
| `net/minecraft/client/data/models/model/ModelTemplates#CUBE_BOTTOM_TOP` | field | 1 |
| `net/minecraft/client/data/models/model/ModelTemplates#FLAT_ITEM` | field | 1 |
| `net/minecraft/client/data/models/model/TextureMapping#<init>` | method | 12 |
| `net/minecraft/client/data/models/model/TextureMapping#cube` | method | 1 |
| `net/minecraft/client/data/models/model/TextureMapping#defaultTexture` | method | 1 |
| `net/minecraft/client/data/models/model/TextureMapping#getBlockTexture` | method | 12 |
| `net/minecraft/client/data/models/model/TextureMapping#getItemTexture` | method | 1 |
| `net/minecraft/client/data/models/model/TextureMapping#put` | method | 12 |
| `net/minecraft/client/data/models/model/TextureSlot#ALL` | field | 1 |
| `net/minecraft/client/data/models/model/TextureSlot#BOTTOM` | field | 7 |
| `net/minecraft/client/data/models/model/TextureSlot#DOWN` | field | 1 |
| `net/minecraft/client/data/models/model/TextureSlot#EAST` | field | 1 |
| `net/minecraft/client/data/models/model/TextureSlot#END` | field | 1 |
| `net/minecraft/client/data/models/model/TextureSlot#NORTH` | field | 1 |
| `net/minecraft/client/data/models/model/TextureSlot#PARTICLE` | field | 1 |
| `net/minecraft/client/data/models/model/TextureSlot#SIDE` | field | 3 |
| `net/minecraft/client/data/models/model/TextureSlot#SOUTH` | field | 1 |
| `net/minecraft/client/data/models/model/TextureSlot#TEXTURE` | field | 5 |
| `net/minecraft/client/data/models/model/TextureSlot#TOP` | field | 7 |
| `net/minecraft/client/data/models/model/TextureSlot#UP` | field | 1 |
| `net/minecraft/client/data/models/model/TextureSlot#WEST` | field | 1 |
| `net/minecraft/client/data/models/model/TextureSlot#create` | method | 1 |
| `net/minecraft/client/gui/Gui#hud` | field | 1 |
| `net/minecraft/client/gui/Gui#screen` | method | 1 |
| `net/minecraft/client/gui/Gui#setScreen` | method | 1 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#blit` | method | 2 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#disableScissor` | method | 1 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#enableScissor` | method | 1 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#fill` | method | 2 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#item` | method | 2 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#setComponentTooltipForNextFrame` | method | 1 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#text` | method | 2 |
| `net/minecraft/client/gui/Hud#getBossOverlay` | method | 1 |
| `net/minecraft/client/gui/components/AbstractScrollArea#<init>` | method | 1 |
| `net/minecraft/client/gui/components/AbstractScrollArea#defaultSettings` | method | 1 |
| `net/minecraft/client/gui/components/BossHealthOverlay#shouldPlayMusic` | method | 1 |
| `net/minecraft/client/gui/narration/NarratedElementType#TITLE` | field | 1 |
| `net/minecraft/client/gui/narration/NarrationElementOutput#add` | method | 1 |
| `net/minecraft/client/gui/screens/MenuScreens#register` | method | 1 |
| `net/minecraft/client/gui/screens/Screen#<init>` | method | 1 |
| `net/minecraft/client/gui/screens/Screen#extractRenderState` | method | 1 |
| `net/minecraft/client/gui/screens/inventory/AbstractContainerScreen#<init>` | method | 1 |
| `net/minecraft/client/gui/screens/inventory/AbstractContainerScreen#extractContents` | method | 1 |
| `net/minecraft/client/gui/screens/inventory/AbstractContainerScreen#init` | method | 1 |
| `net/minecraft/client/input/MouseButtonEvent#x` | method | 1 |
| `net/minecraft/client/input/MouseButtonEvent#y` | method | 1 |
| `net/minecraft/client/model/EntityModel#<init>` | method | 5 |
| `net/minecraft/client/model/HumanoidModel#<init>` | method | 5 |
| `net/minecraft/client/model/HumanoidModel#body` | field | 1 |
| `net/minecraft/client/model/HumanoidModel#hat` | field | 1 |
| `net/minecraft/client/model/HumanoidModel#head` | field | 1 |
| `net/minecraft/client/model/HumanoidModel#leftArm` | field | 1 |
| `net/minecraft/client/model/HumanoidModel#leftLeg` | field | 1 |
| `net/minecraft/client/model/HumanoidModel#rightArm` | field | 1 |
| `net/minecraft/client/model/HumanoidModel#rightLeg` | field | 1 |
| `net/minecraft/client/model/HumanoidModel#root` | method | 1 |
| `net/minecraft/client/model/geom/EntityModelSet#bakeLayer` | method | 5 |
| `net/minecraft/client/model/geom/ModelLayerLocation#<init>` | method | 1 |
| `net/minecraft/client/model/geom/ModelLayers#PLAYER` | field | 1 |
| `net/minecraft/client/model/geom/ModelPart#<init>` | method | 2 |
| `net/minecraft/client/model/geom/ModelPart#getChild` | method | 11 |
| `net/minecraft/client/model/geom/ModelPart#render` | method | 4 |
| `net/minecraft/client/model/geom/ModelPart#resetPose` | method | 1 |
| `net/minecraft/client/model/geom/ModelPart#setInitialPose` | method | 1 |
| `net/minecraft/client/model/geom/ModelPart#translateAndRotate` | method | 2 |
| `net/minecraft/client/model/geom/ModelPart#visible` | field | 1 |
| `net/minecraft/client/model/geom/ModelPart#x` | field | 1 |
| `net/minecraft/client/model/geom/ModelPart#xRot` | field | 4 |
| `net/minecraft/client/model/geom/ModelPart#xScale` | field | 2 |
| `net/minecraft/client/model/geom/ModelPart#y` | field | 1 |
| `net/minecraft/client/model/geom/ModelPart#yRot` | field | 1 |
| `net/minecraft/client/model/geom/ModelPart#yScale` | field | 2 |
| `net/minecraft/client/model/geom/ModelPart#zRot` | field | 2 |
| `net/minecraft/client/model/geom/ModelPart#zScale` | field | 2 |
| `net/minecraft/client/model/geom/PartPose#ZERO` | field | 8 |
| `net/minecraft/client/model/geom/PartPose#offset` | method | 8 |
| `net/minecraft/client/model/geom/PartPose#offsetAndRotation` | method | 5 |
| `net/minecraft/client/model/geom/builders/CubeDeformation#<init>` | method | 4 |
| `net/minecraft/client/model/geom/builders/CubeListBuilder#addBox` | method | 11 |
| `net/minecraft/client/model/geom/builders/CubeListBuilder#create` | method | 11 |
| `net/minecraft/client/model/geom/builders/CubeListBuilder#mirror` | method | 4 |
| `net/minecraft/client/model/geom/builders/CubeListBuilder#texOffs` | method | 11 |
| `net/minecraft/client/model/geom/builders/LayerDefinition#bakeRoot` | method | 6 |
| `net/minecraft/client/model/geom/builders/LayerDefinition#create` | method | 11 |
| `net/minecraft/client/model/geom/builders/MeshDefinition#<init>` | method | 11 |
| `net/minecraft/client/model/geom/builders/MeshDefinition#getRoot` | method | 11 |
| `net/minecraft/client/model/geom/builders/PartDefinition#addOrReplaceChild` | method | 11 |
| `net/minecraft/client/multiplayer/ClientLevel#dimension` | method | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#getBiome` | method | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#getBiomeManager` | method | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#getBlockState` | method | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#getGameTime` | method | 2 |
| `net/minecraft/client/multiplayer/ClientLevel#getOverworldClockTime` | method | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#getRandom` | method | 5 |
| `net/minecraft/client/multiplayer/ClientLevel#setSectionDirtyWithNeighbors` | method | 1 |
| `net/minecraft/client/particle/Particle#setLifetime` | method | 2 |
| `net/minecraft/client/particle/Particle#setParticleSpeed` | method | 2 |
| `net/minecraft/client/particle/SimpleAnimatedParticle#<init>` | method | 6 |
| `net/minecraft/client/particle/SimpleAnimatedParticle#tick` | method | 6 |
| `net/minecraft/client/particle/SingleQuadParticle#<init>` | method | 6 |
| `net/minecraft/client/particle/SingleQuadParticle#tick` | method | 5 |
| `net/minecraft/client/particle/SingleQuadParticle$Layer#TRANSLUCENT` | field | 6 |
| `net/minecraft/client/particle/SpriteSet#first` | method | 1 |
| `net/minecraft/client/particle/SpriteSet#get` | method | 11 |
| `net/minecraft/client/player/LocalPlayer#blockPosition` | method | 2 |
| `net/minecraft/client/player/LocalPlayer#distanceToSqr` | method | 2 |
| `net/minecraft/client/player/LocalPlayer#getX` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#getZ` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#level` | method | 1 |
| `net/minecraft/client/renderer/BiomeColors#getAverageWaterColor` | method | 1 |
| `net/minecraft/client/renderer/DynamicGpuData#writeTransform` | method | 1 |
| `net/minecraft/client/renderer/LevelRenderer#executeSolid` | method | 2 |
| `net/minecraft/client/renderer/RenderPipelines#END_SKY` | field | 1 |
| `net/minecraft/client/renderer/RenderPipelines#GUI_TEXTURED` | field | 2 |
| `net/minecraft/client/renderer/RenderPipelines#STARS` | field | 1 |
| `net/minecraft/client/renderer/SubmitNodeCollector#submitBlockModel` | method | 1 |
| `net/minecraft/client/renderer/SubmitNodeCollector#submitCustomGeometry` | method | 7 |
| `net/minecraft/client/renderer/SubmitNodeCollector#submitModel` | method | 2 |
| `net/minecraft/client/renderer/SubmitNodeCollector#submitShapeOutline` | method | 1 |
| `net/minecraft/client/renderer/block/BlockAndTintGetter#getBlockState` | method | 1 |
| `net/minecraft/client/renderer/block/BlockModelRenderState#<init>` | method | 1 |
| `net/minecraft/client/renderer/block/BlockModelRenderState#EMPTY_TINTS` | field | 1 |
| `net/minecraft/client/renderer/block/BlockModelRenderState#submit` | method | 1 |
| `net/minecraft/client/renderer/block/BlockModelResolver#update` | method | 1 |
| `net/minecraft/client/renderer/block/BlockStateModelSet#get` | method | 1 |
| `net/minecraft/client/renderer/block/dispatch/BlockStateModel#collectParts` | method | 1 |
| `net/minecraft/client/renderer/block/dispatch/BlockStateModelPart#getQuads` | method | 1 |
| `net/minecraft/client/renderer/block/dispatch/BlockStateModelPart#particleMaterial` | method | 1 |
| `net/minecraft/client/renderer/block/dispatch/Variant#<init>` | method | 3 |
| `net/minecraft/client/renderer/block/dispatch/Variant#withXRot` | method | 1 |
| `net/minecraft/client/renderer/block/dispatch/Variant#withYRot` | method | 1 |
| `net/minecraft/client/renderer/block/dispatch/VariantMutator#apply` | method | 1 |
| `net/minecraft/client/renderer/block/model/BlockDisplayContext#create` | method | 1 |
| `net/minecraft/client/renderer/blockentity/BlockEntityRenderer#extractRenderState` | method | 2 |
| `net/minecraft/client/renderer/blockentity/BlockEntityRendererProvider$Context#blockModelResolver` | method | 1 |
| `net/minecraft/client/renderer/blockentity/BlockEntityRendererProvider$Context#itemModelResolver` | method | 1 |
| `net/minecraft/client/renderer/blockentity/state/BlockEntityRenderState#<init>` | method | 2 |
| `net/minecraft/client/renderer/entity/EntityRendererProvider$Context#bakeLayer` | method | 5 |
| `net/minecraft/client/renderer/entity/EntityRendererProvider$Context#getModelSet` | method | 2 |
| `net/minecraft/client/renderer/entity/HumanoidMobRenderer#<init>` | method | 1 |
| `net/minecraft/client/renderer/entity/LivingEntityRenderer#<init>` | method | 2 |
| `net/minecraft/client/renderer/entity/LivingEntityRenderer#getOverlayCoords` | method | 1 |
| `net/minecraft/client/renderer/entity/MobRenderer#<init>` | method | 6 |
| `net/minecraft/client/renderer/entity/MobRenderer#extractRenderState` | method | 3 |
| `net/minecraft/client/renderer/entity/layers/CapeLayer#submit` | method | 1 |
| `net/minecraft/client/renderer/entity/layers/EyesLayer#<init>` | method | 4 |
| `net/minecraft/client/renderer/entity/layers/RenderLayer#<init>` | method | 1 |
| `net/minecraft/client/renderer/entity/state/AvatarRenderState#chestEquipment` | field | 1 |
| `net/minecraft/client/renderer/entity/state/AvatarRenderState#skin` | field | 1 |
| `net/minecraft/client/renderer/entity/state/HumanoidRenderState#<init>` | method | 1 |
| `net/minecraft/client/renderer/entity/state/LivingEntityRenderState#<init>` | method | 4 |
| `net/minecraft/client/renderer/entity/state/SlimeRenderState#<init>` | method | 1 |
| `net/minecraft/client/renderer/item/ItemModelResolver#updateForTopItem` | method | 1 |
| `net/minecraft/client/renderer/item/ItemStackRenderState#<init>` | method | 1 |
| `net/minecraft/client/renderer/rendertype/RenderTypes#beaconBeam` | method | 1 |
| `net/minecraft/client/renderer/rendertype/RenderTypes#entityCutout` | method | 2 |
| `net/minecraft/client/renderer/rendertype/RenderTypes#entityCutoutCull` | method | 4 |
| `net/minecraft/client/renderer/rendertype/RenderTypes#entityTranslucent` | method | 8 |
| `net/minecraft/client/renderer/rendertype/RenderTypes#eyes` | method | 4 |
| `net/minecraft/client/renderer/rendertype/RenderTypes#linesTranslucent` | method | 1 |
| `net/minecraft/client/renderer/state/level/CameraRenderState#viewRotationMatrix` | field | 1 |
| `net/minecraft/client/renderer/state/level/LevelRenderState#cameraRenderState` | field | 1 |
| `net/minecraft/client/renderer/texture/AbstractTexture#getSampler` | method | 1 |
| `net/minecraft/client/renderer/texture/AbstractTexture#getTextureView` | method | 1 |
| `net/minecraft/client/renderer/texture/OverlayTexture#NO_OVERLAY` | field | 9 |
| `net/minecraft/client/renderer/texture/TextureAtlasSprite#getU0` | method | 1 |
| `net/minecraft/client/renderer/texture/TextureAtlasSprite#getU1` | method | 1 |
| `net/minecraft/client/renderer/texture/TextureAtlasSprite#getV0` | method | 1 |
| `net/minecraft/client/renderer/texture/TextureAtlasSprite#getV1` | method | 1 |
| `net/minecraft/client/renderer/texture/TextureManager#getTexture` | method | 1 |
| `net/minecraft/client/resources/language/I18n#get` | method | 1 |
| `net/minecraft/client/resources/model/BlockStateModelLoader#lambda$loadBlockStates$0` | method | 1 |
| `net/minecraft/client/resources/model/ModelManager#getBlockStateModelSet` | method | 1 |
| `net/minecraft/client/resources/model/geometry/BakedQuad#materialInfo` | method | 1 |
| `net/minecraft/client/resources/model/geometry/BakedQuad$MaterialInfo#itemRenderType` | method | 1 |
| `net/minecraft/client/resources/model/geometry/BakedQuad$MaterialInfo#sprite` | method | 1 |
| `net/minecraft/client/resources/model/sprite/Material#<init>` | method | 10 |
| `net/minecraft/client/resources/model/sprite/Material#sprite` | method | 7 |
| `net/minecraft/client/resources/model/sprite/Material$Baked#sprite` | method | 1 |
| `net/minecraft/client/resources/sounds/AbstractSoundInstance#volume` | method | 1 |
| `net/minecraft/client/resources/sounds/SoundInstance#getIdentifier` | method | 1 |
| `net/minecraft/client/resources/sounds/SoundInstance#getSource` | method | 1 |
| `net/minecraft/client/resources/sounds/SoundInstance#getVolume` | method | 1 |
| `net/minecraft/client/sounds/SoundManager#stop` | method | 1 |
| `net/minecraft/commands/CommandSourceStack#getLevel` | method | 1 |
| `net/minecraft/commands/CommandSourceStack#getPlayerOrException` | method | 1 |
| `net/minecraft/commands/CommandSourceStack#getPosition` | method | 1 |
| `net/minecraft/commands/CommandSourceStack#sendFailure` | method | 1 |
| `net/minecraft/commands/CommandSourceStack#sendSuccess` | method | 1 |
| `net/minecraft/commands/Commands#LEVEL_OWNERS` | field | 1 |
| `net/minecraft/commands/Commands#hasPermission` | method | 1 |
| `net/minecraft/commands/Commands#literal` | method | 1 |
| `net/minecraft/core/BlockPos#<init>` | method | 50 |
| `net/minecraft/core/BlockPos#CODEC` | field | 10 |
| `net/minecraft/core/BlockPos#STREAM_CODEC` | field | 1 |
| `net/minecraft/core/BlockPos#ZERO` | field | 10 |
| `net/minecraft/core/BlockPos#above` | method | 46 |
| `net/minecraft/core/BlockPos#asLong` | method | 3 |
| `net/minecraft/core/BlockPos#below` | method | 76 |
| `net/minecraft/core/BlockPos#betweenClosed` | method | 3 |
| `net/minecraft/core/BlockPos#distManhattan` | method | 3 |
| `net/minecraft/core/BlockPos#distSqr` | method | 1 |
| `net/minecraft/core/BlockPos#east` | method | 5 |
| `net/minecraft/core/BlockPos#equals` | method | 2 |
| `net/minecraft/core/BlockPos#getX` | method | 100 |
| `net/minecraft/core/BlockPos#getY` | method | 90 |
| `net/minecraft/core/BlockPos#getZ` | method | 100 |
| `net/minecraft/core/BlockPos#hashCode` | method | 1 |
| `net/minecraft/core/BlockPos#immutable` | method | 7 |
| `net/minecraft/core/BlockPos#mutable` | method | 7 |
| `net/minecraft/core/BlockPos#north` | method | 3 |
| `net/minecraft/core/BlockPos#offset` | method | 17 |
| `net/minecraft/core/BlockPos#relative` | method | 36 |
| `net/minecraft/core/BlockPos#south` | method | 4 |
| `net/minecraft/core/BlockPos#subtract` | method | 1 |
| `net/minecraft/core/BlockPos#west` | method | 4 |
| `net/minecraft/core/BlockPos$MutableBlockPos#<init>` | method | 47 |
| `net/minecraft/core/BlockPos$MutableBlockPos#above` | method | 16 |
| `net/minecraft/core/BlockPos$MutableBlockPos#asLong` | method | 1 |
| `net/minecraft/core/BlockPos$MutableBlockPos#below` | method | 11 |
| `net/minecraft/core/BlockPos$MutableBlockPos#getX` | method | 11 |
| `net/minecraft/core/BlockPos$MutableBlockPos#getY` | method | 27 |
| `net/minecraft/core/BlockPos$MutableBlockPos#getZ` | method | 11 |
| `net/minecraft/core/BlockPos$MutableBlockPos#immutable` | method | 11 |
| `net/minecraft/core/BlockPos$MutableBlockPos#move` | method | 29 |
| `net/minecraft/core/BlockPos$MutableBlockPos#mutable` | method | 1 |
| `net/minecraft/core/BlockPos$MutableBlockPos#north` | method | 1 |
| `net/minecraft/core/BlockPos$MutableBlockPos#offset` | method | 2 |
| `net/minecraft/core/BlockPos$MutableBlockPos#relative` | method | 8 |
| `net/minecraft/core/BlockPos$MutableBlockPos#set` | method | 44 |
| `net/minecraft/core/BlockPos$MutableBlockPos#setWithOffset` | method | 3 |
| `net/minecraft/core/BlockPos$MutableBlockPos#setX` | method | 25 |
| `net/minecraft/core/BlockPos$MutableBlockPos#setY` | method | 40 |
| `net/minecraft/core/BlockPos$MutableBlockPos#setZ` | method | 25 |
| `net/minecraft/core/DefaultedRegistry#byNameCodec` | method | 1 |
| `net/minecraft/core/DefaultedRegistry#get` | method | 1 |
| `net/minecraft/core/DefaultedRegistry#getKey` | method | 10 |
| `net/minecraft/core/DefaultedRegistry#getOptional` | method | 2 |
| `net/minecraft/core/DefaultedRegistry#getValue` | method | 1 |
| `net/minecraft/core/DefaultedRegistry#iterator` | method | 1 |
| `net/minecraft/core/Direction#DOWN` | field | 31 |
| `net/minecraft/core/Direction#EAST` | field | 19 |
| `net/minecraft/core/Direction#NORTH` | field | 19 |
| `net/minecraft/core/Direction#SOUTH` | field | 18 |
| `net/minecraft/core/Direction#UP` | field | 36 |
| `net/minecraft/core/Direction#WEST` | field | 14 |
| `net/minecraft/core/Direction#from3DDataValue` | method | 1 |
| `net/minecraft/core/Direction#fromAxisAndDirection` | method | 1 |
| `net/minecraft/core/Direction#get` | method | 1 |
| `net/minecraft/core/Direction#getAxis` | method | 5 |
| `net/minecraft/core/Direction#getClockWise` | method | 3 |
| `net/minecraft/core/Direction#getCounterClockWise` | method | 1 |
| `net/minecraft/core/Direction#getRandom` | method | 1 |
| `net/minecraft/core/Direction#getStepX` | method | 11 |
| `net/minecraft/core/Direction#getStepY` | method | 7 |
| `net/minecraft/core/Direction#getStepZ` | method | 11 |
| `net/minecraft/core/Direction#ordinal` | method | 2 |
| `net/minecraft/core/Direction#stream` | method | 1 |
| `net/minecraft/core/Direction#toYRot` | method | 2 |
| `net/minecraft/core/Direction#values` | method | 13 |
| `net/minecraft/core/Direction$Axis#X` | field | 10 |
| `net/minecraft/core/Direction$Axis#Y` | field | 4 |
| `net/minecraft/core/Direction$Axis#Z` | field | 7 |
| `net/minecraft/core/Direction$Axis#byName` | method | 2 |
| `net/minecraft/core/Direction$Axis#equals` | method | 1 |
| `net/minecraft/core/Direction$Axis#getName` | method | 2 |
| `net/minecraft/core/Direction$Axis#isHorizontal` | method | 1 |
| `net/minecraft/core/Direction$Axis#isVertical` | method | 1 |
| `net/minecraft/core/Direction$AxisDirection#NEGATIVE` | field | 1 |
| `net/minecraft/core/Direction$AxisDirection#POSITIVE` | field | 2 |
| `net/minecraft/core/Holder#direct` | method | 1 |
| `net/minecraft/core/Holder#equals` | method | 4 |
| `net/minecraft/core/Holder#is` | method | 13 |
| `net/minecraft/core/Holder#isBound` | method | 1 |
| `net/minecraft/core/Holder#toString` | method | 1 |
| `net/minecraft/core/Holder#unwrap` | method | 1 |
| `net/minecraft/core/Holder#unwrapKey` | method | 8 |
| `net/minecraft/core/Holder#value` | method | 15 |
| `net/minecraft/core/Holder$Reference#key` | method | 1 |
| `net/minecraft/core/Holder$Reference#value` | method | 1 |
| `net/minecraft/core/HolderGetter#getOrThrow` | method | 3 |
| `net/minecraft/core/HolderLookup$Provider#lookupOrThrow` | method | 3 |
| `net/minecraft/core/HolderLookup$RegistryLookup#get` | method | 2 |
| `net/minecraft/core/HolderLookup$RegistryLookup#getOrThrow` | method | 1 |
| `net/minecraft/core/HolderLookup$RegistryLookup#listElementIds` | method | 1 |
| `net/minecraft/core/HolderSet#direct` | method | 5 |
| `net/minecraft/core/NonNullList#clear` | method | 1 |
| `net/minecraft/core/NonNullList#get` | method | 2 |
| `net/minecraft/core/NonNullList#iterator` | method | 1 |
| `net/minecraft/core/NonNullList#set` | method | 1 |
| `net/minecraft/core/NonNullList#size` | method | 1 |
| `net/minecraft/core/NonNullList#withSize` | method | 1 |
| `net/minecraft/core/QuartPos#fromBlock` | method | 3 |
| `net/minecraft/core/Registry#get` | method | 3 |
| `net/minecraft/core/Registry#getKey` | method | 1 |
| `net/minecraft/core/Registry#getOptional` | method | 1 |
| `net/minecraft/core/Registry#getTagOrEmpty` | method | 1 |
| `net/minecraft/core/Registry#iterator` | method | 1 |
| `net/minecraft/core/RegistryAccess#lookup` | method | 3 |
| `net/minecraft/core/RegistryAccess#lookupOrThrow` | method | 4 |
| `net/minecraft/core/SectionPos#blockToSectionCoord` | method | 1 |
| `net/minecraft/core/SectionPos#sectionToBlockCoord` | method | 4 |
| `net/minecraft/core/Vec3i#<init>` | method | 3 |
| `net/minecraft/core/Vec3i#getX` | method | 3 |
| `net/minecraft/core/Vec3i#getY` | method | 2 |
| `net/minecraft/core/Vec3i#getZ` | method | 3 |
| `net/minecraft/core/component/DataComponentPatch#CODEC` | field | 1 |
| `net/minecraft/core/component/DataComponentPatch#EMPTY` | field | 2 |
| `net/minecraft/core/component/DataComponentPatch#STREAM_CODEC` | field | 1 |
| `net/minecraft/core/component/DataComponentPatch#builder` | method | 1 |
| `net/minecraft/core/component/DataComponentPatch#isEmpty` | method | 1 |
| `net/minecraft/core/component/DataComponentPatch$Builder#build` | method | 1 |
| `net/minecraft/core/component/DataComponentPatch$Builder#set` | method | 1 |
| `net/minecraft/core/component/DataComponentType$Builder#persistent` | method | 1 |
| `net/minecraft/core/component/DataComponents#BUCKET_ENTITY_DATA` | field | 2 |
| `net/minecraft/core/component/DataComponents#COOKING_FUEL` | field | 1 |
| `net/minecraft/core/component/DataComponents#CUSTOM_NAME` | field | 1 |
| `net/minecraft/core/component/DataComponents#EQUIPPABLE` | field | 1 |
| `net/minecraft/core/component/DataComponents#STORED_ENCHANTMENTS` | field | 2 |
| `net/minecraft/core/particles/BlockParticleOption#<init>` | method | 1 |
| `net/minecraft/core/particles/ColorParticleOption#create` | method | 2 |
| `net/minecraft/core/particles/ParticleType#<init>` | method | 1 |
| `net/minecraft/core/particles/ParticleTypes#ASH` | field | 1 |
| `net/minecraft/core/particles/ParticleTypes#BLOCK` | field | 1 |
| `net/minecraft/core/particles/ParticleTypes#BUBBLE` | field | 4 |
| `net/minecraft/core/particles/ParticleTypes#BUBBLE_COLUMN_UP` | field | 1 |
| `net/minecraft/core/particles/ParticleTypes#DRIPPING_WATER` | field | 1 |
| `net/minecraft/core/particles/ParticleTypes#EFFECT` | field | 2 |
| `net/minecraft/core/particles/ParticleTypes#ENCHANT` | field | 1 |
| `net/minecraft/core/particles/ParticleTypes#ENTITY_EFFECT` | field | 2 |
| `net/minecraft/core/particles/ParticleTypes#GLOW` | field | 1 |
| `net/minecraft/core/particles/ParticleTypes#LARGE_SMOKE` | field | 1 |
| `net/minecraft/core/particles/ParticleTypes#MYCELIUM` | field | 1 |
| `net/minecraft/core/particles/ParticleTypes#PORTAL` | field | 3 |
| `net/minecraft/core/particles/ParticleTypes#REVERSE_PORTAL` | field | 1 |
| `net/minecraft/core/particles/ParticleTypes#SMOKE` | field | 2 |
| `net/minecraft/core/particles/ParticleTypes#SPLASH` | field | 1 |
| `net/minecraft/core/particles/ParticleTypes#WHITE_ASH` | field | 3 |
| `net/minecraft/core/particles/SpellParticleOption#create` | method | 2 |
| `net/minecraft/core/registries/BuiltInRegistries#ATTRIBUTE` | field | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#BLOCK_ENTITY_TYPE` | field | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#CARVER_TYPE` | field | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#ENTITY_TYPE` | field | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#MENU` | field | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#MOB_EFFECT` | field | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#SOUND_EVENT` | field | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#TRIGGER_TYPES` | field | 1 |
| `net/minecraft/core/registries/Registries#BIOME` | field | 9 |
| `net/minecraft/core/registries/Registries#BLOCK` | field | 5 |
| `net/minecraft/core/registries/Registries#CARVER` | field | 2 |
| `net/minecraft/core/registries/Registries#DIMENSION` | field | 1 |
| `net/minecraft/core/registries/Registries#DIMENSION_TYPE` | field | 1 |
| `net/minecraft/core/registries/Registries#ENCHANTMENT` | field | 2 |
| `net/minecraft/core/registries/Registries#ENTITY_TYPE` | field | 1 |
| `net/minecraft/core/registries/Registries#ITEM` | field | 2 |
| `net/minecraft/core/registries/Registries#JUKEBOX_SONG` | field | 2 |
| `net/minecraft/core/registries/Registries#NOISE` | field | 1 |
| `net/minecraft/core/registries/Registries#STRUCTURE` | field | 1 |
| `net/minecraft/core/registries/Registries#TRADE_SET` | field | 1 |
| `net/minecraft/core/registries/Registries#VILLAGER_TRADE` | field | 3 |
| `net/minecraft/core/registries/codec/RegistryCodecs#holderSet` | method | 1 |
| `net/minecraft/data/recipes/RecipeCategory#DECORATIONS` | field | 1 |
| `net/minecraft/data/worldgen/BootstrapContext#lookup` | method | 3 |
| `net/minecraft/data/worldgen/BootstrapContext#register` | method | 4 |
| `net/minecraft/data/worldgen/features/EndFeatures#CHORUS_PLANT` | field | 2 |
| `net/minecraft/data/worldgen/placement/EndPlacements#CHORUS_PLANT` | field | 1 |
| `net/minecraft/data/worldgen/placement/EndPlacements#END_GATEWAY_RETURN` | field | 1 |
| `net/minecraft/data/worldgen/placement/PlacementUtils#filteredByBlockSurvival` | method | 1 |
| `net/minecraft/data/worldgen/placement/PlacementUtils#inlinePlaced` | method | 1 |
| `net/minecraft/nbt/CompoundTag#<init>` | method | 3 |
| `net/minecraft/nbt/CompoundTag#CODEC` | field | 2 |
| `net/minecraft/nbt/CompoundTag#contains` | method | 4 |
| `net/minecraft/nbt/CompoundTag#get` | method | 1 |
| `net/minecraft/nbt/CompoundTag#getBooleanOr` | method | 3 |
| `net/minecraft/nbt/CompoundTag#getByte` | method | 1 |
| `net/minecraft/nbt/CompoundTag#getCompound` | method | 1 |
| `net/minecraft/nbt/CompoundTag#getCompoundOrEmpty` | method | 1 |
| `net/minecraft/nbt/CompoundTag#getFloatOr` | method | 5 |
| `net/minecraft/nbt/CompoundTag#getInt` | method | 3 |
| `net/minecraft/nbt/CompoundTag#getIntArray` | method | 1 |
| `net/minecraft/nbt/CompoundTag#getIntOr` | method | 6 |
| `net/minecraft/nbt/CompoundTag#getList` | method | 1 |
| `net/minecraft/nbt/CompoundTag#getListOrEmpty` | method | 1 |
| `net/minecraft/nbt/CompoundTag#getLongOr` | method | 1 |
| `net/minecraft/nbt/CompoundTag#getStringOr` | method | 6 |
| `net/minecraft/nbt/CompoundTag#put` | method | 3 |
| `net/minecraft/nbt/CompoundTag#putBoolean` | method | 3 |
| `net/minecraft/nbt/CompoundTag#putFloat` | method | 5 |
| `net/minecraft/nbt/CompoundTag#putInt` | method | 10 |
| `net/minecraft/nbt/CompoundTag#putIntArray` | method | 1 |
| `net/minecraft/nbt/CompoundTag#putLong` | method | 1 |
| `net/minecraft/nbt/CompoundTag#putString` | method | 6 |
| `net/minecraft/nbt/CompoundTag#read` | method | 9 |
| `net/minecraft/nbt/CompoundTag#remove` | method | 1 |
| `net/minecraft/nbt/CompoundTag#store` | method | 9 |
| `net/minecraft/nbt/ListTag#<init>` | method | 2 |
| `net/minecraft/nbt/ListTag#add` | method | 2 |
| `net/minecraft/nbt/ListTag#getCompound` | method | 1 |
| `net/minecraft/nbt/ListTag#getCompoundOrEmpty` | method | 1 |
| `net/minecraft/nbt/ListTag#isEmpty` | method | 1 |
| `net/minecraft/nbt/ListTag#size` | method | 2 |
| `net/minecraft/nbt/LongArrayTag#getAsLongArray` | method | 1 |
| `net/minecraft/nbt/NbtUtils#writeBlockState` | method | 1 |
| `net/minecraft/network/RegistryFriendlyByteBuf#readUtf` | method | 1 |
| `net/minecraft/network/RegistryFriendlyByteBuf#readVarInt` | method | 1 |
| `net/minecraft/network/RegistryFriendlyByteBuf#writeUtf` | method | 1 |
| `net/minecraft/network/RegistryFriendlyByteBuf#writeVarInt` | method | 1 |
| `net/minecraft/network/chat/CommonComponents#SPACE` | field | 1 |
| `net/minecraft/network/chat/Component#copy` | method | 1 |
| `net/minecraft/network/chat/Component#empty` | method | 7 |
| `net/minecraft/network/chat/Component#getString` | method | 1 |
| `net/minecraft/network/chat/Component#literal` | method | 2 |
| `net/minecraft/network/chat/Component#translatable` | method | 13 |
| `net/minecraft/network/chat/MutableComponent#setStyle` | method | 3 |
| `net/minecraft/network/chat/Style#EMPTY` | field | 3 |
| `net/minecraft/network/chat/Style#applyFormats` | method | 2 |
| `net/minecraft/network/chat/Style#withColor` | method | 1 |
| `net/minecraft/network/codec/ByteBufCodecs#BYTE` | field | 1 |
| `net/minecraft/network/codec/ByteBufCodecs#STRING_UTF8` | field | 1 |
| `net/minecraft/network/codec/StreamCodec#composite` | method | 1 |
| `net/minecraft/network/codec/StreamCodec#decode` | method | 1 |
| `net/minecraft/network/codec/StreamCodec#encode` | method | 1 |
| `net/minecraft/network/codec/StreamCodec#map` | method | 2 |
| `net/minecraft/network/codec/StreamCodec#of` | method | 1 |
| `net/minecraft/network/protocol/game/ClientboundBlockEntityDataPacket#create` | method | 2 |
| `net/minecraft/network/protocol/game/ClientboundGameEventPacket#<init>` | method | 1 |
| `net/minecraft/network/protocol/game/ClientboundGameEventPacket#PUFFER_FISH_STING` | field | 1 |
| `net/minecraft/network/syncher/EntityDataSerializers#INT` | field | 3 |
| `net/minecraft/network/syncher/SynchedEntityData#defineId` | method | 3 |
| `net/minecraft/network/syncher/SynchedEntityData#get` | method | 3 |
| `net/minecraft/network/syncher/SynchedEntityData#set` | method | 3 |
| `net/minecraft/network/syncher/SynchedEntityData$Builder#define` | method | 3 |
| `net/minecraft/recipebook/ServerPlaceRecipe#placeRecipe` | method | 1 |
| `net/minecraft/resources/FileToIdConverter#idToFile` | method | 1 |
| `net/minecraft/resources/FileToIdConverter#json` | method | 1 |
| `net/minecraft/resources/Identifier#CODEC` | field | 1 |
| `net/minecraft/resources/Identifier#equals` | method | 5 |
| `net/minecraft/resources/Identifier#fromNamespaceAndPath` | method | 2 |
| `net/minecraft/resources/Identifier#getNamespace` | method | 5 |
| `net/minecraft/resources/Identifier#getPath` | method | 10 |
| `net/minecraft/resources/Identifier#parse` | method | 7 |
| `net/minecraft/resources/Identifier#toString` | method | 9 |
| `net/minecraft/resources/Identifier#withDefaultNamespace` | method | 9 |
| `net/minecraft/resources/Identifier#withPrefix` | method | 2 |
| `net/minecraft/resources/Identifier#withSuffix` | method | 9 |
| `net/minecraft/resources/ResourceKey#codec` | method | 2 |
| `net/minecraft/resources/ResourceKey#create` | method | 10 |
| `net/minecraft/resources/ResourceKey#identifier` | method | 19 |
| `net/minecraft/server/MinecraftServer#getAllLevels` | method | 2 |
| `net/minecraft/server/MinecraftServer#getLevel` | method | 4 |
| `net/minecraft/server/MinecraftServer#isUnderSpawnProtection` | method | 1 |
| `net/minecraft/server/MinecraftServer#overworld` | method | 2 |
| `net/minecraft/server/commands/LocateCommand#showLocateResult` | method | 1 |
| `net/minecraft/server/level/ServerChunkCache#addTicketWithRadius` | method | 1 |
| `net/minecraft/server/level/ServerChunkCache#getChunkNow` | method | 1 |
| `net/minecraft/server/level/ServerChunkCache#getGenerator` | method | 1 |
| `net/minecraft/server/level/ServerChunkCache#randomState` | method | 2 |
| `net/minecraft/server/level/ServerLevel#END_SPAWN_POINT` | field | 1 |
| `net/minecraft/server/level/ServerLevel#addFreshEntity` | method | 4 |
| `net/minecraft/server/level/ServerLevel#dimension` | method | 4 |
| `net/minecraft/server/level/ServerLevel#dimensionType` | method | 1 |
| `net/minecraft/server/level/ServerLevel#environmentAttributes` | method | 1 |
| `net/minecraft/server/level/ServerLevel#findClosestBiome3d` | method | 1 |
| `net/minecraft/server/level/ServerLevel#getBlockEntity` | method | 1 |
| `net/minecraft/server/level/ServerLevel#getBlockState` | method | 18 |
| `net/minecraft/server/level/ServerLevel#getBrightness` | method | 1 |
| `net/minecraft/server/level/ServerLevel#getChunkSource` | method | 3 |
| `net/minecraft/server/level/ServerLevel#getEntities` | method | 2 |
| `net/minecraft/server/level/ServerLevel#getEntitiesOfClass` | method | 1 |
| `net/minecraft/server/level/ServerLevel#getFluidState` | method | 1 |
| `net/minecraft/server/level/ServerLevel#getGameRules` | method | 1 |
| `net/minecraft/server/level/ServerLevel#getLogicalHeight` | method | 1 |
| `net/minecraft/server/level/ServerLevel#getPlayers` | method | 1 |
| `net/minecraft/server/level/ServerLevel#getPoiManager` | method | 1 |
| `net/minecraft/server/level/ServerLevel#getRandom` | method | 3 |
| `net/minecraft/server/level/ServerLevel#getRespawnData` | method | 1 |
| `net/minecraft/server/level/ServerLevel#getServer` | method | 3 |
| `net/minecraft/server/level/ServerLevel#getWorldBorder` | method | 4 |
| `net/minecraft/server/level/ServerLevel#isEmptyBlock` | method | 2 |
| `net/minecraft/server/level/ServerLevel#playSound` | method | 3 |
| `net/minecraft/server/level/ServerLevel#recipeAccess` | method | 3 |
| `net/minecraft/server/level/ServerLevel#registryAccess` | method | 1 |
| `net/minecraft/server/level/ServerLevel#sendBlockUpdated` | method | 1 |
| `net/minecraft/server/level/ServerLevel#setBlockAndUpdate` | method | 9 |
| `net/minecraft/server/level/ServerLevel#structureManager` | method | 2 |
| `net/minecraft/server/level/ServerLevel#tickPrecipitation` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#addItem` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#awardRecipes` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#connection` | field | 2 |
| `net/minecraft/server/level/ServerPlayer#findRespawnPositionAndUseSpawnBlock` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#getMainHandItem` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#getNearestViewDirection` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#getRespawnConfig` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#hasCorrectToolForDrops` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#isShiftKeyDown` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#level` | method | 3 |
| `net/minecraft/server/level/ServerPlayer#position` | method | 2 |
| `net/minecraft/server/level/ServerPlayer#preventsBlockDrops` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#sendOverlayMessage` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#setRespawnPosition` | method | 1 |
| `net/minecraft/server/level/ServerPlayer$RespawnConfig#<init>` | method | 1 |
| `net/minecraft/server/level/ServerPlayer$RespawnConfig#respawnData` | method | 1 |
| `net/minecraft/server/level/ServerPlayerGameMode#destroyBlock` | method | 1 |
| `net/minecraft/server/level/TicketType#PORTAL` | field | 1 |
| `net/minecraft/server/network/ServerGamePacketListenerImpl#send` | method | 1 |
| `net/minecraft/server/packs/resources/PreparableReloadListener$SharedState#resourceManager` | method | 1 |
| `net/minecraft/server/packs/resources/ResourceManager#listResources` | method | 1 |
| `net/minecraft/sounds/Music#minDelay` | method | 1 |
| `net/minecraft/sounds/Music#replaceCurrentMusic` | method | 1 |
| `net/minecraft/sounds/Music#sound` | method | 1 |
| `net/minecraft/sounds/Musics#END` | field | 1 |
| `net/minecraft/sounds/Musics#END_BOSS` | field | 1 |
| `net/minecraft/sounds/SoundEvent#createVariableRangeEvent` | method | 1 |
| `net/minecraft/sounds/SoundEvent#location` | method | 1 |
| `net/minecraft/sounds/SoundEvents#ARMOR_EQUIP_DIAMOND` | field | 1 |
| `net/minecraft/sounds/SoundEvents#ARMOR_EQUIP_IRON` | field | 1 |
| `net/minecraft/sounds/SoundEvents#ARMOR_EQUIP_NETHERITE` | field | 1 |
| `net/minecraft/sounds/SoundEvents#BEEHIVE_ENTER` | field | 1 |
| `net/minecraft/sounds/SoundEvents#BEEHIVE_EXIT` | field | 2 |
| `net/minecraft/sounds/SoundEvents#BLASTFURNACE_FIRE_CRACKLE` | field | 1 |
| `net/minecraft/sounds/SoundEvents#BOTTLE_FILL` | field | 1 |
| `net/minecraft/sounds/SoundEvents#BUBBLE_COLUMN_UPWARDS_AMBIENT` | field | 1 |
| `net/minecraft/sounds/SoundEvents#BUCKET_EMPTY` | field | 1 |
| `net/minecraft/sounds/SoundEvents#DISPENSER_FAIL` | field | 1 |
| `net/minecraft/sounds/SoundEvents#ENCHANTMENT_TABLE_USE` | field | 1 |
| `net/minecraft/sounds/SoundEvents#END_PORTAL_SPAWN` | field | 1 |
| `net/minecraft/sounds/SoundEvents#FIRE_EXTINGUISH` | field | 1 |
| `net/minecraft/sounds/SoundEvents#HOE_TILL` | field | 1 |
| `net/minecraft/sounds/SoundEvents#MUSIC_END` | field | 1 |
| `net/minecraft/sounds/SoundEvents#PORTAL_AMBIENT` | field | 1 |
| `net/minecraft/sounds/SoundEvents#RESPAWN_ANCHOR_SET_SPAWN` | field | 1 |
| `net/minecraft/sounds/SoundEvents#SALMON_AMBIENT` | field | 1 |
| `net/minecraft/sounds/SoundEvents#SALMON_DEATH` | field | 1 |
| `net/minecraft/sounds/SoundEvents#SALMON_FLOP` | field | 1 |
| `net/minecraft/sounds/SoundEvents#SALMON_HURT` | field | 1 |
| `net/minecraft/sounds/SoundEvents#SOUL_SOIL_PLACE` | field | 1 |
| `net/minecraft/sounds/SoundEvents#TROPICAL_FISH_FLOP` | field | 1 |
| `net/minecraft/sounds/SoundEvents#VILLAGER_WORK_CARTOGRAPHER` | field | 1 |
| `net/minecraft/sounds/SoundSource#BLOCKS` | field | 11 |
| `net/minecraft/sounds/SoundSource#NEUTRAL` | field | 1 |
| `net/minecraft/tags/BiomeTags#HAS_END_CITY` | field | 9 |
| `net/minecraft/tags/BiomeTags#IS_END` | field | 4 |
| `net/minecraft/tags/BlockItemTagId#block` | method | 1 |
| `net/minecraft/tags/BlockItemTagId#item` | method | 1 |
| `net/minecraft/tags/BlockItemTags#BARS` | field | 1 |
| `net/minecraft/tags/BlockItemTags#CHAINS` | field | 1 |
| `net/minecraft/tags/BlockTags#ANVIL` | field | 2 |
| `net/minecraft/tags/BlockTags#BEACON_BASE_BLOCKS` | field | 2 |
| `net/minecraft/tags/BlockTags#DRAGON_IMMUNE` | field | 3 |
| `net/minecraft/tags/BlockTags#FLOWER_POTS` | field | 1 |
| `net/minecraft/tags/BlockTags#INCORRECT_FOR_DIAMOND_TOOL` | field | 1 |
| `net/minecraft/tags/BlockTags#INCORRECT_FOR_IRON_TOOL` | field | 1 |
| `net/minecraft/tags/BlockTags#LEAVES` | field | 4 |
| `net/minecraft/tags/BlockTags#LOGS` | field | 3 |
| `net/minecraft/tags/BlockTags#NEEDS_DIAMOND_TOOL` | field | 3 |
| `net/minecraft/tags/BlockTags#NEEDS_IRON_TOOL` | field | 2 |
| `net/minecraft/tags/BlockTags#NEEDS_STONE_TOOL` | field | 2 |
| `net/minecraft/tags/BlockTags#NYLIUM` | field | 12 |
| `net/minecraft/tags/BlockTags#SAND` | field | 1 |
| `net/minecraft/tags/BlockTags#SWORD_EFFICIENT` | field | 1 |
| `net/minecraft/tags/BlockTags#WALLS` | field | 1 |
| `net/minecraft/tags/BlockTags#WASHED_AWAY_BY_FLUIDS` | field | 1 |
| `net/minecraft/tags/BlockTags#WITHER_IMMUNE` | field | 1 |
| `net/minecraft/tags/DamageTypeTags#IS_FIRE` | field | 1 |
| `net/minecraft/tags/EnchantmentTags#IN_ENCHANTING_TABLE` | field | 1 |
| `net/minecraft/tags/EnchantmentTags#TRADEABLE` | field | 1 |
| `net/minecraft/tags/FluidTags#WATER` | field | 2 |
| `net/minecraft/tags/ItemTags#BEACON_PAYMENT_ITEMS` | field | 2 |
| `net/minecraft/tags/ItemTags#FISHES` | field | 1 |
| `net/minecraft/tags/ItemTags#HEAD_ARMOR_ENCHANTABLE` | field | 1 |
| `net/minecraft/tags/ItemTags#STONE_CRAFTING_MATERIALS` | field | 2 |
| `net/minecraft/tags/ItemTags#STONE_TOOL_MATERIALS` | field | 2 |
| `net/minecraft/tags/TagKey#create` | method | 2 |
| `net/minecraft/tags/TagKey#equals` | method | 2 |
| `net/minecraft/util/ARGB#blueFloat` | method | 1 |
| `net/minecraft/util/ARGB#color` | method | 2 |
| `net/minecraft/util/ARGB#greenFloat` | method | 1 |
| `net/minecraft/util/ARGB#redFloat` | method | 1 |
| `net/minecraft/util/BlockUtil$FoundRectangle#axis1Size` | field | 1 |
| `net/minecraft/util/BlockUtil$FoundRectangle#axis2Size` | field | 1 |
| `net/minecraft/util/BlockUtil$FoundRectangle#minCorner` | field | 1 |
| `net/minecraft/util/Interval#INFINITE` | field | 1 |
| `net/minecraft/util/Mth#abs` | method | 6 |
| `net/minecraft/util/Mth#cos` | method | 3 |
| `net/minecraft/util/Mth#sin` | method | 10 |
| `net/minecraft/util/RandomSource#create` | method | 7 |
| `net/minecraft/util/RandomSource#nextBoolean` | method | 21 |
| `net/minecraft/util/RandomSource#nextDouble` | method | 18 |
| `net/minecraft/util/RandomSource#nextFloat` | method | 44 |
| `net/minecraft/util/RandomSource#nextGaussian` | method | 21 |
| `net/minecraft/util/RandomSource#nextInt` | method | 82 |
| `net/minecraft/util/RandomSource#nextLong` | method | 15 |
| `net/minecraft/util/StringRepresentable#fromEnum` | method | 2 |
| `net/minecraft/util/Unit#CODEC` | field | 1 |
| `net/minecraft/util/Util#TICKER` | field | 1 |
| `net/minecraft/util/Util#makeDescriptionId` | method | 1 |
| `net/minecraft/util/random/WeightedList#builder` | method | 3 |
| `net/minecraft/util/random/WeightedList#of` | method | 3 |
| `net/minecraft/util/random/WeightedList$Builder#add` | method | 3 |
| `net/minecraft/util/random/WeightedList$Builder#build` | method | 3 |
| `net/minecraft/util/valueproviders/ConstantFloat#of` | method | 1 |
| `net/minecraft/util/valueproviders/ConstantInt#of` | method | 2 |
| `net/minecraft/util/valueproviders/FloatProvider#sample` | method | 1 |
| `net/minecraft/util/valueproviders/FloatProviders#codec` | method | 1 |
| `net/minecraft/util/valueproviders/IntProvider#sample` | method | 3 |
| `net/minecraft/util/valueproviders/IntProviders#CODEC` | field | 2 |
| `net/minecraft/util/valueproviders/IntProviders#codec` | method | 1 |
| `net/minecraft/util/valueproviders/UniformFloat#of` | method | 1 |
| `net/minecraft/util/valueproviders/UniformInt#of` | method | 5 |
| `net/minecraft/world/Container#getItem` | method | 1 |
| `net/minecraft/world/ContainerHelper#loadAllItems` | method | 1 |
| `net/minecraft/world/ContainerHelper#removeItem` | method | 1 |
| `net/minecraft/world/ContainerHelper#saveAllItems` | method | 1 |
| `net/minecraft/world/ContainerHelper#takeItem` | method | 1 |
| `net/minecraft/world/InteractionHand#MAIN_HAND` | field | 4 |
| `net/minecraft/world/InteractionResult#CONSUME` | field | 6 |
| `net/minecraft/world/InteractionResult#FAIL` | field | 7 |
| `net/minecraft/world/InteractionResult#PASS` | field | 2 |
| `net/minecraft/world/InteractionResult#SUCCESS` | field | 8 |
| `net/minecraft/world/InteractionResult#TRY_WITH_EMPTY_HAND` | field | 3 |
| `net/minecraft/world/SimpleContainer#<init>` | method | 1 |
| `net/minecraft/world/attribute/BackgroundMusic#EMPTY` | field | 1 |
| `net/minecraft/world/attribute/EnvironmentAttributeMap#applyModifier` | method | 1 |
| `net/minecraft/world/attribute/EnvironmentAttributeSystem#getValue` | method | 2 |
| `net/minecraft/world/attribute/EnvironmentAttributes#BACKGROUND_MUSIC` | field | 1 |
| `net/minecraft/world/attribute/EnvironmentAttributes#WATER_EVAPORATES` | field | 2 |
| `net/minecraft/world/damagesource/DamageSource#getEntity` | method | 2 |
| `net/minecraft/world/damagesource/DamageSource#is` | method | 1 |
| `net/minecraft/world/damagesource/DamageSources#cactus` | method | 2 |
| `net/minecraft/world/damagesource/DamageSources#mobAttack` | method | 1 |
| `net/minecraft/world/effect/MobEffect#<init>` | method | 1 |
| `net/minecraft/world/effect/MobEffectCategory#BENEFICIAL` | field | 1 |
| `net/minecraft/world/effect/MobEffectInstance#<init>` | method | 9 |
| `net/minecraft/world/effect/MobEffectInstance#getEffect` | method | 1 |
| `net/minecraft/world/effect/MobEffects#BLINDNESS` | field | 3 |
| `net/minecraft/world/effect/MobEffects#HASTE` | field | 1 |
| `net/minecraft/world/effect/MobEffects#LEVITATION` | field | 1 |
| `net/minecraft/world/effect/MobEffects#NIGHT_VISION` | field | 1 |
| `net/minecraft/world/effect/MobEffects#POISON` | field | 1 |
| `net/minecraft/world/effect/MobEffects#REGENERATION` | field | 1 |
| `net/minecraft/world/effect/MobEffects#SPEED` | field | 1 |
| `net/minecraft/world/entity/Entity#<init>` | method | 2 |
| `net/minecraft/world/entity/Entity#canUsePortal` | method | 1 |
| `net/minecraft/world/entity/Entity#getDeltaMovement` | method | 1 |
| `net/minecraft/world/entity/Entity#getDimensions` | method | 1 |
| `net/minecraft/world/entity/Entity#getPose` | method | 1 |
| `net/minecraft/world/entity/Entity#getRelativePortalPosition` | method | 1 |
| `net/minecraft/world/entity/Entity#isPassenger` | method | 1 |
| `net/minecraft/world/entity/Entity#isVehicle` | method | 1 |
| `net/minecraft/world/entity/Entity#level` | method | 2 |
| `net/minecraft/world/entity/Entity#onAboveBubbleColumn` | method | 1 |
| `net/minecraft/world/entity/Entity#onInsideBubbleColumn` | method | 1 |
| `net/minecraft/world/entity/Entity#placePortalTicket` | method | 1 |
| `net/minecraft/world/entity/Entity#remove` | method | 1 |
| `net/minecraft/world/entity/Entity#setAsInsidePortal` | method | 1 |
| `net/minecraft/world/entity/Entity#setIsInPowderSnow` | method | 1 |
| `net/minecraft/world/entity/Entity$MovementEmission#EVENTS` | field | 2 |
| `net/minecraft/world/entity/EntityDimensions#height` | method | 2 |
| `net/minecraft/world/entity/EntityDimensions#width` | method | 1 |
| `net/minecraft/world/entity/EntitySpawnReason#BREEDING` | field | 2 |
| `net/minecraft/world/entity/EntitySpawnReason#STRUCTURE` | field | 1 |
| `net/minecraft/world/entity/EntityType$Builder#build` | method | 1 |
| `net/minecraft/world/entity/EntityType$Builder#of` | method | 1 |
| `net/minecraft/world/entity/EntityType$Builder#sized` | method | 1 |
| `net/minecraft/world/entity/EntityTypes#ENDERMAN` | field | 20 |
| `net/minecraft/world/entity/EntityTypes#END_CRYSTAL` | field | 1 |
| `net/minecraft/world/entity/EntityTypes#PHANTOM` | field | 1 |
| `net/minecraft/world/entity/EntityTypes#PLAYER` | field | 1 |
| `net/minecraft/world/entity/EquipmentSlot#CHEST` | field | 3 |
| `net/minecraft/world/entity/EquipmentSlot#FEET` | field | 2 |
| `net/minecraft/world/entity/EquipmentSlot#HEAD` | field | 3 |
| `net/minecraft/world/entity/EquipmentSlot#LEGS` | field | 2 |
| `net/minecraft/world/entity/EquipmentSlot#MAINHAND` | field | 1 |
| `net/minecraft/world/entity/EquipmentSlot#isArmor` | method | 1 |
| `net/minecraft/world/entity/EquipmentSlot#values` | method | 1 |
| `net/minecraft/world/entity/EquipmentSlotGroup#HEAD` | field | 2 |
| `net/minecraft/world/entity/EquipmentSlotGroup#LEGS` | field | 1 |
| `net/minecraft/world/entity/EquipmentSlotGroup#MAINHAND` | field | 2 |
| `net/minecraft/world/entity/EquipmentSlotGroup#bySlot` | method | 1 |
| `net/minecraft/world/entity/ExperienceOrb#award` | method | 1 |
| `net/minecraft/world/entity/HumanoidArm#RIGHT` | field | 1 |
| `net/minecraft/world/entity/LivingEntity#addEffect` | method | 6 |
| `net/minecraft/world/entity/LivingEntity#canBeAffected` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#canGlideUsing` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#createLivingAttributes` | method | 6 |
| `net/minecraft/world/entity/LivingEntity#dealDefaultKnockback` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#getItemBySlot` | method | 2 |
| `net/minecraft/world/entity/LivingEntity#getMainHandItem` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#getY` | method | 2 |
| `net/minecraft/world/entity/LivingEntity#hurtServer` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#isAlive` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#isFallFlying` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#tickCount` | field | 2 |
| `net/minecraft/world/entity/LivingEntity#tickEffects` | method | 1 |
| `net/minecraft/world/entity/Mob#getAttributeValue` | method | 1 |
| `net/minecraft/world/entity/Mob#getYRot` | method | 1 |
| `net/minecraft/world/entity/Mob#setYRot` | method | 1 |
| `net/minecraft/world/entity/Mob#setZza` | method | 1 |
| `net/minecraft/world/entity/Mob#yBodyRot` | field | 1 |
| `net/minecraft/world/entity/Mob#yHeadRot` | field | 1 |
| `net/minecraft/world/entity/MobCategory#AMBIENT` | field | 1 |
| `net/minecraft/world/entity/MobCategory#MONSTER` | field | 1 |
| `net/minecraft/world/entity/MobCategory#WATER_AMBIENT` | field | 1 |
| `net/minecraft/world/entity/ai/attributes/Attribute#<init>` | method | 1 |
| `net/minecraft/world/entity/ai/attributes/AttributeInstance#setBaseValue` | method | 2 |
| `net/minecraft/world/entity/ai/attributes/AttributeMap#getValue` | method | 1 |
| `net/minecraft/world/entity/ai/attributes/AttributeModifier#<init>` | method | 3 |
| `net/minecraft/world/entity/ai/attributes/AttributeModifier#amount` | method | 1 |
| `net/minecraft/world/entity/ai/attributes/AttributeModifier$Operation#ADD_VALUE` | field | 3 |
| `net/minecraft/world/entity/ai/attributes/AttributeSupplier$Builder#add` | method | 9 |
| `net/minecraft/world/entity/ai/attributes/Attributes#ARMOR` | field | 2 |
| `net/minecraft/world/entity/ai/attributes/Attributes#ARMOR_TOUGHNESS` | field | 1 |
| `net/minecraft/world/entity/ai/attributes/Attributes#ATTACK_DAMAGE` | field | 2 |
| `net/minecraft/world/entity/ai/attributes/Attributes#ATTACK_KNOCKBACK` | field | 2 |
| `net/minecraft/world/entity/ai/attributes/Attributes#FLYING_SPEED` | field | 2 |
| `net/minecraft/world/entity/ai/attributes/Attributes#FOLLOW_RANGE` | field | 6 |
| `net/minecraft/world/entity/ai/attributes/Attributes#KNOCKBACK_RESISTANCE` | field | 1 |
| `net/minecraft/world/entity/ai/attributes/Attributes#MAX_HEALTH` | field | 6 |
| `net/minecraft/world/entity/ai/attributes/Attributes#MOVEMENT_SPEED` | field | 7 |
| `net/minecraft/world/entity/ai/attributes/Attributes#SCALE` | field | 2 |
| `net/minecraft/world/entity/ai/attributes/Attributes#SPAWN_REINFORCEMENTS_CHANCE` | field | 1 |
| `net/minecraft/world/entity/ai/attributes/Attributes#TEMPT_RANGE` | field | 1 |
| `net/minecraft/world/entity/ai/control/FlyingMoveControl#<init>` | method | 2 |
| `net/minecraft/world/entity/ai/control/JumpControl#jump` | method | 2 |
| `net/minecraft/world/entity/ai/control/LookControl#<init>` | method | 2 |
| `net/minecraft/world/entity/ai/control/MoveControl#<init>` | method | 1 |
| `net/minecraft/world/entity/ai/control/MoveControl$Operation#MOVE_TO` | field | 1 |
| `net/minecraft/world/entity/ai/control/MoveControl$Operation#WAIT` | field | 1 |
| `net/minecraft/world/entity/ai/goal/BreedGoal#<init>` | method | 2 |
| `net/minecraft/world/entity/ai/goal/FloatGoal#<init>` | method | 2 |
| `net/minecraft/world/entity/ai/goal/FollowParentGoal#<init>` | method | 2 |
| `net/minecraft/world/entity/ai/goal/Goal#<init>` | method | 7 |
| `net/minecraft/world/entity/ai/goal/Goal#start` | method | 2 |
| `net/minecraft/world/entity/ai/goal/Goal#tick` | method | 1 |
| `net/minecraft/world/entity/ai/goal/Goal$Flag#JUMP` | field | 2 |
| `net/minecraft/world/entity/ai/goal/Goal$Flag#LOOK` | field | 2 |
| `net/minecraft/world/entity/ai/goal/Goal$Flag#MOVE` | field | 5 |
| `net/minecraft/world/entity/ai/goal/GoalSelector#addGoal` | method | 4 |
| `net/minecraft/world/entity/ai/goal/LookAtPlayerGoal#<init>` | method | 1 |
| `net/minecraft/world/entity/ai/goal/MeleeAttackGoal#<init>` | method | 1 |
| `net/minecraft/world/entity/ai/goal/MeleeAttackGoal#start` | method | 1 |
| `net/minecraft/world/entity/ai/goal/MeleeAttackGoal#stop` | method | 1 |
| `net/minecraft/world/entity/ai/goal/MeleeAttackGoal#tick` | method | 1 |
| `net/minecraft/world/entity/ai/goal/RandomLookAroundGoal#<init>` | method | 1 |
| `net/minecraft/world/entity/ai/goal/TemptGoal#<init>` | method | 1 |
| `net/minecraft/world/entity/ai/goal/WaterAvoidingRandomStrollGoal#<init>` | method | 1 |
| `net/minecraft/world/entity/ai/goal/target/NearestAttackableTargetGoal#<init>` | method | 3 |
| `net/minecraft/world/entity/ai/navigation/FlyingPathNavigation#<init>` | method | 2 |
| `net/minecraft/world/entity/ai/navigation/FlyingPathNavigation#setCanFloat` | method | 2 |
| `net/minecraft/world/entity/ai/navigation/FlyingPathNavigation#setCanOpenDoors` | method | 2 |
| `net/minecraft/world/entity/ai/navigation/FlyingPathNavigation#tick` | method | 2 |
| `net/minecraft/world/entity/ai/navigation/PathNavigation#createPath` | method | 3 |
| `net/minecraft/world/entity/ai/navigation/PathNavigation#isDone` | method | 3 |
| `net/minecraft/world/entity/ai/navigation/PathNavigation#isInProgress` | method | 3 |
| `net/minecraft/world/entity/ai/navigation/PathNavigation#moveTo` | method | 3 |
| `net/minecraft/world/entity/ai/navigation/PathNavigation#setCanFloat` | method | 1 |
| `net/minecraft/world/entity/ai/util/AirAndWaterRandomPos#getPos` | method | 2 |
| `net/minecraft/world/entity/ai/util/HoverRandomPos#getPos` | method | 2 |
| `net/minecraft/world/entity/ai/village/poi/PoiManager#ensureLoadedAndValid` | method | 1 |
| `net/minecraft/world/entity/animal/Animal#<init>` | method | 1 |
| `net/minecraft/world/entity/animal/Animal#addAdditionalSaveData` | method | 1 |
| `net/minecraft/world/entity/animal/Animal#mobInteract` | method | 1 |
| `net/minecraft/world/entity/animal/Animal#readAdditionalSaveData` | method | 1 |
| `net/minecraft/world/entity/animal/fish/AbstractSchoolingFish#<init>` | method | 2 |
| `net/minecraft/world/entity/animal/fish/AbstractSchoolingFish#addAdditionalSaveData` | method | 2 |
| `net/minecraft/world/entity/animal/fish/AbstractSchoolingFish#defineSynchedData` | method | 2 |
| `net/minecraft/world/entity/animal/fish/AbstractSchoolingFish#finalizeSpawn` | method | 2 |
| `net/minecraft/world/entity/animal/fish/AbstractSchoolingFish#readAdditionalSaveData` | method | 2 |
| `net/minecraft/world/entity/animal/fish/AbstractSchoolingFish#saveToBucketTag` | method | 2 |
| `net/minecraft/world/entity/animal/fish/AbstractSchoolingFish#tick` | method | 1 |
| `net/minecraft/world/entity/boss/enderdragon/EndCrystal#blockPosition` | method | 1 |
| `net/minecraft/world/entity/boss/enderdragon/EndCrystal#setBeamTarget` | method | 1 |
| `net/minecraft/world/entity/boss/enderdragon/EndCrystal#setPermanentlyInvulnerable` | method | 1 |
| `net/minecraft/world/entity/boss/enderdragon/EndCrystal#snapTo` | method | 1 |
| `net/minecraft/world/entity/item/ItemEntity#<init>` | method | 9 |
| `net/minecraft/world/entity/monster/Enderman#isBeingStaredBy` | method | 1 |
| `net/minecraft/world/entity/monster/Monster#<init>` | method | 1 |
| `net/minecraft/world/entity/monster/Monster#createMonsterAttributes` | method | 1 |
| `net/minecraft/world/entity/monster/Monster#doHurtTarget` | method | 1 |
| `net/minecraft/world/entity/monster/Monster#tick` | method | 1 |
| `net/minecraft/world/entity/monster/cubemob/Slime#<init>` | method | 1 |
| `net/minecraft/world/entity/monster/cubemob/Slime#addAdditionalSaveData` | method | 1 |
| `net/minecraft/world/entity/monster/cubemob/Slime#defineSynchedData` | method | 1 |
| `net/minecraft/world/entity/monster/cubemob/Slime#finalizeSpawn` | method | 1 |
| `net/minecraft/world/entity/monster/cubemob/Slime#readAdditionalSaveData` | method | 1 |
| `net/minecraft/world/entity/monster/cubemob/Slime#setUpSplitCube` | method | 1 |
| `net/minecraft/world/entity/player/Abilities#invulnerable` | field | 1 |
| `net/minecraft/world/entity/player/Inventory#player` | field | 1 |
| `net/minecraft/world/entity/player/Player#addEffect` | method | 1 |
| `net/minecraft/world/entity/player/Player#addItem` | method | 3 |
| `net/minecraft/world/entity/player/Player#damageSources` | method | 1 |
| `net/minecraft/world/entity/player/Player#distanceToSqr` | method | 1 |
| `net/minecraft/world/entity/player/Player#getAbilities` | method | 1 |
| `net/minecraft/world/entity/player/Player#getItemBySlot` | method | 1 |
| `net/minecraft/world/entity/player/Player#getItemInHand` | method | 1 |
| `net/minecraft/world/entity/player/Player#getMainHandItem` | method | 1 |
| `net/minecraft/world/entity/player/Player#hurtServer` | method | 1 |
| `net/minecraft/world/entity/player/Player#isCreative` | method | 8 |
| `net/minecraft/world/entity/player/Player#isSecondaryUseActive` | method | 1 |
| `net/minecraft/world/entity/player/Player#level` | method | 3 |
| `net/minecraft/world/entity/player/Player#openMenu` | method | 1 |
| `net/minecraft/world/entity/player/PlayerModelType#SLIM` | field | 1 |
| `net/minecraft/world/entity/player/PlayerSkin#model` | method | 1 |
| `net/minecraft/world/entity/vehicle/DismountHelper#findSafeDismountLocation` | method | 1 |
| `net/minecraft/world/flag/FeatureFlags#DEFAULT_FLAGS` | field | 1 |
| `net/minecraft/world/food/FoodProperties#nutrition` | method | 2 |
| `net/minecraft/world/food/FoodProperties#saturation` | method | 2 |
| `net/minecraft/world/food/Foods#APPLE` | field | 1 |
| `net/minecraft/world/food/Foods#COOKED_SALMON` | field | 1 |
| `net/minecraft/world/food/Foods#MUSHROOM_STEW` | field | 1 |
| `net/minecraft/world/food/Foods#PUMPKIN_PIE` | field | 1 |
| `net/minecraft/world/food/Foods#SALMON` | field | 1 |
| `net/minecraft/world/inventory/AbstractContainerMenu#getRedstoneSignalFromBlockEntity` | method | 1 |
| `net/minecraft/world/inventory/ContainerData#get` | method | 1 |
| `net/minecraft/world/inventory/ContainerLevelAccess#evaluate` | method | 1 |
| `net/minecraft/world/inventory/CraftingMenu#stillValid` | method | 1 |
| `net/minecraft/world/inventory/FurnaceFuelSlot#isBucket` | method | 1 |
| `net/minecraft/world/inventory/MenuType#<init>` | method | 1 |
| `net/minecraft/world/inventory/RecipeBookMenu#<init>` | method | 1 |
| `net/minecraft/world/inventory/RecipeBookType#BLAST_FURNACE` | field | 1 |
| `net/minecraft/world/inventory/SimpleContainerData#<init>` | method | 1 |
| `net/minecraft/world/inventory/Slot#<init>` | method | 3 |
| `net/minecraft/world/inventory/Slot#getItem` | method | 1 |
| `net/minecraft/world/inventory/Slot#getMaxStackSize` | method | 1 |
| `net/minecraft/world/inventory/Slot#hasItem` | method | 1 |
| `net/minecraft/world/inventory/Slot#onQuickCraft` | method | 1 |
| `net/minecraft/world/inventory/Slot#onTake` | method | 2 |
| `net/minecraft/world/inventory/Slot#set` | method | 1 |
| `net/minecraft/world/inventory/Slot#setChanged` | method | 1 |
| `net/minecraft/world/inventory/StackedContentsCompatible#fillStackedContents` | method | 1 |
| `net/minecraft/world/item/BlockItem#<init>` | method | 1 |
| `net/minecraft/world/item/BlockItem#getBlock` | method | 3 |
| `net/minecraft/world/item/DyeColor#BLACK` | field | 1 |
| `net/minecraft/world/item/DyeColor#BLUE` | field | 1 |
| `net/minecraft/world/item/DyeColor#BROWN` | field | 1 |
| `net/minecraft/world/item/DyeColor#CYAN` | field | 1 |
| `net/minecraft/world/item/DyeColor#GRAY` | field | 1 |
| `net/minecraft/world/item/DyeColor#GREEN` | field | 1 |
| `net/minecraft/world/item/DyeColor#LIGHT_BLUE` | field | 1 |
| `net/minecraft/world/item/DyeColor#LIGHT_GRAY` | field | 1 |
| `net/minecraft/world/item/DyeColor#LIME` | field | 1 |
| `net/minecraft/world/item/DyeColor#MAGENTA` | field | 2 |
| `net/minecraft/world/item/DyeColor#ORANGE` | field | 1 |
| `net/minecraft/world/item/DyeColor#PINK` | field | 1 |
| `net/minecraft/world/item/DyeColor#PURPLE` | field | 1 |
| `net/minecraft/world/item/DyeColor#RED` | field | 1 |
| `net/minecraft/world/item/DyeColor#WHITE` | field | 1 |
| `net/minecraft/world/item/DyeColor#YELLOW` | field | 1 |
| `net/minecraft/world/item/DyeColor#getMapColor` | method | 1 |
| `net/minecraft/world/item/DyeColor#getName` | method | 1 |
| `net/minecraft/world/item/Item#asItem` | method | 2 |
| `net/minecraft/world/item/Item#getCraftingRemainder` | method | 1 |
| `net/minecraft/world/item/Item#getDefaultInstance` | method | 1 |
| `net/minecraft/world/item/Item$Properties#<init>` | method | 1 |
| `net/minecraft/world/item/Item$Properties#attributes` | method | 1 |
| `net/minecraft/world/item/Item$Properties#component` | method | 1 |
| `net/minecraft/world/item/Item$Properties#fireResistant` | method | 2 |
| `net/minecraft/world/item/Item$Properties#rarity` | method | 1 |
| `net/minecraft/world/item/ItemDisplayContext#GROUND` | field | 1 |
| `net/minecraft/world/item/ItemStack#<init>` | method | 26 |
| `net/minecraft/world/item/ItemStack#CODEC` | field | 1 |
| `net/minecraft/world/item/ItemStack#EMPTY` | field | 5 |
| `net/minecraft/world/item/ItemStack#OPTIONAL_CODEC` | field | 1 |
| `net/minecraft/world/item/ItemStack#applyComponents` | method | 1 |
| `net/minecraft/world/item/ItemStack#canDestroyBlock` | method | 1 |
| `net/minecraft/world/item/ItemStack#copy` | method | 3 |
| `net/minecraft/world/item/ItemStack#forEachModifier` | method | 1 |
| `net/minecraft/world/item/ItemStack#get` | method | 2 |
| `net/minecraft/world/item/ItemStack#getComponentsPatch` | method | 1 |
| `net/minecraft/world/item/ItemStack#getDamageValue` | method | 2 |
| `net/minecraft/world/item/ItemStack#getHoverName` | method | 1 |
| `net/minecraft/world/item/ItemStack#getItem` | method | 11 |
| `net/minecraft/world/item/ItemStack#getMaxStackSize` | method | 1 |
| `net/minecraft/world/item/ItemStack#getTooltipLines` | method | 1 |
| `net/minecraft/world/item/ItemStack#grow` | method | 1 |
| `net/minecraft/world/item/ItemStack#has` | method | 1 |
| `net/minecraft/world/item/ItemStack#is` | method | 7 |
| `net/minecraft/world/item/ItemStack#isEmpty` | method | 12 |
| `net/minecraft/world/item/ItemStack#limitSize` | method | 1 |
| `net/minecraft/world/item/ItemStack#mineBlock` | method | 1 |
| `net/minecraft/world/item/ItemStack#onCraftedBy` | method | 1 |
| `net/minecraft/world/item/ItemStack#setDamageValue` | method | 2 |
| `net/minecraft/world/item/ItemStack#split` | method | 1 |
| `net/minecraft/world/item/ItemStackTemplate#<init>` | method | 1 |
| `net/minecraft/world/item/ItemStackTemplate#CODEC` | field | 1 |
| `net/minecraft/world/item/ItemStackTemplate#STREAM_CODEC` | field | 1 |
| `net/minecraft/world/item/ItemStackTemplate#fromNonEmptyStack` | method | 1 |
| `net/minecraft/world/item/ItemStackTemplate#item` | method | 1 |
| `net/minecraft/world/item/Items#AIR` | field | 3 |
| `net/minecraft/world/item/Items#ARROW` | field | 1 |
| `net/minecraft/world/item/Items#BLAZE_POWDER` | field | 1 |
| `net/minecraft/world/item/Items#BLAZE_ROD` | field | 1 |
| `net/minecraft/world/item/Items#BONE_MEAL` | field | 2 |
| `net/minecraft/world/item/Items#BOOK` | field | 3 |
| `net/minecraft/world/item/Items#BOW` | field | 1 |
| `net/minecraft/world/item/Items#BUCKET` | field | 2 |
| `net/minecraft/world/item/Items#CHARCOAL` | field | 1 |
| `net/minecraft/world/item/Items#CHORUS_FRUIT` | field | 1 |
| `net/minecraft/world/item/Items#COAL` | field | 1 |
| `net/minecraft/world/item/Items#COPPER_INGOT` | field | 1 |
| `net/minecraft/world/item/Items#COPPER_ORE` | field | 1 |
| `net/minecraft/world/item/Items#DEEPSLATE_COPPER_ORE` | field | 1 |
| `net/minecraft/world/item/Items#DEEPSLATE_GOLD_ORE` | field | 1 |
| `net/minecraft/world/item/Items#DEEPSLATE_IRON_ORE` | field | 1 |
| `net/minecraft/world/item/Items#DIAMOND` | field | 4 |
| `net/minecraft/world/item/Items#DIAMOND_SWORD` | field | 1 |
| `net/minecraft/world/item/Items#DRAGON_BREATH` | field | 1 |
| `net/minecraft/world/item/Items#DYE` | field | 1 |
| `net/minecraft/world/item/Items#ECHO_SHARD` | field | 1 |
| `net/minecraft/world/item/Items#ELYTRA` | field | 4 |
| `net/minecraft/world/item/Items#EMERALD` | field | 1 |
| `net/minecraft/world/item/Items#ENCHANTED_BOOK` | field | 1 |
| `net/minecraft/world/item/Items#ENDER_EYE` | field | 5 |
| `net/minecraft/world/item/Items#ENDER_PEARL` | field | 4 |
| `net/minecraft/world/item/Items#END_CRYSTAL` | field | 3 |
| `net/minecraft/world/item/Items#EXPERIENCE_BOTTLE` | field | 1 |
| `net/minecraft/world/item/Items#FEATHER` | field | 1 |
| `net/minecraft/world/item/Items#FERMENTED_SPIDER_EYE` | field | 1 |
| `net/minecraft/world/item/Items#FIREWORK_ROCKET` | field | 1 |
| `net/minecraft/world/item/Items#FISHING_ROD` | field | 2 |
| `net/minecraft/world/item/Items#GHAST_TEAR` | field | 1 |
| `net/minecraft/world/item/Items#GLASS_BOTTLE` | field | 2 |
| `net/minecraft/world/item/Items#GLOWSTONE_DUST` | field | 1 |
| `net/minecraft/world/item/Items#GOLDEN_SWORD` | field | 1 |
| `net/minecraft/world/item/Items#GOLD_INGOT` | field | 5 |
| `net/minecraft/world/item/Items#GOLD_NUGGET` | field | 1 |
| `net/minecraft/world/item/Items#GOLD_ORE` | field | 1 |
| `net/minecraft/world/item/Items#GRASS_BLOCK` | field | 1 |
| `net/minecraft/world/item/Items#GUNPOWDER` | field | 1 |
| `net/minecraft/world/item/Items#HEART_OF_THE_SEA` | field | 1 |
| `net/minecraft/world/item/Items#IRON_CHAIN` | field | 2 |
| `net/minecraft/world/item/Items#IRON_DOOR` | field | 1 |
| `net/minecraft/world/item/Items#IRON_INGOT` | field | 6 |
| `net/minecraft/world/item/Items#IRON_ORE` | field | 1 |
| `net/minecraft/world/item/Items#IRON_SWORD` | field | 1 |
| `net/minecraft/world/item/Items#LANTERN` | field | 1 |
| `net/minecraft/world/item/Items#LAPIS_LAZULI` | field | 1 |
| `net/minecraft/world/item/Items#LEAD` | field | 1 |
| `net/minecraft/world/item/Items#LEATHER` | field | 2 |
| `net/minecraft/world/item/Items#MAGMA_CREAM` | field | 1 |
| `net/minecraft/world/item/Items#NAUTILUS_SHELL` | field | 1 |
| `net/minecraft/world/item/Items#NETHERITE_INGOT` | field | 2 |
| `net/minecraft/world/item/Items#NETHERITE_SCRAP` | field | 2 |
| `net/minecraft/world/item/Items#OAK_DOOR` | field | 1 |
| `net/minecraft/world/item/Items#OAK_LEAVES` | field | 1 |
| `net/minecraft/world/item/Items#PAPER` | field | 1 |
| `net/minecraft/world/item/Items#PHANTOM_MEMBRANE` | field | 2 |
| `net/minecraft/world/item/Items#POPPED_CHORUS_FRUIT` | field | 1 |
| `net/minecraft/world/item/Items#POTION` | field | 1 |
| `net/minecraft/world/item/Items#PRISMARINE_CRYSTALS` | field | 1 |
| `net/minecraft/world/item/Items#PRISMARINE_SHARD` | field | 1 |
| `net/minecraft/world/item/Items#QUARTZ` | field | 1 |
| `net/minecraft/world/item/Items#RABBIT_FOOT` | field | 1 |
| `net/minecraft/world/item/Items#RAW_COPPER` | field | 1 |
| `net/minecraft/world/item/Items#RAW_GOLD` | field | 1 |
| `net/minecraft/world/item/Items#RAW_IRON` | field | 1 |
| `net/minecraft/world/item/Items#REDSTONE` | field | 2 |
| `net/minecraft/world/item/Items#SCULK_CATALYST` | field | 1 |
| `net/minecraft/world/item/Items#SCULK_SENSOR` | field | 1 |
| `net/minecraft/world/item/Items#SCULK_VEIN` | field | 1 |
| `net/minecraft/world/item/Items#SHIELD` | field | 1 |
| `net/minecraft/world/item/Items#SHULKER_SHELL` | field | 1 |
| `net/minecraft/world/item/Items#SLIME_BALL` | field | 3 |
| `net/minecraft/world/item/Items#SPECTRAL_ARROW` | field | 1 |
| `net/minecraft/world/item/Items#STICK` | field | 3 |
| `net/minecraft/world/item/Items#STRING` | field | 1 |
| `net/minecraft/world/item/Items#SUGAR` | field | 1 |
| `net/minecraft/world/item/Items#SWEET_BERRIES` | field | 1 |
| `net/minecraft/world/item/Items#TURTLE_HELMET` | field | 2 |
| `net/minecraft/world/item/Items#TURTLE_SCUTE` | field | 1 |
| `net/minecraft/world/item/Items#WATER_BUCKET` | field | 3 |
| `net/minecraft/world/item/JukeboxSong#<init>` | method | 1 |
| `net/minecraft/world/item/MobBucketItem#<init>` | method | 1 |
| `net/minecraft/world/item/PlaceOnWaterBlockItem#<init>` | method | 1 |
| `net/minecraft/world/item/Rarity#EPIC` | field | 2 |
| `net/minecraft/world/item/Rarity#RARE` | field | 2 |
| `net/minecraft/world/item/ToolMaterial#<init>` | method | 1 |
| `net/minecraft/world/item/ToolMaterial#repairItems` | method | 1 |
| `net/minecraft/world/item/alchemy/Potions#AWKWARD` | field | 1 |
| `net/minecraft/world/item/alchemy/Potions#NIGHT_VISION` | field | 1 |
| `net/minecraft/world/item/component/CookingFuel#burnTime` | method | 1 |
| `net/minecraft/world/item/component/CustomData#update` | method | 2 |
| `net/minecraft/world/item/component/ItemAttributeModifiers#<init>` | method | 2 |
| `net/minecraft/world/item/component/ItemAttributeModifiers$Entry#<init>` | method | 3 |
| `net/minecraft/world/item/context/BlockPlaceContext#canPlace` | method | 1 |
| `net/minecraft/world/item/context/BlockPlaceContext#getClickedFace` | method | 4 |
| `net/minecraft/world/item/context/BlockPlaceContext#getClickedPos` | method | 15 |
| `net/minecraft/world/item/context/BlockPlaceContext#getHorizontalDirection` | method | 3 |
| `net/minecraft/world/item/context/BlockPlaceContext#getItemInHand` | method | 1 |
| `net/minecraft/world/item/context/BlockPlaceContext#getLevel` | method | 12 |
| `net/minecraft/world/item/crafting/AbstractCookingRecipe#assemble` | method | 1 |
| `net/minecraft/world/item/crafting/AbstractCookingRecipe#cookingTime` | method | 1 |
| `net/minecraft/world/item/crafting/BlastingRecipe#cookingTime` | method | 1 |
| `net/minecraft/world/item/crafting/BlastingRecipe#experience` | method | 1 |
| `net/minecraft/world/item/crafting/Ingredient#CODEC` | field | 1 |
| `net/minecraft/world/item/crafting/Ingredient#CONTENTS_STREAM_CODEC` | field | 1 |
| `net/minecraft/world/item/crafting/Ingredient#OPTIONAL_CONTENTS_STREAM_CODEC` | field | 1 |
| `net/minecraft/world/item/crafting/Ingredient#isEmpty` | method | 1 |
| `net/minecraft/world/item/crafting/Ingredient#items` | method | 1 |
| `net/minecraft/world/item/crafting/Ingredient#of` | method | 3 |
| `net/minecraft/world/item/crafting/Ingredient#test` | method | 2 |
| `net/minecraft/world/item/crafting/Ingredient#testOptionalIngredient` | method | 1 |
| `net/minecraft/world/item/crafting/PlacementInfo#create` | method | 1 |
| `net/minecraft/world/item/crafting/Recipe#KEY_CODEC` | field | 1 |
| `net/minecraft/world/item/crafting/RecipeHolder#id` | method | 1 |
| `net/minecraft/world/item/crafting/RecipeHolder#value` | method | 5 |
| `net/minecraft/world/item/crafting/RecipeManager#byKey` | method | 1 |
| `net/minecraft/world/item/crafting/RecipeManager#createCheck` | method | 1 |
| `net/minecraft/world/item/crafting/RecipeManager#getRecipeFor` | method | 3 |
| `net/minecraft/world/item/crafting/RecipeManager#getRecipes` | method | 1 |
| `net/minecraft/world/item/crafting/RecipeManager$CachedCheck#getRecipeFor` | method | 1 |
| `net/minecraft/world/item/crafting/RecipeSerializer#<init>` | method | 1 |
| `net/minecraft/world/item/crafting/RecipeType#BLASTING` | field | 2 |
| `net/minecraft/world/item/crafting/SingleRecipeInput#<init>` | method | 2 |
| `net/minecraft/world/item/enchantment/Enchantment#constantCost` | method | 1 |
| `net/minecraft/world/item/enchantment/Enchantment#definition` | method | 1 |
| `net/minecraft/world/item/enchantment/Enchantment#description` | method | 1 |
| `net/minecraft/world/item/enchantment/Enchantment#dynamicCost` | method | 1 |
| `net/minecraft/world/item/enchantment/Enchantment#enchantment` | method | 1 |
| `net/minecraft/world/item/enchantment/Enchantment#getMaxLevel` | method | 1 |
| `net/minecraft/world/item/enchantment/Enchantment$Builder#withEffect` | method | 1 |
| `net/minecraft/world/item/enchantment/EnchantmentHelper#has` | method | 1 |
| `net/minecraft/world/item/enchantment/Enchantments#AQUA_AFFINITY` | field | 1 |
| `net/minecraft/world/item/enchantment/Enchantments#BANE_OF_ARTHROPODS` | field | 1 |
| `net/minecraft/world/item/enchantment/Enchantments#BLAST_PROTECTION` | field | 1 |
| `net/minecraft/world/item/enchantment/Enchantments#CHANNELING` | field | 1 |
| `net/minecraft/world/item/enchantment/Enchantments#DEPTH_STRIDER` | field | 1 |
| `net/minecraft/world/item/enchantment/Enchantments#EFFICIENCY` | field | 1 |
| `net/minecraft/world/item/enchantment/Enchantments#FEATHER_FALLING` | field | 1 |
| `net/minecraft/world/item/enchantment/Enchantments#FIRE_ASPECT` | field | 2 |
| `net/minecraft/world/item/enchantment/Enchantments#FIRE_PROTECTION` | field | 1 |
| `net/minecraft/world/item/enchantment/Enchantments#FLAME` | field | 1 |
| `net/minecraft/world/item/enchantment/Enchantments#FORTUNE` | field | 1 |
| `net/minecraft/world/item/enchantment/Enchantments#FROST_WALKER` | field | 1 |
| `net/minecraft/world/item/enchantment/Enchantments#IMPALING` | field | 1 |
| `net/minecraft/world/item/enchantment/Enchantments#INFINITY` | field | 1 |
| `net/minecraft/world/item/enchantment/Enchantments#KNOCKBACK` | field | 1 |
| `net/minecraft/world/item/enchantment/Enchantments#LOOTING` | field | 3 |
| `net/minecraft/world/item/enchantment/Enchantments#LOYALTY` | field | 1 |
| `net/minecraft/world/item/enchantment/Enchantments#LUCK_OF_THE_SEA` | field | 1 |
| `net/minecraft/world/item/enchantment/Enchantments#LUNGE` | field | 1 |
| `net/minecraft/world/item/enchantment/Enchantments#LURE` | field | 1 |
| `net/minecraft/world/item/enchantment/Enchantments#MENDING` | field | 1 |
| `net/minecraft/world/item/enchantment/Enchantments#MULTISHOT` | field | 1 |
| `net/minecraft/world/item/enchantment/Enchantments#PIERCING` | field | 1 |
| `net/minecraft/world/item/enchantment/Enchantments#POWER` | field | 1 |
| `net/minecraft/world/item/enchantment/Enchantments#PROJECTILE_PROTECTION` | field | 1 |
| `net/minecraft/world/item/enchantment/Enchantments#PROTECTION` | field | 1 |
| `net/minecraft/world/item/enchantment/Enchantments#PUNCH` | field | 1 |
| `net/minecraft/world/item/enchantment/Enchantments#QUICK_CHARGE` | field | 1 |
| `net/minecraft/world/item/enchantment/Enchantments#RESPIRATION` | field | 1 |
| `net/minecraft/world/item/enchantment/Enchantments#RIPTIDE` | field | 1 |
| `net/minecraft/world/item/enchantment/Enchantments#SHARPNESS` | field | 1 |
| `net/minecraft/world/item/enchantment/Enchantments#SILK_TOUCH` | field | 2 |
| `net/minecraft/world/item/enchantment/Enchantments#SMITE` | field | 1 |
| `net/minecraft/world/item/enchantment/Enchantments#SOUL_SPEED` | field | 1 |
| `net/minecraft/world/item/enchantment/Enchantments#SWEEPING_EDGE` | field | 1 |
| `net/minecraft/world/item/enchantment/Enchantments#SWIFT_SNEAK` | field | 1 |
| `net/minecraft/world/item/enchantment/Enchantments#THORNS` | field | 1 |
| `net/minecraft/world/item/enchantment/Enchantments#UNBREAKING` | field | 1 |
| `net/minecraft/world/item/enchantment/ItemEnchantments#EMPTY` | field | 1 |
| `net/minecraft/world/item/enchantment/ItemEnchantments#getLevel` | method | 1 |
| `net/minecraft/world/item/enchantment/ItemEnchantments#isEmpty` | method | 1 |
| `net/minecraft/world/item/enchantment/ItemEnchantments#keySet` | method | 1 |
| `net/minecraft/world/item/enchantment/ItemEnchantments$Mutable#<init>` | method | 1 |
| `net/minecraft/world/item/enchantment/ItemEnchantments$Mutable#set` | method | 1 |
| `net/minecraft/world/item/enchantment/ItemEnchantments$Mutable#toImmutable` | method | 1 |
| `net/minecraft/world/item/equipment/ArmorMaterial#assetId` | method | 1 |
| `net/minecraft/world/item/equipment/ArmorMaterial#defense` | method | 1 |
| `net/minecraft/world/item/equipment/ArmorMaterial#equipSound` | method | 1 |
| `net/minecraft/world/item/equipment/ArmorMaterial#knockbackResistance` | method | 1 |
| `net/minecraft/world/item/equipment/ArmorMaterial#repairIngredient` | method | 2 |
| `net/minecraft/world/item/equipment/ArmorMaterial#toughness` | method | 1 |
| `net/minecraft/world/item/equipment/ArmorType#getDurability` | method | 1 |
| `net/minecraft/world/item/equipment/ArmorType#getSlot` | method | 1 |
| `net/minecraft/world/item/equipment/Equippable#builder` | method | 1 |
| `net/minecraft/world/item/equipment/Equippable$Builder#build` | method | 1 |
| `net/minecraft/world/item/equipment/Equippable$Builder#setAsset` | method | 1 |
| `net/minecraft/world/item/equipment/Equippable$Builder#setEquipSound` | method | 1 |
| `net/minecraft/world/item/trading/TradeCost#<init>` | method | 1 |
| `net/minecraft/world/item/trading/TradeSets#register` | method | 1 |
| `net/minecraft/world/item/trading/VillagerTrade#builder` | method | 2 |
| `net/minecraft/world/item/trading/VillagerTrade$Builder#build` | method | 1 |
| `net/minecraft/world/item/trading/VillagerTrade$Builder#merchantPredicate` | method | 1 |
| `net/minecraft/world/level/BlockGetter#getBlockEntity` | method | 1 |
| `net/minecraft/world/level/BlockGetter#getBlockState` | method | 3 |
| `net/minecraft/world/level/ChunkPos#containing` | method | 1 |
| `net/minecraft/world/level/ChunkPos#equals` | method | 1 |
| `net/minecraft/world/level/ChunkPos#getBlockX` | method | 14 |
| `net/minecraft/world/level/ChunkPos#getBlockZ` | method | 14 |
| `net/minecraft/world/level/ChunkPos#getMinBlockX` | method | 5 |
| `net/minecraft/world/level/ChunkPos#getMinBlockZ` | method | 5 |
| `net/minecraft/world/level/ChunkPos#x` | method | 10 |
| `net/minecraft/world/level/ChunkPos#z` | method | 10 |
| `net/minecraft/world/level/ItemLike#asItem` | method | 3 |
| `net/minecraft/world/level/Level#<init>` | method | 1 |
| `net/minecraft/world/level/Level#END` | field | 8 |
| `net/minecraft/world/level/Level#OVERWORLD` | field | 1 |
| `net/minecraft/world/level/Level#addAlwaysVisibleParticle` | method | 1 |
| `net/minecraft/world/level/Level#addFreshEntity` | method | 7 |
| `net/minecraft/world/level/Level#addParticle` | method | 13 |
| `net/minecraft/world/level/Level#blockEntityChanged` | method | 3 |
| `net/minecraft/world/level/Level#damageSources` | method | 2 |
| `net/minecraft/world/level/Level#dimension` | method | 4 |
| `net/minecraft/world/level/Level#dimensionType` | method | 1 |
| `net/minecraft/world/level/Level#environmentAttributes` | method | 2 |
| `net/minecraft/world/level/Level#getBlockEntity` | method | 8 |
| `net/minecraft/world/level/Level#getBlockState` | method | 23 |
| `net/minecraft/world/level/Level#getChunkAt` | method | 1 |
| `net/minecraft/world/level/Level#getEntitiesOfClass` | method | 1 |
| `net/minecraft/world/level/Level#getFluidState` | method | 1 |
| `net/minecraft/world/level/Level#getGameTime` | method | 2 |
| `net/minecraft/world/level/Level#getRandom` | method | 7 |
| `net/minecraft/world/level/Level#getServer` | method | 2 |
| `net/minecraft/world/level/Level#isEmptyBlock` | method | 2 |
| `net/minecraft/world/level/Level#levelEvent` | method | 2 |
| `net/minecraft/world/level/Level#neighborChanged` | method | 1 |
| `net/minecraft/world/level/Level#playLocalSound` | method | 5 |
| `net/minecraft/world/level/Level#playSound` | method | 5 |
| `net/minecraft/world/level/Level#registryAccess` | method | 1 |
| `net/minecraft/world/level/Level#sendBlockUpdated` | method | 1 |
| `net/minecraft/world/level/Level#setBlockAndUpdate` | method | 7 |
| `net/minecraft/world/level/LevelAccessor#dimensionType` | method | 1 |
| `net/minecraft/world/level/LevelAccessor#getBiome` | method | 1 |
| `net/minecraft/world/level/LevelAccessor#getBlockEntity` | method | 2 |
| `net/minecraft/world/level/LevelAccessor#getBlockState` | method | 5 |
| `net/minecraft/world/level/LevelAccessor#getFluidState` | method | 5 |
| `net/minecraft/world/level/LevelAccessor#getMaxY` | method | 1 |
| `net/minecraft/world/level/LevelAccessor#getMinY` | method | 1 |
| `net/minecraft/world/level/LevelAccessor#getRandom` | method | 1 |
| `net/minecraft/world/level/LevelAccessor#isClientSide` | method | 1 |
| `net/minecraft/world/level/LevelAccessor#isEmptyBlock` | method | 1 |
| `net/minecraft/world/level/LevelAccessor#registryAccess` | method | 1 |
| `net/minecraft/world/level/LevelAccessor#scheduleTick` | method | 1 |
| `net/minecraft/world/level/LevelAccessor#setBlock` | method | 2 |
| `net/minecraft/world/level/LevelHeightAccessor#getHeight` | method | 1 |
| `net/minecraft/world/level/LevelHeightAccessor#getMaxY` | method | 3 |
| `net/minecraft/world/level/LevelHeightAccessor#getMinY` | method | 2 |
| `net/minecraft/world/level/LevelReader#getBlockEntity` | method | 1 |
| `net/minecraft/world/level/LevelReader#getBlockState` | method | 25 |
| `net/minecraft/world/level/LevelReader#getFluidState` | method | 4 |
| `net/minecraft/world/level/LevelReader#isEmptyBlock` | method | 3 |
| `net/minecraft/world/level/LightLayer#BLOCK` | field | 1 |
| `net/minecraft/world/level/NoiseColumn#<init>` | method | 1 |
| `net/minecraft/world/level/ScheduledTickAccess#scheduleTick` | method | 6 |
| `net/minecraft/world/level/ServerLevelAccessor#addFreshEntity` | method | 1 |
| `net/minecraft/world/level/ServerLevelAccessor#getBiome` | method | 3 |
| `net/minecraft/world/level/ServerLevelAccessor#getBlockState` | method | 1 |
| `net/minecraft/world/level/ServerLevelAccessor#getChunk` | method | 1 |
| `net/minecraft/world/level/ServerLevelAccessor#getLevel` | method | 1 |
| `net/minecraft/world/level/ServerLevelAccessor#scheduleTick` | method | 1 |
| `net/minecraft/world/level/ServerLevelAccessor#setBlock` | method | 1 |
| `net/minecraft/world/level/StructureManager#getAllStructuresAt` | method | 2 |
| `net/minecraft/world/level/WorldGenLevel#getBlockState` | method | 57 |
| `net/minecraft/world/level/WorldGenLevel#getChunk` | method | 10 |
| `net/minecraft/world/level/WorldGenLevel#getFluidState` | method | 12 |
| `net/minecraft/world/level/WorldGenLevel#getHeight` | method | 7 |
| `net/minecraft/world/level/WorldGenLevel#getLevel` | method | 3 |
| `net/minecraft/world/level/WorldGenLevel#getMaxY` | method | 1 |
| `net/minecraft/world/level/WorldGenLevel#getMinY` | method | 1 |
| `net/minecraft/world/level/WorldGenLevel#getRandom` | method | 1 |
| `net/minecraft/world/level/WorldGenLevel#isEmptyBlock` | method | 22 |
| `net/minecraft/world/level/WorldGenLevel#isStateAtPosition` | method | 1 |
| `net/minecraft/world/level/WorldGenLevel#registryAccess` | method | 2 |
| `net/minecraft/world/level/WorldGenLevel#scheduleTick` | method | 3 |
| `net/minecraft/world/level/WorldGenLevel#setBlock` | method | 1 |
| `net/minecraft/world/level/biome/Biome#getAttributes` | method | 1 |
| `net/minecraft/world/level/biome/BiomeManager#getNoiseBiomeAtPosition` | method | 1 |
| `net/minecraft/world/level/biome/BiomeResolver#getNoiseBiome` | method | 2 |
| `net/minecraft/world/level/biome/BiomeSource#createResolver` | method | 1 |
| `net/minecraft/world/level/biome/BiomeSource#createUncachedResolver` | method | 1 |
| `net/minecraft/world/level/biome/Biomes#END_BARRENS` | field | 1 |
| `net/minecraft/world/level/biome/Biomes#END_HIGHLANDS` | field | 1 |
| `net/minecraft/world/level/biome/Biomes#END_MIDLANDS` | field | 1 |
| `net/minecraft/world/level/biome/Biomes#SMALL_END_ISLANDS` | field | 1 |
| `net/minecraft/world/level/block/AbstractFurnaceBlock#LIT` | field | 1 |
| `net/minecraft/world/level/block/Block#animateTick` | method | 1 |
| `net/minecraft/world/level/block/Block#asItem` | method | 2 |
| `net/minecraft/world/level/block/Block#box` | method | 25 |
| `net/minecraft/world/level/block/Block#builtInRegistryHolder` | method | 4 |
| `net/minecraft/world/level/block/Block#createBlockStateDefinition` | method | 1 |
| `net/minecraft/world/level/block/Block#defaultBlockState` | method | 74 |
| `net/minecraft/world/level/block/Block#defaultDestroyTime` | method | 1 |
| `net/minecraft/world/level/block/Block#defaultMapColor` | method | 4 |
| `net/minecraft/world/level/block/Block#destroy` | method | 1 |
| `net/minecraft/world/level/block/Block#getDrops` | method | 1 |
| `net/minecraft/world/level/block/Block#getExplosionResistance` | method | 4 |
| `net/minecraft/world/level/block/Block#getFluidState` | method | 3 |
| `net/minecraft/world/level/block/Block#getShape` | method | 2 |
| `net/minecraft/world/level/block/Block#getStateDefinition` | method | 1 |
| `net/minecraft/world/level/block/Block#playerDestroy` | method | 1 |
| `net/minecraft/world/level/block/Block#playerWillDestroy` | method | 3 |
| `net/minecraft/world/level/block/Block#pushEntitiesUp` | method | 1 |
| `net/minecraft/world/level/block/Block#stepOn` | method | 1 |
| `net/minecraft/world/level/block/Block#useWithoutItem` | method | 1 |
| `net/minecraft/world/level/block/Blocks#ACACIA_SAPLING` | field | 1 |
| `net/minecraft/world/level/block/Blocks#AIR` | field | 27 |
| `net/minecraft/world/level/block/Blocks#ALLIUM` | field | 1 |
| `net/minecraft/world/level/block/Blocks#ANCIENT_DEBRIS` | field | 1 |
| `net/minecraft/world/level/block/Blocks#ANDESITE` | field | 1 |
| `net/minecraft/world/level/block/Blocks#ANDESITE_SLAB` | field | 1 |
| `net/minecraft/world/level/block/Blocks#ANVIL` | field | 1 |
| `net/minecraft/world/level/block/Blocks#AZALEA` | field | 1 |
| `net/minecraft/world/level/block/Blocks#AZURE_BLUET` | field | 1 |
| `net/minecraft/world/level/block/Blocks#BAMBOO` | field | 1 |
| `net/minecraft/world/level/block/Blocks#BEDROCK` | field | 5 |
| `net/minecraft/world/level/block/Blocks#BIRCH_SAPLING` | field | 1 |
| `net/minecraft/world/level/block/Blocks#BLACKSTONE` | field | 1 |
| `net/minecraft/world/level/block/Blocks#BLACKSTONE_SLAB` | field | 1 |
| `net/minecraft/world/level/block/Blocks#BLUE_ICE` | field | 1 |
| `net/minecraft/world/level/block/Blocks#BLUE_ORCHID` | field | 1 |
| `net/minecraft/world/level/block/Blocks#BONE_BLOCK` | field | 2 |
| `net/minecraft/world/level/block/Blocks#BROWN_MUSHROOM` | field | 1 |
| `net/minecraft/world/level/block/Blocks#BUBBLE_COLUMN` | field | 1 |
| `net/minecraft/world/level/block/Blocks#CACTUS` | field | 3 |
| `net/minecraft/world/level/block/Blocks#CAVE_AIR` | field | 3 |
| `net/minecraft/world/level/block/Blocks#CHERRY_SAPLING` | field | 1 |
| `net/minecraft/world/level/block/Blocks#CHORUS_FLOWER` | field | 1 |
| `net/minecraft/world/level/block/Blocks#CHORUS_PLANT` | field | 5 |
| `net/minecraft/world/level/block/Blocks#CINNABAR` | field | 3 |
| `net/minecraft/world/level/block/Blocks#CLOSED_EYEBLOSSOM` | field | 1 |
| `net/minecraft/world/level/block/Blocks#COAL_BLOCK` | field | 1 |
| `net/minecraft/world/level/block/Blocks#COARSE_DIRT` | field | 2 |
| `net/minecraft/world/level/block/Blocks#COBWEB` | field | 1 |
| `net/minecraft/world/level/block/Blocks#CONCRETE` | field | 1 |
| `net/minecraft/world/level/block/Blocks#CORNFLOWER` | field | 1 |
| `net/minecraft/world/level/block/Blocks#CRIMSON_FUNGUS` | field | 1 |
| `net/minecraft/world/level/block/Blocks#CRIMSON_ROOTS` | field | 1 |
| `net/minecraft/world/level/block/Blocks#CRYING_OBSIDIAN` | field | 2 |
| `net/minecraft/world/level/block/Blocks#DANDELION` | field | 1 |
| `net/minecraft/world/level/block/Blocks#DARK_OAK_SAPLING` | field | 1 |
| `net/minecraft/world/level/block/Blocks#DEAD_BUSH` | field | 1 |
| `net/minecraft/world/level/block/Blocks#DIORITE` | field | 1 |
| `net/minecraft/world/level/block/Blocks#DIORITE_SLAB` | field | 1 |
| `net/minecraft/world/level/block/Blocks#DIRT` | field | 3 |
| `net/minecraft/world/level/block/Blocks#ENDER_CHEST` | field | 1 |
| `net/minecraft/world/level/block/Blocks#END_ROD` | field | 1 |
| `net/minecraft/world/level/block/Blocks#END_STONE` | field | 35 |
| `net/minecraft/world/level/block/Blocks#END_STONE_BRICKS` | field | 5 |
| `net/minecraft/world/level/block/Blocks#END_STONE_BRICK_SLAB` | field | 1 |
| `net/minecraft/world/level/block/Blocks#END_STONE_BRICK_WALL` | field | 1 |
| `net/minecraft/world/level/block/Blocks#FERN` | field | 1 |
| `net/minecraft/world/level/block/Blocks#FLOWERING_AZALEA` | field | 1 |
| `net/minecraft/world/level/block/Blocks#GLASS` | field | 2 |
| `net/minecraft/world/level/block/Blocks#GOLD_BLOCK` | field | 1 |
| `net/minecraft/world/level/block/Blocks#GRANITE` | field | 1 |
| `net/minecraft/world/level/block/Blocks#GRANITE_SLAB` | field | 1 |
| `net/minecraft/world/level/block/Blocks#GRASS_BLOCK` | field | 3 |
| `net/minecraft/world/level/block/Blocks#GRAVEL` | field | 1 |
| `net/minecraft/world/level/block/Blocks#ICE` | field | 2 |
| `net/minecraft/world/level/block/Blocks#IRON_BARS` | field | 1 |
| `net/minecraft/world/level/block/Blocks#IRON_BLOCK` | field | 2 |
| `net/minecraft/world/level/block/Blocks#IRON_CHAIN` | field | 1 |
| `net/minecraft/world/level/block/Blocks#JUNGLE_SAPLING` | field | 1 |
| `net/minecraft/world/level/block/Blocks#LADDER` | field | 1 |
| `net/minecraft/world/level/block/Blocks#LAPIS_BLOCK` | field | 1 |
| `net/minecraft/world/level/block/Blocks#LILY_OF_THE_VALLEY` | field | 1 |
| `net/minecraft/world/level/block/Blocks#LILY_PAD` | field | 1 |
| `net/minecraft/world/level/block/Blocks#MANGROVE_PROPAGULE` | field | 1 |
| `net/minecraft/world/level/block/Blocks#MOSS_BLOCK` | field | 2 |
| `net/minecraft/world/level/block/Blocks#MUD` | field | 2 |
| `net/minecraft/world/level/block/Blocks#MYCELIUM` | field | 2 |
| `net/minecraft/world/level/block/Blocks#NETHERRACK` | field | 1 |
| `net/minecraft/world/level/block/Blocks#NETHER_PORTAL` | field | 1 |
| `net/minecraft/world/level/block/Blocks#OAK_SAPLING` | field | 1 |
| `net/minecraft/world/level/block/Blocks#OBSIDIAN` | field | 14 |
| `net/minecraft/world/level/block/Blocks#OPEN_EYEBLOSSOM` | field | 1 |
| `net/minecraft/world/level/block/Blocks#ORANGE_TULIP` | field | 1 |
| `net/minecraft/world/level/block/Blocks#OXEYE_DAISY` | field | 1 |
| `net/minecraft/world/level/block/Blocks#PACKED_ICE` | field | 1 |
| `net/minecraft/world/level/block/Blocks#PALE_OAK_SAPLING` | field | 1 |
| `net/minecraft/world/level/block/Blocks#PINK_TULIP` | field | 1 |
| `net/minecraft/world/level/block/Blocks#PISTON` | field | 1 |
| `net/minecraft/world/level/block/Blocks#PODZOL` | field | 2 |
| `net/minecraft/world/level/block/Blocks#POLISHED_ANDESITE` | field | 1 |
| `net/minecraft/world/level/block/Blocks#POLISHED_ANDESITE_SLAB` | field | 1 |
| `net/minecraft/world/level/block/Blocks#POLISHED_DIORITE` | field | 1 |
| `net/minecraft/world/level/block/Blocks#POLISHED_DIORITE_SLAB` | field | 1 |
| `net/minecraft/world/level/block/Blocks#POLISHED_GRANITE` | field | 1 |
| `net/minecraft/world/level/block/Blocks#POLISHED_GRANITE_SLAB` | field | 1 |
| `net/minecraft/world/level/block/Blocks#POPPY` | field | 1 |
| `net/minecraft/world/level/block/Blocks#PUMPKIN` | field | 1 |
| `net/minecraft/world/level/block/Blocks#PURPUR_BLOCK` | field | 4 |
| `net/minecraft/world/level/block/Blocks#PURPUR_PILLAR` | field | 2 |
| `net/minecraft/world/level/block/Blocks#PURPUR_SLAB` | field | 2 |
| `net/minecraft/world/level/block/Blocks#PURPUR_STAIRS` | field | 1 |
| `net/minecraft/world/level/block/Blocks#QUARTZ_BLOCK` | field | 2 |
| `net/minecraft/world/level/block/Blocks#QUARTZ_PILLAR` | field | 1 |
| `net/minecraft/world/level/block/Blocks#QUARTZ_SLAB` | field | 1 |
| `net/minecraft/world/level/block/Blocks#RED_MUSHROOM` | field | 1 |
| `net/minecraft/world/level/block/Blocks#RED_TULIP` | field | 1 |
| `net/minecraft/world/level/block/Blocks#ROOTED_DIRT` | field | 2 |
| `net/minecraft/world/level/block/Blocks#SAND` | field | 1 |
| `net/minecraft/world/level/block/Blocks#SNOW_BLOCK` | field | 1 |
| `net/minecraft/world/level/block/Blocks#SOUL_SAND` | field | 1 |
| `net/minecraft/world/level/block/Blocks#SOUL_SOIL` | field | 1 |
| `net/minecraft/world/level/block/Blocks#SPONGE` | field | 1 |
| `net/minecraft/world/level/block/Blocks#SPRUCE_SAPLING` | field | 1 |
| `net/minecraft/world/level/block/Blocks#STAINED_GLASS` | field | 1 |
| `net/minecraft/world/level/block/Blocks#STONE` | field | 1 |
| `net/minecraft/world/level/block/Blocks#STRUCTURE_BLOCK` | field | 1 |
| `net/minecraft/world/level/block/Blocks#STRUCTURE_VOID` | field | 1 |
| `net/minecraft/world/level/block/Blocks#SULFUR` | field | 3 |
| `net/minecraft/world/level/block/Blocks#SULFUR_SPIKE` | field | 3 |
| `net/minecraft/world/level/block/Blocks#SUNFLOWER` | field | 1 |
| `net/minecraft/world/level/block/Blocks#TINTED_GLASS` | field | 1 |
| `net/minecraft/world/level/block/Blocks#TORCHFLOWER` | field | 1 |
| `net/minecraft/world/level/block/Blocks#WARPED_FUNGUS` | field | 1 |
| `net/minecraft/world/level/block/Blocks#WARPED_ROOTS` | field | 1 |
| `net/minecraft/world/level/block/Blocks#WATER` | field | 23 |
| `net/minecraft/world/level/block/Blocks#WET_SPONGE` | field | 2 |
| `net/minecraft/world/level/block/Blocks#WHITE_TULIP` | field | 1 |
| `net/minecraft/world/level/block/Blocks#WITHER_ROSE` | field | 1 |
| `net/minecraft/world/level/block/Blocks#WOOL` | field | 2 |
| `net/minecraft/world/level/block/ChorusFlowerBlock#AGE` | field | 1 |
| `net/minecraft/world/level/block/ChorusFlowerBlock#canSurvive` | method | 1 |
| `net/minecraft/world/level/block/ChorusFlowerBlock#generatePlant` | method | 1 |
| `net/minecraft/world/level/block/ChorusFlowerBlock#placeDeadFlower` | method | 1 |
| `net/minecraft/world/level/block/ChorusFlowerBlock#randomTick` | method | 1 |
| `net/minecraft/world/level/block/ChorusPlantBlock#DOWN` | field | 1 |
| `net/minecraft/world/level/block/ChorusPlantBlock#UP` | field | 1 |
| `net/minecraft/world/level/block/ChorusPlantBlock#canSurvive` | method | 1 |
| `net/minecraft/world/level/block/ChorusPlantBlock#getStateForPlacement` | method | 1 |
| `net/minecraft/world/level/block/ChorusPlantBlock#getStateWithConnections` | method | 1 |
| `net/minecraft/world/level/block/ChorusPlantBlock#updateShape` | method | 1 |
| `net/minecraft/world/level/block/ColorCollection#black` | method | 2 |
| `net/minecraft/world/level/block/ColorCollection#blue` | method | 1 |
| `net/minecraft/world/level/block/ColorCollection#brown` | method | 1 |
| `net/minecraft/world/level/block/ColorCollection#cyan` | method | 2 |
| `net/minecraft/world/level/block/ColorCollection#gray` | method | 2 |
| `net/minecraft/world/level/block/ColorCollection#green` | method | 1 |
| `net/minecraft/world/level/block/ColorCollection#lightBlue` | method | 3 |
| `net/minecraft/world/level/block/ColorCollection#magenta` | method | 2 |
| `net/minecraft/world/level/block/ColorCollection#orange` | method | 1 |
| `net/minecraft/world/level/block/ColorCollection#pick` | method | 1 |
| `net/minecraft/world/level/block/ColorCollection#pink` | method | 2 |
| `net/minecraft/world/level/block/ColorCollection#purple` | method | 2 |
| `net/minecraft/world/level/block/ColorCollection#red` | method | 2 |
| `net/minecraft/world/level/block/ColorCollection#white` | method | 3 |
| `net/minecraft/world/level/block/ColorCollection#yellow` | method | 2 |
| `net/minecraft/world/level/block/DropExperienceBlock#<init>` | method | 1 |
| `net/minecraft/world/level/block/EndPortalBlock#getPortalDestination` | method | 1 |
| `net/minecraft/world/level/block/FallingBlock#<init>` | method | 1 |
| `net/minecraft/world/level/block/HalfTransparentBlock#<init>` | method | 1 |
| `net/minecraft/world/level/block/HalfTransparentBlock#playerDestroy` | method | 1 |
| `net/minecraft/world/level/block/HorizontalDirectionalBlock#FACING` | field | 3 |
| `net/minecraft/world/level/block/IronBarsBlock#EAST` | field | 1 |
| `net/minecraft/world/level/block/IronBarsBlock#NORTH` | field | 1 |
| `net/minecraft/world/level/block/IronBarsBlock#SOUTH` | field | 1 |
| `net/minecraft/world/level/block/IronBarsBlock#WEST` | field | 1 |
| `net/minecraft/world/level/block/LeavesBlock#DISTANCE` | field | 8 |
| `net/minecraft/world/level/block/LeavesBlock#PERSISTENT` | field | 1 |
| `net/minecraft/world/level/block/Mirror#getRotation` | method | 1 |
| `net/minecraft/world/level/block/Mirror#ordinal` | method | 1 |
| `net/minecraft/world/level/block/Mirror#values` | method | 1 |
| `net/minecraft/world/level/block/MultifaceSpreadeableBlock#<init>` | method | 1 |
| `net/minecraft/world/level/block/MultifaceSpreadeableBlock#canBeReplaced` | method | 1 |
| `net/minecraft/world/level/block/MultifaceSpreadeableBlock#getFluidState` | method | 1 |
| `net/minecraft/world/level/block/MultifaceSpreadeableBlock#updateShape` | method | 1 |
| `net/minecraft/world/level/block/MultifaceSpreader#<init>` | method | 1 |
| `net/minecraft/world/level/block/MultifaceSpreader#canSpreadInAnyDirection` | method | 1 |
| `net/minecraft/world/level/block/MultifaceSpreader#spreadFromRandomFaceTowardRandomDirection` | method | 1 |
| `net/minecraft/world/level/block/NetherPortalBlock#<init>` | method | 1 |
| `net/minecraft/world/level/block/NetherPortalBlock#AXIS` | field | 1 |
| `net/minecraft/world/level/block/NetherPortalBlock#createBlockStateDefinition` | method | 1 |
| `net/minecraft/world/level/block/PathBlock#<init>` | method | 1 |
| `net/minecraft/world/level/block/PathBlock#getStateForPlacement` | method | 1 |
| `net/minecraft/world/level/block/PipeBlock#DOWN` | field | 1 |
| `net/minecraft/world/level/block/RenderShape#INVISIBLE` | field | 1 |
| `net/minecraft/world/level/block/RenderShape#MODEL` | field | 1 |
| `net/minecraft/world/level/block/RotatedPillarBlock#<init>` | method | 4 |
| `net/minecraft/world/level/block/RotatedPillarBlock#asItem` | method | 1 |
| `net/minecraft/world/level/block/RotatedPillarBlock#defaultBlockState` | method | 1 |
| `net/minecraft/world/level/block/Rotation#CLOCKWISE_180` | field | 1 |
| `net/minecraft/world/level/block/Rotation#CLOCKWISE_90` | field | 1 |
| `net/minecraft/world/level/block/Rotation#COUNTERCLOCKWISE_90` | field | 1 |
| `net/minecraft/world/level/block/Rotation#ordinal` | method | 1 |
| `net/minecraft/world/level/block/Rotation#rotate` | method | 1 |
| `net/minecraft/world/level/block/Rotation#values` | method | 1 |
| `net/minecraft/world/level/block/SlimeBlock#<init>` | method | 2 |
| `net/minecraft/world/level/block/SlimeBlock#skipRendering` | method | 1 |
| `net/minecraft/world/level/block/SoundType#AMETHYST` | field | 1 |
| `net/minecraft/world/level/block/SoundType#AMETHYST_CLUSTER` | field | 1 |
| `net/minecraft/world/level/block/SoundType#ANVIL` | field | 1 |
| `net/minecraft/world/level/block/SoundType#BAMBOO` | field | 1 |
| `net/minecraft/world/level/block/SoundType#CORAL_BLOCK` | field | 1 |
| `net/minecraft/world/level/block/SoundType#GLASS` | field | 2 |
| `net/minecraft/world/level/block/SoundType#IRON` | field | 1 |
| `net/minecraft/world/level/block/SoundType#LANTERN` | field | 1 |
| `net/minecraft/world/level/block/SoundType#NETHERITE_BLOCK` | field | 1 |
| `net/minecraft/world/level/block/SoundType#NETHER_WART` | field | 1 |
| `net/minecraft/world/level/block/SoundType#WART_BLOCK` | field | 1 |
| `net/minecraft/world/level/block/SoundType#WET_GRASS` | field | 3 |
| `net/minecraft/world/level/block/SpeleothemBlock#THICKNESS` | field | 1 |
| `net/minecraft/world/level/block/SpeleothemBlock#TIP_DIRECTION` | field | 1 |
| `net/minecraft/world/level/block/TintedParticleLeavesBlock#defaultBlockState` | method | 7 |
| `net/minecraft/world/level/block/TransparentBlock#<init>` | method | 1 |
| `net/minecraft/world/level/block/entity/BaseContainerBlockEntity#<init>` | method | 1 |
| `net/minecraft/world/level/block/entity/BaseContainerBlockEntity#loadAdditional` | method | 1 |
| `net/minecraft/world/level/block/entity/BaseContainerBlockEntity#preRemoveSideEffects` | method | 1 |
| `net/minecraft/world/level/block/entity/BaseContainerBlockEntity#saveAdditional` | method | 1 |
| `net/minecraft/world/level/block/entity/BlockEntity#<init>` | method | 3 |
| `net/minecraft/world/level/block/entity/BlockEntity#getLevel` | method | 1 |
| `net/minecraft/world/level/block/entity/BlockEntity#loadAdditional` | method | 2 |
| `net/minecraft/world/level/block/entity/BlockEntity#saveAdditional` | method | 2 |
| `net/minecraft/world/level/block/entity/BlockEntity#setChanged` | method | 2 |
| `net/minecraft/world/level/block/entity/BlockEntityType#addValidBlock` | method | 1 |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase#canBeReplaced` | method | 1 |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase#isAir` | method | 1 |
| `net/minecraft/world/level/block/state/BlockBehaviour$OffsetType#NONE` | field | 3 |
| `net/minecraft/world/level/block/state/BlockBehaviour$OffsetType#XYZ` | field | 1 |
| `net/minecraft/world/level/block/state/BlockBehaviour$OffsetType#XZ` | field | 1 |
| `net/minecraft/world/level/block/state/BlockBehaviour$Properties#lightLevel` | method | 2 |
| `net/minecraft/world/level/block/state/BlockBehaviour$Properties#noCollision` | method | 1 |
| `net/minecraft/world/level/block/state/BlockBehaviour$Properties#noOcclusion` | method | 1 |
| `net/minecraft/world/level/block/state/BlockBehaviour$Properties#ofFullCopy` | method | 1 |
| `net/minecraft/world/level/block/state/BlockBehaviour$Properties#requiresCorrectToolForDrops` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#CODEC` | field | 1 |
| `net/minecraft/world/level/block/state/BlockState#equals` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#getBlock` | method | 45 |
| `net/minecraft/world/level/block/state/BlockState#getDestroySpeed` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#getFluidState` | method | 25 |
| `net/minecraft/world/level/block/state/BlockState#getLightDampening` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#getLightEmission` | method | 2 |
| `net/minecraft/world/level/block/state/BlockState#getOptionalValue` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#getValue` | method | 58 |
| `net/minecraft/world/level/block/state/BlockState#hasBlockEntity` | method | 2 |
| `net/minecraft/world/level/block/state/BlockState#hasOffsetFunction` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#hasProperty` | method | 6 |
| `net/minecraft/world/level/block/state/BlockState#is` | method | 125 |
| `net/minecraft/world/level/block/state/BlockState#isAir` | method | 48 |
| `net/minecraft/world/level/block/state/BlockState#isCollisionShapeFullBlock` | method | 3 |
| `net/minecraft/world/level/block/state/BlockState#isFaceSturdy` | method | 6 |
| `net/minecraft/world/level/block/state/BlockState#isRandomlyTicking` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#isRedstoneConductor` | method | 2 |
| `net/minecraft/world/level/block/state/BlockState#isSolid` | method | 7 |
| `net/minecraft/world/level/block/state/BlockState#isSolidRender` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#liquid` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#rotate` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#setValue` | method | 94 |
| `net/minecraft/world/level/block/state/BlockState#updateNeighbourShapes` | method | 1 |
| `net/minecraft/world/level/block/state/StateDefinition#any` | method | 12 |
| `net/minecraft/world/level/block/state/StateDefinition#getPossibleStates` | method | 1 |
| `net/minecraft/world/level/block/state/StateDefinition#getProperty` | method | 1 |
| `net/minecraft/world/level/block/state/StateDefinition$Builder#add` | method | 35 |
| `net/minecraft/world/level/block/state/properties/BlockSetType#OAK` | field | 1 |
| `net/minecraft/world/level/block/state/properties/BlockStateProperties#AGE_5` | field | 1 |
| `net/minecraft/world/level/block/state/properties/BlockStateProperties#DOWN` | field | 1 |
| `net/minecraft/world/level/block/state/properties/BlockStateProperties#FACING` | field | 7 |
| `net/minecraft/world/level/block/state/properties/BlockStateProperties#HORIZONTAL_AXIS` | field | 2 |
| `net/minecraft/world/level/block/state/properties/BlockStateProperties#HORIZONTAL_FACING` | field | 5 |
| `net/minecraft/world/level/block/state/properties/BlockStateProperties#LIT` | field | 2 |
| `net/minecraft/world/level/block/state/properties/BlockStateProperties#WATERLOGGED` | field | 12 |
| `net/minecraft/world/level/block/state/properties/BooleanProperty#create` | method | 2 |
| `net/minecraft/world/level/block/state/properties/EnumProperty#create` | method | 2 |
| `net/minecraft/world/level/block/state/properties/IntegerProperty#create` | method | 2 |
| `net/minecraft/world/level/block/state/properties/IntegerProperty#getPossibleValues` | method | 1 |
| `net/minecraft/world/level/block/state/properties/NoteBlockInstrument#BASEDRUM` | field | 2 |
| `net/minecraft/world/level/block/state/properties/NoteBlockInstrument#HARP` | field | 2 |
| `net/minecraft/world/level/block/state/properties/SpeleothemThickness#BASE` | field | 1 |
| `net/minecraft/world/level/block/state/properties/SpeleothemThickness#MIDDLE` | field | 1 |
| `net/minecraft/world/level/block/state/properties/SpeleothemThickness#TIP` | field | 1 |
| `net/minecraft/world/level/block/state/properties/WoodType#OAK` | field | 1 |
| `net/minecraft/world/level/border/WorldBorder#clampToBounds` | method | 2 |
| `net/minecraft/world/level/border/WorldBorder#isWithinBounds` | method | 1 |
| `net/minecraft/world/level/chunk/CarverOutput#carve` | method | 2 |
| `net/minecraft/world/level/chunk/CarverOutput#maxY` | method | 2 |
| `net/minecraft/world/level/chunk/CarverOutput#minY` | method | 2 |
| `net/minecraft/world/level/chunk/ChunkAccess#getBlockState` | method | 8 |
| `net/minecraft/world/level/chunk/ChunkAccess#getFluidState` | method | 4 |
| `net/minecraft/world/level/chunk/ChunkAccess#getHeight` | method | 3 |
| `net/minecraft/world/level/chunk/ChunkAccess#getMaxY` | method | 1 |
| `net/minecraft/world/level/chunk/ChunkAccess#getMinY` | method | 6 |
| `net/minecraft/world/level/chunk/ChunkAccess#getNoiseBiome` | method | 2 |
| `net/minecraft/world/level/chunk/ChunkAccess#getOrCreateHeightmapUnprimed` | method | 2 |
| `net/minecraft/world/level/chunk/ChunkAccess#getPos` | method | 3 |
| `net/minecraft/world/level/chunk/ChunkAccess#hasPrimedHeightmap` | method | 1 |
| `net/minecraft/world/level/chunk/ChunkAccess#markPosForPostProcessing` | method | 2 |
| `net/minecraft/world/level/chunk/ChunkAccess#removeBlockEntity` | method | 1 |
| `net/minecraft/world/level/chunk/ChunkAccess#setBlockState` | method | 1 |
| `net/minecraft/world/level/chunk/ChunkGenerator#getBiomeSource` | method | 2 |
| `net/minecraft/world/level/chunk/ChunkGenerator#getFirstOccupiedHeight` | method | 1 |
| `net/minecraft/world/level/chunk/LevelChunk#getBlockEntity` | method | 1 |
| `net/minecraft/world/level/chunk/LevelChunk$EntityCreationType#CHECK` | field | 1 |
| `net/minecraft/world/level/chunk/status/ChunkStatus#BIOMES` | field | 1 |
| `net/minecraft/world/level/dimension/BuiltinDimensionTypes#END` | field | 1 |
| `net/minecraft/world/level/dimension/BuiltinDimensionTypes#NETHER` | field | 1 |
| `net/minecraft/world/level/dimension/BuiltinDimensionTypes#OVERWORLD` | field | 1 |
| `net/minecraft/world/level/dimension/BuiltinDimensionTypes#OVERWORLD_CAVES` | field | 1 |
| `net/minecraft/world/level/dimension/DimensionType#getTeleportationScale` | method | 2 |
| `net/minecraft/world/level/dimension/DimensionType#hasEnderDragonFight` | method | 2 |
| `net/minecraft/world/level/dimension/LevelStem#generator` | method | 1 |
| `net/minecraft/world/level/gamerules/GameRules#BLOCK_DROPS` | field | 1 |
| `net/minecraft/world/level/gamerules/GameRules#get` | method | 1 |
| `net/minecraft/world/level/levelgen/GenerationStep$Decoration#LAKES` | field | 3 |
| `net/minecraft/world/level/levelgen/GenerationStep$Decoration#RAW_GENERATION` | field | 2 |
| `net/minecraft/world/level/levelgen/GenerationStep$Decoration#SURFACE_STRUCTURES` | field | 4 |
| `net/minecraft/world/level/levelgen/GenerationStep$Decoration#TOP_LAYER_MODIFICATION` | field | 1 |
| `net/minecraft/world/level/levelgen/GenerationStep$Decoration#UNDERGROUND_DECORATION` | field | 2 |
| `net/minecraft/world/level/levelgen/GenerationStep$Decoration#UNDERGROUND_ORES` | field | 1 |
| `net/minecraft/world/level/levelgen/GenerationStep$Decoration#VEGETAL_DECORATION` | field | 3 |
| `net/minecraft/world/level/levelgen/Heightmap#getFirstAvailable` | method | 2 |
| `net/minecraft/world/level/levelgen/Heightmap#primeHeightmaps` | method | 1 |
| `net/minecraft/world/level/levelgen/Heightmap$Types#MOTION_BLOCKING` | field | 2 |
| `net/minecraft/world/level/levelgen/Heightmap$Types#MOTION_BLOCKING_NO_LEAVES` | field | 1 |
| `net/minecraft/world/level/levelgen/Heightmap$Types#OCEAN_FLOOR_WG` | field | 3 |
| `net/minecraft/world/level/levelgen/Heightmap$Types#WORLD_SURFACE` | field | 8 |
| `net/minecraft/world/level/levelgen/Heightmap$Types#WORLD_SURFACE_WG` | field | 15 |
| `net/minecraft/world/level/levelgen/LegacyRandomSource#<init>` | method | 5 |
| `net/minecraft/world/level/levelgen/NoiseBasedChunkGenerator#doFill` | method | 1 |
| `net/minecraft/world/level/levelgen/NoiseBasedChunkGenerator#getBaseColumn` | method | 1 |
| `net/minecraft/world/level/levelgen/NoiseBasedChunkGenerator#getBaseHeight` | method | 1 |
| `net/minecraft/world/level/levelgen/NoiseBasedChunkGenerator#iterateNoiseColumn` | method | 1 |
| `net/minecraft/world/level/levelgen/NoiseBasedChunkGenerator#settings` | method | 1 |
| `net/minecraft/world/level/levelgen/NoiseGeneratorSettings#defaultBlock` | method | 1 |
| `net/minecraft/world/level/levelgen/NoiseGeneratorSettings#noiseRouter` | method | 1 |
| `net/minecraft/world/level/levelgen/NoiseGeneratorSettings#noiseSettings` | method | 4 |
| `net/minecraft/world/level/levelgen/NoiseRouter#finalDensity` | method | 1 |
| `net/minecraft/world/level/levelgen/NoiseSettings#equals` | method | 1 |
| `net/minecraft/world/level/levelgen/NoiseSettings#height` | method | 3 |
| `net/minecraft/world/level/levelgen/NoiseSettings#minY` | method | 2 |
| `net/minecraft/world/level/levelgen/Noises#CLAY_BANDS_OFFSET` | field | 1 |
| `net/minecraft/world/level/levelgen/RandomState#createClimateSampler` | method | 1 |
| `net/minecraft/world/level/levelgen/RandomState#getOrCreateNoise` | method | 1 |
| `net/minecraft/world/level/levelgen/VerticalAnchor#absolute` | method | 3 |
| `net/minecraft/world/level/levelgen/WorldGenerationContext#getGenDepth` | method | 2 |
| `net/minecraft/world/level/levelgen/WorldGenerationContext#getMinGenY` | method | 2 |
| `net/minecraft/world/level/levelgen/WorldgenRandom#nextInt` | method | 1 |
| `net/minecraft/world/level/levelgen/blockpredicates/BlockPredicate#CODEC` | field | 1 |
| `net/minecraft/world/level/levelgen/blockpredicates/BlockPredicate#matchesBlocks` | method | 5 |
| `net/minecraft/world/level/levelgen/blockpredicates/BlockPredicate#matchesTag` | method | 6 |
| `net/minecraft/world/level/levelgen/blockpredicates/BlockPredicate#test` | method | 1 |
| `net/minecraft/world/level/levelgen/densityfunction/DensityBuffer#set` | method | 1 |
| `net/minecraft/world/level/levelgen/densityfunction/DensityFunction#rewriteChildren` | method | 1 |
| `net/minecraft/world/level/levelgen/densityfunction/DensityVolume#blockX` | method | 1 |
| `net/minecraft/world/level/levelgen/densityfunction/DensityVolume#blockY` | method | 1 |
| `net/minecraft/world/level/levelgen/densityfunction/DensityVolume#blockZ` | method | 1 |
| `net/minecraft/world/level/levelgen/densityfunction/DensityVolume#indexUnchecked` | method | 1 |
| `net/minecraft/world/level/levelgen/densityfunction/DensityVolume#sizeX` | method | 1 |
| `net/minecraft/world/level/levelgen/densityfunction/DensityVolume#sizeY` | method | 1 |
| `net/minecraft/world/level/levelgen/densityfunction/DensityVolume#sizeZ` | method | 1 |
| `net/minecraft/world/level/levelgen/densityfunction/SamplerContext#EMPTY_UNCACHED` | field | 1 |
| `net/minecraft/world/level/levelgen/densityfunction/op/InterpolatedFunction#<init>` | method | 1 |
| `net/minecraft/world/level/levelgen/densityfunction/op/InterpolatedFunction#cellSizeXz` | method | 1 |
| `net/minecraft/world/level/levelgen/densityfunction/op/InterpolatedFunction#cellSizeY` | method | 1 |
| `net/minecraft/world/level/levelgen/feature/ChorusPlantFeature#place` | method | 1 |
| `net/minecraft/world/level/levelgen/feature/EndPlatformFeature#createEndPlatform` | method | 1 |
| `net/minecraft/world/level/levelgen/feature/EndPodiumFeature#place` | method | 2 |
| `net/minecraft/world/level/levelgen/feature/EndSpikeFeature#crystalBeamTarget` | method | 1 |
| `net/minecraft/world/level/levelgen/feature/EndSpikeFeature#crystalInvulnerable` | method | 1 |
| `net/minecraft/world/level/levelgen/feature/EndSpikeFeature#place` | method | 1 |
| `net/minecraft/world/level/levelgen/feature/EndSpikeFeature#placeSpike` | method | 1 |
| `net/minecraft/world/level/levelgen/feature/EndSpikeFeature$EndSpike#getCenterX` | method | 1 |
| `net/minecraft/world/level/levelgen/feature/EndSpikeFeature$EndSpike#getCenterZ` | method | 1 |
| `net/minecraft/world/level/levelgen/feature/EndSpikeFeature$EndSpike#getHeight` | method | 2 |
| `net/minecraft/world/level/levelgen/feature/EndSpikeFeature$EndSpike#getRadius` | method | 1 |
| `net/minecraft/world/level/levelgen/feature/EndSpikeFeature$EndSpike#isGuarded` | method | 1 |
| `net/minecraft/world/level/levelgen/feature/Feature#CODEC` | field | 1 |
| `net/minecraft/world/level/levelgen/feature/Feature#place` | method | 1 |
| `net/minecraft/world/level/levelgen/feature/MultifaceGrowthFeature#<init>` | method | 1 |
| `net/minecraft/world/level/levelgen/feature/SpeleothemFeature#<init>` | method | 1 |
| `net/minecraft/world/level/levelgen/feature/VegetationPatchFeature#<init>` | method | 1 |
| `net/minecraft/world/level/levelgen/feature/stateproviders/BlockStateProvider#DIRECT_CODEC` | field | 10 |
| `net/minecraft/world/level/levelgen/feature/stateproviders/BlockStateProvider#getState` | method | 11 |
| `net/minecraft/world/level/levelgen/feature/stateproviders/BlockStateProvider#holderOf` | method | 1 |
| `net/minecraft/world/level/levelgen/feature/stateproviders/BlockStateProvider#of` | method | 10 |
| `net/minecraft/world/level/levelgen/feature/stateproviders/WeightedStateProvider#<init>` | method | 1 |
| `net/minecraft/world/level/levelgen/heightproviders/ConstantHeight#of` | method | 2 |
| `net/minecraft/world/level/levelgen/heightproviders/HeightProvider#CODEC` | field | 1 |
| `net/minecraft/world/level/levelgen/heightproviders/HeightProvider#sample` | method | 1 |
| `net/minecraft/world/level/levelgen/heightproviders/UniformHeight#of` | method | 2 |
| `net/minecraft/world/level/levelgen/material/MaterialRules#ifTrue` | method | 5 |
| `net/minecraft/world/level/levelgen/material/MaterialRules#state` | method | 7 |
| `net/minecraft/world/level/levelgen/material/MaterialRules#stoneDepthCheck` | method | 4 |
| `net/minecraft/world/level/levelgen/placement/CaveSurface#CEILING` | field | 3 |
| `net/minecraft/world/level/levelgen/placement/CaveSurface#FLOOR` | field | 5 |
| `net/minecraft/world/level/levelgen/placement/CaveSurface#getDirection` | method | 1 |
| `net/minecraft/world/level/levelgen/placement/HeightRangePlacement#of` | method | 1 |
| `net/minecraft/world/level/levelgen/placement/HeightRangePlacement#uniform` | method | 1 |
| `net/minecraft/world/level/levelgen/placement/SurfaceRelativeThresholdFilter#of` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/BoundingBox#<init>` | method | 7 |
| `net/minecraft/world/level/levelgen/structure/BoundingBox#fromCorners` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/BoundingBox#isInside` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/BoundingBox#maxX` | method | 3 |
| `net/minecraft/world/level/levelgen/structure/BoundingBox#maxY` | method | 3 |
| `net/minecraft/world/level/levelgen/structure/BoundingBox#maxZ` | method | 3 |
| `net/minecraft/world/level/levelgen/structure/BoundingBox#minX` | method | 3 |
| `net/minecraft/world/level/levelgen/structure/BoundingBox#minY` | method | 2 |
| `net/minecraft/world/level/levelgen/structure/BoundingBox#minZ` | method | 3 |
| `net/minecraft/world/level/levelgen/structure/Structure#<init>` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/Structure$GenerationContext#chunkGenerator` | method | 12 |
| `net/minecraft/world/level/levelgen/structure/Structure$GenerationContext#chunkPos` | method | 12 |
| `net/minecraft/world/level/levelgen/structure/Structure$GenerationContext#heightAccessor` | method | 11 |
| `net/minecraft/world/level/levelgen/structure/Structure$GenerationContext#random` | method | 11 |
| `net/minecraft/world/level/levelgen/structure/Structure$GenerationContext#randomState` | method | 11 |
| `net/minecraft/world/level/levelgen/structure/Structure$GenerationContext#registryAccess` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/Structure$GenerationContext#validBiome` | method | 3 |
| `net/minecraft/world/level/levelgen/structure/Structure$GenerationStub#<init>` | method | 2 |
| `net/minecraft/world/level/levelgen/structure/StructurePiece#<init>` | method | 2 |
| `net/minecraft/world/level/levelgen/structure/TerrainAdjustment#BEARD_THIN` | field | 2 |
| `net/minecraft/world/level/levelgen/structure/TerrainAdjustment#NONE` | field | 2 |
| `net/minecraft/world/level/levelgen/structure/pieces/StructurePiecesBuilder#addPiece` | method | 10 |
| `net/minecraft/world/level/levelgen/structure/pools/DimensionPadding#<init>` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/pools/StructureTemplatePool$Projection#RIGID` | field | 1 |
| `net/minecraft/world/level/levelgen/structure/pools/StructureTemplatePool$Projection#TERRAIN_MATCHING` | field | 1 |
| `net/minecraft/world/level/levelgen/structure/structures/EndCityStructure#findGenerationPoint` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/AlwaysTrueTest#INSTANCE` | field | 1 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/BlockIgnoreProcessor#<init>` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/BlockMatchTest#<init>` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/BlockRotProcessor#<init>` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/ProcessorRule#<init>` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/RandomBlockMatchTest#<init>` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/StructurePlaceSettings#<init>` | method | 3 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/StructurePlaceSettings#setBoundingBox` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/StructurePlaceSettings#setMirror` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/StructurePlaceSettings#setRotation` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/StructureTemplate#getBoundingBox` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/StructureTemplate#getSize` | method | 3 |
| `net/minecraft/world/level/levelgen/synth/Noise#get` | method | 1 |
| `net/minecraft/world/level/material/FlowingFluid#getPickupSound` | method | 1 |
| `net/minecraft/world/level/material/FlowingFluid#getSource` | method | 14 |
| `net/minecraft/world/level/material/FlowingFluid#getTickDelay` | method | 5 |
| `net/minecraft/world/level/material/Fluid#defaultFluidState` | method | 4 |
| `net/minecraft/world/level/material/FluidState#createLegacyBlock` | method | 1 |
| `net/minecraft/world/level/material/FluidState#getAmount` | method | 2 |
| `net/minecraft/world/level/material/FluidState#getType` | method | 17 |
| `net/minecraft/world/level/material/FluidState#is` | method | 2 |
| `net/minecraft/world/level/material/FluidState#isEmpty` | method | 25 |
| `net/minecraft/world/level/material/FluidState#isSource` | method | 3 |
| `net/minecraft/world/level/material/Fluids#EMPTY` | field | 4 |
| `net/minecraft/world/level/material/Fluids#WATER` | field | 22 |
| `net/minecraft/world/level/material/MapColor#COLOR_BLACK` | field | 9 |
| `net/minecraft/world/level/material/MapColor#COLOR_BLUE` | field | 6 |
| `net/minecraft/world/level/material/MapColor#COLOR_BROWN` | field | 3 |
| `net/minecraft/world/level/material/MapColor#COLOR_CYAN` | field | 7 |
| `net/minecraft/world/level/material/MapColor#COLOR_GRAY` | field | 3 |
| `net/minecraft/world/level/material/MapColor#COLOR_GREEN` | field | 6 |
| `net/minecraft/world/level/material/MapColor#COLOR_LIGHT_BLUE` | field | 9 |
| `net/minecraft/world/level/material/MapColor#COLOR_LIGHT_GRAY` | field | 1 |
| `net/minecraft/world/level/material/MapColor#COLOR_LIGHT_GREEN` | field | 1 |
| `net/minecraft/world/level/material/MapColor#COLOR_MAGENTA` | field | 6 |
| `net/minecraft/world/level/material/MapColor#COLOR_ORANGE` | field | 11 |
| `net/minecraft/world/level/material/MapColor#COLOR_PINK` | field | 7 |
| `net/minecraft/world/level/material/MapColor#COLOR_PURPLE` | field | 11 |
| `net/minecraft/world/level/material/MapColor#COLOR_RED` | field | 6 |
| `net/minecraft/world/level/material/MapColor#COLOR_YELLOW` | field | 5 |
| `net/minecraft/world/level/material/MapColor#DEEPSLATE` | field | 1 |
| `net/minecraft/world/level/material/MapColor#NONE` | field | 5 |
| `net/minecraft/world/level/material/MapColor#PLANT` | field | 1 |
| `net/minecraft/world/level/material/MapColor#PODZOL` | field | 1 |
| `net/minecraft/world/level/material/MapColor#SAND` | field | 1 |
| `net/minecraft/world/level/material/MapColor#SNOW` | field | 1 |
| `net/minecraft/world/level/material/MapColor#TERRACOTTA_BROWN` | field | 2 |
| `net/minecraft/world/level/material/MapColor#TERRACOTTA_ORANGE` | field | 2 |
| `net/minecraft/world/level/material/MapColor#WARPED_WART_BLOCK` | field | 2 |
| `net/minecraft/world/level/material/MapColor#WATER` | field | 1 |
| `net/minecraft/world/level/material/MapColor#WOOD` | field | 2 |
| `net/minecraft/world/level/material/MapColor#col` | field | 1 |
| `net/minecraft/world/level/material/PushReaction#IMMOVEABLE` | field | 2 |
| `net/minecraft/world/level/material/PushReaction#POPPED` | field | 4 |
| `net/minecraft/world/level/material/PushReaction#PUSH_PULL` | field | 1 |
| `net/minecraft/world/level/pathfinder/PathType#FIRE_IN_NEIGHBOR` | field | 2 |
| `net/minecraft/world/level/pathfinder/PathType#WATER` | field | 2 |
| `net/minecraft/world/level/portal/PortalShape#findCollisionFreePosition` | method | 1 |
| `net/minecraft/world/level/portal/TeleportTransition#<init>` | method | 2 |
| `net/minecraft/world/level/portal/TeleportTransition#PLAY_PORTAL_SOUND` | field | 1 |
| `net/minecraft/world/level/portal/TeleportTransition$PostTeleportTransition#then` | method | 1 |
| `net/minecraft/world/level/storage/LevelData$RespawnData#dimension` | method | 1 |
| `net/minecraft/world/level/storage/LevelData$RespawnData#of` | method | 2 |
| `net/minecraft/world/level/storage/LevelData$RespawnData#pos` | method | 1 |
| `net/minecraft/world/level/storage/LevelData$RespawnData#yaw` | method | 1 |
| `net/minecraft/world/level/storage/ValueInput#getIntOr` | method | 3 |
| `net/minecraft/world/level/storage/ValueInput#getShortOr` | method | 1 |
| `net/minecraft/world/level/storage/ValueInput#getStringOr` | method | 1 |
| `net/minecraft/world/level/storage/ValueInput#read` | method | 6 |
| `net/minecraft/world/level/storage/ValueOutput#putInt` | method | 3 |
| `net/minecraft/world/level/storage/ValueOutput#putShort` | method | 1 |
| `net/minecraft/world/level/storage/ValueOutput#putString` | method | 1 |
| `net/minecraft/world/level/storage/ValueOutput#store` | method | 6 |
| `net/minecraft/world/level/storage/loot/BuiltInLootTables#END_CITY_TREASURE` | field | 1 |
| `net/minecraft/world/level/storage/loot/BuiltInLootTables#FISHING` | field | 1 |
| `net/minecraft/world/level/storage/loot/LootContext$Builder#<init>` | method | 1 |
| `net/minecraft/world/level/storage/loot/LootContext$Builder#create` | method | 1 |
| `net/minecraft/world/level/storage/loot/LootParams$Builder#<init>` | method | 1 |
| `net/minecraft/world/level/storage/loot/LootParams$Builder#create` | method | 1 |
| `net/minecraft/world/level/storage/loot/LootParams$Builder#getOptionalParameter` | method | 4 |
| `net/minecraft/world/level/storage/loot/LootParams$Builder#withParameter` | method | 1 |
| `net/minecraft/world/level/storage/loot/LootPool#lootPool` | method | 13 |
| `net/minecraft/world/level/storage/loot/LootPool$Builder#add` | method | 13 |
| `net/minecraft/world/level/storage/loot/LootPool$Builder#setRolls` | method | 13 |
| `net/minecraft/world/level/storage/loot/LootPool$Builder#when` | method | 12 |
| `net/minecraft/world/level/storage/loot/LootTable#lootTable` | method | 13 |
| `net/minecraft/world/level/storage/loot/LootTable$Builder#withPool` | method | 12 |
| `net/minecraft/world/level/storage/loot/entries/EmptyLootItem#emptyItem` | method | 1 |
| `net/minecraft/world/level/storage/loot/entries/LootItem#lootTableItem` | method | 12 |
| `net/minecraft/world/level/storage/loot/entries/NestedLootTable#lootTableReference` | method | 1 |
| `net/minecraft/world/level/storage/loot/entries/UniformContainerBase$Builder#apply` | method | 8 |
| `net/minecraft/world/level/storage/loot/entries/UniformContainerBase$Builder#setQuality` | method | 1 |
| `net/minecraft/world/level/storage/loot/entries/UniformContainerBase$Builder#setWeight` | method | 2 |
| `net/minecraft/world/level/storage/loot/entries/UniformContainerBase$Builder#when` | method | 2 |
| `net/minecraft/world/level/storage/loot/functions/ApplyBonusCount#addOreBonusCount` | method | 1 |
| `net/minecraft/world/level/storage/loot/functions/ApplyExplosionDecay#explosionDecay` | method | 2 |
| `net/minecraft/world/level/storage/loot/functions/EnchantRandomlyFunction#randomEnchantment` | method | 1 |
| `net/minecraft/world/level/storage/loot/functions/EnchantRandomlyFunction$Builder#withEnchantment` | method | 1 |
| `net/minecraft/world/level/storage/loot/functions/EnchantWithLevelsFunction#enchantWithLevels` | method | 1 |
| `net/minecraft/world/level/storage/loot/functions/LootItemConditionalFunction$Builder#when` | method | 1 |
| `net/minecraft/world/level/storage/loot/functions/SetItemCountFunction#setCount` | method | 7 |
| `net/minecraft/world/level/storage/loot/functions/SetItemDamageFunction#setDamage` | method | 1 |
| `net/minecraft/world/level/storage/loot/parameters/LootContextParamSets#CHEST` | field | 1 |
| `net/minecraft/world/level/storage/loot/parameters/LootContextParamSets#CONTAINER_PROCESS` | field | 1 |
| `net/minecraft/world/level/storage/loot/parameters/LootContextParams#BLOCK_ENTITY` | field | 5 |
| `net/minecraft/world/level/storage/loot/parameters/LootContextParams#BLOCK_STATE` | field | 1 |
| `net/minecraft/world/level/storage/loot/parameters/LootContextParams#CONTAINER` | field | 1 |
| `net/minecraft/world/level/storage/loot/parameters/LootContextParams#ORIGIN` | field | 1 |
| `net/minecraft/world/level/storage/loot/predicates/AllOfCondition#allOf` | method | 1 |
| `net/minecraft/world/level/storage/loot/predicates/AnyOfCondition#anyOf` | method | 2 |
| `net/minecraft/world/level/storage/loot/predicates/AnyOfCondition$Builder#or` | method | 2 |
| `net/minecraft/world/level/storage/loot/predicates/BonusLevelTableCondition#bonusLevelFlatChance` | method | 1 |
| `net/minecraft/world/level/storage/loot/predicates/ExplosionCondition#survivesExplosion` | method | 1 |
| `net/minecraft/world/level/storage/loot/predicates/InvertedLootItemCondition#invert` | method | 2 |
| `net/minecraft/world/level/storage/loot/predicates/LocationCheck#checkLocation` | method | 2 |
| `net/minecraft/world/level/storage/loot/predicates/LootItemCondition$Builder#build` | method | 1 |
| `net/minecraft/world/level/storage/loot/predicates/LootItemCondition$Builder#invert` | method | 4 |
| `net/minecraft/world/level/storage/loot/predicates/LootItemCondition$Builder#or` | method | 2 |
| `net/minecraft/world/level/storage/loot/predicates/LootItemRandomChanceCondition#randomChance` | method | 4 |
| `net/minecraft/world/level/storage/loot/predicates/MatchBlock#blockMatches` | method | 9 |
| `net/minecraft/world/level/storage/loot/providers/number/floats/ContextFloatProviders#between` | method | 1 |
| `net/minecraft/world/level/storage/loot/providers/number/ints/ContextIntProviders#COOKING_TIME_BLAZE_ROD` | field | 1 |
| `net/minecraft/world/level/storage/loot/providers/number/ints/ContextIntProviders#COOKING_TIME_COAL_BLOCK` | field | 1 |
| `net/minecraft/world/level/storage/loot/providers/number/ints/ContextIntProviders#COOKING_TIME_DRIED_KELP_BLOCK` | field | 1 |
| `net/minecraft/world/level/storage/loot/providers/number/ints/ContextIntProviders#COOKING_TIME_LAVA_BUCKET` | field | 1 |
| `net/minecraft/world/level/storage/loot/providers/number/ints/ContextIntProviders#between` | method | 9 |
| `net/minecraft/world/level/storage/loot/providers/number/ints/ContextIntProviders#exactly` | method | 13 |
| `net/minecraft/world/level/storage/loot/providers/number/ints/ResolvableInt#getFromItem` | method | 1 |
| `net/minecraft/world/level/storage/loot/providers/number/ints/ResolvableInt$Reference#key` | method | 1 |
| `net/minecraft/world/phys/AABB#<init>` | method | 5 |
| `net/minecraft/world/phys/AABB#inflate` | method | 2 |
| `net/minecraft/world/phys/AABB#maxY` | field | 1 |
| `net/minecraft/world/phys/AABB#ofSize` | method | 2 |
| `net/minecraft/world/phys/Vec3#<init>` | method | 6 |
| `net/minecraft/world/phys/Vec3#ZERO` | field | 1 |
| `net/minecraft/world/phys/Vec3#add` | method | 1 |
| `net/minecraft/world/phys/Vec3#atBottomCenterOf` | method | 1 |
| `net/minecraft/world/phys/Vec3#atCenterOf` | method | 9 |
| `net/minecraft/world/phys/Vec3#distanceToSqr` | method | 2 |
| `net/minecraft/world/phys/Vec3#length` | method | 1 |
| `net/minecraft/world/phys/Vec3#subtract` | method | 1 |
| `net/minecraft/world/phys/Vec3#x` | field | 8 |
| `net/minecraft/world/phys/Vec3#y` | field | 9 |
| `net/minecraft/world/phys/Vec3#z` | field | 8 |
| `net/minecraft/world/phys/shapes/Shapes#block` | method | 1 |
| `net/minecraft/world/phys/shapes/Shapes#box` | method | 6 |
| `net/minecraft/world/phys/shapes/Shapes#empty` | method | 1 |
| `net/minecraft/world/phys/shapes/Shapes#or` | method | 9 |
| `net/minecraft/world/phys/shapes/VoxelShape#move` | method | 2 |

</details>

