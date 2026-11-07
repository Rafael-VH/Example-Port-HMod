# Java translated to Rust

Translated 14 of 436 methods (1 more is done another way in Horizon) and 37 of 61 static fields to Rust; 0 of 2 Fabric event handlers. It's in `runtime.rs`, under `pub mod java`; code that calls the game is in `game_side.rs`.

Each method is translated whole or not at all. A method that can't be (it uses Minecraft, a lambda, null...) is left out with its reason, and so is every method that calls it.

## Why methods were left out

| Cause | Methods | For example |
|---|---|---|
| Calls or uses other code that isn't translated | 318 | `BlockDefinition.compareTo(1)`: its class isn't translated (it implements `Comparable`) |
| Other Java this translator doesn't handle yet | 66 | `BlockDefinition.getConfigString(0)`: it has no body |
| Uses Minecraft classes (needs Horizon's game API) | 20 | `SoundTypes.getTranslationMap(0)`: it uses Minecraft's `SoundType` |
| Uses other libraries | 7 | `SimpleVoiceChatPlugin.getDirectGainScale(2)`: it uses `VoicechatClientApi` (de.maxhenkel.voicechat.api) |
| Uses null | 6 | `ReverbParams.getReverb0(0)`: a nullable object kept in a changing static |
| Uses files (java.nio / java.io) | 2 | `CommentedProperties.readLine(1)`: it uses `StringReader` (java.io) |
| Calls the game where Horizon has no helper yet | 1 | `SoundPhysics.reflect(2)`: it calls Minecraft's Vec3.dot (no Horizon helper for it yet) |
| Done another way in Horizon | 1 | `ModMenuIntegration.getModConfigScreenFactory(0)`: done another way in Horizon: the mod's page in Options > Mods (its settings buttons) |
| Uses try/catch | 1 | `SoundPhysicsMod.renameAllowedSounds(0)`: a try/catch (NoSuchFileException, FileAlreadyExistsException, IOException) |

## Fabric event handlers

| Handler | Hook | Result |
|---|---|---|
| com/sonicether/soundphysics/mixin/ClientPacketListenerMixin.java:18 | `join` | not translated: it calls LevelAccessUtils.onLoadLevel, which isn't translated |
| com/sonicether/soundphysics/FabricSoundPhysicsMod.java:9 | `init` | not translated: it calls FabricSoundPhysicsMod.init, which isn't in the mod's source |

## Every class


### `BlockDefinition`: it implements `Comparable`

- [ ] `BlockDefinition.getConfigString(0)`: it has no body (com/sonicether/soundphysics/config/blocksound/BlockDefinition.java:8)
- [ ] `BlockDefinition.getConfigComment(0)`: it has no body (com/sonicether/soundphysics/config/blocksound/BlockDefinition.java:10)
- [ ] `BlockDefinition.getName(0)`: it has no body (com/sonicether/soundphysics/config/blocksound/BlockDefinition.java:13)
- [ ] `BlockDefinition.compareTo(1)`: its class isn't translated (it implements `Comparable`) (com/sonicether/soundphysics/config/blocksound/BlockDefinition.java:15)

### `BlockIdDefinition`: it extends `BlockDefinition`

- [ ] `BlockIdDefinition.BlockIdDefinition(1)`: its class isn't translated (it extends `BlockDefinition`) (com/sonicether/soundphysics/config/blocksound/BlockIdDefinition.java:14)
- [ ] `BlockIdDefinition.getConfigString(0)`: its class isn't translated (it extends `BlockDefinition`) (com/sonicether/soundphysics/config/blocksound/BlockIdDefinition.java:18)
- [ ] `BlockIdDefinition.getConfigComment(0)`: its class isn't translated (it extends `BlockDefinition`) (com/sonicether/soundphysics/config/blocksound/BlockIdDefinition.java:23)
- [ ] `BlockIdDefinition.getName(0)`: its class isn't translated (it extends `BlockDefinition`) (com/sonicether/soundphysics/config/blocksound/BlockIdDefinition.java:29)
- [ ] `BlockIdDefinition.getBlock(0)`: its class isn't translated (it extends `BlockDefinition`) (com/sonicether/soundphysics/config/blocksound/BlockIdDefinition.java:34)
- [ ] `BlockIdDefinition.fromConfigString(1)`: it uses BlockIdDefinition, which isn't translated (it extends `BlockDefinition`) (com/sonicether/soundphysics/config/blocksound/BlockIdDefinition.java:38)
- [ ] `BlockIdDefinition.equals(1)`: its class isn't translated (it extends `BlockDefinition`) (com/sonicether/soundphysics/config/blocksound/BlockIdDefinition.java:50)
- [ ] `BlockIdDefinition.hashCode(0)`: its class isn't translated (it extends `BlockDefinition`) (com/sonicether/soundphysics/config/blocksound/BlockIdDefinition.java:62)

### `BlockSoundConfigBase`: it extends `CommentedPropertyConfig` (de.maxhenkel.sound_physics_remastered.configbuilder)

- [ ] `BlockSoundConfigBase.BlockSoundConfigBase(1)`: its class isn't translated (it extends `CommentedPropertyConfig` (de.maxhenkel.sound_physics_remastered.configbuilder)) (com/sonicether/soundphysics/config/blocksound/BlockSoundConfigBase.java:34)
- [ ] `BlockSoundConfigBase.load(0)`: its class isn't translated (it extends `CommentedPropertyConfig` (de.maxhenkel.sound_physics_remastered.configbuilder)) (com/sonicether/soundphysics/config/blocksound/BlockSoundConfigBase.java:40)
- [ ] `BlockSoundConfigBase.loadBlockDefinition(1)`: it uses BlockDefinition, which isn't translated (it implements `Comparable`) (com/sonicether/soundphysics/config/blocksound/BlockSoundConfigBase.java:71)
- [ ] `BlockSoundConfigBase.saveSync(0)`: its class isn't translated (it extends `CommentedPropertyConfig` (de.maxhenkel.sound_physics_remastered.configbuilder)) (com/sonicether/soundphysics/config/blocksound/BlockSoundConfigBase.java:81)
- [ ] `BlockSoundConfigBase.getBlockDefinitions(0)`: its class isn't translated (it extends `CommentedPropertyConfig` (de.maxhenkel.sound_physics_remastered.configbuilder)) (com/sonicether/soundphysics/config/blocksound/BlockSoundConfigBase.java:114)
- [ ] `BlockSoundConfigBase.getBlockDefinitionValue(1)`: its class isn't translated (it extends `CommentedPropertyConfig` (de.maxhenkel.sound_physics_remastered.configbuilder)) (com/sonicether/soundphysics/config/blocksound/BlockSoundConfigBase.java:118)
- [ ] `BlockSoundConfigBase.isTagIn(2)`: it's generic (com/sonicether/soundphysics/config/blocksound/BlockSoundConfigBase.java:134)
- [ ] `BlockSoundConfigBase.invalidateCaches(0)`: its class isn't translated (it extends `CommentedPropertyConfig` (de.maxhenkel.sound_physics_remastered.configbuilder)) (com/sonicether/soundphysics/config/blocksound/BlockSoundConfigBase.java:147)
- [ ] `BlockSoundConfigBase.getBlockTags(0)`: its class isn't translated (it extends `CommentedPropertyConfig` (de.maxhenkel.sound_physics_remastered.configbuilder)) (com/sonicether/soundphysics/config/blocksound/BlockSoundConfigBase.java:153)
- [ ] `BlockSoundConfigBase.getBlocks(0)`: its class isn't translated (it extends `CommentedPropertyConfig` (de.maxhenkel.sound_physics_remastered.configbuilder)) (com/sonicether/soundphysics/config/blocksound/BlockSoundConfigBase.java:167)
- [ ] `BlockSoundConfigBase.getSoundTypes(0)`: its class isn't translated (it extends `CommentedPropertyConfig` (de.maxhenkel.sound_physics_remastered.configbuilder)) (com/sonicether/soundphysics/config/blocksound/BlockSoundConfigBase.java:181)
- [ ] `BlockSoundConfigBase.setBlockDefinitionValue(2)`: its class isn't translated (it extends `CommentedPropertyConfig` (de.maxhenkel.sound_physics_remastered.configbuilder)) (com/sonicether/soundphysics/config/blocksound/BlockSoundConfigBase.java:195)
- [ ] `BlockSoundConfigBase.addDefaults(1)`: it has no body (com/sonicether/soundphysics/config/blocksound/BlockSoundConfigBase.java:201)
- [ ] `BlockSoundConfigBase.getDefaultValue(0)`: it has no body (com/sonicether/soundphysics/config/blocksound/BlockSoundConfigBase.java:203)
- [ ] `BlockSoundConfigBase.putSoundType(3)`: it uses BlockDefinition, which isn't translated (it implements `Comparable`) (com/sonicether/soundphysics/config/blocksound/BlockSoundConfigBase.java:205)

### `BlockSoundTypeDefinition`: it extends `BlockDefinition`

- [ ] `BlockSoundTypeDefinition.BlockSoundTypeDefinition(1)`: its class isn't translated (it extends `BlockDefinition`) (com/sonicether/soundphysics/config/blocksound/BlockSoundTypeDefinition.java:12)
- [ ] `BlockSoundTypeDefinition.getConfigString(0)`: its class isn't translated (it extends `BlockDefinition`) (com/sonicether/soundphysics/config/blocksound/BlockSoundTypeDefinition.java:16)
- [ ] `BlockSoundTypeDefinition.getConfigComment(0)`: its class isn't translated (it extends `BlockDefinition`) (com/sonicether/soundphysics/config/blocksound/BlockSoundTypeDefinition.java:21)
- [ ] `BlockSoundTypeDefinition.getName(0)`: its class isn't translated (it extends `BlockDefinition`) (com/sonicether/soundphysics/config/blocksound/BlockSoundTypeDefinition.java:27)
- [ ] `BlockSoundTypeDefinition.getSoundType(0)`: its class isn't translated (it extends `BlockDefinition`) (com/sonicether/soundphysics/config/blocksound/BlockSoundTypeDefinition.java:32)
- [ ] `BlockSoundTypeDefinition.fromConfigString(1)`: it uses BlockSoundTypeDefinition, which isn't translated (it extends `BlockDefinition`) (com/sonicether/soundphysics/config/blocksound/BlockSoundTypeDefinition.java:36)
- [ ] `BlockSoundTypeDefinition.equals(1)`: its class isn't translated (it extends `BlockDefinition`) (com/sonicether/soundphysics/config/blocksound/BlockSoundTypeDefinition.java:42)
- [ ] `BlockSoundTypeDefinition.hashCode(0)`: its class isn't translated (it extends `BlockDefinition`) (com/sonicether/soundphysics/config/blocksound/BlockSoundTypeDefinition.java:54)

### `BlockTagDefinition`: it extends `BlockDefinition`

- [ ] `BlockTagDefinition.BlockTagDefinition(1)`: its class isn't translated (it extends `BlockDefinition`) (com/sonicether/soundphysics/config/blocksound/BlockTagDefinition.java:14)
- [ ] `BlockTagDefinition.getConfigString(0)`: its class isn't translated (it extends `BlockDefinition`) (com/sonicether/soundphysics/config/blocksound/BlockTagDefinition.java:18)
- [ ] `BlockTagDefinition.getConfigComment(0)`: its class isn't translated (it extends `BlockDefinition`) (com/sonicether/soundphysics/config/blocksound/BlockTagDefinition.java:23)
- [ ] `BlockTagDefinition.getName(0)`: its class isn't translated (it extends `BlockDefinition`) (com/sonicether/soundphysics/config/blocksound/BlockTagDefinition.java:29)
- [ ] `BlockTagDefinition.getBlockTag(0)`: its class isn't translated (it extends `BlockDefinition`) (com/sonicether/soundphysics/config/blocksound/BlockTagDefinition.java:34)
- [ ] `BlockTagDefinition.fromConfigString(1)`: it uses BlockTagDefinition, which isn't translated (it extends `BlockDefinition`) (com/sonicether/soundphysics/config/blocksound/BlockTagDefinition.java:38)
- [ ] `BlockTagDefinition.equals(1)`: its class isn't translated (it extends `BlockDefinition`) (com/sonicether/soundphysics/config/blocksound/BlockTagDefinition.java:49)
- [ ] `BlockTagDefinition.hashCode(0)`: its class isn't translated (it extends `BlockDefinition`) (com/sonicether/soundphysics/config/blocksound/BlockTagDefinition.java:61)

### `ConfigUtils`

- [ ] `ConfigUtils.sortMap(1)`: it's generic (com/sonicether/soundphysics/config/ConfigUtils.java:10)

### `OcclusionConfig`: it extends `BlockSoundConfigBase` (com.sonicether.soundphysics.config.blocksound)

- [ ] `OcclusionConfig.OcclusionConfig(1)`: its class isn't translated (it extends `BlockSoundConfigBase` (com.sonicether.soundphysics.config.blocksound)) (com/sonicether/soundphysics/config/OcclusionConfig.java:13)
- [ ] `OcclusionConfig.addDefaults(1)`: its class isn't translated (it extends `BlockSoundConfigBase` (com.sonicether.soundphysics.config.blocksound)) (com/sonicether/soundphysics/config/OcclusionConfig.java:17)
- [ ] `OcclusionConfig.getDefaultValue(0)`: its class isn't translated (it extends `BlockSoundConfigBase` (com.sonicether.soundphysics.config.blocksound)) (com/sonicether/soundphysics/config/OcclusionConfig.java:56)

### `ReflectivityConfig`: it extends `BlockSoundConfigBase` (com.sonicether.soundphysics.config.blocksound)

- [ ] `ReflectivityConfig.ReflectivityConfig(1)`: its class isn't translated (it extends `BlockSoundConfigBase` (com.sonicether.soundphysics.config.blocksound)) (com/sonicether/soundphysics/config/ReflectivityConfig.java:11)
- [ ] `ReflectivityConfig.addDefaults(1)`: its class isn't translated (it extends `BlockSoundConfigBase` (com.sonicether.soundphysics.config.blocksound)) (com/sonicether/soundphysics/config/ReflectivityConfig.java:15)
- [ ] `ReflectivityConfig.getDefaultValue(0)`: its class isn't translated (it extends `BlockSoundConfigBase` (com.sonicether.soundphysics.config.blocksound)) (com/sonicether/soundphysics/config/ReflectivityConfig.java:53)

### `ReverbParams`

- [ ] `ReverbParams.getReverb0(0)`: a nullable object kept in a changing static (com/sonicether/soundphysics/config/ReverbParams.java:19)
- [ ] `ReverbParams.getReverb1(0)`: a nullable object kept in a changing static (com/sonicether/soundphysics/config/ReverbParams.java:36)
- [ ] `ReverbParams.getReverb2(0)`: a nullable object kept in a changing static (com/sonicether/soundphysics/config/ReverbParams.java:53)
- [ ] `ReverbParams.getReverb3(0)`: a nullable object kept in a changing static (com/sonicether/soundphysics/config/ReverbParams.java:70)
- [ ] `ReverbParams.globalReverbMultiplier(0)`: a nullable object kept in a changing static (com/sonicether/soundphysics/config/ReverbParams.java:87)

### `SoundPhysicsConfig`: it uses ConfigEntry, which isn't translated (it's an interface)

- [ ] `SoundPhysicsConfig.SoundPhysicsConfig(1)`: its class isn't translated (it uses ConfigEntry, which isn't translated (it's an interface)) (com/sonicether/soundphysics/config/SoundPhysicsConfig.java:47)
- [ ] `SoundPhysicsConfig.reloadClient(0)`: its class isn't translated (it uses ConfigEntry, which isn't translated (it's an interface)) (com/sonicether/soundphysics/config/SoundPhysicsConfig.java:156)

### `SoundRateConfig`: it extends `CommentedPropertyConfig` (de.maxhenkel.sound_physics_remastered.configbuilder)

- [ ] `SoundRateConfig.SoundRateConfig(1)`: its class isn't translated (it extends `CommentedPropertyConfig` (de.maxhenkel.sound_physics_remastered.configbuilder)) (com/sonicether/soundphysics/config/SoundRateConfig.java:22)
- [ ] `SoundRateConfig.load(0)`: its class isn't translated (it extends `CommentedPropertyConfig` (de.maxhenkel.sound_physics_remastered.configbuilder)) (com/sonicether/soundphysics/config/SoundRateConfig.java:28)
- [ ] `SoundRateConfig.logIfUnknownSound(1)`: its class isn't translated (it extends `CommentedPropertyConfig` (de.maxhenkel.sound_physics_remastered.configbuilder)) (com/sonicether/soundphysics/config/SoundRateConfig.java:75)
- [ ] `SoundRateConfig.saveSync(0)`: its class isn't translated (it extends `CommentedPropertyConfig` (de.maxhenkel.sound_physics_remastered.configbuilder)) (com/sonicether/soundphysics/config/SoundRateConfig.java:86)
- [ ] `SoundRateConfig.getSoundRateConfig(0)`: its class isn't translated (it extends `CommentedPropertyConfig` (de.maxhenkel.sound_physics_remastered.configbuilder)) (com/sonicether/soundphysics/config/SoundRateConfig.java:102)
- [ ] `SoundRateConfig.getMaxCount(1)`: its class isn't translated (it extends `CommentedPropertyConfig` (de.maxhenkel.sound_physics_remastered.configbuilder)) (com/sonicether/soundphysics/config/SoundRateConfig.java:106)
- [ ] `SoundRateConfig.createDefaultMap(0)`: its class isn't translated (it extends `CommentedPropertyConfig` (de.maxhenkel.sound_physics_remastered.configbuilder)) (com/sonicether/soundphysics/config/SoundRateConfig.java:111)

### `SoundTypes`

- [ ] `SoundTypes.TRANSLATION_MAP`: it uses Minecraft's `SoundType` (com/sonicether/soundphysics/config/SoundTypes.java:14)
- [ ] `SoundTypes.getTranslationMap(0)`: it uses Minecraft's `SoundType` (com/sonicether/soundphysics/config/SoundTypes.java:16)
- [ ] `SoundTypes.getName(1)`: it uses Minecraft's `SoundType` (com/sonicether/soundphysics/config/SoundTypes.java:20)
- [ ] `SoundTypes.getSoundType(1)`: it uses Minecraft's `SoundType` (com/sonicether/soundphysics/config/SoundTypes.java:25)
- [ ] `SoundTypes.getNameComponent(1)`: it uses Minecraft's `SoundType` (com/sonicether/soundphysics/config/SoundTypes.java:30)

### `RaycastRenderer`: it implements Minecraft's `SimpleDebugRenderer`

- [ ] `RaycastRenderer.rays`: it uses Ray, which isn't translated (it uses Minecraft's `Vec3`) (com/sonicether/soundphysics/debug/RaycastRenderer.java:17)
- [ ] `RaycastRenderer.mc`: it uses Minecraft's `Minecraft` (com/sonicether/soundphysics/debug/RaycastRenderer.java:18)
- [ ] `RaycastRenderer.emitGizmos(6)`: its class isn't translated (it implements Minecraft's `SimpleDebugRenderer`) (com/sonicether/soundphysics/debug/RaycastRenderer.java:20)
- [ ] `RaycastRenderer.color(1)`: it uses Minecraft's `TextColor` (com/sonicether/soundphysics/debug/RaycastRenderer.java:44)
- [ ] `RaycastRenderer.addSoundBounceRay(3)`: it uses SoundPhysicsMod.CONFIG, which isn't translated (it uses SoundPhysicsConfig, which isn't translated (it uses ConfigEntry, which isn't translated (it's an interface))) (com/sonicether/soundphysics/debug/RaycastRenderer.java:48)
- [ ] `RaycastRenderer.addOcclusionRay(3)`: it uses SoundPhysicsMod.CONFIG, which isn't translated (it uses SoundPhysicsConfig, which isn't translated (it uses ConfigEntry, which isn't translated (it's an interface))) (com/sonicether/soundphysics/debug/RaycastRenderer.java:54)
- [ ] `RaycastRenderer.addRay(4)`: it uses RaycastRenderer.mc, which isn't translated (it uses Minecraft's `Minecraft`) (com/sonicether/soundphysics/debug/RaycastRenderer.java:60)

### `RaycastRenderer.Ray`: it uses Minecraft's `Vec3`

- [ ] `RaycastRenderer.Ray.Ray(4)`: its class isn't translated (it uses Minecraft's `Vec3`) (com/sonicether/soundphysics/debug/RaycastRenderer.java:76)

### `FabricSoundPhysicsMod`: it extends `SoundPhysicsMod`

- [ ] `FabricSoundPhysicsMod.onInitialize(0)`: its class isn't translated (it extends `SoundPhysicsMod`) (com/sonicether/soundphysics/FabricSoundPhysicsMod.java:9)
- [ ] `FabricSoundPhysicsMod.onInitializeClient(0)`: its class isn't translated (it extends `SoundPhysicsMod`) (com/sonicether/soundphysics/FabricSoundPhysicsMod.java:13)
- [ ] `FabricSoundPhysicsMod.getConfigFolder(0)`: its class isn't translated (it extends `SoundPhysicsMod`) (com/sonicether/soundphysics/FabricSoundPhysicsMod.java:17)

### `ClothConfigIntegration`

- [ ] `ClothConfigIntegration.createConfigScreen(1)`: it uses Minecraft's `Screen` (com/sonicether/soundphysics/integration/ClothConfigIntegration.java:27)
- [ ] `ClothConfigIntegration.fromConfigEntry(4)`: it's generic (com/sonicether/soundphysics/integration/ClothConfigIntegration.java:328)

### `ModMenuIntegration`: it implements `ModMenuApi` (com.terraformersmc.modmenu.api)

- [ ] `ModMenuIntegration.getModConfigScreenFactory(0)`: done another way in Horizon: the mod's page in Options > Mods (its settings buttons) (com/sonicether/soundphysics/integration/ModMenuIntegration.java:12)
- [x] `ModMenuIntegration.isClothConfigLoaded(0)` (com/sonicether/soundphysics/integration/ModMenuIntegration.java:16)

### `AudioChannel`: it uses `UUID` (java.util)

- [x] `AudioChannel.VOICECHAT` (com/sonicether/soundphysics/integration/voicechat/AudioChannel.java:11)
- [x] `AudioChannel.VOICECHAT_SOUND` (com/sonicether/soundphysics/integration/voicechat/AudioChannel.java:12)
- [ ] `AudioChannel.AudioChannel(1)`: its class isn't translated (it uses `UUID` (java.util)) (com/sonicether/soundphysics/integration/voicechat/AudioChannel.java:17)
- [ ] `AudioChannel.onSound(4)`: its class isn't translated (it uses `UUID` (java.util)) (com/sonicether/soundphysics/integration/voicechat/AudioChannel.java:21)
- [ ] `AudioChannel.getChannelId(0)`: its class isn't translated (it uses `UUID` (java.util)) (com/sonicether/soundphysics/integration/voicechat/AudioChannel.java:35)
- [ ] `AudioChannel.canBeRemoved(0)`: its class isn't translated (it uses `UUID` (java.util)) (com/sonicether/soundphysics/integration/voicechat/AudioChannel.java:39)
- [x] `AudioChannel.isVoicechatSound(1)` (com/sonicether/soundphysics/integration/voicechat/AudioChannel.java:43)

### `SimpleVoiceChatPlugin`: it implements `VoicechatPlugin` (de.maxhenkel.voicechat.api)

- [ ] `SimpleVoiceChatPlugin.OWN_VOICE_ID`: it uses `UUID` (java.util) (com/sonicether/soundphysics/integration/voicechat/SimpleVoiceChatPlugin.java:29)
- [x] `SimpleVoiceChatPlugin.OWN_VOICE_CATEGORY` (com/sonicether/soundphysics/integration/voicechat/SimpleVoiceChatPlugin.java:30)
- [ ] `SimpleVoiceChatPlugin.getPluginId(0)`: its class isn't translated (it implements `VoicechatPlugin` (de.maxhenkel.voicechat.api)) (com/sonicether/soundphysics/integration/voicechat/SimpleVoiceChatPlugin.java:34)
- [ ] `SimpleVoiceChatPlugin.initialize(1)`: its class isn't translated (it implements `VoicechatPlugin` (de.maxhenkel.voicechat.api)) (com/sonicether/soundphysics/integration/voicechat/SimpleVoiceChatPlugin.java:38)
- [ ] `SimpleVoiceChatPlugin.registerEvents(1)`: its class isn't translated (it implements `VoicechatPlugin` (de.maxhenkel.voicechat.api)) (com/sonicether/soundphysics/integration/voicechat/SimpleVoiceChatPlugin.java:43)
- [ ] `SimpleVoiceChatPlugin.onServerStarted(1)`: its class isn't translated (it implements `VoicechatPlugin` (de.maxhenkel.voicechat.api)) (com/sonicether/soundphysics/integration/voicechat/SimpleVoiceChatPlugin.java:51)
- [ ] `SimpleVoiceChatPlugin.onClientSound(1)`: its class isn't translated (it implements `VoicechatPlugin` (de.maxhenkel.voicechat.api)) (com/sonicether/soundphysics/integration/voicechat/SimpleVoiceChatPlugin.java:63)
- [ ] `SimpleVoiceChatPlugin.onCreateALContext(1)`: its class isn't translated (it implements `VoicechatPlugin` (de.maxhenkel.voicechat.api)) (com/sonicether/soundphysics/integration/voicechat/SimpleVoiceChatPlugin.java:74)
- [ ] `SimpleVoiceChatPlugin.onConnection(1)`: its class isn't translated (it implements `VoicechatPlugin` (de.maxhenkel.voicechat.api)) (com/sonicether/soundphysics/integration/voicechat/SimpleVoiceChatPlugin.java:82)
- [ ] `SimpleVoiceChatPlugin.onOpenALSound(1)`: its class isn't translated (it implements `VoicechatPlugin` (de.maxhenkel.voicechat.api)) (com/sonicether/soundphysics/integration/voicechat/SimpleVoiceChatPlugin.java:88)
- [ ] `SimpleVoiceChatPlugin.getDirectGainScale(2)`: it uses `VoicechatClientApi` (de.maxhenkel.voicechat.api) (com/sonicether/soundphysics/integration/voicechat/SimpleVoiceChatPlugin.java:114)

### `Loggers`

- [x] `Loggers.LOG_PREFIX` (com/sonicether/soundphysics/Loggers.java:8)
- [x] `Loggers.DEBUG_LOGGER` (com/sonicether/soundphysics/Loggers.java:9)
- [x] `Loggers.PROFILING_LOGGER` (com/sonicether/soundphysics/Loggers.java:10)
- [x] `Loggers.ENVIRONMENT_LOGGER` (com/sonicether/soundphysics/Loggers.java:11)
- [x] `Loggers.OCCLUSION_LOGGER` (com/sonicether/soundphysics/Loggers.java:12)
- [x] `Loggers.LOGGER` (com/sonicether/soundphysics/Loggers.java:13)
- [ ] `Loggers.log(2)`: it uses `Object` (com/sonicether/soundphysics/Loggers.java:15)
- [ ] `Loggers.warn(2)`: it uses `Object` (com/sonicether/soundphysics/Loggers.java:19)
- [ ] `Loggers.error(2)`: it uses `Object` (com/sonicether/soundphysics/Loggers.java:23)
- [ ] `Loggers.logProfiling(2)`: it uses `Object` (com/sonicether/soundphysics/Loggers.java:27)
- [ ] `Loggers.logDebug(2)`: it uses `Object` (com/sonicether/soundphysics/Loggers.java:33)
- [ ] `Loggers.logOcclusion(2)`: it uses `Object` (com/sonicether/soundphysics/Loggers.java:39)
- [ ] `Loggers.logEnvironment(2)`: it uses `Object` (com/sonicether/soundphysics/Loggers.java:45)
- [ ] `Loggers.logALError(1)`: it calls `AL11` (org.lwjgl.openal).alGetError (com/sonicether/soundphysics/Loggers.java:51)
- [ ] `ChannelAccessor`: a Mixin class: its handlers are the injections, in the bundle's helpers (com/sonicether/soundphysics/Loggers.java:8)
- [ ] `ClientLevelMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (com/sonicether/soundphysics/Loggers.java:18)
- [ ] `ClientPacketListenerMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (com/sonicether/soundphysics/Loggers.java:14)
- [ ] `DebugRendererMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (com/sonicether/soundphysics/Loggers.java:16)
- [ ] `EntityMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (com/sonicether/soundphysics/Loggers.java:15)
- [ ] `LibraryMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (com/sonicether/soundphysics/Loggers.java:14)
- [ ] `LocalPlayerMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (com/sonicether/soundphysics/Loggers.java:15)
- [ ] `MinecraftMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (com/sonicether/soundphysics/Loggers.java:16)
- [ ] `SoundEventMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (com/sonicether/soundphysics/Loggers.java:10)
- [ ] `SoundSystemMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (com/sonicether/soundphysics/Loggers.java:19)
- [ ] `SourceMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (com/sonicether/soundphysics/Loggers.java:18)

### `TaskProfiler`: it keeps an atomic value in an object's field

- [x] `TaskProfiler.TASK_RING_BUFFER_SIZE` (com/sonicether/soundphysics/profiling/TaskProfiler.java:10)
- [x] `TaskProfiler.TASK_RING_TALLY_SIZE` (com/sonicether/soundphysics/profiling/TaskProfiler.java:11)
- [ ] `TaskProfiler.TaskProfiler(1)`: its class isn't translated (it keeps an atomic value in an object's field) (com/sonicether/soundphysics/profiling/TaskProfiler.java:16)
- [ ] `TaskProfiler.profile(0)`: its class isn't translated (it keeps an atomic value in an object's field) (com/sonicether/soundphysics/profiling/TaskProfiler.java:22)
- [ ] `TaskProfiler.addDuration(1)`: its class isn't translated (it keeps an atomic value in an object's field) (com/sonicether/soundphysics/profiling/TaskProfiler.java:26)
- [ ] `TaskProfiler.getTotalDuration(0)`: its class isn't translated (it keeps an atomic value in an object's field) (com/sonicether/soundphysics/profiling/TaskProfiler.java:35)
- [ ] `TaskProfiler.getAverageDuration(0)`: its class isn't translated (it keeps an atomic value in an object's field) (com/sonicether/soundphysics/profiling/TaskProfiler.java:39)
- [ ] `TaskProfiler.getMinDuration(0)`: its class isn't translated (it keeps an atomic value in an object's field) (com/sonicether/soundphysics/profiling/TaskProfiler.java:43)
- [ ] `TaskProfiler.getMaxDuration(0)`: its class isn't translated (it keeps an atomic value in an object's field) (com/sonicether/soundphysics/profiling/TaskProfiler.java:47)
- [ ] `TaskProfiler.logResults(0)`: its class isn't translated (it keeps an atomic value in an object's field) (com/sonicether/soundphysics/profiling/TaskProfiler.java:51)
- [ ] `TaskProfiler.onTally(1)`: its class isn't translated (it keeps an atomic value in an object's field) (com/sonicether/soundphysics/profiling/TaskProfiler.java:62)

### `TaskProfiler.TaskProfilerHandle`: it uses `WeakReference` (java.lang.ref)

- [ ] `TaskProfiler.TaskProfilerHandle.TaskProfilerHandle(0)`: its class isn't translated (it uses `WeakReference` (java.lang.ref)) (com/sonicether/soundphysics/profiling/TaskProfiler.java:74)
- [ ] `TaskProfiler.TaskProfilerHandle.finish(0)`: its class isn't translated (it uses `WeakReference` (java.lang.ref)) (com/sonicether/soundphysics/profiling/TaskProfiler.java:77)
- [ ] `TaskProfiler.TaskProfilerHandle.getDuration(0)`: its class isn't translated (it uses `WeakReference` (java.lang.ref)) (com/sonicether/soundphysics/profiling/TaskProfiler.java:86)

### `ReflectedAudio`: it uses Minecraft's `Vec3`

- [ ] `ReflectedAudio.ReflectedAudio(2)`: its class isn't translated (it uses Minecraft's `Vec3`) (com/sonicether/soundphysics/ReflectedAudio.java:20)
- [ ] `ReflectedAudio.shouldEvaluateDirection(0)`: its class isn't translated (it uses Minecraft's `Vec3`) (com/sonicether/soundphysics/ReflectedAudio.java:26)
- [ ] `ReflectedAudio.getSharedAirspaces(0)`: its class isn't translated (it uses Minecraft's `Vec3`) (com/sonicether/soundphysics/ReflectedAudio.java:32)
- [ ] `ReflectedAudio.addDirectAirspace(1)`: its class isn't translated (it uses Minecraft's `Vec3`) (com/sonicether/soundphysics/ReflectedAudio.java:36)
- [ ] `ReflectedAudio.addSharedAirspace(2)`: its class isn't translated (it uses Minecraft's `Vec3`) (com/sonicether/soundphysics/ReflectedAudio.java:40)
- [ ] `ReflectedAudio.evaluateSoundPosition(2)`: its class isn't translated (it uses Minecraft's `Vec3`) (com/sonicether/soundphysics/ReflectedAudio.java:52)

### `SoundPhysics`

- [x] `SoundPhysics.PHI` (com/sonicether/soundphysics/SoundPhysics.java:34)
- [x] `SoundPhysics.AMBIENT_PATTERN` (com/sonicether/soundphysics/SoundPhysics.java:35)
- [x] `SoundPhysics.BLOCK_PATTERN` (com/sonicether/soundphysics/SoundPhysics.java:36)
- [x] `SoundPhysics.REFERENCE_DISTANCE_BUG_VERSION` (com/sonicether/soundphysics/SoundPhysics.java:37)
- [x] `SoundPhysics.auxFXSlot0` (com/sonicether/soundphysics/SoundPhysics.java:38)
- [x] `SoundPhysics.auxFXSlot1` (com/sonicether/soundphysics/SoundPhysics.java:39)
- [x] `SoundPhysics.auxFXSlot2` (com/sonicether/soundphysics/SoundPhysics.java:40)
- [x] `SoundPhysics.auxFXSlot3` (com/sonicether/soundphysics/SoundPhysics.java:41)
- [x] `SoundPhysics.reverb0` (com/sonicether/soundphysics/SoundPhysics.java:42)
- [x] `SoundPhysics.reverb1` (com/sonicether/soundphysics/SoundPhysics.java:43)
- [x] `SoundPhysics.reverb2` (com/sonicether/soundphysics/SoundPhysics.java:44)
- [x] `SoundPhysics.reverb3` (com/sonicether/soundphysics/SoundPhysics.java:45)
- [x] `SoundPhysics.directFilter0` (com/sonicether/soundphysics/SoundPhysics.java:46)
- [x] `SoundPhysics.sendFilter0` (com/sonicether/soundphysics/SoundPhysics.java:47)
- [x] `SoundPhysics.sendFilter1` (com/sonicether/soundphysics/SoundPhysics.java:48)
- [x] `SoundPhysics.sendFilter2` (com/sonicether/soundphysics/SoundPhysics.java:49)
- [x] `SoundPhysics.sendFilter3` (com/sonicether/soundphysics/SoundPhysics.java:50)
- [ ] `SoundPhysics.minecraft`: it uses Minecraft's `Minecraft` (com/sonicether/soundphysics/SoundPhysics.java:51)
- [ ] `SoundPhysics.profiler`: it uses TaskProfiler, which isn't translated (it keeps an atomic value in an object's field) (com/sonicether/soundphysics/SoundPhysics.java:52)
- [ ] `SoundPhysics.lastSoundCategory`: it uses Minecraft's `SoundSource` (com/sonicether/soundphysics/SoundPhysics.java:53)
- [x] `SoundPhysics.lastSound` (com/sonicether/soundphysics/SoundPhysics.java:54)
- [x] `SoundPhysics.referenceDistanceBug` (com/sonicether/soundphysics/SoundPhysics.java:56)
- [x] `SoundPhysics.maxAuxSends` (com/sonicether/soundphysics/SoundPhysics.java:57)
- [ ] `SoundPhysics.init(0)`: it calls Loggers.log, which isn't translated (com/sonicether/soundphysics/SoundPhysics.java:59)
- [ ] `SoundPhysics.syncReverbParams(0)`: it calls SoundPhysics.setReverbParams, which isn't translated (com/sonicether/soundphysics/SoundPhysics.java:67)
- [ ] `SoundPhysics.setupEFX(0)`: it calls `ALC10` (org.lwjgl.openal).alcGetCurrentContext (com/sonicether/soundphysics/SoundPhysics.java:76)
- [ ] `SoundPhysics.applyReferenceDistanceWorkaround(2)`: it calls `AL10` (org.lwjgl.openal).alSourcef (com/sonicether/soundphysics/SoundPhysics.java:130)
- [ ] `SoundPhysics.hasReferenceDistanceBug(0)`: it calls `AL10` (org.lwjgl.openal).alGetString (com/sonicether/soundphysics/SoundPhysics.java:136)
- [ ] `SoundPhysics.setLastSoundCategoryAndName(2)`: it uses Minecraft's `SoundSource` (com/sonicether/soundphysics/SoundPhysics.java:145)
- [ ] `SoundPhysics.onPlaySound(4)`: it's overloaded with the same number of parameters (com/sonicether/soundphysics/SoundPhysics.java:150)
- [ ] `SoundPhysics.onPlayReverb(4)`: it's overloaded with the same number of parameters (com/sonicether/soundphysics/SoundPhysics.java:154)
- [ ] `SoundPhysics.processSound(6)`: it uses Minecraft's `SoundSource` (com/sonicether/soundphysics/SoundPhysics.java:158)
- [ ] `SoundPhysics.processSound(7)`: it's overloaded with the same number of parameters (com/sonicether/soundphysics/SoundPhysics.java:162)
- [ ] `SoundPhysics.processSound(7)`: it's overloaded with the same number of parameters (com/sonicether/soundphysics/SoundPhysics.java:167)
- [ ] `SoundPhysics.evaluateEnvironment(7)`: its body couldn't be read: an assert (com/sonicether/soundphysics/SoundPhysics.java:184)
- [x] `SoundPhysics.isAmbientSound(1)` (com/sonicether/soundphysics/SoundPhysics.java:397)
- [ ] `SoundPhysics.getBlockReflectivity(1)`: it uses Minecraft's `BlockPos` (com/sonicether/soundphysics/SoundPhysics.java:401)
- [ ] `SoundPhysics.reflect(2)`: it calls Minecraft's Vec3.dot (no Horizon helper for it yet) (com/sonicether/soundphysics/SoundPhysics.java:411)
- [ ] `SoundPhysics.calculateOcclusion(4)`: it uses Minecraft's `SoundSource` (com/sonicether/soundphysics/SoundPhysics.java:419)
- [ ] `SoundPhysics.runOcclusion(2)`: it uses ClientLevelProxy, which isn't translated (it's an interface) (com/sonicether/soundphysics/SoundPhysics.java:446)
- [ ] `SoundPhysics.getLevelProxy(0)`: it uses ClientLevelProxy, which isn't translated (it's an interface) (com/sonicether/soundphysics/SoundPhysics.java:492)
- [ ] `SoundPhysics.getSharedAirspace(2)`: it's overloaded with the same number of parameters (com/sonicether/soundphysics/SoundPhysics.java:496)
- [ ] `SoundPhysics.getSharedAirspace(2)`: it's overloaded with the same number of parameters (com/sonicether/soundphysics/SoundPhysics.java:505)
- [ ] `SoundPhysics.setDefaultEnvironment(1)`: it calls SoundPhysics.setDefaultEnvironment, which isn't translated (com/sonicether/soundphysics/SoundPhysics.java:516)
- [ ] `SoundPhysics.setDefaultEnvironment(2)`: it calls SoundPhysics.setEnvironment, which isn't translated (com/sonicether/soundphysics/SoundPhysics.java:520)
- [ ] `SoundPhysics.setEnvironment(11)`: it uses SoundPhysicsMod.CONFIG, which isn't translated (it uses SoundPhysicsConfig, which isn't translated (it uses ConfigEntry, which isn't translated (it's an interface))) (com/sonicether/soundphysics/SoundPhysics.java:524)
- [ ] `SoundPhysics.setSoundPos(2)`: it calls `AL11` (org.lwjgl.openal).alSource3f (com/sonicether/soundphysics/SoundPhysics.java:575)
- [ ] `SoundPhysics.setReverbParams(3)`: it calls `EXTEfx` (org.lwjgl.openal).alEffectf (com/sonicether/soundphysics/SoundPhysics.java:579)

### `SoundPhysicsMod`

- [x] `SoundPhysicsMod.MODID` (com/sonicether/soundphysics/SoundPhysicsMod.java:15)
- [ ] `SoundPhysicsMod.CONFIG`: it uses SoundPhysicsConfig, which isn't translated (it uses ConfigEntry, which isn't translated (it's an interface)) (com/sonicether/soundphysics/SoundPhysicsMod.java:16)
- [ ] `SoundPhysicsMod.REFLECTIVITY_CONFIG`: it uses ReflectivityConfig, which isn't translated (it extends `BlockSoundConfigBase` (com.sonicether.soundphysics.config.blocksound)) (com/sonicether/soundphysics/SoundPhysicsMod.java:17)
- [ ] `SoundPhysicsMod.OCCLUSION_CONFIG`: it uses OcclusionConfig, which isn't translated (it extends `BlockSoundConfigBase` (com.sonicether.soundphysics.config.blocksound)) (com/sonicether/soundphysics/SoundPhysicsMod.java:18)
- [ ] `SoundPhysicsMod.SOUND_RATE_CONFIG`: it uses SoundRateConfig, which isn't translated (it extends `CommentedPropertyConfig` (de.maxhenkel.sound_physics_remastered.configbuilder)) (com/sonicether/soundphysics/SoundPhysicsMod.java:19)
- [ ] `SoundPhysicsMod.init(0)`: it calls SoundPhysicsMod.initConfig, which isn't translated (com/sonicether/soundphysics/SoundPhysicsMod.java:21)
- [ ] `SoundPhysicsMod.initClient(0)`: it uses SoundPhysicsMod.CONFIG, which isn't translated (it uses SoundPhysicsConfig, which isn't translated (it uses ConfigEntry, which isn't translated (it's an interface))) (com/sonicether/soundphysics/SoundPhysicsMod.java:25)
- [ ] `SoundPhysicsMod.initConfig(0)`: it uses SoundPhysicsMod.CONFIG, which isn't translated (it uses SoundPhysicsConfig, which isn't translated (it uses ConfigEntry, which isn't translated (it's an interface))) (com/sonicether/soundphysics/SoundPhysicsMod.java:34)
- [ ] `SoundPhysicsMod.renameAllowedSounds(0)`: a try/catch (NoSuchFileException, FileAlreadyExistsException, IOException) (com/sonicether/soundphysics/SoundPhysicsMod.java:42)
- [ ] `SoundPhysicsMod.getConfigFolder(0)`: it has no body (com/sonicether/soundphysics/SoundPhysicsMod.java:55)

### `LevelAccessUtils`

- [ ] `LevelAccessUtils.PROFILER`: it uses TaskProfiler, which isn't translated (it keeps an atomic value in an object's field) (com/sonicether/soundphysics/utils/LevelAccessUtils.java:17)
- [ ] `LevelAccessUtils.MC`: it uses Minecraft's `Minecraft` (com/sonicether/soundphysics/utils/LevelAccessUtils.java:18)
- [ ] `LevelAccessUtils.onLoadLevel(1)`: it uses Minecraft's `ClientLevel` (com/sonicether/soundphysics/utils/LevelAccessUtils.java:20)
- [ ] `LevelAccessUtils.onUnloadLevel(1)`: it uses Minecraft's `ClientLevel` (com/sonicether/soundphysics/utils/LevelAccessUtils.java:26)
- [ ] `LevelAccessUtils.tickLevelCache(1)`: it uses Minecraft's `ClientLevel` (com/sonicether/soundphysics/utils/LevelAccessUtils.java:32)
- [ ] `LevelAccessUtils.updateLevelCache(3)`: it uses Minecraft's `ClientLevel` (com/sonicether/soundphysics/utils/LevelAccessUtils.java:61)
- [ ] `LevelAccessUtils.getClientLevelProxy(1)`: it uses ClientLevelProxy, which isn't translated (it's an interface) (com/sonicether/soundphysics/utils/LevelAccessUtils.java:72)
- [ ] `LevelAccessUtils.levelOriginFromPlayer(0)`: it uses Minecraft's `BlockPos` (com/sonicether/soundphysics/utils/LevelAccessUtils.java:92)

### `RaycastUtils`

- [ ] `RaycastUtils.rayCast(4)`: it uses Minecraft's `BlockHitResult` (com/sonicether/soundphysics/utils/RaycastUtils.java:16)

### `SoundRateManager`

- [x] `SoundRateManager.soundCounts` (com/sonicether/soundphysics/utils/SoundRateManager.java:10)
- [x] `SoundRateManager.worldInitialized` (com/sonicether/soundphysics/utils/SoundRateManager.java:11)
- [x] `SoundRateManager.SoundRateManager(0)` (com/sonicether/soundphysics/utils/SoundRateManager.java:13)
- [ ] `SoundRateManager.incrementAndCheckLimit(1)`: it uses SoundPhysicsMod.SOUND_RATE_CONFIG, which isn't translated (it uses SoundRateConfig, which isn't translated (it extends `CommentedPropertyConfig` (de.maxhenkel.sound_physics_remastered.configbuilder))) (com/sonicether/soundphysics/utils/SoundRateManager.java:16)
- [x] `SoundRateManager.isWorldInitialized(0)` (com/sonicether/soundphysics/utils/SoundRateManager.java:27)
- [ ] `SoundRateManager.onClientTick(1)`: it uses Minecraft's `ClientLevel` (com/sonicether/soundphysics/utils/SoundRateManager.java:31)
- [x] `SoundRateManager.clear(0)` (com/sonicether/soundphysics/utils/SoundRateManager.java:36)
- [ ] `SoundRateManager.onLoadLevel(1)`: it uses Minecraft's `ClientLevel` (com/sonicether/soundphysics/utils/SoundRateManager.java:40)
- [ ] `SoundRateManager.onUnloadLevel(1)`: it uses Minecraft's `ClientLevel` (com/sonicether/soundphysics/utils/SoundRateManager.java:45)

### `SoundUtils`

- [x] `SoundUtils.STEP_PATTERN` (com/sonicether/soundphysics/utils/SoundUtils.java:7)
- [ ] `SoundUtils.calculateEntitySoundYOffset(2)`: it uses Minecraft's `SoundEvent` (com/sonicether/soundphysics/utils/SoundUtils.java:9)

### `CachingClientLevel`: it's an interface

- [ ] `CachingClientLevel.sound_physics_remastered$getCachedClone(0)`: it has no body (com/sonicether/soundphysics/world/CachingClientLevel.java:6)
- [ ] `CachingClientLevel.sound_physics_remastered$setCachedClone(1)`: it has no body (com/sonicether/soundphysics/world/CachingClientLevel.java:9)

### `ClientLevelProxy`: it's an interface


### `ClonedClientLevel`: it implements `ClientLevelProxy`

- [ ] `ClonedClientLevel.ClonedClientLevel(4)`: its class isn't translated (it implements `ClientLevelProxy`) (com/sonicether/soundphysics/world/ClonedClientLevel.java:22)
- [ ] `ClonedClientLevel.getOrigin(0)`: its class isn't translated (it implements `ClientLevelProxy`) (com/sonicether/soundphysics/world/ClonedClientLevel.java:45)
- [ ] `ClonedClientLevel.getTick(0)`: its class isn't translated (it implements `ClientLevelProxy`) (com/sonicether/soundphysics/world/ClonedClientLevel.java:49)
- [ ] `ClonedClientLevel.getChunk(2)`: its class isn't translated (it implements `ClientLevelProxy`) (com/sonicether/soundphysics/world/ClonedClientLevel.java:53)
- [ ] `ClonedClientLevel.getBlockState(1)`: its class isn't translated (it implements `ClientLevelProxy`) (com/sonicether/soundphysics/world/ClonedClientLevel.java:58)
- [ ] `ClonedClientLevel.getFluidState(1)`: its class isn't translated (it implements `ClientLevelProxy`) (com/sonicether/soundphysics/world/ClonedClientLevel.java:68)
- [ ] `ClonedClientLevel.getHeight(0)`: its class isn't translated (it implements `ClientLevelProxy`) (com/sonicether/soundphysics/world/ClonedClientLevel.java:78)
- [ ] `ClonedClientLevel.getMinY(0)`: its class isn't translated (it implements `ClientLevelProxy`) (com/sonicether/soundphysics/world/ClonedClientLevel.java:82)
- [ ] `ClonedClientLevel.getBlockEntity(1)`: its class isn't translated (it implements `ClientLevelProxy`) (com/sonicether/soundphysics/world/ClonedClientLevel.java:86)

### `ClonedLevelChunk`: it extends Minecraft's `ChunkAccess`

- [ ] `ClonedLevelChunk.ClonedLevelChunk(3)`: its class isn't translated (it extends Minecraft's `ChunkAccess`) (com/sonicether/soundphysics/world/ClonedLevelChunk.java:37)
- [ ] `ClonedLevelChunk.getBlockEntity(1)`: its class isn't translated (it extends Minecraft's `ChunkAccess`) (com/sonicether/soundphysics/world/ClonedLevelChunk.java:51)
- [ ] `ClonedLevelChunk.getBlockState(1)`: its class isn't translated (it extends Minecraft's `ChunkAccess`) (com/sonicether/soundphysics/world/ClonedLevelChunk.java:55)
- [ ] `ClonedLevelChunk.getFluidState(1)`: its class isn't translated (it extends Minecraft's `ChunkAccess`) (com/sonicether/soundphysics/world/ClonedLevelChunk.java:64)
- [ ] `ClonedLevelChunk.withLevelChunkSectionAtPosition(2)`: it's generic (com/sonicether/soundphysics/world/ClonedLevelChunk.java:73)
- [ ] `ClonedLevelChunk.getBlockTicks(0)`: its class isn't translated (it extends Minecraft's `ChunkAccess`) (com/sonicether/soundphysics/world/ClonedLevelChunk.java:90)
- [ ] `ClonedLevelChunk.getFluidTicks(0)`: its class isn't translated (it extends Minecraft's `ChunkAccess`) (com/sonicether/soundphysics/world/ClonedLevelChunk.java:94)
- [ ] `ClonedLevelChunk.getTicksForSerialization(1)`: its class isn't translated (it extends Minecraft's `ChunkAccess`) (com/sonicether/soundphysics/world/ClonedLevelChunk.java:98)
- [ ] `ClonedLevelChunk.getPersistedStatus(0)`: its class isn't translated (it extends Minecraft's `ChunkAccess`) (com/sonicether/soundphysics/world/ClonedLevelChunk.java:102)
- [ ] `ClonedLevelChunk.addEntity(1)`: its class isn't translated (it extends Minecraft's `ChunkAccess`) (com/sonicether/soundphysics/world/ClonedLevelChunk.java:106)
- [ ] `ClonedLevelChunk.getBlockEntityNbtForSaving(2)`: its class isn't translated (it extends Minecraft's `ChunkAccess`) (com/sonicether/soundphysics/world/ClonedLevelChunk.java:110)
- [ ] `ClonedLevelChunk.removeBlockEntity(1)`: its class isn't translated (it extends Minecraft's `ChunkAccess`) (com/sonicether/soundphysics/world/ClonedLevelChunk.java:114)
- [ ] `ClonedLevelChunk.setBlockState(3)`: its class isn't translated (it extends Minecraft's `ChunkAccess`) (com/sonicether/soundphysics/world/ClonedLevelChunk.java:118)
- [ ] `ClonedLevelChunk.setBlockEntity(1)`: its class isn't translated (it extends Minecraft's `ChunkAccess`) (com/sonicether/soundphysics/world/ClonedLevelChunk.java:123)

### `ClonedLevelHeightAccessor`: it implements Minecraft's `LevelHeightAccessor`

- [ ] `ClonedLevelHeightAccessor.ClonedLevelHeightAccessor(1)`: its class isn't translated (it implements Minecraft's `LevelHeightAccessor`) (com/sonicether/soundphysics/world/ClonedLevelHeightAccessor.java:10)
- [ ] `ClonedLevelHeightAccessor.getHeight(0)`: its class isn't translated (it implements Minecraft's `LevelHeightAccessor`) (com/sonicether/soundphysics/world/ClonedLevelHeightAccessor.java:15)
- [ ] `ClonedLevelHeightAccessor.getMinY(0)`: its class isn't translated (it implements Minecraft's `LevelHeightAccessor`) (com/sonicether/soundphysics/world/ClonedLevelHeightAccessor.java:19)
- [ ] `ClonedLevelHeightAccessor.getMinBuildHeight(0)`: its class isn't translated (it implements Minecraft's `LevelHeightAccessor`) (com/sonicether/soundphysics/world/ClonedLevelHeightAccessor.java:23)

### `UnsafeClientLevel`: it implements `ClientLevelProxy`

- [ ] `UnsafeClientLevel.UnsafeClientLevel(1)`: its class isn't translated (it implements `ClientLevelProxy`) (com/sonicether/soundphysics/world/UnsafeClientLevel.java:13)
- [ ] `UnsafeClientLevel.getBlockEntity(1)`: its class isn't translated (it implements `ClientLevelProxy`) (com/sonicether/soundphysics/world/UnsafeClientLevel.java:17)
- [ ] `UnsafeClientLevel.getBlockState(1)`: its class isn't translated (it implements `ClientLevelProxy`) (com/sonicether/soundphysics/world/UnsafeClientLevel.java:21)
- [ ] `UnsafeClientLevel.getFluidState(1)`: its class isn't translated (it implements `ClientLevelProxy`) (com/sonicether/soundphysics/world/UnsafeClientLevel.java:25)
- [ ] `UnsafeClientLevel.getHeight(0)`: its class isn't translated (it implements `ClientLevelProxy`) (com/sonicether/soundphysics/world/UnsafeClientLevel.java:29)
- [ ] `UnsafeClientLevel.getMinY(0)`: its class isn't translated (it implements `ClientLevelProxy`) (com/sonicether/soundphysics/world/UnsafeClientLevel.java:33)

### `CommentedProperties`: it implements `Map` (java.util)

- [ ] `CommentedProperties.CommentedProperties(1)`: its class isn't translated (it implements `Map` (java.util)) (de/maxhenkel/sound_physics_remastered/configbuilder/CommentedProperties.java:31)
- [ ] `CommentedProperties.CommentedProperties(0)`: its class isn't translated (it implements `Map` (java.util)) (de/maxhenkel/sound_physics_remastered/configbuilder/CommentedProperties.java:37)
- [ ] `CommentedProperties.addHeaderComment(1)`: its class isn't translated (it implements `Map` (java.util)) (de/maxhenkel/sound_physics_remastered/configbuilder/CommentedProperties.java:41)
- [ ] `CommentedProperties.setHeaderComments(1)`: its class isn't translated (it implements `Map` (java.util)) (de/maxhenkel/sound_physics_remastered/configbuilder/CommentedProperties.java:46)
- [ ] `CommentedProperties.sort(1)`: its class isn't translated (it implements `Map` (java.util)) (de/maxhenkel/sound_physics_remastered/configbuilder/CommentedProperties.java:52)
- [ ] `CommentedProperties.get(1)`: its class isn't translated (it implements `Map` (java.util)) (de/maxhenkel/sound_physics_remastered/configbuilder/CommentedProperties.java:62)
- [ ] `CommentedProperties.getComments(1)`: its class isn't translated (it implements `Map` (java.util)) (de/maxhenkel/sound_physics_remastered/configbuilder/CommentedProperties.java:69)
- [ ] `CommentedProperties.setComments(2)`: its class isn't translated (it implements `Map` (java.util)) (de/maxhenkel/sound_physics_remastered/configbuilder/CommentedProperties.java:76)
- [ ] `CommentedProperties.set(3)`: its class isn't translated (it implements `Map` (java.util)) (de/maxhenkel/sound_physics_remastered/configbuilder/CommentedProperties.java:88)
- [ ] `CommentedProperties.load(1)`: its class isn't translated (it implements `Map` (java.util)) (de/maxhenkel/sound_physics_remastered/configbuilder/CommentedProperties.java:95)
- [ ] `CommentedProperties.readLine(1)`: it uses `StringReader` (java.io) (de/maxhenkel/sound_physics_remastered/configbuilder/CommentedProperties.java:147)
- [ ] `CommentedProperties.readEscapedCharacter(2)`: it uses `StringReader` (java.io) (de/maxhenkel/sound_physics_remastered/configbuilder/CommentedProperties.java:214)
- [x] `CommentedProperties.isWhitespace(1)` (de/maxhenkel/sound_physics_remastered/configbuilder/CommentedProperties.java:252)
- [x] `CommentedProperties.isSeparator(1)` (de/maxhenkel/sound_physics_remastered/configbuilder/CommentedProperties.java:256)
- [ ] `CommentedProperties.save(1)`: its class isn't translated (it implements `Map` (java.util)) (de/maxhenkel/sound_physics_remastered/configbuilder/CommentedProperties.java:260)
- [x] `CommentedProperties.removeNewLines(1)` (de/maxhenkel/sound_physics_remastered/configbuilder/CommentedProperties.java:299)
- [ ] `CommentedProperties.escapeKey(1)`: its class isn't translated (it implements `Map` (java.util)) (de/maxhenkel/sound_physics_remastered/configbuilder/CommentedProperties.java:309)
- [ ] `CommentedProperties.escapeValue(1)`: its class isn't translated (it implements `Map` (java.util)) (de/maxhenkel/sound_physics_remastered/configbuilder/CommentedProperties.java:316)
- [ ] `CommentedProperties.escape(1)`: its class isn't translated (it implements `Map` (java.util)) (de/maxhenkel/sound_physics_remastered/configbuilder/CommentedProperties.java:330)
- [ ] `CommentedProperties.size(0)`: its class isn't translated (it implements `Map` (java.util)) (de/maxhenkel/sound_physics_remastered/configbuilder/CommentedProperties.java:348)
- [ ] `CommentedProperties.isEmpty(0)`: its class isn't translated (it implements `Map` (java.util)) (de/maxhenkel/sound_physics_remastered/configbuilder/CommentedProperties.java:353)
- [ ] `CommentedProperties.containsKey(1)`: its class isn't translated (it implements `Map` (java.util)) (de/maxhenkel/sound_physics_remastered/configbuilder/CommentedProperties.java:358)
- [ ] `CommentedProperties.containsValue(1)`: its class isn't translated (it implements `Map` (java.util)) (de/maxhenkel/sound_physics_remastered/configbuilder/CommentedProperties.java:363)
- [ ] `CommentedProperties.get(1)`: its class isn't translated (it implements `Map` (java.util)) (de/maxhenkel/sound_physics_remastered/configbuilder/CommentedProperties.java:375)
- [ ] `CommentedProperties.put(2)`: its class isn't translated (it implements `Map` (java.util)) (de/maxhenkel/sound_physics_remastered/configbuilder/CommentedProperties.java:381)
- [ ] `CommentedProperties.remove(1)`: its class isn't translated (it implements `Map` (java.util)) (de/maxhenkel/sound_physics_remastered/configbuilder/CommentedProperties.java:387)
- [ ] `CommentedProperties.putAll(1)`: its class isn't translated (it implements `Map` (java.util)) (de/maxhenkel/sound_physics_remastered/configbuilder/CommentedProperties.java:392)
- [ ] `CommentedProperties.clear(0)`: its class isn't translated (it implements `Map` (java.util)) (de/maxhenkel/sound_physics_remastered/configbuilder/CommentedProperties.java:400)
- [ ] `CommentedProperties.keySet(0)`: its class isn't translated (it implements `Map` (java.util)) (de/maxhenkel/sound_physics_remastered/configbuilder/CommentedProperties.java:406)
- [ ] `CommentedProperties.values(0)`: its class isn't translated (it implements `Map` (java.util)) (de/maxhenkel/sound_physics_remastered/configbuilder/CommentedProperties.java:411)
- [ ] `CommentedProperties.entrySet(0)`: its class isn't translated (it implements `Map` (java.util)) (de/maxhenkel/sound_physics_remastered/configbuilder/CommentedProperties.java:417)

### `CommentedProperties.LineReader`: it implements `Closeable` (java.io)

- [ ] `CommentedProperties.LineReader.LineReader(1)`: its class isn't translated (it implements `Closeable` (java.io)) (de/maxhenkel/sound_physics_remastered/configbuilder/CommentedProperties.java:426)
- [ ] `CommentedProperties.LineReader.fromInputStream(1)`: it uses LineReader, which isn't translated (it implements `Closeable` (java.io)) (de/maxhenkel/sound_physics_remastered/configbuilder/CommentedProperties.java:430)
- [ ] `CommentedProperties.LineReader.nextLine(0)`: its class isn't translated (it implements `Closeable` (java.io)) (de/maxhenkel/sound_physics_remastered/configbuilder/CommentedProperties.java:434)
- [ ] `CommentedProperties.LineReader.close(0)`: its class isn't translated (it implements `Closeable` (java.io)) (de/maxhenkel/sound_physics_remastered/configbuilder/CommentedProperties.java:448)

### `CommentedProperties.Pair`

- [x] `CommentedProperties.Pair.Pair(2)` (de/maxhenkel/sound_physics_remastered/configbuilder/CommentedProperties.java:458)

### `CommentedProperties.Property`

- [x] `CommentedProperties.Property.Property(2)` (de/maxhenkel/sound_physics_remastered/configbuilder/CommentedProperties.java:468)
- [x] `CommentedProperties.Property.Property(1)` (de/maxhenkel/sound_physics_remastered/configbuilder/CommentedProperties.java:473)

### `CommentedPropertyConfig`: it implements `Config`

- [x] `CommentedPropertyConfig.LOGGER` (de/maxhenkel/sound_physics_remastered/configbuilder/CommentedPropertyConfig.java:18)
- [ ] `CommentedPropertyConfig.SAVE_EXECUTOR_SERVICE`: it uses `ExecutorService` (java.util.concurrent) (de/maxhenkel/sound_physics_remastered/configbuilder/CommentedPropertyConfig.java:19)
- [ ] `CommentedPropertyConfig.CommentedPropertyConfig(1)`: its class isn't translated (it implements `Config`) (de/maxhenkel/sound_physics_remastered/configbuilder/CommentedPropertyConfig.java:29)
- [ ] `CommentedPropertyConfig.builder(0)`: it creates a Builder with a constructor that isn't translated (de/maxhenkel/sound_physics_remastered/configbuilder/CommentedPropertyConfig.java:33)
- [ ] `CommentedPropertyConfig.get(1)`: its class isn't translated (it implements `Config`) (de/maxhenkel/sound_physics_remastered/configbuilder/CommentedPropertyConfig.java:37)
- [ ] `CommentedPropertyConfig.set(3)`: its class isn't translated (it implements `Config`) (de/maxhenkel/sound_physics_remastered/configbuilder/CommentedPropertyConfig.java:41)
- [ ] `CommentedPropertyConfig.getProperties(0)`: its class isn't translated (it implements `Config`) (de/maxhenkel/sound_physics_remastered/configbuilder/CommentedPropertyConfig.java:45)
- [ ] `CommentedPropertyConfig.load(0)`: its class isn't translated (it implements `Config`) (de/maxhenkel/sound_physics_remastered/configbuilder/CommentedPropertyConfig.java:49)
- [ ] `CommentedPropertyConfig.reload(0)`: its class isn't translated (it implements `Config`) (de/maxhenkel/sound_physics_remastered/configbuilder/CommentedPropertyConfig.java:75)
- [ ] `CommentedPropertyConfig.saveSync(0)`: its class isn't translated (it implements `Config`) (de/maxhenkel/sound_physics_remastered/configbuilder/CommentedPropertyConfig.java:85)
- [ ] `CommentedPropertyConfig.save(0)`: its class isn't translated (it implements `Config`) (de/maxhenkel/sound_physics_remastered/configbuilder/CommentedPropertyConfig.java:119)
- [ ] `CommentedPropertyConfig.getEntries(0)`: its class isn't translated (it implements `Config`) (de/maxhenkel/sound_physics_remastered/configbuilder/CommentedPropertyConfig.java:125)

### `CommentedPropertyConfig.Builder`

- [ ] `CommentedPropertyConfig.Builder.Builder(0)`: a Path with no value (null in Java) (de/maxhenkel/sound_physics_remastered/configbuilder/CommentedPropertyConfig.java:135)
- [ ] `CommentedPropertyConfig.Builder.path(1)`: `this` used as a value (de/maxhenkel/sound_physics_remastered/configbuilder/CommentedPropertyConfig.java:138)
- [ ] `CommentedPropertyConfig.Builder.strict(1)`: `this` used as a value (de/maxhenkel/sound_physics_remastered/configbuilder/CommentedPropertyConfig.java:143)
- [ ] `CommentedPropertyConfig.Builder.build(0)`: it uses CommentedPropertyConfig, which isn't translated (it implements `Config`) (de/maxhenkel/sound_physics_remastered/configbuilder/CommentedPropertyConfig.java:148)

### `Config`: it's an interface

- [ ] `Config.getEntries(0)`: it has no body (de/maxhenkel/sound_physics_remastered/configbuilder/Config.java:6)

### `ConfigBuilder`: it's an interface

- [ ] `ConfigBuilder.header(1)`: it has no body (de/maxhenkel/sound_physics_remastered/configbuilder/ConfigBuilder.java:20)
- [ ] `ConfigBuilder.booleanEntry(3)`: it has no body (de/maxhenkel/sound_physics_remastered/configbuilder/ConfigBuilder.java:22)
- [ ] `ConfigBuilder.integerEntry(5)`: it has no body (de/maxhenkel/sound_physics_remastered/configbuilder/ConfigBuilder.java:24)
- [ ] `ConfigBuilder.integerEntry(3)`: its class isn't translated (it's an interface) (de/maxhenkel/sound_physics_remastered/configbuilder/ConfigBuilder.java:26)
- [ ] `ConfigBuilder.longEntry(5)`: it has no body (de/maxhenkel/sound_physics_remastered/configbuilder/ConfigBuilder.java:30)
- [ ] `ConfigBuilder.longEntry(3)`: its class isn't translated (it's an interface) (de/maxhenkel/sound_physics_remastered/configbuilder/ConfigBuilder.java:32)
- [ ] `ConfigBuilder.doubleEntry(5)`: it has no body (de/maxhenkel/sound_physics_remastered/configbuilder/ConfigBuilder.java:36)
- [ ] `ConfigBuilder.doubleEntry(3)`: its class isn't translated (it's an interface) (de/maxhenkel/sound_physics_remastered/configbuilder/ConfigBuilder.java:38)
- [ ] `ConfigBuilder.floatEntry(5)`: it has no body (de/maxhenkel/sound_physics_remastered/configbuilder/ConfigBuilder.java:42)
- [ ] `ConfigBuilder.floatEntry(3)`: its class isn't translated (it's an interface) (de/maxhenkel/sound_physics_remastered/configbuilder/ConfigBuilder.java:44)
- [ ] `ConfigBuilder.stringEntry(3)`: it has no body (de/maxhenkel/sound_physics_remastered/configbuilder/ConfigBuilder.java:48)
- [ ] `ConfigBuilder.enumEntry(3)`: it has no body (de/maxhenkel/sound_physics_remastered/configbuilder/ConfigBuilder.java:50)
- [ ] `ConfigBuilder.entry(3)`: it has no body (de/maxhenkel/sound_physics_remastered/configbuilder/ConfigBuilder.java:52)
- [ ] `ConfigBuilder.builder(1)`: it's generic (de/maxhenkel/sound_physics_remastered/configbuilder/ConfigBuilder.java:54)

### `ConfigBuilder.Builder`: it's generic

- [ ] `ConfigBuilder.Builder.Builder(1)`: its class isn't translated (it's generic) (de/maxhenkel/sound_physics_remastered/configbuilder/ConfigBuilder.java:70)
- [ ] `ConfigBuilder.Builder.path(1)`: its class isn't translated (it's generic) (de/maxhenkel/sound_physics_remastered/configbuilder/ConfigBuilder.java:80)
- [ ] `ConfigBuilder.Builder.addValueSerializer(2)`: it's generic (de/maxhenkel/sound_physics_remastered/configbuilder/ConfigBuilder.java:85)
- [ ] `ConfigBuilder.Builder.removeUnused(1)`: its class isn't translated (it's generic) (de/maxhenkel/sound_physics_remastered/configbuilder/ConfigBuilder.java:90)
- [ ] `ConfigBuilder.Builder.strict(1)`: its class isn't translated (it's generic) (de/maxhenkel/sound_physics_remastered/configbuilder/ConfigBuilder.java:95)
- [ ] `ConfigBuilder.Builder.keepOrder(1)`: its class isn't translated (it's generic) (de/maxhenkel/sound_physics_remastered/configbuilder/ConfigBuilder.java:100)
- [ ] `ConfigBuilder.Builder.saveAfterBuild(1)`: its class isn't translated (it's generic) (de/maxhenkel/sound_physics_remastered/configbuilder/ConfigBuilder.java:105)
- [ ] `ConfigBuilder.Builder.saveSyncAfterBuild(1)`: its class isn't translated (it's generic) (de/maxhenkel/sound_physics_remastered/configbuilder/ConfigBuilder.java:114)
- [ ] `ConfigBuilder.Builder.build(0)`: its class isn't translated (it's generic) (de/maxhenkel/sound_physics_remastered/configbuilder/ConfigBuilder.java:123)

### `ConfigBuilderImpl`: it implements `ConfigBuilder`

- [ ] `ConfigBuilderImpl.ConfigBuilderImpl(1)`: its class isn't translated (it implements `ConfigBuilder`) (de/maxhenkel/sound_physics_remastered/configbuilder/ConfigBuilderImpl.java:47)
- [ ] `ConfigBuilderImpl.ConfigBuilderImpl(2)`: its class isn't translated (it implements `ConfigBuilder`) (de/maxhenkel/sound_physics_remastered/configbuilder/ConfigBuilderImpl.java:51)
- [ ] `ConfigBuilderImpl.removeUnused(0)`: its class isn't translated (it implements `ConfigBuilder`) (de/maxhenkel/sound_physics_remastered/configbuilder/ConfigBuilderImpl.java:61)
- [ ] `ConfigBuilderImpl.sortEntries(0)`: its class isn't translated (it implements `ConfigBuilder`) (de/maxhenkel/sound_physics_remastered/configbuilder/ConfigBuilderImpl.java:69)
- [ ] `ConfigBuilderImpl.reloadFromDisk(0)`: its class isn't translated (it implements `ConfigBuilder`) (de/maxhenkel/sound_physics_remastered/configbuilder/ConfigBuilderImpl.java:79)
- [ ] `ConfigBuilderImpl.freeze(0)`: its class isn't translated (it implements `ConfigBuilder`) (de/maxhenkel/sound_physics_remastered/configbuilder/ConfigBuilderImpl.java:84)
- [ ] `ConfigBuilderImpl.checkFrozen(0)`: its class isn't translated (it implements `ConfigBuilder`) (de/maxhenkel/sound_physics_remastered/configbuilder/ConfigBuilderImpl.java:88)
- [ ] `ConfigBuilderImpl.header(1)`: its class isn't translated (it implements `ConfigBuilder`) (de/maxhenkel/sound_physics_remastered/configbuilder/ConfigBuilderImpl.java:94)
- [ ] `ConfigBuilderImpl.booleanEntry(3)`: its class isn't translated (it implements `ConfigBuilder`) (de/maxhenkel/sound_physics_remastered/configbuilder/ConfigBuilderImpl.java:100)
- [ ] `ConfigBuilderImpl.integerEntry(5)`: its class isn't translated (it implements `ConfigBuilder`) (de/maxhenkel/sound_physics_remastered/configbuilder/ConfigBuilderImpl.java:108)
- [ ] `ConfigBuilderImpl.longEntry(5)`: its class isn't translated (it implements `ConfigBuilder`) (de/maxhenkel/sound_physics_remastered/configbuilder/ConfigBuilderImpl.java:116)
- [ ] `ConfigBuilderImpl.doubleEntry(5)`: its class isn't translated (it implements `ConfigBuilder`) (de/maxhenkel/sound_physics_remastered/configbuilder/ConfigBuilderImpl.java:124)
- [ ] `ConfigBuilderImpl.floatEntry(5)`: its class isn't translated (it implements `ConfigBuilder`) (de/maxhenkel/sound_physics_remastered/configbuilder/ConfigBuilderImpl.java:132)
- [ ] `ConfigBuilderImpl.stringEntry(3)`: its class isn't translated (it implements `ConfigBuilder`) (de/maxhenkel/sound_physics_remastered/configbuilder/ConfigBuilderImpl.java:140)
- [ ] `ConfigBuilderImpl.enumEntry(3)`: it's generic (de/maxhenkel/sound_physics_remastered/configbuilder/ConfigBuilderImpl.java:148)
- [ ] `ConfigBuilderImpl.entry(3)`: it's generic (de/maxhenkel/sound_physics_remastered/configbuilder/ConfigBuilderImpl.java:156)
- [ ] `ConfigBuilderImpl.entryInternal(3)`: it's generic (de/maxhenkel/sound_physics_remastered/configbuilder/ConfigBuilderImpl.java:164)
- [ ] `ConfigBuilderImpl.getDefaultValueSerializers(0)`: it uses `Class` (de/maxhenkel/sound_physics_remastered/configbuilder/ConfigBuilderImpl.java:191)

### `AbstractValueList`: it's generic

- [ ] `AbstractValueList.AbstractValueList(1)`: its class isn't translated (it's generic) (de/maxhenkel/sound_physics_remastered/configbuilder/custom/AbstractValueList.java:14)
- [ ] `AbstractValueList.AbstractValueList(1)`: its class isn't translated (it's generic) (de/maxhenkel/sound_physics_remastered/configbuilder/custom/AbstractValueList.java:18)
- [ ] `AbstractValueList.size(0)`: its class isn't translated (it's generic) (de/maxhenkel/sound_physics_remastered/configbuilder/custom/AbstractValueList.java:22)
- [ ] `AbstractValueList.isEmpty(0)`: its class isn't translated (it's generic) (de/maxhenkel/sound_physics_remastered/configbuilder/custom/AbstractValueList.java:27)
- [ ] `AbstractValueList.contains(1)`: its class isn't translated (it's generic) (de/maxhenkel/sound_physics_remastered/configbuilder/custom/AbstractValueList.java:32)
- [ ] `AbstractValueList.iterator(0)`: its class isn't translated (it's generic) (de/maxhenkel/sound_physics_remastered/configbuilder/custom/AbstractValueList.java:37)
- [ ] `AbstractValueList.toArray(0)`: its class isn't translated (it's generic) (de/maxhenkel/sound_physics_remastered/configbuilder/custom/AbstractValueList.java:42)
- [ ] `AbstractValueList.toArray(1)`: it's generic (de/maxhenkel/sound_physics_remastered/configbuilder/custom/AbstractValueList.java:47)
- [ ] `AbstractValueList.add(1)`: its class isn't translated (it's generic) (de/maxhenkel/sound_physics_remastered/configbuilder/custom/AbstractValueList.java:52)
- [ ] `AbstractValueList.remove(1)`: its class isn't translated (it's generic) (de/maxhenkel/sound_physics_remastered/configbuilder/custom/AbstractValueList.java:57)
- [ ] `AbstractValueList.containsAll(1)`: its class isn't translated (it's generic) (de/maxhenkel/sound_physics_remastered/configbuilder/custom/AbstractValueList.java:62)
- [ ] `AbstractValueList.addAll(1)`: its class isn't translated (it's generic) (de/maxhenkel/sound_physics_remastered/configbuilder/custom/AbstractValueList.java:67)
- [ ] `AbstractValueList.addAll(2)`: its class isn't translated (it's generic) (de/maxhenkel/sound_physics_remastered/configbuilder/custom/AbstractValueList.java:72)
- [ ] `AbstractValueList.removeAll(1)`: its class isn't translated (it's generic) (de/maxhenkel/sound_physics_remastered/configbuilder/custom/AbstractValueList.java:77)
- [ ] `AbstractValueList.retainAll(1)`: its class isn't translated (it's generic) (de/maxhenkel/sound_physics_remastered/configbuilder/custom/AbstractValueList.java:82)
- [ ] `AbstractValueList.clear(0)`: its class isn't translated (it's generic) (de/maxhenkel/sound_physics_remastered/configbuilder/custom/AbstractValueList.java:87)
- [ ] `AbstractValueList.get(1)`: its class isn't translated (it's generic) (de/maxhenkel/sound_physics_remastered/configbuilder/custom/AbstractValueList.java:92)
- [ ] `AbstractValueList.set(2)`: its class isn't translated (it's generic) (de/maxhenkel/sound_physics_remastered/configbuilder/custom/AbstractValueList.java:97)
- [ ] `AbstractValueList.add(2)`: its class isn't translated (it's generic) (de/maxhenkel/sound_physics_remastered/configbuilder/custom/AbstractValueList.java:102)
- [ ] `AbstractValueList.remove(1)`: its class isn't translated (it's generic) (de/maxhenkel/sound_physics_remastered/configbuilder/custom/AbstractValueList.java:107)
- [ ] `AbstractValueList.indexOf(1)`: its class isn't translated (it's generic) (de/maxhenkel/sound_physics_remastered/configbuilder/custom/AbstractValueList.java:112)
- [ ] `AbstractValueList.lastIndexOf(1)`: its class isn't translated (it's generic) (de/maxhenkel/sound_physics_remastered/configbuilder/custom/AbstractValueList.java:117)
- [ ] `AbstractValueList.listIterator(0)`: its class isn't translated (it's generic) (de/maxhenkel/sound_physics_remastered/configbuilder/custom/AbstractValueList.java:122)
- [ ] `AbstractValueList.listIterator(1)`: its class isn't translated (it's generic) (de/maxhenkel/sound_physics_remastered/configbuilder/custom/AbstractValueList.java:127)
- [ ] `AbstractValueList.subList(2)`: its class isn't translated (it's generic) (de/maxhenkel/sound_physics_remastered/configbuilder/custom/AbstractValueList.java:132)
- [ ] `AbstractValueList.equals(1)`: its class isn't translated (it's generic) (de/maxhenkel/sound_physics_remastered/configbuilder/custom/AbstractValueList.java:137)
- [ ] `AbstractValueList.hashCode(0)`: its class isn't translated (it's generic) (de/maxhenkel/sound_physics_remastered/configbuilder/custom/AbstractValueList.java:149)
- [ ] `AbstractValueList.throwException(0)`: it's generic (de/maxhenkel/sound_physics_remastered/configbuilder/custom/AbstractValueList.java:154)

### `AbstractValueMap`: it's generic

- [ ] `AbstractValueMap.AbstractValueMap(1)`: its class isn't translated (it's generic) (de/maxhenkel/sound_physics_remastered/configbuilder/custom/AbstractValueMap.java:13)
- [ ] `AbstractValueMap.size(0)`: its class isn't translated (it's generic) (de/maxhenkel/sound_physics_remastered/configbuilder/custom/AbstractValueMap.java:17)
- [ ] `AbstractValueMap.isEmpty(0)`: its class isn't translated (it's generic) (de/maxhenkel/sound_physics_remastered/configbuilder/custom/AbstractValueMap.java:22)
- [ ] `AbstractValueMap.containsKey(1)`: its class isn't translated (it's generic) (de/maxhenkel/sound_physics_remastered/configbuilder/custom/AbstractValueMap.java:27)
- [ ] `AbstractValueMap.containsValue(1)`: its class isn't translated (it's generic) (de/maxhenkel/sound_physics_remastered/configbuilder/custom/AbstractValueMap.java:32)
- [ ] `AbstractValueMap.get(1)`: its class isn't translated (it's generic) (de/maxhenkel/sound_physics_remastered/configbuilder/custom/AbstractValueMap.java:37)
- [ ] `AbstractValueMap.put(2)`: its class isn't translated (it's generic) (de/maxhenkel/sound_physics_remastered/configbuilder/custom/AbstractValueMap.java:42)
- [ ] `AbstractValueMap.remove(1)`: its class isn't translated (it's generic) (de/maxhenkel/sound_physics_remastered/configbuilder/custom/AbstractValueMap.java:47)
- [ ] `AbstractValueMap.putAll(1)`: its class isn't translated (it's generic) (de/maxhenkel/sound_physics_remastered/configbuilder/custom/AbstractValueMap.java:52)
- [ ] `AbstractValueMap.clear(0)`: its class isn't translated (it's generic) (de/maxhenkel/sound_physics_remastered/configbuilder/custom/AbstractValueMap.java:57)
- [ ] `AbstractValueMap.keySet(0)`: its class isn't translated (it's generic) (de/maxhenkel/sound_physics_remastered/configbuilder/custom/AbstractValueMap.java:62)
- [ ] `AbstractValueMap.values(0)`: its class isn't translated (it's generic) (de/maxhenkel/sound_physics_remastered/configbuilder/custom/AbstractValueMap.java:67)
- [ ] `AbstractValueMap.entrySet(0)`: its class isn't translated (it's generic) (de/maxhenkel/sound_physics_remastered/configbuilder/custom/AbstractValueMap.java:72)
- [ ] `AbstractValueMap.equals(1)`: its class isn't translated (it's generic) (de/maxhenkel/sound_physics_remastered/configbuilder/custom/AbstractValueMap.java:77)
- [ ] `AbstractValueMap.hashCode(0)`: its class isn't translated (it's generic) (de/maxhenkel/sound_physics_remastered/configbuilder/custom/AbstractValueMap.java:89)
- [ ] `AbstractValueMap.throwException(0)`: it's generic (de/maxhenkel/sound_physics_remastered/configbuilder/custom/AbstractValueMap.java:94)

### `AbstractValueMap.Builder`: it's generic

- [ ] `AbstractValueMap.Builder.Builder(0)`: its class isn't translated (it's generic) (de/maxhenkel/sound_physics_remastered/configbuilder/custom/AbstractValueMap.java:101)
- [ ] `AbstractValueMap.Builder.put(2)`: its class isn't translated (it's generic) (de/maxhenkel/sound_physics_remastered/configbuilder/custom/AbstractValueMap.java:104)
- [ ] `AbstractValueMap.Builder.putAll(1)`: its class isn't translated (it's generic) (de/maxhenkel/sound_physics_remastered/configbuilder/custom/AbstractValueMap.java:109)
- [ ] `AbstractValueMap.Builder.build(0)`: it has no body (de/maxhenkel/sound_physics_remastered/configbuilder/custom/AbstractValueMap.java:114)

### `IntegerList`: it extends `AbstractValueList`

- [ ] `IntegerList.IntegerList(1)`: its class isn't translated (it extends `AbstractValueList`) (de/maxhenkel/sound_physics_remastered/configbuilder/custom/IntegerList.java:6)
- [ ] `IntegerList.IntegerList(1)`: its class isn't translated (it extends `AbstractValueList`) (de/maxhenkel/sound_physics_remastered/configbuilder/custom/IntegerList.java:10)
- [ ] `IntegerList.of(1)`: it uses IntegerList, which isn't translated (it extends `AbstractValueList`) (de/maxhenkel/sound_physics_remastered/configbuilder/custom/IntegerList.java:14)
- [ ] `IntegerList.of(1)`: it uses IntegerList, which isn't translated (it extends `AbstractValueList`) (de/maxhenkel/sound_physics_remastered/configbuilder/custom/IntegerList.java:18)

### `IntegerListValueSerializer`: it implements `ValueSerializer` (de.maxhenkel.sound_physics_remastered.configbuilder.entry.serializer)

- [ ] `IntegerListValueSerializer.INSTANCE`: it uses IntegerListValueSerializer, which isn't translated (it implements `ValueSerializer` (de.maxhenkel.sound_physics_remastered.configbuilder.entry.serializer)) (de/maxhenkel/sound_physics_remastered/configbuilder/custom/serializer/IntegerListValueSerializer.java:10)
- [ ] `IntegerListValueSerializer.deserialize(1)`: its class isn't translated (it implements `ValueSerializer` (de.maxhenkel.sound_physics_remastered.configbuilder.entry.serializer)) (de/maxhenkel/sound_physics_remastered/configbuilder/custom/serializer/IntegerListValueSerializer.java:12)
- [ ] `IntegerListValueSerializer.serialize(1)`: its class isn't translated (it implements `ValueSerializer` (de.maxhenkel.sound_physics_remastered.configbuilder.entry.serializer)) (de/maxhenkel/sound_physics_remastered/configbuilder/custom/serializer/IntegerListValueSerializer.java:27)

### `StringListValueSerializer`: it implements `ValueSerializer` (de.maxhenkel.sound_physics_remastered.configbuilder.entry.serializer)

- [ ] `StringListValueSerializer.INSTANCE`: it uses StringListValueSerializer, which isn't translated (it implements `ValueSerializer` (de.maxhenkel.sound_physics_remastered.configbuilder.entry.serializer)) (de/maxhenkel/sound_physics_remastered/configbuilder/custom/serializer/StringListValueSerializer.java:9)
- [ ] `StringListValueSerializer.deserialize(1)`: its class isn't translated (it implements `ValueSerializer` (de.maxhenkel.sound_physics_remastered.configbuilder.entry.serializer)) (de/maxhenkel/sound_physics_remastered/configbuilder/custom/serializer/StringListValueSerializer.java:11)
- [ ] `StringListValueSerializer.serialize(1)`: its class isn't translated (it implements `ValueSerializer` (de.maxhenkel.sound_physics_remastered.configbuilder.entry.serializer)) (de/maxhenkel/sound_physics_remastered/configbuilder/custom/serializer/StringListValueSerializer.java:21)

### `StringMapValueSerializer`: it implements `ValueSerializer` (de.maxhenkel.sound_physics_remastered.configbuilder.entry.serializer)

- [ ] `StringMapValueSerializer.INSTANCE`: it uses StringMapValueSerializer, which isn't translated (it implements `ValueSerializer` (de.maxhenkel.sound_physics_remastered.configbuilder.entry.serializer)) (de/maxhenkel/sound_physics_remastered/configbuilder/custom/serializer/StringMapValueSerializer.java:15)
- [x] `StringMapValueSerializer.QUOTE_ESCAPE_PATTERN` (de/maxhenkel/sound_physics_remastered/configbuilder/custom/serializer/StringMapValueSerializer.java:16)
- [ ] `StringMapValueSerializer.deserialize(1)`: its class isn't translated (it implements `ValueSerializer` (de.maxhenkel.sound_physics_remastered.configbuilder.entry.serializer)) (de/maxhenkel/sound_physics_remastered/configbuilder/custom/serializer/StringMapValueSerializer.java:18)
- [ ] `StringMapValueSerializer.serialize(1)`: its class isn't translated (it implements `ValueSerializer` (de.maxhenkel.sound_physics_remastered.configbuilder.entry.serializer)) (de/maxhenkel/sound_physics_remastered/configbuilder/custom/serializer/StringMapValueSerializer.java:35)
- [x] `StringMapValueSerializer.escape(1)` (de/maxhenkel/sound_physics_remastered/configbuilder/custom/serializer/StringMapValueSerializer.java:45)
- [x] `StringMapValueSerializer.unescape(1)` (de/maxhenkel/sound_physics_remastered/configbuilder/custom/serializer/StringMapValueSerializer.java:49)

### `UUIDSerializer`: it implements `ValueSerializer` (de.maxhenkel.sound_physics_remastered.configbuilder.entry.serializer)

- [ ] `UUIDSerializer.INSTANCE`: it uses UUIDSerializer, which isn't translated (it implements `ValueSerializer` (de.maxhenkel.sound_physics_remastered.configbuilder.entry.serializer)) (de/maxhenkel/sound_physics_remastered/configbuilder/custom/serializer/UUIDSerializer.java:7)
- [ ] `UUIDSerializer.deserialize(1)`: its class isn't translated (it implements `ValueSerializer` (de.maxhenkel.sound_physics_remastered.configbuilder.entry.serializer)) (de/maxhenkel/sound_physics_remastered/configbuilder/custom/serializer/UUIDSerializer.java:9)
- [ ] `UUIDSerializer.serialize(1)`: its class isn't translated (it implements `ValueSerializer` (de.maxhenkel.sound_physics_remastered.configbuilder.entry.serializer)) (de/maxhenkel/sound_physics_remastered/configbuilder/custom/serializer/UUIDSerializer.java:13)

### `StringList`: it extends `AbstractValueList`

- [ ] `StringList.StringList(1)`: its class isn't translated (it extends `AbstractValueList`) (de/maxhenkel/sound_physics_remastered/configbuilder/custom/StringList.java:6)
- [ ] `StringList.StringList(1)`: its class isn't translated (it extends `AbstractValueList`) (de/maxhenkel/sound_physics_remastered/configbuilder/custom/StringList.java:10)
- [ ] `StringList.of(1)`: it uses StringList, which isn't translated (it extends `AbstractValueList`) (de/maxhenkel/sound_physics_remastered/configbuilder/custom/StringList.java:14)
- [ ] `StringList.of(1)`: it uses StringList, which isn't translated (it extends `AbstractValueList`) (de/maxhenkel/sound_physics_remastered/configbuilder/custom/StringList.java:18)

### `StringMap`: it extends `AbstractValueMap`

- [ ] `StringMap.StringMap(1)`: its class isn't translated (it extends `AbstractValueMap`) (de/maxhenkel/sound_physics_remastered/configbuilder/custom/StringMap.java:7)
- [ ] `StringMap.of(1)`: it uses StringMap, which isn't translated (it extends `AbstractValueMap`) (de/maxhenkel/sound_physics_remastered/configbuilder/custom/StringMap.java:11)
- [ ] `StringMap.of(0)`: it uses StringMap, which isn't translated (it extends `AbstractValueMap`) (de/maxhenkel/sound_physics_remastered/configbuilder/custom/StringMap.java:15)
- [ ] `StringMap.builder(0)`: it uses Builder, which isn't translated (it extends `Builder`) (de/maxhenkel/sound_physics_remastered/configbuilder/custom/StringMap.java:19)

### `StringMap.Builder`: it extends `Builder`

- [ ] `StringMap.Builder.build(0)`: its class isn't translated (it extends `Builder`) (de/maxhenkel/sound_physics_remastered/configbuilder/custom/StringMap.java:24)

### `AbstractConfigEntry`: it's generic

- [ ] `AbstractConfigEntry.AbstractConfigEntry(5)`: its class isn't translated (it's generic) (de/maxhenkel/sound_physics_remastered/configbuilder/entry/AbstractConfigEntry.java:18)
- [ ] `AbstractConfigEntry.reload(0)`: its class isn't translated (it's generic) (de/maxhenkel/sound_physics_remastered/configbuilder/entry/AbstractConfigEntry.java:31)
- [ ] `AbstractConfigEntry.get(0)`: its class isn't translated (it's generic) (de/maxhenkel/sound_physics_remastered/configbuilder/entry/AbstractConfigEntry.java:45)
- [ ] `AbstractConfigEntry.set(1)`: its class isn't translated (it's generic) (de/maxhenkel/sound_physics_remastered/configbuilder/entry/AbstractConfigEntry.java:50)
- [ ] `AbstractConfigEntry.getKey(0)`: its class isn't translated (it's generic) (de/maxhenkel/sound_physics_remastered/configbuilder/entry/AbstractConfigEntry.java:60)
- [ ] `AbstractConfigEntry.comment(1)`: its class isn't translated (it's generic) (de/maxhenkel/sound_physics_remastered/configbuilder/entry/AbstractConfigEntry.java:65)
- [ ] `AbstractConfigEntry.getComments(0)`: its class isn't translated (it's generic) (de/maxhenkel/sound_physics_remastered/configbuilder/entry/AbstractConfigEntry.java:71)
- [ ] `AbstractConfigEntry.reset(0)`: its class isn't translated (it's generic) (de/maxhenkel/sound_physics_remastered/configbuilder/entry/AbstractConfigEntry.java:76)
- [ ] `AbstractConfigEntry.syncEntryToProperties(0)`: its class isn't translated (it's generic) (de/maxhenkel/sound_physics_remastered/configbuilder/entry/AbstractConfigEntry.java:83)
- [ ] `AbstractConfigEntry.save(0)`: its class isn't translated (it's generic) (de/maxhenkel/sound_physics_remastered/configbuilder/entry/AbstractConfigEntry.java:96)
- [ ] `AbstractConfigEntry.saveSync(0)`: its class isn't translated (it's generic) (de/maxhenkel/sound_physics_remastered/configbuilder/entry/AbstractConfigEntry.java:102)
- [ ] `AbstractConfigEntry.fixValue(1)`: its class isn't translated (it's generic) (de/maxhenkel/sound_physics_remastered/configbuilder/entry/AbstractConfigEntry.java:108)
- [ ] `AbstractConfigEntry.getDefault(0)`: its class isn't translated (it's generic) (de/maxhenkel/sound_physics_remastered/configbuilder/entry/AbstractConfigEntry.java:112)
- [ ] `AbstractConfigEntry.getConfig(0)`: its class isn't translated (it's generic) (de/maxhenkel/sound_physics_remastered/configbuilder/entry/AbstractConfigEntry.java:117)
- [ ] `AbstractConfigEntry.getSerializer(0)`: its class isn't translated (it's generic) (de/maxhenkel/sound_physics_remastered/configbuilder/entry/AbstractConfigEntry.java:122)

### `AbstractRangedConfigEntry`: it's generic

- [ ] `AbstractRangedConfigEntry.AbstractRangedConfigEntry(7)`: its class isn't translated (it's generic) (de/maxhenkel/sound_physics_remastered/configbuilder/entry/AbstractRangedConfigEntry.java:14)
- [ ] `AbstractRangedConfigEntry.getMin(0)`: its class isn't translated (it's generic) (de/maxhenkel/sound_physics_remastered/configbuilder/entry/AbstractRangedConfigEntry.java:31)
- [ ] `AbstractRangedConfigEntry.getMax(0)`: its class isn't translated (it's generic) (de/maxhenkel/sound_physics_remastered/configbuilder/entry/AbstractRangedConfigEntry.java:37)
- [ ] `AbstractRangedConfigEntry.minimumPossibleValue(0)`: it has no body (de/maxhenkel/sound_physics_remastered/configbuilder/entry/AbstractRangedConfigEntry.java:43)
- [ ] `AbstractRangedConfigEntry.maximumPossibleValue(0)`: it has no body (de/maxhenkel/sound_physics_remastered/configbuilder/entry/AbstractRangedConfigEntry.java:46)

### `BooleanConfigEntry`: it extends `AbstractConfigEntry`

- [ ] `BooleanConfigEntry.BooleanConfigEntry(5)`: its class isn't translated (it extends `AbstractConfigEntry`) (de/maxhenkel/sound_physics_remastered/configbuilder/entry/BooleanConfigEntry.java:7)

### `ConfigEntry`: it's an interface

- [ ] `ConfigEntry.comment(1)`: it has no body (de/maxhenkel/sound_physics_remastered/configbuilder/entry/ConfigEntry.java:6)
- [ ] `ConfigEntry.getComments(0)`: it has no body (de/maxhenkel/sound_physics_remastered/configbuilder/entry/ConfigEntry.java:8)
- [ ] `ConfigEntry.get(0)`: it has no body (de/maxhenkel/sound_physics_remastered/configbuilder/entry/ConfigEntry.java:10)
- [ ] `ConfigEntry.set(1)`: it has no body (de/maxhenkel/sound_physics_remastered/configbuilder/entry/ConfigEntry.java:12)
- [ ] `ConfigEntry.getKey(0)`: it has no body (de/maxhenkel/sound_physics_remastered/configbuilder/entry/ConfigEntry.java:14)
- [ ] `ConfigEntry.reset(0)`: it has no body (de/maxhenkel/sound_physics_remastered/configbuilder/entry/ConfigEntry.java:16)
- [ ] `ConfigEntry.save(0)`: it has no body (de/maxhenkel/sound_physics_remastered/configbuilder/entry/ConfigEntry.java:18)
- [ ] `ConfigEntry.saveSync(0)`: it has no body (de/maxhenkel/sound_physics_remastered/configbuilder/entry/ConfigEntry.java:20)
- [ ] `ConfigEntry.getDefault(0)`: it has no body (de/maxhenkel/sound_physics_remastered/configbuilder/entry/ConfigEntry.java:22)
- [ ] `ConfigEntry.getConfig(0)`: it has no body (de/maxhenkel/sound_physics_remastered/configbuilder/entry/ConfigEntry.java:24)

### `DoubleConfigEntry`: it extends `AbstractRangedConfigEntry`

- [ ] `DoubleConfigEntry.DoubleConfigEntry(7)`: its class isn't translated (it extends `AbstractRangedConfigEntry`) (de/maxhenkel/sound_physics_remastered/configbuilder/entry/DoubleConfigEntry.java:9)
- [ ] `DoubleConfigEntry.minimumPossibleValue(0)`: its class isn't translated (it extends `AbstractRangedConfigEntry`) (de/maxhenkel/sound_physics_remastered/configbuilder/entry/DoubleConfigEntry.java:16)
- [ ] `DoubleConfigEntry.maximumPossibleValue(0)`: its class isn't translated (it extends `AbstractRangedConfigEntry`) (de/maxhenkel/sound_physics_remastered/configbuilder/entry/DoubleConfigEntry.java:21)
- [ ] `DoubleConfigEntry.fixValue(1)`: its class isn't translated (it extends `AbstractRangedConfigEntry`) (de/maxhenkel/sound_physics_remastered/configbuilder/entry/DoubleConfigEntry.java:26)

### `EnumConfigEntry`: it's generic

- [ ] `EnumConfigEntry.EnumConfigEntry(5)`: its class isn't translated (it's generic) (de/maxhenkel/sound_physics_remastered/configbuilder/entry/EnumConfigEntry.java:7)

### `FloatConfigEntry`: it extends `AbstractRangedConfigEntry`

- [ ] `FloatConfigEntry.FloatConfigEntry(7)`: its class isn't translated (it extends `AbstractRangedConfigEntry`) (de/maxhenkel/sound_physics_remastered/configbuilder/entry/FloatConfigEntry.java:9)
- [ ] `FloatConfigEntry.minimumPossibleValue(0)`: its class isn't translated (it extends `AbstractRangedConfigEntry`) (de/maxhenkel/sound_physics_remastered/configbuilder/entry/FloatConfigEntry.java:16)
- [ ] `FloatConfigEntry.maximumPossibleValue(0)`: its class isn't translated (it extends `AbstractRangedConfigEntry`) (de/maxhenkel/sound_physics_remastered/configbuilder/entry/FloatConfigEntry.java:21)
- [ ] `FloatConfigEntry.fixValue(1)`: its class isn't translated (it extends `AbstractRangedConfigEntry`) (de/maxhenkel/sound_physics_remastered/configbuilder/entry/FloatConfigEntry.java:26)

### `GenericConfigEntry`: it's generic

- [ ] `GenericConfigEntry.GenericConfigEntry(5)`: its class isn't translated (it's generic) (de/maxhenkel/sound_physics_remastered/configbuilder/entry/GenericConfigEntry.java:7)

### `IntegerConfigEntry`: it extends `AbstractRangedConfigEntry`

- [ ] `IntegerConfigEntry.IntegerConfigEntry(7)`: its class isn't translated (it extends `AbstractRangedConfigEntry`) (de/maxhenkel/sound_physics_remastered/configbuilder/entry/IntegerConfigEntry.java:9)
- [ ] `IntegerConfigEntry.minimumPossibleValue(0)`: its class isn't translated (it extends `AbstractRangedConfigEntry`) (de/maxhenkel/sound_physics_remastered/configbuilder/entry/IntegerConfigEntry.java:22)
- [ ] `IntegerConfigEntry.maximumPossibleValue(0)`: its class isn't translated (it extends `AbstractRangedConfigEntry`) (de/maxhenkel/sound_physics_remastered/configbuilder/entry/IntegerConfigEntry.java:27)
- [ ] `IntegerConfigEntry.fixValue(1)`: its class isn't translated (it extends `AbstractRangedConfigEntry`) (de/maxhenkel/sound_physics_remastered/configbuilder/entry/IntegerConfigEntry.java:32)

### `LongConfigEntry`: it extends `AbstractRangedConfigEntry`

- [ ] `LongConfigEntry.LongConfigEntry(7)`: its class isn't translated (it extends `AbstractRangedConfigEntry`) (de/maxhenkel/sound_physics_remastered/configbuilder/entry/LongConfigEntry.java:9)
- [ ] `LongConfigEntry.minimumPossibleValue(0)`: its class isn't translated (it extends `AbstractRangedConfigEntry`) (de/maxhenkel/sound_physics_remastered/configbuilder/entry/LongConfigEntry.java:16)
- [ ] `LongConfigEntry.maximumPossibleValue(0)`: its class isn't translated (it extends `AbstractRangedConfigEntry`) (de/maxhenkel/sound_physics_remastered/configbuilder/entry/LongConfigEntry.java:21)
- [ ] `LongConfigEntry.fixValue(1)`: its class isn't translated (it extends `AbstractRangedConfigEntry`) (de/maxhenkel/sound_physics_remastered/configbuilder/entry/LongConfigEntry.java:26)

### `RangedConfigEntry`: it's an interface

- [ ] `RangedConfigEntry.getMin(0)`: it has no body (de/maxhenkel/sound_physics_remastered/configbuilder/entry/RangedConfigEntry.java:6)
- [ ] `RangedConfigEntry.getMax(0)`: it has no body (de/maxhenkel/sound_physics_remastered/configbuilder/entry/RangedConfigEntry.java:9)

### `BooleanSerializer`: it implements `ValueSerializer`

- [ ] `BooleanSerializer.INSTANCE`: it uses BooleanSerializer, which isn't translated (it implements `ValueSerializer`) (de/maxhenkel/sound_physics_remastered/configbuilder/entry/serializer/BooleanSerializer.java:6)
- [ ] `BooleanSerializer.deserialize(1)`: its class isn't translated (it implements `ValueSerializer`) (de/maxhenkel/sound_physics_remastered/configbuilder/entry/serializer/BooleanSerializer.java:8)
- [ ] `BooleanSerializer.serialize(1)`: its class isn't translated (it implements `ValueSerializer`) (de/maxhenkel/sound_physics_remastered/configbuilder/entry/serializer/BooleanSerializer.java:13)

### `DoubleSerializer`: it implements `ValueSerializer`

- [ ] `DoubleSerializer.INSTANCE`: it uses DoubleSerializer, which isn't translated (it implements `ValueSerializer`) (de/maxhenkel/sound_physics_remastered/configbuilder/entry/serializer/DoubleSerializer.java:6)
- [ ] `DoubleSerializer.deserialize(1)`: its class isn't translated (it implements `ValueSerializer`) (de/maxhenkel/sound_physics_remastered/configbuilder/entry/serializer/DoubleSerializer.java:8)
- [ ] `DoubleSerializer.serialize(1)`: its class isn't translated (it implements `ValueSerializer`) (de/maxhenkel/sound_physics_remastered/configbuilder/entry/serializer/DoubleSerializer.java:17)

### `EnumSerializer`: it's generic

- [ ] `EnumSerializer.EnumSerializer(1)`: its class isn't translated (it's generic) (de/maxhenkel/sound_physics_remastered/configbuilder/entry/serializer/EnumSerializer.java:8)
- [ ] `EnumSerializer.deserialize(1)`: its class isn't translated (it's generic) (de/maxhenkel/sound_physics_remastered/configbuilder/entry/serializer/EnumSerializer.java:12)
- [ ] `EnumSerializer.serialize(1)`: its class isn't translated (it's generic) (de/maxhenkel/sound_physics_remastered/configbuilder/entry/serializer/EnumSerializer.java:21)

### `FloatSerializer`: it implements `ValueSerializer`

- [ ] `FloatSerializer.INSTANCE`: it uses FloatSerializer, which isn't translated (it implements `ValueSerializer`) (de/maxhenkel/sound_physics_remastered/configbuilder/entry/serializer/FloatSerializer.java:6)
- [ ] `FloatSerializer.deserialize(1)`: its class isn't translated (it implements `ValueSerializer`) (de/maxhenkel/sound_physics_remastered/configbuilder/entry/serializer/FloatSerializer.java:8)
- [ ] `FloatSerializer.serialize(1)`: its class isn't translated (it implements `ValueSerializer`) (de/maxhenkel/sound_physics_remastered/configbuilder/entry/serializer/FloatSerializer.java:17)

### `IntegerSerializer`: it implements `ValueSerializer`

- [ ] `IntegerSerializer.INSTANCE`: it uses IntegerSerializer, which isn't translated (it implements `ValueSerializer`) (de/maxhenkel/sound_physics_remastered/configbuilder/entry/serializer/IntegerSerializer.java:6)
- [ ] `IntegerSerializer.deserialize(1)`: its class isn't translated (it implements `ValueSerializer`) (de/maxhenkel/sound_physics_remastered/configbuilder/entry/serializer/IntegerSerializer.java:8)
- [ ] `IntegerSerializer.serialize(1)`: its class isn't translated (it implements `ValueSerializer`) (de/maxhenkel/sound_physics_remastered/configbuilder/entry/serializer/IntegerSerializer.java:17)

### `LongSerializer`: it implements `ValueSerializer`

- [ ] `LongSerializer.INSTANCE`: it uses LongSerializer, which isn't translated (it implements `ValueSerializer`) (de/maxhenkel/sound_physics_remastered/configbuilder/entry/serializer/LongSerializer.java:6)
- [ ] `LongSerializer.deserialize(1)`: its class isn't translated (it implements `ValueSerializer`) (de/maxhenkel/sound_physics_remastered/configbuilder/entry/serializer/LongSerializer.java:8)
- [ ] `LongSerializer.serialize(1)`: its class isn't translated (it implements `ValueSerializer`) (de/maxhenkel/sound_physics_remastered/configbuilder/entry/serializer/LongSerializer.java:17)

### `StringSerializer`: it implements `ValueSerializer`

- [ ] `StringSerializer.INSTANCE`: it uses StringSerializer, which isn't translated (it implements `ValueSerializer`) (de/maxhenkel/sound_physics_remastered/configbuilder/entry/serializer/StringSerializer.java:6)
- [ ] `StringSerializer.deserialize(1)`: its class isn't translated (it implements `ValueSerializer`) (de/maxhenkel/sound_physics_remastered/configbuilder/entry/serializer/StringSerializer.java:8)
- [ ] `StringSerializer.serialize(1)`: its class isn't translated (it implements `ValueSerializer`) (de/maxhenkel/sound_physics_remastered/configbuilder/entry/serializer/StringSerializer.java:13)

### `ValueSerializable`: it's an annotation

- [ ] `ValueSerializable.value(0)`: it has no body (de/maxhenkel/sound_physics_remastered/configbuilder/entry/serializer/ValueSerializable.java:11)

### `ValueSerializer`: it's an interface

- [ ] `ValueSerializer.deserialize(1)`: it has no body (de/maxhenkel/sound_physics_remastered/configbuilder/entry/serializer/ValueSerializer.java:6)
- [ ] `ValueSerializer.serialize(1)`: it has no body (de/maxhenkel/sound_physics_remastered/configbuilder/entry/serializer/ValueSerializer.java:9)

### `StringConfigEntry`: it extends `AbstractConfigEntry`

- [ ] `StringConfigEntry.StringConfigEntry(5)`: its class isn't translated (it extends `AbstractConfigEntry`) (de/maxhenkel/sound_physics_remastered/configbuilder/entry/StringConfigEntry.java:7)
