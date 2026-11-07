# Horizon port report: ItemPhysic

`itemphysic` 1.8.18 · ItemPhysic_FABRIC_v1.8.18_mc26.3.jar · 27 classes

**Partly convertible**

| Status | Count | Meaning |
|---|---|---|
| auto | 1 | The Horizon hook or injection is known: the wiring can be generated. The Java body still has to become Rust. |
| content | 1 | Data and assets: horizon-port-pack carries these into a content mod (.hmod). |
| needs-mapping | 22 | The kind of change carries over, but the matching Horizon function isn't in the table yet. Run `match`. |
| needs-translation | 2 | No direct equivalent: a person (or an AI agent) has to rewrite this part. |
| unsupported | 0 | Horizon has nothing this can attach to yet. |

## Entry points

- `main`: team.creative.itemphysic.ItemPhysic
- `client`: team.creative.itemphysic.ItemPhysic

## Mixins

| Injection | In | Horizon | Status | Note |
|---|---|---|---|---|
| @Accessor on Entity.stuckSpeedMultiplier | `EntityAccessor.getStuckSpeedMultiplier` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for Entity.stuckSpeedMultiplier: run `match`. |
| @Invoker on Entity.callGetBlockPosBelowThatAffectsMyMovement | `EntityAccessor.callGetBlockPosBelowThatAffectsMyMovement` | INVOKE | needs-mapping | A [[bridge]] INVOKE of the Rust method. No Horizon function known for Entity.callGetBlockPosBelowThatAffectsMyMovement: run `match`. |
| @Invoker on Entity.callIsInvulnerableToBase | `EntityAccessor.callIsInvulnerableToBase` | INVOKE | needs-mapping | A [[bridge]] INVOKE of the Rust method. No Horizon function known for Entity.callIsInvulnerableToBase: run `match`. |
| @Accessor on ItemEntity.target | `ItemEntityAccessor.getTarget` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for ItemEntity.target: run `match`. |
| @Invoker on ItemEntity.callSetUnderwaterMovement | `ItemEntityAccessor.callSetUnderwaterMovement` | INVOKE | needs-mapping | A [[bridge]] INVOKE of the Rust method. No Horizon function known for ItemEntity.callSetUnderwaterMovement: run `match`. |
| @Invoker on ItemEntity.callSetUnderLavaMovement | `ItemEntityAccessor.callSetUnderLavaMovement` | INVOKE | needs-mapping | A [[bridge]] INVOKE of the Rust method. No Horizon function known for ItemEntity.callSetUnderLavaMovement: run `match`. |
| @Inject(HEAD) on ItemEntity.hurt | `ItemEntityMixin.hurtInject` | HEAD | needs-mapping | No Horizon function known for ItemEntity.hurt: run `match`. |
| @Inject(HEAD) on ItemEntity.playerTouch | `ItemEntityMixin.playerTouchInject` | HEAD | needs-mapping | No Horizon function known for ItemEntity.playerTouch: run `match`. |
| @Inject(HEAD) on ItemEntity.fireImmune | `ItemEntityMixin.fireImmuneInject` | HEAD | needs-mapping | No Horizon function known for ItemEntity.fireImmune: run `match`. |
| @Inject(INVOKE) on ItemEntity.tick at Lnet/minecraft/world/entity/item/ItemEntity;isInWater()Z | `ItemEntityMixin.updatePre` | BEFORE_CALL on horizon_client::entities::ItemEntity::inside_block | needs-mapping | Only whole statements, plain let initializers and block tails can be selected. The call it hooks (ItemEntity.isInWater) isn't mapped to a Rust function yet. |
| @Redirect(INVOKE) on ItemEntity.tick at Lnet/minecraft/world/entity/item/ItemEntity;isInWater()Z | `ItemEntityMixin.isInWaterRedirect` | WRAP_OPERATION on horizon_client::entities::ItemEntity::inside_block | needs-mapping | A redirect is a WRAP_OPERATION that doesn't call `original`. The call it hooks (ItemEntity.isInWater) isn't mapped to a Rust function yet. |
| @Redirect(INVOKE) on ItemEntity.tick at Lnet/minecraft/world/entity/item/ItemEntity;isInLava()Z | `ItemEntityMixin.isInLavaRedirect` | WRAP_OPERATION on horizon_client::entities::ItemEntity::inside_block | needs-mapping | A redirect is a WRAP_OPERATION that doesn't call `original`. The call it hooks (ItemEntity.isInLava) isn't mapped to a Rust function yet. |
| @Redirect(INVOKE) on ItemEntity.tick at Lnet/minecraft/world/entity/item/ItemEntity;applyGravity()V | `ItemEntityMixin.isNoGravityRedirect` | WRAP_OPERATION on horizon_client::entities::ItemEntity::inside_block | needs-mapping | A redirect is a WRAP_OPERATION that doesn't call `original`. The call it hooks (ItemEntity.applyGravity) isn't mapped to a Rust function yet. |
| @Inject(INVOKE) on ItemEntity.tick at Lnet/minecraft/world/entity/item/ItemEntity;setDeltaMovement(Lnet/minecraft/world/phys/Vec3;)V | `ItemEntityMixin.update` | BEFORE_CALL on horizon_client::entities::ItemEntity::inside_block | needs-mapping | Only whole statements, plain let initializers and block tails can be selected. The call it hooks (ItemEntity.setDeltaMovement) isn't mapped to a Rust function yet. |
| @Redirect(INVOKE) on ItemEntity.tick at Lnet/minecraft/world/entity/item/ItemEntity;setDeltaMovement(Lnet/minecraft/world/phys/Vec3;)V | `ItemEntityMixin.setDeltaMovementRedirect` | WRAP_OPERATION on horizon_client::entities::ItemEntity::inside_block | needs-mapping | A redirect is a WRAP_OPERATION that doesn't call `original`. The call it hooks (ItemEntity.setDeltaMovement) isn't mapped to a Rust function yet. |
| mixin into ItemEntity with no injectors (interfaces or @Shadow only) | `ItemEntityMixinClient` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| mixin into ItemEntityRenderState with no injectors (interfaces or @Shadow only) | `ItemEntityRenderStateMixin` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| @Inject(HEAD) on ItemEntityRenderer.submit | `ItemEntityRendererMixin.submit` | HEAD | needs-mapping | No Horizon function known for ItemEntityRenderer.submit: run `match`. |
| @Inject(TAIL) on ItemEntityRenderer.extractRenderState | `ItemEntityRendererMixin.injectExtract` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for ItemEntityRenderer.extractRenderState: run `match`. |
| @Invoker on ItemStackRenderState.callFirstLayer | `ItemStackRenderStateAccessor.callFirstLayer` | INVOKE | needs-mapping | A [[bridge]] INVOKE of the Rust method. No Horizon function known for ItemStackRenderState.callFirstLayer: run `match`. |
| @Accessor on LayerRenderState.itemTransform | `LayerRenderStateAccessor.getItemTransform` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for LayerRenderState.itemTransform: run `match`. |
| @Inject(TAIL) on Minecraft.startUseItem | `MinecraftMixin.rightClickEmpty` | RETURN on horizon_client::first_person::Clicks::start_use_item | auto | Horizon has no TAIL: RETURN runs at every exit, early ones too. |
| @Inject(INVOKE) on Minecraft.startUseItem at Lnet/minecraft/world/InteractionHand;values()[Lnet/minecraft/world/InteractionHand; | `MinecraftMixin.rightClickEarly` | BEFORE_CALL on horizon_client::first_person::Clicks::start_use_item | needs-mapping | Only whole statements, plain let initializers and block tails can be selected. The call it hooks (InteractionHand.values) isn't mapped to a Rust function yet. |
| @Inject(HEAD) on MultiPlayerGameMode.dropItem | `MultiPlayerGameModeMixin.drop` | HEAD | needs-mapping | No Horizon function known for MultiPlayerGameMode.dropItem: run `match`. |
| @Accessor on RenderType.name | `RenderTypeAccessor.getName` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for RenderType.name: run `match`. |

## Content

| What | In | Status | Note |
|---|---|---|---|
| 8 asset files, 0 data files | `jar resources` | content | Recipes, loot, tags, models, textures, worldgen: horizon-port-pack carries these. |

## Game methods and fields used

19 of 312 are mapped to Horizon. Every Java body that uses an unmapped one needs it mapped before its Rust can be written.

<details><summary>Mapped</summary>

| Java | Horizon |
|---|---|
| `net/minecraft/client/KeyMapping#<init>` | `self.native_mod_key(MOD_ID, name, default)` |
| `net/minecraft/client/KeyMapping#isDown` | `self.native_mapping(name) for vanilla keys; track key hook presses for the mod's own` |
| `net/minecraft/client/Minecraft#font` | `(implicit)` |
| `net/minecraft/client/Minecraft#getInstance` | `self` |
| `net/minecraft/client/Minecraft#hasControlDown` | horizon_client::game::Game<'_>::has_control_down (suggested) |
| `net/minecraft/client/Minecraft#player` | `self.native_player()` |
| `net/minecraft/client/Minecraft#startUseItem` | horizon_client::first_person::Clicks::start_use_item (suggested) |
| `net/minecraft/client/gui/Font#width` | `self.native_text_width(text)` |
| `net/minecraft/client/player/LocalPlayer#swing` | horizon_client::mobs::ai::Mob::swing (suggested) |
| `net/minecraft/client/renderer/item/ItemStackRenderState#submit` | horizon_client::item_render::ItemRenderState::submit (suggested) |
| `net/minecraft/server/level/ServerPlayer#swing` | horizon_client::mobs::ai::Mob::swing (suggested) |
| `net/minecraft/world/entity/item/ItemEntity#tick` | horizon_client::entities::ItemEntity::inside_block (suggested) |
| `net/minecraft/world/entity/player/Inventory#add` | horizon_client::cauldron::add (suggested) |
| `net/minecraft/world/entity/player/Player#getViewVector` | horizon_client::mobs::goals::Mob::view_vector (suggested) |
| `net/minecraft/world/entity/player/Player#swing` | horizon_client::mobs::ai::Mob::swing (suggested) |
| `net/minecraft/world/item/ItemStack#getCount` | horizon_world::loot::LootStack::get_count (suggested) |
| `net/minecraft/world/level/Level#isClientSide` | horizon_world::level::is_client_side (suggested) |
| `net/minecraft/world/phys/AABB#intersects` | horizon_core::geometry::Aabb::intersects_corners (suggested) |
| `net/minecraft/world/phys/Vec3#closerThan` | horizon_client_render::hud_overlays::closer_than (suggested) |

</details>

<details><summary>Not mapped yet</summary>

| Java | Kind | Classes using it |
|---|---|---|
| `com/mojang/blaze3d/platform/InputConstants#UNKNOWN` | field | 1 |
| `com/mojang/blaze3d/platform/InputConstants$Key#getValue` | method | 1 |
| `com/mojang/blaze3d/platform/Window#getGuiScaledHeight` | method | 1 |
| `com/mojang/blaze3d/platform/Window#getGuiScaledWidth` | method | 1 |
| `com/mojang/blaze3d/vertex/PoseStack#popPose` | method | 1 |
| `com/mojang/blaze3d/vertex/PoseStack#pushPose` | method | 1 |
| `com/mojang/blaze3d/vertex/PoseStack#rotate` | method | 1 |
| `com/mojang/blaze3d/vertex/PoseStack#translate` | method | 2 |
| `net/minecraft/client/DeltaTracker#getGameTimeDeltaPartialTick` | method | 1 |
| `net/minecraft/client/DeltaTracker#getRealtimeDeltaTicks` | method | 1 |
| `net/minecraft/client/KeyMapping#getTranslatedKeyMessage` | method | 1 |
| `net/minecraft/client/KeyMapping#isUnbound` | method | 1 |
| `net/minecraft/client/KeyMapping$Category#GAMEPLAY` | field | 1 |
| `net/minecraft/client/Minecraft#getDeltaTracker` | method | 2 |
| `net/minecraft/client/Minecraft#getWindow` | method | 1 |
| `net/minecraft/client/Minecraft#gui` | field | 1 |
| `net/minecraft/client/Minecraft#hitResult` | field | 1 |
| `net/minecraft/client/Minecraft#isPaused` | method | 2 |
| `net/minecraft/client/Minecraft#level` | field | 2 |
| `net/minecraft/client/Minecraft#options` | field | 1 |
| `net/minecraft/client/Options#keyDrop` | field | 1 |
| `net/minecraft/client/Options#keyUse` | field | 1 |
| `net/minecraft/client/gui/Gui#screen` | method | 1 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#text` | method | 1 |
| `net/minecraft/client/multiplayer/MultiPlayerGameMode#dropItem` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#getInventory` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#getMainHandItem` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#sendOverlayMessage` | method | 1 |
| `net/minecraft/client/renderer/entity/EntityRenderer#<init>` | method | 1 |
| `net/minecraft/client/renderer/entity/EntityRenderer#submit` | method | 1 |
| `net/minecraft/client/renderer/entity/ItemEntityRenderer#extractRenderState` | method | 1 |
| `net/minecraft/client/renderer/entity/ItemEntityRenderer#submit` | method | 1 |
| `net/minecraft/client/renderer/entity/state/ItemEntityRenderState#ageInTicks` | field | 1 |
| `net/minecraft/client/renderer/entity/state/ItemEntityRenderState#bobOffset` | field | 1 |
| `net/minecraft/client/renderer/entity/state/ItemEntityRenderState#count` | field | 1 |
| `net/minecraft/client/renderer/entity/state/ItemEntityRenderState#item` | field | 2 |
| `net/minecraft/client/renderer/entity/state/ItemEntityRenderState#lightCoords` | field | 1 |
| `net/minecraft/client/renderer/entity/state/ItemEntityRenderState#outlineColor` | field | 1 |
| `net/minecraft/client/renderer/entity/state/ItemEntityRenderState#seed` | field | 1 |
| `net/minecraft/client/renderer/item/ItemStackRenderState#callFirstLayer` | method | 1 |
| `net/minecraft/client/renderer/item/ItemStackRenderState#usesBlockLight` | method | 1 |
| `net/minecraft/client/renderer/item/ItemStackRenderState$LayerRenderState#itemTransform` | method | 1 |
| `net/minecraft/client/renderer/rendertype/RenderType#name` | method | 1 |
| `net/minecraft/client/renderer/texture/OverlayTexture#NO_OVERLAY` | field | 1 |
| `net/minecraft/client/resources/model/cuboid/ItemTransform#scale` | method | 1 |
| `net/minecraft/core/BlockPos#below` | method | 2 |
| `net/minecraft/core/BlockPos#containing` | method | 1 |
| `net/minecraft/core/BlockPos#getY` | method | 1 |
| `net/minecraft/core/Direction#getApproximateNearest` | method | 1 |
| `net/minecraft/core/particles/ParticleTypes#SMOKE` | field | 1 |
| `net/minecraft/network/chat/Component#getString` | method | 1 |
| `net/minecraft/network/chat/Component#literal` | method | 1 |
| `net/minecraft/network/chat/Component#translatable` | method | 1 |
| `net/minecraft/resources/Identifier#tryBuild` | method | 1 |
| `net/minecraft/server/level/ServerLevel#addFreshEntity` | method | 1 |
| `net/minecraft/server/level/ServerLevel#getEntity` | method | 1 |
| `net/minecraft/server/level/ServerLevel#playSound` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#awardStat` | method | 2 |
| `net/minecraft/server/level/ServerPlayer#containerMenu` | field | 1 |
| `net/minecraft/server/level/ServerPlayer#getInventory` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#level` | method | 2 |
| `net/minecraft/sounds/SoundEvents#GENERIC_BURN` | field | 1 |
| `net/minecraft/sounds/SoundEvents#ITEM_PICKUP` | field | 1 |
| `net/minecraft/sounds/SoundEvents#WOOL_FALL` | field | 1 |
| `net/minecraft/sounds/SoundSource#PLAYERS` | field | 1 |
| `net/minecraft/stats/StatType#get` | method | 2 |
| `net/minecraft/stats/Stats#DROP` | field | 1 |
| `net/minecraft/stats/Stats#ITEM_DROPPED` | field | 1 |
| `net/minecraft/stats/Stats#ITEM_PICKED_UP` | field | 1 |
| `net/minecraft/tags/BlockItemTags#LOGS_THAT_BURN` | field | 1 |
| `net/minecraft/tags/BlockTags#FLOWERS` | field | 1 |
| `net/minecraft/tags/BlockTags#ICE` | field | 1 |
| `net/minecraft/tags/BlockTags#LEAVES` | field | 1 |
| `net/minecraft/tags/BlockTags#LOGS` | field | 1 |
| `net/minecraft/tags/BlockTags#MINEABLE_WITH_AXE` | field | 1 |
| `net/minecraft/tags/BlockTags#PLANKS` | field | 1 |
| `net/minecraft/tags/BlockTags#SMALL_FLOWERS` | field | 1 |
| `net/minecraft/tags/BlockTags#SNOW` | field | 1 |
| `net/minecraft/tags/BlockTags#WOOL` | field | 1 |
| `net/minecraft/tags/BlockTags#WOOL_CARPETS` | field | 1 |
| `net/minecraft/tags/DamageTypeTags#IS_EXPLOSION` | field | 1 |
| `net/minecraft/tags/DamageTypeTags#IS_FIRE` | field | 1 |
| `net/minecraft/tags/FluidTags#LAVA` | field | 1 |
| `net/minecraft/tags/FluidTags#WATER` | field | 1 |
| `net/minecraft/util/Mth#cos` | method | 1 |
| `net/minecraft/util/Mth#sin` | method | 1 |
| `net/minecraft/util/Mth#square` | method | 1 |
| `net/minecraft/util/RandomSource#nextDouble` | method | 1 |
| `net/minecraft/util/RandomSource#nextFloat` | method | 4 |
| `net/minecraft/util/RandomSource#setSeed` | method | 1 |
| `net/minecraft/world/InteractionHand#MAIN_HAND` | field | 3 |
| `net/minecraft/world/InteractionHand#values` | method | 1 |
| `net/minecraft/world/InteractionResult#CONSUME` | field | 1 |
| `net/minecraft/world/InteractionResult#PASS` | field | 1 |
| `net/minecraft/world/damagesource/DamageSource#is` | method | 1 |
| `net/minecraft/world/damagesource/DamageSources#cactus` | method | 1 |
| `net/minecraft/world/damagesource/DamageSources#inFire` | method | 1 |
| `net/minecraft/world/damagesource/DamageSources#lava` | method | 2 |
| `net/minecraft/world/damagesource/DamageSources#onFire` | method | 1 |
| `net/minecraft/world/entity/Entity#<init>` | method | 2 |
| `net/minecraft/world/entity/Entity#callGetBlockPosBelowThatAffectsMyMovement` | method | 1 |
| `net/minecraft/world/entity/Entity#callIsInvulnerableToBase` | method | 1 |
| `net/minecraft/world/entity/Entity#checkFallDamage` | method | 1 |
| `net/minecraft/world/entity/Entity#getBoundingBox` | method | 1 |
| `net/minecraft/world/entity/Entity#getFluidInteractionBox` | method | 1 |
| `net/minecraft/world/entity/Entity#isAlive` | method | 1 |
| `net/minecraft/world/entity/Entity#isPickable` | method | 1 |
| `net/minecraft/world/entity/Entity#isSpectator` | method | 1 |
| `net/minecraft/world/entity/Entity#onSyncedDataUpdated` | method | 1 |
| `net/minecraft/world/entity/Entity#stuckSpeedMultiplier` | method | 1 |
| `net/minecraft/world/entity/item/ItemEntity#<init>` | method | 1 |
| `net/minecraft/world/entity/item/ItemEntity#applyGravity` | method | 1 |
| `net/minecraft/world/entity/item/ItemEntity#blockPosition` | method | 3 |
| `net/minecraft/world/entity/item/ItemEntity#callSetUnderLavaMovement` | method | 1 |
| `net/minecraft/world/entity/item/ItemEntity#callSetUnderwaterMovement` | method | 1 |
| `net/minecraft/world/entity/item/ItemEntity#damageSources` | method | 1 |
| `net/minecraft/world/entity/item/ItemEntity#discard` | method | 1 |
| `net/minecraft/world/entity/item/ItemEntity#fallDistance` | field | 1 |
| `net/minecraft/world/entity/item/ItemEntity#fireImmune` | method | 1 |
| `net/minecraft/world/entity/item/ItemEntity#getBoundingBox` | method | 1 |
| `net/minecraft/world/entity/item/ItemEntity#getDeltaMovement` | method | 2 |
| `net/minecraft/world/entity/item/ItemEntity#getFluidHeight` | method | 1 |
| `net/minecraft/world/entity/item/ItemEntity#getGravity` | method | 1 |
| `net/minecraft/world/entity/item/ItemEntity#getItem` | method | 5 |
| `net/minecraft/world/entity/item/ItemEntity#getRandom` | method | 1 |
| `net/minecraft/world/entity/item/ItemEntity#getUUID` | method | 1 |
| `net/minecraft/world/entity/item/ItemEntity#getX` | method | 1 |
| `net/minecraft/world/entity/item/ItemEntity#getXRot` | method | 2 |
| `net/minecraft/world/entity/item/ItemEntity#getY` | method | 2 |
| `net/minecraft/world/entity/item/ItemEntity#getYRot` | method | 1 |
| `net/minecraft/world/entity/item/ItemEntity#getZ` | method | 1 |
| `net/minecraft/world/entity/item/ItemEntity#hasPickUpDelay` | method | 1 |
| `net/minecraft/world/entity/item/ItemEntity#hurt` | method | 1 |
| `net/minecraft/world/entity/item/ItemEntity#isInLava` | method | 2 |
| `net/minecraft/world/entity/item/ItemEntity#isInWater` | method | 3 |
| `net/minecraft/world/entity/item/ItemEntity#isNoGravity` | method | 1 |
| `net/minecraft/world/entity/item/ItemEntity#isRemoved` | method | 1 |
| `net/minecraft/world/entity/item/ItemEntity#level` | method | 4 |
| `net/minecraft/world/entity/item/ItemEntity#onGround` | method | 3 |
| `net/minecraft/world/entity/item/ItemEntity#playSound` | method | 1 |
| `net/minecraft/world/entity/item/ItemEntity#playerTouch` | method | 1 |
| `net/minecraft/world/entity/item/ItemEntity#setDeltaMovement` | method | 5 |
| `net/minecraft/world/entity/item/ItemEntity#setItem` | method | 1 |
| `net/minecraft/world/entity/item/ItemEntity#setPickUpDelay` | method | 1 |
| `net/minecraft/world/entity/item/ItemEntity#setThrower` | method | 1 |
| `net/minecraft/world/entity/item/ItemEntity#setXRot` | method | 1 |
| `net/minecraft/world/entity/item/ItemEntity#target` | method | 1 |
| `net/minecraft/world/entity/player/Inventory#getSelectedItem` | method | 2 |
| `net/minecraft/world/entity/player/Inventory#getSelectedSlot` | method | 2 |
| `net/minecraft/world/entity/player/Inventory#removeFromSelected` | method | 1 |
| `net/minecraft/world/entity/player/Inventory#removeItem` | method | 1 |
| `net/minecraft/world/entity/player/Player#awardStat` | method | 1 |
| `net/minecraft/world/entity/player/Player#blockInteractionRange` | method | 1 |
| `net/minecraft/world/entity/player/Player#entityInteractionRange` | method | 1 |
| `net/minecraft/world/entity/player/Player#getBoundingBox` | method | 3 |
| `net/minecraft/world/entity/player/Player#getEyePosition` | method | 1 |
| `net/minecraft/world/entity/player/Player#getEyeY` | method | 1 |
| `net/minecraft/world/entity/player/Player#getInventory` | method | 1 |
| `net/minecraft/world/entity/player/Player#getRandom` | method | 1 |
| `net/minecraft/world/entity/player/Player#getUUID` | method | 1 |
| `net/minecraft/world/entity/player/Player#getX` | method | 1 |
| `net/minecraft/world/entity/player/Player#getXRot` | method | 1 |
| `net/minecraft/world/entity/player/Player#getYRot` | method | 1 |
| `net/minecraft/world/entity/player/Player#getZ` | method | 1 |
| `net/minecraft/world/entity/player/Player#isCrouching` | method | 1 |
| `net/minecraft/world/entity/player/Player#isSpectator` | method | 1 |
| `net/minecraft/world/entity/player/Player#level` | method | 4 |
| `net/minecraft/world/entity/player/Player#onItemPickup` | method | 1 |
| `net/minecraft/world/entity/player/Player#take` | method | 1 |
| `net/minecraft/world/entity/projectile/ProjectileUtil#getEntityHitResult` | method | 1 |
| `net/minecraft/world/inventory/AbstractContainerMenu#findSlot` | method | 1 |
| `net/minecraft/world/inventory/AbstractContainerMenu#setRemoteSlot` | method | 1 |
| `net/minecraft/world/item/Item$TooltipContext#of` | method | 1 |
| `net/minecraft/world/item/ItemStack#EMPTY` | field | 1 |
| `net/minecraft/world/item/ItemStack#canBeHurtBy` | method | 1 |
| `net/minecraft/world/item/ItemStack#copy` | method | 1 |
| `net/minecraft/world/item/ItemStack#getItem` | method | 2 |
| `net/minecraft/world/item/ItemStack#getTooltipLines` | method | 1 |
| `net/minecraft/world/item/ItemStack#isEmpty` | method | 3 |
| `net/minecraft/world/item/ItemStack#setCount` | method | 1 |
| `net/minecraft/world/item/Items#ACACIA_BOAT` | field | 1 |
| `net/minecraft/world/item/Items#ACACIA_DOOR` | field | 1 |
| `net/minecraft/world/item/Items#APPLE` | field | 1 |
| `net/minecraft/world/item/Items#ARROW` | field | 1 |
| `net/minecraft/world/item/Items#BAKED_POTATO` | field | 1 |
| `net/minecraft/world/item/Items#BEETROOT` | field | 1 |
| `net/minecraft/world/item/Items#BEETROOT_SEEDS` | field | 1 |
| `net/minecraft/world/item/Items#BEETROOT_SOUP` | field | 1 |
| `net/minecraft/world/item/Items#BIRCH_BOAT` | field | 1 |
| `net/minecraft/world/item/Items#BIRCH_DOOR` | field | 1 |
| `net/minecraft/world/item/Items#BONE` | field | 1 |
| `net/minecraft/world/item/Items#BOOK` | field | 1 |
| `net/minecraft/world/item/Items#BOW` | field | 1 |
| `net/minecraft/world/item/Items#BOWL` | field | 1 |
| `net/minecraft/world/item/Items#BREAD` | field | 1 |
| `net/minecraft/world/item/Items#CAKE` | field | 1 |
| `net/minecraft/world/item/Items#CARROT` | field | 1 |
| `net/minecraft/world/item/Items#COOKED_MUTTON` | field | 1 |
| `net/minecraft/world/item/Items#COOKED_RABBIT` | field | 1 |
| `net/minecraft/world/item/Items#DARK_OAK_BOAT` | field | 1 |
| `net/minecraft/world/item/Items#DARK_OAK_DOOR` | field | 1 |
| `net/minecraft/world/item/Items#EGG` | field | 1 |
| `net/minecraft/world/item/Items#ELYTRA` | field | 1 |
| `net/minecraft/world/item/Items#ENCHANTED_BOOK` | field | 1 |
| `net/minecraft/world/item/Items#FEATHER` | field | 1 |
| `net/minecraft/world/item/Items#FILLED_MAP` | field | 1 |
| `net/minecraft/world/item/Items#FISHING_ROD` | field | 1 |
| `net/minecraft/world/item/Items#JUNGLE_BOAT` | field | 1 |
| `net/minecraft/world/item/Items#JUNGLE_DOOR` | field | 1 |
| `net/minecraft/world/item/Items#LEAD` | field | 1 |
| `net/minecraft/world/item/Items#LEATHER` | field | 1 |
| `net/minecraft/world/item/Items#LEATHER_BOOTS` | field | 1 |
| `net/minecraft/world/item/Items#LEATHER_CHESTPLATE` | field | 1 |
| `net/minecraft/world/item/Items#LEATHER_HELMET` | field | 1 |
| `net/minecraft/world/item/Items#LEATHER_LEGGINGS` | field | 1 |
| `net/minecraft/world/item/Items#MAP` | field | 1 |
| `net/minecraft/world/item/Items#MELON` | field | 1 |
| `net/minecraft/world/item/Items#MELON_SEEDS` | field | 1 |
| `net/minecraft/world/item/Items#MUTTON` | field | 1 |
| `net/minecraft/world/item/Items#NAME_TAG` | field | 1 |
| `net/minecraft/world/item/Items#NETHER_STAR` | field | 2 |
| `net/minecraft/world/item/Items#OAK_BOAT` | field | 1 |
| `net/minecraft/world/item/Items#OAK_DOOR` | field | 1 |
| `net/minecraft/world/item/Items#PAINTING` | field | 1 |
| `net/minecraft/world/item/Items#PAPER` | field | 1 |
| `net/minecraft/world/item/Items#POISONOUS_POTATO` | field | 1 |
| `net/minecraft/world/item/Items#POTATO` | field | 1 |
| `net/minecraft/world/item/Items#PUMPKIN_PIE` | field | 1 |
| `net/minecraft/world/item/Items#PUMPKIN_SEEDS` | field | 1 |
| `net/minecraft/world/item/Items#RABBIT` | field | 1 |
| `net/minecraft/world/item/Items#RABBIT_STEW` | field | 1 |
| `net/minecraft/world/item/Items#ROTTEN_FLESH` | field | 1 |
| `net/minecraft/world/item/Items#SADDLE` | field | 1 |
| `net/minecraft/world/item/Items#SHEARS` | field | 1 |
| `net/minecraft/world/item/Items#SHIELD` | field | 1 |
| `net/minecraft/world/item/Items#SNOWBALL` | field | 1 |
| `net/minecraft/world/item/Items#SPIDER_EYE` | field | 1 |
| `net/minecraft/world/item/Items#SPRUCE_BOAT` | field | 1 |
| `net/minecraft/world/item/Items#SPRUCE_DOOR` | field | 1 |
| `net/minecraft/world/item/Items#STRING` | field | 1 |
| `net/minecraft/world/item/Items#SUGAR` | field | 1 |
| `net/minecraft/world/item/Items#WHEAT` | field | 1 |
| `net/minecraft/world/item/Items#WHEAT_SEEDS` | field | 1 |
| `net/minecraft/world/item/Items#WRITABLE_BOOK` | field | 1 |
| `net/minecraft/world/item/Items#WRITTEN_BOOK` | field | 1 |
| `net/minecraft/world/item/TooltipFlag#NORMAL` | field | 1 |
| `net/minecraft/world/item/component/SwingAnimation#DEFAULT` | field | 3 |
| `net/minecraft/world/level/ClipContext#<init>` | method | 1 |
| `net/minecraft/world/level/ClipContext$Block#COLLIDER` | field | 1 |
| `net/minecraft/world/level/ClipContext$Fluid#NONE` | field | 1 |
| `net/minecraft/world/level/Level#addParticle` | method | 1 |
| `net/minecraft/world/level/Level#clip` | method | 1 |
| `net/minecraft/world/level/Level#getBlockState` | method | 1 |
| `net/minecraft/world/level/Level#getEntities` | method | 1 |
| `net/minecraft/world/level/Level#getFluidState` | method | 1 |
| `net/minecraft/world/level/Level#playSound` | method | 1 |
| `net/minecraft/world/level/block/Blocks#BARRIER` | field | 1 |
| `net/minecraft/world/level/block/Blocks#BEDROCK` | field | 1 |
| `net/minecraft/world/level/block/Blocks#CACTUS` | field | 1 |
| `net/minecraft/world/level/block/Blocks#CAKE` | field | 1 |
| `net/minecraft/world/level/block/Blocks#COBWEB` | field | 1 |
| `net/minecraft/world/level/block/Blocks#MUD` | field | 1 |
| `net/minecraft/world/level/block/Blocks#OBSIDIAN` | field | 1 |
| `net/minecraft/world/level/block/Blocks#SNOW` | field | 2 |
| `net/minecraft/world/level/block/Blocks#SOUL_SAND` | field | 1 |
| `net/minecraft/world/level/block/Blocks#SPONGE` | field | 1 |
| `net/minecraft/world/level/block/Blocks#WET_SPONGE` | field | 1 |
| `net/minecraft/world/level/material/Fluid#is` | method | 1 |
| `net/minecraft/world/level/material/FluidState#getHeight` | method | 1 |
| `net/minecraft/world/level/material/FluidState#getType` | method | 1 |
| `net/minecraft/world/level/material/FluidState#isEmpty` | method | 1 |
| `net/minecraft/world/phys/AABB#clip` | method | 1 |
| `net/minecraft/world/phys/AABB#contains` | method | 1 |
| `net/minecraft/world/phys/AABB#expandTowards` | method | 2 |
| `net/minecraft/world/phys/AABB#inflate` | method | 4 |
| `net/minecraft/world/phys/BlockHitResult#miss` | method | 1 |
| `net/minecraft/world/phys/EntityHitResult#<init>` | method | 2 |
| `net/minecraft/world/phys/EntityHitResult#getEntity` | method | 1 |
| `net/minecraft/world/phys/EntityHitResult#getLocation` | method | 1 |
| `net/minecraft/world/phys/HitResult#getLocation` | method | 1 |
| `net/minecraft/world/phys/HitResult#getType` | method | 2 |
| `net/minecraft/world/phys/HitResult$Type#ENTITY` | field | 1 |
| `net/minecraft/world/phys/HitResult$Type#MISS` | field | 2 |
| `net/minecraft/world/phys/Vec3#add` | method | 2 |
| `net/minecraft/world/phys/Vec3#distanceTo` | method | 1 |
| `net/minecraft/world/phys/Vec3#distanceToSqr` | method | 1 |
| `net/minecraft/world/phys/Vec3#lengthSqr` | method | 1 |
| `net/minecraft/world/phys/Vec3#multiply` | method | 1 |
| `net/minecraft/world/phys/Vec3#scale` | method | 2 |
| `net/minecraft/world/phys/Vec3#subtract` | method | 1 |
| `net/minecraft/world/phys/Vec3#x` | field | 3 |
| `net/minecraft/world/phys/Vec3#y` | field | 2 |
| `net/minecraft/world/phys/Vec3#z` | field | 2 |

</details>

