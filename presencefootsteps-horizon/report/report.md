# Horizon port report: Presence Footsteps

`presencefootsteps` 1.14.0+26.3 · PresenceFootsteps-1.14.0+26.3.jar · 107 classes

**Partly convertible**

| Status | Count | Meaning |
|---|---|---|
| auto | 2 | The Horizon hook or injection is known: the wiring can be generated. The Java body still has to become Rust. |
| content | 1 | Data and assets: horizon-port-pack carries these into a content mod (.hmod). |
| needs-mapping | 6 | The kind of change carries over, but the matching Horizon function isn't in the table yet. Run `match`. |
| needs-translation | 3 | No direct equivalent: a person (or an AI agent) has to rewrite this part. |
| unsupported | 0 | Horizon has nothing this can attach to yet. |

## Entry points

- `client`: eu.ha3.presencefootsteps.PresenceFootsteps
- `modmenu`: eu.ha3.presencefootsteps.PFModMenuFactory

## Fabric events

| Event | In | Horizon | Status | Note |
|---|---|---|---|---|
| KeyMappingHelper.registerKeyMapping | `PresenceFootsteps` | hook `key` | auto | Mojang-named variant of KeyBindingHelper. |
| ClientTickEvents.END_CLIENT_TICK | `PresenceFootsteps` | hook `tick` | auto | The Fabric API layer (jfabricapi.rs) runs START_CLIENT_TICK, then the level ticks (START/END_LEVEL_TICK, while in a world), then END_CLIENT_TICK, from Horizon's tick hook, in the window's Game only. |

## Mixins

| Injection | In | Horizon | Status | Note |
|---|---|---|---|---|
| @Inject(HEAD) on Entity.walkingStepSound | `IEntity.onPlayStepSounds` | HEAD | needs-mapping | No Horizon function known for Entity.walkingStepSound: run `match`. |
| @Accessor on LivingEntity.jumping | `ILivingEntity.isJumping` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for LivingEntity.jumping: run `match`. |
| mixin into Block with no injectors (interfaces or @Shadow only) | `MAbstractBlock` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| @Inject(RETURN) on Properties.ofLegacyCopy | `MBlockSettings.onCopy` | RETURN | needs-mapping | Horizon's RETURN runs at every normal exit and passes the return value on. No Horizon function known for Properties.ofLegacyCopy: run `match`. |
| @Inject(INVOKE) on ClientPacketListener.handleSoundEvent at net/minecraft/client/multiplayer/ClientLevel.playSeededSound(Lnet/minecraft/world/entity/Entity;DDDLnet/minecraft/core/Holder;Lnet/minecraft/sounds/SoundSource;FFJ)V | `MClientPlayNetworkHandler.onHandleSoundEffect` | BEFORE_CALL | needs-mapping | Only whole statements, plain let initializers and block tails can be selected. No Horizon function known for ClientPacketListener.handleSoundEvent: run `match`. |
| mixin into LivingEntity with no injectors (interfaces or @Shadow only) | `MLivingEntity` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| @ModifyReturnValue(RETURN) on SoundEngine.calculateVolume | `MSoundSystem.adjustSoundVolume` | RETURN | needs-mapping | No Horizon function known for SoundEngine.calculateVolume: run `match`. |
| @Accessor on StairBlock.baseState | `MStairsBlock.getBaseBlockState` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for StairBlock.baseState: run `match`. |

## Classes extending the game

| Class | In | Status | Note |
|---|---|---|---|
| extends SimpleSoundInstance | `ImmediateSoundPlayer$UncappedSoundInstance` | needs-translation | No mapping for this superclass yet. |

## Content

| What | In | Status | Note |
|---|---|---|---|
| 1017 asset files, 0 data files | `jar resources` | content | Recipes, loot, tags, models, textures, worldgen: horizon-port-pack carries these. |

## Game methods and fields used

21 of 287 are mapped to Horizon. Every Java body that uses an unmapped one needs it mapped before its Rust can be written.

<details><summary>Mapped</summary>

| Java | Horizon |
|---|---|
| `net/minecraft/client/KeyMapping#<init>` | `self.native_mod_key(MOD_ID, name, default)` |
| `net/minecraft/client/KeyMapping#isDown` | `self.native_mapping(name) for vanilla keys; track key hook presses for the mod's own` |
| `net/minecraft/client/Minecraft#getInstance` | `self` |
| `net/minecraft/client/Minecraft#player` | `self.native_player()` |
| `net/minecraft/client/multiplayer/ClientLevel#isRainingAt` | horizon_world::level::weather::Behaviours::is_raining_at (suggested) |
| `net/minecraft/core/registries/BuiltInRegistries#BLOCK` | content |
| `net/minecraft/network/chat/MutableComponent#withStyle` | horizon_client_core::text::Text::with_style (suggested) |
| `net/minecraft/util/Mth#clamp` | horizon_client_render::chat::clamp (suggested) |
| `net/minecraft/util/Mth#floor` | horizon_client_render::gfx::occlusion::floor (suggested) |
| `net/minecraft/util/Mth#lerp` | horizon_world::stepped_interpolation::lerp (suggested) |
| `net/minecraft/util/Mth#wrapDegrees` | horizon_world::stepped_interpolation::wrap_degrees (suggested) |
| `net/minecraft/world/entity/Entity#getX` | `self.native_player()?.x` |
| `net/minecraft/world/entity/Entity#getY` | `self.native_player()?.y` |
| `net/minecraft/world/entity/Entity#getZ` | `self.native_player()?.z` |
| `net/minecraft/world/entity/EntityType#create` | horizon_client::mobs::finalize::create (suggested) |
| `net/minecraft/world/entity/LivingEntity#isInLava` | horizon_client::motion::fluids::PlayerMotion::is_in_lava (suggested) |
| `net/minecraft/world/entity/LivingEntity#onGround` | horizon_client::game::minecarts::Minecart::on_ground (suggested) |
| `net/minecraft/world/level/Level#isClientSide` | horizon_world::level::is_client_side (suggested) |
| `net/minecraft/world/level/Level#isRainingAt` | horizon_world::level::weather::Behaviours::is_raining_at (suggested) |
| `net/minecraft/world/level/block/state/BlockState#getCollisionShape` | horizon_client_render::collisions::<SceneCollisions<'_, '_> as PistonCollisionWorld>::collision_shape (suggested) |
| `net/minecraft/world/phys/AABB#intersects` | horizon_core::geometry::Aabb::intersects_corners (suggested) |

</details>

<details><summary>Not mapped yet</summary>

| Java | Kind | Classes using it |
|---|---|---|
| `com/mojang/blaze3d/platform/InputConstants#UNKNOWN` | field | 1 |
| `com/mojang/blaze3d/platform/InputConstants$Key#getValue` | method | 1 |
| `com/mojang/blaze3d/platform/InputConstants$Type#KEYBOARD` | field | 1 |
| `net/minecraft/ChatFormatting#BLUE` | field | 1 |
| `net/minecraft/ChatFormatting#BOLD` | field | 1 |
| `net/minecraft/ChatFormatting#GRAY` | field | 2 |
| `net/minecraft/ChatFormatting#GREEN` | field | 3 |
| `net/minecraft/ChatFormatting#RED` | field | 2 |
| `net/minecraft/ChatFormatting#RESET` | field | 1 |
| `net/minecraft/ChatFormatting#UNDERLINE` | field | 1 |
| `net/minecraft/CrashReport#addCategory` | method | 1 |
| `net/minecraft/CrashReport#forThrowable` | method | 1 |
| `net/minecraft/CrashReportCategory#setDetail` | method | 2 |
| `net/minecraft/ReportedException#<init>` | method | 1 |
| `net/minecraft/client/Camera#position` | method | 1 |
| `net/minecraft/client/DeltaTracker#getGameTimeDeltaPartialTick` | method | 1 |
| `net/minecraft/client/KeyMapping#getTranslatedKeyMessage` | method | 2 |
| `net/minecraft/client/KeyMapping$Category#register` | method | 1 |
| `net/minecraft/client/Minecraft#debugEntries` | field | 1 |
| `net/minecraft/client/Minecraft#gameRenderer` | field | 1 |
| `net/minecraft/client/Minecraft#getDeltaTracker` | method | 1 |
| `net/minecraft/client/Minecraft#getResourceManager` | method | 2 |
| `net/minecraft/client/Minecraft#getResourcePackDirectory` | method | 1 |
| `net/minecraft/client/Minecraft#getResourcePackRepository` | method | 1 |
| `net/minecraft/client/Minecraft#getSoundManager` | method | 1 |
| `net/minecraft/client/Minecraft#gui` | field | 3 |
| `net/minecraft/client/Minecraft#hitResult` | field | 1 |
| `net/minecraft/client/Minecraft#isLocalServer` | method | 2 |
| `net/minecraft/client/Minecraft#isPaused` | method | 1 |
| `net/minecraft/client/Minecraft#level` | field | 4 |
| `net/minecraft/client/Minecraft#options` | field | 2 |
| `net/minecraft/client/Options#getFinalSoundSourceVolume` | method | 1 |
| `net/minecraft/client/Options#keyDebugModifier` | field | 1 |
| `net/minecraft/client/Options#loadSelectedResourcePacks` | method | 1 |
| `net/minecraft/client/gui/Gui#hud` | field | 1 |
| `net/minecraft/client/gui/Gui#screen` | method | 1 |
| `net/minecraft/client/gui/Gui#setScreen` | method | 2 |
| `net/minecraft/client/gui/Gui#toastManager` | method | 1 |
| `net/minecraft/client/gui/Hud#getChat` | method | 1 |
| `net/minecraft/client/gui/components/ChatComponent#addClientSystemMessage` | method | 1 |
| `net/minecraft/client/gui/components/debug/DebugEntryNoop#<init>` | method | 1 |
| `net/minecraft/client/gui/components/debug/DebugScreenDisplayer#addToGroup` | method | 2 |
| `net/minecraft/client/gui/components/debug/DebugScreenEntries#LOOKING_AT_BLOCK_STATE` | field | 1 |
| `net/minecraft/client/gui/components/debug/DebugScreenEntries#LOOKING_AT_ENTITY` | field | 1 |
| `net/minecraft/client/gui/components/debug/DebugScreenEntries#SOUND_MOOD` | field | 1 |
| `net/minecraft/client/gui/components/debug/DebugScreenEntries#register` | method | 1 |
| `net/minecraft/client/gui/components/debug/DebugScreenEntryList#isCurrentlyEnabled` | method | 1 |
| `net/minecraft/client/gui/components/debug/DebugScreenEntryList#toggleStatus` | method | 1 |
| `net/minecraft/client/gui/components/toasts/SystemToast#addOrUpdate` | method | 1 |
| `net/minecraft/client/gui/components/toasts/SystemToast$SystemToastId#<init>` | method | 1 |
| `net/minecraft/client/gui/screens/packs/PackSelectionScreen#<init>` | method | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#getBiome` | method | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#getBlockState` | method | 2 |
| `net/minecraft/client/multiplayer/ClientLevel#getSeaLevel` | method | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#isRaining` | method | 1 |
| `net/minecraft/client/multiplayer/ClientPacketListener#handleSoundEvent` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#isRemoved` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#sendSystemMessage` | method | 1 |
| `net/minecraft/client/player/RemotePlayer#getAbilities` | method | 1 |
| `net/minecraft/client/renderer/GameRenderer#mainCamera` | method | 1 |
| `net/minecraft/client/resources/sounds/SimpleSoundInstance#<init>` | method | 1 |
| `net/minecraft/client/resources/sounds/SoundInstance#createUnseededRandom` | method | 1 |
| `net/minecraft/client/resources/sounds/SoundInstance#getSource` | method | 1 |
| `net/minecraft/client/resources/sounds/SoundInstance#getVolume` | method | 1 |
| `net/minecraft/client/resources/sounds/SoundInstance$Attenuation#LINEAR` | field | 1 |
| `net/minecraft/client/sounds/SoundEngine#calculateVolume` | method | 1 |
| `net/minecraft/client/sounds/SoundManager#play` | method | 1 |
| `net/minecraft/client/sounds/SoundManager#playDelayed` | method | 1 |
| `net/minecraft/core/BlockPos#ZERO` | field | 1 |
| `net/minecraft/core/BlockPos#above` | method | 3 |
| `net/minecraft/core/BlockPos#asLong` | method | 1 |
| `net/minecraft/core/BlockPos#below` | method | 1 |
| `net/minecraft/core/BlockPos#containing` | method | 3 |
| `net/minecraft/core/BlockPos#getX` | method | 1 |
| `net/minecraft/core/BlockPos#getY` | method | 1 |
| `net/minecraft/core/BlockPos#getZ` | method | 1 |
| `net/minecraft/core/BlockPos#immutable` | method | 1 |
| `net/minecraft/core/BlockPos#mutable` | method | 1 |
| `net/minecraft/core/BlockPos#withinClippedManhattan` | method | 1 |
| `net/minecraft/core/BlockPos$MutableBlockPos#below` | method | 1 |
| `net/minecraft/core/BlockPos$MutableBlockPos#getX` | method | 1 |
| `net/minecraft/core/BlockPos$MutableBlockPos#getY` | method | 1 |
| `net/minecraft/core/BlockPos$MutableBlockPos#getZ` | method | 1 |
| `net/minecraft/core/BlockPos$MutableBlockPos#move` | method | 1 |
| `net/minecraft/core/BlockPos$MutableBlockPos#set` | method | 2 |
| `net/minecraft/core/DefaultedRegistry#containsKey` | method | 2 |
| `net/minecraft/core/DefaultedRegistry#getKey` | method | 3 |
| `net/minecraft/core/DefaultedRegistry#getOptional` | method | 2 |
| `net/minecraft/core/Direction#DOWN` | field | 1 |
| `net/minecraft/core/Direction#UP` | field | 1 |
| `net/minecraft/core/Direction$Axis#X` | field | 1 |
| `net/minecraft/core/Direction$Axis#Y` | field | 1 |
| `net/minecraft/core/Direction$Axis#Z` | field | 1 |
| `net/minecraft/core/Holder#tags` | method | 1 |
| `net/minecraft/core/Holder#unwrap` | method | 1 |
| `net/minecraft/core/Holder#unwrapKey` | method | 2 |
| `net/minecraft/core/Holder#value` | method | 2 |
| `net/minecraft/core/component/DataComponents#EQUIPPABLE` | field | 1 |
| `net/minecraft/core/particles/ParticleTypes#DOLPHIN` | field | 1 |
| `net/minecraft/core/particles/ParticleTypes#DUST_PLUME` | field | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#ENTITY_TYPE` | field | 3 |
| `net/minecraft/core/registries/Registries#BLOCK` | field | 1 |
| `net/minecraft/network/chat/ClickEvent$OpenFile#<init>` | method | 1 |
| `net/minecraft/network/chat/Component#literal` | method | 1 |
| `net/minecraft/network/chat/Component#translatable` | method | 7 |
| `net/minecraft/network/chat/MutableComponent#withColor` | method | 1 |
| `net/minecraft/network/chat/Style#withClickEvent` | method | 1 |
| `net/minecraft/network/chat/Style#withColor` | method | 1 |
| `net/minecraft/network/chat/Style#withUnderlined` | method | 1 |
| `net/minecraft/network/chat/TextColor#YELLOW` | field | 1 |
| `net/minecraft/network/protocol/game/ClientboundSoundPacket#getSound` | method | 1 |
| `net/minecraft/network/protocol/game/ClientboundSoundPacket#getSource` | method | 1 |
| `net/minecraft/network/protocol/game/ClientboundSoundPacket#getX` | method | 1 |
| `net/minecraft/network/protocol/game/ClientboundSoundPacket#getY` | method | 1 |
| `net/minecraft/network/protocol/game/ClientboundSoundPacket#getZ` | method | 1 |
| `net/minecraft/resources/FileToIdConverter#fileToId` | method | 1 |
| `net/minecraft/resources/FileToIdConverter#json` | method | 2 |
| `net/minecraft/resources/FileToIdConverter#listMatchingResourceStacks` | method | 1 |
| `net/minecraft/resources/FileToIdConverter#listMatchingResources` | method | 1 |
| `net/minecraft/resources/Identifier#equals` | method | 1 |
| `net/minecraft/resources/Identifier#fromNamespaceAndPath` | method | 2 |
| `net/minecraft/resources/Identifier#getPath` | method | 6 |
| `net/minecraft/resources/Identifier#parse` | method | 7 |
| `net/minecraft/resources/Identifier#toString` | method | 7 |
| `net/minecraft/resources/Identifier#withDefaultNamespace` | method | 2 |
| `net/minecraft/resources/Identifier#withPath` | method | 1 |
| `net/minecraft/resources/ResourceKey#identifier` | method | 3 |
| `net/minecraft/server/MinecraftServer#isPublished` | method | 1 |
| `net/minecraft/server/packs/PackType#CLIENT_RESOURCES` | field | 1 |
| `net/minecraft/server/packs/resources/PreparableReloadListener$PreparationBarrier#wait` | method | 1 |
| `net/minecraft/server/packs/resources/PreparableReloadListener$SharedState#resourceManager` | method | 1 |
| `net/minecraft/server/packs/resources/Resource#open` | method | 1 |
| `net/minecraft/server/packs/resources/Resource#openAsReader` | method | 1 |
| `net/minecraft/server/packs/resources/Resource#sourcePackId` | method | 1 |
| `net/minecraft/server/packs/resources/ResourceManager#getResourceStack` | method | 1 |
| `net/minecraft/sounds/SoundEvent#location` | method | 5 |
| `net/minecraft/sounds/SoundEvents#GRASS_STEP` | field | 1 |
| `net/minecraft/sounds/SoundEvents#PLAYER_BIG_FALL` | field | 1 |
| `net/minecraft/sounds/SoundEvents#PLAYER_SMALL_FALL` | field | 1 |
| `net/minecraft/sounds/SoundEvents#PLAYER_SPLASH` | field | 1 |
| `net/minecraft/sounds/SoundEvents#PLAYER_SPLASH_HIGH_SPEED` | field | 1 |
| `net/minecraft/sounds/SoundEvents#PLAYER_SWIM` | field | 1 |
| `net/minecraft/sounds/SoundSource#HOSTILE` | field | 1 |
| `net/minecraft/sounds/SoundSource#NEUTRAL` | field | 1 |
| `net/minecraft/sounds/SoundSource#PLAYERS` | field | 2 |
| `net/minecraft/tags/FluidTags#LAVA` | field | 2 |
| `net/minecraft/tags/FluidTags#WATER` | field | 2 |
| `net/minecraft/tags/TagKey#create` | method | 1 |
| `net/minecraft/tags/TagKey#location` | method | 1 |
| `net/minecraft/util/GsonHelper#getAsJsonArray` | method | 1 |
| `net/minecraft/util/GsonHelper#getAsJsonObject` | method | 1 |
| `net/minecraft/util/GsonHelper#getAsLong` | method | 1 |
| `net/minecraft/util/GsonHelper#getAsString` | method | 1 |
| `net/minecraft/util/Mth#equal` | method | 1 |
| `net/minecraft/util/RandomSource#nextInt` | method | 1 |
| `net/minecraft/util/StrictJsonParser#parse` | method | 1 |
| `net/minecraft/util/StringRepresentable#fromEnum` | method | 1 |
| `net/minecraft/util/StringRepresentable#keys` | method | 1 |
| `net/minecraft/util/Util#ioPool` | method | 2 |
| `net/minecraft/util/Util#memoize` | method | 2 |
| `net/minecraft/util/profiling/Profiler#get` | method | 1 |
| `net/minecraft/util/profiling/ProfilerFiller#pop` | method | 1 |
| `net/minecraft/util/profiling/ProfilerFiller#push` | method | 1 |
| `net/minecraft/world/entity/Entity#<init>` | method | 1 |
| `net/minecraft/world/entity/Entity#blockPosition` | method | 1 |
| `net/minecraft/world/entity/Entity#canCollideWith` | method | 1 |
| `net/minecraft/world/entity/Entity#distanceToSqr` | method | 1 |
| `net/minecraft/world/entity/Entity#fillCrashReportCategory` | method | 1 |
| `net/minecraft/world/entity/Entity#getBoundingBox` | method | 3 |
| `net/minecraft/world/entity/Entity#getSoundSource` | method | 1 |
| `net/minecraft/world/entity/Entity#getType` | method | 4 |
| `net/minecraft/world/entity/Entity#getUUID` | method | 1 |
| `net/minecraft/world/entity/Entity#isPassenger` | method | 1 |
| `net/minecraft/world/entity/Entity#isSpectator` | method | 1 |
| `net/minecraft/world/entity/Entity#isSprinting` | method | 1 |
| `net/minecraft/world/entity/Entity#level` | method | 2 |
| `net/minecraft/world/entity/Entity#walkingStepSound` | method | 1 |
| `net/minecraft/world/entity/EntityDimensions#width` | method | 1 |
| `net/minecraft/world/entity/EntitySpawnReason#CHUNK_GENERATION` | field | 1 |
| `net/minecraft/world/entity/EntityType#getDimensions` | method | 1 |
| `net/minecraft/world/entity/EntityType#getKey` | method | 3 |
| `net/minecraft/world/entity/EntityTypes#PLAYER` | field | 3 |
| `net/minecraft/world/entity/EquipmentSlot#FEET` | field | 2 |
| `net/minecraft/world/entity/LivingEntity#blockPosition` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#fallDistance` | field | 1 |
| `net/minecraft/world/entity/LivingEntity#getBbWidth` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#getBoundingBox` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#getDeltaMovement` | method | 4 |
| `net/minecraft/world/entity/LivingEntity#getItemBySlot` | method | 2 |
| `net/minecraft/world/entity/LivingEntity#getLastClimbablePos` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#getRootVehicle` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#getType` | method | 3 |
| `net/minecraft/world/entity/LivingEntity#getX` | method | 2 |
| `net/minecraft/world/entity/LivingEntity#getY` | method | 3 |
| `net/minecraft/world/entity/LivingEntity#getYRot` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#getZ` | method | 2 |
| `net/minecraft/world/entity/LivingEntity#isDiscrete` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#isInWater` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#isPassenger` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#isSleeping` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#isSprinting` | method | 2 |
| `net/minecraft/world/entity/LivingEntity#isSuppressingSlidingDownLadder` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#isUnderWater` | method | 2 |
| `net/minecraft/world/entity/LivingEntity#jumping` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#level` | method | 7 |
| `net/minecraft/world/entity/LivingEntity#moveDist` | field | 1 |
| `net/minecraft/world/entity/LivingEntity#position` | method | 2 |
| `net/minecraft/world/entity/LivingEntity#xxa` | field | 1 |
| `net/minecraft/world/entity/player/Abilities#flying` | field | 1 |
| `net/minecraft/world/entity/player/Player#getUUID` | method | 1 |
| `net/minecraft/world/item/ItemStack#get` | method | 1 |
| `net/minecraft/world/item/ItemStack#isEmpty` | method | 2 |
| `net/minecraft/world/item/equipment/Equippable#equipSound` | method | 1 |
| `net/minecraft/world/level/Level#addAlwaysVisibleParticle` | method | 1 |
| `net/minecraft/world/level/Level#getBiome` | method | 1 |
| `net/minecraft/world/level/Level#getBlockState` | method | 2 |
| `net/minecraft/world/level/Level#getEntities` | method | 2 |
| `net/minecraft/world/level/Level#getGameTime` | method | 2 |
| `net/minecraft/world/level/Level#getRandom` | method | 1 |
| `net/minecraft/world/level/Level#getServer` | method | 1 |
| `net/minecraft/world/level/biome/Biome#getPrecipitationAt` | method | 1 |
| `net/minecraft/world/level/biome/Biome$Precipitation#RAIN` | field | 1 |
| `net/minecraft/world/level/block/Block#defaultBlockState` | method | 4 |
| `net/minecraft/world/level/block/Blocks#AIR` | field | 2 |
| `net/minecraft/world/level/block/Blocks#SNOW` | field | 1 |
| `net/minecraft/world/level/block/SoundType#getPitch` | method | 1 |
| `net/minecraft/world/level/block/SoundType#getStepSound` | method | 7 |
| `net/minecraft/world/level/block/SoundType#getVolume` | method | 1 |
| `net/minecraft/world/level/block/SoundType#pitch` | field | 2 |
| `net/minecraft/world/level/block/SoundType#volume` | field | 2 |
| `net/minecraft/world/level/block/StairBlock#baseState` | method | 1 |
| `net/minecraft/world/level/block/state/BlockBehaviour#<init>` | method | 1 |
| `net/minecraft/world/level/block/state/BlockBehaviour$Properties#ofLegacyCopy` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#getAppearance` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#getBlock` | method | 3 |
| `net/minecraft/world/level/block/state/BlockState#getFluidState` | method | 2 |
| `net/minecraft/world/level/block/state/BlockState#getProperties` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#getSoundType` | method | 6 |
| `net/minecraft/world/level/block/state/BlockState#getValue` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#is` | method | 2 |
| `net/minecraft/world/level/block/state/BlockState#isAir` | method | 4 |
| `net/minecraft/world/level/block/state/BlockState#isFaceSturdy` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#liquid` | method | 2 |
| `net/minecraft/world/level/block/state/BlockState#typeHolder` | method | 2 |
| `net/minecraft/world/level/block/state/properties/Property#getName` | method | 1 |
| `net/minecraft/world/level/material/FluidState#is` | method | 2 |
| `net/minecraft/world/phys/AABB#<init>` | method | 1 |
| `net/minecraft/world/phys/AABB#inflate` | method | 4 |
| `net/minecraft/world/phys/AABB#max` | method | 1 |
| `net/minecraft/world/phys/AABB#maxY` | field | 1 |
| `net/minecraft/world/phys/AABB#min` | method | 1 |
| `net/minecraft/world/phys/AABB#minY` | field | 1 |
| `net/minecraft/world/phys/AABB#move` | method | 2 |
| `net/minecraft/world/phys/BlockHitResult#getBlockPos` | method | 1 |
| `net/minecraft/world/phys/BlockHitResult#getType` | method | 1 |
| `net/minecraft/world/phys/EntityHitResult#getEntity` | method | 1 |
| `net/minecraft/world/phys/HitResult$Type#BLOCK` | field | 1 |
| `net/minecraft/world/phys/Vec3#distanceToSqr` | method | 1 |
| `net/minecraft/world/phys/Vec3#length` | method | 1 |
| `net/minecraft/world/phys/Vec3#lengthSqr` | method | 1 |
| `net/minecraft/world/phys/Vec3#x` | field | 3 |
| `net/minecraft/world/phys/Vec3#y` | field | 2 |
| `net/minecraft/world/phys/Vec3#z` | field | 3 |
| `net/minecraft/world/phys/shapes/VoxelShape#bounds` | method | 1 |
| `net/minecraft/world/phys/shapes/VoxelShape#isEmpty` | method | 1 |
| `net/minecraft/world/phys/shapes/VoxelShape#max` | method | 1 |

</details>

