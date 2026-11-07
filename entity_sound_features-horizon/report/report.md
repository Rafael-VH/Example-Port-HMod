# Horizon port report: Entity Sound Features

`entity_sound_features` 0.8.2 · entity_sound_features-0.8.2-26.1-fabric.jar · 23 classes

**Partly convertible**

| Status | Count | Meaning |
|---|---|---|
| auto | 1 | The Horizon hook or injection is known: the wiring can be generated. The Java body still has to become Rust. |
| content | 1 | Data and assets: horizon-port-pack carries these into a content mod (.hmod). |
| needs-mapping | 5 | The kind of change carries over, but the matching Horizon function isn't in the table yet. Run `match`. |
| needs-translation | 2 | No direct equivalent: a person (or an AI agent) has to rewrite this part. |
| unsupported | 0 | Horizon has nothing this can attach to yet. |

## Entry points

- `client`: traben.entity_sound_features.ESFInit
- `modmenu`: traben.entity_sound_features.ESFModMenu

## Mixins

| Injection | In | Horizon | Status | Note |
|---|---|---|---|---|
| @Inject(HEAD) on ClientLevel.playSound | `MixinClientLevel.esf$discoverEntity` | HEAD | needs-mapping | No Horizon function known for ClientLevel.playSound: run `match`. |
| @Inject(TAIL) on ClientLevel.playSound | `MixinClientLevel.esf$clearEntity` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for ClientLevel.playSound: run `match`. |
| @Inject(HEAD) on Minecraft.reloadResourcePacks | `MixinResourceReload.emf$reloadStart` | HEAD | needs-mapping | No Horizon function known for Minecraft.reloadResourcePacks: run `match`. |
| @Inject(TAIL) on Preparations.listResources | `MixinSoundManager$Preparations.esf$addESFDirectorySounds` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for Preparations.listResources: run `match`. |
| @Inject(TAIL) on WeighedSoundEvents.<init> | `MixinWeighedSoundEvents.esf$init` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for WeighedSoundEvents.<init>: run `match`. |
| @Inject(RETURN) on WeighedSoundEvents.getSound | `MixinWeighedSoundEvents.esf$soundModify` | RETURN on horizon_client_render::sound_events::SoundRegistry::sound | auto | Horizon's RETURN runs at every normal exit and passes the return value on. |

## Classes extending the game

| Class | In | Status | Note |
|---|---|---|---|
| extends Sound | `ESFSound` | needs-translation | No mapping for this superclass yet. |

## Access wideners

| File | In | Status | Note |
|---|---|---|---|
| access widener (opens private game fields) | `entity_sound_features_12.accesswidener` | needs-translation | Native helpers already see the game's fields, once each field is mapped. |

## Content

| What | In | Status | Note |
|---|---|---|---|
| 5 asset files, 0 data files | `jar resources` | content | Recipes, loot, tags, models, textures, worldgen: horizon-port-pack carries these. |

## Game methods and fields used

8 of 78 are mapped to Horizon. Every Java body that uses an unmapped one needs it mapped before its Rust can be written.

<details><summary>Mapped</summary>

| Java | Horizon |
|---|---|
| `net/minecraft/client/Minecraft#getInstance` | `self` |
| `net/minecraft/client/Minecraft#player` | `self.native_player()` |
| `net/minecraft/client/resources/sounds/Sound#getAttenuationDistance` | horizon_client_render::sound_events::Sound::attenuation_distance (suggested) |
| `net/minecraft/client/resources/sounds/Sound#getPath` | horizon_client_render::sound_events::Sound::path (suggested) |
| `net/minecraft/client/sounds/WeighedSoundEvents#getSound` | horizon_client_render::sound_events::SoundRegistry::sound (suggested) |
| `net/minecraft/client/sounds/WeighedSoundEvents#getWeight` | horizon_client_render::sound_events::SoundRegistry::weight (suggested) |
| `net/minecraft/util/Mth#clamp` | horizon_client_render::chat::clamp (suggested) |
| `net/minecraft/world/phys/AABB#intersects` | horizon_core::geometry::Aabb::intersects_corners (suggested) |

</details>

<details><summary>Not mapped yet</summary>

| Java | Kind | Classes using it |
|---|---|---|
| `net/minecraft/client/Minecraft#getResourceManager` | method | 1 |
| `net/minecraft/client/Minecraft#getSoundManager` | method | 3 |
| `net/minecraft/client/Minecraft#reloadResourcePacks` | method | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#playSound` | method | 2 |
| `net/minecraft/client/renderer/blockentity/BlockEntityRenderers#PROVIDERS` | field | 1 |
| `net/minecraft/client/renderer/entity/state/EntityRenderState#<init>` | method | 1 |
| `net/minecraft/client/resources/sounds/Sound#<init>` | method | 2 |
| `net/minecraft/client/resources/sounds/Sound#getLocation` | method | 3 |
| `net/minecraft/client/resources/sounds/Sound#getPitch` | method | 2 |
| `net/minecraft/client/resources/sounds/Sound#getType` | method | 1 |
| `net/minecraft/client/resources/sounds/Sound#getVolume` | method | 2 |
| `net/minecraft/client/resources/sounds/Sound#getWeight` | method | 2 |
| `net/minecraft/client/resources/sounds/Sound#shouldPreload` | method | 2 |
| `net/minecraft/client/resources/sounds/Sound#shouldStream` | method | 2 |
| `net/minecraft/client/resources/sounds/Sound$Type#FILE` | field | 3 |
| `net/minecraft/client/resources/sounds/Sound$Type#SOUND_EVENT` | field | 1 |
| `net/minecraft/client/resources/sounds/Sound$Type#ordinal` | method | 2 |
| `net/minecraft/client/resources/sounds/Sound$Type#values` | method | 1 |
| `net/minecraft/client/resources/sounds/SoundEventRegistration#getSounds` | method | 1 |
| `net/minecraft/client/resources/sounds/SoundEventRegistration#getSubtitle` | method | 1 |
| `net/minecraft/client/resources/sounds/SoundInstance#getIdentifier` | method | 2 |
| `net/minecraft/client/sounds/SoundEngine#instanceToChannel` | field | 2 |
| `net/minecraft/client/sounds/SoundEngine#loaded` | field | 2 |
| `net/minecraft/client/sounds/SoundEngine#soundDeleteTime` | field | 2 |
| `net/minecraft/client/sounds/SoundEngine#tickCount` | field | 2 |
| `net/minecraft/client/sounds/SoundManager#EMPTY_SOUND` | field | 1 |
| `net/minecraft/client/sounds/SoundManager#GSON` | field | 1 |
| `net/minecraft/client/sounds/SoundManager#getSoundEvent` | method | 2 |
| `net/minecraft/client/sounds/SoundManager#soundEngine` | field | 2 |
| `net/minecraft/client/sounds/SoundManager$Preparations#listResources` | method | 1 |
| `net/minecraft/client/sounds/WeighedSoundEvents#<init>` | method | 2 |
| `net/minecraft/client/sounds/WeighedSoundEvents#addSound` | method | 1 |
| `net/minecraft/client/sounds/WeighedSoundEvents#preloadIfRequired` | method | 1 |
| `net/minecraft/core/BlockPos#<init>` | method | 1 |
| `net/minecraft/core/BlockPos#toShortString` | method | 1 |
| `net/minecraft/core/DefaultedRegistry#forEach` | method | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#ENTITY_TYPE` | field | 1 |
| `net/minecraft/network/chat/CommonComponents#OPTION_OFF` | field | 1 |
| `net/minecraft/network/chat/Component#getString` | method | 1 |
| `net/minecraft/network/chat/Component#translatable` | method | 2 |
| `net/minecraft/network/chat/MutableComponent#getString` | method | 2 |
| `net/minecraft/resources/FileToIdConverter#<init>` | method | 1 |
| `net/minecraft/resources/FileToIdConverter#listMatchingResources` | method | 1 |
| `net/minecraft/resources/Identifier#equals` | method | 1 |
| `net/minecraft/resources/Identifier#getNamespace` | method | 2 |
| `net/minecraft/resources/Identifier#getPath` | method | 2 |
| `net/minecraft/resources/Identifier#toString` | method | 2 |
| `net/minecraft/resources/Identifier#tryParse` | method | 1 |
| `net/minecraft/server/packs/resources/Resource#openAsReader` | method | 1 |
| `net/minecraft/server/packs/resources/ResourceManager#getResource` | method | 1 |
| `net/minecraft/sounds/SoundEvent#createFixedRangeEvent` | method | 1 |
| `net/minecraft/sounds/SoundEvent#location` | method | 1 |
| `net/minecraft/sounds/SoundSource#BLOCKS` | field | 1 |
| `net/minecraft/util/GsonHelper#fromJson` | method | 1 |
| `net/minecraft/util/RandomSource#create` | method | 1 |
| `net/minecraft/util/valueproviders/ConstantFloat#of` | method | 1 |
| `net/minecraft/util/valueproviders/MultipliedFloats#<init>` | method | 1 |
| `net/minecraft/world/entity/Entity#distanceToSqr` | method | 1 |
| `net/minecraft/world/entity/Entity#getBoundingBox` | method | 1 |
| `net/minecraft/world/entity/Entity#getSoundSource` | method | 1 |
| `net/minecraft/world/entity/EntityType#getDescriptionId` | method | 1 |
| `net/minecraft/world/level/Level#<init>` | method | 1 |
| `net/minecraft/world/level/Level#getBlockEntity` | method | 1 |
| `net/minecraft/world/level/Level#getBlockState` | method | 1 |
| `net/minecraft/world/level/Level#getGameTime` | method | 1 |
| `net/minecraft/world/level/Level#playSound` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#hasBlockEntity` | method | 1 |
| `net/minecraft/world/level/entity/LevelEntityGetter#getAll` | method | 1 |
| `net/minecraft/world/phys/AABB#contains` | method | 1 |
| `net/minecraft/world/phys/Vec3#<init>` | method | 1 |

</details>

