# Java translated to Rust

Translated 148 of 1013 methods and 256 of 281 static fields to Rust; 1 of 1 Fabric event handlers. It's in `runtime.rs`, under `pub mod java`; code that calls the game is in `game_side.rs`.

Each method is translated whole or not at all. A method that can't be (it uses Minecraft, a lambda, null...) is left out with its reason, and so is every method that calls it.

## Why methods were left out

| Cause | Methods | For example |
|---|---|---|
| Calls or uses other code that isn't translated | 737 | `ChunkyAPIImpl.ChunkyAPIImpl(1)`: its class isn't translated (it implements `ChunkyAPI`) |
| Other Java this translator doesn't handle yet | 110 | `ChunkyAPI.version(0)`: it has no body |
| Uses files (java.nio / java.io) | 10 | `NBT.load(1)`: it uses `InputStream` (java.io) |
| Uses Java collections other than List (Map, Set...) | 3 | `TrimFilters.parseTicks(1)`: it uses `TimeUnit` (java.util.concurrent) |
| Uses Minecraft classes (needs Horizon's game API) | 3 | `WorldSuggestions.suggest(2)`: it uses Minecraft's `Suggestions` |
| Uses null | 1 | `TrimRegion.isValid(1)`: `null` (Rust has no null: this needs an Option) |
| Uses try/catch | 1 | `Reflection.classExists(1)`: a try/catch (ClassNotFoundException) |

## Fabric event handlers

| Handler | Hook | Result |
|---|---|---|
| org/popcraft/chunky/ChunkyFabric.java:29 | `init` | `on_init`, called from the hook |

## Every class


### `ChunkyAPI`: it's an interface

- [ ] `ChunkyAPI.version(0)`: it has no body (org/popcraft/chunky/api/ChunkyAPI.java:9)
- [ ] `ChunkyAPI.isRunning(1)`: it has no body (org/popcraft/chunky/api/ChunkyAPI.java:11)
- [ ] `ChunkyAPI.startTask(7)`: it has no body (org/popcraft/chunky/api/ChunkyAPI.java:13)
- [ ] `ChunkyAPI.pauseTask(1)`: it has no body (org/popcraft/chunky/api/ChunkyAPI.java:15)
- [ ] `ChunkyAPI.continueTask(1)`: it has no body (org/popcraft/chunky/api/ChunkyAPI.java:17)
- [ ] `ChunkyAPI.cancelTask(1)`: it has no body (org/popcraft/chunky/api/ChunkyAPI.java:19)
- [ ] `ChunkyAPI.onGenerationStart(1)`: it has no body (org/popcraft/chunky/api/ChunkyAPI.java:21)
- [ ] `ChunkyAPI.onGenerationProgress(1)`: it has no body (org/popcraft/chunky/api/ChunkyAPI.java:23)
- [ ] `ChunkyAPI.onGenerationComplete(1)`: it has no body (org/popcraft/chunky/api/ChunkyAPI.java:25)

### `ChunkyAPIImpl`: it implements `ChunkyAPI`

- [ ] `ChunkyAPIImpl.ChunkyAPIImpl(1)`: its class isn't translated (it implements `ChunkyAPI`) (org/popcraft/chunky/api/ChunkyAPIImpl.java:17)
- [ ] `ChunkyAPIImpl.version(0)`: its class isn't translated (it implements `ChunkyAPI`) (org/popcraft/chunky/api/ChunkyAPIImpl.java:21)
- [ ] `ChunkyAPIImpl.isRunning(1)`: its class isn't translated (it implements `ChunkyAPI`) (org/popcraft/chunky/api/ChunkyAPIImpl.java:26)
- [ ] `ChunkyAPIImpl.startTask(7)`: its class isn't translated (it implements `ChunkyAPI`) (org/popcraft/chunky/api/ChunkyAPIImpl.java:32)
- [ ] `ChunkyAPIImpl.pauseTask(1)`: its class isn't translated (it implements `ChunkyAPI`) (org/popcraft/chunky/api/ChunkyAPIImpl.java:56)
- [ ] `ChunkyAPIImpl.continueTask(1)`: its class isn't translated (it implements `ChunkyAPI`) (org/popcraft/chunky/api/ChunkyAPIImpl.java:72)
- [ ] `ChunkyAPIImpl.cancelTask(1)`: its class isn't translated (it implements `ChunkyAPI`) (org/popcraft/chunky/api/ChunkyAPIImpl.java:93)
- [ ] `ChunkyAPIImpl.onGenerationStart(1)`: its class isn't translated (it implements `ChunkyAPI`) (org/popcraft/chunky/api/ChunkyAPIImpl.java:109)
- [ ] `ChunkyAPIImpl.onGenerationProgress(1)`: its class isn't translated (it implements `ChunkyAPI`) (org/popcraft/chunky/api/ChunkyAPIImpl.java:114)
- [ ] `ChunkyAPIImpl.onGenerationComplete(1)`: its class isn't translated (it implements `ChunkyAPI`) (org/popcraft/chunky/api/ChunkyAPIImpl.java:119)

### `GenerationCompleteEvent`: it implements `Event` (org.popcraft.chunky.event)

- [ ] `GenerationCompleteEvent.GenerationCompleteEvent(11)`: its class isn't translated (it implements `Event` (org.popcraft.chunky.event)) (org/popcraft/chunky/api/event/task/GenerationCompleteEvent.java:5)
- [ ] `GenerationCompleteEvent.world(0)`: its class isn't translated (it implements `Event` (org.popcraft.chunky.event)) (org/popcraft/chunky/api/event/task/GenerationCompleteEvent.java:5)
- [ ] `GenerationCompleteEvent.shape(0)`: its class isn't translated (it implements `Event` (org.popcraft.chunky.event)) (org/popcraft/chunky/api/event/task/GenerationCompleteEvent.java:5)
- [ ] `GenerationCompleteEvent.centerX(0)`: its class isn't translated (it implements `Event` (org.popcraft.chunky.event)) (org/popcraft/chunky/api/event/task/GenerationCompleteEvent.java:5)
- [ ] `GenerationCompleteEvent.centerZ(0)`: its class isn't translated (it implements `Event` (org.popcraft.chunky.event)) (org/popcraft/chunky/api/event/task/GenerationCompleteEvent.java:5)
- [ ] `GenerationCompleteEvent.radiusX(0)`: its class isn't translated (it implements `Event` (org.popcraft.chunky.event)) (org/popcraft/chunky/api/event/task/GenerationCompleteEvent.java:5)
- [ ] `GenerationCompleteEvent.radiusZ(0)`: its class isn't translated (it implements `Event` (org.popcraft.chunky.event)) (org/popcraft/chunky/api/event/task/GenerationCompleteEvent.java:5)
- [ ] `GenerationCompleteEvent.pattern(0)`: its class isn't translated (it implements `Event` (org.popcraft.chunky.event)) (org/popcraft/chunky/api/event/task/GenerationCompleteEvent.java:5)
- [ ] `GenerationCompleteEvent.owner(0)`: its class isn't translated (it implements `Event` (org.popcraft.chunky.event)) (org/popcraft/chunky/api/event/task/GenerationCompleteEvent.java:5)
- [ ] `GenerationCompleteEvent.chunks(0)`: its class isn't translated (it implements `Event` (org.popcraft.chunky.event)) (org/popcraft/chunky/api/event/task/GenerationCompleteEvent.java:5)
- [ ] `GenerationCompleteEvent.time(0)`: its class isn't translated (it implements `Event` (org.popcraft.chunky.event)) (org/popcraft/chunky/api/event/task/GenerationCompleteEvent.java:5)
- [ ] `GenerationCompleteEvent.reason(0)`: its class isn't translated (it implements `Event` (org.popcraft.chunky.event)) (org/popcraft/chunky/api/event/task/GenerationCompleteEvent.java:5)

### `GenerationCompleteEvent.Reason`


### `GenerationProgressEvent`: it implements `Event` (org.popcraft.chunky.event)

- [ ] `GenerationProgressEvent.GenerationProgressEvent(10)`: its class isn't translated (it implements `Event` (org.popcraft.chunky.event)) (org/popcraft/chunky/api/event/task/GenerationProgressEvent.java:5)
- [ ] `GenerationProgressEvent.world(0)`: its class isn't translated (it implements `Event` (org.popcraft.chunky.event)) (org/popcraft/chunky/api/event/task/GenerationProgressEvent.java:5)
- [ ] `GenerationProgressEvent.chunks(0)`: its class isn't translated (it implements `Event` (org.popcraft.chunky.event)) (org/popcraft/chunky/api/event/task/GenerationProgressEvent.java:5)
- [ ] `GenerationProgressEvent.complete(0)`: its class isn't translated (it implements `Event` (org.popcraft.chunky.event)) (org/popcraft/chunky/api/event/task/GenerationProgressEvent.java:5)
- [ ] `GenerationProgressEvent.progress(0)`: its class isn't translated (it implements `Event` (org.popcraft.chunky.event)) (org/popcraft/chunky/api/event/task/GenerationProgressEvent.java:5)
- [ ] `GenerationProgressEvent.hours(0)`: its class isn't translated (it implements `Event` (org.popcraft.chunky.event)) (org/popcraft/chunky/api/event/task/GenerationProgressEvent.java:5)
- [ ] `GenerationProgressEvent.minutes(0)`: its class isn't translated (it implements `Event` (org.popcraft.chunky.event)) (org/popcraft/chunky/api/event/task/GenerationProgressEvent.java:5)
- [ ] `GenerationProgressEvent.seconds(0)`: its class isn't translated (it implements `Event` (org.popcraft.chunky.event)) (org/popcraft/chunky/api/event/task/GenerationProgressEvent.java:5)
- [ ] `GenerationProgressEvent.rate(0)`: its class isn't translated (it implements `Event` (org.popcraft.chunky.event)) (org/popcraft/chunky/api/event/task/GenerationProgressEvent.java:5)
- [ ] `GenerationProgressEvent.x(0)`: its class isn't translated (it implements `Event` (org.popcraft.chunky.event)) (org/popcraft/chunky/api/event/task/GenerationProgressEvent.java:5)
- [ ] `GenerationProgressEvent.z(0)`: its class isn't translated (it implements `Event` (org.popcraft.chunky.event)) (org/popcraft/chunky/api/event/task/GenerationProgressEvent.java:5)

### `GenerationStartEvent`: it implements `Event` (org.popcraft.chunky.event)

- [ ] `GenerationStartEvent.GenerationStartEvent(9)`: its class isn't translated (it implements `Event` (org.popcraft.chunky.event)) (org/popcraft/chunky/api/event/task/GenerationStartEvent.java:5)
- [ ] `GenerationStartEvent.world(0)`: its class isn't translated (it implements `Event` (org.popcraft.chunky.event)) (org/popcraft/chunky/api/event/task/GenerationStartEvent.java:5)
- [ ] `GenerationStartEvent.shape(0)`: its class isn't translated (it implements `Event` (org.popcraft.chunky.event)) (org/popcraft/chunky/api/event/task/GenerationStartEvent.java:5)
- [ ] `GenerationStartEvent.centerX(0)`: its class isn't translated (it implements `Event` (org.popcraft.chunky.event)) (org/popcraft/chunky/api/event/task/GenerationStartEvent.java:5)
- [ ] `GenerationStartEvent.centerZ(0)`: its class isn't translated (it implements `Event` (org.popcraft.chunky.event)) (org/popcraft/chunky/api/event/task/GenerationStartEvent.java:5)
- [ ] `GenerationStartEvent.radiusX(0)`: its class isn't translated (it implements `Event` (org.popcraft.chunky.event)) (org/popcraft/chunky/api/event/task/GenerationStartEvent.java:5)
- [ ] `GenerationStartEvent.radiusZ(0)`: its class isn't translated (it implements `Event` (org.popcraft.chunky.event)) (org/popcraft/chunky/api/event/task/GenerationStartEvent.java:5)
- [ ] `GenerationStartEvent.pattern(0)`: its class isn't translated (it implements `Event` (org.popcraft.chunky.event)) (org/popcraft/chunky/api/event/task/GenerationStartEvent.java:5)
- [ ] `GenerationStartEvent.owner(0)`: its class isn't translated (it implements `Event` (org.popcraft.chunky.event)) (org/popcraft/chunky/api/event/task/GenerationStartEvent.java:5)
- [ ] `GenerationStartEvent.continued(0)`: its class isn't translated (it implements `Event` (org.popcraft.chunky.event)) (org/popcraft/chunky/api/event/task/GenerationStartEvent.java:5)

### `Chunky`: it uses Server, which isn't translated (it's an interface)

- [ ] `Chunky.Chunky(2)`: its class isn't translated (it uses Server, which isn't translated (it's an interface)) (org/popcraft/chunky/Chunky.java:68)
- [ ] `Chunky.disable(0)`: its class isn't translated (it uses Server, which isn't translated (it's an interface)) (org/popcraft/chunky/Chunky.java:84)
- [ ] `Chunky.loadLimit(0)`: its class isn't translated (it uses Server, which isn't translated (it's an interface)) (org/popcraft/chunky/Chunky.java:92)
- [ ] `Chunky.loadVersion(0)`: its class isn't translated (it uses Server, which isn't translated (it's an interface)) (org/popcraft/chunky/Chunky.java:104)
- [ ] `Chunky.loadCommands(0)`: its class isn't translated (it uses Server, which isn't translated (it's an interface)) (org/popcraft/chunky/Chunky.java:114)
- [ ] `Chunky.getScheduler(0)`: its class isn't translated (it uses Server, which isn't translated (it's an interface)) (org/popcraft/chunky/Chunky.java:141)
- [ ] `Chunky.getServer(0)`: its class isn't translated (it uses Server, which isn't translated (it's an interface)) (org/popcraft/chunky/Chunky.java:145)
- [ ] `Chunky.getConfig(0)`: its class isn't translated (it uses Server, which isn't translated (it's an interface)) (org/popcraft/chunky/Chunky.java:149)
- [ ] `Chunky.getTaskLoader(0)`: its class isn't translated (it uses Server, which isn't translated (it's an interface)) (org/popcraft/chunky/Chunky.java:153)
- [ ] `Chunky.getEventBus(0)`: its class isn't translated (it uses Server, which isn't translated (it's an interface)) (org/popcraft/chunky/Chunky.java:157)
- [ ] `Chunky.getGenerationTasks(0)`: its class isn't translated (it uses Server, which isn't translated (it's an interface)) (org/popcraft/chunky/Chunky.java:161)
- [ ] `Chunky.getTrimTasks(0)`: its class isn't translated (it uses Server, which isn't translated (it's an interface)) (org/popcraft/chunky/Chunky.java:165)
- [ ] `Chunky.getCommands(0)`: its class isn't translated (it uses Server, which isn't translated (it's an interface)) (org/popcraft/chunky/Chunky.java:169)
- [ ] `Chunky.getSelection(0)`: its class isn't translated (it uses Server, which isn't translated (it's an interface)) (org/popcraft/chunky/Chunky.java:173)
- [ ] `Chunky.getPendingAction(1)`: its class isn't translated (it uses Server, which isn't translated (it's an interface)) (org/popcraft/chunky/Chunky.java:177)
- [ ] `Chunky.setPendingAction(2)`: its class isn't translated (it uses Server, which isn't translated (it's an interface)) (org/popcraft/chunky/Chunky.java:183)
- [ ] `Chunky.setLanguage(1)`: its class isn't translated (it uses Server, which isn't translated (it's an interface)) (org/popcraft/chunky/Chunky.java:187)
- [ ] `Chunky.getRegionCache(0)`: its class isn't translated (it uses Server, which isn't translated (it's an interface)) (org/popcraft/chunky/Chunky.java:191)
- [ ] `Chunky.getLimit(0)`: its class isn't translated (it uses Server, which isn't translated (it's an interface)) (org/popcraft/chunky/Chunky.java:195)
- [ ] `Chunky.getVersion(0)`: its class isn't translated (it uses Server, which isn't translated (it's an interface)) (org/popcraft/chunky/Chunky.java:199)
- [ ] `Chunky.getApi(0)`: its class isn't translated (it uses Server, which isn't translated (it's an interface)) (org/popcraft/chunky/Chunky.java:203)

### `ChunkyFabric`: it implements Fabric's `ModInitializer`

- [x] `ChunkyFabric.ENABLE_MOONRISE_WORKAROUNDS` (org/popcraft/chunky/ChunkyFabric.java:25)
- [ ] `ChunkyFabric.onInitialize(0)`: its class isn't translated (it implements Fabric's `ModInitializer`) (org/popcraft/chunky/ChunkyFabric.java:29)
- [ ] `ChunkyFabric.getChunky(0)`: its class isn't translated (it implements Fabric's `ModInitializer`) (org/popcraft/chunky/ChunkyFabric.java:60)

### `ChunkyProvider`

- [ ] `ChunkyProvider.instance`: it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface)) (org/popcraft/chunky/ChunkyProvider.java:4)
- [x] `ChunkyProvider.ChunkyProvider(0)` (org/popcraft/chunky/ChunkyProvider.java:6)
- [ ] `ChunkyProvider.get(0)`: it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface)) (org/popcraft/chunky/ChunkyProvider.java:10)
- [ ] `ChunkyProvider.register(1)`: it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface)) (org/popcraft/chunky/ChunkyProvider.java:18)
- [ ] `ChunkyProvider.unregister(0)`: it uses ChunkyProvider.instance, which isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))) (org/popcraft/chunky/ChunkyProvider.java:22)

### `BrigadierCommand`: it's generic

- [ ] `BrigadierCommand.getChunky(0)`: it has no body (org/popcraft/chunky/command/brigadier/BrigadierCommand.java:25)
- [ ] `BrigadierCommand.getSender(1)`: it has no body (org/popcraft/chunky/command/brigadier/BrigadierCommand.java:27)
- [ ] `BrigadierCommand.worldArgumentType(0)`: it has no body (org/popcraft/chunky/command/brigadier/BrigadierCommand.java:29)
- [ ] `BrigadierCommand.greedyWorldArgumentType(0)`: its class isn't translated (it's generic) (org/popcraft/chunky/command/brigadier/BrigadierCommand.java:31)
- [ ] `BrigadierCommand.hasPermission(1)`: it has no body (org/popcraft/chunky/command/brigadier/BrigadierCommand.java:35)
- [ ] `BrigadierCommand.hasPermission(2)`: its class isn't translated (it's generic) (org/popcraft/chunky/command/brigadier/BrigadierCommand.java:37)
- [ ] `BrigadierCommand.build(0)`: its class isn't translated (it's generic) (org/popcraft/chunky/command/brigadier/BrigadierCommand.java:41)
- [ ] `BrigadierCommand.execute(1)`: its class isn't translated (it's generic) (org/popcraft/chunky/command/brigadier/BrigadierCommand.java:117)
- [ ] `BrigadierCommand.isBorderAvailable(0)`: its class isn't translated (it's generic) (org/popcraft/chunky/command/brigadier/BrigadierCommand.java:127)
- [ ] `BrigadierCommand.literal(1)`: its class isn't translated (it's generic) (org/popcraft/chunky/command/brigadier/BrigadierCommand.java:132)
- [ ] `BrigadierCommand.argument(2)`: it's generic (org/popcraft/chunky/command/brigadier/BrigadierCommand.java:136)
- [ ] `BrigadierCommand.suggestedArgument(2)`: it's generic (org/popcraft/chunky/command/brigadier/BrigadierCommand.java:140)
- [ ] `BrigadierCommand.world(0)`: its class isn't translated (it's generic) (org/popcraft/chunky/command/brigadier/BrigadierCommand.java:144)
- [ ] `BrigadierCommand.greedyWorld(0)`: its class isn't translated (it's generic) (org/popcraft/chunky/command/brigadier/BrigadierCommand.java:148)
- [ ] `BrigadierCommand.registerCommand(3)`: its class isn't translated (it's generic) (org/popcraft/chunky/command/brigadier/BrigadierCommand.java:152)
- [ ] `BrigadierCommand.registerArguments(3)`: its class isn't translated (it's generic) (org/popcraft/chunky/command/brigadier/BrigadierCommand.java:157)

### `FabricCommand`: it extends `BrigadierCommand`

- [ ] `FabricCommand.FabricCommand(1)`: its class isn't translated (it extends `BrigadierCommand`) (org/popcraft/chunky/command/brigadier/FabricCommand.java:17)
- [ ] `FabricCommand.getChunky(0)`: its class isn't translated (it extends `BrigadierCommand`) (org/popcraft/chunky/command/brigadier/FabricCommand.java:21)
- [ ] `FabricCommand.getSender(1)`: its class isn't translated (it extends `BrigadierCommand`) (org/popcraft/chunky/command/brigadier/FabricCommand.java:26)
- [ ] `FabricCommand.worldArgumentType(0)`: its class isn't translated (it extends `BrigadierCommand`) (org/popcraft/chunky/command/brigadier/FabricCommand.java:30)
- [ ] `FabricCommand.hasPermission(1)`: its class isn't translated (it extends `BrigadierCommand`) (org/popcraft/chunky/command/brigadier/FabricCommand.java:35)

### `CancelCommand`: it implements `ChunkyCommand`

- [ ] `CancelCommand.CancelCommand(1)`: its class isn't translated (it implements `ChunkyCommand`) (org/popcraft/chunky/command/CancelCommand.java:17)
- [ ] `CancelCommand.execute(2)`: its class isn't translated (it implements `ChunkyCommand`) (org/popcraft/chunky/command/CancelCommand.java:21)
- [ ] `CancelCommand.suggestions(1)`: its class isn't translated (it implements `ChunkyCommand`) (org/popcraft/chunky/command/CancelCommand.java:65)

### `CenterCommand`: it implements `ChunkyCommand`

- [ ] `CenterCommand.CenterCommand(1)`: its class isn't translated (it implements `ChunkyCommand`) (org/popcraft/chunky/command/CenterCommand.java:14)
- [ ] `CenterCommand.execute(2)`: its class isn't translated (it implements `ChunkyCommand`) (org/popcraft/chunky/command/CenterCommand.java:18)
- [ ] `CenterCommand.suggestions(1)`: its class isn't translated (it implements `ChunkyCommand`) (org/popcraft/chunky/command/CenterCommand.java:46)

### `ChunkyCommand`: it's an interface

- [ ] `ChunkyCommand.execute(2)`: it has no body (org/popcraft/chunky/command/ChunkyCommand.java:7)
- [ ] `ChunkyCommand.suggestions(1)`: it has no body (org/popcraft/chunky/command/ChunkyCommand.java:9)

### `CommandArguments`: it uses `Queue` (java.util)

- [ ] `CommandArguments.CommandArguments(1)`: its class isn't translated (it uses `Queue` (java.util)) (org/popcraft/chunky/command/CommandArguments.java:13)
- [ ] `CommandArguments.of(1)`: it uses CommandArguments, which isn't translated (it uses `Queue` (java.util)) (org/popcraft/chunky/command/CommandArguments.java:18)
- [ ] `CommandArguments.of(1)`: it uses CommandArguments, which isn't translated (it uses `Queue` (java.util)) (org/popcraft/chunky/command/CommandArguments.java:22)
- [ ] `CommandArguments.empty(0)`: it uses CommandArguments, which isn't translated (it uses `Queue` (java.util)) (org/popcraft/chunky/command/CommandArguments.java:26)
- [ ] `CommandArguments.size(0)`: its class isn't translated (it uses `Queue` (java.util)) (org/popcraft/chunky/command/CommandArguments.java:30)
- [ ] `CommandArguments.next(0)`: its class isn't translated (it uses `Queue` (java.util)) (org/popcraft/chunky/command/CommandArguments.java:34)
- [ ] `CommandArguments.remaining(0)`: its class isn't translated (it uses `Queue` (java.util)) (org/popcraft/chunky/command/CommandArguments.java:38)
- [ ] `CommandArguments.joined(0)`: its class isn't translated (it uses `Queue` (java.util)) (org/popcraft/chunky/command/CommandArguments.java:44)

### `CommandLiteral`

- [x] `CommandLiteral.BORDER` (org/popcraft/chunky/command/CommandLiteral.java:4)
- [x] `CommandLiteral.CANCEL` (org/popcraft/chunky/command/CommandLiteral.java:5)
- [x] `CommandLiteral.CENTER` (org/popcraft/chunky/command/CommandLiteral.java:6)
- [x] `CommandLiteral.CENTER_X` (org/popcraft/chunky/command/CommandLiteral.java:7)
- [x] `CommandLiteral.CENTER_Z` (org/popcraft/chunky/command/CommandLiteral.java:8)
- [x] `CommandLiteral.CHUNKY` (org/popcraft/chunky/command/CommandLiteral.java:9)
- [x] `CommandLiteral.CONFIRM` (org/popcraft/chunky/command/CommandLiteral.java:10)
- [x] `CommandLiteral.CONTINUE` (org/popcraft/chunky/command/CommandLiteral.java:11)
- [x] `CommandLiteral.CORNERS` (org/popcraft/chunky/command/CommandLiteral.java:12)
- [x] `CommandLiteral.DEBUG` (org/popcraft/chunky/command/CommandLiteral.java:13)
- [x] `CommandLiteral.FILTERS` (org/popcraft/chunky/command/CommandLiteral.java:14)
- [x] `CommandLiteral.HELP` (org/popcraft/chunky/command/CommandLiteral.java:15)
- [x] `CommandLiteral.INHABITED` (org/popcraft/chunky/command/CommandLiteral.java:16)
- [x] `CommandLiteral.INTERVAL` (org/popcraft/chunky/command/CommandLiteral.java:17)
- [x] `CommandLiteral.LIMIT` (org/popcraft/chunky/command/CommandLiteral.java:18)
- [x] `CommandLiteral.MAP` (org/popcraft/chunky/command/CommandLiteral.java:19)
- [x] `CommandLiteral.PAGE` (org/popcraft/chunky/command/CommandLiteral.java:20)
- [x] `CommandLiteral.PATTERN` (org/popcraft/chunky/command/CommandLiteral.java:21)
- [x] `CommandLiteral.PAUSE` (org/popcraft/chunky/command/CommandLiteral.java:22)
- [x] `CommandLiteral.PROGRESS` (org/popcraft/chunky/command/CommandLiteral.java:23)
- [x] `CommandLiteral.QUIET` (org/popcraft/chunky/command/CommandLiteral.java:24)
- [x] `CommandLiteral.RADIUS` (org/popcraft/chunky/command/CommandLiteral.java:25)
- [x] `CommandLiteral.RADIUS_X` (org/popcraft/chunky/command/CommandLiteral.java:26)
- [x] `CommandLiteral.RADIUS_Z` (org/popcraft/chunky/command/CommandLiteral.java:27)
- [x] `CommandLiteral.RATE` (org/popcraft/chunky/command/CommandLiteral.java:28)
- [x] `CommandLiteral.RELOAD` (org/popcraft/chunky/command/CommandLiteral.java:29)
- [x] `CommandLiteral.SELECTION` (org/popcraft/chunky/command/CommandLiteral.java:30)
- [x] `CommandLiteral.SHAPE` (org/popcraft/chunky/command/CommandLiteral.java:31)
- [x] `CommandLiteral.SILENT` (org/popcraft/chunky/command/CommandLiteral.java:32)
- [x] `CommandLiteral.SPAWN` (org/popcraft/chunky/command/CommandLiteral.java:33)
- [x] `CommandLiteral.START` (org/popcraft/chunky/command/CommandLiteral.java:34)
- [x] `CommandLiteral.TRIM` (org/popcraft/chunky/command/CommandLiteral.java:35)
- [x] `CommandLiteral.TRIM_MODE` (org/popcraft/chunky/command/CommandLiteral.java:36)
- [x] `CommandLiteral.UNMODIFIED` (org/popcraft/chunky/command/CommandLiteral.java:37)
- [x] `CommandLiteral.WORLDBORDER` (org/popcraft/chunky/command/CommandLiteral.java:38)
- [x] `CommandLiteral.WORLD` (org/popcraft/chunky/command/CommandLiteral.java:39)
- [x] `CommandLiteral.X` (org/popcraft/chunky/command/CommandLiteral.java:40)
- [x] `CommandLiteral.X1` (org/popcraft/chunky/command/CommandLiteral.java:41)
- [x] `CommandLiteral.X2` (org/popcraft/chunky/command/CommandLiteral.java:42)
- [x] `CommandLiteral.Z` (org/popcraft/chunky/command/CommandLiteral.java:43)
- [x] `CommandLiteral.Z1` (org/popcraft/chunky/command/CommandLiteral.java:44)
- [x] `CommandLiteral.Z2` (org/popcraft/chunky/command/CommandLiteral.java:45)
- [x] `CommandLiteral.ADD` (org/popcraft/chunky/command/CommandLiteral.java:46)
- [x] `CommandLiteral.BYPASS` (org/popcraft/chunky/command/CommandLiteral.java:47)
- [x] `CommandLiteral.LIST` (org/popcraft/chunky/command/CommandLiteral.java:48)
- [x] `CommandLiteral.LOAD` (org/popcraft/chunky/command/CommandLiteral.java:49)
- [x] `CommandLiteral.REMOVE` (org/popcraft/chunky/command/CommandLiteral.java:50)
- [x] `CommandLiteral.WRAP` (org/popcraft/chunky/command/CommandLiteral.java:51)
- [x] `CommandLiteral.PLAYER` (org/popcraft/chunky/command/CommandLiteral.java:52)
- [x] `CommandLiteral.VALUE` (org/popcraft/chunky/command/CommandLiteral.java:53)
- [x] `CommandLiteral.TYPE` (org/popcraft/chunky/command/CommandLiteral.java:54)
- [x] `CommandLiteral.CommandLiteral(0)` (org/popcraft/chunky/command/CommandLiteral.java:56)

### `ConfirmCommand`: it implements `ChunkyCommand`

- [ ] `ConfirmCommand.ConfirmCommand(1)`: its class isn't translated (it implements `ChunkyCommand`) (org/popcraft/chunky/command/ConfirmCommand.java:11)
- [ ] `ConfirmCommand.execute(2)`: its class isn't translated (it implements `ChunkyCommand`) (org/popcraft/chunky/command/ConfirmCommand.java:15)
- [ ] `ConfirmCommand.suggestions(1)`: its class isn't translated (it implements `ChunkyCommand`) (org/popcraft/chunky/command/ConfirmCommand.java:25)

### `ContinueCommand`: it implements `ChunkyCommand`

- [ ] `ContinueCommand.ContinueCommand(1)`: its class isn't translated (it implements `ChunkyCommand`) (org/popcraft/chunky/command/ContinueCommand.java:17)
- [ ] `ContinueCommand.execute(2)`: its class isn't translated (it implements `ChunkyCommand`) (org/popcraft/chunky/command/ContinueCommand.java:21)
- [ ] `ContinueCommand.suggestions(1)`: its class isn't translated (it implements `ChunkyCommand`) (org/popcraft/chunky/command/ContinueCommand.java:58)

### `CornersCommand`: it implements `ChunkyCommand`

- [ ] `CornersCommand.CornersCommand(1)`: its class isn't translated (it implements `ChunkyCommand`) (org/popcraft/chunky/command/CornersCommand.java:15)
- [ ] `CornersCommand.execute(2)`: its class isn't translated (it implements `ChunkyCommand`) (org/popcraft/chunky/command/CornersCommand.java:19)
- [ ] `CornersCommand.suggestions(1)`: its class isn't translated (it implements `ChunkyCommand`) (org/popcraft/chunky/command/CornersCommand.java:56)

### `DebugCommand`: it implements `ChunkyCommand`

- [ ] `DebugCommand.DebugCommand(1)`: its class isn't translated (it implements `ChunkyCommand`) (org/popcraft/chunky/command/DebugCommand.java:17)
- [ ] `DebugCommand.execute(2)`: its class isn't translated (it implements `ChunkyCommand`) (org/popcraft/chunky/command/DebugCommand.java:21)
- [ ] `DebugCommand.suggestions(1)`: its class isn't translated (it implements `ChunkyCommand`) (org/popcraft/chunky/command/DebugCommand.java:61)

### `HelpCommand`: it implements `ChunkyCommand`

- [ ] `HelpCommand.HelpCommand(1)`: its class isn't translated (it implements `ChunkyCommand`) (org/popcraft/chunky/command/HelpCommand.java:35)
- [ ] `HelpCommand.execute(2)`: its class isn't translated (it implements `ChunkyCommand`) (org/popcraft/chunky/command/HelpCommand.java:39)
- [ ] `HelpCommand.suggestions(1)`: its class isn't translated (it implements `ChunkyCommand`) (org/popcraft/chunky/command/HelpCommand.java:74)

### `LimitCommand`: it implements `ChunkyCommand`

- [ ] `LimitCommand.LimitCommand(1)`: its class isn't translated (it implements `ChunkyCommand`) (org/popcraft/chunky/command/LimitCommand.java:14)
- [ ] `LimitCommand.execute(2)`: its class isn't translated (it implements `ChunkyCommand`) (org/popcraft/chunky/command/LimitCommand.java:18)
- [ ] `LimitCommand.suggestions(1)`: its class isn't translated (it implements `ChunkyCommand`) (org/popcraft/chunky/command/LimitCommand.java:45)

### `PatternCommand`: it implements `ChunkyCommand`

- [ ] `PatternCommand.PatternCommand(1)`: its class isn't translated (it implements `ChunkyCommand`) (org/popcraft/chunky/command/PatternCommand.java:15)
- [ ] `PatternCommand.execute(2)`: its class isn't translated (it implements `ChunkyCommand`) (org/popcraft/chunky/command/PatternCommand.java:19)
- [ ] `PatternCommand.suggestions(1)`: its class isn't translated (it implements `ChunkyCommand`) (org/popcraft/chunky/command/PatternCommand.java:42)

### `PauseCommand`: it implements `ChunkyCommand`

- [ ] `PauseCommand.PauseCommand(1)`: its class isn't translated (it implements `ChunkyCommand`) (org/popcraft/chunky/command/PauseCommand.java:17)
- [ ] `PauseCommand.execute(2)`: its class isn't translated (it implements `ChunkyCommand`) (org/popcraft/chunky/command/PauseCommand.java:21)
- [ ] `PauseCommand.suggestions(1)`: its class isn't translated (it implements `ChunkyCommand`) (org/popcraft/chunky/command/PauseCommand.java:45)

### `ProgressCommand`: it implements `ChunkyCommand`

- [ ] `ProgressCommand.ProgressCommand(1)`: its class isn't translated (it implements `ChunkyCommand`) (org/popcraft/chunky/command/ProgressCommand.java:13)
- [ ] `ProgressCommand.execute(2)`: its class isn't translated (it implements `ChunkyCommand`) (org/popcraft/chunky/command/ProgressCommand.java:17)
- [ ] `ProgressCommand.suggestions(1)`: its class isn't translated (it implements `ChunkyCommand`) (org/popcraft/chunky/command/ProgressCommand.java:31)

### `QuietCommand`: it implements `ChunkyCommand`

- [ ] `QuietCommand.QuietCommand(1)`: its class isn't translated (it implements `ChunkyCommand`) (org/popcraft/chunky/command/QuietCommand.java:12)
- [ ] `QuietCommand.execute(2)`: its class isn't translated (it implements `ChunkyCommand`) (org/popcraft/chunky/command/QuietCommand.java:16)
- [ ] `QuietCommand.suggestions(1)`: its class isn't translated (it implements `ChunkyCommand`) (org/popcraft/chunky/command/QuietCommand.java:28)

### `RadiusCommand`: it implements `ChunkyCommand`

- [ ] `RadiusCommand.RadiusCommand(1)`: its class isn't translated (it implements `ChunkyCommand`) (org/popcraft/chunky/command/RadiusCommand.java:15)
- [ ] `RadiusCommand.execute(2)`: its class isn't translated (it implements `ChunkyCommand`) (org/popcraft/chunky/command/RadiusCommand.java:19)
- [ ] `RadiusCommand.suggestions(1)`: its class isn't translated (it implements `ChunkyCommand`) (org/popcraft/chunky/command/RadiusCommand.java:70)

### `ReloadCommand`: it implements `ChunkyCommand`

- [ ] `ReloadCommand.ReloadCommand(1)`: its class isn't translated (it implements `ChunkyCommand`) (org/popcraft/chunky/command/ReloadCommand.java:12)
- [ ] `ReloadCommand.execute(2)`: its class isn't translated (it implements `ChunkyCommand`) (org/popcraft/chunky/command/ReloadCommand.java:16)
- [ ] `ReloadCommand.suggestions(1)`: its class isn't translated (it implements `ChunkyCommand`) (org/popcraft/chunky/command/ReloadCommand.java:36)

### `SelectionCommand`: it implements `ChunkyCommand`

- [ ] `SelectionCommand.SelectionCommand(1)`: its class isn't translated (it implements `ChunkyCommand`) (org/popcraft/chunky/command/SelectionCommand.java:13)
- [ ] `SelectionCommand.execute(2)`: its class isn't translated (it implements `ChunkyCommand`) (org/popcraft/chunky/command/SelectionCommand.java:17)
- [ ] `SelectionCommand.suggestions(1)`: its class isn't translated (it implements `ChunkyCommand`) (org/popcraft/chunky/command/SelectionCommand.java:33)

### `ShapeCommand`: it implements `ChunkyCommand`

- [ ] `ShapeCommand.ShapeCommand(1)`: its class isn't translated (it implements `ChunkyCommand`) (org/popcraft/chunky/command/ShapeCommand.java:14)
- [ ] `ShapeCommand.execute(2)`: its class isn't translated (it implements `ChunkyCommand`) (org/popcraft/chunky/command/ShapeCommand.java:18)
- [ ] `ShapeCommand.suggestions(1)`: its class isn't translated (it implements `ChunkyCommand`) (org/popcraft/chunky/command/ShapeCommand.java:34)

### `SilentCommand`: it implements `ChunkyCommand`

- [ ] `SilentCommand.SilentCommand(1)`: its class isn't translated (it implements `ChunkyCommand`) (org/popcraft/chunky/command/SilentCommand.java:11)
- [ ] `SilentCommand.execute(2)`: its class isn't translated (it implements `ChunkyCommand`) (org/popcraft/chunky/command/SilentCommand.java:15)
- [ ] `SilentCommand.suggestions(1)`: its class isn't translated (it implements `ChunkyCommand`) (org/popcraft/chunky/command/SilentCommand.java:22)

### `SpawnCommand`: it implements `ChunkyCommand`

- [ ] `SpawnCommand.SpawnCommand(1)`: its class isn't translated (it implements `ChunkyCommand`) (org/popcraft/chunky/command/SpawnCommand.java:12)
- [ ] `SpawnCommand.execute(2)`: its class isn't translated (it implements `ChunkyCommand`) (org/popcraft/chunky/command/SpawnCommand.java:16)
- [ ] `SpawnCommand.suggestions(1)`: its class isn't translated (it implements `ChunkyCommand`) (org/popcraft/chunky/command/SpawnCommand.java:23)

### `StartCommand`: it implements `ChunkyCommand`

- [ ] `StartCommand.StartCommand(1)`: its class isn't translated (it implements `ChunkyCommand`) (org/popcraft/chunky/command/StartCommand.java:19)
- [ ] `StartCommand.execute(2)`: its class isn't translated (it implements `ChunkyCommand`) (org/popcraft/chunky/command/StartCommand.java:23)
- [ ] `StartCommand.suggestions(1)`: its class isn't translated (it implements `ChunkyCommand`) (org/popcraft/chunky/command/StartCommand.java:110)

### `CommandSuggestions`

- [x] `CommandSuggestions.CommandSuggestions(0)` (org/popcraft/chunky/command/suggestion/CommandSuggestions.java:16)
- [ ] `CommandSuggestions.suggest(3)`: it's generic (org/popcraft/chunky/command/suggestion/CommandSuggestions.java:19)

### `WorldSuggestions`

- [x] `WorldSuggestions.WorldSuggestions(0)` (org/popcraft/chunky/command/suggestion/WorldSuggestions.java:10)
- [ ] `WorldSuggestions.suggest(2)`: it uses Minecraft's `Suggestions` (org/popcraft/chunky/command/suggestion/WorldSuggestions.java:13)

### `TrimCommand`: it implements `ChunkyCommand`

- [ ] `TrimCommand.TrimCommand(1)`: its class isn't translated (it implements `ChunkyCommand`) (org/popcraft/chunky/command/TrimCommand.java:31)
- [ ] `TrimCommand.execute(2)`: its class isn't translated (it implements `ChunkyCommand`) (org/popcraft/chunky/command/TrimCommand.java:35)
- [ ] `TrimCommand.checkRegion(5)`: its class isn't translated (it implements `ChunkyCommand`) (org/popcraft/chunky/command/TrimCommand.java:195)
- [ ] `TrimCommand.shouldDeleteRegion(4)`: its class isn't translated (it implements `ChunkyCommand`) (org/popcraft/chunky/command/TrimCommand.java:208)
- [ ] `TrimCommand.deleteRegion(2)`: its class isn't translated (it implements `ChunkyCommand`) (org/popcraft/chunky/command/TrimCommand.java:222)
- [ ] `TrimCommand.trimRegion(7)`: its class isn't translated (it implements `ChunkyCommand`) (org/popcraft/chunky/command/TrimCommand.java:243)
- [ ] `TrimCommand.deleteExternalChunk(5)`: its class isn't translated (it implements `ChunkyCommand`) (org/popcraft/chunky/command/TrimCommand.java:330)
- [ ] `TrimCommand.suggestions(1)`: its class isn't translated (it implements `ChunkyCommand`) (org/popcraft/chunky/command/TrimCommand.java:337)

### `TrimCommand.Task`

- [x] `TrimCommand.Task.isCancelled(0)` (org/popcraft/chunky/command/TrimCommand.java:358)
- [x] `TrimCommand.Task.setCancelled(1)` (org/popcraft/chunky/command/TrimCommand.java:362)

### `WorldBorderCommand`: it implements `ChunkyCommand`

- [ ] `WorldBorderCommand.WorldBorderCommand(1)`: its class isn't translated (it implements `ChunkyCommand`) (org/popcraft/chunky/command/WorldBorderCommand.java:22)
- [ ] `WorldBorderCommand.execute(2)`: its class isn't translated (it implements `ChunkyCommand`) (org/popcraft/chunky/command/WorldBorderCommand.java:26)
- [ ] `WorldBorderCommand.suggestions(1)`: its class isn't translated (it implements `ChunkyCommand`) (org/popcraft/chunky/command/WorldBorderCommand.java:61)
- [ ] `WorldBorderCommand.setBorderViaIntegration(1)`: its class isn't translated (it implements `ChunkyCommand`) (org/popcraft/chunky/command/WorldBorderCommand.java:72)

### `WorldCommand`: it implements `ChunkyCommand`

- [ ] `WorldCommand.WorldCommand(1)`: its class isn't translated (it implements `ChunkyCommand`) (org/popcraft/chunky/command/WorldCommand.java:15)
- [ ] `WorldCommand.execute(2)`: its class isn't translated (it implements `ChunkyCommand`) (org/popcraft/chunky/command/WorldCommand.java:19)
- [ ] `WorldCommand.suggestions(1)`: its class isn't translated (it implements `ChunkyCommand`) (org/popcraft/chunky/command/WorldCommand.java:41)

### `MinecraftServerExtension`: it's an interface

- [ ] `MinecraftServerExtension.chunky$runChunkSystemHousekeeping(1)`: it has no body (org/popcraft/chunky/ducks/MinecraftServerExtension.java:6)
- [ ] `MinecraftServerExtension.chunky$markChunkSystemHousekeeping(0)`: it has no body (org/popcraft/chunky/ducks/MinecraftServerExtension.java:8)

### `Cancellable`: it implements `Event`

- [ ] `Cancellable.isCancelled(0)`: its class isn't translated (it implements `Event`) (org/popcraft/chunky/event/Cancellable.java:6)
- [ ] `Cancellable.setCancelled(1)`: its class isn't translated (it implements `Event`) (org/popcraft/chunky/event/Cancellable.java:10)

### `ReloadCommandEvent`: it implements `Event` (org.popcraft.chunky.event)

- [ ] `ReloadCommandEvent.ReloadCommandEvent(0)`: its class isn't translated (it implements `Event` (org.popcraft.chunky.event)) (org/popcraft/chunky/event/command/ReloadCommandEvent.java:5)

### `Event`: it's an interface


### `EventBus`: it uses `Class`

- [ ] `EventBus.accept`: it uses `MethodHandle` (java.lang.invoke) (org/popcraft/chunky/event/EventBus.java:13)
- [ ] `EventBus.subscribe(2)`: it's generic (org/popcraft/chunky/event/EventBus.java:16)
- [ ] `EventBus.unsubscribe(2)`: it's generic (org/popcraft/chunky/event/EventBus.java:21)
- [ ] `EventBus.unsubscribeAll(0)`: its class isn't translated (it uses `Class`) (org/popcraft/chunky/event/EventBus.java:26)
- [ ] `EventBus.unsubscribeAll(1)`: its class isn't translated (it uses `Class`) (org/popcraft/chunky/event/EventBus.java:30)
- [ ] `EventBus.call(1)`: its class isn't translated (it uses `Class`) (org/popcraft/chunky/event/EventBus.java:34)

### `GenerationTaskFinishEvent`: it implements `Event` (org.popcraft.chunky.event)

- [ ] `GenerationTaskFinishEvent.GenerationTaskFinishEvent(1)`: its class isn't translated (it implements `Event` (org.popcraft.chunky.event)) (org/popcraft/chunky/event/task/GenerationTaskFinishEvent.java:6)
- [ ] `GenerationTaskFinishEvent.generationTask(0)`: its class isn't translated (it implements `Event` (org.popcraft.chunky.event)) (org/popcraft/chunky/event/task/GenerationTaskFinishEvent.java:6)

### `GenerationTaskUpdateEvent`: it implements `Event` (org.popcraft.chunky.event)

- [ ] `GenerationTaskUpdateEvent.GenerationTaskUpdateEvent(1)`: its class isn't translated (it implements `Event` (org.popcraft.chunky.event)) (org/popcraft/chunky/event/task/GenerationTaskUpdateEvent.java:6)
- [ ] `GenerationTaskUpdateEvent.generationTask(0)`: its class isn't translated (it implements `Event` (org.popcraft.chunky.event)) (org/popcraft/chunky/event/task/GenerationTaskUpdateEvent.java:6)

### `GenerationTask`: it implements `Runnable`

- [ ] `GenerationTask.SAMPLE_INTERVAL`: it calls Input.tryInteger, which isn't translated (org/popcraft/chunky/GenerationTask.java:28)
- [ ] `GenerationTask.SAMPLE_SUB_INTERVAL`: it uses GenerationTask.SAMPLE_INTERVAL, which isn't translated (it calls Input.tryInteger, which isn't translated) (org/popcraft/chunky/GenerationTask.java:29)
- [ ] `GenerationTask.AUTO_SAVE_INTERVAL`: it calls Input.tryInteger, which isn't translated (org/popcraft/chunky/GenerationTask.java:30)
- [ ] `GenerationTask.GenerationTask(5)`: its class isn't translated (it implements `Runnable`) (org/popcraft/chunky/GenerationTask.java:51)
- [ ] `GenerationTask.GenerationTask(2)`: its class isn't translated (it implements `Runnable`) (org/popcraft/chunky/GenerationTask.java:60)
- [ ] `GenerationTask.update(3)`: its class isn't translated (it implements `Runnable`) (org/popcraft/chunky/GenerationTask.java:70)
- [ ] `GenerationTask.autoSave(0)`: its class isn't translated (it implements `Runnable`) (org/popcraft/chunky/GenerationTask.java:154)
- [ ] `GenerationTask.run(0)`: its class isn't translated (it implements `Runnable`) (org/popcraft/chunky/GenerationTask.java:166)
- [ ] `GenerationTask.createCompleteEvent(1)`: its class isn't translated (it implements `Runnable`) (org/popcraft/chunky/GenerationTask.java:271)
- [ ] `GenerationTask.stop(1)`: its class isn't translated (it implements `Runnable`) (org/popcraft/chunky/GenerationTask.java:288)
- [ ] `GenerationTask.getOwner(0)`: its class isn't translated (it implements `Runnable`) (org/popcraft/chunky/GenerationTask.java:293)
- [ ] `GenerationTask.setOwner(1)`: its class isn't translated (it implements `Runnable`) (org/popcraft/chunky/GenerationTask.java:297)
- [ ] `GenerationTask.getChunky(0)`: its class isn't translated (it implements `Runnable`) (org/popcraft/chunky/GenerationTask.java:301)
- [ ] `GenerationTask.getSelection(0)`: its class isn't translated (it implements `Runnable`) (org/popcraft/chunky/GenerationTask.java:305)
- [ ] `GenerationTask.getCount(0)`: its class isn't translated (it implements `Runnable`) (org/popcraft/chunky/GenerationTask.java:309)
- [ ] `GenerationTask.getChunkIterator(0)`: its class isn't translated (it implements `Runnable`) (org/popcraft/chunky/GenerationTask.java:313)
- [ ] `GenerationTask.getRateLimiter(0)`: its class isn't translated (it implements `Runnable`) (org/popcraft/chunky/GenerationTask.java:317)
- [ ] `GenerationTask.getShape(0)`: its class isn't translated (it implements `Runnable`) (org/popcraft/chunky/GenerationTask.java:321)
- [ ] `GenerationTask.isCancelled(0)`: its class isn't translated (it implements `Runnable`) (org/popcraft/chunky/GenerationTask.java:325)
- [ ] `GenerationTask.getTotalTime(0)`: its class isn't translated (it implements `Runnable`) (org/popcraft/chunky/GenerationTask.java:329)
- [ ] `GenerationTask.getProgress(0)`: its class isn't translated (it implements `Runnable`) (org/popcraft/chunky/GenerationTask.java:333)

### `GenerationTask.Progress`

- [x] `GenerationTask.Progress.Progress(1)` (org/popcraft/chunky/GenerationTask.java:349)
- [x] `GenerationTask.Progress.getWorld(0)` (org/popcraft/chunky/GenerationTask.java:353)
- [x] `GenerationTask.Progress.getChunkCount(0)` (org/popcraft/chunky/GenerationTask.java:357)
- [x] `GenerationTask.Progress.isComplete(0)` (org/popcraft/chunky/GenerationTask.java:361)
- [x] `GenerationTask.Progress.getPercentComplete(0)` (org/popcraft/chunky/GenerationTask.java:365)
- [x] `GenerationTask.Progress.getHours(0)` (org/popcraft/chunky/GenerationTask.java:369)
- [x] `GenerationTask.Progress.getMinutes(0)` (org/popcraft/chunky/GenerationTask.java:373)
- [x] `GenerationTask.Progress.getSeconds(0)` (org/popcraft/chunky/GenerationTask.java:377)
- [x] `GenerationTask.Progress.getRate(0)` (org/popcraft/chunky/GenerationTask.java:381)
- [x] `GenerationTask.Progress.getChunkX(0)` (org/popcraft/chunky/GenerationTask.java:385)
- [x] `GenerationTask.Progress.getChunkZ(0)` (org/popcraft/chunky/GenerationTask.java:389)
- [ ] `GenerationTask.Progress.sendUpdate(1)`: it uses Sender, which isn't translated (it's an interface) (org/popcraft/chunky/GenerationTask.java:393)

### `BorderIntegration`: it's an interface

- [ ] `BorderIntegration.hasBorder(1)`: it has no body (org/popcraft/chunky/integration/BorderIntegration.java:6)
- [ ] `BorderIntegration.getBorder(1)`: it has no body (org/popcraft/chunky/integration/BorderIntegration.java:8)

### `Integration`: it's an interface


### `ChunkIterator`: it's an interface

- [ ] `ChunkIterator.total(0)`: it has no body (org/popcraft/chunky/iterator/ChunkIterator.java:7)
- [ ] `ChunkIterator.name(0)`: it has no body (org/popcraft/chunky/iterator/ChunkIterator.java:9)
- [ ] `ChunkIterator.process(0)`: its class isn't translated (it's an interface) (org/popcraft/chunky/iterator/ChunkIterator.java:11)

### `ChunkIteratorFactory`

- [x] `ChunkIteratorFactory.ChunkIteratorFactory(0)` (org/popcraft/chunky/iterator/ChunkIteratorFactory.java:6)
- [ ] `ChunkIteratorFactory.getChunkIterator(2)`: it uses ChunkIterator, which isn't translated (it's an interface) (org/popcraft/chunky/iterator/ChunkIteratorFactory.java:9)
- [ ] `ChunkIteratorFactory.getChunkIterator(1)`: it uses ChunkIterator, which isn't translated (it's an interface) (org/popcraft/chunky/iterator/ChunkIteratorFactory.java:33)

### `ConcentricChunkIterator`: it implements `ChunkIterator`

- [ ] `ConcentricChunkIterator.ConcentricChunkIterator(2)`: its class isn't translated (it implements `ChunkIterator`) (org/popcraft/chunky/iterator/ConcentricChunkIterator.java:22)
- [ ] `ConcentricChunkIterator.ConcentricChunkIterator(1)`: its class isn't translated (it implements `ChunkIterator`) (org/popcraft/chunky/iterator/ConcentricChunkIterator.java:50)
- [ ] `ConcentricChunkIterator.hasNext(0)`: its class isn't translated (it implements `ChunkIterator`) (org/popcraft/chunky/iterator/ConcentricChunkIterator.java:60)
- [ ] `ConcentricChunkIterator.next(0)`: its class isn't translated (it implements `ChunkIterator`) (org/popcraft/chunky/iterator/ConcentricChunkIterator.java:65)
- [ ] `ConcentricChunkIterator.total(0)`: its class isn't translated (it implements `ChunkIterator`) (org/popcraft/chunky/iterator/ConcentricChunkIterator.java:100)
- [ ] `ConcentricChunkIterator.name(0)`: its class isn't translated (it implements `ChunkIterator`) (org/popcraft/chunky/iterator/ConcentricChunkIterator.java:105)

### `CsvChunkIterator`: it implements `ChunkIterator`

- [ ] `CsvChunkIterator.CsvChunkIterator(2)`: its class isn't translated (it implements `ChunkIterator`) (org/popcraft/chunky/iterator/CsvChunkIterator.java:21)
- [ ] `CsvChunkIterator.CsvChunkIterator(1)`: its class isn't translated (it implements `ChunkIterator`) (org/popcraft/chunky/iterator/CsvChunkIterator.java:29)
- [ ] `CsvChunkIterator.hasNext(0)`: its class isn't translated (it implements `ChunkIterator`) (org/popcraft/chunky/iterator/CsvChunkIterator.java:59)
- [ ] `CsvChunkIterator.next(0)`: its class isn't translated (it implements `ChunkIterator`) (org/popcraft/chunky/iterator/CsvChunkIterator.java:64)
- [ ] `CsvChunkIterator.total(0)`: its class isn't translated (it implements `ChunkIterator`) (org/popcraft/chunky/iterator/CsvChunkIterator.java:72)
- [ ] `CsvChunkIterator.name(0)`: its class isn't translated (it implements `ChunkIterator`) (org/popcraft/chunky/iterator/CsvChunkIterator.java:77)

### `Loop2ChunkIterator`: it implements `ChunkIterator`

- [ ] `Loop2ChunkIterator.Loop2ChunkIterator(2)`: its class isn't translated (it implements `ChunkIterator`) (org/popcraft/chunky/iterator/Loop2ChunkIterator.java:18)
- [ ] `Loop2ChunkIterator.Loop2ChunkIterator(1)`: its class isn't translated (it implements `ChunkIterator`) (org/popcraft/chunky/iterator/Loop2ChunkIterator.java:29)
- [ ] `Loop2ChunkIterator.hasNext(0)`: its class isn't translated (it implements `ChunkIterator`) (org/popcraft/chunky/iterator/Loop2ChunkIterator.java:45)
- [ ] `Loop2ChunkIterator.next(0)`: its class isn't translated (it implements `ChunkIterator`) (org/popcraft/chunky/iterator/Loop2ChunkIterator.java:50)
- [ ] `Loop2ChunkIterator.total(0)`: its class isn't translated (it implements `ChunkIterator`) (org/popcraft/chunky/iterator/Loop2ChunkIterator.java:66)
- [ ] `Loop2ChunkIterator.name(0)`: its class isn't translated (it implements `ChunkIterator`) (org/popcraft/chunky/iterator/Loop2ChunkIterator.java:71)

### `PatternType`

- [x] `PatternType.CONCENTRIC` (org/popcraft/chunky/iterator/PatternType.java:6)
- [x] `PatternType.LOOP` (org/popcraft/chunky/iterator/PatternType.java:7)
- [x] `PatternType.SPIRAL` (org/popcraft/chunky/iterator/PatternType.java:8)
- [x] `PatternType.CSV` (org/popcraft/chunky/iterator/PatternType.java:9)
- [x] `PatternType.REGION` (org/popcraft/chunky/iterator/PatternType.java:10)
- [x] `PatternType.WORLD` (org/popcraft/chunky/iterator/PatternType.java:11)
- [x] `PatternType.STRIP` (org/popcraft/chunky/iterator/PatternType.java:12)
- [x] `PatternType.RING` (org/popcraft/chunky/iterator/PatternType.java:13)
- [x] `PatternType.ALL` (org/popcraft/chunky/iterator/PatternType.java:14)
- [x] `PatternType.PatternType(0)` (org/popcraft/chunky/iterator/PatternType.java:16)

### `RegionChunkIterator`: it implements `ChunkIterator`

- [ ] `RegionChunkIterator.RegionChunkIterator(2)`: its class isn't translated (it implements `ChunkIterator`) (org/popcraft/chunky/iterator/RegionChunkIterator.java:29)
- [ ] `RegionChunkIterator.RegionChunkIterator(1)`: its class isn't translated (it implements `ChunkIterator`) (org/popcraft/chunky/iterator/RegionChunkIterator.java:73)
- [ ] `RegionChunkIterator.hasNext(0)`: its class isn't translated (it implements `ChunkIterator`) (org/popcraft/chunky/iterator/RegionChunkIterator.java:91)
- [ ] `RegionChunkIterator.next(0)`: its class isn't translated (it implements `ChunkIterator`) (org/popcraft/chunky/iterator/RegionChunkIterator.java:96)
- [ ] `RegionChunkIterator.nextRegionChunkProgress(0)`: its class isn't translated (it implements `ChunkIterator`) (org/popcraft/chunky/iterator/RegionChunkIterator.java:114)
- [ ] `RegionChunkIterator.nextRegionChunkProgress(1)`: its class isn't translated (it implements `ChunkIterator`) (org/popcraft/chunky/iterator/RegionChunkIterator.java:118)
- [ ] `RegionChunkIterator.total(0)`: its class isn't translated (it implements `ChunkIterator`) (org/popcraft/chunky/iterator/RegionChunkIterator.java:153)
- [ ] `RegionChunkIterator.name(0)`: its class isn't translated (it implements `ChunkIterator`) (org/popcraft/chunky/iterator/RegionChunkIterator.java:158)

### `RegionChunkIterator.RegionChunkProgress`: it implements `ChunkIterator`

- [ ] `RegionChunkIterator.RegionChunkProgress.RegionChunkProgress(3)`: its class isn't translated (it implements `ChunkIterator`) (org/popcraft/chunky/iterator/RegionChunkIterator.java:175)
- [ ] `RegionChunkIterator.RegionChunkProgress.RegionChunkProgress(2)`: its class isn't translated (it implements `ChunkIterator`) (org/popcraft/chunky/iterator/RegionChunkIterator.java:190)
- [ ] `RegionChunkIterator.RegionChunkProgress.hasNext(0)`: its class isn't translated (it implements `ChunkIterator`) (org/popcraft/chunky/iterator/RegionChunkIterator.java:214)
- [ ] `RegionChunkIterator.RegionChunkProgress.next(0)`: its class isn't translated (it implements `ChunkIterator`) (org/popcraft/chunky/iterator/RegionChunkIterator.java:219)
- [ ] `RegionChunkIterator.RegionChunkProgress.total(0)`: its class isn't translated (it implements `ChunkIterator`) (org/popcraft/chunky/iterator/RegionChunkIterator.java:244)
- [ ] `RegionChunkIterator.RegionChunkProgress.name(0)`: its class isn't translated (it implements `ChunkIterator`) (org/popcraft/chunky/iterator/RegionChunkIterator.java:249)

### `RingChunkIterator`: it implements `ChunkIterator`

- [x] `RingChunkIterator.SPLIT` (org/popcraft/chunky/iterator/RingChunkIterator.java:9)
- [x] `RingChunkIterator.ALTERNATING` (org/popcraft/chunky/iterator/RingChunkIterator.java:10)
- [ ] `RingChunkIterator.WIDTH`: it calls Input.tryInteger, which isn't translated (org/popcraft/chunky/iterator/RingChunkIterator.java:11)
- [ ] `RingChunkIterator.CENTER_RADIUS`: it uses RingChunkIterator.WIDTH, which isn't translated (it calls Input.tryInteger, which isn't translated) (org/popcraft/chunky/iterator/RingChunkIterator.java:12)
- [ ] `RingChunkIterator.RingChunkIterator(2)`: its class isn't translated (it implements `ChunkIterator`) (org/popcraft/chunky/iterator/RingChunkIterator.java:21)
- [ ] `RingChunkIterator.RingChunkIterator(1)`: its class isn't translated (it implements `ChunkIterator`) (org/popcraft/chunky/iterator/RingChunkIterator.java:36)
- [ ] `RingChunkIterator.nextRingChunkProgress(1)`: its class isn't translated (it implements `ChunkIterator`) (org/popcraft/chunky/iterator/RingChunkIterator.java:45)
- [ ] `RingChunkIterator.hasNext(0)`: its class isn't translated (it implements `ChunkIterator`) (org/popcraft/chunky/iterator/RingChunkIterator.java:55)
- [ ] `RingChunkIterator.next(0)`: its class isn't translated (it implements `ChunkIterator`) (org/popcraft/chunky/iterator/RingChunkIterator.java:60)
- [ ] `RingChunkIterator.total(0)`: its class isn't translated (it implements `ChunkIterator`) (org/popcraft/chunky/iterator/RingChunkIterator.java:78)
- [ ] `RingChunkIterator.name(0)`: its class isn't translated (it implements `ChunkIterator`) (org/popcraft/chunky/iterator/RingChunkIterator.java:83)

### `RingChunkIterator.RingChunkProgress`: it implements `ChunkIterator`

- [ ] `RingChunkIterator.RingChunkProgress.RingChunkProgress(4)`: its class isn't translated (it implements `ChunkIterator`) (org/popcraft/chunky/iterator/RingChunkIterator.java:100)
- [ ] `RingChunkIterator.RingChunkProgress.hasNext(0)`: its class isn't translated (it implements `ChunkIterator`) (org/popcraft/chunky/iterator/RingChunkIterator.java:113)
- [ ] `RingChunkIterator.RingChunkProgress.next(0)`: its class isn't translated (it implements `ChunkIterator`) (org/popcraft/chunky/iterator/RingChunkIterator.java:118)
- [ ] `RingChunkIterator.RingChunkProgress.total(0)`: its class isn't translated (it implements `ChunkIterator`) (org/popcraft/chunky/iterator/RingChunkIterator.java:152)
- [ ] `RingChunkIterator.RingChunkProgress.name(0)`: its class isn't translated (it implements `ChunkIterator`) (org/popcraft/chunky/iterator/RingChunkIterator.java:157)

### `SpiralChunkIterator`: it implements `ChunkIterator`

- [x] `SpiralChunkIterator.RIGHT` (org/popcraft/chunky/iterator/SpiralChunkIterator.java:8)
- [x] `SpiralChunkIterator.DOWN` (org/popcraft/chunky/iterator/SpiralChunkIterator.java:9)
- [x] `SpiralChunkIterator.LEFT` (org/popcraft/chunky/iterator/SpiralChunkIterator.java:10)
- [x] `SpiralChunkIterator.UP` (org/popcraft/chunky/iterator/SpiralChunkIterator.java:11)
- [ ] `SpiralChunkIterator.SpiralChunkIterator(2)`: its class isn't translated (it implements `ChunkIterator`) (org/popcraft/chunky/iterator/SpiralChunkIterator.java:23)
- [ ] `SpiralChunkIterator.SpiralChunkIterator(1)`: its class isn't translated (it implements `ChunkIterator`) (org/popcraft/chunky/iterator/SpiralChunkIterator.java:85)
- [ ] `SpiralChunkIterator.hasNext(0)`: its class isn't translated (it implements `ChunkIterator`) (org/popcraft/chunky/iterator/SpiralChunkIterator.java:95)
- [ ] `SpiralChunkIterator.next(0)`: its class isn't translated (it implements `ChunkIterator`) (org/popcraft/chunky/iterator/SpiralChunkIterator.java:100)
- [ ] `SpiralChunkIterator.total(0)`: its class isn't translated (it implements `ChunkIterator`) (org/popcraft/chunky/iterator/SpiralChunkIterator.java:139)
- [ ] `SpiralChunkIterator.name(0)`: its class isn't translated (it implements `ChunkIterator`) (org/popcraft/chunky/iterator/SpiralChunkIterator.java:144)

### `StripChunkIterator`: it implements `ChunkIterator`

- [ ] `StripChunkIterator.WIDTH`: it calls Input.tryInteger, which isn't translated (org/popcraft/chunky/iterator/StripChunkIterator.java:9)
- [ ] `StripChunkIterator.StripChunkIterator(2)`: its class isn't translated (it implements `ChunkIterator`) (org/popcraft/chunky/iterator/StripChunkIterator.java:17)
- [ ] `StripChunkIterator.StripChunkIterator(1)`: its class isn't translated (it implements `ChunkIterator`) (org/popcraft/chunky/iterator/StripChunkIterator.java:22)
- [ ] `StripChunkIterator.hasNext(0)`: its class isn't translated (it implements `ChunkIterator`) (org/popcraft/chunky/iterator/StripChunkIterator.java:32)
- [ ] `StripChunkIterator.next(0)`: its class isn't translated (it implements `ChunkIterator`) (org/popcraft/chunky/iterator/StripChunkIterator.java:37)
- [ ] `StripChunkIterator.total(0)`: its class isn't translated (it implements `ChunkIterator`) (org/popcraft/chunky/iterator/StripChunkIterator.java:50)
- [ ] `StripChunkIterator.name(0)`: its class isn't translated (it implements `ChunkIterator`) (org/popcraft/chunky/iterator/StripChunkIterator.java:55)

### `WorldChunkIterator`: it implements `ChunkIterator`

- [ ] `WorldChunkIterator.WorldChunkIterator(1)`: its class isn't translated (it implements `ChunkIterator`) (org/popcraft/chunky/iterator/WorldChunkIterator.java:45)
- [ ] `WorldChunkIterator.hasNext(0)`: its class isn't translated (it implements `ChunkIterator`) (org/popcraft/chunky/iterator/WorldChunkIterator.java:71)
- [ ] `WorldChunkIterator.next(0)`: its class isn't translated (it implements `ChunkIterator`) (org/popcraft/chunky/iterator/WorldChunkIterator.java:76)
- [ ] `WorldChunkIterator.total(0)`: its class isn't translated (it implements `ChunkIterator`) (org/popcraft/chunky/iterator/WorldChunkIterator.java:84)
- [ ] `WorldChunkIterator.name(0)`: its class isn't translated (it implements `ChunkIterator`) (org/popcraft/chunky/iterator/WorldChunkIterator.java:89)
- [ ] `WorldChunkIterator.process(0)`: its class isn't translated (it implements `ChunkIterator`) (org/popcraft/chunky/iterator/WorldChunkIterator.java:94)
- [ ] `WorldChunkIterator.isInBounds(1)`: its class isn't translated (it implements `ChunkIterator`) (org/popcraft/chunky/iterator/WorldChunkIterator.java:208)
- [ ] `WorldChunkIterator.addChunk(2)`: its class isn't translated (it implements `ChunkIterator`) (org/popcraft/chunky/iterator/WorldChunkIterator.java:215)
- [ ] `WorldChunkIterator.neighbors(1)`: it uses ChunkCoordinate, which isn't translated (it implements `Comparable`) (org/popcraft/chunky/iterator/WorldChunkIterator.java:221)

### `BossBarTaskFinishListener`: it implements `Consumer` (java.util.function)

- [ ] `BossBarTaskFinishListener.BossBarTaskFinishListener(1)`: its class isn't translated (it implements `Consumer` (java.util.function)) (org/popcraft/chunky/listeners/bossbar/BossBarTaskFinishListener.java:14)
- [ ] `BossBarTaskFinishListener.accept(1)`: its class isn't translated (it implements `Consumer` (java.util.function)) (org/popcraft/chunky/listeners/bossbar/BossBarTaskFinishListener.java:18)

### `BossBarTaskUpdateListener`: it implements `Consumer` (java.util.function)

- [ ] `BossBarTaskUpdateListener.BossBarTaskUpdateListener(1)`: its class isn't translated (it implements `Consumer` (java.util.function)) (org/popcraft/chunky/listeners/bossbar/BossBarTaskUpdateListener.java:24)
- [ ] `BossBarTaskUpdateListener.accept(1)`: its class isn't translated (it implements `Consumer` (java.util.function)) (org/popcraft/chunky/listeners/bossbar/BossBarTaskUpdateListener.java:28)
- [ ] `BossBarTaskUpdateListener.hasBossBarPermission(1)`: it uses Minecraft's `ServerPlayer` (org/popcraft/chunky/listeners/bossbar/BossBarTaskUpdateListener.java:72)
- [ ] `BossBarTaskUpdateListener.createNewBossBar(1)`: its class isn't translated (it implements `Consumer` (java.util.function)) (org/popcraft/chunky/listeners/bossbar/BossBarTaskUpdateListener.java:76)
- [ ] `BossBarTaskUpdateListener.bossBarColor(1)`: it uses Minecraft's `BossBarColor` (org/popcraft/chunky/listeners/bossbar/BossBarTaskUpdateListener.java:86)
- [ ] `ChunkHolderAccess`: a Mixin class: its handlers are the injections, in the bundle's helpers (org/popcraft/chunky/listeners/bossbar/BossBarTaskUpdateListener.java:9)
- [ ] `ChunkHolderMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (org/popcraft/chunky/listeners/bossbar/BossBarTaskUpdateListener.java:11)
- [ ] `ChunkMapMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (org/popcraft/chunky/listeners/bossbar/BossBarTaskUpdateListener.java:16)
- [ ] `ChunkMapPatchMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (org/popcraft/chunky/listeners/bossbar/BossBarTaskUpdateListener.java:15)

### `ChunkyMixinPlugin`: it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)

- [x] `ChunkyMixinPlugin.BOOST_UNLOAD_PRIORITY` (org/popcraft/chunky/mixin/ChunkyMixinPlugin.java:11)
- [x] `ChunkyMixinPlugin.UNLOAD_PRIORITY_MIXINS` (org/popcraft/chunky/mixin/ChunkyMixinPlugin.java:12)
- [ ] `ChunkyMixinPlugin.onLoad(1)`: its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)) (org/popcraft/chunky/mixin/ChunkyMixinPlugin.java:17)
- [ ] `ChunkyMixinPlugin.getRefMapperConfig(0)`: its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)) (org/popcraft/chunky/mixin/ChunkyMixinPlugin.java:21)
- [ ] `ChunkyMixinPlugin.shouldApplyMixin(2)`: its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)) (org/popcraft/chunky/mixin/ChunkyMixinPlugin.java:25)
- [ ] `ChunkyMixinPlugin.acceptTargets(2)`: its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)) (org/popcraft/chunky/mixin/ChunkyMixinPlugin.java:29)
- [ ] `ChunkyMixinPlugin.getMixins(0)`: its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)) (org/popcraft/chunky/mixin/ChunkyMixinPlugin.java:32)
- [ ] `ChunkyMixinPlugin.preApply(4)`: its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)) (org/popcraft/chunky/mixin/ChunkyMixinPlugin.java:36)
- [ ] `ChunkyMixinPlugin.postApply(4)`: its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)) (org/popcraft/chunky/mixin/ChunkyMixinPlugin.java:39)
- [ ] `IntegratedServerMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (org/popcraft/chunky/mixin/ChunkyMixinPlugin.java:12)
- [ ] `MinecraftServerAccess`: a Mixin class: its handlers are the injections, in the bundle's helpers (org/popcraft/chunky/mixin/ChunkyMixinPlugin.java:8)
- [ ] `MinecraftServerMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (org/popcraft/chunky/mixin/ChunkyMixinPlugin.java:18)
- [ ] `ServerChunkCacheMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (org/popcraft/chunky/mixin/ChunkyMixinPlugin.java:13)
- [ ] `ServerLevelMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (org/popcraft/chunky/mixin/ChunkyMixinPlugin.java:10)

### `ByteArrayTag`: it extends `Tag`

- [ ] `ByteArrayTag.ByteArrayTag(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/ByteArrayTag.java:11)
- [ ] `ByteArrayTag.ByteArrayTag(2)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/ByteArrayTag.java:15)
- [ ] `ByteArrayTag.read(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/ByteArrayTag.java:20)
- [ ] `ByteArrayTag.skip(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/ByteArrayTag.java:27)
- [ ] `ByteArrayTag.write(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/ByteArrayTag.java:33)
- [ ] `ByteArrayTag.search(3)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/ByteArrayTag.java:39)
- [ ] `ByteArrayTag.type(0)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/ByteArrayTag.java:45)
- [ ] `ByteArrayTag.typeName(0)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/ByteArrayTag.java:50)
- [ ] `ByteArrayTag.print(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/ByteArrayTag.java:55)
- [ ] `ByteArrayTag.value(0)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/ByteArrayTag.java:60)
- [ ] `ByteArrayTag.value(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/ByteArrayTag.java:64)

### `ByteTag`: it extends `Tag`

- [ ] `ByteTag.ByteTag(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/ByteTag.java:10)
- [ ] `ByteTag.ByteTag(2)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/ByteTag.java:14)
- [ ] `ByteTag.read(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/ByteTag.java:19)
- [ ] `ByteTag.skip(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/ByteTag.java:24)
- [ ] `ByteTag.write(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/ByteTag.java:29)
- [ ] `ByteTag.search(3)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/ByteTag.java:34)
- [ ] `ByteTag.type(0)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/ByteTag.java:40)
- [ ] `ByteTag.typeName(0)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/ByteTag.java:45)
- [ ] `ByteTag.print(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/ByteTag.java:50)
- [ ] `ByteTag.value(0)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/ByteTag.java:55)
- [ ] `ByteTag.value(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/ByteTag.java:59)

### `CompoundTag`: it extends `Tag`

- [ ] `CompoundTag.CompoundTag(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/CompoundTag.java:13)
- [ ] `CompoundTag.CompoundTag(2)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/CompoundTag.java:17)
- [ ] `CompoundTag.read(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/CompoundTag.java:22)
- [ ] `CompoundTag.skip(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/CompoundTag.java:32)
- [ ] `CompoundTag.write(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/CompoundTag.java:38)
- [ ] `CompoundTag.search(3)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/CompoundTag.java:47)
- [ ] `CompoundTag.type(0)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/CompoundTag.java:56)
- [ ] `CompoundTag.typeName(0)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/CompoundTag.java:61)
- [ ] `CompoundTag.print(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/CompoundTag.java:66)
- [ ] `CompoundTag.get(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/CompoundTag.java:82)
- [ ] `CompoundTag.getByteArray(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/CompoundTag.java:86)
- [ ] `CompoundTag.getByte(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/CompoundTag.java:90)
- [ ] `CompoundTag.getCompound(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/CompoundTag.java:94)
- [ ] `CompoundTag.getDouble(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/CompoundTag.java:98)
- [ ] `CompoundTag.getFloat(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/CompoundTag.java:102)
- [ ] `CompoundTag.getIntArray(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/CompoundTag.java:106)
- [ ] `CompoundTag.getInt(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/CompoundTag.java:110)
- [ ] `CompoundTag.getList(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/CompoundTag.java:114)
- [ ] `CompoundTag.getLongArray(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/CompoundTag.java:118)
- [ ] `CompoundTag.getLong(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/CompoundTag.java:122)
- [ ] `CompoundTag.getShort(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/CompoundTag.java:126)
- [ ] `CompoundTag.getString(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/CompoundTag.java:130)
- [ ] `CompoundTag.put(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/CompoundTag.java:134)
- [ ] `CompoundTag.remove(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/CompoundTag.java:138)

### `DoubleTag`: it extends `Tag`

- [ ] `DoubleTag.DoubleTag(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/DoubleTag.java:10)
- [ ] `DoubleTag.DoubleTag(2)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/DoubleTag.java:14)
- [ ] `DoubleTag.read(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/DoubleTag.java:19)
- [ ] `DoubleTag.skip(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/DoubleTag.java:24)
- [ ] `DoubleTag.write(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/DoubleTag.java:29)
- [ ] `DoubleTag.search(3)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/DoubleTag.java:34)
- [ ] `DoubleTag.type(0)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/DoubleTag.java:40)
- [ ] `DoubleTag.typeName(0)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/DoubleTag.java:45)
- [ ] `DoubleTag.print(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/DoubleTag.java:50)
- [ ] `DoubleTag.value(0)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/DoubleTag.java:55)
- [ ] `DoubleTag.value(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/DoubleTag.java:59)

### `EndTag`: it extends `Tag`

- [ ] `EndTag.EndTag(0)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/EndTag.java:8)
- [ ] `EndTag.read(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/EndTag.java:12)
- [ ] `EndTag.skip(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/EndTag.java:16)
- [ ] `EndTag.write(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/EndTag.java:20)
- [ ] `EndTag.search(3)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/EndTag.java:24)
- [ ] `EndTag.type(0)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/EndTag.java:30)
- [ ] `EndTag.typeName(0)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/EndTag.java:35)
- [ ] `EndTag.print(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/EndTag.java:40)

### `FloatTag`: it extends `Tag`

- [ ] `FloatTag.FloatTag(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/FloatTag.java:10)
- [ ] `FloatTag.FloatTag(2)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/FloatTag.java:14)
- [ ] `FloatTag.read(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/FloatTag.java:19)
- [ ] `FloatTag.skip(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/FloatTag.java:24)
- [ ] `FloatTag.write(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/FloatTag.java:29)
- [ ] `FloatTag.search(3)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/FloatTag.java:34)
- [ ] `FloatTag.type(0)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/FloatTag.java:40)
- [ ] `FloatTag.typeName(0)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/FloatTag.java:45)
- [ ] `FloatTag.print(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/FloatTag.java:50)
- [ ] `FloatTag.value(0)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/FloatTag.java:55)
- [ ] `FloatTag.value(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/FloatTag.java:59)

### `IntArrayTag`: it extends `Tag`

- [ ] `IntArrayTag.IntArrayTag(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/IntArrayTag.java:12)
- [ ] `IntArrayTag.IntArrayTag(2)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/IntArrayTag.java:16)
- [ ] `IntArrayTag.read(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/IntArrayTag.java:21)
- [ ] `IntArrayTag.skip(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/IntArrayTag.java:30)
- [ ] `IntArrayTag.write(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/IntArrayTag.java:36)
- [ ] `IntArrayTag.search(3)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/IntArrayTag.java:46)
- [ ] `IntArrayTag.type(0)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/IntArrayTag.java:52)
- [ ] `IntArrayTag.typeName(0)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/IntArrayTag.java:57)
- [ ] `IntArrayTag.print(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/IntArrayTag.java:62)
- [ ] `IntArrayTag.value(0)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/IntArrayTag.java:67)
- [ ] `IntArrayTag.value(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/IntArrayTag.java:71)

### `IntTag`: it extends `Tag`

- [ ] `IntTag.IntTag(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/IntTag.java:10)
- [ ] `IntTag.IntTag(2)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/IntTag.java:14)
- [ ] `IntTag.read(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/IntTag.java:19)
- [ ] `IntTag.skip(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/IntTag.java:24)
- [ ] `IntTag.write(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/IntTag.java:29)
- [ ] `IntTag.search(3)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/IntTag.java:34)
- [ ] `IntTag.type(0)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/IntTag.java:40)
- [ ] `IntTag.typeName(0)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/IntTag.java:45)
- [ ] `IntTag.print(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/IntTag.java:50)
- [ ] `IntTag.value(0)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/IntTag.java:55)
- [ ] `IntTag.value(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/IntTag.java:59)

### `ListTag`: it extends `Tag`

- [ ] `ListTag.ListTag(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/ListTag.java:13)
- [ ] `ListTag.ListTag(3)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/ListTag.java:17)
- [ ] `ListTag.read(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/ListTag.java:23)
- [ ] `ListTag.skip(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/ListTag.java:36)
- [ ] `ListTag.write(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/ListTag.java:57)
- [ ] `ListTag.search(3)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/ListTag.java:68)
- [ ] `ListTag.type(0)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/ListTag.java:74)
- [ ] `ListTag.typeName(0)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/ListTag.java:79)
- [ ] `ListTag.print(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/ListTag.java:84)
- [ ] `ListTag.value(0)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/ListTag.java:100)
- [ ] `ListTag.value(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/ListTag.java:104)

### `LongArrayTag`: it extends `Tag`

- [ ] `LongArrayTag.LongArrayTag(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/LongArrayTag.java:12)
- [ ] `LongArrayTag.LongArrayTag(2)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/LongArrayTag.java:16)
- [ ] `LongArrayTag.read(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/LongArrayTag.java:21)
- [ ] `LongArrayTag.skip(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/LongArrayTag.java:30)
- [ ] `LongArrayTag.write(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/LongArrayTag.java:36)
- [ ] `LongArrayTag.search(3)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/LongArrayTag.java:46)
- [ ] `LongArrayTag.type(0)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/LongArrayTag.java:52)
- [ ] `LongArrayTag.typeName(0)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/LongArrayTag.java:57)
- [ ] `LongArrayTag.print(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/LongArrayTag.java:62)
- [ ] `LongArrayTag.value(0)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/LongArrayTag.java:67)
- [ ] `LongArrayTag.value(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/LongArrayTag.java:71)

### `LongTag`: it extends `Tag`

- [ ] `LongTag.LongTag(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/LongTag.java:10)
- [ ] `LongTag.LongTag(2)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/LongTag.java:14)
- [ ] `LongTag.read(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/LongTag.java:19)
- [ ] `LongTag.skip(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/LongTag.java:24)
- [ ] `LongTag.write(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/LongTag.java:29)
- [ ] `LongTag.search(3)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/LongTag.java:34)
- [ ] `LongTag.type(0)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/LongTag.java:40)
- [ ] `LongTag.typeName(0)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/LongTag.java:45)
- [ ] `LongTag.print(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/LongTag.java:50)
- [ ] `LongTag.value(0)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/LongTag.java:55)
- [ ] `LongTag.value(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/LongTag.java:59)

### `NBT`

- [x] `NBT.NBT(0)` (org/popcraft/chunky/nbt/NBT.java:14)
- [ ] `NBT.load(1)`: it uses `InputStream` (java.io) (org/popcraft/chunky/nbt/NBT.java:17)
- [ ] `NBT.save(2)`: it uses `OutputStream` (java.io) (org/popcraft/chunky/nbt/NBT.java:22)
- [ ] `NBT.loadCompressed(1)`: it uses `InputStream` (java.io) (org/popcraft/chunky/nbt/NBT.java:27)
- [ ] `NBT.saveCompressed(2)`: it uses `OutputStream` (java.io) (org/popcraft/chunky/nbt/NBT.java:34)

### `ShortTag`: it extends `Tag`

- [ ] `ShortTag.ShortTag(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/ShortTag.java:10)
- [ ] `ShortTag.ShortTag(2)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/ShortTag.java:14)
- [ ] `ShortTag.read(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/ShortTag.java:19)
- [ ] `ShortTag.skip(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/ShortTag.java:24)
- [ ] `ShortTag.write(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/ShortTag.java:29)
- [ ] `ShortTag.search(3)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/ShortTag.java:34)
- [ ] `ShortTag.type(0)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/ShortTag.java:40)
- [ ] `ShortTag.typeName(0)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/ShortTag.java:45)
- [ ] `ShortTag.print(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/ShortTag.java:50)
- [ ] `ShortTag.value(0)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/ShortTag.java:55)
- [ ] `ShortTag.value(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/ShortTag.java:59)

### `StringTag`: it extends `Tag`

- [ ] `StringTag.StringTag(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/StringTag.java:10)
- [ ] `StringTag.StringTag(2)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/StringTag.java:14)
- [ ] `StringTag.read(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/StringTag.java:19)
- [ ] `StringTag.skip(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/StringTag.java:24)
- [ ] `StringTag.write(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/StringTag.java:30)
- [ ] `StringTag.search(3)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/StringTag.java:35)
- [ ] `StringTag.type(0)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/StringTag.java:41)
- [ ] `StringTag.typeName(0)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/StringTag.java:46)
- [ ] `StringTag.print(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/StringTag.java:51)
- [ ] `StringTag.value(0)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/StringTag.java:56)
- [ ] `StringTag.value(1)`: its class isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/StringTag.java:60)

### `Tag`

- [x] `Tag.INDENT` (org/popcraft/chunky/nbt/Tag.java:10)
- [x] `Tag.Tag(1)` (org/popcraft/chunky/nbt/Tag.java:13)
- [ ] `Tag.load(1)`: it uses `DataInput` (java.io) (org/popcraft/chunky/nbt/Tag.java:17)
- [ ] `Tag.pass(1)`: it uses `DataInput` (java.io) (org/popcraft/chunky/nbt/Tag.java:29)
- [ ] `Tag.skip(2)`: it uses `DataInput` (java.io) (org/popcraft/chunky/nbt/Tag.java:41)
- [ ] `Tag.save(2)`: it uses `DataOutput` (java.io) (org/popcraft/chunky/nbt/Tag.java:49)
- [ ] `Tag.find(3)`: it uses `DataInput` (java.io) (org/popcraft/chunky/nbt/Tag.java:58)
- [ ] `Tag.findMany(2)`: it uses CompoundTag, which isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/Tag.java:74)
- [ ] `Tag.create(2)`: it creates a EndTag, which isn't translated (it extends `Tag`) (org/popcraft/chunky/nbt/Tag.java:99)
- [x] `Tag.name(0)` (org/popcraft/chunky/nbt/Tag.java:118)
- [ ] `Tag.read(1)`: it has no body (org/popcraft/chunky/nbt/Tag.java:122)
- [ ] `Tag.skip(1)`: it has no body (org/popcraft/chunky/nbt/Tag.java:124)
- [ ] `Tag.write(1)`: it has no body (org/popcraft/chunky/nbt/Tag.java:126)
- [ ] `Tag.search(3)`: it has no body (org/popcraft/chunky/nbt/Tag.java:128)
- [ ] `Tag.type(0)`: it has no body (org/popcraft/chunky/nbt/Tag.java:130)
- [ ] `Tag.typeName(0)`: it has no body (org/popcraft/chunky/nbt/Tag.java:132)
- [ ] `Tag.print(1)`: it has no body (org/popcraft/chunky/nbt/Tag.java:134)
- [ ] `Tag.toString(0)`: it calls Tag.print, which isn't translated (org/popcraft/chunky/nbt/Tag.java:136)

### `Tag.SkipTags`

- [ ] `Tag.SkipTags.TAGS`: a Tag with no value (null in Java) (org/popcraft/chunky/nbt/Tag.java:142)

### `TagType`

- [x] `TagType.END` (org/popcraft/chunky/nbt/TagType.java:4)
- [x] `TagType.BYTE` (org/popcraft/chunky/nbt/TagType.java:5)
- [x] `TagType.SHORT` (org/popcraft/chunky/nbt/TagType.java:6)
- [x] `TagType.INT` (org/popcraft/chunky/nbt/TagType.java:7)
- [x] `TagType.LONG` (org/popcraft/chunky/nbt/TagType.java:8)
- [x] `TagType.FLOAT` (org/popcraft/chunky/nbt/TagType.java:9)
- [x] `TagType.DOUBLE` (org/popcraft/chunky/nbt/TagType.java:10)
- [x] `TagType.BYTE_ARRAY` (org/popcraft/chunky/nbt/TagType.java:11)
- [x] `TagType.STRING` (org/popcraft/chunky/nbt/TagType.java:12)
- [x] `TagType.LIST` (org/popcraft/chunky/nbt/TagType.java:13)
- [x] `TagType.COMPOUND` (org/popcraft/chunky/nbt/TagType.java:14)
- [x] `TagType.INT_ARRAY` (org/popcraft/chunky/nbt/TagType.java:15)
- [x] `TagType.LONG_ARRAY` (org/popcraft/chunky/nbt/TagType.java:16)
- [x] `TagType.TagType(0)` (org/popcraft/chunky/nbt/TagType.java:18)

### `Chunk`

- [x] `Chunk.Chunk(4)` (org/popcraft/chunky/nbt/util/Chunk.java:11)
- [x] `Chunk.getData(0)` (org/popcraft/chunky/nbt/util/Chunk.java:18)
- [x] `Chunk.getX(0)` (org/popcraft/chunky/nbt/util/Chunk.java:22)
- [x] `Chunk.setX(1)` (org/popcraft/chunky/nbt/util/Chunk.java:26)
- [x] `Chunk.getZ(0)` (org/popcraft/chunky/nbt/util/Chunk.java:30)
- [x] `Chunk.setZ(1)` (org/popcraft/chunky/nbt/util/Chunk.java:34)
- [x] `Chunk.setData(1)` (org/popcraft/chunky/nbt/util/Chunk.java:38)
- [x] `Chunk.getLastModified(0)` (org/popcraft/chunky/nbt/util/Chunk.java:42)
- [x] `Chunk.setLastModified(1)` (org/popcraft/chunky/nbt/util/Chunk.java:46)

### `ChunkFilter`

- [x] `ChunkFilter.ChunkFilter(3)` (org/popcraft/chunky/nbt/util/ChunkFilter.java:10)
- [x] `ChunkFilter.of(2)` (org/popcraft/chunky/nbt/util/ChunkFilter.java:16)
- [ ] `ChunkFilter.ofTags(1)`: a Map or Set keyed by a String[] (org/popcraft/chunky/nbt/util/ChunkFilter.java:20)
- [x] `ChunkFilter.getType(0)` (org/popcraft/chunky/nbt/util/ChunkFilter.java:24)
- [x] `ChunkFilter.getName(0)` (org/popcraft/chunky/nbt/util/ChunkFilter.java:28)
- [x] `ChunkFilter.hasTagNames(0)` (org/popcraft/chunky/nbt/util/ChunkFilter.java:32)
- [x] `ChunkFilter.getTagNames(0)` (org/popcraft/chunky/nbt/util/ChunkFilter.java:36)

### `ChunkPos`

- [x] `ChunkPos.ChunkPos(2)` (org/popcraft/chunky/nbt/util/ChunkPos.java:3)
- [x] `ChunkPos.of(2)` (org/popcraft/chunky/nbt/util/ChunkPos.java:4)
- [x] `ChunkPos.x(0)` (org/popcraft/chunky/nbt/util/ChunkPos.java:3)
- [x] `ChunkPos.z(0)` (org/popcraft/chunky/nbt/util/ChunkPos.java:3)

### `CompressionType`

- [x] `CompressionType.GZIP` (org/popcraft/chunky/nbt/util/CompressionType.java:4)
- [x] `CompressionType.ZLIB` (org/popcraft/chunky/nbt/util/CompressionType.java:5)
- [x] `CompressionType.NONE` (org/popcraft/chunky/nbt/util/CompressionType.java:6)
- [x] `CompressionType.LZ4` (org/popcraft/chunky/nbt/util/CompressionType.java:7)
- [x] `CompressionType.EXTERNAL_FLAG` (org/popcraft/chunky/nbt/util/CompressionType.java:8)
- [x] `CompressionType.CompressionType(0)` (org/popcraft/chunky/nbt/util/CompressionType.java:10)

### `FastBufferedInputStream`: it extends `InputStream` (java.io)

- [x] `FastBufferedInputStream.BUFFER_SIZE` (org/popcraft/chunky/nbt/util/FastBufferedInputStream.java:7)
- [ ] `FastBufferedInputStream.FastBufferedInputStream(1)`: its class isn't translated (it extends `InputStream` (java.io)) (org/popcraft/chunky/nbt/util/FastBufferedInputStream.java:13)
- [ ] `FastBufferedInputStream.read(0)`: its class isn't translated (it extends `InputStream` (java.io)) (org/popcraft/chunky/nbt/util/FastBufferedInputStream.java:17)
- [ ] `FastBufferedInputStream.read(3)`: its class isn't translated (it extends `InputStream` (java.io)) (org/popcraft/chunky/nbt/util/FastBufferedInputStream.java:22)
- [ ] `FastBufferedInputStream.skip(1)`: its class isn't translated (it extends `InputStream` (java.io)) (org/popcraft/chunky/nbt/util/FastBufferedInputStream.java:47)
- [ ] `FastBufferedInputStream.available(0)`: its class isn't translated (it extends `InputStream` (java.io)) (org/popcraft/chunky/nbt/util/FastBufferedInputStream.java:62)
- [ ] `FastBufferedInputStream.close(0)`: its class isn't translated (it extends `InputStream` (java.io)) (org/popcraft/chunky/nbt/util/FastBufferedInputStream.java:67)
- [ ] `FastBufferedInputStream.fill(0)`: its class isn't translated (it extends `InputStream` (java.io)) (org/popcraft/chunky/nbt/util/FastBufferedInputStream.java:72)

### `Lz4Decompressor`

- [x] `Lz4Decompressor.Lz4Decompressor(0)` (org/popcraft/chunky/nbt/util/Lz4Decompressor.java:7)
- [ ] `Lz4Decompressor.decompress(1)`: it uses `InputStream` (java.io) (org/popcraft/chunky/nbt/util/Lz4Decompressor.java:10)

### `RegionFile`: a Map or Set keyed by a Chunk

- [x] `RegionFile.ENTRIES` (org/popcraft/chunky/nbt/util/RegionFile.java:24)
- [x] `RegionFile.SECTOR_SIZE` (org/popcraft/chunky/nbt/util/RegionFile.java:25)
- [ ] `RegionFile.RegionFile(1)`: its class isn't translated (a Map or Set keyed by a Chunk) (org/popcraft/chunky/nbt/util/RegionFile.java:33)
- [ ] `RegionFile.RegionFile(2)`: its class isn't translated (a Map or Set keyed by a Chunk) (org/popcraft/chunky/nbt/util/RegionFile.java:37)
- [ ] `RegionFile.RegionFile(4)`: its class isn't translated (a Map or Set keyed by a Chunk) (org/popcraft/chunky/nbt/util/RegionFile.java:41)
- [ ] `RegionFile.RegionFile(2)`: its class isn't translated (a Map or Set keyed by a Chunk) (org/popcraft/chunky/nbt/util/RegionFile.java:45)
- [ ] `RegionFile.RegionFile(5)`: its body couldn't be read: a labelled statement (org/popcraft/chunky/nbt/util/RegionFile.java:49)
- [ ] `RegionFile.decompress(2)`: its class isn't translated (a Map or Set keyed by a Chunk) (org/popcraft/chunky/nbt/util/RegionFile.java:180)
- [ ] `RegionFile.getChunks(0)`: its class isn't translated (a Map or Set keyed by a Chunk) (org/popcraft/chunky/nbt/util/RegionFile.java:193)
- [ ] `RegionFile.getChunk(2)`: its class isn't translated (a Map or Set keyed by a Chunk) (org/popcraft/chunky/nbt/util/RegionFile.java:197)
- [ ] `RegionFile.getHeader(0)`: its class isn't translated (a Map or Set keyed by a Chunk) (org/popcraft/chunky/nbt/util/RegionFile.java:201)
- [ ] `RegionFile.isInvalid(2)`: its class isn't translated (a Map or Set keyed by a Chunk) (org/popcraft/chunky/nbt/util/RegionFile.java:205)
- [ ] `RegionFile.isFullyRead(0)`: its class isn't translated (a Map or Set keyed by a Chunk) (org/popcraft/chunky/nbt/util/RegionFile.java:209)

### `RegionFile.Header`

- [x] `RegionFile.Header.Header(2)` (org/popcraft/chunky/nbt/util/RegionFile.java:213)
- [x] `RegionFile.Header.hasChunk(2)` (org/popcraft/chunky/nbt/util/RegionFile.java:214)
- [ ] `RegionFile.Header.getTimestamp(2)`: it calls `Integer`.toUnsignedLong (org/popcraft/chunky/nbt/util/RegionFile.java:218)
- [x] `RegionFile.Header.index(2)` (org/popcraft/chunky/nbt/util/RegionFile.java:222)
- [x] `RegionFile.Header.locations(0)` (org/popcraft/chunky/nbt/util/RegionFile.java:213)
- [x] `RegionFile.Header.timestamps(0)` (org/popcraft/chunky/nbt/util/RegionFile.java:213)

### `Border`: it's an interface

- [ ] `Border.getCenter(0)`: it has no body (org/popcraft/chunky/platform/Border.java:6)
- [ ] `Border.getRadiusX(0)`: it has no body (org/popcraft/chunky/platform/Border.java:8)
- [ ] `Border.getRadiusZ(0)`: it has no body (org/popcraft/chunky/platform/Border.java:10)
- [ ] `Border.getShape(0)`: it has no body (org/popcraft/chunky/platform/Border.java:12)

### `Config`: it's an interface

- [ ] `Config.getDirectory(0)`: it has no body (org/popcraft/chunky/platform/Config.java:6)
- [ ] `Config.getDataDirectory(0)`: its class isn't translated (it's an interface) (org/popcraft/chunky/platform/Config.java:8)
- [ ] `Config.getVersion(0)`: it has no body (org/popcraft/chunky/platform/Config.java:12)
- [ ] `Config.getLanguage(0)`: it has no body (org/popcraft/chunky/platform/Config.java:14)
- [ ] `Config.getContinueOnRestart(0)`: it has no body (org/popcraft/chunky/platform/Config.java:16)
- [ ] `Config.isForceLoadExistingChunks(0)`: it has no body (org/popcraft/chunky/platform/Config.java:18)
- [ ] `Config.isSilent(0)`: it has no body (org/popcraft/chunky/platform/Config.java:20)
- [ ] `Config.setSilent(1)`: it has no body (org/popcraft/chunky/platform/Config.java:22)
- [ ] `Config.getUpdateInterval(0)`: it has no body (org/popcraft/chunky/platform/Config.java:24)
- [ ] `Config.setUpdateInterval(1)`: it has no body (org/popcraft/chunky/platform/Config.java:26)
- [ ] `Config.getPlayerPauseCount(0)`: it has no body (org/popcraft/chunky/platform/Config.java:28)
- [ ] `Config.reload(0)`: it has no body (org/popcraft/chunky/platform/Config.java:30)

### `FabricBorder`: it implements `Border`

- [ ] `FabricBorder.FabricBorder(1)`: its class isn't translated (it implements `Border`) (org/popcraft/chunky/platform/FabricBorder.java:9)
- [ ] `FabricBorder.getCenter(0)`: its class isn't translated (it implements `Border`) (org/popcraft/chunky/platform/FabricBorder.java:13)
- [ ] `FabricBorder.getRadiusX(0)`: its class isn't translated (it implements `Border`) (org/popcraft/chunky/platform/FabricBorder.java:18)
- [ ] `FabricBorder.getRadiusZ(0)`: its class isn't translated (it implements `Border`) (org/popcraft/chunky/platform/FabricBorder.java:23)
- [ ] `FabricBorder.getShape(0)`: its class isn't translated (it implements `Border`) (org/popcraft/chunky/platform/FabricBorder.java:28)

### `FabricPlayer`: it extends `FabricSender`

- [ ] `FabricPlayer.FabricPlayer(1)`: its class isn't translated (it extends `FabricSender`) (org/popcraft/chunky/platform/FabricPlayer.java:16)
- [ ] `FabricPlayer.isPlayer(0)`: its class isn't translated (it extends `FabricSender`) (org/popcraft/chunky/platform/FabricPlayer.java:21)
- [ ] `FabricPlayer.getName(0)`: its class isn't translated (it extends `FabricSender`) (org/popcraft/chunky/platform/FabricPlayer.java:26)
- [ ] `FabricPlayer.getWorld(0)`: its class isn't translated (it extends `FabricSender`) (org/popcraft/chunky/platform/FabricPlayer.java:31)
- [ ] `FabricPlayer.getLocation(0)`: its class isn't translated (it extends `FabricSender`) (org/popcraft/chunky/platform/FabricPlayer.java:36)
- [ ] `FabricPlayer.sendMessage(3)`: its class isn't translated (it extends `FabricSender`) (org/popcraft/chunky/platform/FabricPlayer.java:41)
- [ ] `FabricPlayer.getUUID(0)`: its class isn't translated (it extends `FabricSender`) (org/popcraft/chunky/platform/FabricPlayer.java:46)
- [ ] `FabricPlayer.teleport(1)`: its class isn't translated (it extends `FabricSender`) (org/popcraft/chunky/platform/FabricPlayer.java:51)
- [ ] `FabricPlayer.sendActionBar(1)`: its class isn't translated (it extends `FabricSender`) (org/popcraft/chunky/platform/FabricPlayer.java:65)
- [ ] `FabricPlayer.formatColored(1)`: its class isn't translated (it extends `FabricSender`) (org/popcraft/chunky/platform/FabricPlayer.java:70)

### `FabricSender`: it implements `Sender`

- [x] `FabricSender.PERMISSION_MANAGERS` (org/popcraft/chunky/platform/FabricSender.java:21)
- [x] `FabricSender.HAS_PERMISSION_MANAGER` (org/popcraft/chunky/platform/FabricSender.java:22)
- [x] `FabricSender.PERMISSION_IDS` (org/popcraft/chunky/platform/FabricSender.java:23)
- [ ] `FabricSender.FabricSender(1)`: its class isn't translated (it implements `Sender`) (org/popcraft/chunky/platform/FabricSender.java:26)
- [ ] `FabricSender.isPlayer(0)`: its class isn't translated (it implements `Sender`) (org/popcraft/chunky/platform/FabricSender.java:30)
- [ ] `FabricSender.getName(0)`: its class isn't translated (it implements `Sender`) (org/popcraft/chunky/platform/FabricSender.java:35)
- [ ] `FabricSender.getWorld(0)`: its class isn't translated (it implements `Sender`) (org/popcraft/chunky/platform/FabricSender.java:40)
- [ ] `FabricSender.getLocation(0)`: its class isn't translated (it implements `Sender`) (org/popcraft/chunky/platform/FabricSender.java:45)
- [ ] `FabricSender.hasPermission(1)`: its class isn't translated (it implements `Sender`) (org/popcraft/chunky/platform/FabricSender.java:52)
- [ ] `FabricSender.hasPermission(2)`: its class isn't translated (it implements `Sender`) (org/popcraft/chunky/platform/FabricSender.java:57)
- [x] `FabricSender.toIdentifier(1)` (org/popcraft/chunky/platform/FabricSender.java:63)
- [ ] `FabricSender.sendMessage(3)`: its class isn't translated (it implements `Sender`) (org/popcraft/chunky/platform/FabricSender.java:76)

### `FabricServer`: it implements `Server`

- [ ] `FabricServer.FabricServer(2)`: its class isn't translated (it implements `Server`) (org/popcraft/chunky/platform/FabricServer.java:22)
- [ ] `FabricServer.getIntegrations(0)`: its class isn't translated (it implements `Server`) (org/popcraft/chunky/platform/FabricServer.java:27)
- [ ] `FabricServer.getWorld(1)`: its class isn't translated (it implements `Server`) (org/popcraft/chunky/platform/FabricServer.java:32)
- [ ] `FabricServer.getWorlds(0)`: its class isn't translated (it implements `Server`) (org/popcraft/chunky/platform/FabricServer.java:48)
- [ ] `FabricServer.getMaxWorldSize(0)`: its class isn't translated (it implements `Server`) (org/popcraft/chunky/platform/FabricServer.java:55)
- [ ] `FabricServer.getConsole(0)`: its class isn't translated (it implements `Server`) (org/popcraft/chunky/platform/FabricServer.java:60)
- [ ] `FabricServer.getPlayers(0)`: its class isn't translated (it implements `Server`) (org/popcraft/chunky/platform/FabricServer.java:65)
- [ ] `FabricServer.getPlayer(1)`: its class isn't translated (it implements `Server`) (org/popcraft/chunky/platform/FabricServer.java:70)
- [ ] `FabricServer.getConfig(0)`: its class isn't translated (it implements `Server`) (org/popcraft/chunky/platform/FabricServer.java:75)

### `FabricWorld`: it implements `World`

- [ ] `FabricWorld.TICKING_LOAD_DURATION`: it calls Input.tryInteger, which isn't translated (org/popcraft/chunky/platform/FabricWorld.java:42)
- [ ] `FabricWorld.CHUNKY`: it uses Minecraft's `TicketType` (org/popcraft/chunky/platform/FabricWorld.java:43)
- [ ] `FabricWorld.CHUNKY_TICKING`: it uses Minecraft's `TicketType` (org/popcraft/chunky/platform/FabricWorld.java:44)
- [x] `FabricWorld.UPDATE_CHUNK_NBT` (org/popcraft/chunky/platform/FabricWorld.java:45)
- [x] `FabricWorld.FULL_STATE_CONFIRMATION_TIMEOUT_SECONDS` (org/popcraft/chunky/platform/FabricWorld.java:46)
- [ ] `FabricWorld.FabricWorld(1)`: its class isn't translated (it implements `World`) (org/popcraft/chunky/platform/FabricWorld.java:51)
- [ ] `FabricWorld.getName(0)`: its class isn't translated (it implements `World`) (org/popcraft/chunky/platform/FabricWorld.java:57)
- [ ] `FabricWorld.getKey(0)`: its class isn't translated (it implements `World`) (org/popcraft/chunky/platform/FabricWorld.java:62)
- [ ] `FabricWorld.isChunkGenerated(2)`: its class isn't translated (it implements `World`) (org/popcraft/chunky/platform/FabricWorld.java:67)
- [ ] `FabricWorld.getChunkAtAsync(2)`: its class isn't translated (it implements `World`) (org/popcraft/chunky/platform/FabricWorld.java:100)
- [ ] `FabricWorld.getUUID(0)`: its class isn't translated (it implements `World`) (org/popcraft/chunky/platform/FabricWorld.java:135)
- [ ] `FabricWorld.getSeaLevel(0)`: its class isn't translated (it implements `World`) (org/popcraft/chunky/platform/FabricWorld.java:140)
- [ ] `FabricWorld.getSpawn(0)`: its class isn't translated (it implements `World`) (org/popcraft/chunky/platform/FabricWorld.java:145)
- [ ] `FabricWorld.getWorldBorder(0)`: its class isn't translated (it implements `World`) (org/popcraft/chunky/platform/FabricWorld.java:153)
- [ ] `FabricWorld.getElevation(2)`: its class isn't translated (it implements `World`) (org/popcraft/chunky/platform/FabricWorld.java:158)
- [ ] `FabricWorld.getElevationAtAsync(2)`: its class isn't translated (it implements `World`) (org/popcraft/chunky/platform/FabricWorld.java:181)
- [ ] `FabricWorld.getMaxElevation(0)`: its class isn't translated (it implements `World`) (org/popcraft/chunky/platform/FabricWorld.java:186)
- [ ] `FabricWorld.playEffect(2)`: its class isn't translated (it implements `World`) (org/popcraft/chunky/platform/FabricWorld.java:191)
- [ ] `FabricWorld.playSound(2)`: its class isn't translated (it implements `World`) (org/popcraft/chunky/platform/FabricWorld.java:198)
- [ ] `FabricWorld.getDirectory(1)`: its class isn't translated (it implements `World`) (org/popcraft/chunky/platform/FabricWorld.java:209)
- [ ] `FabricWorld.getWorld(0)`: its class isn't translated (it implements `World`) (org/popcraft/chunky/platform/FabricWorld.java:219)

### `GsonConfig`: it implements `Config` (org.popcraft.chunky.platform)

- [x] `GsonConfig.GSON` (org/popcraft/chunky/platform/impl/GsonConfig.java:17)
- [ ] `GsonConfig.GsonConfig(2)`: its class isn't translated (it implements `Config` (org.popcraft.chunky.platform)) (org/popcraft/chunky/platform/impl/GsonConfig.java:22)
- [ ] `GsonConfig.getDirectory(0)`: its class isn't translated (it implements `Config` (org.popcraft.chunky.platform)) (org/popcraft/chunky/platform/impl/GsonConfig.java:34)
- [ ] `GsonConfig.getDataDirectory(0)`: its class isn't translated (it implements `Config` (org.popcraft.chunky.platform)) (org/popcraft/chunky/platform/impl/GsonConfig.java:39)
- [ ] `GsonConfig.getVersion(0)`: its class isn't translated (it implements `Config` (org.popcraft.chunky.platform)) (org/popcraft/chunky/platform/impl/GsonConfig.java:44)
- [ ] `GsonConfig.getLanguage(0)`: its class isn't translated (it implements `Config` (org.popcraft.chunky.platform)) (org/popcraft/chunky/platform/impl/GsonConfig.java:49)
- [ ] `GsonConfig.getContinueOnRestart(0)`: its class isn't translated (it implements `Config` (org.popcraft.chunky.platform)) (org/popcraft/chunky/platform/impl/GsonConfig.java:54)
- [ ] `GsonConfig.isForceLoadExistingChunks(0)`: its class isn't translated (it implements `Config` (org.popcraft.chunky.platform)) (org/popcraft/chunky/platform/impl/GsonConfig.java:59)
- [ ] `GsonConfig.isSilent(0)`: its class isn't translated (it implements `Config` (org.popcraft.chunky.platform)) (org/popcraft/chunky/platform/impl/GsonConfig.java:64)
- [ ] `GsonConfig.setSilent(1)`: its class isn't translated (it implements `Config` (org.popcraft.chunky.platform)) (org/popcraft/chunky/platform/impl/GsonConfig.java:69)
- [ ] `GsonConfig.getUpdateInterval(0)`: its class isn't translated (it implements `Config` (org.popcraft.chunky.platform)) (org/popcraft/chunky/platform/impl/GsonConfig.java:74)
- [ ] `GsonConfig.setUpdateInterval(1)`: its class isn't translated (it implements `Config` (org.popcraft.chunky.platform)) (org/popcraft/chunky/platform/impl/GsonConfig.java:79)
- [ ] `GsonConfig.getPlayerPauseCount(0)`: its class isn't translated (it implements `Config` (org.popcraft.chunky.platform)) (org/popcraft/chunky/platform/impl/GsonConfig.java:84)
- [ ] `GsonConfig.reload(0)`: its class isn't translated (it implements `Config` (org.popcraft.chunky.platform)) (org/popcraft/chunky/platform/impl/GsonConfig.java:89)
- [ ] `GsonConfig.saveConfig(0)`: its class isn't translated (it implements `Config` (org.popcraft.chunky.platform)) (org/popcraft/chunky/platform/impl/GsonConfig.java:98)

### `GsonConfig.ConfigModel`

- [x] `GsonConfig.ConfigModel.getVersion(0)` (org/popcraft/chunky/platform/impl/GsonConfig.java:122)
- [x] `GsonConfig.ConfigModel.setVersion(1)` (org/popcraft/chunky/platform/impl/GsonConfig.java:126)
- [x] `GsonConfig.ConfigModel.getLanguage(0)` (org/popcraft/chunky/platform/impl/GsonConfig.java:130)
- [x] `GsonConfig.ConfigModel.setLanguage(1)` (org/popcraft/chunky/platform/impl/GsonConfig.java:134)
- [x] `GsonConfig.ConfigModel.getContinueOnRestart(0)` (org/popcraft/chunky/platform/impl/GsonConfig.java:138)
- [x] `GsonConfig.ConfigModel.setContinueOnRestart(1)` (org/popcraft/chunky/platform/impl/GsonConfig.java:142)
- [x] `GsonConfig.ConfigModel.getForceLoadExistingChunks(0)` (org/popcraft/chunky/platform/impl/GsonConfig.java:146)
- [x] `GsonConfig.ConfigModel.setForceLoadExistingChunks(1)` (org/popcraft/chunky/platform/impl/GsonConfig.java:150)
- [x] `GsonConfig.ConfigModel.getTasks(0)` (org/popcraft/chunky/platform/impl/GsonConfig.java:154)
- [x] `GsonConfig.ConfigModel.setTasks(1)` (org/popcraft/chunky/platform/impl/GsonConfig.java:158)
- [x] `GsonConfig.ConfigModel.isSilent(0)` (org/popcraft/chunky/platform/impl/GsonConfig.java:162)
- [x] `GsonConfig.ConfigModel.setSilent(1)` (org/popcraft/chunky/platform/impl/GsonConfig.java:166)
- [x] `GsonConfig.ConfigModel.getUpdateInterval(0)` (org/popcraft/chunky/platform/impl/GsonConfig.java:170)
- [x] `GsonConfig.ConfigModel.setUpdateInterval(1)` (org/popcraft/chunky/platform/impl/GsonConfig.java:174)
- [x] `GsonConfig.ConfigModel.getPlayerPauseCount(0)` (org/popcraft/chunky/platform/impl/GsonConfig.java:178)
- [x] `GsonConfig.ConfigModel.setPlayerPauseCount(1)` (org/popcraft/chunky/platform/impl/GsonConfig.java:182)

### `GsonConfig.TaskModel`

- [x] `GsonConfig.TaskModel.getCancelled(0)` (org/popcraft/chunky/platform/impl/GsonConfig.java:198)
- [x] `GsonConfig.TaskModel.setCancelled(1)` (org/popcraft/chunky/platform/impl/GsonConfig.java:202)
- [x] `GsonConfig.TaskModel.getRadius(0)` (org/popcraft/chunky/platform/impl/GsonConfig.java:206)
- [x] `GsonConfig.TaskModel.setRadius(1)` (org/popcraft/chunky/platform/impl/GsonConfig.java:210)
- [x] `GsonConfig.TaskModel.getRadiusZ(0)` (org/popcraft/chunky/platform/impl/GsonConfig.java:214)
- [x] `GsonConfig.TaskModel.setRadiusZ(1)` (org/popcraft/chunky/platform/impl/GsonConfig.java:218)
- [x] `GsonConfig.TaskModel.getCenterX(0)` (org/popcraft/chunky/platform/impl/GsonConfig.java:222)
- [x] `GsonConfig.TaskModel.setCenterX(1)` (org/popcraft/chunky/platform/impl/GsonConfig.java:226)
- [x] `GsonConfig.TaskModel.getCenterZ(0)` (org/popcraft/chunky/platform/impl/GsonConfig.java:230)
- [x] `GsonConfig.TaskModel.setCenterZ(1)` (org/popcraft/chunky/platform/impl/GsonConfig.java:234)
- [x] `GsonConfig.TaskModel.getIterator(0)` (org/popcraft/chunky/platform/impl/GsonConfig.java:238)
- [x] `GsonConfig.TaskModel.setIterator(1)` (org/popcraft/chunky/platform/impl/GsonConfig.java:242)
- [x] `GsonConfig.TaskModel.getShape(0)` (org/popcraft/chunky/platform/impl/GsonConfig.java:246)
- [x] `GsonConfig.TaskModel.setShape(1)` (org/popcraft/chunky/platform/impl/GsonConfig.java:250)
- [x] `GsonConfig.TaskModel.getCount(0)` (org/popcraft/chunky/platform/impl/GsonConfig.java:254)
- [x] `GsonConfig.TaskModel.setCount(1)` (org/popcraft/chunky/platform/impl/GsonConfig.java:258)
- [x] `GsonConfig.TaskModel.getTime(0)` (org/popcraft/chunky/platform/impl/GsonConfig.java:262)
- [x] `GsonConfig.TaskModel.setTime(1)` (org/popcraft/chunky/platform/impl/GsonConfig.java:266)

### `Player`: it's an interface

- [ ] `Player.getUUID(0)`: it has no body (org/popcraft/chunky/platform/Player.java:7)
- [ ] `Player.teleport(1)`: it has no body (org/popcraft/chunky/platform/Player.java:9)
- [ ] `Player.sendActionBar(1)`: it has no body (org/popcraft/chunky/platform/Player.java:11)

### `Sender`: it's an interface

- [ ] `Sender.isPlayer(0)`: it has no body (org/popcraft/chunky/platform/Sender.java:6)
- [ ] `Sender.getName(0)`: it has no body (org/popcraft/chunky/platform/Sender.java:8)
- [ ] `Sender.getWorld(0)`: it has no body (org/popcraft/chunky/platform/Sender.java:10)
- [ ] `Sender.getLocation(0)`: it has no body (org/popcraft/chunky/platform/Sender.java:12)
- [ ] `Sender.hasPermission(1)`: it has no body (org/popcraft/chunky/platform/Sender.java:14)
- [ ] `Sender.sendMessage(3)`: it has no body (org/popcraft/chunky/platform/Sender.java:16)
- [ ] `Sender.sendMessage(2)`: its class isn't translated (it's an interface) (org/popcraft/chunky/platform/Sender.java:18)
- [ ] `Sender.sendMessagePrefixed(2)`: its class isn't translated (it's an interface) (org/popcraft/chunky/platform/Sender.java:22)

### `Server`: it's an interface

- [ ] `Server.getIntegrations(0)`: it has no body (org/popcraft/chunky/platform/Server.java:10)
- [ ] `Server.getWorld(1)`: it has no body (org/popcraft/chunky/platform/Server.java:12)
- [ ] `Server.getWorlds(0)`: it has no body (org/popcraft/chunky/platform/Server.java:14)
- [ ] `Server.getMaxWorldSize(0)`: it has no body (org/popcraft/chunky/platform/Server.java:16)
- [ ] `Server.getConsole(0)`: it has no body (org/popcraft/chunky/platform/Server.java:18)
- [ ] `Server.getPlayers(0)`: it has no body (org/popcraft/chunky/platform/Server.java:20)
- [ ] `Server.getPlayer(1)`: it has no body (org/popcraft/chunky/platform/Server.java:22)
- [ ] `Server.getConfig(0)`: it has no body (org/popcraft/chunky/platform/Server.java:24)

### `Location`: it uses World, which isn't translated (it's an interface)

- [ ] `Location.Location(6)`: its class isn't translated (it uses World, which isn't translated (it's an interface)) (org/popcraft/chunky/platform/util/Location.java:13)
- [ ] `Location.Location(4)`: its class isn't translated (it uses World, which isn't translated (it's an interface)) (org/popcraft/chunky/platform/util/Location.java:22)
- [ ] `Location.toVector(0)`: its class isn't translated (it uses World, which isn't translated (it's an interface)) (org/popcraft/chunky/platform/util/Location.java:29)
- [ ] `Location.add(1)`: its class isn't translated (it uses World, which isn't translated (it's an interface)) (org/popcraft/chunky/platform/util/Location.java:33)
- [ ] `Location.setDirection(1)`: its class isn't translated (it uses World, which isn't translated (it's an interface)) (org/popcraft/chunky/platform/util/Location.java:40)
- [ ] `Location.getWorld(0)`: its class isn't translated (it uses World, which isn't translated (it's an interface)) (org/popcraft/chunky/platform/util/Location.java:59)
- [ ] `Location.setWorld(1)`: its class isn't translated (it uses World, which isn't translated (it's an interface)) (org/popcraft/chunky/platform/util/Location.java:63)
- [ ] `Location.getX(0)`: its class isn't translated (it uses World, which isn't translated (it's an interface)) (org/popcraft/chunky/platform/util/Location.java:67)
- [ ] `Location.setX(1)`: its class isn't translated (it uses World, which isn't translated (it's an interface)) (org/popcraft/chunky/platform/util/Location.java:71)
- [ ] `Location.getY(0)`: its class isn't translated (it uses World, which isn't translated (it's an interface)) (org/popcraft/chunky/platform/util/Location.java:75)
- [ ] `Location.setY(1)`: its class isn't translated (it uses World, which isn't translated (it's an interface)) (org/popcraft/chunky/platform/util/Location.java:79)
- [ ] `Location.getZ(0)`: its class isn't translated (it uses World, which isn't translated (it's an interface)) (org/popcraft/chunky/platform/util/Location.java:83)
- [ ] `Location.setZ(1)`: its class isn't translated (it uses World, which isn't translated (it's an interface)) (org/popcraft/chunky/platform/util/Location.java:87)
- [ ] `Location.getYaw(0)`: its class isn't translated (it uses World, which isn't translated (it's an interface)) (org/popcraft/chunky/platform/util/Location.java:91)
- [ ] `Location.setYaw(1)`: its class isn't translated (it uses World, which isn't translated (it's an interface)) (org/popcraft/chunky/platform/util/Location.java:95)
- [ ] `Location.getPitch(0)`: its class isn't translated (it uses World, which isn't translated (it's an interface)) (org/popcraft/chunky/platform/util/Location.java:99)
- [ ] `Location.setPitch(1)`: its class isn't translated (it uses World, which isn't translated (it's an interface)) (org/popcraft/chunky/platform/util/Location.java:103)

### `Vector2`

- [x] `Vector2.Vector2(2)` (org/popcraft/chunky/platform/util/Vector2.java:7)
- [x] `Vector2.of(2)` (org/popcraft/chunky/platform/util/Vector2.java:12)
- [ ] `Vector2.add(1)`: `this` used as a value (org/popcraft/chunky/platform/util/Vector2.java:16)
- [ ] `Vector2.multiply(1)`: `this` used as a value (org/popcraft/chunky/platform/util/Vector2.java:22)
- [ ] `Vector2.normalize(0)`: `this` used as a value (org/popcraft/chunky/platform/util/Vector2.java:28)
- [x] `Vector2.distance(1)` (org/popcraft/chunky/platform/util/Vector2.java:35)
- [x] `Vector2.distanceSquared(1)` (org/popcraft/chunky/platform/util/Vector2.java:39)
- [x] `Vector2.length(0)` (org/popcraft/chunky/platform/util/Vector2.java:45)
- [x] `Vector2.lengthSquared(0)` (org/popcraft/chunky/platform/util/Vector2.java:49)
- [x] `Vector2.getX(0)` (org/popcraft/chunky/platform/util/Vector2.java:53)
- [x] `Vector2.setX(1)` (org/popcraft/chunky/platform/util/Vector2.java:57)
- [x] `Vector2.getZ(0)` (org/popcraft/chunky/platform/util/Vector2.java:61)
- [x] `Vector2.setZ(1)` (org/popcraft/chunky/platform/util/Vector2.java:65)

### `Vector3`

- [x] `Vector3.Vector3(3)` (org/popcraft/chunky/platform/util/Vector3.java:8)
- [x] `Vector3.of(3)` (org/popcraft/chunky/platform/util/Vector3.java:14)
- [ ] `Vector3.add(1)`: `this` used as a value (org/popcraft/chunky/platform/util/Vector3.java:18)
- [ ] `Vector3.multiply(1)`: `this` used as a value (org/popcraft/chunky/platform/util/Vector3.java:25)
- [ ] `Vector3.normalize(0)`: `this` used as a value (org/popcraft/chunky/platform/util/Vector3.java:32)
- [x] `Vector3.distance(1)` (org/popcraft/chunky/platform/util/Vector3.java:40)
- [x] `Vector3.distanceSquared(1)` (org/popcraft/chunky/platform/util/Vector3.java:44)
- [x] `Vector3.length(0)` (org/popcraft/chunky/platform/util/Vector3.java:51)
- [x] `Vector3.lengthSquared(0)` (org/popcraft/chunky/platform/util/Vector3.java:55)
- [x] `Vector3.getX(0)` (org/popcraft/chunky/platform/util/Vector3.java:59)
- [x] `Vector3.setX(1)` (org/popcraft/chunky/platform/util/Vector3.java:63)
- [x] `Vector3.getY(0)` (org/popcraft/chunky/platform/util/Vector3.java:67)
- [x] `Vector3.setY(1)` (org/popcraft/chunky/platform/util/Vector3.java:71)
- [x] `Vector3.getZ(0)` (org/popcraft/chunky/platform/util/Vector3.java:75)
- [x] `Vector3.setZ(1)` (org/popcraft/chunky/platform/util/Vector3.java:79)

### `World`: it's an interface

- [ ] `World.getName(0)`: it has no body (org/popcraft/chunky/platform/World.java:10)
- [ ] `World.getKey(0)`: it has no body (org/popcraft/chunky/platform/World.java:12)
- [ ] `World.isChunkGenerated(2)`: it has no body (org/popcraft/chunky/platform/World.java:14)
- [ ] `World.getChunkAtAsync(2)`: it has no body (org/popcraft/chunky/platform/World.java:16)
- [ ] `World.getUUID(0)`: it has no body (org/popcraft/chunky/platform/World.java:18)
- [ ] `World.getSeaLevel(0)`: it has no body (org/popcraft/chunky/platform/World.java:20)
- [ ] `World.getSpawn(0)`: it has no body (org/popcraft/chunky/platform/World.java:22)
- [ ] `World.getWorldBorder(0)`: it has no body (org/popcraft/chunky/platform/World.java:24)
- [ ] `World.getElevation(2)`: it has no body (org/popcraft/chunky/platform/World.java:26)
- [ ] `World.getElevationAtAsync(2)`: its class isn't translated (it's an interface) (org/popcraft/chunky/platform/World.java:28)
- [ ] `World.getMaxElevation(0)`: it has no body (org/popcraft/chunky/platform/World.java:32)
- [ ] `World.playEffect(2)`: it has no body (org/popcraft/chunky/platform/World.java:34)
- [ ] `World.playSound(2)`: it has no body (org/popcraft/chunky/platform/World.java:36)
- [ ] `World.getDirectory(1)`: it has no body (org/popcraft/chunky/platform/World.java:38)
- [ ] `World.getEntitiesDirectory(0)`: its class isn't translated (it's an interface) (org/popcraft/chunky/platform/World.java:40)
- [ ] `World.getPOIDirectory(0)`: its class isn't translated (it's an interface) (org/popcraft/chunky/platform/World.java:44)
- [ ] `World.getRegionDirectory(0)`: its class isn't translated (it's an interface) (org/popcraft/chunky/platform/World.java:48)

### `Selection`: it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))

- [x] `Selection.DEFAULT_CENTER_X` (org/popcraft/chunky/Selection.java:10)
- [x] `Selection.DEFAULT_CENTER_Z` (org/popcraft/chunky/Selection.java:11)
- [x] `Selection.DEFAULT_RADIUS` (org/popcraft/chunky/Selection.java:12)
- [ ] `Selection.Selection(9)`: its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))) (org/popcraft/chunky/Selection.java:35)
- [ ] `Selection.builder(2)`: it uses Builder, which isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))) (org/popcraft/chunky/Selection.java:59)
- [ ] `Selection.chunky(0)`: its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))) (org/popcraft/chunky/Selection.java:63)
- [ ] `Selection.world(0)`: its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))) (org/popcraft/chunky/Selection.java:67)
- [ ] `Selection.centerX(0)`: its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))) (org/popcraft/chunky/Selection.java:71)
- [ ] `Selection.centerZ(0)`: its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))) (org/popcraft/chunky/Selection.java:75)
- [ ] `Selection.radiusX(0)`: its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))) (org/popcraft/chunky/Selection.java:79)
- [ ] `Selection.radiusZ(0)`: its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))) (org/popcraft/chunky/Selection.java:83)
- [ ] `Selection.pattern(0)`: its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))) (org/popcraft/chunky/Selection.java:87)
- [ ] `Selection.shape(0)`: its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))) (org/popcraft/chunky/Selection.java:91)
- [ ] `Selection.rateLimit(0)`: its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))) (org/popcraft/chunky/Selection.java:95)
- [ ] `Selection.centerChunkX(0)`: its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))) (org/popcraft/chunky/Selection.java:99)
- [ ] `Selection.centerChunkZ(0)`: its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))) (org/popcraft/chunky/Selection.java:103)
- [ ] `Selection.radiusChunksX(0)`: its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))) (org/popcraft/chunky/Selection.java:107)
- [ ] `Selection.radiusChunksZ(0)`: its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))) (org/popcraft/chunky/Selection.java:111)
- [ ] `Selection.diameterChunksX(0)`: its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))) (org/popcraft/chunky/Selection.java:115)
- [ ] `Selection.diameterChunksZ(0)`: its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))) (org/popcraft/chunky/Selection.java:119)
- [ ] `Selection.centerRegionX(0)`: its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))) (org/popcraft/chunky/Selection.java:123)
- [ ] `Selection.centerRegionZ(0)`: its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))) (org/popcraft/chunky/Selection.java:127)
- [ ] `Selection.radiusRegionsX(0)`: its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))) (org/popcraft/chunky/Selection.java:131)
- [ ] `Selection.radiusRegionsZ(0)`: its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))) (org/popcraft/chunky/Selection.java:135)
- [ ] `Selection.diameterRegionsX(0)`: its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))) (org/popcraft/chunky/Selection.java:139)
- [ ] `Selection.diameterRegionsZ(0)`: its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))) (org/popcraft/chunky/Selection.java:143)

### `Selection.Builder`: it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))

- [ ] `Selection.Builder.Builder(2)`: its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))) (org/popcraft/chunky/Selection.java:158)
- [ ] `Selection.Builder.world(1)`: its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))) (org/popcraft/chunky/Selection.java:163)
- [ ] `Selection.Builder.center(2)`: its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))) (org/popcraft/chunky/Selection.java:168)
- [ ] `Selection.Builder.centerX(1)`: its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))) (org/popcraft/chunky/Selection.java:174)
- [ ] `Selection.Builder.centerZ(1)`: its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))) (org/popcraft/chunky/Selection.java:179)
- [ ] `Selection.Builder.radius(1)`: its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))) (org/popcraft/chunky/Selection.java:184)
- [ ] `Selection.Builder.radiusX(1)`: its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))) (org/popcraft/chunky/Selection.java:190)
- [ ] `Selection.Builder.radiusZ(1)`: its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))) (org/popcraft/chunky/Selection.java:195)
- [ ] `Selection.Builder.pattern(1)`: its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))) (org/popcraft/chunky/Selection.java:200)
- [ ] `Selection.Builder.shape(1)`: its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))) (org/popcraft/chunky/Selection.java:205)
- [ ] `Selection.Builder.rateLimit(1)`: its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))) (org/popcraft/chunky/Selection.java:210)
- [ ] `Selection.Builder.spawn(0)`: its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))) (org/popcraft/chunky/Selection.java:215)
- [ ] `Selection.Builder.worldborder(0)`: its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))) (org/popcraft/chunky/Selection.java:222)
- [ ] `Selection.Builder.build(0)`: its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))) (org/popcraft/chunky/Selection.java:232)

### `AbstractEllipse`: it extends `AbstractShape`

- [ ] `AbstractEllipse.AbstractEllipse(2)`: its class isn't translated (it extends `AbstractShape`) (org/popcraft/chunky/shape/AbstractEllipse.java:7)
- [ ] `AbstractEllipse.radii(0)`: it has no body (org/popcraft/chunky/shape/AbstractEllipse.java:11)

### `AbstractPolygon`: it extends `AbstractShape`

- [ ] `AbstractPolygon.AbstractPolygon(2)`: its class isn't translated (it extends `AbstractShape`) (org/popcraft/chunky/shape/AbstractPolygon.java:8)
- [ ] `AbstractPolygon.points(0)`: it has no body (org/popcraft/chunky/shape/AbstractPolygon.java:12)

### `AbstractShape`: it implements `Shape`

- [ ] `AbstractShape.AbstractShape(2)`: its class isn't translated (it implements `Shape`) (org/popcraft/chunky/shape/AbstractShape.java:14)
- [ ] `AbstractShape.center(0)`: its class isn't translated (it implements `Shape`) (org/popcraft/chunky/shape/AbstractShape.java:32)

### `Circle`: it extends `AbstractEllipse`

- [ ] `Circle.Circle(2)`: its class isn't translated (it extends `AbstractEllipse`) (org/popcraft/chunky/shape/Circle.java:7)
- [ ] `Circle.isBounding(2)`: its class isn't translated (it extends `AbstractEllipse`) (org/popcraft/chunky/shape/Circle.java:11)
- [ ] `Circle.name(0)`: its class isn't translated (it extends `AbstractEllipse`) (org/popcraft/chunky/shape/Circle.java:16)
- [ ] `Circle.radii(0)`: its class isn't translated (it extends `AbstractEllipse`) (org/popcraft/chunky/shape/Circle.java:21)

### `Diamond`: it extends `AbstractPolygon`

- [ ] `Diamond.Diamond(2)`: its class isn't translated (it extends `AbstractPolygon`) (org/popcraft/chunky/shape/Diamond.java:18)
- [ ] `Diamond.points(0)`: its class isn't translated (it extends `AbstractPolygon`) (org/popcraft/chunky/shape/Diamond.java:22)
- [ ] `Diamond.isBounding(2)`: its class isn't translated (it extends `AbstractPolygon`) (org/popcraft/chunky/shape/Diamond.java:27)
- [ ] `Diamond.name(0)`: its class isn't translated (it extends `AbstractPolygon`) (org/popcraft/chunky/shape/Diamond.java:40)

### `Ellipse`: it extends `AbstractEllipse`

- [ ] `Ellipse.Ellipse(2)`: its class isn't translated (it extends `AbstractEllipse`) (org/popcraft/chunky/shape/Ellipse.java:7)
- [ ] `Ellipse.isBounding(2)`: its class isn't translated (it extends `AbstractEllipse`) (org/popcraft/chunky/shape/Ellipse.java:11)
- [ ] `Ellipse.name(0)`: its class isn't translated (it extends `AbstractEllipse`) (org/popcraft/chunky/shape/Ellipse.java:16)
- [ ] `Ellipse.radii(0)`: its class isn't translated (it extends `AbstractEllipse`) (org/popcraft/chunky/shape/Ellipse.java:21)

### `Hexagon`: it extends `AbstractPolygon`

- [ ] `Hexagon.Hexagon(2)`: its class isn't translated (it extends `AbstractPolygon`) (org/popcraft/chunky/shape/Hexagon.java:22)
- [ ] `Hexagon.points(0)`: its class isn't translated (it extends `AbstractPolygon`) (org/popcraft/chunky/shape/Hexagon.java:26)
- [ ] `Hexagon.isBounding(2)`: its class isn't translated (it extends `AbstractPolygon`) (org/popcraft/chunky/shape/Hexagon.java:38)
- [ ] `Hexagon.name(0)`: its class isn't translated (it extends `AbstractPolygon`) (org/popcraft/chunky/shape/Hexagon.java:49)

### `Pentagon`: it extends `AbstractPolygon`

- [ ] `Pentagon.Pentagon(2)`: its class isn't translated (it extends `AbstractPolygon`) (org/popcraft/chunky/shape/Pentagon.java:20)
- [ ] `Pentagon.points(0)`: its class isn't translated (it extends `AbstractPolygon`) (org/popcraft/chunky/shape/Pentagon.java:24)
- [ ] `Pentagon.isBounding(2)`: its class isn't translated (it extends `AbstractPolygon`) (org/popcraft/chunky/shape/Pentagon.java:35)
- [ ] `Pentagon.name(0)`: its class isn't translated (it extends `AbstractPolygon`) (org/popcraft/chunky/shape/Pentagon.java:45)

### `Rectangle`: it extends `AbstractPolygon`

- [ ] `Rectangle.Rectangle(2)`: its class isn't translated (it extends `AbstractPolygon`) (org/popcraft/chunky/shape/Rectangle.java:22)
- [ ] `Rectangle.points(0)`: its class isn't translated (it extends `AbstractPolygon`) (org/popcraft/chunky/shape/Rectangle.java:26)
- [ ] `Rectangle.isBounding(2)`: its class isn't translated (it extends `AbstractPolygon`) (org/popcraft/chunky/shape/Rectangle.java:31)
- [ ] `Rectangle.name(0)`: its class isn't translated (it extends `AbstractPolygon`) (org/popcraft/chunky/shape/Rectangle.java:36)

### `Shape`: it's an interface

- [ ] `Shape.isBounding(2)`: it has no body (org/popcraft/chunky/shape/Shape.java:5)
- [ ] `Shape.name(0)`: its class isn't translated (it's an interface) (org/popcraft/chunky/shape/Shape.java:7)

### `ShapeFactory`

- [ ] `ShapeFactory.custom`: it uses Selection, which isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))) (org/popcraft/chunky/shape/ShapeFactory.java:11)
- [x] `ShapeFactory.ShapeFactory(0)` (org/popcraft/chunky/shape/ShapeFactory.java:13)
- [ ] `ShapeFactory.getShape(1)`: it uses Shape, which isn't translated (it's an interface) (org/popcraft/chunky/shape/ShapeFactory.java:16)
- [ ] `ShapeFactory.getShape(2)`: it uses Shape, which isn't translated (it's an interface) (org/popcraft/chunky/shape/ShapeFactory.java:20)
- [ ] `ShapeFactory.registerCustom(2)`: it uses Selection, which isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))) (org/popcraft/chunky/shape/ShapeFactory.java:34)
- [ ] `ShapeFactory.getCustomTypes(0)`: it uses ShapeFactory.custom, which isn't translated (it uses Selection, which isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface)))) (org/popcraft/chunky/shape/ShapeFactory.java:39)

### `ShapeType`

- [x] `ShapeType.CIRCLE` (org/popcraft/chunky/shape/ShapeType.java:8)
- [x] `ShapeType.DIAMOND` (org/popcraft/chunky/shape/ShapeType.java:9)
- [x] `ShapeType.ELLIPSE` (org/popcraft/chunky/shape/ShapeType.java:10)
- [x] `ShapeType.HEXAGON` (org/popcraft/chunky/shape/ShapeType.java:11)
- [x] `ShapeType.OVAL` (org/popcraft/chunky/shape/ShapeType.java:12)
- [x] `ShapeType.PENTAGON` (org/popcraft/chunky/shape/ShapeType.java:13)
- [x] `ShapeType.RECTANGLE` (org/popcraft/chunky/shape/ShapeType.java:14)
- [x] `ShapeType.SQUARE` (org/popcraft/chunky/shape/ShapeType.java:15)
- [x] `ShapeType.STAR` (org/popcraft/chunky/shape/ShapeType.java:16)
- [x] `ShapeType.TRIANGLE` (org/popcraft/chunky/shape/ShapeType.java:17)
- [x] `ShapeType.DEFAULTS` (org/popcraft/chunky/shape/ShapeType.java:18)
- [x] `ShapeType.ShapeType(0)` (org/popcraft/chunky/shape/ShapeType.java:20)
- [ ] `ShapeType.all(0)`: it calls ShapeFactory.getCustomTypes, which isn't translated (org/popcraft/chunky/shape/ShapeType.java:23)

### `ShapeUtil`

- [x] `ShapeUtil.ShapeUtil(0)` (org/popcraft/chunky/shape/ShapeUtil.java:7)
- [x] `ShapeUtil.insideLine(6)` (org/popcraft/chunky/shape/ShapeUtil.java:10)
- [x] `ShapeUtil.intersection(8)` (org/popcraft/chunky/shape/ShapeUtil.java:14)
- [x] `ShapeUtil.pointOnEllipse(5)` (org/popcraft/chunky/shape/ShapeUtil.java:31)
- [ ] `ShapeUtil.closestPointOnLine(6)`: Optional.orElseThrow (org/popcraft/chunky/shape/ShapeUtil.java:37)
- [x] `ShapeUtil.distanceBetweenPoints(4)` (org/popcraft/chunky/shape/ShapeUtil.java:54)

### `Square`: it extends `AbstractPolygon`

- [ ] `Square.Square(2)`: its class isn't translated (it extends `AbstractPolygon`) (org/popcraft/chunky/shape/Square.java:22)
- [ ] `Square.points(0)`: its class isn't translated (it extends `AbstractPolygon`) (org/popcraft/chunky/shape/Square.java:26)
- [ ] `Square.isBounding(2)`: its class isn't translated (it extends `AbstractPolygon`) (org/popcraft/chunky/shape/Square.java:31)
- [ ] `Square.name(0)`: its class isn't translated (it extends `AbstractPolygon`) (org/popcraft/chunky/shape/Square.java:36)

### `Star`: it extends `AbstractPolygon`

- [ ] `Star.Star(2)`: its class isn't translated (it extends `AbstractPolygon`) (org/popcraft/chunky/shape/Star.java:30)
- [ ] `Star.points(0)`: its class isn't translated (it extends `AbstractPolygon`) (org/popcraft/chunky/shape/Star.java:54)
- [ ] `Star.isBounding(2)`: its class isn't translated (it extends `AbstractPolygon`) (org/popcraft/chunky/shape/Star.java:70)
- [ ] `Star.name(0)`: its class isn't translated (it extends `AbstractPolygon`) (org/popcraft/chunky/shape/Star.java:90)

### `Triangle`: it extends `AbstractPolygon`

- [ ] `Triangle.Triangle(2)`: its class isn't translated (it extends `AbstractPolygon`) (org/popcraft/chunky/shape/Triangle.java:16)
- [ ] `Triangle.points(0)`: its class isn't translated (it extends `AbstractPolygon`) (org/popcraft/chunky/shape/Triangle.java:20)
- [ ] `Triangle.isBounding(2)`: its class isn't translated (it extends `AbstractPolygon`) (org/popcraft/chunky/shape/Triangle.java:25)
- [ ] `Triangle.name(0)`: its class isn't translated (it extends `AbstractPolygon`) (org/popcraft/chunky/shape/Triangle.java:36)

### `InhabitedTimeFilter`: it implements `TrimFilter`

- [ ] `InhabitedTimeFilter.InhabitedTimeFilter(2)`: its class isn't translated (it implements `TrimFilter`) (org/popcraft/chunky/trim/InhabitedTimeFilter.java:6)
- [ ] `InhabitedTimeFilter.InhabitedTimeFilter(1)`: its class isn't translated (it implements `TrimFilter`) (org/popcraft/chunky/trim/InhabitedTimeFilter.java:7)
- [ ] `InhabitedTimeFilter.test(1)`: its class isn't translated (it implements `TrimFilter`) (org/popcraft/chunky/trim/InhabitedTimeFilter.java:11)
- [ ] `InhabitedTimeFilter.readsChunkData(0)`: its class isn't translated (it implements `TrimFilter`) (org/popcraft/chunky/trim/InhabitedTimeFilter.java:16)
- [ ] `InhabitedTimeFilter.sendNotice(1)`: its class isn't translated (it implements `TrimFilter`) (org/popcraft/chunky/trim/InhabitedTimeFilter.java:21)
- [ ] `InhabitedTimeFilter.minTicks(0)`: its class isn't translated (it implements `TrimFilter`) (org/popcraft/chunky/trim/InhabitedTimeFilter.java:6)
- [ ] `InhabitedTimeFilter.maxTicks(0)`: its class isn't translated (it implements `TrimFilter`) (org/popcraft/chunky/trim/InhabitedTimeFilter.java:6)

### `TrimChunk`

- [x] `TrimChunk.TrimChunk(3)` (org/popcraft/chunky/trim/TrimChunk.java:3)
- [x] `TrimChunk.inhabitedTime(0)` (org/popcraft/chunky/trim/TrimChunk.java:3)
- [x] `TrimChunk.exists(0)` (org/popcraft/chunky/trim/TrimChunk.java:3)
- [x] `TrimChunk.lastModified(0)` (org/popcraft/chunky/trim/TrimChunk.java:3)

### `TrimFilter`: it's an interface

- [ ] `TrimFilter.test(1)`: it has no body (org/popcraft/chunky/trim/TrimFilter.java:6)
- [ ] `TrimFilter.readsChunkData(0)`: its class isn't translated (it's an interface) (org/popcraft/chunky/trim/TrimFilter.java:8)
- [ ] `TrimFilter.sendNotice(1)`: it has no body (org/popcraft/chunky/trim/TrimFilter.java:12)

### `TrimFilters`

- [x] `TrimFilters.KEYS` (org/popcraft/chunky/trim/TrimFilters.java:13)
- [x] `TrimFilters.RANGE` (org/popcraft/chunky/trim/TrimFilters.java:14)
- [x] `TrimFilters.TICKS_PER_SECOND` (org/popcraft/chunky/trim/TrimFilters.java:15)
- [x] `TrimFilters.TrimFilters(0)` (org/popcraft/chunky/trim/TrimFilters.java:17)
- [x] `TrimFilters.keys(0)` (org/popcraft/chunky/trim/TrimFilters.java:20)
- [ ] `TrimFilters.parse(1)`: it uses TrimFilter, which isn't translated (it's an interface) (org/popcraft/chunky/trim/TrimFilters.java:24)
- [ ] `TrimFilters.suggestions(1)`: List.subList (org/popcraft/chunky/trim/TrimFilters.java:54)
- [ ] `TrimFilters.parseInhabited(1)`: it uses InhabitedTimeFilter, which isn't translated (it implements `TrimFilter`) (org/popcraft/chunky/trim/TrimFilters.java:62)
- [ ] `TrimFilters.parseUnmodified(1)`: it uses UnmodifiedFilter, which isn't translated (it implements `TrimFilter`) (org/popcraft/chunky/trim/TrimFilters.java:81)
- [ ] `TrimFilters.parseTicks(1)`: it uses `TimeUnit` (java.util.concurrent) (org/popcraft/chunky/trim/TrimFilters.java:90)
- [ ] `TrimFilters.timeUnit(1)`: it uses `TimeUnit` (java.util.concurrent) (org/popcraft/chunky/trim/TrimFilters.java:97)
- [ ] `TrimFilters.parseAmount(1)`: it calls Input.tryDouble, which isn't translated (org/popcraft/chunky/trim/TrimFilters.java:111)

### `TrimRegion`: it uses RegionFile, which isn't translated (a Map or Set keyed by a Chunk)

- [x] `TrimRegion.SECTOR_SIZE` (org/popcraft/chunky/trim/TrimRegion.java:14)
- [ ] `TrimRegion.TrimRegion(4)`: its class isn't translated (it uses RegionFile, which isn't translated (a Map or Set keyed by a Chunk)) (org/popcraft/chunky/trim/TrimRegion.java:18)
- [ ] `TrimRegion.isFullyRead(0)`: its class isn't translated (it uses RegionFile, which isn't translated (a Map or Set keyed by a Chunk)) (org/popcraft/chunky/trim/TrimRegion.java:26)
- [ ] `TrimRegion.getTrimChunk(2)`: its class isn't translated (it uses RegionFile, which isn't translated (a Map or Set keyed by a Chunk)) (org/popcraft/chunky/trim/TrimRegion.java:30)
- [ ] `TrimRegion.isValid(1)`: `null` (Rust has no null: this needs an Option) (org/popcraft/chunky/trim/TrimRegion.java:55)

### `UnmodifiedFilter`: it implements `TrimFilter`

- [ ] `UnmodifiedFilter.UnmodifiedFilter(3)`: its class isn't translated (it implements `TrimFilter`) (org/popcraft/chunky/trim/UnmodifiedFilter.java:8)
- [ ] `UnmodifiedFilter.test(1)`: its class isn't translated (it implements `TrimFilter`) (org/popcraft/chunky/trim/UnmodifiedFilter.java:9)
- [ ] `UnmodifiedFilter.sendNotice(1)`: its class isn't translated (it implements `TrimFilter`) (org/popcraft/chunky/trim/UnmodifiedFilter.java:14)
- [ ] `UnmodifiedFilter.unitKey(0)`: its class isn't translated (it implements `TrimFilter`) (org/popcraft/chunky/trim/UnmodifiedFilter.java:19)
- [ ] `UnmodifiedFilter.amount(0)`: its class isn't translated (it implements `TrimFilter`) (org/popcraft/chunky/trim/UnmodifiedFilter.java:8)
- [ ] `UnmodifiedFilter.unit(0)`: its class isn't translated (it implements `TrimFilter`) (org/popcraft/chunky/trim/UnmodifiedFilter.java:8)
- [ ] `UnmodifiedFilter.cutoff(0)`: its class isn't translated (it implements `TrimFilter`) (org/popcraft/chunky/trim/UnmodifiedFilter.java:8)

### `ChunkCoordinate`: it implements `Comparable`

- [ ] `ChunkCoordinate.ChunkCoordinate(2)`: its class isn't translated (it implements `Comparable`) (org/popcraft/chunky/util/ChunkCoordinate.java:6)
- [ ] `ChunkCoordinate.fromRegionFile(1)`: it uses ChunkCoordinate, which isn't translated (it implements `Comparable`) (org/popcraft/chunky/util/ChunkCoordinate.java:7)
- [ ] `ChunkCoordinate.compareTo(1)`: its class isn't translated (it implements `Comparable`) (org/popcraft/chunky/util/ChunkCoordinate.java:24)
- [ ] `ChunkCoordinate.equals(1)`: its class isn't translated (it implements `Comparable`) (org/popcraft/chunky/util/ChunkCoordinate.java:28)
- [ ] `ChunkCoordinate.hashCode(0)`: its class isn't translated (it implements `Comparable`) (org/popcraft/chunky/util/ChunkCoordinate.java:40)
- [ ] `ChunkCoordinate.toString(0)`: its class isn't translated (it implements `Comparable`) (org/popcraft/chunky/util/ChunkCoordinate.java:45)
- [ ] `ChunkCoordinate.x(0)`: its class isn't translated (it implements `Comparable`) (org/popcraft/chunky/util/ChunkCoordinate.java:6)
- [ ] `ChunkCoordinate.z(0)`: its class isn't translated (it implements `Comparable`) (org/popcraft/chunky/util/ChunkCoordinate.java:6)

### `ChunkMath`

- [x] `ChunkMath.ChunkMath(0)` (org/popcraft/chunky/util/ChunkMath.java:4)
- [x] `ChunkMath.pack(2)` (org/popcraft/chunky/util/ChunkMath.java:7)
- [x] `ChunkMath.regionIndex(2)` (org/popcraft/chunky/util/ChunkMath.java:13)

### `Disk`

- [x] `Disk.ESTIMATED_SPACE_PER_CHUNK` (org/popcraft/chunky/util/Disk.java:12)
- [x] `Disk.PERCENT_OVERESTIMATE` (org/popcraft/chunky/util/Disk.java:13)
- [x] `Disk.Disk(0)` (org/popcraft/chunky/util/Disk.java:15)
- [ ] `Disk.estimatedSpace(1)`: it uses Selection, which isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))) (org/popcraft/chunky/util/Disk.java:18)
- [ ] `Disk.remainingSpace(1)`: it uses World, which isn't translated (it's an interface) (org/popcraft/chunky/util/Disk.java:24)

### `EnumUtil`

- [x] `EnumUtil.EnumUtil(0)` (org/popcraft/chunky/util/EnumUtil.java:6)
- [ ] `EnumUtil.valueOf(2)`: it's generic (org/popcraft/chunky/util/EnumUtil.java:9)

### `Formatting`

- [ ] `Formatting.NUMBER_FORMAT`: it uses `DecimalFormat` (java.text) (org/popcraft/chunky/util/Formatting.java:10)
- [x] `Formatting.BINARY_PREFIXES` (org/popcraft/chunky/util/Formatting.java:11)
- [x] `Formatting.Formatting(0)` (org/popcraft/chunky/util/Formatting.java:13)
- [ ] `Formatting.bytes(1)`: String.format with `%cB` (org/popcraft/chunky/util/Formatting.java:16)
- [ ] `Formatting.radius(1)`: it uses Selection, which isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))) (org/popcraft/chunky/util/Formatting.java:32)
- [ ] `Formatting.worlds(1)`: it uses World, which isn't translated (it's an interface) (org/popcraft/chunky/util/Formatting.java:38)
- [ ] `Formatting.number(1)`: it uses Formatting.NUMBER_FORMAT, which isn't translated (it uses `DecimalFormat` (java.text)) (org/popcraft/chunky/util/Formatting.java:44)

### `GenerationMap`: it uses Selection, which isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface)))

- [x] `GenerationMap.FULL` (org/popcraft/chunky/util/GenerationMap.java:28)
- [x] `GenerationMap.UNFINISHED` (org/popcraft/chunky/util/GenerationMap.java:29)
- [x] `GenerationMap.MISSING` (org/popcraft/chunky/util/GenerationMap.java:30)
- [x] `GenerationMap.INCOMPLETE` (org/popcraft/chunky/util/GenerationMap.java:31)
- [x] `GenerationMap.INVALID` (org/popcraft/chunky/util/GenerationMap.java:32)
- [x] `GenerationMap.STATUSES` (org/popcraft/chunky/util/GenerationMap.java:33)
- [x] `GenerationMap.COLORS` (org/popcraft/chunky/util/GenerationMap.java:34)
- [x] `GenerationMap.OUTSIDE` (org/popcraft/chunky/util/GenerationMap.java:35)
- [x] `GenerationMap.MAX_SIZE` (org/popcraft/chunky/util/GenerationMap.java:36)
- [ ] `GenerationMap.FILTER`: it calls ChunkFilter.ofTags, which isn't translated (org/popcraft/chunky/util/GenerationMap.java:37)
- [ ] `GenerationMap.THREADS`: it calls `Runtime`.getRuntime (org/popcraft/chunky/util/GenerationMap.java:38)
- [ ] `GenerationMap.GenerationMap(1)`: its class isn't translated (it uses Selection, which isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface)))) (org/popcraft/chunky/util/GenerationMap.java:52)
- [ ] `GenerationMap.scan(0)`: its class isn't translated (it uses Selection, which isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface)))) (org/popcraft/chunky/util/GenerationMap.java:71)
- [ ] `GenerationMap.scanRegion(1)`: its class isn't translated (it uses Selection, which isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface)))) (org/popcraft/chunky/util/GenerationMap.java:119)
- [ ] `GenerationMap.classify(3)`: its class isn't translated (it uses Selection, which isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface)))) (org/popcraft/chunky/util/GenerationMap.java:139)
- [ ] `GenerationMap.record(3)`: its class isn't translated (it uses Selection, which isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface)))) (org/popcraft/chunky/util/GenerationMap.java:149)
- [ ] `GenerationMap.countMissing(0)`: its class isn't translated (it uses Selection, which isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface)))) (org/popcraft/chunky/util/GenerationMap.java:174)
- [ ] `GenerationMap.isInside(2)`: its class isn't translated (it uses Selection, which isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface)))) (org/popcraft/chunky/util/GenerationMap.java:192)
- [ ] `GenerationMap.getCount(1)`: its class isn't translated (it uses Selection, which isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface)))) (org/popcraft/chunky/util/GenerationMap.java:198)
- [ ] `GenerationMap.isExact(0)`: its class isn't translated (it uses Selection, which isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface)))) (org/popcraft/chunky/util/GenerationMap.java:202)
- [ ] `GenerationMap.write(1)`: its class isn't translated (it uses Selection, which isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface)))) (org/popcraft/chunky/util/GenerationMap.java:206)

### `Hilbert`

- [ ] `Hilbert.regionChunkCoordinateOffsets`: it uses ChunkCoordinate, which isn't translated (it implements `Comparable`) (org/popcraft/chunky/util/Hilbert.java:4)
- [x] `Hilbert.Hilbert(0)` (org/popcraft/chunky/util/Hilbert.java:1031)
- [ ] `Hilbert.regionDistanceToChunkCoordinateOffset(1)`: it uses ChunkCoordinate, which isn't translated (it implements `Comparable`) (org/popcraft/chunky/util/Hilbert.java:1034)
- [ ] `Hilbert.chunkCoordinateOffsets(0)`: it uses ChunkCoordinate, which isn't translated (it implements `Comparable`) (org/popcraft/chunky/util/Hilbert.java:1038)

### `Input`

- [x] `Input.Input(0)` (org/popcraft/chunky/util/Input.java:10)
- [ ] `Input.tryWorld(2)`: it uses World, which isn't translated (it's an interface) (org/popcraft/chunky/util/Input.java:13)
- [x] `Input.tryPattern(1)` (org/popcraft/chunky/util/Input.java:17)
- [ ] `Input.tryShape(1)`: it calls ShapeType.all, which isn't translated (org/popcraft/chunky/util/Input.java:26)
- [ ] `Input.tryBoolean(1)`: it calls `Boolean`.parseBoolean (org/popcraft/chunky/util/Input.java:35)
- [ ] `Input.tryInteger(1)`: parses something that isn't a string (org/popcraft/chunky/util/Input.java:39)
- [ ] `Input.tryIntegerSuffixed(1)`: it calls Input.suffixValue, which isn't translated (org/popcraft/chunky/util/Input.java:51)
- [ ] `Input.tryDouble(1)`: parses something that isn't a string (org/popcraft/chunky/util/Input.java:60)
- [ ] `Input.tryDoubleSuffixed(1)`: it calls Input.suffixValue, which isn't translated (org/popcraft/chunky/util/Input.java:72)
- [ ] `Input.tryLong(1)`: parses something that isn't a string (org/popcraft/chunky/util/Input.java:83)
- [ ] `Input.trySign(1)`: a switch expression giving a Optional<int> (org/popcraft/chunky/util/Input.java:95)
- [x] `Input.isPastWorldLimit(1)` (org/popcraft/chunky/util/Input.java:109)
- [ ] `Input.checkLanguage(1)`: it calls Translator.isValidLanguage, which isn't translated (org/popcraft/chunky/util/Input.java:113)
- [ ] `Input.suffixValue(1)`: a switch expression giving a Optional<int> (org/popcraft/chunky/util/Input.java:117)

### `MaxWorkingCount`

- [x] `MaxWorkingCount.DEFAULT` (org/popcraft/chunky/util/MaxWorkingCount.java:6)
- [ ] `MaxWorkingCount.PROPERTY`: it calls Input.tryInteger, which isn't translated (org/popcraft/chunky/util/MaxWorkingCount.java:7)
- [x] `MaxWorkingCount.MaxWorkingCount(0)` (org/popcraft/chunky/util/MaxWorkingCount.java:9)
- [ ] `MaxWorkingCount.get(0)`: it uses MaxWorkingCount.PROPERTY, which isn't translated (it calls Input.tryInteger, which isn't translated) (org/popcraft/chunky/util/MaxWorkingCount.java:12)

### `Pair`: it's generic

- [ ] `Pair.Pair(2)`: its class isn't translated (it's generic) (org/popcraft/chunky/util/Pair.java:3)
- [ ] `Pair.of(2)`: it's generic (org/popcraft/chunky/util/Pair.java:4)
- [ ] `Pair.left(0)`: its class isn't translated (it's generic) (org/popcraft/chunky/util/Pair.java:3)
- [ ] `Pair.right(0)`: its class isn't translated (it's generic) (org/popcraft/chunky/util/Pair.java:3)

### `Parameter`

- [x] `Parameter.Parameter(2)` (org/popcraft/chunky/util/Parameter.java:9)
- [x] `Parameter.Parameter(1)` (org/popcraft/chunky/util/Parameter.java:14)
- [x] `Parameter.of(1)` (org/popcraft/chunky/util/Parameter.java:20)
- [x] `Parameter.of(2)` (org/popcraft/chunky/util/Parameter.java:24)
- [x] `Parameter.getType(0)` (org/popcraft/chunky/util/Parameter.java:28)
- [x] `Parameter.getValue(0)` (org/popcraft/chunky/util/Parameter.java:32)
- [x] `Parameter.toString(0)` (org/popcraft/chunky/util/Parameter.java:36)

### `PendingAction`: a function kept in a variable or field (only parameters can be)

- [ ] `PendingAction.PendingAction(1)`: its class isn't translated (a function kept in a variable or field (only parameters can be)) (org/popcraft/chunky/util/PendingAction.java:9)
- [ ] `PendingAction.getAction(0)`: its class isn't translated (a function kept in a variable or field (only parameters can be)) (org/popcraft/chunky/util/PendingAction.java:14)
- [ ] `PendingAction.hasExpired(0)`: its class isn't translated (a function kept in a variable or field (only parameters can be)) (org/popcraft/chunky/util/PendingAction.java:18)

### `PlayerPauseTask`: it keeps an atomic value in an object's field

- [ ] `PlayerPauseTask.INTERVAL`: it calls Input.tryInteger, which isn't translated (org/popcraft/chunky/util/PlayerPauseTask.java:14)
- [ ] `PlayerPauseTask.PlayerPauseTask(1)`: its class isn't translated (it keeps an atomic value in an object's field) (org/popcraft/chunky/util/PlayerPauseTask.java:20)
- [ ] `PlayerPauseTask.register(1)`: its class isn't translated (it keeps an atomic value in an object's field) (org/popcraft/chunky/util/PlayerPauseTask.java:24)
- [ ] `PlayerPauseTask.start(0)`: its class isn't translated (it keeps an atomic value in an object's field) (org/popcraft/chunky/util/PlayerPauseTask.java:29)
- [ ] `PlayerPauseTask.stop(0)`: its class isn't translated (it keeps an atomic value in an object's field) (org/popcraft/chunky/util/PlayerPauseTask.java:35)
- [ ] `PlayerPauseTask.schedule(0)`: its class isn't translated (it keeps an atomic value in an object's field) (org/popcraft/chunky/util/PlayerPauseTask.java:39)
- [ ] `PlayerPauseTask.update(0)`: its class isn't translated (it keeps an atomic value in an object's field) (org/popcraft/chunky/util/PlayerPauseTask.java:50)

### `RateLimiter`

- [x] `RateLimiter.SLOT_MILLIS` (org/popcraft/chunky/util/RateLimiter.java:9)
- [ ] `RateLimiter.SECOND_NANOS`: it calls `SECONDS`.toNanos (org/popcraft/chunky/util/RateLimiter.java:10)
- [ ] `RateLimiter.BUDGET_POLL_NANOS`: it calls `MILLISECONDS`.toNanos (org/popcraft/chunky/util/RateLimiter.java:11)
- [x] `RateLimiter.RateLimiter(2)` (org/popcraft/chunky/util/RateLimiter.java:21)
- [x] `RateLimiter.getLimit(0)` (org/popcraft/chunky/util/RateLimiter.java:29)
- [x] `RateLimiter.setLimit(1)` (org/popcraft/chunky/util/RateLimiter.java:33)
- [ ] `RateLimiter.acquire(2)`: it uses `Semaphore` (java.util.concurrent) (org/popcraft/chunky/util/RateLimiter.java:41)
- [ ] `RateLimiter.delay(3)`: it uses RateLimiter.SECOND_NANOS, which isn't translated (it calls `SECONDS`.toNanos) (org/popcraft/chunky/util/RateLimiter.java:57)
- [x] `RateLimiter.release(1)` (org/popcraft/chunky/util/RateLimiter.java:84)
- [x] `RateLimiter.record(2)` (org/popcraft/chunky/util/RateLimiter.java:90)
- [x] `RateLimiter.rate(1)` (org/popcraft/chunky/util/RateLimiter.java:103)
- [x] `RateLimiter.count(2)` (org/popcraft/chunky/util/RateLimiter.java:108)

### `Reflection`

- [x] `Reflection.Reflection(0)` (org/popcraft/chunky/util/Reflection.java:6)
- [ ] `Reflection.classExists(1)`: a try/catch (ClassNotFoundException) (org/popcraft/chunky/util/Reflection.java:9)
- [ ] `Reflection.methodExists(3)`: it uses `Class` (org/popcraft/chunky/util/Reflection.java:18)
- [ ] `Reflection.getStaticField(2)`: it uses `Object` (org/popcraft/chunky/util/Reflection.java:27)
- [ ] `Reflection.invokeMethod(2)`: it uses `Object` (org/popcraft/chunky/util/Reflection.java:35)

### `RegionCache`: it uses WorldState, which isn't translated (it uses `BitSet` (java.util))

- [ ] `RegionCache.getWorld(1)`: its class isn't translated (it uses WorldState, which isn't translated (it uses `BitSet` (java.util))) (org/popcraft/chunky/util/RegionCache.java:10)
- [ ] `RegionCache.clear(1)`: its class isn't translated (it uses WorldState, which isn't translated (it uses `BitSet` (java.util))) (org/popcraft/chunky/util/RegionCache.java:14)

### `RegionCache.WorldState`: it uses `BitSet` (java.util)

- [ ] `RegionCache.WorldState.setGenerated(2)`: its class isn't translated (it uses `BitSet` (java.util)) (org/popcraft/chunky/util/RegionCache.java:21)
- [ ] `RegionCache.WorldState.isGenerated(2)`: its class isn't translated (it uses `BitSet` (java.util)) (org/popcraft/chunky/util/RegionCache.java:32)

### `RegionStatusReader`: it implements `AutoCloseable`

- [x] `RegionStatusReader.UPDATE_CHUNK_NBT` (org/popcraft/chunky/util/RegionStatusReader.java:19)
- [x] `RegionStatusReader.LOAD_OUTDATED_CHUNKS` (org/popcraft/chunky/util/RegionStatusReader.java:20)
- [ ] `RegionStatusReader.THREADS`: it calls Input.tryInteger, which isn't translated (org/popcraft/chunky/util/RegionStatusReader.java:21)
- [ ] `RegionStatusReader.RegionStatusReader(3)`: its class isn't translated (it implements `AutoCloseable`) (org/popcraft/chunky/util/RegionStatusReader.java:30)
- [ ] `RegionStatusReader.getState(2)`: its class isn't translated (it implements `AutoCloseable`) (org/popcraft/chunky/util/RegionStatusReader.java:41)
- [ ] `RegionStatusReader.read(3)`: its class isn't translated (it implements `AutoCloseable`) (org/popcraft/chunky/util/RegionStatusReader.java:66)
- [ ] `RegionStatusReader.isValid(3)`: it uses CompoundTag, which isn't translated (it extends `Tag`) (org/popcraft/chunky/util/RegionStatusReader.java:83)
- [ ] `RegionStatusReader.isFullStatus(1)`: it uses CompoundTag, which isn't translated (it extends `Tag`) (org/popcraft/chunky/util/RegionStatusReader.java:87)
- [ ] `RegionStatusReader.isFull(1)`: it uses CompoundTag, which isn't translated (it extends `Tag`) (org/popcraft/chunky/util/RegionStatusReader.java:91)
- [ ] `RegionStatusReader.close(0)`: its class isn't translated (it implements `AutoCloseable`) (org/popcraft/chunky/util/RegionStatusReader.java:97)

### `RegionStatusReader.ChunkState`


### `TaskLoader`: it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))

- [ ] `TaskLoader.TaskLoader(1)`: its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))) (org/popcraft/chunky/util/TaskLoader.java:26)
- [ ] `TaskLoader.reload(0)`: its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))) (org/popcraft/chunky/util/TaskLoader.java:32)
- [ ] `TaskLoader.loadTask(1)`: its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))) (org/popcraft/chunky/util/TaskLoader.java:59)
- [ ] `TaskLoader.getTask(1)`: its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))) (org/popcraft/chunky/util/TaskLoader.java:89)
- [ ] `TaskLoader.loadTasks(0)`: its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))) (org/popcraft/chunky/util/TaskLoader.java:105)
- [ ] `TaskLoader.saveTask(1)`: its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))) (org/popcraft/chunky/util/TaskLoader.java:115)
- [ ] `TaskLoader.saveTask(3)`: its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))) (org/popcraft/chunky/util/TaskLoader.java:119)
- [ ] `TaskLoader.saveTasks(0)`: its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))) (org/popcraft/chunky/util/TaskLoader.java:148)
- [ ] `TaskLoader.cancelTask(1)`: its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))) (org/popcraft/chunky/util/TaskLoader.java:154)
- [ ] `TaskLoader.cancelTasks(0)`: its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))) (org/popcraft/chunky/util/TaskLoader.java:165)
- [ ] `TaskLoader.writeTask(2)`: its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))) (org/popcraft/chunky/util/TaskLoader.java:169)

### `TaskLoader.TaskProperty`: its constants carry values

- [ ] `TaskLoader.TaskProperty.TaskProperty(1)`: its class isn't translated (its constants carry values) (org/popcraft/chunky/util/TaskLoader.java:207)
- [ ] `TaskLoader.TaskProperty.key(0)`: its class isn't translated (its constants carry values) (org/popcraft/chunky/util/TaskLoader.java:211)

### `TaskLogger`: it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))

- [ ] `TaskLogger.TIMESTAMP_FORMAT`: it uses `DateTimeFormatter` (java.time.format) (org/popcraft/chunky/util/TaskLogger.java:16)
- [ ] `TaskLogger.TaskLogger(1)`: its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))) (org/popcraft/chunky/util/TaskLogger.java:19)
- [ ] `TaskLogger.register(1)`: its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))) (org/popcraft/chunky/util/TaskLogger.java:23)
- [ ] `TaskLogger.onStart(1)`: its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))) (org/popcraft/chunky/util/TaskLogger.java:28)
- [ ] `TaskLogger.onComplete(1)`: its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))) (org/popcraft/chunky/util/TaskLogger.java:34)
- [ ] `TaskLogger.appendTask(9)`: its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))) (org/popcraft/chunky/util/TaskLogger.java:43)
- [ ] `TaskLogger.number(1)`: its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))) (org/popcraft/chunky/util/TaskLogger.java:64)
- [ ] `TaskLogger.write(1)`: its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))) (org/popcraft/chunky/util/TaskLogger.java:68)

### `TaskMetrics`

- [x] `TaskMetrics.TaskMetrics(0)` (org/popcraft/chunky/util/TaskMetrics.java:11)
- [ ] `TaskMetrics.activeTasks(1)`: it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface)) (org/popcraft/chunky/util/TaskMetrics.java:14)
- [ ] `TaskMetrics.chunksPerSecond(1)`: it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface)) (org/popcraft/chunky/util/TaskMetrics.java:18)
- [ ] `TaskMetrics.shapes(1)`: it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface)) (org/popcraft/chunky/util/TaskMetrics.java:22)
- [ ] `TaskMetrics.patterns(1)`: it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface)) (org/popcraft/chunky/util/TaskMetrics.java:26)
- [ ] `TaskMetrics.sizes(1)`: it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface)) (org/popcraft/chunky/util/TaskMetrics.java:30)
- [ ] `TaskMetrics.speeds(1)`: it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface)) (org/popcraft/chunky/util/TaskMetrics.java:34)
- [ ] `TaskMetrics.count(2)`: it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface)) (org/popcraft/chunky/util/TaskMetrics.java:44)

### `TaskScheduler`: it uses `ExecutorService` (java.util.concurrent)

- [ ] `TaskScheduler.TaskScheduler(0)`: its class isn't translated (it uses `ExecutorService` (java.util.concurrent)) (org/popcraft/chunky/util/TaskScheduler.java:15)
- [ ] `TaskScheduler.runTask(1)`: its class isn't translated (it uses `ExecutorService` (java.util.concurrent)) (org/popcraft/chunky/util/TaskScheduler.java:27)
- [ ] `TaskScheduler.cancelTasks(0)`: its class isn't translated (it uses `ExecutorService` (java.util.concurrent)) (org/popcraft/chunky/util/TaskScheduler.java:32)

### `TranslationKey`

- [x] `TranslationKey.BORDER_DEPENDENCY_UPDATE` (org/popcraft/chunky/util/TranslationKey.java:4)
- [x] `TranslationKey.BORDER_LOAD_FAILED` (org/popcraft/chunky/util/TranslationKey.java:5)
- [x] `TranslationKey.BORDER_SAVE_FAILED` (org/popcraft/chunky/util/TranslationKey.java:6)
- [x] `TranslationKey.COMMAND_NO_PERMISSION` (org/popcraft/chunky/util/TranslationKey.java:7)
- [x] `TranslationKey.DISABLED` (org/popcraft/chunky/util/TranslationKey.java:8)
- [x] `TranslationKey.ENABLED` (org/popcraft/chunky/util/TranslationKey.java:9)
- [x] `TranslationKey.ERROR_PAUSE_WHEN_EMPTY_SECONDS` (org/popcraft/chunky/util/TranslationKey.java:10)
- [x] `TranslationKey.FORMAT_BORDER_ADD` (org/popcraft/chunky/util/TranslationKey.java:11)
- [x] `TranslationKey.FORMAT_BORDER_BYPASS` (org/popcraft/chunky/util/TranslationKey.java:12)
- [x] `TranslationKey.FORMAT_BORDER_BYPASS_NO_TARGET` (org/popcraft/chunky/util/TranslationKey.java:13)
- [x] `TranslationKey.FORMAT_BORDER_LIST` (org/popcraft/chunky/util/TranslationKey.java:14)
- [x] `TranslationKey.FORMAT_BORDER_LIST_BORDER` (org/popcraft/chunky/util/TranslationKey.java:15)
- [x] `TranslationKey.FORMAT_BORDER_LIST_NONE` (org/popcraft/chunky/util/TranslationKey.java:16)
- [x] `TranslationKey.FORMAT_BORDER_LOAD` (org/popcraft/chunky/util/TranslationKey.java:17)
- [x] `TranslationKey.FORMAT_BORDER_NO_BORDER` (org/popcraft/chunky/util/TranslationKey.java:18)
- [x] `TranslationKey.FORMAT_BORDER_REMOVE` (org/popcraft/chunky/util/TranslationKey.java:19)
- [x] `TranslationKey.FORMAT_BORDER_WRAP` (org/popcraft/chunky/util/TranslationKey.java:20)
- [x] `TranslationKey.FORMAT_CANCEL` (org/popcraft/chunky/util/TranslationKey.java:21)
- [x] `TranslationKey.FORMAT_CANCEL_ALL` (org/popcraft/chunky/util/TranslationKey.java:22)
- [x] `TranslationKey.FORMAT_CANCEL_CONFIRM` (org/popcraft/chunky/util/TranslationKey.java:23)
- [x] `TranslationKey.FORMAT_CANCEL_NO_TASKS` (org/popcraft/chunky/util/TranslationKey.java:24)
- [x] `TranslationKey.FORMAT_CENTER` (org/popcraft/chunky/util/TranslationKey.java:25)
- [x] `TranslationKey.FORMAT_CONFIRM` (org/popcraft/chunky/util/TranslationKey.java:26)
- [x] `TranslationKey.FORMAT_CONTINUE` (org/popcraft/chunky/util/TranslationKey.java:27)
- [x] `TranslationKey.FORMAT_CONTINUE_NO_TASKS` (org/popcraft/chunky/util/TranslationKey.java:28)
- [x] `TranslationKey.FORMAT_DEBUG_MAP` (org/popcraft/chunky/util/TranslationKey.java:29)
- [x] `TranslationKey.FORMAT_DEBUG_MAP_FAILED` (org/popcraft/chunky/util/TranslationKey.java:30)
- [x] `TranslationKey.FORMAT_DEBUG_MAP_START` (org/popcraft/chunky/util/TranslationKey.java:31)
- [x] `TranslationKey.FORMAT_TRIM_CONFIRM` (org/popcraft/chunky/util/TranslationKey.java:32)
- [x] `TranslationKey.FORMAT_TRIM_CONFIRM_INSIDE` (org/popcraft/chunky/util/TranslationKey.java:33)
- [x] `TranslationKey.FORMAT_TRIM_CONFIRM_INHABITED` (org/popcraft/chunky/util/TranslationKey.java:34)
- [x] `TranslationKey.FORMAT_TRIM_CONFIRM_INHABITED_MINIMUM` (org/popcraft/chunky/util/TranslationKey.java:35)
- [x] `TranslationKey.FORMAT_TRIM_CONFIRM_INHABITED_RANGE` (org/popcraft/chunky/util/TranslationKey.java:36)
- [x] `TranslationKey.FORMAT_TRIM_CONFIRM_UNMODIFIED` (org/popcraft/chunky/util/TranslationKey.java:37)
- [x] `TranslationKey.FORMAT_TRIM_FILTER_INVALID` (org/popcraft/chunky/util/TranslationKey.java:38)
- [x] `TranslationKey.FORMAT_LIMIT` (org/popcraft/chunky/util/TranslationKey.java:39)
- [x] `TranslationKey.FORMAT_LIMIT_DISABLED` (org/popcraft/chunky/util/TranslationKey.java:40)
- [x] `TranslationKey.FORMAT_PATTERN` (org/popcraft/chunky/util/TranslationKey.java:41)
- [x] `TranslationKey.FORMAT_PATTERN_NOT_FOUND` (org/popcraft/chunky/util/TranslationKey.java:42)
- [x] `TranslationKey.FORMAT_PAUSE` (org/popcraft/chunky/util/TranslationKey.java:43)
- [x] `TranslationKey.FORMAT_PAUSE_NO_TASKS` (org/popcraft/chunky/util/TranslationKey.java:44)
- [x] `TranslationKey.FORMAT_PROGRESS_NO_TASKS` (org/popcraft/chunky/util/TranslationKey.java:45)
- [x] `TranslationKey.FORMAT_QUIET` (org/popcraft/chunky/util/TranslationKey.java:46)
- [x] `TranslationKey.FORMAT_RADII` (org/popcraft/chunky/util/TranslationKey.java:47)
- [x] `TranslationKey.FORMAT_RADIUS` (org/popcraft/chunky/util/TranslationKey.java:48)
- [x] `TranslationKey.FORMAT_RELOAD` (org/popcraft/chunky/util/TranslationKey.java:49)
- [x] `TranslationKey.FORMAT_RELOAD_TASKS_RUNNING` (org/popcraft/chunky/util/TranslationKey.java:50)
- [x] `TranslationKey.FORMAT_SELECTION` (org/popcraft/chunky/util/TranslationKey.java:51)
- [x] `TranslationKey.FORMAT_SELECTION_CENTER` (org/popcraft/chunky/util/TranslationKey.java:52)
- [x] `TranslationKey.FORMAT_SELECTION_RADII` (org/popcraft/chunky/util/TranslationKey.java:53)
- [x] `TranslationKey.FORMAT_SELECTION_RADIUS` (org/popcraft/chunky/util/TranslationKey.java:54)
- [x] `TranslationKey.FORMAT_SELECTION_SHAPE` (org/popcraft/chunky/util/TranslationKey.java:55)
- [x] `TranslationKey.FORMAT_SELECTION_WORLD` (org/popcraft/chunky/util/TranslationKey.java:56)
- [x] `TranslationKey.FORMAT_SHAPE` (org/popcraft/chunky/util/TranslationKey.java:57)
- [x] `TranslationKey.FORMAT_SHAPE_NOT_FOUND` (org/popcraft/chunky/util/TranslationKey.java:58)
- [x] `TranslationKey.FORMAT_SILENT` (org/popcraft/chunky/util/TranslationKey.java:59)
- [x] `TranslationKey.FORMAT_START` (org/popcraft/chunky/util/TranslationKey.java:60)
- [x] `TranslationKey.FORMAT_START_CONFIRM` (org/popcraft/chunky/util/TranslationKey.java:61)
- [x] `TranslationKey.FORMAT_START_DISK` (org/popcraft/chunky/util/TranslationKey.java:62)
- [x] `TranslationKey.FORMAT_START_LIMIT` (org/popcraft/chunky/util/TranslationKey.java:63)
- [x] `TranslationKey.FORMAT_STARTED_ALREADY` (org/popcraft/chunky/util/TranslationKey.java:64)
- [x] `TranslationKey.FORMAT_WORLD` (org/popcraft/chunky/util/TranslationKey.java:65)
- [x] `TranslationKey.FORMAT_WORLD_NOT_FOUND` (org/popcraft/chunky/util/TranslationKey.java:66)
- [x] `TranslationKey.FORMAT_WORLDBORDER_TOO_LARGE` (org/popcraft/chunky/util/TranslationKey.java:67)
- [x] `TranslationKey.HELP_BORDER` (org/popcraft/chunky/util/TranslationKey.java:68)
- [x] `TranslationKey.HELP_CANCEL` (org/popcraft/chunky/util/TranslationKey.java:69)
- [x] `TranslationKey.HELP_CENTER` (org/popcraft/chunky/util/TranslationKey.java:70)
- [x] `TranslationKey.HELP_CONTINUE` (org/popcraft/chunky/util/TranslationKey.java:71)
- [x] `TranslationKey.HELP_CORNERS` (org/popcraft/chunky/util/TranslationKey.java:72)
- [x] `TranslationKey.HELP_DEBUG` (org/popcraft/chunky/util/TranslationKey.java:73)
- [x] `TranslationKey.HELP_TRIM` (org/popcraft/chunky/util/TranslationKey.java:74)
- [x] `TranslationKey.HELP_MENU` (org/popcraft/chunky/util/TranslationKey.java:75)
- [x] `TranslationKey.HELP_MORE` (org/popcraft/chunky/util/TranslationKey.java:76)
- [x] `TranslationKey.HELP_LIMIT` (org/popcraft/chunky/util/TranslationKey.java:77)
- [x] `TranslationKey.HELP_PATTERN` (org/popcraft/chunky/util/TranslationKey.java:78)
- [x] `TranslationKey.HELP_PAUSE` (org/popcraft/chunky/util/TranslationKey.java:79)
- [x] `TranslationKey.HELP_PROGRESS` (org/popcraft/chunky/util/TranslationKey.java:80)
- [x] `TranslationKey.HELP_QUIET` (org/popcraft/chunky/util/TranslationKey.java:81)
- [x] `TranslationKey.HELP_RADIUS` (org/popcraft/chunky/util/TranslationKey.java:82)
- [x] `TranslationKey.HELP_RELOAD` (org/popcraft/chunky/util/TranslationKey.java:83)
- [x] `TranslationKey.HELP_SELECTION` (org/popcraft/chunky/util/TranslationKey.java:84)
- [x] `TranslationKey.HELP_SHAPE` (org/popcraft/chunky/util/TranslationKey.java:85)
- [x] `TranslationKey.HELP_SILENT` (org/popcraft/chunky/util/TranslationKey.java:86)
- [x] `TranslationKey.HELP_SPAWN` (org/popcraft/chunky/util/TranslationKey.java:87)
- [x] `TranslationKey.HELP_START` (org/popcraft/chunky/util/TranslationKey.java:88)
- [x] `TranslationKey.HELP_WORLD` (org/popcraft/chunky/util/TranslationKey.java:89)
- [x] `TranslationKey.HELP_WORLDBORDER` (org/popcraft/chunky/util/TranslationKey.java:90)
- [x] `TranslationKey.PATTERN_CONCENTRIC` (org/popcraft/chunky/util/TranslationKey.java:91)
- [x] `TranslationKey.PATTERN_LOOP` (org/popcraft/chunky/util/TranslationKey.java:92)
- [x] `TranslationKey.PATTERN_STRIP` (org/popcraft/chunky/util/TranslationKey.java:93)
- [x] `TranslationKey.PATTERN_SPIRAL` (org/popcraft/chunky/util/TranslationKey.java:94)
- [x] `TranslationKey.PATTERN_CSV` (org/popcraft/chunky/util/TranslationKey.java:95)
- [x] `TranslationKey.PATTERN_REGION` (org/popcraft/chunky/util/TranslationKey.java:96)
- [x] `TranslationKey.PATTERN_RING` (org/popcraft/chunky/util/TranslationKey.java:97)
- [x] `TranslationKey.SHAPE_CIRCLE` (org/popcraft/chunky/util/TranslationKey.java:98)
- [x] `TranslationKey.SHAPE_DIAMOND` (org/popcraft/chunky/util/TranslationKey.java:99)
- [x] `TranslationKey.SHAPE_ELLIPSE` (org/popcraft/chunky/util/TranslationKey.java:100)
- [x] `TranslationKey.SHAPE_HEXAGON` (org/popcraft/chunky/util/TranslationKey.java:101)
- [x] `TranslationKey.SHAPE_PENTAGON` (org/popcraft/chunky/util/TranslationKey.java:102)
- [x] `TranslationKey.SHAPE_RECTANGLE` (org/popcraft/chunky/util/TranslationKey.java:103)
- [x] `TranslationKey.SHAPE_SQUARE` (org/popcraft/chunky/util/TranslationKey.java:104)
- [x] `TranslationKey.SHAPE_STAR` (org/popcraft/chunky/util/TranslationKey.java:105)
- [x] `TranslationKey.SHAPE_TRIANGLE` (org/popcraft/chunky/util/TranslationKey.java:106)
- [x] `TranslationKey.TASK_TRIM` (org/popcraft/chunky/util/TranslationKey.java:107)
- [x] `TranslationKey.TASK_TRIM_UNSUPPORTED_COMPRESSION` (org/popcraft/chunky/util/TranslationKey.java:108)
- [x] `TranslationKey.TASK_TRIM_UPDATE` (org/popcraft/chunky/util/TranslationKey.java:109)
- [x] `TranslationKey.TASK_DONE` (org/popcraft/chunky/util/TranslationKey.java:110)
- [x] `TranslationKey.TASK_STOPPED` (org/popcraft/chunky/util/TranslationKey.java:111)
- [x] `TranslationKey.TASK_UPDATE` (org/popcraft/chunky/util/TranslationKey.java:112)
- [x] `TranslationKey.TIME_DAY` (org/popcraft/chunky/util/TranslationKey.java:113)
- [x] `TranslationKey.TIME_DAYS` (org/popcraft/chunky/util/TranslationKey.java:114)
- [x] `TranslationKey.TIME_HOUR` (org/popcraft/chunky/util/TranslationKey.java:115)
- [x] `TranslationKey.TIME_HOURS` (org/popcraft/chunky/util/TranslationKey.java:116)
- [x] `TranslationKey.TIME_MINUTE` (org/popcraft/chunky/util/TranslationKey.java:117)
- [x] `TranslationKey.TIME_MINUTES` (org/popcraft/chunky/util/TranslationKey.java:118)
- [x] `TranslationKey.TIME_SECOND` (org/popcraft/chunky/util/TranslationKey.java:119)
- [x] `TranslationKey.TIME_SECONDS` (org/popcraft/chunky/util/TranslationKey.java:120)
- [x] `TranslationKey.WRAP_BOTH` (org/popcraft/chunky/util/TranslationKey.java:121)
- [x] `TranslationKey.WRAP_DEFAULT` (org/popcraft/chunky/util/TranslationKey.java:122)
- [x] `TranslationKey.WRAP_EARTH` (org/popcraft/chunky/util/TranslationKey.java:123)
- [x] `TranslationKey.WRAP_NONE` (org/popcraft/chunky/util/TranslationKey.java:124)
- [x] `TranslationKey.WRAP_RADIAL` (org/popcraft/chunky/util/TranslationKey.java:125)
- [x] `TranslationKey.WRAP_X` (org/popcraft/chunky/util/TranslationKey.java:126)
- [x] `TranslationKey.WRAP_Z` (org/popcraft/chunky/util/TranslationKey.java:127)
- [x] `TranslationKey.TranslationKey(0)` (org/popcraft/chunky/util/TranslationKey.java:129)

### `Translator`

- [ ] `Translator.fallbackTranslations`: it calls Translator.load, which isn't translated (org/popcraft/chunky/util/Translator.java:13)
- [x] `Translator.translations` (org/popcraft/chunky/util/Translator.java:14)
- [x] `Translator.Translator(0)` (org/popcraft/chunky/util/Translator.java:16)
- [ ] `Translator.setLanguage(1)`: it calls Translator.load, which isn't translated (org/popcraft/chunky/util/Translator.java:19)
- [ ] `Translator.isValidLanguage(1)`: unknown static Translator.class (org/popcraft/chunky/util/Translator.java:23)
- [ ] `Translator.translateKey(3)`: it uses `Object` (org/popcraft/chunky/util/Translator.java:27)
- [ ] `Translator.translate(2)`: it uses `Object` (org/popcraft/chunky/util/Translator.java:38)
- [ ] `Translator.getMessage(1)`: it uses Translator.fallbackTranslations, which isn't translated (it calls Translator.load, which isn't translated) (org/popcraft/chunky/util/Translator.java:42)
- [ ] `Translator.addCustomTranslation(2)`: it uses Translator.fallbackTranslations, which isn't translated (it calls Translator.load, which isn't translated) (org/popcraft/chunky/util/Translator.java:46)
- [ ] `Translator.load(1)`: its body couldn't be read: an anonymous class (org/popcraft/chunky/util/Translator.java:50)

### `Version`: it implements `Comparable`

- [ ] `Version.INVALID`: it creates a Version, which isn't translated (it implements `Comparable`) (org/popcraft/chunky/util/Version.java:6)
- [ ] `Version.Version(3)`: its class isn't translated (it implements `Comparable`) (org/popcraft/chunky/util/Version.java:11)
- [ ] `Version.Version(1)`: its class isn't translated (it implements `Comparable`) (org/popcraft/chunky/util/Version.java:17)
- [ ] `Version.Version(2)`: its class isn't translated (it implements `Comparable`) (org/popcraft/chunky/util/Version.java:36)
- [ ] `Version.isEqualTo(1)`: its class isn't translated (it implements `Comparable`) (org/popcraft/chunky/util/Version.java:40)
- [ ] `Version.isHigherThan(1)`: its class isn't translated (it implements `Comparable`) (org/popcraft/chunky/util/Version.java:44)
- [ ] `Version.isHigherThanOrEqualTo(1)`: its class isn't translated (it implements `Comparable`) (org/popcraft/chunky/util/Version.java:48)
- [ ] `Version.isLowerThan(1)`: its class isn't translated (it implements `Comparable`) (org/popcraft/chunky/util/Version.java:52)
- [ ] `Version.isLowerThanOrEqualTo(1)`: its class isn't translated (it implements `Comparable`) (org/popcraft/chunky/util/Version.java:56)
- [ ] `Version.isValid(0)`: its class isn't translated (it implements `Comparable`) (org/popcraft/chunky/util/Version.java:60)
- [ ] `Version.compareTo(1)`: its class isn't translated (it implements `Comparable`) (org/popcraft/chunky/util/Version.java:64)
- [ ] `Version.equals(1)`: its class isn't translated (it implements `Comparable`) (org/popcraft/chunky/util/Version.java:72)
- [ ] `Version.hashCode(0)`: its class isn't translated (it implements `Comparable`) (org/popcraft/chunky/util/Version.java:84)
