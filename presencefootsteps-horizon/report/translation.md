# Java translated to Rust

Translated 40 of 482 methods (1 more is done another way in Horizon) and 30 of 73 static fields to Rust (3 of those call the game, through Horizon's helpers); 0 of 2 Fabric event handlers. It's in `runtime.rs`, under `pub mod java`; code that calls the game is in `game_side.rs`.

Each method is translated whole or not at all. A method that can't be (it uses Minecraft, a lambda, null...) is left out with its reason, and so is every method that calls it.

## Why methods were left out

| Cause | Methods | For example |
|---|---|---|
| Calls or uses other code that isn't translated | 353 | `UpdateChecker.UpdateChecker(4)`: its class isn't translated (it uses Reporter, which isn't translated (it's an interface)) |
| Other Java this translator doesn't handle yet | 71 | `Reporter.report(2)`: it has no body |
| Uses Minecraft classes (needs Horizon's game API) | 12 | `DerivedBlock.getBaseOf(1)`: it uses Minecraft's `BlockState` |
| Uses other libraries | 2 | `PFOptionsScreen.formatVolume(1)`: it uses `AbstractSlider` (com.minelittlepony.common.client.gui.element) |
| Done another way in Horizon | 1 | `PFModMenuFactory.getModConfigScreenFactory(0)`: done another way in Horizon: the mod's page in Options > Mods (its settings buttons) |
| Uses files (java.nio / java.io) | 1 | `JsonFile.load(0)`: it calls `Files` (java.nio.file).isReadable |
| Uses null | 1 | `BlockReport.getUniqueFileName(3)`: `null` (Rust has no null: this needs an Option) |
| Uses try/catch | 1 | `JsonFile.save(0)`: a try-with-resources without a catch (its errors go up) |

## Fabric event handlers

| Handler | Hook | Result |
|---|---|---|
| eu/ha3/presencefootsteps/PresenceFootsteps.java:123 | `tick` | not translated: a handler that isn't a lambda or a Class::method reference |
| eu/ha3/presencefootsteps/PresenceFootsteps.java:123 | `init` | not translated: `this` in a static method |

## Every class


### `eu/ha3/presencefootsteps/sound/acoustics/WeightedAcoustic.java`: couldn't be read: expected a name but found `{` (line 39)


### `Reporter`: it's an interface

- [ ] `Reporter.report(2)`: it has no body (eu/ha3/mc/quick/update/Reporter.java:5)

### `TargettedVersion`

- [x] `TargettedVersion.TargettedVersion(2)` (eu/ha3/mc/quick/update/TargettedVersion.java:11)
- [ ] `TargettedVersion.TargettedVersion(1)`: its constructors are overloaded with the same number of parameters (eu/ha3/mc/quick/update/TargettedVersion.java:12)
- [ ] `TargettedVersion.TargettedVersion(1)`: its constructors are overloaded with the same number of parameters (eu/ha3/mc/quick/update/TargettedVersion.java:16)
- [ ] `TargettedVersion.getVersion(1)`: FabricLoader.getModContainer of a computed id (eu/ha3/mc/quick/update/TargettedVersion.java:20)
- [x] `TargettedVersion.minecraft(0)` (eu/ha3/mc/quick/update/TargettedVersion.java:11)
- [x] `TargettedVersion.version(0)` (eu/ha3/mc/quick/update/TargettedVersion.java:11)

### `UpdateChecker`: it uses Reporter, which isn't translated (it's an interface)

- [x] `UpdateChecker.LOGGER` (eu/ha3/mc/quick/update/UpdateChecker.java:15)
- [x] `UpdateChecker.GSON` (eu/ha3/mc/quick/update/UpdateChecker.java:16)
- [ ] `UpdateChecker.UpdateChecker(4)`: its class isn't translated (it uses Reporter, which isn't translated (it's an interface)) (eu/ha3/mc/quick/update/UpdateChecker.java:24)
- [ ] `UpdateChecker.getLast(0)`: its class isn't translated (it uses Reporter, which isn't translated (it's an interface)) (eu/ha3/mc/quick/update/UpdateChecker.java:31)
- [ ] `UpdateChecker.getNewer(0)`: its class isn't translated (it uses Reporter, which isn't translated (it's an interface)) (eu/ha3/mc/quick/update/UpdateChecker.java:35)
- [ ] `UpdateChecker.attempt(0)`: its class isn't translated (it uses Reporter, which isn't translated (it's an interface)) (eu/ha3/mc/quick/update/UpdateChecker.java:39)
- [ ] `UpdateChecker.checkNow(0)`: its class isn't translated (it uses Reporter, which isn't translated (it's an interface)) (eu/ha3/mc/quick/update/UpdateChecker.java:52)

### `UpdaterConfig`: it extends `JsonFile` (eu.ha3.presencefootsteps.config)

- [ ] `UpdaterConfig.UpdaterConfig(1)`: its class isn't translated (it extends `JsonFile` (eu.ha3.presencefootsteps.config)) (eu/ha3/mc/quick/update/UpdaterConfig.java:14)
- [ ] `UpdaterConfig.shouldReport(1)`: its class isn't translated (it extends `JsonFile` (eu.ha3.presencefootsteps.config)) (eu/ha3/mc/quick/update/UpdaterConfig.java:18)

### `Versions`

- [x] `Versions.Versions(2)` (eu/ha3/mc/quick/update/Versions.java:10)
- [ ] `Versions.Versions(1)`: it creates a TargettedVersion with a constructor that isn't translated (eu/ha3/mc/quick/update/Versions.java:11)
- [x] `Versions.latest(0)` (eu/ha3/mc/quick/update/Versions.java:10)
- [x] `Versions.previous(0)` (eu/ha3/mc/quick/update/Versions.java:10)

### `DerivedBlock`: it's an interface

- [ ] `DerivedBlock.getBaseOf(1)`: it uses Minecraft's `BlockState` (eu/ha3/presencefootsteps/api/DerivedBlock.java:8)
- [ ] `DerivedBlock.getBaseBlockState(0)`: it has no body (eu/ha3/presencefootsteps/api/DerivedBlock.java:12)

### `DerivedBlock.Settings`: it's an interface

- [ ] `DerivedBlock.Settings.getBaseBlock(0)`: it has no body (eu/ha3/presencefootsteps/api/DerivedBlock.java:15)
- [ ] `DerivedBlock.Settings.setBaseBlock(1)`: it has no body (eu/ha3/presencefootsteps/api/DerivedBlock.java:18)

### `ContraptionCollidable`: it's an interface

- [ ] `ContraptionCollidable.getCollidedStateAt(1)`: it has no body (eu/ha3/presencefootsteps/compat/ContraptionCollidable.java:7)

### `EntitySelector`: its constants carry values

- [ ] `EntitySelector.VALUES`: it uses EntitySelector, which isn't translated (its constants carry values) (eu/ha3/presencefootsteps/config/EntitySelector.java:27)
- [ ] `EntitySelector.EntitySelector(1)`: its class isn't translated (its constants carry values) (eu/ha3/presencefootsteps/config/EntitySelector.java:30)
- [ ] `EntitySelector.getAffectedSources(0)`: its class isn't translated (its constants carry values) (eu/ha3/presencefootsteps/config/EntitySelector.java:34)

### `JsonFile`

- [ ] `JsonFile.JsonFile(0)`: Gson.registerTypeAdapter (eu/ha3/presencefootsteps/config/JsonFile.java:18)
- [ ] `JsonFile.JsonFile(1)`: Gson.registerTypeAdapter (eu/ha3/presencefootsteps/config/JsonFile.java:22)
- [ ] `JsonFile.load(0)`: it calls `Files` (java.nio.file).isReadable (eu/ha3/presencefootsteps/config/JsonFile.java:27)
- [ ] `JsonFile.load(1)`: Gson.fromJson into something other than a class (a TypeToken) (eu/ha3/presencefootsteps/config/JsonFile.java:40)
- [ ] `JsonFile.save(0)`: a try-with-resources without a catch (its errors go up) (eu/ha3/presencefootsteps/config/JsonFile.java:45)

### `Variator`: it extends `JsonFile`


### `VolumeOption`: it implements `IChangeCallback` (com.minelittlepony.common.client.gui.IField)

- [ ] `VolumeOption.VolumeOption(2)`: its class isn't translated (it implements `IChangeCallback` (com.minelittlepony.common.client.gui.IField)) (eu/ha3/presencefootsteps/config/VolumeOption.java:12)
- [ ] `VolumeOption.get(0)`: its class isn't translated (it implements `IChangeCallback` (com.minelittlepony.common.client.gui.IField)) (eu/ha3/presencefootsteps/config/VolumeOption.java:17)
- [ ] `VolumeOption.getPercentage(0)`: its class isn't translated (it implements `IChangeCallback` (com.minelittlepony.common.client.gui.IField)) (eu/ha3/presencefootsteps/config/VolumeOption.java:21)
- [ ] `VolumeOption.set(1)`: its class isn't translated (it implements `IChangeCallback` (com.minelittlepony.common.client.gui.IField)) (eu/ha3/presencefootsteps/config/VolumeOption.java:25)
- [ ] `VolumeOption.perform(1)`: its class isn't translated (it implements `IChangeCallback` (com.minelittlepony.common.client.gui.IField)) (eu/ha3/presencefootsteps/config/VolumeOption.java:31)
- [ ] `VolumeOption.toString(0)`: its class isn't translated (it implements `IChangeCallback` (com.minelittlepony.common.client.gui.IField)) (eu/ha3/presencefootsteps/config/VolumeOption.java:35)
- [x] `VolumeOption.volumeScaleToInt(1)` (eu/ha3/presencefootsteps/config/VolumeOption.java:40)
- [ ] `IEntity`: a Mixin class: its handlers are the injections, in the bundle's helpers (eu/ha3/presencefootsteps/config/VolumeOption.java:13)
- [ ] `ILivingEntity`: a Mixin class: its handlers are the injections, in the bundle's helpers (eu/ha3/presencefootsteps/config/VolumeOption.java:8)
- [ ] `MAbstractBlock`: a Mixin class: its handlers are the injections, in the bundle's helpers (eu/ha3/presencefootsteps/config/VolumeOption.java:12)
- [ ] `MBlockSettings`: a Mixin class: its handlers are the injections, in the bundle's helpers (eu/ha3/presencefootsteps/config/VolumeOption.java:14)
- [ ] `MClientPlayNetworkHandler`: a Mixin class: its handlers are the injections, in the bundle's helpers (eu/ha3/presencefootsteps/config/VolumeOption.java:13)
- [ ] `MLivingEntity`: a Mixin class: its handlers are the injections, in the bundle's helpers (eu/ha3/presencefootsteps/config/VolumeOption.java:12)
- [ ] `MSoundSystem`: a Mixin class: its handlers are the injections, in the bundle's helpers (eu/ha3/presencefootsteps/config/VolumeOption.java:15)
- [ ] `MStairsBlock`: a Mixin class: its handlers are the injections, in the bundle's helpers (eu/ha3/presencefootsteps/config/VolumeOption.java:10)

### `PFConfig`: it extends `Config` (com.minelittlepony.common.util.settings)

- [ ] `PFConfig.PFConfig(2)`: its class isn't translated (it extends `Config` (com.minelittlepony.common.util.settings)) (eu/ha3/presencefootsteps/PFConfig.java:104)
- [ ] `PFConfig.toggleMultiplayer(0)`: its class isn't translated (it extends `Config` (com.minelittlepony.common.util.settings)) (eu/ha3/presencefootsteps/PFConfig.java:114)
- [ ] `PFConfig.isIgnoredForFootsteps(1)`: its class isn't translated (it extends `Config` (com.minelittlepony.common.util.settings)) (eu/ha3/presencefootsteps/PFConfig.java:120)
- [ ] `PFConfig.cycleTargetSelector(0)`: its class isn't translated (it extends `Config` (com.minelittlepony.common.util.settings)) (eu/ha3/presencefootsteps/PFConfig.java:124)
- [ ] `PFConfig.setLocomotion(1)`: its class isn't translated (it extends `Config` (com.minelittlepony.common.util.settings)) (eu/ha3/presencefootsteps/PFConfig.java:130)
- [ ] `PFConfig.setVisualiserRunning(1)`: its class isn't translated (it extends `Config` (com.minelittlepony.common.util.settings)) (eu/ha3/presencefootsteps/PFConfig.java:140)
- [ ] `PFConfig.isVisualiserRunning(0)`: its class isn't translated (it extends `Config` (com.minelittlepony.common.util.settings)) (eu/ha3/presencefootsteps/PFConfig.java:147)
- [ ] `PFConfig.getLocomotion(0)`: its class isn't translated (it extends `Config` (com.minelittlepony.common.util.settings)) (eu/ha3/presencefootsteps/PFConfig.java:151)
- [ ] `PFConfig.getEntitySelector(0)`: its class isn't translated (it extends `Config` (com.minelittlepony.common.util.settings)) (eu/ha3/presencefootsteps/PFConfig.java:155)
- [ ] `PFConfig.getEnabledFootwear(0)`: its class isn't translated (it extends `Config` (com.minelittlepony.common.util.settings)) (eu/ha3/presencefootsteps/PFConfig.java:159)
- [ ] `PFConfig.toggleFootwear(0)`: its class isn't translated (it extends `Config` (com.minelittlepony.common.util.settings)) (eu/ha3/presencefootsteps/PFConfig.java:163)
- [ ] `PFConfig.isExclusiveMode(0)`: its class isn't translated (it extends `Config` (com.minelittlepony.common.util.settings)) (eu/ha3/presencefootsteps/PFConfig.java:169)
- [ ] `PFConfig.toggleExclusiveMode(0)`: its class isn't translated (it extends `Config` (com.minelittlepony.common.util.settings)) (eu/ha3/presencefootsteps/PFConfig.java:173)
- [ ] `PFConfig.getEnabledMP(0)`: its class isn't translated (it extends `Config` (com.minelittlepony.common.util.settings)) (eu/ha3/presencefootsteps/PFConfig.java:179)
- [ ] `PFConfig.getMaxSteppingEntities(0)`: its class isn't translated (it extends `Config` (com.minelittlepony.common.util.settings)) (eu/ha3/presencefootsteps/PFConfig.java:183)
- [ ] `PFConfig.toggleDisabled(0)`: its class isn't translated (it extends `Config` (com.minelittlepony.common.util.settings)) (eu/ha3/presencefootsteps/PFConfig.java:187)
- [ ] `PFConfig.setDisabled(1)`: its class isn't translated (it extends `Config` (com.minelittlepony.common.util.settings)) (eu/ha3/presencefootsteps/PFConfig.java:194)
- [ ] `PFConfig.getDisabled(0)`: its class isn't translated (it extends `Config` (com.minelittlepony.common.util.settings)) (eu/ha3/presencefootsteps/PFConfig.java:202)
- [ ] `PFConfig.getEnabled(0)`: its class isn't translated (it extends `Config` (com.minelittlepony.common.util.settings)) (eu/ha3/presencefootsteps/PFConfig.java:206)
- [ ] `PFConfig.getGlobalVolume(0)`: its class isn't translated (it extends `Config` (com.minelittlepony.common.util.settings)) (eu/ha3/presencefootsteps/PFConfig.java:210)
- [ ] `PFConfig.getRunningVolumeIncrease(0)`: its class isn't translated (it extends `Config` (com.minelittlepony.common.util.settings)) (eu/ha3/presencefootsteps/PFConfig.java:214)
- [ ] `PFConfig.setGlobalVolume(1)`: its class isn't translated (it extends `Config` (com.minelittlepony.common.util.settings)) (eu/ha3/presencefootsteps/PFConfig.java:218)
- [ ] `PFConfig.setRunningVolumeIncrease(1)`: its class isn't translated (it extends `Config` (com.minelittlepony.common.util.settings)) (eu/ha3/presencefootsteps/PFConfig.java:232)
- [ ] `PFConfig.populateCrashReport(1)`: its class isn't translated (it extends `Config` (com.minelittlepony.common.util.settings)) (eu/ha3/presencefootsteps/PFConfig.java:238)
- [x] `PFConfig.volumeScaleToInt(1)` (eu/ha3/presencefootsteps/PFConfig.java:247)

### `PFDebugHud`: it implements Minecraft's `DebugScreenEntry`

- [x] `PFDebugHud.ID` (eu/ha3/presencefootsteps/PFDebugHud.java:36)
- [ ] `PFDebugHud.PFDebugHud(1)`: its class isn't translated (it implements Minecraft's `DebugScreenEntry`) (eu/ha3/presencefootsteps/PFDebugHud.java:39)
- [ ] `PFDebugHud.isAllowed(1)`: its class isn't translated (it implements Minecraft's `DebugScreenEntry`) (eu/ha3/presencefootsteps/PFDebugHud.java:43)
- [ ] `PFDebugHud.display(4)`: its class isn't translated (it implements Minecraft's `DebugScreenEntry`) (eu/ha3/presencefootsteps/PFDebugHud.java:47)
- [ ] `PFDebugHud.renderSoundList(2)`: its class isn't translated (it implements Minecraft's `DebugScreenEntry`) (eu/ha3/presencefootsteps/PFDebugHud.java:134)

### `PFModMenuFactory`: it implements `ModMenuApi` (com.terraformersmc.modmenu.api)

- [ ] `PFModMenuFactory.getModConfigScreenFactory(0)`: done another way in Horizon: the mod's page in Options > Mods (its settings buttons) (eu/ha3/presencefootsteps/PFModMenuFactory.java:7)

### `PFOptionsScreen`: it extends `GameGui` (com.minelittlepony.common.client.gui)

- [x] `PFOptionsScreen.TITLE` in game_side.rs (eu/ha3/presencefootsteps/PFOptionsScreen.java:37)
- [x] `PFOptionsScreen.VOLUME_MIN` in game_side.rs (eu/ha3/presencefootsteps/PFOptionsScreen.java:38)
- [ ] `PFOptionsScreen.PFOptionsScreen(1)`: its class isn't translated (it extends `GameGui` (com.minelittlepony.common.client.gui)) (eu/ha3/presencefootsteps/PFOptionsScreen.java:41)
- [ ] `PFOptionsScreen.init(0)`: its class isn't translated (it extends `GameGui` (com.minelittlepony.common.client.gui)) (eu/ha3/presencefootsteps/PFOptionsScreen.java:51)
- [ ] `PFOptionsScreen.rebuildContent(0)`: its class isn't translated (it extends `GameGui` (com.minelittlepony.common.client.gui)) (eu/ha3/presencefootsteps/PFOptionsScreen.java:55)
- [ ] `PFOptionsScreen.addVolumeSlider(4)`: its class isn't translated (it extends `GameGui` (com.minelittlepony.common.client.gui)) (eu/ha3/presencefootsteps/PFOptionsScreen.java:214)
- [ ] `PFOptionsScreen.updateDisableState(2)`: its class isn't translated (it extends `GameGui` (com.minelittlepony.common.client.gui)) (eu/ha3/presencefootsteps/PFOptionsScreen.java:222)
- [ ] `PFOptionsScreen.formatVolume(1)`: it uses `AbstractSlider` (com.minelittlepony.common.client.gui.element) (eu/ha3/presencefootsteps/PFOptionsScreen.java:230)
- [ ] `PFOptionsScreen.extractRenderState(4)`: its class isn't translated (it extends `GameGui` (com.minelittlepony.common.client.gui)) (eu/ha3/presencefootsteps/PFOptionsScreen.java:236)
- [ ] `PFOptionsScreen.formatVolume(1)`: it uses `AbstractSlider` (com.minelittlepony.common.client.gui.element) (eu/ha3/presencefootsteps/PFOptionsScreen.java:241)

### `PresenceFootsteps`: it implements Fabric's `ClientModInitializer`

- [x] `PresenceFootsteps.LOGGER` (eu/ha3/presencefootsteps/PresenceFootsteps.java:34)
- [x] `PresenceFootsteps.MODID` (eu/ha3/presencefootsteps/PresenceFootsteps.java:35)
- [ ] `PresenceFootsteps.KEY_BINDING_CATEGORY`: done another way in Horizon: the mod's keys in Options > Mods, read through Horizon's key hook (eu/ha3/presencefootsteps/PresenceFootsteps.java:36)
- [x] `PresenceFootsteps.UPDATER_ENDPOINT` (eu/ha3/presencefootsteps/PresenceFootsteps.java:37)
- [x] `PresenceFootsteps.MOD_NAME` in game_side.rs (eu/ha3/presencefootsteps/PresenceFootsteps.java:38)
- [ ] `PresenceFootsteps.PF_TOGGLED_TOAST_ID`: it uses Minecraft's `SystemToastId` (eu/ha3/presencefootsteps/PresenceFootsteps.java:39)
- [ ] `PresenceFootsteps.instance`: it uses PresenceFootsteps, which isn't translated (it implements Fabric's `ClientModInitializer`) (eu/ha3/presencefootsteps/PresenceFootsteps.java:40)
- [x] `PresenceFootsteps.id(1)` (eu/ha3/presencefootsteps/PresenceFootsteps.java:91)
- [ ] `PresenceFootsteps.getInstance(0)`: it uses PresenceFootsteps, which isn't translated (it implements Fabric's `ClientModInitializer`) (eu/ha3/presencefootsteps/PresenceFootsteps.java:95)
- [ ] `PresenceFootsteps.PresenceFootsteps(0)`: its class isn't translated (it implements Fabric's `ClientModInitializer`) (eu/ha3/presencefootsteps/PresenceFootsteps.java:99)
- [ ] `PresenceFootsteps.getDebugHud(0)`: its class isn't translated (it implements Fabric's `ClientModInitializer`) (eu/ha3/presencefootsteps/PresenceFootsteps.java:103)
- [ ] `PresenceFootsteps.getEngine(0)`: its class isn't translated (it implements Fabric's `ClientModInitializer`) (eu/ha3/presencefootsteps/PresenceFootsteps.java:107)
- [ ] `PresenceFootsteps.getConfig(0)`: its class isn't translated (it implements Fabric's `ClientModInitializer`) (eu/ha3/presencefootsteps/PresenceFootsteps.java:111)
- [ ] `PresenceFootsteps.getOptionsKeyBinding(0)`: its class isn't translated (it implements Fabric's `ClientModInitializer`) (eu/ha3/presencefootsteps/PresenceFootsteps.java:115)
- [ ] `PresenceFootsteps.getUpdateChecker(0)`: its class isn't translated (it implements Fabric's `ClientModInitializer`) (eu/ha3/presencefootsteps/PresenceFootsteps.java:119)
- [ ] `PresenceFootsteps.onInitializeClient(0)`: its class isn't translated (it implements Fabric's `ClientModInitializer`) (eu/ha3/presencefootsteps/PresenceFootsteps.java:123)
- [ ] `PresenceFootsteps.onTick(1)`: its class isn't translated (it implements Fabric's `ClientModInitializer`) (eu/ha3/presencefootsteps/PresenceFootsteps.java:136)
- [ ] `PresenceFootsteps.onEnabledStateChange(1)`: its class isn't translated (it implements Fabric's `ClientModInitializer`) (eu/ha3/presencefootsteps/PresenceFootsteps.java:159)
- [ ] `PresenceFootsteps.showSystemToast(2)`: its class isn't translated (it implements Fabric's `ClientModInitializer`) (eu/ha3/presencefootsteps/PresenceFootsteps.java:168)

### `Acoustic`: it's an interface

- [ ] `Acoustic.register(2)`: it's generic (eu/ha3/presencefootsteps/sound/acoustics/Acoustic.java:48)
- [ ] `Acoustic.read(2)`: it uses Acoustic, which isn't translated (it's an interface) (eu/ha3/presencefootsteps/sound/acoustics/Acoustic.java:53)
- [ ] `Acoustic.read(3)`: it uses Acoustic, which isn't translated (it's an interface) (eu/ha3/presencefootsteps/sound/acoustics/Acoustic.java:58)
- [ ] `Acoustic.getType(2)`: an instance field used from a static method (eu/ha3/presencefootsteps/sound/acoustics/Acoustic.java:67)
- [ ] `Acoustic.checked(2)`: it's generic (eu/ha3/presencefootsteps/sound/acoustics/Acoustic.java:79)
- [ ] `Acoustic.playSound(4)`: it has no body (eu/ha3/presencefootsteps/sound/acoustics/Acoustic.java:88)
- [ ] `Acoustic.type(0)`: it has no body (eu/ha3/presencefootsteps/sound/acoustics/Acoustic.java:90)

### `Acoustic.Serializer`: it's an interface

- [ ] `Acoustic.Serializer.create(2)`: it has no body (eu/ha3/presencefootsteps/sound/acoustics/Acoustic.java:94)
- [ ] `Acoustic.Serializer.ofJsObject(1)`: it uses Serializer, which isn't translated (it's an interface) (eu/ha3/presencefootsteps/sound/acoustics/Acoustic.java:97)

### `AcousticLibrary`: it's an interface

- [ ] `AcousticLibrary.load(1)`: it has no body (eu/ha3/presencefootsteps/sound/acoustics/AcousticLibrary.java:12)
- [ ] `AcousticLibrary.playStep(3)`: it has no body (eu/ha3/presencefootsteps/sound/acoustics/AcousticLibrary.java:14)
- [ ] `AcousticLibrary.playAcoustic(4)`: it has no body (eu/ha3/presencefootsteps/sound/acoustics/AcousticLibrary.java:16)
- [ ] `AcousticLibrary.think(0)`: it has no body (eu/ha3/presencefootsteps/sound/acoustics/AcousticLibrary.java:18)

### `AcousticsFile`

- [x] `AcousticsFile.FILE_LOCATION` (eu/ha3/presencefootsteps/sound/acoustics/AcousticsFile.java:15)
- [x] `AcousticsFile.ENGINE_VERSION` (eu/ha3/presencefootsteps/sound/acoustics/AcousticsFile.java:16)
- [x] `AcousticsFile.AcousticsFile(3)` (eu/ha3/presencefootsteps/sound/acoustics/AcousticsFile.java:14)
- [ ] `AcousticsFile.read(3)`: it uses Acoustic, which isn't translated (it's an interface) (eu/ha3/presencefootsteps/sound/acoustics/AcousticsFile.java:18)
- [x] `AcousticsFile.read(2)` (eu/ha3/presencefootsteps/sound/acoustics/AcousticsFile.java:34)
- [x] `AcousticsFile.expect(2)` (eu/ha3/presencefootsteps/sound/acoustics/AcousticsFile.java:52)
- [x] `AcousticsFile.getSoundName(1)` (eu/ha3/presencefootsteps/sound/acoustics/AcousticsFile.java:59)
- [x] `AcousticsFile.defaultVolume(0)` (eu/ha3/presencefootsteps/sound/acoustics/AcousticsFile.java:14)
- [x] `AcousticsFile.defaultPitch(0)` (eu/ha3/presencefootsteps/sound/acoustics/AcousticsFile.java:14)
- [x] `AcousticsFile.soundRoot(0)` (eu/ha3/presencefootsteps/sound/acoustics/AcousticsFile.java:14)

### `AcousticsPlayer`: it implements `AcousticLibrary`

- [ ] `AcousticsPlayer.AcousticsPlayer(1)`: its class isn't translated (it implements `AcousticLibrary`) (eu/ha3/presencefootsteps/sound/acoustics/AcousticsPlayer.java:21)
- [ ] `AcousticsPlayer.load(1)`: its class isn't translated (it implements `AcousticLibrary`) (eu/ha3/presencefootsteps/sound/acoustics/AcousticsPlayer.java:25)
- [ ] `AcousticsPlayer.addAcoustic(2)`: its class isn't translated (it implements `AcousticLibrary`) (eu/ha3/presencefootsteps/sound/acoustics/AcousticsPlayer.java:35)
- [ ] `AcousticsPlayer.playStep(3)`: its class isn't translated (it implements `AcousticLibrary`) (eu/ha3/presencefootsteps/sound/acoustics/AcousticsPlayer.java:38)
- [ ] `AcousticsPlayer.playVanillaStep(2)`: its class isn't translated (it implements `AcousticLibrary`) (eu/ha3/presencefootsteps/sound/acoustics/AcousticsPlayer.java:57)
- [ ] `AcousticsPlayer.playAcoustic(4)`: its class isn't translated (it implements `AcousticLibrary`) (eu/ha3/presencefootsteps/sound/acoustics/AcousticsPlayer.java:70)
- [ ] `AcousticsPlayer.think(0)`: its class isn't translated (it implements `AcousticLibrary`) (eu/ha3/presencefootsteps/sound/acoustics/AcousticsPlayer.java:82)

### `ChanceAcoustic`: it implements `Acoustic`

- [ ] `ChanceAcoustic.CODEC`: it uses Minecraft's `MapCodec` (eu/ha3/presencefootsteps/sound/acoustics/ChanceAcoustic.java:12)
- [ ] `ChanceAcoustic.FACTORY`: it uses Serializer, which isn't translated (it's an interface) (eu/ha3/presencefootsteps/sound/acoustics/ChanceAcoustic.java:19)
- [ ] `ChanceAcoustic.ChanceAcoustic(2)`: its class isn't translated (it implements `Acoustic`) (eu/ha3/presencefootsteps/sound/acoustics/ChanceAcoustic.java:11)
- [ ] `ChanceAcoustic.type(0)`: its class isn't translated (it implements `Acoustic`) (eu/ha3/presencefootsteps/sound/acoustics/ChanceAcoustic.java:23)
- [ ] `ChanceAcoustic.playSound(4)`: its class isn't translated (it implements `Acoustic`) (eu/ha3/presencefootsteps/sound/acoustics/ChanceAcoustic.java:28)
- [ ] `ChanceAcoustic.acoustic(0)`: its class isn't translated (it implements `Acoustic`) (eu/ha3/presencefootsteps/sound/acoustics/ChanceAcoustic.java:11)
- [ ] `ChanceAcoustic.probability(0)`: its class isn't translated (it implements `Acoustic`) (eu/ha3/presencefootsteps/sound/acoustics/ChanceAcoustic.java:11)

### `DelayedAcoustic`: it implements `Acoustic`

- [ ] `DelayedAcoustic.CODEC`: it uses Minecraft's `MapCodec` (eu/ha3/presencefootsteps/sound/acoustics/DelayedAcoustic.java:12)
- [ ] `DelayedAcoustic.FACTORY`: it uses Serializer, which isn't translated (it's an interface) (eu/ha3/presencefootsteps/sound/acoustics/DelayedAcoustic.java:17)
- [ ] `DelayedAcoustic.DelayedAcoustic(2)`: its class isn't translated (it implements `Acoustic`) (eu/ha3/presencefootsteps/sound/acoustics/DelayedAcoustic.java:11)
- [ ] `DelayedAcoustic.type(0)`: its class isn't translated (it implements `Acoustic`) (eu/ha3/presencefootsteps/sound/acoustics/DelayedAcoustic.java:23)
- [ ] `DelayedAcoustic.playSound(4)`: its class isn't translated (it implements `Acoustic`) (eu/ha3/presencefootsteps/sound/acoustics/DelayedAcoustic.java:28)
- [ ] `DelayedAcoustic.acoustic(0)`: its class isn't translated (it implements `Acoustic`) (eu/ha3/presencefootsteps/sound/acoustics/DelayedAcoustic.java:11)
- [ ] `DelayedAcoustic.delay(0)`: its class isn't translated (it implements `Acoustic`) (eu/ha3/presencefootsteps/sound/acoustics/DelayedAcoustic.java:11)

### `EventSelectorAcoustics`: it implements `Acoustic`

- [ ] `EventSelectorAcoustics.MAP_CODEC`: it uses Minecraft's `MapCodec` (eu/ha3/presencefootsteps/sound/acoustics/EventSelectorAcoustics.java:23)
- [ ] `EventSelectorAcoustics.CODEC`: its value couldn't be read: an anonymous class (eu/ha3/presencefootsteps/sound/acoustics/EventSelectorAcoustics.java:25)
- [ ] `EventSelectorAcoustics.FACTORY`: it uses Serializer, which isn't translated (it's an interface) (eu/ha3/presencefootsteps/sound/acoustics/EventSelectorAcoustics.java:51)
- [ ] `EventSelectorAcoustics.EventSelectorAcoustics(1)`: its class isn't translated (it implements `Acoustic`) (eu/ha3/presencefootsteps/sound/acoustics/EventSelectorAcoustics.java:22)
- [ ] `EventSelectorAcoustics.type(0)`: its class isn't translated (it implements `Acoustic`) (eu/ha3/presencefootsteps/sound/acoustics/EventSelectorAcoustics.java:59)
- [ ] `EventSelectorAcoustics.playSound(4)`: its class isn't translated (it implements `Acoustic`) (eu/ha3/presencefootsteps/sound/acoustics/EventSelectorAcoustics.java:64)
- [ ] `EventSelectorAcoustics.pairs(0)`: its class isn't translated (it implements `Acoustic`) (eu/ha3/presencefootsteps/sound/acoustics/EventSelectorAcoustics.java:22)

### `SimultaneousAcoustic`: it implements `Acoustic`

- [ ] `SimultaneousAcoustic.CODEC`: it uses Minecraft's `Codec` (eu/ha3/presencefootsteps/sound/acoustics/SimultaneousAcoustic.java:13)
- [ ] `SimultaneousAcoustic.MAP_CODEC`: it uses Minecraft's `MapCodec` (eu/ha3/presencefootsteps/sound/acoustics/SimultaneousAcoustic.java:14)
- [ ] `SimultaneousAcoustic.FACTORY`: it uses Serializer, which isn't translated (it's an interface) (eu/ha3/presencefootsteps/sound/acoustics/SimultaneousAcoustic.java:18)
- [ ] `SimultaneousAcoustic.SimultaneousAcoustic(1)`: its class isn't translated (it implements `Acoustic`) (eu/ha3/presencefootsteps/sound/acoustics/SimultaneousAcoustic.java:12)
- [ ] `SimultaneousAcoustic.type(0)`: its class isn't translated (it implements `Acoustic`) (eu/ha3/presencefootsteps/sound/acoustics/SimultaneousAcoustic.java:26)
- [ ] `SimultaneousAcoustic.playSound(4)`: its class isn't translated (it implements `Acoustic`) (eu/ha3/presencefootsteps/sound/acoustics/SimultaneousAcoustic.java:31)
- [ ] `SimultaneousAcoustic.acoustics(0)`: its class isn't translated (it implements `Acoustic`) (eu/ha3/presencefootsteps/sound/acoustics/SimultaneousAcoustic.java:12)

### `VaryingAcoustic`: it implements `Acoustic`

- [ ] `VaryingAcoustic.CODEC`: it uses Minecraft's `MapCodec` (eu/ha3/presencefootsteps/sound/acoustics/VaryingAcoustic.java:15)
- [ ] `VaryingAcoustic.FACTORY`: it uses Serializer, which isn't translated (it's an interface) (eu/ha3/presencefootsteps/sound/acoustics/VaryingAcoustic.java:24)
- [ ] `VaryingAcoustic.VaryingAcoustic(3)`: its class isn't translated (it implements `Acoustic`) (eu/ha3/presencefootsteps/sound/acoustics/VaryingAcoustic.java:14)
- [ ] `VaryingAcoustic.type(0)`: its class isn't translated (it implements `Acoustic`) (eu/ha3/presencefootsteps/sound/acoustics/VaryingAcoustic.java:42)
- [ ] `VaryingAcoustic.playSound(4)`: its class isn't translated (it implements `Acoustic`) (eu/ha3/presencefootsteps/sound/acoustics/VaryingAcoustic.java:47)
- [ ] `VaryingAcoustic.soundName(0)`: its class isn't translated (it implements `Acoustic`) (eu/ha3/presencefootsteps/sound/acoustics/VaryingAcoustic.java:14)
- [ ] `VaryingAcoustic.volume(0)`: its class isn't translated (it implements `Acoustic`) (eu/ha3/presencefootsteps/sound/acoustics/VaryingAcoustic.java:14)
- [ ] `VaryingAcoustic.pitch(0)`: its class isn't translated (it implements `Acoustic`) (eu/ha3/presencefootsteps/sound/acoustics/VaryingAcoustic.java:14)

### `Locomotion`: its constants carry values

- [ ] `Locomotion.REGISTRY`: it uses Locomotion, which isn't translated (its constants carry values) (eu/ha3/presencefootsteps/sound/generator/Locomotion.java:22)
- [x] `Locomotion.AUTO_TRANSLATION_KEY` (eu/ha3/presencefootsteps/sound/generator/Locomotion.java:24)
- [ ] `Locomotion.Locomotion(0)`: its class isn't translated (its constants carry values) (eu/ha3/presencefootsteps/sound/generator/Locomotion.java:27)
- [ ] `Locomotion.Locomotion(1)`: its class isn't translated (its constants carry values) (eu/ha3/presencefootsteps/sound/generator/Locomotion.java:31)
- [ ] `Locomotion.supplyGenerator(2)`: its class isn't translated (its constants carry values) (eu/ha3/presencefootsteps/sound/generator/Locomotion.java:35)
- [ ] `Locomotion.getOptionName(0)`: its class isn't translated (its constants carry values) (eu/ha3/presencefootsteps/sound/generator/Locomotion.java:39)
- [ ] `Locomotion.getOptionTooltip(0)`: its class isn't translated (its constants carry values) (eu/ha3/presencefootsteps/sound/generator/Locomotion.java:43)
- [ ] `Locomotion.byName(1)`: it uses Locomotion, which isn't translated (its constants carry values) (eu/ha3/presencefootsteps/sound/generator/Locomotion.java:47)
- [ ] `Locomotion.forLiving(2)`: it uses Locomotion, which isn't translated (its constants carry values) (eu/ha3/presencefootsteps/sound/generator/Locomotion.java:51)
- [ ] `Locomotion.forPlayer(2)`: it uses Locomotion, which isn't translated (its constants carry values) (eu/ha3/presencefootsteps/sound/generator/Locomotion.java:55)

### `MineLP`

- [x] `MineLP.checkCompleted` (eu/ha3/presencefootsteps/sound/generator/MineLP.java:10)
- [x] `MineLP.hasMineLP` (eu/ha3/presencefootsteps/sound/generator/MineLP.java:11)
- [x] `MineLP.hasPonies(0)` (eu/ha3/presencefootsteps/sound/generator/MineLP.java:13)
- [ ] `MineLP.getLocomotion(2)`: it uses Locomotion, which isn't translated (its constants carry values) (eu/ha3/presencefootsteps/sound/generator/MineLP.java:22)
- [ ] `MineLP.getLocomotion(1)`: it uses Locomotion, which isn't translated (its constants carry values) (eu/ha3/presencefootsteps/sound/generator/MineLP.java:31)

### `Modifier`: it's generic

- [ ] `Modifier.stepped(3)`: its class isn't translated (it's generic) (eu/ha3/presencefootsteps/sound/generator/Modifier.java:7)
- [ ] `Modifier.reevaluateDistance(2)`: its class isn't translated (it's generic) (eu/ha3/presencefootsteps/sound/generator/Modifier.java:10)

### `MotionTracker`: it uses TerrestrialStepSoundGenerator, which isn't translated (it implements `StepSoundGenerator`)

- [ ] `MotionTracker.MotionTracker(1)`: its class isn't translated (it uses TerrestrialStepSoundGenerator, which isn't translated (it implements `StepSoundGenerator`)) (eu/ha3/presencefootsteps/sound/generator/MotionTracker.java:23)
- [ ] `MotionTracker.getMotionX(0)`: its class isn't translated (it uses TerrestrialStepSoundGenerator, which isn't translated (it implements `StepSoundGenerator`)) (eu/ha3/presencefootsteps/sound/generator/MotionTracker.java:27)
- [ ] `MotionTracker.getMotionY(0)`: its class isn't translated (it uses TerrestrialStepSoundGenerator, which isn't translated (it implements `StepSoundGenerator`)) (eu/ha3/presencefootsteps/sound/generator/MotionTracker.java:31)
- [ ] `MotionTracker.getMotionZ(0)`: its class isn't translated (it uses TerrestrialStepSoundGenerator, which isn't translated (it implements `StepSoundGenerator`)) (eu/ha3/presencefootsteps/sound/generator/MotionTracker.java:35)
- [ ] `MotionTracker.getHorizontalSpeed(0)`: its class isn't translated (it uses TerrestrialStepSoundGenerator, which isn't translated (it implements `StepSoundGenerator`)) (eu/ha3/presencefootsteps/sound/generator/MotionTracker.java:39)
- [ ] `MotionTracker.isStationary(0)`: its class isn't translated (it uses TerrestrialStepSoundGenerator, which isn't translated (it implements `StepSoundGenerator`)) (eu/ha3/presencefootsteps/sound/generator/MotionTracker.java:43)
- [ ] `MotionTracker.getDistanceTraveled(0)`: its class isn't translated (it uses TerrestrialStepSoundGenerator, which isn't translated (it implements `StepSoundGenerator`)) (eu/ha3/presencefootsteps/sound/generator/MotionTracker.java:47)
- [ ] `MotionTracker.getFallDistance(0)`: its class isn't translated (it uses TerrestrialStepSoundGenerator, which isn't translated (it implements `StepSoundGenerator`)) (eu/ha3/presencefootsteps/sound/generator/MotionTracker.java:51)
- [ ] `MotionTracker.simulateMotionData(1)`: its class isn't translated (it uses TerrestrialStepSoundGenerator, which isn't translated (it implements `StepSoundGenerator`)) (eu/ha3/presencefootsteps/sound/generator/MotionTracker.java:55)
- [ ] `MotionTracker.pickState(3)`: its class isn't translated (it uses TerrestrialStepSoundGenerator, which isn't translated (it implements `StepSoundGenerator`)) (eu/ha3/presencefootsteps/sound/generator/MotionTracker.java:88)
- [ ] `MotionTracker.getSpeedScalingRatio(1)`: its class isn't translated (it uses TerrestrialStepSoundGenerator, which isn't translated (it implements `StepSoundGenerator`)) (eu/ha3/presencefootsteps/sound/generator/MotionTracker.java:96)
- [ ] `MotionTracker.accumulateSimulatedDistance(2)`: its class isn't translated (it uses TerrestrialStepSoundGenerator, which isn't translated (it implements `StepSoundGenerator`)) (eu/ha3/presencefootsteps/sound/generator/MotionTracker.java:105)

### `QuadrupedModifier`: it extends `Modifier`

- [ ] `QuadrupedModifier.stepped(3)`: its class isn't translated (it extends `Modifier`) (eu/ha3/presencefootsteps/sound/generator/QuadrupedModifier.java:12)
- [ ] `QuadrupedModifier.reevaluateDistance(2)`: its class isn't translated (it extends `Modifier`) (eu/ha3/presencefootsteps/sound/generator/QuadrupedModifier.java:23)

### `StepSoundGenerator`: it's an interface

- [ ] `StepSoundGenerator.getLocalPitch(1)`: it has no body (eu/ha3/presencefootsteps/sound/generator/StepSoundGenerator.java:4)
- [ ] `StepSoundGenerator.getLocalVolume(1)`: it has no body (eu/ha3/presencefootsteps/sound/generator/StepSoundGenerator.java:6)
- [ ] `StepSoundGenerator.getMotionTracker(0)`: it has no body (eu/ha3/presencefootsteps/sound/generator/StepSoundGenerator.java:8)
- [ ] `StepSoundGenerator.generateFootsteps(0)`: it has no body (eu/ha3/presencefootsteps/sound/generator/StepSoundGenerator.java:10)

### `TerrestrialStepSoundGenerator`: it implements `StepSoundGenerator`

- [ ] `TerrestrialStepSoundGenerator.TerrestrialStepSoundGenerator(3)`: its class isn't translated (it implements `StepSoundGenerator`) (eu/ha3/presencefootsteps/sound/generator/TerrestrialStepSoundGenerator.java:55)
- [ ] `TerrestrialStepSoundGenerator.getLocalPitch(1)`: its class isn't translated (it implements `StepSoundGenerator`) (eu/ha3/presencefootsteps/sound/generator/TerrestrialStepSoundGenerator.java:62)
- [ ] `TerrestrialStepSoundGenerator.getLocalVolume(1)`: its class isn't translated (it implements `StepSoundGenerator`) (eu/ha3/presencefootsteps/sound/generator/TerrestrialStepSoundGenerator.java:67)
- [ ] `TerrestrialStepSoundGenerator.getMotionTracker(0)`: its class isn't translated (it implements `StepSoundGenerator`) (eu/ha3/presencefootsteps/sound/generator/TerrestrialStepSoundGenerator.java:72)
- [ ] `TerrestrialStepSoundGenerator.generateFootsteps(0)`: its class isn't translated (it implements `StepSoundGenerator`) (eu/ha3/presencefootsteps/sound/generator/TerrestrialStepSoundGenerator.java:77)
- [ ] `TerrestrialStepSoundGenerator.simulateStationary(0)`: its class isn't translated (it implements `StepSoundGenerator`) (eu/ha3/presencefootsteps/sound/generator/TerrestrialStepSoundGenerator.java:96)
- [ ] `TerrestrialStepSoundGenerator.playbackImmobile(0)`: its class isn't translated (it implements `StepSoundGenerator`) (eu/ha3/presencefootsteps/sound/generator/TerrestrialStepSoundGenerator.java:105)
- [ ] `TerrestrialStepSoundGenerator.updateImmobileState(1)`: its class isn't translated (it implements `StepSoundGenerator`) (eu/ha3/presencefootsteps/sound/generator/TerrestrialStepSoundGenerator.java:119)
- [ ] `TerrestrialStepSoundGenerator.simulateFootsteps(0)`: its class isn't translated (it implements `StepSoundGenerator`) (eu/ha3/presencefootsteps/sound/generator/TerrestrialStepSoundGenerator.java:133)
- [ ] `TerrestrialStepSoundGenerator.produceStep(1)`: its class isn't translated (it implements `StepSoundGenerator`) (eu/ha3/presencefootsteps/sound/generator/TerrestrialStepSoundGenerator.java:205)
- [ ] `TerrestrialStepSoundGenerator.produceStep(2)`: its class isn't translated (it implements `StepSoundGenerator`) (eu/ha3/presencefootsteps/sound/generator/TerrestrialStepSoundGenerator.java:209)
- [ ] `TerrestrialStepSoundGenerator.hasStoppingConditions(0)`: its class isn't translated (it implements `StepSoundGenerator`) (eu/ha3/presencefootsteps/sound/generator/TerrestrialStepSoundGenerator.java:237)
- [ ] `TerrestrialStepSoundGenerator.simulateAirborne(0)`: its class isn't translated (it implements `StepSoundGenerator`) (eu/ha3/presencefootsteps/sound/generator/TerrestrialStepSoundGenerator.java:241)
- [ ] `TerrestrialStepSoundGenerator.isJumping(0)`: its class isn't translated (it implements `StepSoundGenerator`) (eu/ha3/presencefootsteps/sound/generator/TerrestrialStepSoundGenerator.java:248)
- [ ] `TerrestrialStepSoundGenerator.getOffsetMinus(0)`: its class isn't translated (it implements `StepSoundGenerator`) (eu/ha3/presencefootsteps/sound/generator/TerrestrialStepSoundGenerator.java:252)
- [ ] `TerrestrialStepSoundGenerator.simulateJumpingLanding(0)`: its class isn't translated (it implements `StepSoundGenerator`) (eu/ha3/presencefootsteps/sound/generator/TerrestrialStepSoundGenerator.java:256)
- [ ] `TerrestrialStepSoundGenerator.simulateJumping(0)`: its class isn't translated (it implements `StepSoundGenerator`) (eu/ha3/presencefootsteps/sound/generator/TerrestrialStepSoundGenerator.java:266)
- [ ] `TerrestrialStepSoundGenerator.simulateLanding(0)`: its class isn't translated (it implements `StepSoundGenerator`) (eu/ha3/presencefootsteps/sound/generator/TerrestrialStepSoundGenerator.java:277)
- [ ] `TerrestrialStepSoundGenerator.simulateBrushes(0)`: its class isn't translated (it implements `StepSoundGenerator`) (eu/ha3/presencefootsteps/sound/generator/TerrestrialStepSoundGenerator.java:291)
- [ ] `TerrestrialStepSoundGenerator.playStep(2)`: its class isn't translated (it implements `StepSoundGenerator`) (eu/ha3/presencefootsteps/sound/generator/TerrestrialStepSoundGenerator.java:316)
- [ ] `TerrestrialStepSoundGenerator.playSinglefoot(3)`: its class isn't translated (it implements `StepSoundGenerator`) (eu/ha3/presencefootsteps/sound/generator/TerrestrialStepSoundGenerator.java:332)
- [ ] `TerrestrialStepSoundGenerator.playMultifoot(2)`: its class isn't translated (it implements `StepSoundGenerator`) (eu/ha3/presencefootsteps/sound/generator/TerrestrialStepSoundGenerator.java:341)

### `WingedStepSoundGenerator`: it extends `TerrestrialStepSoundGenerator`

- [ ] `WingedStepSoundGenerator.SWIFT`: it calls SoundsKey.of, which isn't translated (eu/ha3/presencefootsteps/sound/generator/WingedStepSoundGenerator.java:13)
- [ ] `WingedStepSoundGenerator.WING`: it calls SoundsKey.of, which isn't translated (eu/ha3/presencefootsteps/sound/generator/WingedStepSoundGenerator.java:14)
- [ ] `WingedStepSoundGenerator.WingedStepSoundGenerator(3)`: its class isn't translated (it extends `TerrestrialStepSoundGenerator`) (eu/ha3/presencefootsteps/sound/generator/WingedStepSoundGenerator.java:21)
- [ ] `WingedStepSoundGenerator.generateFootsteps(0)`: its class isn't translated (it extends `TerrestrialStepSoundGenerator`) (eu/ha3/presencefootsteps/sound/generator/WingedStepSoundGenerator.java:25)
- [ ] `WingedStepSoundGenerator.simulateAirborne(0)`: its class isn't translated (it extends `TerrestrialStepSoundGenerator`) (eu/ha3/presencefootsteps/sound/generator/WingedStepSoundGenerator.java:31)
- [ ] `WingedStepSoundGenerator.updateImmobileState(1)`: its class isn't translated (it extends `TerrestrialStepSoundGenerator`) (eu/ha3/presencefootsteps/sound/generator/WingedStepSoundGenerator.java:40)
- [ ] `WingedStepSoundGenerator.getWingSpeed(0)`: its class isn't translated (it extends `TerrestrialStepSoundGenerator`) (eu/ha3/presencefootsteps/sound/generator/WingedStepSoundGenerator.java:60)
- [ ] `WingedStepSoundGenerator.simulateJumpingLanding(0)`: its class isn't translated (it extends `TerrestrialStepSoundGenerator`) (eu/ha3/presencefootsteps/sound/generator/WingedStepSoundGenerator.java:72)
- [ ] `WingedStepSoundGenerator.simulateFlying(0)`: its class isn't translated (it extends `TerrestrialStepSoundGenerator`) (eu/ha3/presencefootsteps/sound/generator/WingedStepSoundGenerator.java:103)
- [ ] `WingedStepSoundGenerator.updateState(3)`: its class isn't translated (it extends `TerrestrialStepSoundGenerator`) (eu/ha3/presencefootsteps/sound/generator/WingedStepSoundGenerator.java:125)

### `WingedStepSoundGenerator.FlightState`


### `Isolator`: it implements `Reportable`

- [x] `Isolator.BIOME_MAP` (eu/ha3/presencefootsteps/sound/Isolator.java:44)
- [x] `Isolator.GOLEM_MAP` (eu/ha3/presencefootsteps/sound/Isolator.java:45)
- [x] `Isolator.LOCOMOTION_MAP` (eu/ha3/presencefootsteps/sound/Isolator.java:46)
- [x] `Isolator.PRIMITIVE_MAP` (eu/ha3/presencefootsteps/sound/Isolator.java:47)
- [x] `Isolator.ACOUSTICS` (eu/ha3/presencefootsteps/sound/Isolator.java:48)
- [x] `Isolator.VARIATOR` (eu/ha3/presencefootsteps/sound/Isolator.java:49)
- [ ] `Isolator.Isolator(9)`: its class isn't translated (it implements `Reportable`) (eu/ha3/presencefootsteps/sound/Isolator.java:33)
- [ ] `Isolator.Isolator(1)`: its class isn't translated (it implements `Reportable`) (eu/ha3/presencefootsteps/sound/Isolator.java:51)
- [ ] `Isolator.blocks(1)`: its class isn't translated (it implements `Reportable`) (eu/ha3/presencefootsteps/sound/Isolator.java:65)
- [ ] `Isolator.load(1)`: its class isn't translated (it implements `Reportable`) (eu/ha3/presencefootsteps/sound/Isolator.java:69)
- [ ] `Isolator.writeToReport(3)`: its class isn't translated (it implements `Reportable`) (eu/ha3/presencefootsteps/sound/Isolator.java:91)
- [ ] `Isolator.variator(0)`: its class isn't translated (it implements `Reportable`) (eu/ha3/presencefootsteps/sound/Isolator.java:33)
- [ ] `Isolator.locomotions(0)`: its class isn't translated (it implements `Reportable`) (eu/ha3/presencefootsteps/sound/Isolator.java:33)
- [ ] `Isolator.heuristics(0)`: its class isn't translated (it implements `Reportable`) (eu/ha3/presencefootsteps/sound/Isolator.java:33)
- [ ] `Isolator.golems(0)`: its class isn't translated (it implements `Reportable`) (eu/ha3/presencefootsteps/sound/Isolator.java:33)
- [ ] `Isolator.blocks(0)`: its class isn't translated (it implements `Reportable`) (eu/ha3/presencefootsteps/sound/Isolator.java:33)
- [ ] `Isolator.biomes(0)`: its class isn't translated (it implements `Reportable`) (eu/ha3/presencefootsteps/sound/Isolator.java:33)
- [ ] `Isolator.primitives(0)`: its class isn't translated (it implements `Reportable`) (eu/ha3/presencefootsteps/sound/Isolator.java:33)
- [ ] `Isolator.acoustics(0)`: its class isn't translated (it implements `Reportable`) (eu/ha3/presencefootsteps/sound/Isolator.java:33)
- [ ] `Isolator.empty(0)`: its class isn't translated (it implements `Reportable`) (eu/ha3/presencefootsteps/sound/Isolator.java:33)

### `Options`: it's an interface

- [ ] `Options.singular(2)`: it uses Options, which isn't translated (it's an interface) (eu/ha3/presencefootsteps/sound/Options.java:20)
- [ ] `Options.ofGetter(2)`: its body couldn't be read: an anonymous class (eu/ha3/presencefootsteps/sound/Options.java:24)
- [ ] `Options.containsKey(1)`: it has no body (eu/ha3/presencefootsteps/sound/Options.java:38)
- [ ] `Options.get(1)`: it has no body (eu/ha3/presencefootsteps/sound/Options.java:40)
- [ ] `Options.getOrDefault(2)`: its class isn't translated (it's an interface) (eu/ha3/presencefootsteps/sound/Options.java:42)
- [ ] `Options.and(1)`: its body couldn't be read: an anonymous class (eu/ha3/presencefootsteps/sound/Options.java:46)

### `Options.FloatSupplier`: it's an interface

- [ ] `Options.FloatSupplier.get(0)`: it has no body (eu/ha3/presencefootsteps/sound/Options.java:62)

### `DelayedSoundPlayer`: it implements `SoundPlayer`

- [x] `DelayedSoundPlayer.USING_LATENESS` (eu/ha3/presencefootsteps/sound/player/DelayedSoundPlayer.java:13)
- [x] `DelayedSoundPlayer.USING_EARLYNESS` (eu/ha3/presencefootsteps/sound/player/DelayedSoundPlayer.java:14)
- [x] `DelayedSoundPlayer.LATENESS_THRESHOLD` (eu/ha3/presencefootsteps/sound/player/DelayedSoundPlayer.java:15)
- [x] `DelayedSoundPlayer.EARLYNESS_THRESHOLD_POW` (eu/ha3/presencefootsteps/sound/player/DelayedSoundPlayer.java:16)
- [ ] `DelayedSoundPlayer.DelayedSoundPlayer(1)`: its class isn't translated (it implements `SoundPlayer`) (eu/ha3/presencefootsteps/sound/player/DelayedSoundPlayer.java:22)
- [ ] `DelayedSoundPlayer.getRNG(0)`: its class isn't translated (it implements `SoundPlayer`) (eu/ha3/presencefootsteps/sound/player/DelayedSoundPlayer.java:26)
- [ ] `DelayedSoundPlayer.playSound(5)`: its class isn't translated (it implements `SoundPlayer`) (eu/ha3/presencefootsteps/sound/player/DelayedSoundPlayer.java:31)
- [ ] `DelayedSoundPlayer.think(0)`: its class isn't translated (it implements `SoundPlayer`) (eu/ha3/presencefootsteps/sound/player/DelayedSoundPlayer.java:40)

### `DelayedSoundPlayer.PendingSound`: it uses `WeakReference` (java.lang.ref)

- [ ] `DelayedSoundPlayer.PendingSound.PendingSound(5)`: its class isn't translated (it uses `WeakReference` (java.lang.ref)) (eu/ha3/presencefootsteps/sound/player/DelayedSoundPlayer.java:58)
- [ ] `DelayedSoundPlayer.PendingSound.tick(0)`: its class isn't translated (it uses `WeakReference` (java.lang.ref)) (eu/ha3/presencefootsteps/sound/player/DelayedSoundPlayer.java:70)
- [ ] `DelayedSoundPlayer.PendingSound.nextState(1)`: its class isn't translated (it uses `WeakReference` (java.lang.ref)) (eu/ha3/presencefootsteps/sound/player/DelayedSoundPlayer.java:85)

### `DelayedSoundPlayer.State`


### `ImmediateSoundPlayer`: it implements `SoundPlayer`

- [ ] `ImmediateSoundPlayer.ImmediateSoundPlayer(1)`: its class isn't translated (it implements `SoundPlayer`) (eu/ha3/presencefootsteps/sound/player/ImmediateSoundPlayer.java:21)
- [ ] `ImmediateSoundPlayer.getRNG(0)`: its class isn't translated (it implements `SoundPlayer`) (eu/ha3/presencefootsteps/sound/player/ImmediateSoundPlayer.java:25)
- [ ] `ImmediateSoundPlayer.playSound(5)`: its class isn't translated (it implements `SoundPlayer`) (eu/ha3/presencefootsteps/sound/player/ImmediateSoundPlayer.java:30)

### `ImmediateSoundPlayer.UncappedSoundInstance`: it extends Minecraft's `SimpleSoundInstance`

- [ ] `ImmediateSoundPlayer.UncappedSoundInstance.UncappedSoundInstance(4)`: its class isn't translated (it extends Minecraft's `SimpleSoundInstance`) (eu/ha3/presencefootsteps/sound/player/ImmediateSoundPlayer.java:54)
- [ ] `ImmediateSoundPlayer.UncappedSoundInstance.getMaxVolume(0)`: its class isn't translated (it extends Minecraft's `SimpleSoundInstance`) (eu/ha3/presencefootsteps/sound/player/ImmediateSoundPlayer.java:71)
- [ ] `ImmediateSoundPlayer.UncappedSoundInstance.getSoundId(2)`: it uses Minecraft's `Entity` (eu/ha3/presencefootsteps/sound/player/ImmediateSoundPlayer.java:75)

### `SoundPlayer`: it's an interface

- [ ] `SoundPlayer.playSound(5)`: it has no body (eu/ha3/presencefootsteps/sound/player/SoundPlayer.java:8)
- [ ] `SoundPlayer.getRNG(0)`: it has no body (eu/ha3/presencefootsteps/sound/player/SoundPlayer.java:10)
- [ ] `SoundPlayer.think(0)`: its class isn't translated (it's an interface) (eu/ha3/presencefootsteps/sound/player/SoundPlayer.java:12)

### `SoundEngine`: it implements Minecraft's `PreparableReloadListener`

- [x] `SoundEngine.ID` (eu/ha3/presencefootsteps/sound/SoundEngine.java:45)
- [x] `SoundEngine.DEBUG_VISUALISER_ID` (eu/ha3/presencefootsteps/sound/SoundEngine.java:46)
- [ ] `SoundEngine.SoundEngine(1)`: its class isn't translated (it implements Minecraft's `PreparableReloadListener`) (eu/ha3/presencefootsteps/sound/SoundEngine.java:52)
- [ ] `SoundEngine.getVolumeForSource(1)`: its class isn't translated (it implements Minecraft's `PreparableReloadListener`) (eu/ha3/presencefootsteps/sound/SoundEngine.java:56)
- [ ] `SoundEngine.getIsolator(0)`: its class isn't translated (it implements Minecraft's `PreparableReloadListener`) (eu/ha3/presencefootsteps/sound/SoundEngine.java:77)
- [ ] `SoundEngine.getSolver(0)`: its class isn't translated (it implements Minecraft's `PreparableReloadListener`) (eu/ha3/presencefootsteps/sound/SoundEngine.java:81)
- [ ] `SoundEngine.getConfig(0)`: its class isn't translated (it implements Minecraft's `PreparableReloadListener`) (eu/ha3/presencefootsteps/sound/SoundEngine.java:85)
- [ ] `SoundEngine.reload(0)`: its class isn't translated (it implements Minecraft's `PreparableReloadListener`) (eu/ha3/presencefootsteps/sound/SoundEngine.java:89)
- [ ] `SoundEngine.isEnabledFor(1)`: its class isn't translated (it implements Minecraft's `PreparableReloadListener`) (eu/ha3/presencefootsteps/sound/SoundEngine.java:97)
- [ ] `SoundEngine.hasData(0)`: its class isn't translated (it implements Minecraft's `PreparableReloadListener`) (eu/ha3/presencefootsteps/sound/SoundEngine.java:103)
- [ ] `SoundEngine.isRunning(1)`: its class isn't translated (it implements Minecraft's `PreparableReloadListener`) (eu/ha3/presencefootsteps/sound/SoundEngine.java:107)
- [ ] `SoundEngine.isActive(1)`: its class isn't translated (it implements Minecraft's `PreparableReloadListener`) (eu/ha3/presencefootsteps/sound/SoundEngine.java:111)
- [ ] `SoundEngine.getTargets(1)`: its class isn't translated (it implements Minecraft's `PreparableReloadListener`) (eu/ha3/presencefootsteps/sound/SoundEngine.java:115)
- [ ] `SoundEngine.onFrame(2)`: its class isn't translated (it implements Minecraft's `PreparableReloadListener`) (eu/ha3/presencefootsteps/sound/SoundEngine.java:149)
- [ ] `SoundEngine.onSoundRecieved(1)`: its class isn't translated (it implements Minecraft's `PreparableReloadListener`) (eu/ha3/presencefootsteps/sound/SoundEngine.java:173)
- [ ] `SoundEngine.reload(4)`: its class isn't translated (it implements Minecraft's `PreparableReloadListener`) (eu/ha3/presencefootsteps/sound/SoundEngine.java:190)
- [ ] `SoundEngine.reloadEverything(1)`: its class isn't translated (it implements Minecraft's `PreparableReloadListener`) (eu/ha3/presencefootsteps/sound/SoundEngine.java:199)
- [ ] `SoundEngine.shutdown(0)`: its class isn't translated (it implements Minecraft's `PreparableReloadListener`) (eu/ha3/presencefootsteps/sound/SoundEngine.java:203)

### `SoundSourceUtil`

- [ ] `SoundSourceUtil.PLAYER_STEP_SOUNDS`: it uses Minecraft's `SoundEvents`.PLAYER_SWIM (eu/ha3/presencefootsteps/sound/SoundSourceUtil.java:14)
- [x] `SoundSourceUtil.isHorseGallopingSound(1)` (eu/ha3/presencefootsteps/sound/SoundSourceUtil.java:22)
- [x] `SoundSourceUtil.isStepSound(1)` (eu/ha3/presencefootsteps/sound/SoundSourceUtil.java:26)
- [ ] `SoundSourceUtil.isPlayerStepSound(3)`: it uses Minecraft's `SoundSource` (eu/ha3/presencefootsteps/sound/SoundSourceUtil.java:30)
- [ ] `SoundSourceUtil.getStepSoundAtPosition(1)`: it uses Minecraft's `SoundEvent` (eu/ha3/presencefootsteps/sound/SoundSourceUtil.java:34)

### `State`: its constants carry values

- [ ] `State.CODEC`: it uses Minecraft's `EnumCodec` (eu/ha3/presencefootsteps/sound/State.java:22)
- [ ] `State.State(1)`: its class isn't translated (its constants carry values) (eu/ha3/presencefootsteps/sound/State.java:26)
- [ ] `State.getSerializedName(0)`: its class isn't translated (its constants carry values) (eu/ha3/presencefootsteps/sound/State.java:31)
- [ ] `State.getName(0)`: its class isn't translated (its constants carry values) (eu/ha3/presencefootsteps/sound/State.java:35)
- [ ] `State.canTransition(0)`: its class isn't translated (its constants carry values) (eu/ha3/presencefootsteps/sound/State.java:39)
- [ ] `State.getTransitionDestination(0)`: its class isn't translated (its constants carry values) (eu/ha3/presencefootsteps/sound/State.java:43)
- [ ] `State.isExtraLoud(0)`: its class isn't translated (its constants carry values) (eu/ha3/presencefootsteps/sound/State.java:47)

### `StepSoundSource`: it's an interface

- [ ] `StepSoundSource.getStepGenerator(1)`: it has no body (eu/ha3/presencefootsteps/sound/StepSoundSource.java:10)
- [ ] `StepSoundSource.isStepBlocked(0)`: it has no body (eu/ha3/presencefootsteps/sound/StepSoundSource.java:12)

### `StepSoundSource.Container`: it implements `StepSoundSource`

- [ ] `StepSoundSource.Container.Container(1)`: its class isn't translated (it implements `StepSoundSource`) (eu/ha3/presencefootsteps/sound/StepSoundSource.java:19)
- [ ] `StepSoundSource.Container.getStepGenerator(1)`: its class isn't translated (it implements `StepSoundSource`) (eu/ha3/presencefootsteps/sound/StepSoundSource.java:23)
- [ ] `StepSoundSource.Container.isStepBlocked(0)`: its class isn't translated (it implements `StepSoundSource`) (eu/ha3/presencefootsteps/sound/StepSoundSource.java:34)

### `BlockReport`: it's an interface

- [ ] `BlockReport.execute(3)`: it uses `Object` (eu/ha3/presencefootsteps/util/BlockReport.java:20)
- [ ] `BlockReport.execute(3)`: it uses `Object` (eu/ha3/presencefootsteps/util/BlockReport.java:28)
- [ ] `BlockReport.getUniqueFileName(3)`: `null` (Rust has no null: this needs an Option) (eu/ha3/presencefootsteps/util/BlockReport.java:58)

### `BlockReport.Reportable`: it's an interface

- [ ] `BlockReport.Reportable.writeToReport(3)`: it has no body (eu/ha3/presencefootsteps/util/BlockReport.java:70)

### `BlockReport.UnsafeConsumer`: it's an interface

- [ ] `BlockReport.UnsafeConsumer.accept(1)`: it has no body (eu/ha3/presencefootsteps/util/BlockReport.java:74)

### `Edge`: it implements `BooleanConsumer` (it.unimi.dsi.fastutil.booleans)

- [ ] `Edge.Edge(1)`: its class isn't translated (it implements `BooleanConsumer` (it.unimi.dsi.fastutil.booleans)) (eu/ha3/presencefootsteps/util/Edge.java:9)
- [ ] `Edge.accept(1)`: its class isn't translated (it implements `BooleanConsumer` (it.unimi.dsi.fastutil.booleans)) (eu/ha3/presencefootsteps/util/Edge.java:13)

### `JsonObjectWriter`: it's an interface

- [ ] `JsonObjectWriter.of(1)`: it uses JsonObjectWriter, which isn't translated (it's an interface) (eu/ha3/presencefootsteps/util/JsonObjectWriter.java:7)
- [ ] `JsonObjectWriter.writer(0)`: it has no body (eu/ha3/presencefootsteps/util/JsonObjectWriter.java:12)
- [ ] `JsonObjectWriter.field(2)`: its class isn't translated (it's an interface) (eu/ha3/presencefootsteps/util/JsonObjectWriter.java:14)
- [ ] `JsonObjectWriter.field(2)`: its class isn't translated (it's an interface) (eu/ha3/presencefootsteps/util/JsonObjectWriter.java:19)
- [ ] `JsonObjectWriter.field(2)`: its class isn't translated (it's an interface) (eu/ha3/presencefootsteps/util/JsonObjectWriter.java:24)
- [ ] `JsonObjectWriter.field(2)`: its class isn't translated (it's an interface) (eu/ha3/presencefootsteps/util/JsonObjectWriter.java:29)
- [ ] `JsonObjectWriter.field(2)`: its class isn't translated (it's an interface) (eu/ha3/presencefootsteps/util/JsonObjectWriter.java:34)
- [ ] `JsonObjectWriter.object(1)`: its class isn't translated (it's an interface) (eu/ha3/presencefootsteps/util/JsonObjectWriter.java:39)
- [ ] `JsonObjectWriter.object(2)`: its class isn't translated (it's an interface) (eu/ha3/presencefootsteps/util/JsonObjectWriter.java:45)
- [ ] `JsonObjectWriter.array(2)`: its class isn't translated (it's an interface) (eu/ha3/presencefootsteps/util/JsonObjectWriter.java:50)
- [ ] `JsonObjectWriter.array(1)`: its class isn't translated (it's an interface) (eu/ha3/presencefootsteps/util/JsonObjectWriter.java:55)
- [ ] `JsonObjectWriter.each(2)`: it's generic (eu/ha3/presencefootsteps/util/JsonObjectWriter.java:61)
- [ ] `JsonObjectWriter.close(0)`: its class isn't translated (it's an interface) (eu/ha3/presencefootsteps/util/JsonObjectWriter.java:67)

### `JsonObjectWriter.WriteAction`: it's an interface

- [ ] `JsonObjectWriter.WriteAction.write(0)`: it has no body (eu/ha3/presencefootsteps/util/JsonObjectWriter.java:73)

### `JsonObjectWriter.WriteConsumer`: it's an interface

- [ ] `JsonObjectWriter.WriteConsumer.write(1)`: it has no body (eu/ha3/presencefootsteps/util/JsonObjectWriter.java:77)

### `Lerp`

- [x] `Lerp.update(2)` (eu/ha3/presencefootsteps/util/Lerp.java:9)
- [x] `Lerp.get(1)` (eu/ha3/presencefootsteps/util/Lerp.java:20)

### `MathUtil`: it's an interface

- [ ] `MathUtil.randAB(3)`: it's overloaded with the same number of parameters (eu/ha3/presencefootsteps/util/MathUtil.java:7)
- [ ] `MathUtil.randAB(3)`: it's overloaded with the same number of parameters (eu/ha3/presencefootsteps/util/MathUtil.java:11)
- [x] `MathUtil.scalex(3)` (eu/ha3/presencefootsteps/util/MathUtil.java:15)

### `Period`: it implements `Options` (eu.ha3.presencefootsteps.sound)

- [ ] `Period.RANGE_CODEC`: it uses Minecraft's `Codec` (eu/ha3/presencefootsteps/util/Period.java:13)
- [ ] `Period.POINT_CODEC`: it uses Minecraft's `Codec` (eu/ha3/presencefootsteps/util/Period.java:16)
- [ ] `Period.CODEC`: it uses Minecraft's `Codec` (eu/ha3/presencefootsteps/util/Period.java:17)
- [ ] `Period.ZERO`: it uses Period, which isn't translated (it implements `Options` (eu.ha3.presencefootsteps.sound)) (eu/ha3/presencefootsteps/util/Period.java:19)
- [ ] `Period.Period(2)`: its class isn't translated (it implements `Options` (eu.ha3.presencefootsteps.sound)) (eu/ha3/presencefootsteps/util/Period.java:12)
- [ ] `Period.of(1)`: it uses Period, which isn't translated (it implements `Options` (eu.ha3.presencefootsteps.sound)) (eu/ha3/presencefootsteps/util/Period.java:21)
- [ ] `Period.of(2)`: it uses Period, which isn't translated (it implements `Options` (eu.ha3.presencefootsteps.sound)) (eu/ha3/presencefootsteps/util/Period.java:25)
- [ ] `Period.fromJson(2)`: it uses Period, which isn't translated (it implements `Options` (eu.ha3.presencefootsteps.sound)) (eu/ha3/presencefootsteps/util/Period.java:29)
- [ ] `Period.random(1)`: its class isn't translated (it implements `Options` (eu.ha3.presencefootsteps.sound)) (eu/ha3/presencefootsteps/util/Period.java:34)
- [ ] `Period.on(1)`: its class isn't translated (it implements `Options` (eu.ha3.presencefootsteps.sound)) (eu/ha3/presencefootsteps/util/Period.java:38)
- [ ] `Period.containsKey(1)`: its class isn't translated (it implements `Options` (eu.ha3.presencefootsteps.sound)) (eu/ha3/presencefootsteps/util/Period.java:42)
- [ ] `Period.get(1)`: its class isn't translated (it implements `Options` (eu.ha3.presencefootsteps.sound)) (eu/ha3/presencefootsteps/util/Period.java:47)
- [ ] `Period.min(0)`: its class isn't translated (it implements `Options` (eu.ha3.presencefootsteps.sound)) (eu/ha3/presencefootsteps/util/Period.java:12)
- [ ] `Period.max(0)`: its class isn't translated (it implements `Options` (eu.ha3.presencefootsteps.sound)) (eu/ha3/presencefootsteps/util/Period.java:12)

### `PlayerUtil`: it's an interface

- [ ] `PlayerUtil.isClientPlayer(1)`: it uses Minecraft's `Entity` (eu/ha3/presencefootsteps/util/PlayerUtil.java:11)
- [ ] `PlayerUtil.getScale(1)`: it uses Minecraft's `LivingEntity` (eu/ha3/presencefootsteps/util/PlayerUtil.java:16)

### `Range`

- [ ] `Range.PERCENTAGE_CODEC`: it uses Minecraft's `Codec` (eu/ha3/presencefootsteps/util/Range.java:12)
- [ ] `Range.RANGE_CODEC`: it uses Minecraft's `Codec` (eu/ha3/presencefootsteps/util/Range.java:13)
- [ ] `Range.POINT_CODEC`: it uses Minecraft's `Codec` (eu/ha3/presencefootsteps/util/Range.java:16)
- [ ] `Range.CODEC`: it uses Minecraft's `Codec` (eu/ha3/presencefootsteps/util/Range.java:17)
- [x] `Range.DEFAULT` (eu/ha3/presencefootsteps/util/Range.java:19)
- [x] `Range.Range(2)` (eu/ha3/presencefootsteps/util/Range.java:11)
- [x] `Range.exactly(1)` (eu/ha3/presencefootsteps/util/Range.java:21)
- [x] `Range.read(2)` (eu/ha3/presencefootsteps/util/Range.java:25)
- [ ] `Range.random(1)`: it calls MathUtil.randAB, which isn't translated (eu/ha3/presencefootsteps/util/Range.java:41)
- [x] `Range.on(1)` (eu/ha3/presencefootsteps/util/Range.java:45)
- [x] `Range.getPercentage(3)` (eu/ha3/presencefootsteps/util/Range.java:49)
- [x] `Range.min(0)` (eu/ha3/presencefootsteps/util/Range.java:11)
- [x] `Range.max(0)` (eu/ha3/presencefootsteps/util/Range.java:11)

### `ResourceUtils`: it's an interface

- [ ] `ResourceUtils.forEach(3)`: it uses Minecraft's `ResourceManager` (eu/ha3/presencefootsteps/util/ResourceUtils.java:28)
- [ ] `ResourceUtils.load(3)`: it's generic (eu/ha3/presencefootsteps/util/ResourceUtils.java:40)
- [ ] `ResourceUtils.load(3)`: it's generic (eu/ha3/presencefootsteps/util/ResourceUtils.java:44)
- [ ] `ResourceUtils.loadAll(3)`: it's generic (eu/ha3/presencefootsteps/util/ResourceUtils.java:55)
- [ ] `ResourceUtils.loadDir(5)`: it's generic (eu/ha3/presencefootsteps/util/ResourceUtils.java:75)

### `AbstractSubstrateLookup`: it's generic

- [ ] `AbstractSubstrateLookup.AbstractSubstrateLookup(1)`: its class isn't translated (it's generic) (eu/ha3/presencefootsteps/world/AbstractSubstrateLookup.java:15)
- [ ] `AbstractSubstrateLookup.getId(1)`: it has no body (eu/ha3/presencefootsteps/world/AbstractSubstrateLookup.java:29)
- [ ] `AbstractSubstrateLookup.getAssociation(2)`: its class isn't translated (it's generic) (eu/ha3/presencefootsteps/world/AbstractSubstrateLookup.java:31)
- [ ] `AbstractSubstrateLookup.getSubstrateMap(2)`: its class isn't translated (it's generic) (eu/ha3/presencefootsteps/world/AbstractSubstrateLookup.java:41)
- [ ] `AbstractSubstrateLookup.getSubstrates(0)`: its class isn't translated (it's generic) (eu/ha3/presencefootsteps/world/AbstractSubstrateLookup.java:52)
- [ ] `AbstractSubstrateLookup.contains(1)`: its class isn't translated (it's generic) (eu/ha3/presencefootsteps/world/AbstractSubstrateLookup.java:57)
- [ ] `AbstractSubstrateLookup.contains(2)`: its class isn't translated (it's generic) (eu/ha3/presencefootsteps/world/AbstractSubstrateLookup.java:70)
- [ ] `AbstractSubstrateLookup.isEmpty(0)`: its class isn't translated (it's generic) (eu/ha3/presencefootsteps/world/AbstractSubstrateLookup.java:75)

### `Association`: it uses Minecraft's `BlockState`

- [ ] `Association.NOT_EMITTER`: it uses Association, which isn't translated (it uses Minecraft's `BlockState`) (eu/ha3/presencefootsteps/world/Association.java:11)
- [ ] `Association.Association(7)`: its class isn't translated (it uses Minecraft's `BlockState`) (eu/ha3/presencefootsteps/world/Association.java:10)
- [ ] `Association.of(7)`: it uses Association, which isn't translated (it uses Minecraft's `BlockState`) (eu/ha3/presencefootsteps/world/Association.java:15)
- [ ] `Association.isResult(0)`: its class isn't translated (it uses Minecraft's `BlockState`) (eu/ha3/presencefootsteps/world/Association.java:21)
- [ ] `Association.isSilent(0)`: its class isn't translated (it uses Minecraft's `BlockState`) (eu/ha3/presencefootsteps/world/Association.java:25)
- [ ] `Association.dataEquals(1)`: its class isn't translated (it uses Minecraft's `BlockState`) (eu/ha3/presencefootsteps/world/Association.java:29)
- [ ] `Association.state(0)`: its class isn't translated (it uses Minecraft's `BlockState`) (eu/ha3/presencefootsteps/world/Association.java:10)
- [ ] `Association.pos(0)`: its class isn't translated (it uses Minecraft's `BlockState`) (eu/ha3/presencefootsteps/world/Association.java:10)
- [ ] `Association.source(0)`: its class isn't translated (it uses Minecraft's `BlockState`) (eu/ha3/presencefootsteps/world/Association.java:10)
- [ ] `Association.forcePlay(0)`: its class isn't translated (it uses Minecraft's `BlockState`) (eu/ha3/presencefootsteps/world/Association.java:10)
- [ ] `Association.dry(0)`: its class isn't translated (it uses Minecraft's `BlockState`) (eu/ha3/presencefootsteps/world/Association.java:10)
- [ ] `Association.wet(0)`: its class isn't translated (it uses Minecraft's `BlockState`) (eu/ha3/presencefootsteps/world/Association.java:10)
- [ ] `Association.foliage(0)`: its class isn't translated (it uses Minecraft's `BlockState`) (eu/ha3/presencefootsteps/world/Association.java:10)

### `AssociationPool`: it uses Minecraft's `LivingEntity`

- [ ] `AssociationPool.AssociationPool(2)`: its class isn't translated (it uses Minecraft's `LivingEntity`) (eu/ha3/presencefootsteps/world/AssociationPool.java:19)
- [ ] `AssociationPool.reset(0)`: its class isn't translated (it uses Minecraft's `LivingEntity`) (eu/ha3/presencefootsteps/world/AssociationPool.java:25)
- [ ] `AssociationPool.wasLastMatchGolem(0)`: its class isn't translated (it uses Minecraft's `LivingEntity`) (eu/ha3/presencefootsteps/world/AssociationPool.java:29)
- [ ] `AssociationPool.findAssociation(2)`: its class isn't translated (it uses Minecraft's `LivingEntity`) (eu/ha3/presencefootsteps/world/AssociationPool.java:33)
- [ ] `AssociationPool.findAssociation(2)`: its class isn't translated (it uses Minecraft's `LivingEntity`) (eu/ha3/presencefootsteps/world/AssociationPool.java:37)
- [ ] `AssociationPool.get(3)`: its class isn't translated (it uses Minecraft's `LivingEntity`) (eu/ha3/presencefootsteps/world/AssociationPool.java:41)
- [ ] `AssociationPool.getForState(2)`: its class isn't translated (it uses Minecraft's `LivingEntity`) (eu/ha3/presencefootsteps/world/AssociationPool.java:70)
- [ ] `AssociationPool.getForPrimitive(2)`: its class isn't translated (it uses Minecraft's `LivingEntity`) (eu/ha3/presencefootsteps/world/AssociationPool.java:74)

### `BiomeVarianceLookup`: it implements `Index`

- [ ] `BiomeVarianceLookup.lookup(1)`: its class isn't translated (it implements `Index`) (eu/ha3/presencefootsteps/world/BiomeVarianceLookup.java:17)
- [ ] `BiomeVarianceLookup.contains(1)`: its class isn't translated (it implements `Index`) (eu/ha3/presencefootsteps/world/BiomeVarianceLookup.java:21)
- [ ] `BiomeVarianceLookup.createLoader(0)`: its class isn't translated (it implements `Index`) (eu/ha3/presencefootsteps/world/BiomeVarianceLookup.java:26)
- [ ] `BiomeVarianceLookup.writeToReport(3)`: its class isn't translated (it implements `Index`) (eu/ha3/presencefootsteps/world/BiomeVarianceLookup.java:35)

### `BiomeVarianceLookup.BiomeVariance`

- [x] `BiomeVarianceLookup.BiomeVariance.DEFAULT` (eu/ha3/presencefootsteps/world/BiomeVarianceLookup.java:40)
- [ ] `BiomeVarianceLookup.BiomeVariance.CODEC`: it uses Minecraft's `Codec` (eu/ha3/presencefootsteps/world/BiomeVarianceLookup.java:41)
- [x] `BiomeVarianceLookup.BiomeVariance.BiomeVariance(2)` (eu/ha3/presencefootsteps/world/BiomeVarianceLookup.java:39)
- [x] `BiomeVarianceLookup.BiomeVariance.volume(0)` (eu/ha3/presencefootsteps/world/BiomeVarianceLookup.java:39)
- [x] `BiomeVarianceLookup.BiomeVariance.pitch(0)` (eu/ha3/presencefootsteps/world/BiomeVarianceLookup.java:39)

### `ContextualBlockState`: it uses Minecraft's `EntityType`

- [ ] `ContextualBlockState.CACHE`: it uses Minecraft's `EntityType` (eu/ha3/presencefootsteps/world/ContextualBlockState.java:9)
- [ ] `ContextualBlockState.ContextualBlockState(2)`: its class isn't translated (it uses Minecraft's `EntityType`) (eu/ha3/presencefootsteps/world/ContextualBlockState.java:8)
- [ ] `ContextualBlockState.of(2)`: it uses ContextualBlockState, which isn't translated (it uses Minecraft's `EntityType`) (eu/ha3/presencefootsteps/world/ContextualBlockState.java:13)
- [ ] `ContextualBlockState.type(0)`: its class isn't translated (it uses Minecraft's `EntityType`) (eu/ha3/presencefootsteps/world/ContextualBlockState.java:8)
- [ ] `ContextualBlockState.state(0)`: its class isn't translated (it uses Minecraft's `EntityType`) (eu/ha3/presencefootsteps/world/ContextualBlockState.java:8)

### `EntityBlockLookup`: it uses Lookup, which isn't translated (it's generic)

- [x] `EntityBlockLookup.BLOCK_MAP` (eu/ha3/presencefootsteps/world/EntityBlockLookup.java:16)
- [ ] `EntityBlockLookup.getLookup(1)`: its class isn't translated (it uses Lookup, which isn't translated (it's generic)) (eu/ha3/presencefootsteps/world/EntityBlockLookup.java:20)
- [ ] `EntityBlockLookup.load(1)`: its class isn't translated (it uses Lookup, which isn't translated (it's generic)) (eu/ha3/presencefootsteps/world/EntityBlockLookup.java:24)

### `GolemLookup`: it extends `AbstractSubstrateLookup`

- [ ] `GolemLookup.GolemLookup(1)`: its class isn't translated (it extends `AbstractSubstrateLookup`) (eu/ha3/presencefootsteps/world/GolemLookup.java:14)
- [ ] `GolemLookup.getAssociation(2)`: its class isn't translated (it extends `AbstractSubstrateLookup`) (eu/ha3/presencefootsteps/world/GolemLookup.java:18)
- [ ] `GolemLookup.getId(1)`: its class isn't translated (it extends `AbstractSubstrateLookup`) (eu/ha3/presencefootsteps/world/GolemLookup.java:22)
- [ ] `GolemLookup.writeToReport(4)`: it uses Lookup, which isn't translated (it's generic) (eu/ha3/presencefootsteps/world/GolemLookup.java:26)

### `HeuristicStateLookup`: it uses Minecraft's `Block`

- [ ] `HeuristicStateLookup.getMostSimilar(1)`: its class isn't translated (it uses Minecraft's `Block`) (eu/ha3/presencefootsteps/world/HeuristicStateLookup.java:22)

### `Index`: it's an interface

- [ ] `Index.lookup(1)`: it has no body (eu/ha3/presencefootsteps/world/Index.java:12)
- [ ] `Index.contains(1)`: it has no body (eu/ha3/presencefootsteps/world/Index.java:14)
- [ ] `Index.createLoader(0)`: it has no body (eu/ha3/presencefootsteps/world/Index.java:16)

### `Index.Loader`: it's an interface

- [ ] `Index.Loader.accept(1)`: its class isn't translated (it's an interface) (eu/ha3/presencefootsteps/world/Index.java:21)
- [ ] `Index.Loader.accept(2)`: it has no body (eu/ha3/presencefootsteps/world/Index.java:26)

### `LocomotionLookup`: it implements `Index`

- [ ] `LocomotionLookup.LocomotionLookup(1)`: its class isn't translated (it implements `Index`) (eu/ha3/presencefootsteps/world/LocomotionLookup.java:24)
- [ ] `LocomotionLookup.lookup(1)`: its class isn't translated (it implements `Index`) (eu/ha3/presencefootsteps/world/LocomotionLookup.java:28)
- [ ] `LocomotionLookup.createLoader(0)`: its class isn't translated (it implements `Index`) (eu/ha3/presencefootsteps/world/LocomotionLookup.java:34)
- [ ] `LocomotionLookup.contains(1)`: its class isn't translated (it implements `Index`) (eu/ha3/presencefootsteps/world/LocomotionLookup.java:46)
- [ ] `LocomotionLookup.writeToReport(3)`: its class isn't translated (it implements `Index`) (eu/ha3/presencefootsteps/world/LocomotionLookup.java:51)

### `Lookup`: it's generic

- [ ] `Lookup.load(2)`: its class isn't translated (it's generic) (eu/ha3/presencefootsteps/world/Lookup.java:15)
- [ ] `Lookup.load(1)`: its class isn't translated (it's generic) (eu/ha3/presencefootsteps/world/Lookup.java:19)
- [ ] `Lookup.getAssociation(2)`: its class isn't translated (it's generic) (eu/ha3/presencefootsteps/world/Lookup.java:24)
- [ ] `Lookup.getSubstrates(0)`: its class isn't translated (it's generic) (eu/ha3/presencefootsteps/world/Lookup.java:28)
- [ ] `Lookup.getAssociations(1)`: its class isn't translated (it's generic) (eu/ha3/presencefootsteps/world/Lookup.java:32)
- [ ] `Lookup.contains(1)`: its class isn't translated (it's generic) (eu/ha3/presencefootsteps/world/Lookup.java:45)
- [ ] `Lookup.contains(2)`: its class isn't translated (it's generic) (eu/ha3/presencefootsteps/world/Lookup.java:49)

### `Lookup.DataSegment`: it's an interface

- [ ] `Lookup.DataSegment.getAssociation(2)`: it has no body (eu/ha3/presencefootsteps/world/Lookup.java:54)
- [ ] `Lookup.DataSegment.getSubstrates(0)`: it has no body (eu/ha3/presencefootsteps/world/Lookup.java:56)
- [ ] `Lookup.DataSegment.contains(1)`: it has no body (eu/ha3/presencefootsteps/world/Lookup.java:58)
- [ ] `Lookup.DataSegment.contains(2)`: it has no body (eu/ha3/presencefootsteps/world/Lookup.java:60)
- [ ] `Lookup.DataSegment.isEmpty(0)`: it has no body (eu/ha3/presencefootsteps/world/Lookup.java:62)

### `Lookup.UnionDataSegment`: it's generic

- [ ] `Lookup.UnionDataSegment.EMPTY`: it uses UnionDataSegment, which isn't translated (it's generic) (eu/ha3/presencefootsteps/world/Lookup.java:66)
- [ ] `Lookup.UnionDataSegment.UnionDataSegment(2)`: its class isn't translated (it's generic) (eu/ha3/presencefootsteps/world/Lookup.java:65)
- [ ] `Lookup.UnionDataSegment.empty(0)`: it's generic (eu/ha3/presencefootsteps/world/Lookup.java:68)
- [ ] `Lookup.UnionDataSegment.of(1)`: it's generic (eu/ha3/presencefootsteps/world/Lookup.java:72)
- [ ] `Lookup.UnionDataSegment.getAssociation(2)`: its class isn't translated (it's generic) (eu/ha3/presencefootsteps/world/Lookup.java:86)
- [ ] `Lookup.UnionDataSegment.getSubstrates(0)`: its class isn't translated (it's generic) (eu/ha3/presencefootsteps/world/Lookup.java:98)
- [ ] `Lookup.UnionDataSegment.contains(1)`: its class isn't translated (it's generic) (eu/ha3/presencefootsteps/world/Lookup.java:103)
- [ ] `Lookup.UnionDataSegment.contains(2)`: its class isn't translated (it's generic) (eu/ha3/presencefootsteps/world/Lookup.java:114)
- [ ] `Lookup.UnionDataSegment.isEmpty(0)`: its class isn't translated (it's generic) (eu/ha3/presencefootsteps/world/Lookup.java:125)
- [ ] `Lookup.UnionDataSegment.entries(0)`: its class isn't translated (it's generic) (eu/ha3/presencefootsteps/world/Lookup.java:65)
- [ ] `Lookup.UnionDataSegment.substrates(0)`: its class isn't translated (it's generic) (eu/ha3/presencefootsteps/world/Lookup.java:65)

### `PFSolver`: it implements `Solver`

- [x] `PFSolver.TRAP_DOOR_OFFSET` (eu/ha3/presencefootsteps/world/PFSolver.java:25)
- [ ] `PFSolver.PFSolver(1)`: its class isn't translated (it implements `Solver`) (eu/ha3/presencefootsteps/world/PFSolver.java:30)
- [ ] `PFSolver.getBlockStateAt(2)`: its class isn't translated (it implements `Solver`) (eu/ha3/presencefootsteps/world/PFSolver.java:34)
- [ ] `PFSolver.getCollider(1)`: its class isn't translated (it implements `Solver`) (eu/ha3/presencefootsteps/world/PFSolver.java:44)
- [ ] `PFSolver.checkCollision(4)`: its class isn't translated (it implements `Solver`) (eu/ha3/presencefootsteps/world/PFSolver.java:56)
- [ ] `PFSolver.findAssociation(4)`: its class isn't translated (it implements `Solver`) (eu/ha3/presencefootsteps/world/PFSolver.java:65)
- [ ] `PFSolver.findAssociation(4)`: its class isn't translated (it implements `Solver`) (eu/ha3/presencefootsteps/world/PFSolver.java:80)
- [ ] `PFSolver.findAssociation(5)`: its class isn't translated (it implements `Solver`) (eu/ha3/presencefootsteps/world/PFSolver.java:126)
- [ ] `PFSolver.findAssociation(4)`: its class isn't translated (it implements `Solver`) (eu/ha3/presencefootsteps/world/PFSolver.java:190)

### `PrimitiveLookup`: it extends `AbstractSubstrateLookup`

- [ ] `PrimitiveLookup.PrimitiveLookup(1)`: its class isn't translated (it extends `AbstractSubstrateLookup`) (eu/ha3/presencefootsteps/world/PrimitiveLookup.java:13)
- [ ] `PrimitiveLookup.getId(1)`: its class isn't translated (it extends `AbstractSubstrateLookup`) (eu/ha3/presencefootsteps/world/PrimitiveLookup.java:17)
- [ ] `PrimitiveLookup.writeToReport(4)`: it uses Lookup, which isn't translated (it's generic) (eu/ha3/presencefootsteps/world/PrimitiveLookup.java:21)
- [ ] `PrimitiveLookup.getSubstrate(1)`: it uses Minecraft's `SoundType` (eu/ha3/presencefootsteps/world/PrimitiveLookup.java:30)
- [ ] `PrimitiveLookup.getKey(1)`: it uses Minecraft's `SoundType` (eu/ha3/presencefootsteps/world/PrimitiveLookup.java:34)

### `Solver`: it's an interface

- [ ] `Solver.findAssociation(4)`: it has no body (eu/ha3/presencefootsteps/world/Solver.java:9)
- [ ] `Solver.findAssociation(4)`: it has no body (eu/ha3/presencefootsteps/world/Solver.java:11)

### `SoundsKey`

- [ ] `SoundsKey.INTERNER`: it uses `Interner` (com.google.common.collect) (eu/ha3/presencefootsteps/world/SoundsKey.java:9)
- [ ] `SoundsKey.UNASSIGNED`: it uses SoundsKey.INTERNER, which isn't translated (it uses `Interner` (com.google.common.collect)) (eu/ha3/presencefootsteps/world/SoundsKey.java:10)
- [ ] `SoundsKey.NON_EMITTER`: it uses SoundsKey.INTERNER, which isn't translated (it uses `Interner` (com.google.common.collect)) (eu/ha3/presencefootsteps/world/SoundsKey.java:11)
- [ ] `SoundsKey.MESSY_GROUND`: it uses SoundsKey.INTERNER, which isn't translated (it uses `Interner` (com.google.common.collect)) (eu/ha3/presencefootsteps/world/SoundsKey.java:12)
- [ ] `SoundsKey.VANILLA`: it uses SoundsKey.INTERNER, which isn't translated (it uses `Interner` (com.google.common.collect)) (eu/ha3/presencefootsteps/world/SoundsKey.java:13)
- [ ] `SoundsKey.SWIM_WATER`: it calls SoundsKey.of, which isn't translated (eu/ha3/presencefootsteps/world/SoundsKey.java:14)
- [ ] `SoundsKey.SWIM_LAVA`: it calls SoundsKey.of, which isn't translated (eu/ha3/presencefootsteps/world/SoundsKey.java:15)
- [ ] `SoundsKey.WATERFINE`: it calls SoundsKey.of, which isn't translated (eu/ha3/presencefootsteps/world/SoundsKey.java:16)
- [ ] `SoundsKey.LAVAFINE`: it calls SoundsKey.of, which isn't translated (eu/ha3/presencefootsteps/world/SoundsKey.java:17)
- [x] `SoundsKey.SoundsKey(2)` (eu/ha3/presencefootsteps/world/SoundsKey.java:8)
- [ ] `SoundsKey.SoundsKey(1)`: array .isEmpty() (eu/ha3/presencefootsteps/world/SoundsKey.java:19)
- [ ] `SoundsKey.of(1)`: it uses SoundsKey.MESSY_GROUND, which isn't translated (it uses SoundsKey.INTERNER, which isn't translated (it uses `Interner` (com.google.common.collect))) (eu/ha3/presencefootsteps/world/SoundsKey.java:23)
- [ ] `SoundsKey.isResult(0)`: `this` used as a value (eu/ha3/presencefootsteps/world/SoundsKey.java:35)
- [ ] `SoundsKey.isSilent(0)`: `this` used as a value (eu/ha3/presencefootsteps/world/SoundsKey.java:39)
- [ ] `SoundsKey.isVanilla(0)`: `this` used as a value (eu/ha3/presencefootsteps/world/SoundsKey.java:43)
- [ ] `SoundsKey.isEmitter(0)`: it calls SoundsKey.isSilent, which isn't translated (eu/ha3/presencefootsteps/world/SoundsKey.java:47)
- [x] `SoundsKey.raw(0)` (eu/ha3/presencefootsteps/world/SoundsKey.java:8)
- [x] `SoundsKey.names(0)` (eu/ha3/presencefootsteps/world/SoundsKey.java:8)

### `StateLookup`: it implements `DataSegment`

- [ ] `StateLookup.StateLookup(0)`: its class isn't translated (it implements `DataSegment`) (eu/ha3/presencefootsteps/world/StateLookup.java:33)
- [ ] `StateLookup.StateLookup(1)`: its class isn't translated (it implements `DataSegment`) (eu/ha3/presencefootsteps/world/StateLookup.java:37)
- [ ] `StateLookup.getAssociation(2)`: its class isn't translated (it implements `DataSegment`) (eu/ha3/presencefootsteps/world/StateLookup.java:46)
- [ ] `StateLookup.getSubstrates(0)`: its class isn't translated (it implements `DataSegment`) (eu/ha3/presencefootsteps/world/StateLookup.java:50)
- [ ] `StateLookup.contains(1)`: its class isn't translated (it implements `DataSegment`) (eu/ha3/presencefootsteps/world/StateLookup.java:55)
- [ ] `StateLookup.contains(2)`: its class isn't translated (it implements `DataSegment`) (eu/ha3/presencefootsteps/world/StateLookup.java:65)
- [ ] `StateLookup.isEmpty(0)`: its class isn't translated (it implements `DataSegment`) (eu/ha3/presencefootsteps/world/StateLookup.java:69)
- [ ] `StateLookup.writeToReport(4)`: it uses Lookup, which isn't translated (it's generic) (eu/ha3/presencefootsteps/world/StateLookup.java:74)
- [ ] `StateLookup.getSoundData(1)`: it uses Minecraft's `SoundType` (eu/ha3/presencefootsteps/world/StateLookup.java:114)
- [ ] `StateLookup.getClassData(1)`: it uses Minecraft's `BlockState` (eu/ha3/presencefootsteps/world/StateLookup.java:122)
- [ ] `StateLookup.getTagData(1)`: it uses Minecraft's `BlockState` (eu/ha3/presencefootsteps/world/StateLookup.java:136)
- [ ] `StateLookup.substrates(0)`: its class isn't translated (it implements `DataSegment`) (eu/ha3/presencefootsteps/world/StateLookup.java:32)

### `StateLookup.Bucket`: it's an interface

- [ ] `StateLookup.Bucket.add(1)`: its class isn't translated (it's an interface) (eu/ha3/presencefootsteps/world/StateLookup.java:143)
- [ ] `StateLookup.Bucket.get(1)`: it has no body (eu/ha3/presencefootsteps/world/StateLookup.java:146)
- [ ] `StateLookup.Bucket.contains(1)`: its class isn't translated (it's an interface) (eu/ha3/presencefootsteps/world/StateLookup.java:148)

### `StateLookup.Bucket.Substrate`: it implements `Bucket`

- [ ] `StateLookup.Bucket.Substrate.Substrate(3)`: its class isn't translated (it implements `Bucket`) (eu/ha3/presencefootsteps/world/StateLookup.java:152)
- [ ] `StateLookup.Bucket.Substrate.Substrate(1)`: its class isn't translated (it implements `Bucket`) (eu/ha3/presencefootsteps/world/StateLookup.java:154)
- [ ] `StateLookup.Bucket.Substrate.add(1)`: its class isn't translated (it implements `Bucket`) (eu/ha3/presencefootsteps/world/StateLookup.java:158)
- [ ] `StateLookup.Bucket.Substrate.get(1)`: its class isn't translated (it implements `Bucket`) (eu/ha3/presencefootsteps/world/StateLookup.java:167)
- [ ] `StateLookup.Bucket.Substrate.contains(1)`: its class isn't translated (it implements `Bucket`) (eu/ha3/presencefootsteps/world/StateLookup.java:173)
- [ ] `StateLookup.Bucket.Substrate.getTile(1)`: its class isn't translated (it implements `Bucket`) (eu/ha3/presencefootsteps/world/StateLookup.java:178)
- [ ] `StateLookup.Bucket.Substrate.wildcards(0)`: its class isn't translated (it implements `Bucket`) (eu/ha3/presencefootsteps/world/StateLookup.java:152)
- [ ] `StateLookup.Bucket.Substrate.blocks(0)`: its class isn't translated (it implements `Bucket`) (eu/ha3/presencefootsteps/world/StateLookup.java:152)
- [ ] `StateLookup.Bucket.Substrate.tags(0)`: its class isn't translated (it implements `Bucket`) (eu/ha3/presencefootsteps/world/StateLookup.java:152)

### `StateLookup.Bucket.Tile`: it implements `Bucket`

- [ ] `StateLookup.Bucket.Tile.Tile(2)`: its class isn't translated (it implements `Bucket`) (eu/ha3/presencefootsteps/world/StateLookup.java:191)
- [ ] `StateLookup.Bucket.Tile.Tile(1)`: its class isn't translated (it implements `Bucket`) (eu/ha3/presencefootsteps/world/StateLookup.java:192)
- [ ] `StateLookup.Bucket.Tile.add(1)`: its class isn't translated (it implements `Bucket`) (eu/ha3/presencefootsteps/world/StateLookup.java:196)
- [ ] `StateLookup.Bucket.Tile.get(1)`: its class isn't translated (it implements `Bucket`) (eu/ha3/presencefootsteps/world/StateLookup.java:201)
- [ ] `StateLookup.Bucket.Tile.contains(1)`: its class isn't translated (it implements `Bucket`) (eu/ha3/presencefootsteps/world/StateLookup.java:206)
- [ ] `StateLookup.Bucket.Tile.cache(0)`: its class isn't translated (it implements `Bucket`) (eu/ha3/presencefootsteps/world/StateLookup.java:191)
- [ ] `StateLookup.Bucket.Tile.keys(0)`: its class isn't translated (it implements `Bucket`) (eu/ha3/presencefootsteps/world/StateLookup.java:191)

### `StateLookup.Key`: a Map or Set keyed by a StateLookup.Key.Attribute

- [ ] `StateLookup.Key.NULL`: it uses Key, which isn't translated (a Map or Set keyed by a StateLookup.Key.Attribute) (eu/ha3/presencefootsteps/world/StateLookup.java:222)
- [ ] `StateLookup.Key.Key(7)`: its class isn't translated (a Map or Set keyed by a StateLookup.Key.Attribute) (eu/ha3/presencefootsteps/world/StateLookup.java:213)
- [ ] `StateLookup.Key.of(2)`: it uses Key, which isn't translated (a Map or Set keyed by a StateLookup.Key.Attribute) (eu/ha3/presencefootsteps/world/StateLookup.java:226)
- [ ] `StateLookup.Key.matches(1)`: its class isn't translated (a Map or Set keyed by a StateLookup.Key.Attribute) (eu/ha3/presencefootsteps/world/StateLookup.java:265)
- [ ] `StateLookup.Key.toString(0)`: its class isn't translated (a Map or Set keyed by a StateLookup.Key.Attribute) (eu/ha3/presencefootsteps/world/StateLookup.java:286)
- [ ] `StateLookup.Key.hashCode(0)`: its class isn't translated (a Map or Set keyed by a StateLookup.Key.Attribute) (eu/ha3/presencefootsteps/world/StateLookup.java:298)
- [ ] `StateLookup.Key.equals(1)`: its class isn't translated (a Map or Set keyed by a StateLookup.Key.Attribute) (eu/ha3/presencefootsteps/world/StateLookup.java:303)
- [ ] `StateLookup.Key.equals(1)`: its class isn't translated (a Map or Set keyed by a StateLookup.Key.Attribute) (eu/ha3/presencefootsteps/world/StateLookup.java:308)
- [ ] `StateLookup.Key.identifier(0)`: its class isn't translated (a Map or Set keyed by a StateLookup.Key.Attribute) (eu/ha3/presencefootsteps/world/StateLookup.java:213)
- [ ] `StateLookup.Key.substrate(0)`: its class isn't translated (a Map or Set keyed by a StateLookup.Key.Attribute) (eu/ha3/presencefootsteps/world/StateLookup.java:213)
- [ ] `StateLookup.Key.properties(0)`: its class isn't translated (a Map or Set keyed by a StateLookup.Key.Attribute) (eu/ha3/presencefootsteps/world/StateLookup.java:213)
- [ ] `StateLookup.Key.value(0)`: its class isn't translated (a Map or Set keyed by a StateLookup.Key.Attribute) (eu/ha3/presencefootsteps/world/StateLookup.java:213)
- [ ] `StateLookup.Key.empty(0)`: its class isn't translated (a Map or Set keyed by a StateLookup.Key.Attribute) (eu/ha3/presencefootsteps/world/StateLookup.java:213)
- [ ] `StateLookup.Key.isTag(0)`: its class isn't translated (a Map or Set keyed by a StateLookup.Key.Attribute) (eu/ha3/presencefootsteps/world/StateLookup.java:213)
- [ ] `StateLookup.Key.isWildcard(0)`: its class isn't translated (a Map or Set keyed by a StateLookup.Key.Attribute) (eu/ha3/presencefootsteps/world/StateLookup.java:213)

### `StateLookup.Key.Attribute`

- [x] `StateLookup.Key.Attribute.Attribute(2)` (eu/ha3/presencefootsteps/world/StateLookup.java:317)
- [ ] `StateLookup.Key.Attribute.Attribute(1)`: its constructors are overloaded with the same number of parameters (eu/ha3/presencefootsteps/world/StateLookup.java:318)
- [ ] `StateLookup.Key.Attribute.Attribute(1)`: its constructors are overloaded with the same number of parameters (eu/ha3/presencefootsteps/world/StateLookup.java:322)
- [x] `StateLookup.Key.Attribute.toString(0)` (eu/ha3/presencefootsteps/world/StateLookup.java:326)
- [x] `StateLookup.Key.Attribute.name(0)` (eu/ha3/presencefootsteps/world/StateLookup.java:317)
- [x] `StateLookup.Key.Attribute.value(0)` (eu/ha3/presencefootsteps/world/StateLookup.java:317)

### `StateLookup.KeyList`: it uses Key, which isn't translated (a Map or Set keyed by a StateLookup.Key.Attribute)

- [ ] `StateLookup.KeyList.KeyList(2)`: its class isn't translated (it uses Key, which isn't translated (a Map or Set keyed by a StateLookup.Key.Attribute)) (eu/ha3/presencefootsteps/world/StateLookup.java:333)
- [ ] `StateLookup.KeyList.KeyList(0)`: its class isn't translated (it uses Key, which isn't translated (a Map or Set keyed by a StateLookup.Key.Attribute)) (eu/ha3/presencefootsteps/world/StateLookup.java:334)
- [ ] `StateLookup.KeyList.add(1)`: its class isn't translated (it uses Key, which isn't translated (a Map or Set keyed by a StateLookup.Key.Attribute)) (eu/ha3/presencefootsteps/world/StateLookup.java:338)
- [ ] `StateLookup.KeyList.getSetFor(1)`: its class isn't translated (it uses Key, which isn't translated (a Map or Set keyed by a StateLookup.Key.Attribute)) (eu/ha3/presencefootsteps/world/StateLookup.java:344)
- [ ] `StateLookup.KeyList.findMatch(1)`: its class isn't translated (it uses Key, which isn't translated (a Map or Set keyed by a StateLookup.Key.Attribute)) (eu/ha3/presencefootsteps/world/StateLookup.java:348)
- [ ] `StateLookup.KeyList.priorityKeys(0)`: its class isn't translated (it uses Key, which isn't translated (a Map or Set keyed by a StateLookup.Key.Attribute)) (eu/ha3/presencefootsteps/world/StateLookup.java:333)
- [ ] `StateLookup.KeyList.keys(0)`: its class isn't translated (it uses Key, which isn't translated (a Map or Set keyed by a StateLookup.Key.Attribute)) (eu/ha3/presencefootsteps/world/StateLookup.java:333)

### `Substrates`: it's an interface

- [x] `Substrates.isDefault(1)` (eu/ha3/presencefootsteps/world/Substrates.java:14)
- [ ] `Substrates.isSupplimentary(1)`: an instance field used from a static method (eu/ha3/presencefootsteps/world/Substrates.java:18)
