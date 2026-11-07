# Horizon port report: AmbientSounds

`ambientsounds` 6.3.11 · AmbientSounds_FABRIC_v6.3.11_mc26.3.jar · 66 classes

**Partly convertible**

| Status | Count | Meaning |
|---|---|---|
| auto | 0 | The Horizon hook or injection is known: the wiring can be generated. The Java body still has to become Rust. |
| content | 1 | Data and assets: horizon-port-pack carries these into a content mod (.hmod). |
| needs-mapping | 1 | The kind of change carries over, but the matching Horizon function isn't in the table yet. Run `match`. |
| needs-translation | 3 | No direct equivalent: a person (or an AI agent) has to rewrite this part. |
| unsupported | 0 | Horizon has nothing this can attach to yet. |

## Entry points

- `client`: team.creative.ambientsounds.AmbientSounds

## Mixins

| Injection | In | Horizon | Status | Note |
|---|---|---|---|---|
| mixin into JOrbisAudioStream with no injectors (interfaces or @Shadow only) | `OggAudioStreamMixin` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| @Accessor on SoundBufferLibrary.resourceManager | `SoundBufferLibraryAccessor.getResourceManager` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for SoundBufferLibrary.resourceManager: run `match`. |

## Classes extending the game

| Class | In | Status | Note |
|---|---|---|---|
| extends SimplePreparableReloadListener | `AmbientSounds$1` | needs-translation | No mapping for this superclass yet. |
| extends BlockPos | `AirPocketScanner$BlockPosInspection` | needs-translation | No mapping for this superclass yet. |

## Content

| What | In | Status | Note |
|---|---|---|---|
| 203 asset files, 0 data files | `jar resources` | content | Recipes, loot, tags, models, textures, worldgen: horizon-port-pack carries these. |

## Game methods and fields used

15 of 172 are mapped to Horizon. Every Java body that uses an unmapped one needs it mapped before its Rust can be written.

<details><summary>Mapped</summary>

| Java | Horizon |
|---|---|
| `net/minecraft/client/Minecraft#font` | `(implicit)` |
| `net/minecraft/client/Minecraft#getInstance` | `self` |
| `net/minecraft/client/Minecraft#player` | `self.native_player()` |
| `net/minecraft/client/sounds/SoundManager#isActive` | horizon_client_render::sound_engine::is_active (suggested) |
| `net/minecraft/client/sounds/WeighedSoundEvents#getSound` | horizon_client_render::sound_events::SoundRegistry::sound (suggested) |
| `net/minecraft/core/Direction#getOpposite` | horizon_client::mobs::shulker::opposite (suggested) |
| `net/minecraft/nbt/NbtUtils#compareNbt` | horizon_client_core::nbt_path::compare_nbt (suggested) |
| `net/minecraft/util/Mth#clamp` | horizon_client_render::chat::clamp (suggested) |
| `net/minecraft/world/entity/Entity#getXRot` | `self.native_player()?.pitch` |
| `net/minecraft/world/entity/Entity#getYRot` | `self.native_player()?.yaw` |
| `net/minecraft/world/level/Level#getMaxY` | horizon_client::game::Dim::max_y (suggested) |
| `net/minecraft/world/level/Level#getMinY` | horizon_client::game::Dim::min_y (suggested) |
| `net/minecraft/world/level/Level#isRainingAt` | horizon_world::level::weather::Behaviours::is_raining_at (suggested) |
| `net/minecraft/world/level/biome/Biome#coldEnoughToSnow` | horizon_world::worldgen::structure::start::cold_enough_to_snow (suggested) |
| `net/minecraft/world/level/block/state/BlockState#canOcclude` | horizon_world::worldgen::placement::context::can_occlude (suggested) |

</details>

<details><summary>Not mapped yet</summary>

| Java | Kind | Classes using it |
|---|---|---|
| `net/minecraft/ChatFormatting#DARK_GREEN` | field | 1 |
| `net/minecraft/ChatFormatting#RESET` | field | 1 |
| `net/minecraft/client/DeltaTracker#getGameTimeDeltaPartialTick` | method | 1 |
| `net/minecraft/client/Minecraft#getDeltaTracker` | method | 1 |
| `net/minecraft/client/Minecraft#getResourceManager` | method | 1 |
| `net/minecraft/client/Minecraft#getSoundManager` | method | 1 |
| `net/minecraft/client/Minecraft#isPaused` | method | 1 |
| `net/minecraft/client/Minecraft#level` | field | 1 |
| `net/minecraft/client/Minecraft#options` | field | 1 |
| `net/minecraft/client/Options#getSoundSourceVolume` | method | 1 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#nextStratum` | method | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#dimension` | method | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#entitiesForRendering` | method | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#registryAccess` | method | 1 |
| `net/minecraft/client/resources/sounds/SoundInstance$Attenuation#NONE` | field | 1 |
| `net/minecraft/client/sounds/JOrbisAudioStream#<init>` | method | 2 |
| `net/minecraft/client/sounds/JOrbisAudioStream#readChunk` | method | 1 |
| `net/minecraft/client/sounds/LoopingAudioStream#<init>` | method | 1 |
| `net/minecraft/client/sounds/SoundBufferLibrary#resourceManager` | method | 1 |
| `net/minecraft/client/sounds/SoundManager#EMPTY_SOUND` | field | 1 |
| `net/minecraft/client/sounds/SoundManager#INTENTIONALLY_EMPTY_SOUND` | field | 1 |
| `net/minecraft/client/sounds/SoundManager#INTENTIONALLY_EMPTY_SOUND_EVENT` | field | 1 |
| `net/minecraft/client/sounds/SoundManager#INTENTIONALLY_EMPTY_SOUND_LOCATION` | field | 1 |
| `net/minecraft/client/sounds/SoundManager#getSoundEvent` | method | 1 |
| `net/minecraft/client/sounds/SoundManager#play` | method | 1 |
| `net/minecraft/client/sounds/SoundManager#stop` | method | 1 |
| `net/minecraft/core/BlockPos#<init>` | method | 1 |
| `net/minecraft/core/BlockPos#above` | method | 1 |
| `net/minecraft/core/BlockPos#containing` | method | 3 |
| `net/minecraft/core/BlockPos#distSqr` | method | 1 |
| `net/minecraft/core/BlockPos#get` | method | 1 |
| `net/minecraft/core/BlockPos#getX` | method | 4 |
| `net/minecraft/core/BlockPos#getY` | method | 4 |
| `net/minecraft/core/BlockPos#getZ` | method | 4 |
| `net/minecraft/core/BlockPos$MutableBlockPos#<init>` | method | 3 |
| `net/minecraft/core/BlockPos$MutableBlockPos#getX` | method | 1 |
| `net/minecraft/core/BlockPos$MutableBlockPos#getY` | method | 1 |
| `net/minecraft/core/BlockPos$MutableBlockPos#getZ` | method | 1 |
| `net/minecraft/core/BlockPos$MutableBlockPos#set` | method | 3 |
| `net/minecraft/core/BlockPos$MutableBlockPos#setY` | method | 2 |
| `net/minecraft/core/DefaultedRegistry#getOptional` | method | 1 |
| `net/minecraft/core/Direction#DOWN` | field | 2 |
| `net/minecraft/core/Direction#EAST` | field | 2 |
| `net/minecraft/core/Direction#NORTH` | field | 2 |
| `net/minecraft/core/Direction#SOUTH` | field | 2 |
| `net/minecraft/core/Direction#UP` | field | 2 |
| `net/minecraft/core/Direction#WEST` | field | 2 |
| `net/minecraft/core/Direction#getAxis` | method | 1 |
| `net/minecraft/core/Direction#getAxisDirection` | method | 1 |
| `net/minecraft/core/Direction#ordinal` | method | 2 |
| `net/minecraft/core/Direction#values` | method | 2 |
| `net/minecraft/core/Direction$AxisDirection#POSITIVE` | field | 1 |
| `net/minecraft/core/Holder#equals` | method | 1 |
| `net/minecraft/core/Holder#hashCode` | method | 1 |
| `net/minecraft/core/Holder#is` | method | 1 |
| `net/minecraft/core/Holder#tags` | method | 1 |
| `net/minecraft/core/Holder#unwrapKey` | method | 1 |
| `net/minecraft/core/Holder#value` | method | 3 |
| `net/minecraft/core/SectionPos#blockToSectionCoord` | method | 1 |
| `net/minecraft/core/SectionPos#sectionToBlockCoord` | method | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#ENTITY_TYPE` | field | 1 |
| `net/minecraft/core/registries/Registries#BLOCK` | field | 1 |
| `net/minecraft/core/registries/Registries#FLUID` | field | 1 |
| `net/minecraft/nbt/TagParser#parseCompoundFully` | method | 1 |
| `net/minecraft/network/chat/Component#getString` | method | 1 |
| `net/minecraft/resources/Identifier#equals` | method | 1 |
| `net/minecraft/resources/Identifier#getNamespace` | method | 1 |
| `net/minecraft/resources/Identifier#getPath` | method | 1 |
| `net/minecraft/resources/Identifier#parse` | method | 3 |
| `net/minecraft/resources/Identifier#toString` | method | 5 |
| `net/minecraft/resources/Identifier#tryBuild` | method | 2 |
| `net/minecraft/resources/Identifier#tryParse` | method | 1 |
| `net/minecraft/resources/ResourceKey#identifier` | method | 4 |
| `net/minecraft/server/packs/PackType#CLIENT_RESOURCES` | field | 1 |
| `net/minecraft/server/packs/resources/Resource#open` | method | 2 |
| `net/minecraft/server/packs/resources/Resource#sourcePackId` | method | 1 |
| `net/minecraft/server/packs/resources/ResourceManager#getResource` | method | 1 |
| `net/minecraft/server/packs/resources/ResourceManager#listResourceStacks` | method | 1 |
| `net/minecraft/server/packs/resources/ResourceProvider#getResourceOrThrow` | method | 1 |
| `net/minecraft/server/packs/resources/SimplePreparableReloadListener#<init>` | method | 1 |
| `net/minecraft/sounds/SoundSource#AMBIENT` | field | 2 |
| `net/minecraft/sounds/SoundSource#MASTER` | field | 1 |
| `net/minecraft/sounds/SoundSource#getName` | method | 1 |
| `net/minecraft/sounds/SoundSource#values` | method | 1 |
| `net/minecraft/tags/BlockTags#LEAVES` | field | 1 |
| `net/minecraft/tags/FluidTags#WATER` | field | 1 |
| `net/minecraft/tags/TagKey#create` | method | 2 |
| `net/minecraft/tags/TagKey#location` | method | 1 |
| `net/minecraft/util/GsonHelper#convertToString` | method | 1 |
| `net/minecraft/util/ProblemReporter#DISCARDING` | field | 1 |
| `net/minecraft/util/RandomSource#createThreadLocalInstance` | method | 1 |
| `net/minecraft/util/Util#backgroundExecutor` | method | 1 |
| `net/minecraft/util/Util#nonCriticalIoPool` | method | 1 |
| `net/minecraft/world/attribute/EnvironmentAttributeSystem#getValue` | method | 1 |
| `net/minecraft/world/attribute/EnvironmentAttributes#SUN_ANGLE` | field | 1 |
| `net/minecraft/world/entity/Entity#distanceToSqr` | method | 1 |
| `net/minecraft/world/entity/Entity#entityTags` | method | 1 |
| `net/minecraft/world/entity/Entity#getCustomName` | method | 1 |
| `net/minecraft/world/entity/Entity#getStringUUID` | method | 1 |
| `net/minecraft/world/entity/Entity#getTeam` | method | 1 |
| `net/minecraft/world/entity/Entity#getType` | method | 1 |
| `net/minecraft/world/entity/Entity#hasCustomName` | method | 1 |
| `net/minecraft/world/entity/Entity#position` | method | 1 |
| `net/minecraft/world/entity/Entity#registryAccess` | method | 1 |
| `net/minecraft/world/entity/Entity#saveWithoutId` | method | 1 |
| `net/minecraft/world/entity/player/Player#blockPosition` | method | 3 |
| `net/minecraft/world/entity/player/Player#getEyePosition` | method | 4 |
| `net/minecraft/world/entity/player/Player#getEyeY` | method | 1 |
| `net/minecraft/world/entity/player/Player#getGameProfile` | method | 1 |
| `net/minecraft/world/entity/player/Player#isEyeInFluid` | method | 1 |
| `net/minecraft/world/entity/player/Player#level` | method | 1 |
| `net/minecraft/world/entity/player/Player#position` | method | 1 |
| `net/minecraft/world/level/Level#dimension` | method | 2 |
| `net/minecraft/world/level/Level#environmentAttributes` | method | 1 |
| `net/minecraft/world/level/Level#getBiome` | method | 3 |
| `net/minecraft/world/level/Level#getBlockState` | method | 2 |
| `net/minecraft/world/level/Level#getBrightness` | method | 1 |
| `net/minecraft/world/level/Level#getChunk` | method | 1 |
| `net/minecraft/world/level/Level#getFluidState` | method | 2 |
| `net/minecraft/world/level/Level#getMinSectionY` | method | 1 |
| `net/minecraft/world/level/Level#getSeaLevel` | method | 3 |
| `net/minecraft/world/level/Level#hasChunk` | method | 1 |
| `net/minecraft/world/level/Level#isRaining` | method | 2 |
| `net/minecraft/world/level/Level#isThundering` | method | 1 |
| `net/minecraft/world/level/LevelAccessor#isClientSide` | method | 1 |
| `net/minecraft/world/level/LevelReader#canSeeSky` | method | 1 |
| `net/minecraft/world/level/LevelReader#getBlockState` | method | 1 |
| `net/minecraft/world/level/LevelReader#getHeight` | method | 1 |
| `net/minecraft/world/level/LightLayer#BLOCK` | field | 1 |
| `net/minecraft/world/level/LightLayer#SKY` | field | 1 |
| `net/minecraft/world/level/biome/Biome#getBaseTemperature` | method | 1 |
| `net/minecraft/world/level/biome/Biome#getPrecipitationAt` | method | 1 |
| `net/minecraft/world/level/biome/Biome$Precipitation#RAIN` | field | 1 |
| `net/minecraft/world/level/block/Block#defaultBlockState` | method | 1 |
| `net/minecraft/world/level/block/Block#getStateDefinition` | method | 1 |
| `net/minecraft/world/level/block/Blocks#AIR` | field | 1 |
| `net/minecraft/world/level/block/Blocks#VOID_AIR` | field | 1 |
| `net/minecraft/world/level/block/state/BlockState#getBlock` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#getFluidState` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#getValue` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#is` | method | 2 |
| `net/minecraft/world/level/block/state/BlockState#isAir` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#isCollisionShapeFullBlock` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#isSolidRender` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#typeHolder` | method | 1 |
| `net/minecraft/world/level/block/state/StateDefinition#getProperty` | method | 1 |
| `net/minecraft/world/level/block/state/properties/Property#getValue` | method | 1 |
| `net/minecraft/world/level/chunk/LevelChunk#getSections` | method | 1 |
| `net/minecraft/world/level/chunk/LevelChunkSection#maybeHas` | method | 1 |
| `net/minecraft/world/level/levelgen/Heightmap$Types#WORLD_SURFACE` | field | 1 |
| `net/minecraft/world/level/material/FluidState#is` | method | 2 |
| `net/minecraft/world/level/storage/TagValueOutput#buildResult` | method | 1 |
| `net/minecraft/world/level/storage/TagValueOutput#createWithContext` | method | 1 |
| `net/minecraft/world/phys/Vec3#x` | field | 2 |
| `net/minecraft/world/phys/Vec3#y` | field | 2 |
| `net/minecraft/world/phys/Vec3#z` | field | 2 |
| `net/minecraft/world/scores/PlayerTeam#getName` | method | 1 |

</details>

