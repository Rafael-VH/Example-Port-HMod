# Horizon port report: Sound Physics Remastered

`sound_physics_remastered` 1.5.2+26.3 · sound-physics-remastered-fabric-1.5.2+26.3.jar · 90 classes

**Partly convertible**

| Status | Count | Meaning |
|---|---|---|
| auto | 1 | The Horizon hook or injection is known: the wiring can be generated. The Java body still has to become Rust. |
| content | 1 | Data and assets: horizon-port-pack carries these into a content mod (.hmod). |
| needs-mapping | 16 | The kind of change carries over, but the matching Horizon function isn't in the table yet. Run `match`. |
| needs-translation | 1 | No direct equivalent: a person (or an AI agent) has to rewrite this part. |
| unsupported | 0 | Horizon has nothing this can attach to yet. |

## Entry points

- `main`: com.sonicether.soundphysics.FabricSoundPhysicsMod
- `client`: com.sonicether.soundphysics.FabricSoundPhysicsMod
- `modmenu`: com.sonicether.soundphysics.integration.ModMenuIntegration
- `voicechat`: com.sonicether.soundphysics.integration.voicechat.SimpleVoiceChatPlugin

## Mixins

| Injection | In | Horizon | Status | Note |
|---|---|---|---|---|
| @Accessor on Channel.source | `ChannelAccessor.getSource` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for Channel.source: run `match`. |
| @Inject(TAIL) on ClientLevel.tick | `ClientLevelMixin.tick` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for ClientLevel.tick: run `match`. |
| @Inject(TAIL) on ClientPacketListener.handleLogin | `ClientPacketListenerMixin.handleLogin` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for ClientPacketListener.handleLogin: run `match`. |
| @Inject(RETURN) on DebugRenderer.refreshRendererList | `DebugRendererMixin.refreshRendererList` | RETURN | needs-mapping | Horizon's RETURN runs at every normal exit and passes the return value on. No Horizon function known for DebugRenderer.refreshRendererList: run `match`. |
| @ModifyArg(INVOKE) on Entity.playSound at Lnet/minecraft/world/level/Level;playSound(Lnet/minecraft/world/entity/Entity;DDDLnet/minecraft/sounds/SoundEvent;Lnet/minecraft/sounds/SoundSource;FF)V | `EntityMixin.playSound` | MODIFY_ARG on horizon_client::game::consume::<Emitter<'_, '_> as EmissionWorld>::play_sound | needs-mapping | The call it hooks (Level.playSound) isn't mapped to a Rust function yet. |
| @ModifyArgs(INVOKE) on Library.init at Lorg/lwjgl/openal/ALC10;alcCreateContext(JLjava/nio/IntBuffer;)J | `LibraryMixin.modifyContext` | MODIFY_ARGS | needs-mapping | No Horizon function known for Library.init: run `match`. The call it hooks (ALC10.alcCreateContext) isn't mapped to a Rust function yet. |
| @Inject(HEAD) on LocalPlayer.playSound | `LocalPlayerMixin.playSound` | HEAD on horizon_client::game::consume::<Emitter<'_, '_> as EmissionWorld>::play_sound | auto |  |
| @Inject(HEAD) on Minecraft.setLevel | `MinecraftMixin.setLevel` | HEAD | needs-mapping | No Horizon function known for Minecraft.setLevel: run `match`. |
| @Inject(HEAD) on Minecraft.disconnect | `MinecraftMixin.disconnect` | HEAD | needs-mapping | No Horizon function known for Minecraft.disconnect: run `match`. |
| @ModifyConstant on SoundEvent.getRange | `SoundEventMixin.allowance1` | MODIFY_CONSTANT | needs-mapping | Selects an exact Rust literal: pick it in the target function. No Horizon function known for SoundEvent.getRange: run `match`. |
| @Inject(INVOKE) on SoundEngine.loadLibrary at Lcom/mojang/blaze3d/audio/Listener;reset()V | `SoundSystemMixin.loadLibrary` | BEFORE_CALL | needs-mapping | Only whole statements, plain let initializers and block tails can be selected. No Horizon function known for SoundEngine.loadLibrary: run `match`. The call it hooks (Listener.reset) isn't mapped to a Rust function yet. |
| @ModifyArg(INVOKE) on SoundEngine.play at Lcom/google/common/collect/Multimap;put(Ljava/lang/Object;Ljava/lang/Object;)Z | `SoundSystemMixin.play` | MODIFY_ARG | needs-mapping | No Horizon function known for SoundEngine.play: run `match`. The call it hooks (Multimap.put) isn't mapped to a Rust function yet. |
| @Inject(INVOKE) on SoundEngine.tickInGameSound at Lnet/minecraft/client/sounds/ChannelAccess$ChannelHandle;isStopped()Z | `SoundSystemMixin.tickNonPaused` | BEFORE_CALL | needs-mapping | Only whole statements, plain let initializers and block tails can be selected. No Horizon function known for SoundEngine.tickInGameSound: run `match`. The call it hooks (ChannelHandle.isStopped) isn't mapped to a Rust function yet. |
| @Inject(HEAD) on Channel.setSelfPosition | `SourceMixin.setSelfPosition` | HEAD | needs-mapping | No Horizon function known for Channel.setSelfPosition: run `match`. |
| @Inject(HEAD) on Channel.play | `SourceMixin.play` | HEAD | needs-mapping | No Horizon function known for Channel.play: run `match`. |
| @ModifyVariable(HEAD) on Channel.linearAttenuation | `SourceMixin.injected` | MODIFY_VARIABLE | needs-mapping | Selects a Rust local by name: pick it in the target function. No Horizon function known for Channel.linearAttenuation: run `match`. |
| @Inject(RETURN) on Channel.linearAttenuation | `SourceMixin.linearAttenuation2` | RETURN | needs-mapping | Horizon's RETURN runs at every normal exit and passes the return value on. No Horizon function known for Channel.linearAttenuation: run `match`. |

## Classes extending the game

| Class | In | Status | Note |
|---|---|---|---|
| extends ChunkAccess | `ClonedLevelChunk` | needs-translation | No mapping for this superclass yet. |

## Content

| What | In | Status | Note |
|---|---|---|---|
| 7 asset files, 0 data files | `jar resources` | content | Recipes, loot, tags, models, textures, worldgen: horizon-port-pack carries these. |

## Game methods and fields used

15 of 271 are mapped to Horizon. Every Java body that uses an unmapped one needs it mapped before its Rust can be written.

<details><summary>Mapped</summary>

| Java | Horizon |
|---|---|
| `net/minecraft/client/Minecraft#getInstance` | `self` |
| `net/minecraft/client/Minecraft#player` | `self.native_player()` |
| `net/minecraft/client/multiplayer/ClientLevel#getHeight` | horizon_world::level::feature_world::<LevelFeatureWorld<'_, '_, L> as PlacementContext>::height (suggested) |
| `net/minecraft/client/multiplayer/ClientLevel#getMinY` | horizon_client::game::Dim::min_y (suggested) |
| `net/minecraft/client/player/LocalPlayer#isUnderWater` | horizon_client::motion::fluids::PlayerMotion::is_under_water (suggested) |
| `net/minecraft/client/player/LocalPlayer#playSound` | horizon_client::game::consume::<Emitter<'_, '_> as EmissionWorld>::play_sound (suggested) |
| `net/minecraft/core/registries/BuiltInRegistries#BLOCK` | content |
| `net/minecraft/gizmos/Gizmos#line` | horizon_client::gizmos::Gizmos::line (suggested) |
| `net/minecraft/network/chat/MutableComponent#append` | horizon_client::game::dedicated::append (suggested) |
| `net/minecraft/util/Mth#clamp` | horizon_client_render::chat::clamp (suggested) |
| `net/minecraft/util/Mth#hsvToArgb` | horizon_client_render::hud_layout::hsv_to_argb (suggested) |
| `net/minecraft/world/entity/Entity#playSound` | horizon_client::game::consume::<Emitter<'_, '_> as EmissionWorld>::play_sound (suggested) |
| `net/minecraft/world/level/Level#getHeight` | horizon_world::level::feature_world::<LevelFeatureWorld<'_, '_, L> as PlacementContext>::height (suggested) |
| `net/minecraft/world/level/Level#getMinY` | horizon_client::game::Dim::min_y (suggested) |
| `net/minecraft/world/phys/Vec3#normalize` | horizon_client::mobs::creaking::normalize (suggested) |

</details>

<details><summary>Not mapped yet</summary>

| Java | Kind | Classes using it |
|---|---|---|
| `com/google/common/collect/Multimap#put` | method | 1 |
| `com/mojang/blaze3d/audio/Channel#linearAttenuation` | method | 2 |
| `com/mojang/blaze3d/audio/Channel#play` | method | 1 |
| `com/mojang/blaze3d/audio/Channel#setSelfPosition` | method | 1 |
| `com/mojang/blaze3d/audio/Channel#source` | method | 1 |
| `com/mojang/blaze3d/audio/Library#init` | method | 1 |
| `com/mojang/blaze3d/audio/Listener#reset` | method | 1 |
| `net/minecraft/CrashReport#addCategory` | method | 1 |
| `net/minecraft/CrashReport#forThrowable` | method | 1 |
| `net/minecraft/CrashReportCategory#formatLocation` | method | 1 |
| `net/minecraft/CrashReportCategory#setDetail` | method | 1 |
| `net/minecraft/ReportedException#<init>` | method | 1 |
| `net/minecraft/client/Camera#position` | method | 1 |
| `net/minecraft/client/Minecraft#disconnect` | method | 1 |
| `net/minecraft/client/Minecraft#gameRenderer` | field | 1 |
| `net/minecraft/client/Minecraft#level` | field | 5 |
| `net/minecraft/client/Minecraft#setLevel` | method | 1 |
| `net/minecraft/client/multiplayer/ClientChunkCache#getChunk` | method | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#getBlockEntity` | method | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#getBlockState` | method | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#getChunkSource` | method | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#getFluidState` | method | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#getGameTime` | method | 4 |
| `net/minecraft/client/multiplayer/ClientLevel#tick` | method | 1 |
| `net/minecraft/client/multiplayer/ClientPacketListener#handleLogin` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#position` | method | 4 |
| `net/minecraft/client/renderer/GameRenderer#mainCamera` | method | 1 |
| `net/minecraft/client/renderer/debug/DebugRenderer#refreshRendererList` | method | 1 |
| `net/minecraft/client/resources/sounds/SoundInstance#getIdentifier` | method | 1 |
| `net/minecraft/client/resources/sounds/SoundInstance#getSource` | method | 1 |
| `net/minecraft/client/resources/sounds/SoundInstance#getX` | method | 1 |
| `net/minecraft/client/resources/sounds/SoundInstance#getY` | method | 1 |
| `net/minecraft/client/resources/sounds/SoundInstance#getZ` | method | 1 |
| `net/minecraft/client/resources/sounds/SoundInstance#hashCode` | method | 1 |
| `net/minecraft/client/sounds/ChannelAccess$ChannelHandle#execute` | method | 1 |
| `net/minecraft/client/sounds/ChannelAccess$ChannelHandle#isStopped` | method | 1 |
| `net/minecraft/client/sounds/SoundEngine#loadLibrary` | method | 1 |
| `net/minecraft/client/sounds/SoundEngine#play` | method | 1 |
| `net/minecraft/client/sounds/SoundEngine#tickInGameSound` | method | 1 |
| `net/minecraft/core/BlockPos#containing` | method | 3 |
| `net/minecraft/core/BlockPos#distSqr` | method | 1 |
| `net/minecraft/core/BlockPos#equals` | method | 1 |
| `net/minecraft/core/BlockPos#getX` | method | 3 |
| `net/minecraft/core/BlockPos#getY` | method | 2 |
| `net/minecraft/core/BlockPos#getZ` | method | 3 |
| `net/minecraft/core/BlockPos#toShortString` | method | 1 |
| `net/minecraft/core/DefaultedRegistry#get` | method | 1 |
| `net/minecraft/core/DefaultedRegistry#getKey` | method | 1 |
| `net/minecraft/core/Direction#getApproximateNearest` | method | 2 |
| `net/minecraft/core/Direction#step` | method | 1 |
| `net/minecraft/core/Holder$Reference#is` | method | 1 |
| `net/minecraft/core/Holder$Reference#key` | method | 1 |
| `net/minecraft/core/Holder$Reference#value` | method | 2 |
| `net/minecraft/core/Registry#get` | method | 1 |
| `net/minecraft/core/Registry#getOptional` | method | 1 |
| `net/minecraft/core/Registry#getOrThrow` | method | 1 |
| `net/minecraft/core/Registry#getResourceKey` | method | 1 |
| `net/minecraft/core/Registry#iterator` | method | 1 |
| `net/minecraft/core/Registry#key` | method | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#REGISTRY` | field | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#SOUND_EVENT` | field | 1 |
| `net/minecraft/core/registries/Registries#BLOCK` | field | 1 |
| `net/minecraft/gizmos/GizmoProperties#fadeOut` | method | 1 |
| `net/minecraft/gizmos/GizmoProperties#persistForMillis` | method | 1 |
| `net/minecraft/gizmos/GizmoProperties#setAlwaysOnTop` | method | 1 |
| `net/minecraft/network/chat/Component#getString` | method | 3 |
| `net/minecraft/network/chat/Component#literal` | method | 4 |
| `net/minecraft/network/chat/Component#translatable` | method | 1 |
| `net/minecraft/network/chat/TextColor#BLUE` | field | 1 |
| `net/minecraft/network/chat/TextColor#GREEN` | field | 1 |
| `net/minecraft/network/chat/TextColor#RED` | field | 1 |
| `net/minecraft/network/chat/TextColor#WHITE` | field | 1 |
| `net/minecraft/network/chat/TextColor#getValue` | method | 1 |
| `net/minecraft/resources/Identifier#fromNamespaceAndPath` | method | 1 |
| `net/minecraft/resources/Identifier#getNamespace` | method | 1 |
| `net/minecraft/resources/Identifier#getPath` | method | 1 |
| `net/minecraft/resources/Identifier#toString` | method | 3 |
| `net/minecraft/resources/Identifier#tryParse` | method | 3 |
| `net/minecraft/resources/ResourceKey#identifier` | method | 2 |
| `net/minecraft/sounds/SoundEvent#getRange` | method | 1 |
| `net/minecraft/sounds/SoundEvent#location` | method | 2 |
| `net/minecraft/sounds/SoundEvents#GOAT_HORN_SOUND_VARIANTS` | field | 1 |
| `net/minecraft/sounds/SoundEvents#LIGHTNING_BOLT_THUNDER` | field | 1 |
| `net/minecraft/sounds/SoundEvents#WEATHER_RAIN` | field | 1 |
| `net/minecraft/sounds/SoundEvents#WEATHER_RAIN_ABOVE` | field | 1 |
| `net/minecraft/sounds/SoundSource#BLOCKS` | field | 1 |
| `net/minecraft/sounds/SoundSource#MASTER` | field | 1 |
| `net/minecraft/sounds/SoundSource#RECORDS` | field | 1 |
| `net/minecraft/sounds/SoundSource#toString` | method | 1 |
| `net/minecraft/tags/FluidTags#WATER` | field | 1 |
| `net/minecraft/tags/TagKey#create` | method | 1 |
| `net/minecraft/tags/TagKey#hashCode` | method | 1 |
| `net/minecraft/tags/TagKey#isFor` | method | 1 |
| `net/minecraft/tags/TagKey#location` | method | 1 |
| `net/minecraft/tags/TagKey#registry` | method | 1 |
| `net/minecraft/world/entity/Entity#<init>` | method | 1 |
| `net/minecraft/world/level/BlockGetter#clipWithInteractionOverride` | method | 1 |
| `net/minecraft/world/level/BlockGetter#getBlockState` | method | 1 |
| `net/minecraft/world/level/BlockGetter#getFluidState` | method | 1 |
| `net/minecraft/world/level/BlockGetter#traverseBlocks` | method | 1 |
| `net/minecraft/world/level/ChunkPos#<init>` | method | 1 |
| `net/minecraft/world/level/ChunkPos#x` | method | 1 |
| `net/minecraft/world/level/ChunkPos#z` | method | 1 |
| `net/minecraft/world/level/ClipContext$Block#COLLIDER` | field | 1 |
| `net/minecraft/world/level/ClipContext$Block#get` | method | 1 |
| `net/minecraft/world/level/Level#palettedContainerFactory` | method | 1 |
| `net/minecraft/world/level/Level#playLocalSound` | method | 1 |
| `net/minecraft/world/level/Level#playSound` | method | 1 |
| `net/minecraft/world/level/block/Block#defaultBlockState` | method | 2 |
| `net/minecraft/world/level/block/Block#getDescriptionId` | method | 1 |
| `net/minecraft/world/level/block/Block#getName` | method | 1 |
| `net/minecraft/world/level/block/Blocks#AIR` | field | 1 |
| `net/minecraft/world/level/block/Blocks#JUKEBOX` | field | 1 |
| `net/minecraft/world/level/block/Blocks#LAVA` | field | 1 |
| `net/minecraft/world/level/block/Blocks#VOID_AIR` | field | 1 |
| `net/minecraft/world/level/block/Blocks#WATER` | field | 1 |
| `net/minecraft/world/level/block/SoundType#AMETHYST` | field | 2 |
| `net/minecraft/world/level/block/SoundType#AMETHYST_CLUSTER` | field | 1 |
| `net/minecraft/world/level/block/SoundType#ANCIENT_DEBRIS` | field | 1 |
| `net/minecraft/world/level/block/SoundType#ANVIL` | field | 1 |
| `net/minecraft/world/level/block/SoundType#AZALEA` | field | 1 |
| `net/minecraft/world/level/block/SoundType#AZALEA_LEAVES` | field | 1 |
| `net/minecraft/world/level/block/SoundType#BAMBOO` | field | 2 |
| `net/minecraft/world/level/block/SoundType#BAMBOO_SAPLING` | field | 2 |
| `net/minecraft/world/level/block/SoundType#BAMBOO_WOOD` | field | 1 |
| `net/minecraft/world/level/block/SoundType#BAMBOO_WOOD_HANGING_SIGN` | field | 1 |
| `net/minecraft/world/level/block/SoundType#BASALT` | field | 2 |
| `net/minecraft/world/level/block/SoundType#BIG_DRIPLEAF` | field | 1 |
| `net/minecraft/world/level/block/SoundType#BONE_BLOCK` | field | 2 |
| `net/minecraft/world/level/block/SoundType#CALCITE` | field | 2 |
| `net/minecraft/world/level/block/SoundType#CANDLE` | field | 1 |
| `net/minecraft/world/level/block/SoundType#CAVE_VINES` | field | 1 |
| `net/minecraft/world/level/block/SoundType#CHAIN` | field | 2 |
| `net/minecraft/world/level/block/SoundType#CHERRY_LEAVES` | field | 1 |
| `net/minecraft/world/level/block/SoundType#CHERRY_SAPLING` | field | 1 |
| `net/minecraft/world/level/block/SoundType#CHERRY_WOOD` | field | 1 |
| `net/minecraft/world/level/block/SoundType#CHERRY_WOOD_HANGING_SIGN` | field | 1 |
| `net/minecraft/world/level/block/SoundType#CHISELED_BOOKSHELF` | field | 1 |
| `net/minecraft/world/level/block/SoundType#COPPER` | field | 2 |
| `net/minecraft/world/level/block/SoundType#CORAL_BLOCK` | field | 2 |
| `net/minecraft/world/level/block/SoundType#CROP` | field | 2 |
| `net/minecraft/world/level/block/SoundType#DECORATED_POT` | field | 1 |
| `net/minecraft/world/level/block/SoundType#DECORATED_POT_CRACKED` | field | 1 |
| `net/minecraft/world/level/block/SoundType#DEEPSLATE` | field | 2 |
| `net/minecraft/world/level/block/SoundType#DEEPSLATE_BRICKS` | field | 2 |
| `net/minecraft/world/level/block/SoundType#DEEPSLATE_TILES` | field | 2 |
| `net/minecraft/world/level/block/SoundType#DRIPSTONE_BLOCK` | field | 1 |
| `net/minecraft/world/level/block/SoundType#FLOWERING_AZALEA` | field | 1 |
| `net/minecraft/world/level/block/SoundType#FROGLIGHT` | field | 1 |
| `net/minecraft/world/level/block/SoundType#FROGSPAWN` | field | 1 |
| `net/minecraft/world/level/block/SoundType#FUNGUS` | field | 2 |
| `net/minecraft/world/level/block/SoundType#GILDED_BLACKSTONE` | field | 1 |
| `net/minecraft/world/level/block/SoundType#GLASS` | field | 3 |
| `net/minecraft/world/level/block/SoundType#GLOW_LICHEN` | field | 2 |
| `net/minecraft/world/level/block/SoundType#GRASS` | field | 2 |
| `net/minecraft/world/level/block/SoundType#GRAVEL` | field | 2 |
| `net/minecraft/world/level/block/SoundType#HANGING_ROOTS` | field | 1 |
| `net/minecraft/world/level/block/SoundType#HANGING_SIGN` | field | 1 |
| `net/minecraft/world/level/block/SoundType#HARD_CROP` | field | 1 |
| `net/minecraft/world/level/block/SoundType#HONEY_BLOCK` | field | 3 |
| `net/minecraft/world/level/block/SoundType#LADDER` | field | 2 |
| `net/minecraft/world/level/block/SoundType#LANTERN` | field | 1 |
| `net/minecraft/world/level/block/SoundType#LARGE_AMETHYST_BUD` | field | 2 |
| `net/minecraft/world/level/block/SoundType#LILY_PAD` | field | 2 |
| `net/minecraft/world/level/block/SoundType#LODESTONE` | field | 1 |
| `net/minecraft/world/level/block/SoundType#MANGROVE_ROOTS` | field | 1 |
| `net/minecraft/world/level/block/SoundType#MEDIUM_AMETHYST_BUD` | field | 2 |
| `net/minecraft/world/level/block/SoundType#METAL` | field | 2 |
| `net/minecraft/world/level/block/SoundType#MOSS` | field | 3 |
| `net/minecraft/world/level/block/SoundType#MOSS_CARPET` | field | 2 |
| `net/minecraft/world/level/block/SoundType#MUD` | field | 1 |
| `net/minecraft/world/level/block/SoundType#MUDDY_MANGROVE_ROOTS` | field | 1 |
| `net/minecraft/world/level/block/SoundType#MUD_BRICKS` | field | 1 |
| `net/minecraft/world/level/block/SoundType#NETHERITE_BLOCK` | field | 2 |
| `net/minecraft/world/level/block/SoundType#NETHERRACK` | field | 2 |
| `net/minecraft/world/level/block/SoundType#NETHER_BRICKS` | field | 2 |
| `net/minecraft/world/level/block/SoundType#NETHER_GOLD_ORE` | field | 2 |
| `net/minecraft/world/level/block/SoundType#NETHER_ORE` | field | 2 |
| `net/minecraft/world/level/block/SoundType#NETHER_SPROUTS` | field | 1 |
| `net/minecraft/world/level/block/SoundType#NETHER_WART` | field | 1 |
| `net/minecraft/world/level/block/SoundType#NETHER_WOOD` | field | 1 |
| `net/minecraft/world/level/block/SoundType#NETHER_WOOD_HANGING_SIGN` | field | 1 |
| `net/minecraft/world/level/block/SoundType#NYLIUM` | field | 1 |
| `net/minecraft/world/level/block/SoundType#PACKED_MUD` | field | 1 |
| `net/minecraft/world/level/block/SoundType#POINTED_DRIPSTONE` | field | 2 |
| `net/minecraft/world/level/block/SoundType#POLISHED_DEEPSLATE` | field | 2 |
| `net/minecraft/world/level/block/SoundType#POWDER_SNOW` | field | 2 |
| `net/minecraft/world/level/block/SoundType#ROOTED_DIRT` | field | 1 |
| `net/minecraft/world/level/block/SoundType#ROOTS` | field | 2 |
| `net/minecraft/world/level/block/SoundType#SAND` | field | 2 |
| `net/minecraft/world/level/block/SoundType#SCAFFOLDING` | field | 2 |
| `net/minecraft/world/level/block/SoundType#SCULK` | field | 1 |
| `net/minecraft/world/level/block/SoundType#SCULK_CATALYST` | field | 1 |
| `net/minecraft/world/level/block/SoundType#SCULK_SENSOR` | field | 1 |
| `net/minecraft/world/level/block/SoundType#SCULK_SHRIEKER` | field | 1 |
| `net/minecraft/world/level/block/SoundType#SCULK_VEIN` | field | 1 |
| `net/minecraft/world/level/block/SoundType#SHROOMLIGHT` | field | 1 |
| `net/minecraft/world/level/block/SoundType#SLIME_BLOCK` | field | 1 |
| `net/minecraft/world/level/block/SoundType#SMALL_AMETHYST_BUD` | field | 2 |
| `net/minecraft/world/level/block/SoundType#SMALL_DRIPLEAF` | field | 2 |
| `net/minecraft/world/level/block/SoundType#SNOW` | field | 3 |
| `net/minecraft/world/level/block/SoundType#SOUL_SAND` | field | 2 |
| `net/minecraft/world/level/block/SoundType#SOUL_SOIL` | field | 2 |
| `net/minecraft/world/level/block/SoundType#SPORE_BLOSSOM` | field | 2 |
| `net/minecraft/world/level/block/SoundType#STEM` | field | 2 |
| `net/minecraft/world/level/block/SoundType#STONE` | field | 2 |
| `net/minecraft/world/level/block/SoundType#SUSPICIOUS_GRAVEL` | field | 1 |
| `net/minecraft/world/level/block/SoundType#SUSPICIOUS_SAND` | field | 1 |
| `net/minecraft/world/level/block/SoundType#SWEET_BERRY_BUSH` | field | 2 |
| `net/minecraft/world/level/block/SoundType#TUFF` | field | 2 |
| `net/minecraft/world/level/block/SoundType#TWISTING_VINES` | field | 2 |
| `net/minecraft/world/level/block/SoundType#VINE` | field | 2 |
| `net/minecraft/world/level/block/SoundType#WART_BLOCK` | field | 1 |
| `net/minecraft/world/level/block/SoundType#WEEPING_VINES` | field | 2 |
| `net/minecraft/world/level/block/SoundType#WET_GRASS` | field | 2 |
| `net/minecraft/world/level/block/SoundType#WOOD` | field | 2 |
| `net/minecraft/world/level/block/SoundType#WOOL` | field | 3 |
| `net/minecraft/world/level/block/state/BlockState#getBlock` | method | 2 |
| `net/minecraft/world/level/block/state/BlockState#getSoundType` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#isFaceSturdy` | method | 1 |
| `net/minecraft/world/level/chunk/ChunkAccess#<init>` | method | 1 |
| `net/minecraft/world/level/chunk/ChunkAccess$PackedTicks#<init>` | method | 1 |
| `net/minecraft/world/level/chunk/LevelChunk#getSections` | method | 1 |
| `net/minecraft/world/level/chunk/LevelChunkSection#getBlockState` | method | 1 |
| `net/minecraft/world/level/chunk/LevelChunkSection#getFluidState` | method | 1 |
| `net/minecraft/world/level/chunk/LevelChunkSection#hasOnlyAir` | method | 1 |
| `net/minecraft/world/level/chunk/status/ChunkStatus#FULL` | field | 1 |
| `net/minecraft/world/level/chunk/status/ChunkStatus#heightmapsAfter` | method | 1 |
| `net/minecraft/world/level/levelgen/Heightmap#<init>` | method | 1 |
| `net/minecraft/world/level/levelgen/Heightmap$Types#values` | method | 1 |
| `net/minecraft/world/level/material/Fluid#defaultFluidState` | method | 2 |
| `net/minecraft/world/level/material/FluidState#getShape` | method | 1 |
| `net/minecraft/world/level/material/FluidState#is` | method | 1 |
| `net/minecraft/world/level/material/Fluids#EMPTY` | field | 2 |
| `net/minecraft/world/phys/BlockHitResult#getBlockPos` | method | 1 |
| `net/minecraft/world/phys/BlockHitResult#getDirection` | method | 1 |
| `net/minecraft/world/phys/BlockHitResult#getLocation` | method | 2 |
| `net/minecraft/world/phys/BlockHitResult#getType` | method | 1 |
| `net/minecraft/world/phys/BlockHitResult#miss` | method | 1 |
| `net/minecraft/world/phys/HitResult$Type#BLOCK` | field | 1 |
| `net/minecraft/world/phys/HitResult$Type#MISS` | field | 1 |
| `net/minecraft/world/phys/Vec3#<init>` | method | 4 |
| `net/minecraft/world/phys/Vec3#add` | method | 3 |
| `net/minecraft/world/phys/Vec3#distanceTo` | method | 4 |
| `net/minecraft/world/phys/Vec3#distanceToSqr` | method | 1 |
| `net/minecraft/world/phys/Vec3#dot` | method | 1 |
| `net/minecraft/world/phys/Vec3#length` | method | 1 |
| `net/minecraft/world/phys/Vec3#scale` | method | 2 |
| `net/minecraft/world/phys/Vec3#subtract` | method | 3 |
| `net/minecraft/world/phys/Vec3#x` | field | 4 |
| `net/minecraft/world/phys/Vec3#y` | field | 4 |
| `net/minecraft/world/phys/Vec3#z` | field | 4 |
| `net/minecraft/world/phys/shapes/CollisionContext#empty` | method | 1 |
| `net/minecraft/world/phys/shapes/VoxelShape#clip` | method | 1 |
| `net/minecraft/world/ticks/LevelChunkTicks#<init>` | method | 1 |
| `org/lwjgl/openal/ALC10#alcCreateContext` | method | 1 |

</details>

