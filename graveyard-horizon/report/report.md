# Horizon port report: The Graveyard

`graveyard` 4.0.0+mc26.3 · graveyard-FABRIC-4.0.0+mc26.3.jar · 366 classes

**Partly convertible**

| Status | Count | Meaning |
|---|---|---|
| auto | 4 | The Horizon hook or injection is known: the wiring can be generated. The Java body still has to become Rust. |
| content | 2 | Data and assets: horizon-port-pack carries these into a content mod (.hmod). |
| needs-mapping | 23 | The kind of change carries over, but the matching Horizon function isn't in the table yet. Run `match`. |
| needs-translation | 115 | No direct equivalent: a person (or an AI agent) has to rewrite this part. |
| unsupported | 17 | Horizon has nothing this can attach to yet. |

## Entry points

- `main`: com.lion.graveyard.fabric.GraveyardFabric
- `client`: com.lion.graveyard.fabric.GraveyardClientFabric

## Fabric events

| Event | In | Horizon | Status | Note |
|---|---|---|---|---|
| ClientTickEvents.END_CLIENT_TICK | `GraveyardClientFabric` | hook `tick` | auto | The Fabric API layer (jfabricapi.rs) runs START_CLIENT_TICK, then the level ticks (START/END_LEVEL_TICK, while in a world), then END_CLIENT_TICK, from Horizon's tick hook, in the window's Game only. |
| ScreenEvents.AFTER_INIT | `GraveyardClientFabric` | – | unsupported | No screen hook in Horizon yet. |
| ClientPlayNetworking.registerGlobalReceiver | `GraveyardClientFabric` | – | unsupported | Custom packets aren't supported. |
| PayloadTypeRegistry.clientboundPlay | `GraveyardFabric` | – | unsupported | Horizon speaks the vanilla protocol; custom packets aren't supported. |
| PayloadTypeRegistry.register | `GraveyardFabric` | – | unsupported | Horizon speaks the vanilla protocol; custom packets aren't supported. |
| CommandRegistrationCallback.EVENT | `GraveyardFabric` | hook `command` | needs-translation | Server commands. Horizon's command hook is client-side. |
| ServerTickEvents.END_LEVEL_TICK | `GraveyardFabric` | hook `tick` | auto | The tick hook runs in every Game, the server's too: run server logic only when `!self.native_window()`. |
| UseItemCallback.EVENT | `GraveyardFabric` | – | unsupported | No interaction hook yet: needs a raw target in Horizon's game code. |
| UseBlockCallback.EVENT | `GraveyardFabric` | – | unsupported | No interaction hook yet: needs a raw target in Horizon's game code. |
| ServerPlayNetworking.send | `ChronicleSyncImpl` | – | unsupported | Custom packets aren't supported. |
| EntityRendererRegistry.register | `RegistryHelperImpl` | – | unsupported | Custom entity renderers aren't supported. |

## Mixins

| Injection | In | Horizon | Status | Note |
|---|---|---|---|---|
| @Inject(TAIL) on BellBlockEntity.onHit | `BellBlockEntityMixin.graveyard$redeemTormentedVillagers` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for BellBlockEntity.onHit: run `match`. |
| @Inject(HEAD) on BlockEntityType.onlyOpCanSetNbt | `BlockEntityTypeMixin.graveyard$gravestoneIsOpOnly` | HEAD | needs-mapping | No Horizon function known for BlockEntityType.onlyOpCanSetNbt: run `match`. |
| @Inject(HEAD) on ClientPacketListener.handleOpenSignEditor | `ClientPlayNetworkHandlerMixin.openSignEditor` | HEAD | needs-mapping | No Horizon function known for ClientPacketListener.handleOpenSignEditor: run `match`. |
| @Inject(RETURN) on CrafterBlock.getPotentialResults | `CrafterBlockMixin.graveyard$skipStaffRecipes` | RETURN | needs-mapping | Horizon's RETURN runs at every normal exit and passes the return value on. No Horizon function known for CrafterBlock.getPotentialResults: run `match`. |
| mixin into ResultSlot with no injectors (interfaces or @Shadow only) | `CraftingResultSlotMixin` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| @Inject(HEAD) on CraftingMenu.slotChangedCraftingGrid | `CraftingScreenHandlerMixin.graveyard$requireKnownStaffRecipe` | HEAD | needs-mapping | No Horizon function known for CraftingMenu.slotChangedCraftingGrid: run `match`. |
| @Inject(HEAD) on CraftingMenu.quickMoveStack | `CraftingScreenHandlerMixin.graveyard$blockLockedStaffShiftClick` | HEAD | needs-mapping | No Horizon function known for CraftingMenu.quickMoveStack: run `match`. |
| @Inject(HEAD) on Creeper.explodeCreeper | `CreeperEntityMixin.explode` | HEAD | needs-mapping | No Horizon function known for Creeper.explodeCreeper: run `match`. |
| @Inject(HEAD) on SimpleRegionStorage.upgradeChunkTag | `EntityChunkUpgradeMixin.graveyard$upgradeAroundModEntities` | HEAD | needs-mapping | No Horizon function known for SimpleRegionStorage.upgradeChunkTag: run `match`. |
| @Inject(TAIL) on InventoryChangeTrigger.trigger | `InventoryChangedCriterionMixin.graveyard$countSkeletonDecoration` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for InventoryChangeTrigger.trigger: run `match`. |
| @ModifyVariable(HEAD) on LevelDatToSavedDataFileFix.extractWorldGenSettingsToFile | `LevelStorageMixin.graveyard$dropPastDimension` | MODIFY_VARIABLE | needs-mapping | Selects a Rust local by name: pick it in the target function. No Horizon function known for LevelDatToSavedDataFileFix.extractWorldGenSettingsToFile: run `match`. |
| @Accessor on Mob.goalSelector | `MobEntityAccessor.graveyard$getGoalSelector` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for Mob.goalSelector: run `match`. |
| @Inject(HEAD) on Mob.checkSpawnRules | `MobEntitySpawnMixin.graveyard$gateAutomaticSpawn` | HEAD | needs-mapping | No Horizon function known for Mob.checkSpawnRules: run `match`. |
| @Inject(HEAD) on Mob.finalizeSpawn | `MobEntitySpawnMixin.graveyard$discardGatedStructureMob` | HEAD on horizon_client::mobs::creaking_heart::finalize_spawn | auto |  |
| @Inject(HEAD) on ChunkSystemConverters.convertEntityChunkCompoundTag | `MoonriseEntityChunkMixin.graveyard$upgradeAroundModEntities` | HEAD | needs-mapping | No Horizon function known for ChunkSystemConverters.convertEntityChunkCompoundTag: run `match`. |
| @Inject(HEAD) on AbstractThrownPotion.douseFire | `PotionEntityMixin.graveyard$extinguishSmoldering` | HEAD on horizon_client::game::splash_water::Game<'_>::douse_fire | auto |  |
| @Inject(HEAD) on ServerGamePacketListenerImpl.updateSignText | `ServerPlayNetworkHandlerMixin.signUpdate` | HEAD | needs-mapping | No Horizon function known for ServerGamePacketListenerImpl.updateSignText: run `match`. |
| @Inject(RETURN) on SinglePoolElement.getShuffledJigsawBlocks | `SinglePoolElementMixin.graveyard$applyPoolAliases` | RETURN | needs-mapping | Horizon's RETURN runs at every normal exit and passes the return value on. No Horizon function known for SinglePoolElement.getShuffledJigsawBlocks: run `match`. |
| @Inject(RETURN) on SinglePoolElement.getSettings | `SinglePoolElementMixin.graveyard$keepWorldWaterOut` | RETURN | needs-mapping | Horizon's RETURN runs at every normal exit and passes the return value on. No Horizon function known for SinglePoolElement.getSettings: run `match`. |
| @Inject(HEAD) on TreeFeature.doPlace | `TreeFeatureMixin.noTreesInStructuresGenerate` | HEAD | needs-mapping | No Horizon function known for TreeFeature.doPlace: run `match`. |
| @Accessor on AbstractContainerScreen.leftPos | `HandledScreenAccessor.graveyard$getX` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for AbstractContainerScreen.leftPos: run `match`. |
| @Accessor on AbstractContainerScreen.topPos | `HandledScreenAccessor.graveyard$getY` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for AbstractContainerScreen.topPos: run `match`. |
| @Accessor on AbstractContainerScreen.imageWidth | `HandledScreenAccessor.graveyard$getBackgroundWidth` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for AbstractContainerScreen.imageWidth: run `match`. |
| @Inject(HEAD) on SoundEngine.play | `SoundEngineMixin.graveyard$readPlayedSound` | HEAD | needs-mapping | No Horizon function known for SoundEngine.play: run `match`. |
| @Inject(HEAD) on SoundEngine.calculateVolume | `SoundEngineMixin.graveyard$readTickedSound` | HEAD | needs-mapping | No Horizon function known for SoundEngine.calculateVolume: run `match`. |
| @Inject(RETURN) on SoundEngine.calculateVolume | `SoundEngineMixin.graveyard$scaleVolume` | RETURN | needs-mapping | Horizon's RETURN runs at every normal exit and passes the return value on. No Horizon function known for SoundEngine.calculateVolume: run `match`. |

## Classes extending the game

| Class | In | Status | Note |
|---|---|---|---|
| extends SimpleJsonResourceReloadListener | `NamelessHangedTradeOfferResourceListener` | needs-translation | No mapping for this superclass yet. |
| extends SimpleCriterionTrigger | `TGAdvancementTrigger` | needs-translation | No mapping for this superclass yet. |
| extends BlockEntity | `BrazierBlockEntity` | unsupported | Block entities aren't supported in Horizon mods yet. |
| extends BlockEntity | `GravestoneBlockEntity` | unsupported | Block entities aren't supported in Horizon mods yet. |
| extends BlockEntity | `LoreBookBlockEntity` | unsupported | Block entities aren't supported in Horizon mods yet. |
| extends BlockEntity | `OssuaryBlockEntity` | unsupported | Block entities aren't supported in Horizon mods yet. |
| extends ContainerOpenersCounter | `SarcophagusBlockEntity$1` | needs-translation | No mapping for this superclass yet. |
| extends RandomizableContainerBlockEntity | `SarcophagusBlockEntity` | needs-translation | No mapping for this superclass yet. |
| extends BlockEntity | `SmolderingDebrisBlockEntity` | unsupported | Block entities aren't supported in Horizon mods yet. |
| extends ContainerOpenersCounter | `UrnBlockEntity$1` | needs-translation | No mapping for this superclass yet. |
| extends RandomizableContainerBlockEntity | `UrnBlockEntity` | needs-translation | No mapping for this superclass yet. |
| extends Model | `GravestoneBlockEntityRenderer$QuadModel` | needs-translation | No mapping for this superclass yet. |
| extends BlockEntityRenderState | `GravestoneBlockEntityRenderer$State` | needs-translation | No mapping for this superclass yet. |
| extends BlockEntityRenderState | `SarcophagusBlockEntityRenderer$State` | needs-translation | No mapping for this superclass yet. |
| extends Block | `AltarBlock` | needs-translation | Overrides createBlockStateDefinition, animateTick, getCompletedFramePattern, getAnyOrderFramePattern, findSummonFailure, hasLivingLich, useItemOn: the content ports, that behaviour needs native code. |
| extends Block | `AltarParticleBlock` | needs-translation | Overrides animateTick: the content ports, that behaviour needs native code. |
| extends Block | `BoneDisplayBlock` | needs-translation | Overrides propagatesSkylightDown, getShadeBrightness, createBlockStateDefinition, getStateForPlacement, getFluidState, rotate, isCollisionShapeFullBlock: the content ports, that behaviour needs native code. |
| extends Block | `BrazierBlock` | needs-translation | Overrides useItemOn, getFluidState, getStateForPlacement, updateShape, isPathfindable, placeLiquid, getShape, animateTick, onProjectileHit, newBlockEntity, getRenderShape, createBlockStateDefinition, extinguish, spawnSmokeParticle, setLit: the content ports, that behaviour needs native code. |
| extends Block | `CandleHolderBlock` | needs-translation | Overrides getShape, getStateForPlacement, updateShape, getFluidState, createBlockStateDefinition, rotate: the content ports, that behaviour needs native code. |
| extends IronBarsBlock | `DarkIronBars` | needs-translation | No mapping for this superclass yet. |
| extends StandingSignBlock | `GravestoneBlock` | needs-translation | No mapping for this superclass yet. |
| extends Block | `LongDecoBlock` | needs-translation | Overrides createBlockStateDefinition, getStateForPlacement, setPlacedBy, otherPartPos, updateShape, getDrops, playerWillDestroy, getShape, getFluidState, rotate, mirror: the content ports, that behaviour needs native code. |
| extends BaseEntityBlock | `LoreBookBlock` | needs-translation | No mapping for this superclass yet. |
| extends Block | `OminousBoneStaffFragment` | needs-translation | Overrides animateTick, appendItemTooltip, setPlacedBy, propagatesSkylightDown, getShadeBrightness, createBlockStateDefinition, getStateForPlacement, getFluidState, rotate, isCollisionShapeFullBlock: the content ports, that behaviour needs native code. |
| extends BaseEntityBlock | `OssuaryBlock` | needs-translation | No mapping for this superclass yet. |
| extends Block | `PillarBlock` | needs-translation | Overrides getShape, getStateForPlacement, updateShape, getFluidState, createBlockStateDefinition, isPathfindable: the content ports, that behaviour needs native code. |
| extends BaseEntityBlock | `SarcophagusBlock` | needs-translation | No mapping for this superclass yet. |
| extends BaseEntityBlock | `SmolderingDebrisBlock` | needs-translation | No mapping for this superclass yet. |
| extends DoorBlock | `SmolderingDoorBlock` | needs-translation | No mapping for this superclass yet. |
| extends FenceBlock | `SmolderingFenceBlock` | needs-translation | No mapping for this superclass yet. |
| extends FenceGateBlock | `SmolderingFenceGateBlock` | needs-translation | No mapping for this superclass yet. |
| extends RotatedPillarBlock | `SmolderingPillarBlock` | needs-translation | No mapping for this superclass yet. |
| extends SlabBlock | `SmolderingSlabBlock` | needs-translation | No mapping for this superclass yet. |
| extends StairBlock | `SmolderingStairsBlock` | needs-translation | No mapping for this superclass yet. |
| extends TrapDoorBlock | `SmolderingTrapdoorBlock` | needs-translation | No mapping for this superclass yet. |
| extends Block | `SmolderingWoodBlock` | needs-translation | Overrides createBlockStateDefinition, useItemOn, animateTick: the content ports, that behaviour needs native code. |
| extends RotatedPillarBlock | `TGDeepslateBlock` | needs-translation | No mapping for this superclass yet. |
| extends DoorBlock | `TGDoorBlock` | needs-translation | No mapping for this superclass yet. |
| extends GrassBlock | `TGGrassBlock` | needs-translation | No mapping for this superclass yet. |
| extends BonemealableFeaturePlacerBlock | `TGMossBlock` | needs-translation | No mapping for this superclass yet. |
| extends StairBlock | `TGStairsBlock` | needs-translation | No mapping for this superclass yet. |
| extends Block | `TGStoneBlock` | needs-translation | Overrides getCloneItemStack: the content ports, that behaviour needs native code. |
| extends TrapDoorBlock | `TGTrapDoorBlock` | needs-translation | No mapping for this superclass yet. |
| extends Block | `TallDecoBlock` | needs-translation | Overrides createBlockStateDefinition, getStateForPlacement, setPlacedBy, canSurvive, updateShape, playerWillDestroy, getDrops, getShape, getFluidState, rotate, mirror: the content ports, that behaviour needs native code. |
| extends BaseEntityBlock | `UrnBlock` | needs-translation | No mapping for this superclass yet. |
| extends Block | `VaseBlock` | needs-translation | Overrides createBlockStateDefinition, getStateForPlacement, getFluidState, canBeReplaced, getShape, canSurvive: the content ports, that behaviour needs native code. |
| extends SavedData | `ChronicleState` | needs-translation | No mapping for this superclass yet. |
| extends Button | `ChronicleButton` | needs-translation | No mapping for this superclass yet. |
| extends Screen | `ChronicleScreen` | needs-translation | No general screen hook; simple option pages can use the settings hook. |
| extends MeleeAttackGoal | `CorruptedIllager$AttackGoal` | needs-translation | No mapping for this superclass yet. |
| extends PathfinderMob | `CorvidEntity` | unsupported | New entities aren't supported. |
| extends Monster | `FallingCorpse` | needs-translation | No mapping for this superclass yet. |
| extends SavedData | `GhoulingBindings` | needs-translation | No mapping for this superclass yet. |
| extends SimpleContainer | `GhoulingEntity$CoffinInventory` | needs-translation | No mapping for this superclass yet. |
| extends PanicGoal | `GhoulingEntity$GhoulingEscapeDangerGoal` | needs-translation | No mapping for this superclass yet. |
| extends PathfinderMob | `GraveyardMinionEntity` | unsupported | New entities aren't supported. |
| extends Goal | `HordeGraveyardEntity$PatrolApproachGoal` | needs-translation | No mapping for this superclass yet. |
| extends Goal | `HordeGraveyardEntity$PatrolGoal` | needs-translation | No mapping for this superclass yet. |
| extends Monster | `HostileGraveyardEntity` | needs-translation | No mapping for this superclass yet. |
| extends Goal | `LichEntity$ConjureFangsGoal` | needs-translation | No mapping for this superclass yet. |
| extends Goal | `LichEntity$LevitationGoal` | needs-translation | No mapping for this superclass yet. |
| extends Goal | `LichEntity$ShootSkullGoal` | needs-translation | No mapping for this superclass yet. |
| extends Goal | `LichEntity$SummonFallenCorpsesGoal` | needs-translation | No mapping for this superclass yet. |
| extends Goal | `LichEntity$TeleportAndHealGoal` | needs-translation | No mapping for this superclass yet. |
| extends Monster | `LichEntity` | needs-translation | No mapping for this superclass yet. |
| extends AbstractVillager | `NamelessHangedEntity` | needs-translation | No mapping for this superclass yet. |
| extends Goal | `NightmareEntity$ChasePlayerGoal` | needs-translation | No mapping for this superclass yet. |
| extends NearestAttackableTargetGoal | `NightmareEntity$TeleportTowardsPlayerGoal` | needs-translation | No mapping for this superclass yet. |
| extends Goal | `ReaperEntity$ChargeTargetGoal` | needs-translation | No mapping for this superclass yet. |
| extends Goal | `ReaperEntity$LookAtTargetGoal` | needs-translation | No mapping for this superclass yet. |
| extends MoveControl | `ReaperEntity$ReaperMoveControl` | needs-translation | No mapping for this superclass yet. |
| extends TargetGoal | `ReaperEntity$TrackOwnerTargetGoal` | needs-translation | No mapping for this superclass yet. |
| extends Creeper | `SkeletonCreeper` | needs-translation | No mapping for this superclass yet. |
| extends WaterAvoidingRandomStrollGoal | `TormentedVillagerEntity$1` | needs-translation | No mapping for this superclass yet. |
| extends PathfinderMob | `TormentedVillagerEntity` | unsupported | New entities aren't supported. |
| extends FlyingPathNavigation | `WraithEntity$1` | needs-translation | No mapping for this superclass yet. |
| extends Goal | `WraithEntity$ExtinguishGoal` | needs-translation | No mapping for this superclass yet. |
| extends MeleeAttackGoal | `WraithEntity$WraithMeleeGoal` | needs-translation | No mapping for this superclass yet. |
| extends Goal | `WraithEntity$WraithWanderAroundGoal` | needs-translation | No mapping for this superclass yet. |
| extends FlyingMoveControl | `CorvidMoveControl` | needs-translation | No mapping for this superclass yet. |
| extends Goal | `AbstractGraveyardMeleeGoal` | needs-translation | No mapping for this superclass yet. |
| extends TargetGoal | `AttackWithOwnerGoal` | needs-translation | No mapping for this superclass yet. |
| extends Goal | `CircleAnchorGoal` | needs-translation | No mapping for this superclass yet. |
| extends AvoidEntityGoal | `FleeGraveyardMobsGoal` | needs-translation | No mapping for this superclass yet. |
| extends Goal | `FlockStartleGoal` | needs-translation | No mapping for this superclass yet. |
| extends WaterAvoidingRandomFlyingGoal | `FlyOntoPerchGoal` | needs-translation | No mapping for this superclass yet. |
| extends Goal | `FollowOwnerGoal` | needs-translation | No mapping for this superclass yet. |
| extends Goal | `GhoulingCollectItemsGoal` | needs-translation | No mapping for this superclass yet. |
| extends Goal | `HopGoal` | needs-translation | No mapping for this superclass yet. |
| extends Goal | `JoinFlockGoal` | needs-translation | No mapping for this superclass yet. |
| extends Goal | `PickTreasureGoal` | needs-translation | No mapping for this superclass yet. |
| extends Goal | `SitGoal` | needs-translation | No mapping for this superclass yet. |
| extends Goal | `StartleGoal` | needs-translation | No mapping for this superclass yet. |
| extends TargetGoal | `TrackOwnerAttackerGoal` | needs-translation | No mapping for this superclass yet. |
| extends EntityModel | `CorruptedIllagerModel` | needs-translation | No mapping for this superclass yet. |
| extends AbstractHurtingProjectile | `SkullEntity` | needs-translation | No mapping for this superclass yet. |
| extends ItemInHandLayer | `AcolyteRender$1` | needs-translation | No mapping for this superclass yet. |
| extends IllagerRenderState | `CorruptedIllagerRenderState` | needs-translation | No mapping for this superclass yet. |
| extends MobRenderer | `CorruptedIllagerRenderer` | needs-translation | No mapping for this superclass yet. |
| extends MobRenderer | `SkeletonCreeperRender` | needs-translation | No mapping for this superclass yet. |
| extends EntityRenderer | `SkullEntityRenderer` | needs-translation | No mapping for this superclass yet. |
| extends EyesLayer | `AcolyteEyes` | needs-translation | No mapping for this superclass yet. |
| extends EnergySwirlLayer | `SkeletonCreeperChargeFeatureRenderer` | needs-translation | No mapping for this superclass yet. |
| extends EyesLayer | `SkeletonCreeperEyes` | needs-translation | No mapping for this superclass yet. |
| extends Screen | `GravestoneScreen` | needs-translation | No general screen hook; simple option pages can use the settings hook. |
| extends AbstractContainerScreen | `OssuaryScreen` | needs-translation | No mapping for this superclass yet. |
| extends SimpleContainer | `OssuaryScreenHandler$1` | needs-translation | No mapping for this superclass yet. |
| extends Slot | `OssuaryScreenHandler$2` | needs-translation | No mapping for this superclass yet. |
| extends AbstractContainerMenu | `OssuaryScreenHandler` | unsupported | Menus aren't supported. |
| extends Item | `BoneStaffItem` | needs-translation | Overrides useOn, spawnPosition, clipAtBlocks, isBoundTo, startDrawing, stopDrawSound, streamSouls, release, store, fitsInStack, getUseDuration, getUseAnimation, onUseTick, releaseUsing, isFoil, appendHoverText, findLoaded, isUnbound, rebind, rejectsStaff, ownerAirborne, isOwnedBy, isAlive, inventoryTick, call, findLanding, fetch, findItemInSight, startFarDraw, use: the content ports, that behaviour needs native code. |
| extends Item | `DaggerItem` | needs-translation | Overrides getDestroySpeed, hurtEnemy, fillHotbarVial, convertHotbarBottle, giveVial: the content ports, that behaviour needs native code. |
| extends BlockItem | `DiaryPageItem` | needs-translation | Overrides of, pageOf, use, appendHoverText: the content ports, that behaviour needs native code. |
| extends BlockItem | `SarcophagusItem` | needs-translation | Overrides placeBlock: the content ports, that behaviour needs native code. |
| extends BlockItem | `TooltipBlockItem` | needs-translation | Overrides appendHoverText: the content ports, that behaviour needs native code. |
| extends Item | `VialOfBlood` | needs-translation | Overrides getBlood, setBlood, appendHoverText: the content ports, that behaviour needs native code. |
| extends SingleQuadParticle | `GraveyardFogParticle` | needs-translation | No mapping for this superclass yet. |
| extends RisingParticle | `GraveyardHandParticle` | needs-translation | No mapping for this superclass yet. |
| extends RisingParticle | `GraveyardLineFogParticle` | needs-translation | No mapping for this superclass yet. |
| extends RisingParticle | `GraveyardSoulParticle` | needs-translation | No mapping for this superclass yet. |
| extends AbstractSoundInstance | `LichBossMusic` | needs-translation | No mapping for this superclass yet. |
| extends Structure | `TGJigsawStructure` | needs-translation | No mapping for this superclass yet. |

## Access wideners

| File | In | Status | Note |
|---|---|---|---|
| access widener (opens private game fields) | `graveyard.accesswidener` | needs-translation | Native helpers already see the game's fields, once each field is mapped. |

## Content

| What | In | Status | Note |
|---|---|---|---|
| registers blocks or items in code | `RegistryHelperImpl` | content | Registration and properties: horizon-port-pack carries them as a content mod. |
| 1607 asset files, 857 data files | `jar resources` | content | Recipes, loot, tags, models, textures, worldgen: horizon-port-pack carries these. |

## Game methods and fields used

99 of 1804 are mapped to Horizon. Every Java body that uses an unmapped one needs it mapped before its Rust can be written.

<details><summary>Mapped</summary>

| Java | Horizon |
|---|---|
| `com/mojang/blaze3d/vertex/PoseStack#rotateDegrees` | horizon_client_render::camera::rotate_degrees (suggested) |
| `net/minecraft/client/Minecraft#font` | `(implicit)` |
| `net/minecraft/client/Minecraft#getInstance` | `self` |
| `net/minecraft/client/Minecraft#player` | `self.native_player()` |
| `net/minecraft/client/gui/Font#split` | horizon_client_render::font::Font::split_formatted (suggested) |
| `net/minecraft/client/gui/Font#width` | `self.native_text_width(text)` |
| `net/minecraft/client/gui/GuiGraphicsExtractor#blitSprite` | horizon_client_render::ui::blit_sprite (suggested) |
| `net/minecraft/client/gui/GuiGraphicsExtractor#textHighlight` | horizon_client_render::hud::HudBatch::text_highlight (suggested) |
| `net/minecraft/client/model/AnimationUtils#swingWeaponDown` | horizon_client::mobs::anim::swing_weapon_down (suggested) |
| `net/minecraft/client/renderer/item/ItemStackRenderState#submit` | horizon_client::item_render::ItemRenderState::submit (suggested) |
| `net/minecraft/client/resources/sounds/SimpleSoundInstance#forUI` | horizon_client_render::sound_engine::SoundInstance::for_ui (suggested) |
| `net/minecraft/client/sounds/MusicManager#stopPlaying` | horizon_client::music::MusicManager::stop_playing (suggested) |
| `net/minecraft/client/sounds/SoundManager#isActive` | horizon_client_render::sound_engine::is_active (suggested) |
| `net/minecraft/core/Direction#getOpposite` | horizon_client::mobs::shulker::opposite (suggested) |
| `net/minecraft/core/Registry#register` | content |
| `net/minecraft/core/registries/BuiltInRegistries#BLOCK` | content |
| `net/minecraft/core/registries/BuiltInRegistries#ITEM` | content |
| `net/minecraft/nbt/NbtUtils#getDataVersion` | horizon_world::datafix::files::data_version (suggested) |
| `net/minecraft/network/chat/MutableComponent#append` | horizon_client::game::dedicated::append (suggested) |
| `net/minecraft/network/chat/MutableComponent#withStyle` | horizon_client_core::text::Text::with_style (suggested) |
| `net/minecraft/network/codec/ByteBufCodecs#list` | horizon_protocol::item::skip::list (suggested) |
| `net/minecraft/server/PlayerAdvancements#award` | horizon_client::game::advancements::Game<'_>::award (suggested) |
| `net/minecraft/server/level/ServerLevel#getHeight` | horizon_world::level::feature_world::<LevelFeatureWorld<'_, '_, L> as PlacementContext>::height (suggested) |
| `net/minecraft/server/level/ServerLevel#hasChunkAt` | horizon_client::raid::has_chunk_at (suggested) |
| `net/minecraft/server/level/ServerLevel#hasChunksAt` | horizon_client::raid::has_chunks_at (suggested) |
| `net/minecraft/server/level/ServerLevel#isClientSide` | horizon_world::level::is_client_side (suggested) |
| `net/minecraft/server/level/ServerLevel#noCollision` | horizon_client::mobs::shulker::no_collision (suggested) |
| `net/minecraft/server/level/ServerLevel#sendParticles` | horizon_client::game::fishing::Game<'_>::send_particles (suggested) |
| `net/minecraft/server/level/ServerPlayer#hasEffect` | horizon_client::mobs::PlayerView::has_effect (suggested) |
| `net/minecraft/util/ARGB#opaque` | horizon_client_render::level_loading::opaque (suggested) |
| `net/minecraft/util/ExtraCodecs#nonEmptyList` | horizon_command::component::non_empty_list (suggested) |
| `net/minecraft/util/Mth#atan2` | horizon_core::geometry::trig::atan2 (suggested) |
| `net/minecraft/util/Mth#ceil` | horizon_client_render::hud_layout::ceil (suggested) |
| `net/minecraft/util/Mth#clamp` | horizon_client_render::chat::clamp (suggested) |
| `net/minecraft/util/Mth#floor` | horizon_client_render::gfx::occlusion::floor (suggested) |
| `net/minecraft/util/Mth#lerp` | horizon_world::stepped_interpolation::lerp (suggested) |
| `net/minecraft/util/Mth#nextInt` | horizon_world::worldgen::structure::ocean_ruin::next_int (suggested) |
| `net/minecraft/util/Mth#rotLerp` | horizon_world::stepped_interpolation::rot_lerp (suggested) |
| `net/minecraft/util/Mth#wrapDegrees` | horizon_world::stepped_interpolation::wrap_degrees (suggested) |
| `net/minecraft/util/RandomSource#triangle` | horizon_client::mobs::nether::triangle (suggested) |
| `net/minecraft/world/SimpleContainer#addItem` | horizon_client::mobs::villager::Ext::add_item (suggested) |
| `net/minecraft/world/damagesource/DamageSources#indirectMagic` | horizon_client::mobs::Damage::indirect_magic (suggested) |
| `net/minecraft/world/entity/Entity#getX` | `self.native_player()?.x` |
| `net/minecraft/world/entity/Entity#getY` | `self.native_player()?.y` |
| `net/minecraft/world/entity/Entity#getZ` | `self.native_player()?.z` |
| `net/minecraft/world/entity/EntityType#create` | horizon_client::mobs::finalize::create (suggested) |
| `net/minecraft/world/entity/Mob#finalizeSpawn` | horizon_client::mobs::creaking_heart::finalize_spawn (suggested) |
| `net/minecraft/world/entity/Mob#onGround` | horizon_client::game::minecarts::Minecart::on_ground (suggested) |
| `net/minecraft/world/entity/Mob#setDropChance` | horizon_client::mobs::finalize::set_drop_chance (suggested) |
| `net/minecraft/world/entity/Mob#setSpeed` | horizon_client::mobs::control::Mob::set_speed (suggested) |
| `net/minecraft/world/entity/PathfinderMob#aiStep` | horizon_client::mobs::living::Mob::handle_jumping (suggested) |
| `net/minecraft/world/entity/PathfinderMob#canAttack` | horizon_client::mobs::monster_brain::can_attack (suggested) |
| `net/minecraft/world/entity/PathfinderMob#die` | horizon_client::mobs::drops::die (suggested) |
| `net/minecraft/world/entity/PathfinderMob#finalizeSpawn` | horizon_client::mobs::creaking_heart::finalize_spawn (suggested) |
| `net/minecraft/world/entity/PathfinderMob#getViewVector` | horizon_client::mobs::goals::Mob::view_vector (suggested) |
| `net/minecraft/world/entity/PathfinderMob#hurtServer` | horizon_client::mobs::Mob::worn_armor (suggested) |
| `net/minecraft/world/entity/PathfinderMob#isOnFire` | horizon_client::mobs::Mob::is_on_fire (suggested) |
| `net/minecraft/world/entity/PathfinderMob#isWithinMeleeAttackRange` | horizon_client::mobs::goals::Mob::within_melee_range (suggested) |
| `net/minecraft/world/entity/PathfinderMob#tick` | horizon_client::mobs::Mob::tick_avatar_pose (suggested) |
| `net/minecraft/world/entity/player/Inventory#add` | horizon_client::cauldron::add (suggested) |
| `net/minecraft/world/entity/player/Player#getViewVector` | horizon_client::mobs::goals::Mob::view_vector (suggested) |
| `net/minecraft/world/entity/player/Player#hasEffect` | horizon_client::mobs::PlayerView::has_effect (suggested) |
| `net/minecraft/world/entity/player/Player#hurt` | horizon_client::mobs::Mob::hurt (suggested) |
| `net/minecraft/world/entity/player/Player#onGround` | horizon_client::game::minecarts::Minecart::on_ground (suggested) |
| `net/minecraft/world/entity/projectile/Projectile#mayInteract` | horizon_client::arrows::Ticker<'_, '_, '_, '_>::may_interact (suggested) |
| `net/minecraft/world/entity/projectile/throwableitemprojectile/AbstractThrownPotion#douseFire` | horizon_client::game::splash_water::Game<'_>::douse_fire (suggested) |
| `net/minecraft/world/inventory/Slot#remove` | horizon_client::menu::Menu::remove (suggested) |
| `net/minecraft/world/inventory/Slot#setByPlayer` | horizon_client::menu::Menu::set_by_player (suggested) |
| `net/minecraft/world/item/DyeColor#getTextColor` | horizon_client_render::signs::SignText::text_color (suggested) |
| `net/minecraft/world/item/Item#<init>` | content |
| `net/minecraft/world/item/Item$Properties#stacksTo` | content |
| `net/minecraft/world/item/ItemStack#getCount` | horizon_world::loot::LootStack::get_count (suggested) |
| `net/minecraft/world/item/ItemStack#hurtAndBreak` | horizon_client::game::dispenser::Game<'_>::hurt_and_break (suggested) |
| `net/minecraft/world/item/ItemStack#shrink` | horizon_client::game::dispenser::shrink (suggested) |
| `net/minecraft/world/item/ItemStackTemplate#create` | horizon_client::crafting::Template::create (suggested) |
| `net/minecraft/world/level/Level#canSeeSky` | horizon_client::mobs::world::MobWorld<'_>::can_see_sky (suggested) |
| `net/minecraft/world/level/Level#getHeight` | horizon_world::level::feature_world::<LevelFeatureWorld<'_, '_, L> as PlacementContext>::height (suggested) |
| `net/minecraft/world/level/Level#getMaxY` | horizon_client::game::Dim::max_y (suggested) |
| `net/minecraft/world/level/Level#getMinY` | horizon_client::game::Dim::min_y (suggested) |
| `net/minecraft/world/level/Level#hasChunkAt` | horizon_client::raid::has_chunk_at (suggested) |
| `net/minecraft/world/level/Level#isClientSide` | horizon_world::level::is_client_side (suggested) |
| `net/minecraft/world/level/Level#scheduleTick` | horizon_world::level::Behaviours::schedule_tick (suggested) |
| `net/minecraft/world/level/Level#setBlock` | horizon_world::level::Behaviours::set_block (suggested) |
| `net/minecraft/world/level/Level#updateNeighborsAt` | horizon_world::level::Behaviours::update_neighbors_at (suggested) |
| `net/minecraft/world/level/block/Block#<init>` | content |
| `net/minecraft/world/level/block/Block#canSupportCenter` | horizon_world::level::query::Sim<'_, L>::can_support_center (suggested) |
| `net/minecraft/world/level/block/Block#getStateForPlacement` | horizon_world::level::place::Sim<'_, L>::state_for_placement (suggested) |
| `net/minecraft/world/level/block/Block#setPlacedBy` | horizon_world::level::place::Sim<'_, L>::set_placed_by (suggested) |
| `net/minecraft/world/level/block/Block#withPropertiesOf` | horizon_world::worldgen::structure::processor::with_properties_of (suggested) |
| `net/minecraft/world/level/block/state/BlockState#canBeReplaced` | horizon_world::level::place::Sim<'_, L>::can_be_replaced (suggested) |
| `net/minecraft/world/level/block/state/BlockState#canOcclude` | horizon_world::worldgen::placement::context::can_occlude (suggested) |
| `net/minecraft/world/level/block/state/BlockState#canSurvive` | horizon_world::level::survive::Sim<'_, L>::can_survive (suggested) |
| `net/minecraft/world/level/block/state/BlockState#getCollisionShape` | horizon_client_render::collisions::<SceneCollisions<'_, '_> as PistonCollisionWorld>::collision_shape (suggested) |
| `net/minecraft/world/level/block/state/pattern/BlockPattern#find` | horizon_client::block_pattern::BlockPattern::find (suggested) |
| `net/minecraft/world/level/chunk/ChunkGenerator#getBaseColumn` | horizon_world::worldgen::generator::Pipeline<R>::base_column (suggested) |
| `net/minecraft/world/level/pathfinder/WalkNodeEvaluator#getPathTypeStatic` | horizon_client::mobs::path::MobWorld<'_>::path_type_static (suggested) |
| `net/minecraft/world/phys/AABB#intersects` | horizon_core::geometry::Aabb::intersects_corners (suggested) |
| `net/minecraft/world/phys/Vec3#normalize` | horizon_client::mobs::creaking::normalize (suggested) |
| `net/minecraft/world/phys/Vec3#yRot` | horizon_client::mobs::raider::y_rot (suggested) |

</details>

<details><summary>Not mapped yet</summary>

| Java | Kind | Classes using it |
|---|---|---|
| `ca/spottedleaf/moonrise/patches/chunk_system/ChunkSystemConverters#convertEntityChunkCompoundTag` | method | 1 |
| `com/mojang/blaze3d/platform/TextInputManager#startTextInput` | method | 1 |
| `com/mojang/blaze3d/platform/TextInputManager#stopTextInput` | method | 1 |
| `com/mojang/blaze3d/vertex/PoseStack#popPose` | method | 3 |
| `com/mojang/blaze3d/vertex/PoseStack#pushPose` | method | 3 |
| `com/mojang/blaze3d/vertex/PoseStack#scale` | method | 6 |
| `com/mojang/blaze3d/vertex/PoseStack#translate` | method | 5 |
| `net/minecraft/ChatFormatting#BOLD` | field | 1 |
| `net/minecraft/ChatFormatting#DARK_GRAY` | field | 3 |
| `net/minecraft/ChatFormatting#GOLD` | field | 1 |
| `net/minecraft/ChatFormatting#GRAY` | field | 4 |
| `net/minecraft/ChatFormatting#ITALIC` | field | 3 |
| `net/minecraft/SharedConstants#getCurrentVersion` | method | 2 |
| `net/minecraft/WorldVersion#dataVersion` | method | 2 |
| `net/minecraft/advancements/AdvancementProgress#getCriterion` | method | 1 |
| `net/minecraft/advancements/CriterionProgress#isDone` | method | 1 |
| `net/minecraft/advancements/triggers/CriteriaTriggers#ITEM_USED_ON_BLOCK` | field | 1 |
| `net/minecraft/advancements/triggers/CriteriaTriggers#SUMMONED_ENTITY` | field | 1 |
| `net/minecraft/advancements/triggers/InventoryChangeTrigger#trigger` | method | 1 |
| `net/minecraft/advancements/triggers/ItemUsedOnLocationTrigger#trigger` | method | 1 |
| `net/minecraft/advancements/triggers/SimpleCriterionTrigger#<init>` | method | 1 |
| `net/minecraft/advancements/triggers/SummonedEntityTrigger#trigger` | method | 1 |
| `net/minecraft/client/Camera#position` | method | 1 |
| `net/minecraft/client/CameraType#isFirstPerson` | method | 1 |
| `net/minecraft/client/Minecraft#gameMode` | field | 1 |
| `net/minecraft/client/Minecraft#getCameraEntity` | method | 1 |
| `net/minecraft/client/Minecraft#getConnection` | method | 1 |
| `net/minecraft/client/Minecraft#getItemModelResolver` | method | 1 |
| `net/minecraft/client/Minecraft#getMusicManager` | method | 1 |
| `net/minecraft/client/Minecraft#getResourceManager` | method | 1 |
| `net/minecraft/client/Minecraft#getSoundManager` | method | 3 |
| `net/minecraft/client/Minecraft#gui` | field | 6 |
| `net/minecraft/client/Minecraft#isTextFilteringEnabled` | method | 1 |
| `net/minecraft/client/Minecraft#level` | field | 4 |
| `net/minecraft/client/Minecraft#options` | field | 2 |
| `net/minecraft/client/Minecraft#packetProcessor` | method | 1 |
| `net/minecraft/client/Minecraft#textInputManager` | method | 1 |
| `net/minecraft/client/Options#getCameraType` | method | 1 |
| `net/minecraft/client/Options#getSoundSourceVolume` | method | 1 |
| `net/minecraft/client/color/block/BlockTintSources#grassBlock` | method | 1 |
| `net/minecraft/client/gui/Font#bidirectionalShaping` | method | 1 |
| `net/minecraft/client/gui/Font#isBidirectional` | method | 1 |
| `net/minecraft/client/gui/Font$DisplayMode#NORMAL` | field | 1 |
| `net/minecraft/client/gui/Gui#screen` | method | 1 |
| `net/minecraft/client/gui/Gui#setScreen` | method | 5 |
| `net/minecraft/client/gui/Gui#toastManager` | method | 1 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#blit` | method | 4 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#centeredText` | method | 1 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#fakeItem` | method | 1 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#fill` | method | 2 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#item` | method | 2 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#pose` | method | 1 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#setTooltipForNextFrame` | method | 2 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#text` | method | 4 |
| `net/minecraft/client/gui/components/Button#<init>` | method | 1 |
| `net/minecraft/client/gui/components/Button#builder` | method | 2 |
| `net/minecraft/client/gui/components/Button$Builder#bounds` | method | 2 |
| `net/minecraft/client/gui/components/Button$Builder#build` | method | 2 |
| `net/minecraft/client/gui/components/Tooltip#create` | method | 1 |
| `net/minecraft/client/gui/components/toasts/Toast$Visibility#HIDE` | field | 1 |
| `net/minecraft/client/gui/components/toasts/Toast$Visibility#SHOW` | field | 1 |
| `net/minecraft/client/gui/components/toasts/ToastManager#addToast` | method | 1 |
| `net/minecraft/client/gui/font/TextFieldHelper#<init>` | method | 1 |
| `net/minecraft/client/gui/font/TextFieldHelper#charTyped` | method | 1 |
| `net/minecraft/client/gui/font/TextFieldHelper#createClipboardGetter` | method | 1 |
| `net/minecraft/client/gui/font/TextFieldHelper#createClipboardSetter` | method | 1 |
| `net/minecraft/client/gui/font/TextFieldHelper#getCursorPos` | method | 1 |
| `net/minecraft/client/gui/font/TextFieldHelper#getSelectionPos` | method | 1 |
| `net/minecraft/client/gui/font/TextFieldHelper#keyPressed` | method | 1 |
| `net/minecraft/client/gui/font/TextFieldHelper#setCursorToEnd` | method | 1 |
| `net/minecraft/client/gui/screens/MenuScreens#register` | method | 1 |
| `net/minecraft/client/gui/screens/Screen#<init>` | method | 2 |
| `net/minecraft/client/gui/screens/Screen#extractRenderState` | method | 1 |
| `net/minecraft/client/gui/screens/Screen#keyPressed` | method | 1 |
| `net/minecraft/client/gui/screens/Screen#mouseClicked` | method | 1 |
| `net/minecraft/client/gui/screens/inventory/AbstractContainerScreen#<init>` | method | 1 |
| `net/minecraft/client/gui/screens/inventory/AbstractContainerScreen#extractBackground` | method | 1 |
| `net/minecraft/client/gui/screens/inventory/AbstractContainerScreen#extractTooltip` | method | 1 |
| `net/minecraft/client/gui/screens/inventory/AbstractContainerScreen#imageWidth` | method | 1 |
| `net/minecraft/client/gui/screens/inventory/AbstractContainerScreen#leftPos` | method | 1 |
| `net/minecraft/client/gui/screens/inventory/AbstractContainerScreen#mouseClicked` | method | 1 |
| `net/minecraft/client/gui/screens/inventory/AbstractContainerScreen#mouseDragged` | method | 1 |
| `net/minecraft/client/gui/screens/inventory/AbstractContainerScreen#topPos` | method | 1 |
| `net/minecraft/client/gui/screens/inventory/PageButton#<init>` | method | 1 |
| `net/minecraft/client/gui/screens/inventory/PageButton#visible` | field | 1 |
| `net/minecraft/client/input/KeyEvent#isConfirmation` | method | 1 |
| `net/minecraft/client/input/KeyEvent#isDown` | method | 1 |
| `net/minecraft/client/input/KeyEvent#isUp` | method | 1 |
| `net/minecraft/client/input/MouseButtonEvent#button` | method | 1 |
| `net/minecraft/client/input/MouseButtonEvent#x` | method | 2 |
| `net/minecraft/client/input/MouseButtonEvent#y` | method | 2 |
| `net/minecraft/client/model/AnimationUtils#animateCrossbowCharge` | method | 1 |
| `net/minecraft/client/model/AnimationUtils#animateCrossbowHold` | method | 1 |
| `net/minecraft/client/model/AnimationUtils#animateZombieArms` | method | 1 |
| `net/minecraft/client/model/EntityModel#<init>` | method | 1 |
| `net/minecraft/client/model/Model#<init>` | method | 1 |
| `net/minecraft/client/model/geom/EntityModelSet#bakeLayer` | method | 1 |
| `net/minecraft/client/model/geom/ModelLayerLocation#<init>` | method | 1 |
| `net/minecraft/client/model/geom/ModelLayers#CREEPER` | field | 1 |
| `net/minecraft/client/model/geom/ModelLayers#CREEPER_ARMOR` | field | 1 |
| `net/minecraft/client/model/geom/ModelLayers#SKELETON_SKULL` | field | 1 |
| `net/minecraft/client/model/geom/ModelPart#<init>` | method | 1 |
| `net/minecraft/client/model/geom/ModelPart#getChild` | method | 1 |
| `net/minecraft/client/model/geom/ModelPart#translateAndRotate` | method | 1 |
| `net/minecraft/client/model/geom/ModelPart#visible` | field | 2 |
| `net/minecraft/client/model/geom/ModelPart#x` | field | 1 |
| `net/minecraft/client/model/geom/ModelPart#xRot` | field | 1 |
| `net/minecraft/client/model/geom/ModelPart#yRot` | field | 1 |
| `net/minecraft/client/model/geom/ModelPart#z` | field | 1 |
| `net/minecraft/client/model/geom/ModelPart#zRot` | field | 1 |
| `net/minecraft/client/model/geom/ModelPart$Cube#<init>` | method | 1 |
| `net/minecraft/client/model/geom/ModelPart$Cube#polygons` | field | 1 |
| `net/minecraft/client/model/geom/ModelPart$Polygon#<init>` | method | 1 |
| `net/minecraft/client/model/geom/ModelPart$Vertex#<init>` | method | 1 |
| `net/minecraft/client/model/geom/PartPose#ZERO` | field | 1 |
| `net/minecraft/client/model/geom/PartPose#offset` | method | 1 |
| `net/minecraft/client/model/geom/PartPose#offsetAndRotation` | method | 1 |
| `net/minecraft/client/model/geom/builders/CubeDeformation#<init>` | method | 1 |
| `net/minecraft/client/model/geom/builders/CubeListBuilder#addBox` | method | 2 |
| `net/minecraft/client/model/geom/builders/CubeListBuilder#create` | method | 1 |
| `net/minecraft/client/model/geom/builders/CubeListBuilder#mirror` | method | 1 |
| `net/minecraft/client/model/geom/builders/CubeListBuilder#texOffs` | method | 1 |
| `net/minecraft/client/model/geom/builders/LayerDefinition#create` | method | 1 |
| `net/minecraft/client/model/geom/builders/MeshDefinition#<init>` | method | 1 |
| `net/minecraft/client/model/geom/builders/MeshDefinition#getRoot` | method | 1 |
| `net/minecraft/client/model/geom/builders/PartDefinition#addOrReplaceChild` | method | 1 |
| `net/minecraft/client/model/geom/builders/UVPair#unpackU` | method | 1 |
| `net/minecraft/client/model/geom/builders/UVPair#unpackV` | method | 1 |
| `net/minecraft/client/model/monster/creeper/CreeperModel#<init>` | method | 2 |
| `net/minecraft/client/model/object/skull/SkullModel#<init>` | method | 1 |
| `net/minecraft/client/model/object/skull/SkullModel#renderType` | method | 1 |
| `net/minecraft/client/model/object/skull/SkullModelBase$State#animationPos` | field | 1 |
| `net/minecraft/client/model/object/skull/SkullModelBase$State#xRot` | field | 1 |
| `net/minecraft/client/model/object/skull/SkullModelBase$State#yRot` | field | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#addParticle` | method | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#dimensionType` | method | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#getBlockEntity` | method | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#getBlockState` | method | 2 |
| `net/minecraft/client/multiplayer/ClientLevel#getGameTime` | method | 2 |
| `net/minecraft/client/multiplayer/ClientLevel#getRandom` | method | 3 |
| `net/minecraft/client/multiplayer/ClientLevel#isDarkOutside` | method | 1 |
| `net/minecraft/client/multiplayer/ClientPacketListener#handleOpenSignEditor` | method | 1 |
| `net/minecraft/client/multiplayer/ClientPacketListener#send` | method | 1 |
| `net/minecraft/client/multiplayer/MultiPlayerGameMode#handleInventoryButtonClick` | method | 1 |
| `net/minecraft/client/particle/RisingParticle#<init>` | method | 3 |
| `net/minecraft/client/particle/RisingParticle#getLightCoords` | method | 3 |
| `net/minecraft/client/particle/RisingParticle#tick` | method | 3 |
| `net/minecraft/client/particle/SingleQuadParticle#<init>` | method | 1 |
| `net/minecraft/client/particle/SingleQuadParticle$Layer#TRANSLUCENT` | field | 4 |
| `net/minecraft/client/particle/SonicBoomParticle$Provider#<init>` | method | 1 |
| `net/minecraft/client/particle/SpriteSet#first` | method | 3 |
| `net/minecraft/client/particle/SpriteSet#get` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#blockPosition` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#containerMenu` | field | 1 |
| `net/minecraft/client/player/LocalPlayer#getActiveEffects` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#getMainHandItem` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#getOffhandItem` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#isScoping` | method | 1 |
| `net/minecraft/client/renderer/OrderedSubmitNodeCollector#submitCrumblingOverlay` | method | 1 |
| `net/minecraft/client/renderer/RenderPipelines#GUI_TEXTURED` | field | 5 |
| `net/minecraft/client/renderer/SubmitNodeCollector#order` | method | 1 |
| `net/minecraft/client/renderer/SubmitNodeCollector#submitBlockModel` | method | 1 |
| `net/minecraft/client/renderer/SubmitNodeCollector#submitBreakingBlockModel` | method | 1 |
| `net/minecraft/client/renderer/SubmitNodeCollector#submitCrumblingOverlay` | method | 1 |
| `net/minecraft/client/renderer/SubmitNodeCollector#submitCustomGeometry` | method | 1 |
| `net/minecraft/client/renderer/SubmitNodeCollector#submitFlame` | method | 1 |
| `net/minecraft/client/renderer/SubmitNodeCollector#submitGizmoPrimitives` | method | 1 |
| `net/minecraft/client/renderer/SubmitNodeCollector#submitLeash` | method | 1 |
| `net/minecraft/client/renderer/SubmitNodeCollector#submitModel` | method | 2 |
| `net/minecraft/client/renderer/SubmitNodeCollector#submitMovingBlock` | method | 1 |
| `net/minecraft/client/renderer/SubmitNodeCollector#submitNameTag` | method | 1 |
| `net/minecraft/client/renderer/SubmitNodeCollector#submitQuadParticleGroup` | method | 1 |
| `net/minecraft/client/renderer/SubmitNodeCollector#submitShadow` | method | 1 |
| `net/minecraft/client/renderer/SubmitNodeCollector#submitShapeOutline` | method | 1 |
| `net/minecraft/client/renderer/SubmitNodeCollector#submitText` | method | 2 |
| `net/minecraft/client/renderer/SubmitNodeCollector#submitTextBackground` | method | 1 |
| `net/minecraft/client/renderer/blockentity/BlockEntityRenderer#extractRenderState` | method | 2 |
| `net/minecraft/client/renderer/blockentity/BlockEntityRendererProvider$Context#font` | method | 1 |
| `net/minecraft/client/renderer/blockentity/BlockEntityRendererProvider$Context#itemModelResolver` | method | 2 |
| `net/minecraft/client/renderer/blockentity/state/BlockEntityRenderState#<init>` | method | 2 |
| `net/minecraft/client/renderer/entity/EntityRenderer#<init>` | method | 1 |
| `net/minecraft/client/renderer/entity/EntityRenderer#extractRenderState` | method | 1 |
| `net/minecraft/client/renderer/entity/EntityRenderer#submit` | method | 1 |
| `net/minecraft/client/renderer/entity/EntityRendererProvider$Context#bakeLayer` | method | 3 |
| `net/minecraft/client/renderer/entity/EntityRendererProvider$Context#getModelSet` | method | 2 |
| `net/minecraft/client/renderer/entity/EntityRendererProvider$Context#getPlayerSkinRenderCache` | method | 1 |
| `net/minecraft/client/renderer/entity/MobRenderer#<init>` | method | 2 |
| `net/minecraft/client/renderer/entity/MobRenderer#extractRenderState` | method | 2 |
| `net/minecraft/client/renderer/entity/layers/CustomHeadLayer#<init>` | method | 1 |
| `net/minecraft/client/renderer/entity/layers/EnergySwirlLayer#<init>` | method | 1 |
| `net/minecraft/client/renderer/entity/layers/EyesLayer#<init>` | method | 2 |
| `net/minecraft/client/renderer/entity/layers/ItemInHandLayer#<init>` | method | 1 |
| `net/minecraft/client/renderer/entity/layers/ItemInHandLayer#submit` | method | 1 |
| `net/minecraft/client/renderer/entity/state/ArmedEntityRenderState#extractArmedEntityRenderState` | method | 1 |
| `net/minecraft/client/renderer/entity/state/CreeperRenderState#<init>` | method | 1 |
| `net/minecraft/client/renderer/entity/state/CreeperRenderState#isPowered` | field | 2 |
| `net/minecraft/client/renderer/entity/state/CreeperRenderState#swelling` | field | 1 |
| `net/minecraft/client/renderer/entity/state/EntityRenderState#ageInTicks` | field | 1 |
| `net/minecraft/client/renderer/entity/state/EntityRenderState#outlineColor` | field | 1 |
| `net/minecraft/client/renderer/entity/state/IllagerRenderState#<init>` | method | 1 |
| `net/minecraft/client/renderer/entity/state/LivingEntityRenderState#addGeckolibData` | method | 3 |
| `net/minecraft/client/renderer/entity/state/LivingEntityRenderState#getGeckolibData` | method | 2 |
| `net/minecraft/client/renderer/entity/state/LivingEntityRenderState#getOrDefaultGeckolibData` | method | 2 |
| `net/minecraft/client/renderer/entity/state/LivingEntityRenderState#walkAnimationSpeed` | field | 1 |
| `net/minecraft/client/renderer/entity/state/WitherSkullRenderState#<init>` | method | 1 |
| `net/minecraft/client/renderer/entity/state/WitherSkullRenderState#lightCoords` | field | 1 |
| `net/minecraft/client/renderer/entity/state/WitherSkullRenderState#modelState` | field | 1 |
| `net/minecraft/client/renderer/entity/state/WitherSkullRenderState#outlineColor` | field | 1 |
| `net/minecraft/client/renderer/item/ItemModelResolver#updateForTopItem` | method | 3 |
| `net/minecraft/client/renderer/item/ItemStackRenderState#<init>` | method | 3 |
| `net/minecraft/client/renderer/item/ItemStackRenderState#clear` | method | 2 |
| `net/minecraft/client/renderer/item/properties/numeric/RangeSelectItemModelProperties#ID_MAPPER` | field | 1 |
| `net/minecraft/client/renderer/rendertype/RenderTypes#entityCutout` | method | 4 |
| `net/minecraft/client/renderer/rendertype/RenderTypes#entityTranslucent` | method | 4 |
| `net/minecraft/client/renderer/rendertype/RenderTypes#eyes` | method | 3 |
| `net/minecraft/client/renderer/texture/MissingTextureAtlasSprite#getLocation` | method | 2 |
| `net/minecraft/client/renderer/texture/OverlayTexture#NO_OVERLAY` | field | 4 |
| `net/minecraft/client/resources/model/geometry/BakedQuad#direction` | method | 1 |
| `net/minecraft/client/resources/model/geometry/BakedQuad#materialInfo` | method | 1 |
| `net/minecraft/client/resources/model/geometry/BakedQuad#packedUV` | method | 1 |
| `net/minecraft/client/resources/model/geometry/BakedQuad#position` | method | 1 |
| `net/minecraft/client/resources/model/geometry/BakedQuad$MaterialInfo#itemRenderType` | method | 1 |
| `net/minecraft/client/resources/model/geometry/ItemQuads#all` | method | 1 |
| `net/minecraft/client/resources/sounds/AbstractSoundInstance#<init>` | method | 1 |
| `net/minecraft/client/resources/sounds/EntityBoundSoundInstance#<init>` | method | 1 |
| `net/minecraft/client/resources/sounds/SoundInstance#getIdentifier` | method | 1 |
| `net/minecraft/client/resources/sounds/SoundInstance$Attenuation#NONE` | field | 1 |
| `net/minecraft/client/sounds/SoundEngine#calculateVolume` | method | 2 |
| `net/minecraft/client/sounds/SoundEngine#play` | method | 1 |
| `net/minecraft/client/sounds/SoundManager#play` | method | 3 |
| `net/minecraft/client/sounds/SoundManager#stop` | method | 1 |
| `net/minecraft/commands/CommandSource#NULL` | field | 1 |
| `net/minecraft/commands/CommandSourceStack#<init>` | method | 2 |
| `net/minecraft/commands/CommandSourceStack#getLevel` | method | 1 |
| `net/minecraft/commands/CommandSourceStack#getPlayerOrException` | method | 1 |
| `net/minecraft/commands/CommandSourceStack#getPosition` | method | 1 |
| `net/minecraft/commands/CommandSourceStack#sendSuccess` | method | 2 |
| `net/minecraft/commands/Commands#LEVEL_ADMINS` | field | 1 |
| `net/minecraft/commands/Commands#LEVEL_GAMEMASTERS` | field | 1 |
| `net/minecraft/commands/Commands#argument` | method | 1 |
| `net/minecraft/commands/Commands#hasPermission` | method | 2 |
| `net/minecraft/commands/Commands#literal` | method | 2 |
| `net/minecraft/commands/Commands#performPrefixedCommand` | method | 1 |
| `net/minecraft/core/BlockPos#<init>` | method | 11 |
| `net/minecraft/core/BlockPos#ZERO` | field | 2 |
| `net/minecraft/core/BlockPos#above` | method | 10 |
| `net/minecraft/core/BlockPos#asLong` | method | 3 |
| `net/minecraft/core/BlockPos#below` | method | 21 |
| `net/minecraft/core/BlockPos#betweenClosed` | method | 1 |
| `net/minecraft/core/BlockPos#closerThan` | method | 1 |
| `net/minecraft/core/BlockPos#closerToCenterThan` | method | 1 |
| `net/minecraft/core/BlockPos#containing` | method | 6 |
| `net/minecraft/core/BlockPos#distSqr` | method | 2 |
| `net/minecraft/core/BlockPos#distToCenterSqr` | method | 1 |
| `net/minecraft/core/BlockPos#equals` | method | 3 |
| `net/minecraft/core/BlockPos#getX` | method | 36 |
| `net/minecraft/core/BlockPos#getY` | method | 36 |
| `net/minecraft/core/BlockPos#getZ` | method | 36 |
| `net/minecraft/core/BlockPos#hashCode` | method | 1 |
| `net/minecraft/core/BlockPos#immutable` | method | 2 |
| `net/minecraft/core/BlockPos#mutable` | method | 1 |
| `net/minecraft/core/BlockPos#of` | method | 3 |
| `net/minecraft/core/BlockPos#offset` | method | 10 |
| `net/minecraft/core/BlockPos#relative` | method | 10 |
| `net/minecraft/core/BlockPos#subtract` | method | 1 |
| `net/minecraft/core/BlockPos$MutableBlockPos#<init>` | method | 7 |
| `net/minecraft/core/BlockPos$MutableBlockPos#getX` | method | 3 |
| `net/minecraft/core/BlockPos$MutableBlockPos#getY` | method | 1 |
| `net/minecraft/core/BlockPos$MutableBlockPos#getZ` | method | 3 |
| `net/minecraft/core/BlockPos$MutableBlockPos#immutable` | method | 1 |
| `net/minecraft/core/BlockPos$MutableBlockPos#move` | method | 1 |
| `net/minecraft/core/BlockPos$MutableBlockPos#set` | method | 5 |
| `net/minecraft/core/BlockPos$MutableBlockPos#setWithOffset` | method | 1 |
| `net/minecraft/core/BlockPos$MutableBlockPos#setX` | method | 2 |
| `net/minecraft/core/BlockPos$MutableBlockPos#setY` | method | 2 |
| `net/minecraft/core/BlockPos$MutableBlockPos#setZ` | method | 2 |
| `net/minecraft/core/DefaultedRegistry#byNameCodec` | method | 2 |
| `net/minecraft/core/DefaultedRegistry#getKey` | method | 4 |
| `net/minecraft/core/DefaultedRegistry#getOptional` | method | 1 |
| `net/minecraft/core/DefaultedRegistry#getTagOrEmpty` | method | 1 |
| `net/minecraft/core/Direction#DOWN` | field | 5 |
| `net/minecraft/core/Direction#EAST` | field | 5 |
| `net/minecraft/core/Direction#NORTH` | field | 17 |
| `net/minecraft/core/Direction#SOUTH` | field | 6 |
| `net/minecraft/core/Direction#UP` | field | 12 |
| `net/minecraft/core/Direction#WEST` | field | 3 |
| `net/minecraft/core/Direction#from2DDataValue` | method | 1 |
| `net/minecraft/core/Direction#get2DDataValue` | method | 1 |
| `net/minecraft/core/Direction#getAxis` | method | 2 |
| `net/minecraft/core/Direction#getClockWise` | method | 1 |
| `net/minecraft/core/Direction#getCounterClockWise` | method | 1 |
| `net/minecraft/core/Direction#getRandom` | method | 1 |
| `net/minecraft/core/Direction#getStepX` | method | 1 |
| `net/minecraft/core/Direction#getStepZ` | method | 1 |
| `net/minecraft/core/Direction#getUnitVec3f` | method | 1 |
| `net/minecraft/core/Direction#ordinal` | method | 8 |
| `net/minecraft/core/Direction#toYRot` | method | 3 |
| `net/minecraft/core/Direction#values` | method | 6 |
| `net/minecraft/core/Direction$Axis#Y` | field | 4 |
| `net/minecraft/core/Direction$Plane#HORIZONTAL` | field | 3 |
| `net/minecraft/core/Direction$Plane#getRandomDirection` | method | 1 |
| `net/minecraft/core/Direction$Plane#iterator` | method | 3 |
| `net/minecraft/core/Holder#is` | method | 1 |
| `net/minecraft/core/Holder#isBound` | method | 1 |
| `net/minecraft/core/Holder#value` | method | 3 |
| `net/minecraft/core/Holder$Reference#value` | method | 8 |
| `net/minecraft/core/HolderLookup$Provider#createSerializationContext` | method | 4 |
| `net/minecraft/core/NonNullList#get` | method | 2 |
| `net/minecraft/core/NonNullList#set` | method | 1 |
| `net/minecraft/core/NonNullList#size` | method | 1 |
| `net/minecraft/core/NonNullList#withSize` | method | 2 |
| `net/minecraft/core/Registry#getValue` | method | 1 |
| `net/minecraft/core/Registry#wrapAsHolder` | method | 1 |
| `net/minecraft/core/RegistryAccess#EMPTY` | field | 1 |
| `net/minecraft/core/UUIDUtil#CODEC` | field | 5 |
| `net/minecraft/core/Vec3i#getY` | method | 1 |
| `net/minecraft/core/component/DataComponentMap$Builder#set` | method | 2 |
| `net/minecraft/core/component/DataComponents#CUSTOM_DATA` | field | 1 |
| `net/minecraft/core/component/DataComponents#CUSTOM_NAME` | field | 2 |
| `net/minecraft/core/component/DataComponents#DYE` | field | 1 |
| `net/minecraft/core/component/DataComponents#MAP_DECORATIONS` | field | 1 |
| `net/minecraft/core/component/DataComponents#WRITABLE_BOOK_CONTENT` | field | 1 |
| `net/minecraft/core/component/DataComponents#WRITTEN_BOOK_CONTENT` | field | 1 |
| `net/minecraft/core/particles/BlockParticleOption#<init>` | method | 5 |
| `net/minecraft/core/particles/DustParticleOptions#REDSTONE` | field | 1 |
| `net/minecraft/core/particles/ParticleTypes#ASH` | field | 5 |
| `net/minecraft/core/particles/ParticleTypes#BLOCK` | field | 5 |
| `net/minecraft/core/particles/ParticleTypes#CAMPFIRE_COSY_SMOKE` | field | 1 |
| `net/minecraft/core/particles/ParticleTypes#FLAME` | field | 1 |
| `net/minecraft/core/particles/ParticleTypes#LARGE_SMOKE` | field | 2 |
| `net/minecraft/core/particles/ParticleTypes#LAVA` | field | 2 |
| `net/minecraft/core/particles/ParticleTypes#SMOKE` | field | 4 |
| `net/minecraft/core/particles/ParticleTypes#SOUL` | field | 5 |
| `net/minecraft/core/particles/ParticleTypes#SOUL_FIRE_FLAME` | field | 4 |
| `net/minecraft/core/particles/ParticleTypes#WHITE_ASH` | field | 1 |
| `net/minecraft/core/particles/SimpleParticleType#<init>` | method | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#BLOCK_ENTITY_TYPE` | field | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#CREATIVE_MODE_TAB` | field | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#DATA_COMPONENT_TYPE` | field | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#ENTITY_TYPE` | field | 4 |
| `net/minecraft/core/registries/BuiltInRegistries#LOOT_CONDITION_TYPE` | field | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#MENU` | field | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#PARTICLE_TYPE` | field | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#RECIPE_SERIALIZER` | field | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#RECIPE_TYPE` | field | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#SOUND_EVENT` | field | 3 |
| `net/minecraft/core/registries/BuiltInRegistries#STRUCTURE_PROCESSOR` | field | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#STRUCTURE_TYPE` | field | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#TRIGGER_TYPES` | field | 1 |
| `net/minecraft/core/registries/Registries#BIOME` | field | 3 |
| `net/minecraft/core/registries/Registries#BLOCK` | field | 3 |
| `net/minecraft/core/registries/Registries#CREATIVE_MODE_TAB` | field | 1 |
| `net/minecraft/core/registries/Registries#DIMENSION` | field | 1 |
| `net/minecraft/core/registries/Registries#ENTITY_TYPE` | field | 1 |
| `net/minecraft/core/registries/Registries#ITEM` | field | 2 |
| `net/minecraft/core/registries/Registries#JUKEBOX_SONG` | field | 1 |
| `net/minecraft/core/registries/Registries#STRUCTURE` | field | 1 |
| `net/minecraft/core/registries/Registries#TEMPLATE_POOL` | field | 1 |
| `net/minecraft/data/worldgen/features/CaveFeatures#MOSS_PATCH_BONEMEAL` | field | 1 |
| `net/minecraft/nbt/CompoundTag#<init>` | method | 6 |
| `net/minecraft/nbt/CompoundTag#CODEC` | field | 6 |
| `net/minecraft/nbt/CompoundTag#contains` | method | 5 |
| `net/minecraft/nbt/CompoundTag#copy` | method | 6 |
| `net/minecraft/nbt/CompoundTag#equals` | method | 1 |
| `net/minecraft/nbt/CompoundTag#get` | method | 3 |
| `net/minecraft/nbt/CompoundTag#getBooleanOr` | method | 1 |
| `net/minecraft/nbt/CompoundTag#getByteOr` | method | 1 |
| `net/minecraft/nbt/CompoundTag#getCompound` | method | 1 |
| `net/minecraft/nbt/CompoundTag#getCompoundOrEmpty` | method | 4 |
| `net/minecraft/nbt/CompoundTag#getDoubleOr` | method | 1 |
| `net/minecraft/nbt/CompoundTag#getFloatOr` | method | 1 |
| `net/minecraft/nbt/CompoundTag#getIntArray` | method | 1 |
| `net/minecraft/nbt/CompoundTag#getIntOr` | method | 3 |
| `net/minecraft/nbt/CompoundTag#getList` | method | 1 |
| `net/minecraft/nbt/CompoundTag#getListOrEmpty` | method | 5 |
| `net/minecraft/nbt/CompoundTag#getLongOr` | method | 1 |
| `net/minecraft/nbt/CompoundTag#getString` | method | 1 |
| `net/minecraft/nbt/CompoundTag#getStringOr` | method | 5 |
| `net/minecraft/nbt/CompoundTag#isEmpty` | method | 2 |
| `net/minecraft/nbt/CompoundTag#keySet` | method | 1 |
| `net/minecraft/nbt/CompoundTag#put` | method | 5 |
| `net/minecraft/nbt/CompoundTag#putBoolean` | method | 1 |
| `net/minecraft/nbt/CompoundTag#putDouble` | method | 1 |
| `net/minecraft/nbt/CompoundTag#putFloat` | method | 1 |
| `net/minecraft/nbt/CompoundTag#putInt` | method | 2 |
| `net/minecraft/nbt/CompoundTag#putLong` | method | 1 |
| `net/minecraft/nbt/CompoundTag#putString` | method | 4 |
| `net/minecraft/nbt/CompoundTag#read` | method | 2 |
| `net/minecraft/nbt/CompoundTag#remove` | method | 4 |
| `net/minecraft/nbt/CompoundTag#store` | method | 2 |
| `net/minecraft/nbt/IntArrayTag#<init>` | method | 1 |
| `net/minecraft/nbt/ListTag#<init>` | method | 4 |
| `net/minecraft/nbt/ListTag#add` | method | 4 |
| `net/minecraft/nbt/ListTag#addAll` | method | 2 |
| `net/minecraft/nbt/ListTag#get` | method | 1 |
| `net/minecraft/nbt/ListTag#getCompoundOrEmpty` | method | 3 |
| `net/minecraft/nbt/ListTag#isEmpty` | method | 1 |
| `net/minecraft/nbt/ListTag#iterator` | method | 2 |
| `net/minecraft/nbt/ListTag#remove` | method | 1 |
| `net/minecraft/nbt/ListTag#set` | method | 1 |
| `net/minecraft/nbt/ListTag#size` | method | 4 |
| `net/minecraft/nbt/NbtAccounter#create` | method | 1 |
| `net/minecraft/nbt/NbtAccounter#unlimitedHeap` | method | 1 |
| `net/minecraft/nbt/NbtIo#read` | method | 1 |
| `net/minecraft/nbt/NbtIo#readCompressed` | method | 1 |
| `net/minecraft/nbt/NbtIo#write` | method | 1 |
| `net/minecraft/nbt/NbtOps#INSTANCE` | field | 4 |
| `net/minecraft/nbt/StringTag#valueOf` | method | 1 |
| `net/minecraft/nbt/Tag#asString` | method | 3 |
| `net/minecraft/nbt/Tag#copy` | method | 1 |
| `net/minecraft/network/FriendlyByteBuf#readBoolean` | method | 1 |
| `net/minecraft/network/FriendlyByteBuf#readUtf` | method | 1 |
| `net/minecraft/network/FriendlyByteBuf#readVarInt` | method | 1 |
| `net/minecraft/network/FriendlyByteBuf#writeBoolean` | method | 1 |
| `net/minecraft/network/FriendlyByteBuf#writeUtf` | method | 1 |
| `net/minecraft/network/FriendlyByteBuf#writeVarInt` | method | 1 |
| `net/minecraft/network/chat/ClickEvent$RunCommand#command` | method | 1 |
| `net/minecraft/network/chat/CommonComponents#EMPTY` | field | 1 |
| `net/minecraft/network/chat/CommonComponents#GUI_DONE` | field | 2 |
| `net/minecraft/network/chat/Component#empty` | method | 1 |
| `net/minecraft/network/chat/Component#equals` | method | 1 |
| `net/minecraft/network/chat/Component#getContents` | method | 1 |
| `net/minecraft/network/chat/Component#getString` | method | 3 |
| `net/minecraft/network/chat/Component#getStyle` | method | 1 |
| `net/minecraft/network/chat/Component#literal` | method | 7 |
| `net/minecraft/network/chat/Component#translatable` | method | 20 |
| `net/minecraft/network/chat/ComponentSerialization#CODEC` | field | 5 |
| `net/minecraft/network/chat/ComponentUtils#resolve` | method | 1 |
| `net/minecraft/network/chat/MutableComponent#setStyle` | method | 1 |
| `net/minecraft/network/chat/ResolutionContext#create` | method | 1 |
| `net/minecraft/network/chat/Style#getClickEvent` | method | 1 |
| `net/minecraft/network/codec/ByteBufCodecs#STRING_UTF8` | field | 1 |
| `net/minecraft/network/codec/ByteBufCodecs#VAR_INT` | field | 1 |
| `net/minecraft/network/codec/StreamCodec#apply` | method | 1 |
| `net/minecraft/network/codec/StreamCodec#composite` | method | 2 |
| `net/minecraft/network/codec/StreamCodec#of` | method | 1 |
| `net/minecraft/network/protocol/PacketUtils#ensureRunningOnSameThread` | method | 1 |
| `net/minecraft/network/protocol/common/ClientboundCustomPayloadPacket#<init>` | method | 1 |
| `net/minecraft/network/protocol/common/custom/CustomPacketPayload$Type#<init>` | method | 2 |
| `net/minecraft/network/protocol/game/ClientboundBlockEntityDataPacket#create` | method | 2 |
| `net/minecraft/network/protocol/game/ClientboundContainerSetSlotPacket#<init>` | method | 1 |
| `net/minecraft/network/protocol/game/ClientboundOpenSignEditorPacket#<init>` | method | 1 |
| `net/minecraft/network/protocol/game/ClientboundOpenSignEditorPacket#pos` | method | 1 |
| `net/minecraft/network/protocol/game/ClientboundSoundPacket#<init>` | method | 1 |
| `net/minecraft/network/protocol/game/ClientboundStopSoundPacket#<init>` | method | 1 |
| `net/minecraft/network/protocol/game/ServerboundSignUpdatePacket#<init>` | method | 1 |
| `net/minecraft/network/protocol/game/ServerboundSignUpdatePacket#pos` | method | 1 |
| `net/minecraft/network/syncher/EntityDataSerializers#BOOLEAN` | field | 12 |
| `net/minecraft/network/syncher/EntityDataSerializers#BYTE` | field | 8 |
| `net/minecraft/network/syncher/EntityDataSerializers#INT` | field | 9 |
| `net/minecraft/network/syncher/EntityDataSerializers#ITEM_STACK` | field | 1 |
| `net/minecraft/network/syncher/EntityDataSerializers#STRING` | field | 1 |
| `net/minecraft/network/syncher/SynchedEntityData#defineId` | method | 16 |
| `net/minecraft/network/syncher/SynchedEntityData#get` | method | 16 |
| `net/minecraft/network/syncher/SynchedEntityData#set` | method | 16 |
| `net/minecraft/network/syncher/SynchedEntityData$Builder#define` | method | 16 |
| `net/minecraft/resources/FileToIdConverter#fileToId` | method | 1 |
| `net/minecraft/resources/FileToIdConverter#json` | method | 2 |
| `net/minecraft/resources/FileToIdConverter#listMatchingResources` | method | 1 |
| `net/minecraft/resources/Identifier#CODEC` | field | 2 |
| `net/minecraft/resources/Identifier#fromNamespaceAndPath` | method | 3 |
| `net/minecraft/resources/Identifier#getNamespace` | method | 5 |
| `net/minecraft/resources/Identifier#getPath` | method | 6 |
| `net/minecraft/resources/Identifier#toString` | method | 3 |
| `net/minecraft/resources/Identifier#tryParse` | method | 2 |
| `net/minecraft/resources/Identifier#withDefaultNamespace` | method | 6 |
| `net/minecraft/resources/ResourceKey#create` | method | 5 |
| `net/minecraft/resources/ResourceKey#identifier` | method | 5 |
| `net/minecraft/server/MinecraftServer#getAdvancements` | method | 1 |
| `net/minecraft/server/MinecraftServer#getAllLevels` | method | 1 |
| `net/minecraft/server/MinecraftServer#getCommands` | method | 1 |
| `net/minecraft/server/MinecraftServer#getDataStorage` | method | 2 |
| `net/minecraft/server/MinecraftServer#getLevel` | method | 1 |
| `net/minecraft/server/MinecraftServer#getWorldPath` | method | 2 |
| `net/minecraft/server/PlayerAdvancements#getOrStartProgress` | method | 1 |
| `net/minecraft/server/ServerAdvancementManager#get` | method | 1 |
| `net/minecraft/server/level/ServerBossEvent#<init>` | method | 1 |
| `net/minecraft/server/level/ServerBossEvent#addPlayer` | method | 1 |
| `net/minecraft/server/level/ServerBossEvent#removePlayer` | method | 1 |
| `net/minecraft/server/level/ServerBossEvent#setDarkenScreen` | method | 1 |
| `net/minecraft/server/level/ServerBossEvent#setName` | method | 1 |
| `net/minecraft/server/level/ServerBossEvent#setProgress` | method | 1 |
| `net/minecraft/server/level/ServerChunkCache#addTicketWithRadius` | method | 1 |
| `net/minecraft/server/level/ServerLevel#addFreshEntity` | method | 2 |
| `net/minecraft/server/level/ServerLevel#addFreshEntityWithPassengers` | method | 2 |
| `net/minecraft/server/level/ServerLevel#containsAnyLiquid` | method | 1 |
| `net/minecraft/server/level/ServerLevel#dimension` | method | 2 |
| `net/minecraft/server/level/ServerLevel#environmentAttributes` | method | 1 |
| `net/minecraft/server/level/ServerLevel#getBlockEntity` | method | 3 |
| `net/minecraft/server/level/ServerLevel#getBlockState` | method | 3 |
| `net/minecraft/server/level/ServerLevel#getChunkSource` | method | 1 |
| `net/minecraft/server/level/ServerLevel#getCurrentDifficultyAt` | method | 2 |
| `net/minecraft/server/level/ServerLevel#getDifficulty` | method | 1 |
| `net/minecraft/server/level/ServerLevel#getEntitiesOfClass` | method | 2 |
| `net/minecraft/server/level/ServerLevel#getEntity` | method | 1 |
| `net/minecraft/server/level/ServerLevel#getFluidState` | method | 1 |
| `net/minecraft/server/level/ServerLevel#getGameRules` | method | 3 |
| `net/minecraft/server/level/ServerLevel#getGameTime` | method | 3 |
| `net/minecraft/server/level/ServerLevel#getHeightmapPos` | method | 2 |
| `net/minecraft/server/level/ServerLevel#getNearbyEntities` | method | 1 |
| `net/minecraft/server/level/ServerLevel#getNearestPlayer` | method | 1 |
| `net/minecraft/server/level/ServerLevel#getPlayers` | method | 3 |
| `net/minecraft/server/level/ServerLevel#getRandom` | method | 4 |
| `net/minecraft/server/level/ServerLevel#getSeed` | method | 1 |
| `net/minecraft/server/level/ServerLevel#getServer` | method | 5 |
| `net/minecraft/server/level/ServerLevel#hasChunk` | method | 1 |
| `net/minecraft/server/level/ServerLevel#isCloseToVillage` | method | 1 |
| `net/minecraft/server/level/ServerLevel#isDarkOutside` | method | 2 |
| `net/minecraft/server/level/ServerLevel#isPvpAllowed` | method | 1 |
| `net/minecraft/server/level/ServerLevel#isRaided` | method | 1 |
| `net/minecraft/server/level/ServerLevel#isSpawningMonsters` | method | 1 |
| `net/minecraft/server/level/ServerLevel#playSound` | method | 2 |
| `net/minecraft/server/level/ServerLevel#players` | method | 1 |
| `net/minecraft/server/level/ServerLevel#recipeAccess` | method | 2 |
| `net/minecraft/server/level/ServerLevel#registryAccess` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#addItem` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#closeContainer` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#connection` | field | 5 |
| `net/minecraft/server/level/ServerPlayer#containerMenu` | field | 1 |
| `net/minecraft/server/level/ServerPlayer#distanceToSqr` | method | 3 |
| `net/minecraft/server/level/ServerPlayer#getAbilities` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#getAdvancements` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#getItemInHand` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#getMainHandItem` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#getRecipeBook` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#getUUID` | method | 2 |
| `net/minecraft/server/level/ServerPlayer#getX` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#getY` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#getZ` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#isSpectator` | method | 3 |
| `net/minecraft/server/level/ServerPlayer#level` | method | 5 |
| `net/minecraft/server/level/ServerPlayer#resetLastActionTime` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#sendOverlayMessage` | method | 2 |
| `net/minecraft/server/level/ServerPlayer#sendSystemMessage` | method | 1 |
| `net/minecraft/server/level/TicketType#<init>` | method | 1 |
| `net/minecraft/server/network/Filterable#raw` | method | 1 |
| `net/minecraft/server/network/FilteredText#filteredOrEmpty` | method | 1 |
| `net/minecraft/server/network/FilteredText#raw` | method | 1 |
| `net/minecraft/server/network/ServerGamePacketListenerImpl#player` | field | 1 |
| `net/minecraft/server/network/ServerGamePacketListenerImpl#send` | method | 5 |
| `net/minecraft/server/network/ServerGamePacketListenerImpl#updateSignText` | method | 1 |
| `net/minecraft/server/packs/PackType#SERVER_DATA` | field | 1 |
| `net/minecraft/server/packs/resources/Resource#open` | method | 1 |
| `net/minecraft/server/packs/resources/Resource#openAsReader` | method | 1 |
| `net/minecraft/server/packs/resources/ResourceManager#listResources` | method | 1 |
| `net/minecraft/server/packs/resources/SimpleJsonResourceReloadListener#<init>` | method | 1 |
| `net/minecraft/server/permissions/LevelBasedPermissionSet#GAMEMASTER` | field | 1 |
| `net/minecraft/server/players/OldUsersConverter#convertMobOwnerIfNecessary` | method | 1 |
| `net/minecraft/sounds/SoundEvent#createVariableRangeEvent` | method | 1 |
| `net/minecraft/sounds/SoundEvent#location` | method | 1 |
| `net/minecraft/sounds/SoundEvents#AXE_STRIP` | field | 1 |
| `net/minecraft/sounds/SoundEvents#BOOK_PAGE_TURN` | field | 1 |
| `net/minecraft/sounds/SoundEvents#CAMPFIRE_CRACKLE` | field | 3 |
| `net/minecraft/sounds/SoundEvents#CANDLE_EXTINGUISH` | field | 2 |
| `net/minecraft/sounds/SoundEvents#CHAIN_BREAK` | field | 1 |
| `net/minecraft/sounds/SoundEvents#CHEST_CLOSE` | field | 1 |
| `net/minecraft/sounds/SoundEvents#CHORUS_FRUIT_TELEPORT` | field | 1 |
| `net/minecraft/sounds/SoundEvents#DYE_USE` | field | 1 |
| `net/minecraft/sounds/SoundEvents#ENDERMAN_TELEPORT` | field | 1 |
| `net/minecraft/sounds/SoundEvents#END_PORTAL_SPAWN` | field | 1 |
| `net/minecraft/sounds/SoundEvents#FIRECHARGE_USE` | field | 1 |
| `net/minecraft/sounds/SoundEvents#FIRE_EXTINGUISH` | field | 2 |
| `net/minecraft/sounds/SoundEvents#FLINTANDSTEEL_USE` | field | 1 |
| `net/minecraft/sounds/SoundEvents#GENERIC_SMALL_FALL` | field | 1 |
| `net/minecraft/sounds/SoundEvents#GLOW_INK_SAC_USE` | field | 1 |
| `net/minecraft/sounds/SoundEvents#HOSTILE_BIG_FALL` | field | 1 |
| `net/minecraft/sounds/SoundEvents#INK_SAC_USE` | field | 1 |
| `net/minecraft/sounds/SoundEvents#SOUL_ESCAPE` | field | 7 |
| `net/minecraft/sounds/SoundEvents#UI_STONECUTTER_SELECT_RECIPE` | field | 1 |
| `net/minecraft/sounds/SoundEvents#WARDEN_SONIC_BOOM` | field | 1 |
| `net/minecraft/sounds/SoundEvents#WAXED_SIGN_INTERACT_FAIL` | field | 1 |
| `net/minecraft/sounds/SoundSource#BLOCKS` | field | 12 |
| `net/minecraft/sounds/SoundSource#HOSTILE` | field | 6 |
| `net/minecraft/sounds/SoundSource#MUSIC` | field | 2 |
| `net/minecraft/sounds/SoundSource#PLAYERS` | field | 2 |
| `net/minecraft/stats/ServerRecipeBook#contains` | method | 1 |
| `net/minecraft/stats/StatType#get` | method | 1 |
| `net/minecraft/stats/Stats#ITEM_USED` | field | 1 |
| `net/minecraft/tags/BiomeTags#IS_OVERWORLD` | field | 1 |
| `net/minecraft/tags/BlockItemTagId#block` | method | 2 |
| `net/minecraft/tags/BlockItemTags#LOGS_THAT_BURN` | field | 2 |
| `net/minecraft/tags/BlockTags#CANDLES` | field | 1 |
| `net/minecraft/tags/BlockTags#CROPS` | field | 3 |
| `net/minecraft/tags/BlockTags#ENDERMAN_DOES_NOT_TELEPORT_TO` | field | 1 |
| `net/minecraft/tags/BlockTags#FENCES` | field | 1 |
| `net/minecraft/tags/BlockTags#FENCE_GATES` | field | 1 |
| `net/minecraft/tags/BlockTags#LANTERNS` | field | 1 |
| `net/minecraft/tags/BlockTags#LOGS` | field | 1 |
| `net/minecraft/tags/BlockTags#PLANKS` | field | 2 |
| `net/minecraft/tags/BlockTags#SWORD_EFFICIENT` | field | 1 |
| `net/minecraft/tags/BlockTags#WOODEN_FENCES` | field | 1 |
| `net/minecraft/tags/BlockTags#WOODEN_SLABS` | field | 1 |
| `net/minecraft/tags/BlockTags#WOODEN_STAIRS` | field | 1 |
| `net/minecraft/tags/BlockTags#WOODEN_TRAPDOORS` | field | 1 |
| `net/minecraft/tags/DamageTypeTags#BYPASSES_ARMOR` | field | 1 |
| `net/minecraft/tags/DamageTypeTags#BYPASSES_EFFECTS` | field | 1 |
| `net/minecraft/tags/DamageTypeTags#BYPASSES_INVULNERABILITY` | field | 4 |
| `net/minecraft/tags/DamageTypeTags#BYPASSES_RESISTANCE` | field | 3 |
| `net/minecraft/tags/DamageTypeTags#BYPASSES_SHIELD` | field | 1 |
| `net/minecraft/tags/DamageTypeTags#IS_FIRE` | field | 1 |
| `net/minecraft/tags/DamageTypeTags#IS_PROJECTILE` | field | 2 |
| `net/minecraft/tags/EntityTypeTags#ILLAGER` | field | 1 |
| `net/minecraft/tags/FluidTags#LAVA` | field | 1 |
| `net/minecraft/tags/FluidTags#WATER` | field | 2 |
| `net/minecraft/tags/ItemTags#AXES` | field | 1 |
| `net/minecraft/tags/ItemTags#SHOVELS` | field | 1 |
| `net/minecraft/tags/TagKey#codec` | method | 1 |
| `net/minecraft/tags/TagKey#create` | method | 4 |
| `net/minecraft/util/ARGB#blue` | method | 1 |
| `net/minecraft/util/ARGB#color` | method | 1 |
| `net/minecraft/util/ARGB#green` | method | 1 |
| `net/minecraft/util/ARGB#red` | method | 1 |
| `net/minecraft/util/ExtraCodecs#JSON` | field | 1 |
| `net/minecraft/util/ExtraCodecs#NBT` | field | 1 |
| `net/minecraft/util/ExtraCodecs#POSITIVE_INT` | field | 1 |
| `net/minecraft/util/ExtraCodecs$LateBoundIdMapper#put` | method | 1 |
| `net/minecraft/util/FormattedCharSequence#EMPTY` | field | 2 |
| `net/minecraft/util/GsonHelper#getAsInt` | method | 1 |
| `net/minecraft/util/GsonHelper#getAsJsonArray` | method | 1 |
| `net/minecraft/util/GsonHelper#getAsString` | method | 1 |
| `net/minecraft/util/Mth#cos` | method | 5 |
| `net/minecraft/util/Mth#getSeed` | method | 2 |
| `net/minecraft/util/Mth#lerpInt` | method | 1 |
| `net/minecraft/util/Mth#randomBetween` | method | 2 |
| `net/minecraft/util/Mth#sin` | method | 5 |
| `net/minecraft/util/Mth#sqrt` | method | 2 |
| `net/minecraft/util/Mth#square` | method | 1 |
| `net/minecraft/util/Prediction#SERVER_ONLY` | field | 1 |
| `net/minecraft/util/ProblemReporter#DISCARDING` | field | 2 |
| `net/minecraft/util/RandomSource#create` | method | 1 |
| `net/minecraft/util/RandomSource#forkPositional` | method | 1 |
| `net/minecraft/util/RandomSource#nextBoolean` | method | 9 |
| `net/minecraft/util/RandomSource#nextDouble` | method | 16 |
| `net/minecraft/util/RandomSource#nextFloat` | method | 25 |
| `net/minecraft/util/RandomSource#nextInt` | method | 45 |
| `net/minecraft/util/RandomSource#nextIntBetweenInclusive` | method | 1 |
| `net/minecraft/util/RandomSource#nextLong` | method | 3 |
| `net/minecraft/util/StringRepresentable#fromEnum` | method | 1 |
| `net/minecraft/util/StringRepresentable$EnumCodec#optionalFieldOf` | method | 1 |
| `net/minecraft/util/TimeUtil#rangeOfSeconds` | method | 2 |
| `net/minecraft/util/datafix/DataFixTypes#ENTITY_CHUNK` | field | 1 |
| `net/minecraft/util/datafix/DataFixTypes#LEVEL` | field | 2 |
| `net/minecraft/util/datafix/DataFixers#getDataFixer` | method | 3 |
| `net/minecraft/util/datafix/fixes/References#BLOCK_ENTITY` | field | 2 |
| `net/minecraft/util/datafix/fixes/References#ITEM_STACK` | field | 1 |
| `net/minecraft/util/filefix/fixes/LevelDatToSavedDataFileFix#extractWorldGenSettingsToFile` | method | 1 |
| `net/minecraft/util/valueproviders/ConstantInt#of` | method | 1 |
| `net/minecraft/util/valueproviders/UniformInt#of` | method | 1 |
| `net/minecraft/util/valueproviders/UniformInt#sample` | method | 2 |
| `net/minecraft/world/BossEvent#setCreateWorldFog` | method | 1 |
| `net/minecraft/world/BossEvent$BossBarColor#WHITE` | field | 1 |
| `net/minecraft/world/BossEvent$BossBarOverlay#PROGRESS` | field | 1 |
| `net/minecraft/world/CompoundContainer#contains` | method | 2 |
| `net/minecraft/world/Container#getContainerSize` | method | 1 |
| `net/minecraft/world/Container#getItem` | method | 2 |
| `net/minecraft/world/Container#removeItemNoUpdate` | method | 1 |
| `net/minecraft/world/Container#setItem` | method | 1 |
| `net/minecraft/world/ContainerHelper#saveAllItems` | method | 1 |
| `net/minecraft/world/Containers#updateNeighboursAfterDestroy` | method | 1 |
| `net/minecraft/world/Difficulty#PEACEFUL` | field | 2 |
| `net/minecraft/world/InteractionHand#MAIN_HAND` | field | 3 |
| `net/minecraft/world/InteractionHand#asEquipmentSlot` | method | 2 |
| `net/minecraft/world/InteractionResult#CONSUME` | field | 6 |
| `net/minecraft/world/InteractionResult#FAIL` | field | 1 |
| `net/minecraft/world/InteractionResult#PASS` | field | 6 |
| `net/minecraft/world/InteractionResult#SUCCESS` | field | 16 |
| `net/minecraft/world/InteractionResult#TRY_WITH_EMPTY_HAND` | field | 11 |
| `net/minecraft/world/SimpleContainer#<init>` | method | 2 |
| `net/minecraft/world/SimpleContainer#setChanged` | method | 1 |
| `net/minecraft/world/SimpleMenuProvider#<init>` | method | 1 |
| `net/minecraft/world/attribute/EnvironmentAttributeReader#getValue` | method | 1 |
| `net/minecraft/world/attribute/EnvironmentAttributeSystem#getValue` | method | 2 |
| `net/minecraft/world/attribute/EnvironmentAttributes#CAN_PILLAGER_PATROL_SPAWN` | field | 2 |
| `net/minecraft/world/attribute/EnvironmentAttributes#MONSTERS_BURN` | field | 1 |
| `net/minecraft/world/damagesource/DamageSource#getDirectEntity` | method | 1 |
| `net/minecraft/world/damagesource/DamageSource#getEntity` | method | 2 |
| `net/minecraft/world/damagesource/DamageSource#is` | method | 7 |
| `net/minecraft/world/damagesource/DamageSources#fall` | method | 1 |
| `net/minecraft/world/effect/MobEffectInstance#<init>` | method | 6 |
| `net/minecraft/world/effect/MobEffectInstance#CODEC` | field | 1 |
| `net/minecraft/world/effect/MobEffectInstance#getEffect` | method | 1 |
| `net/minecraft/world/effect/MobEffectInstance#is` | method | 1 |
| `net/minecraft/world/effect/MobEffects#BLINDNESS` | field | 4 |
| `net/minecraft/world/effect/MobEffects#DARKNESS` | field | 1 |
| `net/minecraft/world/effect/MobEffects#LEVITATION` | field | 1 |
| `net/minecraft/world/effect/MobEffects#MINING_FATIGUE` | field | 1 |
| `net/minecraft/world/effect/MobEffects#WITHER` | field | 2 |
| `net/minecraft/world/entity/AgeableMob#createLivingAttributes` | method | 1 |
| `net/minecraft/world/entity/AreaEffectCloud#<init>` | method | 1 |
| `net/minecraft/world/entity/AreaEffectCloud#addEffect` | method | 1 |
| `net/minecraft/world/entity/AreaEffectCloud#getDuration` | method | 1 |
| `net/minecraft/world/entity/AreaEffectCloud#getRadius` | method | 1 |
| `net/minecraft/world/entity/AreaEffectCloud#setDuration` | method | 1 |
| `net/minecraft/world/entity/AreaEffectCloud#setRadius` | method | 1 |
| `net/minecraft/world/entity/AreaEffectCloud#setRadiusOnUse` | method | 1 |
| `net/minecraft/world/entity/AreaEffectCloud#setRadiusPerTick` | method | 1 |
| `net/minecraft/world/entity/AreaEffectCloud#setWaitTime` | method | 1 |
| `net/minecraft/world/entity/ContainerUser#getContainerInteractionRange` | method | 2 |
| `net/minecraft/world/entity/ContainerUser#getLivingEntity` | method | 2 |
| `net/minecraft/world/entity/Entity#distanceToSqr` | method | 1 |
| `net/minecraft/world/entity/Entity#getBbWidth` | method | 1 |
| `net/minecraft/world/entity/Entity#getCustomName` | method | 1 |
| `net/minecraft/world/entity/Entity#getEyeY` | method | 1 |
| `net/minecraft/world/entity/Entity#getSoundSource` | method | 2 |
| `net/minecraft/world/entity/Entity#getTeam` | method | 2 |
| `net/minecraft/world/entity/Entity#getType` | method | 1 |
| `net/minecraft/world/entity/Entity#getUUID` | method | 1 |
| `net/minecraft/world/entity/Entity#hurtServer` | method | 1 |
| `net/minecraft/world/entity/Entity#is` | method | 1 |
| `net/minecraft/world/entity/Entity#isAlive` | method | 2 |
| `net/minecraft/world/entity/Entity#isRemoved` | method | 1 |
| `net/minecraft/world/entity/Entity#level` | method | 6 |
| `net/minecraft/world/entity/Entity#tickCount` | field | 3 |
| `net/minecraft/world/entity/Entity$RemovalReason#DISCARDED` | field | 1 |
| `net/minecraft/world/entity/Entity$RemovalReason#KILLED` | field | 9 |
| `net/minecraft/world/entity/Entity$RemovalReason#shouldDestroy` | method | 2 |
| `net/minecraft/world/entity/EntityDimensions#fixed` | method | 1 |
| `net/minecraft/world/entity/EntityDimensions#makeBoundingBox` | method | 1 |
| `net/minecraft/world/entity/EntityDimensions#withEyeHeight` | method | 1 |
| `net/minecraft/world/entity/EntityReference#getLivingEntity` | method | 1 |
| `net/minecraft/world/entity/EntityReference#getUUID` | method | 2 |
| `net/minecraft/world/entity/EntityReference#of` | method | 1 |
| `net/minecraft/world/entity/EntitySelector#NO_CREATIVE_OR_SPECTATOR` | field | 4 |
| `net/minecraft/world/entity/EntitySpawnReason#BUCKET` | field | 1 |
| `net/minecraft/world/entity/EntitySpawnReason#CHUNK_GENERATION` | field | 2 |
| `net/minecraft/world/entity/EntitySpawnReason#COMMAND` | field | 1 |
| `net/minecraft/world/entity/EntitySpawnReason#EVENT` | field | 2 |
| `net/minecraft/world/entity/EntitySpawnReason#NATURAL` | field | 1 |
| `net/minecraft/world/entity/EntitySpawnReason#PATROL` | field | 4 |
| `net/minecraft/world/entity/EntitySpawnReason#REINFORCEMENT` | field | 1 |
| `net/minecraft/world/entity/EntitySpawnReason#SPAWNER` | field | 2 |
| `net/minecraft/world/entity/EntitySpawnReason#SPAWN_ITEM_USE` | field | 2 |
| `net/minecraft/world/entity/EntitySpawnReason#STRUCTURE` | field | 5 |
| `net/minecraft/world/entity/EntitySpawnReason#TRIGGERED` | field | 5 |
| `net/minecraft/world/entity/EntitySpawnRequest#<init>` | method | 1 |
| `net/minecraft/world/entity/EntityType#getDescription` | method | 1 |
| `net/minecraft/world/entity/EntityType#toString` | method | 1 |
| `net/minecraft/world/entity/EntityType$Builder#build` | method | 1 |
| `net/minecraft/world/entity/EntityType$Builder#clientTrackingRange` | method | 1 |
| `net/minecraft/world/entity/EntityType$Builder#eyeHeight` | method | 1 |
| `net/minecraft/world/entity/EntityType$Builder#of` | method | 1 |
| `net/minecraft/world/entity/EntityType$Builder#ridingOffset` | method | 1 |
| `net/minecraft/world/entity/EntityType$Builder#sized` | method | 1 |
| `net/minecraft/world/entity/EntityType$Builder#updateInterval` | method | 1 |
| `net/minecraft/world/entity/EntityTypes#PLAYER` | field | 1 |
| `net/minecraft/world/entity/EquipmentSlot#CHEST` | field | 1 |
| `net/minecraft/world/entity/EquipmentSlot#FEET` | field | 1 |
| `net/minecraft/world/entity/EquipmentSlot#HEAD` | field | 4 |
| `net/minecraft/world/entity/EquipmentSlot#LEGS` | field | 1 |
| `net/minecraft/world/entity/EquipmentSlot#MAINHAND` | field | 5 |
| `net/minecraft/world/entity/EquipmentSlot#OFFHAND` | field | 3 |
| `net/minecraft/world/entity/ExperienceOrb#<init>` | method | 1 |
| `net/minecraft/world/entity/ExperienceOrb#award` | method | 1 |
| `net/minecraft/world/entity/HumanoidArm#LEFT` | field | 1 |
| `net/minecraft/world/entity/ItemOwner#asLivingEntity` | method | 1 |
| `net/minecraft/world/entity/Leashable$LeashData#CODEC` | field | 1 |
| `net/minecraft/world/entity/LivingEntity#addEffect` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#blockPosition` | method | 3 |
| `net/minecraft/world/entity/LivingEntity#distanceToSqr` | method | 5 |
| `net/minecraft/world/entity/LivingEntity#getAttributeValue` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#getBbWidth` | method | 5 |
| `net/minecraft/world/entity/LivingEntity#getBoundingBox` | method | 2 |
| `net/minecraft/world/entity/LivingEntity#getEyePosition` | method | 2 |
| `net/minecraft/world/entity/LivingEntity#getEyeY` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#getLastHurtByMob` | method | 2 |
| `net/minecraft/world/entity/LivingEntity#getLastHurtByMobTimestamp` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#getLastHurtMob` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#getLastHurtMobTimestamp` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#getOffhandItem` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#getType` | method | 2 |
| `net/minecraft/world/entity/LivingEntity#getUUID` | method | 2 |
| `net/minecraft/world/entity/LivingEntity#getX` | method | 8 |
| `net/minecraft/world/entity/LivingEntity#getY` | method | 4 |
| `net/minecraft/world/entity/LivingEntity#getZ` | method | 8 |
| `net/minecraft/world/entity/LivingEntity#hasLineOfSight` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#isAlive` | method | 10 |
| `net/minecraft/world/entity/LivingEntity#isAlliedTo` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#isAlwaysTicking` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#isBaby` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#isDeadOrDying` | method | 3 |
| `net/minecraft/world/entity/LivingEntity#isRemoved` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#isSpectator` | method | 4 |
| `net/minecraft/world/entity/LivingEntity#level` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#push` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#releaseUsingItem` | method | 1 |
| `net/minecraft/world/entity/Mob#blockPosition` | method | 1 |
| `net/minecraft/world/entity/Mob#checkMobSpawnRules` | method | 1 |
| `net/minecraft/world/entity/Mob#checkSpawnRules` | method | 1 |
| `net/minecraft/world/entity/Mob#createMobAttributes` | method | 2 |
| `net/minecraft/world/entity/Mob#discard` | method | 1 |
| `net/minecraft/world/entity/Mob#distanceToSqr` | method | 1 |
| `net/minecraft/world/entity/Mob#fallDistance` | field | 1 |
| `net/minecraft/world/entity/Mob#getActiveEffectsMap` | method | 1 |
| `net/minecraft/world/entity/Mob#getAttributeValue` | method | 2 |
| `net/minecraft/world/entity/Mob#getAttributes` | method | 1 |
| `net/minecraft/world/entity/Mob#getDeltaMovement` | method | 1 |
| `net/minecraft/world/entity/Mob#getLeashData` | method | 1 |
| `net/minecraft/world/entity/Mob#getMaxHealth` | method | 1 |
| `net/minecraft/world/entity/Mob#getTarget` | method | 2 |
| `net/minecraft/world/entity/Mob#getType` | method | 1 |
| `net/minecraft/world/entity/Mob#getX` | method | 1 |
| `net/minecraft/world/entity/Mob#getXRot` | method | 1 |
| `net/minecraft/world/entity/Mob#getY` | method | 1 |
| `net/minecraft/world/entity/Mob#getYRot` | method | 1 |
| `net/minecraft/world/entity/Mob#getZ` | method | 1 |
| `net/minecraft/world/entity/Mob#goalSelector` | method | 1 |
| `net/minecraft/world/entity/Mob#horizontalCollision` | field | 1 |
| `net/minecraft/world/entity/Mob#isInWater` | method | 1 |
| `net/minecraft/world/entity/Mob#level` | method | 1 |
| `net/minecraft/world/entity/Mob#setCustomName` | method | 1 |
| `net/minecraft/world/entity/Mob#setDeltaMovement` | method | 1 |
| `net/minecraft/world/entity/Mob#setHealth` | method | 1 |
| `net/minecraft/world/entity/Mob#setItemSlot` | method | 1 |
| `net/minecraft/world/entity/Mob#setLeashData` | method | 1 |
| `net/minecraft/world/entity/Mob#setLeftHanded` | method | 1 |
| `net/minecraft/world/entity/Mob#setNoGravity` | method | 1 |
| `net/minecraft/world/entity/Mob#setTarget` | method | 3 |
| `net/minecraft/world/entity/Mob#setXRot` | method | 1 |
| `net/minecraft/world/entity/Mob#setYRot` | method | 1 |
| `net/minecraft/world/entity/Mob#setYya` | method | 1 |
| `net/minecraft/world/entity/Mob#setZza` | method | 1 |
| `net/minecraft/world/entity/MobCategory#AMBIENT` | field | 1 |
| `net/minecraft/world/entity/MobCategory#CREATURE` | field | 1 |
| `net/minecraft/world/entity/MobCategory#MISC` | field | 1 |
| `net/minecraft/world/entity/MobCategory#MONSTER` | field | 1 |
| `net/minecraft/world/entity/NeutralMob#getPersistentAngerTarget` | method | 1 |
| `net/minecraft/world/entity/NeutralMob#isAngryAt` | method | 1 |
| `net/minecraft/world/entity/NeutralMob#readPersistentAngerSaveData` | method | 2 |
| `net/minecraft/world/entity/NeutralMob#setPersistentAngerTarget` | method | 1 |
| `net/minecraft/world/entity/PathfinderMob#<init>` | method | 3 |
| `net/minecraft/world/entity/PathfinderMob#addAdditionalSaveData` | method | 3 |
| `net/minecraft/world/entity/PathfinderMob#blockPosition` | method | 1 |
| `net/minecraft/world/entity/PathfinderMob#considersEntityAsAlly` | method | 1 |
| `net/minecraft/world/entity/PathfinderMob#createMobAttributes` | method | 1 |
| `net/minecraft/world/entity/PathfinderMob#customServerAiStep` | method | 1 |
| `net/minecraft/world/entity/PathfinderMob#defineSynchedData` | method | 3 |
| `net/minecraft/world/entity/PathfinderMob#distanceToSqr` | method | 2 |
| `net/minecraft/world/entity/PathfinderMob#getBbWidth` | method | 1 |
| `net/minecraft/world/entity/PathfinderMob#getLookControl` | method | 1 |
| `net/minecraft/world/entity/PathfinderMob#getNavigation` | method | 1 |
| `net/minecraft/world/entity/PathfinderMob#getRandom` | method | 2 |
| `net/minecraft/world/entity/PathfinderMob#getSensing` | method | 2 |
| `net/minecraft/world/entity/PathfinderMob#getTarget` | method | 3 |
| `net/minecraft/world/entity/PathfinderMob#getVehicle` | method | 1 |
| `net/minecraft/world/entity/PathfinderMob#isFreezing` | method | 1 |
| `net/minecraft/world/entity/PathfinderMob#isInWater` | method | 1 |
| `net/minecraft/world/entity/PathfinderMob#isWithinHome` | method | 1 |
| `net/minecraft/world/entity/PathfinderMob#level` | method | 2 |
| `net/minecraft/world/entity/PathfinderMob#makeStuckInBlock` | method | 1 |
| `net/minecraft/world/entity/PathfinderMob#readAdditionalSaveData` | method | 3 |
| `net/minecraft/world/entity/PathfinderMob#remove` | method | 1 |
| `net/minecraft/world/entity/PathfinderMob#restoreFrom` | method | 2 |
| `net/minecraft/world/entity/PathfinderMob#setAggressive` | method | 1 |
| `net/minecraft/world/entity/PathfinderMob#setTarget` | method | 2 |
| `net/minecraft/world/entity/PathfinderMob#tickHeadTurn` | method | 1 |
| `net/minecraft/world/entity/Pose#CROUCHING` | field | 1 |
| `net/minecraft/world/entity/SpawnPlacementTypes#NO_RESTRICTIONS` | field | 1 |
| `net/minecraft/world/entity/SpawnPlacementTypes#ON_GROUND` | field | 1 |
| `net/minecraft/world/entity/SpawnPlacements#register` | method | 1 |
| `net/minecraft/world/entity/SpawnPlacements$SpawnPredicate#test` | method | 1 |
| `net/minecraft/world/entity/TamableAnimal#getOwnerReference` | method | 2 |
| `net/minecraft/world/entity/TamableAnimal#isTame` | method | 1 |
| `net/minecraft/world/entity/WalkAnimationState#speed` | method | 2 |
| `net/minecraft/world/entity/ai/attributes/AttributeInstance#addTransientModifier` | method | 5 |
| `net/minecraft/world/entity/ai/attributes/AttributeInstance#hasModifier` | method | 5 |
| `net/minecraft/world/entity/ai/attributes/AttributeInstance#removeModifier` | method | 5 |
| `net/minecraft/world/entity/ai/attributes/AttributeInstance$Packed#CODEC` | field | 1 |
| `net/minecraft/world/entity/ai/attributes/AttributeMap#apply` | method | 1 |
| `net/minecraft/world/entity/ai/attributes/AttributeModifier#<init>` | method | 5 |
| `net/minecraft/world/entity/ai/attributes/AttributeModifier$Operation#ADD_VALUE` | field | 5 |
| `net/minecraft/world/entity/ai/attributes/AttributeSupplier#getBaseValue` | method | 1 |
| `net/minecraft/world/entity/ai/attributes/AttributeSupplier#hasAttribute` | method | 1 |
| `net/minecraft/world/entity/ai/attributes/AttributeSupplier$Builder#add` | method | 15 |
| `net/minecraft/world/entity/ai/attributes/AttributeSupplier$Builder#build` | method | 1 |
| `net/minecraft/world/entity/ai/attributes/Attributes#ARMOR` | field | 6 |
| `net/minecraft/world/entity/ai/attributes/Attributes#ARMOR_TOUGHNESS` | field | 2 |
| `net/minecraft/world/entity/ai/attributes/Attributes#ATTACK_DAMAGE` | field | 10 |
| `net/minecraft/world/entity/ai/attributes/Attributes#FLYING_SPEED` | field | 4 |
| `net/minecraft/world/entity/ai/attributes/Attributes#FOLLOW_RANGE` | field | 10 |
| `net/minecraft/world/entity/ai/attributes/Attributes#KNOCKBACK_RESISTANCE` | field | 5 |
| `net/minecraft/world/entity/ai/attributes/Attributes#MAX_HEALTH` | field | 13 |
| `net/minecraft/world/entity/ai/attributes/Attributes#MOVEMENT_SPEED` | field | 14 |
| `net/minecraft/world/entity/ai/attributes/Attributes#STEP_HEIGHT` | field | 1 |
| `net/minecraft/world/entity/ai/control/FlyingMoveControl#<init>` | method | 2 |
| `net/minecraft/world/entity/ai/control/LookControl#setLookAt` | method | 9 |
| `net/minecraft/world/entity/ai/control/MoveControl#<init>` | method | 1 |
| `net/minecraft/world/entity/ai/control/MoveControl#hasWanted` | method | 2 |
| `net/minecraft/world/entity/ai/control/MoveControl#setWantedPosition` | method | 5 |
| `net/minecraft/world/entity/ai/control/MoveControl$Operation#MOVE_TO` | field | 2 |
| `net/minecraft/world/entity/ai/control/MoveControl$Operation#WAIT` | field | 2 |
| `net/minecraft/world/entity/ai/goal/AvoidEntityGoal#<init>` | method | 1 |
| `net/minecraft/world/entity/ai/goal/FloatGoal#<init>` | method | 11 |
| `net/minecraft/world/entity/ai/goal/Goal#<init>` | method | 22 |
| `net/minecraft/world/entity/ai/goal/Goal#canContinueToUse` | method | 4 |
| `net/minecraft/world/entity/ai/goal/Goal#start` | method | 2 |
| `net/minecraft/world/entity/ai/goal/Goal#stop` | method | 6 |
| `net/minecraft/world/entity/ai/goal/Goal#tick` | method | 1 |
| `net/minecraft/world/entity/ai/goal/Goal$Flag#JUMP` | field | 2 |
| `net/minecraft/world/entity/ai/goal/Goal$Flag#LOOK` | field | 4 |
| `net/minecraft/world/entity/ai/goal/Goal$Flag#MOVE` | field | 16 |
| `net/minecraft/world/entity/ai/goal/Goal$Flag#TARGET` | field | 2 |
| `net/minecraft/world/entity/ai/goal/GoalSelector#addGoal` | method | 15 |
| `net/minecraft/world/entity/ai/goal/GoalSelector#getAvailableGoals` | method | 1 |
| `net/minecraft/world/entity/ai/goal/InteractGoal#<init>` | method | 1 |
| `net/minecraft/world/entity/ai/goal/LookAtPlayerGoal#<init>` | method | 14 |
| `net/minecraft/world/entity/ai/goal/LookAtTradingPlayerGoal#<init>` | method | 1 |
| `net/minecraft/world/entity/ai/goal/MeleeAttackGoal#<init>` | method | 2 |
| `net/minecraft/world/entity/ai/goal/MeleeAttackGoal#canContinueToUse` | method | 2 |
| `net/minecraft/world/entity/ai/goal/MeleeAttackGoal#canPerformAttack` | method | 1 |
| `net/minecraft/world/entity/ai/goal/MeleeAttackGoal#canUse` | method | 1 |
| `net/minecraft/world/entity/ai/goal/PanicGoal#<init>` | method | 1 |
| `net/minecraft/world/entity/ai/goal/RandomLookAroundGoal#<init>` | method | 9 |
| `net/minecraft/world/entity/ai/goal/RandomStrollGoal#<init>` | method | 1 |
| `net/minecraft/world/entity/ai/goal/WaterAvoidingRandomFlyingGoal#<init>` | method | 2 |
| `net/minecraft/world/entity/ai/goal/WaterAvoidingRandomFlyingGoal#canUse` | method | 1 |
| `net/minecraft/world/entity/ai/goal/WaterAvoidingRandomFlyingGoal#start` | method | 1 |
| `net/minecraft/world/entity/ai/goal/WaterAvoidingRandomStrollGoal#<init>` | method | 4 |
| `net/minecraft/world/entity/ai/goal/WaterAvoidingRandomStrollGoal#canUse` | method | 1 |
| `net/minecraft/world/entity/ai/goal/WrappedGoal#getGoal` | method | 1 |
| `net/minecraft/world/entity/ai/goal/target/HurtByTargetGoal#<init>` | method | 8 |
| `net/minecraft/world/entity/ai/goal/target/HurtByTargetGoal#setAlertOthers` | method | 2 |
| `net/minecraft/world/entity/ai/goal/target/NearestAttackableTargetGoal#<init>` | method | 7 |
| `net/minecraft/world/entity/ai/goal/target/NearestAttackableTargetGoal#canContinueToUse` | method | 1 |
| `net/minecraft/world/entity/ai/goal/target/NearestAttackableTargetGoal#setTarget` | method | 1 |
| `net/minecraft/world/entity/ai/goal/target/NearestAttackableTargetGoal#start` | method | 1 |
| `net/minecraft/world/entity/ai/goal/target/NearestAttackableTargetGoal#stop` | method | 1 |
| `net/minecraft/world/entity/ai/goal/target/NearestAttackableTargetGoal#tick` | method | 1 |
| `net/minecraft/world/entity/ai/goal/target/ResetUniversalAngerTargetGoal#<init>` | method | 1 |
| `net/minecraft/world/entity/ai/goal/target/TargetGoal#<init>` | method | 3 |
| `net/minecraft/world/entity/ai/goal/target/TargetGoal#start` | method | 3 |
| `net/minecraft/world/entity/ai/navigation/FlyingPathNavigation#<init>` | method | 2 |
| `net/minecraft/world/entity/ai/navigation/FlyingPathNavigation#getNodeEvaluator` | method | 2 |
| `net/minecraft/world/entity/ai/navigation/FlyingPathNavigation#setCanFloat` | method | 2 |
| `net/minecraft/world/entity/ai/navigation/FlyingPathNavigation#setCanOpenDoors` | method | 2 |
| `net/minecraft/world/entity/ai/navigation/FlyingPathNavigation#tick` | method | 1 |
| `net/minecraft/world/entity/ai/navigation/GroundPathNavigation#setCanOpenDoors` | method | 1 |
| `net/minecraft/world/entity/ai/navigation/PathNavigation#createPath` | method | 2 |
| `net/minecraft/world/entity/ai/navigation/PathNavigation#getPath` | method | 1 |
| `net/minecraft/world/entity/ai/navigation/PathNavigation#isDone` | method | 10 |
| `net/minecraft/world/entity/ai/navigation/PathNavigation#isInProgress` | method | 1 |
| `net/minecraft/world/entity/ai/navigation/PathNavigation#moveTo` | method | 10 |
| `net/minecraft/world/entity/ai/navigation/PathNavigation#stop` | method | 19 |
| `net/minecraft/world/entity/ai/sensing/Sensing#hasLineOfSight` | method | 4 |
| `net/minecraft/world/entity/ai/targeting/TargetingConditions#DEFAULT` | field | 2 |
| `net/minecraft/world/entity/ai/targeting/TargetingConditions#forCombat` | method | 2 |
| `net/minecraft/world/entity/ai/targeting/TargetingConditions#forNonCombat` | method | 2 |
| `net/minecraft/world/entity/ai/targeting/TargetingConditions#ignoreInvisibilityTesting` | method | 2 |
| `net/minecraft/world/entity/ai/targeting/TargetingConditions#ignoreLineOfSight` | method | 3 |
| `net/minecraft/world/entity/ai/targeting/TargetingConditions#range` | method | 3 |
| `net/minecraft/world/entity/ai/targeting/TargetingConditions#selector` | method | 2 |
| `net/minecraft/world/entity/ai/targeting/TargetingConditions#test` | method | 1 |
| `net/minecraft/world/entity/ai/util/AirAndWaterRandomPos#getPos` | method | 2 |
| `net/minecraft/world/entity/ai/util/DefaultRandomPos#getPosAway` | method | 2 |
| `net/minecraft/world/entity/ai/util/DefaultRandomPos#getPosTowards` | method | 1 |
| `net/minecraft/world/entity/ai/util/GoalUtils#hasGroundPathNavigation` | method | 1 |
| `net/minecraft/world/entity/ai/util/HoverRandomPos#getPos` | method | 4 |
| `net/minecraft/world/entity/ai/util/LandRandomPos#getPos` | method | 1 |
| `net/minecraft/world/entity/animal/equine/AbstractHorse#isTamed` | method | 1 |
| `net/minecraft/world/entity/boss/wither/WitherBoss#canDestroy` | method | 1 |
| `net/minecraft/world/entity/decoration/ArmorStand#getItemBySlot` | method | 1 |
| `net/minecraft/world/entity/item/ItemEntity#discard` | method | 2 |
| `net/minecraft/world/entity/item/ItemEntity#distanceToSqr` | method | 1 |
| `net/minecraft/world/entity/item/ItemEntity#getBoundingBox` | method | 2 |
| `net/minecraft/world/entity/item/ItemEntity#getItem` | method | 4 |
| `net/minecraft/world/entity/item/ItemEntity#getOwner` | method | 1 |
| `net/minecraft/world/entity/item/ItemEntity#hasPickUpDelay` | method | 3 |
| `net/minecraft/world/entity/item/ItemEntity#isAlive` | method | 2 |
| `net/minecraft/world/entity/item/ItemEntity#isRemoved` | method | 1 |
| `net/minecraft/world/entity/item/ItemEntity#setItem` | method | 2 |
| `net/minecraft/world/entity/item/ItemEntity#setPermanentlyInvulnerable` | method | 1 |
| `net/minecraft/world/entity/item/ItemEntity#setUnlimitedLifetime` | method | 1 |
| `net/minecraft/world/entity/monster/Creeper#<init>` | method | 1 |
| `net/minecraft/world/entity/monster/Creeper#addAdditionalSaveData` | method | 1 |
| `net/minecraft/world/entity/monster/Creeper#aiStep` | method | 1 |
| `net/minecraft/world/entity/monster/Creeper#dropCustomDeathLoot` | method | 1 |
| `net/minecraft/world/entity/monster/Creeper#explodeCreeper` | method | 1 |
| `net/minecraft/world/entity/monster/Creeper#isPowered` | method | 1 |
| `net/minecraft/world/entity/monster/Creeper#level` | method | 1 |
| `net/minecraft/world/entity/monster/Creeper#readAdditionalSaveData` | method | 1 |
| `net/minecraft/world/entity/monster/Monster#<init>` | method | 3 |
| `net/minecraft/world/entity/monster/Monster#addAdditionalSaveData` | method | 3 |
| `net/minecraft/world/entity/monster/Monster#aiStep` | method | 3 |
| `net/minecraft/world/entity/monster/Monster#baseTick` | method | 1 |
| `net/minecraft/world/entity/monster/Monster#canBeAffected` | method | 1 |
| `net/minecraft/world/entity/monster/Monster#checkAnyLightMonsterSpawnRules` | method | 2 |
| `net/minecraft/world/entity/monster/Monster#checkMonsterSpawnRules` | method | 1 |
| `net/minecraft/world/entity/monster/Monster#createMonsterAttributes` | method | 10 |
| `net/minecraft/world/entity/monster/Monster#customServerAiStep` | method | 2 |
| `net/minecraft/world/entity/monster/Monster#defineSynchedData` | method | 3 |
| `net/minecraft/world/entity/monster/Monster#getDefaultDimensions` | method | 1 |
| `net/minecraft/world/entity/monster/Monster#handleEntityEvent` | method | 1 |
| `net/minecraft/world/entity/monster/Monster#hurtServer` | method | 2 |
| `net/minecraft/world/entity/monster/Monster#isDarkEnoughToSpawn` | method | 1 |
| `net/minecraft/world/entity/monster/Monster#playerTouch` | method | 1 |
| `net/minecraft/world/entity/monster/Monster#readAdditionalSaveData` | method | 3 |
| `net/minecraft/world/entity/monster/Monster#registerGoals` | method | 1 |
| `net/minecraft/world/entity/monster/Monster#remove` | method | 1 |
| `net/minecraft/world/entity/monster/Monster#resetFallDistance` | method | 1 |
| `net/minecraft/world/entity/monster/Monster#setCustomName` | method | 1 |
| `net/minecraft/world/entity/monster/Monster#startSeenByPlayer` | method | 1 |
| `net/minecraft/world/entity/monster/Monster#stopSeenByPlayer` | method | 1 |
| `net/minecraft/world/entity/monster/Monster#tick` | method | 1 |
| `net/minecraft/world/entity/npc/villager/AbstractVillager#<init>` | method | 1 |
| `net/minecraft/world/entity/npc/villager/AbstractVillager#addAdditionalSaveData` | method | 1 |
| `net/minecraft/world/entity/npc/villager/AbstractVillager#aiStep` | method | 1 |
| `net/minecraft/world/entity/npc/villager/AbstractVillager#mobInteract` | method | 1 |
| `net/minecraft/world/entity/npc/villager/AbstractVillager#readAdditionalSaveData` | method | 1 |
| `net/minecraft/world/entity/npc/villager/AbstractVillager#tick` | method | 1 |
| `net/minecraft/world/entity/player/Abilities#flying` | field | 1 |
| `net/minecraft/world/entity/player/Abilities#instabuild` | field | 2 |
| `net/minecraft/world/entity/player/Abilities#mayBuild` | field | 3 |
| `net/minecraft/world/entity/player/Inventory#getContainerSize` | method | 1 |
| `net/minecraft/world/entity/player/Inventory#getItem` | method | 2 |
| `net/minecraft/world/entity/player/Inventory#player` | field | 1 |
| `net/minecraft/world/entity/player/Inventory#setItem` | method | 1 |
| `net/minecraft/world/entity/player/Player#addEffect` | method | 4 |
| `net/minecraft/world/entity/player/Player#awardStat` | method | 1 |
| `net/minecraft/world/entity/player/Player#blockPosition` | method | 6 |
| `net/minecraft/world/entity/player/Player#canHarmPlayer` | method | 1 |
| `net/minecraft/world/entity/player/Player#containerMenu` | field | 2 |
| `net/minecraft/world/entity/player/Player#distanceToSqr` | method | 2 |
| `net/minecraft/world/entity/player/Player#drop` | method | 1 |
| `net/minecraft/world/entity/player/Player#getAbilities` | method | 5 |
| `net/minecraft/world/entity/player/Player#getBoundingBox` | method | 1 |
| `net/minecraft/world/entity/player/Player#getCooldowns` | method | 2 |
| `net/minecraft/world/entity/player/Player#getEyePosition` | method | 1 |
| `net/minecraft/world/entity/player/Player#getEyeY` | method | 1 |
| `net/minecraft/world/entity/player/Player#getId` | method | 1 |
| `net/minecraft/world/entity/player/Player#getInventory` | method | 1 |
| `net/minecraft/world/entity/player/Player#getItemBySlot` | method | 1 |
| `net/minecraft/world/entity/player/Player#getItemInHand` | method | 4 |
| `net/minecraft/world/entity/player/Player#getMainHandItem` | method | 1 |
| `net/minecraft/world/entity/player/Player#getName` | method | 1 |
| `net/minecraft/world/entity/player/Player#getOffhandItem` | method | 1 |
| `net/minecraft/world/entity/player/Player#getUUID` | method | 4 |
| `net/minecraft/world/entity/player/Player#getUsedItemHand` | method | 1 |
| `net/minecraft/world/entity/player/Player#getX` | method | 3 |
| `net/minecraft/world/entity/player/Player#getY` | method | 2 |
| `net/minecraft/world/entity/player/Player#getZ` | method | 3 |
| `net/minecraft/world/entity/player/Player#hasLineOfSight` | method | 1 |
| `net/minecraft/world/entity/player/Player#isAlive` | method | 1 |
| `net/minecraft/world/entity/player/Player#isCreative` | method | 11 |
| `net/minecraft/world/entity/player/Player#isFallFlying` | method | 1 |
| `net/minecraft/world/entity/player/Player#isInWater` | method | 1 |
| `net/minecraft/world/entity/player/Player#isPassenger` | method | 1 |
| `net/minecraft/world/entity/player/Player#isSecondaryUseActive` | method | 1 |
| `net/minecraft/world/entity/player/Player#isShiftKeyDown` | method | 6 |
| `net/minecraft/world/entity/player/Player#isSpectator` | method | 8 |
| `net/minecraft/world/entity/player/Player#isTextFilteringEnabled` | method | 2 |
| `net/minecraft/world/entity/player/Player#level` | method | 4 |
| `net/minecraft/world/entity/player/Player#mayBuild` | method | 4 |
| `net/minecraft/world/entity/player/Player#openMenu` | method | 4 |
| `net/minecraft/world/entity/player/Player#playSound` | method | 1 |
| `net/minecraft/world/entity/player/Player#position` | method | 4 |
| `net/minecraft/world/entity/player/Player#sendOverlayMessage` | method | 4 |
| `net/minecraft/world/entity/player/Player#startUsingItem` | method | 2 |
| `net/minecraft/world/entity/player/Player#teleportTo` | method | 1 |
| `net/minecraft/world/entity/projectile/EvokerFangs#<init>` | method | 1 |
| `net/minecraft/world/entity/projectile/Projectile#getOwner` | method | 1 |
| `net/minecraft/world/entity/projectile/Projectile#isOnFire` | method | 1 |
| `net/minecraft/world/entity/projectile/Projectile#level` | method | 1 |
| `net/minecraft/world/entity/projectile/ProjectileUtil#getEntityHitResult` | method | 1 |
| `net/minecraft/world/entity/projectile/hurtingprojectile/AbstractHurtingProjectile#<init>` | method | 2 |
| `net/minecraft/world/entity/projectile/hurtingprojectile/AbstractHurtingProjectile#defineSynchedData` | method | 1 |
| `net/minecraft/world/entity/projectile/hurtingprojectile/AbstractHurtingProjectile#getInertia` | method | 1 |
| `net/minecraft/world/entity/projectile/hurtingprojectile/AbstractHurtingProjectile#onHit` | method | 1 |
| `net/minecraft/world/entity/projectile/hurtingprojectile/AbstractHurtingProjectile#onHitEntity` | method | 1 |
| `net/minecraft/world/entity/projectile/hurtingprojectile/AbstractHurtingProjectile#tick` | method | 1 |
| `net/minecraft/world/flag/FeatureFlags#VANILLA_SET` | field | 1 |
| `net/minecraft/world/inventory/AbstractContainerMenu#<init>` | method | 1 |
| `net/minecraft/world/inventory/AbstractContainerMenu#canTakeItemForPickAll` | method | 1 |
| `net/minecraft/world/inventory/AbstractContainerMenu#containerId` | field | 1 |
| `net/minecraft/world/inventory/AbstractContainerMenu#incrementStateId` | method | 1 |
| `net/minecraft/world/inventory/AbstractContainerMenu#removed` | method | 1 |
| `net/minecraft/world/inventory/AbstractContainerMenu#setRemoteSlot` | method | 1 |
| `net/minecraft/world/inventory/AbstractContainerMenu#stillValid` | method | 1 |
| `net/minecraft/world/inventory/ChestMenu#getContainer` | method | 3 |
| `net/minecraft/world/inventory/ChestMenu#sixRows` | method | 3 |
| `net/minecraft/world/inventory/ChestMenu#threeRows` | method | 1 |
| `net/minecraft/world/inventory/ContainerLevelAccess#NULL` | field | 1 |
| `net/minecraft/world/inventory/ContainerLevelAccess#create` | method | 1 |
| `net/minecraft/world/inventory/ContainerLevelAccess#execute` | method | 2 |
| `net/minecraft/world/inventory/CraftingContainer#asCraftInput` | method | 1 |
| `net/minecraft/world/inventory/CraftingMenu#quickMoveStack` | method | 1 |
| `net/minecraft/world/inventory/CraftingMenu#slotChangedCraftingGrid` | method | 1 |
| `net/minecraft/world/inventory/CraftingMenu#slots` | field | 1 |
| `net/minecraft/world/inventory/DataSlot#get` | method | 1 |
| `net/minecraft/world/inventory/DataSlot#set` | method | 1 |
| `net/minecraft/world/inventory/DataSlot#standalone` | method | 1 |
| `net/minecraft/world/inventory/MenuType#<init>` | method | 1 |
| `net/minecraft/world/inventory/RecipeCraftingHolder#getRecipeUsed` | method | 1 |
| `net/minecraft/world/inventory/ResultContainer#<init>` | method | 1 |
| `net/minecraft/world/inventory/ResultContainer#awardUsedRecipes` | method | 1 |
| `net/minecraft/world/inventory/ResultContainer#removeItemNoUpdate` | method | 1 |
| `net/minecraft/world/inventory/ResultContainer#setItem` | method | 1 |
| `net/minecraft/world/inventory/ResultContainer#setRecipeUsed` | method | 1 |
| `net/minecraft/world/inventory/Slot#<init>` | method | 3 |
| `net/minecraft/world/inventory/Slot#container` | field | 1 |
| `net/minecraft/world/inventory/Slot#getItem` | method | 2 |
| `net/minecraft/world/inventory/Slot#hasItem` | method | 2 |
| `net/minecraft/world/inventory/Slot#mayPickup` | method | 2 |
| `net/minecraft/world/inventory/Slot#onQuickCraft` | method | 1 |
| `net/minecraft/world/inventory/Slot#onTake` | method | 2 |
| `net/minecraft/world/inventory/Slot#setChanged` | method | 1 |
| `net/minecraft/world/inventory/Slot#x` | field | 1 |
| `net/minecraft/world/inventory/Slot#y` | field | 1 |
| `net/minecraft/world/item/BlockItem#<init>` | method | 5 |
| `net/minecraft/world/item/BlockItem#appendHoverText` | method | 1 |
| `net/minecraft/world/item/BlockItem#getBlock` | method | 2 |
| `net/minecraft/world/item/CreativeModeTab$Builder#build` | method | 1 |
| `net/minecraft/world/item/CreativeModeTab$Builder#icon` | method | 1 |
| `net/minecraft/world/item/CreativeModeTab$Builder#title` | method | 1 |
| `net/minecraft/world/item/CreativeModeTab$Output#accept` | method | 1 |
| `net/minecraft/world/item/CrossbowItem#getChargeDuration` | method | 1 |
| `net/minecraft/world/item/DoubleHighBlockItem#<init>` | method | 1 |
| `net/minecraft/world/item/DyeColor#BLACK` | field | 1 |
| `net/minecraft/world/item/Item#appendHoverText` | method | 1 |
| `net/minecraft/world/item/Item#getDefaultInstance` | method | 2 |
| `net/minecraft/world/item/Item#getDescriptionId` | method | 2 |
| `net/minecraft/world/item/Item#hurtEnemy` | method | 1 |
| `net/minecraft/world/item/Item#isFoil` | method | 1 |
| `net/minecraft/world/item/Item#onCraftedBy` | method | 1 |
| `net/minecraft/world/item/Item#use` | method | 1 |
| `net/minecraft/world/item/Item#useOn` | method | 1 |
| `net/minecraft/world/item/Item$Properties#<init>` | method | 2 |
| `net/minecraft/world/item/Item$Properties#component` | method | 1 |
| `net/minecraft/world/item/Item$Properties#jukeboxPlayable` | method | 1 |
| `net/minecraft/world/item/Item$Properties#overrideDescription` | method | 1 |
| `net/minecraft/world/item/Item$Properties#setId` | method | 1 |
| `net/minecraft/world/item/Item$Properties#spawnEgg` | method | 1 |
| `net/minecraft/world/item/Item$Properties#sword` | method | 1 |
| `net/minecraft/world/item/Item$Properties#useBlockDescriptionPrefix` | method | 2 |
| `net/minecraft/world/item/Item$TooltipContext#registries` | method | 1 |
| `net/minecraft/world/item/ItemCooldowns#addCooldown` | method | 1 |
| `net/minecraft/world/item/ItemCooldowns#isOnCooldown` | method | 1 |
| `net/minecraft/world/item/ItemDisplayContext#GROUND` | field | 2 |
| `net/minecraft/world/item/ItemDisplayContext#NONE` | field | 1 |
| `net/minecraft/world/item/ItemDisplayContext#THIRD_PERSON_LEFT_HAND` | field | 1 |
| `net/minecraft/world/item/ItemStack#<init>` | method | 16 |
| `net/minecraft/world/item/ItemStack#EMPTY` | field | 8 |
| `net/minecraft/world/item/ItemStack#OPTIONAL_CODEC` | field | 2 |
| `net/minecraft/world/item/ItemStack#STREAM_CODEC` | field | 1 |
| `net/minecraft/world/item/ItemStack#canPlaceOnBlockInAdventureMode` | method | 1 |
| `net/minecraft/world/item/ItemStack#copy` | method | 5 |
| `net/minecraft/world/item/ItemStack#get` | method | 3 |
| `net/minecraft/world/item/ItemStack#getDamageValue` | method | 1 |
| `net/minecraft/world/item/ItemStack#getItem` | method | 10 |
| `net/minecraft/world/item/ItemStack#getMaxDamage` | method | 1 |
| `net/minecraft/world/item/ItemStack#has` | method | 1 |
| `net/minecraft/world/item/ItemStack#is` | method | 12 |
| `net/minecraft/world/item/ItemStack#isDamageableItem` | method | 1 |
| `net/minecraft/world/item/ItemStack#isEmpty` | method | 14 |
| `net/minecraft/world/item/ItemStack#isItemEnabled` | method | 1 |
| `net/minecraft/world/item/ItemStack#onCraftedBy` | method | 1 |
| `net/minecraft/world/item/ItemStack#remove` | method | 1 |
| `net/minecraft/world/item/ItemStack#set` | method | 1 |
| `net/minecraft/world/item/ItemStack#setDamageValue` | method | 1 |
| `net/minecraft/world/item/ItemStack#split` | method | 1 |
| `net/minecraft/world/item/ItemStackTemplate#<init>` | method | 1 |
| `net/minecraft/world/item/ItemStackTemplate#STREAM_CODEC` | field | 1 |
| `net/minecraft/world/item/ItemStackTemplate#count` | method | 1 |
| `net/minecraft/world/item/ItemStackTemplate#item` | method | 1 |
| `net/minecraft/world/item/ItemUseAnimation#TRIDENT` | field | 1 |
| `net/minecraft/world/item/Items#AIR` | field | 1 |
| `net/minecraft/world/item/Items#BONE_BLOCK` | field | 1 |
| `net/minecraft/world/item/Items#CARVED_PUMPKIN` | field | 1 |
| `net/minecraft/world/item/Items#FIRE_CHARGE` | field | 1 |
| `net/minecraft/world/item/Items#FLINT_AND_STEEL` | field | 1 |
| `net/minecraft/world/item/Items#GLASS_BOTTLE` | field | 1 |
| `net/minecraft/world/item/Items#JACK_O_LANTERN` | field | 1 |
| `net/minecraft/world/item/Items#SHIELD` | field | 1 |
| `net/minecraft/world/item/Items#SKELETON_SKULL` | field | 1 |
| `net/minecraft/world/item/Items#WATER_BUCKET` | field | 1 |
| `net/minecraft/world/item/Items#WRITABLE_BOOK` | field | 1 |
| `net/minecraft/world/item/Items#WRITTEN_BOOK` | field | 1 |
| `net/minecraft/world/item/MapItem#<init>` | method | 1 |
| `net/minecraft/world/item/SignApplicator#canApplyToSign` | method | 1 |
| `net/minecraft/world/item/SpawnEggItem#<init>` | method | 1 |
| `net/minecraft/world/item/ToolMaterial#STONE` | field | 1 |
| `net/minecraft/world/item/component/CustomData#copyTag` | method | 1 |
| `net/minecraft/world/item/component/CustomData#of` | method | 1 |
| `net/minecraft/world/item/component/MapDecorations#EMPTY` | field | 1 |
| `net/minecraft/world/item/component/WritableBookContent#pages` | method | 1 |
| `net/minecraft/world/item/component/WrittenBookContent#title` | method | 1 |
| `net/minecraft/world/item/context/BlockPlaceContext#getClickedPos` | method | 16 |
| `net/minecraft/world/item/context/BlockPlaceContext#getHorizontalDirection` | method | 11 |
| `net/minecraft/world/item/context/BlockPlaceContext#getItemInHand` | method | 1 |
| `net/minecraft/world/item/context/BlockPlaceContext#getLevel` | method | 15 |
| `net/minecraft/world/item/context/BlockPlaceContext#isSecondaryUseActive` | method | 1 |
| `net/minecraft/world/item/context/UseOnContext#getClickedFace` | method | 1 |
| `net/minecraft/world/item/context/UseOnContext#getClickedPos` | method | 1 |
| `net/minecraft/world/item/context/UseOnContext#getItemInHand` | method | 1 |
| `net/minecraft/world/item/context/UseOnContext#getLevel` | method | 1 |
| `net/minecraft/world/item/context/UseOnContext#getPlayer` | method | 1 |
| `net/minecraft/world/item/crafting/Ingredient#CODEC` | field | 2 |
| `net/minecraft/world/item/crafting/Ingredient#CONTENTS_STREAM_CODEC` | field | 1 |
| `net/minecraft/world/item/crafting/Ingredient#test` | method | 1 |
| `net/minecraft/world/item/crafting/PlacementInfo#create` | method | 1 |
| `net/minecraft/world/item/crafting/RecipeBookCategories#CRAFTING_MISC` | field | 1 |
| `net/minecraft/world/item/crafting/RecipeHolder#<init>` | method | 1 |
| `net/minecraft/world/item/crafting/RecipeHolder#id` | method | 2 |
| `net/minecraft/world/item/crafting/RecipeHolder#value` | method | 1 |
| `net/minecraft/world/item/crafting/RecipeManager#getRecipeFor` | method | 1 |
| `net/minecraft/world/item/crafting/RecipeManager#getRecipes` | method | 1 |
| `net/minecraft/world/item/crafting/RecipeSerializer#<init>` | method | 1 |
| `net/minecraft/world/item/crafting/RecipeType#CRAFTING` | field | 1 |
| `net/minecraft/world/item/crafting/SingleRecipeInput#<init>` | method | 1 |
| `net/minecraft/world/item/crafting/SingleRecipeInput#item` | method | 1 |
| `net/minecraft/world/item/enchantment/EnchantmentEffectComponents#PREVENT_EQUIPMENT_DROP` | field | 1 |
| `net/minecraft/world/item/enchantment/EnchantmentHelper#doPostAttackEffects` | method | 1 |
| `net/minecraft/world/item/enchantment/EnchantmentHelper#has` | method | 1 |
| `net/minecraft/world/item/trading/MerchantOffer#shouldRewardExp` | method | 1 |
| `net/minecraft/world/item/trading/MerchantOffers#add` | method | 1 |
| `net/minecraft/world/item/trading/MerchantOffers#clear` | method | 1 |
| `net/minecraft/world/item/trading/MerchantOffers#isEmpty` | method | 1 |
| `net/minecraft/world/item/trading/MerchantOffers#size` | method | 1 |
| `net/minecraft/world/level/ChunkPos#containing` | method | 1 |
| `net/minecraft/world/level/ChunkPos#getMaxBlockX` | method | 1 |
| `net/minecraft/world/level/ChunkPos#getMaxBlockZ` | method | 1 |
| `net/minecraft/world/level/ChunkPos#getMiddleBlockPosition` | method | 1 |
| `net/minecraft/world/level/ChunkPos#getMinBlockX` | method | 1 |
| `net/minecraft/world/level/ChunkPos#getMinBlockZ` | method | 1 |
| `net/minecraft/world/level/ChunkPos#getX` | method | 1 |
| `net/minecraft/world/level/ChunkPos#getZ` | method | 1 |
| `net/minecraft/world/level/ChunkPos#pack` | method | 2 |
| `net/minecraft/world/level/ClipContext#<init>` | method | 1 |
| `net/minecraft/world/level/ClipContext$Block#COLLIDER` | field | 1 |
| `net/minecraft/world/level/ClipContext$Fluid#NONE` | field | 1 |
| `net/minecraft/world/level/EmptyBlockGetter#INSTANCE` | field | 2 |
| `net/minecraft/world/level/Explosion$BlockInteraction#KEEP` | field | 1 |
| `net/minecraft/world/level/Level#OVERWORLD` | field | 2 |
| `net/minecraft/world/level/Level#addAlwaysVisibleParticle` | method | 1 |
| `net/minecraft/world/level/Level#addFreshEntity` | method | 8 |
| `net/minecraft/world/level/Level#addParticle` | method | 15 |
| `net/minecraft/world/level/Level#blockEvent` | method | 2 |
| `net/minecraft/world/level/Level#broadcastEntityEvent` | method | 9 |
| `net/minecraft/world/level/Level#clip` | method | 1 |
| `net/minecraft/world/level/Level#dimension` | method | 2 |
| `net/minecraft/world/level/Level#dimensionType` | method | 1 |
| `net/minecraft/world/level/Level#enabledFeatures` | method | 1 |
| `net/minecraft/world/level/Level#environmentAttributes` | method | 1 |
| `net/minecraft/world/level/Level#explode` | method | 2 |
| `net/minecraft/world/level/Level#gameEvent` | method | 2 |
| `net/minecraft/world/level/Level#getBlockEntity` | method | 5 |
| `net/minecraft/world/level/Level#getBlockState` | method | 23 |
| `net/minecraft/world/level/Level#getChunkSource` | method | 1 |
| `net/minecraft/world/level/Level#getDifficulty` | method | 1 |
| `net/minecraft/world/level/Level#getEntitiesOfClass` | method | 10 |
| `net/minecraft/world/level/Level#getFluidState` | method | 12 |
| `net/minecraft/world/level/Level#getGameTime` | method | 6 |
| `net/minecraft/world/level/Level#getHeightmapPos` | method | 1 |
| `net/minecraft/world/level/Level#getNearestPlayer` | method | 7 |
| `net/minecraft/world/level/Level#getOverworldClockTime` | method | 1 |
| `net/minecraft/world/level/Level#getPlayerByUUID` | method | 2 |
| `net/minecraft/world/level/Level#getRandom` | method | 8 |
| `net/minecraft/world/level/Level#getServer` | method | 2 |
| `net/minecraft/world/level/Level#getWorldBorder` | method | 1 |
| `net/minecraft/world/level/Level#isBrightOutside` | method | 3 |
| `net/minecraft/world/level/Level#isDarkOutside` | method | 2 |
| `net/minecraft/world/level/Level#isEmptyBlock` | method | 2 |
| `net/minecraft/world/level/Level#isLoaded` | method | 1 |
| `net/minecraft/world/level/Level#levelEvent` | method | 4 |
| `net/minecraft/world/level/Level#playLocalSound` | method | 8 |
| `net/minecraft/world/level/Level#playSound` | method | 15 |
| `net/minecraft/world/level/Level#players` | method | 1 |
| `net/minecraft/world/level/Level#sendBlockUpdated` | method | 1 |
| `net/minecraft/world/level/Level#setBlockAndUpdate` | method | 4 |
| `net/minecraft/world/level/Level$ExplosionInteraction#MOB` | field | 2 |
| `net/minecraft/world/level/Level$ExplosionInteraction#NONE` | field | 2 |
| `net/minecraft/world/level/LevelAccessor#addParticle` | method | 1 |
| `net/minecraft/world/level/LevelAccessor#gameEvent` | method | 1 |
| `net/minecraft/world/level/LevelAccessor#getBiome` | method | 1 |
| `net/minecraft/world/level/LevelAccessor#getBlockState` | method | 1 |
| `net/minecraft/world/level/LevelAccessor#getBrightness` | method | 1 |
| `net/minecraft/world/level/LevelAccessor#getFluidState` | method | 2 |
| `net/minecraft/world/level/LevelAccessor#getRandom` | method | 1 |
| `net/minecraft/world/level/LevelAccessor#playSound` | method | 1 |
| `net/minecraft/world/level/LevelAccessor#scheduleTick` | method | 1 |
| `net/minecraft/world/level/LevelAccessor#setBlock` | method | 1 |
| `net/minecraft/world/level/LevelReader#getBlockEntity` | method | 1 |
| `net/minecraft/world/level/LevelReader#getBlockState` | method | 4 |
| `net/minecraft/world/level/LevelReader#noCollision` | method | 1 |
| `net/minecraft/world/level/LightLayer#SKY` | field | 1 |
| `net/minecraft/world/level/NaturalSpawner#isValidEmptySpawnBlock` | method | 1 |
| `net/minecraft/world/level/NoiseColumn#getBlock` | method | 1 |
| `net/minecraft/world/level/ScheduledTickAccess#scheduleTick` | method | 6 |
| `net/minecraft/world/level/ServerLevelAccessor#environmentAttributes` | method | 1 |
| `net/minecraft/world/level/ServerLevelAccessor#getBlockState` | method | 2 |
| `net/minecraft/world/level/ServerLevelAccessor#getHeight` | method | 2 |
| `net/minecraft/world/level/ServerLevelAccessor#getLevel` | method | 2 |
| `net/minecraft/world/level/ServerLevelAccessor#getRandom` | method | 1 |
| `net/minecraft/world/level/ServerLevelAccessor#playSound` | method | 1 |
| `net/minecraft/world/level/WorldGenLevel#getBlockState` | method | 1 |
| `net/minecraft/world/level/biome/MobSpawnSettings$SpawnerData#<init>` | method | 1 |
| `net/minecraft/world/level/biome/MobSpawnSettings$SpawnerData#type` | method | 1 |
| `net/minecraft/world/level/block/BaseEntityBlock#<init>` | method | 5 |
| `net/minecraft/world/level/block/BaseEntityBlock#createTickerHelper` | method | 1 |
| `net/minecraft/world/level/block/BaseEntityBlock#getFluidState` | method | 3 |
| `net/minecraft/world/level/block/BaseEntityBlock#playerWillDestroy` | method | 1 |
| `net/minecraft/world/level/block/BaseEntityBlock#setPlacedBy` | method | 1 |
| `net/minecraft/world/level/block/BaseEntityBlock#updateShape` | method | 2 |
| `net/minecraft/world/level/block/BedBlock#FACING` | field | 1 |
| `net/minecraft/world/level/block/BedBlock#PART` | field | 1 |
| `net/minecraft/world/level/block/Block#animateTick` | method | 2 |
| `net/minecraft/world/level/block/Block#asItem` | method | 3 |
| `net/minecraft/world/level/block/Block#box` | method | 9 |
| `net/minecraft/world/level/block/Block#canBeReplaced` | method | 1 |
| `net/minecraft/world/level/block/Block#defaultBlockState` | method | 11 |
| `net/minecraft/world/level/block/Block#getDrops` | method | 2 |
| `net/minecraft/world/level/block/Block#getFluidState` | method | 8 |
| `net/minecraft/world/level/block/Block#getId` | method | 3 |
| `net/minecraft/world/level/block/Block#getLootTable` | method | 1 |
| `net/minecraft/world/level/block/Block#playerWillDestroy` | method | 2 |
| `net/minecraft/world/level/block/Block#toString` | method | 1 |
| `net/minecraft/world/level/block/Block#updateShape` | method | 3 |
| `net/minecraft/world/level/block/Block#useItemOn` | method | 1 |
| `net/minecraft/world/level/block/Blocks#AIR` | field | 6 |
| `net/minecraft/world/level/block/Blocks#ANDESITE` | field | 1 |
| `net/minecraft/world/level/block/Blocks#BLACKSTONE` | field | 1 |
| `net/minecraft/world/level/block/Blocks#BONE_BLOCK` | field | 1 |
| `net/minecraft/world/level/block/Blocks#CARVED_PUMPKIN` | field | 1 |
| `net/minecraft/world/level/block/Blocks#COARSE_DIRT` | field | 2 |
| `net/minecraft/world/level/block/Blocks#COBBLESTONE` | field | 1 |
| `net/minecraft/world/level/block/Blocks#COBBLESTONE_SLAB` | field | 1 |
| `net/minecraft/world/level/block/Blocks#COBWEB` | field | 3 |
| `net/minecraft/world/level/block/Blocks#COPPER_TORCH` | field | 1 |
| `net/minecraft/world/level/block/Blocks#COPPER_WALL_TORCH` | field | 1 |
| `net/minecraft/world/level/block/Blocks#DEEPSLATE` | field | 1 |
| `net/minecraft/world/level/block/Blocks#DIORITE` | field | 1 |
| `net/minecraft/world/level/block/Blocks#DIRT` | field | 2 |
| `net/minecraft/world/level/block/Blocks#DIRT_PATH` | field | 2 |
| `net/minecraft/world/level/block/Blocks#DYED_TERRACOTTA` | field | 1 |
| `net/minecraft/world/level/block/Blocks#FARMLAND` | field | 3 |
| `net/minecraft/world/level/block/Blocks#GLASS` | field | 1 |
| `net/minecraft/world/level/block/Blocks#GRANITE` | field | 1 |
| `net/minecraft/world/level/block/Blocks#GRASS_BLOCK` | field | 3 |
| `net/minecraft/world/level/block/Blocks#GRAVEL` | field | 1 |
| `net/minecraft/world/level/block/Blocks#LANTERN` | field | 1 |
| `net/minecraft/world/level/block/Blocks#LIGHT` | field | 1 |
| `net/minecraft/world/level/block/Blocks#MAGMA_BLOCK` | field | 1 |
| `net/minecraft/world/level/block/Blocks#MOSS_BLOCK` | field | 2 |
| `net/minecraft/world/level/block/Blocks#MUD` | field | 1 |
| `net/minecraft/world/level/block/Blocks#OAK_PLANKS` | field | 2 |
| `net/minecraft/world/level/block/Blocks#OAK_SLAB` | field | 1 |
| `net/minecraft/world/level/block/Blocks#PODZOL` | field | 1 |
| `net/minecraft/world/level/block/Blocks#ROOTED_DIRT` | field | 2 |
| `net/minecraft/world/level/block/Blocks#SNOW` | field | 1 |
| `net/minecraft/world/level/block/Blocks#SOUL_LANTERN` | field | 1 |
| `net/minecraft/world/level/block/Blocks#SOUL_TORCH` | field | 1 |
| `net/minecraft/world/level/block/Blocks#SOUL_WALL_TORCH` | field | 1 |
| `net/minecraft/world/level/block/Blocks#STONE` | field | 1 |
| `net/minecraft/world/level/block/Blocks#STRUCTURE_VOID` | field | 1 |
| `net/minecraft/world/level/block/Blocks#TINTED_GLASS` | field | 1 |
| `net/minecraft/world/level/block/Blocks#TORCH` | field | 1 |
| `net/minecraft/world/level/block/Blocks#TUFF` | field | 1 |
| `net/minecraft/world/level/block/Blocks#WALL_TORCH` | field | 1 |
| `net/minecraft/world/level/block/Blocks#WATER` | field | 3 |
| `net/minecraft/world/level/block/BonemealableFeaturePlacerBlock#<init>` | method | 1 |
| `net/minecraft/world/level/block/BonemealableFeaturePlacerBlock#animateTick` | method | 1 |
| `net/minecraft/world/level/block/CampfireBlock#makeParticles` | method | 1 |
| `net/minecraft/world/level/block/ChestBlock#FACING` | field | 1 |
| `net/minecraft/world/level/block/ColorCollection#black` | method | 1 |
| `net/minecraft/world/level/block/ColorCollection#gray` | method | 1 |
| `net/minecraft/world/level/block/CrafterBlock#getPotentialResults` | method | 1 |
| `net/minecraft/world/level/block/DoorBlock#<init>` | method | 2 |
| `net/minecraft/world/level/block/DoorBlock#createBlockStateDefinition` | method | 1 |
| `net/minecraft/world/level/block/DoorBlock#useItemOn` | method | 1 |
| `net/minecraft/world/level/block/DoorBlock#useWithoutItem` | method | 1 |
| `net/minecraft/world/level/block/DoubleBlockCombiner#combineWithNeigbour` | method | 1 |
| `net/minecraft/world/level/block/DoubleBlockCombiner$BlockType#FIRST` | field | 1 |
| `net/minecraft/world/level/block/DoubleBlockCombiner$BlockType#SECOND` | field | 1 |
| `net/minecraft/world/level/block/DoubleBlockCombiner$NeighborCombineResult#apply` | method | 1 |
| `net/minecraft/world/level/block/FenceBlock#<init>` | method | 1 |
| `net/minecraft/world/level/block/FenceBlock#connectsTo` | method | 1 |
| `net/minecraft/world/level/block/FenceBlock#createBlockStateDefinition` | method | 1 |
| `net/minecraft/world/level/block/FenceBlock#useItemOn` | method | 1 |
| `net/minecraft/world/level/block/FenceGateBlock#<init>` | method | 1 |
| `net/minecraft/world/level/block/FenceGateBlock#createBlockStateDefinition` | method | 1 |
| `net/minecraft/world/level/block/FenceGateBlock#useItemOn` | method | 1 |
| `net/minecraft/world/level/block/GrassBlock#<init>` | method | 1 |
| `net/minecraft/world/level/block/HorizontalDirectionalBlock#FACING` | field | 11 |
| `net/minecraft/world/level/block/IronBarsBlock#<init>` | method | 1 |
| `net/minecraft/world/level/block/IronBarsBlock#attachsTo` | method | 1 |
| `net/minecraft/world/level/block/LadderBlock#FACING` | field | 1 |
| `net/minecraft/world/level/block/LightBlock#LEVEL` | field | 1 |
| `net/minecraft/world/level/block/Mirror#getRotation` | method | 6 |
| `net/minecraft/world/level/block/Mirror#mirror` | method | 2 |
| `net/minecraft/world/level/block/PipeBlock#PROPERTY_BY_DIRECTION` | field | 2 |
| `net/minecraft/world/level/block/RenderShape#INVISIBLE` | field | 6 |
| `net/minecraft/world/level/block/RenderShape#MODEL` | field | 3 |
| `net/minecraft/world/level/block/RotatedPillarBlock#<init>` | method | 2 |
| `net/minecraft/world/level/block/RotatedPillarBlock#createBlockStateDefinition` | method | 1 |
| `net/minecraft/world/level/block/Rotation#rotate` | method | 13 |
| `net/minecraft/world/level/block/SignBlock#WATERLOGGED` | field | 1 |
| `net/minecraft/world/level/block/SlabBlock#<init>` | method | 1 |
| `net/minecraft/world/level/block/SlabBlock#createBlockStateDefinition` | method | 1 |
| `net/minecraft/world/level/block/SlabBlock#placeLiquid` | method | 1 |
| `net/minecraft/world/level/block/SlabBlock#useItemOn` | method | 1 |
| `net/minecraft/world/level/block/SoundType#BASALT` | field | 1 |
| `net/minecraft/world/level/block/SoundType#BONE_BLOCK` | field | 1 |
| `net/minecraft/world/level/block/SoundType#DEEPSLATE` | field | 1 |
| `net/minecraft/world/level/block/SoundType#DEEPSLATE_BRICKS` | field | 1 |
| `net/minecraft/world/level/block/SoundType#GLASS` | field | 1 |
| `net/minecraft/world/level/block/SoundType#METAL` | field | 1 |
| `net/minecraft/world/level/block/SoundType#WOOD` | field | 1 |
| `net/minecraft/world/level/block/SoundType#WOOL` | field | 1 |
| `net/minecraft/world/level/block/SoundType#getBreakSound` | method | 1 |
| `net/minecraft/world/level/block/SoundType#getPitch` | method | 1 |
| `net/minecraft/world/level/block/SoundType#getStepSound` | method | 1 |
| `net/minecraft/world/level/block/SoundType#getVolume` | method | 1 |
| `net/minecraft/world/level/block/StairBlock#<init>` | method | 2 |
| `net/minecraft/world/level/block/StairBlock#createBlockStateDefinition` | method | 1 |
| `net/minecraft/world/level/block/StairBlock#placeLiquid` | method | 1 |
| `net/minecraft/world/level/block/StairBlock#useItemOn` | method | 1 |
| `net/minecraft/world/level/block/StandingSignBlock#<init>` | method | 1 |
| `net/minecraft/world/level/block/TrapDoorBlock#<init>` | method | 2 |
| `net/minecraft/world/level/block/TrapDoorBlock#createBlockStateDefinition` | method | 1 |
| `net/minecraft/world/level/block/TrapDoorBlock#placeLiquid` | method | 1 |
| `net/minecraft/world/level/block/TrapDoorBlock#useItemOn` | method | 1 |
| `net/minecraft/world/level/block/TrapDoorBlock#useWithoutItem` | method | 1 |
| `net/minecraft/world/level/block/WallBlock#UP` | field | 1 |
| `net/minecraft/world/level/block/WallTorchBlock#FACING` | field | 1 |
| `net/minecraft/world/level/block/entity/BellBlockEntity#onHit` | method | 1 |
| `net/minecraft/world/level/block/entity/BlockEntity#<init>` | method | 5 |
| `net/minecraft/world/level/block/entity/BlockEntity#getBlockPos` | method | 2 |
| `net/minecraft/world/level/block/entity/BlockEntity#getLevel` | method | 1 |
| `net/minecraft/world/level/block/entity/BlockEntity#loadAdditional` | method | 2 |
| `net/minecraft/world/level/block/entity/BlockEntity#saveAdditional` | method | 2 |
| `net/minecraft/world/level/block/entity/BlockEntityType#<init>` | method | 2 |
| `net/minecraft/world/level/block/entity/BlockEntityType#isValid` | method | 1 |
| `net/minecraft/world/level/block/entity/BlockEntityType#onlyOpCanSetNbt` | method | 1 |
| `net/minecraft/world/level/block/entity/ContainerOpenersCounter#<init>` | method | 2 |
| `net/minecraft/world/level/block/entity/ContainerOpenersCounter#decrementOpeners` | method | 2 |
| `net/minecraft/world/level/block/entity/ContainerOpenersCounter#incrementOpeners` | method | 2 |
| `net/minecraft/world/level/block/entity/ContainerOpenersCounter#recheckOpeners` | method | 2 |
| `net/minecraft/world/level/block/entity/LecternBlockEntity#getBook` | method | 1 |
| `net/minecraft/world/level/block/entity/LecternBlockEntity#hasBook` | method | 1 |
| `net/minecraft/world/level/block/entity/LidBlockEntity#getOpenNess` | method | 1 |
| `net/minecraft/world/level/block/entity/RandomizableContainerBlockEntity#<init>` | method | 2 |
| `net/minecraft/world/level/block/entity/RandomizableContainerBlockEntity#applyImplicitComponents` | method | 2 |
| `net/minecraft/world/level/block/entity/RandomizableContainerBlockEntity#collectImplicitComponents` | method | 2 |
| `net/minecraft/world/level/block/entity/RandomizableContainerBlockEntity#getCustomName` | method | 2 |
| `net/minecraft/world/level/block/entity/RandomizableContainerBlockEntity#getName` | method | 2 |
| `net/minecraft/world/level/block/entity/RandomizableContainerBlockEntity#loadAdditional` | method | 2 |
| `net/minecraft/world/level/block/entity/RandomizableContainerBlockEntity#saveAdditional` | method | 2 |
| `net/minecraft/world/level/block/entity/RandomizableContainerBlockEntity#triggerEvent` | method | 1 |
| `net/minecraft/world/level/block/entity/SignText#CODEC` | field | 1 |
| `net/minecraft/world/level/block/entity/SignText#EMPTY` | field | 1 |
| `net/minecraft/world/level/block/entity/SignText#asMutable` | method | 2 |
| `net/minecraft/world/level/block/entity/SignText#getColor` | method | 2 |
| `net/minecraft/world/level/block/entity/SignText#getMessages` | method | 3 |
| `net/minecraft/world/level/block/entity/SignText#getRenderMessages` | method | 1 |
| `net/minecraft/world/level/block/entity/SignText#hasGlowingText` | method | 1 |
| `net/minecraft/world/level/block/entity/SignText#withColor` | method | 1 |
| `net/minecraft/world/level/block/entity/SignText#withGlowingText` | method | 1 |
| `net/minecraft/world/level/block/entity/SignText$Mutable#asImmutable` | method | 2 |
| `net/minecraft/world/level/block/entity/SignText$Mutable#setLine` | method | 3 |
| `net/minecraft/world/level/block/entity/SignTextSlot#FRONT` | field | 2 |
| `net/minecraft/world/level/block/state/BlockBehaviour$Properties#forceSolidOn` | method | 1 |
| `net/minecraft/world/level/block/state/BlockBehaviour$Properties#ignitedByLava` | method | 1 |
| `net/minecraft/world/level/block/state/BlockBehaviour$Properties#instabreak` | method | 1 |
| `net/minecraft/world/level/block/state/BlockBehaviour$Properties#instrument` | method | 1 |
| `net/minecraft/world/level/block/state/BlockBehaviour$Properties#isValidSpawn` | method | 1 |
| `net/minecraft/world/level/block/state/BlockBehaviour$Properties#lightLevel` | method | 2 |
| `net/minecraft/world/level/block/state/BlockBehaviour$Properties#mapColor` | method | 1 |
| `net/minecraft/world/level/block/state/BlockBehaviour$Properties#noCollision` | method | 1 |
| `net/minecraft/world/level/block/state/BlockBehaviour$Properties#noLootTable` | method | 1 |
| `net/minecraft/world/level/block/state/BlockBehaviour$Properties#noOcclusion` | method | 2 |
| `net/minecraft/world/level/block/state/BlockBehaviour$Properties#of` | method | 2 |
| `net/minecraft/world/level/block/state/BlockBehaviour$Properties#ofFullCopy` | method | 1 |
| `net/minecraft/world/level/block/state/BlockBehaviour$Properties#overrideLootTable` | method | 1 |
| `net/minecraft/world/level/block/state/BlockBehaviour$Properties#pushReaction` | method | 1 |
| `net/minecraft/world/level/block/state/BlockBehaviour$Properties#requiresCorrectToolForDrops` | method | 1 |
| `net/minecraft/world/level/block/state/BlockBehaviour$Properties#setId` | method | 1 |
| `net/minecraft/world/level/block/state/BlockBehaviour$Properties#sound` | method | 2 |
| `net/minecraft/world/level/block/state/BlockBehaviour$Properties#strength` | method | 4 |
| `net/minecraft/world/level/block/state/BlockState#cycle` | method | 2 |
| `net/minecraft/world/level/block/state/BlockState#getBlock` | method | 20 |
| `net/minecraft/world/level/block/state/BlockState#getFluidState` | method | 3 |
| `net/minecraft/world/level/block/state/BlockState#getMenuProvider` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#getRenderShape` | method | 3 |
| `net/minecraft/world/level/block/state/BlockState#getSoundType` | method | 2 |
| `net/minecraft/world/level/block/state/BlockState#getValue` | method | 30 |
| `net/minecraft/world/level/block/state/BlockState#getValueOrElse` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#hasProperty` | method | 5 |
| `net/minecraft/world/level/block/state/BlockState#ignitedByLava` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#is` | method | 24 |
| `net/minecraft/world/level/block/state/BlockState#isAir` | method | 11 |
| `net/minecraft/world/level/block/state/BlockState#isCollisionShapeFullBlock` | method | 2 |
| `net/minecraft/world/level/block/state/BlockState#isFaceSturdy` | method | 7 |
| `net/minecraft/world/level/block/state/BlockState#isRedstoneConductor` | method | 3 |
| `net/minecraft/world/level/block/state/BlockState#isSolid` | method | 2 |
| `net/minecraft/world/level/block/state/BlockState#isSolidRender` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#mirror` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#rotate` | method | 7 |
| `net/minecraft/world/level/block/state/BlockState#setValue` | method | 31 |
| `net/minecraft/world/level/block/state/BlockState#updateNeighbourShapes` | method | 1 |
| `net/minecraft/world/level/block/state/StateDefinition#any` | method | 15 |
| `net/minecraft/world/level/block/state/StateDefinition$Builder#add` | method | 24 |
| `net/minecraft/world/level/block/state/pattern/BlockInWorld#<init>` | method | 1 |
| `net/minecraft/world/level/block/state/pattern/BlockInWorld#hasState` | method | 1 |
| `net/minecraft/world/level/block/state/pattern/BlockPattern$BlockPatternMatch#getUp` | method | 1 |
| `net/minecraft/world/level/block/state/pattern/BlockPatternBuilder#aisle` | method | 1 |
| `net/minecraft/world/level/block/state/pattern/BlockPatternBuilder#build` | method | 1 |
| `net/minecraft/world/level/block/state/pattern/BlockPatternBuilder#start` | method | 1 |
| `net/minecraft/world/level/block/state/pattern/BlockPatternBuilder#where` | method | 1 |
| `net/minecraft/world/level/block/state/predicate/BlockStatePredicate#ANY` | field | 1 |
| `net/minecraft/world/level/block/state/predicate/BlockStatePredicate#forBlock` | method | 1 |
| `net/minecraft/world/level/block/state/predicate/BlockStatePredicate#or` | method | 1 |
| `net/minecraft/world/level/block/state/properties/BedPart#FOOT` | field | 2 |
| `net/minecraft/world/level/block/state/properties/BedPart#HEAD` | field | 2 |
| `net/minecraft/world/level/block/state/properties/BellAttachType#CEILING` | field | 1 |
| `net/minecraft/world/level/block/state/properties/BellAttachType#FLOOR` | field | 1 |
| `net/minecraft/world/level/block/state/properties/BlockSetType#IRON` | field | 2 |
| `net/minecraft/world/level/block/state/properties/BlockSetType#OAK` | field | 1 |
| `net/minecraft/world/level/block/state/properties/BlockStateProperties#AXIS` | field | 1 |
| `net/minecraft/world/level/block/state/properties/BlockStateProperties#BED_PART` | field | 1 |
| `net/minecraft/world/level/block/state/properties/BlockStateProperties#BELL_ATTACHMENT` | field | 1 |
| `net/minecraft/world/level/block/state/properties/BlockStateProperties#BOTTOM` | field | 1 |
| `net/minecraft/world/level/block/state/properties/BlockStateProperties#CANDLES` | field | 1 |
| `net/minecraft/world/level/block/state/properties/BlockStateProperties#DOUBLE_BLOCK_HALF` | field | 3 |
| `net/minecraft/world/level/block/state/properties/BlockStateProperties#EAST_WALL` | field | 1 |
| `net/minecraft/world/level/block/state/properties/BlockStateProperties#HALF` | field | 1 |
| `net/minecraft/world/level/block/state/properties/BlockStateProperties#HANGING` | field | 2 |
| `net/minecraft/world/level/block/state/properties/BlockStateProperties#HORIZONTAL_FACING` | field | 2 |
| `net/minecraft/world/level/block/state/properties/BlockStateProperties#LIT` | field | 2 |
| `net/minecraft/world/level/block/state/properties/BlockStateProperties#LOCKED` | field | 1 |
| `net/minecraft/world/level/block/state/properties/BlockStateProperties#NORTH_WALL` | field | 1 |
| `net/minecraft/world/level/block/state/properties/BlockStateProperties#OPEN` | field | 2 |
| `net/minecraft/world/level/block/state/properties/BlockStateProperties#ROTATION_16` | field | 1 |
| `net/minecraft/world/level/block/state/properties/BlockStateProperties#SLAB_TYPE` | field | 1 |
| `net/minecraft/world/level/block/state/properties/BlockStateProperties#SOUTH_WALL` | field | 1 |
| `net/minecraft/world/level/block/state/properties/BlockStateProperties#STAIRS_SHAPE` | field | 1 |
| `net/minecraft/world/level/block/state/properties/BlockStateProperties#WATERLOGGED` | field | 16 |
| `net/minecraft/world/level/block/state/properties/BlockStateProperties#WEST_WALL` | field | 1 |
| `net/minecraft/world/level/block/state/properties/BooleanProperty#create` | method | 4 |
| `net/minecraft/world/level/block/state/properties/DoubleBlockHalf#LOWER` | field | 4 |
| `net/minecraft/world/level/block/state/properties/DoubleBlockHalf#UPPER` | field | 3 |
| `net/minecraft/world/level/block/state/properties/EnumProperty#create` | method | 2 |
| `net/minecraft/world/level/block/state/properties/Half#BOTTOM` | field | 1 |
| `net/minecraft/world/level/block/state/properties/IntegerProperty#create` | method | 1 |
| `net/minecraft/world/level/block/state/properties/NoteBlockInstrument#BASS` | field | 1 |
| `net/minecraft/world/level/block/state/properties/SlabType#BOTTOM` | field | 1 |
| `net/minecraft/world/level/block/state/properties/StairsShape#STRAIGHT` | field | 1 |
| `net/minecraft/world/level/block/state/properties/WallSide#NONE` | field | 1 |
| `net/minecraft/world/level/block/state/properties/WoodType#OAK` | field | 2 |
| `net/minecraft/world/level/border/WorldBorder#isWithinBounds` | method | 1 |
| `net/minecraft/world/level/chunk/ChunkGenerator#getFirstOccupiedHeight` | method | 1 |
| `net/minecraft/world/level/chunk/ChunkSource#getChunkNow` | method | 1 |
| `net/minecraft/world/level/chunk/LevelChunk#getSection` | method | 1 |
| `net/minecraft/world/level/chunk/LevelChunk#getSectionIndex` | method | 1 |
| `net/minecraft/world/level/chunk/LevelChunk#getSectionYFromSectionIndex` | method | 1 |
| `net/minecraft/world/level/chunk/LevelChunkSection#getBlockState` | method | 1 |
| `net/minecraft/world/level/chunk/LevelChunkSection#hasOnlyAir` | method | 1 |
| `net/minecraft/world/level/chunk/LevelChunkSection#maybeHas` | method | 1 |
| `net/minecraft/world/level/chunk/storage/SimpleRegionStorage#upgradeChunkTag` | method | 2 |
| `net/minecraft/world/level/dimension/DimensionType#hasFixedTime` | method | 2 |
| `net/minecraft/world/level/gameevent/GameEvent#BLOCK_CHANGE` | field | 3 |
| `net/minecraft/world/level/gameevent/GameEvent$Context#of` | method | 2 |
| `net/minecraft/world/level/gamerules/GameRules#MOB_GRIEFING` | field | 2 |
| `net/minecraft/world/level/gamerules/GameRules#SPAWN_MOBS` | field | 1 |
| `net/minecraft/world/level/gamerules/GameRules#get` | method | 3 |
| `net/minecraft/world/level/levelgen/Heightmap$Types#CODEC` | field | 1 |
| `net/minecraft/world/level/levelgen/Heightmap$Types#MOTION_BLOCKING` | field | 3 |
| `net/minecraft/world/level/levelgen/Heightmap$Types#MOTION_BLOCKING_NO_LEAVES` | field | 4 |
| `net/minecraft/world/level/levelgen/Heightmap$Types#WORLD_SURFACE_WG` | field | 3 |
| `net/minecraft/world/level/levelgen/PositionalRandomFactory#at` | method | 1 |
| `net/minecraft/world/level/levelgen/WorldgenRandom#nextDouble` | method | 1 |
| `net/minecraft/world/level/levelgen/feature/TreeFeature#doPlace` | method | 1 |
| `net/minecraft/world/level/levelgen/heightproviders/HeightProvider#CODEC` | field | 1 |
| `net/minecraft/world/level/levelgen/structure/BoundingBox#maxX` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/BoundingBox#maxZ` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/BoundingBox#minX` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/BoundingBox#minZ` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/Structure#<init>` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/Structure$GenerationContext#chunkGenerator` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/Structure$GenerationContext#chunkPos` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/Structure$GenerationContext#heightAccessor` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/Structure$GenerationContext#isValidBiome` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/Structure$GenerationContext#random` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/Structure$GenerationContext#randomState` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/Structure$StructureSettings#CODEC` | field | 1 |
| `net/minecraft/world/level/levelgen/structure/pools/DimensionPadding#ZERO` | field | 1 |
| `net/minecraft/world/level/levelgen/structure/pools/JigsawPlacement#addPieces` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/pools/SinglePoolElement#getSettings` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/pools/SinglePoolElement#getShuffledJigsawBlocks` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/pools/StructureTemplatePool#CODEC` | field | 1 |
| `net/minecraft/world/level/levelgen/structure/pools/alias/PoolAliasLookup#EMPTY` | field | 1 |
| `net/minecraft/world/level/levelgen/structure/structures/JigsawStructure$MaxDistance#<init>` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/LiquidSettings#APPLY_WATERLOGGING` | field | 1 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/LiquidSettings#IGNORE_WATERLOGGING` | field | 1 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/StructurePlaceSettings#getBoundingBox` | method | 2 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/StructurePlaceSettings#getMirror` | method | 2 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/StructurePlaceSettings#getRotation` | method | 2 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/StructurePlaceSettings#setLiquidSettings` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/StructureProcessorList#list` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/StructureTemplate#calculateRelativePosition` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/StructureTemplate$JigsawBlockInfo#<init>` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/StructureTemplate$JigsawBlockInfo#jointType` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/StructureTemplate$JigsawBlockInfo#name` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/StructureTemplate$JigsawBlockInfo#placementPriority` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/StructureTemplate$JigsawBlockInfo#pool` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/StructureTemplate$JigsawBlockInfo#pos` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/StructureTemplate$JigsawBlockInfo#selectionPriority` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/StructureTemplate$JigsawBlockInfo#state` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/StructureTemplate$JigsawBlockInfo#target` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/StructureTemplate$StructureBlockInfo#<init>` | method | 3 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/StructureTemplate$StructureBlockInfo#nbt` | method | 3 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/StructureTemplate$StructureBlockInfo#pos` | method | 3 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/StructureTemplate$StructureBlockInfo#state` | method | 3 |
| `net/minecraft/world/level/material/FlowingFluid#getSource` | method | 11 |
| `net/minecraft/world/level/material/FlowingFluid#getTickDelay` | method | 7 |
| `net/minecraft/world/level/material/Fluid#getTickDelay` | method | 1 |
| `net/minecraft/world/level/material/FluidState#createLegacyBlock` | method | 1 |
| `net/minecraft/world/level/material/FluidState#getType` | method | 13 |
| `net/minecraft/world/level/material/FluidState#is` | method | 3 |
| `net/minecraft/world/level/material/Fluids#WATER` | field | 14 |
| `net/minecraft/world/level/material/MapColor#COLOR_BLACK` | field | 1 |
| `net/minecraft/world/level/material/MapColor#COLOR_BROWN` | field | 1 |
| `net/minecraft/world/level/material/PushReaction#POPPED` | field | 1 |
| `net/minecraft/world/level/pathfinder/Node#distanceToSqr` | method | 1 |
| `net/minecraft/world/level/pathfinder/NodeEvaluator#setCanPassDoors` | method | 2 |
| `net/minecraft/world/level/pathfinder/Path#canReach` | method | 1 |
| `net/minecraft/world/level/pathfinder/Path#getEndNode` | method | 1 |
| `net/minecraft/world/level/pathfinder/PathType#COCOA` | field | 1 |
| `net/minecraft/world/level/pathfinder/PathType#FENCE` | field | 1 |
| `net/minecraft/world/level/pathfinder/PathType#FIRE` | field | 2 |
| `net/minecraft/world/level/pathfinder/PathType#FIRE_IN_NEIGHBOR` | field | 1 |
| `net/minecraft/world/level/pathfinder/PathType#ON_TOP_OF_POWDER_SNOW` | field | 1 |
| `net/minecraft/world/level/pathfinder/PathType#POWDER_SNOW` | field | 1 |
| `net/minecraft/world/level/pathfinder/PathType#WALKABLE` | field | 1 |
| `net/minecraft/world/level/pathfinder/PathType#WATER` | field | 3 |
| `net/minecraft/world/level/pathfinder/PathType#WATER_BORDER` | field | 1 |
| `net/minecraft/world/level/saveddata/SavedData#<init>` | method | 2 |
| `net/minecraft/world/level/saveddata/SavedDataType#<init>` | method | 2 |
| `net/minecraft/world/level/storage/DataVersion#version` | method | 2 |
| `net/minecraft/world/level/storage/LevelResource#DATA` | field | 2 |
| `net/minecraft/world/level/storage/LevelResource#ROOT` | field | 1 |
| `net/minecraft/world/level/storage/SavedDataStorage#get` | method | 2 |
| `net/minecraft/world/level/storage/SavedDataStorage#set` | method | 2 |
| `net/minecraft/world/level/storage/TagValueInput#create` | method | 1 |
| `net/minecraft/world/level/storage/TagValueOutput#buildResult` | method | 2 |
| `net/minecraft/world/level/storage/TagValueOutput#createWithContext` | method | 2 |
| `net/minecraft/world/level/storage/ValueInput#child` | method | 2 |
| `net/minecraft/world/level/storage/ValueInput#getBooleanOr` | method | 7 |
| `net/minecraft/world/level/storage/ValueInput#getByteOr` | method | 1 |
| `net/minecraft/world/level/storage/ValueInput#getFloatOr` | method | 2 |
| `net/minecraft/world/level/storage/ValueInput#getInt` | method | 5 |
| `net/minecraft/world/level/storage/ValueInput#getIntArray` | method | 1 |
| `net/minecraft/world/level/storage/ValueInput#getIntOr` | method | 6 |
| `net/minecraft/world/level/storage/ValueInput#getLong` | method | 3 |
| `net/minecraft/world/level/storage/ValueInput#getLongOr` | method | 1 |
| `net/minecraft/world/level/storage/ValueInput#getString` | method | 2 |
| `net/minecraft/world/level/storage/ValueInput#getStringOr` | method | 1 |
| `net/minecraft/world/level/storage/ValueInput#list` | method | 1 |
| `net/minecraft/world/level/storage/ValueInput#listOrEmpty` | method | 2 |
| `net/minecraft/world/level/storage/ValueInput#lookup` | method | 5 |
| `net/minecraft/world/level/storage/ValueInput#read` | method | 7 |
| `net/minecraft/world/level/storage/ValueInput$TypedInputList#forEach` | method | 1 |
| `net/minecraft/world/level/storage/ValueInput$TypedInputList#iterator` | method | 2 |
| `net/minecraft/world/level/storage/ValueInput$TypedInputList#stream` | method | 1 |
| `net/minecraft/world/level/storage/ValueOutput#child` | method | 1 |
| `net/minecraft/world/level/storage/ValueOutput#list` | method | 1 |
| `net/minecraft/world/level/storage/ValueOutput#putBoolean` | method | 7 |
| `net/minecraft/world/level/storage/ValueOutput#putByte` | method | 3 |
| `net/minecraft/world/level/storage/ValueOutput#putFloat` | method | 1 |
| `net/minecraft/world/level/storage/ValueOutput#putInt` | method | 7 |
| `net/minecraft/world/level/storage/ValueOutput#putLong` | method | 4 |
| `net/minecraft/world/level/storage/ValueOutput#putString` | method | 1 |
| `net/minecraft/world/level/storage/ValueOutput#store` | method | 6 |
| `net/minecraft/world/level/storage/ValueOutput$TypedOutputList#add` | method | 1 |
| `net/minecraft/world/level/storage/loot/LootParams$Builder#getOptionalParameter` | method | 1 |
| `net/minecraft/world/level/storage/loot/parameters/LootContextParams#BLOCK_ENTITY` | field | 1 |
| `net/minecraft/world/level/storage/loot/predicates/LootItemCondition#CODEC` | field | 1 |
| `net/minecraft/world/phys/AABB#<init>` | method | 6 |
| `net/minecraft/world/phys/AABB#clip` | method | 1 |
| `net/minecraft/world/phys/AABB#expandTowards` | method | 2 |
| `net/minecraft/world/phys/AABB#getSize` | method | 1 |
| `net/minecraft/world/phys/AABB#inflate` | method | 13 |
| `net/minecraft/world/phys/AABB#move` | method | 1 |
| `net/minecraft/world/phys/BlockHitResult#getBlockPos` | method | 3 |
| `net/minecraft/world/phys/EntityHitResult#getEntity` | method | 2 |
| `net/minecraft/world/phys/HitResult#getLocation` | method | 1 |
| `net/minecraft/world/phys/HitResult#getType` | method | 1 |
| `net/minecraft/world/phys/HitResult$Type#ENTITY` | field | 1 |
| `net/minecraft/world/phys/HitResult$Type#MISS` | field | 1 |
| `net/minecraft/world/phys/Vec2#ZERO` | field | 1 |
| `net/minecraft/world/phys/Vec3#<init>` | method | 10 |
| `net/minecraft/world/phys/Vec3#add` | method | 6 |
| `net/minecraft/world/phys/Vec3#atBottomCenterOf` | method | 2 |
| `net/minecraft/world/phys/Vec3#atCenterOf` | method | 5 |
| `net/minecraft/world/phys/Vec3#cross` | method | 1 |
| `net/minecraft/world/phys/Vec3#distanceToSqr` | method | 1 |
| `net/minecraft/world/phys/Vec3#dot` | method | 1 |
| `net/minecraft/world/phys/Vec3#length` | method | 4 |
| `net/minecraft/world/phys/Vec3#lengthSqr` | method | 1 |
| `net/minecraft/world/phys/Vec3#lerp` | method | 1 |
| `net/minecraft/world/phys/Vec3#reverse` | method | 1 |
| `net/minecraft/world/phys/Vec3#scale` | method | 5 |
| `net/minecraft/world/phys/Vec3#subtract` | method | 7 |
| `net/minecraft/world/phys/Vec3#x` | field | 20 |
| `net/minecraft/world/phys/Vec3#y` | field | 15 |
| `net/minecraft/world/phys/Vec3#z` | field | 20 |
| `net/minecraft/world/phys/shapes/VoxelShape#isEmpty` | method | 6 |
| `net/minecraft/world/phys/shapes/VoxelShape#max` | method | 2 |

</details>

