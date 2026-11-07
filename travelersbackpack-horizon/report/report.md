# Horizon port report: Traveler's Backpack

`travelersbackpack` 11.4.3 · travelersbackpack-fabric-26.3-11.4.3.jar · 358 classes

**Partly convertible**

| Status | Count | Meaning |
|---|---|---|
| auto | 10 | The Horizon hook or injection is known: the wiring can be generated. The Java body still has to become Rust. |
| content | 10 | Data and assets: horizon-port-pack carries these into a content mod (.hmod). |
| needs-mapping | 22 | The kind of change carries over, but the matching Horizon function isn't in the table yet. Run `match`. |
| needs-translation | 28 | No direct equivalent: a person (or an AI agent) has to rewrite this part. |
| unsupported | 15 | Horizon has nothing this can attach to yet. |

## Entry points

- `fabric-datagen`: com.tiviacz.travelersbackpack.datagen.ModDataGeneration
- `main`: com.tiviacz.travelersbackpack.TravelersBackpack
- `client`: com.tiviacz.travelersbackpack.TravelersBackpackClient
- `jei_mod_plugin`: com.tiviacz.travelersbackpack.compat.jei.TravelersBackpackPlugin
- `lambdynlights:initializer`: com.tiviacz.travelersbackpack.compat.lambdynamiclight.BackpackDynamicLight

## Fabric events

| Event | In | Horizon | Status | Note |
|---|---|---|---|---|
| EntityRendererRegistry.register | `TravelersBackpackClient` | – | unsupported | Custom entity renderers aren't supported. |
| HudElementRegistry.attachElementBefore | `TravelersBackpackClient` | hook `hud` | auto |  |
| ClientCommandRegistrationCallback.EVENT | `TravelersBackpackClient` | hook `command` | auto | The Fabric API layer's command engine runs the Brigadier tree: literals, integer/long/float/double/bool/string arguments with bounds, executes; requires and suggests are left out. |
| ServerPlayNetworking.send | `AttachmentUtils` | – | unsupported | Custom packets aren't supported. |
| PayloadTypeRegistry.serverboundPlay | `VinURLNetwork` | – | unsupported | Horizon speaks the vanilla protocol; custom packets aren't supported. |
| PayloadTypeRegistry.register | `VinURLNetwork` | – | unsupported | Horizon speaks the vanilla protocol; custom packets aren't supported. |
| ServerPlayNetworking.registerGlobalReceiver | `VinURLNetwork` | – | unsupported | Custom packets aren't supported. |
| KeyMappingHelper.registerKeyMapping | `KeybindHandler` | hook `key` | auto | Mojang-named variant of KeyBindingHelper. |
| ClientTickEvents.START_CLIENT_TICK | `KeybindHandler` | hook `tick` | auto | The Fabric API layer (jfabricapi.rs) runs START_CLIENT_TICK, then the level ticks (START/END_LEVEL_TICK, while in a world), then END_CLIENT_TICK, from Horizon's tick hook, in the window's Game only. |
| ScreenEvents.BEFORE_INIT | `KeybindHandler` | – | unsupported | No screen hook in Horizon yet. |
| UseBlockCallback.EVENT | `RightClickHandler` | – | unsupported | No interaction hook yet: needs a raw target in Horizon's game code. |
| ScreenEvents.AFTER_INIT | `ScreenRenderHandler` | – | unsupported | No screen hook in Horizon yet. |
| ServerTickEvents.END_SERVER_TICK | `TickHandler` | hook `tick` | auto | The tick hook runs in every Game, the server's too: run server logic only when `!self.native_window()`. |
| FabricBlockEntityTypeBuilder.create | `ModBlockEntityTypes` | – | unsupported | Block entities aren't supported in Horizon mods yet. |
| FabricBlockEntityTypeBuilder.build | `ModBlockEntityTypes` | – | unsupported | Block entities aren't supported in Horizon mods yet. |
| CommandRegistrationCallback.EVENT | `ModCommands` | hook `command` | needs-translation | Server commands. Horizon's command hook is client-side. |
| ClientPlayNetworking.registerGlobalReceiver | `ModNetwork` | – | unsupported | Custom packets aren't supported. |
| ClientPlayConnectionEvents.JOIN | `ModNetwork` | hook `tick` | auto | The Fabric API layer runs JOIN on the window's first tick with a player after none, DISCONNECT on the first tick without one after. |
| PayloadTypeRegistry.clientboundPlay | `ModNetwork` | – | unsupported | Horizon speaks the vanilla protocol; custom packets aren't supported. |
| ClientPlayNetworking.send | `PacketDistributor` | – | unsupported | Custom packets aren't supported. |

## Mixins

| Injection | In | Horizon | Status | Note |
|---|---|---|---|---|
| @Inject(TAIL) on AbstractContainerScreen.extractContents | `AbstractContainerScreenMixin.renderContents` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for AbstractContainerScreen.extractContents: run `match`. |
| @Inject(HEAD) on Hud.extractCrosshair | `HudMixin.onExtractCrosshair` | HEAD | needs-mapping | No Horizon function known for Hud.extractCrosshair: run `match`. |
| mixin into HumanoidRenderState with no injectors (interfaces or @Shadow only) | `HumanoidRenderStateMixin` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| @Inject(TAIL) on HumanoidMobRenderer.extractHumanoidRenderState | `HumoanoidMobRenderer.render` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for HumanoidMobRenderer.extractHumanoidRenderState: run `match`. |
| @Inject(TAIL) on ItemStack.overrideOtherStackedOnMe | `ItemStackMixin.afterOverrideOtherStackedOnMe` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for ItemStack.overrideOtherStackedOnMe: run `match`. |
| @Invoker on LevelResource.<init> | `LevelResourceMixin.invokeInit` | INVOKE | needs-mapping | A [[bridge]] INVOKE of the Rust method. No Horizon function known for LevelResource.<init>: run `match`. |
| @Inject(HEAD) on LivingEntity.dropAllDeathLoot | `LivingEntityPriorityMixin.drop` | HEAD | needs-mapping | No Horizon function known for LivingEntity.dropAllDeathLoot: run `match`. |
| @Inject(INVOKE) on LocalPlayer.aiStep at Lnet/minecraft/client/player/ClientInput;tick()V | `LocalPlayerMixin.afterInputTick` | AFTER_CALL | needs-mapping | No Horizon function known for LocalPlayer.aiStep: run `match`. The call it hooks (ClientInput.tick) isn't mapped to a Rust function yet. |
| @Inject(TAIL) on Mob.finalizeSpawn | `MobEntityMixin.initialize` | RETURN on horizon_client::mobs::creaking_heart::finalize_spawn | auto | Horizon has no TAIL: RETURN runs at every exit, early ones too. |
| @Inject(TAIL) on Player.tick | `PlayerMixin.abilityTick` | RETURN on horizon_client::attack_strength::AttackStrength::tick | auto | Horizon has no TAIL: RETURN runs at every exit, early ones too. |
| @Inject(HEAD) on Player.attack | `PlayerMixin.attack` | HEAD on horizon_client::game::Game<'_>::attack | auto |  |
| @Inject(RETURN) on Player.getFlyingSpeed | `PlayerMixin.getFlyingSpeed` | RETURN | needs-mapping | Horizon's RETURN runs at every normal exit and passes the return value on. No Horizon function known for Player.getFlyingSpeed: run `match`. |
| @Inject(HEAD) on Player.readAdditionalSaveData | `PlayerMixin.fromTag` | HEAD | needs-mapping | No Horizon function known for Player.readAdditionalSaveData: run `match`. |
| @Inject(HEAD) on ShulkerBoxBlockEntity.canPlaceItemThroughFace | `ShulkerBoxBlockEntityMixin.canInsert` | HEAD | needs-mapping | No Horizon function known for ShulkerBoxBlockEntity.canPlaceItemThroughFace: run `match`. |
| @Inject(HEAD) on Enderman.isBeingStaredBy | `EndermanEntityMixin.isPlayerStaring` | HEAD | needs-mapping | No Horizon function known for Enderman.isBeingStaredBy: run `match`. |
| @Inject(INVOKE) on ExperienceOrb.playerTouch at Lnet/minecraft/world/entity/player/Player;take(Lnet/minecraft/world/entity/Entity;I)V | `ExperienceOrbEntityMixin.onPlayerCollision` | AFTER_CALL | needs-mapping | No Horizon function known for ExperienceOrb.playerTouch: run `match`. The call it hooks (Player.take) isn't mapped to a Rust function yet. |
| @Inject(HEAD) on Mob.setTarget | `GhastEntityMixin.setTarget` | HEAD | needs-mapping | No Horizon function known for Mob.setTarget: run `match`. |
| @Inject(TAIL) on LootTable.getRandomItems | `LootTableMixin.setTarget` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for LootTable.getRandomItems: run `match`. |
| @Inject(HEAD) on SmallFireball.onHitEntity | `SmallFireballEntityMixin.onEntityHit` | HEAD | needs-mapping | No Horizon function known for SmallFireball.onHitEntity: run `match`. |
| @Inject(HEAD) on Entity.isInWater | `EntityMixin.travelersbackpack$isInWater` | HEAD | needs-mapping | No Horizon function known for Entity.isInWater: run `match`. |
| @Inject(HEAD) on Entity.isInvulnerable | `EntityMixin.travelersbackpack$isInvulnerable` | HEAD | needs-mapping | No Horizon function known for Entity.isInvulnerable: run `match`. |
| @Inject(HEAD) on Entity.onBelowWorld | `EntityMixin.travelersbackpack$onBelowWorld` | HEAD | needs-mapping | No Horizon function known for Entity.onBelowWorld: run `match`. |
| @Inject(INVOKE) on ItemEntity.playerTouch at Lnet/minecraft/world/item/ItemStack;getCount()I | `ItemEntityMixin.playerTouch` | BEFORE_CALL | needs-mapping | Only whole statements, plain let initializers and block tails can be selected. No Horizon function known for ItemEntity.playerTouch: run `match`. |
| @Inject(TAIL) on ItemEntity.setItem | `ItemEntityMixin.travelersbackpack$onSetItem` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for ItemEntity.setItem: run `match`. |
| @Inject(HEAD) on ItemEntity.tick | `ItemEntityMixin.travelersbackpack$onTick` | HEAD on horizon_client::entities::ItemEntity::inside_block | auto |  |
| @Inject(HEAD) on ItemEntity.fireImmune | `ItemEntityMixin.travelersbackpack$fireImmune` | HEAD | needs-mapping | No Horizon function known for ItemEntity.fireImmune: run `match`. |
| @Inject(HEAD) on ItemEntity.ignoreExplosion | `ItemEntityMixin.travelersbackpack$ignoreExplosion` | HEAD | needs-mapping | No Horizon function known for ItemEntity.ignoreExplosion: run `match`. |

## Classes extending the game

| Class | In | Status | Note |
|---|---|---|---|
| extends SimpleCriterionTrigger | `ActionTypeTrigger` | needs-translation | No mapping for this superclass yet. |
| extends BedBlock | `SleepingBagBlock` | needs-translation | No mapping for this superclass yet. |
| extends Block | `TravelersBackpackBlock` | needs-translation | Overrides getRenderShape, getShape, useWithoutItem, onExplosionHit, playerWillDestroy, getStateForPlacement, createBlockStateDefinition, entityInside, hasAnalogOutputSignal, getAnalogOutputSignal, getCloneItemStack, newBlockEntity, getTicker, animateTick, getSignal, isSignalSource, onPlace, neighborChanged, tryAbsorbWater, removeWaterBreadthFirstSearch, registerDispenserBehaviour: the content ports, that behaviour needs native code. |
| extends BlockEntity | `BackpackBlockEntity` | unsupported | Block entities aren't supported in Horizon mods yet. |
| extends RenderLayer | `BackpackEntityLayer` | needs-translation | No mapping for this superclass yet. |
| extends RenderLayer | `BackpackLayer` | needs-translation | No mapping for this superclass yet. |
| extends AbstractContainerScreen | `AbstractBackpackScreen` | needs-translation | No mapping for this superclass yet. |
| extends Screen | `ToolsScreen` | needs-translation | No general screen hook; simple option pages can use the settings hook. |
| extends AbstractContainerEventHandler | `ScrollPanel` | needs-translation | No mapping for this superclass yet. |
| extends SimpleSmithingRecipe | `BackpackUpgradeRecipe` | needs-translation | No mapping for this superclass yet. |
| extends NormalCraftingRecipe | `ShapedBackpackRecipe` | needs-translation | No mapping for this superclass yet. |
| extends RecipeProvider | `ModRecipeProvider$1` | needs-translation | No mapping for this superclass yet. |
| extends ItemEntity | `BackpackItemEntity` | needs-translation | No mapping for this superclass yet. |
| extends FlowingFluid | `PotionFluid` | needs-translation | No mapping for this superclass yet. |
| extends SimpleContainer | `ItemStackHandler` | needs-translation | No mapping for this superclass yet. |
| extends AbstractContainerMenu | `AbstractBackpackMenu` | unsupported | Menus aren't supported. |
| extends Slot | `CraftingSlot` | needs-translation | No mapping for this superclass yet. |
| extends Slot | `DisabledSlot` | needs-translation | No mapping for this superclass yet. |
| extends ResultSlot | `ResultSlotExt` | needs-translation | No mapping for this superclass yet. |
| extends Slot | `SlotItemHandler` | needs-translation | No mapping for this superclass yet. |
| extends AbstractTickableSoundInstance | `JukeboxWidget$MovingSound` | needs-translation | No mapping for this superclass yet. |
| extends Item | `BackpackTankItem` | needs-translation | Overrides isEnabled: the content ports, that behaviour needs native code. |
| extends Item | `HoseItem` | needs-translation | Overrides getUseAnimation, getUseDuration, onUseTick, shouldEmitDrinkingSounds, consumeTicks, consumeSeconds, emitDrinkingSound, use, drink, spillPotion, emptyContents, playEmptySound, triggerAdvancement, finishUsingItem, getHoseMode, getHoseTank, getSelectedFluidTank, appendHoverText: the content ports, that behaviour needs native code. |
| extends BlockItem | `SleepingBagItem` | needs-translation | Overrides isEnabled, getDefaultColor, appendHoverText: the content ports, that behaviour needs native code. |
| extends BlockItem | `TravelersBackpackItem` | needs-translation | Overrides use, useOn, place, isCreative, overrideStackedOnOther, overrideOtherStackedOnMe, add, playInsertSound, appendHoverText, addAttributeModifierTooltip, getTooltipImage, canFitInsideContainerItems: the content ports, that behaviour needs native code. |
| extends Item | `TierUpgrade` | needs-translation | Overrides appendHoverText: the content ports, that behaviour needs native code. |
| extends Item | `UpgradeItem` | needs-translation | Overrides appendHoverText, isTickingUpgrade, requiresEquippedBackpack, hasBlockFunctionality, getUpgradeClass, getUpgrade: the content ports, that behaviour needs native code. |

## Access wideners

| File | In | Status | Note |
|---|---|---|---|
| access widener (opens private game fields) | `travelersbackpack.accesswidener` | needs-translation | Native helpers already see the game's fields, once each field is mapped. |

## Content

| What | In | Status | Note |
|---|---|---|---|
| registers blocks or items in code | `ModAdvancements` | content | Registration and properties: horizon-port-pack carries them as a content mod. |
| registers blocks or items in code | `ModBlockEntityTypes` | content | Registration and properties: horizon-port-pack carries them as a content mod. |
| registers blocks or items in code | `ModBlocks` | content | Registration and properties: horizon-port-pack carries them as a content mod. |
| registers blocks or items in code | `ModDataComponents` | content | Registration and properties: horizon-port-pack carries them as a content mod. |
| registers blocks or items in code | `ModFluids` | content | Registration and properties: horizon-port-pack carries them as a content mod. |
| registers blocks or items in code | `ModItemGroups` | content | Registration and properties: horizon-port-pack carries them as a content mod. |
| registers blocks or items in code | `ModItems` | content | Registration and properties: horizon-port-pack carries them as a content mod. |
| registers blocks or items in code | `ModRecipeSerializers` | content | Registration and properties: horizon-port-pack carries them as a content mod. |
| registers blocks or items in code | `ModScreenHandlerTypes` | content | Registration and properties: horizon-port-pack carries them as a content mod. |
| 507 asset files, 381 data files | `jar resources` | content | Recipes, loot, tags, models, textures, worldgen: horizon-port-pack carries these. |

## Game methods and fields used

72 of 1477 are mapped to Horizon. Every Java body that uses an unmapped one needs it mapped before its Rust can be written.

<details><summary>Mapped</summary>

| Java | Horizon |
|---|---|
| `com/mojang/blaze3d/vertex/PoseStack#rotateDegrees` | horizon_client_render::camera::rotate_degrees (suggested) |
| `net/minecraft/client/KeyMapping#<init>` | `self.native_mod_key(MOD_ID, name, default)` |
| `net/minecraft/client/KeyMapping#consumeClick` | `key == self.native_mod_key(..) && pressed, in the key hook` |
| `net/minecraft/client/KeyMapping#isDown` | `self.native_mapping(name) for vanilla keys; track key hook presses for the mod's own` |
| `net/minecraft/client/Minecraft#font` | `(implicit)` |
| `net/minecraft/client/Minecraft#getInstance` | `self` |
| `net/minecraft/client/Minecraft#player` | `self.native_player()` |
| `net/minecraft/client/gui/Font#width` | `self.native_text_width(text)` |
| `net/minecraft/client/gui/GuiGraphicsExtractor#blitSprite` | horizon_client_render::ui::blit_sprite (suggested) |
| `net/minecraft/client/gui/GuiGraphicsExtractor#itemDecorations` | horizon_client_render::hud_overlays::item_decorations (suggested) |
| `net/minecraft/client/renderer/item/ItemStackRenderState#submit` | horizon_client::item_render::ItemRenderState::submit (suggested) |
| `net/minecraft/client/resources/model/cuboid/ItemTransforms#getTransform` | horizon_assets::item_bake::BakedItemModel::transform (suggested) |
| `net/minecraft/core/Direction#getOpposite` | horizon_client::mobs::shulker::opposite (suggested) |
| `net/minecraft/core/Registry#register` | content |
| `net/minecraft/core/registries/BuiltInRegistries#BLOCK` | content |
| `net/minecraft/core/registries/BuiltInRegistries#ITEM` | content |
| `net/minecraft/network/chat/MutableComponent#append` | horizon_client::game::dedicated::append (suggested) |
| `net/minecraft/network/chat/MutableComponent#withStyle` | horizon_client_core::text::Text::with_style (suggested) |
| `net/minecraft/network/codec/ByteBufCodecs#list` | horizon_protocol::item::skip::list (suggested) |
| `net/minecraft/server/level/ServerLevel#isClientSide` | horizon_world::level::is_client_side (suggested) |
| `net/minecraft/server/level/ServerLevel#sendParticles` | horizon_client::game::fishing::Game<'_>::send_particles (suggested) |
| `net/minecraft/server/level/ServerPlayer#getDisplayName` | horizon_client::lan::hub::display_name (suggested) |
| `net/minecraft/util/ARGB#opaque` | horizon_client_render::level_loading::opaque (suggested) |
| `net/minecraft/util/ARGB#white` | horizon_client_render::hud_layout::white (suggested) |
| `net/minecraft/util/Mth#clamp` | horizon_client_render::chat::clamp (suggested) |
| `net/minecraft/util/Mth#floor` | horizon_client_render::gfx::occlusion::floor (suggested) |
| `net/minecraft/util/Mth#lerpDiscrete` | horizon_client_render::hud_layout::lerp_discrete (suggested) |
| `net/minecraft/world/Containers#dropItemStack` | horizon_world::level::removal::drop_item_stack (suggested) |
| `net/minecraft/world/effect/MobEffect#isInstantaneous` | horizon_world::loot::is_instantaneous (suggested) |
| `net/minecraft/world/entity/Entity#getX` | `self.native_player()?.x` |
| `net/minecraft/world/entity/Entity#getY` | `self.native_player()?.y` |
| `net/minecraft/world/entity/Entity#getZ` | `self.native_player()?.z` |
| `net/minecraft/world/entity/Entity#playSound` | horizon_client::game::consume::<Emitter<'_, '_> as EmissionWorld>::play_sound (suggested) |
| `net/minecraft/world/entity/LivingEntity#playSound` | horizon_client::game::consume::<Emitter<'_, '_> as EmissionWorld>::play_sound (suggested) |
| `net/minecraft/world/entity/Mob#finalizeSpawn` | horizon_client::mobs::creaking_heart::finalize_spawn (suggested) |
| `net/minecraft/world/entity/item/ItemEntity#tick` | horizon_client::entities::ItemEntity::inside_block (suggested) |
| `net/minecraft/world/entity/player/Inventory#add` | horizon_client::cauldron::add (suggested) |
| `net/minecraft/world/entity/player/Player#attack` | horizon_client::game::Game<'_>::attack (suggested) |
| `net/minecraft/world/entity/player/Player#clearFire` | horizon_client::arrows::ArrowState::clear_fire (suggested) |
| `net/minecraft/world/entity/player/Player#hasEffect` | horizon_client::mobs::PlayerView::has_effect (suggested) |
| `net/minecraft/world/entity/player/Player#isOnFire` | horizon_client::mobs::Mob::is_on_fire (suggested) |
| `net/minecraft/world/entity/player/Player#onGround` | horizon_client::game::minecarts::Minecart::on_ground (suggested) |
| `net/minecraft/world/entity/player/Player#spawnAtLocation` | horizon_client::game::dispenser::Game<'_>::spawn_at_location (suggested) |
| `net/minecraft/world/entity/player/Player#swing` | horizon_client::mobs::ai::Mob::swing (suggested) |
| `net/minecraft/world/entity/player/Player#tick` | horizon_client::attack_strength::AttackStrength::tick (suggested) |
| `net/minecraft/world/entity/player/StackedItemContents#accountStack` | horizon_client::menu::place::Stacked::account_stack (suggested) |
| `net/minecraft/world/inventory/Slot#setByPlayer` | horizon_client::menu::Menu::set_by_player (suggested) |
| `net/minecraft/world/item/Item#<init>` | content |
| `net/minecraft/world/item/Item$Properties#stacksTo` | content |
| `net/minecraft/world/item/ItemStack#getCount` | horizon_world::loot::LootStack::get_count (suggested) |
| `net/minecraft/world/item/ItemStack#hasFoil` | horizon_items::item_data::ItemStack::has_foil (suggested) |
| `net/minecraft/world/item/ItemStack#hurtAndBreak` | horizon_client::game::dispenser::Game<'_>::hurt_and_break (suggested) |
| `net/minecraft/world/item/ItemStack#isSameItemSameComponents` | horizon_items::item_data::ItemStack::same_item_same_components (suggested) |
| `net/minecraft/world/item/ItemStack#matches` | horizon_client_render::vault::matches (suggested) |
| `net/minecraft/world/item/ItemStack#shrink` | horizon_client::game::dispenser::shrink (suggested) |
| `net/minecraft/world/item/ItemStackTemplate#create` | horizon_client::crafting::Template::create (suggested) |
| `net/minecraft/world/level/Level#canSeeSky` | horizon_client::mobs::world::MobWorld<'_>::can_see_sky (suggested) |
| `net/minecraft/world/level/Level#destroyBlock` | horizon_world::level::update::Sim<'_, L>::destroy_block (suggested) |
| `net/minecraft/world/level/Level#getHeight` | horizon_world::level::feature_world::<LevelFeatureWorld<'_, '_, L> as PlacementContext>::height (suggested) |
| `net/minecraft/world/level/Level#getMinY` | horizon_client::game::Dim::min_y (suggested) |
| `net/minecraft/world/level/Level#isClientSide` | horizon_world::level::is_client_side (suggested) |
| `net/minecraft/world/level/Level#removeBlock` | horizon_world::level::update::Sim<'_, L>::remove_block (suggested) |
| `net/minecraft/world/level/Level#setBlock` | horizon_world::level::Behaviours::set_block (suggested) |
| `net/minecraft/world/level/Level#updateNeighborsAt` | horizon_world::level::Behaviours::update_neighbors_at (suggested) |
| `net/minecraft/world/level/Level#updateNeighbourForOutputSignal` | horizon_world::level::update::Sim<'_, L>::update_neighbour_for_output_signal (suggested) |
| `net/minecraft/world/level/block/Block#<init>` | content |
| `net/minecraft/world/level/block/Block#popResource` | horizon_client::game::spill::GameLevel<'_>::pop_resource (suggested) |
| `net/minecraft/world/level/block/Block#setPlacedBy` | horizon_world::level::place::Sim<'_, L>::set_placed_by (suggested) |
| `net/minecraft/world/level/block/BucketPickup#pickupBlock` | horizon_world::level::fluids::Sim<'_, L>::pickup_block (suggested) |
| `net/minecraft/world/level/block/LiquidBlockContainer#canPlaceLiquid` | horizon_world::level::fluids::Sim<'_, L>::can_place_liquid (suggested) |
| `net/minecraft/world/level/block/LiquidBlockContainer#placeLiquid` | horizon_world::level::fluids::Sim<'_, L>::place_liquid (suggested) |
| `net/minecraft/world/level/block/state/BlockState#canBeReplaced` | horizon_world::level::place::Sim<'_, L>::can_be_replaced (suggested) |

</details>

<details><summary>Not mapped yet</summary>

| Java | Kind | Classes using it |
|---|---|---|
| `com/mojang/blaze3d/platform/InputConstants#UNKNOWN` | field | 1 |
| `com/mojang/blaze3d/platform/InputConstants#isKeyDown` | method | 3 |
| `com/mojang/blaze3d/platform/InputConstants$Key#getType` | method | 1 |
| `com/mojang/blaze3d/platform/InputConstants$Key#getValue` | method | 1 |
| `com/mojang/blaze3d/platform/InputConstants$Type#KEYBOARD` | field | 2 |
| `com/mojang/blaze3d/platform/InputConstants$Type#MOUSE` | field | 1 |
| `com/mojang/blaze3d/platform/InputConstants$Type#ordinal` | method | 2 |
| `com/mojang/blaze3d/platform/InputConstants$Type#values` | method | 1 |
| `com/mojang/blaze3d/platform/Window#getGuiScaledHeight` | method | 3 |
| `com/mojang/blaze3d/platform/Window#getGuiScaledWidth` | method | 2 |
| `com/mojang/blaze3d/platform/Window#getScreenHeight` | method | 1 |
| `com/mojang/blaze3d/platform/Window#getScreenWidth` | method | 1 |
| `com/mojang/blaze3d/platform/Window#handle` | method | 1 |
| `com/mojang/blaze3d/vertex/PoseStack#popPose` | method | 4 |
| `com/mojang/blaze3d/vertex/PoseStack#pushPose` | method | 4 |
| `com/mojang/blaze3d/vertex/PoseStack#rotate` | method | 2 |
| `com/mojang/blaze3d/vertex/PoseStack#scale` | method | 5 |
| `com/mojang/blaze3d/vertex/PoseStack#translate` | method | 6 |
| `net/minecraft/ChatFormatting#BLUE` | field | 8 |
| `net/minecraft/ChatFormatting#DARK_GRAY` | field | 1 |
| `net/minecraft/ChatFormatting#DARK_PURPLE` | field | 1 |
| `net/minecraft/ChatFormatting#GRAY` | field | 1 |
| `net/minecraft/ChatFormatting#GREEN` | field | 1 |
| `net/minecraft/ChatFormatting#RED` | field | 3 |
| `net/minecraft/ChatFormatting#YELLOW` | field | 2 |
| `net/minecraft/advancements/predicates/StatePropertiesPredicate$Builder#hasProperty` | method | 1 |
| `net/minecraft/advancements/predicates/StatePropertiesPredicate$Builder#properties` | method | 1 |
| `net/minecraft/advancements/triggers/CriteriaTriggers#FILLED_BUCKET` | field | 1 |
| `net/minecraft/advancements/triggers/CriteriaTriggers#PLACED_BLOCK` | field | 2 |
| `net/minecraft/advancements/triggers/FilledBucketTrigger#trigger` | method | 1 |
| `net/minecraft/advancements/triggers/ItemUsedOnLocationTrigger#trigger` | method | 2 |
| `net/minecraft/advancements/triggers/SimpleCriterionTrigger#<init>` | method | 1 |
| `net/minecraft/client/KeyMapping#getTranslatedKeyMessage` | method | 1 |
| `net/minecraft/client/KeyMapping#isUnbound` | method | 1 |
| `net/minecraft/client/KeyMapping#key` | field | 1 |
| `net/minecraft/client/KeyMapping#matches` | method | 3 |
| `net/minecraft/client/KeyMapping#matchesMouse` | method | 1 |
| `net/minecraft/client/KeyMapping$Category#<init>` | method | 1 |
| `net/minecraft/client/Minecraft#execute` | method | 6 |
| `net/minecraft/client/Minecraft#gameMode` | field | 2 |
| `net/minecraft/client/Minecraft#getAtlasManager` | method | 1 |
| `net/minecraft/client/Minecraft#getItemModelResolver` | method | 2 |
| `net/minecraft/client/Minecraft#getModelManager` | method | 3 |
| `net/minecraft/client/Minecraft#getSoundManager` | method | 1 |
| `net/minecraft/client/Minecraft#getWindow` | method | 3 |
| `net/minecraft/client/Minecraft#gui` | field | 10 |
| `net/minecraft/client/Minecraft#level` | field | 6 |
| `net/minecraft/client/Minecraft#options` | field | 2 |
| `net/minecraft/client/Options#advancedItemTooltips` | field | 1 |
| `net/minecraft/client/Options#keyDown` | field | 1 |
| `net/minecraft/client/Options#keyJump` | field | 1 |
| `net/minecraft/client/Options#keyLeft` | field | 1 |
| `net/minecraft/client/Options#keyRight` | field | 1 |
| `net/minecraft/client/Options#keyShift` | field | 1 |
| `net/minecraft/client/Options#keySprint` | field | 1 |
| `net/minecraft/client/Options#keyUp` | field | 1 |
| `net/minecraft/client/ScrollWheelHandler#getNextScrollWheelSelection` | method | 1 |
| `net/minecraft/client/color/item/ItemTintSource#calculate` | method | 1 |
| `net/minecraft/client/color/item/ItemTintSources#CODEC` | field | 1 |
| `net/minecraft/client/color/item/ItemTintSources#ID_MAPPER` | field | 1 |
| `net/minecraft/client/gui/Gui#hud` | field | 5 |
| `net/minecraft/client/gui/Gui#screen` | method | 7 |
| `net/minecraft/client/gui/Gui#setScreen` | method | 1 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#blit` | method | 23 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#disableScissor` | method | 3 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#enableScissor` | method | 3 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#fakeItem` | method | 3 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#fill` | method | 7 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#fillGradient` | method | 2 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#innerBlit` | method | 1 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#item` | method | 6 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#pose` | method | 4 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#setComponentTooltipForNextFrame` | method | 4 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#setTooltipForNextFrame` | method | 25 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#text` | method | 5 |
| `net/minecraft/client/gui/Hud#extractCrosshair` | method | 1 |
| `net/minecraft/client/gui/Hud#getChat` | method | 2 |
| `net/minecraft/client/gui/Hud#isHidden` | method | 2 |
| `net/minecraft/client/gui/Hud#setNowPlaying` | method | 1 |
| `net/minecraft/client/gui/components/ChatComponent#addClientSystemMessage` | method | 2 |
| `net/minecraft/client/gui/components/events/AbstractContainerEventHandler#<init>` | method | 1 |
| `net/minecraft/client/gui/components/events/AbstractContainerEventHandler#mouseClicked` | method | 1 |
| `net/minecraft/client/gui/components/events/AbstractContainerEventHandler#mouseReleased` | method | 1 |
| `net/minecraft/client/gui/components/events/GuiEventListener#isMouseOver` | method | 1 |
| `net/minecraft/client/gui/components/events/GuiEventListener#mouseDragged` | method | 1 |
| `net/minecraft/client/gui/narration/NarratableEntry$NarrationPriority#NONE` | field | 2 |
| `net/minecraft/client/gui/screens/MenuScreens#register` | method | 1 |
| `net/minecraft/client/gui/screens/Screen#<init>` | method | 1 |
| `net/minecraft/client/gui/screens/Screen#MENU_BACKGROUND` | field | 1 |
| `net/minecraft/client/gui/screens/Screen#extractMenuBackgroundTexture` | method | 1 |
| `net/minecraft/client/gui/screens/Screen#init` | method | 1 |
| `net/minecraft/client/gui/screens/Screen#mouseClicked` | method | 1 |
| `net/minecraft/client/gui/screens/Screen#mouseScrolled` | method | 1 |
| `net/minecraft/client/gui/screens/inventory/AbstractContainerScreen#<init>` | method | 1 |
| `net/minecraft/client/gui/screens/inventory/AbstractContainerScreen#extractBackground` | method | 1 |
| `net/minecraft/client/gui/screens/inventory/AbstractContainerScreen#extractContents` | method | 1 |
| `net/minecraft/client/gui/screens/inventory/AbstractContainerScreen#getMenu` | method | 1 |
| `net/minecraft/client/gui/screens/inventory/AbstractContainerScreen#hoveredSlot` | field | 2 |
| `net/minecraft/client/gui/screens/inventory/AbstractContainerScreen#leftPos` | field | 1 |
| `net/minecraft/client/gui/screens/inventory/AbstractContainerScreen#mouseClicked` | method | 1 |
| `net/minecraft/client/gui/screens/inventory/AbstractContainerScreen#mouseDragged` | method | 1 |
| `net/minecraft/client/gui/screens/inventory/AbstractContainerScreen#mouseScrolled` | method | 1 |
| `net/minecraft/client/gui/screens/inventory/AbstractContainerScreen#topPos` | field | 1 |
| `net/minecraft/client/gui/screens/inventory/InventoryScreen#leftPos` | field | 1 |
| `net/minecraft/client/gui/screens/inventory/InventoryScreen#topPos` | field | 1 |
| `net/minecraft/client/input/MouseButtonEvent#button` | method | 5 |
| `net/minecraft/client/input/MouseButtonEvent#x` | method | 18 |
| `net/minecraft/client/input/MouseButtonEvent#y` | method | 18 |
| `net/minecraft/client/model/HumanoidModel#body` | field | 1 |
| `net/minecraft/client/model/HumanoidModel#root` | method | 1 |
| `net/minecraft/client/model/geom/ModelPart#translateAndRotate` | method | 1 |
| `net/minecraft/client/model/geom/ModelPart#x` | field | 2 |
| `net/minecraft/client/model/geom/ModelPart#xRot` | field | 2 |
| `net/minecraft/client/model/geom/ModelPart#xScale` | field | 2 |
| `net/minecraft/client/model/geom/ModelPart#y` | field | 2 |
| `net/minecraft/client/model/geom/ModelPart#yRot` | field | 2 |
| `net/minecraft/client/model/geom/ModelPart#yScale` | field | 2 |
| `net/minecraft/client/model/geom/ModelPart#z` | field | 2 |
| `net/minecraft/client/model/geom/ModelPart#zRot` | field | 2 |
| `net/minecraft/client/model/geom/ModelPart#zScale` | field | 2 |
| `net/minecraft/client/model/geom/builders/UVPair#pack` | method | 1 |
| `net/minecraft/client/model/geom/builders/UVPair#unpackU` | method | 1 |
| `net/minecraft/client/model/geom/builders/UVPair#unpackV` | method | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#getEntity` | method | 2 |
| `net/minecraft/client/multiplayer/ClientLevel#registryAccess` | method | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#tickRateManager` | method | 1 |
| `net/minecraft/client/multiplayer/MultiPlayerGameMode#getPlayerMode` | method | 2 |
| `net/minecraft/client/player/ClientInput#keyPresses` | field | 1 |
| `net/minecraft/client/player/ClientInput#moveVector` | field | 1 |
| `net/minecraft/client/player/ClientInput#tick` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#aiStep` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#getInventory` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#getItemInHand` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#isMovingSlowly` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#level` | method | 2 |
| `net/minecraft/client/renderer/Rect2i#<init>` | method | 4 |
| `net/minecraft/client/renderer/Rect2i#getHeight` | method | 1 |
| `net/minecraft/client/renderer/Rect2i#getWidth` | method | 1 |
| `net/minecraft/client/renderer/Rect2i#getX` | method | 1 |
| `net/minecraft/client/renderer/Rect2i#getY` | method | 1 |
| `net/minecraft/client/renderer/RenderPipelines#GUI` | field | 2 |
| `net/minecraft/client/renderer/RenderPipelines#GUI_TEXTURED` | field | 24 |
| `net/minecraft/client/renderer/Sheets#cutoutBlockItemGlintSheet` | method | 1 |
| `net/minecraft/client/renderer/Sheets#cutoutBlockItemGlintSpecialSheet` | method | 1 |
| `net/minecraft/client/renderer/Sheets#cutoutBlockItemSheet` | method | 1 |
| `net/minecraft/client/renderer/SubmitNodeCollector#submitItem` | method | 1 |
| `net/minecraft/client/renderer/block/BlockAndTintGetter#EMPTY` | field | 1 |
| `net/minecraft/client/renderer/block/BlockAndTintGetter#getBlockEntity` | method | 3 |
| `net/minecraft/client/renderer/block/BlockAndTintGetter#getBlockEntityRenderData` | method | 1 |
| `net/minecraft/client/renderer/block/BlockModelRenderState#<init>` | method | 1 |
| `net/minecraft/client/renderer/block/BlockModelRenderState#scratchRandomSource` | method | 1 |
| `net/minecraft/client/renderer/block/BlockModelRenderState#setupMesh` | method | 1 |
| `net/minecraft/client/renderer/block/BlockModelRenderState#submit` | method | 1 |
| `net/minecraft/client/renderer/block/FluidModel#stillMaterial` | method | 2 |
| `net/minecraft/client/renderer/block/FluidModel$Unbaked#<init>` | method | 1 |
| `net/minecraft/client/renderer/block/FluidStateModelSet#get` | method | 2 |
| `net/minecraft/client/renderer/block/dispatch/BlockModelRotation#IDENTITY` | field | 1 |
| `net/minecraft/client/renderer/block/dispatch/BlockStateModel#emitQuads` | method | 1 |
| `net/minecraft/client/renderer/block/dispatch/BlockStateModel#hasMaterialFlag` | method | 1 |
| `net/minecraft/client/renderer/block/dispatch/BlockStateModelPart#getQuads` | method | 1 |
| `net/minecraft/client/renderer/block/dispatch/ModelState#transformation` | method | 1 |
| `net/minecraft/client/renderer/block/dispatch/Variant#MAP_CODEC` | field | 1 |
| `net/minecraft/client/renderer/block/dispatch/Variant#modelLocation` | method | 1 |
| `net/minecraft/client/renderer/block/dispatch/Variant#modelState` | method | 1 |
| `net/minecraft/client/renderer/block/dispatch/Variant$SimpleModelState#asModelState` | method | 1 |
| `net/minecraft/client/renderer/chunk/ChunkSectionLayer#CUTOUT` | field | 1 |
| `net/minecraft/client/renderer/entity/HumanoidMobRenderer#extractHumanoidRenderState` | method | 1 |
| `net/minecraft/client/renderer/entity/ItemEntityRenderer#<init>` | method | 1 |
| `net/minecraft/client/renderer/entity/LivingEntityRenderer#getModel` | method | 1 |
| `net/minecraft/client/renderer/entity/layers/RenderLayer#<init>` | method | 2 |
| `net/minecraft/client/renderer/entity/state/HumanoidRenderState#chestEquipment` | field | 1 |
| `net/minecraft/client/renderer/entity/state/HumanoidRenderState#isBaby` | field | 1 |
| `net/minecraft/client/renderer/entity/state/HumanoidRenderState#scale` | field | 1 |
| `net/minecraft/client/renderer/item/CuboidItemModelWrapper#computeExtents` | method | 1 |
| `net/minecraft/client/renderer/item/ItemModel$BakingContext#blockModelBaker` | method | 1 |
| `net/minecraft/client/renderer/item/ItemModelResolver#updateForTopItem` | method | 2 |
| `net/minecraft/client/renderer/item/ItemModels#ID_MAPPER` | field | 1 |
| `net/minecraft/client/renderer/item/ItemStackRenderState#<init>` | method | 4 |
| `net/minecraft/client/renderer/item/ItemStackRenderState#appendModelIdentityElement` | method | 1 |
| `net/minecraft/client/renderer/item/ItemStackRenderState#newLayer` | method | 1 |
| `net/minecraft/client/renderer/item/ItemStackRenderState$FoilType#NONE` | field | 1 |
| `net/minecraft/client/renderer/item/ItemStackRenderState$FoilType#STANDARD` | field | 2 |
| `net/minecraft/client/renderer/item/ItemStackRenderState$LayerRenderState#EMPTY_TINTS` | field | 1 |
| `net/minecraft/client/renderer/item/ItemStackRenderState$LayerRenderState#setExtents` | method | 1 |
| `net/minecraft/client/renderer/item/ItemStackRenderState$LayerRenderState#setFoilType` | method | 1 |
| `net/minecraft/client/renderer/item/ItemStackRenderState$LayerRenderState#setItemTransform` | method | 1 |
| `net/minecraft/client/renderer/item/ItemStackRenderState$LayerRenderState#setParticleMaterial` | method | 1 |
| `net/minecraft/client/renderer/item/ItemStackRenderState$LayerRenderState#setQuads` | method | 1 |
| `net/minecraft/client/renderer/item/ItemStackRenderState$LayerRenderState#setUsesBlockLight` | method | 1 |
| `net/minecraft/client/renderer/item/ItemStackRenderState$LayerRenderState#setupSpecialModel` | method | 1 |
| `net/minecraft/client/renderer/item/ItemStackRenderState$LayerRenderState#tintLayers` | method | 1 |
| `net/minecraft/client/renderer/item/properties/select/SelectItemModelProperties#ID_MAPPER` | field | 1 |
| `net/minecraft/client/renderer/item/properties/select/SelectItemModelProperty$Type#create` | method | 1 |
| `net/minecraft/client/renderer/texture/MissingTextureAtlasSprite#getLocation` | method | 1 |
| `net/minecraft/client/renderer/texture/OverlayTexture#NO_OVERLAY` | field | 2 |
| `net/minecraft/client/renderer/texture/TextureAtlas#getSprite` | method | 1 |
| `net/minecraft/client/renderer/texture/TextureAtlasSprite#atlasLocation` | method | 1 |
| `net/minecraft/client/renderer/texture/TextureAtlasSprite#getU` | method | 1 |
| `net/minecraft/client/renderer/texture/TextureAtlasSprite#getU0` | method | 2 |
| `net/minecraft/client/renderer/texture/TextureAtlasSprite#getU1` | method | 2 |
| `net/minecraft/client/renderer/texture/TextureAtlasSprite#getV` | method | 1 |
| `net/minecraft/client/renderer/texture/TextureAtlasSprite#getV0` | method | 2 |
| `net/minecraft/client/renderer/texture/TextureAtlasSprite#getV1` | method | 2 |
| `net/minecraft/client/resources/language/I18n#get` | method | 2 |
| `net/minecraft/client/resources/model/ModelBaker#getModel` | method | 3 |
| `net/minecraft/client/resources/model/ModelManager#getFluidStateModelSet` | method | 2 |
| `net/minecraft/client/resources/model/ModelManager#getModel` | method | 1 |
| `net/minecraft/client/resources/model/ResolvableModel$Resolver#markDependency` | method | 3 |
| `net/minecraft/client/resources/model/ResolvedModel#getClass` | method | 1 |
| `net/minecraft/client/resources/model/ResolvedModel#getTopGeometry` | method | 1 |
| `net/minecraft/client/resources/model/ResolvedModel#parent` | method | 1 |
| `net/minecraft/client/resources/model/ResolvedModel#resolveParticleMaterial` | method | 1 |
| `net/minecraft/client/resources/model/ResolvedModel#wrapped` | method | 3 |
| `net/minecraft/client/resources/model/SimpleModelWrapper#<init>` | method | 1 |
| `net/minecraft/client/resources/model/UnbakedModel#getClass` | method | 1 |
| `net/minecraft/client/resources/model/UnbakedModel#parent` | method | 1 |
| `net/minecraft/client/resources/model/UnbakedModel#textureSlots` | method | 1 |
| `net/minecraft/client/resources/model/cuboid/ItemTransform#<init>` | method | 1 |
| `net/minecraft/client/resources/model/cuboid/ItemTransforms#<init>` | method | 1 |
| `net/minecraft/client/resources/model/cuboid/ItemTransforms#NO_TRANSFORMS` | field | 1 |
| `net/minecraft/client/resources/model/geometry/BakedQuad#<init>` | method | 1 |
| `net/minecraft/client/resources/model/geometry/BakedQuad#direction` | method | 1 |
| `net/minecraft/client/resources/model/geometry/BakedQuad#materialInfo` | method | 1 |
| `net/minecraft/client/resources/model/geometry/BakedQuad#packedUV0` | method | 1 |
| `net/minecraft/client/resources/model/geometry/BakedQuad#packedUV1` | method | 1 |
| `net/minecraft/client/resources/model/geometry/BakedQuad#packedUV2` | method | 1 |
| `net/minecraft/client/resources/model/geometry/BakedQuad#packedUV3` | method | 1 |
| `net/minecraft/client/resources/model/geometry/BakedQuad#position0` | method | 1 |
| `net/minecraft/client/resources/model/geometry/BakedQuad#position1` | method | 1 |
| `net/minecraft/client/resources/model/geometry/BakedQuad#position2` | method | 1 |
| `net/minecraft/client/resources/model/geometry/BakedQuad#position3` | method | 1 |
| `net/minecraft/client/resources/model/geometry/BakedQuad$MaterialInfo#<init>` | method | 1 |
| `net/minecraft/client/resources/model/geometry/BakedQuad$MaterialInfo#itemGlintRenderType` | method | 1 |
| `net/minecraft/client/resources/model/geometry/BakedQuad$MaterialInfo#itemGlintSpecialRenderType` | method | 1 |
| `net/minecraft/client/resources/model/geometry/BakedQuad$MaterialInfo#itemRenderType` | method | 1 |
| `net/minecraft/client/resources/model/geometry/BakedQuad$MaterialInfo#layer` | method | 1 |
| `net/minecraft/client/resources/model/geometry/BakedQuad$MaterialInfo#lightEmission` | method | 1 |
| `net/minecraft/client/resources/model/geometry/BakedQuad$MaterialInfo#shadeDirectionOverride` | method | 1 |
| `net/minecraft/client/resources/model/geometry/BakedQuad$MaterialInfo#sprite` | method | 1 |
| `net/minecraft/client/resources/model/geometry/BakedQuad$MaterialInfo#tintIndex` | method | 1 |
| `net/minecraft/client/resources/model/geometry/ItemQuads#split` | method | 1 |
| `net/minecraft/client/resources/model/geometry/QuadCollection#getAll` | method | 1 |
| `net/minecraft/client/resources/model/geometry/QuadCollection#getQuads` | method | 1 |
| `net/minecraft/client/resources/model/geometry/QuadCollection#materialFlags` | method | 1 |
| `net/minecraft/client/resources/model/geometry/QuadCollection$Builder#<init>` | method | 1 |
| `net/minecraft/client/resources/model/geometry/QuadCollection$Builder#addCulledFace` | method | 1 |
| `net/minecraft/client/resources/model/geometry/QuadCollection$Builder#addUnculledFace` | method | 1 |
| `net/minecraft/client/resources/model/geometry/QuadCollection$Builder#build` | method | 1 |
| `net/minecraft/client/resources/model/geometry/UnbakedGeometry#bake` | method | 1 |
| `net/minecraft/client/resources/model/sprite/AtlasManager#getAtlasOrThrow` | method | 1 |
| `net/minecraft/client/resources/model/sprite/Material#<init>` | method | 2 |
| `net/minecraft/client/resources/model/sprite/Material$Baked#sprite` | method | 2 |
| `net/minecraft/client/resources/model/sprite/TextureSlots$Data$Builder#<init>` | method | 1 |
| `net/minecraft/client/resources/model/sprite/TextureSlots$Data$Builder#addTexture` | method | 1 |
| `net/minecraft/client/resources/model/sprite/TextureSlots$Data$Builder#build` | method | 1 |
| `net/minecraft/client/resources/model/sprite/TextureSlots$Resolver#<init>` | method | 1 |
| `net/minecraft/client/resources/model/sprite/TextureSlots$Resolver#addLast` | method | 1 |
| `net/minecraft/client/resources/model/sprite/TextureSlots$Resolver#resolve` | method | 1 |
| `net/minecraft/client/resources/sounds/AbstractTickableSoundInstance#<init>` | method | 1 |
| `net/minecraft/client/sounds/SoundManager#queueTickingSound` | method | 1 |
| `net/minecraft/client/sounds/SoundManager#stop` | method | 1 |
| `net/minecraft/commands/CommandSourceStack#getLevel` | method | 3 |
| `net/minecraft/commands/CommandSourceStack#getPlayerOrException` | method | 2 |
| `net/minecraft/commands/CommandSourceStack#permissions` | method | 4 |
| `net/minecraft/commands/CommandSourceStack#sendFailure` | method | 4 |
| `net/minecraft/commands/CommandSourceStack#sendSuccess` | method | 4 |
| `net/minecraft/commands/Commands#argument` | method | 4 |
| `net/minecraft/commands/Commands#literal` | method | 4 |
| `net/minecraft/commands/SharedSuggestionProvider#suggest` | method | 1 |
| `net/minecraft/commands/arguments/EntityArgument#getPlayer` | method | 4 |
| `net/minecraft/commands/arguments/EntityArgument#player` | method | 2 |
| `net/minecraft/commands/arguments/EntityArgument#players` | method | 2 |
| `net/minecraft/commands/arguments/coordinates/BlockPosArgument#blockPos` | method | 2 |
| `net/minecraft/commands/arguments/coordinates/BlockPosArgument#getLoadedBlockPos` | method | 2 |
| `net/minecraft/core/BlockPos#<init>` | method | 2 |
| `net/minecraft/core/BlockPos#CODEC` | field | 1 |
| `net/minecraft/core/BlockPos#STREAM_CODEC` | field | 3 |
| `net/minecraft/core/BlockPos#ZERO` | field | 2 |
| `net/minecraft/core/BlockPos#above` | method | 2 |
| `net/minecraft/core/BlockPos#below` | method | 3 |
| `net/minecraft/core/BlockPos#breadthFirstTraversal` | method | 1 |
| `net/minecraft/core/BlockPos#containing` | method | 1 |
| `net/minecraft/core/BlockPos#equals` | method | 1 |
| `net/minecraft/core/BlockPos#getX` | method | 9 |
| `net/minecraft/core/BlockPos#getY` | method | 8 |
| `net/minecraft/core/BlockPos#getZ` | method | 9 |
| `net/minecraft/core/BlockPos#relative` | method | 8 |
| `net/minecraft/core/BlockPos#toShortString` | method | 2 |
| `net/minecraft/core/BlockPos$TraversalNodeStatus#ACCEPT` | field | 1 |
| `net/minecraft/core/BlockPos$TraversalNodeStatus#SKIP` | field | 1 |
| `net/minecraft/core/DefaultedRegistry#containsKey` | method | 1 |
| `net/minecraft/core/DefaultedRegistry#get` | method | 1 |
| `net/minecraft/core/DefaultedRegistry#getKey` | method | 3 |
| `net/minecraft/core/DefaultedRegistry#getValue` | method | 2 |
| `net/minecraft/core/DefaultedRegistry#stream` | method | 3 |
| `net/minecraft/core/Direction#DOWN` | field | 1 |
| `net/minecraft/core/Direction#EAST` | field | 5 |
| `net/minecraft/core/Direction#NORTH` | field | 5 |
| `net/minecraft/core/Direction#SOUTH` | field | 5 |
| `net/minecraft/core/Direction#UP` | field | 1 |
| `net/minecraft/core/Direction#WEST` | field | 5 |
| `net/minecraft/core/Direction#getAxis` | method | 1 |
| `net/minecraft/core/Direction#getStepX` | method | 1 |
| `net/minecraft/core/Direction#getStepZ` | method | 1 |
| `net/minecraft/core/Direction#ordinal` | method | 6 |
| `net/minecraft/core/Direction#values` | method | 5 |
| `net/minecraft/core/Direction$Axis#X` | field | 1 |
| `net/minecraft/core/Direction$Axis#Z` | field | 1 |
| `net/minecraft/core/Holder#getRegisteredName` | method | 1 |
| `net/minecraft/core/Holder#is` | method | 1 |
| `net/minecraft/core/Holder#value` | method | 11 |
| `net/minecraft/core/Holder$Reference#value` | method | 5 |
| `net/minecraft/core/HolderGetter#getOrThrow` | method | 1 |
| `net/minecraft/core/HolderLookup$Provider#lookupOrThrow` | method | 1 |
| `net/minecraft/core/HolderLookup$RegistryLookup#getOrThrow` | method | 1 |
| `net/minecraft/core/NonNullList#add` | method | 4 |
| `net/minecraft/core/NonNullList#addAll` | method | 2 |
| `net/minecraft/core/NonNullList#clear` | method | 4 |
| `net/minecraft/core/NonNullList#create` | method | 4 |
| `net/minecraft/core/NonNullList#createWithCapacity` | method | 1 |
| `net/minecraft/core/NonNullList#get` | method | 17 |
| `net/minecraft/core/NonNullList#isEmpty` | method | 1 |
| `net/minecraft/core/NonNullList#iterator` | method | 2 |
| `net/minecraft/core/NonNullList#set` | method | 4 |
| `net/minecraft/core/NonNullList#size` | method | 16 |
| `net/minecraft/core/NonNullList#stream` | method | 5 |
| `net/minecraft/core/NonNullList#subList` | method | 1 |
| `net/minecraft/core/NonNullList#withSize` | method | 3 |
| `net/minecraft/core/Registry#get` | method | 1 |
| `net/minecraft/core/RegistryAccess#createSerializationContext` | method | 1 |
| `net/minecraft/core/component/DataComponentMap#CODEC` | field | 2 |
| `net/minecraft/core/component/DataComponentMap#builder` | method | 2 |
| `net/minecraft/core/component/DataComponentMap#get` | method | 4 |
| `net/minecraft/core/component/DataComponentMap#getOrDefault` | method | 3 |
| `net/minecraft/core/component/DataComponentMap#has` | method | 3 |
| `net/minecraft/core/component/DataComponentMap#keySet` | method | 1 |
| `net/minecraft/core/component/DataComponentMap$Builder#addAll` | method | 1 |
| `net/minecraft/core/component/DataComponentMap$Builder#build` | method | 2 |
| `net/minecraft/core/component/DataComponentMap$Builder#set` | method | 2 |
| `net/minecraft/core/component/DataComponentPatch#builder` | method | 1 |
| `net/minecraft/core/component/DataComponentPatch#isEmpty` | method | 1 |
| `net/minecraft/core/component/DataComponentPatch$Builder#build` | method | 1 |
| `net/minecraft/core/component/DataComponentPatch$Builder#set` | method | 1 |
| `net/minecraft/core/component/DataComponentType#builder` | method | 1 |
| `net/minecraft/core/component/DataComponentType$Builder#build` | method | 1 |
| `net/minecraft/core/component/DataComponentType$Builder#networkSynchronized` | method | 1 |
| `net/minecraft/core/component/DataComponentType$Builder#persistent` | method | 1 |
| `net/minecraft/core/component/DataComponents#CONSUMABLE` | field | 1 |
| `net/minecraft/core/component/DataComponents#COOKING_FUEL` | field | 2 |
| `net/minecraft/core/component/DataComponents#CUSTOM_DATA` | field | 3 |
| `net/minecraft/core/component/DataComponents#CUSTOM_NAME` | field | 2 |
| `net/minecraft/core/component/DataComponents#DAMAGE` | field | 2 |
| `net/minecraft/core/component/DataComponents#DYED_COLOR` | field | 8 |
| `net/minecraft/core/component/DataComponents#ENCHANTMENTS` | field | 1 |
| `net/minecraft/core/component/DataComponents#FOOD` | field | 3 |
| `net/minecraft/core/component/DataComponents#ITEM_MODEL` | field | 1 |
| `net/minecraft/core/component/DataComponents#ITEM_NAME` | field | 1 |
| `net/minecraft/core/component/DataComponents#JUKEBOX_PLAYABLE` | field | 2 |
| `net/minecraft/core/component/DataComponents#POTION_CONTENTS` | field | 5 |
| `net/minecraft/core/component/DataComponents#STORED_ENCHANTMENTS` | field | 1 |
| `net/minecraft/core/component/DataComponents#TOOL` | field | 1 |
| `net/minecraft/core/component/DataComponents#WEAPON` | field | 1 |
| `net/minecraft/core/dispenser/ShulkerBoxDispenseBehavior#<init>` | method | 1 |
| `net/minecraft/core/particles/BlockParticleOption#<init>` | method | 1 |
| `net/minecraft/core/particles/ParticleTypes#BLOCK` | field | 1 |
| `net/minecraft/core/particles/ParticleTypes#DRAGON_BREATH` | field | 2 |
| `net/minecraft/core/particles/ParticleTypes#DRIPPING_WATER` | field | 1 |
| `net/minecraft/core/particles/ParticleTypes#ENCHANT` | field | 1 |
| `net/minecraft/core/particles/ParticleTypes#FLAME` | field | 2 |
| `net/minecraft/core/particles/ParticleTypes#GLOW` | field | 1 |
| `net/minecraft/core/particles/ParticleTypes#HEART` | field | 2 |
| `net/minecraft/core/particles/ParticleTypes#LARGE_SMOKE` | field | 2 |
| `net/minecraft/core/particles/ParticleTypes#SMOKE` | field | 1 |
| `net/minecraft/core/particles/ParticleTypes#SPLASH` | field | 1 |
| `net/minecraft/core/particles/PowerParticleOption#create` | method | 2 |
| `net/minecraft/core/registries/BuiltInRegistries#BLOCK_ENTITY_TYPE` | field | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#CREATIVE_MODE_TAB` | field | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#DATA_COMPONENT_TYPE` | field | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#ENTITY_TYPE` | field | 2 |
| `net/minecraft/core/registries/BuiltInRegistries#FLUID` | field | 2 |
| `net/minecraft/core/registries/BuiltInRegistries#MENU` | field | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#MOB_EFFECT` | field | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#RECIPE_SERIALIZER` | field | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#TRIGGER_TYPES` | field | 1 |
| `net/minecraft/core/registries/Registries#BLOCK` | field | 1 |
| `net/minecraft/core/registries/Registries#CREATIVE_MODE_TAB` | field | 1 |
| `net/minecraft/core/registries/Registries#ENTITY_TYPE` | field | 1 |
| `net/minecraft/core/registries/Registries#ITEM` | field | 4 |
| `net/minecraft/core/registries/Registries#RECIPE` | field | 2 |
| `net/minecraft/data/AtlasIds#BLOCKS` | field | 1 |
| `net/minecraft/data/recipes/CustomCraftingRecipeBuilder#customCrafting` | method | 1 |
| `net/minecraft/data/recipes/CustomCraftingRecipeBuilder#group` | method | 1 |
| `net/minecraft/data/recipes/CustomCraftingRecipeBuilder#save` | method | 1 |
| `net/minecraft/data/recipes/CustomCraftingRecipeBuilder#unlockedBy` | method | 1 |
| `net/minecraft/data/recipes/RecipeBuilder#createCraftingBookInfo` | method | 1 |
| `net/minecraft/data/recipes/RecipeBuilder#createCraftingCommonInfo` | method | 1 |
| `net/minecraft/data/recipes/RecipeBuilder#getDefaultRecipeId` | method | 1 |
| `net/minecraft/data/recipes/RecipeCategory#BUILDING_BLOCKS` | field | 1 |
| `net/minecraft/data/recipes/RecipeCategory#DECORATIONS` | field | 1 |
| `net/minecraft/data/recipes/RecipeCategory#MISC` | field | 1 |
| `net/minecraft/data/recipes/RecipeCategory#TOOLS` | field | 1 |
| `net/minecraft/data/recipes/RecipeOutput#accept` | method | 2 |
| `net/minecraft/data/recipes/RecipeProvider#<init>` | method | 1 |
| `net/minecraft/data/recipes/RecipeUnlockAdvancementBuilder#<init>` | method | 2 |
| `net/minecraft/data/recipes/RecipeUnlockAdvancementBuilder#build` | method | 2 |
| `net/minecraft/data/recipes/RecipeUnlockAdvancementBuilder#unlockedBy` | method | 2 |
| `net/minecraft/data/recipes/ShapedRecipeBuilder#define` | method | 3 |
| `net/minecraft/data/recipes/ShapedRecipeBuilder#group` | method | 1 |
| `net/minecraft/data/recipes/ShapedRecipeBuilder#pattern` | method | 1 |
| `net/minecraft/data/recipes/ShapedRecipeBuilder#save` | method | 1 |
| `net/minecraft/data/recipes/ShapedRecipeBuilder#shaped` | method | 2 |
| `net/minecraft/data/recipes/ShapedRecipeBuilder#unlockedBy` | method | 1 |
| `net/minecraft/data/recipes/ShapelessRecipeBuilder#group` | method | 1 |
| `net/minecraft/data/recipes/ShapelessRecipeBuilder#requires` | method | 2 |
| `net/minecraft/data/recipes/ShapelessRecipeBuilder#save` | method | 1 |
| `net/minecraft/data/recipes/ShapelessRecipeBuilder#shapeless` | method | 1 |
| `net/minecraft/data/recipes/ShapelessRecipeBuilder#unlockedBy` | method | 1 |
| `net/minecraft/data/recipes/SmithingTransformRecipeBuilder#save` | method | 1 |
| `net/minecraft/data/recipes/SmithingTransformRecipeBuilder#smithing` | method | 1 |
| `net/minecraft/data/recipes/SmithingTransformRecipeBuilder#unlocks` | method | 1 |
| `net/minecraft/nbt/CompoundTag#<init>` | method | 6 |
| `net/minecraft/nbt/CompoundTag#CODEC` | field | 1 |
| `net/minecraft/nbt/CompoundTag#contains` | method | 3 |
| `net/minecraft/nbt/CompoundTag#copy` | method | 2 |
| `net/minecraft/nbt/CompoundTag#equals` | method | 2 |
| `net/minecraft/nbt/CompoundTag#get` | method | 1 |
| `net/minecraft/nbt/CompoundTag#getBooleanOr` | method | 1 |
| `net/minecraft/nbt/CompoundTag#getCompoundOrEmpty` | method | 3 |
| `net/minecraft/nbt/CompoundTag#getIntOr` | method | 3 |
| `net/minecraft/nbt/CompoundTag#getLongOr` | method | 1 |
| `net/minecraft/nbt/CompoundTag#hashCode` | method | 1 |
| `net/minecraft/nbt/CompoundTag#isEmpty` | method | 1 |
| `net/minecraft/nbt/CompoundTag#put` | method | 6 |
| `net/minecraft/nbt/CompoundTag#putBoolean` | method | 1 |
| `net/minecraft/nbt/CompoundTag#putByte` | method | 1 |
| `net/minecraft/nbt/CompoundTag#putDouble` | method | 1 |
| `net/minecraft/nbt/CompoundTag#putInt` | method | 4 |
| `net/minecraft/nbt/CompoundTag#putLong` | method | 2 |
| `net/minecraft/nbt/CompoundTag#remove` | method | 2 |
| `net/minecraft/nbt/NbtIo#read` | method | 1 |
| `net/minecraft/nbt/NbtIo#write` | method | 1 |
| `net/minecraft/nbt/NbtOps#INSTANCE` | field | 3 |
| `net/minecraft/network/chat/Component#getContents` | method | 1 |
| `net/minecraft/network/chat/Component#getString` | method | 11 |
| `net/minecraft/network/chat/Component#literal` | method | 14 |
| `net/minecraft/network/chat/Component#translatable` | method | 52 |
| `net/minecraft/network/chat/MutableComponent#create` | method | 1 |
| `net/minecraft/network/chat/MutableComponent#getString` | method | 1 |
| `net/minecraft/network/codec/ByteBufCodecs#BOOL` | field | 7 |
| `net/minecraft/network/codec/ByteBufCodecs#COMPOUND_TAG` | field | 1 |
| `net/minecraft/network/codec/ByteBufCodecs#FLOAT` | field | 1 |
| `net/minecraft/network/codec/ByteBufCodecs#INT` | field | 11 |
| `net/minecraft/network/codec/ByteBufCodecs#STRING_UTF8` | field | 3 |
| `net/minecraft/network/codec/ByteBufCodecs#VAR_LONG` | field | 2 |
| `net/minecraft/network/codec/ByteBufCodecs#fromCodec` | method | 3 |
| `net/minecraft/network/codec/ByteBufCodecs#fromCodecWithRegistries` | method | 3 |
| `net/minecraft/network/codec/StreamCodec#apply` | method | 5 |
| `net/minecraft/network/codec/StreamCodec#composite` | method | 23 |
| `net/minecraft/network/codec/StreamCodec#map` | method | 1 |
| `net/minecraft/network/protocol/common/custom/CustomPacketPayload$Type#<init>` | method | 14 |
| `net/minecraft/network/protocol/game/ClientboundBlockEntityDataPacket#create` | method | 1 |
| `net/minecraft/network/protocol/game/ClientboundSetHeldSlotPacket#<init>` | method | 1 |
| `net/minecraft/resources/Identifier#CODEC` | field | 1 |
| `net/minecraft/resources/Identifier#equals` | method | 1 |
| `net/minecraft/resources/Identifier#fromNamespaceAndPath` | method | 43 |
| `net/minecraft/resources/Identifier#getNamespace` | method | 1 |
| `net/minecraft/resources/Identifier#parse` | method | 1 |
| `net/minecraft/resources/Identifier#toString` | method | 3 |
| `net/minecraft/resources/Identifier#tryParse` | method | 2 |
| `net/minecraft/resources/Identifier#withDefaultNamespace` | method | 2 |
| `net/minecraft/resources/ResourceKey#create` | method | 5 |
| `net/minecraft/server/MinecraftServer#execute` | method | 8 |
| `net/minecraft/server/MinecraftServer#getTickCount` | method | 1 |
| `net/minecraft/server/MinecraftServer#getWorldPath` | method | 1 |
| `net/minecraft/server/MinecraftServer#registryAccess` | method | 1 |
| `net/minecraft/server/level/ServerLevel#addFreshEntity` | method | 1 |
| `net/minecraft/server/level/ServerLevel#enabledFeatures` | method | 1 |
| `net/minecraft/server/level/ServerLevel#getBlockEntity` | method | 3 |
| `net/minecraft/server/level/ServerLevel#getGameRules` | method | 1 |
| `net/minecraft/server/level/ServerLevel#getNearestEntity` | method | 1 |
| `net/minecraft/server/level/ServerLevel#getRandom` | method | 2 |
| `net/minecraft/server/level/ServerLevel#getServer` | method | 1 |
| `net/minecraft/server/level/ServerLevel#playSound` | method | 1 |
| `net/minecraft/server/level/ServerLevel#recipeAccess` | method | 1 |
| `net/minecraft/server/level/ServerLevel#registryAccess` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#addItem` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#blockPosition` | method | 3 |
| `net/minecraft/server/level/ServerPlayer#closeContainer` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#connection` | field | 2 |
| `net/minecraft/server/level/ServerPlayer#containerMenu` | field | 6 |
| `net/minecraft/server/level/ServerPlayer#drop` | method | 3 |
| `net/minecraft/server/level/ServerPlayer#getId` | method | 5 |
| `net/minecraft/server/level/ServerPlayer#getInventory` | method | 4 |
| `net/minecraft/server/level/ServerPlayer#getUUID` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#getX` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#getXRot` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#getY` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#getYRot` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#getZ` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#hasDisconnected` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#hasInfiniteMaterials` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#inventoryMenu` | field | 1 |
| `net/minecraft/server/level/ServerPlayer#isWithinBlockInteractionRange` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#level` | method | 10 |
| `net/minecraft/server/level/ServerPlayer#openMenu` | method | 2 |
| `net/minecraft/server/level/ServerPlayer#registryAccess` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#sendOverlayMessage` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#sendSystemMessage` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#setRespawnPosition` | method | 1 |
| `net/minecraft/server/level/ServerPlayer$RespawnConfig#<init>` | method | 1 |
| `net/minecraft/server/network/ServerGamePacketListenerImpl#send` | method | 1 |
| `net/minecraft/server/packs/PackType#CLIENT_RESOURCES` | field | 1 |
| `net/minecraft/server/permissions/PermissionSet#hasPermission` | method | 4 |
| `net/minecraft/server/permissions/Permissions#COMMANDS_GAMEMASTER` | field | 4 |
| `net/minecraft/sounds/SoundEvent#location` | method | 1 |
| `net/minecraft/sounds/SoundEvents#ARMOR_EQUIP_LEATHER` | field | 4 |
| `net/minecraft/sounds/SoundEvents#BOTTLE_FILL` | field | 1 |
| `net/minecraft/sounds/SoundEvents#BREWING_STAND_BREW` | field | 2 |
| `net/minecraft/sounds/SoundEvents#BUCKET_EMPTY` | field | 3 |
| `net/minecraft/sounds/SoundEvents#BUCKET_EMPTY_LAVA` | field | 1 |
| `net/minecraft/sounds/SoundEvents#BUCKET_FILL` | field | 4 |
| `net/minecraft/sounds/SoundEvents#BUCKET_FILL_LAVA` | field | 1 |
| `net/minecraft/sounds/SoundEvents#BUNDLE_INSERT` | field | 1 |
| `net/minecraft/sounds/SoundEvents#CHICKEN_EGG` | field | 1 |
| `net/minecraft/sounds/SoundEvents#COPPER_BULB_HIT` | field | 1 |
| `net/minecraft/sounds/SoundEvents#CREEPER_PRIMED` | field | 1 |
| `net/minecraft/sounds/SoundEvents#FIRE_EXTINGUISH` | field | 1 |
| `net/minecraft/sounds/SoundEvents#FURNACE_FIRE_CRACKLE` | field | 1 |
| `net/minecraft/sounds/SoundEvents#GENERIC_DRINK` | field | 1 |
| `net/minecraft/sounds/SoundEvents#GENERIC_EAT` | field | 1 |
| `net/minecraft/sounds/SoundEvents#HONEYCOMB_WAX_ON` | field | 1 |
| `net/minecraft/sounds/SoundEvents#ITEM_PICKUP` | field | 2 |
| `net/minecraft/sounds/SoundEvents#LINGERING_POTION_THROW` | field | 1 |
| `net/minecraft/sounds/SoundEvents#SHEEP_SHEAR` | field | 1 |
| `net/minecraft/sounds/SoundEvents#SHIELD_BLOCK` | field | 1 |
| `net/minecraft/sounds/SoundEvents#SPLASH_POTION_THROW` | field | 1 |
| `net/minecraft/sounds/SoundEvents#SPONGE_ABSORB` | field | 1 |
| `net/minecraft/sounds/SoundEvents#UI_BUTTON_CLICK` | field | 2 |
| `net/minecraft/sounds/SoundEvents#WOOL_PLACE` | field | 2 |
| `net/minecraft/sounds/SoundSource#AMBIENT` | field | 1 |
| `net/minecraft/sounds/SoundSource#BLOCKS` | field | 8 |
| `net/minecraft/sounds/SoundSource#MASTER` | field | 1 |
| `net/minecraft/sounds/SoundSource#NEUTRAL` | field | 1 |
| `net/minecraft/sounds/SoundSource#PLAYERS` | field | 6 |
| `net/minecraft/sounds/SoundSource#RECORDS` | field | 2 |
| `net/minecraft/stats/StatType#get` | method | 1 |
| `net/minecraft/stats/Stats#ITEM_USED` | field | 1 |
| `net/minecraft/tags/FluidTags#LAVA` | field | 1 |
| `net/minecraft/tags/FluidTags#WATER` | field | 2 |
| `net/minecraft/tags/ItemTags#CAULDRON_CAN_REMOVE_DYE` | field | 1 |
| `net/minecraft/tags/ItemTags#COALS` | field | 1 |
| `net/minecraft/tags/ItemTags#DYES` | field | 1 |
| `net/minecraft/tags/ItemTags#PLANKS` | field | 1 |
| `net/minecraft/tags/TagKey#create` | method | 2 |
| `net/minecraft/tags/TagKey#location` | method | 1 |
| `net/minecraft/util/ExtraCodecs#ARGB_COLOR_CODEC` | field | 3 |
| `net/minecraft/util/ExtraCodecs$LateBoundIdMapper#put` | method | 2 |
| `net/minecraft/util/Mth#cos` | method | 4 |
| `net/minecraft/util/Mth#getSeed` | method | 1 |
| `net/minecraft/util/Mth#randomBetween` | method | 1 |
| `net/minecraft/util/Mth#sin` | method | 4 |
| `net/minecraft/util/Prediction#SERVER_ONLY` | field | 8 |
| `net/minecraft/util/RandomSource#create` | method | 1 |
| `net/minecraft/util/RandomSource#nextDouble` | method | 5 |
| `net/minecraft/util/RandomSource#nextFloat` | method | 13 |
| `net/minecraft/util/RandomSource#nextGaussian` | method | 1 |
| `net/minecraft/util/RandomSource#nextInt` | method | 4 |
| `net/minecraft/util/RandomSource#nextIntBetweenInclusive` | method | 2 |
| `net/minecraft/world/Container#canPlaceItem` | method | 1 |
| `net/minecraft/world/Container#equals` | method | 1 |
| `net/minecraft/world/Container#getContainerSize` | method | 1 |
| `net/minecraft/world/Container#getItem` | method | 4 |
| `net/minecraft/world/Container#getMaxStackSize` | method | 2 |
| `net/minecraft/world/Container#hashCode` | method | 1 |
| `net/minecraft/world/Container#removeItem` | method | 1 |
| `net/minecraft/world/Container#setChanged` | method | 1 |
| `net/minecraft/world/Container#setItem` | method | 4 |
| `net/minecraft/world/Containers#dropContents` | method | 2 |
| `net/minecraft/world/InteractionHand#MAIN_HAND` | field | 8 |
| `net/minecraft/world/InteractionHand#OFF_HAND` | field | 2 |
| `net/minecraft/world/InteractionHand#asEquipmentSlot` | method | 1 |
| `net/minecraft/world/InteractionResult#CONSUME` | field | 3 |
| `net/minecraft/world/InteractionResult#FAIL` | field | 3 |
| `net/minecraft/world/InteractionResult#PASS` | field | 2 |
| `net/minecraft/world/InteractionResult#SUCCESS` | field | 3 |
| `net/minecraft/world/InteractionResult#SUCCESS_SERVER` | field | 3 |
| `net/minecraft/world/InteractionResult#consumesAction` | method | 1 |
| `net/minecraft/world/ItemStackWithSlot#<init>` | method | 1 |
| `net/minecraft/world/ItemStackWithSlot#CODEC` | field | 1 |
| `net/minecraft/world/ItemStackWithSlot#isValidInContainer` | method | 1 |
| `net/minecraft/world/ItemStackWithSlot#slot` | method | 1 |
| `net/minecraft/world/ItemStackWithSlot#stack` | method | 1 |
| `net/minecraft/world/MenuProvider#createMenu` | method | 1 |
| `net/minecraft/world/MenuProvider#getDisplayName` | method | 1 |
| `net/minecraft/world/MenuProvider#shouldCloseCurrentScreen` | method | 1 |
| `net/minecraft/world/SimpleContainer#<init>` | method | 3 |
| `net/minecraft/world/TickRateManager#tickrate` | method | 1 |
| `net/minecraft/world/attribute/BedRule#DESTROY_ON_LEAVE` | field | 1 |
| `net/minecraft/world/attribute/BedRule#destroyOnUse` | method | 2 |
| `net/minecraft/world/attribute/BedRule#errorMessage` | method | 1 |
| `net/minecraft/world/attribute/EnvironmentAttributeSystem#getValue` | method | 2 |
| `net/minecraft/world/attribute/EnvironmentAttributes#BED_RULE` | field | 1 |
| `net/minecraft/world/attribute/EnvironmentAttributes#WATER_EVAPORATES` | field | 1 |
| `net/minecraft/world/damagesource/DamageSource#getDirectEntity` | method | 1 |
| `net/minecraft/world/damagesource/DamageSources#playerAttack` | method | 1 |
| `net/minecraft/world/damagesource/DamageSources#sting` | method | 1 |
| `net/minecraft/world/effect/MobEffect#applyInstantaneousEffect` | method | 1 |
| `net/minecraft/world/effect/MobEffect#getCategory` | method | 3 |
| `net/minecraft/world/effect/MobEffect#getDescriptionId` | method | 1 |
| `net/minecraft/world/effect/MobEffectCategory#HARMFUL` | field | 2 |
| `net/minecraft/world/effect/MobEffectCategory#getTooltipFormatting` | method | 1 |
| `net/minecraft/world/effect/MobEffectInstance#<init>` | method | 7 |
| `net/minecraft/world/effect/MobEffectInstance#getAmplifier` | method | 1 |
| `net/minecraft/world/effect/MobEffectInstance#getDuration` | method | 1 |
| `net/minecraft/world/effect/MobEffectInstance#getEffect` | method | 3 |
| `net/minecraft/world/effect/MobEffects#ABSORPTION` | field | 1 |
| `net/minecraft/world/effect/MobEffects#FIRE_RESISTANCE` | field | 1 |
| `net/minecraft/world/effect/MobEffects#JUMP_BOOST` | field | 1 |
| `net/minecraft/world/effect/MobEffects#MINING_FATIGUE` | field | 1 |
| `net/minecraft/world/effect/MobEffects#NIGHT_VISION` | field | 1 |
| `net/minecraft/world/effect/MobEffects#POISON` | field | 1 |
| `net/minecraft/world/effect/MobEffects#REGENERATION` | field | 2 |
| `net/minecraft/world/effect/MobEffects#SLOWNESS` | field | 2 |
| `net/minecraft/world/effect/MobEffects#SPEED` | field | 2 |
| `net/minecraft/world/effect/MobEffects#WITHER` | field | 1 |
| `net/minecraft/world/entity/Entity#<init>` | method | 1 |
| `net/minecraft/world/entity/Entity#equals` | method | 1 |
| `net/minecraft/world/entity/Entity#hurtServer` | method | 1 |
| `net/minecraft/world/entity/Entity#isAlive` | method | 1 |
| `net/minecraft/world/entity/Entity#isInWater` | method | 1 |
| `net/minecraft/world/entity/Entity#isInvulnerable` | method | 1 |
| `net/minecraft/world/entity/Entity#level` | method | 3 |
| `net/minecraft/world/entity/Entity#onBelowWorld` | method | 1 |
| `net/minecraft/world/entity/EntityDimensions#width` | method | 1 |
| `net/minecraft/world/entity/EntityType$Builder#build` | method | 1 |
| `net/minecraft/world/entity/EntityType$Builder#clientTrackingRange` | method | 1 |
| `net/minecraft/world/entity/EntityType$Builder#of` | method | 1 |
| `net/minecraft/world/entity/EntityType$Builder#sized` | method | 1 |
| `net/minecraft/world/entity/EntityType$Builder#updateInterval` | method | 1 |
| `net/minecraft/world/entity/EntityTypes#ITEM` | field | 2 |
| `net/minecraft/world/entity/EntityTypes#PIGLIN` | field | 1 |
| `net/minecraft/world/entity/EntityTypes#PLAYER` | field | 3 |
| `net/minecraft/world/entity/EntityTypes#WITHER_SKELETON` | field | 1 |
| `net/minecraft/world/entity/EquipmentSlot#CHEST` | field | 3 |
| `net/minecraft/world/entity/ExperienceOrb#playerTouch` | method | 1 |
| `net/minecraft/world/entity/ItemOwner#asLivingEntity` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#<init>` | method | 3 |
| `net/minecraft/world/entity/LivingEntity#addEffect` | method | 2 |
| `net/minecraft/world/entity/LivingEntity#dropAllDeathLoot` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#getItemBySlot` | method | 3 |
| `net/minecraft/world/entity/LivingEntity#getRandom` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#getStingerCount` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#getType` | method | 2 |
| `net/minecraft/world/entity/LivingEntity#getX` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#getY` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#getZ` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#isBaby` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#isShiftKeyDown` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#isSleeping` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#level` | method | 2 |
| `net/minecraft/world/entity/LivingEntity#setItemSlot` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#setStingerCount` | method | 1 |
| `net/minecraft/world/entity/Mob#setTarget` | method | 1 |
| `net/minecraft/world/entity/MobCategory#MISC` | field | 1 |
| `net/minecraft/world/entity/Pose#STANDING` | field | 1 |
| `net/minecraft/world/entity/ai/attributes/Attribute#getDescriptionId` | method | 1 |
| `net/minecraft/world/entity/ai/attributes/Attribute#getStyle` | method | 1 |
| `net/minecraft/world/entity/ai/attributes/AttributeInstance#addPermanentModifier` | method | 1 |
| `net/minecraft/world/entity/ai/attributes/AttributeInstance#hasModifier` | method | 1 |
| `net/minecraft/world/entity/ai/attributes/AttributeInstance#removeModifier` | method | 1 |
| `net/minecraft/world/entity/ai/attributes/AttributeModifier#<init>` | method | 1 |
| `net/minecraft/world/entity/ai/attributes/AttributeModifier#amount` | method | 1 |
| `net/minecraft/world/entity/ai/attributes/AttributeModifier#id` | method | 1 |
| `net/minecraft/world/entity/ai/attributes/AttributeModifier#operation` | method | 1 |
| `net/minecraft/world/entity/ai/attributes/AttributeModifier$Operation#ADD_MULTIPLIED_BASE` | field | 1 |
| `net/minecraft/world/entity/ai/attributes/AttributeModifier$Operation#ADD_MULTIPLIED_TOTAL` | field | 2 |
| `net/minecraft/world/entity/ai/attributes/AttributeModifier$Operation#ADD_VALUE` | field | 1 |
| `net/minecraft/world/entity/ai/attributes/AttributeModifier$Operation#id` | method | 1 |
| `net/minecraft/world/entity/ai/attributes/Attributes#ARMOR` | field | 1 |
| `net/minecraft/world/entity/ai/attributes/Attributes#BLOCK_INTERACTION_RANGE` | field | 1 |
| `net/minecraft/world/entity/ai/attributes/Attributes#ENTITY_INTERACTION_RANGE` | field | 1 |
| `net/minecraft/world/entity/ai/attributes/Attributes#KNOCKBACK_RESISTANCE` | field | 1 |
| `net/minecraft/world/entity/ai/attributes/Attributes#LUCK` | field | 1 |
| `net/minecraft/world/entity/ai/attributes/Attributes#MAX_HEALTH` | field | 1 |
| `net/minecraft/world/entity/ai/attributes/Attributes#MOVEMENT_SPEED` | field | 1 |
| `net/minecraft/world/entity/ai/targeting/TargetingConditions#forCombat` | method | 1 |
| `net/minecraft/world/entity/ai/targeting/TargetingConditions#range` | method | 1 |
| `net/minecraft/world/entity/item/ItemEntity#<init>` | method | 3 |
| `net/minecraft/world/entity/item/ItemEntity#fireImmune` | method | 1 |
| `net/minecraft/world/entity/item/ItemEntity#getItem` | method | 4 |
| `net/minecraft/world/entity/item/ItemEntity#getOwner` | method | 1 |
| `net/minecraft/world/entity/item/ItemEntity#ignoreExplosion` | method | 1 |
| `net/minecraft/world/entity/item/ItemEntity#isAlive` | method | 1 |
| `net/minecraft/world/entity/item/ItemEntity#isInWater` | method | 1 |
| `net/minecraft/world/entity/item/ItemEntity#playerTouch` | method | 1 |
| `net/minecraft/world/entity/item/ItemEntity#setDefaultPickUpDelay` | method | 2 |
| `net/minecraft/world/entity/item/ItemEntity#setItem` | method | 2 |
| `net/minecraft/world/entity/item/ItemEntity#setNoPickUpDelay` | method | 1 |
| `net/minecraft/world/entity/item/ItemEntity#setPos` | method | 2 |
| `net/minecraft/world/entity/item/ItemEntity#tickCount` | field | 1 |
| `net/minecraft/world/entity/monster/Enderman#isBeingStaredBy` | method | 1 |
| `net/minecraft/world/entity/monster/Ghast#getLastAttacker` | method | 1 |
| `net/minecraft/world/entity/monster/Monster#<init>` | method | 1 |
| `net/minecraft/world/entity/npc/villager/Villager#stopSleeping` | method | 1 |
| `net/minecraft/world/entity/player/Abilities#flying` | field | 1 |
| `net/minecraft/world/entity/player/Abilities#instabuild` | field | 2 |
| `net/minecraft/world/entity/player/Input#<init>` | method | 1 |
| `net/minecraft/world/entity/player/Input#backward` | method | 1 |
| `net/minecraft/world/entity/player/Input#forward` | method | 1 |
| `net/minecraft/world/entity/player/Input#left` | method | 1 |
| `net/minecraft/world/entity/player/Input#right` | method | 1 |
| `net/minecraft/world/entity/player/Inventory#findSlotMatchingItem` | method | 1 |
| `net/minecraft/world/entity/player/Inventory#getContainerSize` | method | 1 |
| `net/minecraft/world/entity/player/Inventory#getFreeSlot` | method | 2 |
| `net/minecraft/world/entity/player/Inventory#getItem` | method | 3 |
| `net/minecraft/world/entity/player/Inventory#getNonEquipmentItems` | method | 8 |
| `net/minecraft/world/entity/player/Inventory#getSelectedItem` | method | 3 |
| `net/minecraft/world/entity/player/Inventory#getSelectedSlot` | method | 7 |
| `net/minecraft/world/entity/player/Inventory#getSelectionSize` | method | 1 |
| `net/minecraft/world/entity/player/Inventory#getSlotWithRemainingSpace` | method | 1 |
| `net/minecraft/world/entity/player/Inventory#getSuitableHotbarSlot` | method | 1 |
| `net/minecraft/world/entity/player/Inventory#placeItemBackInInventory` | method | 3 |
| `net/minecraft/world/entity/player/Inventory#player` | field | 9 |
| `net/minecraft/world/entity/player/Inventory#setItem` | method | 1 |
| `net/minecraft/world/entity/player/Inventory#setSelectedSlot` | method | 2 |
| `net/minecraft/world/entity/player/Player#addEffect` | method | 5 |
| `net/minecraft/world/entity/player/Player#addItem` | method | 1 |
| `net/minecraft/world/entity/player/Player#awardRecipes` | method | 1 |
| `net/minecraft/world/entity/player/Player#awardStat` | method | 1 |
| `net/minecraft/world/entity/player/Player#blockPosition` | method | 13 |
| `net/minecraft/world/entity/player/Player#containerMenu` | field | 10 |
| `net/minecraft/world/entity/player/Player#damageSources` | method | 1 |
| `net/minecraft/world/entity/player/Player#drop` | method | 4 |
| `net/minecraft/world/entity/player/Player#fallDistance` | field | 1 |
| `net/minecraft/world/entity/player/Player#gameEvent` | method | 1 |
| `net/minecraft/world/entity/player/Player#getAbilities` | method | 2 |
| `net/minecraft/world/entity/player/Player#getActiveEffects` | method | 1 |
| `net/minecraft/world/entity/player/Player#getAttachedOrCreate` | method | 1 |
| `net/minecraft/world/entity/player/Player#getAttribute` | method | 1 |
| `net/minecraft/world/entity/player/Player#getBoundingBox` | method | 1 |
| `net/minecraft/world/entity/player/Player#getCooldowns` | method | 1 |
| `net/minecraft/world/entity/player/Player#getDeltaMovement` | method | 1 |
| `net/minecraft/world/entity/player/Player#getDimensions` | method | 1 |
| `net/minecraft/world/entity/player/Player#getDirection` | method | 2 |
| `net/minecraft/world/entity/player/Player#getEffect` | method | 1 |
| `net/minecraft/world/entity/player/Player#getFlyingSpeed` | method | 1 |
| `net/minecraft/world/entity/player/Player#getFoodData` | method | 3 |
| `net/minecraft/world/entity/player/Player#getGameProfile` | method | 3 |
| `net/minecraft/world/entity/player/Player#getId` | method | 7 |
| `net/minecraft/world/entity/player/Player#getInventory` | method | 16 |
| `net/minecraft/world/entity/player/Player#getItemInHand` | method | 5 |
| `net/minecraft/world/entity/player/Player#getMainHandItem` | method | 5 |
| `net/minecraft/world/entity/player/Player#getOffhandItem` | method | 2 |
| `net/minecraft/world/entity/player/Player#getRandom` | method | 1 |
| `net/minecraft/world/entity/player/Player#getRandomX` | method | 1 |
| `net/minecraft/world/entity/player/Player#getRandomY` | method | 1 |
| `net/minecraft/world/entity/player/Player#getRandomZ` | method | 1 |
| `net/minecraft/world/entity/player/Player#getX` | method | 3 |
| `net/minecraft/world/entity/player/Player#getY` | method | 3 |
| `net/minecraft/world/entity/player/Player#getZ` | method | 3 |
| `net/minecraft/world/entity/player/Player#horizontalCollision` | field | 1 |
| `net/minecraft/world/entity/player/Player#isAlive` | method | 4 |
| `net/minecraft/world/entity/player/Player#isCreative` | method | 2 |
| `net/minecraft/world/entity/player/Player#isCrouching` | method | 2 |
| `net/minecraft/world/entity/player/Player#isDeadOrDying` | method | 1 |
| `net/minecraft/world/entity/player/Player#isFallFlying` | method | 1 |
| `net/minecraft/world/entity/player/Player#isInLiquid` | method | 1 |
| `net/minecraft/world/entity/player/Player#isInWater` | method | 1 |
| `net/minecraft/world/entity/player/Player#isShiftKeyDown` | method | 1 |
| `net/minecraft/world/entity/player/Player#isUsingItem` | method | 1 |
| `net/minecraft/world/entity/player/Player#isWithinBlockInteractionRange` | method | 2 |
| `net/minecraft/world/entity/player/Player#level` | method | 27 |
| `net/minecraft/world/entity/player/Player#mayUseItemAt` | method | 1 |
| `net/minecraft/world/entity/player/Player#openMenu` | method | 1 |
| `net/minecraft/world/entity/player/Player#playSound` | method | 2 |
| `net/minecraft/world/entity/player/Player#position` | method | 4 |
| `net/minecraft/world/entity/player/Player#readAdditionalSaveData` | method | 1 |
| `net/minecraft/world/entity/player/Player#removeAllEffects` | method | 2 |
| `net/minecraft/world/entity/player/Player#removeEffect` | method | 1 |
| `net/minecraft/world/entity/player/Player#sendOverlayMessage` | method | 4 |
| `net/minecraft/world/entity/player/Player#setDeltaMovement` | method | 1 |
| `net/minecraft/world/entity/player/Player#setHealth` | method | 1 |
| `net/minecraft/world/entity/player/Player#setItemInHand` | method | 3 |
| `net/minecraft/world/entity/player/Player#setRemainingFireTicks` | method | 1 |
| `net/minecraft/world/entity/player/Player#startSleepInBed` | method | 2 |
| `net/minecraft/world/entity/player/Player#startUsingItem` | method | 1 |
| `net/minecraft/world/entity/player/Player#stopFallFlying` | method | 1 |
| `net/minecraft/world/entity/player/Player#take` | method | 1 |
| `net/minecraft/world/entity/player/Player#triggerRecipeCrafted` | method | 1 |
| `net/minecraft/world/entity/player/Player$BedSleepingProblem#message` | method | 2 |
| `net/minecraft/world/entity/player/StackedItemContents#accountSimpleStack` | method | 1 |
| `net/minecraft/world/entity/player/StackedItemContents#clear` | method | 1 |
| `net/minecraft/world/entity/projectile/Projectile#spawnProjectileFromRotation` | method | 1 |
| `net/minecraft/world/entity/projectile/hurtingprojectile/Fireball#<init>` | method | 1 |
| `net/minecraft/world/entity/projectile/hurtingprojectile/SmallFireball#discard` | method | 1 |
| `net/minecraft/world/entity/projectile/hurtingprojectile/SmallFireball#onHitEntity` | method | 1 |
| `net/minecraft/world/entity/projectile/throwableitemprojectile/ThrownLingeringPotion#<init>` | method | 1 |
| `net/minecraft/world/entity/projectile/throwableitemprojectile/ThrownSplashPotion#<init>` | method | 1 |
| `net/minecraft/world/food/FoodData#eat` | method | 1 |
| `net/minecraft/world/food/FoodData#getFoodLevel` | method | 2 |
| `net/minecraft/world/food/FoodData#needsFood` | method | 1 |
| `net/minecraft/world/food/FoodProperties#nutrition` | method | 2 |
| `net/minecraft/world/inventory/AbstractContainerMenu#<init>` | method | 1 |
| `net/minecraft/world/inventory/AbstractContainerMenu#broadcastChanges` | method | 3 |
| `net/minecraft/world/inventory/AbstractContainerMenu#getCarried` | method | 2 |
| `net/minecraft/world/inventory/AbstractContainerMenu#getSlot` | method | 1 |
| `net/minecraft/world/inventory/AbstractContainerMenu#slots` | field | 5 |
| `net/minecraft/world/inventory/ClickAction#SECONDARY` | field | 1 |
| `net/minecraft/world/inventory/ContainerInput#PICKUP` | field | 1 |
| `net/minecraft/world/inventory/ContainerInput#SWAP` | field | 1 |
| `net/minecraft/world/inventory/ContainerLevelAccess#create` | method | 2 |
| `net/minecraft/world/inventory/ContainerLevelAccess#evaluate` | method | 2 |
| `net/minecraft/world/inventory/CraftingContainer#asCraftInput` | method | 1 |
| `net/minecraft/world/inventory/CraftingContainer#asPositionedCraftInput` | method | 1 |
| `net/minecraft/world/inventory/CraftingContainer#getItem` | method | 1 |
| `net/minecraft/world/inventory/CraftingContainer#getItems` | method | 1 |
| `net/minecraft/world/inventory/CraftingContainer#getWidth` | method | 1 |
| `net/minecraft/world/inventory/CraftingContainer#setItem` | method | 1 |
| `net/minecraft/world/inventory/InventoryMenu#broadcastChanges` | method | 1 |
| `net/minecraft/world/inventory/InventoryMenu#getCarried` | method | 1 |
| `net/minecraft/world/inventory/InventoryMenu#setCarried` | method | 1 |
| `net/minecraft/world/inventory/ResultContainer#<init>` | method | 1 |
| `net/minecraft/world/inventory/ResultContainer#getItem` | method | 1 |
| `net/minecraft/world/inventory/ResultContainer#getRecipeUsed` | method | 2 |
| `net/minecraft/world/inventory/ResultContainer#setItem` | method | 3 |
| `net/minecraft/world/inventory/ResultContainer#setRecipeUsed` | method | 1 |
| `net/minecraft/world/inventory/ResultSlot#<init>` | method | 1 |
| `net/minecraft/world/inventory/ResultSlot#getItem` | method | 1 |
| `net/minecraft/world/inventory/ResultSlot#onSwapCraft` | method | 1 |
| `net/minecraft/world/inventory/ResultSlot#safeTake` | method | 1 |
| `net/minecraft/world/inventory/Slot#<init>` | method | 6 |
| `net/minecraft/world/inventory/Slot#allowModification` | method | 3 |
| `net/minecraft/world/inventory/Slot#container` | field | 5 |
| `net/minecraft/world/inventory/Slot#getContainerSlot` | method | 5 |
| `net/minecraft/world/inventory/Slot#getItem` | method | 8 |
| `net/minecraft/world/inventory/Slot#getMaxStackSize` | method | 1 |
| `net/minecraft/world/inventory/Slot#hasItem` | method | 3 |
| `net/minecraft/world/inventory/Slot#index` | field | 5 |
| `net/minecraft/world/inventory/Slot#mayPickup` | method | 3 |
| `net/minecraft/world/inventory/Slot#mayPlace` | method | 2 |
| `net/minecraft/world/inventory/Slot#onTake` | method | 1 |
| `net/minecraft/world/inventory/Slot#safeTake` | method | 1 |
| `net/minecraft/world/inventory/Slot#set` | method | 3 |
| `net/minecraft/world/inventory/Slot#setChanged` | method | 1 |
| `net/minecraft/world/inventory/Slot#x` | field | 6 |
| `net/minecraft/world/inventory/Slot#y` | field | 7 |
| `net/minecraft/world/item/BlockItem#<init>` | method | 2 |
| `net/minecraft/world/item/BlockItem#appendHoverText` | method | 2 |
| `net/minecraft/world/item/BlockItem#getBlock` | method | 1 |
| `net/minecraft/world/item/BlockItem#isEnabled` | method | 1 |
| `net/minecraft/world/item/BlockItem#overrideOtherStackedOnMe` | method | 1 |
| `net/minecraft/world/item/BlockItem#overrideStackedOnOther` | method | 1 |
| `net/minecraft/world/item/CreativeModeTab$Builder#build` | method | 1 |
| `net/minecraft/world/item/CreativeModeTab$Builder#icon` | method | 1 |
| `net/minecraft/world/item/CreativeModeTab$Builder#title` | method | 1 |
| `net/minecraft/world/item/DyeColor#BLACK` | field | 2 |
| `net/minecraft/world/item/DyeColor#BLUE` | field | 2 |
| `net/minecraft/world/item/DyeColor#BROWN` | field | 2 |
| `net/minecraft/world/item/DyeColor#CYAN` | field | 2 |
| `net/minecraft/world/item/DyeColor#GRAY` | field | 2 |
| `net/minecraft/world/item/DyeColor#GREEN` | field | 2 |
| `net/minecraft/world/item/DyeColor#LIGHT_BLUE` | field | 2 |
| `net/minecraft/world/item/DyeColor#LIGHT_GRAY` | field | 2 |
| `net/minecraft/world/item/DyeColor#LIME` | field | 2 |
| `net/minecraft/world/item/DyeColor#MAGENTA` | field | 2 |
| `net/minecraft/world/item/DyeColor#ORANGE` | field | 2 |
| `net/minecraft/world/item/DyeColor#PINK` | field | 2 |
| `net/minecraft/world/item/DyeColor#PURPLE` | field | 2 |
| `net/minecraft/world/item/DyeColor#RED` | field | 9 |
| `net/minecraft/world/item/DyeColor#WHITE` | field | 2 |
| `net/minecraft/world/item/DyeColor#YELLOW` | field | 2 |
| `net/minecraft/world/item/DyeColor#byId` | method | 1 |
| `net/minecraft/world/item/DyeColor#getId` | method | 8 |
| `net/minecraft/world/item/DyeColor#getMapColor` | method | 1 |
| `net/minecraft/world/item/DyeColor#getName` | method | 1 |
| `net/minecraft/world/item/DyeColor#getTextureDiffuseColor` | method | 1 |
| `net/minecraft/world/item/DyeColor#values` | method | 2 |
| `net/minecraft/world/item/Item#CODEC` | field | 1 |
| `net/minecraft/world/item/Item#appendHoverText` | method | 2 |
| `net/minecraft/world/item/Item#canFitInsideContainerItems` | method | 2 |
| `net/minecraft/world/item/Item#getDefaultInstance` | method | 10 |
| `net/minecraft/world/item/Item#getDefaultMaxStackSize` | method | 1 |
| `net/minecraft/world/item/Item#getDescriptionId` | method | 1 |
| `net/minecraft/world/item/Item#getName` | method | 1 |
| `net/minecraft/world/item/Item#isEnabled` | method | 2 |
| `net/minecraft/world/item/Item$Properties#<init>` | method | 1 |
| `net/minecraft/world/item/Item$Properties#component` | method | 3 |
| `net/minecraft/world/item/Item$Properties#fireResistant` | method | 1 |
| `net/minecraft/world/item/Item$Properties#setId` | method | 1 |
| `net/minecraft/world/item/Item$Properties#useBlockDescriptionPrefix` | method | 2 |
| `net/minecraft/world/item/Item$TooltipContext#of` | method | 2 |
| `net/minecraft/world/item/ItemCooldowns#isOnCooldown` | method | 1 |
| `net/minecraft/world/item/ItemDisplayContext#GUI` | field | 1 |
| `net/minecraft/world/item/ItemDisplayContext#NONE` | field | 3 |
| `net/minecraft/world/item/ItemStack#<init>` | method | 19 |
| `net/minecraft/world/item/ItemStack#CODEC` | field | 1 |
| `net/minecraft/world/item/ItemStack#EMPTY` | field | 31 |
| `net/minecraft/world/item/ItemStack#OPTIONAL_CODEC` | field | 6 |
| `net/minecraft/world/item/ItemStack#OPTIONAL_STREAM_CODEC` | field | 4 |
| `net/minecraft/world/item/ItemStack#STREAM_CODEC` | field | 2 |
| `net/minecraft/world/item/ItemStack#applyComponents` | method | 5 |
| `net/minecraft/world/item/ItemStack#copy` | method | 31 |
| `net/minecraft/world/item/ItemStack#copyWithCount` | method | 6 |
| `net/minecraft/world/item/ItemStack#finishUsingItem` | method | 1 |
| `net/minecraft/world/item/ItemStack#get` | method | 24 |
| `net/minecraft/world/item/ItemStack#getComponents` | method | 1 |
| `net/minecraft/world/item/ItemStack#getComponentsPatch` | method | 2 |
| `net/minecraft/world/item/ItemStack#getCraftingRemainder` | method | 1 |
| `net/minecraft/world/item/ItemStack#getDamageValue` | method | 2 |
| `net/minecraft/world/item/ItemStack#getHoverName` | method | 1 |
| `net/minecraft/world/item/ItemStack#getInteractAnimation` | method | 1 |
| `net/minecraft/world/item/ItemStack#getItem` | method | 51 |
| `net/minecraft/world/item/ItemStack#getMaxStackSize` | method | 11 |
| `net/minecraft/world/item/ItemStack#getOrDefault` | method | 42 |
| `net/minecraft/world/item/ItemStack#getPrototype` | method | 1 |
| `net/minecraft/world/item/ItemStack#getTooltipImage` | method | 1 |
| `net/minecraft/world/item/ItemStack#getTooltipLines` | method | 1 |
| `net/minecraft/world/item/ItemStack#grow` | method | 8 |
| `net/minecraft/world/item/ItemStack#has` | method | 23 |
| `net/minecraft/world/item/ItemStack#hashItemAndComponents` | method | 1 |
| `net/minecraft/world/item/ItemStack#is` | method | 14 |
| `net/minecraft/world/item/ItemStack#isEmpty` | method | 58 |
| `net/minecraft/world/item/ItemStack#isItemEnabled` | method | 1 |
| `net/minecraft/world/item/ItemStack#isSameItem` | method | 8 |
| `net/minecraft/world/item/ItemStack#isStackable` | method | 3 |
| `net/minecraft/world/item/ItemStack#onCraftedBy` | method | 2 |
| `net/minecraft/world/item/ItemStack#overrideOtherStackedOnMe` | method | 1 |
| `net/minecraft/world/item/ItemStack#remove` | method | 4 |
| `net/minecraft/world/item/ItemStack#set` | method | 19 |
| `net/minecraft/world/item/ItemStack#setCount` | method | 9 |
| `net/minecraft/world/item/ItemStack#split` | method | 5 |
| `net/minecraft/world/item/ItemStack#tags` | method | 1 |
| `net/minecraft/world/item/ItemStack#typeHolder` | method | 2 |
| `net/minecraft/world/item/ItemStack#update` | method | 4 |
| `net/minecraft/world/item/ItemStack#use` | method | 1 |
| `net/minecraft/world/item/ItemStackTemplate#<init>` | method | 3 |
| `net/minecraft/world/item/ItemStackTemplate#CODEC` | field | 2 |
| `net/minecraft/world/item/ItemStackTemplate#STREAM_CODEC` | field | 2 |
| `net/minecraft/world/item/ItemStackTemplate#components` | method | 1 |
| `net/minecraft/world/item/ItemUseAnimation#DRINK` | field | 1 |
| `net/minecraft/world/item/ItemUseAnimation#NONE` | field | 1 |
| `net/minecraft/world/item/Items#AIR` | field | 9 |
| `net/minecraft/world/item/Items#APPLE` | field | 1 |
| `net/minecraft/world/item/Items#ARROW` | field | 1 |
| `net/minecraft/world/item/Items#BEEF` | field | 1 |
| `net/minecraft/world/item/Items#BEETROOT` | field | 1 |
| `net/minecraft/world/item/Items#BLACKSTONE` | field | 1 |
| `net/minecraft/world/item/Items#BLAZE_POWDER` | field | 1 |
| `net/minecraft/world/item/Items#BLAZE_ROD` | field | 1 |
| `net/minecraft/world/item/Items#BONE` | field | 1 |
| `net/minecraft/world/item/Items#BOOK` | field | 1 |
| `net/minecraft/world/item/Items#BOW` | field | 1 |
| `net/minecraft/world/item/Items#BUCKET` | field | 2 |
| `net/minecraft/world/item/Items#CACTUS` | field | 1 |
| `net/minecraft/world/item/Items#CARROT` | field | 1 |
| `net/minecraft/world/item/Items#CARVED_PUMPKIN` | field | 1 |
| `net/minecraft/world/item/Items#CHISELED_SANDSTONE` | field | 1 |
| `net/minecraft/world/item/Items#COAL` | field | 1 |
| `net/minecraft/world/item/Items#COD` | field | 1 |
| `net/minecraft/world/item/Items#CRAFTING_TABLE` | field | 1 |
| `net/minecraft/world/item/Items#CREEPER_HEAD` | field | 1 |
| `net/minecraft/world/item/Items#DIAMOND` | field | 1 |
| `net/minecraft/world/item/Items#DRAGON_BREATH` | field | 1 |
| `net/minecraft/world/item/Items#DRAGON_HEAD` | field | 1 |
| `net/minecraft/world/item/Items#DROPPER` | field | 1 |
| `net/minecraft/world/item/Items#DYE` | field | 1 |
| `net/minecraft/world/item/Items#ECHO_SHARD` | field | 1 |
| `net/minecraft/world/item/Items#EGG` | field | 2 |
| `net/minecraft/world/item/Items#EMERALD` | field | 1 |
| `net/minecraft/world/item/Items#ENDER_EYE` | field | 1 |
| `net/minecraft/world/item/Items#ENDER_PEARL` | field | 1 |
| `net/minecraft/world/item/Items#END_STONE` | field | 1 |
| `net/minecraft/world/item/Items#FEATHER` | field | 1 |
| `net/minecraft/world/item/Items#FIRE_CHARGE` | field | 1 |
| `net/minecraft/world/item/Items#GHAST_TEAR` | field | 1 |
| `net/minecraft/world/item/Items#GLASS_BOTTLE` | field | 3 |
| `net/minecraft/world/item/Items#GLOW_INK_SAC` | field | 1 |
| `net/minecraft/world/item/Items#GOLDEN_APPLE` | field | 1 |
| `net/minecraft/world/item/Items#GOLDEN_CARROT` | field | 1 |
| `net/minecraft/world/item/Items#GOLD_INGOT` | field | 1 |
| `net/minecraft/world/item/Items#GUNPOWDER` | field | 1 |
| `net/minecraft/world/item/Items#HONEYCOMB` | field | 1 |
| `net/minecraft/world/item/Items#HONEY_BOTTLE` | field | 1 |
| `net/minecraft/world/item/Items#HOPPER` | field | 1 |
| `net/minecraft/world/item/Items#ICE` | field | 1 |
| `net/minecraft/world/item/Items#INK_SAC` | field | 1 |
| `net/minecraft/world/item/Items#IRON_INGOT` | field | 1 |
| `net/minecraft/world/item/Items#JUKEBOX` | field | 1 |
| `net/minecraft/world/item/Items#LANTERN` | field | 1 |
| `net/minecraft/world/item/Items#LAPIS_LAZULI` | field | 1 |
| `net/minecraft/world/item/Items#LAVA_BUCKET` | field | 1 |
| `net/minecraft/world/item/Items#LEATHER` | field | 1 |
| `net/minecraft/world/item/Items#LINGERING_POTION` | field | 3 |
| `net/minecraft/world/item/Items#MAGMA_CREAM` | field | 1 |
| `net/minecraft/world/item/Items#MELON_SEEDS` | field | 1 |
| `net/minecraft/world/item/Items#MELON_SLICE` | field | 2 |
| `net/minecraft/world/item/Items#MILK_BUCKET` | field | 2 |
| `net/minecraft/world/item/Items#MUTTON` | field | 1 |
| `net/minecraft/world/item/Items#NETHERITE_INGOT` | field | 1 |
| `net/minecraft/world/item/Items#NETHERITE_UPGRADE_SMITHING_TEMPLATE` | field | 1 |
| `net/minecraft/world/item/Items#NETHERRACK` | field | 1 |
| `net/minecraft/world/item/Items#NETHER_WART` | field | 1 |
| `net/minecraft/world/item/Items#PORKCHOP` | field | 1 |
| `net/minecraft/world/item/Items#POTATO` | field | 1 |
| `net/minecraft/world/item/Items#POTION` | field | 1 |
| `net/minecraft/world/item/Items#PUMPKIN` | field | 1 |
| `net/minecraft/world/item/Items#PUMPKIN_SEEDS` | field | 1 |
| `net/minecraft/world/item/Items#QUARTZ` | field | 1 |
| `net/minecraft/world/item/Items#QUARTZ_BLOCK` | field | 1 |
| `net/minecraft/world/item/Items#REDSTONE` | field | 1 |
| `net/minecraft/world/item/Items#SAND` | field | 1 |
| `net/minecraft/world/item/Items#SANDSTONE` | field | 1 |
| `net/minecraft/world/item/Items#SHEARS` | field | 1 |
| `net/minecraft/world/item/Items#SMITHING_TABLE` | field | 1 |
| `net/minecraft/world/item/Items#SNOWBALL` | field | 1 |
| `net/minecraft/world/item/Items#SNOW_BLOCK` | field | 1 |
| `net/minecraft/world/item/Items#SOUL_SAND` | field | 1 |
| `net/minecraft/world/item/Items#SPIDER_EYE` | field | 1 |
| `net/minecraft/world/item/Items#SPLASH_POTION` | field | 3 |
| `net/minecraft/world/item/Items#SPONGE` | field | 1 |
| `net/minecraft/world/item/Items#SPYGLASS` | field | 1 |
| `net/minecraft/world/item/Items#STRING` | field | 1 |
| `net/minecraft/world/item/Items#SUGAR` | field | 1 |
| `net/minecraft/world/item/Items#SWEET_BERRIES` | field | 1 |
| `net/minecraft/world/item/Items#TNT` | field | 1 |
| `net/minecraft/world/item/Items#TRIDENT` | field | 1 |
| `net/minecraft/world/item/Items#WARD_ARMOR_TRIM_SMITHING_TEMPLATE` | field | 1 |
| `net/minecraft/world/item/Items#WATER_BUCKET` | field | 1 |
| `net/minecraft/world/item/Items#WHEAT` | field | 1 |
| `net/minecraft/world/item/Items#WITHER_SKELETON_SKULL` | field | 1 |
| `net/minecraft/world/item/Items#WOOL` | field | 1 |
| `net/minecraft/world/item/JukeboxSong#description` | method | 1 |
| `net/minecraft/world/item/JukeboxSong#fromStack` | method | 1 |
| `net/minecraft/world/item/JukeboxSong#soundEvent` | method | 1 |
| `net/minecraft/world/item/TooltipFlag$Default#ADVANCED` | field | 1 |
| `net/minecraft/world/item/TooltipFlag$Default#NORMAL` | field | 2 |
| `net/minecraft/world/item/alchemy/Potion#name` | method | 1 |
| `net/minecraft/world/item/alchemy/PotionContents#EMPTY` | field | 2 |
| `net/minecraft/world/item/alchemy/PotionContents#addPotionTooltip` | method | 1 |
| `net/minecraft/world/item/alchemy/PotionContents#customName` | method | 1 |
| `net/minecraft/world/item/alchemy/PotionContents#getAllEffects` | method | 2 |
| `net/minecraft/world/item/alchemy/PotionContents#getColor` | method | 1 |
| `net/minecraft/world/item/alchemy/PotionContents#potion` | method | 1 |
| `net/minecraft/world/item/component/Consumable#onConsumeEffects` | method | 1 |
| `net/minecraft/world/item/component/CookingFuel#burnTime` | method | 1 |
| `net/minecraft/world/item/component/CookingFuel#speedMultiplier` | method | 1 |
| `net/minecraft/world/item/component/CustomData#EMPTY` | field | 1 |
| `net/minecraft/world/item/component/CustomData#copyTag` | method | 2 |
| `net/minecraft/world/item/component/CustomData#of` | method | 1 |
| `net/minecraft/world/item/component/DyedItemColor#<init>` | method | 3 |
| `net/minecraft/world/item/component/DyedItemColor#rgb` | method | 3 |
| `net/minecraft/world/item/component/ItemAttributeModifiers#ATTRIBUTE_MODIFIER_FORMAT` | field | 1 |
| `net/minecraft/world/item/component/ItemContainerContents#CODEC` | field | 1 |
| `net/minecraft/world/item/component/ItemContainerContents#EMPTY` | field | 15 |
| `net/minecraft/world/item/component/ItemContainerContents#STREAM_CODEC` | field | 1 |
| `net/minecraft/world/item/component/ItemContainerContents#copyInto` | method | 1 |
| `net/minecraft/world/item/component/ItemContainerContents#fromItems` | method | 10 |
| `net/minecraft/world/item/component/ItemContainerContents#itemCopies` | method | 1 |
| `net/minecraft/world/item/component/ItemContainerContents#nonEmptyItemCopyStream` | method | 3 |
| `net/minecraft/world/item/component/TooltipDisplay#DEFAULT` | field | 1 |
| `net/minecraft/world/item/consume_effects/ApplyStatusEffectsConsumeEffect#effects` | method | 1 |
| `net/minecraft/world/item/consume_effects/ConsumeEffect#getType` | method | 1 |
| `net/minecraft/world/item/consume_effects/ConsumeEffect$Type#APPLY_EFFECTS` | field | 1 |
| `net/minecraft/world/item/context/BlockPlaceContext#<init>` | method | 2 |
| `net/minecraft/world/item/context/BlockPlaceContext#canPlace` | method | 1 |
| `net/minecraft/world/item/context/BlockPlaceContext#getClickedPos` | method | 2 |
| `net/minecraft/world/item/context/BlockPlaceContext#getHand` | method | 1 |
| `net/minecraft/world/item/context/BlockPlaceContext#getHorizontalDirection` | method | 2 |
| `net/minecraft/world/item/context/BlockPlaceContext#getItemInHand` | method | 1 |
| `net/minecraft/world/item/context/BlockPlaceContext#getLevel` | method | 2 |
| `net/minecraft/world/item/context/BlockPlaceContext#getPlayer` | method | 1 |
| `net/minecraft/world/item/context/UseOnContext#<init>` | method | 1 |
| `net/minecraft/world/item/context/UseOnContext#getHand` | method | 1 |
| `net/minecraft/world/item/context/UseOnContext#getLevel` | method | 1 |
| `net/minecraft/world/item/context/UseOnContext#getPlayer` | method | 1 |
| `net/minecraft/world/item/crafting/AbstractCookingRecipe#assemble` | method | 1 |
| `net/minecraft/world/item/crafting/AbstractCookingRecipe#cookingTime` | method | 1 |
| `net/minecraft/world/item/crafting/CraftingInput#getItem` | method | 1 |
| `net/minecraft/world/item/crafting/CraftingInput#height` | method | 1 |
| `net/minecraft/world/item/crafting/CraftingInput#items` | method | 1 |
| `net/minecraft/world/item/crafting/CraftingInput#size` | method | 1 |
| `net/minecraft/world/item/crafting/CraftingInput#stackedContents` | method | 1 |
| `net/minecraft/world/item/crafting/CraftingInput#width` | method | 1 |
| `net/minecraft/world/item/crafting/CraftingInput$Positioned#input` | method | 1 |
| `net/minecraft/world/item/crafting/CraftingInput$Positioned#left` | method | 1 |
| `net/minecraft/world/item/crafting/CraftingInput$Positioned#top` | method | 1 |
| `net/minecraft/world/item/crafting/CraftingRecipe#assemble` | method | 1 |
| `net/minecraft/world/item/crafting/CraftingRecipe#getRemainingItems` | method | 1 |
| `net/minecraft/world/item/crafting/CraftingRecipe#isSpecial` | method | 1 |
| `net/minecraft/world/item/crafting/CraftingRecipe#matches` | method | 2 |
| `net/minecraft/world/item/crafting/CraftingRecipe$CraftingBookInfo#MAP_CODEC` | field | 1 |
| `net/minecraft/world/item/crafting/CraftingRecipe$CraftingBookInfo#STREAM_CODEC` | field | 1 |
| `net/minecraft/world/item/crafting/DyeRecipe#<init>` | method | 1 |
| `net/minecraft/world/item/crafting/Ingredient#CODEC` | field | 1 |
| `net/minecraft/world/item/crafting/Ingredient#CONTENTS_STREAM_CODEC` | field | 1 |
| `net/minecraft/world/item/crafting/Ingredient#OPTIONAL_CONTENTS_STREAM_CODEC` | field | 1 |
| `net/minecraft/world/item/crafting/Ingredient#display` | method | 3 |
| `net/minecraft/world/item/crafting/Ingredient#of` | method | 5 |
| `net/minecraft/world/item/crafting/Ingredient#optionalIngredientToDisplay` | method | 1 |
| `net/minecraft/world/item/crafting/NormalCraftingRecipe#<init>` | method | 1 |
| `net/minecraft/world/item/crafting/PlacementInfo#createFromOptionals` | method | 2 |
| `net/minecraft/world/item/crafting/Recipe#isSpecial` | method | 1 |
| `net/minecraft/world/item/crafting/Recipe$CommonInfo#<init>` | method | 1 |
| `net/minecraft/world/item/crafting/Recipe$CommonInfo#MAP_CODEC` | field | 2 |
| `net/minecraft/world/item/crafting/Recipe$CommonInfo#STREAM_CODEC` | field | 2 |
| `net/minecraft/world/item/crafting/RecipeHolder#id` | method | 1 |
| `net/minecraft/world/item/crafting/RecipeHolder#value` | method | 3 |
| `net/minecraft/world/item/crafting/RecipeManager#createCheck` | method | 1 |
| `net/minecraft/world/item/crafting/RecipeManager#getRecipeFor` | method | 1 |
| `net/minecraft/world/item/crafting/RecipeManager$CachedCheck#getRecipeFor` | method | 1 |
| `net/minecraft/world/item/crafting/RecipeSerializer#<init>` | method | 2 |
| `net/minecraft/world/item/crafting/RecipeType#BLASTING` | field | 1 |
| `net/minecraft/world/item/crafting/RecipeType#CRAFTING` | field | 2 |
| `net/minecraft/world/item/crafting/RecipeType#SMELTING` | field | 1 |
| `net/minecraft/world/item/crafting/RecipeType#SMOKING` | field | 1 |
| `net/minecraft/world/item/crafting/ShapedRecipePattern#MAP_CODEC` | field | 1 |
| `net/minecraft/world/item/crafting/ShapedRecipePattern#STREAM_CODEC` | field | 1 |
| `net/minecraft/world/item/crafting/ShapedRecipePattern#height` | method | 1 |
| `net/minecraft/world/item/crafting/ShapedRecipePattern#ingredients` | method | 1 |
| `net/minecraft/world/item/crafting/ShapedRecipePattern#matches` | method | 1 |
| `net/minecraft/world/item/crafting/ShapedRecipePattern#of` | method | 1 |
| `net/minecraft/world/item/crafting/ShapedRecipePattern#width` | method | 1 |
| `net/minecraft/world/item/crafting/SimpleSmithingRecipe#<init>` | method | 1 |
| `net/minecraft/world/item/crafting/SingleRecipeInput#<init>` | method | 1 |
| `net/minecraft/world/item/crafting/SmithingRecipeInput#<init>` | method | 1 |
| `net/minecraft/world/item/crafting/SmithingRecipeInput#base` | method | 1 |
| `net/minecraft/world/item/crafting/SmithingRecipeInput#getItem` | method | 1 |
| `net/minecraft/world/item/crafting/TransmuteRecipe#createWithOriginalComponents` | method | 1 |
| `net/minecraft/world/item/crafting/display/ShapedCraftingRecipeDisplay#<init>` | method | 1 |
| `net/minecraft/world/item/crafting/display/SlotDisplay#resolveForFirstStack` | method | 1 |
| `net/minecraft/world/item/crafting/display/SlotDisplay#resolveForStacks` | method | 1 |
| `net/minecraft/world/item/crafting/display/SlotDisplay$Empty#INSTANCE` | field | 1 |
| `net/minecraft/world/item/crafting/display/SlotDisplay$ItemSlotDisplay#<init>` | method | 2 |
| `net/minecraft/world/item/crafting/display/SlotDisplay$ItemStackSlotDisplay#<init>` | method | 2 |
| `net/minecraft/world/item/crafting/display/SlotDisplayContext#fromLevel` | method | 1 |
| `net/minecraft/world/item/crafting/display/SmithingRecipeDisplay#<init>` | method | 1 |
| `net/minecraft/world/item/enchantment/Enchantment#getFullname` | method | 1 |
| `net/minecraft/world/item/enchantment/EnchantmentHelper#doPostAttackEffects` | method | 1 |
| `net/minecraft/world/item/enchantment/ItemEnchantments#entrySet` | method | 1 |
| `net/minecraft/world/level/BlockGetter#getBlockEntity` | method | 1 |
| `net/minecraft/world/level/ClipContext$Fluid#NONE` | field | 1 |
| `net/minecraft/world/level/ClipContext$Fluid#SOURCE_ONLY` | field | 1 |
| `net/minecraft/world/level/GameType#SPECTATOR` | field | 2 |
| `net/minecraft/world/level/ItemLike#asItem` | method | 1 |
| `net/minecraft/world/level/Level#addFreshEntity` | method | 2 |
| `net/minecraft/world/level/Level#addParticle` | method | 3 |
| `net/minecraft/world/level/Level#dimension` | method | 1 |
| `net/minecraft/world/level/Level#enabledFeatures` | method | 2 |
| `net/minecraft/world/level/Level#environmentAttributes` | method | 2 |
| `net/minecraft/world/level/Level#explode` | method | 1 |
| `net/minecraft/world/level/Level#gameEvent` | method | 2 |
| `net/minecraft/world/level/Level#getBiome` | method | 1 |
| `net/minecraft/world/level/Level#getBlockEntity` | method | 9 |
| `net/minecraft/world/level/Level#getBlockState` | method | 13 |
| `net/minecraft/world/level/Level#getEntities` | method | 4 |
| `net/minecraft/world/level/Level#getEntitiesOfClass` | method | 1 |
| `net/minecraft/world/level/Level#getEntity` | method | 3 |
| `net/minecraft/world/level/Level#getFluidState` | method | 1 |
| `net/minecraft/world/level/Level#getGameTime` | method | 5 |
| `net/minecraft/world/level/Level#getRandom` | method | 10 |
| `net/minecraft/world/level/Level#getWorldBorder` | method | 1 |
| `net/minecraft/world/level/Level#isEmptyBlock` | method | 1 |
| `net/minecraft/world/level/Level#isRaining` | method | 1 |
| `net/minecraft/world/level/Level#levelEvent` | method | 2 |
| `net/minecraft/world/level/Level#mayInteract` | method | 1 |
| `net/minecraft/world/level/Level#playLocalSound` | method | 1 |
| `net/minecraft/world/level/Level#playSound` | method | 15 |
| `net/minecraft/world/level/Level#players` | method | 1 |
| `net/minecraft/world/level/Level#registryAccess` | method | 1 |
| `net/minecraft/world/level/Level#sendBlockUpdated` | method | 1 |
| `net/minecraft/world/level/Level#setBlockAndUpdate` | method | 4 |
| `net/minecraft/world/level/Level$ExplosionInteraction#NONE` | field | 1 |
| `net/minecraft/world/level/LevelAccessor#gameEvent` | method | 1 |
| `net/minecraft/world/level/LevelAccessor#getBlockEntity` | method | 1 |
| `net/minecraft/world/level/LevelAccessor#playSound` | method | 1 |
| `net/minecraft/world/level/LevelReader#getBlockEntity` | method | 1 |
| `net/minecraft/world/level/ServerLevelAccessor#getRandom` | method | 1 |
| `net/minecraft/world/level/biome/Biome#getBaseTemperature` | method | 1 |
| `net/minecraft/world/level/block/AbstractBedBlock#defaultBlockState` | method | 1 |
| `net/minecraft/world/level/block/BedBlock#<init>` | method | 1 |
| `net/minecraft/world/level/block/BedBlock#getDrops` | method | 1 |
| `net/minecraft/world/level/block/BedBlock#playerWillDestroy` | method | 1 |
| `net/minecraft/world/level/block/BedBlock#setPlacedBy` | method | 1 |
| `net/minecraft/world/level/block/BedBlock#updateShape` | method | 1 |
| `net/minecraft/world/level/block/Block#animateTick` | method | 1 |
| `net/minecraft/world/level/block/Block#asItem` | method | 3 |
| `net/minecraft/world/level/block/Block#box` | method | 2 |
| `net/minecraft/world/level/block/Block#byItem` | method | 3 |
| `net/minecraft/world/level/block/Block#defaultBlockState` | method | 12 |
| `net/minecraft/world/level/block/Block#dropResources` | method | 1 |
| `net/minecraft/world/level/block/Block#entityInside` | method | 1 |
| `net/minecraft/world/level/block/Block#getDescriptionId` | method | 1 |
| `net/minecraft/world/level/block/Block#getExplosionResistance` | method | 1 |
| `net/minecraft/world/level/block/Block#getId` | method | 1 |
| `net/minecraft/world/level/block/Block#getSignal` | method | 1 |
| `net/minecraft/world/level/block/Block#neighborChanged` | method | 1 |
| `net/minecraft/world/level/block/Block#onPlace` | method | 1 |
| `net/minecraft/world/level/block/Block#playerWillDestroy` | method | 1 |
| `net/minecraft/world/level/block/Blocks#AIR` | field | 8 |
| `net/minecraft/world/level/block/Blocks#BLAST_FURNACE` | field | 2 |
| `net/minecraft/world/level/block/Blocks#ENCHANTING_TABLE` | field | 1 |
| `net/minecraft/world/level/block/Blocks#FURNACE` | field | 2 |
| `net/minecraft/world/level/block/Blocks#KELP` | field | 1 |
| `net/minecraft/world/level/block/Blocks#KELP_PLANT` | field | 1 |
| `net/minecraft/world/level/block/Blocks#SEAGRASS` | field | 1 |
| `net/minecraft/world/level/block/Blocks#SMOKER` | field | 2 |
| `net/minecraft/world/level/block/Blocks#TALL_SEAGRASS` | field | 1 |
| `net/minecraft/world/level/block/Blocks#WATER` | field | 1 |
| `net/minecraft/world/level/block/Blocks#WET_SPONGE` | field | 1 |
| `net/minecraft/world/level/block/BucketPickup#getPickupSound` | method | 1 |
| `net/minecraft/world/level/block/ColorCollection#pick` | method | 1 |
| `net/minecraft/world/level/block/CropBlock#AGE` | field | 1 |
| `net/minecraft/world/level/block/DispenserBlock#registerBehavior` | method | 1 |
| `net/minecraft/world/level/block/RenderShape#MODEL` | field | 2 |
| `net/minecraft/world/level/block/SoundType#BONE_BLOCK` | field | 1 |
| `net/minecraft/world/level/block/SoundType#GLASS` | field | 1 |
| `net/minecraft/world/level/block/SoundType#GRASS` | field | 1 |
| `net/minecraft/world/level/block/SoundType#METAL` | field | 1 |
| `net/minecraft/world/level/block/SoundType#NETHERITE_BLOCK` | field | 1 |
| `net/minecraft/world/level/block/SoundType#NETHER_BRICKS` | field | 1 |
| `net/minecraft/world/level/block/SoundType#SCULK_SHRIEKER` | field | 1 |
| `net/minecraft/world/level/block/SoundType#SLIME_BLOCK` | field | 1 |
| `net/minecraft/world/level/block/SoundType#SNOW` | field | 1 |
| `net/minecraft/world/level/block/SoundType#STONE` | field | 1 |
| `net/minecraft/world/level/block/SoundType#WOOD` | field | 1 |
| `net/minecraft/world/level/block/SoundType#WOOL` | field | 2 |
| `net/minecraft/world/level/block/SoundType#getPitch` | method | 1 |
| `net/minecraft/world/level/block/SoundType#getPlaceSound` | method | 1 |
| `net/minecraft/world/level/block/SoundType#getVolume` | method | 1 |
| `net/minecraft/world/level/block/entity/BlockEntity#<init>` | method | 1 |
| `net/minecraft/world/level/block/entity/BlockEntity#collectImplicitComponents` | method | 1 |
| `net/minecraft/world/level/block/entity/BlockEntity#loadAdditional` | method | 1 |
| `net/minecraft/world/level/block/entity/BlockEntity#preRemoveSideEffects` | method | 1 |
| `net/minecraft/world/level/block/entity/BlockEntity#saveAdditional` | method | 1 |
| `net/minecraft/world/level/block/entity/BlockEntity#setLevel` | method | 1 |
| `net/minecraft/world/level/block/entity/ShulkerBoxBlockEntity#canPlaceItemThroughFace` | method | 1 |
| `net/minecraft/world/level/block/state/BlockBehaviour$Properties#bounceRestitution` | method | 1 |
| `net/minecraft/world/level/block/state/BlockBehaviour$Properties#fallDistanceReduction` | method | 1 |
| `net/minecraft/world/level/block/state/BlockBehaviour$Properties#forceSolidOn` | method | 2 |
| `net/minecraft/world/level/block/state/BlockBehaviour$Properties#ignitedByLava` | method | 1 |
| `net/minecraft/world/level/block/state/BlockBehaviour$Properties#isRedstoneConductor` | method | 1 |
| `net/minecraft/world/level/block/state/BlockBehaviour$Properties#lightLevel` | method | 1 |
| `net/minecraft/world/level/block/state/BlockBehaviour$Properties#mapColor` | method | 2 |
| `net/minecraft/world/level/block/state/BlockBehaviour$Properties#noOcclusion` | method | 1 |
| `net/minecraft/world/level/block/state/BlockBehaviour$Properties#of` | method | 1 |
| `net/minecraft/world/level/block/state/BlockBehaviour$Properties#pushReaction` | method | 2 |
| `net/minecraft/world/level/block/state/BlockBehaviour$Properties#setId` | method | 1 |
| `net/minecraft/world/level/block/state/BlockBehaviour$Properties#sound` | method | 2 |
| `net/minecraft/world/level/block/state/BlockBehaviour$Properties#strength` | method | 2 |
| `net/minecraft/world/level/block/state/BlockState#entityCanStandOn` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#getBlock` | method | 13 |
| `net/minecraft/world/level/block/state/BlockState#getCloneItemStack` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#getFluidState` | method | 3 |
| `net/minecraft/world/level/block/state/BlockState#getSoundType` | method | 2 |
| `net/minecraft/world/level/block/state/BlockState#getValue` | method | 7 |
| `net/minecraft/world/level/block/state/BlockState#hasBlockEntity` | method | 2 |
| `net/minecraft/world/level/block/state/BlockState#hasProperty` | method | 3 |
| `net/minecraft/world/level/block/state/BlockState#is` | method | 5 |
| `net/minecraft/world/level/block/state/BlockState#isAir` | method | 4 |
| `net/minecraft/world/level/block/state/BlockState#liquid` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#setValue` | method | 4 |
| `net/minecraft/world/level/block/state/BlockState#updateNeighbourShapes` | method | 1 |
| `net/minecraft/world/level/block/state/StateDefinition#any` | method | 2 |
| `net/minecraft/world/level/block/state/StateDefinition$Builder#add` | method | 3 |
| `net/minecraft/world/level/block/state/properties/BedPart#FOOT` | field | 4 |
| `net/minecraft/world/level/block/state/properties/BedPart#HEAD` | field | 5 |
| `net/minecraft/world/level/block/state/properties/BedPart#ordinal` | method | 2 |
| `net/minecraft/world/level/block/state/properties/BedPart#values` | method | 1 |
| `net/minecraft/world/level/block/state/properties/BlockStateProperties#BED_PART` | field | 1 |
| `net/minecraft/world/level/block/state/properties/BlockStateProperties#CONDITIONAL` | field | 1 |
| `net/minecraft/world/level/block/state/properties/BlockStateProperties#HORIZONTAL_FACING` | field | 1 |
| `net/minecraft/world/level/block/state/properties/BlockStateProperties#OCCUPIED` | field | 1 |
| `net/minecraft/world/level/border/WorldBorder#isWithinBounds` | method | 1 |
| `net/minecraft/world/level/gameevent/GameEvent#BLOCK_PLACE` | field | 1 |
| `net/minecraft/world/level/gameevent/GameEvent#FLUID_PICKUP` | field | 1 |
| `net/minecraft/world/level/gameevent/GameEvent#FLUID_PLACE` | field | 1 |
| `net/minecraft/world/level/gameevent/GameEvent#SHEAR` | field | 1 |
| `net/minecraft/world/level/gamerules/GameRules#KEEP_INVENTORY` | field | 1 |
| `net/minecraft/world/level/gamerules/GameRules#get` | method | 1 |
| `net/minecraft/world/level/material/FlowingFluid#<init>` | method | 1 |
| `net/minecraft/world/level/material/FlowingFluid#getSource` | method | 1 |
| `net/minecraft/world/level/material/Fluid#defaultFluidState` | method | 3 |
| `net/minecraft/world/level/material/Fluid#getBucket` | method | 1 |
| `net/minecraft/world/level/material/Fluid#is` | method | 2 |
| `net/minecraft/world/level/material/Fluid#isSame` | method | 4 |
| `net/minecraft/world/level/material/FluidState#createLegacyBlock` | method | 1 |
| `net/minecraft/world/level/material/FluidState#getType` | method | 1 |
| `net/minecraft/world/level/material/FluidState#is` | method | 3 |
| `net/minecraft/world/level/material/FluidState#isSource` | method | 1 |
| `net/minecraft/world/level/material/Fluids#EMPTY` | field | 1 |
| `net/minecraft/world/level/material/Fluids#LAVA` | field | 2 |
| `net/minecraft/world/level/material/Fluids#WATER` | field | 10 |
| `net/minecraft/world/level/material/MapColor#COLOR_BLACK` | field | 1 |
| `net/minecraft/world/level/material/MapColor#COLOR_BROWN` | field | 1 |
| `net/minecraft/world/level/material/MapColor#COLOR_GRAY` | field | 1 |
| `net/minecraft/world/level/material/MapColor#COLOR_GREEN` | field | 1 |
| `net/minecraft/world/level/material/MapColor#COLOR_LIGHT_GRAY` | field | 1 |
| `net/minecraft/world/level/material/MapColor#COLOR_LIGHT_GREEN` | field | 1 |
| `net/minecraft/world/level/material/MapColor#COLOR_ORANGE` | field | 1 |
| `net/minecraft/world/level/material/MapColor#COLOR_PINK` | field | 1 |
| `net/minecraft/world/level/material/MapColor#COLOR_YELLOW` | field | 1 |
| `net/minecraft/world/level/material/MapColor#DIAMOND` | field | 1 |
| `net/minecraft/world/level/material/MapColor#EMERALD` | field | 1 |
| `net/minecraft/world/level/material/MapColor#FIRE` | field | 1 |
| `net/minecraft/world/level/material/MapColor#GOLD` | field | 1 |
| `net/minecraft/world/level/material/MapColor#LAPIS` | field | 1 |
| `net/minecraft/world/level/material/MapColor#METAL` | field | 1 |
| `net/minecraft/world/level/material/MapColor#NETHER` | field | 1 |
| `net/minecraft/world/level/material/MapColor#NONE` | field | 1 |
| `net/minecraft/world/level/material/MapColor#PLANT` | field | 1 |
| `net/minecraft/world/level/material/MapColor#QUARTZ` | field | 1 |
| `net/minecraft/world/level/material/MapColor#SAND` | field | 1 |
| `net/minecraft/world/level/material/MapColor#SNOW` | field | 1 |
| `net/minecraft/world/level/material/MapColor#TERRACOTTA_BLACK` | field | 1 |
| `net/minecraft/world/level/material/MapColor#TERRACOTTA_BLUE` | field | 1 |
| `net/minecraft/world/level/material/MapColor#TERRACOTTA_ORANGE` | field | 1 |
| `net/minecraft/world/level/material/MapColor#TERRACOTTA_WHITE` | field | 1 |
| `net/minecraft/world/level/material/MapColor#WOOL` | field | 2 |
| `net/minecraft/world/level/material/PushReaction#POPPED` | field | 2 |
| `net/minecraft/world/level/storage/LevelData$RespawnData#of` | method | 1 |
| `net/minecraft/world/level/storage/LevelResource#<init>` | method | 1 |
| `net/minecraft/world/level/storage/ValueInput#child` | method | 1 |
| `net/minecraft/world/level/storage/ValueInput#getBooleanOr` | method | 1 |
| `net/minecraft/world/level/storage/ValueInput#getIntOr` | method | 2 |
| `net/minecraft/world/level/storage/ValueInput#listOrEmpty` | method | 1 |
| `net/minecraft/world/level/storage/ValueInput#read` | method | 2 |
| `net/minecraft/world/level/storage/ValueInput$TypedInputList#forEach` | method | 1 |
| `net/minecraft/world/level/storage/ValueOutput#list` | method | 1 |
| `net/minecraft/world/level/storage/ValueOutput#putBoolean` | method | 1 |
| `net/minecraft/world/level/storage/ValueOutput#putInt` | method | 1 |
| `net/minecraft/world/level/storage/ValueOutput#store` | method | 1 |
| `net/minecraft/world/level/storage/ValueOutput$TypedOutputList#add` | method | 1 |
| `net/minecraft/world/level/storage/loot/BuiltInLootTables#ABANDONED_CAMP_COMMON_CHEST` | field | 1 |
| `net/minecraft/world/level/storage/loot/BuiltInLootTables#ABANDONED_CAMP_SECRET_CHEST` | field | 1 |
| `net/minecraft/world/level/storage/loot/BuiltInLootTables#ABANDONED_MINESHAFT` | field | 1 |
| `net/minecraft/world/level/storage/loot/BuiltInLootTables#BASTION_TREASURE` | field | 1 |
| `net/minecraft/world/level/storage/loot/BuiltInLootTables#DESERT_PYRAMID` | field | 1 |
| `net/minecraft/world/level/storage/loot/BuiltInLootTables#END_CITY_TREASURE` | field | 1 |
| `net/minecraft/world/level/storage/loot/BuiltInLootTables#NETHER_BRIDGE` | field | 1 |
| `net/minecraft/world/level/storage/loot/BuiltInLootTables#PILLAGER_OUTPOST` | field | 1 |
| `net/minecraft/world/level/storage/loot/BuiltInLootTables#SHIPWRECK_TREASURE` | field | 1 |
| `net/minecraft/world/level/storage/loot/BuiltInLootTables#SIMPLE_DUNGEON` | field | 1 |
| `net/minecraft/world/level/storage/loot/BuiltInLootTables#WOODLAND_MANSION` | field | 1 |
| `net/minecraft/world/level/storage/loot/LootContext#getOptional` | method | 1 |
| `net/minecraft/world/level/storage/loot/LootContext#getRandom` | method | 1 |
| `net/minecraft/world/level/storage/loot/LootContext$Builder#<init>` | method | 1 |
| `net/minecraft/world/level/storage/loot/LootContext$Builder#create` | method | 1 |
| `net/minecraft/world/level/storage/loot/LootParams$Builder#<init>` | method | 1 |
| `net/minecraft/world/level/storage/loot/LootParams$Builder#create` | method | 1 |
| `net/minecraft/world/level/storage/loot/LootParams$Builder#withParameter` | method | 1 |
| `net/minecraft/world/level/storage/loot/LootPool#lootPool` | method | 2 |
| `net/minecraft/world/level/storage/loot/LootPool$Builder#add` | method | 3 |
| `net/minecraft/world/level/storage/loot/LootPool$Builder#build` | method | 1 |
| `net/minecraft/world/level/storage/loot/LootPool$Builder#setRolls` | method | 2 |
| `net/minecraft/world/level/storage/loot/LootPool$Builder#when` | method | 2 |
| `net/minecraft/world/level/storage/loot/LootTable#getRandomItems` | method | 1 |
| `net/minecraft/world/level/storage/loot/LootTable#lootTable` | method | 1 |
| `net/minecraft/world/level/storage/loot/LootTable$Builder#pool` | method | 1 |
| `net/minecraft/world/level/storage/loot/LootTable$Builder#withPool` | method | 1 |
| `net/minecraft/world/level/storage/loot/entries/LootItem#lootTableItem` | method | 2 |
| `net/minecraft/world/level/storage/loot/entries/UniformContainerBase$Builder#apply` | method | 2 |
| `net/minecraft/world/level/storage/loot/entries/UniformContainerBase$Builder#build` | method | 1 |
| `net/minecraft/world/level/storage/loot/entries/UniformContainerBase$Builder#setWeight` | method | 1 |
| `net/minecraft/world/level/storage/loot/entries/UniformContainerBase$Builder#when` | method | 1 |
| `net/minecraft/world/level/storage/loot/functions/CopyComponentsFunction#copyComponentsFromBlockEntity` | method | 1 |
| `net/minecraft/world/level/storage/loot/functions/CopyComponentsFunction$Builder#include` | method | 1 |
| `net/minecraft/world/level/storage/loot/functions/SetComponentsFunction#setComponent` | method | 1 |
| `net/minecraft/world/level/storage/loot/parameters/LootContextParamSets#BLOCK_INTERACT` | field | 1 |
| `net/minecraft/world/level/storage/loot/parameters/LootContextParams#BLOCK_ENTITY` | field | 1 |
| `net/minecraft/world/level/storage/loot/parameters/LootContextParams#BLOCK_STATE` | field | 2 |
| `net/minecraft/world/level/storage/loot/parameters/LootContextParams#ORIGIN` | field | 1 |
| `net/minecraft/world/level/storage/loot/parameters/LootContextParams#THIS_ENTITY` | field | 1 |
| `net/minecraft/world/level/storage/loot/predicates/LootItemCondition#CODEC` | field | 1 |
| `net/minecraft/world/level/storage/loot/predicates/LootItemCondition$Builder#build` | method | 1 |
| `net/minecraft/world/level/storage/loot/predicates/LootItemRandomChanceCondition#randomChance` | method | 1 |
| `net/minecraft/world/level/storage/loot/predicates/MatchBlock#blockMatches` | method | 1 |
| `net/minecraft/world/level/storage/loot/providers/number/floats/ResolvableFloat#getFromItem` | method | 1 |
| `net/minecraft/world/level/storage/loot/providers/number/ints/ContextIntProviders#exactly` | method | 2 |
| `net/minecraft/world/level/storage/loot/providers/number/ints/ResolvableInt#getFromItem` | method | 1 |
| `net/minecraft/world/phys/AABB#<init>` | method | 6 |
| `net/minecraft/world/phys/AABB#inflate` | method | 4 |
| `net/minecraft/world/phys/AABB#maxX` | field | 1 |
| `net/minecraft/world/phys/AABB#maxY` | field | 1 |
| `net/minecraft/world/phys/AABB#maxZ` | field | 1 |
| `net/minecraft/world/phys/AABB#minX` | field | 1 |
| `net/minecraft/world/phys/AABB#minY` | field | 2 |
| `net/minecraft/world/phys/AABB#minZ` | field | 1 |
| `net/minecraft/world/phys/BlockHitResult#getBlockPos` | method | 2 |
| `net/minecraft/world/phys/BlockHitResult#getDirection` | method | 1 |
| `net/minecraft/world/phys/BlockHitResult#getType` | method | 1 |
| `net/minecraft/world/phys/EntityHitResult#getEntity` | method | 1 |
| `net/minecraft/world/phys/HitResult$Type#BLOCK` | field | 1 |
| `net/minecraft/world/phys/Vec2#<init>` | method | 1 |
| `net/minecraft/world/phys/Vec2#x` | field | 1 |
| `net/minecraft/world/phys/Vec2#y` | field | 1 |
| `net/minecraft/world/phys/Vec3#ZERO` | field | 2 |
| `net/minecraft/world/phys/Vec3#add` | method | 1 |
| `net/minecraft/world/phys/Vec3#atBottomCenterOf` | method | 1 |
| `net/minecraft/world/phys/Vec3#atCenterOf` | method | 2 |
| `net/minecraft/world/phys/Vec3#relative` | method | 1 |
| `net/minecraft/world/phys/Vec3#x` | field | 5 |
| `net/minecraft/world/phys/Vec3#y` | method | 4 |
| `net/minecraft/world/phys/Vec3#z` | field | 5 |
| `net/minecraft/world/phys/shapes/BooleanOp#OR` | field | 2 |
| `net/minecraft/world/phys/shapes/Shapes#join` | method | 2 |

</details>

