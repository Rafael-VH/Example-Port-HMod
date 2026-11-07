# Java translated to Rust

Translated 20 of 259 methods and 15 of 36 static fields to Rust. It's in `runtime.rs`, under `pub mod java`; code that calls the game is in `game_side.rs`.

Each method is translated whole or not at all. A method that can't be (it uses Minecraft, a lambda, null...) is left out with its reason, and so is every method that calls it.

## Why methods were left out

| Cause | Methods | For example |
|---|---|---|
| Calls or uses other code that isn't translated | 206 | `AmbientSounds.onInitializeClient(0)`: its class isn't translated (it implements `ClientLoader` (team.creative.creativecore.client)) |
| Other Java this translator doesn't handle yet | 20 | `AmbientSounds.AmbientSounds(0)`: its body couldn't be read: an anonymous class |
| Uses Minecraft classes (needs Horizon's game API) | 5 | `AmbientBlockGroup.is(1)`: it uses Minecraft's `BlockState` |
| Shares one object between names (Java references) | 2 | `BiomeCondition.BiomeCondition(2)`: it stores a second reference to an object (Rust would need it copied or shared) |
| Uses null | 2 | `AmbientSounds.scheduleReload(0)`: a nullable object kept in a changing static |
| Uses other libraries | 2 | `AmbientBlock.parse(1)`: it uses `TupleList` (team.creative.creativecore.common.util.type.list) |
| Uses lambdas or method references | 1 | `AmbientSounds.reloadAsync(0)`: a method reference |
| Uses try/catch | 1 | `AmbientBlockGroup.onClientLoad(0)`: a try/catch (AmbientEngineLoadException) |

## Every class


### `AmbientSounds`: it implements `ClientLoader` (team.creative.creativecore.client)

- [x] `AmbientSounds.LOGGER` (team/creative/ambientsounds/AmbientSounds.java:24)
- [x] `AmbientSounds.MODID` (team/creative/ambientsounds/AmbientSounds.java:25)
- [ ] `AmbientSounds.CONFIG`: it uses AmbientSoundsConfig, which isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)) (team/creative/ambientsounds/AmbientSounds.java:26)
- [ ] `AmbientSounds.TICK_HANDLER`: it uses AmbientTickHandler, which isn't translated (it uses AmbientEngine, which isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition))))) (team/creative/ambientsounds/AmbientSounds.java:27)
- [ ] `AmbientSounds.AmbientSounds(0)`: its body couldn't be read: an anonymous class (team/creative/ambientsounds/AmbientSounds.java:29)
- [ ] `AmbientSounds.scheduleReload(0)`: a nullable object kept in a changing static (team/creative/ambientsounds/AmbientSounds.java:51)
- [ ] `AmbientSounds.reloadAsync(0)`: a method reference (team/creative/ambientsounds/AmbientSounds.java:55)
- [ ] `AmbientSounds.reload(0)`: a nullable object kept in a changing static (team/creative/ambientsounds/AmbientSounds.java:59)
- [ ] `AmbientSounds.onInitializeClient(0)`: its class isn't translated (it implements `ClientLoader` (team.creative.creativecore.client)) (team/creative/ambientsounds/AmbientSounds.java:71)
- [ ] `AmbientSounds.registerClientCommands(1)`: it's generic (team/creative/ambientsounds/AmbientSounds.java:80)

### `AmbientSoundsConfig`: it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)

- [ ] `AmbientSoundsConfig.configured(1)`: its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)) (team/creative/ambientsounds/AmbientSoundsConfig.java:23)

### `AmbientBlock`

- [ ] `AmbientBlock.parse(1)`: it uses `TupleList` (team.creative.creativecore.common.util.type.list) (team/creative/ambientsounds/block/AmbientBlock.java:16)
- [ ] `AmbientBlock.parseFirst(1)`: it creates a AmbientBlockTag, which isn't translated (it extends `AmbientBlock`) (team/creative/ambientsounds/block/AmbientBlock.java:46)
- [ ] `AmbientBlock.is(1)`: it has no body (team/creative/ambientsounds/block/AmbientBlock.java:52)

### `AmbientBlock.AmbientBlockBlock`: it extends `AmbientBlock`

- [ ] `AmbientBlock.AmbientBlockBlock.AmbientBlockBlock(1)`: its class isn't translated (it extends `AmbientBlock`) (team/creative/ambientsounds/block/AmbientBlock.java:57)
- [ ] `AmbientBlock.AmbientBlockBlock.is(1)`: its class isn't translated (it extends `AmbientBlock`) (team/creative/ambientsounds/block/AmbientBlock.java:61)

### `AmbientBlock.AmbientBlockProperty`: it extends `AmbientBlock`

- [ ] `AmbientBlock.AmbientBlockProperty.AmbientBlockProperty(2)`: its class isn't translated (it extends `AmbientBlock`) (team/creative/ambientsounds/block/AmbientBlock.java:71)
- [ ] `AmbientBlock.AmbientBlockProperty.is(1)`: its class isn't translated (it extends `AmbientBlock`) (team/creative/ambientsounds/block/AmbientBlock.java:76)

### `AmbientBlock.AmbientBlockTag`: it extends `AmbientBlock`

- [ ] `AmbientBlock.AmbientBlockTag.AmbientBlockTag(1)`: its class isn't translated (it extends `AmbientBlock`) (team/creative/ambientsounds/block/AmbientBlock.java:101)
- [ ] `AmbientBlock.AmbientBlockTag.is(1)`: its class isn't translated (it extends `AmbientBlock`) (team/creative/ambientsounds/block/AmbientBlock.java:105)

### `AmbientBlockGroup`

- [ ] `AmbientBlockGroup.onClientLoad(0)`: a try/catch (AmbientEngineLoadException) (team/creative/ambientsounds/block/AmbientBlockGroup.java:14)
- [x] `AmbientBlockGroup.add(1)` (team/creative/ambientsounds/block/AmbientBlockGroup.java:26)
- [x] `AmbientBlockGroup.isEmpty(0)` (team/creative/ambientsounds/block/AmbientBlockGroup.java:30)
- [ ] `AmbientBlockGroup.is(1)`: it uses Minecraft's `BlockState` (team/creative/ambientsounds/block/AmbientBlockGroup.java:34)

### `AmbientCondition`: it extends `AmbientSoundProperties` (team.creative.ambientsounds.sound)

- [ ] `AmbientCondition.regionName(0)`: its class isn't translated (it extends `AmbientSoundProperties` (team.creative.ambientsounds.sound)) (team/creative/ambientsounds/condition/AmbientCondition.java:65)
- [ ] `AmbientCondition.init(1)`: its class isn't translated (it extends `AmbientSoundProperties` (team.creative.ambientsounds.sound)) (team/creative/ambientsounds/condition/AmbientCondition.java:69)
- [ ] `AmbientCondition.value(1)`: its class isn't translated (it extends `AmbientSoundProperties` (team.creative.ambientsounds.sound)) (team/creative/ambientsounds/condition/AmbientCondition.java:114)

### `AmbientCondition.AmbientMinMaxCondition`

- [x] `AmbientCondition.AmbientMinMaxCondition.is(1)` (team/creative/ambientsounds/condition/AmbientCondition.java:368)
- [ ] `AmbientCondition.AmbientMinMaxCondition.randomValue(0)`: Math.random (no random numbers without a crate) (team/creative/ambientsounds/condition/AmbientCondition.java:372)

### `AmbientCondition.AmbientMinMaxFadeCondition`: it extends `AmbientMinMaxCondition`

- [ ] `AmbientCondition.AmbientMinMaxFadeCondition.volume(1)`: its class isn't translated (it extends `AmbientMinMaxCondition`) (team/creative/ambientsounds/condition/AmbientCondition.java:389)

### `AmbientCondition.AmbientMinMaxFadeSpecialCondition`: it extends `AmbientMinMaxFadeCondition`

- [ ] `AmbientCondition.AmbientMinMaxFadeSpecialCondition.volume(3)`: its class isn't translated (it extends `AmbientMinMaxFadeCondition`) (team/creative/ambientsounds/condition/AmbientCondition.java:412)

### `AmbientSelection`: it extends `AmbientVolume`

- [ ] `AmbientSelection.AmbientSelection(1)`: its class isn't translated (it extends `AmbientVolume`) (team/creative/ambientsounds/condition/AmbientSelection.java:10)
- [ ] `AmbientSelection.assign(1)`: its class isn't translated (it extends `AmbientVolume`) (team/creative/ambientsounds/condition/AmbientSelection.java:15)
- [ ] `AmbientSelection.conditionVolume(0)`: its class isn't translated (it extends `AmbientVolume`) (team/creative/ambientsounds/condition/AmbientSelection.java:24)
- [ ] `AmbientSelection.settingVolume(0)`: its class isn't translated (it extends `AmbientVolume`) (team/creative/ambientsounds/condition/AmbientSelection.java:29)
- [ ] `AmbientSelection.volume(0)`: its class isn't translated (it extends `AmbientVolume`) (team/creative/ambientsounds/condition/AmbientSelection.java:34)
- [ ] `AmbientSelection.last(0)`: its class isn't translated (it extends `AmbientVolume`) (team/creative/ambientsounds/condition/AmbientSelection.java:39)
- [ ] `AmbientSelection.getProperties(0)`: its class isn't translated (it extends `AmbientVolume`) (team/creative/ambientsounds/condition/AmbientSelection.java:43)
- [ ] `AmbientSelection.assignProperties(1)`: its class isn't translated (it extends `AmbientVolume`) (team/creative/ambientsounds/condition/AmbientSelection.java:49)

### `AmbientSelectionMulti`: it extends `AmbientSelection`

- [ ] `AmbientSelectionMulti.AmbientSelectionMulti(2)`: its class isn't translated (it extends `AmbientSelection`) (team/creative/ambientsounds/condition/AmbientSelectionMulti.java:9)
- [ ] `AmbientSelectionMulti.conditionVolume(0)`: its class isn't translated (it extends `AmbientSelection`) (team/creative/ambientsounds/condition/AmbientSelectionMulti.java:15)
- [ ] `AmbientSelectionMulti.settingVolume(0)`: its class isn't translated (it extends `AmbientSelection`) (team/creative/ambientsounds/condition/AmbientSelectionMulti.java:26)
- [ ] `AmbientSelectionMulti.volume(0)`: its class isn't translated (it extends `AmbientSelection`) (team/creative/ambientsounds/condition/AmbientSelectionMulti.java:37)
- [ ] `AmbientSelectionMulti.assignProperties(1)`: its class isn't translated (it extends `AmbientSelection`) (team/creative/ambientsounds/condition/AmbientSelectionMulti.java:48)

### `AmbientTime`

- [ ] `AmbientTime.PREMADE`: it uses AmbientTimePremade, which isn't translated (it extends `AmbientTimeHour`) (team/creative/ambientsounds/condition/AmbientTime.java:26)
- [x] `AmbientTime.ANGLE_TO_TIME` (team/creative/ambientsounds/condition/AmbientTime.java:27)
- [x] `AmbientTime.TIME_TO_ANGLE` (team/creative/ambientsounds/condition/AmbientTime.java:28)
- [x] `AmbientTime.FADE` (team/creative/ambientsounds/condition/AmbientTime.java:29)
- [ ] `AmbientTime.NONE`: its value couldn't be read: an anonymous class (team/creative/ambientsounds/condition/AmbientTime.java:30)
- [ ] `AmbientTime.DAY`: it uses AmbientTimePremade, which isn't translated (it extends `AmbientTimeHour`) (team/creative/ambientsounds/condition/AmbientTime.java:41)
- [ ] `AmbientTime.NIGHT`: it uses AmbientTimePremade, which isn't translated (it extends `AmbientTimeHour`) (team/creative/ambientsounds/condition/AmbientTime.java:42)
- [ ] `AmbientTime.SUNRISE`: it uses AmbientTimePremade, which isn't translated (it extends `AmbientTimeHour`) (team/creative/ambientsounds/condition/AmbientTime.java:45)
- [ ] `AmbientTime.SUNSET`: it uses AmbientTimePremade, which isn't translated (it extends `AmbientTimeHour`) (team/creative/ambientsounds/condition/AmbientTime.java:46)
- [ ] `AmbientTime.NOON`: it uses AmbientTimePremade, which isn't translated (it extends `AmbientTimeHour`) (team/creative/ambientsounds/condition/AmbientTime.java:47)
- [ ] `AmbientTime.MIDNIGHT`: it uses AmbientTimePremade, which isn't translated (it extends `AmbientTimeHour`) (team/creative/ambientsounds/condition/AmbientTime.java:48)
- [ ] `AmbientTime.value(1)`: it has no body (team/creative/ambientsounds/condition/AmbientTime.java:52)
- [ ] `AmbientTime.toJson(0)`: it has no body (team/creative/ambientsounds/condition/AmbientTime.java:54)

### `AmbientTime.AmbientTimeHour`: it extends `AmbientTime`

- [ ] `AmbientTime.AmbientTimeHour.AmbientTimeHour(1)`: its class isn't translated (it extends `AmbientTime`) (team/creative/ambientsounds/condition/AmbientTime.java:60)
- [ ] `AmbientTime.AmbientTimeHour.value(1)`: its class isn't translated (it extends `AmbientTime`) (team/creative/ambientsounds/condition/AmbientTime.java:82)
- [ ] `AmbientTime.AmbientTimeHour.toJson(0)`: its class isn't translated (it extends `AmbientTime`) (team/creative/ambientsounds/condition/AmbientTime.java:87)

### `AmbientTime.AmbientTimeMulti`: it extends `AmbientTime`

- [ ] `AmbientTime.AmbientTimeMulti.AmbientTimeMulti(1)`: its class isn't translated (it extends `AmbientTime`) (team/creative/ambientsounds/condition/AmbientTime.java:102)
- [ ] `AmbientTime.AmbientTimeMulti.value(1)`: its class isn't translated (it extends `AmbientTime`) (team/creative/ambientsounds/condition/AmbientTime.java:106)
- [ ] `AmbientTime.AmbientTimeMulti.toJson(0)`: its class isn't translated (it extends `AmbientTime`) (team/creative/ambientsounds/condition/AmbientTime.java:117)

### `AmbientTime.AmbientTimePremade`: it extends `AmbientTimeHour`

- [ ] `AmbientTime.AmbientTimePremade.AmbientTimePremade(4)`: its class isn't translated (it extends `AmbientTimeHour`) (team/creative/ambientsounds/condition/AmbientTime.java:132)
- [ ] `AmbientTime.AmbientTimePremade.AmbientTimePremade(2)`: its class isn't translated (it extends `AmbientTimeHour`) (team/creative/ambientsounds/condition/AmbientTime.java:138)
- [ ] `AmbientTime.AmbientTimePremade.toJson(0)`: its class isn't translated (it extends `AmbientTimeHour`) (team/creative/ambientsounds/condition/AmbientTime.java:144)

### `AmbientTime.AmbientTimeSunAngle`: it extends `AmbientTime`

- [ ] `AmbientTime.AmbientTimeSunAngle.AmbientTimeSunAngle(1)`: its class isn't translated (it extends `AmbientTime`) (team/creative/ambientsounds/condition/AmbientTime.java:154)
- [ ] `AmbientTime.AmbientTimeSunAngle.value(1)`: its class isn't translated (it extends `AmbientTime`) (team/creative/ambientsounds/condition/AmbientTime.java:171)
- [ ] `AmbientTime.AmbientTimeSunAngle.toJson(0)`: its class isn't translated (it extends `AmbientTime`) (team/creative/ambientsounds/condition/AmbientTime.java:176)

### `AmbientTime.Serializer`: it implements `JsonDeserializer` (com.google.gson)

- [ ] `AmbientTime.Serializer.serialize(3)`: its class isn't translated (it implements `JsonDeserializer` (com.google.gson)) (team/creative/ambientsounds/condition/AmbientTime.java:190)
- [ ] `AmbientTime.Serializer.deserialize(3)`: its class isn't translated (it implements `JsonDeserializer` (com.google.gson)) (team/creative/ambientsounds/condition/AmbientTime.java:194)

### `AmbientVolume`

- [ ] `AmbientVolume.SILENT`: its value couldn't be read: an anonymous class (team/creative/ambientsounds/condition/AmbientVolume.java:6)
- [ ] `AmbientVolume.MAX`: its value couldn't be read: an anonymous class (team/creative/ambientsounds/condition/AmbientVolume.java:15)
- [x] `AmbientVolume.AmbientVolume(2)` (team/creative/ambientsounds/condition/AmbientVolume.java:27)
- [x] `AmbientVolume.AmbientVolume(0)` (team/creative/ambientsounds/condition/AmbientVolume.java:32)
- [x] `AmbientVolume.assign(1)` (team/creative/ambientsounds/condition/AmbientVolume.java:36)
- [x] `AmbientVolume.setConditionVolumeDirect(1)` (team/creative/ambientsounds/condition/AmbientVolume.java:41)
- [x] `AmbientVolume.mulVolume(1)` (team/creative/ambientsounds/condition/AmbientVolume.java:46)
- [x] `AmbientVolume.mulCondition(1)` (team/creative/ambientsounds/condition/AmbientVolume.java:51)
- [x] `AmbientVolume.mulSetting(1)` (team/creative/ambientsounds/condition/AmbientVolume.java:55)
- [x] `AmbientVolume.conditionVolume(0)` (team/creative/ambientsounds/condition/AmbientVolume.java:59)
- [x] `AmbientVolume.settingVolume(0)` (team/creative/ambientsounds/condition/AmbientVolume.java:63)
- [x] `AmbientVolume.volume(0)` (team/creative/ambientsounds/condition/AmbientVolume.java:67)
- [ ] `AmbientVolume.toString(0)`: it uses `DebugTextRenderer` (team.creative.creativecore.client.render.text).DECIMAL_FORMAT (team/creative/ambientsounds/condition/AmbientVolume.java:71)
- [x] `AmbientVolume.copy(0)` (team/creative/ambientsounds/condition/AmbientVolume.java:76)

### `BiomeCondition`

- [ ] `BiomeCondition.BiomeCondition(2)`: it stores a second reference to an object (Rust would need it copied or shared) (team/creative/ambientsounds/condition/BiomeCondition.java:6)
- [ ] `BiomeCondition.of(1)`: it's overloaded with the same number of parameters (team/creative/ambientsounds/condition/BiomeCondition.java:7)
- [ ] `BiomeCondition.of(1)`: it's overloaded with the same number of parameters (team/creative/ambientsounds/condition/BiomeCondition.java:13)
- [ ] `BiomeCondition.matches(1)`: it calls BiomeMatchCache.matches, which isn't translated (team/creative/ambientsounds/condition/BiomeCondition.java:27)
- [x] `BiomeCondition.tag(0)` (team/creative/ambientsounds/condition/BiomeCondition.java:6)
- [ ] `BiomeCondition.pattern(0)`: it stores a second reference to an object (Rust would need it copied or shared) (team/creative/ambientsounds/condition/BiomeCondition.java:6)

### `BiomeMatchCache`

- [x] `BiomeMatchCache.MAX_LOCATIONS_PER_PATTERN` (team/creative/ambientsounds/condition/BiomeMatchCache.java:10)
- [ ] `BiomeMatchCache.CACHE`: it uses `ThreadLocal` (team/creative/ambientsounds/condition/BiomeMatchCache.java:11)
- [ ] `BiomeMatchCache.matches(2)`: a Map or Set keyed by a Pattern (team/creative/ambientsounds/condition/BiomeMatchCache.java:13)

### `AmbientDimension`: it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition))

- [ ] `AmbientDimension.load(4)`: its class isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition))) (team/creative/ambientsounds/dimension/AmbientDimension.java:45)
- [ ] `AmbientDimension.init(1)`: its class isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition))) (team/creative/ambientsounds/dimension/AmbientDimension.java:67)
- [ ] `AmbientDimension.is(1)`: its class isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition))) (team/creative/ambientsounds/dimension/AmbientDimension.java:95)
- [ ] `AmbientDimension.manipulateEnviroment(1)`: its class isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition))) (team/creative/ambientsounds/dimension/AmbientDimension.java:116)
- [ ] `AmbientDimension.toString(0)`: its class isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition))) (team/creative/ambientsounds/dimension/AmbientDimension.java:151)

### `AmbientEngine`: it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)))

- [x] `AmbientEngine.CONFIG_LOCATION` (team/creative/ambientsounds/engine/AmbientEngine.java:54)
- [x] `AmbientEngine.ENGINE_LOCATION` (team/creative/ambientsounds/engine/AmbientEngine.java:55)
- [x] `AmbientEngine.DIMENSIONS_LOCATION` (team/creative/ambientsounds/engine/AmbientEngine.java:56)
- [x] `AmbientEngine.REGIONS_LOCATION` (team/creative/ambientsounds/engine/AmbientEngine.java:57)
- [x] `AmbientEngine.SOUNDCOLLECTIONS_LOCATION` (team/creative/ambientsounds/engine/AmbientEngine.java:58)
- [x] `AmbientEngine.SOUNDCATEGORIES_LOCATION` (team/creative/ambientsounds/engine/AmbientEngine.java:59)
- [x] `AmbientEngine.BLOCKGROUPS_LOCATION` (team/creative/ambientsounds/engine/AmbientEngine.java:60)
- [x] `AmbientEngine.FEATURES_LOCATION` (team/creative/ambientsounds/engine/AmbientEngine.java:61)
- [ ] `AmbientEngine.GSON`: Gson.registerTypeAdapter (team/creative/ambientsounds/engine/AmbientEngine.java:62)
- [x] `AmbientEngine.loadedEngine` (team/creative/ambientsounds/engine/AmbientEngine.java:63)
- [x] `AmbientEngine.hasLoadedAtLeastOnce(0)` (team/creative/ambientsounds/engine/AmbientEngine.java:129)
- [x] `AmbientEngine.hasEngineChanged(1)` (team/creative/ambientsounds/engine/AmbientEngine.java:133)
- [ ] `AmbientEngine.attemptToLoadEngine(3)`: it uses AmbientEngine, which isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)))) (team/creative/ambientsounds/engine/AmbientEngine.java:137)
- [ ] `AmbientEngine.applyStackType(3)`: it's generic (team/creative/ambientsounds/engine/AmbientEngine.java:247)
- [ ] `AmbientEngine.loadMultiple(5)`: it's generic (team/creative/ambientsounds/engine/AmbientEngine.java:258)
- [ ] `AmbientEngine.loadAmbientEngine(1)`: it uses AmbientEngine, which isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)))) (team/creative/ambientsounds/engine/AmbientEngine.java:299)
- [ ] `AmbientEngine.getRegion(1)`: its class isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)))) (team/creative/ambientsounds/engine/AmbientEngine.java:327)
- [ ] `AmbientEngine.checkRegion(3)`: its class isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)))) (team/creative/ambientsounds/engine/AmbientEngine.java:331)
- [ ] `AmbientEngine.addRegion(1)`: its class isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)))) (team/creative/ambientsounds/engine/AmbientEngine.java:345)
- [ ] `AmbientEngine.getDimension(1)`: its class isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)))) (team/creative/ambientsounds/engine/AmbientEngine.java:358)
- [ ] `AmbientEngine.stopEngine(0)`: its class isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)))) (team/creative/ambientsounds/engine/AmbientEngine.java:376)
- [ ] `AmbientEngine.consumeSoundCollections(2)`: its class isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)))) (team/creative/ambientsounds/engine/AmbientEngine.java:386)
- [ ] `AmbientEngine.airPocketVolume(1)`: its class isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)))) (team/creative/ambientsounds/engine/AmbientEngine.java:397)
- [ ] `AmbientEngine.init(0)`: its class isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)))) (team/creative/ambientsounds/engine/AmbientEngine.java:420)
- [ ] `AmbientEngine.onClientLoad(0)`: its class isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)))) (team/creative/ambientsounds/engine/AmbientEngine.java:489)
- [ ] `AmbientEngine.airWeightFactor(1)`: its class isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)))) (team/creative/ambientsounds/engine/AmbientEngine.java:494)
- [ ] `AmbientEngine.tick(1)`: its class isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)))) (team/creative/ambientsounds/engine/AmbientEngine.java:498)
- [ ] `AmbientEngine.fastTick(1)`: its class isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)))) (team/creative/ambientsounds/engine/AmbientEngine.java:530)
- [ ] `AmbientEngine.changeDimension(2)`: its class isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)))) (team/creative/ambientsounds/engine/AmbientEngine.java:547)
- [ ] `AmbientEngine.collectDetails(1)`: its class isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)))) (team/creative/ambientsounds/engine/AmbientEngine.java:558)
- [ ] `AmbientEngine.getSoundCategory(1)`: its class isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)))) (team/creative/ambientsounds/engine/AmbientEngine.java:562)

### `AmbientEngine.Serializer`: it implements `JsonDeserializer` (com.google.gson)

- [ ] `AmbientEngine.Serializer.deserialize(3)`: its class isn't translated (it implements `JsonDeserializer` (com.google.gson)) (team/creative/ambientsounds/engine/AmbientEngine.java:567)
- [ ] `AmbientEngine.Serializer.serialize(3)`: its class isn't translated (it implements `JsonDeserializer` (com.google.gson)) (team/creative/ambientsounds/engine/AmbientEngine.java:571)

### `AmbientEngineConfig`


### `AmbientEngineLoadException`: it extends `Exception`

- [ ] `AmbientEngineLoadException.AmbientEngineLoadException(1)`: its class isn't translated (it extends `Exception`) (team/creative/ambientsounds/engine/AmbientEngineLoadException.java:4)

### `AmbientLoader`: it's an interface

- [ ] `AmbientLoader.setNameAndLoad(3)`: it has no body (team/creative/ambientsounds/engine/AmbientLoader.java:7)

### `AmbientStackType`: its constants carry values

- [ ] `AmbientStackType.apply(3)`: it has no body (team/creative/ambientsounds/engine/AmbientStackType.java:53)

### `AmbientTickHandler`: it uses AmbientEngine, which isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition))))

- [ ] `AmbientTickHandler.mc`: it uses Minecraft's `Minecraft` (team/creative/ambientsounds/engine/AmbientTickHandler.java:28)
- [ ] `AmbientTickHandler.scheduleReload(0)`: its class isn't translated (it uses AmbientEngine, which isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition))))) (team/creative/ambientsounds/engine/AmbientTickHandler.java:37)
- [ ] `AmbientTickHandler.setEngine(1)`: its class isn't translated (it uses AmbientEngine, which isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition))))) (team/creative/ambientsounds/engine/AmbientTickHandler.java:41)
- [ ] `AmbientTickHandler.initConfiguration(0)`: its class isn't translated (it uses AmbientEngine, which isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition))))) (team/creative/ambientsounds/engine/AmbientTickHandler.java:47)
- [ ] `AmbientTickHandler.createSoundCategoryConfiguration(3)`: its class isn't translated (it uses AmbientEngine, which isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition))))) (team/creative/ambientsounds/engine/AmbientTickHandler.java:88)
- [ ] `AmbientTickHandler.onRender(1)`: its class isn't translated (it uses AmbientEngine, which isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition))))) (team/creative/ambientsounds/engine/AmbientTickHandler.java:100)
- [ ] `AmbientTickHandler.loadLevel(1)`: its class isn't translated (it uses AmbientEngine, which isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition))))) (team/creative/ambientsounds/engine/AmbientTickHandler.java:146)
- [ ] `AmbientTickHandler.onTick(0)`: its class isn't translated (it uses AmbientEngine, which isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition))))) (team/creative/ambientsounds/engine/AmbientTickHandler.java:160)

### `AmbientEntityCondition`: it uses AmbientMinMaxFadeCondition, which isn't translated (it extends `AmbientMinMaxCondition`)

- [ ] `AmbientEntityCondition.init(1)`: its class isn't translated (it uses AmbientMinMaxFadeCondition, which isn't translated (it extends `AmbientMinMaxCondition`)) (team/creative/ambientsounds/entity/AmbientEntityCondition.java:75)
- [ ] `AmbientEntityCondition.value(1)`: its class isn't translated (it uses AmbientMinMaxFadeCondition, which isn't translated (it extends `AmbientMinMaxCondition`)) (team/creative/ambientsounds/entity/AmbientEntityCondition.java:125)
- [ ] `AmbientEntityCondition.hasNbt(2)`: its class isn't translated (it uses AmbientMinMaxFadeCondition, which isn't translated (it extends `AmbientMinMaxCondition`)) (team/creative/ambientsounds/entity/AmbientEntityCondition.java:181)
- [ ] `AmbientEntityCondition.isTeam(2)`: its class isn't translated (it uses AmbientMinMaxFadeCondition, which isn't translated (it extends `AmbientMinMaxCondition`)) (team/creative/ambientsounds/entity/AmbientEntityCondition.java:195)
- [ ] `AmbientEntityCondition.containsTag(2)`: its class isn't translated (it uses AmbientMinMaxFadeCondition, which isn't translated (it extends `AmbientMinMaxCondition`)) (team/creative/ambientsounds/entity/AmbientEntityCondition.java:199)
- [ ] `AmbientEntityCondition.checkEntityName(2)`: its class isn't translated (it uses AmbientMinMaxFadeCondition, which isn't translated (it extends `AmbientMinMaxCondition`)) (team/creative/ambientsounds/entity/AmbientEntityCondition.java:211)

### `AmbientEntityCondition.StringJson`: it extends `TypeAdapter` (com.google.gson)

- [ ] `AmbientEntityCondition.StringJson.write(2)`: its class isn't translated (it extends `TypeAdapter` (com.google.gson)) (team/creative/ambientsounds/entity/AmbientEntityCondition.java:222)
- [ ] `AmbientEntityCondition.StringJson.read(1)`: its class isn't translated (it extends `TypeAdapter` (com.google.gson)) (team/creative/ambientsounds/entity/AmbientEntityCondition.java:236)

### `AmbientEnvironment`: it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)))

- [ ] `AmbientEnvironment.isRainAudibleAtSurface(0)`: its class isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)))) (team/creative/ambientsounds/environment/AmbientEnvironment.java:40)
- [ ] `AmbientEnvironment.analyzeFast(5)`: its class isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)))) (team/creative/ambientsounds/environment/AmbientEnvironment.java:44)
- [ ] `AmbientEnvironment.analyzeTime(3)`: its class isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)))) (team/creative/ambientsounds/environment/AmbientEnvironment.java:59)
- [ ] `AmbientEnvironment.isEyeInFluid(2)`: its class isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)))) (team/creative/ambientsounds/environment/AmbientEnvironment.java:69)
- [ ] `AmbientEnvironment.isWater(2)`: its class isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)))) (team/creative/ambientsounds/environment/AmbientEnvironment.java:79)
- [ ] `AmbientEnvironment.analyzeUnderwater(3)`: its class isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)))) (team/creative/ambientsounds/environment/AmbientEnvironment.java:89)
- [ ] `AmbientEnvironment.analyzeSlow(5)`: its class isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)))) (team/creative/ambientsounds/environment/AmbientEnvironment.java:103)
- [ ] `AmbientEnvironment.collectLevelDetails(1)`: its class isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)))) (team/creative/ambientsounds/environment/AmbientEnvironment.java:109)
- [ ] `AmbientEnvironment.collectPlayerDetails(2)`: its class isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)))) (team/creative/ambientsounds/environment/AmbientEnvironment.java:120)
- [ ] `AmbientEnvironment.collectTerrainDetails(1)`: its class isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)))) (team/creative/ambientsounds/environment/AmbientEnvironment.java:137)
- [ ] `AmbientEnvironment.collectBiomeDetails(1)`: its class isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)))) (team/creative/ambientsounds/environment/AmbientEnvironment.java:141)
- [ ] `AmbientEnvironment.reload(0)`: its class isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)))) (team/creative/ambientsounds/environment/AmbientEnvironment.java:146)

### `BiomeEnvironment`: it implements `Iterable`

- [ ] `BiomeEnvironment.BiomeEnvironment(0)`: its class isn't translated (it implements `Iterable`) (team/creative/ambientsounds/environment/BiomeEnvironment.java:26)
- [ ] `BiomeEnvironment.BiomeEnvironment(4)`: its class isn't translated (it implements `Iterable`) (team/creative/ambientsounds/environment/BiomeEnvironment.java:29)
- [ ] `BiomeEnvironment.iterator(0)`: its class isn't translated (it implements `Iterable`) (team/creative/ambientsounds/environment/BiomeEnvironment.java:58)
- [ ] `BiomeEnvironment.rainVolume(0)`: its class isn't translated (it implements `Iterable`) (team/creative/ambientsounds/environment/BiomeEnvironment.java:63)
- [ ] `BiomeEnvironment.collectDetails(1)`: its class isn't translated (it implements `Iterable`) (team/creative/ambientsounds/environment/BiomeEnvironment.java:67)

### `BiomeEnvironment.BiomeArea`: it uses Minecraft's `Holder`

- [ ] `BiomeEnvironment.BiomeArea.BiomeArea(2)`: its class isn't translated (it uses Minecraft's `Holder`) (team/creative/ambientsounds/environment/BiomeEnvironment.java:78)
- [ ] `BiomeEnvironment.BiomeArea.checkBiome(1)`: its class isn't translated (it uses Minecraft's `Holder`) (team/creative/ambientsounds/environment/BiomeEnvironment.java:84)
- [ ] `BiomeEnvironment.BiomeArea.equals(1)`: its class isn't translated (it uses Minecraft's `Holder`) (team/creative/ambientsounds/environment/BiomeEnvironment.java:98)
- [ ] `BiomeEnvironment.BiomeArea.hashCode(0)`: its class isn't translated (it uses Minecraft's `Holder`) (team/creative/ambientsounds/environment/BiomeEnvironment.java:103)

### `EntityEnvironment`: it uses Minecraft's `Entity`

- [ ] `EntityEnvironment.analyzeFast(4)`: its class isn't translated (it uses Minecraft's `Entity`) (team/creative/ambientsounds/environment/EntityEnvironment.java:16)
- [ ] `EntityEnvironment.squaredDistance(1)`: its class isn't translated (it uses Minecraft's `Entity`) (team/creative/ambientsounds/environment/EntityEnvironment.java:25)
- [ ] `EntityEnvironment.x(0)`: its class isn't translated (it uses Minecraft's `Entity`) (team/creative/ambientsounds/environment/EntityEnvironment.java:29)
- [ ] `EntityEnvironment.y(0)`: its class isn't translated (it uses Minecraft's `Entity`) (team/creative/ambientsounds/environment/EntityEnvironment.java:33)
- [ ] `EntityEnvironment.z(0)`: its class isn't translated (it uses Minecraft's `Entity`) (team/creative/ambientsounds/environment/EntityEnvironment.java:37)
- [ ] `EntityEnvironment.all(0)`: its class isn't translated (it uses Minecraft's `Entity`) (team/creative/ambientsounds/environment/EntityEnvironment.java:41)

### `AmbientFeature`: it extends `AmbientFeatureSelection`


### `AmbientFeatureSelection`

- [ ] `AmbientFeatureSelection.collectGroups(1)`: Set.addAll of a List<String[]> (team/creative/ambientsounds/environment/feature/AmbientFeatureSelection.java:27)
- [ ] `AmbientFeatureSelection.volume(1)`: it's overloaded with the same number of parameters (team/creative/ambientsounds/environment/feature/AmbientFeatureSelection.java:51)

### `AirPocket`: it uses `HashMapDouble` (team.creative.creativecore.common.util.type.map)

- [ ] `AirPocket.AirPocket(0)`: its class isn't translated (it uses `HashMapDouble` (team.creative.creativecore.common.util.type.map)) (team/creative/ambientsounds/environment/pocket/AirPocket.java:16)
- [ ] `AirPocket.AirPocket(7)`: its class isn't translated (it uses `HashMapDouble` (team.creative.creativecore.common.util.type.map)) (team/creative/ambientsounds/environment/pocket/AirPocket.java:24)
- [ ] `AirPocket.volume(1)`: its class isn't translated (it uses `HashMapDouble` (team.creative.creativecore.common.util.type.map)) (team/creative/ambientsounds/environment/pocket/AirPocket.java:60)

### `AirPocketGroup`


### `AirPocketScanner`: it extends `Thread`

- [ ] `AirPocketScanner.AirPocketScanner(4)`: its class isn't translated (it extends `Thread`) (team/creative/ambientsounds/environment/pocket/AirPocketScanner.java:45)
- [ ] `AirPocketScanner.run(0)`: its class isn't translated (it extends `Thread`) (team/creative/ambientsounds/environment/pocket/AirPocketScanner.java:53)
- [ ] `AirPocketScanner.getOrCreate(1)`: its class isn't translated (it extends `Thread`) (team/creative/ambientsounds/environment/pocket/AirPocketScanner.java:106)
- [ ] `AirPocketScanner.findState(2)`: its class isn't translated (it extends `Thread`) (team/creative/ambientsounds/environment/pocket/AirPocketScanner.java:116)
- [ ] `AirPocketScanner.hearThrough(3)`: its class isn't translated (it extends `Thread`) (team/creative/ambientsounds/environment/pocket/AirPocketScanner.java:122)
- [ ] `AirPocketScanner.scan(3)`: its class isn't translated (it extends `Thread`) (team/creative/ambientsounds/environment/pocket/AirPocketScanner.java:126)
- [ ] `AirPocketScanner.canSeeSkyConsiderSolids(2)`: its class isn't translated (it extends `Thread`) (team/creative/ambientsounds/environment/pocket/AirPocketScanner.java:196)

### `AirPocketScanner.BlockPosInspection`: it extends Minecraft's `BlockPos`

- [ ] `AirPocketScanner.BlockPosInspection.BlockPosInspection(1)`: its class isn't translated (it extends Minecraft's `BlockPos`) (team/creative/ambientsounds/environment/pocket/AirPocketScanner.java:222)
- [ ] `AirPocketScanner.BlockPosInspection.BlockPosInspection(2)`: its class isn't translated (it extends Minecraft's `BlockPos`) (team/creative/ambientsounds/environment/pocket/AirPocketScanner.java:227)
- [ ] `AirPocketScanner.BlockPosInspection.add(1)`: its class isn't translated (it extends Minecraft's `BlockPos`) (team/creative/ambientsounds/environment/pocket/AirPocketScanner.java:232)
- [ ] `AirPocketScanner.BlockPosInspection.isUp(0)`: its class isn't translated (it extends Minecraft's `BlockPos`) (team/creative/ambientsounds/environment/pocket/AirPocketScanner.java:254)
- [ ] `AirPocketScanner.BlockPosInspection.is(1)`: its class isn't translated (it extends Minecraft's `BlockPos`) (team/creative/ambientsounds/environment/pocket/AirPocketScanner.java:258)
- [ ] `AirPocketScanner.BlockPosInspection.iterator(0)`: its body couldn't be read: an anonymous class (team/creative/ambientsounds/environment/pocket/AirPocketScanner.java:277)

### `BlockDistribution`

- [ ] `BlockDistribution.add(1)`: it's overloaded with the same number of parameters (team/creative/ambientsounds/environment/pocket/BlockDistribution.java:7)
- [x] `BlockDistribution.calculatePercentage(1)` (team/creative/ambientsounds/environment/pocket/BlockDistribution.java:11)
- [ ] `BlockDistribution.add(1)`: it's overloaded with the same number of parameters (team/creative/ambientsounds/environment/pocket/BlockDistribution.java:15)
- [x] `BlockDistribution.toString(0)` (team/creative/ambientsounds/environment/pocket/BlockDistribution.java:20)

### `TerrainEnvironment`: it uses AirPocket, which isn't translated (it uses `HashMapDouble` (team.creative.creativecore.common.util.type.map))

- [ ] `TerrainEnvironment.POSSIBLE_HEIGHT_BLOCK`: it uses Minecraft's `BlockState` (team/creative/ambientsounds/environment/TerrainEnvironment.java:23)
- [ ] `TerrainEnvironment.getHeightBlock(2)`: it uses Minecraft's `Level` (team/creative/ambientsounds/environment/TerrainEnvironment.java:32)
- [ ] `TerrainEnvironment.getHeightScanStart(2)`: it uses Minecraft's `Level` (team/creative/ambientsounds/environment/TerrainEnvironment.java:47)
- [ ] `TerrainEnvironment.TerrainEnvironment(0)`: its class isn't translated (it uses AirPocket, which isn't translated (it uses `HashMapDouble` (team.creative.creativecore.common.util.type.map))) (team/creative/ambientsounds/environment/TerrainEnvironment.java:70)
- [ ] `TerrainEnvironment.analyze(4)`: its class isn't translated (it uses AirPocket, which isn't translated (it uses `HashMapDouble` (team.creative.creativecore.common.util.type.map))) (team/creative/ambientsounds/environment/TerrainEnvironment.java:76)
- [ ] `TerrainEnvironment.analyzeHeight(4)`: its class isn't translated (it uses AirPocket, which isn't translated (it uses `HashMapDouble` (team.creative.creativecore.common.util.type.map))) (team/creative/ambientsounds/environment/TerrainEnvironment.java:81)
- [ ] `TerrainEnvironment.analyzeAirPocket(3)`: its class isn't translated (it uses AirPocket, which isn't translated (it uses `HashMapDouble` (team.creative.creativecore.common.util.type.map))) (team/creative/ambientsounds/environment/TerrainEnvironment.java:111)
- [ ] `TerrainEnvironment.collectDetails(1)`: its class isn't translated (it uses AirPocket, which isn't translated (it uses `HashMapDouble` (team.creative.creativecore.common.util.type.map))) (team/creative/ambientsounds/environment/TerrainEnvironment.java:120)
- [ ] `OggAudioStreamMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (team/creative/ambientsounds/environment/TerrainEnvironment.java:18)
- [ ] `SoundBufferLibraryAccessor`: a Mixin class: its handlers are the injections, in the bundle's helpers (team/creative/ambientsounds/environment/TerrainEnvironment.java:9)

### `SereneSeasonsCompat`

- [ ] `SereneSeasonsCompat.getBiomeTemperature`: it uses `Method` (java.lang.reflect) (team/creative/ambientsounds/mod/SereneSeasonsCompat.java:13)
- [ ] `SereneSeasonsCompat.getTemperature(1)`: it uses Minecraft's `Level` (team/creative/ambientsounds/mod/SereneSeasonsCompat.java:15)

### `AmbientRegion`: it extends `AmbientCondition` (team.creative.ambientsounds.condition)

- [ ] `AmbientRegion.load(3)`: its class isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)) (team/creative/ambientsounds/region/AmbientRegion.java:33)
- [ ] `AmbientRegion.regionName(0)`: its class isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)) (team/creative/ambientsounds/region/AmbientRegion.java:44)
- [ ] `AmbientRegion.init(1)`: its class isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)) (team/creative/ambientsounds/region/AmbientRegion.java:49)
- [ ] `AmbientRegion.value(1)`: its class isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)) (team/creative/ambientsounds/region/AmbientRegion.java:67)
- [ ] `AmbientRegion.fastTick(1)`: its class isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)) (team/creative/ambientsounds/region/AmbientRegion.java:85)
- [ ] `AmbientRegion.tick(1)`: its class isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)) (team/creative/ambientsounds/region/AmbientRegion.java:101)
- [ ] `AmbientRegion.isActive(0)`: its class isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)) (team/creative/ambientsounds/region/AmbientRegion.java:123)
- [ ] `AmbientRegion.activate(0)`: its class isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)) (team/creative/ambientsounds/region/AmbientRegion.java:127)
- [ ] `AmbientRegion.deactivate(0)`: its class isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)) (team/creative/ambientsounds/region/AmbientRegion.java:131)
- [ ] `AmbientRegion.toString(0)`: its class isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)) (team/creative/ambientsounds/region/AmbientRegion.java:142)

### `AmbientSound`: it extends `AmbientCondition` (team.creative.ambientsounds.condition)

- [ ] `AmbientSound.RANDOM`: it creates `Random` (java.util) (team/creative/ambientsounds/sound/AmbientSound.java:48)
- [ ] `AmbientSound.COPYFIELDS`: it uses `Field` (java.lang.reflect) (team/creative/ambientsounds/sound/AmbientSound.java:49)
- [ ] `AmbientSound.AmbientSound(0)`: its class isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)) (team/creative/ambientsounds/sound/AmbientSound.java:74)
- [ ] `AmbientSound.getSoundSource(1)`: it uses Minecraft's `SoundSource` (team/creative/ambientsounds/sound/AmbientSound.java:87)
- [ ] `AmbientSound.init(1)`: its class isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)) (team/creative/ambientsounds/sound/AmbientSound.java:105)
- [ ] `AmbientSound.getRandomFile(0)`: its class isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)) (team/creative/ambientsounds/sound/AmbientSound.java:144)
- [ ] `AmbientSound.getRandomFileExcept(1)`: its class isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)) (team/creative/ambientsounds/sound/AmbientSound.java:148)
- [ ] `AmbientSound.fastTick(1)`: its class isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)) (team/creative/ambientsounds/sound/AmbientSound.java:161)
- [ ] `AmbientSound.value(1)`: its class isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)) (team/creative/ambientsounds/sound/AmbientSound.java:260)
- [ ] `AmbientSound.isAudible(0)`: its class isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)) (team/creative/ambientsounds/sound/AmbientSound.java:284)
- [ ] `AmbientSound.tick(2)`: its class isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)) (team/creative/ambientsounds/sound/AmbientSound.java:288)
- [ ] `AmbientSound.play(2)`: its class isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)) (team/creative/ambientsounds/sound/AmbientSound.java:312)
- [ ] `AmbientSound.playTransition(2)`: its class isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)) (team/creative/ambientsounds/sound/AmbientSound.java:323)
- [ ] `AmbientSound.isPlaying(0)`: its class isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)) (team/creative/ambientsounds/sound/AmbientSound.java:336)
- [ ] `AmbientSound.inTransition(0)`: its class isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)) (team/creative/ambientsounds/sound/AmbientSound.java:340)
- [ ] `AmbientSound.isActive(0)`: its class isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)) (team/creative/ambientsounds/sound/AmbientSound.java:344)
- [ ] `AmbientSound.activate(0)`: its class isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)) (team/creative/ambientsounds/sound/AmbientSound.java:348)
- [ ] `AmbientSound.deactivate(0)`: its class isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)) (team/creative/ambientsounds/sound/AmbientSound.java:352)
- [ ] `AmbientSound.onSoundFinished(0)`: its class isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)) (team/creative/ambientsounds/sound/AmbientSound.java:365)
- [ ] `AmbientSound.loop(0)`: its class isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)) (team/creative/ambientsounds/sound/AmbientSound.java:374)
- [ ] `AmbientSound.toString(0)`: its class isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)) (team/creative/ambientsounds/sound/AmbientSound.java:378)
- [ ] `AmbientSound.copy(0)`: its class isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)) (team/creative/ambientsounds/sound/AmbientSound.java:396)

### `AmbientSound.SoundStream`: it implements Minecraft's `TickableSoundInstance`

- [ ] `AmbientSound.SoundStream.rand`: it calls Minecraft's `RandomSource`.createThreadLocalInstance (team/creative/ambientsounds/sound/AmbientSound.java:411)
- [ ] `AmbientSound.SoundStream.SoundStream(1)`: its class isn't translated (it implements Minecraft's `TickableSoundInstance`) (team/creative/ambientsounds/sound/AmbientSound.java:425)
- [ ] `AmbientSound.SoundStream.loop(0)`: its class isn't translated (it implements Minecraft's `TickableSoundInstance`) (team/creative/ambientsounds/sound/AmbientSound.java:432)
- [ ] `AmbientSound.SoundStream.remaining(0)`: its class isn't translated (it implements Minecraft's `TickableSoundInstance`) (team/creative/ambientsounds/sound/AmbientSound.java:436)
- [ ] `AmbientSound.SoundStream.conditionVolume(0)`: its class isn't translated (it implements Minecraft's `TickableSoundInstance`) (team/creative/ambientsounds/sound/AmbientSound.java:440)
- [ ] `AmbientSound.SoundStream.combinedVolume(0)`: its class isn't translated (it implements Minecraft's `TickableSoundInstance`) (team/creative/ambientsounds/sound/AmbientSound.java:444)
- [ ] `AmbientSound.SoundStream.mute(0)`: its class isn't translated (it implements Minecraft's `TickableSoundInstance`) (team/creative/ambientsounds/sound/AmbientSound.java:448)
- [ ] `AmbientSound.SoundStream.mutePriority(0)`: its class isn't translated (it implements Minecraft's `TickableSoundInstance`) (team/creative/ambientsounds/sound/AmbientSound.java:452)
- [ ] `AmbientSound.SoundStream.onStart(0)`: its class isn't translated (it implements Minecraft's `TickableSoundInstance`) (team/creative/ambientsounds/sound/AmbientSound.java:456)
- [ ] `AmbientSound.SoundStream.onFinished(0)`: its class isn't translated (it implements Minecraft's `TickableSoundInstance`) (team/creative/ambientsounds/sound/AmbientSound.java:461)
- [ ] `AmbientSound.SoundStream.hasPlayedOnce(0)`: its class isn't translated (it implements Minecraft's `TickableSoundInstance`) (team/creative/ambientsounds/sound/AmbientSound.java:466)
- [ ] `AmbientSound.SoundStream.setPlayedOnce(0)`: its class isn't translated (it implements Minecraft's `TickableSoundInstance`) (team/creative/ambientsounds/sound/AmbientSound.java:470)
- [ ] `AmbientSound.SoundStream.hasFinished(0)`: its class isn't translated (it implements Minecraft's `TickableSoundInstance`) (team/creative/ambientsounds/sound/AmbientSound.java:474)
- [ ] `AmbientSound.SoundStream.toString(0)`: its class isn't translated (it implements Minecraft's `TickableSoundInstance`) (team/creative/ambientsounds/sound/AmbientSound.java:478)
- [ ] `AmbientSound.SoundStream.isLooping(0)`: its class isn't translated (it implements Minecraft's `TickableSoundInstance`) (team/creative/ambientsounds/sound/AmbientSound.java:496)
- [ ] `AmbientSound.SoundStream.getSoundEvent(0)`: its class isn't translated (it implements Minecraft's `TickableSoundInstance`) (team/creative/ambientsounds/sound/AmbientSound.java:500)
- [ ] `AmbientSound.SoundStream.getOrResolve(1)`: its class isn't translated (it implements Minecraft's `TickableSoundInstance`) (team/creative/ambientsounds/sound/AmbientSound.java:504)
- [ ] `AmbientSound.SoundStream.getAttenuation(0)`: its class isn't translated (it implements Minecraft's `TickableSoundInstance`) (team/creative/ambientsounds/sound/AmbientSound.java:520)
- [ ] `AmbientSound.SoundStream.getSource(0)`: its class isn't translated (it implements Minecraft's `TickableSoundInstance`) (team/creative/ambientsounds/sound/AmbientSound.java:524)
- [ ] `AmbientSound.SoundStream.getPitch(0)`: its class isn't translated (it implements Minecraft's `TickableSoundInstance`) (team/creative/ambientsounds/sound/AmbientSound.java:528)
- [ ] `AmbientSound.SoundStream.getDelay(0)`: its class isn't translated (it implements Minecraft's `TickableSoundInstance`) (team/creative/ambientsounds/sound/AmbientSound.java:532)
- [ ] `AmbientSound.SoundStream.getSound(0)`: its class isn't translated (it implements Minecraft's `TickableSoundInstance`) (team/creative/ambientsounds/sound/AmbientSound.java:536)
- [ ] `AmbientSound.SoundStream.getIdentifier(0)`: its class isn't translated (it implements Minecraft's `TickableSoundInstance`) (team/creative/ambientsounds/sound/AmbientSound.java:540)
- [ ] `AmbientSound.SoundStream.getVolume(0)`: its class isn't translated (it implements Minecraft's `TickableSoundInstance`) (team/creative/ambientsounds/sound/AmbientSound.java:544)
- [ ] `AmbientSound.SoundStream.getX(0)`: its class isn't translated (it implements Minecraft's `TickableSoundInstance`) (team/creative/ambientsounds/sound/AmbientSound.java:548)
- [ ] `AmbientSound.SoundStream.getY(0)`: its class isn't translated (it implements Minecraft's `TickableSoundInstance`) (team/creative/ambientsounds/sound/AmbientSound.java:552)
- [ ] `AmbientSound.SoundStream.getZ(0)`: its class isn't translated (it implements Minecraft's `TickableSoundInstance`) (team/creative/ambientsounds/sound/AmbientSound.java:556)
- [ ] `AmbientSound.SoundStream.isStopped(0)`: its class isn't translated (it implements Minecraft's `TickableSoundInstance`) (team/creative/ambientsounds/sound/AmbientSound.java:560)
- [ ] `AmbientSound.SoundStream.tick(0)`: its class isn't translated (it implements Minecraft's `TickableSoundInstance`) (team/creative/ambientsounds/sound/AmbientSound.java:564)
- [ ] `AmbientSound.SoundStream.isRelative(0)`: its class isn't translated (it implements Minecraft's `TickableSoundInstance`) (team/creative/ambientsounds/sound/AmbientSound.java:567)
- [ ] `AmbientSound.SoundStream.canStartSilent(0)`: its class isn't translated (it implements Minecraft's `TickableSoundInstance`) (team/creative/ambientsounds/sound/AmbientSound.java:571)
- [ ] `AmbientSound.SoundStream.getAudioStream(3)`: its body couldn't be read: an anonymous class (team/creative/ambientsounds/sound/AmbientSound.java:575)
- [ ] `AmbientSound.SoundStream.collectDetails(1)`: its class isn't translated (it implements Minecraft's `TickableSoundInstance`) (team/creative/ambientsounds/sound/AmbientSound.java:619)

### `AmbientSoundCategory`: it extends `AmbientCondition` (team.creative.ambientsounds.condition)

- [ ] `AmbientSoundCategory.init(1)`: its class isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)) (team/creative/ambientsounds/sound/AmbientSoundCategory.java:25)
- [ ] `AmbientSoundCategory.hashCode(0)`: its class isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)) (team/creative/ambientsounds/sound/AmbientSoundCategory.java:38)
- [ ] `AmbientSoundCategory.postInit(1)`: its class isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)) (team/creative/ambientsounds/sound/AmbientSoundCategory.java:43)
- [ ] `AmbientSoundCategory.tick(2)`: its class isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)) (team/creative/ambientsounds/sound/AmbientSoundCategory.java:51)
- [ ] `AmbientSoundCategory.collectDetails(1)`: its class isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)) (team/creative/ambientsounds/sound/AmbientSoundCategory.java:66)

### `AmbientSoundCollection`: it uses AmbientStackType, which isn't translated (its constants carry values)


### `AmbientSoundEngine`: it uses SoundStream, which isn't translated (it implements Minecraft's `TickableSoundInstance`)

- [ ] `AmbientSoundEngine.mc`: it uses Minecraft's `Minecraft` (team/creative/ambientsounds/sound/AmbientSoundEngine.java:17)
- [ ] `AmbientSoundEngine.playingCount(0)`: its class isn't translated (it uses SoundStream, which isn't translated (it implements Minecraft's `TickableSoundInstance`)) (team/creative/ambientsounds/sound/AmbientSoundEngine.java:20)
- [ ] `AmbientSoundEngine.getManager(0)`: its class isn't translated (it uses SoundStream, which isn't translated (it implements Minecraft's `TickableSoundInstance`)) (team/creative/ambientsounds/sound/AmbientSoundEngine.java:26)
- [ ] `AmbientSoundEngine.tick(1)`: its class isn't translated (it uses SoundStream, which isn't translated (it implements Minecraft's `TickableSoundInstance`)) (team/creative/ambientsounds/sound/AmbientSoundEngine.java:30)
- [ ] `AmbientSoundEngine.stop(1)`: its class isn't translated (it uses SoundStream, which isn't translated (it implements Minecraft's `TickableSoundInstance`)) (team/creative/ambientsounds/sound/AmbientSoundEngine.java:102)
- [ ] `AmbientSoundEngine.play(1)`: its class isn't translated (it uses SoundStream, which isn't translated (it implements Minecraft's `TickableSoundInstance`)) (team/creative/ambientsounds/sound/AmbientSoundEngine.java:109)
- [ ] `AmbientSoundEngine.stopAll(0)`: its class isn't translated (it uses SoundStream, which isn't translated (it implements Minecraft's `TickableSoundInstance`)) (team/creative/ambientsounds/sound/AmbientSoundEngine.java:117)

### `AmbientSoundProperties`

- [ ] `AmbientSoundProperties.init(1)`: it uses AmbientEngine, which isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)))) (team/creative/ambientsounds/sound/AmbientSoundProperties.java:37)
- [ ] `AmbientSoundProperties.getFadeInVolume(1)`: it uses AmbientEngine, which isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)))) (team/creative/ambientsounds/sound/AmbientSoundProperties.java:43)
- [ ] `AmbientSoundProperties.getFadeOutVolume(1)`: it uses AmbientEngine, which isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)))) (team/creative/ambientsounds/sound/AmbientSoundProperties.java:51)
- [ ] `AmbientSoundProperties.getFadeInPitch(1)`: it uses AmbientEngine, which isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)))) (team/creative/ambientsounds/sound/AmbientSoundProperties.java:59)
- [ ] `AmbientSoundProperties.getFadeOutPitch(1)`: it uses AmbientEngine, which isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)))) (team/creative/ambientsounds/sound/AmbientSoundProperties.java:67)
- [ ] `AmbientSoundProperties.getPitch(1)`: it uses AmbientEnvironment, which isn't translated (it uses AmbientDimension, which isn't translated (it uses AmbientRegion, which isn't translated (it extends `AmbientCondition` (team.creative.ambientsounds.condition)))) (team/creative/ambientsounds/sound/AmbientSoundProperties.java:75)

### `AmbientSoundProperties.AmbientMinMaxClimbingProperty`

- [x] `AmbientSoundProperties.AmbientMinMaxClimbingProperty.getValue(1)` (team/creative/ambientsounds/sound/AmbientSoundProperties.java:89)

### `OggAudioStreamExtended`: it's an interface

- [ ] `OggAudioStreamExtended.setPositionRandomly(2)`: it has no body (team/creative/ambientsounds/sound/OggAudioStreamExtended.java:7)
