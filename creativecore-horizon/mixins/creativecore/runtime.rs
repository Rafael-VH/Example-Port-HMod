// CreativeCore, ported to Horizon by horizon-port from CreativeCore_FABRIC_v2.14.24_mc26.3.jar.
// This file is both the bundle's Cargo library (its tests) and the module the bundle adds to
// the game (crate::game::port_creativecore). Put the mod's own logic here, in plain Rust
// that doesn't need the game, and call it from the helpers. No `//!` comments: module payloads
// can't hold inner attributes.

/// The mod's id (its settings folder), name, version and description.
pub const ID: &str = "creativecore";
pub const NAME: &str = "CreativeCore";
pub const VERSION: &str = "2.14.24";
pub const DESCRIPTION: &str = "A simple but powerful coremod.";

/// The mod's keys for Options > Mods: name, label, default key.
pub const KEYS: &[[&str; 3]] = &[];

/// The mod's own Java that doesn't need the game, translated to Rust by horizon-port.
/// Each Java method is here whole or not at all; the ones left out say why.
#[allow(dead_code, unused_variables, unused_mut, unused_parens, unused_imports, unreachable_code,
    non_snake_case, non_upper_case_globals, non_camel_case_types, clippy::all)]
pub mod java {
    use std::sync::{LazyLock, Mutex};

    /// A Java static that changes: one value behind a lock.
    pub struct JCell<T>(Mutex<T>);
    impl<T: Clone> JCell<T> {
        pub const fn new(v: T) -> Self {
            Self(Mutex::new(v))
        }
        pub fn get(&self) -> T {
            self.0.lock().unwrap_or_else(|e| e.into_inner()).clone()
        }
        pub fn set(&self, v: T) {
            *self.0.lock().unwrap_or_else(|e| e.into_inner()) = v;
        }
        pub fn with<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
            f(&mut self.0.lock().unwrap_or_else(|e| e.into_inner()))
        }
    }

    /// Java's HashMap, LinkedHashMap and TreeMap. Keeps insertion order (LinkedHashMap's; HashMap's
    /// order is unspecified), or key order when made with `new_sorted` (TreeMap's).
    #[derive(Clone, Debug)]
    pub struct JMap<K, V> {
        index: std::collections::HashMap<K, usize>,
        entries: Vec<(K, V)>,
        sorted: bool,
    }
    /// Java's Map.equals: the same keys with equal values, in any order.
    impl<K: std::hash::Hash + Eq, V: PartialEq> PartialEq for JMap<K, V> {
        fn eq(&self, o: &Self) -> bool {
            self.entries.len() == o.entries.len()
                && self.entries.iter().all(|(k, v)| o.index.get(k).is_some_and(|&i| o.entries[i].1 == *v))
        }
    }
    impl<K, V> Default for JMap<K, V> {
        fn default() -> Self {
            Self { index: std::collections::HashMap::new(), entries: Vec::new(), sorted: false }
        }
    }
    impl<K: std::hash::Hash + Ord + Clone, V> JMap<K, V> {
        pub fn new() -> Self {
            Self::default()
        }
        pub fn new_sorted() -> Self {
            Self { sorted: true, ..Self::default() }
        }
        pub fn from_iter(pairs: impl IntoIterator<Item = (K, V)>) -> Self {
            let mut m = Self::new();
            for (k, v) in pairs {
                m.insert(k, v);
            }
            m
        }
        pub fn from_iter_sorted(pairs: impl IntoIterator<Item = (K, V)>) -> Self {
            let mut m = Self::new_sorted();
            for (k, v) in pairs {
                m.insert(k, v);
            }
            m
        }
        pub fn len(&self) -> usize {
            self.entries.len()
        }
        pub fn is_empty(&self) -> bool {
            self.entries.is_empty()
        }
        pub fn clear(&mut self) {
            self.index.clear();
            self.entries.clear();
        }
        pub fn contains_key<Q: std::hash::Hash + Eq + ?Sized>(&self, k: &Q) -> bool
        where
            K: std::borrow::Borrow<Q>,
        {
            self.index.contains_key(k)
        }
        pub fn get<Q: std::hash::Hash + Eq + ?Sized>(&self, k: &Q) -> Option<&V>
        where
            K: std::borrow::Borrow<Q>,
        {
            self.index.get(k).map(|&i| &self.entries[i].1)
        }
        fn reindex(&mut self) {
            self.index.clear();
            for (i, (k, _)) in self.entries.iter().enumerate() {
                self.index.insert(k.clone(), i);
            }
        }
        fn slot(&mut self, k: K, v: V) -> usize {
            if self.sorted {
                let at = self.entries.partition_point(|e| e.0 < k);
                self.entries.insert(at, (k, v));
                self.reindex();
                at
            } else {
                self.index.insert(k.clone(), self.entries.len());
                self.entries.push((k, v));
                self.entries.len() - 1
            }
        }
        pub fn insert(&mut self, k: K, v: V) -> Option<V> {
            if let Some(&i) = self.index.get(&k) {
                return Some(std::mem::replace(&mut self.entries[i].1, v));
            }
            self.slot(k, v);
            None
        }
        pub fn remove<Q: std::hash::Hash + Eq + ?Sized>(&mut self, k: &Q) -> Option<V>
        where
            K: std::borrow::Borrow<Q>,
        {
            let i = *self.index.get(k)?;
            let (_, v) = self.entries.remove(i);
            self.reindex();
            Some(v)
        }
        /// computeIfAbsent and putIfAbsent: the value, made first when the key is missing.
        pub fn or_insert_with(&mut self, k: K, f: impl FnOnce() -> V) -> &mut V {
            let i = match self.index.get(&k) {
                Some(&i) => i,
                None => self.slot(k, f()),
            };
            &mut self.entries[i].1
        }
        pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
            self.entries.iter().map(|(k, v)| (k, v))
        }
        pub fn keys(&self) -> impl Iterator<Item = &K> {
            self.entries.iter().map(|e| &e.0)
        }
        pub fn values(&self) -> impl Iterator<Item = &V> {
            self.entries.iter().map(|e| &e.1)
        }
        pub fn clone_pairs(&self) -> Vec<(K, V)>
        where
            V: Clone,
        {
            self.entries.clone()
        }
    }

    /// Java's HashSet, LinkedHashSet and TreeSet, kept like JMap's keys.
    #[derive(Clone, Debug)]
    pub struct JSet<T>(JMap<T, ()>);
    impl<T: std::hash::Hash + Eq> PartialEq for JSet<T> {
        fn eq(&self, o: &Self) -> bool {
            self.0 == o.0
        }
    }
    impl<T> Default for JSet<T> {
        fn default() -> Self {
            Self(JMap::default())
        }
    }
    impl<T: std::hash::Hash + Ord + Clone> JSet<T> {
        pub fn new() -> Self {
            Self::default()
        }
        pub fn new_sorted() -> Self {
            Self(JMap::new_sorted())
        }
        pub fn from_iter(items: impl IntoIterator<Item = T>) -> Self {
            let mut s = Self::new();
            for v in items {
                s.insert(v);
            }
            s
        }
        pub fn from_iter_sorted(items: impl IntoIterator<Item = T>) -> Self {
            let mut s = Self::new_sorted();
            for v in items {
                s.insert(v);
            }
            s
        }
        pub fn len(&self) -> usize {
            self.0.len()
        }
        pub fn is_empty(&self) -> bool {
            self.0.is_empty()
        }
        pub fn clear(&mut self) {
            self.0.clear();
        }
        /// True when it wasn't there yet, as Java's add.
        pub fn insert(&mut self, v: T) -> bool {
            self.0.insert(v, ()).is_none()
        }
        pub fn contains<Q: std::hash::Hash + Eq + ?Sized>(&self, v: &Q) -> bool
        where
            T: std::borrow::Borrow<Q>,
        {
            self.0.contains_key(v)
        }
        pub fn remove<Q: std::hash::Hash + Eq + ?Sized>(&mut self, v: &Q) -> bool
        where
            T: std::borrow::Borrow<Q>,
        {
            self.0.remove(v).is_some()
        }
        pub fn iter(&self) -> impl Iterator<Item = &T> {
            self.0.keys()
        }
        pub fn clone_pairs(&self) -> Vec<T> {
            self.0.keys().cloned().collect()
        }
    }

    /// The mod's data folder in Horizon, set by the hooks (game_side.rs `ensure_init`) before the mod's code runs.
    pub static J_DATA_DIR: JCell<Option<std::path::PathBuf>> = JCell::new(None);
    /// FabricLoader's config folder: the mod's data folder in Horizon (./config until the game sets it).
    pub fn j_config_dir() -> std::path::PathBuf {
        J_DATA_DIR.get().unwrap_or_else(|| std::path::PathBuf::from("config"))
    }

    /// Minecraft's RandomSource / java.util.Random: other numbers than Java's, the same ranges.
    #[derive(Clone, Debug)]
    pub struct JRandom(u64);
    impl JRandom {
        pub fn new() -> Self {
            let t = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_nanos() as u64).unwrap_or(0x9E37_79B9);
            Self(t | 1)
        }
        fn next(&mut self) -> u64 {
            let mut v = self.0;
            v ^= v << 13;
            v ^= v >> 7;
            v ^= v << 17;
            self.0 = v;
            v
        }
        /// 0 inclusive to n exclusive; Java throws on n <= 0.
        pub fn next_int(&mut self, n: i32) -> i32 {
            assert!(n > 0, "bound must be positive");
            (self.next() % n as u64) as i32
        }
        pub fn next_int_between(&mut self, lo: i32, hi: i32) -> i32 {
            lo + self.next_int(hi - lo)
        }
        pub fn next_float(&mut self) -> f32 {
            (self.next() >> 40) as f32 / (1u64 << 24) as f32
        }
        pub fn next_double(&mut self) -> f64 {
            (self.next() >> 11) as f64 / (1u64 << 53) as f64
        }
        pub fn next_boolean(&mut self) -> bool {
            self.next() & 1 == 1
        }
        pub fn next_gaussian(&mut self) -> f64 {
            let (u, v) = (self.next_double().max(1e-12), self.next_double());
            (-2.0 * u.ln()).sqrt() * (std::f64::consts::TAU * v).cos()
        }
    }

    /// Minecraft's Identifier (`namespace:path`), which is only text.
    #[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct JIdentifier {
        pub namespace: String,
        pub path: String,
    }
    impl JIdentifier {
        pub fn new(namespace: &str, path: &str) -> Self {
            Self { namespace: namespace.to_string(), path: path.to_string() }
        }
        /// `a:b`, or `minecraft:b` without a namespace.
        pub fn parse(s: &str) -> Self {
            match s.split_once(':') {
                Some((n, p)) => Self::new(n, p),
                None => Self::new("minecraft", s),
            }
        }
        /// Identifier.tryParse: None (null) when it has characters an identifier can't.
        pub fn try_parse(s: &str) -> Option<Self> {
            let id = Self::parse(s);
            Self::try_build(&id.namespace, &id.path)
        }
        pub fn try_build(namespace: &str, path: &str) -> Option<Self> {
            let ns_ok = namespace.chars().all(|c| matches!(c, 'a'..='z' | '0'..='9' | '_' | '.' | '-'));
            let path_ok = path.chars().all(|c| matches!(c, 'a'..='z' | '0'..='9' | '_' | '.' | '-' | '/'));
            if ns_ok && path_ok { Some(Self::new(namespace, path)) } else { None }
        }
    }
    impl std::fmt::Display for JIdentifier {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}:{}", self.namespace, self.path)
        }
    }

    /// Java's String.equals.
    pub fn j_eq(a: &str, b: &str) -> bool {
        a == b
    }
    pub fn j_eq_ignore_case(a: &str, b: &str) -> bool {
        a.to_lowercase() == b.to_lowercase()
    }
    /// Java's String.length (in chars).
    pub fn j_len(s: &str) -> i32 {
        s.chars().count() as i32
    }
    pub fn j_char_at(s: &str, i: i32) -> char {
        s.chars().nth(i as usize).expect("charAt out of range")
    }
    /// Java's substring, by char.
    pub fn j_substring(s: &str, from: i32, to: Option<i32>) -> String {
        let n = s.chars().count() as i32;
        let to = to.unwrap_or(n);
        assert!(0 <= from && from <= to && to <= n, "substring out of range");
        s.chars().skip(from as usize).take((to - from) as usize).collect()
    }
    pub fn j_index_of(s: &str, sub: &str) -> i32 {
        s.find(sub).map_or(-1, |b| s[..b].chars().count() as i32)
    }
    pub fn j_index_of_char(s: &str, c: char) -> i32 {
        s.chars().position(|x| x == c).map_or(-1, |i| i as i32)
    }
    pub fn j_last_index_of(s: &str, sub: &str) -> i32 {
        s.rfind(sub).map_or(-1, |b| s[..b].chars().count() as i32)
    }
    /// Java's String.split on plain text: trailing empty parts dropped.
    pub fn j_split(s: &str, sep: &str) -> Vec<String> {
        let mut v: Vec<String> = s.split(sep).map(str::to_string).collect();
        while v.len() > 1 && v.last().is_some_and(|p| p.is_empty()) {
            v.pop();
        }
        v
    }
    /// Java's String.split(sep, limit) on plain text.
    pub fn j_split_limit(s: &str, sep: &str, limit: i32) -> Vec<String> {
        if limit == 0 {
            return j_split(s, sep);
        }
        if limit > 0 {
            return s.splitn(limit as usize, sep).map(str::to_string).collect();
        }
        s.split(sep).map(str::to_string).collect()
    }
    /// Java's String.compareTo: by UTF-16 unit, then by length.
    pub fn j_compare(a: &str, b: &str) -> i32 {
        let (x, y): (Vec<u16>, Vec<u16>) = (a.encode_utf16().collect(), b.encode_utf16().collect());
        for (p, q) in x.iter().zip(y.iter()) {
            if p != q {
                return *p as i32 - *q as i32;
            }
        }
        x.len() as i32 - y.len() as i32
    }
    /// Java's String.regionMatches, by char.
    pub fn j_region_matches(s: &str, ignore_case: bool, from: i32, other: &str, other_from: i32, len: i32) -> bool {
        let (a, b): (Vec<char>, Vec<char>) = (s.chars().collect(), other.chars().collect());
        if from < 0 || other_from < 0 || from as i64 + len as i64 > a.len() as i64 || other_from as i64 + len as i64 > b.len() as i64 {
            return false;
        }
        (0..len.max(0) as usize).all(|i| {
            let (p, q) = (a[from as usize + i], b[other_from as usize + i]);
            p == q || ignore_case && (j_upper_char(p) == j_upper_char(q) || j_lower_char(p) == j_lower_char(q))
        })
    }
    /// Java's Character.toLowerCase / toUpperCase (one char).
    pub fn j_lower_char(c: char) -> char {
        let mut l = c.to_lowercase();
        if l.len() == 1 { l.next().unwrap_or(c) } else { c }
    }
    pub fn j_upper_char(c: char) -> char {
        let mut u = c.to_uppercase();
        if u.len() == 1 { u.next().unwrap_or(c) } else { c }
    }
    /// Java's Arrays.copyOfRange: past the end is filled with defaults.
    pub fn j_copy_of_range<T: Clone + Default>(a: &[T], from: usize, to: usize) -> Vec<T> {
        assert!(from <= to && from <= a.len(), "copyOfRange out of range");
        let mut v: Vec<T> = a[from..to.min(a.len())].to_vec();
        v.resize(to - from, T::default());
        v
    }
    pub fn j_hash(s: &str) -> i32 {
        s.encode_utf16().fold(0i32, |h, c| h.wrapping_mul(31).wrapping_add(c as i32))
    }
    /// Java's Boolean.parseBoolean.
    pub fn j_parse_bool(s: &str) -> bool {
        s.eq_ignore_ascii_case("true")
    }
    /// A float as Java prints it in "..." + x (1.0, not 1).
    pub fn j_float(x: f64) -> String {
        if x.is_finite() && x == x.trunc() && x.abs() < 1e7 { format!("{x:.1}") } else { format!("{x}") }
    }
    /// Java's Math.round.
    pub fn j_round_f(x: f32) -> i32 {
        (x + 0.5).floor() as i32
    }
    pub fn j_round_d(x: f64) -> i64 {
        (x + 0.5).floor() as i64
    }
    /// Java's Math.clamp / Mth.clamp (no panic when min > max).
    pub fn j_clamp<T: PartialOrd>(v: T, min: T, max: T) -> T {
        if v < min { min } else if v > max { max } else { v }
    }
    pub fn j_lerp<T: Copy + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::ops::Mul<Output = T>>(d: T, a: T, b: T) -> T {
        a + d * (b - a)
    }
    pub fn j_signum_f(x: f32) -> f32 {
        if x > 0.0 { 1.0 } else if x < 0.0 { -1.0 } else { x }
    }
    pub fn j_signum_d(x: f64) -> f64 {
        if x > 0.0 { 1.0 } else if x < 0.0 { -1.0 } else { x }
    }
    pub fn j_wrap_degrees(x: f64) -> f64 {
        let mut v = x % 360.0;
        if v >= 180.0 {
            v -= 360.0;
        }
        if v < -180.0 {
            v += 360.0;
        }
        v
    }
    pub fn j_frac(x: f64) -> f64 {
        x - x.floor()
    }
    pub fn j_compare_f(a: f64, b: f64) -> i32 {
        a.partial_cmp(&b).map_or(0, |o| o as i32)
    }
    pub fn j_copy_of<T: Clone + Default>(a: &[T], n: usize) -> Vec<T> {
        let mut v: Vec<T> = a.iter().take(n).cloned().collect();
        v.resize(n, T::default());
        v
    }
    /// Java's System.currentTimeMillis.
    pub fn j_millis() -> i64 {
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_millis() as i64)
    }
    /// Java's System.nanoTime: only differences mean anything.
    pub fn j_nanos() -> i64 {
        static START: LazyLock<std::time::Instant> = LazyLock::new(std::time::Instant::now);
        START.elapsed().as_nanos() as i64
    }

    /// `Dist` (net/neoforged/api/distmarker/Dist.java)
    pub mod dist {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // Dist.isDedicatedServer(0): not translated, it has no body.
        // Dist.isClient(0): not translated, it has no body.
    }

    /// `Mod` (net/neoforged/fml/common/Mod.java)
    pub mod mod_ {
        use super::*;
        // The class itself isn't translated: it's an annotation. Its static parts are below.
        // Mod.value(0): not translated, it has no body.
        // Mod.dist(0): not translated, it has no body.
    }

    /// `ClientLoader` (team/creative/creativecore/client/ClientLoader.java)
    pub mod client_loader {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ClientLoader.onInitializeClient(0): not translated, it has no body.
        // ClientLoader.registerClientCommands(1): not translated, it's generic.
    }

    /// `GuiClientTreeCheckBox` (team/creative/creativecore/client/config/gui/GuiClientTreeCheckBox.java)
    pub mod gui_client_tree_check_box {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GuiClientTreeCheckBox.GuiClientTreeCheckBox(1): not translated, its class isn't translated (it's generic).
        // GuiClientTreeCheckBox.mouseClicked(3): not translated, its class isn't translated (it's generic).
    }

    /// `CreativeCoreClient` (team/creative/creativecore/client/CreativeCoreClient.java)
    pub mod creative_core_client {
        use super::*;
        // The class itself isn't translated: it implements Fabric's `ClientModInitializer`. Its static parts are below.
        // LOADER: not translated, it uses ICreativeClientLoader, which isn't translated (it's an interface).
        // mc: not translated, it uses Minecraft's `Minecraft`.
        pub static MOD_CONFIGS: LazyLock<JCell<Vec<String>>> = LazyLock::new(|| JCell::new(Vec::new()));
        pub fn get_mod_configs() -> Vec<String> {
            return creative_core_client::MOD_CONFIGS.get();
        }
        pub fn register_client_config(modid: &str) {
            { let a_0 = modid.to_string(); creative_core_client::MOD_CONFIGS.with(|o| o.push(a_0)) };
        }
        // CreativeCoreClient.getFrameTime(0): not translated, it uses CreativeCoreClient.mc, which isn't translated (it uses Minecraft's `Minecraft`).
        // CreativeCoreClient.commands(2): not translated, it uses Minecraft's `CommandDispatcher`.
        // CreativeCoreClient.onInitializeClient(0): not translated, its body couldn't be read: an anonymous class.
        // CreativeCoreClient.clientTick(1): not translated, it uses Minecraft's `Minecraft`.
        // CreativeCoreClient.postBackgroundEvent(4): not translated, it uses Minecraft's `Screen`.
        // CreativeCoreClient.gatherTooltipComponents(7): not translated, it uses Minecraft's `ClientTooltipComponent`.
        // CreativeCoreClient.gatherTooltipComponentsFromElements(6): not translated, it uses Minecraft's `ClientTooltipComponent`.
        // CreativeCoreClient.splitLine(3): not translated, it uses `Stream` (java.util.stream).
        // CreativeCoreClient.makeParticleRenderTypeComparator(1): not translated, it uses `Comparator` (java.util).
        // CreativeCoreClient.loader(0): not translated, it uses ICreativeClientLoader, which isn't translated (it's an interface).
    }

    /// `CreativeFabricClientLoader` (team/creative/creativecore/client/CreativeFabricClientLoader.java)
    pub mod creative_fabric_client_loader {
        use super::*;
        // The class itself isn't translated: it implements `ICreativeClientLoader`. Its static parts are below.
        // CreativeFabricClientLoader.registerKeybind(1): not translated, its class isn't translated (it implements `ICreativeClientLoader`).
        // CreativeFabricClientLoader.registerCategory(1): not translated, its class isn't translated (it implements `ICreativeClientLoader`).
        // CreativeFabricClientLoader.registerModifyTooltip(1): not translated, its class isn't translated (it implements `ICreativeClientLoader`).
        // CreativeFabricClientLoader.registerMenu(2): not translated, it's generic.
        // CreativeFabricClientLoader.addScreenWidget(2): not translated, it's generic.
    }

    /// `GuiClientCheckList` (team/creative/creativecore/client/gui/control/collection/GuiClientCheckList.java)
    pub mod gui_client_check_list {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GuiClientCheckList.GuiClientCheckList(1): not translated, its class isn't translated (it's generic).
        // GuiClientCheckList.flowX(2): not translated, its class isn't translated (it's generic).
        // GuiClientCheckList.flowY(3): not translated, its class isn't translated (it's generic).
        // GuiClientCheckList.reflowInternal(0): not translated, its class isn't translated (it's generic).
    }

    /// `GuiClientComboBox` (team/creative/creativecore/client/gui/control/collection/GuiClientComboBox.java)
    pub mod gui_client_combo_box {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GuiClientComboBox.GuiClientComboBox(1): not translated, its class isn't translated (it's generic).
        // GuiClientComboBox.hasSearchbar(0): not translated, its class isn't translated (it's generic).
        // GuiClientComboBox.updateDisplay(0): not translated, its class isn't translated (it's generic).
        // GuiClientComboBox.setSearchbar(1): not translated, its class isn't translated (it's generic).
        // GuiClientComboBox.setDirection(1): not translated, its class isn't translated (it's generic).
        // GuiClientComboBox.set(2): not translated, its class isn't translated (it's generic).
        // GuiClientComboBox.selected(0): not translated, its class isn't translated (it's generic).
        // GuiClientComboBox.selected(1): not translated, its class isn't translated (it's generic).
        // GuiClientComboBox.select(2): not translated, its class isn't translated (it's generic).
        // GuiClientComboBox.select(2): not translated, its class isn't translated (it's generic).
        // GuiClientComboBox.indexOf(1): not translated, its class isn't translated (it's generic).
        // GuiClientComboBox.next(0): not translated, its class isn't translated (it's generic).
        // GuiClientComboBox.previous(0): not translated, its class isn't translated (it's generic).
        // GuiClientComboBox.selectedIndex(0): not translated, its class isn't translated (it's generic).
        // GuiClientComboBox.flowX(2): not translated, its class isn't translated (it's generic).
        // GuiClientComboBox.flowY(3): not translated, its class isn't translated (it's generic).
        // GuiClientComboBox.preferredWidth(1): not translated, its class isn't translated (it's generic).
        // GuiClientComboBox.preferredHeight(2): not translated, its class isn't translated (it's generic).
        // GuiClientComboBox.mouseClicked(3): not translated, its class isn't translated (it's generic).
        // GuiClientComboBox.defaultFormatting(0): not translated, its class isn't translated (it's generic).
        // GuiClientComboBox.createBox(1): not translated, its class isn't translated (it's generic).
        // GuiClientComboBox.looseFocus(0): not translated, its class isn't translated (it's generic).
        // GuiClientComboBox.lines(0): not translated, its class isn't translated (it's generic).
    }

    /// `GuiClientComboBoxExtension` (team/creative/creativecore/client/gui/control/collection/GuiClientComboBoxExtension.java)
    pub mod gui_client_combo_box_extension {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GuiClientComboBoxExtension.GuiClientComboBoxExtension(1): not translated, its class isn't translated (it's generic).
        // GuiClientComboBoxExtension.init(1): not translated, its class isn't translated (it's generic).
        // GuiClientComboBoxExtension.creator(0): not translated, its class isn't translated (it's generic).
        // GuiClientComboBoxExtension.looseFocus(0): not translated, its class isn't translated (it's generic).
        // GuiClientComboBoxExtension.reloadControls(0): not translated, its class isn't translated (it's generic).
        // GuiClientComboBoxExtension.mouseClicked(3): not translated, its class isn't translated (it's generic).
        // GuiClientComboBoxExtension.maxHeight(2): not translated, its class isn't translated (it's generic).
    }

    /// `GuiClientComboBoxFlexible` (team/creative/creativecore/client/gui/control/collection/GuiClientComboBoxFlexible.java)
    pub mod gui_client_combo_box_flexible {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GuiClientComboBoxFlexible.GuiClientComboBoxFlexible(1): not translated, its class isn't translated (it's generic).
        // GuiClientComboBoxFlexible.init(1): not translated, its class isn't translated (it's generic).
        // GuiClientComboBoxFlexible.updateDisplay(0): not translated, its class isn't translated (it's generic).
        // GuiClientComboBoxFlexible.forceSelect(2): not translated, its class isn't translated (it's generic).
        // GuiClientComboBoxFlexible.select(2): not translated, its class isn't translated (it's generic).
        // GuiClientComboBoxFlexible.selected(0): not translated, its class isn't translated (it's generic).
    }

    /// `GuiClientComboBoxTree` (team/creative/creativecore/client/gui/control/collection/GuiClientComboBoxTree.java)
    pub mod gui_client_combo_box_tree {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GuiClientComboBoxTree.GuiClientComboBoxTree(1): not translated, its class isn't translated (it's generic).
        // GuiClientComboBoxTree.init(1): not translated, its class isn't translated (it's generic).
        // GuiClientComboBoxTree.hasSearchbar(0): not translated, its class isn't translated (it's generic).
        // GuiClientComboBoxTree.setSearchbar(1): not translated, its class isn't translated (it's generic).
        // GuiClientComboBoxTree.setDirection(1): not translated, its class isn't translated (it's generic).
        // GuiClientComboBoxTree.set(2): not translated, its class isn't translated (it's generic).
        // GuiClientComboBoxTree.selected(0): not translated, its class isn't translated (it's generic).
        // GuiClientComboBoxTree.selected(1): not translated, its class isn't translated (it's generic).
        // GuiClientComboBoxTree.select(3): not translated, its class isn't translated (it's generic).
        // GuiClientComboBoxTree.select(2): not translated, its class isn't translated (it's generic).
        // GuiClientComboBoxTree.updateDisplay(0): not translated, its class isn't translated (it's generic).
        // GuiClientComboBoxTree.mouseClicked(3): not translated, its class isn't translated (it's generic).
        // GuiClientComboBoxTree.defaultFormatting(0): not translated, its class isn't translated (it's generic).
        // GuiClientComboBoxTree.createBox(1): not translated, its class isn't translated (it's generic).
        // GuiClientComboBoxTree.looseFocus(0): not translated, its class isn't translated (it's generic).
    }

    /// `GuiClientListBoxBase` (team/creative/creativecore/client/gui/control/collection/GuiClientListBoxBase.java)
    pub mod gui_client_list_box_base {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GuiClientListBoxBase.GuiClientListBoxBase(1): not translated, its class isn't translated (it's generic).
        // GuiClientListBoxBase.flowY(3): not translated, its class isn't translated (it's generic).
        // GuiClientListBoxBase.reflowInternal(0): not translated, its class isn't translated (it's generic).
    }

    /// `GuiClientStackSelector` (team/creative/creativecore/client/gui/control/collection/GuiClientStackSelector.java)
    pub mod gui_client_stack_selector {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GuiClientStackSelector.GuiClientStackSelector(1): not translated, its class isn't translated (it's generic).
        // GuiClientStackSelector.setCollector(1): not translated, its class isn't translated (it's generic).
        // GuiClientStackSelector.hasSearchbar(0): not translated, its class isn't translated (it's generic).
        // GuiClientStackSelector.setSearchbar(1): not translated, its class isn't translated (it's generic).
        // GuiClientStackSelector.selectFirst(1): not translated, its class isn't translated (it's generic).
        // GuiClientStackSelector.updateCollectedStacks(1): not translated, its class isn't translated (it's generic).
        // GuiClientStackSelector.setSelectedForce(2): not translated, its class isn't translated (it's generic).
        // GuiClientStackSelector.setSelected(2): not translated, its class isn't translated (it's generic).
        // GuiClientStackSelector.getStacks(0): not translated, its class isn't translated (it's generic).
        // GuiClientStackSelector.getSelected(0): not translated, its class isn't translated (it's generic).
        // GuiClientStackSelector.mouseClicked(3): not translated, its class isn't translated (it's generic).
        // GuiClientStackSelector.defaultFormatting(0): not translated, its class isn't translated (it's generic).
        // GuiClientStackSelector.createBox(1): not translated, its class isn't translated (it's generic).
        // GuiClientStackSelector.looseFocus(0): not translated, its class isn't translated (it's generic).
    }

    /// `GuiClientStackSelectorExtension` (team/creative/creativecore/client/gui/control/collection/GuiClientStackSelectorExtension.java)
    pub mod gui_client_stack_selector_extension {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GuiClientStackSelectorExtension.GuiClientStackSelectorExtension(1): not translated, its class isn't translated (it's generic).
        // GuiClientStackSelectorExtension.init(1): not translated, its class isn't translated (it's generic).
        // GuiClientStackSelectorExtension.creator(0): not translated, its class isn't translated (it's generic).
        // GuiClientStackSelectorExtension.mouseClicked(3): not translated, its class isn't translated (it's generic).
        // GuiClientStackSelectorExtension.flowX(2): not translated, its class isn't translated (it's generic).
        // GuiClientStackSelectorExtension.reflowInternal(0): not translated, its class isn't translated (it's generic).
        // GuiClientStackSelectorExtension.reloadControls(0): not translated, its class isn't translated (it's generic).
        // GuiClientStackSelectorExtension.looseFocus(0): not translated, its class isn't translated (it's generic).
    }

    /// `GuiFocusControl` (team/creative/creativecore/client/gui/control/GuiFocusControl.java)
    pub mod gui_focus_control {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GuiFocusControl.GuiFocusControl(1): not translated, its class isn't translated (it's generic).
        // GuiFocusControl.isFocused(0): not translated, its class isn't translated (it's generic).
        // GuiFocusControl.focus(0): not translated, its class isn't translated (it's generic).
        // GuiFocusControl.looseFocus(0): not translated, its class isn't translated (it's generic).
        // GuiFocusControl.mouseClicked(3): not translated, its class isn't translated (it's generic).
        // GuiFocusControl.focusChanged(0): not translated, its class isn't translated (it's generic).
    }

    /// `GuiClientInventoryGrid` (team/creative/creativecore/client/gui/control/inventory/GuiClientInventoryGrid.java)
    pub mod gui_client_inventory_grid {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GuiClientInventoryGrid.GuiClientInventoryGrid(1): not translated, its class isn't translated (it's generic).
        // GuiClientInventoryGrid.setGridDim(2): not translated, its class isn't translated (it's generic).
        // GuiClientInventoryGrid.minWidth(1): not translated, its class isn't translated (it's generic).
        // GuiClientInventoryGrid.minHeight(2): not translated, its class isn't translated (it's generic).
        // GuiClientInventoryGrid.flowX(2): not translated, its class isn't translated (it's generic).
        // GuiClientInventoryGrid.flowY(3): not translated, its class isn't translated (it's generic).
        // GuiClientInventoryGrid.preferredWidth(1): not translated, its class isn't translated (it's generic).
        // GuiClientInventoryGrid.preferredHeight(2): not translated, its class isn't translated (it's generic).
        // GuiClientInventoryGrid.defaultFormatting(0): not translated, its class isn't translated (it's generic).
    }

    /// `GuiClientInventoryGridPreview` (team/creative/creativecore/client/gui/control/inventory/GuiClientInventoryGridPreview.java)
    pub mod gui_client_inventory_grid_preview {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GuiClientInventoryGridPreview.GuiClientInventoryGridPreview(1): not translated, its class isn't translated (it's generic).
        // GuiClientInventoryGridPreview.setGridDim(2): not translated, its class isn't translated (it's generic).
        // GuiClientInventoryGridPreview.minWidth(1): not translated, its class isn't translated (it's generic).
        // GuiClientInventoryGridPreview.minHeight(2): not translated, its class isn't translated (it's generic).
        // GuiClientInventoryGridPreview.flowX(2): not translated, its class isn't translated (it's generic).
        // GuiClientInventoryGridPreview.flowY(3): not translated, its class isn't translated (it's generic).
        // GuiClientInventoryGridPreview.preferredWidth(1): not translated, its class isn't translated (it's generic).
        // GuiClientInventoryGridPreview.preferredHeight(2): not translated, its class isn't translated (it's generic).
        // GuiClientInventoryGridPreview.defaultFormatting(0): not translated, its class isn't translated (it's generic).
    }

    /// `GuiClientSlot` (team/creative/creativecore/client/gui/control/inventory/GuiClientSlot.java)
    pub mod gui_client_slot {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GuiClientSlot.GuiClientSlot(1): not translated, its class isn't translated (it's generic).
        // GuiClientSlot.itemManager(0): not translated, its class isn't translated (it's generic).
        // GuiClientSlot.inventory(0): not translated, its class isn't translated (it's generic).
        // GuiClientSlot.changed(0): not translated, its class isn't translated (it's generic).
        // GuiClientSlot.getStackToRender(0): not translated, its class isn't translated (it's generic).
        // GuiClientSlot.keyPressed(1): not translated, its class isn't translated (it's generic).
        // GuiClientSlot.mouseScrolled(3): not translated, its class isn't translated (it's generic).
        // GuiClientSlot.mouseClicked(3): not translated, its class isn't translated (it's generic).
        // GuiClientSlot.mouseMoved(2): not translated, its class isn't translated (it's generic).
        // GuiClientSlot.slot(0): not translated, its class isn't translated (it's generic).
    }

    /// `GuiClientSlotBase` (team/creative/creativecore/client/gui/control/inventory/GuiClientSlotBase.java)
    pub mod gui_client_slot_base {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        pub const SLOT_SIZE: i32 = 18;
        // HOVER: not translated, it uses DisplayColor, which isn't translated (it extends `StyleDisplay`).
        // GuiClientSlotBase.GuiClientSlotBase(1): not translated, its class isn't translated (it's generic).
        // GuiClientSlotBase.getStackToRender(0): not translated, it has no body.
        // GuiClientSlotBase.defaultFormatting(0): not translated, its class isn't translated (it's generic).
        // GuiClientSlotBase.getTooltip(0): not translated, its class isn't translated (it's generic).
        // GuiClientSlotBase.hasTooltipExtraSpaceAfterFirstLine(0): not translated, its class isn't translated (it's generic).
        // GuiClientSlotBase.isHovered(0): not translated, its class isn't translated (it's generic).
        // GuiClientSlotBase.renderContent(6): not translated, its class isn't translated (it's generic).
        // GuiClientSlotBase.renderContent(3): not translated, its class isn't translated (it's generic).
        // GuiClientSlotBase.mouseClicked(3): not translated, its class isn't translated (it's generic).
        // GuiClientSlotBase.flowX(2): not translated, its class isn't translated (it's generic).
        // GuiClientSlotBase.flowY(3): not translated, its class isn't translated (it's generic).
        // GuiClientSlotBase.maxWidth(1): not translated, its class isn't translated (it's generic).
        // GuiClientSlotBase.maxHeight(2): not translated, its class isn't translated (it's generic).
        // GuiClientSlotBase.preferredWidth(1): not translated, its class isn't translated (it's generic).
        // GuiClientSlotBase.preferredHeight(2): not translated, its class isn't translated (it's generic).
        // GuiClientSlotBase.minWidth(1): not translated, its class isn't translated (it's generic).
        // GuiClientSlotBase.minHeight(2): not translated, its class isn't translated (it's generic).
    }

    /// `GuiClientSlotViewer` (team/creative/creativecore/client/gui/control/inventory/GuiClientSlotViewer.java)
    pub mod gui_client_slot_viewer {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GuiClientSlotViewer.GuiClientSlotViewer(1): not translated, its class isn't translated (it's generic).
        // GuiClientSlotViewer.mouseClicked(3): not translated, its class isn't translated (it's generic).
        // GuiClientSlotViewer.getStackToRender(0): not translated, its class isn't translated (it's generic).
    }

    /// `GuiClientMenuRoot` (team/creative/creativecore/client/gui/control/menu/GuiClientMenuRoot.java)
    pub mod gui_client_menu_root {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GuiClientMenuRoot.GuiClientMenuRoot(1): not translated, its class isn't translated (it's generic).
        // GuiClientMenuRoot.set(2): not translated, its class isn't translated (it's generic).
        // GuiClientMenuRoot.isRoot(0): not translated, its class isn't translated (it's generic).
        // GuiClientMenuRoot.root(0): not translated, its class isn't translated (it's generic).
        // GuiClientMenuRoot.parentCreator(0): not translated, its class isn't translated (it's generic).
        // GuiClientMenuRoot.translate(2): not translated, its class isn't translated (it's generic).
        // GuiClientMenuRoot.select(2): not translated, its class isn't translated (it's generic).
    }

    /// `GuiClientMenuSub` (team/creative/creativecore/client/gui/control/menu/GuiClientMenuSub.java)
    pub mod gui_client_menu_sub {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GuiClientMenuSub.GuiClientMenuSub(1): not translated, its class isn't translated (it's generic).
        // GuiClientMenuSub.isRoot(0): not translated, its class isn't translated (it's generic).
        // GuiClientMenuSub.setRoot(1): not translated, its class isn't translated (it's generic).
        // GuiClientMenuSub.root(0): not translated, its class isn't translated (it's generic).
        // GuiClientMenuSub.parentCreator(0): not translated, its class isn't translated (it's generic).
    }

    /// `GuiClientPanel` (team/creative/creativecore/client/gui/control/parent/GuiClientPanel.java)
    pub mod gui_client_panel {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GuiClientPanel.GuiClientPanel(1): not translated, its class isn't translated (it's generic).
        // GuiClientPanel.defaultFormatting(0): not translated, its class isn't translated (it's generic).
    }

    /// `GuiClientScrollX` (team/creative/creativecore/client/gui/control/parent/GuiClientScrollX.java)
    pub mod gui_client_scroll_x {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GuiClientScrollX.GuiClientScrollX(1): not translated, its class isn't translated (it's generic).
        // GuiClientScrollX.setHovered(0): not translated, its class isn't translated (it's generic).
        // GuiClientScrollX.setHover(1): not translated, its class isn't translated (it's generic).
        // GuiClientScrollX.getOffsetX(0): not translated, its class isn't translated (it's generic).
        // GuiClientScrollX.defaultFormatting(0): not translated, its class isn't translated (it's generic).
        // GuiClientScrollX.onScrolled(0): not translated, its class isn't translated (it's generic).
        // GuiClientScrollX.mouseScrolled(3): not translated, its class isn't translated (it's generic).
        // GuiClientScrollX.scroll(1): not translated, its class isn't translated (it's generic).
        // GuiClientScrollX.mouseClicked(3): not translated, its class isn't translated (it's generic).
        // GuiClientScrollX.mouseMoved(2): not translated, its class isn't translated (it's generic).
        // GuiClientScrollX.mouseReleased(3): not translated, its class isn't translated (it's generic).
        // GuiClientScrollX.needsScrollbar(0): not translated, its class isn't translated (it's generic).
        // GuiClientScrollX.renderContent(8): not translated, its class isn't translated (it's generic).
        // GuiClientScrollX.minWidth(1): not translated, its class isn't translated (it's generic).
        // GuiClientScrollX.minHeight(2): not translated, its class isn't translated (it's generic).
        // GuiClientScrollX.endlessX(0): not translated, its class isn't translated (it's generic).
        // GuiClientScrollX.flowX(2): not translated, its class isn't translated (it's generic).
        // GuiClientScrollX.flowY(3): not translated, its class isn't translated (it's generic).
    }

    /// `GuiClientScrollXY` (team/creative/creativecore/client/gui/control/parent/GuiClientScrollXY.java)
    pub mod gui_client_scroll_xy {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GuiClientScrollXY.GuiClientScrollXY(1): not translated, its class isn't translated (it's generic).
        // GuiClientScrollXY.getOffsetX(0): not translated, its class isn't translated (it's generic).
        // GuiClientScrollXY.getOffsetY(0): not translated, its class isn't translated (it's generic).
        // GuiClientScrollXY.defaultFormatting(0): not translated, its class isn't translated (it's generic).
        // GuiClientScrollXY.onScrolledX(0): not translated, its class isn't translated (it's generic).
        // GuiClientScrollXY.onScrolledY(0): not translated, its class isn't translated (it's generic).
        // GuiClientScrollXY.mouseScrolled(3): not translated, its class isn't translated (it's generic).
        // GuiClientScrollXY.scroll(1): not translated, its class isn't translated (it's generic).
        // GuiClientScrollXY.mouseClicked(3): not translated, its class isn't translated (it's generic).
        // GuiClientScrollXY.mouseMoved(2): not translated, its class isn't translated (it's generic).
        // GuiClientScrollXY.mouseReleased(3): not translated, its class isn't translated (it's generic).
        // GuiClientScrollXY.needsScrollbarX(0): not translated, its class isn't translated (it's generic).
        // GuiClientScrollXY.needsScrollbarY(0): not translated, its class isn't translated (it's generic).
        // GuiClientScrollXY.renderContent(8): not translated, its class isn't translated (it's generic).
        // GuiClientScrollXY.minWidth(1): not translated, its class isn't translated (it's generic).
        // GuiClientScrollXY.minHeight(2): not translated, its class isn't translated (it's generic).
        // GuiClientScrollXY.flowX(2): not translated, its class isn't translated (it's generic).
        // GuiClientScrollXY.updateWidth(0): not translated, its class isn't translated (it's generic).
        // GuiClientScrollXY.flowY(3): not translated, its class isn't translated (it's generic).
        // GuiClientScrollXY.updateHeight(0): not translated, its class isn't translated (it's generic).
    }

    /// `GuiClientScrollY` (team/creative/creativecore/client/gui/control/parent/GuiClientScrollY.java)
    pub mod gui_client_scroll_y {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GuiClientScrollY.GuiClientScrollY(1): not translated, its class isn't translated (it's generic).
        // GuiClientScrollY.setScrollWhenCTRL(0): not translated, its class isn't translated (it's generic).
        // GuiClientScrollY.setHovered(0): not translated, its class isn't translated (it's generic).
        // GuiClientScrollY.setHover(1): not translated, its class isn't translated (it's generic).
        // GuiClientScrollY.getOffsetY(0): not translated, its class isn't translated (it's generic).
        // GuiClientScrollY.defaultFormatting(0): not translated, its class isn't translated (it's generic).
        // GuiClientScrollY.onScrolled(0): not translated, its class isn't translated (it's generic).
        // GuiClientScrollY.mouseScrolled(3): not translated, its class isn't translated (it's generic).
        // GuiClientScrollY.scroll(1): not translated, its class isn't translated (it's generic).
        // GuiClientScrollY.mouseClicked(3): not translated, its class isn't translated (it's generic).
        // GuiClientScrollY.mouseMoved(2): not translated, its class isn't translated (it's generic).
        // GuiClientScrollY.mouseReleased(3): not translated, its class isn't translated (it's generic).
        // GuiClientScrollY.needsScrollbar(0): not translated, its class isn't translated (it's generic).
        // GuiClientScrollY.renderContent(8): not translated, its class isn't translated (it's generic).
        // GuiClientScrollY.minWidth(1): not translated, its class isn't translated (it's generic).
        // GuiClientScrollY.minHeight(2): not translated, its class isn't translated (it's generic).
        // GuiClientScrollY.flowX(2): not translated, its class isn't translated (it's generic).
        // GuiClientScrollY.flowY(3): not translated, its class isn't translated (it's generic).
        // GuiClientScrollY.endlessY(0): not translated, its class isn't translated (it's generic).
    }

    /// `GuiClientTable` (team/creative/creativecore/client/gui/control/parent/GuiClientTable.java)
    pub mod gui_client_table {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GuiClientTable.GuiClientTable(1): not translated, its class isn't translated (it's generic).
        // GuiClientTable.flowX(2): not translated, its class isn't translated (it's generic).
    }

    /// `GuiClientTable.GuiTableGroup` (team/creative/creativecore/client/gui/control/parent/GuiClientTable.java)
    pub mod gui_client_table_gui_table_group {
        use super::*;
        // The class itself isn't translated: it extends `GuiControlRect` (team.creative.creativecore.client.gui). Its static parts are below.
        // GuiClientTable.GuiTableGroup.GuiTableGroup(0): not translated, its class isn't translated (it extends `GuiControlRect` (team.creative.creativecore.client.gui)).
        // GuiClientTable.GuiTableGroup.getMinWidth(1): not translated, its class isn't translated (it extends `GuiControlRect` (team.creative.creativecore.client.gui)).
        // GuiClientTable.GuiTableGroup.getMaxWidth(1): not translated, its class isn't translated (it extends `GuiControlRect` (team.creative.creativecore.client.gui)).
        // GuiClientTable.GuiTableGroup.getPreferredWidth(1): not translated, its class isn't translated (it extends `GuiControlRect` (team.creative.creativecore.client.gui)).
        // GuiClientTable.GuiTableGroup.getMinHeight(1): not translated, its class isn't translated (it extends `GuiControlRect` (team.creative.creativecore.client.gui)).
        // GuiClientTable.GuiTableGroup.getMaxHeight(1): not translated, its class isn't translated (it extends `GuiControlRect` (team.creative.creativecore.client.gui)).
        // GuiClientTable.GuiTableGroup.getPreferredHeight(1): not translated, its class isn't translated (it extends `GuiControlRect` (team.creative.creativecore.client.gui)).
        // GuiClientTable.GuiTableGroup.setWidth(2): not translated, its class isn't translated (it extends `GuiControlRect` (team.creative.creativecore.client.gui)).
        // GuiClientTable.GuiTableGroup.setHeight(2): not translated, its class isn't translated (it extends `GuiControlRect` (team.creative.creativecore.client.gui)).
        // GuiClientTable.GuiTableGroup.setX(1): not translated, its class isn't translated (it extends `GuiControlRect` (team.creative.creativecore.client.gui)).
        // GuiClientTable.GuiTableGroup.setY(1): not translated, its class isn't translated (it extends `GuiControlRect` (team.creative.creativecore.client.gui)).
        // GuiClientTable.GuiTableGroup.flowX(0): not translated, its class isn't translated (it extends `GuiControlRect` (team.creative.creativecore.client.gui)).
        // GuiClientTable.GuiTableGroup.flowY(0): not translated, its class isn't translated (it extends `GuiControlRect` (team.creative.creativecore.client.gui)).
        // GuiClientTable.GuiTableGroup.isExpandableX(0): not translated, its class isn't translated (it extends `GuiControlRect` (team.creative.creativecore.client.gui)).
        // GuiClientTable.GuiTableGroup.isExpandableY(0): not translated, its class isn't translated (it extends `GuiControlRect` (team.creative.creativecore.client.gui)).
    }

    /// `GuiClientTableScrollable` (team/creative/creativecore/client/gui/control/parent/GuiClientTableScrollable.java)
    pub mod gui_client_table_scrollable {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GuiClientTableScrollable.GuiClientTableScrollable(1): not translated, its class isn't translated (it's generic).
        // GuiClientTableScrollable.createScrollBox(1): not translated, its body couldn't be read: `var`.
        // GuiClientTableScrollable.createTopRow(1): not translated, its body couldn't be read: an anonymous class.
        // GuiClientTableScrollable.createFirstCol(1): not translated, its body couldn't be read: an anonymous class.
        // GuiClientTableScrollable.createCols(0): not translated, its class isn't translated (it's generic).
        // GuiClientTableScrollable.preferredWidth(1): not translated, its class isn't translated (it's generic).
        // GuiClientTableScrollable.flowX(2): not translated, its class isn't translated (it's generic).
        // GuiClientTableScrollable.createRows(0): not translated, its class isn't translated (it's generic).
        // GuiClientTableScrollable.preferredHeight(2): not translated, its class isn't translated (it's generic).
        // GuiClientTableScrollable.flowY(3): not translated, its class isn't translated (it's generic).
        // GuiClientTableScrollable.endlessX(0): not translated, its class isn't translated (it's generic).
        // GuiClientTableScrollable.endlessY(0): not translated, its class isn't translated (it's generic).
    }

    /// `GuiClientTabs` (team/creative/creativecore/client/gui/control/parent/GuiClientTabs.java)
    pub mod gui_client_tabs {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GuiClientTabs.GuiClientTabs(1): not translated, its class isn't translated (it's generic).
        // GuiClientTabs.init(2): not translated, its class isn't translated (it's generic).
        // GuiClientTabs.flowX(2): not translated, its class isn't translated (it's generic).
        // GuiClientTabs.flowY(3): not translated, its class isn't translated (it's generic).
        // GuiClientTabs.defaultFormatting(0): not translated, its class isn't translated (it's generic).
        // GuiClientTabs.select(2): not translated, its class isn't translated (it's generic).
        // GuiClientTabs.index(0): not translated, its class isn't translated (it's generic).
        // GuiClientTabs.createSpecialParent(1): not translated, its class isn't translated (it's generic).
    }

    /// `GuiClientTabs.GuiClientTabSpecialParent` (team/creative/creativecore/client/gui/control/parent/GuiClientTabs.java)
    pub mod gui_client_tabs_gui_client_tab_special_parent {
        use super::*;
        // The class itself isn't translated: it extends `GuiClientParent` (team.creative.creativecore.client.gui). Its static parts are below.
        // GuiClientTabs.GuiClientTabSpecialParent.GuiClientTabSpecialParent(1): not translated, its class isn't translated (it extends `GuiClientParent` (team.creative.creativecore.client.gui)).
        // GuiClientTabs.GuiClientTabSpecialParent.getControlFormatting(0): not translated, its class isn't translated (it extends `GuiClientParent` (team.creative.creativecore.client.gui)).
        // GuiClientTabs.GuiClientTabSpecialParent.preferredWidth(1): not translated, its class isn't translated (it extends `GuiClientParent` (team.creative.creativecore.client.gui)).
        // GuiClientTabs.GuiClientTabSpecialParent.preferredWidthOriginal(1): not translated, its class isn't translated (it extends `GuiClientParent` (team.creative.creativecore.client.gui)).
        // GuiClientTabs.GuiClientTabSpecialParent.preferredHeight(2): not translated, its class isn't translated (it extends `GuiClientParent` (team.creative.creativecore.client.gui)).
        // GuiClientTabs.GuiClientTabSpecialParent.preferredHeightOriginal(2): not translated, its class isn't translated (it extends `GuiClientParent` (team.creative.creativecore.client.gui)).
    }

    /// `GuiClientArraySlider` (team/creative/creativecore/client/gui/control/simple/GuiClientArraySlider.java)
    pub mod gui_client_array_slider {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GuiClientArraySlider.GuiClientArraySlider(1): not translated, its class isn't translated (it's generic).
        // GuiClientArraySlider.getTextByValue(0): not translated, its class isn't translated (it's generic).
        // GuiClientArraySlider.select(2): not translated, its class isn't translated (it's generic).
        // GuiClientArraySlider.setValues(2): not translated, its class isn't translated (it's generic).
        // GuiClientArraySlider.get(0): not translated, its class isn't translated (it's generic).
    }

    /// `GuiClientButton` (team/creative/creativecore/client/gui/control/simple/GuiClientButton.java)
    pub mod gui_client_button {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GuiClientButton.GuiClientButton(1): not translated, its class isn't translated (it's generic).
        // GuiClientButton.setHoverEffect(1): not translated, its class isn't translated (it's generic).
        // GuiClientButton.renderContent(3): not translated, its class isn't translated (it's generic).
        // GuiClientButton.mouseClicked(3): not translated, its class isn't translated (it's generic).
        // GuiClientButton.defaultFormatting(0): not translated, its class isn't translated (it's generic).
        // GuiClientButton.setPressed(1): not translated, its class isn't translated (it's generic).
    }

    /// `GuiClientButtonContext` (team/creative/creativecore/client/gui/control/simple/GuiClientButtonContext.java)
    pub mod gui_client_button_context {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GuiClientButtonContext.GuiClientButtonContext(1): not translated, its class isn't translated (it's generic).
        // GuiClientButtonContext.set(2): not translated, its class isn't translated (it's generic).
        // GuiClientButtonContext.mouseMoved(2): not translated, its class isn't translated (it's generic).
        // GuiClientButtonContext.createBox(1): not translated, its class isn't translated (it's generic).
    }

    /// `GuiClientButtonContext.GuiButtonContextMenu` (team/creative/creativecore/client/gui/control/simple/GuiClientButtonContext.java)
    pub mod gui_client_button_context_gui_button_context_menu {
        use super::*;
        // The class itself isn't translated: it extends `GuiParent` (team.creative.creativecore.common.gui). Its static parts are below.
        // GuiClientButtonContext.GuiButtonContextMenu.GuiButtonContextMenu(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
    }

    /// `GuiClientButtonContext.GuiClientButtonContextMenu` (team/creative/creativecore/client/gui/control/simple/GuiClientButtonContext.java)
    pub mod gui_client_button_context_gui_client_button_context_menu {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GuiClientButtonContext.GuiClientButtonContextMenu.GuiClientButtonContextMenu(1): not translated, its class isn't translated (it's generic).
        // GuiClientButtonContext.GuiClientButtonContextMenu.mouseMoved(2): not translated, its class isn't translated (it's generic).
        // GuiClientButtonContext.GuiClientButtonContextMenu.defaultFormatting(0): not translated, its class isn't translated (it's generic).
    }

    /// `GuiClientButtonHold` (team/creative/creativecore/client/gui/control/simple/GuiClientButtonHold.java)
    pub mod gui_client_button_hold {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        pub const INITIAL_WAIT: i32 = 250;
        pub const CONTINOUS_WAIT: i32 = 100;
        // GuiClientButtonHold.GuiClientButtonHold(1): not translated, its class isn't translated (it's generic).
        // GuiClientButtonHold.mouseClicked(3): not translated, its class isn't translated (it's generic).
        // GuiClientButtonHold.mouseReleased(3): not translated, its class isn't translated (it's generic).
        // GuiClientButtonHold.tick(0): not translated, its class isn't translated (it's generic).
        // GuiClientButtonHold.mouseMoved(2): not translated, its class isn't translated (it's generic).
    }

    /// `GuiClientButtonIcon` (team/creative/creativecore/client/gui/control/simple/GuiClientButtonIcon.java)
    pub mod gui_client_button_icon {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GuiClientButtonIcon.GuiClientButtonIcon(1): not translated, its class isn't translated (it's generic).
        // GuiClientButtonIcon.getBackground(2): not translated, its class isn't translated (it's generic).
        // GuiClientButtonIcon.preferredWidth(1): not translated, its class isn't translated (it's generic).
        // GuiClientButtonIcon.preferredHeight(2): not translated, its class isn't translated (it's generic).
        // GuiClientButtonIcon.mouseClicked(3): not translated, its class isn't translated (it's generic).
        // GuiClientButtonIcon.setPressed(1): not translated, its class isn't translated (it's generic).
    }

    /// `GuiClientCheckBox` (team/creative/creativecore/client/gui/control/simple/GuiClientCheckBox.java)
    pub mod gui_client_check_box {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        pub const CHECKBOX_WIDTH: i32 = 7;
        // PARTIAL_STYLE: not translated, it creates a DisplayColor, which isn't translated (it extends `StyleDisplay`).
        // GuiClientCheckBox.GuiClientCheckBox(1): not translated, its class isn't translated (it's generic).
        // GuiClientCheckBox.setPartial(2): not translated, its class isn't translated (it's generic).
        // GuiClientCheckBox.getPartial(0): not translated, its class isn't translated (it's generic).
        // GuiClientCheckBox.preferredWidth(1): not translated, its class isn't translated (it's generic).
        // GuiClientCheckBox.preferredHeight(2): not translated, its class isn't translated (it's generic).
        // GuiClientCheckBox.defaultFormatting(0): not translated, its class isn't translated (it's generic).
        // GuiClientCheckBox.renderContent(3): not translated, its class isn't translated (it's generic).
        // GuiClientCheckBox.consumeChanged(1): not translated, its class isn't translated (it's generic).
        // GuiClientCheckBox.set(2): not translated, its class isn't translated (it's generic).
        // GuiClientCheckBox.get(0): not translated, its class isn't translated (it's generic).
        // GuiClientCheckBox.mouseClicked(3): not translated, its class isn't translated (it's generic).
    }

    /// `GuiClientCheckButtonIcon` (team/creative/creativecore/client/gui/control/simple/GuiClientCheckButtonIcon.java)
    pub mod gui_client_check_button_icon {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GuiClientCheckButtonIcon.GuiClientCheckButtonIcon(1): not translated, its class isn't translated (it's generic).
        // GuiClientCheckButtonIcon.setOnIcon(1): not translated, its class isn't translated (it's generic).
        // GuiClientCheckButtonIcon.setOffIcon(1): not translated, its class isn't translated (it's generic).
        // GuiClientCheckButtonIcon.getState(0): not translated, its class isn't translated (it's generic).
        // GuiClientCheckButtonIcon.setState(2): not translated, its class isn't translated (it's generic).
        // GuiClientCheckButtonIcon.mouseClicked(3): not translated, its class isn't translated (it's generic).
    }

    /// `GuiClientColoredSteppedSlider` (team/creative/creativecore/client/gui/control/simple/GuiClientColoredSteppedSlider.java)
    pub mod gui_client_colored_stepped_slider {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GuiClientColoredSteppedSlider.GuiClientColoredSteppedSlider(1): not translated, its class isn't translated (it's generic).
        // GuiClientColoredSteppedSlider.setPicker(1): not translated, its class isn't translated (it's generic).
        // GuiClientColoredSteppedSlider.setPart(1): not translated, its class isn't translated (it's generic).
        // GuiClientColoredSteppedSlider.setValue(2): not translated, its class isn't translated (it's generic).
        // GuiClientColoredSteppedSlider.setValueSilent(1): not translated, its class isn't translated (it's generic).
        // GuiClientColoredSteppedSlider.renderContent(6): not translated, its class isn't translated (it's generic).
    }

    /// `GuiClientColorPlate` (team/creative/creativecore/client/gui/control/simple/GuiClientColorPlate.java)
    pub mod gui_client_color_plate {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GuiClientColorPlate.GuiClientColorPlate(1): not translated, its class isn't translated (it's generic).
        // GuiClientColorPlate.setColor(2): not translated, its class isn't translated (it's generic).
        // GuiClientColorPlate.getColor(0): not translated, its class isn't translated (it's generic).
        // GuiClientColorPlate.defaultFormatting(0): not translated, its class isn't translated (it's generic).
        // GuiClientColorPlate.renderContent(3): not translated, its class isn't translated (it's generic).
        // GuiClientColorPlate.flowX(2): not translated, its class isn't translated (it's generic).
        // GuiClientColorPlate.flowY(3): not translated, its class isn't translated (it's generic).
        // GuiClientColorPlate.preferredWidth(1): not translated, its class isn't translated (it's generic).
        // GuiClientColorPlate.preferredHeight(2): not translated, its class isn't translated (it's generic).
    }

    /// `GuiClientHexButton` (team/creative/creativecore/client/gui/control/simple/GuiClientHexButton.java)
    pub mod gui_client_hex_button {
        use super::*;
        // The class itself isn't translated: it extends `GuiClientButton`. Its static parts are below.
        // GuiClientHexButton.GuiClientHexButton(1): not translated, its class isn't translated (it extends `GuiClientButton`).
        // GuiClientHexButton.setConsumer(2): not translated, its class isn't translated (it extends `GuiClientButton`).
        // GuiClientHexButton.colorChanged(0): not translated, its class isn't translated (it extends `GuiClientButton`).
        // GuiClientHexButton.createBox(1): not translated, its class isn't translated (it extends `GuiClientButton`).
    }

    /// `GuiClientIcon` (team/creative/creativecore/client/gui/control/simple/GuiClientIcon.java)
    pub mod gui_client_icon {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GuiClientIcon.GuiClientIcon(1): not translated, its class isn't translated (it's generic).
        // GuiClientIcon.setIcon(1): not translated, its class isn't translated (it's generic).
        // GuiClientIcon.setColor(1): not translated, its class isn't translated (it's generic).
        // GuiClientIcon.setShadow(1): not translated, its class isn't translated (it's generic).
        // GuiClientIcon.setSquared(1): not translated, its class isn't translated (it's generic).
        // GuiClientIcon.flowX(2): not translated, its class isn't translated (it's generic).
        // GuiClientIcon.flowY(3): not translated, its class isn't translated (it's generic).
        // GuiClientIcon.preferredWidth(1): not translated, its class isn't translated (it's generic).
        // GuiClientIcon.preferredHeight(2): not translated, its class isn't translated (it's generic).
        // GuiClientIcon.getBackground(2): not translated, its class isn't translated (it's generic).
        // GuiClientIcon.renderContent(3): not translated, its class isn't translated (it's generic).
        // GuiClientIcon.defaultFormatting(0): not translated, its class isn't translated (it's generic).
    }

    /// `GuiClientLabel` (team/creative/creativecore/client/gui/control/simple/GuiClientLabel.java)
    pub mod gui_client_label {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GuiClientLabel.GuiClientLabel(1): not translated, its class isn't translated (it's generic).
        // GuiClientLabel.setDefaultColor(1): not translated, its class isn't translated (it's generic).
        // GuiClientLabel.setDropShadow(1): not translated, its class isn't translated (it's generic).
        // GuiClientLabel.setAlign(1): not translated, its class isn't translated (it's generic).
        // GuiClientLabel.setVAlign(1): not translated, its class isn't translated (it's generic).
        // GuiClientLabel.setTitle(1): not translated, its class isn't translated (it's generic).
        // GuiClientLabel.setTitle(1): not translated, its class isn't translated (it's generic).
        // GuiClientLabel.setText(1): not translated, its class isn't translated (it's generic).
        // GuiClientLabel.setScale(1): not translated, its class isn't translated (it's generic).
        // GuiClientLabel.defaultFormatting(0): not translated, its class isn't translated (it's generic).
        // GuiClientLabel.renderContent(3): not translated, its class isn't translated (it's generic).
        // GuiClientLabel.flowX(2): not translated, its class isn't translated (it's generic).
        // GuiClientLabel.flowY(3): not translated, its class isn't translated (it's generic).
        // GuiClientLabel.minWidth(1): not translated, its class isn't translated (it's generic).
        // GuiClientLabel.preferredWidth(1): not translated, its class isn't translated (it's generic).
        // GuiClientLabel.minHeight(2): not translated, its class isn't translated (it's generic).
        // GuiClientLabel.preferredHeight(2): not translated, its class isn't translated (it's generic).
    }

    /// `GuiClientLabelHover` (team/creative/creativecore/client/gui/control/simple/GuiClientLabelHover.java)
    pub mod gui_client_label_hover {
        use super::*;
        // The class itself isn't translated: it extends `GuiClientLabel`. Its static parts are below.
        // GuiClientLabelHover.GuiClientLabelHover(1): not translated, its class isn't translated (it extends `GuiClientLabel`).
        // GuiClientLabelHover.renderContent(6): not translated, its class isn't translated (it extends `GuiClientLabel`).
    }

    /// `GuiClientListEntry` (team/creative/creativecore/client/gui/control/simple/GuiClientListEntry.java)
    pub mod gui_client_list_entry {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GuiClientListEntry.GuiClientListEntry(1): not translated, its class isn't translated (it's generic).
        // GuiClientListEntry.renderContent(6): not translated, its class isn't translated (it's generic).
        // GuiClientListEntry.mouseClicked(3): not translated, its class isn't translated (it's generic).
    }

    /// `GuiClientProgressbar` (team/creative/creativecore/client/gui/control/simple/GuiClientProgressbar.java)
    pub mod gui_client_progressbar {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GuiClientProgressbar.GuiClientProgressbar(1): not translated, its class isn't translated (it's generic).
        // GuiClientProgressbar.getTooltip(0): not translated, its class isn't translated (it's generic).
        // GuiClientProgressbar.defaultFormatting(0): not translated, its class isn't translated (it's generic).
        // GuiClientProgressbar.renderContent(3): not translated, its class isn't translated (it's generic).
        // GuiClientProgressbar.renderProgress(2): not translated, its class isn't translated (it's generic).
        // GuiClientProgressbar.flowX(2): not translated, its class isn't translated (it's generic).
        // GuiClientProgressbar.flowY(3): not translated, its class isn't translated (it's generic).
        // GuiClientProgressbar.preferredWidth(1): not translated, its class isn't translated (it's generic).
        // GuiClientProgressbar.preferredHeight(2): not translated, its class isn't translated (it's generic).
    }

    /// `GuiClientSeekBar` (team/creative/creativecore/client/gui/control/simple/GuiClientSeekBar.java)
    pub mod gui_client_seek_bar {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GuiClientSeekBar.GuiClientSeekBar(1): not translated, its class isn't translated (it's generic).
        // GuiClientSeekBar.init(3): not translated, its class isn't translated (it's generic).
        // GuiClientSeekBar.setOnTimeUpdate(1): not translated, its class isn't translated (it's generic).
        // GuiClientSeekBar.setOnLastTimeUpdate(1): not translated, its class isn't translated (it's generic).
        // GuiClientSeekBar.setPosition(2): not translated, its class isn't translated (it's generic).
        // GuiClientSeekBar.renderContent(3): not translated, its class isn't translated (it's generic).
        // GuiClientSeekBar.renderProgress(2): not translated, its class isn't translated (it's generic).
        // GuiClientSeekBar.mouseClicked(3): not translated, its class isn't translated (it's generic).
        // GuiClientSeekBar.mouseMoved(2): not translated, its class isn't translated (it's generic).
        // GuiClientSeekBar.mouseReleased(3): not translated, its class isn't translated (it's generic).
        // GuiClientSeekBar.tick(0): not translated, its class isn't translated (it's generic).
        // GuiClientSeekBar.defaultFormatting(0): not translated, its class isn't translated (it's generic).
        // GuiClientSeekBar.flowX(2): not translated, its class isn't translated (it's generic).
        // GuiClientSeekBar.flowY(3): not translated, its class isn't translated (it's generic).
        // GuiClientSeekBar.preferredWidth(1): not translated, its class isn't translated (it's generic).
        // GuiClientSeekBar.preferredHeight(2): not translated, its class isn't translated (it's generic).
    }

    /// `GuiClientShowItem` (team/creative/creativecore/client/gui/control/simple/GuiClientShowItem.java)
    pub mod gui_client_show_item {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GuiClientShowItem.GuiClientShowItem(1): not translated, its class isn't translated (it's generic).
        // GuiClientShowItem.flowX(2): not translated, its class isn't translated (it's generic).
        // GuiClientShowItem.flowY(3): not translated, its class isn't translated (it's generic).
        // GuiClientShowItem.preferredWidth(1): not translated, its class isn't translated (it's generic).
        // GuiClientShowItem.preferredHeight(2): not translated, its class isn't translated (it's generic).
        // GuiClientShowItem.defaultFormatting(0): not translated, its class isn't translated (it's generic).
        // GuiClientShowItem.renderContent(3): not translated, its class isn't translated (it's generic).
    }

    /// `GuiClientSlider` (team/creative/creativecore/client/gui/control/simple/GuiClientSlider.java)
    pub mod gui_client_slider {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GuiClientSlider.GuiClientSlider(1): not translated, its class isn't translated (it's generic).
        // GuiClientSlider.getTextByValue(0): not translated, its class isn't translated (it's generic).
        // GuiClientSlider.getTextfieldValue(0): not translated, its class isn't translated (it's generic).
        // GuiClientSlider.getPercentage(0): not translated, its class isn't translated (it's generic).
        // GuiClientSlider.setParser(1): not translated, its class isn't translated (it's generic).
        // GuiClientSlider.setSliderSize(1): not translated, its class isn't translated (it's generic).
        // GuiClientSlider.setMinSlider(1): not translated, its class isn't translated (it's generic).
        // GuiClientSlider.setMaxSlider(1): not translated, its class isn't translated (it's generic).
        // GuiClientSlider.textfield(0): not translated, its class isn't translated (it's generic).
        // GuiClientSlider.setEnabled(1): not translated, its class isn't translated (it's generic).
        // GuiClientSlider.mouseClicked(3): not translated, its class isn't translated (it's generic).
        // GuiClientSlider.createTextfield(0): not translated, its class isn't translated (it's generic).
        // GuiClientSlider.closeTextField(0): not translated, its class isn't translated (it's generic).
        // GuiClientSlider.keyPressed(1): not translated, its class isn't translated (it's generic).
        // GuiClientSlider.charTyped(1): not translated, its class isn't translated (it's generic).
        // GuiClientSlider.setBounds(3): not translated, its class isn't translated (it's generic).
        // GuiClientSlider.setMaxValue(1): not translated, its class isn't translated (it's generic).
        // GuiClientSlider.setMinValue(1): not translated, its class isn't translated (it's generic).
        // GuiClientSlider.setValue(2): not translated, its class isn't translated (it's generic).
        // GuiClientSlider.getValue(0): not translated, its class isn't translated (it's generic).
        // GuiClientSlider.getMinValue(0): not translated, its class isn't translated (it's generic).
        // GuiClientSlider.getMaxValue(0): not translated, its class isn't translated (it's generic).
        // GuiClientSlider.defaultFormatting(0): not translated, its class isn't translated (it's generic).
        // GuiClientSlider.mouseMoved(2): not translated, its class isn't translated (it's generic).
        // GuiClientSlider.looseFocus(0): not translated, its class isn't translated (it's generic).
        // GuiClientSlider.mouseReleased(3): not translated, its class isn't translated (it's generic).
        // GuiClientSlider.renderContent(6): not translated, its class isn't translated (it's generic).
        // GuiClientSlider.renderContent(3): not translated, its class isn't translated (it's generic).
        // GuiClientSlider.flowX(2): not translated, its class isn't translated (it's generic).
        // GuiClientSlider.flowY(3): not translated, its class isn't translated (it's generic).
        // GuiClientSlider.preferredWidth(1): not translated, its class isn't translated (it's generic).
        // GuiClientSlider.preferredHeight(2): not translated, its class isn't translated (it's generic).
    }

    /// `GuiClientStateButton` (team/creative/creativecore/client/gui/control/simple/GuiClientStateButton.java)
    pub mod gui_client_state_button {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GuiClientStateButton.GuiClientStateButton(1): not translated, its class isn't translated (it's generic).
        // GuiClientStateButton.set(2): not translated, its class isn't translated (it's generic).
        // GuiClientStateButton.flowX(2): not translated, its class isn't translated (it's generic).
        // GuiClientStateButton.flowY(3): not translated, its class isn't translated (it's generic).
        // GuiClientStateButton.preferredWidth(1): not translated, its class isn't translated (it's generic).
        // GuiClientStateButton.preferredHeight(2): not translated, its class isn't translated (it's generic).
        // GuiClientStateButton.selected(0): not translated, its class isn't translated (it's generic).
        // GuiClientStateButton.selected(1): not translated, its class isn't translated (it's generic).
        // GuiClientStateButton.select(2): not translated, its class isn't translated (it's generic).
        // GuiClientStateButton.select(2): not translated, its class isn't translated (it's generic).
        // GuiClientStateButton.indexOf(1): not translated, its class isn't translated (it's generic).
        // GuiClientStateButton.next(0): not translated, its class isn't translated (it's generic).
        // GuiClientStateButton.previous(0): not translated, its class isn't translated (it's generic).
        // GuiClientStateButton.updateDisplay(0): not translated, its class isn't translated (it's generic).
        // GuiClientStateButton.defaultFormatting(0): not translated, its class isn't translated (it's generic).
    }

    /// `GuiClientStateButtonIcon` (team/creative/creativecore/client/gui/control/simple/GuiClientStateButtonIcon.java)
    pub mod gui_client_state_button_icon {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GuiClientStateButtonIcon.GuiClientStateButtonIcon(1): not translated, its class isn't translated (it's generic).
        // GuiClientStateButtonIcon.getState(0): not translated, its class isn't translated (it's generic).
        // GuiClientStateButtonIcon.setIcons(2): not translated, its class isn't translated (it's generic).
        // GuiClientStateButtonIcon.setState(2): not translated, its class isn't translated (it's generic).
        // GuiClientStateButtonIcon.previousState(0): not translated, its class isn't translated (it's generic).
        // GuiClientStateButtonIcon.nextState(0): not translated, its class isn't translated (it's generic).
        // GuiClientStateButtonIcon.mouseClicked(3): not translated, its class isn't translated (it's generic).
    }

    /// `GuiClientSteppedSlider` (team/creative/creativecore/client/gui/control/simple/GuiClientSteppedSlider.java)
    pub mod gui_client_stepped_slider {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GuiClientSteppedSlider.GuiClientSteppedSlider(1): not translated, its class isn't translated (it's generic).
        // GuiClientSteppedSlider.setSteppedParser(1): not translated, its class isn't translated (it's generic).
        // GuiClientSteppedSlider.mouseScrolled(3): not translated, its class isn't translated (it's generic).
        // GuiClientSteppedSlider.getTextByValue(0): not translated, its class isn't translated (it's generic).
        // GuiClientSteppedSlider.getTextfieldValue(0): not translated, its class isn't translated (it's generic).
        // GuiClientSteppedSlider.mouseMoved(2): not translated, its class isn't translated (it's generic).
        // GuiClientSteppedSlider.createTextfield(0): not translated, its class isn't translated (it's generic).
        // GuiClientSteppedSlider.getIntValue(0): not translated, its class isn't translated (it's generic).
        // GuiClientSteppedSlider.getIntMaxValue(0): not translated, its class isn't translated (it's generic).
        // GuiClientSteppedSlider.getIntMinValue(0): not translated, its class isn't translated (it's generic).
    }

    /// `GuiClientTabButton` (team/creative/creativecore/client/gui/control/simple/GuiClientTabButton.java)
    pub mod gui_client_tab_button {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GuiClientTabButton.GuiClientTabButton(1): not translated, its class isn't translated (it's generic).
        // GuiClientTabButton.set(2): not translated, its class isn't translated (it's generic).
        // GuiClientTabButton.selected(0): not translated, its class isn't translated (it's generic).
        // GuiClientTabButton.selected(1): not translated, its class isn't translated (it's generic).
        // GuiClientTabButton.select(2): not translated, its class isn't translated (it's generic).
        // GuiClientTabButton.select(2): not translated, its class isn't translated (it's generic).
        // GuiClientTabButton.indexOf(1): not translated, its class isn't translated (it's generic).
        // GuiClientTabButton.next(0): not translated, its class isn't translated (it's generic).
        // GuiClientTabButton.previous(0): not translated, its class isn't translated (it's generic).
        // GuiClientTabButton.index(0): not translated, its class isn't translated (it's generic).
    }

    /// `GuiClientTextfield` (team/creative/creativecore/client/gui/control/simple/GuiClientTextfield.java)
    pub mod gui_client_textfield {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GuiClientTextfield.GuiClientTextfield(1): not translated, its class isn't translated (it's generic).
        // GuiClientTextfield.setFloatOnly(0): not translated, its class isn't translated (it's generic).
        // GuiClientTextfield.setNumbersIncludingNegativeOnly(0): not translated, its class isn't translated (it's generic).
        // GuiClientTextfield.setNumbersOnly(0): not translated, its class isn't translated (it's generic).
        // GuiClientTextfield.setHexOnly(0): not translated, its class isn't translated (it's generic).
        // GuiClientTextfield.setText(3): not translated, its class isn't translated (it's generic).
        // GuiClientTextfield.getText(0): not translated, its class isn't translated (it's generic).
        // GuiClientTextfield.onTextChanged(1): not translated, its class isn't translated (it's generic).
        // GuiClientTextfield.setSuggestion(1): not translated, its class isn't translated (it's generic).
        // GuiClientTextfield.getHighlighted(0): not translated, its class isn't translated (it's generic).
        // GuiClientTextfield.defaultFormatting(0): not translated, its class isn't translated (it's generic).
        // GuiClientTextfield.renderContent(3): not translated, its class isn't translated (it's generic).
        // GuiClientTextfield.renderContent(6): not translated, its class isn't translated (it's generic).
        // GuiClientTextfield.writeText(1): not translated, its class isn't translated (it's generic).
        // GuiClientTextfield.deleteText(2): not translated, its class isn't translated (it's generic).
        // GuiClientTextfield.deleteWords(1): not translated, its class isn't translated (it's generic).
        // GuiClientTextfield.deleteChars(1): not translated, its class isn't translated (it's generic).
        // GuiClientTextfield.deleteCharsToPos(1): not translated, its class isn't translated (it's generic).
        // GuiClientTextfield.getWordPosition(1): not translated, its class isn't translated (it's generic).
        // GuiClientTextfield.getWordPosition(2): not translated, its class isn't translated (it's generic).
        // GuiClientTextfield.getWordPosition(3): not translated, its class isn't translated (it's generic).
        // GuiClientTextfield.moveCursor(2): not translated, its class isn't translated (it's generic).
        // GuiClientTextfield.getCursorPos(1): not translated, its class isn't translated (it's generic).
        // GuiClientTextfield.moveCursorTo(2): not translated, its class isn't translated (it's generic).
        // GuiClientTextfield.setCursorPosition(1): not translated, its class isn't translated (it's generic).
        // GuiClientTextfield.moveCursorToStart(1): not translated, its class isn't translated (it's generic).
        // GuiClientTextfield.moveCursorToEnd(1): not translated, its class isn't translated (it's generic).
        // GuiClientTextfield.keyPressed(1): not translated, its class isn't translated (it's generic).
        // GuiClientTextfield.canWrite(0): not translated, its class isn't translated (it's generic).
        // GuiClientTextfield.charTyped(1): not translated, its class isn't translated (it's generic).
        // GuiClientTextfield.findClickedPositionInText(2): not translated, its class isn't translated (it's generic).
        // GuiClientTextfield.mouseClicked(3): not translated, its class isn't translated (it's generic).
        // GuiClientTextfield.mouseDoubleClicked(3): not translated, its class isn't translated (it's generic).
        // GuiClientTextfield.mouseDragged(6): not translated, its class isn't translated (it's generic).
        // GuiClientTextfield.testForDoubleClick(3): not translated, its class isn't translated (it's generic).
        // GuiClientTextfield.getCursorPosition(0): not translated, its class isn't translated (it's generic).
        // GuiClientTextfield.flowX(2): not translated, its class isn't translated (it's generic).
        // GuiClientTextfield.flowY(3): not translated, its class isn't translated (it's generic).
        // GuiClientTextfield.preferredHeight(2): not translated, its class isn't translated (it's generic).
        // GuiClientTextfield.preferredWidth(1): not translated, its class isn't translated (it's generic).
        // GuiClientTextfield.setHighlightPos(1): not translated, its class isn't translated (it's generic).
        // GuiClientTextfield.scrollTo(1): not translated, its class isn't translated (it's generic).
        // GuiClientTextfield.setValidator(1): not translated, its class isn't translated (it's generic).
        // GuiClientTextfield.setMaxStringLength(1): not translated, its class isn't translated (it's generic).
    }

    /// `GuiClientTimeline` (team/creative/creativecore/client/gui/control/timeline/GuiClientTimeline.java)
    pub mod gui_client_timeline {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        pub const MAXIMUM_ZOOM: f64 = 10.0f64;
        // CURSOR_HIGHLIGHT: not translated, it creates a DisplayColor, which isn't translated (it extends `StyleDisplay`).
        // GuiClientTimeline.GuiClientTimeline(1): not translated, its class isn't translated (it's generic).
        // GuiClientTimeline.scrolledX(0): not translated, its class isn't translated (it's generic).
        // GuiClientTimeline.afterDurationSet(0): not translated, its class isn't translated (it's generic).
        // GuiClientTimeline.resized(1): not translated, its class isn't translated (it's generic).
        // GuiClientTimeline.updateTickWidth(0): not translated, its class isn't translated (it's generic).
        // GuiClientTimeline.setLastZoom(1): not translated, its class isn't translated (it's generic).
        // GuiClientTimeline.getLastZoom(0): not translated, its class isn't translated (it's generic).
        // GuiClientTimeline.adjustKeyPositionX(1): not translated, its class isn't translated (it's generic).
        // GuiClientTimeline.adjustKeysPositionX(0): not translated, its class isn't translated (it's generic).
        // GuiClientTimeline.mouseClicked(3): not translated, its class isn't translated (it's generic).
        // GuiClientTimeline.render(6): not translated, its class isn't translated (it's generic).
        // GuiClientTimeline.mouseScrolled(3): not translated, its class isn't translated (it's generic).
        // GuiClientTimeline.scrolled(3): not translated, its class isn't translated (it's generic).
        // GuiClientTimeline.getTickWidth(0): not translated, its class isn't translated (it's generic).
        // GuiClientTimeline.getTickWidthAimed(0): not translated, its class isn't translated (it's generic).
        // GuiClientTimeline.getTimeAt(1): not translated, its class isn't translated (it's generic).
        // GuiClientTimeline.getTimeAtAimed(1): not translated, its class isn't translated (it's generic).
    }

    /// `GuiClientTimelineChannel` (team/creative/creativecore/client/gui/control/timeline/GuiClientTimelineChannel.java)
    pub mod gui_client_timeline_channel {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GuiClientTimelineChannel.GuiClientTimelineChannel(1): not translated, its class isn't translated (it's generic).
        // GuiClientTimelineChannel.timeline(0): not translated, its class isn't translated (it's generic).
        // GuiClientTimelineChannel.keyInitialFlow(1): not translated, its class isn't translated (it's generic).
        // GuiClientTimelineChannel.dragKey(1): not translated, its class isn't translated (it's generic).
        // GuiClientTimelineChannel.flowX(2): not translated, its class isn't translated (it's generic).
        // GuiClientTimelineChannel.defaultFormatting(0): not translated, its class isn't translated (it's generic).
        // GuiClientTimelineChannel.getOffsetX(0): not translated, its class isn't translated (it's generic).
        // GuiClientTimelineChannel.mouseMoved(2): not translated, its class isn't translated (it's generic).
        // GuiClientTimelineChannel.mouseReleased(3): not translated, its class isn't translated (it's generic).
        // GuiClientTimelineChannel.mouseScrolled(3): not translated, its class isn't translated (it's generic).
        // GuiClientTimelineChannel.mouseClicked(3): not translated, its class isn't translated (it's generic).
    }

    /// `GuiClientTimelineHeader` (team/creative/creativecore/client/gui/control/timeline/GuiClientTimelineHeader.java)
    pub mod gui_client_timeline_header {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GuiClientTimelineHeader.GuiClientTimelineHeader(1): not translated, its class isn't translated (it's generic).
        // GuiClientTimelineHeader.timeline(0): not translated, its class isn't translated (it's generic).
        // GuiClientTimelineHeader.handler(0): not translated, its class isn't translated (it's generic).
        // GuiClientTimelineHeader.mouseClicked(3): not translated, its class isn't translated (it's generic).
        // GuiClientTimelineHeader.mouseMoved(2): not translated, its class isn't translated (it's generic).
        // GuiClientTimelineHeader.mouseReleased(3): not translated, its class isn't translated (it's generic).
        // GuiClientTimelineHeader.mouseScrolled(3): not translated, its class isn't translated (it's generic).
        // GuiClientTimelineHeader.renderContent(3): not translated, its class isn't translated (it's generic).
        // GuiClientTimelineHeader.flowX(2): not translated, its class isn't translated (it's generic).
    }

    /// `GuiClientTimelineKey` (team/creative/creativecore/client/gui/control/timeline/GuiClientTimelineKey.java)
    pub mod gui_client_timeline_key {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        pub const DRAG_TIME: f64 = 2.0f64;
        // GuiClientTimelineKey.GuiClientTimelineKey(1): not translated, its class isn't translated (it's generic).
        // GuiClientTimelineKey.createChildRect(4): not translated, its class isn't translated (it's generic).
        // GuiClientTimelineKey.mouseClicked(3): not translated, its class isn't translated (it's generic).
        // GuiClientTimelineKey.mouseDragged(6): not translated, its class isn't translated (it's generic).
        // GuiClientTimelineKey.mouseReleased(3): not translated, its class isn't translated (it's generic).
        // GuiClientTimelineKey.flowX(2): not translated, its class isn't translated (it's generic).
        // GuiClientTimelineKey.flowY(3): not translated, its class isn't translated (it's generic).
        // GuiClientTimelineKey.preferredWidth(1): not translated, its class isn't translated (it's generic).
        // GuiClientTimelineKey.preferredHeight(2): not translated, its class isn't translated (it's generic).
        // GuiClientTimelineKey.defaultFormatting(0): not translated, its class isn't translated (it's generic).
        // GuiClientTimelineKey.render(6): not translated, its class isn't translated (it's generic).
        // GuiClientTimelineKey.renderContent(3): not translated, its class isn't translated (it's generic).
        // GuiClientTimelineKey.setSelected(1): not translated, its class isn't translated (it's generic).
    }

    /// `GuiClientTree` (team/creative/creativecore/client/gui/control/tree/GuiClientTree.java)
    pub mod gui_client_tree {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        pub const LEVEL_SPACING: i32 = 10;
        // GuiClientTree.GuiClientTree(1): not translated, its class isn't translated (it's generic).
        // GuiClientTree.setKeepOrder(1): not translated, its class isn't translated (it's generic).
        // GuiClientTree.keepOrder(0): not translated, its class isn't translated (it's generic).
        // GuiClientTree.reflowTree(0): not translated, its class isn't translated (it's generic).
        // GuiClientTree.setLineThickness(1): not translated, its class isn't translated (it's generic).
        // GuiClientTree.offsetByLevel(1): not translated, its class isn't translated (it's generic).
        // GuiClientTree.preferredWidth(1): not translated, its class isn't translated (it's generic).
        // GuiClientTree.flowX(2): not translated, its class isn't translated (it's generic).
        // GuiClientTree.flowY(3): not translated, its class isn't translated (it's generic).
        // GuiClientTree.renderContent(8): not translated, its body couldn't be read: a labelled statement.
        // GuiClientTree.createPosition(3): not translated, its class isn't translated (it's generic).
        // GuiClientTree.calculatePosition(2): not translated, its class isn't translated (it's generic).
        // GuiClientTree.mouseClicked(3): not translated, its class isn't translated (it's generic).
        // GuiClientTree.isDragged(0): not translated, its class isn't translated (it's generic).
        // GuiClientTree.startDrag(1): not translated, its class isn't translated (it's generic).
        // GuiClientTree.endDrag(0): not translated, its class isn't translated (it's generic).
    }

    /// `GuiClientTree.GuiTreeLine` (team/creative/creativecore/client/gui/control/tree/GuiClientTree.java)
    pub mod gui_client_tree_gui_tree_line {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct GuiTreeLine {
            pub x: i32,
            pub y: i32,
            pub y2: i32,
            pub invalid: bool,
        }
        impl GuiTreeLine {
            // GuiClientTree.GuiTreeLine.GuiTreeLine(3): not translated, unknown static GuiClientTree.this.
            pub fn set(&mut self, min_y: i32, max_y: i32) {
                self.y = min_y;
                self.y2 = max_y;
                self.invalid = false;
            }
            // GuiClientTree.GuiTreeLine.render(1): not translated, it uses Minecraft's `GuiGraphicsExtractor`.
        }
    }

    /// `GuiClientTreeItem` (team/creative/creativecore/client/gui/control/tree/GuiClientTreeItem.java)
    pub mod gui_client_tree_item {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GuiClientTreeItem.GuiClientTreeItem(1): not translated, its class isn't translated (it's generic).
        // GuiClientTreeItem.mouseMoved(2): not translated, its class isn't translated (it's generic).
        // GuiClientTreeItem.mouseReleased(3): not translated, its class isn't translated (it's generic).
        // GuiClientTreeItem.mouseClicked(3): not translated, its class isn't translated (it's generic).
        // GuiClientTreeItem.mouseDoubleClicked(3): not translated, its class isn't translated (it's generic).
        // GuiClientTreeItem.testForDoubleClick(3): not translated, its class isn't translated (it's generic).
        // GuiClientTreeItem.defaultFormatting(0): not translated, its class isn't translated (it's generic).
    }

    /// `GuiClientTreeItem.ItemClickState` (team/creative/creativecore/client/gui/control/tree/GuiClientTreeItem.java)
    pub mod gui_client_tree_item_item_click_state {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum ItemClickState {
            Clicked,
            Dragged,
        }
        impl ItemClickState {
            pub const VALUES: [ItemClickState; 2] = [ItemClickState::Clicked, ItemClickState::Dragged, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    ItemClickState::Clicked => "CLICKED",
                    ItemClickState::Dragged => "DRAGGED",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "CLICKED" => ItemClickState::Clicked,
                    "DRAGGED" => ItemClickState::Dragged,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `GuiExtensionCreator` (team/creative/creativecore/client/gui/extension/GuiExtensionCreator.java)
    pub mod gui_extension_creator {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GuiExtensionCreator.GuiExtensionCreator(1): not translated, its class isn't translated (it's generic).
        // GuiExtensionCreator.toggle(1): not translated, its class isn't translated (it's generic).
        // GuiExtensionCreator.toggle(2): not translated, its class isn't translated (it's generic).
        // GuiExtensionCreator.open(1): not translated, its class isn't translated (it's generic).
        // GuiExtensionCreator.open(2): not translated, its class isn't translated (it's generic).
        // GuiExtensionCreator.open(3): not translated, its class isn't translated (it's generic).
        // GuiExtensionCreator.get(0): not translated, its class isn't translated (it's generic).
        // GuiExtensionCreator.close(0): not translated, its class isn't translated (it's generic).
        // GuiExtensionCreator.hasLostFocus(0): not translated, its class isn't translated (it's generic).
        // GuiExtensionCreator.hasExtension(0): not translated, its class isn't translated (it's generic).
        // GuiExtensionCreator.checkShouldClose(0): not translated, its class isn't translated (it's generic).
        // GuiExtensionCreator.markLostFocus(0): not translated, its class isn't translated (it's generic).
        // GuiExtensionCreator.markKeptFocus(0): not translated, its class isn't translated (it's generic).
    }

    /// `GuiExtensionCreator.ExtensionDirection` (team/creative/creativecore/client/gui/extension/GuiExtensionCreator.java)
    pub mod gui_extension_creator_extension_direction {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // GuiExtensionCreator.ExtensionDirection.apply(4): not translated, it has no body.
    }

    /// `GuiClientControl` (team/creative/creativecore/client/gui/GuiClientControl.java)
    pub mod gui_client_control {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GuiClientControl.getScreenRect(0): not translated, it uses Minecraft's `Minecraft`.
        // GuiClientControl.scissor(2): not translated, it uses Minecraft's `GuiGraphicsExtractor`.
        // GuiClientControl.GuiClientControl(1): not translated, its class isn't translated (it's generic).
        // GuiClientControl.getParent(0): not translated, its class isn't translated (it's generic).
        // GuiClientControl.setVisible(1): not translated, its class isn't translated (it's generic).
        // GuiClientControl.setFixed(0): not translated, its class isn't translated (it's generic).
        // GuiClientControl.setFixedX(0): not translated, its class isn't translated (it's generic).
        // GuiClientControl.setFixedY(0): not translated, its class isn't translated (it's generic).
        // GuiClientControl.setExpandable(0): not translated, its class isn't translated (it's generic).
        // GuiClientControl.setExpandableX(0): not translated, its class isn't translated (it's generic).
        // GuiClientControl.setExpandableY(0): not translated, its class isn't translated (it's generic).
        // GuiClientControl.setDim(2): not translated, its class isn't translated (it's generic).
        // GuiClientControl.setDim(1): not translated, its class isn't translated (it's generic).
        // GuiClientControl.setEnabled(1): not translated, its class isn't translated (it's generic).
        // GuiClientControl.removeFormatting(0): not translated, its class isn't translated (it's generic).
        // GuiClientControl.setFormatting(1): not translated, its class isn't translated (it's generic).
        // GuiClientControl.setCustomFormatting(0): not translated, its class isn't translated (it's generic).
        // GuiClientControl.isExpandableX(0): not translated, its class isn't translated (it's generic).
        // GuiClientControl.isExpandableY(0): not translated, its class isn't translated (it's generic).
        // GuiClientControl.getStyle(0): not translated, its class isn't translated (it's generic).
        // GuiClientControl.raiseEvent(1): not translated, its class isn't translated (it's generic).
        // GuiClientControl.createChildRect(4): not translated, its class isn't translated (it's generic).
        // GuiClientControl.flowX(2): not translated, it has no body.
        // GuiClientControl.flowY(3): not translated, it has no body.
        // GuiClientControl.minWidth(1): not translated, its class isn't translated (it's generic).
        // GuiClientControl.preferredWidth(1): not translated, it has no body.
        // GuiClientControl.maxWidth(1): not translated, its class isn't translated (it's generic).
        // GuiClientControl.minHeight(2): not translated, its class isn't translated (it's generic).
        // GuiClientControl.preferredHeight(2): not translated, it has no body.
        // GuiClientControl.maxHeight(2): not translated, its class isn't translated (it's generic).
        // GuiClientControl.testForDoubleClick(3): not translated, its class isn't translated (it's generic).
        // GuiClientControl.isInteractable(0): not translated, its class isn't translated (it's generic).
        // GuiClientControl.mouseMoved(2): not translated, its class isn't translated (it's generic).
        // GuiClientControl.mouseClicked(3): not translated, its class isn't translated (it's generic).
        // GuiClientControl.mouseDoubleClicked(3): not translated, its class isn't translated (it's generic).
        // GuiClientControl.mouseReleased(3): not translated, its class isn't translated (it's generic).
        // GuiClientControl.mouseDragged(6): not translated, its class isn't translated (it's generic).
        // GuiClientControl.mouseScrolled(3): not translated, its class isn't translated (it's generic).
        // GuiClientControl.keyPressed(1): not translated, its class isn't translated (it's generic).
        // GuiClientControl.keyReleased(1): not translated, its class isn't translated (it's generic).
        // GuiClientControl.charTyped(1): not translated, its class isn't translated (it's generic).
        // GuiClientControl.looseFocus(0): not translated, its class isn't translated (it's generic).
        // GuiClientControl.defaultFormatting(0): not translated, it has no body.
        // GuiClientControl.getControlFormatting(0): not translated, its class isn't translated (it's generic).
        // GuiClientControl.getContentOffset(0): not translated, its class isn't translated (it's generic).
        // GuiClientControl.getBorder(2): not translated, its class isn't translated (it's generic).
        // GuiClientControl.getBackground(2): not translated, its class isn't translated (it's generic).
        // GuiClientControl.render(6): not translated, its class isn't translated (it's generic).
        // GuiClientControl.renderContent(8): not translated, its class isn't translated (it's generic).
        // GuiClientControl.renderContent(6): not translated, its class isn't translated (it's generic).
        // GuiClientControl.renderContent(3): not translated, it has no body.
        // GuiClientControl.getTooltip(0): not translated, its class isn't translated (it's generic).
        // GuiClientControl.hasTooltipExtraSpaceAfterFirstLine(0): not translated, its class isn't translated (it's generic).
        // GuiClientControl.getTooltipEvent(2): not translated, its class isn't translated (it's generic).
        // GuiClientControl.setTooltip(1): not translated, its class isn't translated (it's generic).
        // GuiClientControl.setTooltip(1): not translated, its class isn't translated (it's generic).
        // GuiClientControl.playSound(1): not translated, its class isn't translated (it's generic).
        // GuiClientControl.playSound(1): not translated, its class isn't translated (it's generic).
        // GuiClientControl.playSound(1): not translated, its class isn't translated (it's generic).
        // GuiClientControl.playSound(3): not translated, its class isn't translated (it's generic).
        // GuiClientControl.playSound(3): not translated, its class isn't translated (it's generic).
    }

    /// `GuiClientLayer` (team/creative/creativecore/client/gui/GuiClientLayer.java)
    pub mod gui_client_layer {
        use super::*;
        // The class itself isn't translated: it extends `GuiClientParent`. Its static parts are below.
        pub const MINIMUM_OUTER_SPACING: i32 = 10;
        // GuiClientLayer.GuiClientLayer(1): not translated, its class isn't translated (it extends `GuiClientParent`).
        // GuiClientLayer.managers(0): not translated, its class isn't translated (it extends `GuiClientParent`).
        // GuiClientLayer.getControlFormatting(0): not translated, its class isn't translated (it extends `GuiClientParent`).
        // GuiClientLayer.getWidth(0): not translated, its class isn't translated (it extends `GuiClientParent`).
        // GuiClientLayer.getHeight(0): not translated, its class isn't translated (it extends `GuiClientParent`).
        // GuiClientLayer.reflow(0): not translated, its class isn't translated (it extends `GuiClientParent`).
        // GuiClientLayer.getStyle(0): not translated, its class isn't translated (it extends `GuiClientParent`).
        // GuiClientLayer.renderContent(6): not translated, its class isn't translated (it extends `GuiClientParent`).
        // GuiClientLayer.renderContent(3): not translated, its class isn't translated (it extends `GuiClientParent`).
        // GuiClientLayer.getSettings(0): not translated, its class isn't translated (it extends `GuiClientParent`).
        // GuiClientLayer.hasGrayBackground(0): not translated, its class isn't translated (it extends `GuiClientParent`).
        // GuiClientLayer.mouseClicked(3): not translated, its class isn't translated (it extends `GuiClientParent`).
        // GuiClientLayer.mouseReleased(3): not translated, its class isn't translated (it extends `GuiClientParent`).
        // GuiClientLayer.keyPressed(1): not translated, its class isn't translated (it extends `GuiClientParent`).
        // GuiClientLayer.applyOffset(2): not translated, its class isn't translated (it extends `GuiClientParent`).
    }

    /// `GuiClientParent` (team/creative/creativecore/client/gui/GuiClientParent.java)
    pub mod gui_client_parent {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GuiClientParent.GuiClientParent(1): not translated, its class isn't translated (it's generic).
        // GuiClientParent.initControlList(1): not translated, its class isn't translated (it's generic).
        // GuiClientParent.controls(0): not translated, its class isn't translated (it's generic).
        // GuiClientParent.hoverControls(0): not translated, its class isn't translated (it's generic).
        // GuiClientParent.all(0): not translated, its class isn't translated (it's generic).
        // GuiClientParent.setScale(1): not translated, its class isn't translated (it's generic).
        // GuiClientParent.scaleFactor(0): not translated, its class isn't translated (it's generic).
        // GuiClientParent.scaleFactorInv(0): not translated, its class isn't translated (it's generic).
        // GuiClientParent.getOffsetY(0): not translated, its class isn't translated (it's generic).
        // GuiClientParent.getOffsetX(0): not translated, its class isn't translated (it's generic).
        // GuiClientParent.setAlign(1): not translated, its class isn't translated (it's generic).
        // GuiClientParent.setVAlign(1): not translated, its class isn't translated (it's generic).
        // GuiClientParent.setSpacing(1): not translated, its class isn't translated (it's generic).
        // GuiClientParent.setFlow(1): not translated, its class isn't translated (it's generic).
        // GuiClientParent.renderControls(10): not translated, its class isn't translated (it's generic).
        // GuiClientParent.renderControl(8): not translated, its class isn't translated (it's generic).
        // GuiClientParent.renderContent(6): not translated, its class isn't translated (it's generic).
        // GuiClientParent.renderContent(3): not translated, its class isn't translated (it's generic).
        // GuiClientParent.isMouseOverHovered(2): not translated, its class isn't translated (it's generic).
        // GuiClientParent.getTooltipEvent(2): not translated, its class isn't translated (it's generic).
        // GuiClientParent.testForDoubleClick(3): not translated, its class isn't translated (it's generic).
        // GuiClientParent.mouseMoved(2): not translated, its class isn't translated (it's generic).
        // GuiClientParent.mouseClicked(3): not translated, its class isn't translated (it's generic).
        // GuiClientParent.mouseDoubleClicked(3): not translated, its class isn't translated (it's generic).
        // GuiClientParent.mouseReleased(3): not translated, its class isn't translated (it's generic).
        // GuiClientParent.mouseDragged(6): not translated, its class isn't translated (it's generic).
        // GuiClientParent.mouseScrolled(3): not translated, its class isn't translated (it's generic).
        // GuiClientParent.keyPressed(1): not translated, its class isn't translated (it's generic).
        // GuiClientParent.keyReleased(1): not translated, its class isn't translated (it's generic).
        // GuiClientParent.charTyped(1): not translated, its class isn't translated (it's generic).
        // GuiClientParent.looseFocus(0): not translated, its class isn't translated (it's generic).
        // GuiClientParent.flowX(2): not translated, its class isn't translated (it's generic).
        // GuiClientParent.flowY(3): not translated, its class isn't translated (it's generic).
        // GuiClientParent.endlessX(0): not translated, its class isn't translated (it's generic).
        // GuiClientParent.endlessY(0): not translated, its class isn't translated (it's generic).
        // GuiClientParent.minWidth(1): not translated, its class isn't translated (it's generic).
        // GuiClientParent.preferredWidth(1): not translated, its class isn't translated (it's generic).
        // GuiClientParent.minHeight(2): not translated, its class isn't translated (it's generic).
        // GuiClientParent.preferredHeight(2): not translated, its class isn't translated (it's generic).
        // GuiClientParent.defaultFormatting(0): not translated, its class isn't translated (it's generic).
        // GuiClientParent.applyOffset(2): not translated, its class isn't translated (it's generic).
    }

    /// `GuiClientRegistrySided` (team/creative/creativecore/client/gui/GuiClientRegistrySided.java)
    pub mod gui_client_registry_sided {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct GuiClientRegistrySided {
        }
        impl GuiClientRegistrySided {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn register() {
        }
    }

    /// `GuiControlRect` (team/creative/creativecore/client/gui/GuiControlRect.java)
    pub mod gui_control_rect {
        use super::*;
        // The class itself isn't translated: it uses GuiClientControl, which isn't translated (it's generic). Its static parts are below.
        // GuiControlRect.getMinWidth(2): not translated, it uses GuiClientControl, which isn't translated (it's generic).
        // GuiControlRect.getPreferredWidth(2): not translated, it uses GuiClientControl, which isn't translated (it's generic).
        // GuiControlRect.getMaxWidth(2): not translated, it uses GuiClientControl, which isn't translated (it's generic).
        // GuiControlRect.getMinHeight(3): not translated, it uses GuiClientControl, which isn't translated (it's generic).
        // GuiControlRect.getPreferredHeight(3): not translated, it uses GuiClientControl, which isn't translated (it's generic).
        // GuiControlRect.getMaxHeight(3): not translated, it uses GuiClientControl, which isn't translated (it's generic).
        // GuiControlRect.GuiControlRect(1): not translated, its class isn't translated (it uses GuiClientControl, which isn't translated (it's generic)).
        // GuiControlRect.getX(0): not translated, its class isn't translated (it uses GuiClientControl, which isn't translated (it's generic)).
        // GuiControlRect.centerX(0): not translated, its class isn't translated (it uses GuiClientControl, which isn't translated (it's generic)).
        // GuiControlRect.setX(1): not translated, its class isn't translated (it uses GuiClientControl, which isn't translated (it's generic)).
        // GuiControlRect.getY(0): not translated, its class isn't translated (it uses GuiClientControl, which isn't translated (it's generic)).
        // GuiControlRect.centerY(0): not translated, its class isn't translated (it uses GuiClientControl, which isn't translated (it's generic)).
        // GuiControlRect.setY(1): not translated, its class isn't translated (it uses GuiClientControl, which isn't translated (it's generic)).
        // GuiControlRect.getWidth(0): not translated, its class isn't translated (it uses GuiClientControl, which isn't translated (it's generic)).
        // GuiControlRect.getHeight(0): not translated, its class isn't translated (it uses GuiClientControl, which isn't translated (it's generic)).
        // GuiControlRect.getBottom(0): not translated, its class isn't translated (it uses GuiClientControl, which isn't translated (it's generic)).
        // GuiControlRect.getRight(0): not translated, its class isn't translated (it uses GuiClientControl, which isn't translated (it's generic)).
        // GuiControlRect.setRight(1): not translated, its class isn't translated (it uses GuiClientControl, which isn't translated (it's generic)).
        // GuiControlRect.setBottom(1): not translated, its class isn't translated (it uses GuiClientControl, which isn't translated (it's generic)).
        // GuiControlRect.getContentWidth(0): not translated, its class isn't translated (it uses GuiClientControl, which isn't translated (it's generic)).
        // GuiControlRect.getContentHeight(0): not translated, its class isn't translated (it uses GuiClientControl, which isn't translated (it's generic)).
        // GuiControlRect.clampWidth(2): not translated, its class isn't translated (it uses GuiClientControl, which isn't translated (it's generic)).
        // GuiControlRect.setWidth(2): not translated, its class isn't translated (it uses GuiClientControl, which isn't translated (it's generic)).
        // GuiControlRect.clampHeight(2): not translated, its class isn't translated (it uses GuiClientControl, which isn't translated (it's generic)).
        // GuiControlRect.setHeight(2): not translated, its class isn't translated (it uses GuiClientControl, which isn't translated (it's generic)).
        // GuiControlRect.addWidth(2): not translated, its class isn't translated (it uses GuiClientControl, which isn't translated (it's generic)).
        // GuiControlRect.addHeight(2): not translated, its class isn't translated (it uses GuiClientControl, which isn't translated (it's generic)).
        // GuiControlRect.isMaxWidth(1): not translated, its class isn't translated (it uses GuiClientControl, which isn't translated (it's generic)).
        // GuiControlRect.isMaxHeight(1): not translated, its class isn't translated (it uses GuiClientControl, which isn't translated (it's generic)).
        // GuiControlRect.getMinWidth(1): not translated, its class isn't translated (it uses GuiClientControl, which isn't translated (it's generic)).
        // GuiControlRect.getMaxWidth(1): not translated, its class isn't translated (it uses GuiClientControl, which isn't translated (it's generic)).
        // GuiControlRect.getPreferredWidth(1): not translated, its class isn't translated (it uses GuiClientControl, which isn't translated (it's generic)).
        // GuiControlRect.getMinHeight(1): not translated, its class isn't translated (it uses GuiClientControl, which isn't translated (it's generic)).
        // GuiControlRect.getMaxHeight(1): not translated, its class isn't translated (it uses GuiClientControl, which isn't translated (it's generic)).
        // GuiControlRect.getPreferredHeight(1): not translated, its class isn't translated (it uses GuiClientControl, which isn't translated (it's generic)).
        // GuiControlRect.flowX(0): not translated, its class isn't translated (it uses GuiClientControl, which isn't translated (it's generic)).
        // GuiControlRect.flowY(0): not translated, its class isn't translated (it uses GuiClientControl, which isn't translated (it's generic)).
        // GuiControlRect.inside(2): not translated, its class isn't translated (it uses GuiClientControl, which isn't translated (it's generic)).
        // GuiControlRect.insideLocalPos(2): not translated, its class isn't translated (it uses GuiClientControl, which isn't translated (it's generic)).
        // GuiControlRect.isExpandableX(0): not translated, its class isn't translated (it uses GuiClientControl, which isn't translated (it's generic)).
        // GuiControlRect.isExpandableY(0): not translated, its class isn't translated (it uses GuiClientControl, which isn't translated (it's generic)).
        // GuiControlRect.rectCopy(0): not translated, its class isn't translated (it uses GuiClientControl, which isn't translated (it's generic)).
        // GuiControlRect.toString(0): not translated, its class isn't translated (it uses GuiClientControl, which isn't translated (it's generic)).
    }

    /// `IGuiClientIntegratedParent` (team/creative/creativecore/client/gui/IGuiClientIntegratedParent.java)
    pub mod i_gui_client_integrated_parent {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IGuiClientIntegratedParent.render(6): not translated, it uses IGuiIntegratedParent, which isn't translated (it's an interface).
        // IGuiClientIntegratedParent.toScreenRect(2): not translated, its class isn't translated (it's an interface).
    }

    /// `ContainerScreenIntegration` (team/creative/creativecore/client/gui/integration/ContainerScreenIntegration.java)
    pub mod container_screen_integration {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `AbstractContainerScreen`. Its static parts are below.
        // ContainerScreenIntegration.ContainerScreenIntegration(2): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerScreen`).
        // ContainerScreenIntegration.init(0): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerScreen`).
        // ContainerScreenIntegration.rebuildWidgets(0): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerScreen`).
        // ContainerScreenIntegration.mouseDragged(3): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerScreen`).
        // ContainerScreenIntegration.extractRenderState(4): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerScreen`).
        // ContainerScreenIntegration.clientTick(0): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerScreen`).
        // ContainerScreenIntegration.getWidth(0): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerScreen`).
        // ContainerScreenIntegration.getHeight(0): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerScreen`).
        // ContainerScreenIntegration.mouseMoved(2): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerScreen`).
        // ContainerScreenIntegration.mouseScrolled(4): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerScreen`).
        // ContainerScreenIntegration.keyPressed(1): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerScreen`).
        // ContainerScreenIntegration.keyReleased(1): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerScreen`).
        // ContainerScreenIntegration.charTyped(1): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerScreen`).
        // ContainerScreenIntegration.provider(0): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerScreen`).
        // ContainerScreenIntegration.getLayers(0): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerScreen`).
        // ContainerScreenIntegration.getTopLayer(0): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerScreen`).
        // ContainerScreenIntegration.openLayer(1): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerScreen`).
        // ContainerScreenIntegration.closeLayer(1): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerScreen`).
        // ContainerScreenIntegration.send(1): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerScreen`).
        // ContainerScreenIntegration.isContainer(0): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerScreen`).
        // ContainerScreenIntegration.isClient(0): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerScreen`).
        // ContainerScreenIntegration.getPlayer(0): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerScreen`).
        // ContainerScreenIntegration.closeTopLayer(0): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerScreen`).
        // ContainerScreenIntegration.closeLayer(1): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerScreen`).
        // ContainerScreenIntegration.createDist(1): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerScreen`).
        // ContainerScreenIntegration.createDist(1): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerScreen`).
    }

    /// `GuiScreenIntegration` (team/creative/creativecore/client/gui/integration/GuiScreenIntegration.java)
    pub mod gui_screen_integration {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Screen`. Its static parts are below.
        // GuiScreenIntegration.GuiScreenIntegration(1): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // GuiScreenIntegration.init(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // GuiScreenIntegration.rebuildWidgets(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // GuiScreenIntegration.getWidth(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // GuiScreenIntegration.getHeight(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // GuiScreenIntegration.clientTick(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // GuiScreenIntegration.removed(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // GuiScreenIntegration.isContainer(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // GuiScreenIntegration.isClient(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // GuiScreenIntegration.getPlayer(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // GuiScreenIntegration.extractRenderState(4): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // GuiScreenIntegration.getLayers(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // GuiScreenIntegration.getTopLayer(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // GuiScreenIntegration.openLayer(1): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // GuiScreenIntegration.closeLayer(1): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // GuiScreenIntegration.closeLayer(1): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // GuiScreenIntegration.closeTopLayer(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // GuiScreenIntegration.mouseMoved(2): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // GuiScreenIntegration.mouseScrolled(4): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // GuiScreenIntegration.send(1): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // GuiScreenIntegration.provider(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // GuiScreenIntegration.createDist(1): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // GuiScreenIntegration.createDist(1): not translated, its class isn't translated (it extends Minecraft's `Screen`).
    }

    /// `ScreenEventListener` (team/creative/creativecore/client/gui/integration/ScreenEventListener.java)
    pub mod screen_event_listener {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `GuiEventListener`. Its static parts are below.
        pub const DOUBLE_CLICK_TIME: f64 = 0.2f64;
        // ScreenEventListener.ScreenEventListener(2): not translated, its class isn't translated (it implements Minecraft's `GuiEventListener`).
        // ScreenEventListener.getTopLayer(0): not translated, its class isn't translated (it implements Minecraft's `GuiEventListener`).
        // ScreenEventListener.getOffsetX(0): not translated, its class isn't translated (it implements Minecraft's `GuiEventListener`).
        // ScreenEventListener.getOffsetY(0): not translated, its class isn't translated (it implements Minecraft's `GuiEventListener`).
        // ScreenEventListener.tick(0): not translated, its class isn't translated (it implements Minecraft's `GuiEventListener`).
        // ScreenEventListener.getEventTime(0): not translated, its class isn't translated (it implements Minecraft's `GuiEventListener`).
        // ScreenEventListener.fireRemaingEvents(0): not translated, its class isn't translated (it implements Minecraft's `GuiEventListener`).
        // ScreenEventListener.mouseMoved(2): not translated, its class isn't translated (it implements Minecraft's `GuiEventListener`).
        // ScreenEventListener.mouseClicked(2): not translated, its class isn't translated (it implements Minecraft's `GuiEventListener`).
        // ScreenEventListener.mouseReleased(1): not translated, its class isn't translated (it implements Minecraft's `GuiEventListener`).
        // ScreenEventListener.mouseDragged(3): not translated, its class isn't translated (it implements Minecraft's `GuiEventListener`).
        // ScreenEventListener.mouseScrolled(4): not translated, its class isn't translated (it implements Minecraft's `GuiEventListener`).
        // ScreenEventListener.keyPressed(1): not translated, its class isn't translated (it implements Minecraft's `GuiEventListener`).
        // ScreenEventListener.keyReleased(1): not translated, its class isn't translated (it implements Minecraft's `GuiEventListener`).
        // ScreenEventListener.charTyped(1): not translated, its class isn't translated (it implements Minecraft's `GuiEventListener`).
        // ScreenEventListener.setFocused(1): not translated, its class isn't translated (it implements Minecraft's `GuiEventListener`).
        // ScreenEventListener.isFocused(0): not translated, its class isn't translated (it implements Minecraft's `GuiEventListener`).
        // ScreenEventListener.isMouseOver(2): not translated, its class isn't translated (it implements Minecraft's `GuiEventListener`).
        // ScreenEventListener.updateNarration(1): not translated, its class isn't translated (it implements Minecraft's `GuiEventListener`).
        // ScreenEventListener.narrationPriority(0): not translated, its class isn't translated (it implements Minecraft's `GuiEventListener`).
        // ScreenEventListener.capturesInput(0): not translated, its class isn't translated (it implements Minecraft's `GuiEventListener`).
    }

    /// `GuiClientManager` (team/creative/creativecore/client/gui/manager/GuiClientManager.java)
    pub mod gui_client_manager {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GuiClientManager.GuiClientManager(1): not translated, its class isn't translated (it's generic).
        // GuiClientManager.renderOverlay(4): not translated, its class isn't translated (it's generic).
        // GuiClientManager.mouseReleased(3): not translated, its class isn't translated (it's generic).
        // GuiClientManager.mouseClickedOutside(2): not translated, its class isn't translated (it's generic).
    }

    /// `GuiClientManagerItem` (team/creative/creativecore/client/gui/manager/GuiClientManagerItem.java)
    pub mod gui_client_manager_item {
        use super::*;
        // The class itself isn't translated: it extends `GuiClientManager`. Its static parts are below.
        // GuiClientManagerItem.freeSpace(2): not translated, it uses Minecraft's `Slot`.
        // GuiClientManagerItem.GuiClientManagerItem(1): not translated, its class isn't translated (it extends `GuiClientManager`).
        // GuiClientManagerItem.getHand(0): not translated, its class isn't translated (it extends `GuiClientManager`).
        // GuiClientManagerItem.setHandChanged(0): not translated, its class isn't translated (it extends `GuiClientManager`).
        // GuiClientManagerItem.mouseClickedOutside(2): not translated, its class isn't translated (it extends `GuiClientManager`).
        // GuiClientManagerItem.mouseReleased(3): not translated, its class isn't translated (it extends `GuiClientManager`).
        // GuiClientManagerItem.renderOverlay(4): not translated, its class isn't translated (it extends `GuiClientManager`).
        // GuiClientManagerItem.startDrag(3): not translated, its class isn't translated (it extends `GuiClientManager`).
        // GuiClientManagerItem.addToDrag(1): not translated, its class isn't translated (it extends `GuiClientManager`).
        // GuiClientManagerItem.modifyDrag(1): not translated, its class isn't translated (it extends `GuiClientManager`).
        // GuiClientManagerItem.abortDrag(0): not translated, its class isn't translated (it extends `GuiClientManager`).
        // GuiClientManagerItem.endDrag(0): not translated, its class isn't translated (it extends `GuiClientManager`).
        // GuiClientManagerItem.isDragged(0): not translated, its class isn't translated (it extends `GuiClientManager`).
        // GuiClientManagerItem.setHand(1): not translated, its class isn't translated (it extends `GuiClientManager`).
        // GuiClientManagerItem.additionalDragCount(1): not translated, its class isn't translated (it extends `GuiClientManager`).
    }

    /// `GuiClientRegistry` (team/creative/creativecore/client/gui/registry/GuiClientRegistry.java)
    pub mod gui_client_registry {
        use super::*;
        // CONTROL_FACTORY: not translated, it uses PairList, which isn't translated (it's generic).
        // BACKUP_CONTROL_FACTORY: not translated, it uses `Object2ObjectMap` (it.unimi.dsi.fastutil.objects).
        // CONTROL_SPECIAL_FACTORY: not translated, it uses GuiControl, which isn't translated (it uses IGuiParent, which isn't translated (it's an interface)).
        // MANAGER_FACTORY: not translated, it uses `Object2ObjectMap` (it.unimi.dsi.fastutil.objects).
        #[derive(Clone, Debug)]
        pub struct GuiClientRegistry {
        }
        impl GuiClientRegistry {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // GuiClientRegistry.create(1): not translated, it uses GuiClientControl, which isn't translated (it's generic).
        // GuiClientRegistry.create(1): not translated, it uses GuiClientControl, which isn't translated (it's generic).
        // GuiClientRegistry.register(2): not translated, it's generic.
        // GuiClientRegistry.registerManager(2): not translated, it's generic.
        // GuiClientRegistry.registerSpecial(1): not translated, it uses GuiClientControl, which isn't translated (it's generic).
    }

    /// `ICreativeClientLoader` (team/creative/creativecore/client/ICreativeClientLoader.java)
    pub mod i_creative_client_loader {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ICreativeClientLoader.registerModifyTooltip(1): not translated, it has no body.
        // ICreativeClientLoader.registerMenu(2): not translated, it has no body.
        // ICreativeClientLoader.addScreenWidget(2): not translated, it has no body.
        // ICreativeClientLoader.registerCategory(1): not translated, it has no body.
        // ICreativeClientLoader.registerKeybind(1): not translated, it has no body.
    }

    /// `ICreativeClientLoader.TooltipModifier` (team/creative/creativecore/client/ICreativeClientLoader.java)
    pub mod i_creative_client_loader_tooltip_modifier {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ICreativeClientLoader.TooltipModifier.modifyTooltip(4): not translated, it has no body.
    }

    /// `ModMenuImpl` (team/creative/creativecore/client/ModMenuImpl.java)
    pub mod mod_menu_impl {
        use super::*;
        // The class itself isn't translated: it implements `ModMenuApi` (com.terraformersmc.modmenu.api). Its static parts are below.
        // ModMenuImpl.getModConfigScreenFactory(0): not translated, its class isn't translated (it implements `ModMenuApi` (com.terraformersmc.modmenu.api)).
        // ModMenuImpl.getConfig(1): not translated, its body couldn't be read: an anonymous class.
        // ModMenuImpl.getProvidedConfigScreenFactories(0): not translated, its class isn't translated (it implements `ModMenuApi` (com.terraformersmc.modmenu.api)).
    }

    /// `QuadGeneratorContext` (team/creative/creativecore/client/render/box/QuadGeneratorContext.java)
    pub mod quad_generator_context {
        use super::*;
        // The class itself isn't translated: it uses RenderBox, which isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box)). Its static parts are below.
        // QuadGeneratorContext.set(4): not translated, its class isn't translated (it uses RenderBox, which isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box))).
        // QuadGeneratorContext.setQuad(3): not translated, its class isn't translated (it uses RenderBox, which isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box))).
        // QuadGeneratorContext.setBounds(6): not translated, its class isn't translated (it uses RenderBox, which isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box))).
        // QuadGeneratorContext.hasBounds(0): not translated, its class isn't translated (it uses RenderBox, which isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box))).
        // QuadGeneratorContext.clear(0): not translated, its class isn't translated (it uses RenderBox, which isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box))).
    }

    /// `RenderBox` (team/creative/creativecore/client/render/box/RenderBox.java)
    pub mod render_box {
        use super::*;
        // The class itself isn't translated: it extends `AlignedBox` (team.creative.creativecore.common.util.math.box). Its static parts are below.
        // DOWN: not translated, it uses VectorFan, which isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)).
        // UP: not translated, it uses VectorFan, which isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)).
        // NORTH: not translated, it uses VectorFan, which isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)).
        // SOUTH: not translated, it uses VectorFan, which isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)).
        // WEST: not translated, it uses VectorFan, which isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)).
        // EAST: not translated, it uses VectorFan, which isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)).
        // RenderBox.RenderBox(1): not translated, its class isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box)).
        // RenderBox.RenderBox(2): not translated, its class isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box)).
        // RenderBox.RenderBox(2): not translated, its class isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box)).
        // RenderBox.RenderBox(2): not translated, its class isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box)).
        // RenderBox.RenderBox(7): not translated, its class isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box)).
        // RenderBox.RenderBox(7): not translated, its class isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box)).
        // RenderBox.setColor(1): not translated, its class isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box)).
        // RenderBox.setKeepUV(1): not translated, its class isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box)).
        // RenderBox.setQuad(2): not translated, its class isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box)).
        // RenderBox.getQuad(1): not translated, its class isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box)).
        // RenderBox.countQuads(0): not translated, its class isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box)).
        // RenderBox.setFace(2): not translated, its class isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box)).
        // RenderBox.getFace(1): not translated, its class isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box)).
        // RenderBox.shouldRenderFace(1): not translated, its class isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box)).
        // RenderBox.intersectsWithFace(3): not translated, its class isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box)).
        // RenderBox.getRenderQuads(1): not translated, its class isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box)).
        // RenderBox.getOffsetX(0): not translated, its class isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box)).
        // RenderBox.getOffsetY(0): not translated, its class isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box)).
        // RenderBox.getOffsetZ(0): not translated, its class isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box)).
        // RenderBox.getOverallScale(1): not translated, its class isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box)).
        // RenderBox.getScaleX(0): not translated, its class isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box)).
        // RenderBox.getScaleY(0): not translated, its class isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box)).
        // RenderBox.getScaleZ(0): not translated, its class isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box)).
        // RenderBox.scaleAndOffsetQuads(1): not translated, its class isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box)).
        // RenderBox.onlyScaleOnceNoOffset(1): not translated, its class isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box)).
        // RenderBox.deleteQuadCache(0): not translated, its class isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box)).
        // RenderBox.previewScalingAndOffset(0): not translated, its class isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box)).
        // RenderBox.getPreviewOffX(0): not translated, its class isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box)).
        // RenderBox.getPreviewOffY(0): not translated, its class isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box)).
        // RenderBox.getPreviewOffZ(0): not translated, its class isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box)).
        // RenderBox.getPreviewScaleX(0): not translated, its class isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box)).
        // RenderBox.getPreviewScaleY(0): not translated, its class isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box)).
        // RenderBox.getPreviewScaleZ(0): not translated, its class isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box)).
        // RenderBox.isTranslucent(0): not translated, its class isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box)).
    }

    /// `RenderBox.VectorFanSimple` (team/creative/creativecore/client/render/box/RenderBox.java)
    pub mod render_box_vector_fan_simple {
        use super::*;
        // The class itself isn't translated: it extends `VectorFan` (team.creative.creativecore.common.util.math.geo). Its static parts are below.
        // RenderBox.VectorFanSimple.VectorFanSimple(1): not translated, its class isn't translated (it extends `VectorFan` (team.creative.creativecore.common.util.math.geo)).
    }

    /// `CreativePlatformHooks` (team/creative/creativecore/client/render/CreativePlatformHooks.java)
    pub mod creative_platform_hooks {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct CreativePlatformHooks {
        }
        impl CreativePlatformHooks {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn backup_render_state() {
        }
        pub fn restore_render_state() {
        }
    }

    /// `RenderBoxFace` (team/creative/creativecore/client/render/face/RenderBoxFace.java)
    pub mod render_box_face {
        use super::*;
        // RENDER: not translated, its value couldn't be read: an anonymous class.
        // NOT_RENDER: not translated, its value couldn't be read: an anonymous class.
        #[derive(Clone, Debug)]
        pub struct RenderBoxFace {
        }
        impl RenderBoxFace {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // RenderBoxFace.shouldRender(0): not translated, it has no body.
            // RenderBoxFace.hasCachedFans(0): not translated, it has no body.
            // RenderBoxFace.getCachedFans(0): not translated, it has no body.
            // RenderBoxFace.getScale(0): not translated, it has no body.
        }
    }

    /// `RenderBoxFace.RenderBoxFaceStatic` (team/creative/creativecore/client/render/face/RenderBoxFace.java)
    pub mod render_box_face_render_box_face_static {
        use super::*;
        // The class itself isn't translated: it extends `RenderBoxFace`. Its static parts are below.
        // RenderBoxFace.RenderBoxFaceStatic.hasCachedFans(0): not translated, its class isn't translated (it extends `RenderBoxFace`).
        // RenderBoxFace.RenderBoxFaceStatic.getCachedFans(0): not translated, its class isn't translated (it extends `RenderBoxFace`).
        // RenderBoxFace.RenderBoxFaceStatic.getScale(0): not translated, its class isn't translated (it extends `RenderBoxFace`).
    }

    /// `RenderBoxFaceSpecial` (team/creative/creativecore/client/render/face/RenderBoxFaceSpecial.java)
    pub mod render_box_face_special {
        use super::*;
        // The class itself isn't translated: it extends `RenderBoxFace`. Its static parts are below.
        // RenderBoxFaceSpecial.RenderBoxFaceSpecial(2): not translated, its class isn't translated (it extends `RenderBoxFace`).
        // RenderBoxFaceSpecial.shouldRender(0): not translated, its class isn't translated (it extends `RenderBoxFace`).
        // RenderBoxFaceSpecial.hasCachedFans(0): not translated, its class isn't translated (it extends `RenderBoxFace`).
        // RenderBoxFaceSpecial.getCachedFans(0): not translated, its class isn't translated (it extends `RenderBoxFace`).
        // RenderBoxFaceSpecial.getScale(0): not translated, its class isn't translated (it extends `RenderBoxFace`).
    }

    /// `VectorFanClient` (team/creative/creativecore/client/render/face/VectorFanClient.java)
    pub mod vector_fan_client {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct VectorFanClient {
        }
        impl VectorFanClient {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // VectorFanClient.setLineNormal(3): not translated, it uses Vec3f, which isn't translated (it extends `VecNf`).
        // VectorFanClient.setLineNormal(7): not translated, it uses Vec3f, which isn't translated (it extends `VecNf`).
        // VectorFanClient.generate(3): not translated, it uses QuadGeneratorContext, which isn't translated (it uses RenderBox, which isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box))).
        // VectorFanClient.generate(7): not translated, it uses QuadGeneratorContext, which isn't translated (it uses RenderBox, which isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box))).
        // VectorFanClient.renderLines(7): not translated, it uses Minecraft's `Pose`.
        // VectorFanClient.renderLines(9): not translated, it uses Minecraft's `Pose`.
        // VectorFanClient.renderLines(13): not translated, it uses Minecraft's `Pose`.
        // VectorFanClient.renderLines(15): not translated, it uses Minecraft's `Pose`.
    }

    /// `ColorRect` (team/creative/creativecore/client/render/gui/ColorRect.java)
    pub mod color_rect {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `GuiElementRenderState`. Its static parts are below.
        // ColorRect.ColorRect(9): not translated, its class isn't translated (it implements Minecraft's `GuiElementRenderState`).
        // ColorRect.buildVertices(1): not translated, its class isn't translated (it implements Minecraft's `GuiElementRenderState`).
        // ColorRect.bounds(0): not translated, its class isn't translated (it implements Minecraft's `GuiElementRenderState`).
        // ColorRect.pipeline(0): not translated, its class isn't translated (it implements Minecraft's `GuiElementRenderState`).
        // ColorRect.textureSetup(0): not translated, its class isn't translated (it implements Minecraft's `GuiElementRenderState`).
        // ColorRect.pose(0): not translated, its class isn't translated (it implements Minecraft's `GuiElementRenderState`).
        // ColorRect.x0(0): not translated, its class isn't translated (it implements Minecraft's `GuiElementRenderState`).
        // ColorRect.y0(0): not translated, its class isn't translated (it implements Minecraft's `GuiElementRenderState`).
        // ColorRect.x1(0): not translated, its class isn't translated (it implements Minecraft's `GuiElementRenderState`).
        // ColorRect.y1(0): not translated, its class isn't translated (it implements Minecraft's `GuiElementRenderState`).
        // ColorRect.color(0): not translated, its class isn't translated (it implements Minecraft's `GuiElementRenderState`).
        // ColorRect.scissorArea(0): not translated, its class isn't translated (it implements Minecraft's `GuiElementRenderState`).
    }

    /// `ColorRectF` (team/creative/creativecore/client/render/gui/ColorRectF.java)
    pub mod color_rect_f {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `GuiElementRenderState`. Its static parts are below.
        // ColorRectF.ColorRectF(9): not translated, its class isn't translated (it implements Minecraft's `GuiElementRenderState`).
        // ColorRectF.buildVertices(1): not translated, its class isn't translated (it implements Minecraft's `GuiElementRenderState`).
        // ColorRectF.bounds(0): not translated, its class isn't translated (it implements Minecraft's `GuiElementRenderState`).
        // ColorRectF.pipeline(0): not translated, its class isn't translated (it implements Minecraft's `GuiElementRenderState`).
        // ColorRectF.textureSetup(0): not translated, its class isn't translated (it implements Minecraft's `GuiElementRenderState`).
        // ColorRectF.pose(0): not translated, its class isn't translated (it implements Minecraft's `GuiElementRenderState`).
        // ColorRectF.x0(0): not translated, its class isn't translated (it implements Minecraft's `GuiElementRenderState`).
        // ColorRectF.y0(0): not translated, its class isn't translated (it implements Minecraft's `GuiElementRenderState`).
        // ColorRectF.x1(0): not translated, its class isn't translated (it implements Minecraft's `GuiElementRenderState`).
        // ColorRectF.y1(0): not translated, its class isn't translated (it implements Minecraft's `GuiElementRenderState`).
        // ColorRectF.color(0): not translated, its class isn't translated (it implements Minecraft's `GuiElementRenderState`).
        // ColorRectF.scissorArea(0): not translated, its class isn't translated (it implements Minecraft's `GuiElementRenderState`).
    }

    /// `CreativeGuiGraphics` (team/creative/creativecore/client/render/gui/CreativeGuiGraphics.java)
    pub mod creative_gui_graphics {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // CreativeGuiGraphics.as(1): not translated, it uses CreativeGuiGraphics, which isn't translated (it's an interface).
        // CreativeGuiGraphics.drawStringCentered(5): not translated, its class isn't translated (it's an interface).
        // CreativeGuiGraphics.drawString(6): not translated, its class isn't translated (it's an interface).
        // CreativeGuiGraphics.horizontalGradientRect(6): not translated, its class isn't translated (it's an interface).
        // CreativeGuiGraphics.horizontalGradientRect(6): not translated, its class isn't translated (it's an interface).
        // CreativeGuiGraphics.verticalGradientRect(6): not translated, its class isn't translated (it's an interface).
        // CreativeGuiGraphics.verticalGradientRect(6): not translated, its class isn't translated (it's an interface).
        // CreativeGuiGraphics.horizontalGradientMaskRect(6): not translated, its class isn't translated (it's an interface).
        // CreativeGuiGraphics.horizontalGradientMaskRect(6): not translated, its class isn't translated (it's an interface).
        // CreativeGuiGraphics.colorRect(5): not translated, its class isn't translated (it's an interface).
        // CreativeGuiGraphics.colorRect(5): not translated, its class isn't translated (it's an interface).
        // CreativeGuiGraphics.textureRect(7): not translated, its class isn't translated (it's an interface).
        // CreativeGuiGraphics.textureRect(7): not translated, its class isn't translated (it's an interface).
        // CreativeGuiGraphics.textureRect(9): not translated, its class isn't translated (it's an interface).
        // CreativeGuiGraphics.textureRect(9): not translated, its class isn't translated (it's an interface).
        // CreativeGuiGraphics.textureRectColor(10): not translated, its class isn't translated (it's an interface).
        // CreativeGuiGraphics.textureRectStrechted(5): not translated, its class isn't translated (it's an interface).
        // CreativeGuiGraphics.textureRectColorStretched(6): not translated, its class isn't translated (it's an interface).
        // CreativeGuiGraphics.renderItemDecorations(3): not translated, its class isn't translated (it's an interface).
        // CreativeGuiGraphics.renderItemDecorations(4): not translated, its class isn't translated (it's an interface).
        // CreativeGuiGraphics.blit(11): not translated, it has no body.
        // CreativeGuiGraphics.as(0): not translated, it has no body.
        // CreativeGuiGraphics.minecraft(0): not translated, it has no body.
        // CreativeGuiGraphics.pose(0): not translated, it has no body.
        // CreativeGuiGraphics.font(0): not translated, it has no body.
        // CreativeGuiGraphics.guiRenderState(0): not translated, it has no body.
        // CreativeGuiGraphics.peekScissor(0): not translated, it has no body.
        // CreativeGuiGraphics.setOverrideScissor(1): not translated, it has no body.
        // CreativeGuiGraphics.clearOverrideScissor(0): not translated, it has no body.
    }

    /// `HorizontalGradientRect` (team/creative/creativecore/client/render/gui/HorizontalGradientRect.java)
    pub mod horizontal_gradient_rect {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `GuiElementRenderState`. Its static parts are below.
        // HorizontalGradientRect.HorizontalGradientRect(10): not translated, its class isn't translated (it implements Minecraft's `GuiElementRenderState`).
        // HorizontalGradientRect.buildVertices(1): not translated, its class isn't translated (it implements Minecraft's `GuiElementRenderState`).
        // HorizontalGradientRect.bounds(0): not translated, its class isn't translated (it implements Minecraft's `GuiElementRenderState`).
        // HorizontalGradientRect.pipeline(0): not translated, its class isn't translated (it implements Minecraft's `GuiElementRenderState`).
        // HorizontalGradientRect.textureSetup(0): not translated, its class isn't translated (it implements Minecraft's `GuiElementRenderState`).
        // HorizontalGradientRect.pose(0): not translated, its class isn't translated (it implements Minecraft's `GuiElementRenderState`).
        // HorizontalGradientRect.x0(0): not translated, its class isn't translated (it implements Minecraft's `GuiElementRenderState`).
        // HorizontalGradientRect.y0(0): not translated, its class isn't translated (it implements Minecraft's `GuiElementRenderState`).
        // HorizontalGradientRect.x1(0): not translated, its class isn't translated (it implements Minecraft's `GuiElementRenderState`).
        // HorizontalGradientRect.y1(0): not translated, its class isn't translated (it implements Minecraft's `GuiElementRenderState`).
        // HorizontalGradientRect.colorFrom(0): not translated, its class isn't translated (it implements Minecraft's `GuiElementRenderState`).
        // HorizontalGradientRect.colorTo(0): not translated, its class isn't translated (it implements Minecraft's `GuiElementRenderState`).
        // HorizontalGradientRect.scissorArea(0): not translated, its class isn't translated (it implements Minecraft's `GuiElementRenderState`).
    }

    /// `HorizontalGradientRectF` (team/creative/creativecore/client/render/gui/HorizontalGradientRectF.java)
    pub mod horizontal_gradient_rect_f {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `GuiElementRenderState`. Its static parts are below.
        // HorizontalGradientRectF.HorizontalGradientRectF(10): not translated, its class isn't translated (it implements Minecraft's `GuiElementRenderState`).
        // HorizontalGradientRectF.buildVertices(1): not translated, its class isn't translated (it implements Minecraft's `GuiElementRenderState`).
        // HorizontalGradientRectF.bounds(0): not translated, its class isn't translated (it implements Minecraft's `GuiElementRenderState`).
        // HorizontalGradientRectF.pipeline(0): not translated, its class isn't translated (it implements Minecraft's `GuiElementRenderState`).
        // HorizontalGradientRectF.textureSetup(0): not translated, its class isn't translated (it implements Minecraft's `GuiElementRenderState`).
        // HorizontalGradientRectF.pose(0): not translated, its class isn't translated (it implements Minecraft's `GuiElementRenderState`).
        // HorizontalGradientRectF.x0(0): not translated, its class isn't translated (it implements Minecraft's `GuiElementRenderState`).
        // HorizontalGradientRectF.y0(0): not translated, its class isn't translated (it implements Minecraft's `GuiElementRenderState`).
        // HorizontalGradientRectF.x1(0): not translated, its class isn't translated (it implements Minecraft's `GuiElementRenderState`).
        // HorizontalGradientRectF.y1(0): not translated, its class isn't translated (it implements Minecraft's `GuiElementRenderState`).
        // HorizontalGradientRectF.colorFrom(0): not translated, its class isn't translated (it implements Minecraft's `GuiElementRenderState`).
        // HorizontalGradientRectF.colorTo(0): not translated, its class isn't translated (it implements Minecraft's `GuiElementRenderState`).
        // HorizontalGradientRectF.scissorArea(0): not translated, its class isn't translated (it implements Minecraft's `GuiElementRenderState`).
    }

    /// `VerticalGradientRect` (team/creative/creativecore/client/render/gui/VerticalGradientRect.java)
    pub mod vertical_gradient_rect {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `GuiElementRenderState`. Its static parts are below.
        // VerticalGradientRect.VerticalGradientRect(10): not translated, its class isn't translated (it implements Minecraft's `GuiElementRenderState`).
        // VerticalGradientRect.buildVertices(1): not translated, its class isn't translated (it implements Minecraft's `GuiElementRenderState`).
        // VerticalGradientRect.bounds(0): not translated, its class isn't translated (it implements Minecraft's `GuiElementRenderState`).
        // VerticalGradientRect.pipeline(0): not translated, its class isn't translated (it implements Minecraft's `GuiElementRenderState`).
        // VerticalGradientRect.textureSetup(0): not translated, its class isn't translated (it implements Minecraft's `GuiElementRenderState`).
        // VerticalGradientRect.pose(0): not translated, its class isn't translated (it implements Minecraft's `GuiElementRenderState`).
        // VerticalGradientRect.x0(0): not translated, its class isn't translated (it implements Minecraft's `GuiElementRenderState`).
        // VerticalGradientRect.y0(0): not translated, its class isn't translated (it implements Minecraft's `GuiElementRenderState`).
        // VerticalGradientRect.x1(0): not translated, its class isn't translated (it implements Minecraft's `GuiElementRenderState`).
        // VerticalGradientRect.y1(0): not translated, its class isn't translated (it implements Minecraft's `GuiElementRenderState`).
        // VerticalGradientRect.colorFrom(0): not translated, its class isn't translated (it implements Minecraft's `GuiElementRenderState`).
        // VerticalGradientRect.colorTo(0): not translated, its class isn't translated (it implements Minecraft's `GuiElementRenderState`).
        // VerticalGradientRect.scissorArea(0): not translated, its class isn't translated (it implements Minecraft's `GuiElementRenderState`).
    }

    /// `VerticalGradientRectF` (team/creative/creativecore/client/render/gui/VerticalGradientRectF.java)
    pub mod vertical_gradient_rect_f {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `GuiElementRenderState`. Its static parts are below.
        // VerticalGradientRectF.VerticalGradientRectF(10): not translated, its class isn't translated (it implements Minecraft's `GuiElementRenderState`).
        // VerticalGradientRectF.buildVertices(1): not translated, its class isn't translated (it implements Minecraft's `GuiElementRenderState`).
        // VerticalGradientRectF.bounds(0): not translated, its class isn't translated (it implements Minecraft's `GuiElementRenderState`).
        // VerticalGradientRectF.pipeline(0): not translated, its class isn't translated (it implements Minecraft's `GuiElementRenderState`).
        // VerticalGradientRectF.textureSetup(0): not translated, its class isn't translated (it implements Minecraft's `GuiElementRenderState`).
        // VerticalGradientRectF.pose(0): not translated, its class isn't translated (it implements Minecraft's `GuiElementRenderState`).
        // VerticalGradientRectF.x0(0): not translated, its class isn't translated (it implements Minecraft's `GuiElementRenderState`).
        // VerticalGradientRectF.y0(0): not translated, its class isn't translated (it implements Minecraft's `GuiElementRenderState`).
        // VerticalGradientRectF.x1(0): not translated, its class isn't translated (it implements Minecraft's `GuiElementRenderState`).
        // VerticalGradientRectF.y1(0): not translated, its class isn't translated (it implements Minecraft's `GuiElementRenderState`).
        // VerticalGradientRectF.colorFrom(0): not translated, its class isn't translated (it implements Minecraft's `GuiElementRenderState`).
        // VerticalGradientRectF.colorTo(0): not translated, its class isn't translated (it implements Minecraft's `GuiElementRenderState`).
        // VerticalGradientRectF.scissorArea(0): not translated, its class isn't translated (it implements Minecraft's `GuiElementRenderState`).
    }

    /// `ScissorStackExtender` (team/creative/creativecore/client/render/ScissorStackExtender.java)
    pub mod scissor_stack_extender {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ScissorStackExtender.setOverrideScissor(1): not translated, it has no body.
        // ScissorStackExtender.clearOverrideScissor(0): not translated, it has no body.
    }

    /// `CompiledText` (team/creative/creativecore/client/render/text/CompiledText.java)
    pub mod compiled_text {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Component`. Its static parts are below.
        // EMPTY: not translated, its value couldn't be read: an anonymous class.
        // CompiledText.createAnySize(0): not translated, it uses CompiledText, which isn't translated (it uses Minecraft's `Component`).
        // CompiledText.width(1): not translated, it uses Minecraft's `FormattedText`.
        // CompiledText.lineHeight(1): not translated, it uses Minecraft's `FormattedText`.
        // CompiledText.CompiledText(2): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // CompiledText.setMaxHeight(1): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // CompiledText.setDimension(2): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // CompiledText.getMaxWidth(0): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // CompiledText.getMaxHeight(0): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // CompiledText.getDefaultColor(0): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // CompiledText.setDefaultColor(1): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // CompiledText.setShadow(1): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // CompiledText.setAlign(1): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // CompiledText.setVAlign(1): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // CompiledText.setScale(1): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // CompiledText.getScale(0): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // CompiledText.setText(1): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // CompiledText.setText(1): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // CompiledText.compile(0): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // CompiledText.compileNext(3): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // CompiledText.compileNext(3): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // CompiledText.compileNext(2): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // CompiledText.getTotalHeight(0): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // CompiledText.render(1): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // CompiledText.splitByWidth(4): not translated, its body couldn't be read: an anonymous class.
        // CompiledText.getTotalWidth(0): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // CompiledText.calculateWidth(3): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // CompiledText.sameDimensions(0): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // CompiledText.copy(0): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // CompiledText.contains(1): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // CompiledText.getUsedWidth(0): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // CompiledText.getUsedHeight(0): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // CompiledText.untrimmedContent(0): not translated, its class isn't translated (it uses Minecraft's `Component`).
    }

    /// `CompiledText.CompiledLine` (team/creative/creativecore/client/render/text/CompiledText.java)
    pub mod compiled_text_compiled_line {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `FormattedText`. Its static parts are below.
        // CompiledText.CompiledLine.contains(1): not translated, its class isn't translated (it uses Minecraft's `FormattedText`).
        // CompiledText.CompiledLine.render(1): not translated, its class isn't translated (it uses Minecraft's `FormattedText`).
        // CompiledText.CompiledLine.updateDimension(2): not translated, its class isn't translated (it uses Minecraft's `FormattedText`).
        // CompiledText.CompiledLine.add(1): not translated, its class isn't translated (it uses Minecraft's `FormattedText`).
    }

    /// `CompiledText.FormattedTextSplit` (team/creative/creativecore/client/render/text/CompiledText.java)
    pub mod compiled_text_formatted_text_split {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `FormattedText`. Its static parts are below.
        // CompiledText.FormattedTextSplit.FormattedTextSplit(2): not translated, its class isn't translated (it uses Minecraft's `FormattedText`).
        // CompiledText.FormattedTextSplit.head(0): not translated, its class isn't translated (it uses Minecraft's `FormattedText`).
        // CompiledText.FormattedTextSplit.tail(0): not translated, its class isn't translated (it uses Minecraft's `FormattedText`).
    }

    /// `DebugTextRenderer` (team/creative/creativecore/client/render/text/DebugTextRenderer.java)
    pub mod debug_text_renderer {
        use super::*;
        // DECIMAL_FORMAT: not translated, it uses `DecimalFormat` (java.text).
        #[derive(Clone, Debug)]
        pub struct DebugTextRenderer {
            pub lines: Vec<String>,
        }
        impl DebugTextRenderer {
            pub fn new() -> Self {
                let mut s = Self { lines: Vec::new() };
                s.lines.push("".to_string());
                s
            }
            // DebugTextRenderer.text(1): not translated, List.set.
            // DebugTextRenderer.detail(2): not translated, it uses `Object`.
            // DebugTextRenderer.newLine(0): not translated, `this` used as a value.
            // DebugTextRenderer.render(2): not translated, it uses Minecraft's `Font`.
        }
        // DebugTextRenderer.format(1): not translated, it uses `Object`.
    }

    /// `Linebreaker` (team/creative/creativecore/client/render/text/Linebreaker.java)
    pub mod linebreaker {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // Linebreaker.Linebreaker(1): not translated, its class isn't translated (its constants carry values).
        // Linebreaker.Linebreaker(1): not translated, its class isn't translated (its constants carry values).
        // Linebreaker.Linebreaker(3): not translated, its class isn't translated (its constants carry values).
        // Linebreaker.Linebreaker(3): not translated, its class isn't translated (its constants carry values).
    }

    /// `WidthLimitedCharSink` (team/creative/creativecore/client/render/text/WidthLimitedCharSink.java)
    pub mod width_limited_char_sink {
        use super::*;
        // The class itself isn't translated: it implements `FormattedSingleSink` (team.creative.creativecore.common.util.text.content). Its static parts are below.
        // WidthLimitedCharSink.WidthLimitedCharSink(2): not translated, its class isn't translated (it implements `FormattedSingleSink` (team.creative.creativecore.common.util.text.content)).
        // WidthLimitedCharSink.accept(3): not translated, its class isn't translated (it implements `FormattedSingleSink` (team.creative.creativecore.common.util.text.content)).
        // WidthLimitedCharSink.accept(2): not translated, its class isn't translated (it implements `FormattedSingleSink` (team.creative.creativecore.common.util.text.content)).
        // WidthLimitedCharSink.getPosition(0): not translated, its class isn't translated (it implements `FormattedSingleSink` (team.creative.creativecore.common.util.text.content)).
        // WidthLimitedCharSink.lastBreaker(0): not translated, its class isn't translated (it implements `FormattedSingleSink` (team.creative.creativecore.common.util.text.content)).
        // WidthLimitedCharSink.lastBreakerPos(0): not translated, its class isn't translated (it implements `FormattedSingleSink` (team.creative.creativecore.common.util.text.content)).
        // WidthLimitedCharSink.resetPosition(0): not translated, its class isn't translated (it implements `FormattedSingleSink` (team.creative.creativecore.common.util.text.content)).
    }

    /// `VertexFormatUtils` (team/creative/creativecore/client/render/VertexFormatUtils.java)
    pub mod vertex_format_utils {
        use super::*;
        pub static VERTEX_FORMAT_SIZE: JCell<i32> = JCell::new(0i32);
        pub static VERTEX_FORMAT_INT_SIZE: JCell<i32> = JCell::new(0i32);
        pub static POSITION_OFFSET: JCell<i32> = JCell::new(0i32);
        pub static UV_OFFSET: JCell<i32> = JCell::new(0i32);
        #[derive(Clone, Debug)]
        pub struct VertexFormatUtils {
        }
        impl VertexFormatUtils {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // VertexFormatUtils.update(0): not translated, it uses Minecraft's `VertexFormat`.
        pub fn block_format_size() -> i32 {
            return vertex_format_utils::VERTEX_FORMAT_SIZE.get();
        }
        pub fn block_format_int_size() -> i32 {
            return vertex_format_utils::VERTEX_FORMAT_INT_SIZE.get();
        }
        pub fn block_position_offset() -> i32 {
            return vertex_format_utils::POSITION_OFFSET.get();
        }
        pub fn block_uv_offset() -> i32 {
            return vertex_format_utils::UV_OFFSET.get();
        }
    }

    /// `EntitySound` (team/creative/creativecore/client/sound/EntitySound.java)
    pub mod entity_sound {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `AbstractTickableSoundInstance`. Its static parts are below.
        // EntitySound.EntitySound(5): not translated, its class isn't translated (it extends Minecraft's `AbstractTickableSoundInstance`).
        // EntitySound.tick(0): not translated, its class isn't translated (it extends Minecraft's `AbstractTickableSoundInstance`).
    }

    /// `SpecialSoundInstance` (team/creative/creativecore/client/sound/SpecialSoundInstance.java)
    pub mod special_sound_instance {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
    }

    /// `VertexFormatExtender` (team/creative/creativecore/client/VertexFormatExtender.java)
    pub mod vertex_format_extender {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // VertexFormatExtender.getOffset(1): not translated, it has no body.
    }

    /// `CommonRegistry` (team/creative/creativecore/CommonRegistry.java)
    pub mod common_registry {
        use super::*;
        // The class itself isn't translated: it uses Entry, which isn't translated (it's generic). Its static parts are below.
        // CommonRegistry.bindToNamespace(1): not translated, its class isn't translated (it uses Entry, which isn't translated (it's generic)).
        // CommonRegistry.register(4): not translated, it's generic.
        // CommonRegistry.register(2): not translated, it's generic.
        // CommonRegistry.addEntries(1): not translated, its class isn't translated (it uses Entry, which isn't translated (it's generic)).
    }

    /// `CommonRegistry.BoundRegistry` (team/creative/creativecore/CommonRegistry.java)
    pub mod common_registry_bound_registry {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // CommonRegistry.BoundRegistry.BoundRegistry(3): not translated, its class isn't translated (it's generic).
        // CommonRegistry.BoundRegistry.register(2): not translated, it's generic.
        // CommonRegistry.BoundRegistry.register(2): not translated, it's generic.
        // CommonRegistry.BoundRegistry.createTagKey(1): not translated, its class isn't translated (it's generic).
        // CommonRegistry.BoundRegistry.register(2): not translated, it's generic.
    }

    /// `CommonRegistry.BoundRegistry.Items` (team/creative/creativecore/CommonRegistry.java)
    pub mod common_registry_bound_registry_items {
        use super::*;
        // The class itself isn't translated: it extends `BoundRegistry`. Its static parts are below.
        // CommonRegistry.BoundRegistry.Items.Items(1): not translated, its class isn't translated (it extends `BoundRegistry`).
        // CommonRegistry.BoundRegistry.Items.registerItem(2): not translated, it's generic.
    }

    /// `CommonRegistry.DeferredHolder` (team/creative/creativecore/CommonRegistry.java)
    pub mod common_registry_deferred_holder {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // CommonRegistry.DeferredHolder.value(0): not translated, it has no body.
        // CommonRegistry.DeferredHolder.get(0): not translated, it has no body.
    }

    /// `CommonRegistry.Entry` (team/creative/creativecore/CommonRegistry.java)
    pub mod common_registry_entry {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // CommonRegistry.Entry.Entry(4): not translated, its class isn't translated (it's generic).
        // CommonRegistry.Entry.bind(0): not translated, its class isn't translated (it's generic).
        // CommonRegistry.Entry.computeValue(0): not translated, its class isn't translated (it's generic).
        // CommonRegistry.Entry.value(0): not translated, its class isn't translated (it's generic).
        // CommonRegistry.Entry.get(0): not translated, its class isn't translated (it's generic).
        // CommonRegistry.Entry.toVanilla(0): not translated, its class isn't translated (it's generic).
    }

    /// `CommonRegistry.Holder` (team/creative/creativecore/CommonRegistry.java)
    pub mod common_registry_holder {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // CommonRegistry.Holder.value(0): not translated, it has no body.
        // CommonRegistry.Holder.toVanilla(0): not translated, it has no body.
    }

    /// `CommonRegistry.NamespacedRegistry` (team/creative/creativecore/CommonRegistry.java)
    pub mod common_registry_namespaced_registry {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct NamespacedRegistry {
            pub namespace: String,
        }
        impl NamespacedRegistry {
            pub fn new(namespace: &str) -> Self {
                let mut s = Self { namespace: String::new() };
                s.namespace = namespace.to_string();
                s
            }
            pub fn namespace(&self) -> String {
                return self.namespace.to_string();
            }
            // CommonRegistry.NamespacedRegistry.register(4): not translated, it's generic.
            // CommonRegistry.NamespacedRegistry.register(4): not translated, it's generic.
            // CommonRegistry.NamespacedRegistry.createItems(0): not translated, it uses Items, which isn't translated (it extends `BoundRegistry`).
            // CommonRegistry.NamespacedRegistry.bindToRegistry(2): not translated, it's generic.
            // CommonRegistry.NamespacedRegistry.bindToRegistry(1): not translated, it's generic.
            // CommonRegistry.NamespacedRegistry.createTagKey(2): not translated, it's generic.
        }
    }

    /// `CommonRegistry.RegisterHelper` (team/creative/creativecore/CommonRegistry.java)
    pub mod common_registry_register_helper {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // CommonRegistry.RegisterHelper.register(3): not translated, it has no body.
    }

    /// `CommonRegistry.UnitSupplier` (team/creative/creativecore/CommonRegistry.java)
    pub mod common_registry_unit_supplier {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // CommonRegistry.UnitSupplier.UnitSupplier(1): not translated, its class isn't translated (it's generic).
        // CommonRegistry.UnitSupplier.get(0): not translated, its class isn't translated (it's generic).
        // CommonRegistry.UnitSupplier.instance(0): not translated, its class isn't translated (it's generic).
    }

    /// `CommonLoader` (team/creative/creativecore/common/CommonLoader.java)
    pub mod common_loader {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // CommonLoader.onInitialize(0): not translated, it has no body.
        // CommonLoader.registerCommands(1): not translated, its class isn't translated (it's an interface).
    }

    /// `CreativeConfig` (team/creative/creativecore/common/config/api/CreativeConfig.java)
    pub mod creative_config {
        use super::*;
        // The class itself isn't translated: it's an annotation. Its static parts are below.
        // CreativeConfig.name(0): not translated, it has no body.
        // CreativeConfig.type(0): not translated, it has no body.
        // CreativeConfig.requiresRestart(0): not translated, it has no body.
        // CreativeConfig.hideFromGUI(0): not translated, it has no body.
    }

    /// `CreativeConfig.DecimalRange` (team/creative/creativecore/common/config/api/CreativeConfig.java)
    pub mod creative_config_decimal_range {
        use super::*;
        // The class itself isn't translated: it's an annotation. Its static parts are below.
        // CreativeConfig.DecimalRange.min(0): not translated, it has no body.
        // CreativeConfig.DecimalRange.max(0): not translated, it has no body.
        // CreativeConfig.DecimalRange.slider(0): not translated, it has no body.
    }

    /// `CreativeConfig.DecimalRangeSupplier` (team/creative/creativecore/common/config/api/CreativeConfig.java)
    pub mod creative_config_decimal_range_supplier {
        use super::*;
        // The class itself isn't translated: it's an annotation. Its static parts are below.
        // CreativeConfig.DecimalRangeSupplier.supplier(0): not translated, it has no body.
        // CreativeConfig.DecimalRangeSupplier.slider(0): not translated, it has no body.
    }

    /// `CreativeConfig.IntRange` (team/creative/creativecore/common/config/api/CreativeConfig.java)
    pub mod creative_config_int_range {
        use super::*;
        // The class itself isn't translated: it's an annotation. Its static parts are below.
        // CreativeConfig.IntRange.min(0): not translated, it has no body.
        // CreativeConfig.IntRange.max(0): not translated, it has no body.
        // CreativeConfig.IntRange.slider(0): not translated, it has no body.
    }

    /// `CreativeConfig.IntRangeSupplier` (team/creative/creativecore/common/config/api/CreativeConfig.java)
    pub mod creative_config_int_range_supplier {
        use super::*;
        // The class itself isn't translated: it's an annotation. Its static parts are below.
        // CreativeConfig.IntRangeSupplier.supplier(0): not translated, it has no body.
        // CreativeConfig.IntRangeSupplier.slider(0): not translated, it has no body.
    }

    /// `DecimalRangeSupplier` (team/creative/creativecore/common/config/api/DecimalRangeSupplier.java)
    pub mod decimal_range_supplier {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // DecimalRangeSupplier.getMin(0): not translated, it has no body.
        // DecimalRangeSupplier.getMax(0): not translated, it has no body.
    }

    /// `IConfigObject` (team/creative/creativecore/common/config/api/IConfigObject.java)
    pub mod i_config_object {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IConfigObject.isDefault(1): not translated, it has no body.
        // IConfigObject.restoreDefault(2): not translated, it has no body.
    }

    /// `ICreativeConfig` (team/creative/creativecore/common/config/api/ICreativeConfig.java)
    pub mod i_creative_config {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ICreativeConfig.configured(1): not translated, it has no body.
    }

    /// `IntRangeSupplier` (team/creative/creativecore/common/config/api/IntRangeSupplier.java)
    pub mod int_range_supplier {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IntRangeSupplier.getMin(0): not translated, it has no body.
        // IntRangeSupplier.getMax(0): not translated, it has no body.
    }

    /// `ConfigTypeArray` (team/creative/creativecore/common/config/converation/ConfigTypeArray.java)
    pub mod config_type_array {
        use super::*;
        // The class itself isn't translated: it extends `ConfigTypeConveration`. Its static parts are below.
        // ConfigTypeArray.readElement(7): not translated, its class isn't translated (it extends `ConfigTypeConveration`).
        // ConfigTypeArray.writeElement(6): not translated, its class isn't translated (it extends `ConfigTypeConveration`).
        // ConfigTypeArray.createControls(4): not translated, its class isn't translated (it extends `ConfigTypeConveration`).
        // ConfigTypeArray.loadValue(6): not translated, its class isn't translated (it extends `ConfigTypeConveration`).
        // ConfigTypeArray.saveValue(4): not translated, its class isn't translated (it extends `ConfigTypeConveration`).
        // ConfigTypeArray.set(2): not translated, its class isn't translated (it extends `ConfigTypeConveration`).
        // ConfigTypeArray.areEqual(4): not translated, its class isn't translated (it extends `ConfigTypeConveration`).
    }

    /// `ConfigTypeConveration` (team/creative/creativecore/common/config/converation/ConfigTypeConveration.java)
    pub mod config_type_converation {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // TYPE_CREATORS: not translated, it uses `Class`.
        // TYPE_GETTERS: not translated, it uses `Class`.
        // COLLECTION_CREATORS: not translated, it uses `Class`.
        // TYPES: not translated, it uses `Class`.
        // SPECIAL_TYPES: not translated, it uses PairList, which isn't translated (it's generic).
        // FAKE_PARENT: not translated, its value couldn't be read: an anonymous class.
        // ConfigTypeConveration.isTypeCreator(1): not translated, it uses `Class`.
        // ConfigTypeConveration.registerTypeCreator(2): not translated, it's generic.
        // ConfigTypeConveration.registerTypeCreator(2): not translated, it's generic.
        // ConfigTypeConveration.registerTypeGetter(2): not translated, it uses `Class`.
        // ConfigTypeConveration.registerCollectionCreator(2): not translated, it's generic.
        // ConfigTypeConveration.registerType(2): not translated, it's generic.
        // ConfigTypeConveration.registerTypes(2): not translated, it's generic.
        // ConfigTypeConveration.registerSpecialType(2): not translated, it uses `Class`.
        // ConfigTypeConveration.has(1): not translated, it uses `Class`.
        // ConfigTypeConveration.get(1): not translated, it uses ConfigTypeConveration, which isn't translated (it's generic).
        // ConfigTypeConveration.getUnsafe(1): not translated, it uses ConfigTypeConveration, which isn't translated (it's generic).
        // ConfigTypeConveration.createObject(1): not translated, it uses `Object`.
        // ConfigTypeConveration.getGenericType(1): not translated, it uses `Type` (java.lang.reflect).
        // ConfigTypeConveration.getGenericType(1): not translated, it uses `Type` (java.lang.reflect).
        // ConfigTypeConveration.createCollection(1): not translated, it uses `Object`.
        // ConfigTypeConveration.createCollection(1): not translated, it uses `Object`.
        // ConfigTypeConveration.readElement(7): not translated, it has no body.
        // ConfigTypeConveration.writeElement(6): not translated, it has no body.
        // ConfigTypeConveration.createControls(4): not translated, it has no body.
        // ConfigTypeConveration.loadValue(6): not translated, it has no body.
        // ConfigTypeConveration.restoreDefault(5): not translated, its class isn't translated (it's generic).
        // ConfigTypeConveration.shouldSave(5): not translated, its class isn't translated (it's generic).
        // ConfigTypeConveration.saveValue(4): not translated, it has no body.
        // ConfigTypeConveration.set(2): not translated, it has no body.
        // ConfigTypeConveration.save(4): not translated, its class isn't translated (it's generic).
        // ConfigTypeConveration.areEqual(4): not translated, its class isn't translated (it's generic).
    }

    /// `ConfigTypeConveration.SimpleConfigTypeConveration` (team/creative/creativecore/common/config/converation/ConfigTypeConveration.java)
    pub mod config_type_converation_simple_config_type_converation {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // ConfigTypeConveration.SimpleConfigTypeConveration.readElement(7): not translated, its class isn't translated (it's generic).
        // ConfigTypeConveration.SimpleConfigTypeConveration.readElement(4): not translated, it has no body.
        // ConfigTypeConveration.SimpleConfigTypeConveration.writeElement(6): not translated, its class isn't translated (it's generic).
        // ConfigTypeConveration.SimpleConfigTypeConveration.writeElement(3): not translated, it has no body.
        // ConfigTypeConveration.SimpleConfigTypeConveration.createControls(4): not translated, its class isn't translated (it's generic).
        // ConfigTypeConveration.SimpleConfigTypeConveration.createControls(2): not translated, it has no body.
        // ConfigTypeConveration.SimpleConfigTypeConveration.loadValue(6): not translated, its class isn't translated (it's generic).
        // ConfigTypeConveration.SimpleConfigTypeConveration.loadValue(2): not translated, it has no body.
        // ConfigTypeConveration.SimpleConfigTypeConveration.saveValue(4): not translated, its class isn't translated (it's generic).
        // ConfigTypeConveration.SimpleConfigTypeConveration.saveValue(2): not translated, it has no body.
    }

    /// `ConfigTypeConverationSided` (team/creative/creativecore/common/config/converation/ConfigTypeConverationSided.java)
    pub mod config_type_converation_sided {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ConfigTypeConverationSided {
        }
        impl ConfigTypeConverationSided {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn register_side() {
        }
    }

    /// `ConfigTypeList` (team/creative/creativecore/common/config/converation/ConfigTypeList.java)
    pub mod config_type_list {
        use super::*;
        // The class itself isn't translated: it extends `ConfigTypeConveration`. Its static parts are below.
        // ConfigTypeList.readElement(7): not translated, its class isn't translated (it extends `ConfigTypeConveration`).
        // ConfigTypeList.writeElement(6): not translated, its class isn't translated (it extends `ConfigTypeConveration`).
        // ConfigTypeList.createControls(4): not translated, its class isn't translated (it extends `ConfigTypeConveration`).
        // ConfigTypeList.restoreDefault(5): not translated, its class isn't translated (it extends `ConfigTypeConveration`).
        // ConfigTypeList.loadValue(6): not translated, its class isn't translated (it extends `ConfigTypeConveration`).
        // ConfigTypeList.saveValue(4): not translated, its class isn't translated (it extends `ConfigTypeConveration`).
        // ConfigTypeList.set(2): not translated, its class isn't translated (it extends `ConfigTypeConveration`).
        // ConfigTypeList.createList(2): not translated, its class isn't translated (it extends `ConfigTypeConveration`).
        // ConfigTypeList.areEqual(4): not translated, its class isn't translated (it extends `ConfigTypeConveration`).
    }

    /// `ConfigTypeNamedList` (team/creative/creativecore/common/config/converation/ConfigTypeNamedList.java)
    pub mod config_type_named_list {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // ConfigTypeNamedList.addToList(3): not translated, its class isn't translated (it's generic).
        // ConfigTypeNamedList.create(2): not translated, its class isn't translated (it's generic).
        // ConfigTypeNamedList.readElement(7): not translated, its class isn't translated (it's generic).
        // ConfigTypeNamedList.writeElement(6): not translated, its class isn't translated (it's generic).
        // ConfigTypeNamedList.createControls(4): not translated, its class isn't translated (it's generic).
        // ConfigTypeNamedList.restoreDefault(5): not translated, its class isn't translated (it's generic).
        // ConfigTypeNamedList.loadValue(6): not translated, its class isn't translated (it's generic).
        // ConfigTypeNamedList.saveValue(4): not translated, its class isn't translated (it's generic).
        // ConfigTypeNamedList.set(2): not translated, its class isn't translated (it's generic).
        // ConfigTypeNamedList.areEqual(4): not translated, its class isn't translated (it's generic).
    }

    /// `ConfigTypeNumber` (team/creative/creativecore/common/config/converation/ConfigTypeNumber.java)
    pub mod config_type_number {
        use super::*;
        // NUMBER_FORMAT: not translated, it uses `NumberFormat` (java.text).
        // INT_SUPPLIERS: not translated, it uses `Class`.
        // DECIMAL_SUPPLIERS: not translated, it uses `Class`.
        #[derive(Clone, Debug)]
        pub struct ConfigTypeNumber {
        }
        impl ConfigTypeNumber {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ConfigTypeNumber.getIntRangeSupplier(1): not translated, it uses IntRangeSupplier, which isn't translated (it's an interface).
        // ConfigTypeNumber.getDecimalRangeSupplier(1): not translated, it uses DecimalRangeSupplier, which isn't translated (it's an interface).
        // ConfigTypeNumber.createFormat(0): not translated, it uses `NumberFormat` (java.text).
        // ConfigTypeNumber.init(0): not translated, its body couldn't be read: an anonymous class.
    }

    /// `ConfigTypePermission` (team/creative/creativecore/common/config/converation/ConfigTypePermission.java)
    pub mod config_type_permission {
        use super::*;
        // The class itself isn't translated: it extends `ConfigTypeNamedList`. Its static parts are below.
        // PERMISSION_DIALOG: not translated, it uses GuiSyncGlobalLayer, which isn't translated (it's generic).
        // ConfigTypePermission.create(2): not translated, its class isn't translated (it extends `ConfigTypeNamedList`).
        // ConfigTypePermission.addToList(3): not translated, its class isn't translated (it extends `ConfigTypeNamedList`).
        // ConfigTypePermission.shouldSave(5): not translated, its class isn't translated (it extends `ConfigTypeNamedList`).
        // ConfigTypePermission.areEqual(4): not translated, its class isn't translated (it extends `ConfigTypeNamedList`).
        // ConfigTypePermission.createControls(4): not translated, its class isn't translated (it extends `ConfigTypeNamedList`).
        // ConfigTypePermission.loadValue(6): not translated, its class isn't translated (it extends `ConfigTypeNamedList`).
        // ConfigTypePermission.saveValue(4): not translated, its class isn't translated (it extends `ConfigTypeNamedList`).
    }

    /// `ConfigTypePermission.GuiPermissionConfigButton` (team/creative/creativecore/common/config/converation/ConfigTypePermission.java)
    pub mod config_type_permission_gui_permission_config_button {
        use super::*;
        // The class itself isn't translated: it extends `GuiButton` (team.creative.creativecore.common.gui.control.simple). Its static parts are below.
        // ConfigTypePermission.GuiPermissionConfigButton.GuiPermissionConfigButton(6): not translated, its class isn't translated (it extends `GuiButton` (team.creative.creativecore.common.gui.control.simple)).
        // ConfigTypePermission.GuiPermissionConfigButton.setNewValue(1): not translated, its class isn't translated (it extends `GuiButton` (team.creative.creativecore.common.gui.control.simple)).
    }

    /// `ConfigTypeToggleable` (team/creative/creativecore/common/config/converation/ConfigTypeToggleable.java)
    pub mod config_type_toggleable {
        use super::*;
        // The class itself isn't translated: it extends `ConfigTypeConveration`. Its static parts are below.
        // ConfigTypeToggleable.readElement(7): not translated, its class isn't translated (it extends `ConfigTypeConveration`).
        // ConfigTypeToggleable.writeElement(6): not translated, its class isn't translated (it extends `ConfigTypeConveration`).
        // ConfigTypeToggleable.createControls(4): not translated, its class isn't translated (it extends `ConfigTypeConveration`).
        // ConfigTypeToggleable.loadValue(6): not translated, its class isn't translated (it extends `ConfigTypeConveration`).
        // ConfigTypeToggleable.saveValue(4): not translated, its class isn't translated (it extends `ConfigTypeConveration`).
        // ConfigTypeToggleable.set(2): not translated, its class isn't translated (it extends `ConfigTypeConveration`).
        // ConfigTypeToggleable.areEqual(4): not translated, its class isn't translated (it extends `ConfigTypeConveration`).
    }

    /// `ConfigTypeRegistryObject` (team/creative/creativecore/common/config/converation/registry/ConfigTypeRegistryObject.java)
    pub mod config_type_registry_object {
        use super::*;
        // The class itself isn't translated: it extends `ConfigTypeConveration` (team.creative.creativecore.common.config.converation). Its static parts are below.
        // ConfigTypeRegistryObject.readElement(7): not translated, its class isn't translated (it extends `ConfigTypeConveration` (team.creative.creativecore.common.config.converation)).
        // ConfigTypeRegistryObject.writeElement(6): not translated, its class isn't translated (it extends `ConfigTypeConveration` (team.creative.creativecore.common.config.converation)).
        // ConfigTypeRegistryObject.createControls(4): not translated, its class isn't translated (it extends `ConfigTypeConveration` (team.creative.creativecore.common.config.converation)).
        // ConfigTypeRegistryObject.loadValue(6): not translated, its class isn't translated (it extends `ConfigTypeConveration` (team.creative.creativecore.common.config.converation)).
        // ConfigTypeRegistryObject.saveValue(4): not translated, its class isn't translated (it extends `ConfigTypeConveration` (team.creative.creativecore.common.config.converation)).
        // ConfigTypeRegistryObject.set(2): not translated, its class isn't translated (it extends `ConfigTypeConveration` (team.creative.creativecore.common.config.converation)).
    }

    /// `ConfigTypeRegistryObjectList` (team/creative/creativecore/common/config/converation/registry/ConfigTypeRegistryObjectList.java)
    pub mod config_type_registry_object_list {
        use super::*;
        // The class itself isn't translated: it extends `ConfigTypeConveration` (team.creative.creativecore.common.config.converation). Its static parts are below.
        // ConfigTypeRegistryObjectList.readElement(7): not translated, its class isn't translated (it extends `ConfigTypeConveration` (team.creative.creativecore.common.config.converation)).
        // ConfigTypeRegistryObjectList.writeElement(6): not translated, its class isn't translated (it extends `ConfigTypeConveration` (team.creative.creativecore.common.config.converation)).
        // ConfigTypeRegistryObjectList.createControls(4): not translated, its class isn't translated (it extends `ConfigTypeConveration` (team.creative.creativecore.common.config.converation)).
        // ConfigTypeRegistryObjectList.loadValue(6): not translated, its class isn't translated (it extends `ConfigTypeConveration` (team.creative.creativecore.common.config.converation)).
        // ConfigTypeRegistryObjectList.saveValue(4): not translated, its class isn't translated (it extends `ConfigTypeConveration` (team.creative.creativecore.common.config.converation)).
        // ConfigTypeRegistryObjectList.set(2): not translated, its class isn't translated (it extends `ConfigTypeConveration` (team.creative.creativecore.common.config.converation)).
        // ConfigTypeRegistryObjectList.areEqual(4): not translated, its class isn't translated (it extends `ConfigTypeConveration` (team.creative.creativecore.common.config.converation)).
    }

    /// `ConfigTypeRegistryTag` (team/creative/creativecore/common/config/converation/registry/ConfigTypeRegistryTag.java)
    pub mod config_type_registry_tag {
        use super::*;
        // The class itself isn't translated: it extends `ConfigTypeConveration` (team.creative.creativecore.common.config.converation). Its static parts are below.
        // ConfigTypeRegistryTag.readElement(7): not translated, its class isn't translated (it extends `ConfigTypeConveration` (team.creative.creativecore.common.config.converation)).
        // ConfigTypeRegistryTag.writeElement(6): not translated, its class isn't translated (it extends `ConfigTypeConveration` (team.creative.creativecore.common.config.converation)).
        // ConfigTypeRegistryTag.createControls(4): not translated, its class isn't translated (it extends `ConfigTypeConveration` (team.creative.creativecore.common.config.converation)).
        // ConfigTypeRegistryTag.loadValue(6): not translated, its class isn't translated (it extends `ConfigTypeConveration` (team.creative.creativecore.common.config.converation)).
        // ConfigTypeRegistryTag.saveValue(4): not translated, its class isn't translated (it extends `ConfigTypeConveration` (team.creative.creativecore.common.config.converation)).
        // ConfigTypeRegistryTag.set(2): not translated, its class isn't translated (it extends `ConfigTypeConveration` (team.creative.creativecore.common.config.converation)).
    }

    /// `ConfigTypeRegistryTagList` (team/creative/creativecore/common/config/converation/registry/ConfigTypeRegistryTagList.java)
    pub mod config_type_registry_tag_list {
        use super::*;
        // The class itself isn't translated: it extends `ConfigTypeConveration` (team.creative.creativecore.common.config.converation). Its static parts are below.
        // ConfigTypeRegistryTagList.readElement(7): not translated, its class isn't translated (it extends `ConfigTypeConveration` (team.creative.creativecore.common.config.converation)).
        // ConfigTypeRegistryTagList.writeElement(6): not translated, its class isn't translated (it extends `ConfigTypeConveration` (team.creative.creativecore.common.config.converation)).
        // ConfigTypeRegistryTagList.createControls(4): not translated, its class isn't translated (it extends `ConfigTypeConveration` (team.creative.creativecore.common.config.converation)).
        // ConfigTypeRegistryTagList.loadValue(6): not translated, its class isn't translated (it extends `ConfigTypeConveration` (team.creative.creativecore.common.config.converation)).
        // ConfigTypeRegistryTagList.saveValue(4): not translated, its class isn't translated (it extends `ConfigTypeConveration` (team.creative.creativecore.common.config.converation)).
        // ConfigTypeRegistryTagList.set(2): not translated, its class isn't translated (it extends `ConfigTypeConveration` (team.creative.creativecore.common.config.converation)).
        // ConfigTypeRegistryTagList.areEqual(4): not translated, its class isn't translated (it extends `ConfigTypeConveration` (team.creative.creativecore.common.config.converation)).
    }

    /// `GuiRegistryObjectHandler` (team/creative/creativecore/common/config/converation/registry/GuiRegistryObjectHandler.java)
    pub mod gui_registry_object_handler {
        use super::*;
        // REGISTRY: not translated, its value couldn't be read: an anonymous class.
        #[derive(Clone, Debug)]
        pub struct GuiRegistryObjectHandler {
        }
        impl GuiRegistryObjectHandler {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // GuiRegistryObjectHandler.createControls(2): not translated, it has no body.
            // GuiRegistryObjectHandler.loadValue(3): not translated, it has no body.
            // GuiRegistryObjectHandler.saveValue(2): not translated, it has no body.
        }
    }

    /// `GuiRegistryTagHandler` (team/creative/creativecore/common/config/converation/registry/GuiRegistryTagHandler.java)
    pub mod gui_registry_tag_handler {
        use super::*;
        // REGISTRY: not translated, its value couldn't be read: an anonymous class.
        #[derive(Clone, Debug)]
        pub struct GuiRegistryTagHandler {
        }
        impl GuiRegistryTagHandler {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // GuiRegistryTagHandler.createControls(2): not translated, it has no body.
            // GuiRegistryTagHandler.loadValue(3): not translated, it has no body.
            // GuiRegistryTagHandler.saveValue(2): not translated, it has no body.
        }
    }

    /// `ConfigEqualChecker` (team/creative/creativecore/common/config/core/ConfigEqualChecker.java)
    pub mod config_equal_checker {
        use super::*;
        // The class itself isn't translated: it uses `Class`. Its static parts are below.
        // ConfigEqualChecker.register(2): not translated, its class isn't translated (it uses `Class`).
        // ConfigEqualChecker.equals(4): not translated, its class isn't translated (it uses `Class`).
    }

    /// `IConfigRegistry` (team/creative/creativecore/common/config/core/IConfigRegistry.java)
    pub mod i_config_registry {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IConfigRegistry.getEqualChecker(0): not translated, it has no body.
        // IConfigRegistry.is(1): not translated, it has no body.
        // IConfigRegistry.is(2): not translated, its class isn't translated (it's an interface).
        // IConfigRegistry.equals(3): not translated, its class isn't translated (it's an interface).
    }

    /// `ConfigEventHandler` (team/creative/creativecore/common/config/event/ConfigEventHandler.java)
    pub mod config_event_handler {
        use super::*;
        // The class itself isn't translated: it uses `DecimalFormat` (java.text). Its static parts are below.
        // ConfigEventHandler.client(0): not translated, its class isn't translated (it uses `DecimalFormat` (java.text)).
        // ConfigEventHandler.ConfigEventHandler(2): not translated, its class isn't translated (it uses `DecimalFormat` (java.text)).
        // ConfigEventHandler.loadClientFieldList(1): not translated, it uses ICreativeConfigHolder, which isn't translated (it's an interface).
        // ConfigEventHandler.loadClientFieldList(3): not translated, it uses ICreativeConfigHolder, which isn't translated (it's an interface).
        // ConfigEventHandler.saveClientFieldList(2): not translated, it uses ICreativeConfigHolder, which isn't translated (it's an interface).
        // ConfigEventHandler.saveClientFieldList(3): not translated, it uses ICreativeConfigHolder, which isn't translated (it's an interface).
        // ConfigEventHandler.enable(1): not translated, it uses ICreativeConfigHolder, which isn't translated (it's an interface).
        // ConfigEventHandler.generateFormat(0): not translated, its class isn't translated (it uses `DecimalFormat` (java.text)).
        // ConfigEventHandler.playerLoggedIn(3): not translated, its class isn't translated (it uses `DecimalFormat` (java.text)).
        // ConfigEventHandler.isOwner(1): not translated, its class isn't translated (it uses `DecimalFormat` (java.text)).
        // ConfigEventHandler.sync(2): not translated, its class isn't translated (it uses `DecimalFormat` (java.text)).
        // ConfigEventHandler.sync(2): not translated, its class isn't translated (it uses `DecimalFormat` (java.text)).
        // ConfigEventHandler.syncAll(1): not translated, its class isn't translated (it uses `DecimalFormat` (java.text)).
        // ConfigEventHandler.syncAll(1): not translated, its class isn't translated (it uses `DecimalFormat` (java.text)).
        // ConfigEventHandler.saveClientFields(0): not translated, its class isn't translated (it uses `DecimalFormat` (java.text)).
        // ConfigEventHandler.save(3): not translated, its body couldn't be read: an anonymous class.
        // ConfigEventHandler.save(2): not translated, its class isn't translated (it uses `DecimalFormat` (java.text)).
        // ConfigEventHandler.loadClientFields(0): not translated, its class isn't translated (it uses `DecimalFormat` (java.text)).
        // ConfigEventHandler.load(3): not translated, its class isn't translated (it uses `DecimalFormat` (java.text)).
        // ConfigEventHandler.load(2): not translated, its class isn't translated (it uses `DecimalFormat` (java.text)).
        // ConfigEventHandler.isSynchronizedWithServer(1): not translated, its class isn't translated (it uses `DecimalFormat` (java.text)).
        // ConfigEventHandler.modFileExist(2): not translated, its class isn't translated (it uses `DecimalFormat` (java.text)).
    }

    /// `ConfigField` (team/creative/creativecore/common/config/field/ConfigField.java)
    pub mod config_field {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ConfigField.get(0): not translated, it has no body.
        // ConfigField.set(1): not translated, it has no body.
        // ConfigField.getAnnotation(1): not translated, it has no body.
        // ConfigField.getGenericType(0): not translated, it has no body.
        // ConfigField.getType(0): not translated, it has no body.
    }

    /// `ConfigFieldDynamic` (team/creative/creativecore/common/config/field/ConfigFieldDynamic.java)
    pub mod config_field_dynamic {
        use super::*;
        // The class itself isn't translated: it implements `ConfigField`. Its static parts are below.
        // ConfigFieldDynamic.ConfigFieldDynamic(1): not translated, its class isn't translated (it implements `ConfigField`).
        // ConfigFieldDynamic.get(0): not translated, its class isn't translated (it implements `ConfigField`).
        // ConfigFieldDynamic.set(1): not translated, its class isn't translated (it implements `ConfigField`).
        // ConfigFieldDynamic.getAnnotation(1): not translated, it's generic.
        // ConfigFieldDynamic.getGenericType(0): not translated, its class isn't translated (it implements `ConfigField`).
        // ConfigFieldDynamic.getType(0): not translated, its class isn't translated (it implements `ConfigField`).
    }

    /// `ConfigFieldTyped` (team/creative/creativecore/common/config/field/ConfigFieldTyped.java)
    pub mod config_field_typed {
        use super::*;
        // The class itself isn't translated: it implements `ConfigField`. Its static parts are below.
        // ConfigFieldTyped.ConfigFieldTyped(3): not translated, its class isn't translated (it implements `ConfigField`).
        // ConfigFieldTyped.get(0): not translated, its class isn't translated (it implements `ConfigField`).
        // ConfigFieldTyped.set(1): not translated, its class isn't translated (it implements `ConfigField`).
        // ConfigFieldTyped.getAnnotation(1): not translated, it's generic.
        // ConfigFieldTyped.getGenericType(0): not translated, its class isn't translated (it implements `ConfigField`).
        // ConfigFieldTyped.getType(0): not translated, its class isn't translated (it implements `ConfigField`).
    }

    /// `ConfigFieldWrapper` (team/creative/creativecore/common/config/field/ConfigFieldWrapper.java)
    pub mod config_field_wrapper {
        use super::*;
        // The class itself isn't translated: it implements `ConfigField`. Its static parts are below.
        // ConfigFieldWrapper.ConfigFieldWrapper(2): not translated, its class isn't translated (it implements `ConfigField`).
        // ConfigFieldWrapper.get(0): not translated, its class isn't translated (it implements `ConfigField`).
        // ConfigFieldWrapper.set(1): not translated, its class isn't translated (it implements `ConfigField`).
        // ConfigFieldWrapper.getAnnotation(1): not translated, it's generic.
        // ConfigFieldWrapper.getGenericType(0): not translated, its class isn't translated (it implements `ConfigField`).
        // ConfigFieldWrapper.getType(0): not translated, its class isn't translated (it implements `ConfigField`).
        // ConfigFieldWrapper.parent(0): not translated, its class isn't translated (it implements `ConfigField`).
        // ConfigFieldWrapper.field(0): not translated, its class isn't translated (it implements `ConfigField`).
    }

    /// `Usergroup` (team/creative/creativecore/common/config/group/Usergroup.java)
    pub mod usergroup {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Usergroup {
            pub filters: Vec<player_selector::PlayerSelector>,
        }
        impl Usergroup {
            pub fn new() -> Self {
                let mut s = Self { filters: Vec::new() };
                s.filters = Vec::new();
                s
            }
            pub fn new_1(selectors: &[player_selector::PlayerSelector]) -> Self {
                let mut s = Self { filters: Vec::new() };
                s.filters = selectors.to_vec();
                s
            }
            // Usergroup.is(1): not translated, it uses Minecraft's `Player`.
        }
    }

    /// `ClientSyncGuiLayer` (team/creative/creativecore/common/config/gui/ClientSyncGuiLayer.java)
    pub mod client_sync_gui_layer {
        use super::*;
        // The class itself isn't translated: it extends `GuiLayer` (team.creative.creativecore.common.gui). Its static parts are below.
        // ClientSyncGuiLayer.ClientSyncGuiLayer(2): not translated, its class isn't translated (it extends `GuiLayer` (team.creative.creativecore.common.gui)).
        // ClientSyncGuiLayer.create(0): not translated, its class isn't translated (it extends `GuiLayer` (team.creative.creativecore.common.gui)).
        // ClientSyncGuiLayer.save(0): not translated, its class isn't translated (it extends `GuiLayer` (team.creative.creativecore.common.gui)).
        // ClientSyncGuiLayer.load(1): not translated, its class isn't translated (it extends `GuiLayer` (team.creative.creativecore.common.gui)).
        // ClientSyncGuiLayer.closeTopLayer(0): not translated, its class isn't translated (it extends `GuiLayer` (team.creative.creativecore.common.gui)).
    }

    /// `ClientSyncGuiLayer.GuiTreeCheckBox` (team/creative/creativecore/common/config/gui/ClientSyncGuiLayer.java)
    pub mod client_sync_gui_layer_gui_tree_check_box {
        use super::*;
        // The class itself isn't translated: it extends `GuiCheckBox` (team.creative.creativecore.common.gui.control.simple). Its static parts are below.
        // ClientSyncGuiLayer.GuiTreeCheckBox.GuiTreeCheckBox(2): not translated, its class isn't translated (it extends `GuiCheckBox` (team.creative.creativecore.common.gui.control.simple)).
    }

    /// `ConfigGuiLayer` (team/creative/creativecore/common/config/gui/ConfigGuiLayer.java)
    pub mod config_gui_layer {
        use super::*;
        // The class itself isn't translated: it extends `GuiLayer` (team.creative.creativecore.common.gui). Its static parts are below.
        // ConfigGuiLayer.ConfigGuiLayer(3): not translated, its class isn't translated (it extends `GuiLayer` (team.creative.creativecore.common.gui)).
        // ConfigGuiLayer.create(0): not translated, its class isn't translated (it extends `GuiLayer` (team.creative.creativecore.common.gui)).
        // ConfigGuiLayer.savePage(0): not translated, its class isn't translated (it extends `GuiLayer` (team.creative.creativecore.common.gui)).
        // ConfigGuiLayer.loadHolder(1): not translated, its class isn't translated (it extends `GuiLayer` (team.creative.creativecore.common.gui)).
        // ConfigGuiLayer.sendUpdate(0): not translated, its class isn't translated (it extends `GuiLayer` (team.creative.creativecore.common.gui)).
        // ConfigGuiLayer.closeTopLayer(0): not translated, its class isn't translated (it extends `GuiLayer` (team.creative.creativecore.common.gui)).
    }

    /// `FullItemDialogGuiLayer` (team/creative/creativecore/common/config/gui/FullItemDialogGuiLayer.java)
    pub mod full_item_dialog_gui_layer {
        use super::*;
        // The class itself isn't translated: it extends `GuiLayer` (team.creative.creativecore.common.gui). Its static parts are below.
        pub static LATEST: LazyLock<JCell<Vec<creative_ingredient::CreativeIngredient>>> = LazyLock::new(|| JCell::new(Vec::new()));
        // FullItemDialogGuiLayer.FullItemDialogGuiLayer(1): not translated, its class isn't translated (it extends `GuiLayer` (team.creative.creativecore.common.gui)).
        // FullItemDialogGuiLayer.create(0): not translated, its class isn't translated (it extends `GuiLayer` (team.creative.creativecore.common.gui)).
    }

    /// `GuiConfigControl` (team/creative/creativecore/common/config/gui/GuiConfigControl.java)
    pub mod gui_config_control {
        use super::*;
        // The class itself isn't translated: it extends `GuiRow` (team.creative.creativecore.common.gui.control.parent). Its static parts are below.
        // GuiConfigControl.GuiConfigControl(5): not translated, its class isn't translated (it extends `GuiRow` (team.creative.creativecore.common.gui.control.parent)).
        // GuiConfigControl.GuiConfigControl(5): not translated, its class isn't translated (it extends `GuiRow` (team.creative.creativecore.common.gui.control.parent)).
        // GuiConfigControl.GuiConfigControl(7): not translated, its class isn't translated (it extends `GuiRow` (team.creative.creativecore.common.gui.control.parent)).
        // GuiConfigControl.isDefault(0): not translated, its class isn't translated (it extends `GuiRow` (team.creative.creativecore.common.gui.control.parent)).
        // GuiConfigControl.updateButton(0): not translated, its class isn't translated (it extends `GuiRow` (team.creative.creativecore.common.gui.control.parent)).
        // GuiConfigControl.init(1): not translated, its class isn't translated (it extends `GuiRow` (team.creative.creativecore.common.gui.control.parent)).
        // GuiConfigControl.reset(0): not translated, its class isn't translated (it extends `GuiRow` (team.creative.creativecore.common.gui.control.parent)).
        // GuiConfigControl.changed(0): not translated, its class isn't translated (it extends `GuiRow` (team.creative.creativecore.common.gui.control.parent)).
        // GuiConfigControl.save(0): not translated, its class isn't translated (it extends `GuiRow` (team.creative.creativecore.common.gui.control.parent)).
        // GuiConfigControl.setCustomData(1): not translated, its class isn't translated (it extends `GuiRow` (team.creative.creativecore.common.gui.control.parent)).
        // GuiConfigControl.getCustomData(0): not translated, its class isn't translated (it extends `GuiRow` (team.creative.creativecore.common.gui.control.parent)).
    }

    /// `GuiConfigSubControl` (team/creative/creativecore/common/config/gui/GuiConfigSubControl.java)
    pub mod gui_config_sub_control {
        use super::*;
        // The class itself isn't translated: it extends `GuiParent` (team.creative.creativecore.common.gui). Its static parts are below.
        // GuiConfigSubControl.GuiConfigSubControl(2): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiConfigSubControl.getName(0): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiConfigSubControl.addNameUnmodifieable(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiConfigSubControl.addNameTextfield(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
    }

    /// `GuiConfigSubControlHolder` (team/creative/creativecore/common/config/gui/GuiConfigSubControlHolder.java)
    pub mod gui_config_sub_control_holder {
        use super::*;
        // The class itself isn't translated: it extends `GuiConfigSubControl`. Its static parts are below.
        // GuiConfigSubControlHolder.GuiConfigSubControlHolder(6): not translated, its class isn't translated (it extends `GuiConfigSubControl`).
        // GuiConfigSubControlHolder.load(2): not translated, its class isn't translated (it extends `GuiConfigSubControl`).
        // GuiConfigSubControlHolder.createControls(0): not translated, its body couldn't be read: an anonymous class.
        // GuiConfigSubControlHolder.save(0): not translated, its class isn't translated (it extends `GuiConfigSubControl`).
    }

    /// `GuiConfigSubControlNested` (team/creative/creativecore/common/config/gui/GuiConfigSubControlNested.java)
    pub mod gui_config_sub_control_nested {
        use super::*;
        // The class itself isn't translated: it extends `GuiConfigSubControl`. Its static parts are below.
        // GuiConfigSubControlNested.GuiConfigSubControlNested(7): not translated, its class isn't translated (it extends `GuiConfigSubControl`).
        // GuiConfigSubControlNested.load(2): not translated, its class isn't translated (it extends `GuiConfigSubControl`).
        // GuiConfigSubControlNested.addNameUnmodifieable(1): not translated, its class isn't translated (it extends `GuiConfigSubControl`).
        // GuiConfigSubControlNested.addNameTextfield(1): not translated, its class isn't translated (it extends `GuiConfigSubControl`).
        // GuiConfigSubControlNested.createControls(0): not translated, its body couldn't be read: an anonymous class.
        // GuiConfigSubControlNested.save(0): not translated, its class isn't translated (it extends `GuiConfigSubControl`).
    }

    /// `GuiInfoStackButton` (team/creative/creativecore/common/config/gui/GuiInfoStackButton.java)
    pub mod gui_info_stack_button {
        use super::*;
        // The class itself isn't translated: it extends `GuiButton` (team.creative.creativecore.common.gui.control.simple). Its static parts are below.
        // ITEM_DIALOG: not translated, it uses GuiSyncGlobalLayer, which isn't translated (it's generic).
        // GuiInfoStackButton.GuiInfoStackButton(3): not translated, its class isn't translated (it extends `GuiButton` (team.creative.creativecore.common.gui.control.simple)).
        // GuiInfoStackButton.set(1): not translated, its class isn't translated (it extends `GuiButton` (team.creative.creativecore.common.gui.control.simple)).
        // GuiInfoStackButton.getLabelText(1): not translated, it uses Minecraft's `Component`.
        // GuiInfoStackButton.get(0): not translated, its class isn't translated (it extends `GuiButton` (team.creative.creativecore.common.gui.control.simple)).
    }

    /// `GuiPlayerSelectorButton` (team/creative/creativecore/common/config/gui/GuiPlayerSelectorButton.java)
    pub mod gui_player_selector_button {
        use super::*;
        // The class itself isn't translated: it extends `GuiButton` (team.creative.creativecore.common.gui.control.simple). Its static parts are below.
        // PLAYER_SELECTOR: not translated, it uses GuiSyncGlobalLayer, which isn't translated (it's generic).
        // GuiPlayerSelectorButton.GuiPlayerSelectorButton(3): not translated, its class isn't translated (it extends `GuiButton` (team.creative.creativecore.common.gui.control.simple)).
        // GuiPlayerSelectorButton.set(1): not translated, its class isn't translated (it extends `GuiButton` (team.creative.creativecore.common.gui.control.simple)).
        // GuiPlayerSelectorButton.getLabelText(1): not translated, it uses Minecraft's `Component`.
        // GuiPlayerSelectorButton.get(0): not translated, its class isn't translated (it extends `GuiButton` (team.creative.creativecore.common.gui.control.simple)).
    }

    /// `GuiPlayerSelectorHandler` (team/creative/creativecore/common/config/gui/GuiPlayerSelectorHandler.java)
    pub mod gui_player_selector_handler {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // REGISTRY: not translated, it uses NamedHandlerRegistry, which isn't translated (it's generic).
        // GuiPlayerSelectorHandler.get(1): not translated, it uses GuiPlayerSelectorHandler, which isn't translated (it's generic).
        // GuiPlayerSelectorHandler.getName(0): not translated, its class isn't translated (it's generic).
        // GuiPlayerSelectorHandler.createControls(2): not translated, it has no body.
        // GuiPlayerSelectorHandler.parseSelector(1): not translated, it has no body.
        // GuiPlayerSelectorHandler.onChanged(2): not translated, its class isn't translated (it's generic).
    }

    /// `GuiPlayerSelectorHandler.GuiPlayerSelectorHandlerMultiple` (team/creative/creativecore/common/config/gui/GuiPlayerSelectorHandler.java)
    pub mod gui_player_selector_handler_gui_player_selector_handler_multiple {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GuiPlayerSelectorHandler.GuiPlayerSelectorHandlerMultiple.getChildren(1): not translated, its class isn't translated (it's generic).
        // GuiPlayerSelectorHandler.GuiPlayerSelectorHandlerMultiple.createControls(2): not translated, its class isn't translated (it's generic).
        // GuiPlayerSelectorHandler.GuiPlayerSelectorHandlerMultiple.parseSelector(1): not translated, its class isn't translated (it's generic).
        // GuiPlayerSelectorHandler.GuiPlayerSelectorHandlerMultiple.parseSelector(1): not translated, its class isn't translated (it's generic).
    }

    /// `IGuiConfigParent` (team/creative/creativecore/common/config/gui/IGuiConfigParent.java)
    pub mod i_gui_config_parent {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IGuiConfigParent.provider(0): not translated, it has no body.
        // IGuiConfigParent.setCustomData(1): not translated, it has no body.
        // IGuiConfigParent.getCustomData(0): not translated, it has no body.
        // IGuiConfigParent.changed(0): not translated, it has no body.
    }

    /// `PermissionGuiLayer` (team/creative/creativecore/common/config/gui/PermissionGuiLayer.java)
    pub mod permission_gui_layer {
        use super::*;
        // The class itself isn't translated: it extends `GuiLayer` (team.creative.creativecore.common.gui). Its static parts are below.
        // PermissionGuiLayer.PermissionGuiLayer(1): not translated, its class isn't translated (it extends `GuiLayer` (team.creative.creativecore.common.gui)).
        // PermissionGuiLayer.create(0): not translated, its body couldn't be read: an anonymous class.
        // PermissionGuiLayer.addGroupHeader(3): not translated, its class isn't translated (it extends `GuiLayer` (team.creative.creativecore.common.gui)).
        // PermissionGuiLayer.closeTopLayer(0): not translated, its class isn't translated (it extends `GuiLayer` (team.creative.creativecore.common.gui)).
        // PermissionGuiLayer.save(0): not translated, its class isn't translated (it extends `GuiLayer` (team.creative.creativecore.common.gui)).
    }

    /// `PermissionGuiLayer.PermissionGuiGroup` (team/creative/creativecore/common/config/gui/PermissionGuiLayer.java)
    pub mod permission_gui_layer_permission_gui_group {
        use super::*;
        // The class itself isn't translated: it uses GuiComboBox, which isn't translated (it's generic). Its static parts are below.
        // PermissionGuiLayer.PermissionGuiGroup.PermissionGuiGroup(1): not translated, its class isn't translated (it uses GuiComboBox, which isn't translated (it's generic)).
        // PermissionGuiLayer.PermissionGuiGroup.getTitle(0): not translated, its class isn't translated (it uses GuiComboBox, which isn't translated (it's generic)).
        // PermissionGuiLayer.PermissionGuiGroup.updateResetButton(0): not translated, its class isn't translated (it uses GuiComboBox, which isn't translated (it's generic)).
        // PermissionGuiLayer.PermissionGuiGroup.save(0): not translated, it has no body.
        // PermissionGuiLayer.PermissionGuiGroup.reset(0): not translated, its class isn't translated (it uses GuiComboBox, which isn't translated (it's generic)).
        // PermissionGuiLayer.PermissionGuiGroup.resetInternal(0): not translated, it has no body.
        // PermissionGuiLayer.PermissionGuiGroup.isDefault(0): not translated, it has no body.
        // PermissionGuiLayer.PermissionGuiGroup.isDefaultGroup(0): not translated, its class isn't translated (it uses GuiComboBox, which isn't translated (it's generic)).
    }

    /// `PermissionGuiLayer.PermissionGuiGroupMulti` (team/creative/creativecore/common/config/gui/PermissionGuiLayer.java)
    pub mod permission_gui_layer_permission_gui_group_multi {
        use super::*;
        // The class itself isn't translated: it extends `PermissionGuiGroup`. Its static parts are below.
        // PermissionGuiLayer.PermissionGuiGroupMulti.PermissionGuiGroupMulti(3): not translated, its class isn't translated (it extends `PermissionGuiGroup`).
        // PermissionGuiLayer.PermissionGuiGroupMulti.save(0): not translated, its class isn't translated (it extends `PermissionGuiGroup`).
        // PermissionGuiLayer.PermissionGuiGroupMulti.resetInternal(0): not translated, its class isn't translated (it extends `PermissionGuiGroup`).
        // PermissionGuiLayer.PermissionGuiGroupMulti.isDefault(0): not translated, its class isn't translated (it extends `PermissionGuiGroup`).
    }

    /// `PermissionGuiLayer.PermissionGuiGroupSimple` (team/creative/creativecore/common/config/gui/PermissionGuiLayer.java)
    pub mod permission_gui_layer_permission_gui_group_simple {
        use super::*;
        // The class itself isn't translated: it extends `PermissionGuiGroup`. Its static parts are below.
        // PermissionGuiLayer.PermissionGuiGroupSimple.PermissionGuiGroupSimple(2): not translated, its class isn't translated (it extends `PermissionGuiGroup`).
        // PermissionGuiLayer.PermissionGuiGroupSimple.resetInternal(0): not translated, its class isn't translated (it extends `PermissionGuiGroup`).
        // PermissionGuiLayer.PermissionGuiGroupSimple.isDefault(0): not translated, its class isn't translated (it extends `PermissionGuiGroup`).
        // PermissionGuiLayer.PermissionGuiGroupSimple.save(0): not translated, its class isn't translated (it extends `PermissionGuiGroup`).
    }

    /// `PlayerSelectorDialog` (team/creative/creativecore/common/config/gui/PlayerSelectorDialog.java)
    pub mod player_selector_dialog {
        use super::*;
        // The class itself isn't translated: it extends `GuiLayer` (team.creative.creativecore.common.gui). Its static parts are below.
        // PlayerSelectorDialog.PlayerSelectorDialog(1): not translated, its class isn't translated (it extends `GuiLayer` (team.creative.creativecore.common.gui)).
        // PlayerSelectorDialog.create(0): not translated, its class isn't translated (it extends `GuiLayer` (team.creative.creativecore.common.gui)).
    }

    /// `ConfigHolder` (team/creative/creativecore/common/config/holder/ConfigHolder.java)
    pub mod config_holder {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // ConfigHolder.ConfigHolder(3): not translated, its class isn't translated (it's generic).
        // ConfigHolder.ConfigHolder(0): not translated, its class isn't translated (it's generic).
        // ConfigHolder.synchronization(0): not translated, its class isn't translated (it's generic).
        // ConfigHolder.parent(0): not translated, its class isn't translated (it's generic).
        // ConfigHolder.path(0): not translated, its class isn't translated (it's generic).
        // ConfigHolder.getField(1): not translated, its class isn't translated (it's generic).
        // ConfigHolder.fields(0): not translated, its class isn't translated (it's generic).
        // ConfigHolder.names(0): not translated, its class isn't translated (it's generic).
        // ConfigHolder.get(1): not translated, its class isn't translated (it's generic).
        // ConfigHolder.isEmpty(1): not translated, its class isn't translated (it's generic).
        // ConfigHolder.isEmptyWithoutForce(1): not translated, its class isn't translated (it's generic).
        // ConfigHolder.isDefault(1): not translated, its class isn't translated (it's generic).
        // ConfigHolder.restoreDefault(2): not translated, its class isn't translated (it's generic).
        // ConfigHolder.load(5): not translated, its class isn't translated (it's generic).
        // ConfigHolder.save(4): not translated, its class isn't translated (it's generic).
        // ConfigHolder.getRegistry(0): not translated, its class isn't translated (it's generic).
    }

    /// `ConfigHolderDynamic` (team/creative/creativecore/common/config/holder/ConfigHolderDynamic.java)
    pub mod config_holder_dynamic {
        use super::*;
        // The class itself isn't translated: it extends `ConfigHolder`. Its static parts are below.
        // ConfigHolderDynamic.ConfigHolderDynamic(3): not translated, its class isn't translated (it extends `ConfigHolder`).
        // ConfigHolderDynamic.ConfigHolderDynamic(0): not translated, its class isn't translated (it extends `ConfigHolder`).
        // ConfigHolderDynamic.clear(0): not translated, its class isn't translated (it extends `ConfigHolder`).
        // ConfigHolderDynamic.registerFolder(1): not translated, its class isn't translated (it extends `ConfigHolder`).
        // ConfigHolderDynamic.registerFolder(2): not translated, its class isn't translated (it extends `ConfigHolder`).
        // ConfigHolderDynamic.registerValue(2): not translated, its class isn't translated (it extends `ConfigHolder`).
        // ConfigHolderDynamic.registerValue(4): not translated, its class isn't translated (it extends `ConfigHolder`).
        // ConfigHolderDynamic.registerValue(5): not translated, its class isn't translated (it extends `ConfigHolder`).
        // ConfigHolderDynamic.registerField(3): not translated, its class isn't translated (it extends `ConfigHolder`).
        // ConfigHolderDynamic.registerField(5): not translated, its class isn't translated (it extends `ConfigHolder`).
        // ConfigHolderDynamic.registerField(6): not translated, its class isn't translated (it extends `ConfigHolder`).
        // ConfigHolderDynamic.configured(1): not translated, its class isn't translated (it extends `ConfigHolder`).
    }

    /// `ConfigHolderObject` (team/creative/creativecore/common/config/holder/ConfigHolderObject.java)
    pub mod config_holder_object {
        use super::*;
        // The class itself isn't translated: it extends `ConfigHolder`. Its static parts are below.
        // ConfigHolderObject.createUnrelated(3): not translated, it uses ConfigHolderObject, which isn't translated (it extends `ConfigHolder`).
        // ConfigHolderObject.createUnrelated(4): not translated, it uses ConfigHolderObject, which isn't translated (it extends `ConfigHolder`).
        // ConfigHolderObject.collectFields(3): not translated, it uses `Field` (java.lang.reflect).
        // ConfigHolderObject.ConfigHolderObject(4): not translated, its class isn't translated (it extends `ConfigHolder`).
        // ConfigHolderObject.ConfigHolderObject(5): not translated, its class isn't translated (it extends `ConfigHolder`).
        // ConfigHolderObject.ConfigHolderObject(6): not translated, its class isn't translated (it extends `ConfigHolder`).
        // ConfigHolderObject.getRegistry(0): not translated, its class isn't translated (it extends `ConfigHolder`).
        // ConfigHolderObject.restoreDefault(2): not translated, its class isn't translated (it extends `ConfigHolder`).
        // ConfigHolderObject.load(5): not translated, its class isn't translated (it extends `ConfigHolder`).
        // ConfigHolderObject.configured(1): not translated, its class isn't translated (it extends `ConfigHolder`).
    }

    /// `CreativeConfigRegistry` (team/creative/creativecore/common/config/holder/CreativeConfigRegistry.java)
    pub mod creative_config_registry {
        use super::*;
        // The class itself isn't translated: it extends `ConfigHolderDynamic`. Its static parts are below.
        // ROOT: not translated, it uses CreativeConfigRegistry, which isn't translated (it extends `ConfigHolderDynamic`).
        // FIELD_PREDICATE: not translated, it uses `Field` (java.lang.reflect).
        // EQUAL_CHECKER: not translated, it uses ConfigEqualChecker, which isn't translated (it uses `Class`).
        // CreativeConfigRegistry.registerValue(2): not translated, its class isn't translated (it extends `ConfigHolderDynamic`).
        // CreativeConfigRegistry.registerValue(4): not translated, its class isn't translated (it extends `ConfigHolderDynamic`).
        // CreativeConfigRegistry.followPath(1): not translated, its class isn't translated (it extends `ConfigHolderDynamic`).
        // CreativeConfigRegistry.findKey(1): not translated, its class isn't translated (it extends `ConfigHolderDynamic`).
        // CreativeConfigRegistry.removeField(1): not translated, its class isn't translated (it extends `ConfigHolderDynamic`).
        // CreativeConfigRegistry.load(3): not translated, it uses Minecraft's `Provider`.
        // CreativeConfigRegistry.getEqualChecker(0): not translated, its class isn't translated (it extends `ConfigHolderDynamic`).
        // CreativeConfigRegistry.is(1): not translated, its class isn't translated (it extends `ConfigHolderDynamic`).
        // CreativeConfigRegistry.getRegistry(0): not translated, its class isn't translated (it extends `ConfigHolderDynamic`).
    }

    /// `ICreativeConfigHolder` (team/creative/creativecore/common/config/holder/ICreativeConfigHolder.java)
    pub mod i_creative_config_holder {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ICreativeConfigHolder.read(6): not translated, it uses Minecraft's `Provider`.
        // ICreativeConfigHolder.write(5): not translated, it uses Minecraft's `Provider`.
        // ICreativeConfigHolder.parent(0): not translated, it has no body.
        // ICreativeConfigHolder.getName(0): not translated, its class isn't translated (it's an interface).
        // ICreativeConfigHolder.path(0): not translated, it has no body.
        // ICreativeConfigHolder.fields(0): not translated, it has no body.
        // ICreativeConfigHolder.names(0): not translated, it has no body.
        // ICreativeConfigHolder.get(1): not translated, it has no body.
        // ICreativeConfigHolder.getField(1): not translated, it has no body.
        // ICreativeConfigHolder.load(5): not translated, it has no body.
        // ICreativeConfigHolder.save(4): not translated, it has no body.
        // ICreativeConfigHolder.isEmpty(1): not translated, it has no body.
        // ICreativeConfigHolder.isEmptyWithoutForce(1): not translated, it has no body.
        // ICreativeConfigHolder.synchronization(0): not translated, it has no body.
        // ICreativeConfigHolder.getRegistry(0): not translated, it has no body.
    }

    /// `ConfigKey` (team/creative/creativecore/common/config/key/ConfigKey.java)
    pub mod config_key {
        use super::*;
        // The class itself isn't translated: it uses ConfigSynchronization, which isn't translated (its constants carry values). Its static parts are below.
        // ConfigKey.of(8): not translated, it uses ConfigKey, which isn't translated (it uses ConfigSynchronization, which isn't translated (its constants carry values)).
        // ConfigKey.of(7): not translated, it uses ConfigKey, which isn't translated (it uses ConfigSynchronization, which isn't translated (its constants carry values)).
        // ConfigKey.ofGenericType(2): not translated, it uses ConfigKey, which isn't translated (it uses ConfigSynchronization, which isn't translated (its constants carry values)).
        // ConfigKey.ofArrayType(2): not translated, it uses ConfigKey, which isn't translated (it uses ConfigSynchronization, which isn't translated (its constants carry values)).
        // ConfigKey.ofType(3): not translated, it uses ConfigKey, which isn't translated (it uses ConfigSynchronization, which isn't translated (its constants carry values)).
        // ConfigKey.ConfigKey(5): not translated, its class isn't translated (it uses ConfigSynchronization, which isn't translated (its constants carry values)).
        // ConfigKey.get(0): not translated, its class isn't translated (it uses ConfigSynchronization, which isn't translated (its constants carry values)).
        // ConfigKey.hashCode(0): not translated, its class isn't translated (it uses ConfigSynchronization, which isn't translated (its constants carry values)).
        // ConfigKey.equals(1): not translated, its class isn't translated (it uses ConfigSynchronization, which isn't translated (its constants carry values)).
        // ConfigKey.toString(0): not translated, its class isn't translated (it uses ConfigSynchronization, which isn't translated (its constants carry values)).
        // ConfigKey.checkEqual(3): not translated, its class isn't translated (it uses ConfigSynchronization, which isn't translated (its constants carry values)).
        // ConfigKey.isFolder(0): not translated, it has no body.
        // ConfigKey.holder(0): not translated, it has no body.
        // ConfigKey.converation(0): not translated, it has no body.
        // ConfigKey.is(1): not translated, its class isn't translated (it uses ConfigSynchronization, which isn't translated (its constants carry values)).
        // ConfigKey.isWithoutForce(1): not translated, its class isn't translated (it uses ConfigSynchronization, which isn't translated (its constants carry values)).
        // ConfigKey.triggerConfigured(1): not translated, it has no body.
        // ConfigKey.isDefault(1): not translated, it has no body.
        // ConfigKey.isDefault(2): not translated, it has no body.
        // ConfigKey.restoreDefault(2): not translated, it has no body.
        // ConfigKey.forceValue(2): not translated, it has no body.
        // ConfigKey.read(5): not translated, it has no body.
        // ConfigKey.write(4): not translated, it has no body.
        // ConfigKey.copy(2): not translated, it has no body.
        // ConfigKey.create(3): not translated, it has no body.
        // ConfigKey.load(3): not translated, it has no body.
        // ConfigKey.save(3): not translated, it has no body.
        // ConfigKey.field(0): not translated, its class isn't translated (it uses ConfigSynchronization, which isn't translated (its constants carry values)).
        // ConfigKey.getRegistry(0): not translated, it has no body.
    }

    /// `ConfigKeyHolder` (team/creative/creativecore/common/config/key/ConfigKeyHolder.java)
    pub mod config_key_holder {
        use super::*;
        // The class itself isn't translated: it extends `ConfigKey`. Its static parts are below.
        // ConfigKeyHolder.ConfigKeyHolder(6): not translated, its class isn't translated (it extends `ConfigKey`).
        // ConfigKeyHolder.triggerConfigured(1): not translated, its class isn't translated (it extends `ConfigKey`).
        // ConfigKeyHolder.isDefault(1): not translated, its class isn't translated (it extends `ConfigKey`).
        // ConfigKeyHolder.isDefault(2): not translated, its class isn't translated (it extends `ConfigKey`).
        // ConfigKeyHolder.restoreDefault(2): not translated, its class isn't translated (it extends `ConfigKey`).
        // ConfigKeyHolder.isFolder(0): not translated, its class isn't translated (it extends `ConfigKey`).
        // ConfigKeyHolder.holder(0): not translated, its class isn't translated (it extends `ConfigKey`).
        // ConfigKeyHolder.copy(2): not translated, its class isn't translated (it extends `ConfigKey`).
        // ConfigKeyHolder.read(5): not translated, its class isn't translated (it extends `ConfigKey`).
        // ConfigKeyHolder.write(4): not translated, its class isn't translated (it extends `ConfigKey`).
        // ConfigKeyHolder.forceValue(2): not translated, its class isn't translated (it extends `ConfigKey`).
        // ConfigKeyHolder.converation(0): not translated, its class isn't translated (it extends `ConfigKey`).
        // ConfigKeyHolder.create(3): not translated, its class isn't translated (it extends `ConfigKey`).
        // ConfigKeyHolder.load(3): not translated, its class isn't translated (it extends `ConfigKey`).
        // ConfigKeyHolder.save(3): not translated, its class isn't translated (it extends `ConfigKey`).
        // ConfigKeyHolder.getRegistry(0): not translated, its class isn't translated (it extends `ConfigKey`).
    }

    /// `ConfigKeyType` (team/creative/creativecore/common/config/key/ConfigKeyType.java)
    pub mod config_key_type {
        use super::*;
        // The class itself isn't translated: it extends `ConfigKey`. Its static parts are below.
        // ConfigKeyType.ConfigKeyType(7): not translated, its class isn't translated (it extends `ConfigKey`).
        // ConfigKeyType.set(2): not translated, its class isn't translated (it extends `ConfigKey`).
        // ConfigKeyType.checkEqual(3): not translated, its class isn't translated (it extends `ConfigKey`).
        // ConfigKeyType.isDefault(1): not translated, its class isn't translated (it extends `ConfigKey`).
        // ConfigKeyType.isDefault(2): not translated, its class isn't translated (it extends `ConfigKey`).
        // ConfigKeyType.restoreDefault(2): not translated, its class isn't translated (it extends `ConfigKey`).
        // ConfigKeyType.read(5): not translated, its class isn't translated (it extends `ConfigKey`).
        // ConfigKeyType.write(4): not translated, its class isn't translated (it extends `ConfigKey`).
        // ConfigKeyType.copy(2): not translated, its class isn't translated (it extends `ConfigKey`).
        // ConfigKeyType.isFolder(0): not translated, its class isn't translated (it extends `ConfigKey`).
        // ConfigKeyType.holder(0): not translated, its class isn't translated (it extends `ConfigKey`).
        // ConfigKeyType.triggerConfigured(1): not translated, its class isn't translated (it extends `ConfigKey`).
        // ConfigKeyType.forceValue(2): not translated, its class isn't translated (it extends `ConfigKey`).
        // ConfigKeyType.converation(0): not translated, its class isn't translated (it extends `ConfigKey`).
        // ConfigKeyType.create(3): not translated, its class isn't translated (it extends `ConfigKey`).
        // ConfigKeyType.load(3): not translated, its class isn't translated (it extends `ConfigKey`).
        // ConfigKeyType.save(3): not translated, its class isn't translated (it extends `ConfigKey`).
        // ConfigKeyType.getRegistry(0): not translated, its class isn't translated (it extends `ConfigKey`).
    }

    /// `Curve` (team/creative/creativecore/common/config/premade/curve/Curve.java)
    pub mod curve {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // Curve.valueAt(1): not translated, it has no body.
    }

    /// `DecimalCurve` (team/creative/creativecore/common/config/premade/curve/DecimalCurve.java)
    pub mod decimal_curve {
        use super::*;
        // The class itself isn't translated: it implements `ICreativeConfig` (team.creative.creativecore.common.config.api). Its static parts are below.
        // DecimalCurve.DecimalCurve(4): not translated, its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)).
        // DecimalCurve.DecimalCurve(3): not translated, its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)).
        // DecimalCurve.DecimalCurve(3): not translated, its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)).
        // DecimalCurve.valueAt(1): not translated, its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)).
        // DecimalCurve.configured(1): not translated, its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)).
    }

    /// `IntCurve` (team/creative/creativecore/common/config/premade/curve/IntCurve.java)
    pub mod int_curve {
        use super::*;
        // The class itself isn't translated: it implements `ICreativeConfig` (team.creative.creativecore.common.config.api). Its static parts are below.
        // IntCurve.IntCurve(4): not translated, its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)).
        // IntCurve.valueAt(1): not translated, its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)).
        // IntCurve.configured(1): not translated, its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)).
    }

    /// `LinearCurve` (team/creative/creativecore/common/config/premade/curve/LinearCurve.java)
    pub mod linear_curve {
        use super::*;
        // The class itself isn't translated: it implements `ICreativeConfig` (team.creative.creativecore.common.config.api). Its static parts are below.
        // LinearCurve.LinearCurve(1): not translated, its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)).
        // LinearCurve.valueAt(1): not translated, its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)).
        // LinearCurve.configured(1): not translated, its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)).
    }

    /// `DecimalMinMax` (team/creative/creativecore/common/config/premade/DecimalMinMax.java)
    pub mod decimal_min_max {
        use super::*;
        // The class itself isn't translated: it implements `ICreativeConfig` (team.creative.creativecore.common.config.api). Its static parts are below.
        // DecimalMinMax.DecimalMinMax(2): not translated, its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)).
        // DecimalMinMax.next(1): not translated, its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)).
        // DecimalMinMax.configured(1): not translated, its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)).
        // DecimalMinMax.spanLength(0): not translated, its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)).
    }

    /// `IntMinMax` (team/creative/creativecore/common/config/premade/IntMinMax.java)
    pub mod int_min_max {
        use super::*;
        // The class itself isn't translated: it implements `ICreativeConfig` (team.creative.creativecore.common.config.api). Its static parts are below.
        // IntMinMax.IntMinMax(2): not translated, its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)).
        // IntMinMax.next(1): not translated, its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)).
        // IntMinMax.configured(1): not translated, its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)).
        // IntMinMax.spanLength(0): not translated, its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)).
    }

    /// `MobEffectConfig` (team/creative/creativecore/common/config/premade/MobEffectConfig.java)
    pub mod mob_effect_config {
        use super::*;
        // The class itself isn't translated: it uses RegistryObjectConfig, which isn't translated (it's generic). Its static parts are below.
        // MobEffectConfig.MobEffectConfig(5): not translated, its class isn't translated (it uses RegistryObjectConfig, which isn't translated (it's generic)).
        // MobEffectConfig.create(0): not translated, its class isn't translated (it uses RegistryObjectConfig, which isn't translated (it's generic)).
        // MobEffectConfig.equals(1): not translated, its class isn't translated (it uses RegistryObjectConfig, which isn't translated (it's generic)).
    }

    /// `NamedList` (team/creative/creativecore/common/config/premade/NamedList.java)
    pub mod named_list {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // NamedList.entrySet(0): not translated, its class isn't translated (it's generic).
        // NamedList.keySet(0): not translated, its class isn't translated (it's generic).
    }

    /// `Permission` (team/creative/creativecore/common/config/premade/Permission.java)
    pub mod permission {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // Permission.Permission(1): not translated, its class isn't translated (it's generic).
        // Permission.getDefault(0): not translated, its class isn't translated (it's generic).
        // Permission.add(2): not translated, its class isn't translated (it's generic).
        // Permission.put(2): not translated, its class isn't translated (it's generic).
        // Permission.get(1): not translated, its class isn't translated (it's generic).
        // Permission.getDirect(1): not translated, its class isn't translated (it's generic).
        // Permission.get(1): not translated, its class isn't translated (it's generic).
        // Permission.getOrDefault(2): not translated, its class isn't translated (it's generic).
    }

    /// `RegistryObjectConfig` (team/creative/creativecore/common/config/premade/registry/RegistryObjectConfig.java)
    pub mod registry_object_config {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // RegistryObjectConfig.RegistryObjectConfig(2): not translated, its class isn't translated (it's generic).
        // RegistryObjectConfig.configured(1): not translated, its class isn't translated (it's generic).
        // RegistryObjectConfig.getHolder(0): not translated, its class isn't translated (it's generic).
        // RegistryObjectConfig.equals(1): not translated, its class isn't translated (it's generic).
    }

    /// `RegistryObjectListConfig` (team/creative/creativecore/common/config/premade/registry/RegistryObjectListConfig.java)
    pub mod registry_object_list_config {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // RegistryObjectListConfig.RegistryObjectListConfig(1): not translated, its class isn't translated (it's generic).
        // RegistryObjectListConfig.add(1): not translated, its class isn't translated (it's generic).
        // RegistryObjectListConfig.iterator(0): not translated, its class isn't translated (it's generic).
        // RegistryObjectListConfig.identifiers(0): not translated, its class isn't translated (it's generic).
        // RegistryObjectListConfig.contains(1): not translated, its class isn't translated (it's generic).
        // RegistryObjectListConfig.getLocation(1): not translated, its class isn't translated (it's generic).
        // RegistryObjectListConfig.size(0): not translated, its class isn't translated (it's generic).
    }

    /// `RegistryObjectListConfig.RegistryHolder` (team/creative/creativecore/common/config/premade/registry/RegistryObjectListConfig.java)
    pub mod registry_object_list_config_registry_holder {
        use super::*;
        // The class itself isn't translated: it uses `T`. Its static parts are below.
        // RegistryObjectListConfig.RegistryHolder.RegistryHolder(1): not translated, its class isn't translated (it uses `T`).
        // RegistryObjectListConfig.RegistryHolder.hashCode(0): not translated, its class isn't translated (it uses `T`).
        // RegistryObjectListConfig.RegistryHolder.equals(1): not translated, its class isn't translated (it uses `T`).
        // RegistryObjectListConfig.RegistryHolder.get(0): not translated, its class isn't translated (it uses `T`).
    }

    /// `RegistryTagConfig` (team/creative/creativecore/common/config/premade/registry/RegistryTagConfig.java)
    pub mod registry_tag_config {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // RegistryTagConfig.RegistryTagConfig(2): not translated, its class isn't translated (it's generic).
        // RegistryTagConfig.equals(1): not translated, its class isn't translated (it's generic).
    }

    /// `RegistryTagListConfig` (team/creative/creativecore/common/config/premade/registry/RegistryTagListConfig.java)
    pub mod registry_tag_list_config {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // RegistryTagListConfig.RegistryTagListConfig(1): not translated, its class isn't translated (it's generic).
        // RegistryTagListConfig.add(1): not translated, its class isn't translated (it's generic).
        // RegistryTagListConfig.iterator(0): not translated, its class isn't translated (it's generic).
        // RegistryTagListConfig.get(1): not translated, its class isn't translated (it's generic).
        // RegistryTagListConfig.size(0): not translated, its class isn't translated (it's generic).
    }

    /// `SelectableConfig` (team/creative/creativecore/common/config/premade/SelectableConfig.java)
    pub mod selectable_config {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // SelectableConfig.SelectableConfig(2): not translated, its class isn't translated (it's generic).
        // SelectableConfig.reset(0): not translated, its class isn't translated (it's generic).
        // SelectableConfig.select(1): not translated, its class isn't translated (it's generic).
        // SelectableConfig.get(0): not translated, its class isn't translated (it's generic).
        // SelectableConfig.getArray(0): not translated, its class isn't translated (it's generic).
        // SelectableConfig.getSelected(0): not translated, its class isn't translated (it's generic).
        // SelectableConfig.updateArray(2): not translated, its class isn't translated (it's generic).
        // SelectableConfig.isDefault(1): not translated, its class isn't translated (it's generic).
        // SelectableConfig.restoreDefault(2): not translated, its class isn't translated (it's generic).
        // SelectableConfig.configured(1): not translated, its class isn't translated (it's generic).
    }

    /// `SoundConfig` (team/creative/creativecore/common/config/premade/SoundConfig.java)
    pub mod sound_config {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct SoundConfig {
            pub event: Option<JIdentifier>,
            pub volume: f32,
            pub pitch: f32,
        }
        impl SoundConfig {
            pub fn new_3(location: &JIdentifier, volume: f32, pitch: f32) -> Self {
                let mut s = Self { event: Some(location.clone()), volume: volume, pitch: pitch };
                s.event = Some(location.clone());
                s.volume = volume;
                s.pitch = pitch;
                s
            }
            pub fn new_1(location: &JIdentifier) -> Self {
                let mut s = Self::new_3(location, 1.0f32, 1.0f32);
                s
            }
            // SoundConfig.getEvent(0): not translated, it uses Minecraft's `SoundEvent`.
            // SoundConfig.play(2): not translated, it uses Minecraft's `Entity`.
            // SoundConfig.play(5): not translated, it uses Minecraft's `Level`.
            // SoundConfig.equals(1): not translated, it uses `Object`.
        }
    }

    /// `ToggleableConfig` (team/creative/creativecore/common/config/premade/ToggleableConfig.java)
    pub mod toggleable_config {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // ToggleableConfig.ToggleableConfig(1): not translated, its class isn't translated (it's generic).
        // ToggleableConfig.ToggleableConfig(2): not translated, its class isn't translated (it's generic).
        // ToggleableConfig.configured(1): not translated, its class isn't translated (it's generic).
        // ToggleableConfig.setEnabled(1): not translated, its class isn't translated (it's generic).
        // ToggleableConfig.isEnabled(0): not translated, its class isn't translated (it's generic).
        // ToggleableConfig.equals(1): not translated, its class isn't translated (it's generic).
    }

    /// `ConfigSynchronization` (team/creative/creativecore/common/config/sync/ConfigSynchronization.java)
    pub mod config_synchronization {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // ConfigSynchronization.useFolder(2): not translated, it has no body.
        // ConfigSynchronization.useValue(2): not translated, it has no body.
    }

    /// `ConfigurationChangePacket` (team/creative/creativecore/common/config/sync/ConfigurationChangePacket.java)
    pub mod configuration_change_packet {
        use super::*;
        // The class itself isn't translated: it extends `CreativePacket` (team.creative.creativecore.common.network). Its static parts are below.
        // ConfigurationChangePacket.ConfigurationChangePacket(2): not translated, its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)).
        // ConfigurationChangePacket.ConfigurationChangePacket(0): not translated, its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)).
        // ConfigurationChangePacket.executeClient(1): not translated, its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)).
        // ConfigurationChangePacket.executeServer(1): not translated, its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)).
    }

    /// `ConfigurationClientPacket` (team/creative/creativecore/common/config/sync/ConfigurationClientPacket.java)
    pub mod configuration_client_packet {
        use super::*;
        // The class itself isn't translated: it extends `CreativePacket` (team.creative.creativecore.common.network). Its static parts are below.
        // ConfigurationClientPacket.ConfigurationClientPacket(2): not translated, its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)).
        // ConfigurationClientPacket.ConfigurationClientPacket(1): not translated, its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)).
        // ConfigurationClientPacket.buildClientFieldList(2): not translated, its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)).
        // ConfigurationClientPacket.ConfigurationClientPacket(0): not translated, its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)).
        // ConfigurationClientPacket.run(0): not translated, its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)).
        // ConfigurationClientPacket.executeClient(1): not translated, its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)).
        // ConfigurationClientPacket.executeServer(1): not translated, its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)).
    }

    /// `ConfigurationPacket` (team/creative/creativecore/common/config/sync/ConfigurationPacket.java)
    pub mod configuration_packet {
        use super::*;
        // The class itself isn't translated: it extends `CreativePacket` (team.creative.creativecore.common.network). Its static parts are below.
        // ConfigurationPacket.ConfigurationPacket(3): not translated, its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)).
        // ConfigurationPacket.ConfigurationPacket(0): not translated, its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)).
        // ConfigurationPacket.executeClient(1): not translated, its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)).
        // ConfigurationPacket.executeServer(1): not translated, its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)).
        // ConfigurationPacket.updateGui(1): not translated, it uses Minecraft's `Player`.
    }

    /// `Align` (team/creative/creativecore/common/gui/Align.java)
    pub mod align {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum Align {
            Left,
            Center,
            Right,
            Stretch,
        }
        impl Align {
            pub const VALUES: [Align; 4] = [Align::Left, Align::Center, Align::Right, Align::Stretch, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    Align::Left => "LEFT",
                    Align::Center => "CENTER",
                    Align::Right => "RIGHT",
                    Align::Stretch => "STRETCH",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "LEFT" => Align::Left,
                    "CENTER" => Align::Center,
                    "RIGHT" => Align::Right,
                    "STRETCH" => Align::Stretch,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `GuiCheckList` (team/creative/creativecore/common/gui/control/collection/GuiCheckList.java)
    pub mod gui_check_list {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GuiCheckList.GuiCheckList(5): not translated, its class isn't translated (it's generic).
        // GuiCheckList.dist(0): not translated, its class isn't translated (it's generic).
        // GuiCheckList.set(2): not translated, its class isn't translated (it's generic).
        // GuiCheckList.createControl(3): not translated, its class isn't translated (it's generic).
        // GuiCheckList.removeControl(1): not translated, its class isn't translated (it's generic).
        // GuiCheckList.addCustomControl(1): not translated, its class isn't translated (it's generic).
        // GuiCheckList.addCustom(1): not translated, its class isn't translated (it's generic).
        // GuiCheckList.add(1): not translated, its class isn't translated (it's generic).
        // GuiCheckList.addHover(1): not translated, its class isn't translated (it's generic).
        // GuiCheckList.removeItem(1): not translated, its class isn't translated (it's generic).
        // GuiCheckList.isEmpty(0): not translated, its class isn't translated (it's generic).
        // GuiCheckList.size(0): not translated, its class isn't translated (it's generic).
        // GuiCheckList.getValue(1): not translated, its class isn't translated (it's generic).
        // GuiCheckList.selectedItems(0): not translated, its class isn't translated (it's generic).
        // GuiCheckList.allItems(0): not translated, its class isn't translated (it's generic).
        // GuiCheckList.checked(1): not translated, its class isn't translated (it's generic).
        // GuiCheckList.indexOf(1): not translated, its class isn't translated (it's generic).
    }

    /// `GuiCheckList.GuiCheckListDist` (team/creative/creativecore/common/gui/control/collection/GuiCheckList.java)
    pub mod gui_check_list_gui_check_list_dist {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // GuiCheckList.GuiCheckListDist.reflowInternal(0): not translated, it has no body.
    }

    /// `GuiCheckList.GuiCheckListRow` (team/creative/creativecore/common/gui/control/collection/GuiCheckList.java)
    pub mod gui_check_list_gui_check_list_row {
        use super::*;
        // The class itself isn't translated: it extends `GuiRow` (team.creative.creativecore.common.gui.control.parent). Its static parts are below.
        // GuiCheckList.GuiCheckListRow.GuiCheckListRow(4): not translated, its class isn't translated (it extends `GuiRow` (team.creative.creativecore.common.gui.control.parent)).
    }

    /// `GuiComboBox` (team/creative/creativecore/common/gui/control/collection/GuiComboBox.java)
    pub mod gui_combo_box {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GuiComboBox.GuiComboBox(4): not translated, its class isn't translated (it's generic).
        // GuiComboBox.GuiComboBox(3): not translated, its class isn't translated (it's generic).
        // GuiComboBox.dist(0): not translated, its class isn't translated (it's generic).
        // GuiComboBox.hasSearchbar(0): not translated, its class isn't translated (it's generic).
        // GuiComboBox.setSearchbar(1): not translated, its class isn't translated (it's generic).
        // GuiComboBox.setDirection(1): not translated, its class isn't translated (it's generic).
        // GuiComboBox.set(1): not translated, its class isn't translated (it's generic).
        // GuiComboBox.selected(0): not translated, its class isn't translated (it's generic).
        // GuiComboBox.selected(1): not translated, its class isn't translated (it's generic).
        // GuiComboBox.selectSilent(1): not translated, its class isn't translated (it's generic).
        // GuiComboBox.select(1): not translated, its class isn't translated (it's generic).
        // GuiComboBox.selectSilent(1): not translated, its class isn't translated (it's generic).
        // GuiComboBox.select(1): not translated, its class isn't translated (it's generic).
        // GuiComboBox.indexOf(1): not translated, its class isn't translated (it's generic).
        // GuiComboBox.next(0): not translated, its class isn't translated (it's generic).
        // GuiComboBox.previous(0): not translated, its class isn't translated (it's generic).
        // GuiComboBox.selectedIndex(0): not translated, its class isn't translated (it's generic).
    }

    /// `GuiComboBox.GuiComboBoxDist` (team/creative/creativecore/common/gui/control/collection/GuiComboBox.java)
    pub mod gui_combo_box_gui_combo_box_dist {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // GuiComboBox.GuiComboBoxDist.hasSearchbar(0): not translated, it has no body.
        // GuiComboBox.GuiComboBoxDist.setSearchbar(1): not translated, it has no body.
        // GuiComboBox.GuiComboBoxDist.setDirection(1): not translated, it has no body.
        // GuiComboBox.GuiComboBoxDist.set(2): not translated, it has no body.
        // GuiComboBox.GuiComboBoxDist.selected(0): not translated, it has no body.
        // GuiComboBox.GuiComboBoxDist.selected(1): not translated, it has no body.
        // GuiComboBox.GuiComboBoxDist.select(2): not translated, it has no body.
        // GuiComboBox.GuiComboBoxDist.select(2): not translated, it has no body.
        // GuiComboBox.GuiComboBoxDist.indexOf(1): not translated, it has no body.
        // GuiComboBox.GuiComboBoxDist.next(0): not translated, it has no body.
        // GuiComboBox.GuiComboBoxDist.previous(0): not translated, it has no body.
        // GuiComboBox.GuiComboBoxDist.selectedIndex(0): not translated, it has no body.
    }

    /// `GuiComboBoxExtension` (team/creative/creativecore/common/gui/control/collection/GuiComboBoxExtension.java)
    pub mod gui_combo_box_extension {
        use super::*;
        // The class itself isn't translated: it extends `GuiListBoxBase`. Its static parts are below.
        // GuiComboBoxExtension.GuiComboBoxExtension(2): not translated, its class isn't translated (it extends `GuiListBoxBase`).
    }

    /// `GuiComboBoxFlexible` (team/creative/creativecore/common/gui/control/collection/GuiComboBoxFlexible.java)
    pub mod gui_combo_box_flexible {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GuiComboBoxFlexible.GuiComboBoxFlexible(4): not translated, its class isn't translated (it's generic).
        // GuiComboBoxFlexible.dist(0): not translated, its class isn't translated (it's generic).
        // GuiComboBoxFlexible.forceSelect(1): not translated, its class isn't translated (it's generic).
    }

    /// `GuiComboBoxFlexible.GuiComboBoxFlexibleDist` (team/creative/creativecore/common/gui/control/collection/GuiComboBoxFlexible.java)
    pub mod gui_combo_box_flexible_gui_combo_box_flexible_dist {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // GuiComboBoxFlexible.GuiComboBoxFlexibleDist.init(1): not translated, it has no body.
        // GuiComboBoxFlexible.GuiComboBoxFlexibleDist.forceSelect(2): not translated, it has no body.
    }

    /// `GuiComboBoxTree` (team/creative/creativecore/common/gui/control/collection/GuiComboBoxTree.java)
    pub mod gui_combo_box_tree {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GuiComboBoxTree.GuiComboBoxTree(5): not translated, its class isn't translated (it's generic).
        // GuiComboBoxTree.GuiComboBoxTree(4): not translated, its class isn't translated (it's generic).
        // GuiComboBoxTree.dist(0): not translated, its class isn't translated (it's generic).
        // GuiComboBoxTree.hasSearchbar(0): not translated, its class isn't translated (it's generic).
        // GuiComboBoxTree.setSearchbar(1): not translated, its class isn't translated (it's generic).
        // GuiComboBoxTree.setDirection(1): not translated, its class isn't translated (it's generic).
        // GuiComboBoxTree.set(1): not translated, its class isn't translated (it's generic).
        // GuiComboBoxTree.selected(0): not translated, its class isn't translated (it's generic).
        // GuiComboBoxTree.selected(1): not translated, its class isn't translated (it's generic).
        // GuiComboBoxTree.select(2): not translated, its class isn't translated (it's generic).
        // GuiComboBoxTree.select(1): not translated, its class isn't translated (it's generic).
    }

    /// `GuiComboBoxTree.GuiComboBoxTreeDist` (team/creative/creativecore/common/gui/control/collection/GuiComboBoxTree.java)
    pub mod gui_combo_box_tree_gui_combo_box_tree_dist {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // GuiComboBoxTree.GuiComboBoxTreeDist.init(1): not translated, it has no body.
        // GuiComboBoxTree.GuiComboBoxTreeDist.hasSearchbar(0): not translated, it has no body.
        // GuiComboBoxTree.GuiComboBoxTreeDist.setSearchbar(1): not translated, it has no body.
        // GuiComboBoxTree.GuiComboBoxTreeDist.setDirection(1): not translated, it has no body.
        // GuiComboBoxTree.GuiComboBoxTreeDist.set(2): not translated, it has no body.
        // GuiComboBoxTree.GuiComboBoxTreeDist.selected(0): not translated, it has no body.
        // GuiComboBoxTree.GuiComboBoxTreeDist.selected(1): not translated, it has no body.
        // GuiComboBoxTree.GuiComboBoxTreeDist.select(3): not translated, it has no body.
        // GuiComboBoxTree.GuiComboBoxTreeDist.select(2): not translated, it has no body.
    }

    /// `GuiListBoxBase` (team/creative/creativecore/common/gui/control/collection/GuiListBoxBase.java)
    pub mod gui_list_box_base {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GuiListBoxBase.GuiListBoxBase(4): not translated, its class isn't translated (it's generic).
        // GuiListBoxBase.dist(0): not translated, its class isn't translated (it's generic).
        // GuiListBoxBase.createItems(0): not translated, its class isn't translated (it's generic).
        // GuiListBoxBase.createControl(1): not translated, its class isn't translated (it's generic).
        // GuiListBoxBase.removeControl(1): not translated, its class isn't translated (it's generic).
        // GuiListBoxBase.addCustomControl(1): not translated, its class isn't translated (it's generic).
        // GuiListBoxBase.add(1): not translated, its class isn't translated (it's generic).
        // GuiListBoxBase.addHover(1): not translated, its class isn't translated (it's generic).
        // GuiListBoxBase.removeItem(1): not translated, its class isn't translated (it's generic).
        // GuiListBoxBase.clearItems(0): not translated, its class isn't translated (it's generic).
        // GuiListBoxBase.addAllItems(1): not translated, its class isn't translated (it's generic).
        // GuiListBoxBase.addItem(1): not translated, its class isn't translated (it's generic).
        // GuiListBoxBase.isListEmpty(0): not translated, its class isn't translated (it's generic).
        // GuiListBoxBase.itemSize(0): not translated, its class isn't translated (it's generic).
        // GuiListBoxBase.getItem(1): not translated, its class isn't translated (it's generic).
        // GuiListBoxBase.items(0): not translated, its class isn't translated (it's generic).
    }

    /// `GuiListBoxBase.GuiButtonRemove` (team/creative/creativecore/common/gui/control/collection/GuiListBoxBase.java)
    pub mod gui_list_box_base_gui_button_remove {
        use super::*;
        // The class itself isn't translated: it extends `GuiButton` (team.creative.creativecore.common.gui.control.simple). Its static parts are below.
        // GuiListBoxBase.GuiButtonRemove.GuiButtonRemove(2): not translated, its class isn't translated (it extends `GuiButton` (team.creative.creativecore.common.gui.control.simple)).
    }

    /// `GuiListBoxBase.GuiListBoxBaseDist` (team/creative/creativecore/common/gui/control/collection/GuiListBoxBase.java)
    pub mod gui_list_box_base_gui_list_box_base_dist {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // GuiListBoxBase.GuiListBoxBaseDist.reflowInternal(0): not translated, it has no body.
    }

    /// `GuiStackSelector` (team/creative/creativecore/common/gui/control/collection/GuiStackSelector.java)
    pub mod gui_stack_selector {
        use super::*;
        // The class itself isn't translated: it extends `GuiLabel` (team.creative.creativecore.common.gui.control.simple). Its static parts are below.
        // GuiStackSelector.GuiStackSelector(4): not translated, its class isn't translated (it extends `GuiLabel` (team.creative.creativecore.common.gui.control.simple)).
        // GuiStackSelector.GuiStackSelector(3): not translated, its class isn't translated (it extends `GuiLabel` (team.creative.creativecore.common.gui.control.simple)).
        // GuiStackSelector.dist(0): not translated, its class isn't translated (it extends `GuiLabel` (team.creative.creativecore.common.gui.control.simple)).
        // GuiStackSelector.setWidth(1): not translated, its class isn't translated (it extends `GuiLabel` (team.creative.creativecore.common.gui.control.simple)).
        // GuiStackSelector.hasSearchbar(0): not translated, its class isn't translated (it extends `GuiLabel` (team.creative.creativecore.common.gui.control.simple)).
        // GuiStackSelector.setSearchbar(1): not translated, its class isn't translated (it extends `GuiLabel` (team.creative.creativecore.common.gui.control.simple)).
        // GuiStackSelector.selectFirst(0): not translated, its class isn't translated (it extends `GuiLabel` (team.creative.creativecore.common.gui.control.simple)).
        // GuiStackSelector.updateCollectedStacks(0): not translated, its class isn't translated (it extends `GuiLabel` (team.creative.creativecore.common.gui.control.simple)).
        // GuiStackSelector.setSelectedForce(1): not translated, its class isn't translated (it extends `GuiLabel` (team.creative.creativecore.common.gui.control.simple)).
        // GuiStackSelector.setSelected(1): not translated, its class isn't translated (it extends `GuiLabel` (team.creative.creativecore.common.gui.control.simple)).
        // GuiStackSelector.getStacks(0): not translated, its class isn't translated (it extends `GuiLabel` (team.creative.creativecore.common.gui.control.simple)).
        // GuiStackSelector.getSelected(0): not translated, its class isn't translated (it extends `GuiLabel` (team.creative.creativecore.common.gui.control.simple)).
        // GuiStackSelector.contains(2): not translated, it uses Minecraft's `ItemStack`.
        // GuiStackSelector.getItemName(1): not translated, it uses Minecraft's `ItemStack`.
    }

    /// `GuiStackSelector.CreativeCollector` (team/creative/creativecore/common/gui/control/collection/GuiStackSelector.java)
    pub mod gui_stack_selector_creative_collector {
        use super::*;
        // The class itself isn't translated: it extends `InventoryCollector`. Its static parts are below.
        // GuiStackSelector.CreativeCollector.CreativeCollector(1): not translated, its class isn't translated (it extends `InventoryCollector`).
        // GuiStackSelector.CreativeCollector.collect(1): not translated, its class isn't translated (it extends `InventoryCollector`).
    }

    /// `GuiStackSelector.GuiBlockSelector` (team/creative/creativecore/common/gui/control/collection/GuiStackSelector.java)
    pub mod gui_stack_selector_gui_block_selector {
        use super::*;
        // The class itself isn't translated: it extends `SearchSelector`. Its static parts are below.
        // GuiStackSelector.GuiBlockSelector.allow(1): not translated, its class isn't translated (it extends `SearchSelector`).
    }

    /// `GuiStackSelector.GuiStackSelectorDist` (team/creative/creativecore/common/gui/control/collection/GuiStackSelector.java)
    pub mod gui_stack_selector_gui_stack_selector_dist {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // GuiStackSelector.GuiStackSelectorDist.setCollector(1): not translated, it has no body.
        // GuiStackSelector.GuiStackSelectorDist.hasSearchbar(0): not translated, it has no body.
        // GuiStackSelector.GuiStackSelectorDist.setSearchbar(1): not translated, it has no body.
        // GuiStackSelector.GuiStackSelectorDist.selectFirst(1): not translated, it has no body.
        // GuiStackSelector.GuiStackSelectorDist.updateCollectedStacks(1): not translated, it has no body.
        // GuiStackSelector.GuiStackSelectorDist.setSelectedForce(2): not translated, it has no body.
        // GuiStackSelector.GuiStackSelectorDist.setSelected(2): not translated, it has no body.
        // GuiStackSelector.GuiStackSelectorDist.getStacks(0): not translated, it has no body.
        // GuiStackSelector.GuiStackSelectorDist.getSelected(0): not translated, it has no body.
    }

    /// `GuiStackSelector.InventoryCollector` (team/creative/creativecore/common/gui/control/collection/GuiStackSelector.java)
    pub mod gui_stack_selector_inventory_collector {
        use super::*;
        // The class itself isn't translated: it extends `StackCollector`. Its static parts are below.
        // GuiStackSelector.InventoryCollector.InventoryCollector(1): not translated, its class isn't translated (it extends `StackCollector`).
        // GuiStackSelector.InventoryCollector.collect(1): not translated, its class isn't translated (it extends `StackCollector`).
    }

    /// `GuiStackSelector.SearchSelector` (team/creative/creativecore/common/gui/control/collection/GuiStackSelector.java)
    pub mod gui_stack_selector_search_selector {
        use super::*;
        // The class itself isn't translated: it extends `StackSelector`. Its static parts are below.
        // GuiStackSelector.SearchSelector.allow(1): not translated, its class isn't translated (it extends `StackSelector`).
    }

    /// `GuiStackSelector.StackCollector` (team/creative/creativecore/common/gui/control/collection/GuiStackSelector.java)
    pub mod gui_stack_selector_stack_collector {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct StackCollector {
            pub selector: gui_stack_selector_stack_selector::StackSelector,
        }
        impl StackCollector {
            pub fn new(selector: &gui_stack_selector_stack_selector::StackSelector) -> Self {
                let mut s = Self { selector: selector.clone() };
                s.selector = selector.clone();
                s
            }
            // GuiStackSelector.StackCollector.collect(1): not translated, it has no body.
        }
    }

    /// `GuiStackSelector.StackSelector` (team/creative/creativecore/common/gui/control/collection/GuiStackSelector.java)
    pub mod gui_stack_selector_stack_selector {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct StackSelector {
        }
        impl StackSelector {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // GuiStackSelector.StackSelector.allow(1): not translated, it has no body.
        }
    }

    /// `GuiStackSelectorExtension` (team/creative/creativecore/common/gui/control/collection/GuiStackSelectorExtension.java)
    pub mod gui_stack_selector_extension {
        use super::*;
        // The class itself isn't translated: it extends `GuiScrollY` (team.creative.creativecore.common.gui.control.parent). Its static parts are below.
        // GuiStackSelectorExtension.GuiStackSelectorExtension(2): not translated, its class isn't translated (it extends `GuiScrollY` (team.creative.creativecore.common.gui.control.parent)).
        // GuiStackSelectorExtension.dist(0): not translated, its class isn't translated (it extends `GuiScrollY` (team.creative.creativecore.common.gui.control.parent)).
    }

    /// `GuiStackSelectorExtension.GuiStackSelectorExtensionDist` (team/creative/creativecore/common/gui/control/collection/GuiStackSelectorExtension.java)
    pub mod gui_stack_selector_extension_gui_stack_selector_extension_dist {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // GuiStackSelectorExtension.GuiStackSelectorExtensionDist.reflowInternal(0): not translated, it has no body.
    }

    /// `GuiInventoryGrid` (team/creative/creativecore/common/gui/control/inventory/GuiInventoryGrid.java)
    pub mod gui_inventory_grid {
        use super::*;
        // The class itself isn't translated: it extends `GuiParent` (team.creative.creativecore.common.gui). Its static parts are below.
        // GuiInventoryGrid.GuiInventoryGrid(3): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiInventoryGrid.GuiInventoryGrid(4): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiInventoryGrid.GuiInventoryGrid(5): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiInventoryGrid.GuiInventoryGrid(6): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiInventoryGrid.dist(0): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiInventoryGrid.createInventoryGrid(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiInventoryGrid.disableSlot(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiInventoryGrid.addListener(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiInventoryGrid.addSlot(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiInventoryGrid.add(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiInventoryGrid.addHover(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiInventoryGrid.getSlot(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiInventoryGrid.inventorySize(0): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiInventoryGrid.hasFixedSize(0): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiInventoryGrid.name(0): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiInventoryGrid.tick(0): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiInventoryGrid.setChanged(0): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiInventoryGrid.setChanged(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
    }

    /// `GuiInventoryGrid.GuiInventoryGridDist` (team/creative/creativecore/common/gui/control/inventory/GuiInventoryGrid.java)
    pub mod gui_inventory_grid_gui_inventory_grid_dist {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // GuiInventoryGrid.GuiInventoryGridDist.setGridDim(2): not translated, it has no body.
    }

    /// `GuiInventoryGridPreview` (team/creative/creativecore/common/gui/control/inventory/GuiInventoryGridPreview.java)
    pub mod gui_inventory_grid_preview {
        use super::*;
        // The class itself isn't translated: it extends `GuiParent` (team.creative.creativecore.common.gui). Its static parts are below.
        // GuiInventoryGridPreview.GuiInventoryGridPreview(3): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiInventoryGridPreview.GuiInventoryGridPreview(4): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiInventoryGridPreview.GuiInventoryGridPreview(5): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiInventoryGridPreview.dist(0): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiInventoryGridPreview.hasFixedSize(0): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiInventoryGridPreview.add(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiInventoryGridPreview.addHover(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
    }

    /// `GuiInventoryGridPreview.GuiInventoryGridPreviewDist` (team/creative/creativecore/common/gui/control/inventory/GuiInventoryGridPreview.java)
    pub mod gui_inventory_grid_preview_gui_inventory_grid_preview_dist {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // GuiInventoryGridPreview.GuiInventoryGridPreviewDist.setGridDim(2): not translated, it has no body.
    }

    /// `GuiPlayerInventoryGrid` (team/creative/creativecore/common/gui/control/inventory/GuiPlayerInventoryGrid.java)
    pub mod gui_player_inventory_grid {
        use super::*;
        // The class itself isn't translated: it extends `GuiInventoryGrid`. Its static parts are below.
        // GuiPlayerInventoryGrid.GuiPlayerInventoryGrid(1): not translated, its class isn't translated (it extends `GuiInventoryGrid`).
        // GuiPlayerInventoryGrid.createInventoryGrid(1): not translated, its class isn't translated (it extends `GuiInventoryGrid`).
        // GuiPlayerInventoryGrid.moveInside(2): not translated, its class isn't translated (it extends `GuiInventoryGrid`).
    }

    /// `GuiSlot` (team/creative/creativecore/common/gui/control/inventory/GuiSlot.java)
    pub mod gui_slot {
        use super::*;
        // The class itself isn't translated: it extends `GuiSlotBase`. Its static parts are below.
        // GuiSlot.GuiSlot(3): not translated, its class isn't translated (it extends `GuiSlotBase`).
        // GuiSlot.GuiSlot(4): not translated, its class isn't translated (it extends `GuiSlotBase`).
        // GuiSlot.GuiSlot(2): not translated, its class isn't translated (it extends `GuiSlotBase`).
        // GuiSlot.GuiSlot(3): not translated, its class isn't translated (it extends `GuiSlotBase`).
        // GuiSlot.dist(0): not translated, its class isn't translated (it extends `GuiSlotBase`).
        // GuiSlot.getStack(0): not translated, its class isn't translated (it extends `GuiSlotBase`).
        // GuiSlot.itemManager(0): not translated, its class isn't translated (it extends `GuiSlotBase`).
        // GuiSlot.inventory(0): not translated, its class isn't translated (it extends `GuiSlotBase`).
        // GuiSlot.tick(0): not translated, its class isn't translated (it extends `GuiSlotBase`).
        // GuiSlot.onSendUpdate(0): not translated, its class isn't translated (it extends `GuiSlotBase`).
        // GuiSlot.changed(0): not translated, its class isn't translated (it extends `GuiSlotBase`).
        // GuiSlot.insert(1): not translated, its class isn't translated (it extends `GuiSlotBase`).
    }

    /// `GuiSlot.GuiSlotDist` (team/creative/creativecore/common/gui/control/inventory/GuiSlot.java)
    pub mod gui_slot_gui_slot_dist {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // GuiSlot.GuiSlotDist.changed(0): not translated, it has no body.
    }

    /// `GuiSlotBase` (team/creative/creativecore/common/gui/control/inventory/GuiSlotBase.java)
    pub mod gui_slot_base {
        use super::*;
        // The class itself isn't translated: it extends `GuiControl` (team.creative.creativecore.common.gui). Its static parts are below.
        // GuiSlotBase.GuiSlotBase(2): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiSlotBase.init(0): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiSlotBase.closed(0): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiSlotBase.tick(0): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiSlotBase.getStack(0): not translated, it has no body.
    }

    /// `GuiSlotViewer` (team/creative/creativecore/common/gui/control/inventory/GuiSlotViewer.java)
    pub mod gui_slot_viewer {
        use super::*;
        // The class itself isn't translated: it extends `GuiSlotBase`. Its static parts are below.
        // GuiSlotViewer.GuiSlotViewer(3): not translated, its class isn't translated (it extends `GuiSlotBase`).
        // GuiSlotViewer.GuiSlotViewer(2): not translated, its class isn't translated (it extends `GuiSlotBase`).
        // GuiSlotViewer.getStack(0): not translated, its class isn't translated (it extends `GuiSlotBase`).
    }

    /// `IGuiInventory` (team/creative/creativecore/common/gui/control/inventory/IGuiInventory.java)
    pub mod i_gui_inventory {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IGuiInventory.getSlot(1): not translated, it has no body.
        // IGuiInventory.inventorySize(0): not translated, it has no body.
        // IGuiInventory.name(0): not translated, it has no body.
        // IGuiInventory.setChanged(0): not translated, it has no body.
        // IGuiInventory.setChanged(1): not translated, it has no body.
        // IGuiInventory.moveInside(2): not translated, its class isn't translated (it's an interface).
        // IGuiInventory.insertClever(1): not translated, its class isn't translated (it's an interface).
        // IGuiInventory.insertClever(3): not translated, its class isn't translated (it's an interface).
        // IGuiInventory.insert(2): not translated, its class isn't translated (it's an interface).
        // IGuiInventory.insert(4): not translated, its class isn't translated (it's an interface).
        // IGuiInventory.extract(1): not translated, its class isn't translated (it's an interface).
        // IGuiInventory.sync(1): not translated, its class isn't translated (it's an interface).
        // IGuiInventory.syncAll(0): not translated, its class isn't translated (it's an interface).
    }

    /// `GuiMenu` (team/creative/creativecore/common/gui/control/menu/GuiMenu.java)
    pub mod gui_menu {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GuiMenu.GuiMenu(2): not translated, its class isn't translated (it's generic).
        // GuiMenu.populateEntryTree(1): not translated, its class isn't translated (it's generic).
        // GuiMenu.dist(0): not translated, its class isn't translated (it's generic).
        // GuiMenu.closed(0): not translated, its class isn't translated (it's generic).
        // GuiMenu.isRoot(0): not translated, its class isn't translated (it's generic).
        // GuiMenu.root(0): not translated, its class isn't translated (it's generic).
    }

    /// `GuiMenu.GuiMenuDist` (team/creative/creativecore/common/gui/control/menu/GuiMenu.java)
    pub mod gui_menu_gui_menu_dist {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // GuiMenu.GuiMenuDist.closed(0): not translated, it has no body.
        // GuiMenu.GuiMenuDist.init(1): not translated, it has no body.
        // GuiMenu.GuiMenuDist.isRoot(0): not translated, it has no body.
        // GuiMenu.GuiMenuDist.root(0): not translated, it has no body.
        // GuiMenu.GuiMenuDist.buildTree(0): not translated, it has no body.
        // GuiMenu.GuiMenuDist.populateEntryTree(1): not translated, it has no body.
    }

    /// `GuiMenu.GuiMenuEntry` (team/creative/creativecore/common/gui/control/menu/GuiMenu.java)
    pub mod gui_menu_gui_menu_entry {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GuiMenu.GuiMenuEntry.GuiMenuEntry(4): not translated, its class isn't translated (it's generic).
        // GuiMenu.GuiMenuEntry.dist(0): not translated, its class isn't translated (it's generic).
        // GuiMenu.GuiMenuEntry.setHighlighted(1): not translated, its class isn't translated (it's generic).
        // GuiMenu.GuiMenuEntry.close(0): not translated, its class isn't translated (it's generic).
        // GuiMenu.GuiMenuEntry.open(0): not translated, its class isn't translated (it's generic).
        // GuiMenu.GuiMenuEntry.menu(0): not translated, its class isn't translated (it's generic).
    }

    /// `GuiMenu.GuiMenuEntryDist` (team/creative/creativecore/common/gui/control/menu/GuiMenu.java)
    pub mod gui_menu_gui_menu_entry_dist {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // GuiMenu.GuiMenuEntryDist.setHighlighted(1): not translated, it has no body.
        // GuiMenu.GuiMenuEntryDist.close(0): not translated, it has no body.
        // GuiMenu.GuiMenuEntryDist.open(0): not translated, it has no body.
    }

    /// `GuiMenuRoot` (team/creative/creativecore/common/gui/control/menu/GuiMenuRoot.java)
    pub mod gui_menu_root {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GuiMenuRoot.GuiMenuRoot(4): not translated, its class isn't translated (it's generic).
        // GuiMenuRoot.dist(0): not translated, its class isn't translated (it's generic).
        // GuiMenuRoot.translate(2): not translated, its class isn't translated (it's generic).
        // GuiMenuRoot.select(2): not translated, its class isn't translated (it's generic).
        // GuiMenuRoot.createEntryTree(0): not translated, its class isn't translated (it's generic).
    }

    /// `GuiMenuRoot.GuiMenuRootDist` (team/creative/creativecore/common/gui/control/menu/GuiMenuRoot.java)
    pub mod gui_menu_root_gui_menu_root_dist {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // GuiMenuRoot.GuiMenuRootDist.set(2): not translated, it has no body.
        // GuiMenuRoot.GuiMenuRootDist.translate(2): not translated, it has no body.
        // GuiMenuRoot.GuiMenuRootDist.select(2): not translated, it has no body.
    }

    /// `GuiMenuSub` (team/creative/creativecore/common/gui/control/menu/GuiMenuSub.java)
    pub mod gui_menu_sub {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GuiMenuSub.GuiMenuSub(2): not translated, its class isn't translated (it's generic).
        // GuiMenuSub.dist(0): not translated, its class isn't translated (it's generic).
    }

    /// `GuiMenuSub.GuiMenuSubDist` (team/creative/creativecore/common/gui/control/menu/GuiMenuSub.java)
    pub mod gui_menu_sub_gui_menu_sub_dist {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // GuiMenuSub.GuiMenuSubDist.setRoot(1): not translated, it has no body.
    }

    /// `GuiColumn` (team/creative/creativecore/common/gui/control/parent/GuiColumn.java)
    pub mod gui_column {
        use super::*;
        // The class itself isn't translated: it extends `GuiParent` (team.creative.creativecore.common.gui). Its static parts are below.
        // GuiColumn.GuiColumn(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiColumn.GuiColumn(2): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiColumn.GuiColumn(2): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiColumn.GuiColumn(3): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
    }

    /// `GuiColumn.GuiColumnHeader` (team/creative/creativecore/common/gui/control/parent/GuiColumn.java)
    pub mod gui_column_gui_column_header {
        use super::*;
        // The class itself isn't translated: it extends `GuiColumn`. Its static parts are below.
        // GuiColumn.GuiColumnHeader.GuiColumnHeader(1): not translated, its class isn't translated (it extends `GuiColumn`).
        // GuiColumn.GuiColumnHeader.GuiColumnHeader(2): not translated, its class isn't translated (it extends `GuiColumn`).
        // GuiColumn.GuiColumnHeader.GuiColumnHeader(2): not translated, its class isn't translated (it extends `GuiColumn`).
        // GuiColumn.GuiColumnHeader.GuiColumnHeader(3): not translated, its class isn't translated (it extends `GuiColumn`).
    }

    /// `GuiLabeledControl` (team/creative/creativecore/common/gui/control/parent/GuiLabeledControl.java)
    pub mod gui_labeled_control {
        use super::*;
        // The class itself isn't translated: it extends `GuiParent` (team.creative.creativecore.common.gui). Its static parts are below.
        // GuiLabeledControl.GuiLabeledControl(3): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiLabeledControl.GuiLabeledControl(3): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiLabeledControl.GuiLabeledControl(2): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiLabeledControl.getInnerControl(0): not translated, it's generic.
        // GuiLabeledControl.setTranslate(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiLabeledControl.setTranslate(2): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiLabeledControl.setTitle(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiLabeledControl.setTitle(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiLabeledControl.setTooltip(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiLabeledControl.setTooltip(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
    }

    /// `GuiLeftRightBox` (team/creative/creativecore/common/gui/control/parent/GuiLeftRightBox.java)
    pub mod gui_left_right_box {
        use super::*;
        // The class itself isn't translated: it extends `GuiRow`. Its static parts are below.
        // GuiLeftRightBox.GuiLeftRightBox(1): not translated, its class isn't translated (it extends `GuiRow`).
        // GuiLeftRightBox.isExpandableX(0): not translated, its class isn't translated (it extends `GuiRow`).
        // GuiLeftRightBox.getLeft(0): not translated, its class isn't translated (it extends `GuiRow`).
        // GuiLeftRightBox.getRight(0): not translated, its class isn't translated (it extends `GuiRow`).
        // GuiLeftRightBox.addLeft(1): not translated, its class isn't translated (it extends `GuiRow`).
        // GuiLeftRightBox.addRight(1): not translated, its class isn't translated (it extends `GuiRow`).
        // GuiLeftRightBox.setVAlign(1): not translated, its class isn't translated (it extends `GuiRow`).
        // GuiLeftRightBox.addColumn(1): not translated, its class isn't translated (it extends `GuiRow`).
    }

    /// `GuiPanel` (team/creative/creativecore/common/gui/control/parent/GuiPanel.java)
    pub mod gui_panel {
        use super::*;
        // The class itself isn't translated: it extends `GuiParent` (team.creative.creativecore.common.gui). Its static parts are below.
        // GuiPanel.GuiPanel(3): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiPanel.GuiPanel(4): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiPanel.GuiPanel(5): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiPanel.GuiPanel(2): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiPanel.GuiPanel(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiPanel.GuiPanel(2): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
    }

    /// `GuiRow` (team/creative/creativecore/common/gui/control/parent/GuiRow.java)
    pub mod gui_row {
        use super::*;
        // The class itself isn't translated: it extends `GuiParent` (team.creative.creativecore.common.gui). Its static parts are below.
        // GuiRow.GuiRow(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiRow.GuiRow(2): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiRow.addColumn(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiRow.removeCol(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiRow.getCol(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiRow.colCount(0): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiRow.add(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiRow.addHover(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
    }

    /// `GuiScrollX` (team/creative/creativecore/common/gui/control/parent/GuiScrollX.java)
    pub mod gui_scroll_x {
        use super::*;
        // The class itself isn't translated: it extends `GuiParent` (team.creative.creativecore.common.gui). Its static parts are below.
        // GuiScrollX.GuiScrollX(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiScrollX.GuiScrollX(2): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiScrollX.dist(0): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiScrollX.setHovered(0): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiScrollX.setHover(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiScrollX.setDim(2): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiScrollX.setDim(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiScrollX.setExpandable(0): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
    }

    /// `GuiScrollX.GuiScrollXDist` (team/creative/creativecore/common/gui/control/parent/GuiScrollX.java)
    pub mod gui_scroll_x_gui_scroll_x_dist {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // GuiScrollX.GuiScrollXDist.setHovered(0): not translated, it has no body.
        // GuiScrollX.GuiScrollXDist.setHover(1): not translated, it has no body.
    }

    /// `GuiScrollXY` (team/creative/creativecore/common/gui/control/parent/GuiScrollXY.java)
    pub mod gui_scroll_xy {
        use super::*;
        // The class itself isn't translated: it extends `GuiParent` (team.creative.creativecore.common.gui). Its static parts are below.
        // GuiScrollXY.GuiScrollXY(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiScrollXY.GuiScrollXY(2): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiScrollXY.GuiScrollXY(3): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiScrollXY.dist(0): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
    }

    /// `GuiScrollXY.GuiScrollXYDist` (team/creative/creativecore/common/gui/control/parent/GuiScrollXY.java)
    pub mod gui_scroll_xy_gui_scroll_xy_dist {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
    }

    /// `GuiScrollY` (team/creative/creativecore/common/gui/control/parent/GuiScrollY.java)
    pub mod gui_scroll_y {
        use super::*;
        // The class itself isn't translated: it extends `GuiParent` (team.creative.creativecore.common.gui). Its static parts are below.
        // GuiScrollY.GuiScrollY(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiScrollY.GuiScrollY(2): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiScrollY.dist(0): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiScrollY.setHovered(0): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiScrollY.setHover(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiScrollY.setScrollWhenCTRL(0): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiScrollY.setFormatting(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiScrollY.setDim(2): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiScrollY.setDim(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiScrollY.setExpandable(0): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
    }

    /// `GuiScrollY.GuiScrollYDist` (team/creative/creativecore/common/gui/control/parent/GuiScrollY.java)
    pub mod gui_scroll_y_gui_scroll_y_dist {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // GuiScrollY.GuiScrollYDist.setHovered(0): not translated, it has no body.
        // GuiScrollY.GuiScrollYDist.setHover(1): not translated, it has no body.
        // GuiScrollY.GuiScrollYDist.setScrollWhenCTRL(0): not translated, it has no body.
    }

    /// `GuiTable` (team/creative/creativecore/common/gui/control/parent/GuiTable.java)
    pub mod gui_table {
        use super::*;
        // The class itself isn't translated: it extends `GuiParent` (team.creative.creativecore.common.gui). Its static parts are below.
        // GuiTable.GuiTable(2): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTable.GuiTable(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTable.GuiTable(2): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTable.addRow(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTable.setExpandable(0): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTable.add(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTable.addHover(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
    }

    /// `GuiTableScrollable` (team/creative/creativecore/common/gui/control/parent/GuiTableScrollable.java)
    pub mod gui_table_scrollable {
        use super::*;
        // The class itself isn't translated: it extends `GuiParent` (team.creative.creativecore.common.gui). Its static parts are below.
        // GuiTableScrollable.GuiTableScrollable(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTableScrollable.GuiTableScrollable(2): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTableScrollable.dist(0): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTableScrollable.add(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTableScrollable.addHover(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTableScrollable.setExpandable(0): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTableScrollable.getTopRow(0): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTableScrollable.addRow(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTableScrollable.isExpandableX(0): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTableScrollable.isExpandableY(0): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTableScrollable.removeContentCol(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTableScrollable.contentRows(0): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTableScrollable.createDist(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
    }

    /// `GuiTableScrollable.GuiTableScrollableDist` (team/creative/creativecore/common/gui/control/parent/GuiTableScrollable.java)
    pub mod gui_table_scrollable_gui_table_scrollable_dist {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // GuiTableScrollable.GuiTableScrollableDist.createScrollBox(1): not translated, it has no body.
        // GuiTableScrollable.GuiTableScrollableDist.createTopRow(1): not translated, it has no body.
        // GuiTableScrollable.GuiTableScrollableDist.createFirstCol(1): not translated, it has no body.
    }

    /// `GuiTabs` (team/creative/creativecore/common/gui/control/parent/GuiTabs.java)
    pub mod gui_tabs {
        use super::*;
        // The class itself isn't translated: it extends `GuiParent` (team.creative.creativecore.common.gui). Its static parts are below.
        // GuiTabs.GuiTabs(2): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTabs.dist(0): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTabs.createDist(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTabs.createTab(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTabs.getTabButton(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTabs.getTab(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTabs.select(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTabs.index(0): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
    }

    /// `GuiTabs.GuiTabBar` (team/creative/creativecore/common/gui/control/parent/GuiTabs.java)
    pub mod gui_tabs_gui_tab_bar {
        use super::*;
        // The class itself isn't translated: it extends `GuiParent` (team.creative.creativecore.common.gui). Its static parts are below.
        // GuiTabs.GuiTabBar.GuiTabBar(2): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTabs.GuiTabBar.highlight(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTabs.GuiTabBar.addTab(2): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTabs.GuiTabBar.getTab(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTabs.GuiTabBar.removeTab(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
    }

    /// `GuiTabs.GuiTabSpecialParent` (team/creative/creativecore/common/gui/control/parent/GuiTabs.java)
    pub mod gui_tabs_gui_tab_special_parent {
        use super::*;
        // The class itself isn't translated: it extends `GuiParent` (team.creative.creativecore.common.gui). Its static parts are below.
        // GuiTabs.GuiTabSpecialParent.GuiTabSpecialParent(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
    }

    /// `GuiTabs.GuiTabsDist` (team/creative/creativecore/common/gui/control/parent/GuiTabs.java)
    pub mod gui_tabs_gui_tabs_dist {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // GuiTabs.GuiTabsDist.init(2): not translated, it has no body.
        // GuiTabs.GuiTabsDist.index(0): not translated, it has no body.
        // GuiTabs.GuiTabsDist.select(2): not translated, it has no body.
        // GuiTabs.GuiTabsDist.createSpecialParent(1): not translated, it has no body.
    }

    /// `GuiTabsMapped` (team/creative/creativecore/common/gui/control/parent/GuiTabsMapped.java)
    pub mod gui_tabs_mapped {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GuiTabsMapped.GuiTabsMapped(2): not translated, its class isn't translated (it's generic).
        // GuiTabsMapped.createTab(1): not translated, its class isn't translated (it's generic).
        // GuiTabsMapped.createTab(2): not translated, its class isn't translated (it's generic).
        // GuiTabsMapped.getSelected(0): not translated, its class isn't translated (it's generic).
        // GuiTabsMapped.getSelected(1): not translated, its class isn't translated (it's generic).
        // GuiTabsMapped.select(1): not translated, its class isn't translated (it's generic).
    }

    /// `GuiTopBottomBox` (team/creative/creativecore/common/gui/control/parent/GuiTopBottomBox.java)
    pub mod gui_top_bottom_box {
        use super::*;
        // The class itself isn't translated: it extends `GuiParent` (team.creative.creativecore.common.gui). Its static parts are below.
        // GuiTopBottomBox.GuiTopBottomBox(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTopBottomBox.addTop(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTopBottomBox.addBottom(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTopBottomBox.add(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTopBottomBox.addHover(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
    }

    /// `GuiArraySlider` (team/creative/creativecore/common/gui/control/simple/GuiArraySlider.java)
    pub mod gui_array_slider {
        use super::*;
        // The class itself isn't translated: it extends `GuiSteppedSlider`. Its static parts are below.
        // GuiArraySlider.GuiArraySlider(4): not translated, its class isn't translated (it extends `GuiSteppedSlider`).
        // GuiArraySlider.GuiArraySlider(2): not translated, its class isn't translated (it extends `GuiSteppedSlider`).
        // GuiArraySlider.dist(0): not translated, its class isn't translated (it extends `GuiSteppedSlider`).
        // GuiArraySlider.select(1): not translated, its class isn't translated (it extends `GuiSteppedSlider`).
        // GuiArraySlider.setValues(1): not translated, its class isn't translated (it extends `GuiSteppedSlider`).
        // GuiArraySlider.get(0): not translated, its class isn't translated (it extends `GuiSteppedSlider`).
    }

    /// `GuiArraySlider.GuiArraySliderDist` (team/creative/creativecore/common/gui/control/simple/GuiArraySlider.java)
    pub mod gui_array_slider_gui_array_slider_dist {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // GuiArraySlider.GuiArraySliderDist.select(2): not translated, it has no body.
        // GuiArraySlider.GuiArraySliderDist.setValues(2): not translated, it has no body.
        // GuiArraySlider.GuiArraySliderDist.get(0): not translated, it has no body.
    }

    /// `GuiButton` (team/creative/creativecore/common/gui/control/simple/GuiButton.java)
    pub mod gui_button {
        use super::*;
        // The class itself isn't translated: it extends `GuiLabel`. Its static parts are below.
        // GuiButton.GuiButton(3): not translated, its class isn't translated (it extends `GuiLabel`).
        // GuiButton.dist(0): not translated, its class isn't translated (it extends `GuiLabel`).
        // GuiButton.setPressed(1): not translated, its class isn't translated (it extends `GuiLabel`).
        // GuiButton.setHoverEffect(1): not translated, its class isn't translated (it extends `GuiLabel`).
    }

    /// `GuiButton.GuiButtonDist` (team/creative/creativecore/common/gui/control/simple/GuiButton.java)
    pub mod gui_button_gui_button_dist {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // GuiButton.GuiButtonDist.setPressed(1): not translated, it has no body.
        // GuiButton.GuiButtonDist.setHoverEffect(1): not translated, it has no body.
    }

    /// `GuiButtonContext` (team/creative/creativecore/common/gui/control/simple/GuiButtonContext.java)
    pub mod gui_button_context {
        use super::*;
        // The class itself isn't translated: it extends `GuiButton`. Its static parts are below.
        // GuiButtonContext.GuiButtonContext(3): not translated, its class isn't translated (it extends `GuiButton`).
        // GuiButtonContext.dist(0): not translated, its class isn't translated (it extends `GuiButton`).
        // GuiButtonContext.set(1): not translated, its class isn't translated (it extends `GuiButton`).
    }

    /// `GuiButtonContext.GuiButtonContextDist` (team/creative/creativecore/common/gui/control/simple/GuiButtonContext.java)
    pub mod gui_button_context_gui_button_context_dist {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // GuiButtonContext.GuiButtonContextDist.set(2): not translated, it has no body.
    }

    /// `GuiButtonHold` (team/creative/creativecore/common/gui/control/simple/GuiButtonHold.java)
    pub mod gui_button_hold {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GuiButtonHold.GuiButtonHold(3): not translated, its class isn't translated (it's generic).
        // GuiButtonHold.dist(0): not translated, its class isn't translated (it's generic).
        // GuiButtonHold.tick(0): not translated, its class isn't translated (it's generic).
    }

    /// `GuiButtonHold.GuiButtonHoldDist` (team/creative/creativecore/common/gui/control/simple/GuiButtonHold.java)
    pub mod gui_button_hold_gui_button_hold_dist {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // GuiButtonHold.GuiButtonHoldDist.tick(0): not translated, it has no body.
    }

    /// `GuiButtonIcon` (team/creative/creativecore/common/gui/control/simple/GuiButtonIcon.java)
    pub mod gui_button_icon {
        use super::*;
        // The class itself isn't translated: it extends `GuiIcon`. Its static parts are below.
        // GuiButtonIcon.GuiButtonIcon(4): not translated, its class isn't translated (it extends `GuiIcon`).
        // GuiButtonIcon.dist(0): not translated, its class isn't translated (it extends `GuiIcon`).
        // GuiButtonIcon.setIcon(1): not translated, its class isn't translated (it extends `GuiIcon`).
        // GuiButtonIcon.setColor(1): not translated, its class isn't translated (it extends `GuiIcon`).
        // GuiButtonIcon.setShadow(1): not translated, its class isn't translated (it extends `GuiIcon`).
        // GuiButtonIcon.setSquared(1): not translated, its class isn't translated (it extends `GuiIcon`).
    }

    /// `GuiButtonIcon.GuiButtonIconDist` (team/creative/creativecore/common/gui/control/simple/GuiButtonIcon.java)
    pub mod gui_button_icon_gui_button_icon_dist {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // GuiButtonIcon.GuiButtonIconDist.setPressed(1): not translated, it has no body.
    }

    /// `GuiCheckBox` (team/creative/creativecore/common/gui/control/simple/GuiCheckBox.java)
    pub mod gui_check_box {
        use super::*;
        // The class itself isn't translated: it extends `GuiLabel`. Its static parts are below.
        // GuiCheckBox.GuiCheckBox(3): not translated, its class isn't translated (it extends `GuiLabel`).
        // GuiCheckBox.dist(0): not translated, its class isn't translated (it extends `GuiLabel`).
        // GuiCheckBox.set(1): not translated, its class isn't translated (it extends `GuiLabel`).
        // GuiCheckBox.get(0): not translated, its class isn't translated (it extends `GuiLabel`).
        // GuiCheckBox.setPartial(1): not translated, its class isn't translated (it extends `GuiLabel`).
        // GuiCheckBox.getPartial(0): not translated, its class isn't translated (it extends `GuiLabel`).
        // GuiCheckBox.consumeChanged(1): not translated, its class isn't translated (it extends `GuiLabel`).
        // GuiCheckBox.setTranslate(1): not translated, its class isn't translated (it extends `GuiLabel`).
        // GuiCheckBox.setTitle(1): not translated, its class isn't translated (it extends `GuiLabel`).
        // GuiCheckBox.setTitle(1): not translated, its class isn't translated (it extends `GuiLabel`).
    }

    /// `GuiCheckBox.GuiCheckBoxDist` (team/creative/creativecore/common/gui/control/simple/GuiCheckBox.java)
    pub mod gui_check_box_gui_check_box_dist {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // GuiCheckBox.GuiCheckBoxDist.consumeChanged(1): not translated, it has no body.
        // GuiCheckBox.GuiCheckBoxDist.set(2): not translated, it has no body.
        // GuiCheckBox.GuiCheckBoxDist.get(0): not translated, it has no body.
        // GuiCheckBox.GuiCheckBoxDist.setPartial(2): not translated, it has no body.
        // GuiCheckBox.GuiCheckBoxDist.getPartial(0): not translated, it has no body.
    }

    /// `GuiCheckButtonIcon` (team/creative/creativecore/common/gui/control/simple/GuiCheckButtonIcon.java)
    pub mod gui_check_button_icon {
        use super::*;
        // The class itself isn't translated: it extends `GuiButtonIcon`. Its static parts are below.
        // GuiCheckButtonIcon.GuiCheckButtonIcon(5): not translated, its class isn't translated (it extends `GuiButtonIcon`).
        // GuiCheckButtonIcon.GuiCheckButtonIcon(6): not translated, its class isn't translated (it extends `GuiButtonIcon`).
        // GuiCheckButtonIcon.dist(0): not translated, its class isn't translated (it extends `GuiButtonIcon`).
        // GuiCheckButtonIcon.setOnIcon(1): not translated, its class isn't translated (it extends `GuiButtonIcon`).
        // GuiCheckButtonIcon.setOffIcon(1): not translated, its class isn't translated (it extends `GuiButtonIcon`).
        // GuiCheckButtonIcon.setColor(1): not translated, its class isn't translated (it extends `GuiButtonIcon`).
        // GuiCheckButtonIcon.setShadow(1): not translated, its class isn't translated (it extends `GuiButtonIcon`).
        // GuiCheckButtonIcon.setSquared(1): not translated, its class isn't translated (it extends `GuiButtonIcon`).
        // GuiCheckButtonIcon.setDim(1): not translated, its class isn't translated (it extends `GuiButtonIcon`).
        // GuiCheckButtonIcon.setDim(2): not translated, its class isn't translated (it extends `GuiButtonIcon`).
        // GuiCheckButtonIcon.getState(0): not translated, its class isn't translated (it extends `GuiButtonIcon`).
        // GuiCheckButtonIcon.setState(1): not translated, its class isn't translated (it extends `GuiButtonIcon`).
    }

    /// `GuiCheckButtonIcon.GuiCheckButtonIconDist` (team/creative/creativecore/common/gui/control/simple/GuiCheckButtonIcon.java)
    pub mod gui_check_button_icon_gui_check_button_icon_dist {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // GuiCheckButtonIcon.GuiCheckButtonIconDist.setOnIcon(1): not translated, it has no body.
        // GuiCheckButtonIcon.GuiCheckButtonIconDist.setOffIcon(1): not translated, it has no body.
        // GuiCheckButtonIcon.GuiCheckButtonIconDist.getState(0): not translated, it has no body.
        // GuiCheckButtonIcon.GuiCheckButtonIconDist.setState(2): not translated, it has no body.
    }

    /// `GuiColoredSteppedSlider` (team/creative/creativecore/common/gui/control/simple/GuiColoredSteppedSlider.java)
    pub mod gui_colored_stepped_slider {
        use super::*;
        // The class itself isn't translated: it extends `GuiSteppedSlider`. Its static parts are below.
        // GuiColoredSteppedSlider.GuiColoredSteppedSlider(4): not translated, its class isn't translated (it extends `GuiSteppedSlider`).
        // GuiColoredSteppedSlider.setValueSilent(1): not translated, its class isn't translated (it extends `GuiSteppedSlider`).
        // GuiColoredSteppedSlider.dist(0): not translated, its class isn't translated (it extends `GuiSteppedSlider`).
    }

    /// `GuiColoredSteppedSlider.GuiColoredSteppedSliderDist` (team/creative/creativecore/common/gui/control/simple/GuiColoredSteppedSlider.java)
    pub mod gui_colored_stepped_slider_gui_colored_stepped_slider_dist {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // GuiColoredSteppedSlider.GuiColoredSteppedSliderDist.setPicker(1): not translated, it has no body.
        // GuiColoredSteppedSlider.GuiColoredSteppedSliderDist.setPart(1): not translated, it has no body.
        // GuiColoredSteppedSlider.GuiColoredSteppedSliderDist.setValueSilent(1): not translated, it has no body.
    }

    /// `GuiColorPicker` (team/creative/creativecore/common/gui/control/simple/GuiColorPicker.java)
    pub mod gui_color_picker {
        use super::*;
        // The class itself isn't translated: it extends `GuiParent` (team.creative.creativecore.common.gui). Its static parts are below.
        // GuiColorPicker.GuiColorPicker(5): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiColorPicker.setColor(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiColorPicker.onColorChanged(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
    }

    /// `GuiColorPicker.GuiHexButton` (team/creative/creativecore/common/gui/control/simple/GuiColorPicker.java)
    pub mod gui_color_picker_gui_hex_button {
        use super::*;
        // The class itself isn't translated: it extends `GuiButton`. Its static parts are below.
        // GuiColorPicker.GuiHexButton.GuiHexButton(4): not translated, its class isn't translated (it extends `GuiButton`).
        // GuiColorPicker.GuiHexButton.dist(0): not translated, its class isn't translated (it extends `GuiButton`).
        // GuiColorPicker.GuiHexButton.colorChanged(0): not translated, its class isn't translated (it extends `GuiButton`).
    }

    /// `GuiColorPicker.GuiHexButtonDist` (team/creative/creativecore/common/gui/control/simple/GuiColorPicker.java)
    pub mod gui_color_picker_gui_hex_button_dist {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // GuiColorPicker.GuiHexButtonDist.setConsumer(2): not translated, it has no body.
        // GuiColorPicker.GuiHexButtonDist.colorChanged(0): not translated, it has no body.
    }

    /// `GuiColorPlate` (team/creative/creativecore/common/gui/control/simple/GuiColorPlate.java)
    pub mod gui_color_plate {
        use super::*;
        // The class itself isn't translated: it extends `GuiControl` (team.creative.creativecore.common.gui). Its static parts are below.
        // GuiColorPlate.GuiColorPlate(3): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiColorPlate.dist(0): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiColorPlate.setColor(1): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiColorPlate.getColor(0): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiColorPlate.init(0): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiColorPlate.closed(0): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiColorPlate.tick(0): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
    }

    /// `GuiColorPlate.GuiColorPlateDist` (team/creative/creativecore/common/gui/control/simple/GuiColorPlate.java)
    pub mod gui_color_plate_gui_color_plate_dist {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // GuiColorPlate.GuiColorPlateDist.setColor(2): not translated, it has no body.
        // GuiColorPlate.GuiColorPlateDist.getColor(0): not translated, it has no body.
    }

    /// `GuiCounter` (team/creative/creativecore/common/gui/control/simple/GuiCounter.java)
    pub mod gui_counter {
        use super::*;
        // The class itself isn't translated: it extends `GuiParent` (team.creative.creativecore.common.gui). Its static parts are below.
        // GuiCounter.GuiCounter(3): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiCounter.GuiCounter(5): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiCounter.GuiCounter(6): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiCounter.createButtons(0): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiCounter.setSpacing(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiCounter.add(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiCounter.isExpandableX(0): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiCounter.getPlusButton(0): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiCounter.getMinusButton(0): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiCounter.resetTextfield(0): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiCounter.raiseEvent(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiCounter.stepUp(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiCounter.stepDown(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiCounter.getValue(0): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiCounter.setValue(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
    }

    /// `GuiCounterDecimal` (team/creative/creativecore/common/gui/control/simple/GuiCounterDecimal.java)
    pub mod gui_counter_decimal {
        use super::*;
        // The class itself isn't translated: it extends `GuiParent` (team.creative.creativecore.common.gui). Its static parts are below.
        // FORMAT: not translated, it uses `DecimalFormat` (java.text).
        // GuiCounterDecimal.GuiCounterDecimal(3): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiCounterDecimal.GuiCounterDecimal(5): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiCounterDecimal.GuiCounterDecimal(6): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiCounterDecimal.createButtons(0): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiCounterDecimal.setSpacing(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiCounterDecimal.add(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiCounterDecimal.isExpandableX(0): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiCounterDecimal.getPlusButton(0): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiCounterDecimal.getMinusButton(0): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiCounterDecimal.setStep(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiCounterDecimal.getFormat(0): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiCounterDecimal.resetTextfield(0): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiCounterDecimal.raiseEvent(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiCounterDecimal.stepUp(0): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiCounterDecimal.stepDown(0): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiCounterDecimal.getValue(0): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiCounterDecimal.setValue(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
    }

    /// `GuiDuration` (team/creative/creativecore/common/gui/control/simple/GuiDuration.java)
    pub mod gui_duration {
        use super::*;
        // The class itself isn't translated: it extends `GuiParent` (team.creative.creativecore.common.gui). Its static parts are below.
        pub const TICKS_PER_SECOND: i32 = 20;
        pub const TICKS_PER_MINUTE: i32 = 1200;
        pub const TICKS_PER_HOUR: i32 = 72000;
        pub const TICKS_PER_DAY: i32 = 1728000;
        // GuiDuration.GuiDuration(7): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiDuration.setDuration(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiDuration.getDuration(0): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
    }

    /// `GuiIcon` (team/creative/creativecore/common/gui/control/simple/GuiIcon.java)
    pub mod gui_icon {
        use super::*;
        // The class itself isn't translated: it extends `GuiControl` (team.creative.creativecore.common.gui). Its static parts are below.
        // GuiIcon.GuiIcon(3): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiIcon.dist(0): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiIcon.setIcon(1): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiIcon.setColor(1): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiIcon.setShadow(1): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiIcon.setSquared(1): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiIcon.setDim(2): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiIcon.setDim(1): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiIcon.init(0): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiIcon.closed(0): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiIcon.tick(0): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
    }

    /// `GuiIcon.GuiIconDist` (team/creative/creativecore/common/gui/control/simple/GuiIcon.java)
    pub mod gui_icon_gui_icon_dist {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // GuiIcon.GuiIconDist.setIcon(1): not translated, it has no body.
        // GuiIcon.GuiIconDist.setColor(1): not translated, it has no body.
        // GuiIcon.GuiIconDist.setShadow(1): not translated, it has no body.
        // GuiIcon.GuiIconDist.setSquared(1): not translated, it has no body.
    }

    /// `GuiLabel` (team/creative/creativecore/common/gui/control/simple/GuiLabel.java)
    pub mod gui_label {
        use super::*;
        // The class itself isn't translated: it extends `GuiControl` (team.creative.creativecore.common.gui). Its static parts are below.
        // GuiLabel.GuiLabel(2): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiLabel.dist(0): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiLabel.setDefaultColor(1): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiLabel.setDropShadow(1): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiLabel.setAlign(1): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiLabel.setVAlign(1): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiLabel.setTranslate(1): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiLabel.setTranslate(2): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiLabel.setTitle(1): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiLabel.setTitle(1): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiLabel.setScale(1): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiLabel.init(0): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiLabel.closed(0): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiLabel.tick(0): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
    }

    /// `GuiLabel.GuiLabelDist` (team/creative/creativecore/common/gui/control/simple/GuiLabel.java)
    pub mod gui_label_gui_label_dist {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // GuiLabel.GuiLabelDist.setDefaultColor(1): not translated, it has no body.
        // GuiLabel.GuiLabelDist.setDropShadow(1): not translated, it has no body.
        // GuiLabel.GuiLabelDist.setAlign(1): not translated, it has no body.
        // GuiLabel.GuiLabelDist.setVAlign(1): not translated, it has no body.
        // GuiLabel.GuiLabelDist.setTitle(1): not translated, it has no body.
        // GuiLabel.GuiLabelDist.setTitle(1): not translated, it has no body.
        // GuiLabel.GuiLabelDist.setScale(1): not translated, it has no body.
    }

    /// `GuiLabelHover` (team/creative/creativecore/common/gui/control/simple/GuiLabelHover.java)
    pub mod gui_label_hover {
        use super::*;
        // The class itself isn't translated: it extends `GuiLabel`. Its static parts are below.
        // GuiLabelHover.GuiLabelHover(2): not translated, its class isn't translated (it extends `GuiLabel`).
    }

    /// `GuiListEntry` (team/creative/creativecore/common/gui/control/simple/GuiListEntry.java)
    pub mod gui_list_entry {
        use super::*;
        // The class itself isn't translated: it extends `GuiLabel`. Its static parts are below.
        // GuiListEntry.GuiListEntry(5): not translated, its class isn't translated (it extends `GuiLabel`).
    }

    /// `GuiProgressbar` (team/creative/creativecore/common/gui/control/simple/GuiProgressbar.java)
    pub mod gui_progressbar {
        use super::*;
        // The class itself isn't translated: it extends `GuiControl` (team.creative.creativecore.common.gui). Its static parts are below.
        // GuiProgressbar.GuiProgressbar(4): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiProgressbar.GuiProgressbar(5): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiProgressbar.init(0): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiProgressbar.closed(0): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiProgressbar.tick(0): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiProgressbar.getPercentage(0): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
    }

    /// `GuiSeekBar` (team/creative/creativecore/common/gui/control/simple/GuiSeekBar.java)
    pub mod gui_seek_bar {
        use super::*;
        // The class itself isn't translated: it extends `GuiControl` (team.creative.creativecore.common.gui). Its static parts are below.
        // GuiSeekBar.GuiSeekBar(4): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiSeekBar.GuiSeekBar(5): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiSeekBar.dist(0): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiSeekBar.setOnTimeUpdate(1): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiSeekBar.setOnLastTimeUpdate(1): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiSeekBar.setPosition(1): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiSeekBar.setTooltip(1): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiSeekBar.init(0): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiSeekBar.closed(0): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiSeekBar.tick(0): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
    }

    /// `GuiSeekBar.GuiSeekBarDist` (team/creative/creativecore/common/gui/control/simple/GuiSeekBar.java)
    pub mod gui_seek_bar_gui_seek_bar_dist {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // GuiSeekBar.GuiSeekBarDist.init(3): not translated, it has no body.
        // GuiSeekBar.GuiSeekBarDist.setOnTimeUpdate(1): not translated, it has no body.
        // GuiSeekBar.GuiSeekBarDist.setOnLastTimeUpdate(1): not translated, it has no body.
        // GuiSeekBar.GuiSeekBarDist.setPosition(2): not translated, it has no body.
        // GuiSeekBar.GuiSeekBarDist.tick(0): not translated, it has no body.
    }

    /// `GuiShowItem` (team/creative/creativecore/common/gui/control/simple/GuiShowItem.java)
    pub mod gui_show_item {
        use super::*;
        // The class itself isn't translated: it extends `GuiControl` (team.creative.creativecore.common.gui). Its static parts are below.
        // GuiShowItem.GuiShowItem(2): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiShowItem.GuiShowItem(3): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiShowItem.init(0): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiShowItem.closed(0): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiShowItem.tick(0): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
    }

    /// `GuiSlider` (team/creative/creativecore/common/gui/control/simple/GuiSlider.java)
    pub mod gui_slider {
        use super::*;
        // The class itself isn't translated: it extends `GuiControl` (team.creative.creativecore.common.gui). Its static parts are below.
        // GuiSlider.GuiSlider(5): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiSlider.GuiSlider(6): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiSlider.dist(0): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiSlider.getTextByValue(0): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiSlider.getTextfieldValue(0): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiSlider.getPercentage(0): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiSlider.closeTextField(0): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiSlider.setMaxValue(1): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiSlider.setMinValue(1): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiSlider.setValue(1): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiSlider.getValue(0): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiSlider.getMinValue(0): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiSlider.getMaxValue(0): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiSlider.setSliderSize(1): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiSlider.setMinSlider(1): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiSlider.setMaxSlider(1): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiSlider.isContainer(0): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiSlider.closeTopLayer(0): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiSlider.toLayerRect(2): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiSlider.toScreenRect(2): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiSlider.init(0): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiSlider.closed(0): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiSlider.tick(0): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiSlider.closeLayer(1): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiSlider.createDist(1): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiSlider.createDist(1): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
    }

    /// `GuiSlider.GuiSliderDist` (team/creative/creativecore/common/gui/control/simple/GuiSlider.java)
    pub mod gui_slider_gui_slider_dist {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // GuiSlider.GuiSliderDist.setParser(1): not translated, it has no body.
        // GuiSlider.GuiSliderDist.getTextByValue(0): not translated, it has no body.
        // GuiSlider.GuiSliderDist.getTextfieldValue(0): not translated, it has no body.
        // GuiSlider.GuiSliderDist.getPercentage(0): not translated, it has no body.
        // GuiSlider.GuiSliderDist.closeTextField(0): not translated, it has no body.
        // GuiSlider.GuiSliderDist.setBounds(3): not translated, it has no body.
        // GuiSlider.GuiSliderDist.setMaxValue(1): not translated, it has no body.
        // GuiSlider.GuiSliderDist.setMinValue(1): not translated, it has no body.
        // GuiSlider.GuiSliderDist.setValue(2): not translated, it has no body.
        // GuiSlider.GuiSliderDist.getValue(0): not translated, it has no body.
        // GuiSlider.GuiSliderDist.getMinValue(0): not translated, it has no body.
        // GuiSlider.GuiSliderDist.getMaxValue(0): not translated, it has no body.
        // GuiSlider.GuiSliderDist.setSliderSize(1): not translated, it has no body.
        // GuiSlider.GuiSliderDist.setMinSlider(1): not translated, it has no body.
        // GuiSlider.GuiSliderDist.setMaxSlider(1): not translated, it has no body.
    }

    /// `GuiStateButton` (team/creative/creativecore/common/gui/control/simple/GuiStateButton.java)
    pub mod gui_state_button {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GuiStateButton.GuiStateButton(3): not translated, its class isn't translated (it's generic).
        // GuiStateButton.GuiStateButton(4): not translated, its class isn't translated (it's generic).
        // GuiStateButton.GuiStateButton(4): not translated, its class isn't translated (it's generic).
        // GuiStateButton.dist(0): not translated, its class isn't translated (it's generic).
        // GuiStateButton.set(1): not translated, its class isn't translated (it's generic).
        // GuiStateButton.selected(0): not translated, its class isn't translated (it's generic).
        // GuiStateButton.selected(1): not translated, its class isn't translated (it's generic).
        // GuiStateButton.select(1): not translated, its class isn't translated (it's generic).
        // GuiStateButton.select(1): not translated, its class isn't translated (it's generic).
        // GuiStateButton.indexOf(1): not translated, its class isn't translated (it's generic).
        // GuiStateButton.next(0): not translated, its class isn't translated (it's generic).
        // GuiStateButton.previous(0): not translated, its class isn't translated (it's generic).
    }

    /// `GuiStateButton.GuiStateButtonDist` (team/creative/creativecore/common/gui/control/simple/GuiStateButton.java)
    pub mod gui_state_button_gui_state_button_dist {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // GuiStateButton.GuiStateButtonDist.set(2): not translated, it has no body.
        // GuiStateButton.GuiStateButtonDist.selected(0): not translated, it has no body.
        // GuiStateButton.GuiStateButtonDist.selected(1): not translated, it has no body.
        // GuiStateButton.GuiStateButtonDist.select(2): not translated, it has no body.
        // GuiStateButton.GuiStateButtonDist.select(2): not translated, it has no body.
        // GuiStateButton.GuiStateButtonDist.indexOf(1): not translated, it has no body.
        // GuiStateButton.GuiStateButtonDist.next(0): not translated, it has no body.
        // GuiStateButton.GuiStateButtonDist.previous(0): not translated, it has no body.
    }

    /// `GuiStateButtonIcon` (team/creative/creativecore/common/gui/control/simple/GuiStateButtonIcon.java)
    pub mod gui_state_button_icon {
        use super::*;
        // The class itself isn't translated: it extends `GuiButtonIcon`. Its static parts are below.
        // GuiStateButtonIcon.GuiStateButtonIcon(3): not translated, its class isn't translated (it extends `GuiButtonIcon`).
        // GuiStateButtonIcon.setState(1): not translated, its class isn't translated (it extends `GuiButtonIcon`).
        // GuiStateButtonIcon.dist(0): not translated, its class isn't translated (it extends `GuiButtonIcon`).
        // GuiStateButtonIcon.getState(0): not translated, its class isn't translated (it extends `GuiButtonIcon`).
        // GuiStateButtonIcon.setShadow(1): not translated, its class isn't translated (it extends `GuiButtonIcon`).
        // GuiStateButtonIcon.setColor(1): not translated, its class isn't translated (it extends `GuiButtonIcon`).
        // GuiStateButtonIcon.previousState(0): not translated, its class isn't translated (it extends `GuiButtonIcon`).
        // GuiStateButtonIcon.nextState(0): not translated, its class isn't translated (it extends `GuiButtonIcon`).
    }

    /// `GuiStateButtonIcon.GuiStateButtonIconDist` (team/creative/creativecore/common/gui/control/simple/GuiStateButtonIcon.java)
    pub mod gui_state_button_icon_gui_state_button_icon_dist {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // GuiStateButtonIcon.GuiStateButtonIconDist.getState(0): not translated, it has no body.
        // GuiStateButtonIcon.GuiStateButtonIconDist.setState(2): not translated, it has no body.
        // GuiStateButtonIcon.GuiStateButtonIconDist.setIcons(2): not translated, it has no body.
        // GuiStateButtonIcon.GuiStateButtonIconDist.previousState(0): not translated, it has no body.
        // GuiStateButtonIcon.GuiStateButtonIconDist.nextState(0): not translated, it has no body.
    }

    /// `GuiSteppedSlider` (team/creative/creativecore/common/gui/control/simple/GuiSteppedSlider.java)
    pub mod gui_stepped_slider {
        use super::*;
        // The class itself isn't translated: it extends `GuiSlider`. Its static parts are below.
        // GuiSteppedSlider.GuiSteppedSlider(5): not translated, its class isn't translated (it extends `GuiSlider`).
        // GuiSteppedSlider.GuiSteppedSlider(6): not translated, its class isn't translated (it extends `GuiSlider`).
        // GuiSteppedSlider.dist(0): not translated, its class isn't translated (it extends `GuiSlider`).
        // GuiSteppedSlider.stepUp(0): not translated, its class isn't translated (it extends `GuiSlider`).
        // GuiSteppedSlider.stepDown(0): not translated, its class isn't translated (it extends `GuiSlider`).
        // GuiSteppedSlider.setValue(1): not translated, its class isn't translated (it extends `GuiSlider`).
        // GuiSteppedSlider.setMaxValue(1): not translated, its class isn't translated (it extends `GuiSlider`).
        // GuiSteppedSlider.setMinValue(1): not translated, its class isn't translated (it extends `GuiSlider`).
        // GuiSteppedSlider.getValue(0): not translated, its class isn't translated (it extends `GuiSlider`).
        // GuiSteppedSlider.getMinValue(0): not translated, its class isn't translated (it extends `GuiSlider`).
        // GuiSteppedSlider.getMaxValue(0): not translated, its class isn't translated (it extends `GuiSlider`).
        // GuiSteppedSlider.getIntValue(0): not translated, its class isn't translated (it extends `GuiSlider`).
        // GuiSteppedSlider.getIntMaxValue(0): not translated, its class isn't translated (it extends `GuiSlider`).
        // GuiSteppedSlider.getIntMinValue(0): not translated, its class isn't translated (it extends `GuiSlider`).
    }

    /// `GuiSteppedSlider.GuiSteppedSliderDist` (team/creative/creativecore/common/gui/control/simple/GuiSteppedSlider.java)
    pub mod gui_stepped_slider_gui_stepped_slider_dist {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // GuiSteppedSlider.GuiSteppedSliderDist.setSteppedParser(1): not translated, it has no body.
        // GuiSteppedSlider.GuiSteppedSliderDist.getIntValue(0): not translated, it has no body.
        // GuiSteppedSlider.GuiSteppedSliderDist.getIntMaxValue(0): not translated, it has no body.
        // GuiSteppedSlider.GuiSteppedSliderDist.getIntMinValue(0): not translated, it has no body.
    }

    /// `GuiTabButton` (team/creative/creativecore/common/gui/control/simple/GuiTabButton.java)
    pub mod gui_tab_button {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // BUTTON_ACTIVE: not translated, it uses ControlFormatting, which isn't translated (it's an interface).
        // BUTTON_INACTIVE: not translated, it uses ControlFormatting, which isn't translated (it's an interface).
        // GuiTabButton.GuiTabButton(3): not translated, its class isn't translated (it's generic).
        // GuiTabButton.GuiTabButton(4): not translated, its class isn't translated (it's generic).
        // GuiTabButton.dist(0): not translated, its class isn't translated (it's generic).
        // GuiTabButton.set(1): not translated, its class isn't translated (it's generic).
        // GuiTabButton.selected(0): not translated, its class isn't translated (it's generic).
        // GuiTabButton.selected(1): not translated, its class isn't translated (it's generic).
        // GuiTabButton.select(1): not translated, its class isn't translated (it's generic).
        // GuiTabButton.select(1): not translated, its class isn't translated (it's generic).
        // GuiTabButton.indexOf(1): not translated, its class isn't translated (it's generic).
        // GuiTabButton.next(0): not translated, its class isn't translated (it's generic).
        // GuiTabButton.previous(0): not translated, its class isn't translated (it's generic).
        // GuiTabButton.index(0): not translated, its class isn't translated (it's generic).
    }

    /// `GuiTabButton.GuiTabButtonDist` (team/creative/creativecore/common/gui/control/simple/GuiTabButton.java)
    pub mod gui_tab_button_gui_tab_button_dist {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // GuiTabButton.GuiTabButtonDist.set(2): not translated, it has no body.
        // GuiTabButton.GuiTabButtonDist.selected(0): not translated, it has no body.
        // GuiTabButton.GuiTabButtonDist.selected(1): not translated, it has no body.
        // GuiTabButton.GuiTabButtonDist.select(2): not translated, it has no body.
        // GuiTabButton.GuiTabButtonDist.select(2): not translated, it has no body.
        // GuiTabButton.GuiTabButtonDist.indexOf(1): not translated, it has no body.
        // GuiTabButton.GuiTabButtonDist.next(0): not translated, it has no body.
        // GuiTabButton.GuiTabButtonDist.previous(0): not translated, it has no body.
        // GuiTabButton.GuiTabButtonDist.index(0): not translated, it has no body.
    }

    /// `GuiTextfield` (team/creative/creativecore/common/gui/control/simple/GuiTextfield.java)
    pub mod gui_textfield {
        use super::*;
        // The class itself isn't translated: it extends `GuiControl` (team.creative.creativecore.common.gui). Its static parts are below.
        // GuiTextfield.GuiTextfield(2): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiTextfield.GuiTextfield(3): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiTextfield.GuiTextfield(4): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiTextfield.dist(0): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiTextfield.setDim(2): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiTextfield.setDim(1): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiTextfield.setFloatOnly(0): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiTextfield.setNumbersIncludingNegativeOnly(0): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiTextfield.setNumbersOnly(0): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiTextfield.setHexOnly(0): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiTextfield.parseFloat(0): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiTextfield.parseDouble(0): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiTextfield.parseInteger(0): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiTextfield.parseLong(0): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiTextfield.init(0): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiTextfield.closed(0): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiTextfield.tick(0): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiTextfield.setTextSilent(1): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiTextfield.setText(1): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiTextfield.setText(2): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiTextfield.getText(0): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiTextfield.setValidator(1): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiTextfield.setMaxStringLength(1): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiTextfield.setSuggestion(1): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
        // GuiTextfield.moveCursorToEnd(1): not translated, its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)).
    }

    /// `GuiTextfield.GuiTextfieldDist` (team/creative/creativecore/common/gui/control/simple/GuiTextfield.java)
    pub mod gui_textfield_gui_textfield_dist {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // GuiTextfield.GuiTextfieldDist.setFloatOnly(0): not translated, it has no body.
        // GuiTextfield.GuiTextfieldDist.setNumbersIncludingNegativeOnly(0): not translated, it has no body.
        // GuiTextfield.GuiTextfieldDist.setNumbersOnly(0): not translated, it has no body.
        // GuiTextfield.GuiTextfieldDist.setHexOnly(0): not translated, it has no body.
        // GuiTextfield.GuiTextfieldDist.setText(3): not translated, it has no body.
        // GuiTextfield.GuiTextfieldDist.getText(0): not translated, it has no body.
        // GuiTextfield.GuiTextfieldDist.setValidator(1): not translated, it has no body.
        // GuiTextfield.GuiTextfieldDist.setSuggestion(1): not translated, it has no body.
        // GuiTextfield.GuiTextfieldDist.setMaxStringLength(1): not translated, it has no body.
        // GuiTextfield.GuiTextfieldDist.moveCursorToEnd(1): not translated, it has no body.
    }

    /// `GuiAnimationHandler` (team/creative/creativecore/common/gui/control/timeline/GuiAnimationHandler.java)
    pub mod gui_animation_handler {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // GuiAnimationHandler.loop(1): not translated, it has no body.
        // GuiAnimationHandler.play(0): not translated, it has no body.
        // GuiAnimationHandler.pause(0): not translated, it has no body.
        // GuiAnimationHandler.stop(0): not translated, it has no body.
        // GuiAnimationHandler.set(1): not translated, it has no body.
        // GuiAnimationHandler.get(0): not translated, it has no body.
    }

    /// `GuiTimeline` (team/creative/creativecore/common/gui/control/timeline/GuiTimeline.java)
    pub mod gui_timeline {
        use super::*;
        // The class itself isn't translated: it extends `GuiParent` (team.creative.creativecore.common.gui). Its static parts are below.
        // GuiTimeline.GuiTimeline(2): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTimeline.channels(0): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTimeline.getChannelParent(0): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTimeline.dist(0): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTimeline.getDuration(0): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTimeline.setDuration(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTimeline.selectKey(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTimeline.deselect(0): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTimeline.adjustKeyPositionX(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTimeline.adjustKeysPositionX(0): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTimeline.setSidebarWidth(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTimeline.addGuiTimelineChannel(2): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTimeline.removeChannel(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTimeline.isExpandableX(0): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
    }

    /// `GuiTimeline.GuiTimelineDist` (team/creative/creativecore/common/gui/control/timeline/GuiTimeline.java)
    pub mod gui_timeline_gui_timeline_dist {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // GuiTimeline.GuiTimelineDist.afterDurationSet(0): not translated, it has no body.
        // GuiTimeline.GuiTimelineDist.adjustKeyPositionX(1): not translated, it has no body.
        // GuiTimeline.GuiTimelineDist.adjustKeysPositionX(0): not translated, it has no body.
    }

    /// `GuiTimeline.KeySelectedEvent` (team/creative/creativecore/common/gui/control/timeline/GuiTimeline.java)
    pub mod gui_timeline_key_selected_event {
        use super::*;
        // The class itself isn't translated: it extends `GuiControlEvent` (team.creative.creativecore.common.gui.event). Its static parts are below.
        // GuiTimeline.KeySelectedEvent.KeySelectedEvent(1): not translated, its class isn't translated (it extends `GuiControlEvent` (team.creative.creativecore.common.gui.event)).
        // GuiTimeline.KeySelectedEvent.cancelable(0): not translated, its class isn't translated (it extends `GuiControlEvent` (team.creative.creativecore.common.gui.event)).
    }

    /// `GuiTimeline.NoKeySelectedEvent` (team/creative/creativecore/common/gui/control/timeline/GuiTimeline.java)
    pub mod gui_timeline_no_key_selected_event {
        use super::*;
        // The class itself isn't translated: it extends `GuiControlEvent` (team.creative.creativecore.common.gui.event). Its static parts are below.
        // GuiTimeline.NoKeySelectedEvent.NoKeySelectedEvent(1): not translated, its class isn't translated (it extends `GuiControlEvent` (team.creative.creativecore.common.gui.event)).
        // GuiTimeline.NoKeySelectedEvent.cancelable(0): not translated, its class isn't translated (it extends `GuiControlEvent` (team.creative.creativecore.common.gui.event)).
    }

    /// `GuiTimelineChannel` (team/creative/creativecore/common/gui/control/timeline/GuiTimelineChannel.java)
    pub mod gui_timeline_channel {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GuiTimelineChannel.GuiTimelineChannel(1): not translated, its class isn't translated (it's generic).
        // GuiTimelineChannel.dist(0): not translated, its class isn't translated (it's generic).
        // GuiTimelineChannel.setTooltip(1): not translated, its class isn't translated (it's generic).
        // GuiTimelineChannel.addKeyFixed(2): not translated, its class isn't translated (it's generic).
        // GuiTimelineChannel.addKey(2): not translated, its class isn't translated (it's generic).
        // GuiTimelineChannel.select(1): not translated, its class isn't translated (it's generic).
        // GuiTimelineChannel.deslect(0): not translated, its class isn't translated (it's generic).
        // GuiTimelineChannel.removeChannel(0): not translated, its class isn't translated (it's generic).
        // GuiTimelineChannel.removeKey(1): not translated, its class isn't translated (it's generic).
        // GuiTimelineChannel.movedKey(1): not translated, its class isn't translated (it's generic).
        // GuiTimelineChannel.dragKey(1): not translated, its class isn't translated (it's generic).
        // GuiTimelineChannel.isSpaceFor(2): not translated, its class isn't translated (it's generic).
        // GuiTimelineChannel.getValueAt(1): not translated, it has no body.
        // GuiTimelineChannel.isChannelEmpty(0): not translated, its class isn't translated (it's generic).
        // GuiTimelineChannel.keys(0): not translated, its class isn't translated (it's generic).
        // GuiTimelineChannel.getFirst(0): not translated, its class isn't translated (it's generic).
        // GuiTimelineChannel.getLast(0): not translated, its class isn't translated (it's generic).
    }

    /// `GuiTimelineChannel.GuiTimelineChannelDist` (team/creative/creativecore/common/gui/control/timeline/GuiTimelineChannel.java)
    pub mod gui_timeline_channel_gui_timeline_channel_dist {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // GuiTimelineChannel.GuiTimelineChannelDist.keyInitialFlow(1): not translated, it has no body.
        // GuiTimelineChannel.GuiTimelineChannelDist.dragKey(1): not translated, it has no body.
    }

    /// `GuiTimelineChannelDouble` (team/creative/creativecore/common/gui/control/timeline/GuiTimelineChannelDouble.java)
    pub mod gui_timeline_channel_double {
        use super::*;
        // The class itself isn't translated: it extends `GuiTimelineChannel`. Its static parts are below.
        // GuiTimelineChannelDouble.GuiTimelineChannelDouble(1): not translated, its class isn't translated (it extends `GuiTimelineChannel`).
        // GuiTimelineChannelDouble.getValueAt(1): not translated, its class isn't translated (it extends `GuiTimelineChannel`).
    }

    /// `GuiTimelineHeader` (team/creative/creativecore/common/gui/control/timeline/GuiTimelineHeader.java)
    pub mod gui_timeline_header {
        use super::*;
        // The class itself isn't translated: it extends `GuiColumnHeader`. Its static parts are below.
        // GuiTimelineHeader.GuiTimelineHeader(1): not translated, its class isn't translated (it extends `GuiColumnHeader`).
    }

    /// `GuiTimelineKey` (team/creative/creativecore/common/gui/control/timeline/GuiTimelineKey.java)
    pub mod gui_timeline_key {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GuiTimelineKey.GuiTimelineKey(3): not translated, its class isn't translated (it's generic).
        // GuiTimelineKey.dist(0): not translated, its class isn't translated (it's generic).
        // GuiTimelineKey.setSelected(1): not translated, its class isn't translated (it's generic).
        // GuiTimelineKey.removeKey(0): not translated, its class isn't translated (it's generic).
        // GuiTimelineKey.compareTo(1): not translated, its class isn't translated (it's generic).
        // GuiTimelineKey.init(0): not translated, its class isn't translated (it's generic).
        // GuiTimelineKey.closed(0): not translated, its class isn't translated (it's generic).
        // GuiTimelineKey.tick(0): not translated, its class isn't translated (it's generic).
    }

    /// `GuiTimelineKey.GuiTimelineKeyDist` (team/creative/creativecore/common/gui/control/timeline/GuiTimelineKey.java)
    pub mod gui_timeline_key_gui_timeline_key_dist {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // GuiTimelineKey.GuiTimelineKeyDist.setSelected(1): not translated, it has no body.
    }

    /// `GuiTree` (team/creative/creativecore/common/gui/control/tree/GuiTree.java)
    pub mod gui_tree {
        use super::*;
        // The class itself isn't translated: it extends `GuiScrollXY` (team.creative.creativecore.common.gui.control.parent). Its static parts are below.
        // GuiTree.GuiTree(2): not translated, its class isn't translated (it extends `GuiScrollXY` (team.creative.creativecore.common.gui.control.parent)).
        // GuiTree.GuiTree(3): not translated, its class isn't translated (it extends `GuiScrollXY` (team.creative.creativecore.common.gui.control.parent)).
        // GuiTree.dist(0): not translated, its class isn't translated (it extends `GuiScrollXY` (team.creative.creativecore.common.gui.control.parent)).
        // GuiTree.setCheckboxes(2): not translated, its class isn't translated (it extends `GuiScrollXY` (team.creative.creativecore.common.gui.control.parent)).
        // GuiTree.setKeepOrder(1): not translated, its class isn't translated (it extends `GuiScrollXY` (team.creative.creativecore.common.gui.control.parent)).
        // GuiTree.keepOrder(0): not translated, its class isn't translated (it extends `GuiScrollXY` (team.creative.creativecore.common.gui.control.parent)).
        // GuiTree.keepSelected(0): not translated, its class isn't translated (it extends `GuiScrollXY` (team.creative.creativecore.common.gui.control.parent)).
        // GuiTree.canDeselect(0): not translated, its class isn't translated (it extends `GuiScrollXY` (team.creative.creativecore.common.gui.control.parent)).
        // GuiTree.hasCheckboxes(0): not translated, its class isn't translated (it extends `GuiScrollXY` (team.creative.creativecore.common.gui.control.parent)).
        // GuiTree.hasCheckboxesPartial(0): not translated, its class isn't translated (it extends `GuiScrollXY` (team.creative.creativecore.common.gui.control.parent)).
        // GuiTree.allItems(0): not translated, its class isn't translated (it extends `GuiScrollXY` (team.creative.creativecore.common.gui.control.parent)).
        // GuiTree.itemsChecked(0): not translated, its class isn't translated (it extends `GuiScrollXY` (team.creative.creativecore.common.gui.control.parent)).
        // GuiTree.selected(0): not translated, its class isn't translated (it extends `GuiScrollXY` (team.creative.creativecore.common.gui.control.parent)).
        // GuiTree.getFirst(0): not translated, its class isn't translated (it extends `GuiScrollXY` (team.creative.creativecore.common.gui.control.parent)).
        // GuiTree.selectFirst(0): not translated, its class isn't translated (it extends `GuiScrollXY` (team.creative.creativecore.common.gui.control.parent)).
        // GuiTree.select(1): not translated, its class isn't translated (it extends `GuiScrollXY` (team.creative.creativecore.common.gui.control.parent)).
        // GuiTree.setLineThickness(1): not translated, its class isn't translated (it extends `GuiScrollXY` (team.creative.creativecore.common.gui.control.parent)).
        // GuiTree.setRootVisibility(1): not translated, its class isn't translated (it extends `GuiScrollXY` (team.creative.creativecore.common.gui.control.parent)).
        // GuiTree.isRootVisible(0): not translated, its class isn't translated (it extends `GuiScrollXY` (team.creative.creativecore.common.gui.control.parent)).
        // GuiTree.root(0): not translated, its class isn't translated (it extends `GuiScrollXY` (team.creative.creativecore.common.gui.control.parent)).
        // GuiTree.updateTree(0): not translated, its class isn't translated (it extends `GuiScrollXY` (team.creative.creativecore.common.gui.control.parent)).
        // GuiTree.addItem(1): not translated, its class isn't translated (it extends `GuiScrollXY` (team.creative.creativecore.common.gui.control.parent)).
        // GuiTree.performModication(2): not translated, its class isn't translated (it extends `GuiScrollXY` (team.creative.creativecore.common.gui.control.parent)).
        // GuiTree.moveUp(0): not translated, its class isn't translated (it extends `GuiScrollXY` (team.creative.creativecore.common.gui.control.parent)).
        // GuiTree.moveDown(0): not translated, its class isn't translated (it extends `GuiScrollXY` (team.creative.creativecore.common.gui.control.parent)).
        // GuiTree.isDragged(0): not translated, its class isn't translated (it extends `GuiScrollXY` (team.creative.creativecore.common.gui.control.parent)).
        // GuiTree.startDrag(1): not translated, its class isn't translated (it extends `GuiScrollXY` (team.creative.creativecore.common.gui.control.parent)).
        // GuiTree.endDrag(0): not translated, its class isn't translated (it extends `GuiScrollXY` (team.creative.creativecore.common.gui.control.parent)).
    }

    /// `GuiTree.GuiTreeDist` (team/creative/creativecore/common/gui/control/tree/GuiTree.java)
    pub mod gui_tree_gui_tree_dist {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // GuiTree.GuiTreeDist.reflowTree(0): not translated, it has no body.
        // GuiTree.GuiTreeDist.setLineThickness(1): not translated, it has no body.
        // GuiTree.GuiTreeDist.isDragged(0): not translated, it has no body.
        // GuiTree.GuiTreeDist.startDrag(1): not translated, it has no body.
        // GuiTree.GuiTreeDist.endDrag(0): not translated, it has no body.
        // GuiTree.GuiTreeDist.setKeepOrder(1): not translated, it has no body.
        // GuiTree.GuiTreeDist.keepOrder(0): not translated, it has no body.
    }

    /// `GuiTree.GuiTreeSelectionChanged` (team/creative/creativecore/common/gui/control/tree/GuiTree.java)
    pub mod gui_tree_gui_tree_selection_changed {
        use super::*;
        // The class itself isn't translated: it extends `GuiControlChangedEvent` (team.creative.creativecore.common.gui.event). Its static parts are below.
        // GuiTree.GuiTreeSelectionChanged.GuiTreeSelectionChanged(3): not translated, its class isn't translated (it extends `GuiControlChangedEvent` (team.creative.creativecore.common.gui.event)).
    }

    /// `GuiTreeDragPosition` (team/creative/creativecore/common/gui/control/tree/GuiTreeDragPosition.java)
    pub mod gui_tree_drag_position {
        use super::*;
        // The class itself isn't translated: it uses ItemPosition, which isn't translated (its constants carry values). Its static parts are below.
        // GuiTreeDragPosition.GuiTreeDragPosition(3): not translated, its class isn't translated (it uses ItemPosition, which isn't translated (its constants carry values)).
        // GuiTreeDragPosition.insert(1): not translated, its class isn't translated (it uses ItemPosition, which isn't translated (its constants carry values)).
        // GuiTreeDragPosition.toString(0): not translated, its class isn't translated (it uses ItemPosition, which isn't translated (its constants carry values)).
        // GuiTreeDragPosition.position(0): not translated, its class isn't translated (it uses ItemPosition, which isn't translated (its constants carry values)).
        // GuiTreeDragPosition.item(0): not translated, its class isn't translated (it uses ItemPosition, which isn't translated (its constants carry values)).
        // GuiTreeDragPosition.above(0): not translated, its class isn't translated (it uses ItemPosition, which isn't translated (its constants carry values)).
    }

    /// `GuiTreeDragPosition.ItemPosition` (team/creative/creativecore/common/gui/control/tree/GuiTreeDragPosition.java)
    pub mod gui_tree_drag_position_item_position {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // GuiTreeDragPosition.ItemPosition.insert(2): not translated, it has no body.
    }

    /// `GuiTreeItem` (team/creative/creativecore/common/gui/control/tree/GuiTreeItem.java)
    pub mod gui_tree_item {
        use super::*;
        // The class itself isn't translated: it extends `GuiParent` (team.creative.creativecore.common.gui). Its static parts are below.
        // GuiTreeItem.GuiTreeItem(3): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTreeItem.setTranslate(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTreeItem.setTitle(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTreeItem.setTitle(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTreeItem.resetCheckboxPartial(0): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTreeItem.getOrCreateCheckbox(0): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTreeItem.isMoving(0): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTreeItem.setMoving(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTreeItem.opened(0): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTreeItem.toggle(0): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTreeItem.isAtLeastPartiallyChecked(0): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTreeItem.isChecked(0): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTreeItem.setChecked(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTreeItem.childCheckedChanged(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTreeItem.updateControls(0): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTreeItem.getParentItem(0): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTreeItem.clearItems(0): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTreeItem.removeItem(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTreeItem.removed(0): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTreeItem.insertItemAfter(2): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTreeItem.insertItemAfter(2): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTreeItem.insertItem(2): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTreeItem.insertItem(2): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTreeItem.addItem(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTreeItem.added(0): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTreeItem.updateLevel(0): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTreeItem.getLevel(0): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTreeItem.items(0): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTreeItem.itemsChecked(0): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTreeItem.getItem(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTreeItem.indexOf(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTreeItem.itemsCount(0): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTreeItem.isChild(1): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTreeItem.selected(0): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTreeItem.select(0): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTreeItem.deselect(0): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
        // GuiTreeItem.updateColor(0): not translated, its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)).
    }

    /// `BlockGuiCreator` (team/creative/creativecore/common/gui/creator/BlockGuiCreator.java)
    pub mod block_gui_creator {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // BlockGuiCreator.create(5): not translated, it has no body.
    }

    /// `GuiCreator` (team/creative/creativecore/common/gui/creator/GuiCreator.java)
    pub mod gui_creator {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `CompoundTag`. Its static parts are below.
        // REGISTRY: not translated, it uses NamedHandlerRegistry, which isn't translated (it's generic).
        // ITEM_OPENER: not translated, it uses GuiCreatorItem, which isn't translated (it extends `GuiCreator`).
        // BLOCK_OPENER: not translated, it uses GuiCreatorBlock, which isn't translated (it extends `GuiCreator`).
        // GuiCreator.register(2): not translated, it's generic.
        // GuiCreator.GuiCreator(1): not translated, its class isn't translated (it uses Minecraft's `CompoundTag`).
        // GuiCreator.getName(0): not translated, its class isn't translated (it uses Minecraft's `CompoundTag`).
        // GuiCreator.openGui(2): not translated, its class isn't translated (it uses Minecraft's `CompoundTag`).
    }

    /// `GuiCreator.GuiCreatorBasic` (team/creative/creativecore/common/gui/creator/GuiCreator.java)
    pub mod gui_creator_gui_creator_basic {
        use super::*;
        // The class itself isn't translated: it extends `GuiCreator`. Its static parts are below.
        // GuiCreator.GuiCreatorBasic.GuiCreatorBasic(1): not translated, its class isn't translated (it extends `GuiCreator`).
        // GuiCreator.GuiCreatorBasic.open(1): not translated, its class isn't translated (it extends `GuiCreator`).
        // GuiCreator.GuiCreatorBasic.open(2): not translated, its class isn't translated (it extends `GuiCreator`).
    }

    /// `GuiCreator.GuiCreatorBlock` (team/creative/creativecore/common/gui/creator/GuiCreator.java)
    pub mod gui_creator_gui_creator_block {
        use super::*;
        // The class itself isn't translated: it extends `GuiCreator`. Its static parts are below.
        // GuiCreator.GuiCreatorBlock.GuiCreatorBlock(0): not translated, its class isn't translated (it extends `GuiCreator`).
        // GuiCreator.GuiCreatorBlock.open(2): not translated, its class isn't translated (it extends `GuiCreator`).
        // GuiCreator.GuiCreatorBlock.open(3): not translated, its class isn't translated (it extends `GuiCreator`).
    }

    /// `GuiCreator.GuiCreatorItem` (team/creative/creativecore/common/gui/creator/GuiCreator.java)
    pub mod gui_creator_gui_creator_item {
        use super::*;
        // The class itself isn't translated: it extends `GuiCreator`. Its static parts are below.
        // GuiCreator.GuiCreatorItem.GuiCreatorItem(0): not translated, its class isn't translated (it extends `GuiCreator`).
        // GuiCreator.GuiCreatorItem.open(2): not translated, its class isn't translated (it extends `GuiCreator`).
        // GuiCreator.GuiCreatorItem.open(3): not translated, its class isn't translated (it extends `GuiCreator`).
    }

    /// `ItemGuiCreator` (team/creative/creativecore/common/gui/creator/ItemGuiCreator.java)
    pub mod item_gui_creator {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ItemGuiCreator.create(2): not translated, it has no body.
    }

    /// `DialogGuiLayer` (team/creative/creativecore/common/gui/dialog/DialogGuiLayer.java)
    pub mod dialog_gui_layer {
        use super::*;
        // The class itself isn't translated: it extends `GuiLayer` (team.creative.creativecore.common.gui). Its static parts are below.
        // DialogGuiLayer.DialogGuiLayer(5): not translated, its class isn't translated (it extends `GuiLayer` (team.creative.creativecore.common.gui)).
        // DialogGuiLayer.create(0): not translated, its class isn't translated (it extends `GuiLayer` (team.creative.creativecore.common.gui)).
        // DialogGuiLayer.closeDialog(1): not translated, its class isn't translated (it extends `GuiLayer` (team.creative.creativecore.common.gui)).
    }

    /// `DialogGuiLayer.DialogButton` (team/creative/creativecore/common/gui/dialog/DialogGuiLayer.java)
    pub mod dialog_gui_layer_dialog_button {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum DialogButton {
            Ok,
            Yes,
            No,
            Maybe,
            Cancel,
            Abort,
            Confirm,
        }
        impl DialogButton {
            pub const VALUES: [DialogButton; 7] = [DialogButton::Ok, DialogButton::Yes, DialogButton::No, DialogButton::Maybe, DialogButton::Cancel, DialogButton::Abort, DialogButton::Confirm, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    DialogButton::Ok => "OK",
                    DialogButton::Yes => "YES",
                    DialogButton::No => "NO",
                    DialogButton::Maybe => "MAYBE",
                    DialogButton::Cancel => "CANCEL",
                    DialogButton::Abort => "ABORT",
                    DialogButton::Confirm => "CONFIRM",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "OK" => DialogButton::Ok,
                    "YES" => DialogButton::Yes,
                    "NO" => DialogButton::No,
                    "MAYBE" => DialogButton::Maybe,
                    "CANCEL" => DialogButton::Cancel,
                    "ABORT" => DialogButton::Abort,
                    "CONFIRM" => DialogButton::Confirm,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `GuiDialogHandler` (team/creative/creativecore/common/gui/dialog/GuiDialogHandler.java)
    pub mod gui_dialog_handler {
        use super::*;
        // DIALOG_HANDLER: not translated, it uses GuiSyncGlobalLayer, which isn't translated (it's generic).
        #[derive(Clone, Debug)]
        pub struct GuiDialogHandler {
        }
        impl GuiDialogHandler {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn init() {
        }
        // GuiDialogHandler.openDialog(4): not translated, it uses GuiLayer, which isn't translated (it extends `GuiParent`).
        // GuiDialogHandler.openDialog(5): not translated, it uses GuiLayer, which isn't translated (it extends `GuiParent`).
    }

    /// `GuiControlChangedEvent` (team/creative/creativecore/common/gui/event/GuiControlChangedEvent.java)
    pub mod gui_control_changed_event {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GuiControlChangedEvent.GuiControlChangedEvent(1): not translated, its class isn't translated (it's generic).
        // GuiControlChangedEvent.cancelable(0): not translated, its class isn't translated (it's generic).
    }

    /// `GuiControlClickEvent` (team/creative/creativecore/common/gui/event/GuiControlClickEvent.java)
    pub mod gui_control_click_event {
        use super::*;
        // The class itself isn't translated: it extends `GuiControlEvent`. Its static parts are below.
        // GuiControlClickEvent.GuiControlClickEvent(3): not translated, its class isn't translated (it extends `GuiControlEvent`).
        // GuiControlClickEvent.cancelable(0): not translated, its class isn't translated (it extends `GuiControlEvent`).
    }

    /// `GuiControlEvent` (team/creative/creativecore/common/gui/event/GuiControlEvent.java)
    pub mod gui_control_event {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GuiControlEvent.GuiControlEvent(1): not translated, its class isn't translated (it's generic).
    }

    /// `GuiEvent` (team/creative/creativecore/common/gui/event/GuiEvent.java)
    pub mod gui_event {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct GuiEvent {
            pub canceled: bool,
        }
        impl GuiEvent {
            pub fn new() -> Self {
                let mut s = Self { canceled: false };
                s
            }
            pub fn is_canceled(&self) -> bool {
                return self.canceled;
            }
            // GuiEvent.cancel(0): not translated, it calls GuiEvent.getClass, which isn't in the mod's source.
            // GuiEvent.cancelable(0): not translated, it has no body.
        }
    }

    /// `GuiEventManager` (team/creative/creativecore/common/gui/event/GuiEventManager.java)
    pub mod gui_event_manager {
        use super::*;
        // The class itself isn't translated: it uses GuiEventHandler, which isn't translated (it's generic). Its static parts are below.
        // GuiEventManager.raiseEvent(1): not translated, its class isn't translated (it uses GuiEventHandler, which isn't translated (it's generic)).
        // GuiEventManager.registerEvent(2): not translated, it's generic.
        // GuiEventManager.clear(0): not translated, its class isn't translated (it uses GuiEventHandler, which isn't translated (it's generic)).
    }

    /// `GuiEventManager.GuiEventHandler` (team/creative/creativecore/common/gui/event/GuiEventManager.java)
    pub mod gui_event_manager_gui_event_handler {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GuiEventManager.GuiEventHandler.GuiEventHandler(2): not translated, its class isn't translated (it's generic).
        // GuiEventManager.GuiEventHandler.react(1): not translated, its class isn't translated (it's generic).
    }

    /// `GuiSliderUpdateEvent` (team/creative/creativecore/common/gui/event/GuiSliderUpdateEvent.java)
    pub mod gui_slider_update_event {
        use super::*;
        // The class itself isn't translated: it extends `GuiControlChangedEvent`. Its static parts are below.
        // GuiSliderUpdateEvent.GuiSliderUpdateEvent(1): not translated, its class isn't translated (it extends `GuiControlChangedEvent`).
    }

    /// `GuiTextUpdateEvent` (team/creative/creativecore/common/gui/event/GuiTextUpdateEvent.java)
    pub mod gui_text_update_event {
        use super::*;
        // The class itself isn't translated: it extends `GuiControlChangedEvent`. Its static parts are below.
        // GuiTextUpdateEvent.GuiTextUpdateEvent(1): not translated, its class isn't translated (it extends `GuiControlChangedEvent`).
    }

    /// `GuiTooltipEvent` (team/creative/creativecore/common/gui/event/GuiTooltipEvent.java)
    pub mod gui_tooltip_event {
        use super::*;
        // The class itself isn't translated: it extends `GuiControlEvent`. Its static parts are below.
        // GuiTooltipEvent.GuiTooltipEvent(3): not translated, its class isn't translated (it extends `GuiControlEvent`).
        // GuiTooltipEvent.cancelable(0): not translated, its class isn't translated (it extends `GuiControlEvent`).
    }

    /// `GuiFlow` (team/creative/creativecore/common/gui/flow/GuiFlow.java)
    pub mod gui_flow {
        use super::*;
        // STACK_X: not translated, it uses GuiStackX, which isn't translated (it extends `GuiFlow`).
        // FIT_X: not translated, it uses GuiFlowFitX, which isn't translated (it extends `GuiStackX`).
        // STACK_Y: not translated, it uses GuiStackY, which isn't translated (it extends `GuiFlow`).
        #[derive(Clone, Debug)]
        pub struct GuiFlow {
        }
        impl GuiFlow {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // GuiFlow.minWidth(3): not translated, it has no body.
            // GuiFlow.preferredWidth(3): not translated, it has no body.
            // GuiFlow.minHeight(4): not translated, it has no body.
            // GuiFlow.preferredHeight(4): not translated, it has no body.
            // GuiFlow.flowX(6): not translated, it has no body.
            // GuiFlow.flowY(7): not translated, it has no body.
        }
        // GuiFlow.areChildrenExpandableX(1): not translated, it uses GuiControlRect, which isn't translated (it uses GuiClientControl, which isn't translated (it's generic)).
        // GuiFlow.areChildrenExpandableY(1): not translated, it uses GuiControlRect, which isn't translated (it uses GuiClientControl, which isn't translated (it's generic)).
    }

    /// `GuiFlowFitX` (team/creative/creativecore/common/gui/flow/GuiFlowFitX.java)
    pub mod gui_flow_fit_x {
        use super::*;
        // The class itself isn't translated: it extends `GuiStackX`. Its static parts are below.
        // GuiFlowFitX.minWidth(3): not translated, its class isn't translated (it extends `GuiStackX`).
        // GuiFlowFitX.minHeight(4): not translated, its class isn't translated (it extends `GuiStackX`).
        // GuiFlowFitX.preferredHeight(4): not translated, its class isn't translated (it extends `GuiStackX`).
        // GuiFlowFitX.flowX(6): not translated, its class isn't translated (it extends `GuiStackX`).
        // GuiFlowFitX.flowY(7): not translated, its class isn't translated (it extends `GuiStackX`).
    }

    /// `GuiFlowFitX.GuiRowControl` (team/creative/creativecore/common/gui/flow/GuiFlowFitX.java)
    pub mod gui_flow_fit_x_gui_row_control {
        use super::*;
        // The class itself isn't translated: it extends `GuiControlRect` (team.creative.creativecore.client.gui). Its static parts are below.
        // GuiFlowFitX.GuiRowControl.GuiRowControl(4): not translated, its class isn't translated (it extends `GuiControlRect` (team.creative.creativecore.client.gui)).
        // GuiFlowFitX.GuiRowControl.getContentWidth(0): not translated, its class isn't translated (it extends `GuiControlRect` (team.creative.creativecore.client.gui)).
        // GuiFlowFitX.GuiRowControl.getMinWidth(1): not translated, its class isn't translated (it extends `GuiControlRect` (team.creative.creativecore.client.gui)).
        // GuiFlowFitX.GuiRowControl.getMaxWidth(1): not translated, its class isn't translated (it extends `GuiControlRect` (team.creative.creativecore.client.gui)).
        // GuiFlowFitX.GuiRowControl.getPreferredWidth(1): not translated, its class isn't translated (it extends `GuiControlRect` (team.creative.creativecore.client.gui)).
        // GuiFlowFitX.GuiRowControl.getContentHeight(0): not translated, its class isn't translated (it extends `GuiControlRect` (team.creative.creativecore.client.gui)).
        // GuiFlowFitX.GuiRowControl.getMinHeight(1): not translated, its class isn't translated (it extends `GuiControlRect` (team.creative.creativecore.client.gui)).
        // GuiFlowFitX.GuiRowControl.getMaxHeight(1): not translated, its class isn't translated (it extends `GuiControlRect` (team.creative.creativecore.client.gui)).
        // GuiFlowFitX.GuiRowControl.getPreferredHeight(1): not translated, its class isn't translated (it extends `GuiControlRect` (team.creative.creativecore.client.gui)).
        // GuiFlowFitX.GuiRowControl.flowX(0): not translated, its class isn't translated (it extends `GuiControlRect` (team.creative.creativecore.client.gui)).
        // GuiFlowFitX.GuiRowControl.flowY(0): not translated, its class isn't translated (it extends `GuiControlRect` (team.creative.creativecore.client.gui)).
        // GuiFlowFitX.GuiRowControl.isExpandableX(0): not translated, its class isn't translated (it extends `GuiControlRect` (team.creative.creativecore.client.gui)).
        // GuiFlowFitX.GuiRowControl.isExpandableY(0): not translated, its class isn't translated (it extends `GuiControlRect` (team.creative.creativecore.client.gui)).
    }

    /// `GuiSizeRule` (team/creative/creativecore/common/gui/flow/GuiSizeRule.java)
    pub mod gui_size_rule {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct GuiSizeRule {
        }
        impl GuiSizeRule {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // GuiSizeRule.minWidth(2): not translated, it has no body.
            // GuiSizeRule.preferredWidth(2): not translated, it has no body.
            // GuiSizeRule.maxWidth(2): not translated, it has no body.
            // GuiSizeRule.minHeight(3): not translated, it has no body.
            // GuiSizeRule.preferredHeight(3): not translated, it has no body.
            // GuiSizeRule.maxHeight(3): not translated, it has no body.
        }
    }

    /// `GuiSizeRule.GuiFixedDimension` (team/creative/creativecore/common/gui/flow/GuiSizeRule.java)
    pub mod gui_size_rule_gui_fixed_dimension {
        use super::*;
        // The class itself isn't translated: it extends `GuiSizeRule`. Its static parts are below.
        // GuiSizeRule.GuiFixedDimension.GuiFixedDimension(2): not translated, its class isn't translated (it extends `GuiSizeRule`).
        // GuiSizeRule.GuiFixedDimension.GuiFixedDimension(1): not translated, its class isn't translated (it extends `GuiSizeRule`).
        // GuiSizeRule.GuiFixedDimension.minWidth(2): not translated, its class isn't translated (it extends `GuiSizeRule`).
        // GuiSizeRule.GuiFixedDimension.preferredWidth(2): not translated, its class isn't translated (it extends `GuiSizeRule`).
        // GuiSizeRule.GuiFixedDimension.maxWidth(2): not translated, its class isn't translated (it extends `GuiSizeRule`).
        // GuiSizeRule.GuiFixedDimension.minHeight(3): not translated, its class isn't translated (it extends `GuiSizeRule`).
        // GuiSizeRule.GuiFixedDimension.preferredHeight(3): not translated, its class isn't translated (it extends `GuiSizeRule`).
        // GuiSizeRule.GuiFixedDimension.maxHeight(3): not translated, its class isn't translated (it extends `GuiSizeRule`).
    }

    /// `GuiSizeRule.GuiRatioDimension` (team/creative/creativecore/common/gui/flow/GuiSizeRule.java)
    pub mod gui_size_rule_gui_ratio_dimension {
        use super::*;
        // The class itself isn't translated: it extends `GuiSizeRule`. Its static parts are below.
        // GuiSizeRule.GuiRatioDimension.GuiRatioDimension(2): not translated, its class isn't translated (it extends `GuiSizeRule`).
        // GuiSizeRule.GuiRatioDimension.minWidth(2): not translated, its class isn't translated (it extends `GuiSizeRule`).
        // GuiSizeRule.GuiRatioDimension.preferredWidth(2): not translated, its class isn't translated (it extends `GuiSizeRule`).
        // GuiSizeRule.GuiRatioDimension.maxWidth(2): not translated, its class isn't translated (it extends `GuiSizeRule`).
        // GuiSizeRule.GuiRatioDimension.minHeight(3): not translated, its class isn't translated (it extends `GuiSizeRule`).
        // GuiSizeRule.GuiRatioDimension.preferredHeight(3): not translated, its class isn't translated (it extends `GuiSizeRule`).
        // GuiSizeRule.GuiRatioDimension.maxHeight(3): not translated, its class isn't translated (it extends `GuiSizeRule`).
    }

    /// `GuiSizeRule.GuiSizeRatioRules` (team/creative/creativecore/common/gui/flow/GuiSizeRule.java)
    pub mod gui_size_rule_gui_size_ratio_rules {
        use super::*;
        // The class itself isn't translated: it extends `GuiSizeRule`. Its static parts are below.
        // GuiSizeRule.GuiSizeRatioRules.GuiSizeRatioRules(0): not translated, its class isn't translated (it extends `GuiSizeRule`).
        // GuiSizeRule.GuiSizeRatioRules.GuiSizeRatioRules(2): not translated, its class isn't translated (it extends `GuiSizeRule`).
        // GuiSizeRule.GuiSizeRatioRules.minWidth(1): not translated, its class isn't translated (it extends `GuiSizeRule`).
        // GuiSizeRule.GuiSizeRatioRules.widthRatio(1): not translated, its class isn't translated (it extends `GuiSizeRule`).
        // GuiSizeRule.GuiSizeRatioRules.maxWidth(1): not translated, its class isn't translated (it extends `GuiSizeRule`).
        // GuiSizeRule.GuiSizeRatioRules.minHeight(1): not translated, its class isn't translated (it extends `GuiSizeRule`).
        // GuiSizeRule.GuiSizeRatioRules.heightRatio(1): not translated, its class isn't translated (it extends `GuiSizeRule`).
        // GuiSizeRule.GuiSizeRatioRules.maxHeight(1): not translated, its class isn't translated (it extends `GuiSizeRule`).
        // GuiSizeRule.GuiSizeRatioRules.minWidth(2): not translated, its class isn't translated (it extends `GuiSizeRule`).
        // GuiSizeRule.GuiSizeRatioRules.preferredWidth(2): not translated, its class isn't translated (it extends `GuiSizeRule`).
        // GuiSizeRule.GuiSizeRatioRules.maxWidth(2): not translated, its class isn't translated (it extends `GuiSizeRule`).
        // GuiSizeRule.GuiSizeRatioRules.minHeight(3): not translated, its class isn't translated (it extends `GuiSizeRule`).
        // GuiSizeRule.GuiSizeRatioRules.preferredHeight(3): not translated, its class isn't translated (it extends `GuiSizeRule`).
        // GuiSizeRule.GuiSizeRatioRules.maxHeight(3): not translated, its class isn't translated (it extends `GuiSizeRule`).
    }

    /// `GuiSizeRule.GuiSizeRules` (team/creative/creativecore/common/gui/flow/GuiSizeRule.java)
    pub mod gui_size_rule_gui_size_rules {
        use super::*;
        // The class itself isn't translated: it extends `GuiSizeRule`. Its static parts are below.
        // GuiSizeRule.GuiSizeRules.minWidth(1): not translated, its class isn't translated (it extends `GuiSizeRule`).
        // GuiSizeRule.GuiSizeRules.prefWidth(1): not translated, its class isn't translated (it extends `GuiSizeRule`).
        // GuiSizeRule.GuiSizeRules.maxWidth(1): not translated, its class isn't translated (it extends `GuiSizeRule`).
        // GuiSizeRule.GuiSizeRules.minHeight(1): not translated, its class isn't translated (it extends `GuiSizeRule`).
        // GuiSizeRule.GuiSizeRules.prefHeight(1): not translated, its class isn't translated (it extends `GuiSizeRule`).
        // GuiSizeRule.GuiSizeRules.maxHeight(1): not translated, its class isn't translated (it extends `GuiSizeRule`).
        // GuiSizeRule.GuiSizeRules.minWidth(2): not translated, its class isn't translated (it extends `GuiSizeRule`).
        // GuiSizeRule.GuiSizeRules.preferredWidth(2): not translated, its class isn't translated (it extends `GuiSizeRule`).
        // GuiSizeRule.GuiSizeRules.maxWidth(2): not translated, its class isn't translated (it extends `GuiSizeRule`).
        // GuiSizeRule.GuiSizeRules.minHeight(3): not translated, its class isn't translated (it extends `GuiSizeRule`).
        // GuiSizeRule.GuiSizeRules.preferredHeight(3): not translated, its class isn't translated (it extends `GuiSizeRule`).
        // GuiSizeRule.GuiSizeRules.maxHeight(3): not translated, its class isn't translated (it extends `GuiSizeRule`).
    }

    /// `GuiStackX` (team/creative/creativecore/common/gui/flow/GuiStackX.java)
    pub mod gui_stack_x {
        use super::*;
        // The class itself isn't translated: it extends `GuiFlow`. Its static parts are below.
        // GuiStackX.minWidth(3): not translated, its class isn't translated (it extends `GuiFlow`).
        // GuiStackX.preferredWidth(3): not translated, its class isn't translated (it extends `GuiFlow`).
        // GuiStackX.minHeight(4): not translated, its class isn't translated (it extends `GuiFlow`).
        // GuiStackX.preferredHeight(4): not translated, its class isn't translated (it extends `GuiFlow`).
        // GuiStackX.flowX(6): not translated, its class isn't translated (it extends `GuiFlow`).
        // GuiStackX.flowY(7): not translated, its class isn't translated (it extends `GuiFlow`).
    }

    /// `GuiStackY` (team/creative/creativecore/common/gui/flow/GuiStackY.java)
    pub mod gui_stack_y {
        use super::*;
        // The class itself isn't translated: it extends `GuiFlow`. Its static parts are below.
        // GuiStackY.minWidth(3): not translated, its class isn't translated (it extends `GuiFlow`).
        // GuiStackY.preferredWidth(3): not translated, its class isn't translated (it extends `GuiFlow`).
        // GuiStackY.minHeight(4): not translated, its class isn't translated (it extends `GuiFlow`).
        // GuiStackY.preferredHeight(4): not translated, its class isn't translated (it extends `GuiFlow`).
        // GuiStackY.flowX(6): not translated, its class isn't translated (it extends `GuiFlow`).
        // GuiStackY.flowY(7): not translated, its class isn't translated (it extends `GuiFlow`).
    }

    /// `GuiControl` (team/creative/creativecore/common/gui/GuiControl.java)
    pub mod gui_control {
        use super::*;
        // The class itself isn't translated: it uses IGuiParent, which isn't translated (it's an interface). Its static parts are below.
        // GuiControl.setParent(2): not translated, it's overloaded with the same number of parameters.
        // GuiControl.setParent(2): not translated, it's overloaded with the same number of parameters.
        // GuiControl.GuiControl(2): not translated, its class isn't translated (it uses IGuiParent, which isn't translated (it's an interface)).
        // GuiControl.dist(0): not translated, its class isn't translated (it uses IGuiParent, which isn't translated (it's an interface)).
        // GuiControl.setVisible(1): not translated, its class isn't translated (it uses IGuiParent, which isn't translated (it's an interface)).
        // GuiControl.setFixed(0): not translated, its class isn't translated (it uses IGuiParent, which isn't translated (it's an interface)).
        // GuiControl.setFixedX(0): not translated, its class isn't translated (it uses IGuiParent, which isn't translated (it's an interface)).
        // GuiControl.setFixedY(0): not translated, its class isn't translated (it uses IGuiParent, which isn't translated (it's an interface)).
        // GuiControl.setExpandable(0): not translated, its class isn't translated (it uses IGuiParent, which isn't translated (it's an interface)).
        // GuiControl.setExpandableX(0): not translated, its class isn't translated (it uses IGuiParent, which isn't translated (it's an interface)).
        // GuiControl.setExpandableY(0): not translated, its class isn't translated (it uses IGuiParent, which isn't translated (it's an interface)).
        // GuiControl.isExpandableX(0): not translated, its class isn't translated (it uses IGuiParent, which isn't translated (it's an interface)).
        // GuiControl.isExpandableY(0): not translated, its class isn't translated (it uses IGuiParent, which isn't translated (it's an interface)).
        // GuiControl.setDim(2): not translated, its class isn't translated (it uses IGuiParent, which isn't translated (it's an interface)).
        // GuiControl.setDim(1): not translated, its class isn't translated (it uses IGuiParent, which isn't translated (it's an interface)).
        // GuiControl.setEnabled(1): not translated, its class isn't translated (it uses IGuiParent, which isn't translated (it's an interface)).
        // GuiControl.removeFormatting(0): not translated, its class isn't translated (it uses IGuiParent, which isn't translated (it's an interface)).
        // GuiControl.setFormatting(1): not translated, its class isn't translated (it uses IGuiParent, which isn't translated (it's an interface)).
        // GuiControl.setCustomFormatting(0): not translated, its class isn't translated (it uses IGuiParent, which isn't translated (it's an interface)).
        // GuiControl.reflow(0): not translated, its class isn't translated (it uses IGuiParent, which isn't translated (it's an interface)).
        // GuiControl.provider(0): not translated, its class isn't translated (it uses IGuiParent, which isn't translated (it's an interface)).
        // GuiControl.isClient(0): not translated, its class isn't translated (it uses IGuiParent, which isn't translated (it's an interface)).
        // GuiControl.setTooltip(1): not translated, its class isn't translated (it uses IGuiParent, which isn't translated (it's an interface)).
        // GuiControl.setTooltip(1): not translated, its class isn't translated (it uses IGuiParent, which isn't translated (it's an interface)).
        // GuiControl.hasGui(0): not translated, its class isn't translated (it uses IGuiParent, which isn't translated (it's an interface)).
        // GuiControl.isParent(1): not translated, its class isn't translated (it uses IGuiParent, which isn't translated (it's an interface)).
        // GuiControl.getParent(0): not translated, its class isn't translated (it uses IGuiParent, which isn't translated (it's an interface)).
        // GuiControl.getIntegratedParent(0): not translated, its class isn't translated (it uses IGuiParent, which isn't translated (it's an interface)).
        // GuiControl.getNestedName(0): not translated, its class isn't translated (it uses IGuiParent, which isn't translated (it's an interface)).
        // GuiControl.hasLayer(0): not translated, its class isn't translated (it uses IGuiParent, which isn't translated (it's an interface)).
        // GuiControl.getLayer(0): not translated, its class isn't translated (it uses IGuiParent, which isn't translated (it's an interface)).
        // GuiControl.init(0): not translated, it has no body.
        // GuiControl.closed(0): not translated, it has no body.
        // GuiControl.tick(0): not translated, it has no body.
        // GuiControl.is(1): not translated, its class isn't translated (it uses IGuiParent, which isn't translated (it's an interface)).
        // GuiControl.is(1): not translated, its class isn't translated (it uses IGuiParent, which isn't translated (it's an interface)).
        // GuiControl.raiseEvent(1): not translated, its class isn't translated (it uses IGuiParent, which isn't translated (it's an interface)).
        // GuiControl.getPlayer(0): not translated, its class isn't translated (it uses IGuiParent, which isn't translated (it's an interface)).
        // GuiControl.toLayerRect(1): not translated, its class isn't translated (it uses IGuiParent, which isn't translated (it's an interface)).
        // GuiControl.toScreenRect(1): not translated, its class isn't translated (it uses IGuiParent, which isn't translated (it's an interface)).
        // GuiControl.translatable(1): not translated, it uses Minecraft's `MutableComponent`.
        // GuiControl.translatable(2): not translated, it uses Minecraft's `MutableComponent`.
        // GuiControl.translate(1): not translated, it calls LanguageUtils.translate, which isn't translated.
        // GuiControl.translate(2): not translated, it uses `Object`.
        // GuiControl.translateOrDefault(2): not translated, it calls LanguageUtils.translateOr, which isn't translated.
    }

    /// `GuiControl.GuiFocusControlDist` (team/creative/creativecore/common/gui/GuiControl.java)
    pub mod gui_control_gui_focus_control_dist {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // GuiControl.GuiFocusControlDist.isFocused(0): not translated, it has no body.
        // GuiControl.GuiFocusControlDist.focus(0): not translated, it has no body.
    }

    /// `GuiControlDistHandler` (team/creative/creativecore/common/gui/GuiControlDistHandler.java)
    pub mod gui_control_dist_handler {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // GuiControlDistHandler.setVisible(1): not translated, it has no body.
        // GuiControlDistHandler.setFixed(0): not translated, it has no body.
        // GuiControlDistHandler.setFixedX(0): not translated, it has no body.
        // GuiControlDistHandler.setFixedY(0): not translated, it has no body.
        // GuiControlDistHandler.setExpandable(0): not translated, it has no body.
        // GuiControlDistHandler.setExpandableX(0): not translated, it has no body.
        // GuiControlDistHandler.setExpandableY(0): not translated, it has no body.
        // GuiControlDistHandler.isExpandableX(0): not translated, it has no body.
        // GuiControlDistHandler.isExpandableY(0): not translated, it has no body.
        // GuiControlDistHandler.setDim(2): not translated, it has no body.
        // GuiControlDistHandler.setDim(1): not translated, it has no body.
        // GuiControlDistHandler.setEnabled(1): not translated, it has no body.
        // GuiControlDistHandler.setTooltip(1): not translated, it has no body.
        // GuiControlDistHandler.setTooltip(1): not translated, it has no body.
        // GuiControlDistHandler.removeFormatting(0): not translated, it has no body.
        // GuiControlDistHandler.setFormatting(1): not translated, it has no body.
        // GuiControlDistHandler.setCustomFormatting(0): not translated, it has no body.
        // GuiControlDistHandler.playSound(1): not translated, it has no body.
        // GuiControlDistHandler.playSound(1): not translated, it has no body.
        // GuiControlDistHandler.playSound(3): not translated, it has no body.
        // GuiControlDistHandler.playSound(3): not translated, it has no body.
    }

    /// `GuiControls` (team/creative/creativecore/common/gui/GuiControls.java)
    pub mod gui_controls {
        use super::*;
        // The class itself isn't translated: it implements `Iterable`. Its static parts are below.
        // GuiControls.get(1): not translated, its class isn't translated (it implements `Iterable`).
        // GuiControls.remove(1): not translated, its class isn't translated (it implements `Iterable`).
        // GuiControls.get(1): not translated, its class isn't translated (it implements `Iterable`).
        // GuiControls.get(2): not translated, its class isn't translated (it implements `Iterable`).
        // GuiControls.insertControlBefore(2): not translated, it's generic.
        // GuiControls.insertControlAfter(2): not translated, it's generic.
        // GuiControls.add(1): not translated, its class isn't translated (it implements `Iterable`).
        // GuiControls.addHover(1): not translated, its class isn't translated (it implements `Iterable`).
        // GuiControls.contains(1): not translated, its class isn't translated (it implements `Iterable`).
        // GuiControls.remove(1): not translated, its class isn't translated (it implements `Iterable`).
        // GuiControls.replace(2): not translated, its class isn't translated (it implements `Iterable`).
        // GuiControls.remove(1): not translated, its class isn't translated (it implements `Iterable`).
        // GuiControls.removeExclude(1): not translated, its class isn't translated (it implements `Iterable`).
        // GuiControls.isEmpty(0): not translated, its class isn't translated (it implements `Iterable`).
        // GuiControls.clear(0): not translated, its class isn't translated (it implements `Iterable`).
        // GuiControls.controlSize(0): not translated, its class isn't translated (it implements `Iterable`).
        // GuiControls.hoverSize(0): not translated, its class isn't translated (it implements `Iterable`).
        // GuiControls.totalSize(0): not translated, its class isn't translated (it implements `Iterable`).
        // GuiControls.iterator(0): not translated, its class isn't translated (it implements `Iterable`).
        // GuiControls.distControls(0): not translated, its class isn't translated (it implements `Iterable`).
        // GuiControls.distHoverControls(0): not translated, its class isn't translated (it implements `Iterable`).
        // GuiControls.distAll(0): not translated, its class isn't translated (it implements `Iterable`).
        // GuiControls.streamDistControls(0): not translated, its class isn't translated (it implements `Iterable`).
        // GuiControls.streamDistHoverControls(0): not translated, its class isn't translated (it implements `Iterable`).
        // GuiControls.distListControls(1): not translated, its class isn't translated (it implements `Iterable`).
        // GuiControls.distHoverListControls(1): not translated, its class isn't translated (it implements `Iterable`).
    }

    /// `GuiLayer` (team/creative/creativecore/common/gui/GuiLayer.java)
    pub mod gui_layer {
        use super::*;
        // The class itself isn't translated: it extends `GuiParent`. Its static parts are below.
        // CLIENT: not translated, its value couldn't be read: an anonymous class.
        // SERVER: not translated, its value couldn't be read: an anonymous class.
        // GuiLayer.collectInventories(2): not translated, it uses GuiControl, which isn't translated (it uses IGuiParent, which isn't translated (it's an interface)).
        // GuiLayer.GuiLayer(2): not translated, its class isn't translated (it extends `GuiParent`).
        // GuiLayer.GuiLayer(4): not translated, its class isn't translated (it extends `GuiParent`).
        // GuiLayer.dist(0): not translated, its class isn't translated (it extends `GuiParent`).
        // GuiLayer.setParent(1): not translated, its class isn't translated (it extends `GuiParent`).
        // GuiLayer.inventoriesToInsert(0): not translated, its class isn't translated (it extends `GuiParent`).
        // GuiLayer.inventoriesToExtract(0): not translated, its class isn't translated (it extends `GuiParent`).
        // GuiLayer.has(1): not translated, its class isn't translated (it extends `GuiParent`).
        // GuiLayer.getOrCreate(1): not translated, it's generic.
        // GuiLayer.itemManager(0): not translated, its class isn't translated (it extends `GuiParent`).
        // GuiLayer.managers(0): not translated, its class isn't translated (it extends `GuiParent`).
        // GuiLayer.getSyncHolder(0): not translated, its class isn't translated (it extends `GuiParent`).
        // GuiLayer.init(0): not translated, its class isn't translated (it extends `GuiParent`).
        // GuiLayer.reinit(0): not translated, its class isn't translated (it extends `GuiParent`).
        // GuiLayer.hasMinimumOuterSpacing(0): not translated, its class isn't translated (it extends `GuiParent`).
        // GuiLayer.create(0): not translated, it has no body.
        // GuiLayer.becameTopLayer(0): not translated, its class isn't translated (it extends `GuiParent`).
        // GuiLayer.getLayer(0): not translated, its class isn't translated (it extends `GuiParent`).
        // GuiLayer.closeLayerUsingEscape(0): not translated, its class isn't translated (it extends `GuiParent`).
        // GuiLayer.hasLayer(0): not translated, its class isn't translated (it extends `GuiParent`).
        // GuiLayer.tick(0): not translated, its class isn't translated (it extends `GuiParent`).
        // GuiLayer.closed(0): not translated, its class isn't translated (it extends `GuiParent`).
        // GuiLayer.reflow(0): not translated, its class isn't translated (it extends `GuiParent`).
        // GuiLayer.toLayerRect(2): not translated, its class isn't translated (it extends `GuiParent`).
    }

    /// `GuiLayer.GuiLayerDistHandler` (team/creative/creativecore/common/gui/GuiLayer.java)
    pub mod gui_layer_gui_layer_dist_handler {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // GuiLayer.GuiLayerDistHandler.reflow(0): not translated, it has no body.
    }

    /// `GuiLayer.GuiLayerDistination` (team/creative/creativecore/common/gui/GuiLayer.java)
    pub mod gui_layer_gui_layer_distination {
        use super::*;
        // The class itself isn't translated: it implements `IGuiParent`. Its static parts are below.
        // GuiLayer.GuiLayerDistination.isContainer(0): not translated, its class isn't translated (it implements `IGuiParent`).
        // GuiLayer.GuiLayerDistination.getPlayer(0): not translated, its class isn't translated (it implements `IGuiParent`).
        // GuiLayer.GuiLayerDistination.closeTopLayer(0): not translated, its class isn't translated (it implements `IGuiParent`).
        // GuiLayer.GuiLayerDistination.closeLayer(1): not translated, its class isn't translated (it implements `IGuiParent`).
        // GuiLayer.GuiLayerDistination.raiseEvent(1): not translated, its class isn't translated (it implements `IGuiParent`).
        // GuiLayer.GuiLayerDistination.reflow(0): not translated, its class isn't translated (it implements `IGuiParent`).
        // GuiLayer.GuiLayerDistination.hasGui(0): not translated, its class isn't translated (it implements `IGuiParent`).
        // GuiLayer.GuiLayerDistination.isParent(1): not translated, its class isn't translated (it implements `IGuiParent`).
        // GuiLayer.GuiLayerDistination.toScreenRect(2): not translated, its class isn't translated (it implements `IGuiParent`).
        // GuiLayer.GuiLayerDistination.toLayerRect(2): not translated, its class isn't translated (it implements `IGuiParent`).
        // GuiLayer.GuiLayerDistination.getIntegratedParent(0): not translated, its class isn't translated (it implements `IGuiParent`).
    }

    /// `GuiParent` (team/creative/creativecore/common/gui/GuiParent.java)
    pub mod gui_parent {
        use super::*;
        // The class itself isn't translated: it extends `GuiControl`. Its static parts are below.
        // GuiParent.GuiParent(3): not translated, its class isn't translated (it extends `GuiControl`).
        // GuiParent.GuiParent(4): not translated, its class isn't translated (it extends `GuiControl`).
        // GuiParent.GuiParent(4): not translated, its class isn't translated (it extends `GuiControl`).
        // GuiParent.GuiParent(5): not translated, its class isn't translated (it extends `GuiControl`).
        // GuiParent.GuiParent(2): not translated, its class isn't translated (it extends `GuiControl`).
        // GuiParent.GuiParent(1): not translated, its class isn't translated (it extends `GuiControl`).
        // GuiParent.GuiParent(2): not translated, its class isn't translated (it extends `GuiControl`).
        // GuiParent.dist(0): not translated, its class isn't translated (it extends `GuiControl`).
        // GuiParent.isClient(0): not translated, its class isn't translated (it extends `GuiControl`).
        // GuiParent.setScale(1): not translated, its class isn't translated (it extends `GuiControl`).
        // GuiParent.setAlign(1): not translated, its class isn't translated (it extends `GuiControl`).
        // GuiParent.setVAlign(1): not translated, its class isn't translated (it extends `GuiControl`).
        // GuiParent.setSpacing(1): not translated, its class isn't translated (it extends `GuiControl`).
        // GuiParent.setFlow(1): not translated, its class isn't translated (it extends `GuiControl`).
        // GuiParent.isExpandableX(0): not translated, its class isn't translated (it extends `GuiControl`).
        // GuiParent.isExpandableY(0): not translated, its class isn't translated (it extends `GuiControl`).
        // GuiParent.get(1): not translated, it's generic.
        // GuiParent.get(2): not translated, it's generic.
        // GuiParent.has(1): not translated, its class isn't translated (it extends `GuiControl`).
        // GuiParent.insertControlBefore(2): not translated, it's generic.
        // GuiParent.insertControlAfter(2): not translated, it's generic.
        // GuiParent.contains(1): not translated, its class isn't translated (it extends `GuiControl`).
        // GuiParent.add(1): not translated, its class isn't translated (it extends `GuiControl`).
        // GuiParent.add(1): not translated, its class isn't translated (it extends `GuiControl`).
        // GuiParent.add(2): not translated, its class isn't translated (it extends `GuiControl`).
        // GuiParent.addHover(1): not translated, its class isn't translated (it extends `GuiControl`).
        // GuiParent.addHover(1): not translated, its class isn't translated (it extends `GuiControl`).
        // GuiParent.addHover(2): not translated, its class isn't translated (it extends `GuiControl`).
        // GuiParent.remove(1): not translated, its class isn't translated (it extends `GuiControl`).
        // GuiParent.remove(1): not translated, its class isn't translated (it extends `GuiControl`).
        // GuiParent.replace(2): not translated, its class isn't translated (it extends `GuiControl`).
        // GuiParent.remove(1): not translated, its class isn't translated (it extends `GuiControl`).
        // GuiParent.removeExclude(1): not translated, its class isn't translated (it extends `GuiControl`).
        // GuiParent.isEmpty(0): not translated, its class isn't translated (it extends `GuiControl`).
        // GuiParent.clear(0): not translated, its class isn't translated (it extends `GuiControl`).
        // GuiParent.get(1): not translated, it's generic.
        // GuiParent.totalSize(0): not translated, its class isn't translated (it extends `GuiControl`).
        // GuiParent.size(0): not translated, its class isn't translated (it extends `GuiControl`).
        // GuiParent.iterator(0): not translated, its class isn't translated (it extends `GuiControl`).
        // GuiParent.isContainer(0): not translated, its class isn't translated (it extends `GuiControl`).
        // GuiParent.init(0): not translated, its class isn't translated (it extends `GuiControl`).
        // GuiParent.closed(0): not translated, its class isn't translated (it extends `GuiControl`).
        // GuiParent.tick(0): not translated, its class isn't translated (it extends `GuiControl`).
        // GuiParent.closeTopLayer(0): not translated, its class isn't translated (it extends `GuiControl`).
        // GuiParent.closeThisLayer(0): not translated, its class isn't translated (it extends `GuiControl`).
        // GuiParent.closeLayer(1): not translated, its class isn't translated (it extends `GuiControl`).
        // GuiParent.raiseEvent(1): not translated, its class isn't translated (it extends `GuiControl`).
        // GuiParent.registerEventClick(1): not translated, its class isn't translated (it extends `GuiControl`).
        // GuiParent.registerEventChanged(1): not translated, its class isn't translated (it extends `GuiControl`).
        // GuiParent.registerEvent(2): not translated, it's generic.
        // GuiParent.clearEvents(0): not translated, its class isn't translated (it extends `GuiControl`).
        // GuiParent.getNestedName(0): not translated, its class isn't translated (it extends `GuiControl`).
        // GuiParent.toLayerRect(2): not translated, its class isn't translated (it extends `GuiControl`).
        // GuiParent.toScreenRect(2): not translated, its class isn't translated (it extends `GuiControl`).
        // GuiParent.createDist(1): not translated, its class isn't translated (it extends `GuiControl`).
        // GuiParent.createDist(1): not translated, its class isn't translated (it extends `GuiControl`).
    }

    /// `GuiParent.GuiParentDistHandler` (team/creative/creativecore/common/gui/GuiParent.java)
    pub mod gui_parent_gui_parent_dist_handler {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // GuiParent.GuiParentDistHandler.setScale(1): not translated, it has no body.
        // GuiParent.GuiParentDistHandler.setAlign(1): not translated, it has no body.
        // GuiParent.GuiParentDistHandler.setVAlign(1): not translated, it has no body.
        // GuiParent.GuiParentDistHandler.setSpacing(1): not translated, it has no body.
        // GuiParent.GuiParentDistHandler.setFlow(1): not translated, it has no body.
        // GuiParent.GuiParentDistHandler.initControlList(1): not translated, it has no body.
        // GuiParent.GuiParentDistHandler.applyOffset(2): not translated, it has no body.
        // GuiParent.GuiParentDistHandler.controls(0): not translated, it has no body.
        // GuiParent.GuiParentDistHandler.hoverControls(0): not translated, it has no body.
        // GuiParent.GuiParentDistHandler.all(0): not translated, it has no body.
    }

    /// `IGuiParent` (team/creative/creativecore/common/gui/IGuiParent.java)
    pub mod i_gui_parent {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IGuiParent.isContainer(0): not translated, it has no body.
        // IGuiParent.isClient(0): not translated, it has no body.
        // IGuiParent.getPlayer(0): not translated, it has no body.
        // IGuiParent.closeTopLayer(0): not translated, it has no body.
        // IGuiParent.closeLayer(1): not translated, it has no body.
        // IGuiParent.raiseEvent(1): not translated, it has no body.
        // IGuiParent.reflow(0): not translated, it has no body.
        // IGuiParent.hasGui(0): not translated, it has no body.
        // IGuiParent.isParent(1): not translated, it has no body.
        // IGuiParent.toScreenRect(2): not translated, it has no body.
        // IGuiParent.toLayerRect(2): not translated, it has no body.
        // IGuiParent.getIntegratedParent(0): not translated, it has no body.
        // IGuiParent.createDist(1): not translated, it has no body.
        // IGuiParent.createDist(1): not translated, it has no body.
    }

    /// `ContainerIntegration` (team/creative/creativecore/common/gui/integration/ContainerIntegration.java)
    pub mod container_integration {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `AbstractContainerMenu`. Its static parts are below.
        // EMPTY_CLIENT: not translated, it uses GuiLayer, which isn't translated (it extends `GuiParent`).
        // EMPTY_SERVER: not translated, its value couldn't be read: an anonymous class.
        // ContainerIntegration.ContainerIntegration(4): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerMenu`).
        // ContainerIntegration.ContainerIntegration(3): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerMenu`).
        // ContainerIntegration.setScreen(1): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerMenu`).
        // ContainerIntegration.broadcastChanges(0): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerMenu`).
        // ContainerIntegration.tick(0): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerMenu`).
        // ContainerIntegration.getLayers(0): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerMenu`).
        // ContainerIntegration.getTopLayer(0): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerMenu`).
        // ContainerIntegration.isContainer(0): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerMenu`).
        // ContainerIntegration.removed(1): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerMenu`).
        // ContainerIntegration.stillValid(1): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerMenu`).
        // ContainerIntegration.getPlayer(0): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerMenu`).
        // ContainerIntegration.isClient(0): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerMenu`).
        // ContainerIntegration.openLayer(1): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerMenu`).
        // ContainerIntegration.closeTopLayer(0): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerMenu`).
        // ContainerIntegration.closeLayer(1): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerMenu`).
        // ContainerIntegration.closeLayer(1): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerMenu`).
        // ContainerIntegration.send(1): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerMenu`).
        // ContainerIntegration.quickMoveStack(2): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerMenu`).
        // ContainerIntegration.provider(0): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerMenu`).
        // ContainerIntegration.createDist(1): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerMenu`).
        // ContainerIntegration.createDist(1): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerMenu`).
        // ContainerIntegration.toScreenRect(2): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerMenu`).
    }

    /// `GuiEventHandler` (team/creative/creativecore/common/gui/integration/GuiEventHandler.java)
    pub mod gui_event_handler {
        use super::*;
        pub static DISPLAY_WIDTH: JCell<i32> = JCell::new(0i32);
        pub static DISPLAY_HEIGHT: JCell<i32> = JCell::new(0i32);
        pub static DEFAULT_SCALE: JCell<i32> = JCell::new(0i32);
        pub static CHANGED: JCell<bool> = JCell::new(false);
        // displayScreen: not translated, it uses Minecraft's `Screen`.
        #[derive(Clone, Debug)]
        pub struct GuiEventHandler {
        }
        impl GuiEventHandler {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // GuiEventHandler.queueScreen(1): not translated, it uses Minecraft's `Screen`.
        // GuiEventHandler.onTick(1): not translated, it uses Minecraft's `Minecraft`.
    }

    /// `IGuiIntegratedParent` (team/creative/creativecore/common/gui/integration/IGuiIntegratedParent.java)
    pub mod i_gui_integrated_parent {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IGuiIntegratedParent.provider(0): not translated, it has no body.
        // IGuiIntegratedParent.getLayers(0): not translated, it has no body.
        // IGuiIntegratedParent.getTopLayer(0): not translated, it has no body.
        // IGuiIntegratedParent.isOpen(1): not translated, its class isn't translated (it's an interface).
        // IGuiIntegratedParent.raiseEvent(1): not translated, its class isn't translated (it's an interface).
        // IGuiIntegratedParent.reflow(0): not translated, its class isn't translated (it's an interface).
        // IGuiIntegratedParent.openLayer(1): not translated, it has no body.
        // IGuiIntegratedParent.closeLayer(1): not translated, it has no body.
        // IGuiIntegratedParent.isParent(1): not translated, its class isn't translated (it's an interface).
        // IGuiIntegratedParent.hasGui(0): not translated, its class isn't translated (it's an interface).
        // IGuiIntegratedParent.get(1): not translated, its class isn't translated (it's an interface).
        // IGuiIntegratedParent.toLayerRect(2): not translated, its class isn't translated (it's an interface).
        // IGuiIntegratedParent.getIntegratedParent(0): not translated, its class isn't translated (it's an interface).
        // IGuiIntegratedParent.send(1): not translated, it has no body.
    }

    /// `IScaleableGuiScreen` (team/creative/creativecore/common/gui/IScaleableGuiScreen.java)
    pub mod i_scaleable_gui_screen {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IScaleableGuiScreen.getWidth(0): not translated, it has no body.
        // IScaleableGuiScreen.getHeight(0): not translated, it has no body.
        // IScaleableGuiScreen.getMaxScale(2): not translated, its class isn't translated (it's an interface).
        // IScaleableGuiScreen.clientTick(0): not translated, it has no body.
    }

    /// `GuiManager` (team/creative/creativecore/common/gui/manager/GuiManager.java)
    pub mod gui_manager {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // REGISTRY: not translated, it uses NamedHandlerRegistry, which isn't translated (it's generic).
        // ITEM: not translated, it uses GuiManagerType, which isn't translated (it's generic).
        // GuiManager.register(3): not translated, it's generic.
        // GuiManager.GuiManager(1): not translated, its class isn't translated (it's generic).
        // GuiManager.tick(0): not translated, its class isn't translated (it's generic).
        // GuiManager.closed(0): not translated, its class isn't translated (it's generic).
    }

    /// `GuiManager.GuiManagerType` (team/creative/creativecore/common/gui/manager/GuiManager.java)
    pub mod gui_manager_gui_manager_type {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GuiManager.GuiManagerType.GuiManagerType(3): not translated, its class isn't translated (it's generic).
        // GuiManager.GuiManagerType.name(0): not translated, its class isn't translated (it's generic).
        // GuiManager.GuiManagerType.managerClass(0): not translated, its class isn't translated (it's generic).
        // GuiManager.GuiManagerType.factory(0): not translated, its class isn't translated (it's generic).
    }

    /// `GuiManagerDist` (team/creative/creativecore/common/gui/manager/GuiManagerDist.java)
    pub mod gui_manager_dist {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
    }

    /// `GuiManagerItem` (team/creative/creativecore/common/gui/manager/GuiManagerItem.java)
    pub mod gui_manager_item {
        use super::*;
        // The class itself isn't translated: it extends `GuiManager`. Its static parts are below.
        // GuiManagerItem.GuiManagerItem(1): not translated, its class isn't translated (it extends `GuiManager`).
        // GuiManagerItem.tick(0): not translated, its class isn't translated (it extends `GuiManager`).
        // GuiManagerItem.closed(0): not translated, its class isn't translated (it extends `GuiManager`).
        // GuiManagerItem.getHand(0): not translated, its class isn't translated (it extends `GuiManager`).
        // GuiManagerItem.setHandChanged(0): not translated, its class isn't translated (it extends `GuiManager`).
        // GuiManagerItem.setHand(1): not translated, its class isn't translated (it extends `GuiManager`).
    }

    /// `ControlSyncPacket` (team/creative/creativecore/common/gui/packet/ControlSyncPacket.java)
    pub mod control_sync_packet {
        use super::*;
        // The class itself isn't translated: it extends `LayerPacket`. Its static parts are below.
        // ControlSyncPacket.ControlSyncPacket(3): not translated, its class isn't translated (it extends `LayerPacket`).
        // ControlSyncPacket.ControlSyncPacket(0): not translated, its class isn't translated (it extends `LayerPacket`).
        // ControlSyncPacket.execute(2): not translated, its class isn't translated (it extends `LayerPacket`).
    }

    /// `ImmediateItemStackPacket` (team/creative/creativecore/common/gui/packet/ImmediateItemStackPacket.java)
    pub mod immediate_item_stack_packet {
        use super::*;
        // The class itself isn't translated: it extends `CreativePacket` (team.creative.creativecore.common.network). Its static parts are below.
        // ImmediateItemStackPacket.ImmediateItemStackPacket(1): not translated, its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)).
        // ImmediateItemStackPacket.ImmediateItemStackPacket(2): not translated, its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)).
        // ImmediateItemStackPacket.ImmediateItemStackPacket(0): not translated, its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)).
        // ImmediateItemStackPacket.executeClient(1): not translated, its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)).
        // ImmediateItemStackPacket.executeServer(1): not translated, its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)).
    }

    /// `LayerClosePacket` (team/creative/creativecore/common/gui/packet/LayerClosePacket.java)
    pub mod layer_close_packet {
        use super::*;
        // The class itself isn't translated: it extends `LayerPacket`. Its static parts are below.
        // LayerClosePacket.LayerClosePacket(1): not translated, its class isn't translated (it extends `LayerPacket`).
        // LayerClosePacket.LayerClosePacket(0): not translated, its class isn't translated (it extends `LayerPacket`).
        // LayerClosePacket.execute(2): not translated, its class isn't translated (it extends `LayerPacket`).
    }

    /// `LayerPacket` (team/creative/creativecore/common/gui/packet/LayerPacket.java)
    pub mod layer_packet {
        use super::*;
        // The class itself isn't translated: it extends `CreativePacket` (team.creative.creativecore.common.network). Its static parts are below.
        // LayerPacket.executeClient(1): not translated, its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)).
        // LayerPacket.executeServer(1): not translated, its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)).
        // LayerPacket.execute(2): not translated, it has no body.
    }

    /// `OpenGuiPacket` (team/creative/creativecore/common/gui/packet/OpenGuiPacket.java)
    pub mod open_gui_packet {
        use super::*;
        // The class itself isn't translated: it extends `CreativePacket` (team.creative.creativecore.common.network). Its static parts are below.
        // OpenGuiPacket.OpenGuiPacket(0): not translated, its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)).
        // OpenGuiPacket.OpenGuiPacket(2): not translated, its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)).
        // OpenGuiPacket.executeClient(1): not translated, its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)).
        // OpenGuiPacket.executeServer(1): not translated, its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)).
        // OpenGuiPacket.openGuiOnServer(3): not translated, it uses GuiCreator, which isn't translated (it uses Minecraft's `CompoundTag`).
    }

    /// `SyncPacket` (team/creative/creativecore/common/gui/packet/SyncPacket.java)
    pub mod sync_packet {
        use super::*;
        // The class itself isn't translated: it extends `LayerPacket`. Its static parts are below.
        // SyncPacket.SyncPacket(2): not translated, its class isn't translated (it extends `LayerPacket`).
        // SyncPacket.SyncPacket(0): not translated, its class isn't translated (it extends `LayerPacket`).
        // SyncPacket.execute(2): not translated, its class isn't translated (it extends `LayerPacket`).
    }

    /// `DoubleValueParser` (team/creative/creativecore/common/gui/parser/DoubleValueParser.java)
    pub mod double_value_parser {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // DoubleValueParser.parse(2): not translated, it has no body.
    }

    /// `IntValueParser` (team/creative/creativecore/common/gui/parser/IntValueParser.java)
    pub mod int_value_parser {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IntValueParser.parse(2): not translated, it has no body.
    }

    /// `LongValueParser` (team/creative/creativecore/common/gui/parser/LongValueParser.java)
    pub mod long_value_parser {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // LongValueParser.parse(2): not translated, it has no body.
    }

    /// `ShortValueParser` (team/creative/creativecore/common/gui/parser/ShortValueParser.java)
    pub mod short_value_parser {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ShortValueParser.parse(2): not translated, it has no body.
    }

    /// `ControlFormatting` (team/creative/creativecore/common/gui/style/ControlFormatting.java)
    pub mod control_formatting {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ControlFormatting.border(0): not translated, it has no body.
        // ControlFormatting.padding(0): not translated, it has no body.
        // ControlFormatting.face(0): not translated, it has no body.
        // ControlFormatting.hasDisabledEffect(0): not translated, it has no body.
    }

    /// `ControlFormatting.ControlFormattingStatic` (team/creative/creativecore/common/gui/style/ControlFormatting.java)
    pub mod control_formatting_control_formatting_static {
        use super::*;
        // The class itself isn't translated: it implements `ControlFormatting`. Its static parts are below.
        // ControlFormatting.ControlFormattingStatic.ControlFormattingStatic(3): not translated, its class isn't translated (it implements `ControlFormatting`).
        // ControlFormatting.ControlFormattingStatic.ControlFormattingStatic(4): not translated, its class isn't translated (it implements `ControlFormatting`).
        // ControlFormatting.ControlFormattingStatic.border(0): not translated, its class isn't translated (it implements `ControlFormatting`).
        // ControlFormatting.ControlFormattingStatic.padding(0): not translated, its class isn't translated (it implements `ControlFormatting`).
        // ControlFormatting.ControlFormattingStatic.face(0): not translated, its class isn't translated (it implements `ControlFormatting`).
        // ControlFormatting.ControlFormattingStatic.hasDisabledEffect(0): not translated, its class isn't translated (it implements `ControlFormatting`).
    }

    /// `ControlFormatting.ControlStyleBorder` (team/creative/creativecore/common/gui/style/ControlFormatting.java)
    pub mod control_formatting_control_style_border {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum ControlStyleBorder {
            Big,
            Small,
            None,
        }
        impl ControlStyleBorder {
            pub const VALUES: [ControlStyleBorder; 3] = [ControlStyleBorder::Big, ControlStyleBorder::Small, ControlStyleBorder::None, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    ControlStyleBorder::Big => "BIG",
                    ControlStyleBorder::Small => "SMALL",
                    ControlStyleBorder::None => "NONE",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "BIG" => ControlStyleBorder::Big,
                    "SMALL" => ControlStyleBorder::Small,
                    "NONE" => ControlStyleBorder::None,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `ControlFormatting.ControlStyleFace` (team/creative/creativecore/common/gui/style/ControlFormatting.java)
    pub mod control_formatting_control_style_face {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum ControlStyleFace {
            Bar,
            Clickable,
            ClickableInactive,
            HeaderBackground,
            NestedBackground,
            Background,
            Slot,
            None,
            Disabled,
        }
        impl ControlStyleFace {
            pub const VALUES: [ControlStyleFace; 9] = [ControlStyleFace::Bar, ControlStyleFace::Clickable, ControlStyleFace::ClickableInactive, ControlStyleFace::HeaderBackground, ControlStyleFace::NestedBackground, ControlStyleFace::Background, ControlStyleFace::Slot, ControlStyleFace::None, ControlStyleFace::Disabled, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    ControlStyleFace::Bar => "BAR",
                    ControlStyleFace::Clickable => "CLICKABLE",
                    ControlStyleFace::ClickableInactive => "CLICKABLE_INACTIVE",
                    ControlStyleFace::HeaderBackground => "HEADER_BACKGROUND",
                    ControlStyleFace::NestedBackground => "NESTED_BACKGROUND",
                    ControlStyleFace::Background => "BACKGROUND",
                    ControlStyleFace::Slot => "SLOT",
                    ControlStyleFace::None => "NONE",
                    ControlStyleFace::Disabled => "DISABLED",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "BAR" => ControlStyleFace::Bar,
                    "CLICKABLE" => ControlStyleFace::Clickable,
                    "CLICKABLE_INACTIVE" => ControlStyleFace::ClickableInactive,
                    "HEADER_BACKGROUND" => ControlStyleFace::HeaderBackground,
                    "NESTED_BACKGROUND" => ControlStyleFace::NestedBackground,
                    "BACKGROUND" => ControlStyleFace::Background,
                    "SLOT" => ControlStyleFace::Slot,
                    "NONE" => ControlStyleFace::None,
                    "DISABLED" => ControlStyleFace::Disabled,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `ControlFormattingFlexible` (team/creative/creativecore/common/gui/style/ControlFormattingFlexible.java)
    pub mod control_formatting_flexible {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ControlFormattingFlexible.setBorder(1): not translated, it has no body.
        // ControlFormattingFlexible.setPadding(1): not translated, it has no body.
        // ControlFormattingFlexible.setFace(1): not translated, it has no body.
        // ControlFormattingFlexible.setDisabledEffect(1): not translated, it has no body.
    }

    /// `ControlFormattingFlexible.ControlFormattingFlexibleEmpty` (team/creative/creativecore/common/gui/style/ControlFormattingFlexible.java)
    pub mod control_formatting_flexible_control_formatting_flexible_empty {
        use super::*;
        // The class itself isn't translated: it implements `ControlFormattingFlexible`. Its static parts are below.
        // ControlFormattingFlexible.ControlFormattingFlexibleEmpty.border(0): not translated, its class isn't translated (it implements `ControlFormattingFlexible`).
        // ControlFormattingFlexible.ControlFormattingFlexibleEmpty.padding(0): not translated, its class isn't translated (it implements `ControlFormattingFlexible`).
        // ControlFormattingFlexible.ControlFormattingFlexibleEmpty.face(0): not translated, its class isn't translated (it implements `ControlFormattingFlexible`).
        // ControlFormattingFlexible.ControlFormattingFlexibleEmpty.hasDisabledEffect(0): not translated, its class isn't translated (it implements `ControlFormattingFlexible`).
        // ControlFormattingFlexible.ControlFormattingFlexibleEmpty.setBorder(1): not translated, its class isn't translated (it implements `ControlFormattingFlexible`).
        // ControlFormattingFlexible.ControlFormattingFlexibleEmpty.setPadding(1): not translated, its class isn't translated (it implements `ControlFormattingFlexible`).
        // ControlFormattingFlexible.ControlFormattingFlexibleEmpty.setFace(1): not translated, its class isn't translated (it implements `ControlFormattingFlexible`).
        // ControlFormattingFlexible.ControlFormattingFlexibleEmpty.setDisabledEffect(1): not translated, its class isn't translated (it implements `ControlFormattingFlexible`).
    }

    /// `ControlFormattingFlexible.ControlFormattingFlexibleImpl` (team/creative/creativecore/common/gui/style/ControlFormattingFlexible.java)
    pub mod control_formatting_flexible_control_formatting_flexible_impl {
        use super::*;
        // The class itself isn't translated: it implements `ControlFormattingFlexible`. Its static parts are below.
        // ControlFormattingFlexible.ControlFormattingFlexibleImpl.ControlFormattingFlexibleImpl(1): not translated, its class isn't translated (it implements `ControlFormattingFlexible`).
        // ControlFormattingFlexible.ControlFormattingFlexibleImpl.setBorder(1): not translated, its class isn't translated (it implements `ControlFormattingFlexible`).
        // ControlFormattingFlexible.ControlFormattingFlexibleImpl.setPadding(1): not translated, its class isn't translated (it implements `ControlFormattingFlexible`).
        // ControlFormattingFlexible.ControlFormattingFlexibleImpl.setFace(1): not translated, its class isn't translated (it implements `ControlFormattingFlexible`).
        // ControlFormattingFlexible.ControlFormattingFlexibleImpl.setDisabledEffect(1): not translated, its class isn't translated (it implements `ControlFormattingFlexible`).
        // ControlFormattingFlexible.ControlFormattingFlexibleImpl.border(0): not translated, its class isn't translated (it implements `ControlFormattingFlexible`).
        // ControlFormattingFlexible.ControlFormattingFlexibleImpl.padding(0): not translated, its class isn't translated (it implements `ControlFormattingFlexible`).
        // ControlFormattingFlexible.ControlFormattingFlexibleImpl.face(0): not translated, its class isn't translated (it implements `ControlFormattingFlexible`).
        // ControlFormattingFlexible.ControlFormattingFlexibleImpl.hasDisabledEffect(0): not translated, its class isn't translated (it implements `ControlFormattingFlexible`).
    }

    /// `DisplayColor` (team/creative/creativecore/common/gui/style/display/DisplayColor.java)
    pub mod display_color {
        use super::*;
        // The class itself isn't translated: it extends `StyleDisplay`. Its static parts are below.
        // DisplayColor.DisplayColor(0): not translated, its class isn't translated (it extends `StyleDisplay`).
        // DisplayColor.DisplayColor(1): not translated, its class isn't translated (it extends `StyleDisplay`).
        // DisplayColor.DisplayColor(4): not translated, its class isn't translated (it extends `StyleDisplay`).
        // DisplayColor.DisplayColor(1): not translated, its class isn't translated (it extends `StyleDisplay`).
        // DisplayColor.set(1): not translated, its class isn't translated (it extends `StyleDisplay`).
        // DisplayColor.DisplayColor(4): not translated, its class isn't translated (it extends `StyleDisplay`).
        // DisplayColor.set(4): not translated, its class isn't translated (it extends `StyleDisplay`).
        // DisplayColor.set(1): not translated, its class isn't translated (it extends `StyleDisplay`).
        // DisplayColor.render(5): not translated, its class isn't translated (it extends `StyleDisplay`).
    }

    /// `DisplayTexture` (team/creative/creativecore/common/gui/style/display/DisplayTexture.java)
    pub mod display_texture {
        use super::*;
        // The class itself isn't translated: it extends `StyleDisplay`. Its static parts are below.
        // DisplayTexture.DisplayTexture(0): not translated, its class isn't translated (it extends `StyleDisplay`).
        // DisplayTexture.DisplayTexture(3): not translated, its class isn't translated (it extends `StyleDisplay`).
        // DisplayTexture.render(5): not translated, its class isn't translated (it extends `StyleDisplay`).
    }

    /// `DisplayTextureRepeat` (team/creative/creativecore/common/gui/style/display/DisplayTextureRepeat.java)
    pub mod display_texture_repeat {
        use super::*;
        // The class itself isn't translated: it extends `DisplayTexture`. Its static parts are below.
        // DisplayTextureRepeat.DisplayTextureRepeat(0): not translated, its class isn't translated (it extends `DisplayTexture`).
        // DisplayTextureRepeat.DisplayTextureRepeat(5): not translated, its class isn't translated (it extends `DisplayTexture`).
        // DisplayTextureRepeat.render(5): not translated, its class isn't translated (it extends `DisplayTexture`).
    }

    /// `DisplayTextureStretch` (team/creative/creativecore/common/gui/style/display/DisplayTextureStretch.java)
    pub mod display_texture_stretch {
        use super::*;
        // The class itself isn't translated: it extends `DisplayTexture`. Its static parts are below.
        // DisplayTextureStretch.DisplayTextureStretch(0): not translated, its class isn't translated (it extends `DisplayTexture`).
        // DisplayTextureStretch.DisplayTextureStretch(5): not translated, its class isn't translated (it extends `DisplayTexture`).
        // DisplayTextureStretch.render(5): not translated, its class isn't translated (it extends `DisplayTexture`).
    }

    /// `StyleDisplay` (team/creative/creativecore/common/gui/style/display/StyleDisplay.java)
    pub mod style_display {
        use super::*;
        // NONE: not translated, its value couldn't be read: an anonymous class.
        // types: not translated, it uses `Class`.
        #[derive(Clone, Debug)]
        pub struct StyleDisplay {
        }
        impl StyleDisplay {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // StyleDisplay.render(3): not translated, it's overloaded with the same number of parameters.
            // StyleDisplay.render(3): not translated, it's overloaded with the same number of parameters.
            // StyleDisplay.render(2): not translated, it uses Minecraft's `GuiGraphicsExtractor`.
            // StyleDisplay.render(5): not translated, it has no body.
        }
        // StyleDisplay.registerType(2): not translated, it uses `Class`.
    }

    /// `StyleDisplay.StyleDisplayDeserializer` (team/creative/creativecore/common/gui/style/display/StyleDisplay.java)
    pub mod style_display_style_display_deserializer {
        use super::*;
        // The class itself isn't translated: it implements `JsonDeserializer` (com.google.gson). Its static parts are below.
        // StyleDisplay.StyleDisplayDeserializer.deserialize(3): not translated, its class isn't translated (it implements `JsonDeserializer` (com.google.gson)).
    }

    /// `GuiStyle` (team/creative/creativecore/common/gui/style/GuiStyle.java)
    pub mod gui_style {
        use super::*;
        // mc: not translated, it uses Minecraft's `Minecraft`.
        pub static GSON: LazyLock<JGson> = LazyLock::new(|| JGson::default());
        // REGISTRY: not translated, it uses NamedHandlerRegistry, which isn't translated (it's generic).
        // LOADED: not translated, it uses `Object2BooleanMap` (it.unimi.dsi.fastutil.objects).
        #[derive(Clone, Debug)]
        pub struct GuiStyle {
            pub font_color: color::Color,
            pub font_color_highlight: color::Color,
            pub font_color_disabled: color::Color,
            pub border_width: i32,
            pub border_thick_width: i32,
            pub disabled: style_display::StyleDisplay,
            pub border: style_display::StyleDisplay,
            pub border_thick: style_display::StyleDisplay,
            pub background: style_display::StyleDisplay,
            pub secondary_background: style_display::StyleDisplay,
            pub header_background: style_display::StyleDisplay,
            pub bar: style_display::StyleDisplay,
            pub clickable: style_display::StyleDisplay,
            pub clickable_highlight: style_display::StyleDisplay,
            pub clickable_inactive: style_display::StyleDisplay,
            pub clickable_inactive_highlight: style_display::StyleDisplay,
            pub disabled_background: style_display::StyleDisplay,
            pub slot: style_display::StyleDisplay,
            pub transparency_background: style_display::StyleDisplay,
        }
        impl GuiStyle {
            // GuiStyle.get(1): not translated, it uses StyleDisplay.NONE, which isn't translated (its value couldn't be read: an anonymous class).
            // GuiStyle.get(2): not translated, it uses StyleDisplay.NONE, which isn't translated (its value couldn't be read: an anonymous class).
            pub fn get_border(&self, border: control_formatting_control_style_border::ControlStyleBorder) -> i32 {
                return match border {
                    control_formatting_control_style_border::ControlStyleBorder::Big => self.border_thick_width,
                    control_formatting_control_style_border::ControlStyleBorder::Small => self.border_width,
                    _ => 0,
                };
            }
            // GuiStyle.getContentOffset(1): not translated, it uses ControlFormatting, which isn't translated (it's an interface).
        }
        // GuiStyle.reload(0): not translated, it uses GuiStyle.LOADED, which isn't translated (it uses `Object2BooleanMap` (it.unimi.dsi.fastutil.objects)).
        // GuiStyle.getStyle(1): not translated, it uses GuiStyle.REGISTRY, which isn't translated (it uses NamedHandlerRegistry, which isn't translated (it's generic)).
    }

    /// `GuiStyleUtils` (team/creative/creativecore/common/gui/style/GuiStyleUtils.java)
    pub mod gui_style_utils {
        use super::*;
        pub static DEFAULT_STYLE_LOCATION: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::try_build("creativecore", "gui/default_style.json").expect("null"));
        pub static GUI_ASSETS: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::try_build("creativecore", "textures/gui_assets.png").expect("null"));
        #[derive(Clone, Debug)]
        pub struct GuiStyleUtils {
        }
        impl GuiStyleUtils {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `Icon` (team/creative/creativecore/common/gui/style/Icon.java)
    pub mod icon {
        use super::*;
        pub static EMPTY: LazyLock<icon::Icon> = LazyLock::new(|| icon::Icon::new(&(*gui_style_utils::GUI_ASSETS), 224, 224, 16, 16));
        pub static ARROW_RIGHT: LazyLock<icon::Icon> = LazyLock::new(|| icon::Icon::new(&(*gui_style_utils::GUI_ASSETS), 240, 0, 16, 16));
        pub static ARROW_UP: LazyLock<icon::Icon> = LazyLock::new(|| icon::Icon::new(&(*gui_style_utils::GUI_ASSETS), 240, 16, 16, 16));
        pub static ARROW_LEFT: LazyLock<icon::Icon> = LazyLock::new(|| icon::Icon::new(&(*gui_style_utils::GUI_ASSETS), 240, 32, 16, 16));
        pub static ARROW_DOWN: LazyLock<icon::Icon> = LazyLock::new(|| icon::Icon::new(&(*gui_style_utils::GUI_ASSETS), 240, 48, 16, 16));
        pub static MIRROR: LazyLock<icon::Icon> = LazyLock::new(|| icon::Icon::new(&(*gui_style_utils::GUI_ASSETS), 240, 64, 16, 16));
        pub static ARROW_OUT: LazyLock<icon::Icon> = LazyLock::new(|| icon::Icon::new(&(*gui_style_utils::GUI_ASSETS), 240, 80, 16, 16));
        pub static ARROW_IN: LazyLock<icon::Icon> = LazyLock::new(|| icon::Icon::new(&(*gui_style_utils::GUI_ASSETS), 240, 96, 16, 16));
        pub static COORDS: LazyLock<icon::Icon> = LazyLock::new(|| icon::Icon::new(&(*gui_style_utils::GUI_ASSETS), 240, 112, 16, 16));
        pub static HOUSE: LazyLock<icon::Icon> = LazyLock::new(|| icon::Icon::new(&(*gui_style_utils::GUI_ASSETS), 240, 128, 16, 16));
        pub static PAUSE: LazyLock<icon::Icon> = LazyLock::new(|| icon::Icon::new(&(*gui_style_utils::GUI_ASSETS), 240, 144, 16, 16));
        pub static PLAY: LazyLock<icon::Icon> = LazyLock::new(|| icon::Icon::new(&(*gui_style_utils::GUI_ASSETS), 240, 160, 16, 16));
        pub static STOP: LazyLock<icon::Icon> = LazyLock::new(|| icon::Icon::new(&(*gui_style_utils::GUI_ASSETS), 240, 176, 16, 16));
        pub static DUPLICATE: LazyLock<icon::Icon> = LazyLock::new(|| icon::Icon::new(&(*gui_style_utils::GUI_ASSETS), 240, 192, 16, 16));
        pub static MERGE: LazyLock<icon::Icon> = LazyLock::new(|| icon::Icon::new(&(*gui_style_utils::GUI_ASSETS), 240, 208, 16, 16));
        pub static MOVE: LazyLock<icon::Icon> = LazyLock::new(|| icon::Icon::new(&(*gui_style_utils::GUI_ASSETS), 240, 224, 16, 16));
        pub static CAMERA: LazyLock<icon::Icon> = LazyLock::new(|| icon::Icon::new(&(*gui_style_utils::GUI_ASSETS), 240, 240, 16, 16));
        #[derive(Clone, Debug, PartialEq)]
        pub struct Icon {
            pub identifier: JIdentifier,
            pub min_x: i32,
            pub min_y: i32,
            pub width: i32,
            pub height: i32,
        }
        impl Icon {
            pub fn new(identifier: &JIdentifier, min_x: i32, min_y: i32, width: i32, height: i32) -> Self {
                let mut s = Self { identifier: identifier.clone(), min_x: min_x, min_y: min_y, width: width, height: height };
                s.identifier = identifier.clone();
                s.min_x = min_x;
                s.min_y = min_y;
                s.width = width;
                s.height = height;
                s
            }
            pub fn identifier(&self) -> JIdentifier {
                return self.identifier.clone();
            }
            pub fn min_x(&self) -> i32 {
                return self.min_x;
            }
            pub fn min_y(&self) -> i32 {
                return self.min_y;
            }
            pub fn width(&self) -> i32 {
                return self.width;
            }
            pub fn height(&self) -> i32 {
                return self.height;
            }
        }
    }

    /// `GuiSync` (team/creative/creativecore/common/gui/sync/GuiSync.java)
    pub mod gui_sync {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GuiSync.GuiSync(2): not translated, its class isn't translated (it's generic).
        // GuiSync.receive(2): not translated, it has no body.
        // GuiSync.syncPath(0): not translated, its class isn't translated (it's generic).
    }

    /// `GuiSyncControl` (team/creative/creativecore/common/gui/sync/GuiSyncControl.java)
    pub mod gui_sync_control {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GuiSyncControl.GuiSyncControl(2): not translated, its class isn't translated (it's generic).
        // GuiSyncControl.receive(2): not translated, it has no body.
        // GuiSyncControl.syncPath(0): not translated, its class isn't translated (it's generic).
    }

    /// `GuiSyncGlobal` (team/creative/creativecore/common/gui/sync/GuiSyncGlobal.java)
    pub mod gui_sync_global {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GuiSyncGlobal.GuiSyncGlobal(3): not translated, its class isn't translated (it's generic).
        // GuiSyncGlobal.receive(2): not translated, its class isn't translated (it's generic).
        // GuiSyncGlobal.send(2): not translated, its class isn't translated (it's generic).
        // GuiSyncGlobal.sendAndExecute(2): not translated, its class isn't translated (it's generic).
    }

    /// `GuiSyncGlobalLayer` (team/creative/creativecore/common/gui/sync/GuiSyncGlobalLayer.java)
    pub mod gui_sync_global_layer {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GuiSyncGlobalLayer.GuiSyncGlobalLayer(3): not translated, its class isn't translated (it's generic).
        // GuiSyncGlobalLayer.receive(2): not translated, its class isn't translated (it's generic).
        // GuiSyncGlobalLayer.open(2): not translated, its class isn't translated (it's generic).
    }

    /// `GuiSyncHolder` (team/creative/creativecore/common/gui/sync/GuiSyncHolder.java)
    pub mod gui_sync_holder {
        use super::*;
        // GLOBAL: not translated, it uses GuiSyncHolderGlobal, which isn't translated (it extends `GuiSyncHolder`).
        #[derive(Clone, Debug)]
        pub struct GuiSyncHolder {
        }
        impl GuiSyncHolder {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // GuiSyncHolder.path(0): not translated, it has no body.
            // GuiSyncHolder.getSync(1): not translated, it has no body.
            // GuiSyncHolder.getControlSync(1): not translated, it has no body.
        }
        // GuiSyncHolder.followPath(2): not translated, it uses GuiSync, which isn't translated (it's generic).
        // GuiSyncHolder.followPathControl(2): not translated, it uses GuiSyncControl, which isn't translated (it's generic).
    }

    /// `GuiSyncHolder.GuiSyncHolderGlobal` (team/creative/creativecore/common/gui/sync/GuiSyncHolder.java)
    pub mod gui_sync_holder_gui_sync_holder_global {
        use super::*;
        // The class itself isn't translated: it extends `GuiSyncHolder`. Its static parts are below.
        // GuiSyncHolder.GuiSyncHolderGlobal.register(2): not translated, it's generic.
        // GuiSyncHolder.GuiSyncHolderGlobal.layer(2): not translated, it's generic.
        // GuiSyncHolder.GuiSyncHolderGlobal.path(0): not translated, its class isn't translated (it extends `GuiSyncHolder`).
        // GuiSyncHolder.GuiSyncHolderGlobal.getSync(1): not translated, its class isn't translated (it extends `GuiSyncHolder`).
        // GuiSyncHolder.GuiSyncHolderGlobal.getControlSync(1): not translated, its class isn't translated (it extends `GuiSyncHolder`).
    }

    /// `GuiSyncHolder.GuiSyncHolderLayer` (team/creative/creativecore/common/gui/sync/GuiSyncHolder.java)
    pub mod gui_sync_holder_gui_sync_holder_layer {
        use super::*;
        // The class itself isn't translated: it extends `GuiSyncHolder`. Its static parts are below.
        // GuiSyncHolder.GuiSyncHolderLayer.GuiSyncHolderLayer(1): not translated, its class isn't translated (it extends `GuiSyncHolder`).
        // GuiSyncHolder.GuiSyncHolderLayer.register(2): not translated, it's generic.
        // GuiSyncHolder.GuiSyncHolderLayer.layer(2): not translated, it's generic.
        // GuiSyncHolder.GuiSyncHolderLayer.path(0): not translated, its class isn't translated (it extends `GuiSyncHolder`).
        // GuiSyncHolder.GuiSyncHolderLayer.getSync(1): not translated, its class isn't translated (it extends `GuiSyncHolder`).
        // GuiSyncHolder.GuiSyncHolderLayer.getControlSync(1): not translated, its class isn't translated (it extends `GuiSyncHolder`).
    }

    /// `GuiSyncLocal` (team/creative/creativecore/common/gui/sync/GuiSyncLocal.java)
    pub mod gui_sync_local {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GuiSyncLocal.GuiSyncLocal(3): not translated, its class isn't translated (it's generic).
        // GuiSyncLocal.receive(2): not translated, its class isn't translated (it's generic).
        // GuiSyncLocal.send(1): not translated, its class isn't translated (it's generic).
        // GuiSyncLocal.sendAndExecute(2): not translated, its class isn't translated (it's generic).
    }

    /// `GuiSyncLocalLayer` (team/creative/creativecore/common/gui/sync/GuiSyncLocalLayer.java)
    pub mod gui_sync_local_layer {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GuiSyncLocalLayer.GuiSyncLocalLayer(3): not translated, its class isn't translated (it's generic).
        // GuiSyncLocalLayer.receive(2): not translated, its class isn't translated (it's generic).
        // GuiSyncLocalLayer.open(1): not translated, its class isn't translated (it's generic).
    }

    /// `VAlign` (team/creative/creativecore/common/gui/VAlign.java)
    pub mod v_align {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum VAlign {
            Top,
            Center,
            Bottom,
            Stretch,
        }
        impl VAlign {
            pub const VALUES: [VAlign; 4] = [VAlign::Top, VAlign::Center, VAlign::Bottom, VAlign::Stretch, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    VAlign::Top => "TOP",
                    VAlign::Center => "CENTER",
                    VAlign::Bottom => "BOTTOM",
                    VAlign::Stretch => "STRETCH",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "TOP" => VAlign::Top,
                    "CENTER" => VAlign::Center,
                    "BOTTOM" => VAlign::Bottom,
                    "STRETCH" => VAlign::Stretch,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `BlockGetterFake` (team/creative/creativecore/common/level/BlockGetterFake.java)
    pub mod block_getter_fake {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `BlockGetter`. Its static parts are below.
        // BlockGetterFake.set(3): not translated, its class isn't translated (it implements Minecraft's `BlockGetter`).
        // BlockGetterFake.getBlockEntity(1): not translated, its class isn't translated (it implements Minecraft's `BlockGetter`).
        // BlockGetterFake.getBlockState(1): not translated, its class isn't translated (it implements Minecraft's `BlockGetter`).
        // BlockGetterFake.getFluidState(1): not translated, its class isn't translated (it implements Minecraft's `BlockGetter`).
        // BlockGetterFake.getHeight(0): not translated, its class isn't translated (it implements Minecraft's `BlockGetter`).
        // BlockGetterFake.getMinY(0): not translated, its class isn't translated (it implements Minecraft's `BlockGetter`).
    }

    /// `IOrientatedLevel` (team/creative/creativecore/common/level/IOrientatedLevel.java)
    pub mod i_orientated_level {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IOrientatedLevel.getOrigin(0): not translated, it has no body.
        // IOrientatedLevel.setOrigin(1): not translated, it has no body.
        // IOrientatedLevel.getHolder(0): not translated, it has no body.
        // IOrientatedLevel.setHolder(1): not translated, it has no body.
        // IOrientatedLevel.unload(0): not translated, it has no body.
    }

    /// `ISubLevel` (team/creative/creativecore/common/level/ISubLevel.java)
    pub mod i_sub_level {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ISubLevel.getParent(0): not translated, it has no body.
        // ISubLevel.getRealLevel(0): not translated, it has no body.
        // ISubLevel.transformToRealWorld(1): not translated, its class isn't translated (it's an interface).
    }

    /// `LevelAccessorFake` (team/creative/creativecore/common/level/LevelAccessorFake.java)
    pub mod level_accessor_fake {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `LevelAccessor`. Its static parts are below.
        // LevelAccessorFake.set(3): not translated, its class isn't translated (it implements Minecraft's `LevelAccessor`).
        // LevelAccessorFake.registryAccess(0): not translated, its class isn't translated (it implements Minecraft's `LevelAccessor`).
        // LevelAccessorFake.getEntities(3): not translated, its class isn't translated (it implements Minecraft's `LevelAccessor`).
        // LevelAccessorFake.getEntities(3): not translated, its class isn't translated (it implements Minecraft's `LevelAccessor`).
        // LevelAccessorFake.players(0): not translated, its class isn't translated (it implements Minecraft's `LevelAccessor`).
        // LevelAccessorFake.getChunk(4): not translated, its class isn't translated (it implements Minecraft's `LevelAccessor`).
        // LevelAccessorFake.getHeight(3): not translated, its class isn't translated (it implements Minecraft's `LevelAccessor`).
        // LevelAccessorFake.getSkyDarken(0): not translated, its class isn't translated (it implements Minecraft's `LevelAccessor`).
        // LevelAccessorFake.getBiomeManager(0): not translated, its class isn't translated (it implements Minecraft's `LevelAccessor`).
        // LevelAccessorFake.getUncachedNoiseBiome(3): not translated, its class isn't translated (it implements Minecraft's `LevelAccessor`).
        // LevelAccessorFake.isClientSide(0): not translated, its class isn't translated (it implements Minecraft's `LevelAccessor`).
        // LevelAccessorFake.getSeaLevel(0): not translated, its class isn't translated (it implements Minecraft's `LevelAccessor`).
        // LevelAccessorFake.dimensionType(0): not translated, its class isn't translated (it implements Minecraft's `LevelAccessor`).
        // LevelAccessorFake.getLightEngine(0): not translated, its class isn't translated (it implements Minecraft's `LevelAccessor`).
        // LevelAccessorFake.getBlockEntity(1): not translated, its class isn't translated (it implements Minecraft's `LevelAccessor`).
        // LevelAccessorFake.getBlockState(1): not translated, its class isn't translated (it implements Minecraft's `LevelAccessor`).
        // LevelAccessorFake.getFluidState(1): not translated, its class isn't translated (it implements Minecraft's `LevelAccessor`).
        // LevelAccessorFake.getWorldBorder(0): not translated, its class isn't translated (it implements Minecraft's `LevelAccessor`).
        // LevelAccessorFake.isStateAtPosition(2): not translated, its class isn't translated (it implements Minecraft's `LevelAccessor`).
        // LevelAccessorFake.isFluidAtPosition(2): not translated, its class isn't translated (it implements Minecraft's `LevelAccessor`).
        // LevelAccessorFake.setBlock(4): not translated, its class isn't translated (it implements Minecraft's `LevelAccessor`).
        // LevelAccessorFake.removeBlock(2): not translated, its class isn't translated (it implements Minecraft's `LevelAccessor`).
        // LevelAccessorFake.destroyBlock(4): not translated, its class isn't translated (it implements Minecraft's `LevelAccessor`).
        // LevelAccessorFake.nextSubTickCount(0): not translated, its class isn't translated (it implements Minecraft's `LevelAccessor`).
        // LevelAccessorFake.getBlockTicks(0): not translated, its class isn't translated (it implements Minecraft's `LevelAccessor`).
        // LevelAccessorFake.getFluidTicks(0): not translated, its class isn't translated (it implements Minecraft's `LevelAccessor`).
        // LevelAccessorFake.getLevelData(0): not translated, its class isn't translated (it implements Minecraft's `LevelAccessor`).
        // LevelAccessorFake.getServer(0): not translated, its class isn't translated (it implements Minecraft's `LevelAccessor`).
        // LevelAccessorFake.getChunkSource(0): not translated, its class isn't translated (it implements Minecraft's `LevelAccessor`).
        // LevelAccessorFake.getRandom(0): not translated, its class isn't translated (it implements Minecraft's `LevelAccessor`).
        // LevelAccessorFake.playSound(6): not translated, its class isn't translated (it implements Minecraft's `LevelAccessor`).
        // LevelAccessorFake.addParticle(7): not translated, its class isn't translated (it implements Minecraft's `LevelAccessor`).
        // LevelAccessorFake.levelEvent(4): not translated, its class isn't translated (it implements Minecraft's `LevelAccessor`).
        // LevelAccessorFake.enabledFeatures(0): not translated, its class isn't translated (it implements Minecraft's `LevelAccessor`).
        // LevelAccessorFake.gameEvent(3): not translated, its class isn't translated (it implements Minecraft's `LevelAccessor`).
        // LevelAccessorFake.environmentAttributes(0): not translated, its class isn't translated (it implements Minecraft's `LevelAccessor`).
    }

    /// `NeighborUpdateCollector` (team/creative/creativecore/common/level/NeighborUpdateCollector.java)
    pub mod neighbor_update_collector {
        use super::*;
        // The class itself isn't translated: it uses HashMapList, which isn't translated (it's generic). Its static parts are below.
        // NeighborUpdateCollector.NeighborUpdateCollector(2): not translated, its class isn't translated (it uses HashMapList, which isn't translated (it's generic)).
        // NeighborUpdateCollector.NeighborUpdateCollector(0): not translated, its class isn't translated (it uses HashMapList, which isn't translated (it's generic)).
        // NeighborUpdateCollector.add(2): not translated, its class isn't translated (it uses HashMapList, which isn't translated (it's generic)).
        // NeighborUpdateCollector.add(1): not translated, its class isn't translated (it uses HashMapList, which isn't translated (it's generic)).
        // NeighborUpdateCollector.add(2): not translated, its class isn't translated (it uses HashMapList, which isn't translated (it's generic)).
        // NeighborUpdateCollector.processPosition(3): not translated, its class isn't translated (it uses HashMapList, which isn't translated (it's generic)).
        // NeighborUpdateCollector.process(1): not translated, its class isn't translated (it uses HashMapList, which isn't translated (it's generic)).
        // NeighborUpdateCollector.process(0): not translated, its class isn't translated (it uses HashMapList, which isn't translated (it's generic)).
    }

    /// `FabricLoaderUtils` (team/creative/creativecore/common/loader/FabricLoaderUtils.java)
    pub mod fabric_loader_utils {
        use super::*;
        // The class itself isn't translated: it implements `ILoaderUtils`. Its static parts are below.
        // FabricLoaderUtils.getLifeSpan(1): not translated, its class isn't translated (it implements `ILoaderUtils`).
        // FabricLoaderUtils.fireItemPickupPre(2): not translated, its class isn't translated (it implements `ILoaderUtils`).
        // FabricLoaderUtils.fireItemPickupPost(3): not translated, its class isn't translated (it implements `ILoaderUtils`).
    }

    /// `ILoaderUtils` (team/creative/creativecore/common/loader/ILoaderUtils.java)
    pub mod i_loader_utils {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ILoaderUtils.getLifeSpan(1): not translated, it has no body.
        // ILoaderUtils.fireItemPickupPre(2): not translated, it has no body.
        // ILoaderUtils.fireItemPickupPost(3): not translated, it has no body.
    }

    /// `UniEvent` (team/creative/creativecore/common/loader/UniEvent.java)
    pub mod uni_event {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // UniEvent.cancelable(0): not translated, it has no body.
        // UniEvent.cancel(0): not translated, it has no body.
    }

    /// `BundlePacketWrapper` (team/creative/creativecore/common/network/BundlePacketWrapper.java)
    pub mod bundle_packet_wrapper {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // BundlePacketWrapper.BundlePacketWrapper(1): not translated, its class isn't translated (it's generic).
        // BundlePacketWrapper.handle(1): not translated, its class isn't translated (it's generic).
        // BundlePacketWrapper.type(0): not translated, its class isn't translated (it's generic).
    }

    /// `CanBeNull` (team/creative/creativecore/common/network/CanBeNull.java)
    pub mod can_be_null {
        use super::*;
        // The class itself isn't translated: it's an annotation. Its static parts are below.
    }

    /// `CreativeByteBuf` (team/creative/creativecore/common/network/CreativeByteBuf.java)
    pub mod creative_byte_buf {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `RegistryFriendlyByteBuf`. Its static parts are below.
        // CreativeByteBuf.CreativeByteBuf(3): not translated, its class isn't translated (it extends Minecraft's `RegistryFriendlyByteBuf`).
    }

    /// `CreativeNetwork` (team/creative/creativecore/common/network/CreativeNetwork.java)
    pub mod creative_network {
        use super::*;
        // The class itself isn't translated: it uses `Class`. Its static parts are below.
        // CreativeNetwork.isSideOnlyPresent(1): not translated, it uses `Field` (java.lang.reflect).
        // CreativeNetwork.CreativeNetwork(3): not translated, its class isn't translated (it uses `Class`).
        // CreativeNetwork.registerType(2): not translated, it's generic.
        // CreativeNetwork.getPacketType(1): not translated, its class isn't translated (it uses `Class`).
        // CreativeNetwork.prepare(2): not translated, it's generic.
        // CreativeNetwork.sendToServer(1): not translated, its class isn't translated (it uses `Class`).
        // CreativeNetwork.sendToClient(2): not translated, its class isn't translated (it uses `Class`).
        // CreativeNetwork.sendToClient(3): not translated, its class isn't translated (it uses `Class`).
        // CreativeNetwork.sendToClient(2): not translated, its class isn't translated (it uses `Class`).
        // CreativeNetwork.sendToClientTracking(2): not translated, its class isn't translated (it uses `Class`).
        // CreativeNetwork.sendToClientTrackingAndSelf(2): not translated, its class isn't translated (it uses `Class`).
        // CreativeNetwork.sendToClientAll(2): not translated, its class isn't translated (it uses `Class`).
    }

    /// `CreativeNetworkClient` (team/creative/creativecore/common/network/CreativeNetworkClient.java)
    pub mod creative_network_client {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct CreativeNetworkClient {
        }
        impl CreativeNetworkClient {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // CreativeNetworkClient.registerClientType(1): not translated, it's generic.
    }

    /// `CreativeNetworkField` (team/creative/creativecore/common/network/CreativeNetworkField.java)
    pub mod creative_network_field {
        use super::*;
        // The class itself isn't translated: it uses `Field` (java.lang.reflect). Its static parts are below.
        // CreativeNetworkField.CreativeNetworkField(2): not translated, its class isn't translated (it uses `Field` (java.lang.reflect)).
        // CreativeNetworkField.write(3): not translated, its class isn't translated (it uses `Field` (java.lang.reflect)).
        // CreativeNetworkField.read(3): not translated, its class isn't translated (it uses `Field` (java.lang.reflect)).
        // CreativeNetworkField.create(1): not translated, it uses CreativeNetworkField, which isn't translated (it uses `Field` (java.lang.reflect)).
    }

    /// `CreativeNetworkPacket` (team/creative/creativecore/common/network/CreativeNetworkPacket.java)
    pub mod creative_network_packet {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // CreativeNetworkPacket.CreativeNetworkPacket(4): not translated, its class isn't translated (it's generic).
        // CreativeNetworkPacket.write(3): not translated, its class isn't translated (it's generic).
        // CreativeNetworkPacket.read(2): not translated, its class isn't translated (it's generic).
    }

    /// `CreativeNetworkUtils` (team/creative/creativecore/common/network/CreativeNetworkUtils.java)
    pub mod creative_network_utils {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct CreativeNetworkUtils {
        }
        impl CreativeNetworkUtils {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // CreativeNetworkUtils.flatten(1): not translated, it's generic.
    }

    /// `CreativePacket` (team/creative/creativecore/common/network/CreativePacket.java)
    pub mod creative_packet {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `CustomPacketPayload`. Its static parts are below.
        // CreativePacket.execute(1): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // CreativePacket.type(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // CreativePacket.setType(1): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // CreativePacket.executeClient(1): not translated, it has no body.
        // CreativePacket.executeServer(1): not translated, it has no body.
        // CreativePacket.requiresClient(1): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // CreativePacket.requiresServer(1): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
    }

    /// `InvalidSideException` (team/creative/creativecore/common/network/InvalidSideException.java)
    pub mod invalid_side_exception {
        use super::*;
        // The class itself isn't translated: it extends `RuntimeException`. Its static parts are below.
        // InvalidSideException.InvalidSideException(1): not translated, its class isn't translated (it extends `RuntimeException`).
        // InvalidSideException.InvalidSideException(1): not translated, its class isn't translated (it extends `RuntimeException`).
        // InvalidSideException.InvalidSideException(1): not translated, its class isn't translated (it extends `RuntimeException`).
        // InvalidSideException.InvalidSideException(1): not translated, its class isn't translated (it extends `RuntimeException`).
    }

    /// `NetworkFieldType` (team/creative/creativecore/common/network/type/NetworkFieldType.java)
    pub mod network_field_type {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // NetworkFieldType.write(5): not translated, it has no body.
        // NetworkFieldType.read(4): not translated, it has no body.
    }

    /// `NetworkFieldTypeClass` (team/creative/creativecore/common/network/type/NetworkFieldTypeClass.java)
    pub mod network_field_type_class {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // NetworkFieldTypeClass.writeContent(2): not translated, it has no body.
        // NetworkFieldTypeClass.write(5): not translated, its class isn't translated (it's generic).
        // NetworkFieldTypeClass.readContent(1): not translated, it has no body.
        // NetworkFieldTypeClass.read(4): not translated, its class isn't translated (it's generic).
    }

    /// `NetworkFieldTypeCodec` (team/creative/creativecore/common/network/type/NetworkFieldTypeCodec.java)
    pub mod network_field_type_codec {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // NetworkFieldTypeCodec.NetworkFieldTypeCodec(1): not translated, its class isn't translated (it's generic).
        // NetworkFieldTypeCodec.write(5): not translated, its class isn't translated (it's generic).
        // NetworkFieldTypeCodec.read(4): not translated, its class isn't translated (it's generic).
    }

    /// `NetworkFieldTypes` (team/creative/creativecore/common/network/type/NetworkFieldTypes.java)
    pub mod network_field_types {
        use super::*;
        pub static GSON: LazyLock<JGson> = LazyLock::new(|| JGson::default());
        // SPECIAL_PARSERS: not translated, it uses NetworkFieldTypeSpecial, which isn't translated (it's generic).
        // PARSERS: not translated, it uses `Class`.
        #[derive(Clone, Debug)]
        pub struct NetworkFieldTypes {
        }
        impl NetworkFieldTypes {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // NetworkFieldTypes.register(2): not translated, it's generic.
        // NetworkFieldTypes.register(2): not translated, it's generic.
        // NetworkFieldTypes.registerAndCodec(2): not translated, it's generic.
        // NetworkFieldTypes.register(2): not translated, it's generic.
        // NetworkFieldTypes.register(1): not translated, it's generic.
        // NetworkFieldTypes.get(1): not translated, it uses NetworkFieldType, which isn't translated (it's generic).
        // NetworkFieldTypes.get(1): not translated, it uses NetworkFieldType, which isn't translated (it's generic).
        // NetworkFieldTypes.get(2): not translated, it uses NetworkFieldType, which isn't translated (it's generic).
        // NetworkFieldTypes.write(4): not translated, it's generic.
        // NetworkFieldTypes.writeMany(4): not translated, it's generic.
        // NetworkFieldTypes.writeMany(4): not translated, it's generic.
        // NetworkFieldTypes.writeMany(4): not translated, it's generic.
        // NetworkFieldTypes.read(3): not translated, it's generic.
        // NetworkFieldTypes.readMany(3): not translated, its body couldn't be read: an anonymous class.
        // NetworkFieldTypes.writeIntArray(2): not translated, it uses Minecraft's `RegistryFriendlyByteBuf`.
        // NetworkFieldTypes.readIntArray(1): not translated, it uses Minecraft's `RegistryFriendlyByteBuf`.
    }

    /// `NetworkFieldTypeSpecial` (team/creative/creativecore/common/network/type/NetworkFieldTypeSpecial.java)
    pub mod network_field_type_special {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // NetworkFieldTypeSpecial.NetworkFieldTypeSpecial(1): not translated, its class isn't translated (it's generic).
    }

    /// `CreativeTest` (team/creative/creativecore/common/test/CreativeTest.java)
    pub mod creative_test {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct CreativeTest {
        }
        impl CreativeTest {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // CreativeTest.test(1): not translated, it has no body.
        }
    }

    /// `CreativeTestArgument` (team/creative/creativecore/common/test/CreativeTestArgument.java)
    pub mod creative_test_argument {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `ArgumentType`. Its static parts are below.
        // CreativeTestArgument.test(0): not translated, it uses CreativeTestArgument, which isn't translated (it implements Minecraft's `ArgumentType`).
        // CreativeTestArgument.getTest(2): not translated, it uses Minecraft's `CommandContext`.
        // CreativeTestArgument.parse(1): not translated, its class isn't translated (it implements Minecraft's `ArgumentType`).
        // CreativeTestArgument.listSuggestions(2): not translated, it's generic.
        // CreativeTestArgument.getExamples(0): not translated, its class isn't translated (it implements Minecraft's `ArgumentType`).
        // CreativeTestArgument.getAll(0): not translated, its class isn't translated (it implements Minecraft's `ArgumentType`).
        // CreativeTestArgument.isAllowed(1): not translated, its class isn't translated (it implements Minecraft's `ArgumentType`).
    }

    /// `CreativeTestException` (team/creative/creativecore/common/test/CreativeTestException.java)
    pub mod creative_test_exception {
        use super::*;
        // The class itself isn't translated: it extends `Exception`. Its static parts are below.
        // CreativeTestException.CreativeTestException(1): not translated, its class isn't translated (it extends `Exception`).
    }

    /// `CreativeTestHelper` (team/creative/creativecore/common/test/CreativeTestHelper.java)
    pub mod creative_test_helper {
        use super::*;
        // The class itself isn't translated: it uses `Object2LongMap` (it.unimi.dsi.fastutil.objects). Its static parts are below.
        // CreativeTestHelper.trackTime(2): not translated, it's generic.
        // CreativeTestHelper.incrementCount(0): not translated, its class isn't translated (it uses `Object2LongMap` (it.unimi.dsi.fastutil.objects)).
        // CreativeTestHelper.assertTrue(3): not translated, its class isn't translated (it uses `Object2LongMap` (it.unimi.dsi.fastutil.objects)).
        // CreativeTestHelper.assertFalse(3): not translated, its class isn't translated (it uses `Object2LongMap` (it.unimi.dsi.fastutil.objects)).
        // CreativeTestHelper.results(0): not translated, its class isn't translated (it uses `Object2LongMap` (it.unimi.dsi.fastutil.objects)).
    }

    /// `CreativeTestRegistry` (team/creative/creativecore/common/test/CreativeTestRegistry.java)
    pub mod creative_test_registry {
        use super::*;
        // REGISTRY: not translated, it uses NamedHandlerRegistry, which isn't translated (it's generic).
        #[derive(Clone, Debug)]
        pub struct CreativeTestRegistry {
        }
        impl CreativeTestRegistry {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `TestVectorFan` (team/creative/creativecore/common/test/geo/TestVectorFan.java)
    pub mod test_vector_fan {
        use super::*;
        // The class itself isn't translated: it extends `CreativeTest` (team.creative.creativecore.common.test). Its static parts are below.
        // TestVectorFan.test(1): not translated, its class isn't translated (it extends `CreativeTest` (team.creative.creativecore.common.test)).
        // TestVectorFan.generateVecs(5): not translated, its class isn't translated (it extends `CreativeTest` (team.creative.creativecore.common.test)).
    }

    /// `StringArrayArgumentType` (team/creative/creativecore/common/util/argument/StringArrayArgumentType.java)
    pub mod string_array_argument_type {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `ArgumentType`. Its static parts are below.
        pub static EXAMPLES: LazyLock<Vec<String>> = LazyLock::new(|| vec!["name1".to_string(), "name1,name2".to_string(), "name1,name2,name3".to_string()]);
        // StringArrayArgumentType.getStringArray(2): not translated, it uses Minecraft's `CommandContext`.
        // StringArrayArgumentType.stringArray(0): not translated, it uses StringArrayArgumentType, which isn't translated (it implements Minecraft's `ArgumentType`).
        // StringArrayArgumentType.parse(1): not translated, its class isn't translated (it implements Minecraft's `ArgumentType`).
        // StringArrayArgumentType.getExamples(0): not translated, its class isn't translated (it implements Minecraft's `ArgumentType`).
    }

    /// `CompoundSerializer` (team/creative/creativecore/common/util/CompoundSerializer.java)
    pub mod compound_serializer {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // CompoundSerializer.write(0): not translated, it has no body.
    }

    /// `BiFilterSerializer` (team/creative/creativecore/common/util/filter/BiFilterSerializer.java)
    pub mod bi_filter_serializer {
        use super::*;
        // The class itself isn't translated: it uses NamedTypeRegistry, which isn't translated (it's generic). Its static parts are below.
        // BiFilterSerializer.register(2): not translated, it's generic.
        // BiFilterSerializer.write(1): not translated, its class isn't translated (it uses NamedTypeRegistry, which isn't translated (it's generic)).
        // BiFilterSerializer.read(1): not translated, its class isn't translated (it uses NamedTypeRegistry, which isn't translated (it's generic)).
    }

    /// `FilterSerializer` (team/creative/creativecore/common/util/filter/FilterSerializer.java)
    pub mod filter_serializer {
        use super::*;
        // The class itself isn't translated: it uses NamedTypeRegistry, which isn't translated (it's generic). Its static parts are below.
        // FilterSerializer.register(2): not translated, it's generic.
        // FilterSerializer.write(1): not translated, its class isn't translated (it uses NamedTypeRegistry, which isn't translated (it's generic)).
        // FilterSerializer.read(1): not translated, its class isn't translated (it uses NamedTypeRegistry, which isn't translated (it's generic)).
    }

    /// `BlockFilters` (team/creative/creativecore/common/util/filter/premade/BlockFilters.java)
    pub mod block_filters {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct BlockFilters {
        }
        impl BlockFilters {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // BlockFilters.block(1): not translated, it uses `Filter` (team.creative.creativecore.common.util.filter).
        // BlockFilters.blocks(1): not translated, it uses `Filter` (team.creative.creativecore.common.util.filter).
        // BlockFilters.instance(1): not translated, it uses `Filter` (team.creative.creativecore.common.util.filter).
        // BlockFilters.and(1): not translated, it uses `Filter` (team.creative.creativecore.common.util.filter).
        // BlockFilters.or(1): not translated, it uses `Filter` (team.creative.creativecore.common.util.filter).
        // BlockFilters.not(1): not translated, it uses `Filter` (team.creative.creativecore.common.util.filter).
        // BlockFilters.property(1): not translated, it uses `Filter` (team.creative.creativecore.common.util.filter).
        // BlockFilters.tag(1): not translated, it uses `Filter` (team.creative.creativecore.common.util.filter).
    }

    /// `BlockFilters.BlockClassFilter` (team/creative/creativecore/common/util/filter/premade/BlockFilters.java)
    pub mod block_filters_block_class_filter {
        use super::*;
        // The class itself isn't translated: it implements `Filter` (team.creative.creativecore.common.util.filter). Its static parts are below.
        // BlockFilters.BlockClassFilter.BlockClassFilter(1): not translated, its class isn't translated (it implements `Filter` (team.creative.creativecore.common.util.filter)).
        // BlockFilters.BlockClassFilter.BlockClassFilter(1): not translated, its class isn't translated (it implements `Filter` (team.creative.creativecore.common.util.filter)).
        // BlockFilters.BlockClassFilter.is(1): not translated, its class isn't translated (it implements `Filter` (team.creative.creativecore.common.util.filter)).
        // BlockFilters.BlockClassFilter.write(0): not translated, its class isn't translated (it implements `Filter` (team.creative.creativecore.common.util.filter)).
    }

    /// `BlockFilters.BlockFilter` (team/creative/creativecore/common/util/filter/premade/BlockFilters.java)
    pub mod block_filters_block_filter {
        use super::*;
        // The class itself isn't translated: it implements `Filter` (team.creative.creativecore.common.util.filter). Its static parts are below.
        // BlockFilters.BlockFilter.BlockFilter(1): not translated, its class isn't translated (it implements `Filter` (team.creative.creativecore.common.util.filter)).
        // BlockFilters.BlockFilter.BlockFilter(1): not translated, its class isn't translated (it implements `Filter` (team.creative.creativecore.common.util.filter)).
        // BlockFilters.BlockFilter.is(1): not translated, its class isn't translated (it implements `Filter` (team.creative.creativecore.common.util.filter)).
        // BlockFilters.BlockFilter.write(0): not translated, its class isn't translated (it implements `Filter` (team.creative.creativecore.common.util.filter)).
    }

    /// `BlockFilters.BlockPropertyFilter` (team/creative/creativecore/common/util/filter/premade/BlockFilters.java)
    pub mod block_filters_block_property_filter {
        use super::*;
        // The class itself isn't translated: it implements `Filter` (team.creative.creativecore.common.util.filter). Its static parts are below.
        // BlockFilters.BlockPropertyFilter.BlockPropertyFilter(1): not translated, its class isn't translated (it implements `Filter` (team.creative.creativecore.common.util.filter)).
        // BlockFilters.BlockPropertyFilter.is(1): not translated, its class isn't translated (it implements `Filter` (team.creative.creativecore.common.util.filter)).
        // BlockFilters.BlockPropertyFilter.property(0): not translated, its class isn't translated (it implements `Filter` (team.creative.creativecore.common.util.filter)).
    }

    /// `BlockFilters.BlockTagFilter` (team/creative/creativecore/common/util/filter/premade/BlockFilters.java)
    pub mod block_filters_block_tag_filter {
        use super::*;
        // The class itself isn't translated: it implements `Filter` (team.creative.creativecore.common.util.filter). Its static parts are below.
        // BlockFilters.BlockTagFilter.BlockTagFilter(1): not translated, its class isn't translated (it implements `Filter` (team.creative.creativecore.common.util.filter)).
        // BlockFilters.BlockTagFilter.is(1): not translated, its class isn't translated (it implements `Filter` (team.creative.creativecore.common.util.filter)).
        // BlockFilters.BlockTagFilter.tag(0): not translated, its class isn't translated (it implements `Filter` (team.creative.creativecore.common.util.filter)).
    }

    /// `BlockFilters.BlocksFilter` (team/creative/creativecore/common/util/filter/premade/BlockFilters.java)
    pub mod block_filters_blocks_filter {
        use super::*;
        // The class itself isn't translated: it implements `Filter` (team.creative.creativecore.common.util.filter). Its static parts are below.
        // BlockFilters.BlocksFilter.BlocksFilter(1): not translated, its class isn't translated (it implements `Filter` (team.creative.creativecore.common.util.filter)).
        // BlockFilters.BlocksFilter.BlocksFilter(1): not translated, its class isn't translated (it implements `Filter` (team.creative.creativecore.common.util.filter)).
        // BlockFilters.BlocksFilter.is(1): not translated, its class isn't translated (it implements `Filter` (team.creative.creativecore.common.util.filter)).
        // BlockFilters.BlocksFilter.write(0): not translated, its class isn't translated (it implements `Filter` (team.creative.creativecore.common.util.filter)).
    }

    /// `ItemFilters` (team/creative/creativecore/common/util/filter/premade/ItemFilters.java)
    pub mod item_filters {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ItemFilters {
        }
        impl ItemFilters {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ItemFilters.item(1): not translated, it uses `Filter` (team.creative.creativecore.common.util.filter).
        // ItemFilters.items(1): not translated, it uses `Filter` (team.creative.creativecore.common.util.filter).
        // ItemFilters.instance(1): not translated, it uses `Filter` (team.creative.creativecore.common.util.filter).
        // ItemFilters.and(1): not translated, it uses `Filter` (team.creative.creativecore.common.util.filter).
        // ItemFilters.or(1): not translated, it uses `Filter` (team.creative.creativecore.common.util.filter).
        // ItemFilters.not(1): not translated, it uses `Filter` (team.creative.creativecore.common.util.filter).
    }

    /// `ItemFilters.ItemClassFilter` (team/creative/creativecore/common/util/filter/premade/ItemFilters.java)
    pub mod item_filters_item_class_filter {
        use super::*;
        // The class itself isn't translated: it implements `Filter` (team.creative.creativecore.common.util.filter). Its static parts are below.
        // ItemFilters.ItemClassFilter.ItemClassFilter(1): not translated, its class isn't translated (it implements `Filter` (team.creative.creativecore.common.util.filter)).
        // ItemFilters.ItemClassFilter.ItemClassFilter(1): not translated, its class isn't translated (it implements `Filter` (team.creative.creativecore.common.util.filter)).
        // ItemFilters.ItemClassFilter.is(1): not translated, its class isn't translated (it implements `Filter` (team.creative.creativecore.common.util.filter)).
        // ItemFilters.ItemClassFilter.write(0): not translated, its class isn't translated (it implements `Filter` (team.creative.creativecore.common.util.filter)).
    }

    /// `ItemFilters.ItemFilter` (team/creative/creativecore/common/util/filter/premade/ItemFilters.java)
    pub mod item_filters_item_filter {
        use super::*;
        // The class itself isn't translated: it implements `Filter` (team.creative.creativecore.common.util.filter). Its static parts are below.
        // ItemFilters.ItemFilter.ItemFilter(1): not translated, its class isn't translated (it implements `Filter` (team.creative.creativecore.common.util.filter)).
        // ItemFilters.ItemFilter.ItemFilter(1): not translated, its class isn't translated (it implements `Filter` (team.creative.creativecore.common.util.filter)).
        // ItemFilters.ItemFilter.is(1): not translated, its class isn't translated (it implements `Filter` (team.creative.creativecore.common.util.filter)).
        // ItemFilters.ItemFilter.write(0): not translated, its class isn't translated (it implements `Filter` (team.creative.creativecore.common.util.filter)).
    }

    /// `ItemFilters.ItemsFilter` (team/creative/creativecore/common/util/filter/premade/ItemFilters.java)
    pub mod item_filters_items_filter {
        use super::*;
        // The class itself isn't translated: it implements `Filter` (team.creative.creativecore.common.util.filter). Its static parts are below.
        // ItemFilters.ItemsFilter.ItemsFilter(1): not translated, its class isn't translated (it implements `Filter` (team.creative.creativecore.common.util.filter)).
        // ItemFilters.ItemsFilter.ItemsFilter(1): not translated, its class isn't translated (it implements `Filter` (team.creative.creativecore.common.util.filter)).
        // ItemFilters.ItemsFilter.is(1): not translated, its class isn't translated (it implements `Filter` (team.creative.creativecore.common.util.filter)).
        // ItemFilters.ItemsFilter.write(0): not translated, its class isn't translated (it implements `Filter` (team.creative.creativecore.common.util.filter)).
    }

    /// `CreativeIngredient` (team/creative/creativecore/common/util/ingredient/CreativeIngredient.java)
    pub mod creative_ingredient {
        use super::*;
        // REGISTRY: not translated, it uses NamedTypeRegistry, which isn't translated (it's generic).
        // OBJECT_PARSERS: not translated, it uses `Object`.
        // EMPTY: not translated, its value couldn't be read: an anonymous class.
        #[derive(Clone, Debug)]
        pub struct CreativeIngredient {
        }
        impl CreativeIngredient {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // CreativeIngredient.save(1): not translated, it uses Minecraft's `CompoundTag`.
            // CreativeIngredient.saveExtra(2): not translated, it has no body.
            // CreativeIngredient.loadExtra(2): not translated, it has no body.
            // CreativeIngredient.is(2): not translated, it has no body.
            // CreativeIngredient.is(1): not translated, it has no body.
            // CreativeIngredient.getExample(0): not translated, it has no body.
            // CreativeIngredient.copy(0): not translated, it has no body.
            // CreativeIngredient.equals(1): not translated, it's overloaded with the same number of parameters.
            // CreativeIngredient.equals(1): not translated, it's overloaded with the same number of parameters.
            // CreativeIngredient.description(0): not translated, it has no body.
            // CreativeIngredient.descriptionDetail(0): not translated, it has no body.
        }
        // CreativeIngredient.registerType(3): not translated, it's generic.
        // CreativeIngredient.parse(1): not translated, it uses `Object`.
        // CreativeIngredient.load(2): not translated, it uses Minecraft's `Provider`.
    }

    /// `CreativeIngredientBlock` (team/creative/creativecore/common/util/ingredient/CreativeIngredientBlock.java)
    pub mod creative_ingredient_block {
        use super::*;
        // The class itself isn't translated: it extends `CreativeIngredient`. Its static parts are below.
        // CreativeIngredientBlock.CreativeIngredientBlock(1): not translated, its class isn't translated (it extends `CreativeIngredient`).
        // CreativeIngredientBlock.CreativeIngredientBlock(0): not translated, its class isn't translated (it extends `CreativeIngredient`).
        // CreativeIngredientBlock.loadExtra(2): not translated, its class isn't translated (it extends `CreativeIngredient`).
        // CreativeIngredientBlock.saveExtra(2): not translated, its class isn't translated (it extends `CreativeIngredient`).
        // CreativeIngredientBlock.is(1): not translated, its class isn't translated (it extends `CreativeIngredient`).
        // CreativeIngredientBlock.is(2): not translated, its class isn't translated (it extends `CreativeIngredient`).
        // CreativeIngredientBlock.equals(1): not translated, its class isn't translated (it extends `CreativeIngredient`).
        // CreativeIngredientBlock.getExample(0): not translated, its class isn't translated (it extends `CreativeIngredient`).
        // CreativeIngredientBlock.copy(0): not translated, its class isn't translated (it extends `CreativeIngredient`).
        // CreativeIngredientBlock.description(0): not translated, its class isn't translated (it extends `CreativeIngredient`).
        // CreativeIngredientBlock.descriptionDetail(0): not translated, its class isn't translated (it extends `CreativeIngredient`).
    }

    /// `CreativeIngredientBlockTag` (team/creative/creativecore/common/util/ingredient/CreativeIngredientBlockTag.java)
    pub mod creative_ingredient_block_tag {
        use super::*;
        // The class itself isn't translated: it extends `CreativeIngredient`. Its static parts are below.
        // CreativeIngredientBlockTag.CreativeIngredientBlockTag(1): not translated, its class isn't translated (it extends `CreativeIngredient`).
        // CreativeIngredientBlockTag.CreativeIngredientBlockTag(0): not translated, its class isn't translated (it extends `CreativeIngredient`).
        // CreativeIngredientBlockTag.saveExtra(2): not translated, its class isn't translated (it extends `CreativeIngredient`).
        // CreativeIngredientBlockTag.loadExtra(2): not translated, its class isn't translated (it extends `CreativeIngredient`).
        // CreativeIngredientBlockTag.is(2): not translated, its class isn't translated (it extends `CreativeIngredient`).
        // CreativeIngredientBlockTag.is(1): not translated, its class isn't translated (it extends `CreativeIngredient`).
        // CreativeIngredientBlockTag.getExample(0): not translated, its class isn't translated (it extends `CreativeIngredient`).
        // CreativeIngredientBlockTag.equals(1): not translated, its class isn't translated (it extends `CreativeIngredient`).
        // CreativeIngredientBlockTag.copy(0): not translated, its class isn't translated (it extends `CreativeIngredient`).
        // CreativeIngredientBlockTag.description(0): not translated, its class isn't translated (it extends `CreativeIngredient`).
        // CreativeIngredientBlockTag.descriptionDetail(0): not translated, its class isn't translated (it extends `CreativeIngredient`).
    }

    /// `CreativeIngredientFuel` (team/creative/creativecore/common/util/ingredient/CreativeIngredientFuel.java)
    pub mod creative_ingredient_fuel {
        use super::*;
        // The class itself isn't translated: it extends `CreativeIngredient`. Its static parts are below.
        // CreativeIngredientFuel.saveExtra(2): not translated, its class isn't translated (it extends `CreativeIngredient`).
        // CreativeIngredientFuel.loadExtra(2): not translated, its class isn't translated (it extends `CreativeIngredient`).
        // CreativeIngredientFuel.getExample(0): not translated, its class isn't translated (it extends `CreativeIngredient`).
        // CreativeIngredientFuel.is(2): not translated, its class isn't translated (it extends `CreativeIngredient`).
        // CreativeIngredientFuel.is(1): not translated, its class isn't translated (it extends `CreativeIngredient`).
        // CreativeIngredientFuel.equals(1): not translated, its class isn't translated (it extends `CreativeIngredient`).
        // CreativeIngredientFuel.copy(0): not translated, its class isn't translated (it extends `CreativeIngredient`).
        // CreativeIngredientFuel.description(0): not translated, its class isn't translated (it extends `CreativeIngredient`).
        // CreativeIngredientFuel.descriptionDetail(0): not translated, its class isn't translated (it extends `CreativeIngredient`).
    }

    /// `CreativeIngredientItem` (team/creative/creativecore/common/util/ingredient/CreativeIngredientItem.java)
    pub mod creative_ingredient_item {
        use super::*;
        // The class itself isn't translated: it extends `CreativeIngredient`. Its static parts are below.
        // CreativeIngredientItem.CreativeIngredientItem(1): not translated, its class isn't translated (it extends `CreativeIngredient`).
        // CreativeIngredientItem.CreativeIngredientItem(0): not translated, its class isn't translated (it extends `CreativeIngredient`).
        // CreativeIngredientItem.saveExtra(2): not translated, its class isn't translated (it extends `CreativeIngredient`).
        // CreativeIngredientItem.loadExtra(2): not translated, its class isn't translated (it extends `CreativeIngredient`).
        // CreativeIngredientItem.getExample(0): not translated, its class isn't translated (it extends `CreativeIngredient`).
        // CreativeIngredientItem.is(1): not translated, its class isn't translated (it extends `CreativeIngredient`).
        // CreativeIngredientItem.is(2): not translated, its class isn't translated (it extends `CreativeIngredient`).
        // CreativeIngredientItem.equals(1): not translated, its class isn't translated (it extends `CreativeIngredient`).
        // CreativeIngredientItem.copy(0): not translated, its class isn't translated (it extends `CreativeIngredient`).
        // CreativeIngredientItem.description(0): not translated, its class isn't translated (it extends `CreativeIngredient`).
        // CreativeIngredientItem.descriptionDetail(0): not translated, its class isn't translated (it extends `CreativeIngredient`).
    }

    /// `CreativeIngredientItemStack` (team/creative/creativecore/common/util/ingredient/CreativeIngredientItemStack.java)
    pub mod creative_ingredient_item_stack {
        use super::*;
        // The class itself isn't translated: it extends `CreativeIngredient`. Its static parts are below.
        // CreativeIngredientItemStack.CreativeIngredientItemStack(1): not translated, its class isn't translated (it extends `CreativeIngredient`).
        // CreativeIngredientItemStack.CreativeIngredientItemStack(2): not translated, its class isn't translated (it extends `CreativeIngredient`).
        // CreativeIngredientItemStack.CreativeIngredientItemStack(0): not translated, its class isn't translated (it extends `CreativeIngredient`).
        // CreativeIngredientItemStack.getIncluded(0): not translated, its class isn't translated (it extends `CreativeIngredient`).
        // CreativeIngredientItemStack.saveExtra(2): not translated, its class isn't translated (it extends `CreativeIngredient`).
        // CreativeIngredientItemStack.loadExtra(2): not translated, its class isn't translated (it extends `CreativeIngredient`).
        // CreativeIngredientItemStack.is(1): not translated, its class isn't translated (it extends `CreativeIngredient`).
        // CreativeIngredientItemStack.is(2): not translated, its class isn't translated (it extends `CreativeIngredient`).
        // CreativeIngredientItemStack.getExample(0): not translated, its class isn't translated (it extends `CreativeIngredient`).
        // CreativeIngredientItemStack.equals(1): not translated, its class isn't translated (it extends `CreativeIngredient`).
        // CreativeIngredientItemStack.copy(0): not translated, its class isn't translated (it extends `CreativeIngredient`).
        // CreativeIngredientItemStack.description(0): not translated, its class isn't translated (it extends `CreativeIngredient`).
        // CreativeIngredientItemStack.descriptionDetail(0): not translated, its class isn't translated (it extends `CreativeIngredient`).
    }

    /// `CreativeIngredientItemTag` (team/creative/creativecore/common/util/ingredient/CreativeIngredientItemTag.java)
    pub mod creative_ingredient_item_tag {
        use super::*;
        // The class itself isn't translated: it extends `CreativeIngredient`. Its static parts are below.
        // CreativeIngredientItemTag.CreativeIngredientItemTag(1): not translated, its class isn't translated (it extends `CreativeIngredient`).
        // CreativeIngredientItemTag.CreativeIngredientItemTag(0): not translated, its class isn't translated (it extends `CreativeIngredient`).
        // CreativeIngredientItemTag.saveExtra(2): not translated, its class isn't translated (it extends `CreativeIngredient`).
        // CreativeIngredientItemTag.loadExtra(2): not translated, its class isn't translated (it extends `CreativeIngredient`).
        // CreativeIngredientItemTag.is(2): not translated, its class isn't translated (it extends `CreativeIngredient`).
        // CreativeIngredientItemTag.is(1): not translated, its class isn't translated (it extends `CreativeIngredient`).
        // CreativeIngredientItemTag.getExample(0): not translated, its class isn't translated (it extends `CreativeIngredient`).
        // CreativeIngredientItemTag.equals(1): not translated, its class isn't translated (it extends `CreativeIngredient`).
        // CreativeIngredientItemTag.copy(0): not translated, its class isn't translated (it extends `CreativeIngredient`).
        // CreativeIngredientItemTag.description(0): not translated, its class isn't translated (it extends `CreativeIngredient`).
        // CreativeIngredientItemTag.descriptionDetail(0): not translated, its class isn't translated (it extends `CreativeIngredient`).
    }

    /// `GuiCreativeIngredientHandler` (team/creative/creativecore/common/util/ingredient/GuiCreativeIngredientHandler.java)
    pub mod gui_creative_ingredient_handler {
        use super::*;
        // REGISTRY: not translated, it uses NamedHandlerRegistry, which isn't translated (it's generic).
        #[derive(Clone, Debug)]
        pub struct GuiCreativeIngredientHandler {
        }
        impl GuiCreativeIngredientHandler {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // GuiCreativeIngredientHandler.canHandle(1): not translated, it has no body.
            // GuiCreativeIngredientHandler.createControls(2): not translated, it has no body.
            // GuiCreativeIngredientHandler.parseControls(1): not translated, it has no body.
            // GuiCreativeIngredientHandler.onChanged(2): not translated, it uses GuiParent, which isn't translated (it extends `GuiControl`).
        }
        // GuiCreativeIngredientHandler.find(1): not translated, it uses GuiCreativeIngredientHandler.REGISTRY, which isn't translated (it uses NamedHandlerRegistry, which isn't translated (it's generic)).
    }

    /// `GuiCreativeIngredientHandler.GuiDataCheckList` (team/creative/creativecore/common/util/ingredient/GuiCreativeIngredientHandler.java)
    pub mod gui_creative_ingredient_handler_gui_data_check_list {
        use super::*;
        // The class itself isn't translated: it extends `GuiCheckList` (team.creative.creativecore.common.gui.control.collection). Its static parts are below.
        // GuiCreativeIngredientHandler.GuiDataCheckList.GuiDataCheckList(5): not translated, its class isn't translated (it extends `GuiCheckList` (team.creative.creativecore.common.gui.control.collection)).
        // GuiCreativeIngredientHandler.GuiDataCheckList.includes(1): not translated, its class isn't translated (it extends `GuiCheckList` (team.creative.creativecore.common.gui.control.collection)).
        // GuiCreativeIngredientHandler.GuiDataCheckList.getConfiguredIncluded(0): not translated, its class isn't translated (it extends `GuiCheckList` (team.creative.creativecore.common.gui.control.collection)).
    }

    /// `ContainerSlotView` (team/creative/creativecore/common/util/inventory/ContainerSlotView.java)
    pub mod container_slot_view {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Player`. Its static parts are below.
        // EMPTY: not translated, its value couldn't be read: an anonymous class.
        // ContainerSlotView.mainHand(1): not translated, it uses ContainerSlotView, which isn't translated (it uses Minecraft's `Player`).
        // ContainerSlotView.offHand(1): not translated, it uses ContainerSlotView, which isn't translated (it uses Minecraft's `Player`).
        // ContainerSlotView.ContainerSlotView(3): not translated, its class isn't translated (it uses Minecraft's `Player`).
        // ContainerSlotView.get(0): not translated, its class isn't translated (it uses Minecraft's `Player`).
        // ContainerSlotView.set(1): not translated, its class isn't translated (it uses Minecraft's `Player`).
        // ContainerSlotView.changed(0): not translated, its class isn't translated (it uses Minecraft's `Player`).
    }

    /// `InventoryUtils` (team/creative/creativecore/common/util/inventory/InventoryUtils.java)
    pub mod inventory_utils {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct InventoryUtils {
        }
        impl InventoryUtils {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // InventoryUtils.save(2): not translated, it uses Minecraft's `CompoundTag`.
        // InventoryUtils.load(2): not translated, it uses Minecraft's `SimpleContainer`.
        // InventoryUtils.load(3): not translated, it uses Minecraft's `SimpleContainer`.
        // InventoryUtils.isItemStackEqual(2): not translated, it uses Minecraft's `ItemStack`.
        // InventoryUtils.consumeItemStack(2): not translated, it uses Minecraft's `Container`.
        // InventoryUtils.addItemStackToInventory(2): not translated, it uses Minecraft's `Container`.
        // InventoryUtils.getAmount(2): not translated, it uses Minecraft's `Container`.
        // InventoryUtils.cleanInventory(1): not translated, it uses Minecraft's `Container`.
        // InventoryUtils.consume(3): not translated, it uses Minecraft's `Level`.
        // InventoryUtils.consume(4): not translated, it uses Minecraft's `Level`.
        // InventoryUtils.sortInventory(2): not translated, its body couldn't be read: an anonymous class.
        // InventoryUtils.toString(1): not translated, it uses Minecraft's `Container`.
        // InventoryUtils.asContent(1): not translated, it uses Minecraft's `ItemContainerContents`.
    }

    /// `Axis` (team/creative/creativecore/common/util/math/base/Axis.java)
    pub mod axis {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // Axis.get(1): not translated, it uses Axis, which isn't translated (its constants carry values).
        // Axis.third(2): not translated, it uses Axis, which isn't translated (its constants carry values).
        // Axis.getMirrorAxis(1): not translated, it uses Axis, which isn't translated (its constants carry values).
        // Axis.Axis(1): not translated, its class isn't translated (its constants carry values).
        // Axis.getMatrix(0): not translated, its class isn't translated (its constants carry values).
        // Axis.one(0): not translated, it has no body.
        // Axis.two(0): not translated, it has no body.
        // Axis.facing(1): not translated, it has no body.
        // Axis.get(3): not translated, it has no body.
        // Axis.get(3): not translated, it has no body.
        // Axis.get(3): not translated, it has no body.
        // Axis.get(1): not translated, it uses Axis, which isn't translated (its constants carry values).
        // Axis.set(2): not translated, it has no body.
        // Axis.set(2): not translated, it has no body.
        // Axis.set(2): not translated, it has no body.
        // Axis.set(2): not translated, it has no body.
        // Axis.get(1): not translated, it uses Axis, which isn't translated (its constants carry values).
        // Axis.set(2): not translated, it has no body.
        // Axis.get(1): not translated, it uses Axis, which isn't translated (its constants carry values).
        // Axis.set(2): not translated, it has no body.
        // Axis.get(1): not translated, it uses Axis, which isn't translated (its constants carry values).
        // Axis.set(2): not translated, it has no body.
        // Axis.get(1): not translated, it uses Axis, which isn't translated (its constants carry values).
        // Axis.set(2): not translated, it has no body.
        // Axis.get(3): not translated, it has no body.
        // Axis.toVanilla(0): not translated, it has no body.
        // Axis.mirror(1): not translated, its class isn't translated (its constants carry values).
        // Axis.mirror(1): not translated, its class isn't translated (its constants carry values).
        // Axis.mirror(1): not translated, its class isn't translated (its constants carry values).
        // Axis.mirror(1): not translated, its class isn't translated (its constants carry values).
        // Axis.mirror(1): not translated, its class isn't translated (its constants carry values).
        // Axis.mirror(1): not translated, its class isn't translated (its constants carry values).
        // Axis.transform(1): not translated, its class isn't translated (its constants carry values).
    }

    /// `Facing` (team/creative/creativecore/common/util/math/base/Facing.java)
    pub mod facing {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // VALUES: not translated, it uses Facing, which isn't translated (its constants carry values).
        // HORIZONTA_VALUES: not translated, it uses Facing, which isn't translated (its constants carry values).
        pub static FACING_NAMES: LazyLock<Vec<String>> = LazyLock::new(|| vec!["down".to_string(), "up".to_string(), "north".to_string(), "south".to_string(), "west".to_string(), "east".to_string()]);
        pub static HORIZONTAL_FACING_NAMES: LazyLock<Vec<String>> = LazyLock::new(|| vec!["north".to_string(), "south".to_string(), "west".to_string(), "east".to_string()]);
        // Facing.get(1): not translated, it uses Facing, which isn't translated (its constants carry values).
        // Facing.get(1): not translated, it uses Facing, which isn't translated (its constants carry values).
        // Facing.get(2): not translated, it uses Facing, which isn't translated (its constants carry values).
        // Facing.getHorizontal(1): not translated, it uses Facing, which isn't translated (its constants carry values).
        // Facing.of(1): not translated, it uses Facing, which isn't translated (its constants carry values).
        // Facing.ofNormal(1): not translated, it uses Facing, which isn't translated (its constants carry values).
        // Facing.direction(2): not translated, it uses Facing, which isn't translated (its constants carry values).
        // Facing.nearest(1): not translated, it uses Facing, which isn't translated (its constants carry values).
        // Facing.nearest(3): not translated, it uses Facing, which isn't translated (its constants carry values).
        // Facing.Facing(4): not translated, its class isn't translated (its constants carry values).
        // Facing.offset(0): not translated, its class isn't translated (its constants carry values).
        // Facing.offset(1): not translated, its class isn't translated (its constants carry values).
        // Facing.translate(0): not translated, its class isn't translated (its constants carry values).
        // Facing.opposite(0): not translated, it has no body.
        // Facing.toVanilla(0): not translated, it has no body.
        // Facing.one(0): not translated, its class isn't translated (its constants carry values).
        // Facing.two(0): not translated, its class isn't translated (its constants carry values).
        // Facing.getUAxis(0): not translated, its class isn't translated (its constants carry values).
        // Facing.getVAxis(0): not translated, its class isn't translated (its constants carry values).
        // Facing.getU(3): not translated, its class isn't translated (its constants carry values).
        // Facing.getV(3): not translated, its class isn't translated (its constants carry values).
        // Facing.get(1): not translated, it uses Facing, which isn't translated (its constants carry values).
        // Facing.set(2): not translated, it has no body.
        // Facing.rotation(0): not translated, it has no body.
        // Facing.get(6): not translated, it has no body.
        // Facing.get(6): not translated, it has no body.
        // Facing.get(6): not translated, it has no body.
        // Facing.transform(1): not translated, its class isn't translated (its constants carry values).
    }

    /// `ABB` (team/creative/creativecore/common/util/math/box/ABB.java)
    pub mod abb {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ABB {
            pub min_x: f64,
            pub min_y: f64,
            pub min_z: f64,
            pub max_x: f64,
            pub max_y: f64,
            pub max_z: f64,
        }
        impl ABB {
            // ABB.get(2): not translated, it uses Minecraft's `AABB`.
            pub fn new_6(min_x: f64, min_y: f64, min_z: f64, max_x: f64, max_y: f64, max_z: f64) -> Self {
                let mut s = Self { min_x: min_x, min_y: min_y, min_z: min_z, max_x: max_x, max_y: max_y, max_z: max_z };
                s.min_x = min_x;
                s.min_y = min_y;
                s.min_z = min_z;
                s.max_x = max_x;
                s.max_y = max_y;
                s.max_z = max_z;
                s
            }
            // ABB.ABB(1): not translated, its constructors are overloaded with the same number of parameters.
            // ABB.ABB(1): not translated, its constructors are overloaded with the same number of parameters.
            // ABB.get(1): not translated, it uses Facing, which isn't translated (its constants carry values).
            // ABB.min(1): not translated, it's overloaded with the same number of parameters.
            // ABB.max(1): not translated, it's overloaded with the same number of parameters.
            // ABB.min(1): not translated, it's overloaded with the same number of parameters.
            // ABB.max(1): not translated, it's overloaded with the same number of parameters.
            // ABB.corner(1): not translated, it uses Vec3d, which isn't translated (it extends `VecNd`).
            // ABB.cornerValue(3): not translated, it uses Minecraft's `AABB`.
            // ABB.cornerX(1): not translated, it uses BoxCorner, which isn't translated (its constants carry values).
            // ABB.cornerY(1): not translated, it uses BoxCorner, which isn't translated (its constants carry values).
            // ABB.cornerZ(1): not translated, it uses BoxCorner, which isn't translated (its constants carry values).
            // ABB.intersectsWithAxis(2): not translated, it's overloaded with the same number of parameters.
            // ABB.intersectsWithAxis(2): not translated, it's overloaded with the same number of parameters.
            // ABB.intersectsWithAxis(4): not translated, it uses Minecraft's `Axis`.
            // ABB.calculateAxisOffset(5): not translated, it uses Minecraft's `Axis`.
            // ABB.copy(0): not translated, it creates a ABB with a constructor that isn't translated.
            pub fn r#move(&mut self, x: f64, y: f64, z: f64) {
                self.min_x += x;
                self.min_y += y;
                self.min_z += z;
                self.max_x += x;
                self.max_y += y;
                self.max_z += z;
            }
            // ABB.moveCopy(3): not translated, it calls ABB.copy, which isn't translated.
            // ABB.getCorners(0): not translated, it uses Vec3d, which isn't translated (it extends `VecNd`).
            // ABB.getRotatedCorners(1): not translated, it uses Vec3d, which isn't translated (it extends `VecNd`).
            // ABB.getOuterCorner(6): not translated, it uses Vec3d, which isn't translated (it extends `VecNd`).
            // ABB.include(1): not translated, it uses Vec3d, which isn't translated (it extends `VecNd`).
            // ABB.include(2): not translated, it uses Facing, which isn't translated (its constants carry values).
            // ABB.toVanilla(0): not translated, it uses Minecraft's `AABB`.
            // ABB.rayTrace(3): not translated, it uses Minecraft's `BlockHitResult`.
            // ABB.clipFacing(7): not translated, it uses Facing, which isn't translated (its constants carry values).
            // ABB.clipPoint(8): not translated, it uses Facing, which isn't translated (its constants carry values).
            // ABB.createRotatedSurrounding(1): not translated, it uses Vec3d, which isn't translated (it extends `VecNd`).
            // ABB.createRotatedSurroundingInverseInternal(1): not translated, it uses Vec3d, which isn't translated (it extends `VecNd`).
            // ABB.contains(1): not translated, it's overloaded with the same number of parameters.
            // ABB.contains(1): not translated, it's overloaded with the same number of parameters.
            // ABB.intersectsPrecise(1): not translated, it uses Minecraft's `AABB`.
            // ABB.intersects(1): not translated, it's overloaded with the same number of parameters.
            // ABB.intersects(1): not translated, it's overloaded with the same number of parameters.
            pub fn intersects_6(&self, min_x: f64, min_y: f64, min_z: f64, max_x: f64, max_y: f64, max_z: f64) -> bool {
                return (((((self.min_x < max_x) && (self.max_x > min_x)) && (self.min_y < max_y)) && (self.max_y > min_y)) && (self.min_z < max_z)) && (self.max_z > min_z);
            }
            // ABB.intersects(2): not translated, it uses Minecraft's `Vec3`.
            pub fn contains_3(&self, x: f64, y: f64, z: f64) -> bool {
                return (((((x >= self.min_x) && (x < self.max_x)) && (y >= self.min_y)) && (y < self.max_y)) && (z >= self.min_z)) && (z < self.max_z);
            }
            // ABB.getCenter(0): not translated, it uses Minecraft's `Vec3`.
            pub fn to_string(&self) -> String {
                return format!("ABB[{}, {}, {}] -> [{}, {}, {}]", j_float(self.min_x), j_float(self.min_y), j_float(self.min_z), j_float(self.max_x), j_float(self.max_y), j_float(self.max_z));
            }
            // ABB.equals(1): not translated, it's overloaded with the same number of parameters.
            // ABB.equals(1): not translated, it's overloaded with the same number of parameters.
            // ABB.combine(1): not translated, `this` used as a value.
            pub fn deflate_1(&mut self, amount: f64) {
                self.deflate_3(amount, amount, amount);
            }
            pub fn deflate_3(&mut self, x: f64, y: f64, z: f64) {
                self.inflate_3(-x, -y, -z);
            }
            pub fn inflate_1(&mut self, amount: f64) {
                self.inflate_3(amount, amount, amount);
            }
            pub fn inflate_3(&mut self, x: f64, y: f64, z: f64) {
                self.min_x -= x;
                self.min_y -= y;
                self.min_z -= z;
                self.max_x += x;
                self.max_y += y;
                self.max_z += z;
            }
        }
        // ABB.min(2): not translated, it's overloaded with the same number of parameters.
        // ABB.max(2): not translated, it's overloaded with the same number of parameters.
        // ABB.min(2): not translated, it's overloaded with the same number of parameters.
        // ABB.max(2): not translated, it's overloaded with the same number of parameters.
        pub fn create_empty_box() -> abb::ABB {
            return abb::ABB::new_6(f64::INFINITY, f64::INFINITY, f64::INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);
        }
        // ABB.clip(4): not translated, it uses Minecraft's `BlockHitResult`.
    }

    /// `ABBs` (team/creative/creativecore/common/util/math/box/ABBs.java)
    pub mod ab_bs {
        use super::*;
        // The class itself isn't translated: it implements `Iterable`. Its static parts are below.
        // ABBs.cutOut(8): not translated, a `continue` outside a loop.
        // ABBs.ABBs(1): not translated, its class isn't translated (it implements `Iterable`).
        // ABBs.ABBs(1): not translated, its class isn't translated (it implements `Iterable`).
        // ABBs.ABBs(1): not translated, its class isn't translated (it implements `Iterable`).
        // ABBs.addShape(1): not translated, its class isn't translated (it implements `Iterable`).
        // ABBs.cutOutVanilla(1): not translated, its class isn't translated (it implements `Iterable`).
        // ABBs.cutOut(1): not translated, its class isn't translated (it implements `Iterable`).
        // ABBs.cutOut(6): not translated, its class isn't translated (it implements `Iterable`).
        // ABBs.intersectionVanilla(1): not translated, its class isn't translated (it implements `Iterable`).
        // ABBs.intersection(1): not translated, its class isn't translated (it implements `Iterable`).
        // ABBs.isEmpty(0): not translated, its class isn't translated (it implements `Iterable`).
        // ABBs.getBoxes(0): not translated, its class isn't translated (it implements `Iterable`).
        // ABBs.addNonOverlappingVanilla(1): not translated, its class isn't translated (it implements `Iterable`).
        // ABBs.addNonOverlapping(1): not translated, its class isn't translated (it implements `Iterable`).
        // ABBs.addNonOverlapping(1): not translated, its class isn't translated (it implements `Iterable`).
        // ABBs.optimize(0): not translated, its class isn't translated (it implements `Iterable`).
        // ABBs.iterator(0): not translated, its class isn't translated (it implements `Iterable`).
    }

    /// `AlignedBox` (team/creative/creativecore/common/util/math/box/AlignedBox.java)
    pub mod aligned_box {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct AlignedBox {
            pub min_x: f32,
            pub min_y: f32,
            pub min_z: f32,
            pub max_x: f32,
            pub max_y: f32,
            pub max_z: f32,
        }
        impl AlignedBox {
            pub fn new_6(min_x: f32, min_y: f32, min_z: f32, max_x: f32, max_y: f32, max_z: f32) -> Self {
                let mut s = Self { min_x: min_x, min_y: min_y, min_z: min_z, max_x: max_x, max_y: max_y, max_z: max_z };
                s.min_x = min_x;
                s.min_y = min_y;
                s.min_z = min_z;
                s.max_x = max_x;
                s.max_y = max_y;
                s.max_z = max_z;
                s
            }
            // AlignedBox.AlignedBox(1): not translated, its constructors are overloaded with the same number of parameters.
            pub fn new() -> Self {
                let mut s = Self::new_6(0.0f32, 0.0f32, 0.0f32, 1.0f32, 1.0f32, 1.0f32);
                s
            }
            // AlignedBox.AlignedBox(1): not translated, its constructors are overloaded with the same number of parameters.
            pub fn add_3(&mut self, x: f32, y: f32, z: f32) {
                self.min_x += x;
                self.min_y += y;
                self.min_z += z;
                self.max_x += x;
                self.max_y += y;
                self.max_z += z;
            }
            pub fn sub_3(&mut self, x: f32, y: f32, z: f32) {
                self.min_x -= x;
                self.min_y -= y;
                self.min_z -= z;
                self.max_x -= x;
                self.max_y -= y;
                self.max_z -= z;
            }
            // AlignedBox.add(1): not translated, it's overloaded with the same number of parameters.
            // AlignedBox.sub(1): not translated, it's overloaded with the same number of parameters.
            // AlignedBox.add(1): not translated, it's overloaded with the same number of parameters.
            // AlignedBox.sub(1): not translated, it's overloaded with the same number of parameters.
            pub fn scale(&mut self, scale: f32) {
                self.min_x *= scale;
                self.min_y *= scale;
                self.min_z *= scale;
                self.max_x *= scale;
                self.max_y *= scale;
                self.max_z *= scale;
            }
            // AlignedBox.getSize(1): not translated, it uses Axis, which isn't translated (its constants carry values).
            // AlignedBox.getSize(0): not translated, it uses Vec3d, which isn't translated (it extends `VecNd`).
            // AlignedBox.getCenter(0): not translated, it uses Vec3d, which isn't translated (it extends `VecNd`).
            pub fn to_string(&self) -> String {
                return format!("cube[{}, {}, {} -> {}, {}, {}]", j_float((self.min_x as f64)), j_float((self.min_y as f64)), j_float((self.min_z as f64)), j_float((self.max_x as f64)), j_float((self.max_y as f64)), j_float((self.max_z as f64)));
            }
            // AlignedBox.getCorner(1): not translated, it uses Vec3f, which isn't translated (it extends `VecNf`).
            // AlignedBox.getBB(0): not translated, it uses Minecraft's `AABB`.
            // AlignedBox.getBB(1): not translated, it uses Minecraft's `AABB`.
            // AlignedBox.voxelShape(0): not translated, it uses Minecraft's `VoxelShape`.
            // AlignedBox.voxelShape(1): not translated, it uses Minecraft's `VoxelShape`.
            // AlignedBox.rotate(2): not translated, it's overloaded with the same number of parameters.
            // AlignedBox.rotate(2): not translated, it's overloaded with the same number of parameters.
            pub fn set_6(&mut self, x: f32, y: f32, z: f32, x2: f32, y2: f32, z2: f32) {
                self.min_x = x.min(x2);
                self.min_y = y.min(y2);
                self.min_z = z.min(z2);
                self.max_x = x.max(x2);
                self.max_y = y.max(y2);
                self.max_z = z.max(z2);
            }
            // AlignedBox.getOffset(0): not translated, it uses Minecraft's `BlockPos`.
            // AlignedBox.get(1): not translated, it uses Facing, which isn't translated (its constants carry values).
            // AlignedBox.set(2): not translated, it uses Facing, which isn't translated (its constants carry values).
            // AlignedBox.setMin(2): not translated, it uses Axis, which isn't translated (its constants carry values).
            // AlignedBox.getMin(1): not translated, it uses Axis, which isn't translated (its constants carry values).
            // AlignedBox.setMax(2): not translated, it uses Axis, which isn't translated (its constants carry values).
            // AlignedBox.getMax(1): not translated, it uses Axis, which isn't translated (its constants carry values).
            pub fn grow_1(&mut self, value: f32) {
                self.min_x -= value;
                self.min_y -= value;
                self.min_z -= value;
                self.max_x += value;
                self.max_y += value;
                self.max_z += value;
            }
            // AlignedBox.grow(2): not translated, it uses Axis, which isn't translated (its constants carry values).
            // AlignedBox.shrink(2): not translated, it uses Axis, which isn't translated (its constants carry values).
        }
    }

    /// `BoxCorner` (team/creative/creativecore/common/util/math/box/BoxCorner.java)
    pub mod box_corner {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // LOOKUP: not translated, it uses BoxCorner, which isn't translated (its constants carry values).
        // FACING_CORNERS: not translated, it uses BoxCorner, which isn't translated (its constants carry values).
        // BoxCorner.BoxCorner(3): not translated, its class isn't translated (its constants carry values).
        // BoxCorner.lookup(3): not translated, it uses BoxCorner, which isn't translated (its constants carry values).
        // BoxCorner.getCornerUnsorted(3): not translated, it uses BoxCorner, which isn't translated (its constants carry values).
        // BoxCorner.getCorner(3): not translated, it uses BoxCorner, which isn't translated (its constants carry values).
        // BoxCorner.faceCorners(1): not translated, it uses BoxCorner, which isn't translated (its constants carry values).
        // BoxCorner.isFacing(1): not translated, its class isn't translated (its constants carry values).
        // BoxCorner.isFacingPositive(1): not translated, its class isn't translated (its constants carry values).
        // BoxCorner.getFacing(1): not translated, its class isn't translated (its constants carry values).
        // BoxCorner.transform(1): not translated, its class isn't translated (its constants carry values).
        // BoxCorner.mirror(1): not translated, its class isn't translated (its constants carry values).
        // BoxCorner.rotate(1): not translated, its class isn't translated (its constants carry values).
        // BoxCorner.get(1): not translated, its class isn't translated (its constants carry values).
        // BoxCorner.set(2): not translated, its class isn't translated (its constants carry values).
        // BoxCorner.get(1): not translated, its class isn't translated (its constants carry values).
        // BoxCorner.set(2): not translated, its class isn't translated (its constants carry values).
        // BoxCorner.facingTo(1): not translated, its class isn't translated (its constants carry values).
    }

    /// `BoxesVoxelShape` (team/creative/creativecore/common/util/math/box/BoxesVoxelShape.java)
    pub mod boxes_voxel_shape {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `SliceShape`. Its static parts are below.
        // DISCRETE_SHAPE: not translated, its value couldn't be read: an anonymous class.
        // BoxesVoxelShape.create(1): not translated, it uses BoxesVoxelShape, which isn't translated (it extends Minecraft's `SliceShape`).
        // BoxesVoxelShape.createVanilla(1): not translated, it uses BoxesVoxelShape, which isn't translated (it extends Minecraft's `SliceShape`).
        // BoxesVoxelShape.create(1): not translated, it uses BoxesVoxelShape, which isn't translated (it extends Minecraft's `SliceShape`).
        // BoxesVoxelShape.BoxesVoxelShape(0): not translated, its class isn't translated (it extends Minecraft's `SliceShape`).
        // BoxesVoxelShape.move(3): not translated, its class isn't translated (it extends Minecraft's `SliceShape`).
        // BoxesVoxelShape.getCoords(1): not translated, its class isn't translated (it extends Minecraft's `SliceShape`).
        // BoxesVoxelShape.get(2): not translated, its class isn't translated (it extends Minecraft's `SliceShape`).
        // BoxesVoxelShape.min(1): not translated, its class isn't translated (it extends Minecraft's `SliceShape`).
        // BoxesVoxelShape.max(1): not translated, its class isn't translated (it extends Minecraft's `SliceShape`).
        // BoxesVoxelShape.forAllEdges(1): not translated, its class isn't translated (it extends Minecraft's `SliceShape`).
        // BoxesVoxelShape.forAllEdgesCorner(4): not translated, its class isn't translated (it extends Minecraft's `SliceShape`).
        // BoxesVoxelShape.forAllBoxes(1): not translated, its class isn't translated (it extends Minecraft's `SliceShape`).
        // BoxesVoxelShape.toAabbs(0): not translated, its class isn't translated (it extends Minecraft's `SliceShape`).
        // BoxesVoxelShape.min(3): not translated, its class isn't translated (it extends Minecraft's `SliceShape`).
        // BoxesVoxelShape.max(3): not translated, its class isn't translated (it extends Minecraft's `SliceShape`).
        // BoxesVoxelShape.findIndex(2): not translated, its class isn't translated (it extends Minecraft's `SliceShape`).
        // BoxesVoxelShape.clip(3): not translated, its class isn't translated (it extends Minecraft's `SliceShape`).
        // BoxesVoxelShape.closestPointTo(1): not translated, its class isn't translated (it extends Minecraft's `SliceShape`).
        // BoxesVoxelShape.getFaceShape(1): not translated, its class isn't translated (it extends Minecraft's `SliceShape`).
        // BoxesVoxelShape.isEmpty(0): not translated, its class isn't translated (it extends Minecraft's `SliceShape`).
        // BoxesVoxelShape.optimize(0): not translated, its class isn't translated (it extends Minecraft's `SliceShape`).
        // BoxesVoxelShape.requiresAdvancedEntityStep(0): not translated, its class isn't translated (it extends Minecraft's `SliceShape`).
        // BoxesVoxelShape.intersectsWith(1): not translated, its class isn't translated (it extends Minecraft's `SliceShape`).
        // BoxesVoxelShape.onlyKeepIntersecting(1): not translated, its class isn't translated (it extends Minecraft's `SliceShape`).
        // BoxesVoxelShape.collide(3): not translated, its class isn't translated (it extends Minecraft's `SliceShape`).
        // BoxesVoxelShape.collideX(3): not translated, its class isn't translated (it extends Minecraft's `SliceShape`).
        // BoxesVoxelShape.bounds(0): not translated, its class isn't translated (it extends Minecraft's `SliceShape`).
        // BoxesVoxelShape.toString(0): not translated, its class isn't translated (it extends Minecraft's `SliceShape`).
    }

    /// `BoxFace` (team/creative/creativecore/common/util/math/box/BoxFace.java)
    pub mod box_face {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // BoxFace.BoxFace(4): not translated, its class isn't translated (its constants carry values).
        // BoxFace.getCornerInQuestion(2): not translated, its class isn't translated (its constants carry values).
        // BoxFace.getTriangle(2): not translated, its class isn't translated (its constants carry values).
        // BoxFace.getTexUAxis(0): not translated, its class isn't translated (its constants carry values).
        // BoxFace.getTexVAxis(0): not translated, its class isn't translated (its constants carry values).
        // BoxFace.getTexU(0): not translated, its class isn't translated (its constants carry values).
        // BoxFace.getTexV(0): not translated, its class isn't translated (its constants carry values).
        // BoxFace.getTriangleFirst(1): not translated, its class isn't translated (its constants carry values).
        // BoxFace.getTriangleSecond(1): not translated, its class isn't translated (its constants carry values).
        // BoxFace.getCorners(0): not translated, its class isn't translated (its constants carry values).
        // BoxFace.getFacing(0): not translated, its class isn't translated (its constants carry values).
        // BoxFace.getOne(0): not translated, its class isn't translated (its constants carry values).
        // BoxFace.getTwo(0): not translated, its class isn't translated (its constants carry values).
        // BoxFace.first(1): not translated, its class isn't translated (its constants carry values).
        // BoxFace.normal(1): not translated, its class isn't translated (its constants carry values).
        // BoxFace.isFacingOutwards(3): not translated, its class isn't translated (its constants carry values).
        // BoxFace.getTraingleNormal(2): not translated, it uses Vec3f, which isn't translated (it extends `VecNf`).
        // BoxFace.ensureSameLength(2): not translated, it uses Vec3f, which isn't translated (it extends `VecNf`).
        // BoxFace.get(1): not translated, it uses BoxFace, which isn't translated (its constants carry values).
        // BoxFace.get(2): not translated, it uses BoxFace, which isn't translated (its constants carry values).
        // BoxFace.getVecArray(2): not translated, it uses Vec3f, which isn't translated (it extends `VecNf`).
    }

    /// `BoxPlane` (team/creative/creativecore/common/util/math/box/BoxPlane.java)
    pub mod box_plane {
        use super::*;
        // The class itself isn't translated: it uses BoxFace, which isn't translated (its constants carry values). Its static parts are below.
        // BoxPlane.BoxPlane(2): not translated, its class isn't translated (it uses BoxFace, which isn't translated (its constants carry values)).
        // BoxPlane.getIntersectingScale(2): not translated, its class isn't translated (it uses BoxFace, which isn't translated (its constants carry values)).
        // BoxPlane.linePlaneIntersection(4): not translated, it uses Vec3d, which isn't translated (it extends `VecNd`).
        // BoxPlane.createPlane(3): not translated, it uses BoxPlane, which isn't translated (it uses BoxFace, which isn't translated (its constants carry values)).
        // BoxPlane.createOppositePlane(3): not translated, it uses BoxPlane, which isn't translated (it uses BoxFace, which isn't translated (its constants carry values)).
    }

    /// `BoxUtils` (team/creative/creativecore/common/util/math/box/BoxUtils.java)
    pub mod box_utils {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct BoxUtils {
        }
        impl BoxUtils {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn equals(a: f64, b: f64, deviation: f64) -> bool {
            return (a == b) || (((a - b).abs()) < deviation);
        }
        pub fn greater_equals(a: f64, b: f64, deviation: f64) -> bool {
            return a >= (if b > 0.0f64 { b - deviation } else { b + deviation });
        }
        pub fn inside_rect(one: f64, two: f64, min_one: f64, min_two: f64, max_one: f64, max_two: f64) -> bool {
            return (((one > min_one) && (one < max_one)) && (two > min_two)) && (two < max_two);
        }
        // BoxUtils.lengthIgnoreAxis(3): not translated, it uses Vec3d, which isn't translated (it extends `VecNd`).
        // BoxUtils.includeMaxRotationInBox(4): not translated, it uses Vec3d, which isn't translated (it extends `VecNd`).
        // BoxUtils.includeMaxRotationInBoxInverse(4): not translated, it uses Vec3d, which isn't translated (it extends `VecNd`).
        // BoxUtils.includeMaxRotationInBox(7): not translated, it uses Vec3d, which isn't translated (it extends `VecNd`).
        // BoxUtils.get(2): not translated, it uses Minecraft's `AABB`.
        // BoxUtils.min(2): not translated, it's overloaded with the same number of parameters.
        // BoxUtils.max(2): not translated, it's overloaded with the same number of parameters.
        // BoxUtils.min(2): not translated, it's overloaded with the same number of parameters.
        // BoxUtils.max(2): not translated, it's overloaded with the same number of parameters.
        // BoxUtils.corner(2): not translated, it uses Vec3d, which isn't translated (it extends `VecNd`).
        // BoxUtils.cornerValue(3): not translated, it uses Minecraft's `AABB`.
        // BoxUtils.cornerX(2): not translated, it uses Minecraft's `AABB`.
        // BoxUtils.cornerY(2): not translated, it uses Minecraft's `AABB`.
        // BoxUtils.cornerZ(2): not translated, it uses Minecraft's `AABB`.
        // BoxUtils.getCorners(1): not translated, it uses Vec3d, which isn't translated (it extends `VecNd`).
        // BoxUtils.getRotatedCorners(2): not translated, it uses Vec3d, which isn't translated (it extends `VecNd`).
        // BoxUtils.intersectsWithAxis(4): not translated, it uses Minecraft's `AABB`.
        // BoxUtils.intersectsWithAxis(5): not translated, it uses Minecraft's `AABB`.
        // BoxUtils.calculateAxisOffset(6): not translated, it uses Axis, which isn't translated (its constants carry values).
        // BoxUtils.getIntersectionVolume(2): not translated, it uses Minecraft's `AABB`.
    }

    /// `OBB` (team/creative/creativecore/common/util/math/box/OBB.java)
    pub mod obb {
        use super::*;
        // The class itself isn't translated: it extends `ABB`. Its static parts are below.
        pub fn get_corner_offset(value: f64, min: f64, max: f64) -> i32 {
            if value <= min {
                return -1;
            } else {
                return if value >= max { 1 } else { 0 };
            }
            #[allow(unreachable_code)]
            unreachable!()
        }
        // OBB.calculateDistanceFromPlane(6): not translated, it uses Vec2d, which isn't translated (it extends `VecNd`).
        // OBB.OBB(2): not translated, its class isn't translated (it extends `ABB`).
        // OBB.OBB(2): not translated, its class isn't translated (it extends `ABB`).
        // OBB.copy(0): not translated, its class isn't translated (it extends `ABB`).
        // OBB.intersectsPrecise(1): not translated, its class isn't translated (it extends `ABB`).
        // OBB.calculateDistanceRotated(3): not translated, its body couldn't be read: a labelled statement.
        // OBB.calculateAxisOffset(5): not translated, its class isn't translated (it extends `ABB`).
        // OBB.toString(0): not translated, its class isn't translated (it extends `ABB`).
    }

    /// `CollidingPlane` (team/creative/creativecore/common/util/math/collision/CollidingPlane.java)
    pub mod colliding_plane {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `AABB`. Its static parts are below.
        pub const ACCURACY_STEPS: i32 = 10;
        // CollidingPlane.CollidingPlane(5): not translated, its class isn't translated (it uses Minecraft's `AABB`).
        // CollidingPlane.isInFront(1): not translated, its class isn't translated (it uses Minecraft's `AABB`).
        // CollidingPlane.binarySearch(5): not translated, its class isn't translated (it uses Minecraft's `AABB`).
        // CollidingPlane.searchBetweenSimple(8): not translated, its class isn't translated (it uses Minecraft's `AABB`).
        // CollidingPlane.intersects(5): not translated, its class isn't translated (it uses Minecraft's `AABB`).
        // CollidingPlane.getDirection(3): not translated, it uses Facing, which isn't translated (its constants carry values).
    }

    /// `CollisionCoordinator` (team/creative/creativecore/common/util/math/collision/CollisionCoordinator.java)
    pub mod collision_coordinator {
        use super::*;
        // The class itself isn't translated: a Map or Set of floating-point numbers. Its static parts are below.
        // CollisionCoordinator.CollisionCoordinator(7): not translated, its class isn't translated (a Map or Set of floating-point numbers).
        // CollisionCoordinator.hasOnlyTranslation(0): not translated, its class isn't translated (a Map or Set of floating-point numbers).
        // CollisionCoordinator.getRotationDegree(1): not translated, its class isn't translated (a Map or Set of floating-point numbers).
        // CollisionCoordinator.getRotationMatrix(1): not translated, its class isn't translated (a Map or Set of floating-point numbers).
        // CollisionCoordinator.getRotationMatrixInv(1): not translated, its class isn't translated (a Map or Set of floating-point numbers).
        // CollisionCoordinator.original(0): not translated, its class isn't translated (a Map or Set of floating-point numbers).
        // CollisionCoordinator.moved(0): not translated, its class isn't translated (a Map or Set of floating-point numbers).
        // CollisionCoordinator.computeSurroundingBox(1): not translated, its class isn't translated (a Map or Set of floating-point numbers).
        // CollisionCoordinator.computeInverseSurroundingBoxInternal(1): not translated, its class isn't translated (a Map or Set of floating-point numbers).
        // CollisionCoordinator.isSimple(0): not translated, its class isn't translated (a Map or Set of floating-point numbers).
        // CollisionCoordinator.getNumberOfHalfRotations(0): not translated, its class isn't translated (a Map or Set of floating-point numbers).
        // CollisionCoordinator.getInverted(1): not translated, its class isn't translated (a Map or Set of floating-point numbers).
        // CollisionCoordinator.get(1): not translated, its class isn't translated (a Map or Set of floating-point numbers).
        // CollisionCoordinator.transform(2): not translated, its class isn't translated (a Map or Set of floating-point numbers).
        // CollisionCoordinator.transformInverted(2): not translated, its class isn't translated (a Map or Set of floating-point numbers).
        // CollisionCoordinator.transform(2): not translated, its class isn't translated (a Map or Set of floating-point numbers).
        // CollisionCoordinator.finish(0): not translated, its class isn't translated (a Map or Set of floating-point numbers).
    }

    /// `IntersectionHelper` (team/creative/creativecore/common/util/math/collision/IntersectionHelper.java)
    pub mod intersection_helper {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct IntersectionHelper {
        }
        impl IntersectionHelper {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // IntersectionHelper.cutMinMax(5): not translated, it uses Vec2d, which isn't translated (it extends `VecNd`).
        // IntersectionHelper.cutMinMax(7): not translated, it uses Vec2f, which isn't translated (it extends `VecNf`).
        // IntersectionHelper.add(2): not translated, it's overloaded with the same number of parameters.
        // IntersectionHelper.iterateLines(15): not translated, it uses Edge, which isn't translated (its constants carry values).
        // IntersectionHelper.findIntersection(13): not translated, it uses Edge, which isn't translated (its constants carry values).
        // IntersectionHelper.addCornersBetween(8): not translated, it's overloaded with the same number of parameters.
        // IntersectionHelper.add(2): not translated, it's overloaded with the same number of parameters.
        // IntersectionHelper.iterateLines(13): not translated, it uses Edge, which isn't translated (its constants carry values).
        // IntersectionHelper.findIntersection(13): not translated, it uses Edge, which isn't translated (its constants carry values).
        // IntersectionHelper.addCornersBetween(8): not translated, it's overloaded with the same number of parameters.
    }

    /// `IntersectionHelper.Edge` (team/creative/creativecore/common/util/math/collision/IntersectionHelper.java)
    pub mod intersection_helper_edge {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // IntersectionHelper.Edge.one(0): not translated, it has no body.
        // IntersectionHelper.Edge.positive(0): not translated, it has no body.
        // IntersectionHelper.Edge.clockwise(0): not translated, it has no body.
        // IntersectionHelper.Edge.counterClockwise(0): not translated, it has no body.
    }

    /// `IntersectionHelper.EdgeStatus` (team/creative/creativecore/common/util/math/collision/IntersectionHelper.java)
    pub mod intersection_helper_edge_status {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // IntersectionHelper.EdgeStatus.of(3): not translated, it uses EdgeStatus, which isn't translated (its constants carry values).
        // IntersectionHelper.EdgeStatus.of(3): not translated, it uses EdgeStatus, which isn't translated (its constants carry values).
        // IntersectionHelper.EdgeStatus.positive(0): not translated, its class isn't translated (its constants carry values).
        // IntersectionHelper.EdgeStatus.edge(0): not translated, its class isn't translated (its constants carry values).
        // IntersectionHelper.EdgeStatus.outside(0): not translated, its class isn't translated (its constants carry values).
    }

    /// `IntersectionHelper.InsideStatus` (team/creative/creativecore/common/util/math/collision/IntersectionHelper.java)
    pub mod intersection_helper_inside_status {
        use super::*;
        // The class itself isn't translated: it uses EdgeStatus, which isn't translated (its constants carry values). Its static parts are below.
        // CACHE: not translated, it uses InsideStatus, which isn't translated (it uses EdgeStatus, which isn't translated (its constants carry values)).
        // IntersectionHelper.InsideStatus.of(2): not translated, it uses InsideStatus, which isn't translated (it uses EdgeStatus, which isn't translated (its constants carry values)).
        // IntersectionHelper.InsideStatus.get(6): not translated, it uses InsideStatus, which isn't translated (it uses EdgeStatus, which isn't translated (its constants carry values)).
        // IntersectionHelper.InsideStatus.get(6): not translated, it uses InsideStatus, which isn't translated (it uses EdgeStatus, which isn't translated (its constants carry values)).
        // IntersectionHelper.InsideStatus.InsideStatus(2): not translated, its class isn't translated (it uses EdgeStatus, which isn't translated (its constants carry values)).
        // IntersectionHelper.InsideStatus.isInside(0): not translated, its class isn't translated (it uses EdgeStatus, which isn't translated (its constants carry values)).
        // IntersectionHelper.InsideStatus.isInsideOrOnEdge(0): not translated, its class isn't translated (it uses EdgeStatus, which isn't translated (its constants carry values)).
        // IntersectionHelper.InsideStatus.isOutside(0): not translated, its class isn't translated (it uses EdgeStatus, which isn't translated (its constants carry values)).
        // IntersectionHelper.InsideStatus.oneEdge(0): not translated, its class isn't translated (it uses EdgeStatus, which isn't translated (its constants carry values)).
        // IntersectionHelper.InsideStatus.twoEdge(0): not translated, its class isn't translated (it uses EdgeStatus, which isn't translated (its constants carry values)).
    }

    /// `MatrixUtils` (team/creative/creativecore/common/util/math/collision/MatrixUtils.java)
    pub mod matrix_utils {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct MatrixUtils {
        }
        impl MatrixUtils {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn create_identity_matrix() -> matrix3::Matrix3 {
            let mut matrix: matrix3::Matrix3 = matrix3::Matrix3::new();
            matrix.set_identity();
            return matrix.clone();
        }
        // MatrixUtils.createRotationMatrix(3): not translated, it's overloaded with the same number of parameters.
        pub fn mul(matrix: &mut matrix4::Matrix4, matrix2: &matrix3::Matrix3) {
            let m00: f64 = ((matrix.m00 * matrix2.m00) + (matrix.m01 * matrix2.m10)) + (matrix.m02 * matrix2.m20);
            let m01: f64 = ((matrix.m00 * matrix2.m01) + (matrix.m01 * matrix2.m11)) + (matrix.m02 * matrix2.m21);
            let m02: f64 = ((matrix.m00 * matrix2.m02) + (matrix.m01 * matrix2.m12)) + (matrix.m02 * matrix2.m22);
            let m10: f64 = ((matrix.m10 * matrix2.m00) + (matrix.m11 * matrix2.m10)) + (matrix.m12 * matrix2.m20);
            let m11: f64 = ((matrix.m10 * matrix2.m01) + (matrix.m11 * matrix2.m11)) + (matrix.m12 * matrix2.m21);
            let m12: f64 = ((matrix.m10 * matrix2.m02) + (matrix.m11 * matrix2.m12)) + (matrix.m12 * matrix2.m22);
            let m20: f64 = ((matrix.m20 * matrix2.m00) + (matrix.m21 * matrix2.m10)) + (matrix.m22 * matrix2.m20);
            let m21: f64 = ((matrix.m20 * matrix2.m01) + (matrix.m21 * matrix2.m11)) + (matrix.m22 * matrix2.m21);
            let m22: f64 = ((matrix.m20 * matrix2.m02) + (matrix.m21 * matrix2.m12)) + (matrix.m22 * matrix2.m22);
            matrix.m00 = m00;
            matrix.m01 = m01;
            matrix.m02 = m02;
            matrix.m10 = m10;
            matrix.m11 = m11;
            matrix.m12 = m12;
            matrix.m20 = m20;
            matrix.m21 = m21;
            matrix.m22 = m22;
        }
        pub fn create_rotation_matrix_and_translation_radians(x: f64, y: f64, z: f64, rot_x: f64, rot_y: f64, rot_z: f64) -> matrix4::Matrix4 {
            let mut matrix: matrix4::Matrix4 = matrix4::Matrix4::new();
            if rot_x != 0.0f64 {
                matrix.rot_x(rot_x);
            } else {
                matrix.set_identity();
            }
            if rot_y != 0.0f64 {
                matrix_utils::mul(&mut matrix, &matrix_utils::create_rotation_matrix_y_radians(rot_y));
            }
            if rot_z != 0.0f64 {
                matrix_utils::mul(&mut matrix, &matrix_utils::create_rotation_matrix_z_radians(rot_z));
            }
            matrix.m03 = x;
            matrix.m13 = y;
            matrix.m23 = z;
            return matrix.clone();
        }
        pub fn create_rotation_matrix_and_translation(x: f64, y: f64, z: f64, rot_x: f64, rot_y: f64, rot_z: f64) -> matrix4::Matrix4 {
            let mut matrix: matrix4::Matrix4 = matrix4::Matrix4::new();
            if rot_x != 0.0f64 {
                matrix.rot_x(rot_x.to_radians());
            } else {
                matrix.set_identity();
            }
            if rot_y != 0.0f64 {
                matrix_utils::mul(&mut matrix, &matrix_utils::create_rotation_matrix_y(rot_y));
            }
            if rot_z != 0.0f64 {
                matrix_utils::mul(&mut matrix, &matrix_utils::create_rotation_matrix_z(rot_z));
            }
            matrix.m03 = x;
            matrix.m13 = y;
            matrix.m23 = z;
            return matrix.clone();
        }
        pub fn create_rotation_matrix_x(angle: f64) -> matrix3::Matrix3 {
            let mut matrix: matrix3::Matrix3 = matrix3::Matrix3::new();
            matrix.rot_x(angle.to_radians());
            return matrix.clone();
        }
        pub fn create_rotation_matrix_x_radians(radians: f64) -> matrix3::Matrix3 {
            let mut matrix: matrix3::Matrix3 = matrix3::Matrix3::new();
            matrix.rot_x(radians);
            return matrix.clone();
        }
        pub fn create_rotation_matrix_y(angle: f64) -> matrix3::Matrix3 {
            let mut matrix: matrix3::Matrix3 = matrix3::Matrix3::new();
            matrix.rot_y(angle.to_radians());
            return matrix.clone();
        }
        pub fn create_rotation_matrix_y_radians(radians: f64) -> matrix3::Matrix3 {
            let mut matrix: matrix3::Matrix3 = matrix3::Matrix3::new();
            matrix.rot_y(radians);
            return matrix.clone();
        }
        pub fn create_rotation_matrix_z(angle: f64) -> matrix3::Matrix3 {
            let mut matrix: matrix3::Matrix3 = matrix3::Matrix3::new();
            matrix.rot_z(angle.to_radians());
            return matrix.clone();
        }
        pub fn create_rotation_matrix_z_radians(radians: f64) -> matrix3::Matrix3 {
            let mut matrix: matrix3::Matrix3 = matrix3::Matrix3::new();
            matrix.rot_z(radians);
            return matrix.clone();
        }
    }

    /// `PlaneCache` (team/creative/creativecore/common/util/math/collision/PlaneCache.java)
    pub mod plane_cache {
        use super::*;
        // The class itself isn't translated: it uses CollidingPlane, which isn't translated (it uses Minecraft's `AABB`). Its static parts are below.
        // PlaneCache.PlaneCache(2): not translated, its class isn't translated (it uses CollidingPlane, which isn't translated (it uses Minecraft's `AABB`)).
        // PlaneCache.calculatePlanes(1): not translated, its class isn't translated (it uses CollidingPlane, which isn't translated (it uses Minecraft's `AABB`)).
        // PlaneCache.getPushOutScale(3): not translated, its class isn't translated (it uses CollidingPlane, which isn't translated (it uses Minecraft's `AABB`)).
    }

    /// `NormalPlaneD` (team/creative/creativecore/common/util/math/geo/NormalPlaneD.java)
    pub mod normal_plane_d {
        use super::*;
        // The class itself isn't translated: it uses Vec3d, which isn't translated (it extends `VecNd`). Its static parts are below.
        // NormalPlaneD.NormalPlaneD(2): not translated, its class isn't translated (it uses Vec3d, which isn't translated (it extends `VecNd`)).
        // NormalPlaneD.NormalPlaneD(2): not translated, its class isn't translated (it uses Vec3d, which isn't translated (it extends `VecNd`)).
        // NormalPlaneD.NormalPlaneD(1): not translated, its class isn't translated (it uses Vec3d, which isn't translated (it extends `VecNd`)).
        // NormalPlaneD.NormalPlaneD(3): not translated, its class isn't translated (it uses Vec3d, which isn't translated (it extends `VecNd`)).
        // NormalPlaneD.isInvalid(0): not translated, its class isn't translated (it uses Vec3d, which isn't translated (it extends `VecNd`)).
        // NormalPlaneD.isInFront(1): not translated, its class isn't translated (it uses Vec3d, which isn't translated (it extends `VecNd`)).
        // NormalPlaneD.isInFront(2): not translated, its class isn't translated (it uses Vec3d, which isn't translated (it extends `VecNd`)).
        // NormalPlaneD.isInFront(1): not translated, its class isn't translated (it uses Vec3d, which isn't translated (it extends `VecNd`)).
        // NormalPlaneD.isInFront(2): not translated, its class isn't translated (it uses Vec3d, which isn't translated (it extends `VecNd`)).
        // NormalPlaneD.cuts(1): not translated, its class isn't translated (it uses Vec3d, which isn't translated (it extends `VecNd`)).
        // NormalPlaneD.intersect(2): not translated, its class isn't translated (it uses Vec3d, which isn't translated (it extends `VecNd`)).
        // NormalPlaneD.intersect(1): not translated, its class isn't translated (it uses Vec3d, which isn't translated (it extends `VecNd`)).
        // NormalPlaneD.project(5): not translated, its class isn't translated (it uses Vec3d, which isn't translated (it extends `VecNd`)).
        // NormalPlaneD.toString(0): not translated, its class isn't translated (it uses Vec3d, which isn't translated (it extends `VecNd`)).
        // NormalPlaneD.toFloat(0): not translated, its class isn't translated (it uses Vec3d, which isn't translated (it extends `VecNd`)).
    }

    /// `NormalPlaneF` (team/creative/creativecore/common/util/math/geo/NormalPlaneF.java)
    pub mod normal_plane_f {
        use super::*;
        // The class itself isn't translated: it uses Vec3f, which isn't translated (it extends `VecNf`). Its static parts are below.
        // NormalPlaneF.NormalPlaneF(2): not translated, its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)).
        // NormalPlaneF.NormalPlaneF(1): not translated, its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)).
        // NormalPlaneF.NormalPlaneF(3): not translated, its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)).
        // NormalPlaneF.isInvalid(0): not translated, its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)).
        // NormalPlaneF.isInFront(1): not translated, its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)).
        // NormalPlaneF.isInFront(2): not translated, its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)).
        // NormalPlaneF.cuts(1): not translated, its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)).
        // NormalPlaneF.intersect(2): not translated, its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)).
        // NormalPlaneF.intersect(1): not translated, its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)).
        // NormalPlaneF.project(5): not translated, its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)).
        // NormalPlaneF.toString(0): not translated, its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)).
        // NormalPlaneF.isFacing(1): not translated, its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)).
    }

    /// `Ray2d` (team/creative/creativecore/common/util/math/geo/Ray2d.java)
    pub mod ray2d {
        use super::*;
        // The class itself isn't translated: it uses Axis, which isn't translated (its constants carry values). Its static parts are below.
        // Ray2d.Ray2d(5): not translated, its class isn't translated (it uses Axis, which isn't translated (its constants carry values)).
        // Ray2d.Ray2d(6): not translated, its class isn't translated (it uses Axis, which isn't translated (its constants carry values)).
        // Ray2d.getOrigin(1): not translated, its class isn't translated (it uses Axis, which isn't translated (its constants carry values)).
        // Ray2d.getDirection(1): not translated, its class isn't translated (it uses Axis, which isn't translated (its constants carry values)).
        // Ray2d.getOther(1): not translated, its class isn't translated (it uses Axis, which isn't translated (its constants carry values)).
        // Ray2d.set(6): not translated, its class isn't translated (it uses Axis, which isn't translated (its constants carry values)).
        // Ray2d.set(4): not translated, its class isn't translated (it uses Axis, which isn't translated (its constants carry values)).
        // Ray2d.getT(2): not translated, its class isn't translated (it uses Axis, which isn't translated (its constants carry values)).
        // Ray2d.get(2): not translated, its class isn't translated (it uses Axis, which isn't translated (its constants carry values)).
        // Ray2d.get(1): not translated, its class isn't translated (it uses Axis, which isn't translated (its constants carry values)).
        // Ray2d.getFloat(1): not translated, its class isn't translated (it uses Axis, which isn't translated (its constants carry values)).
        // Ray2d.getWithLimits(2): not translated, its class isn't translated (it uses Axis, which isn't translated (its constants carry values)).
        // Ray2d.getWithLimits(4): not translated, its class isn't translated (it uses Axis, which isn't translated (its constants carry values)).
        // Ray2d.isCoordinateOnLine(2): not translated, its class isn't translated (it uses Axis, which isn't translated (its constants carry values)).
        // Ray2d.isCoordinateOnLine(2): not translated, its class isn't translated (it uses Axis, which isn't translated (its constants carry values)).
        // Ray2d.isCoordinateToTheRight(2): not translated, its class isn't translated (it uses Axis, which isn't translated (its constants carry values)).
        // Ray2d.isCoordinateToTheRight(2): not translated, its class isn't translated (it uses Axis, which isn't translated (its constants carry values)).
        // Ray2d.isParallel(1): not translated, its class isn't translated (it uses Axis, which isn't translated (its constants carry values)).
        // Ray2d.intersect(3): not translated, its class isn't translated (it uses Axis, which isn't translated (its constants carry values)).
        // Ray2d.intersectWhen(1): not translated, its class isn't translated (it uses Axis, which isn't translated (its constants carry values)).
        // Ray2d.intersect(2): not translated, its class isn't translated (it uses Axis, which isn't translated (its constants carry values)).
        // Ray2d.toString(0): not translated, its class isn't translated (it uses Axis, which isn't translated (its constants carry values)).
    }

    /// `Ray3d` (team/creative/creativecore/common/util/math/geo/Ray3d.java)
    pub mod ray3d {
        use super::*;
        // The class itself isn't translated: it uses Vec3d, which isn't translated (it extends `VecNd`). Its static parts are below.
        // Ray3d.Ray3d(2): not translated, its class isn't translated (it uses Vec3d, which isn't translated (it extends `VecNd`)).
        // Ray3d.Ray3d(3): not translated, its class isn't translated (it uses Vec3d, which isn't translated (it extends `VecNd`)).
        // Ray3d.Ray3d(2): not translated, its class isn't translated (it uses Vec3d, which isn't translated (it extends `VecNd`)).
        // Ray3d.set(6): not translated, its class isn't translated (it uses Vec3d, which isn't translated (it extends `VecNd`)).
        // Ray3d.getT(2): not translated, its class isn't translated (it uses Vec3d, which isn't translated (it extends `VecNd`)).
        // Ray3d.get(1): not translated, its class isn't translated (it uses Vec3d, which isn't translated (it extends `VecNd`)).
    }

    /// `Ray3f` (team/creative/creativecore/common/util/math/geo/Ray3f.java)
    pub mod ray3f {
        use super::*;
        // The class itself isn't translated: it uses Vec3f, which isn't translated (it extends `VecNf`). Its static parts are below.
        // Ray3f.Ray3f(2): not translated, its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)).
        // Ray3f.Ray3f(2): not translated, its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)).
        // Ray3f.set(6): not translated, its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)).
        // Ray3f.getT(2): not translated, its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)).
        // Ray3f.get(1): not translated, its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)).
    }

    /// `Rect` (team/creative/creativecore/common/util/math/geo/Rect.java)
    pub mod rect {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Rect {
            pub min_x: f64,
            pub min_y: f64,
            pub max_x: f64,
            pub max_y: f64,
        }
        impl Rect {
            pub fn new(x: f64, y: f64, x2: f64, y2: f64) -> Self {
                let mut s = Self { min_x: x, min_y: y, max_x: x2, max_y: y2 };
                s.min_x = x;
                s.min_y = y;
                s.max_x = x2;
                s.max_y = y2;
                s
            }
            pub fn intersects(&self, other: &rect::Rect) -> bool {
                return (((self.min_x < other.max_x) && (self.max_x > other.min_x)) && (self.min_y < other.max_y)) && (self.max_y > other.min_y);
            }
            pub fn intersection(&self, other: &rect::Rect) -> Option<rect::Rect> {
                if self.intersects(other) {
                    let x: f64 = self.min_x.max(other.min_x);
                    let y: f64 = self.min_y.max(other.min_y);
                    let x2: f64 = self.max_x.min(other.max_x);
                    let y2: f64 = self.max_y.min(other.max_y);
                    return Some(rect::Rect::new(x, y, x2, y2));
                } else {
                    return None;
                }
                #[allow(unreachable_code)]
                unreachable!()
            }
            pub fn r#move(&mut self, x: f64, y: f64) {
                self.min_x += x;
                self.max_x += x;
                self.min_y += y;
                self.max_y += y;
            }
            pub fn shrink(&mut self, shrink: f64) {
                self.min_x += shrink;
                self.min_y += shrink;
                self.max_x -= shrink;
                self.max_y -= shrink;
            }
            pub fn grow(&mut self, grow: f64) {
                self.min_x -= grow;
                self.min_y -= grow;
                self.max_x += grow;
                self.max_y += grow;
            }
            pub fn get_width(&self) -> f64 {
                return self.max_x - self.min_x;
            }
            pub fn get_height(&self) -> f64 {
                return self.max_y - self.min_y;
            }
            // Rect.getSize(1): not translated, it uses Axis, which isn't translated (its constants carry values).
            pub fn copy(&self) -> rect::Rect {
                return rect::Rect::new(self.min_x, self.min_y, self.max_x, self.max_y);
            }
            // Rect.child(4): not translated, it's overloaded with the same number of parameters.
            // Rect.child(4): not translated, it's overloaded with the same number of parameters.
            // Rect.child(4): not translated, it's overloaded with the same number of parameters.
            pub fn inside(&self, x: f64, y: f64) -> bool {
                return (((x >= self.min_x) && (x < self.max_x)) && (y >= self.min_y)) && (y < self.max_y);
            }
            pub fn to_string(&self) -> String {
                return format!("[{},{},{},{}]", j_float(self.min_x), j_float(self.min_y), j_float(self.max_x), j_float(self.max_y));
            }
            pub fn scale(&mut self, scale: f64) {
                self.min_x *= scale;
                self.min_y *= scale;
                self.max_x *= scale;
                self.max_y *= scale;
            }
        }
    }

    /// `VectorFan` (team/creative/creativecore/common/util/math/geo/VectorFan.java)
    pub mod vector_fan {
        use super::*;
        // The class itself isn't translated: it uses Vec3f, which isn't translated (it extends `VecNf`). Its static parts are below.
        pub const EPSILON: f32 = 1.0e-4f32;
        // VectorFan.isInside(6): not translated, it uses NormalPlaneF, which isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)).
        // VectorFan.isInside(3): not translated, it uses NormalPlaneF, which isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)).
        // VectorFan.isPointBetween(3): not translated, it uses Vec3f, which isn't translated (it extends `VecNf`).
        // VectorFan.VectorFan(1): not translated, its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)).
        // VectorFan.getCoords(0): not translated, its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)).
        // VectorFan.get(1): not translated, its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)).
        // VectorFan.count(0): not translated, its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)).
        // VectorFan.cutMinMax(7): not translated, its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)).
        // VectorFan.doMinMaxLate(0): not translated, its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)).
        // VectorFan.doSimpleRendering(0): not translated, its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)).
        // VectorFan.add(2): not translated, its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)).
        // VectorFan.set(1): not translated, its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)).
        // VectorFan.set(1): not translated, its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)).
        // VectorFan.cutWithoutCopy(1): not translated, its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)).
        // VectorFan.cutWithoutCopy(1): not translated, its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)).
        // VectorFan.cutWithoutCopy(1): not translated, its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)).
        // VectorFan.cutWithoutCopy(1): not translated, its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)).
        // VectorFan.isEmpty(0): not translated, its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)).
        // VectorFan.cutInternal(2): not translated, its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)).
        // VectorFan.cut(1): not translated, its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)).
        // VectorFan.move(3): not translated, its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)).
        // VectorFan.scale(1): not translated, its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)).
        // VectorFan.divide(1): not translated, its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)).
        // VectorFan.intersects(2): not translated, its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)).
        // VectorFan.copy(0): not translated, its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)).
        // VectorFan.equals(1): not translated, its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)).
        // VectorFan.equals(4): not translated, it uses Vec3f, which isn't translated (it extends `VecNf`).
        // VectorFan.equalsIgnoreOrder(2): not translated, its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)).
        // VectorFan.calculateIntercept(4): not translated, it uses Vec3d, which isn't translated (it extends `VecNd`).
        // VectorFan.calculateIntercept(1): not translated, its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)).
        // VectorFan.createNormal(0): not translated, its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)).
        // VectorFan.createPlane(0): not translated, its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)).
        // VectorFan.createPlane(1): not translated, its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)).
        // VectorFan.forAllEdges(1): not translated, its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)).
        // VectorFan.isInside(1): not translated, its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)).
        // VectorFan.intersect2d(5): not translated, its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)).
        // VectorFan.isInside2d(4): not translated, its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)).
        // VectorFan.cut2d(5): not translated, its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)).
        // VectorFan.cut2d(5): not translated, its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)).
        // VectorFan.cut2d(5): not translated, its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)).
        // VectorFan.combine(2): not translated, its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)).
        // VectorFan.checkConvexAndShrink(2): not translated, it uses Axis, which isn't translated (its constants carry values).
        // VectorFan.toString(0): not translated, its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)).
    }

    /// `VectorFan.ParallelException` (team/creative/creativecore/common/util/math/geo/VectorFan.java)
    pub mod vector_fan_parallel_exception {
        use super::*;
        // The class itself isn't translated: it extends `Exception`. Its static parts are below.
    }

    /// `CosineInterpolation` (team/creative/creativecore/common/util/math/interpolation/CosineInterpolation.java)
    pub mod cosine_interpolation {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // CosineInterpolation.CosineInterpolation(1): not translated, its class isn't translated (it's generic).
        // CosineInterpolation.CosineInterpolation(2): not translated, its class isn't translated (it's generic).
        // CosineInterpolation.CosineInterpolation(1): not translated, its class isn't translated (it's generic).
        // CosineInterpolation.CosineInterpolation(2): not translated, its class isn't translated (it's generic).
        // CosineInterpolation.estimateDistance(0): not translated, its class isn't translated (it's generic).
        // CosineInterpolation.valueAt(4): not translated, its class isn't translated (it's generic).
    }

    /// `CubicInterpolation` (team/creative/creativecore/common/util/math/interpolation/CubicInterpolation.java)
    pub mod cubic_interpolation {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // CubicInterpolation.CubicInterpolation(2): not translated, its class isn't translated (it's generic).
        // CubicInterpolation.CubicInterpolation(1): not translated, its class isn't translated (it's generic).
        // CubicInterpolation.CubicInterpolation(3): not translated, its class isn't translated (it's generic).
        // CubicInterpolation.CubicInterpolation(4): not translated, its class isn't translated (it's generic).
        // CubicInterpolation.CubicInterpolation(1): not translated, its class isn't translated (it's generic).
        // CubicInterpolation.getValue(2): not translated, its class isn't translated (it's generic).
        // CubicInterpolation.estimateDistance(0): not translated, its class isn't translated (it's generic).
        // CubicInterpolation.valueAt(4): not translated, its class isn't translated (it's generic).
    }

    /// `HermiteInterpolation` (team/creative/creativecore/common/util/math/interpolation/HermiteInterpolation.java)
    pub mod hermite_interpolation {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // HermiteInterpolation.HermiteInterpolation(4): not translated, its class isn't translated (it's generic).
        // HermiteInterpolation.HermiteInterpolation(3): not translated, its class isn't translated (it's generic).
        // HermiteInterpolation.HermiteInterpolation(2): not translated, its class isn't translated (it's generic).
        // HermiteInterpolation.HermiteInterpolation(3): not translated, its class isn't translated (it's generic).
        // HermiteInterpolation.HermiteInterpolation(3): not translated, its class isn't translated (it's generic).
        // HermiteInterpolation.HermiteInterpolation(5): not translated, its class isn't translated (it's generic).
        // HermiteInterpolation.HermiteInterpolation(6): not translated, its class isn't translated (it's generic).
        // HermiteInterpolation.HermiteInterpolation(2): not translated, its class isn't translated (it's generic).
        // HermiteInterpolation.HermiteInterpolation(2): not translated, its class isn't translated (it's generic).
        // HermiteInterpolation.HermiteInterpolation(1): not translated, its class isn't translated (it's generic).
        // HermiteInterpolation.HermiteInterpolation(1): not translated, its class isn't translated (it's generic).
        // HermiteInterpolation.HermiteInterpolation(3): not translated, its class isn't translated (it's generic).
        // HermiteInterpolation.HermiteInterpolation(4): not translated, its class isn't translated (it's generic).
        // HermiteInterpolation.valueAt(4): not translated, its class isn't translated (it's generic).
    }

    /// `HermiteInterpolation.Tension` (team/creative/creativecore/common/util/math/interpolation/HermiteInterpolation.java)
    pub mod hermite_interpolation_tension {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // HermiteInterpolation.Tension.Tension(1): not translated, its class isn't translated (its constants carry values).
    }

    /// `Interpolation` (team/creative/creativecore/common/util/math/interpolation/Interpolation.java)
    pub mod interpolation {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // Interpolation.Interpolation(2): not translated, its class isn't translated (it's generic).
        // Interpolation.Interpolation(1): not translated, its class isn't translated (it's generic).
        // Interpolation.Interpolation(2): not translated, its class isn't translated (it's generic).
        // Interpolation.Interpolation(1): not translated, its class isn't translated (it's generic).
        // Interpolation.Interpolation(1): not translated, its class isn't translated (it's generic).
        // Interpolation.getValue(2): not translated, its class isn't translated (it's generic).
        // Interpolation.valueAt(1): not translated, its class isn't translated (it's generic).
        // Interpolation.createVec(0): not translated, its class isn't translated (it's generic).
        // Interpolation.valueAt(4): not translated, it has no body.
        // Interpolation.estimateDistance(0): not translated, it has no body.
    }

    /// `LinearInterpolation` (team/creative/creativecore/common/util/math/interpolation/LinearInterpolation.java)
    pub mod linear_interpolation {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // LinearInterpolation.LinearInterpolation(2): not translated, its class isn't translated (it's generic).
        // LinearInterpolation.LinearInterpolation(1): not translated, its class isn't translated (it's generic).
        // LinearInterpolation.LinearInterpolation(2): not translated, its class isn't translated (it's generic).
        // LinearInterpolation.LinearInterpolation(1): not translated, its class isn't translated (it's generic).
        // LinearInterpolation.LinearInterpolation(1): not translated, its class isn't translated (it's generic).
        // LinearInterpolation.estimateDistance(0): not translated, its class isn't translated (it's generic).
        // LinearInterpolation.valueAt(4): not translated, its class isn't translated (it's generic).
    }

    /// `Maths` (team/creative/creativecore/common/util/math/Maths.java)
    pub mod maths {
        use super::*;
        pub const EPSILON: f32 = 0.001f32;
        pub const EPSILON_UP: f32 = 999.99994f32;
        pub const EPSILON_DOUBLE: f64 = 0.001f64;
        pub const EPSILON_UP_DOUBLE: f64 = 1000.0f64;
        pub const EPSILON_PRECISE: f32 = 1.0e-5f32;
        pub const EPSILON_PRECISE_UP: f32 = 100000.0f32;
        pub const EPSILON_PRECISE_DOUBLE: f64 = 1.0e-5f64;
        pub const EPSILON_PRECISE_UP_DOUBLE: f64 = 99999.99999999999f64;
        #[derive(Clone, Debug)]
        pub struct Maths {
        }
        impl Maths {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // Maths.within(3): not translated, it calls Maths.within, which isn't translated.
        // Maths.within(4): not translated, it calls Maths.greaterThanOrEquals, which isn't translated.
        // Maths.smallerThanOrEquals(2): not translated, it's overloaded with the same number of parameters.
        // Maths.smallerThanOrEquals(3): not translated, it's overloaded with the same number of parameters.
        // Maths.greaterThanOrEquals(2): not translated, it's overloaded with the same number of parameters.
        // Maths.greaterThanOrEquals(3): not translated, it's overloaded with the same number of parameters.
        // Maths.equals(2): not translated, it's overloaded with the same number of parameters.
        // Maths.equals(3): not translated, it's overloaded with the same number of parameters.
        // Maths.equals(2): not translated, it's overloaded with the same number of parameters.
        // Maths.equals(3): not translated, it's overloaded with the same number of parameters.
        pub fn tick_to_ms(ticks: i32) -> i64 {
            return (ticks as i64) * 50i64;
        }
        pub fn ms_to_tick(ms: i64) -> i32 {
            return ((ms / 50i64) as i32);
        }
        // Maths.round(1): not translated, it's overloaded with the same number of parameters.
        // Maths.round(1): not translated, it's overloaded with the same number of parameters.
        pub fn safe_divide(v1: f64, v2: f64) -> f64 {
            return if (v1 != 0.0f64) && (v2 != 0.0f64) { v1 / v2 } else { 0.0f64 };
        }
        pub fn safe_percent(v1: i64, v2: i64) -> i64 {
            return if (v1 != 0i64) && (v2 != 0i64) { v1 % v2 } else { 0i64 };
        }
        pub fn safe_round(value: f64) -> f32 {
            return if value != 0.0f64 { (j_round_d(value * 100.0f64) as f32) / 100.0f32 } else { 0.0f32 };
        }
        // Maths.min(3): not translated, it's overloaded with the same number of parameters.
        // Maths.min(3): not translated, it's overloaded with the same number of parameters.
        // Maths.min(3): not translated, it's overloaded with the same number of parameters.
        // Maths.min(3): not translated, it's overloaded with the same number of parameters.
        // Maths.max(3): not translated, it's overloaded with the same number of parameters.
        // Maths.max(3): not translated, it's overloaded with the same number of parameters.
        // Maths.max(3): not translated, it's overloaded with the same number of parameters.
        // Maths.max(3): not translated, it's overloaded with the same number of parameters.
    }

    /// `ChildVecOrigin` (team/creative/creativecore/common/util/math/matrix/ChildVecOrigin.java)
    pub mod child_vec_origin {
        use super::*;
        // The class itself isn't translated: it extends `VecOrigin`. Its static parts are below.
        // ChildVecOrigin.ChildVecOrigin(2): not translated, its class isn't translated (it extends `VecOrigin`).
        // ChildVecOrigin.onlyRotateWithoutCenter(1): not translated, its class isn't translated (it extends `VecOrigin`).
        // ChildVecOrigin.transformPointToWorld(1): not translated, its class isn't translated (it extends `VecOrigin`).
        // ChildVecOrigin.transformPointToFakeWorld(1): not translated, its class isn't translated (it extends `VecOrigin`).
        // ChildVecOrigin.translationCombined(1): not translated, its class isn't translated (it extends `VecOrigin`).
        // ChildVecOrigin.hasChanged(0): not translated, its class isn't translated (it extends `VecOrigin`).
        // ChildVecOrigin.getParent(0): not translated, its class isn't translated (it extends `VecOrigin`).
        // ChildVecOrigin.createInternalCopy(0): not translated, its class isn't translated (it extends `VecOrigin`).
    }

    /// `IntMatrix3` (team/creative/creativecore/common/util/math/matrix/IntMatrix3.java)
    pub mod int_matrix3 {
        use super::*;
        // The class itself isn't translated: it implements `IntMatrix3c`. Its static parts are below.
        pub fn fma(a: i32, b: i32, c: i32) -> i32 {
            return (a * b) + c;
        }
        // IntMatrix3.IntMatrix3(0): not translated, its class isn't translated (it implements `IntMatrix3c`).
        // IntMatrix3.IntMatrix3(1): not translated, its class isn't translated (it implements `IntMatrix3c`).
        // IntMatrix3.IntMatrix3(9): not translated, its class isn't translated (it implements `IntMatrix3c`).
        // IntMatrix3.IntMatrix3(1): not translated, its class isn't translated (it implements `IntMatrix3c`).
        // IntMatrix3.IntMatrix3(2): not translated, its class isn't translated (it implements `IntMatrix3c`).
        // IntMatrix3.getX(1): not translated, its class isn't translated (it implements `IntMatrix3c`).
        // IntMatrix3.getX(1): not translated, its class isn't translated (it implements `IntMatrix3c`).
        // IntMatrix3.getX(3): not translated, its class isn't translated (it implements `IntMatrix3c`).
        // IntMatrix3.getX(3): not translated, its class isn't translated (it implements `IntMatrix3c`).
        // IntMatrix3.getY(1): not translated, its class isn't translated (it implements `IntMatrix3c`).
        // IntMatrix3.getY(1): not translated, its class isn't translated (it implements `IntMatrix3c`).
        // IntMatrix3.getY(3): not translated, its class isn't translated (it implements `IntMatrix3c`).
        // IntMatrix3.getY(3): not translated, its class isn't translated (it implements `IntMatrix3c`).
        // IntMatrix3.getZ(1): not translated, its class isn't translated (it implements `IntMatrix3c`).
        // IntMatrix3.getZ(1): not translated, its class isn't translated (it implements `IntMatrix3c`).
        // IntMatrix3.getZ(3): not translated, its class isn't translated (it implements `IntMatrix3c`).
        // IntMatrix3.getZ(3): not translated, its class isn't translated (it implements `IntMatrix3c`).
        // IntMatrix3.transform(1): not translated, its class isn't translated (it implements `IntMatrix3c`).
        // IntMatrix3.transform(1): not translated, its class isn't translated (it implements `IntMatrix3c`).
        // IntMatrix3.transform(1): not translated, its class isn't translated (it implements `IntMatrix3c`).
        // IntMatrix3.transform(1): not translated, its class isn't translated (it implements `IntMatrix3c`).
        // IntMatrix3.transform(1): not translated, its class isn't translated (it implements `IntMatrix3c`).
        // IntMatrix3.mul(1): not translated, its class isn't translated (it implements `IntMatrix3c`).
        // IntMatrix3.mul(2): not translated, its class isn't translated (it implements `IntMatrix3c`).
        // IntMatrix3.equals(1): not translated, its class isn't translated (it implements `IntMatrix3c`).
        // IntMatrix3.toString(0): not translated, its class isn't translated (it implements `IntMatrix3c`).
        // IntMatrix3.m00(0): not translated, its class isn't translated (it implements `IntMatrix3c`).
        // IntMatrix3.m01(0): not translated, its class isn't translated (it implements `IntMatrix3c`).
        // IntMatrix3.m02(0): not translated, its class isn't translated (it implements `IntMatrix3c`).
        // IntMatrix3.m10(0): not translated, its class isn't translated (it implements `IntMatrix3c`).
        // IntMatrix3.m11(0): not translated, its class isn't translated (it implements `IntMatrix3c`).
        // IntMatrix3.m12(0): not translated, its class isn't translated (it implements `IntMatrix3c`).
        // IntMatrix3.m20(0): not translated, its class isn't translated (it implements `IntMatrix3c`).
        // IntMatrix3.m21(0): not translated, its class isn't translated (it implements `IntMatrix3c`).
        // IntMatrix3.m22(0): not translated, its class isn't translated (it implements `IntMatrix3c`).
        // IntMatrix3.invertedX(0): not translated, its class isn't translated (it implements `IntMatrix3c`).
        // IntMatrix3.getX(3): not translated, its class isn't translated (it implements `IntMatrix3c`).
        // IntMatrix3.invertedY(0): not translated, its class isn't translated (it implements `IntMatrix3c`).
        // IntMatrix3.getY(3): not translated, its class isn't translated (it implements `IntMatrix3c`).
        // IntMatrix3.invertedZ(0): not translated, its class isn't translated (it implements `IntMatrix3c`).
        // IntMatrix3.getZ(3): not translated, its class isn't translated (it implements `IntMatrix3c`).
        // IntMatrix3.getAsArray(0): not translated, its class isn't translated (it implements `IntMatrix3c`).
        // IntMatrix3.isIdentity(0): not translated, its class isn't translated (it implements `IntMatrix3c`).
    }

    /// `IntMatrix3c` (team/creative/creativecore/common/util/math/matrix/IntMatrix3c.java)
    pub mod int_matrix3c {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IntMatrix3c.m00(0): not translated, it has no body.
        // IntMatrix3c.m01(0): not translated, it has no body.
        // IntMatrix3c.m02(0): not translated, it has no body.
        // IntMatrix3c.m10(0): not translated, it has no body.
        // IntMatrix3c.m11(0): not translated, it has no body.
        // IntMatrix3c.m12(0): not translated, it has no body.
        // IntMatrix3c.m20(0): not translated, it has no body.
        // IntMatrix3c.m21(0): not translated, it has no body.
        // IntMatrix3c.m22(0): not translated, it has no body.
        // IntMatrix3c.getX(1): not translated, it has no body.
        // IntMatrix3c.getX(1): not translated, it has no body.
        // IntMatrix3c.getX(3): not translated, it has no body.
        // IntMatrix3c.getX(3): not translated, it has no body.
        // IntMatrix3c.getY(1): not translated, it has no body.
        // IntMatrix3c.getY(1): not translated, it has no body.
        // IntMatrix3c.getY(3): not translated, it has no body.
        // IntMatrix3c.getY(3): not translated, it has no body.
        // IntMatrix3c.getZ(1): not translated, it has no body.
        // IntMatrix3c.getZ(1): not translated, it has no body.
        // IntMatrix3c.getZ(3): not translated, it has no body.
        // IntMatrix3c.getZ(3): not translated, it has no body.
        // IntMatrix3c.transform(1): not translated, it has no body.
        // IntMatrix3c.transform(1): not translated, it has no body.
        // IntMatrix3c.transform(1): not translated, it has no body.
        // IntMatrix3c.transform(1): not translated, it has no body.
        // IntMatrix3c.transform(1): not translated, it has no body.
        // IntMatrix3c.invertedX(0): not translated, it has no body.
        // IntMatrix3c.getX(3): not translated, it has no body.
        // IntMatrix3c.invertedY(0): not translated, it has no body.
        // IntMatrix3c.getY(3): not translated, it has no body.
        // IntMatrix3c.invertedZ(0): not translated, it has no body.
        // IntMatrix3c.getZ(3): not translated, it has no body.
        // IntMatrix3c.getAsArray(0): not translated, it has no body.
        // IntMatrix3c.isIdentity(0): not translated, it has no body.
    }

    /// `IVecOrigin` (team/creative/creativecore/common/util/math/matrix/IVecOrigin.java)
    pub mod i_vec_origin {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IVecOrigin.offX(0): not translated, it has no body.
        // IVecOrigin.offY(0): not translated, it has no body.
        // IVecOrigin.offZ(0): not translated, it has no body.
        // IVecOrigin.rotX(0): not translated, it has no body.
        // IVecOrigin.rotY(0): not translated, it has no body.
        // IVecOrigin.rotZ(0): not translated, it has no body.
        // IVecOrigin.offXLast(0): not translated, it has no body.
        // IVecOrigin.offYLast(0): not translated, it has no body.
        // IVecOrigin.offZLast(0): not translated, it has no body.
        // IVecOrigin.rotXLast(0): not translated, it has no body.
        // IVecOrigin.rotYLast(0): not translated, it has no body.
        // IVecOrigin.rotZLast(0): not translated, it has no body.
        // IVecOrigin.isRotated(0): not translated, it has no body.
        // IVecOrigin.offX(1): not translated, it has no body.
        // IVecOrigin.offY(1): not translated, it has no body.
        // IVecOrigin.offZ(1): not translated, it has no body.
        // IVecOrigin.off(3): not translated, it has no body.
        // IVecOrigin.rotX(1): not translated, it has no body.
        // IVecOrigin.rotY(1): not translated, it has no body.
        // IVecOrigin.rotZ(1): not translated, it has no body.
        // IVecOrigin.rot(3): not translated, it has no body.
        // IVecOrigin.center(0): not translated, it has no body.
        // IVecOrigin.setCenter(1): not translated, it has no body.
        // IVecOrigin.rotation(0): not translated, it has no body.
        // IVecOrigin.rotationInv(0): not translated, it has no body.
        // IVecOrigin.translation(0): not translated, it has no body.
        // IVecOrigin.tick(0): not translated, it has no body.
        // IVecOrigin.getParent(0): not translated, it has no body.
        // IVecOrigin.set(1): not translated, its class isn't translated (it's an interface).
        // IVecOrigin.translationCombined(1): not translated, its class isn't translated (it's an interface).
        // IVecOrigin.onlyRotateWithoutCenter(1): not translated, its class isn't translated (it's an interface).
        // IVecOrigin.transformPointToWorld(1): not translated, its class isn't translated (it's an interface).
        // IVecOrigin.transformPointToFakeWorld(1): not translated, its class isn't translated (it's an interface).
        // IVecOrigin.transformPointToWorld(1): not translated, its class isn't translated (it's an interface).
        // IVecOrigin.transformPointToFakeWorld(1): not translated, its class isn't translated (it's an interface).
        // IVecOrigin.transformPointToWorld(1): not translated, its class isn't translated (it's an interface).
        // IVecOrigin.transformPointToFakeWorld(1): not translated, its class isn't translated (it's an interface).
        // IVecOrigin.transformPointToWorld(1): not translated, its class isn't translated (it's an interface).
        // IVecOrigin.transformPointToFakeWorld(1): not translated, its class isn't translated (it's an interface).
        // IVecOrigin.getAABB(1): not translated, its class isn't translated (it's an interface).
        // IVecOrigin.getOBB(1): not translated, its class isn't translated (it's an interface).
        // IVecOrigin.getAABB(1): not translated, its class isn't translated (it's an interface).
        // IVecOrigin.getOBB(1): not translated, its class isn't translated (it's an interface).
        // IVecOrigin.hasChanged(0): not translated, its class isn't translated (it's an interface).
        // IVecOrigin.copy(0): not translated, it has no body.
    }

    /// `Matrix3` (team/creative/creativecore/common/util/math/matrix/Matrix3.java)
    pub mod matrix3 {
        use super::*;
        pub const EPS: f64 = 1.110223024e-16f64;
        #[derive(Clone, Debug)]
        pub struct Matrix3 {
            pub m00: f64,
            pub m01: f64,
            pub m02: f64,
            pub m10: f64,
            pub m11: f64,
            pub m12: f64,
            pub m20: f64,
            pub m21: f64,
            pub m22: f64,
        }
        impl Matrix3 {
            pub fn new() -> Self {
                let mut s = Self::new_9(0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64);
                s
            }
            pub fn new_9(m00: f64, m01: f64, m02: f64, m10: f64, m11: f64, m12: f64, m20: f64, m21: f64, m22: f64) -> Self {
                let mut s = Self { m00: m00, m01: m01, m02: m02, m10: m10, m11: m11, m12: m12, m20: m20, m21: m21, m22: m22 };
                s.m00 = m00;
                s.m01 = m01;
                s.m02 = m02;
                s.m10 = m10;
                s.m11 = m11;
                s.m12 = m12;
                s.m20 = m20;
                s.m21 = m21;
                s.m22 = m22;
                s
            }
            // Matrix3.Matrix3(1): not translated, it's overloaded with the same number of parameters.
            pub fn to_string(&self) -> String {
                return format!("{}, {}, {}\n{}, {}, {}\n{}, {}, {}\n", j_float(self.m00), j_float(self.m01), j_float(self.m02), j_float(self.m10), j_float(self.m11), j_float(self.m12), j_float(self.m20), j_float(self.m21), j_float(self.m22));
            }
            pub fn set_identity(&mut self) {
                self.m00 = 1.0f64;
                self.m01 = 0.0f64;
                self.m02 = 0.0f64;
                self.m10 = 0.0f64;
                self.m11 = 1.0f64;
                self.m12 = 0.0f64;
                self.m20 = 0.0f64;
                self.m21 = 0.0f64;
                self.m22 = 1.0f64;
            }
            // Matrix3.setScale(1): not translated, it calls Matrix3.getScaleRotate, which isn't translated.
            // Matrix3.setElement(3): not translated, a switch case that falls through.
            // Matrix3.getElement(2): not translated, a switch case that falls through.
            // Matrix3.getScale(0): not translated, it calls Matrix3.getScaleRotate, which isn't translated.
            // Matrix3.add(1): not translated, it's overloaded with the same number of parameters.
            // Matrix3.add(1): not translated, it's overloaded with the same number of parameters.
            pub fn sub(&mut self, m1: &matrix3::Matrix3) {
                self.m00 = self.m00 - m1.m00;
                self.m01 = self.m01 - m1.m01;
                self.m02 = self.m02 - m1.m02;
                self.m10 = self.m10 - m1.m10;
                self.m11 = self.m11 - m1.m11;
                self.m12 = self.m12 - m1.m12;
                self.m20 = self.m20 - m1.m20;
                self.m21 = self.m21 - m1.m21;
                self.m22 = self.m22 - m1.m22;
            }
            pub fn transpose(&mut self) {
                let mut temp: f64 = self.m10;
                self.m10 = self.m01;
                self.m01 = temp;
                temp = self.m20;
                self.m20 = self.m02;
                self.m02 = temp;
                temp = self.m21;
                self.m21 = self.m12;
                self.m12 = temp;
            }
            // Matrix3.set(1): not translated, it's overloaded with the same number of parameters.
            // Matrix3.invert(0): not translated, it calls Matrix3.luDecomposition, which isn't translated.
            // Matrix3.invert(1): not translated, it calls Matrix3.luDecomposition, which isn't translated.
            pub fn determinant(&self) -> f64 {
                return ((self.m00 * ((self.m11 * self.m22) - (self.m12 * self.m21))) + (self.m01 * ((self.m12 * self.m20) - (self.m10 * self.m22)))) + (self.m02 * ((self.m10 * self.m21) - (self.m11 * self.m20)));
            }
            // Matrix3.set(1): not translated, it's overloaded with the same number of parameters.
            pub fn rot_x(&mut self, angle: f64) {
                let sin_angle: f64 = angle.sin();
                let cos_angle: f64 = angle.cos();
                self.m00 = 1.0f64;
                self.m01 = 0.0f64;
                self.m02 = 0.0f64;
                self.m10 = 0.0f64;
                self.m11 = cos_angle;
                self.m12 = -sin_angle;
                self.m20 = 0.0f64;
                self.m21 = sin_angle;
                self.m22 = cos_angle;
            }
            pub fn rot_y(&mut self, angle: f64) {
                let sin_angle: f64 = angle.sin();
                let cos_angle: f64 = angle.cos();
                self.m00 = cos_angle;
                self.m01 = 0.0f64;
                self.m02 = sin_angle;
                self.m10 = 0.0f64;
                self.m11 = 1.0f64;
                self.m12 = 0.0f64;
                self.m20 = -sin_angle;
                self.m21 = 0.0f64;
                self.m22 = cos_angle;
            }
            pub fn rot_z(&mut self, angle: f64) {
                let sin_angle: f64 = angle.sin();
                let cos_angle: f64 = angle.cos();
                self.m00 = cos_angle;
                self.m01 = -sin_angle;
                self.m02 = 0.0f64;
                self.m10 = sin_angle;
                self.m11 = cos_angle;
                self.m12 = 0.0f64;
                self.m20 = 0.0f64;
                self.m21 = 0.0f64;
                self.m22 = 1.0f64;
            }
            // Matrix3.mul(1): not translated, it's overloaded with the same number of parameters.
            // Matrix3.mul(1): not translated, it's overloaded with the same number of parameters.
            // Matrix3.mulNormalize(1): not translated, it calls Matrix3.compute_svd, which isn't translated.
            // Matrix3.normalize(0): not translated, it calls Matrix3.getScaleRotate, which isn't translated.
            pub fn normalize_cp(&mut self) {
                let mut mag: f64 = 1.0f64 / ((((self.m00 * self.m00) + (self.m10 * self.m10)) + (self.m20 * self.m20)).sqrt());
                self.m00 *= mag;
                self.m10 *= mag;
                self.m20 *= mag;
                mag = 1.0f64 / ((((self.m01 * self.m01) + (self.m11 * self.m11)) + (self.m21 * self.m21)).sqrt());
                self.m01 *= mag;
                self.m11 *= mag;
                self.m21 *= mag;
                self.m02 = (self.m10 * self.m21) - (self.m11 * self.m20);
                self.m12 = (self.m01 * self.m20) - (self.m00 * self.m21);
                self.m22 = (self.m00 * self.m11) - (self.m01 * self.m10);
            }
            // Matrix3.equals(1): not translated, it's overloaded with the same number of parameters.
            // Matrix3.equals(1): not translated, it's overloaded with the same number of parameters.
            // Matrix3.transform(1): not translated, it's overloaded with the same number of parameters.
            // Matrix3.transform(1): not translated, it's overloaded with the same number of parameters.
            // Matrix3.getScaleRotate(2): not translated, it calls Matrix3.compute_svd, which isn't translated.
        }
        // Matrix3.luDecomposition(2): not translated, `--` inside an expression.
        pub fn lu_backsubstitution(matrix1: &[f64], row_perm: &[i32], matrix2: &mut [f64]) {
            let rp: i32 = 0;
            for k in 0..3 {
                let cv: i32 = k;
                let mut ii: i32 = -1;
                for i in 0..3 {
                    let ip: i32 = row_perm[(rp + i) as usize];
                    let mut sum: f64 = matrix2[(cv + (3 * ip)) as usize];
                    matrix2[(cv + (3 * ip)) as usize] = matrix2[(cv + (3 * i)) as usize];
                    if ii >= 0 {
                        let mut rv: i32 = i * 3;
                        for j in ii..=(i - 1) {
                            sum -= matrix1[(rv + j) as usize] * matrix2[(cv + (3 * j)) as usize];
                        }
                    } else if sum != 0.0f64 {
                        ii = i;
                    }
                    matrix2[(cv + (3 * i)) as usize] = sum;
                }
                let mut rv: i32 = 6;
                matrix2[(cv + 6) as usize] = matrix2[(cv + 6) as usize] / matrix1[(rv + 2) as usize];
                rv -= 3;
                matrix2[(cv + 3) as usize] = (matrix2[(cv + 3) as usize] - (matrix1[(rv + 2) as usize] * matrix2[(cv + 6) as usize])) / matrix1[(rv + 1) as usize];
                rv -= 3;
                matrix2[cv as usize] = ((matrix2[cv as usize] - (matrix1[(rv + 1) as usize] * matrix2[(cv + 3) as usize])) - (matrix1[(rv + 2) as usize] * matrix2[(cv + 6) as usize])) / matrix1[rv as usize];
            }
        }
        // Matrix3.compute_svd(3): not translated, it changes an array or object through a second name (tmp).
        pub fn svd_reorder(m: &[f64], t1: &[f64], t2: &mut [f64], scales: &mut [f64], out_rot: &mut [f64], out_scale: &mut [f64]) {
            let mut out: Vec<i32> = vec![0i32; 3];
            let mut mag: Vec<f64> = vec![0.0f64; 3];
            let mut rot: Vec<f64> = vec![0.0f64; 9];
            if scales[0] < 0.0f64 {
                scales[0] = -scales[0];
                t2[0] = -t2[0];
                t2[1] = -t2[1];
                t2[2] = -t2[2];
            }
            if scales[1] < 0.0f64 {
                scales[1] = -scales[1];
                t2[3] = -t2[3];
                t2[4] = -t2[4];
                t2[5] = -t2[5];
            }
            if scales[2] < 0.0f64 {
                scales[2] = -scales[2];
                t2[6] = -t2[6];
                t2[7] = -t2[7];
                t2[8] = -t2[8];
            }
            matrix3::mat_mul(t1, t2, &mut rot);
            if (matrix3::almost_equal(scales[0].abs(), scales[1].abs())) && (matrix3::almost_equal(scales[1].abs(), scales[2].abs())) {
                for i in 0..9 {
                    out_rot[i as usize] = rot[i as usize];
                }
                for var25 in 0..3 {
                    out_scale[var25 as usize] = scales[var25 as usize];
                }
            } else {
                if scales[0] > scales[1] {
                    if scales[0] > scales[2] {
                        if scales[2] > scales[1] {
                            out[0] = 0;
                            out[1] = 2;
                            out[2] = 1;
                        } else {
                            out[0] = 0;
                            out[1] = 1;
                            out[2] = 2;
                        }
                    } else {
                        out[0] = 2;
                        out[1] = 0;
                        out[2] = 1;
                    }
                } else if scales[1] > scales[2] {
                    if scales[2] > scales[0] {
                        out[0] = 1;
                        out[1] = 2;
                        out[2] = 0;
                    } else {
                        out[0] = 1;
                        out[1] = 0;
                        out[2] = 2;
                    }
                } else {
                    out[0] = 2;
                    out[1] = 1;
                    out[2] = 0;
                }
                mag[0] = ((m[0] * m[0]) + (m[1] * m[1])) + (m[2] * m[2]);
                mag[1] = ((m[3] * m[3]) + (m[4] * m[4])) + (m[5] * m[5]);
                mag[2] = ((m[6] * m[6]) + (m[7] * m[7])) + (m[8] * m[8]);
                let mut in0: i32;
                let mut in1: i32;
                let mut in2: i32;
                if mag[0] > mag[1] {
                    if mag[0] > mag[2] {
                        if mag[2] > mag[1] {
                            in0 = 0;
                            in2 = 1;
                            in1 = 2;
                        } else {
                            in0 = 0;
                            in1 = 1;
                            in2 = 2;
                        }
                    } else {
                        in2 = 0;
                        in0 = 1;
                        in1 = 2;
                    }
                } else if mag[1] > mag[2] {
                    if mag[2] > mag[0] {
                        in1 = 0;
                        in2 = 1;
                        in0 = 2;
                    } else {
                        in1 = 0;
                        in0 = 1;
                        in2 = 2;
                    }
                } else {
                    in2 = 0;
                    in1 = 1;
                    in0 = 2;
                }
                let mut index: i32 = out[in0 as usize];
                out_scale[0] = scales[index as usize];
                index = out[in1 as usize];
                out_scale[1] = scales[index as usize];
                index = out[in2 as usize];
                out_scale[2] = scales[index as usize];
                index = out[in0 as usize];
                out_rot[0] = rot[index as usize];
                index = out[in0 as usize] + 3;
                out_rot[3] = rot[index as usize];
                index = out[in0 as usize] + 6;
                out_rot[6] = rot[index as usize];
                index = out[in1 as usize];
                out_rot[1] = rot[index as usize];
                index = out[in1 as usize] + 3;
                out_rot[4] = rot[index as usize];
                index = out[in1 as usize] + 6;
                out_rot[7] = rot[index as usize];
                index = out[in2 as usize];
                out_rot[2] = rot[index as usize];
                index = out[in2 as usize] + 3;
                out_rot[5] = rot[index as usize];
                index = out[in2 as usize] + 6;
                out_rot[8] = rot[index as usize];
            }
        }
        pub fn compute_qr(s: &mut [f64], e: &mut [f64], u: &mut [f64], v: &mut [f64]) -> i32 {
            let mut cosl: Vec<f64> = vec![0.0f64; 2];
            let mut cosr: Vec<f64> = vec![0.0f64; 2];
            let mut sinl: Vec<f64> = vec![0.0f64; 2];
            let mut sinr: Vec<f64> = vec![0.0f64; 2];
            let mut m: Vec<f64> = vec![0.0f64; 9];
            let max_interations: i32 = 10;
            let converge_tol: f64 = 4.89e-15f64;
            let c_b48: f64 = 1.0f64;
            let mut converged: bool = false;
            let mut first: i32 = 1;
            if ((e[1].abs()) < (4.89e-15f64)) || ((e[0].abs()) < (4.89e-15f64)) {
                converged = true;
            }
            {
                let mut k: i32 = 0;
                while (k < 10) && (!converged) {
                    let shift: f64 = matrix3::compute_shift(s[1], e[1], s[2]);
                    let mut f: f64 = ((s[0].abs()) - shift) * (matrix3::d_sign(c_b48, s[0]) + (shift / s[0]));
                    let mut g: f64 = e[0];
                    let mut r: f64 = matrix3::compute_rot(f, g, &mut sinr, &mut cosr, 0, first);
                    f = (cosr[0] * s[0]) + (sinr[0] * e[0]);
                    e[0] = (cosr[0] * e[0]) - (sinr[0] * s[0]);
                    g = sinr[0] * s[1];
                    s[1] = cosr[0] * s[1];
                    r = matrix3::compute_rot(f, g, &mut sinl, &mut cosl, 0, first);
                    first = 0;
                    s[0] = r;
                    f = (cosl[0] * e[0]) + (sinl[0] * s[1]);
                    s[1] = (cosl[0] * s[1]) - (sinl[0] * e[0]);
                    g = sinl[0] * e[1];
                    e[1] = cosl[0] * e[1];
                    r = matrix3::compute_rot(f, g, &mut sinr, &mut cosr, 1, first);
                    e[0] = r;
                    f = (cosr[1] * s[1]) + (sinr[1] * e[1]);
                    e[1] = (cosr[1] * e[1]) - (sinr[1] * s[1]);
                    g = sinr[1] * s[2];
                    s[2] = cosr[1] * s[2];
                    r = matrix3::compute_rot(f, g, &mut sinl, &mut cosl, 1, first);
                    s[1] = r;
                    f = (cosl[1] * e[1]) + (sinl[1] * s[2]);
                    s[2] = (cosl[1] * s[2]) - (sinl[1] * e[1]);
                    e[1] = f;
                    let mut utemp: f64 = u[0];
                    u[0] = (cosl[0] * utemp) + (sinl[0] * u[3]);
                    u[3] = ((-sinl[0]) * utemp) + (cosl[0] * u[3]);
                    utemp = u[1];
                    u[1] = (cosl[0] * utemp) + (sinl[0] * u[4]);
                    u[4] = ((-sinl[0]) * utemp) + (cosl[0] * u[4]);
                    utemp = u[2];
                    u[2] = (cosl[0] * utemp) + (sinl[0] * u[5]);
                    u[5] = ((-sinl[0]) * utemp) + (cosl[0] * u[5]);
                    utemp = u[3];
                    u[3] = (cosl[1] * utemp) + (sinl[1] * u[6]);
                    u[6] = ((-sinl[1]) * utemp) + (cosl[1] * u[6]);
                    utemp = u[4];
                    u[4] = (cosl[1] * utemp) + (sinl[1] * u[7]);
                    u[7] = ((-sinl[1]) * utemp) + (cosl[1] * u[7]);
                    utemp = u[5];
                    u[5] = (cosl[1] * utemp) + (sinl[1] * u[8]);
                    u[8] = ((-sinl[1]) * utemp) + (cosl[1] * u[8]);
                    let mut vtemp: f64 = v[0];
                    v[0] = (cosr[0] * vtemp) + (sinr[0] * v[1]);
                    v[1] = ((-sinr[0]) * vtemp) + (cosr[0] * v[1]);
                    vtemp = v[3];
                    v[3] = (cosr[0] * vtemp) + (sinr[0] * v[4]);
                    v[4] = ((-sinr[0]) * vtemp) + (cosr[0] * v[4]);
                    vtemp = v[6];
                    v[6] = (cosr[0] * vtemp) + (sinr[0] * v[7]);
                    v[7] = ((-sinr[0]) * vtemp) + (cosr[0] * v[7]);
                    vtemp = v[1];
                    v[1] = (cosr[1] * vtemp) + (sinr[1] * v[2]);
                    v[2] = ((-sinr[1]) * vtemp) + (cosr[1] * v[2]);
                    vtemp = v[4];
                    v[4] = (cosr[1] * vtemp) + (sinr[1] * v[5]);
                    v[5] = ((-sinr[1]) * vtemp) + (cosr[1] * v[5]);
                    vtemp = v[7];
                    v[7] = (cosr[1] * vtemp) + (sinr[1] * v[8]);
                    v[8] = ((-sinr[1]) * vtemp) + (cosr[1] * v[8]);
                    m[0] = s[0];
                    m[1] = e[0];
                    m[2] = 0.0f64;
                    m[3] = 0.0f64;
                    m[4] = s[1];
                    m[5] = e[1];
                    m[6] = 0.0f64;
                    m[7] = 0.0f64;
                    m[8] = s[2];
                    if ((e[1].abs()) < (4.89e-15f64)) || ((e[0].abs()) < (4.89e-15f64)) {
                        converged = true;
                    }
                    k += 1;
                }
            }
            if (e[1].abs()) < (4.89e-15f64) {
                matrix3::compute_2_x2(s[0], e[0], s[1], &mut *s, &mut sinl, &mut cosl, &mut sinr, &mut cosr, 0);
                let mut utemp: f64 = u[0];
                u[0] = (cosl[0] * utemp) + (sinl[0] * u[3]);
                u[3] = ((-sinl[0]) * utemp) + (cosl[0] * u[3]);
                utemp = u[1];
                u[1] = (cosl[0] * utemp) + (sinl[0] * u[4]);
                u[4] = ((-sinl[0]) * utemp) + (cosl[0] * u[4]);
                utemp = u[2];
                u[2] = (cosl[0] * utemp) + (sinl[0] * u[5]);
                u[5] = ((-sinl[0]) * utemp) + (cosl[0] * u[5]);
                let mut vtemp: f64 = v[0];
                v[0] = (cosr[0] * vtemp) + (sinr[0] * v[1]);
                v[1] = ((-sinr[0]) * vtemp) + (cosr[0] * v[1]);
                vtemp = v[3];
                v[3] = (cosr[0] * vtemp) + (sinr[0] * v[4]);
                v[4] = ((-sinr[0]) * vtemp) + (cosr[0] * v[4]);
                vtemp = v[6];
                v[6] = (cosr[0] * vtemp) + (sinr[0] * v[7]);
                v[7] = ((-sinr[0]) * vtemp) + (cosr[0] * v[7]);
            } else {
                matrix3::compute_2_x2(s[1], e[1], s[2], &mut *s, &mut sinl, &mut cosl, &mut sinr, &mut cosr, 1);
                let mut utemp: f64 = u[3];
                u[3] = (cosl[0] * utemp) + (sinl[0] * u[6]);
                u[6] = ((-sinl[0]) * utemp) + (cosl[0] * u[6]);
                utemp = u[4];
                u[4] = (cosl[0] * utemp) + (sinl[0] * u[7]);
                u[7] = ((-sinl[0]) * utemp) + (cosl[0] * u[7]);
                utemp = u[5];
                u[5] = (cosl[0] * utemp) + (sinl[0] * u[8]);
                u[8] = ((-sinl[0]) * utemp) + (cosl[0] * u[8]);
                let mut vtemp: f64 = v[1];
                v[1] = (cosr[0] * vtemp) + (sinr[0] * v[2]);
                v[2] = ((-sinr[0]) * vtemp) + (cosr[0] * v[2]);
                vtemp = v[4];
                v[4] = (cosr[0] * vtemp) + (sinr[0] * v[5]);
                v[5] = ((-sinr[0]) * vtemp) + (cosr[0] * v[5]);
                vtemp = v[7];
                v[7] = (cosr[0] * vtemp) + (sinr[0] * v[8]);
                v[8] = ((-sinr[0]) * vtemp) + (cosr[0] * v[8]);
            }
            return 0;
        }
        pub fn max(a: f64, b: f64) -> f64 {
            return if a > b { a } else { b };
        }
        pub fn min(a: f64, b: f64) -> f64 {
            return if a < b { a } else { b };
        }
        pub fn d_sign(a: f64, b: f64) -> f64 {
            let x: f64 = if a >= 0.0f64 { a } else { -a };
            return if b >= 0.0f64 { x } else { -x };
        }
        pub fn compute_shift(f: f64, g: f64, h: f64) -> f64 {
            let fa: f64 = f.abs();
            let ga: f64 = g.abs();
            let ha: f64 = h.abs();
            let fhmn: f64 = matrix3::min(fa, ha);
            let fhmx: f64 = matrix3::max(fa, ha);
            let mut ssmin: f64;
            if fhmn == 0.0f64 {
                ssmin = 0.0f64;
                if fhmx != 0.0f64 {
                    let d__1: f64 = matrix3::min(fhmx, ga) / matrix3::max(fhmx, ga);
                }
            } else if ga < fhmx {
                let r#as: f64 = (fhmn / fhmx) + 1.0f64;
                let at: f64 = (fhmx - fhmn) / fhmx;
                let d__1: f64 = ga / fhmx;
                let au: f64 = d__1 * d__1;
                let c: f64 = 2.0f64 / (((((r#as) * (r#as)) + au).sqrt()) + (((at * at) + au).sqrt()));
                ssmin = fhmn * c;
            } else {
                let au: f64 = fhmx / ga;
                if au == 0.0f64 {
                    ssmin = (fhmn * fhmx) / ga;
                } else {
                    let r#as: f64 = (fhmn / fhmx) + 1.0f64;
                    let at: f64 = (fhmx - fhmn) / fhmx;
                    let d__1: f64 = (r#as) * au;
                    let d__2: f64 = at * au;
                    let c: f64 = 1.0f64 / ((((d__1 * d__1) + 1.0f64).sqrt()) + (((d__2 * d__2) + 1.0f64).sqrt()));
                    ssmin = (fhmn * c) * au;
                    ssmin += ssmin;
                }
            }
            return ssmin;
        }
        pub fn compute_2_x2(f: f64, g: f64, h: f64, single_values: &mut [f64], snl: &mut [f64], csl: &mut [f64], snr: &mut [f64], csr: &mut [f64], index: i32) -> i32 {
            let c_b3: f64 = 2.0f64;
            let c_b4: f64 = 1.0f64;
            let mut ssmax: f64 = single_values[0];
            let mut ssmin: f64 = single_values[1];
            let mut clt: f64 = 0.0f64;
            let mut crt: f64 = 0.0f64;
            let mut slt: f64 = 0.0f64;
            let mut srt: f64 = 0.0f64;
            let mut tsign: f64 = 0.0f64;
            let mut ft: f64 = f;
            let mut fa: f64 = ft.abs();
            let mut ht: f64 = h;
            let mut ha: f64 = h.abs();
            let mut pmax: i32 = 1;
            let swap: bool = ha > fa;
            if swap {
                pmax = 3;
                let mut temp: f64 = ft;
                ft = ht;
                ht = temp;
                temp = fa;
                fa = ha;
                ha = temp;
            }
            let gt: f64 = g;
            let ga: f64 = gt.abs();
            if ga == 0.0f64 {
                single_values[1] = ha;
                single_values[0] = fa;
                clt = 1.0f64;
                crt = 1.0f64;
                slt = 0.0f64;
                srt = 0.0f64;
            } else {
                let mut gasmal: bool = true;
                if ga > fa {
                    pmax = 2;
                    if (fa / ga) < (1.110223024e-16f64) {
                        gasmal = false;
                        ssmax = ga;
                        if ha > 1.0f64 {
                            ssmin = fa / (ga / ha);
                        } else {
                            ssmin = (fa / ga) * ha;
                        }
                        clt = 1.0f64;
                        slt = ht / gt;
                        srt = 1.0f64;
                        crt = ft / gt;
                    }
                }
                if gasmal {
                    let mut d: f64 = fa - ha;
                    let mut l: f64;
                    if d == fa {
                        l = 1.0f64;
                    } else {
                        l = d / fa;
                    }
                    let mut m: f64 = gt / ft;
                    let mut t: f64 = 2.0f64 - l;
                    let mut mm: f64 = m * m;
                    let mut tt: f64 = t * t;
                    let mut s: f64 = (tt + mm).sqrt();
                    let mut r: f64;
                    if l == 0.0f64 {
                        r = m.abs();
                    } else {
                        r = ((l * l) + mm).sqrt();
                    }
                    let mut a: f64 = (s + r) * 0.5f64;
                    if ga > fa {
                        pmax = 2;
                        if (fa / ga) < (1.110223024e-16f64) {
                            gasmal = false;
                            ssmax = ga;
                            if ha > 1.0f64 {
                                ssmin = fa / (ga / ha);
                            } else {
                                ssmin = (fa / ga) * ha;
                            }
                            clt = 1.0f64;
                            slt = ht / gt;
                            srt = 1.0f64;
                            crt = ft / gt;
                        }
                    }
                    if gasmal {
                        d = fa - ha;
                        if d == fa {
                            l = 1.0f64;
                        } else {
                            l = d / fa;
                        }
                        m = gt / ft;
                        t = 2.0f64 - l;
                        mm = m * m;
                        tt = t * t;
                        s = (tt + mm).sqrt();
                        if l == 0.0f64 {
                            r = m.abs();
                        } else {
                            r = ((l * l) + mm).sqrt();
                        }
                        a = (s + r) * 0.5f64;
                        ssmin = ha / a;
                        ssmax = fa * a;
                        if mm == 0.0f64 {
                            if l == 0.0f64 {
                                t = matrix3::d_sign(c_b3, ft) * matrix3::d_sign(c_b4, gt);
                            } else {
                                t = (gt / matrix3::d_sign(d, ft)) + (m / t);
                            }
                        } else {
                            t = ((m / (s + t)) + (m / (r + l))) * (a + 1.0f64);
                        }
                        l = ((t * t) + 4.0f64).sqrt();
                        crt = 2.0f64 / l;
                        srt = t / l;
                        clt = (crt + (srt * m)) / a;
                        slt = ((ht / ft) * srt) / a;
                    }
                }
                if swap {
                    csl[0] = srt;
                    snl[0] = crt;
                    csr[0] = slt;
                    snr[0] = clt;
                } else {
                    csl[0] = clt;
                    snl[0] = slt;
                    csr[0] = crt;
                    snr[0] = srt;
                }
                if pmax == 1 {
                    tsign = (matrix3::d_sign(c_b4, csr[0]) * matrix3::d_sign(c_b4, csl[0])) * matrix3::d_sign(c_b4, f);
                }
                if pmax == 2 {
                    tsign = (matrix3::d_sign(c_b4, snr[0]) * matrix3::d_sign(c_b4, csl[0])) * matrix3::d_sign(c_b4, g);
                }
                if pmax == 3 {
                    tsign = (matrix3::d_sign(c_b4, snr[0]) * matrix3::d_sign(c_b4, snl[0])) * matrix3::d_sign(c_b4, h);
                }
                single_values[index as usize] = matrix3::d_sign(ssmax, tsign);
                let d__1: f64 = (tsign * matrix3::d_sign(c_b4, f)) * matrix3::d_sign(c_b4, h);
                single_values[(index + 1) as usize] = matrix3::d_sign(ssmin, d__1);
            }
            return 0;
        }
        pub fn compute_rot(f: f64, g: f64, sin: &mut [f64], cos: &mut [f64], index: i32, first: i32) -> f64 {
            let safmn2: f64 = 2.002083095183101e-146f64;
            let safmx2: f64 = 4.994797680505588e145f64;
            let mut cs: f64;
            let mut sn: f64;
            let mut r: f64;
            if g == 0.0f64 {
                cs = 1.0f64;
                sn = 0.0f64;
                r = f;
            } else if f == 0.0f64 {
                cs = 0.0f64;
                sn = 1.0f64;
                r = g;
            } else {
                let mut f1: f64 = f;
                let mut g1: f64 = g;
                let mut scale: f64 = matrix3::max(f1.abs(), g1.abs());
                if scale >= 4.994797680505588e145f64 {
                    let mut count: i32 = 0;
                    while scale >= 4.994797680505588e145f64 {
                        count += 1;
                        f1 *= 2.002083095183101e-146f64;
                        g1 *= 2.002083095183101e-146f64;
                        scale = matrix3::max(f1.abs(), g1.abs());
                    }
                    r = ((f1 * f1) + (g1 * g1)).sqrt();
                    cs = f1 / r;
                    sn = g1 / r;
                    for i in 1..=count {
                        r *= 4.994797680505588e145f64;
                    }
                } else if !(scale <= (2.002083095183101e-146f64)) {
                    r = ((f1 * f1) + (g1 * g1)).sqrt();
                    cs = f1 / r;
                    sn = g1 / r;
                } else {
                    let mut count: i32 = 0;
                    while scale <= (2.002083095183101e-146f64) {
                        count += 1;
                        f1 *= 4.994797680505588e145f64;
                        g1 *= 4.994797680505588e145f64;
                        scale = matrix3::max(f1.abs(), g1.abs());
                    }
                    r = ((f1 * f1) + (g1 * g1)).sqrt();
                    cs = f1 / r;
                    sn = g1 / r;
                    for i in 1..=count {
                        r *= 2.002083095183101e-146f64;
                    }
                }
                if (f.abs() > g.abs()) && (cs < 0.0f64) {
                    cs = -cs;
                    sn = -sn;
                    r = -r;
                }
            }
            sin[index as usize] = sn;
            cos[index as usize] = cs;
            return r;
        }
        pub fn print_mat(mat: &[f64]) {
            for i in 0..3 {
                // log:
            }
        }
        pub fn print_det(mat: &[f64]) {
            let det: f64 = ((((((mat[0] * mat[4]) * mat[8]) + ((mat[1] * mat[5]) * mat[6])) + ((mat[2] * mat[3]) * mat[7])) - ((mat[2] * mat[4]) * mat[6])) - ((mat[0] * mat[5]) * mat[7])) - ((mat[1] * mat[3]) * mat[8]);
            // log:
        }
        pub fn mat_mul(m1: &[f64], m2: &[f64], m3: &mut [f64]) {
            let tmp: Vec<f64> = vec![((m1[0] * m2[0]) + (m1[1] * m2[3])) + (m1[2] * m2[6]), ((m1[0] * m2[1]) + (m1[1] * m2[4])) + (m1[2] * m2[7]), ((m1[0] * m2[2]) + (m1[1] * m2[5])) + (m1[2] * m2[8]), ((m1[3] * m2[0]) + (m1[4] * m2[3])) + (m1[5] * m2[6]), ((m1[3] * m2[1]) + (m1[4] * m2[4])) + (m1[5] * m2[7]), ((m1[3] * m2[2]) + (m1[4] * m2[5])) + (m1[5] * m2[8]), ((m1[6] * m2[0]) + (m1[7] * m2[3])) + (m1[8] * m2[6]), ((m1[6] * m2[1]) + (m1[7] * m2[4])) + (m1[8] * m2[7]), ((m1[6] * m2[2]) + (m1[7] * m2[5])) + (m1[8] * m2[8])];
            for i in 0..9 {
                m3[i as usize] = tmp[i as usize];
            }
        }
        pub fn transpose_mat(r#in: &[f64], out: &mut [f64]) {
            out[0] = (r#in)[0];
            out[1] = (r#in)[3];
            out[2] = (r#in)[6];
            out[3] = (r#in)[1];
            out[4] = (r#in)[4];
            out[5] = (r#in)[7];
            out[6] = (r#in)[2];
            out[7] = (r#in)[5];
            out[8] = (r#in)[8];
        }
        pub fn max3(values: &[f64]) -> f64 {
            if values[0] > values[1] {
                return if values[0] > values[2] { values[0] } else { values[2] };
            } else {
                return if values[1] > values[2] { values[1] } else { values[2] };
            }
            #[allow(unreachable_code)]
            unreachable!()
        }
        pub fn almost_equal(a: f64, b: f64) -> bool {
            if a == b {
                return true;
            }
            let epsilon_absolute: f64 = 1.0e-6f64;
            let epsilon_relative: f64 = 1.0e-4f64;
            let diff: f64 = (a - b).abs();
            let abs_a: f64 = a.abs();
            let abs_b: f64 = b.abs();
            let max: f64 = if abs_a >= abs_b { abs_a } else { abs_b };
            return if diff < (1.0e-6f64) { true } else { (diff / max) < (1.0e-4f64) };
        }
    }

    /// `Matrix4` (team/creative/creativecore/common/util/math/matrix/Matrix4.java)
    pub mod matrix4 {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Matrix4 {
            pub m00: f64,
            pub m01: f64,
            pub m02: f64,
            pub m03: f64,
            pub m10: f64,
            pub m11: f64,
            pub m12: f64,
            pub m13: f64,
            pub m20: f64,
            pub m21: f64,
            pub m22: f64,
            pub m23: f64,
            pub m30: f64,
            pub m31: f64,
            pub m32: f64,
            pub m33: f64,
        }
        impl Matrix4 {
            pub fn new_16(var1: f64, var3: f64, var5: f64, var7: f64, var9: f64, var11: f64, var13: f64, var15: f64, var17: f64, var19: f64, var21: f64, var23: f64, var25: f64, var27: f64, var29: f64, var31: f64) -> Self {
                let mut s = Self { m00: var1, m01: var3, m02: var5, m03: var7, m10: var9, m11: var11, m12: var13, m13: var15, m20: var17, m21: var19, m22: var21, m23: var23, m30: var25, m31: var27, m32: var29, m33: var31 };
                s.m00 = var1;
                s.m01 = var3;
                s.m02 = var5;
                s.m03 = var7;
                s.m10 = var9;
                s.m11 = var11;
                s.m12 = var13;
                s.m13 = var15;
                s.m20 = var17;
                s.m21 = var19;
                s.m22 = var21;
                s.m23 = var23;
                s.m30 = var25;
                s.m31 = var27;
                s.m32 = var29;
                s.m33 = var31;
                s
            }
            // Matrix4.Matrix4(1): not translated, its constructors are overloaded with the same number of parameters.
            // Matrix4.Matrix4(1): not translated, its constructors are overloaded with the same number of parameters.
            // Matrix4.Matrix4(3): not translated, it uses Vec3d, which isn't translated (it extends `VecNd`).
            pub fn new() -> Self {
                let mut s = Self { m00: 0.0f64, m01: 0.0f64, m02: 0.0f64, m03: 0.0f64, m10: 0.0f64, m11: 0.0f64, m12: 0.0f64, m13: 0.0f64, m20: 0.0f64, m21: 0.0f64, m22: 0.0f64, m23: 0.0f64, m30: 0.0f64, m31: 0.0f64, m32: 0.0f64, m33: 0.0f64 };
                s.m00 = 0.0f64;
                s.m01 = 0.0f64;
                s.m02 = 0.0f64;
                s.m03 = 0.0f64;
                s.m10 = 0.0f64;
                s.m11 = 0.0f64;
                s.m12 = 0.0f64;
                s.m13 = 0.0f64;
                s.m20 = 0.0f64;
                s.m21 = 0.0f64;
                s.m22 = 0.0f64;
                s.m23 = 0.0f64;
                s.m30 = 0.0f64;
                s.m31 = 0.0f64;
                s.m32 = 0.0f64;
                s.m33 = 0.0f64;
                s
            }
            pub fn to_string(&self) -> String {
                return format!("{}, {}, {}, {}\n{}, {}, {}, {}\n{}, {}, {}, {}\n{}, {}, {}, {}\n", j_float(self.m00), j_float(self.m01), j_float(self.m02), j_float(self.m03), j_float(self.m10), j_float(self.m11), j_float(self.m12), j_float(self.m13), j_float(self.m20), j_float(self.m21), j_float(self.m22), j_float(self.m23), j_float(self.m30), j_float(self.m31), j_float(self.m32), j_float(self.m33));
            }
            pub fn set_identity(&mut self) {
                self.m00 = 1.0f64;
                self.m01 = 0.0f64;
                self.m02 = 0.0f64;
                self.m03 = 0.0f64;
                self.m10 = 0.0f64;
                self.m11 = 1.0f64;
                self.m12 = 0.0f64;
                self.m13 = 0.0f64;
                self.m20 = 0.0f64;
                self.m21 = 0.0f64;
                self.m22 = 1.0f64;
                self.m23 = 0.0f64;
                self.m30 = 0.0f64;
                self.m31 = 0.0f64;
                self.m32 = 0.0f64;
                self.m33 = 1.0f64;
            }
            // Matrix4.setElement(3): not translated, a switch case that falls through.
            // Matrix4.getElement(2): not translated, a switch case that falls through.
            // Matrix4.getRow(2): not translated, it's overloaded with the same number of parameters.
            // Matrix4.getRow(2): not translated, it's overloaded with the same number of parameters.
            // Matrix4.getColumn(2): not translated, it's overloaded with the same number of parameters.
            // Matrix4.getColumn(2): not translated, it's overloaded with the same number of parameters.
            // Matrix4.get(1): not translated, it's overloaded with the same number of parameters.
            // Matrix4.get(2): not translated, it uses Vec3d, which isn't translated (it extends `VecNd`).
            // Matrix4.get(1): not translated, it's overloaded with the same number of parameters.
            pub fn get_rotation_scale(&self, var1: &mut matrix3::Matrix3) {
                var1.m00 = self.m00;
                var1.m01 = self.m01;
                var1.m02 = self.m02;
                var1.m10 = self.m10;
                var1.m11 = self.m11;
                var1.m12 = self.m12;
                var1.m20 = self.m20;
                var1.m21 = self.m21;
                var1.m22 = self.m22;
            }
            // Matrix4.getScale(0): not translated, it calls Matrix4.getScaleRotate, which isn't translated.
            pub fn set_rotation_scale(&mut self, var1: &matrix3::Matrix3) {
                self.m00 = var1.m00;
                self.m01 = var1.m01;
                self.m02 = var1.m02;
                self.m10 = var1.m10;
                self.m11 = var1.m11;
                self.m12 = var1.m12;
                self.m20 = var1.m20;
                self.m21 = var1.m21;
                self.m22 = var1.m22;
            }
            // Matrix4.setScale(1): not translated, it calls Matrix4.getScaleRotate, which isn't translated.
            pub fn set_row_5(&mut self, var1: i32, var2: f64, var4: f64, var6: f64, var8: f64) {
                match var1 {
                    0 => {
                        self.m00 = var2;
                        self.m01 = var4;
                        self.m02 = var6;
                        self.m03 = var8;
                    }
                    1 => {
                        self.m10 = var2;
                        self.m11 = var4;
                        self.m12 = var6;
                        self.m13 = var8;
                    }
                    2 => {
                        self.m20 = var2;
                        self.m21 = var4;
                        self.m22 = var6;
                        self.m23 = var8;
                    }
                    3 => {
                        self.m30 = var2;
                        self.m31 = var4;
                        self.m32 = var6;
                        self.m33 = var8;
                    }
                    _ => {
                        panic!("ArrayIndexOutOfBoundsException");
                    }
                }
            }
            // Matrix4.setRow(2): not translated, it's overloaded with the same number of parameters.
            // Matrix4.setRow(2): not translated, it's overloaded with the same number of parameters.
            pub fn set_column_5(&mut self, var1: i32, var2: f64, var4: f64, var6: f64, var8: f64) {
                match var1 {
                    0 => {
                        self.m00 = var2;
                        self.m10 = var4;
                        self.m20 = var6;
                        self.m30 = var8;
                    }
                    1 => {
                        self.m01 = var2;
                        self.m11 = var4;
                        self.m21 = var6;
                        self.m31 = var8;
                    }
                    2 => {
                        self.m02 = var2;
                        self.m12 = var4;
                        self.m22 = var6;
                        self.m32 = var8;
                    }
                    3 => {
                        self.m03 = var2;
                        self.m13 = var4;
                        self.m23 = var6;
                        self.m33 = var8;
                    }
                    _ => {
                        panic!("ArrayIndexOutOfBoundsException");
                    }
                }
            }
            // Matrix4.setColumn(2): not translated, it's overloaded with the same number of parameters.
            // Matrix4.setColumn(2): not translated, it's overloaded with the same number of parameters.
            // Matrix4.add(1): not translated, it's overloaded with the same number of parameters.
            // Matrix4.add(2): not translated, it's overloaded with the same number of parameters.
            // Matrix4.add(2): not translated, it's overloaded with the same number of parameters.
            // Matrix4.add(1): not translated, it's overloaded with the same number of parameters.
            pub fn sub_2(&mut self, var1: &matrix4::Matrix4, var2: &matrix4::Matrix4) {
                self.m00 = var1.m00 - var2.m00;
                self.m01 = var1.m01 - var2.m01;
                self.m02 = var1.m02 - var2.m02;
                self.m03 = var1.m03 - var2.m03;
                self.m10 = var1.m10 - var2.m10;
                self.m11 = var1.m11 - var2.m11;
                self.m12 = var1.m12 - var2.m12;
                self.m13 = var1.m13 - var2.m13;
                self.m20 = var1.m20 - var2.m20;
                self.m21 = var1.m21 - var2.m21;
                self.m22 = var1.m22 - var2.m22;
                self.m23 = var1.m23 - var2.m23;
                self.m30 = var1.m30 - var2.m30;
                self.m31 = var1.m31 - var2.m31;
                self.m32 = var1.m32 - var2.m32;
                self.m33 = var1.m33 - var2.m33;
            }
            pub fn sub_1(&mut self, var1: &matrix4::Matrix4) {
                self.m00 = self.m00 - var1.m00;
                self.m01 = self.m01 - var1.m01;
                self.m02 = self.m02 - var1.m02;
                self.m03 = self.m03 - var1.m03;
                self.m10 = self.m10 - var1.m10;
                self.m11 = self.m11 - var1.m11;
                self.m12 = self.m12 - var1.m12;
                self.m13 = self.m13 - var1.m13;
                self.m20 = self.m20 - var1.m20;
                self.m21 = self.m21 - var1.m21;
                self.m22 = self.m22 - var1.m22;
                self.m23 = self.m23 - var1.m23;
                self.m30 = self.m30 - var1.m30;
                self.m31 = self.m31 - var1.m31;
                self.m32 = self.m32 - var1.m32;
                self.m33 = self.m33 - var1.m33;
            }
            pub fn transpose_0(&mut self) {
                let mut var1: f64 = self.m10;
                self.m10 = self.m01;
                self.m01 = var1;
                var1 = self.m20;
                self.m20 = self.m02;
                self.m02 = var1;
                var1 = self.m30;
                self.m30 = self.m03;
                self.m03 = var1;
                var1 = self.m21;
                self.m21 = self.m12;
                self.m12 = var1;
                var1 = self.m31;
                self.m31 = self.m13;
                self.m13 = var1;
                var1 = self.m32;
                self.m32 = self.m23;
                self.m23 = var1;
            }
            // Matrix4.transpose(1): not translated, `this` used as a value.
            // Matrix4.set(1): not translated, it's overloaded with the same number of parameters.
            // Matrix4.set(1): not translated, it's overloaded with the same number of parameters.
            // Matrix4.set(1): not translated, it's overloaded with the same number of parameters.
            // Matrix4.invert(1): not translated, it calls Matrix4.invertGeneral, which isn't translated.
            // Matrix4.invert(0): not translated, `this` used as a value.
            // Matrix4.invertGeneral(1): not translated, it calls Matrix4.luDecomposition, which isn't translated.
            pub fn determinant(&self) -> f64 {
                let mut var1: f64 = self.m00 * (((((((self.m11 * self.m22) * self.m33) + ((self.m12 * self.m23) * self.m31)) + ((self.m13 * self.m21) * self.m32)) - ((self.m13 * self.m22) * self.m31)) - ((self.m11 * self.m23) * self.m32)) - ((self.m12 * self.m21) * self.m33));
                var1 -= self.m01 * (((((((self.m10 * self.m22) * self.m33) + ((self.m12 * self.m23) * self.m30)) + ((self.m13 * self.m20) * self.m32)) - ((self.m13 * self.m22) * self.m30)) - ((self.m10 * self.m23) * self.m32)) - ((self.m12 * self.m20) * self.m33));
                var1 += self.m02 * (((((((self.m10 * self.m21) * self.m33) + ((self.m11 * self.m23) * self.m30)) + ((self.m13 * self.m20) * self.m31)) - ((self.m13 * self.m21) * self.m30)) - ((self.m10 * self.m23) * self.m31)) - ((self.m11 * self.m20) * self.m33));
                return var1 - (self.m03 * (((((((self.m10 * self.m21) * self.m32) + ((self.m11 * self.m22) * self.m30)) + ((self.m12 * self.m20) * self.m31)) - ((self.m12 * self.m21) * self.m30)) - ((self.m10 * self.m22) * self.m31)) - ((self.m11 * self.m20) * self.m32)));
            }
            // Matrix4.set(1): not translated, it's overloaded with the same number of parameters.
            // Matrix4.set(1): not translated, it's overloaded with the same number of parameters.
            // Matrix4.set(2): not translated, it's overloaded with the same number of parameters.
            // Matrix4.set(2): not translated, it's overloaded with the same number of parameters.
            // Matrix4.set(3): not translated, it uses Vec3d, which isn't translated (it extends `VecNd`).
            // Matrix4.setTranslation(1): not translated, it uses Vec3d, which isn't translated (it extends `VecNd`).
            pub fn rot_x(&mut self, var1: f64) {
                let var3: f64 = var1.sin();
                let var5: f64 = var1.cos();
                self.m00 = 1.0f64;
                self.m01 = 0.0f64;
                self.m02 = 0.0f64;
                self.m03 = 0.0f64;
                self.m10 = 0.0f64;
                self.m11 = var5;
                self.m12 = -var3;
                self.m13 = 0.0f64;
                self.m20 = 0.0f64;
                self.m21 = var3;
                self.m22 = var5;
                self.m23 = 0.0f64;
                self.m30 = 0.0f64;
                self.m31 = 0.0f64;
                self.m32 = 0.0f64;
                self.m33 = 1.0f64;
            }
            pub fn rot_y(&mut self, var1: f64) {
                let var3: f64 = var1.sin();
                let var5: f64 = var1.cos();
                self.m00 = var5;
                self.m01 = 0.0f64;
                self.m02 = var3;
                self.m03 = 0.0f64;
                self.m10 = 0.0f64;
                self.m11 = 1.0f64;
                self.m12 = 0.0f64;
                self.m13 = 0.0f64;
                self.m20 = -var3;
                self.m21 = 0.0f64;
                self.m22 = var5;
                self.m23 = 0.0f64;
                self.m30 = 0.0f64;
                self.m31 = 0.0f64;
                self.m32 = 0.0f64;
                self.m33 = 1.0f64;
            }
            pub fn rot_z(&mut self, var1: f64) {
                let var3: f64 = var1.sin();
                let var5: f64 = var1.cos();
                self.m00 = var5;
                self.m01 = -var3;
                self.m02 = 0.0f64;
                self.m03 = 0.0f64;
                self.m10 = var3;
                self.m11 = var5;
                self.m12 = 0.0f64;
                self.m13 = 0.0f64;
                self.m20 = 0.0f64;
                self.m21 = 0.0f64;
                self.m22 = 1.0f64;
                self.m23 = 0.0f64;
                self.m30 = 0.0f64;
                self.m31 = 0.0f64;
                self.m32 = 0.0f64;
                self.m33 = 1.0f64;
            }
            // Matrix4.mul(1): not translated, it's overloaded with the same number of parameters.
            // Matrix4.mul(2): not translated, it's overloaded with the same number of parameters.
            // Matrix4.mul(1): not translated, it's overloaded with the same number of parameters.
            // Matrix4.mul(2): not translated, it's overloaded with the same number of parameters.
            // Matrix4.mulTransposeBoth(2): not translated, `this` used as a value.
            // Matrix4.mulTransposeRight(2): not translated, `this` used as a value.
            // Matrix4.mulTransposeLeft(2): not translated, `this` used as a value.
            // Matrix4.equals(1): not translated, it's overloaded with the same number of parameters.
            // Matrix4.equals(1): not translated, it's overloaded with the same number of parameters.
            // Matrix4.epsilonEquals(2): not translated, it's overloaded with the same number of parameters.
            // Matrix4.epsilonEquals(2): not translated, it's overloaded with the same number of parameters.
            // Matrix4.hashCode(0): not translated, it calls `Double`.doubleToLongBits.
            // Matrix4.transform(2): not translated, it's overloaded with the same number of parameters.
            // Matrix4.transform(1): not translated, it's overloaded with the same number of parameters.
            // Matrix4.transform(2): not translated, it's overloaded with the same number of parameters.
            // Matrix4.transform(1): not translated, it's overloaded with the same number of parameters.
            // Matrix4.transform(2): not translated, it's overloaded with the same number of parameters.
            // Matrix4.transform(1): not translated, it's overloaded with the same number of parameters.
            // Matrix4.setRotation(1): not translated, it calls Matrix4.getScaleRotate, which isn't translated.
            pub fn set_zero(&mut self) {
                self.m00 = 0.0f64;
                self.m01 = 0.0f64;
                self.m02 = 0.0f64;
                self.m03 = 0.0f64;
                self.m10 = 0.0f64;
                self.m11 = 0.0f64;
                self.m12 = 0.0f64;
                self.m13 = 0.0f64;
                self.m20 = 0.0f64;
                self.m21 = 0.0f64;
                self.m22 = 0.0f64;
                self.m23 = 0.0f64;
                self.m30 = 0.0f64;
                self.m31 = 0.0f64;
                self.m32 = 0.0f64;
                self.m33 = 0.0f64;
            }
            pub fn negate_0(&mut self) {
                self.m00 = -self.m00;
                self.m01 = -self.m01;
                self.m02 = -self.m02;
                self.m03 = -self.m03;
                self.m10 = -self.m10;
                self.m11 = -self.m11;
                self.m12 = -self.m12;
                self.m13 = -self.m13;
                self.m20 = -self.m20;
                self.m21 = -self.m21;
                self.m22 = -self.m22;
                self.m23 = -self.m23;
                self.m30 = -self.m30;
                self.m31 = -self.m31;
                self.m32 = -self.m32;
                self.m33 = -self.m33;
            }
            pub fn negate_1(&mut self, var1: &matrix4::Matrix4) {
                self.m00 = -var1.m00;
                self.m01 = -var1.m01;
                self.m02 = -var1.m02;
                self.m03 = -var1.m03;
                self.m10 = -var1.m10;
                self.m11 = -var1.m11;
                self.m12 = -var1.m12;
                self.m13 = -var1.m13;
                self.m20 = -var1.m20;
                self.m21 = -var1.m21;
                self.m22 = -var1.m22;
                self.m23 = -var1.m23;
                self.m30 = -var1.m30;
                self.m31 = -var1.m31;
                self.m32 = -var1.m32;
                self.m33 = -var1.m33;
            }
            // Matrix4.getScaleRotate(2): not translated, it calls Matrix3.compute_svd, which isn't translated.
            // Matrix4.clone(0): not translated, it uses `Object`.
            pub fn get_m00(&self) -> f64 {
                return self.m00;
            }
            pub fn set_m00(&mut self, var1: f64) {
                self.m00 = var1;
            }
            pub fn get_m01(&self) -> f64 {
                return self.m01;
            }
            pub fn set_m01(&mut self, var1: f64) {
                self.m01 = var1;
            }
            pub fn get_m02(&self) -> f64 {
                return self.m02;
            }
            pub fn set_m02(&mut self, var1: f64) {
                self.m02 = var1;
            }
            pub fn get_m10(&self) -> f64 {
                return self.m10;
            }
            pub fn set_m10(&mut self, var1: f64) {
                self.m10 = var1;
            }
            pub fn get_m11(&self) -> f64 {
                return self.m11;
            }
            pub fn set_m11(&mut self, var1: f64) {
                self.m11 = var1;
            }
            pub fn get_m12(&self) -> f64 {
                return self.m12;
            }
            pub fn set_m12(&mut self, var1: f64) {
                self.m12 = var1;
            }
            pub fn get_m20(&self) -> f64 {
                return self.m20;
            }
            pub fn set_m20(&mut self, var1: f64) {
                self.m20 = var1;
            }
            pub fn get_m21(&self) -> f64 {
                return self.m21;
            }
            pub fn set_m21(&mut self, var1: f64) {
                self.m21 = var1;
            }
            pub fn get_m22(&self) -> f64 {
                return self.m22;
            }
            pub fn set_m22(&mut self, var1: f64) {
                self.m22 = var1;
            }
            pub fn get_m03(&self) -> f64 {
                return self.m03;
            }
            pub fn set_m03(&mut self, var1: f64) {
                self.m03 = var1;
            }
            pub fn get_m13(&self) -> f64 {
                return self.m13;
            }
            pub fn set_m13(&mut self, var1: f64) {
                self.m13 = var1;
            }
            pub fn get_m23(&self) -> f64 {
                return self.m23;
            }
            pub fn set_m23(&mut self, var1: f64) {
                self.m23 = var1;
            }
            pub fn get_m30(&self) -> f64 {
                return self.m30;
            }
            pub fn set_m30(&mut self, var1: f64) {
                self.m30 = var1;
            }
            pub fn get_m31(&self) -> f64 {
                return self.m31;
            }
            pub fn set_m31(&mut self, var1: f64) {
                self.m31 = var1;
            }
            pub fn get_m32(&self) -> f64 {
                return self.m32;
            }
            pub fn set_m32(&mut self, var1: f64) {
                self.m32 = var1;
            }
            pub fn get_m33(&self) -> f64 {
                return self.m33;
            }
            pub fn set_m33(&mut self, var1: f64) {
                self.m33 = var1;
            }
        }
        // Matrix4.luDecomposition(2): not translated, `--` inside an expression.
        pub fn lu_backsubstitution(var0: &[f64], var1: &[i32], var2: &mut [f64]) {
            let var8: i8 = 0i8;
            for var7 in 0..4 {
                let var9: i32 = var7;
                let mut var4: i32 = -1;
                for var3 in 0..4 {
                    let var5: i32 = var1[((var8 as i32) + var3) as usize];
                    let mut var11: f64 = var2[(var9 + (4 * var5)) as usize];
                    var2[(var9 + (4 * var5)) as usize] = var2[(var9 + (4 * var3)) as usize];
                    if var4 >= 0 {
                        let mut var10: i32 = var3 * 4;
                        for var6 in var4..=(var3 - 1) {
                            var11 -= var0[(var10 + var6) as usize] * var2[(var9 + (4 * var6)) as usize];
                        }
                    } else if var11 != 0.0f64 {
                        var4 = var3;
                    }
                    var2[(var9 + (4 * var3)) as usize] = var11;
                }
                let var13: i8 = 12i8;
                var2[(var9 + 12) as usize] = var2[(var9 + 12) as usize] / var0[((var13 as i32) + 3) as usize];
                let mut var10: i32 = (var13 as i32) - 4;
                var2[(var9 + 8) as usize] = (var2[(var9 + 8) as usize] - (var0[(var10 + 3) as usize] * var2[(var9 + 12) as usize])) / var0[(var10 + 2) as usize];
                var10 -= 4;
                var2[(var9 + 4) as usize] = ((var2[(var9 + 4) as usize] - (var0[(var10 + 2) as usize] * var2[(var9 + 8) as usize])) - (var0[(var10 + 3) as usize] * var2[(var9 + 12) as usize])) / var0[(var10 + 1) as usize];
                var10 -= 4;
                var2[var9 as usize] = (((var2[var9 as usize] - (var0[(var10 + 1) as usize] * var2[(var9 + 4) as usize])) - (var0[(var10 + 2) as usize] * var2[(var9 + 8) as usize])) - (var0[(var10 + 3) as usize] * var2[(var9 + 12) as usize])) / var0[var10 as usize];
            }
        }
    }

    /// `VecOrigin` (team/creative/creativecore/common/util/math/matrix/VecOrigin.java)
    pub mod vec_origin {
        use super::*;
        // The class itself isn't translated: it implements `IVecOrigin`. Its static parts are below.
        // VecOrigin.VecOrigin(1): not translated, its class isn't translated (it implements `IVecOrigin`).
        // VecOrigin.offX(0): not translated, its class isn't translated (it implements `IVecOrigin`).
        // VecOrigin.offY(0): not translated, its class isn't translated (it implements `IVecOrigin`).
        // VecOrigin.offZ(0): not translated, its class isn't translated (it implements `IVecOrigin`).
        // VecOrigin.rotX(0): not translated, its class isn't translated (it implements `IVecOrigin`).
        // VecOrigin.rotY(0): not translated, its class isn't translated (it implements `IVecOrigin`).
        // VecOrigin.rotZ(0): not translated, its class isn't translated (it implements `IVecOrigin`).
        // VecOrigin.isRotated(0): not translated, its class isn't translated (it implements `IVecOrigin`).
        // VecOrigin.updateRotated(0): not translated, its class isn't translated (it implements `IVecOrigin`).
        // VecOrigin.updateTranslation(0): not translated, its class isn't translated (it implements `IVecOrigin`).
        // VecOrigin.offX(1): not translated, its class isn't translated (it implements `IVecOrigin`).
        // VecOrigin.offY(1): not translated, its class isn't translated (it implements `IVecOrigin`).
        // VecOrigin.offZ(1): not translated, its class isn't translated (it implements `IVecOrigin`).
        // VecOrigin.off(3): not translated, its class isn't translated (it implements `IVecOrigin`).
        // VecOrigin.rotX(1): not translated, its class isn't translated (it implements `IVecOrigin`).
        // VecOrigin.rotY(1): not translated, its class isn't translated (it implements `IVecOrigin`).
        // VecOrigin.rotZ(1): not translated, its class isn't translated (it implements `IVecOrigin`).
        // VecOrigin.rot(3): not translated, its class isn't translated (it implements `IVecOrigin`).
        // VecOrigin.center(0): not translated, its class isn't translated (it implements `IVecOrigin`).
        // VecOrigin.rotation(0): not translated, its class isn't translated (it implements `IVecOrigin`).
        // VecOrigin.rotationInv(0): not translated, its class isn't translated (it implements `IVecOrigin`).
        // VecOrigin.translation(0): not translated, its class isn't translated (it implements `IVecOrigin`).
        // VecOrigin.setCenter(1): not translated, its class isn't translated (it implements `IVecOrigin`).
        // VecOrigin.offXLast(0): not translated, its class isn't translated (it implements `IVecOrigin`).
        // VecOrigin.offYLast(0): not translated, its class isn't translated (it implements `IVecOrigin`).
        // VecOrigin.offZLast(0): not translated, its class isn't translated (it implements `IVecOrigin`).
        // VecOrigin.rotXLast(0): not translated, its class isn't translated (it implements `IVecOrigin`).
        // VecOrigin.rotYLast(0): not translated, its class isn't translated (it implements `IVecOrigin`).
        // VecOrigin.rotZLast(0): not translated, its class isn't translated (it implements `IVecOrigin`).
        // VecOrigin.tick(0): not translated, its class isn't translated (it implements `IVecOrigin`).
        // VecOrigin.createInternalCopy(0): not translated, its class isn't translated (it implements `IVecOrigin`).
        // VecOrigin.copy(0): not translated, its class isn't translated (it implements `IVecOrigin`).
        // VecOrigin.getParent(0): not translated, its class isn't translated (it implements `IVecOrigin`).
    }

    /// `TimeMath` (team/creative/creativecore/common/util/math/TimeMath.java)
    pub mod time_math {
        use super::*;
        // FORMAT: not translated, it uses `DateFormat` (java.text).
        #[derive(Clone, Debug)]
        pub struct TimeMath {
        }
        impl TimeMath {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // TimeMath.timestamp(1): not translated, it calls `MILLISECONDS`.toMinutes.
    }

    /// `BooleanRotation` (team/creative/creativecore/common/util/math/transformation/BooleanRotation.java)
    pub mod boolean_rotation {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // ROTATIONS: not translated, it uses BooleanRotation, which isn't translated (its constants carry values).
        // BooleanRotation.BooleanRotation(4): not translated, its class isn't translated (its constants carry values).
        // BooleanRotation.one(1): not translated, it uses Axis, which isn't translated (its constants carry values).
        // BooleanRotation.two(1): not translated, it uses Axis, which isn't translated (its constants carry values).
        // BooleanRotation.clockwise(0): not translated, its class isn't translated (its constants carry values).
        // BooleanRotation.counterClockwise(0): not translated, its class isn't translated (its constants carry values).
        // BooleanRotation.clockwiseMaxFacing(0): not translated, its class isn't translated (its constants carry values).
        // BooleanRotation.counterMaxClockwiseFacing(0): not translated, its class isn't translated (its constants carry values).
        // BooleanRotation.getFacingInBetween(1): not translated, its class isn't translated (its constants carry values).
        // BooleanRotation.is(1): not translated, its class isn't translated (its constants carry values).
        // BooleanRotation.get(2): not translated, it uses BooleanRotation, which isn't translated (its constants carry values).
    }

    /// `Mirror` (team/creative/creativecore/common/util/math/transformation/Mirror.java)
    pub mod mirror {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Mirror {
        }
        impl Mirror {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // Mirror.mirror(2): not translated, it uses Minecraft's `Direction`.
        // Mirror.mirror(2): not translated, it uses Minecraft's `Direction`.
        // Mirror.mirror(2): not translated, it uses Minecraft's `Direction`.
        // Mirror.mirror(2): not translated, it uses Minecraft's `Direction`.
        // Mirror.mirror(2): not translated, it uses Minecraft's `Direction`.
    }

    /// `Rotation` (team/creative/creativecore/common/util/math/transformation/Rotation.java)
    pub mod rotation {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // FACING_ROTATION: not translated, it uses Facing, which isn't translated (its constants carry values).
        // Rotation.getRotation(2): not translated, it uses Rotation, which isn't translated (its constants carry values).
        // Rotation.ofNormal(1): not translated, it uses Rotation, which isn't translated (its constants carry values).
        // Rotation.getRotation(1): not translated, it uses Rotation, which isn't translated (its constants carry values).
        // Rotation.getRotationCount(1): not translated, it uses `Rotation`.
        // Rotation.Rotation(3): not translated, its class isn't translated (its constants carry values).
        // Rotation.getMatrix(0): not translated, its class isn't translated (its constants carry values).
        // Rotation.opposite(0): not translated, it has no body.
        // Rotation.rotate(1): not translated, its class isn't translated (its constants carry values).
        // Rotation.rotate(1): not translated, its class isn't translated (its constants carry values).
        // Rotation.mirror(1): not translated, its class isn't translated (its constants carry values).
        // Rotation.rotate(1): not translated, its class isn't translated (its constants carry values).
        // Rotation.transform(1): not translated, its class isn't translated (its constants carry values).
        // Rotation.transform(1): not translated, its class isn't translated (its constants carry values).
        // Rotation.transform(1): not translated, its class isn't translated (its constants carry values).
        // Rotation.transform(1): not translated, its class isn't translated (its constants carry values).
        // Rotation.transform(1): not translated, its class isn't translated (its constants carry values).
        // Rotation.transform(1): not translated, its class isn't translated (its constants carry values).
    }

    /// `BooleanUtils` (team/creative/creativecore/common/util/math/utils/BooleanUtils.java)
    pub mod boolean_utils {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct BooleanUtils {
        }
        impl BooleanUtils {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn to_int(value: bool) -> i32 {
            return if value { 1 } else { 0 };
        }
        pub fn as_array(value: bool) -> Vec<bool> {
            return if value { vec![true] } else { vec![false] };
        }
        pub fn is_true(value: Option<bool>) -> bool {
            return value.is_some() && (value.expect("null"));
        }
        pub fn is_false(value: Option<bool>) -> bool {
            return value.is_some() && (!(value.expect("null")));
        }
        pub fn explicit_one_true(array: &[bool]) -> bool {
            let mut was_true: bool = false;
            for i in 0..(array.len() as i32) {
                if array[i as usize] {
                    if was_true {
                        return false;
                    }
                    was_true = true;
                }
            }
            return was_true;
        }
        pub fn explicit_one_false(array: &[bool]) -> bool {
            let mut was_false: bool = false;
            for i in 0..(array.len() as i32) {
                if !array[i as usize] {
                    if was_false {
                        return false;
                    }
                    was_false = true;
                }
            }
            return was_false;
        }
        pub fn count_true(array: &[bool]) -> i32 {
            let mut count: i32 = 0;
            for i in 0..(array.len() as i32) {
                if array[i as usize] {
                    count += 1;
                }
            }
            return count;
        }
        pub fn set(state: &mut [bool], new_state: &[bool]) {
            for i in 0..(state.len() as i32) {
                if i < (new_state.len() as i32) {
                    state[i as usize] = new_state[i as usize];
                } else {
                    state[i as usize] = false;
                }
            }
        }
        pub fn reset(state: &[bool]) {
            state.fill(false);
        }
        pub fn any(state: &[bool]) -> bool {
            for i in 0..(state.len() as i32) {
                if state[i as usize] {
                    return true;
                }
            }
            return false;
        }
        pub fn or(state: &mut [bool], state2: &[bool]) {
            for i in 0..(state.len() as i32) {
                state[i as usize] = state[i as usize] || state2[i as usize];
            }
        }
        pub fn equals(state: &[bool], state2: &[bool]) -> bool {
            for i in 0..(state.len() as i32) {
                if state[i as usize] != state2[i as usize] {
                    return false;
                }
            }
            return true;
        }
        pub fn bool_to_int(state: &[bool]) -> i32 {
            if (state.len() as i32) > 32 {
                panic!("{}", "Cannot convert more than 32 bits to an integer");
            }
            let mut n: i32 = 0;
            for i in 0..(state.len() as i32) {
                n = (n << 1) | (if state[i as usize] { 1 } else { 0 });
            }
            return n;
        }
        pub fn int_to_bool(number: i32, state: &mut [bool]) {
            if (state.len() as i32) > 32 {
                panic!("{}", "More than 32 bits cannot be stored in an integer");
            }
            for i in 0..(state.len() as i32) {
                state[(((state.len() as i32) - 1) - i) as usize] = ((1 << i) & number) != 0;
            }
        }
        // BooleanUtils.print(1): not translated, it calls `Arrays` (java.util).toString.
        pub fn get_required_bandwidth(mut number: i32) -> i32 {
            let mut digit: i32 = 0;
            while number != 0 {
                digit += 1;
                number /= 10;
            }
            return digit;
        }
        pub fn to_bits(number: i32, bandwidth: i32) -> Vec<bool> {
            let mut b: Vec<bool> = vec![false; bandwidth as usize];
            for i in 0..bandwidth {
                b[(((b.len() as i32) - i) - 1) as usize] = ((1 << ((bandwidth - i) - 1)) & number) != 0;
            }
            return b.to_vec();
        }
        pub fn to_number(b: &[bool]) -> i32 {
            let mut x: i32 = 0;
            for i in (0..=((b.len() as i32) - 1)).rev() {
                x = (x << 1) | (if b[i as usize] { 1 } else { 0 });
            }
            return x;
        }
        pub fn copy(state: Option<Vec<bool>>) -> Option<Vec<bool>> {
            if state.is_none() {
                return None;
            }
            let mut new_state: Vec<bool> = vec![false; ((state.as_ref().expect("null")).len() as i32) as usize];
            new_state[0..0 + (new_state.len() as i32) as usize].copy_from_slice(&(state.as_ref().expect("null"))[0..0 + (new_state.len() as i32) as usize]);
            return Some(new_state.to_vec());
        }
    }

    /// `IntegerUtils` (team/creative/creativecore/common/util/math/utils/IntegerUtils.java)
    pub mod integer_utils {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct IntegerUtils {
        }
        impl IntegerUtils {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn bit_is(number: i32, index: i32) -> bool {
            return (number & integer_utils::get_mask(index)) != 0;
        }
        pub fn get_mask(offset: i32) -> i32 {
            return 1 << offset;
        }
        pub fn set_3(number: i32, index: i32, value: bool) -> i32 {
            return if value { integer_utils::set_2(number, index) } else { integer_utils::unset(number, index) };
        }
        pub fn unset(number: i32, index: i32) -> i32 {
            return number & (!integer_utils::get_mask(index));
        }
        pub fn set_2(number: i32, index: i32) -> i32 {
            return number | integer_utils::get_mask(index);
        }
        pub fn print(number: i32) -> String {
            let mut builder: String = String::new();
            for i in 0..4 {
                for j in 0..8 {
                    builder.push_str(&(if integer_utils::bit_is(number, 31 - ((i * 8) + j)) { "1" } else { "0" }));
                }
                builder.push_str(" ");
            }
            return builder.to_string();
        }
    }

    /// `RangedBitSet` (team/creative/creativecore/common/util/math/vec/RangedBitSet.java)
    pub mod ranged_bit_set {
        use super::*;
        // The class itself isn't translated: it uses `BitSet` (java.util). Its static parts are below.
        // RangedBitSet.RangedBitSet(2): not translated, its class isn't translated (it uses `BitSet` (java.util)).
        // RangedBitSet.getMin(0): not translated, its class isn't translated (it uses `BitSet` (java.util)).
        // RangedBitSet.getMax(0): not translated, its class isn't translated (it uses `BitSet` (java.util)).
        // RangedBitSet.add(1): not translated, its class isn't translated (it uses `BitSet` (java.util)).
        // RangedBitSet.getRanges(0): not translated, its class isn't translated (it uses `BitSet` (java.util)).
    }

    /// `RangedBitSet.BitRange` (team/creative/creativecore/common/util/math/vec/RangedBitSet.java)
    pub mod ranged_bit_set_bit_range {
        use super::*;
        #[derive(Clone, Debug, PartialEq)]
        pub struct BitRange {
            pub min: i32,
            pub max: i32,
        }
        impl BitRange {
            pub fn new(min: i32, max: i32) -> Self {
                let mut s = Self { min: min, max: max };
                s.min = min;
                s.max = max;
                s
            }
            pub fn min(&self) -> i32 {
                return self.min;
            }
            pub fn max(&self) -> i32 {
                return self.max;
            }
        }
    }

    /// `SmoothValue` (team/creative/creativecore/common/util/math/vec/SmoothValue.java)
    pub mod smooth_value {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct SmoothValue {
            pub aimed: f64,
            pub current: f64,
            pub before: f64,
            pub timestamp: i64,
            pub time: i64,
        }
        impl SmoothValue {
            pub fn new_2(time: i64, inital_value: f64) -> Self {
                let mut s = Self { aimed: 0.0f64, current: 0.0f64, before: 0.0f64, timestamp: 0i64, time: time };
                s.time = time;
                s.set_start(inital_value);
                s
            }
            pub fn new_1(time: i64) -> Self {
                let mut s = Self::new_2(time, 0.0f64);
                s
            }
            pub fn set_start(&mut self, value: f64) {
                self.aimed = value;
                self.current = value;
                self.before = value;
                self.timestamp = 0i64;
            }
            pub fn add(&mut self, value: f64) {
                self.set(self.aimed + value);
            }
            pub fn set(&mut self, value: f64) {
                self.timestamp = j_millis();
                self.aimed = value;
                self.before = self.current;
            }
            pub fn tick(&mut self) {
                if self.timestamp != 0i64 {
                    if (self.timestamp + self.time) <= j_millis() {
                        self.current = self.aimed;
                        self.before = self.current;
                        self.timestamp = 0i64;
                    } else {
                        self.current = self.before + ((self.aimed - self.before) * (((j_millis() - self.timestamp) as f64) / (self.time as f64)));
                    }
                }
            }
            pub fn current(&self) -> f64 {
                return self.current;
            }
            pub fn aimed(&self) -> f64 {
                return self.aimed;
            }
        }
    }

    /// `Vec1d` (team/creative/creativecore/common/util/math/vec/Vec1d.java)
    pub mod vec1d {
        use super::*;
        // The class itself isn't translated: it extends `VecNd`. Its static parts are below.
        // Vec1d.Vec1d(0): not translated, its class isn't translated (it extends `VecNd`).
        // Vec1d.Vec1d(1): not translated, its class isn't translated (it extends `VecNd`).
        // Vec1d.Vec1d(1): not translated, its class isn't translated (it extends `VecNd`).
        // Vec1d.set(1): not translated, its class isn't translated (it extends `VecNd`).
        // Vec1d.get(1): not translated, its class isn't translated (it extends `VecNd`).
        // Vec1d.set(2): not translated, its class isn't translated (it extends `VecNd`).
        // Vec1d.dimensions(0): not translated, its class isn't translated (it extends `VecNd`).
        // Vec1d.copy(0): not translated, its class isn't translated (it extends `VecNd`).
        // Vec1d.add(1): not translated, its class isn't translated (it extends `VecNd`).
        // Vec1d.sub(1): not translated, its class isn't translated (it extends `VecNd`).
        // Vec1d.scale(1): not translated, its class isn't translated (it extends `VecNd`).
        // Vec1d.equals(1): not translated, its class isn't translated (it extends `VecNd`).
        // Vec1d.epsilonEquals(2): not translated, its class isn't translated (it extends `VecNd`).
        // Vec1d.distance(1): not translated, its class isn't translated (it extends `VecNd`).
        // Vec1d.distanceSqr(1): not translated, its class isn't translated (it extends `VecNd`).
        // Vec1d.length(0): not translated, its class isn't translated (it extends `VecNd`).
        // Vec1d.lengthSquared(0): not translated, its class isn't translated (it extends `VecNd`).
        // Vec1d.angle(1): not translated, its class isn't translated (it extends `VecNd`).
        // Vec1d.dot(1): not translated, its class isn't translated (it extends `VecNd`).
    }

    /// `Vec1f` (team/creative/creativecore/common/util/math/vec/Vec1f.java)
    pub mod vec1f {
        use super::*;
        // The class itself isn't translated: it extends `VecNf`. Its static parts are below.
        // Vec1f.Vec1f(0): not translated, its class isn't translated (it extends `VecNf`).
        // Vec1f.Vec1f(1): not translated, its class isn't translated (it extends `VecNf`).
        // Vec1f.Vec1f(1): not translated, its class isn't translated (it extends `VecNf`).
        // Vec1f.set(1): not translated, its class isn't translated (it extends `VecNf`).
        // Vec1f.get(1): not translated, its class isn't translated (it extends `VecNf`).
        // Vec1f.set(2): not translated, its class isn't translated (it extends `VecNf`).
        // Vec1f.dimensions(0): not translated, its class isn't translated (it extends `VecNf`).
        // Vec1f.copy(0): not translated, its class isn't translated (it extends `VecNf`).
        // Vec1f.add(1): not translated, its class isn't translated (it extends `VecNf`).
        // Vec1f.sub(1): not translated, its class isn't translated (it extends `VecNf`).
        // Vec1f.scale(1): not translated, its class isn't translated (it extends `VecNf`).
        // Vec1f.equals(1): not translated, its class isn't translated (it extends `VecNf`).
        // Vec1f.epsilonEquals(2): not translated, its class isn't translated (it extends `VecNf`).
        // Vec1f.distance(1): not translated, its class isn't translated (it extends `VecNf`).
        // Vec1f.distanceSqr(1): not translated, its class isn't translated (it extends `VecNf`).
        // Vec1f.length(0): not translated, its class isn't translated (it extends `VecNf`).
        // Vec1f.lengthSquared(0): not translated, its class isn't translated (it extends `VecNf`).
        // Vec1f.angle(1): not translated, its class isn't translated (it extends `VecNf`).
        // Vec1f.dot(1): not translated, its class isn't translated (it extends `VecNf`).
    }

    /// `Vec2d` (team/creative/creativecore/common/util/math/vec/Vec2d.java)
    pub mod vec2d {
        use super::*;
        // The class itself isn't translated: it extends `VecNd`. Its static parts are below.
        // Vec2d.Vec2d(0): not translated, its class isn't translated (it extends `VecNd`).
        // Vec2d.Vec2d(2): not translated, its class isn't translated (it extends `VecNd`).
        // Vec2d.Vec2d(1): not translated, its class isn't translated (it extends `VecNd`).
        // Vec2d.set(1): not translated, its class isn't translated (it extends `VecNd`).
        // Vec2d.get(1): not translated, its class isn't translated (it extends `VecNd`).
        // Vec2d.set(2): not translated, its class isn't translated (it extends `VecNd`).
        // Vec2d.set(2): not translated, its class isn't translated (it extends `VecNd`).
        // Vec2d.dimensions(0): not translated, its class isn't translated (it extends `VecNd`).
        // Vec2d.copy(0): not translated, its class isn't translated (it extends `VecNd`).
        // Vec2d.add(2): not translated, its class isn't translated (it extends `VecNd`).
        // Vec2d.add(1): not translated, its class isn't translated (it extends `VecNd`).
        // Vec2d.sub(2): not translated, its class isn't translated (it extends `VecNd`).
        // Vec2d.sub(1): not translated, its class isn't translated (it extends `VecNd`).
        // Vec2d.scale(1): not translated, its class isn't translated (it extends `VecNd`).
        // Vec2d.equals(1): not translated, its class isn't translated (it extends `VecNd`).
        // Vec2d.epsilonEquals(2): not translated, its class isn't translated (it extends `VecNd`).
        // Vec2d.distance(1): not translated, its class isn't translated (it extends `VecNd`).
        // Vec2d.distanceSqr(1): not translated, its class isn't translated (it extends `VecNd`).
        // Vec2d.length(0): not translated, its class isn't translated (it extends `VecNd`).
        // Vec2d.lengthSquared(0): not translated, its class isn't translated (it extends `VecNd`).
        // Vec2d.angle(1): not translated, its class isn't translated (it extends `VecNd`).
        // Vec2d.dot(1): not translated, its class isn't translated (it extends `VecNd`).
    }

    /// `Vec2f` (team/creative/creativecore/common/util/math/vec/Vec2f.java)
    pub mod vec2f {
        use super::*;
        // The class itself isn't translated: it extends `VecNf`. Its static parts are below.
        // Vec2f.Vec2f(0): not translated, its class isn't translated (it extends `VecNf`).
        // Vec2f.Vec2f(2): not translated, its class isn't translated (it extends `VecNf`).
        // Vec2f.Vec2f(1): not translated, its class isn't translated (it extends `VecNf`).
        // Vec2f.set(1): not translated, its class isn't translated (it extends `VecNf`).
        // Vec2f.get(1): not translated, its class isn't translated (it extends `VecNf`).
        // Vec2f.set(2): not translated, its class isn't translated (it extends `VecNf`).
        // Vec2f.dimensions(0): not translated, its class isn't translated (it extends `VecNf`).
        // Vec2f.copy(0): not translated, its class isn't translated (it extends `VecNf`).
        // Vec2f.add(2): not translated, its class isn't translated (it extends `VecNf`).
        // Vec2f.add(1): not translated, its class isn't translated (it extends `VecNf`).
        // Vec2f.sub(2): not translated, its class isn't translated (it extends `VecNf`).
        // Vec2f.sub(1): not translated, its class isn't translated (it extends `VecNf`).
        // Vec2f.scale(1): not translated, its class isn't translated (it extends `VecNf`).
        // Vec2f.equals(1): not translated, its class isn't translated (it extends `VecNf`).
        // Vec2f.epsilonEquals(2): not translated, its class isn't translated (it extends `VecNf`).
        // Vec2f.distance(1): not translated, its class isn't translated (it extends `VecNf`).
        // Vec2f.distanceSqr(1): not translated, its class isn't translated (it extends `VecNf`).
        // Vec2f.length(0): not translated, its class isn't translated (it extends `VecNf`).
        // Vec2f.lengthSquared(0): not translated, its class isn't translated (it extends `VecNf`).
        // Vec2f.angle(1): not translated, its class isn't translated (it extends `VecNf`).
        // Vec2f.dot(1): not translated, its class isn't translated (it extends `VecNf`).
    }

    /// `Vec3d` (team/creative/creativecore/common/util/math/vec/Vec3d.java)
    pub mod vec3d {
        use super::*;
        // The class itself isn't translated: it extends `VecNd`. Its static parts are below.
        // Vec3d.Vec3d(0): not translated, its class isn't translated (it extends `VecNd`).
        // Vec3d.Vec3d(1): not translated, its class isn't translated (it extends `VecNd`).
        // Vec3d.Vec3d(3): not translated, its class isn't translated (it extends `VecNd`).
        // Vec3d.Vec3d(1): not translated, its class isn't translated (it extends `VecNd`).
        // Vec3d.Vec3d(1): not translated, its class isn't translated (it extends `VecNd`).
        // Vec3d.Vec3d(1): not translated, its class isn't translated (it extends `VecNd`).
        // Vec3d.Vec3d(1): not translated, its class isn't translated (it extends `VecNd`).
        // Vec3d.Vec3d(1): not translated, its class isn't translated (it extends `VecNd`).
        // Vec3d.toVanilla(0): not translated, its class isn't translated (it extends `VecNd`).
        // Vec3d.toBlockPos(0): not translated, its class isn't translated (it extends `VecNd`).
        // Vec3d.set(1): not translated, its class isn't translated (it extends `VecNd`).
        // Vec3d.set(1): not translated, its class isn't translated (it extends `VecNd`).
        // Vec3d.get(1): not translated, its class isn't translated (it extends `VecNd`).
        // Vec3d.get(1): not translated, its class isn't translated (it extends `VecNd`).
        // Vec3d.set(3): not translated, its class isn't translated (it extends `VecNd`).
        // Vec3d.set(2): not translated, its class isn't translated (it extends `VecNd`).
        // Vec3d.set(2): not translated, its class isn't translated (it extends `VecNd`).
        // Vec3d.dimensions(0): not translated, its class isn't translated (it extends `VecNd`).
        // Vec3d.copy(0): not translated, its class isn't translated (it extends `VecNd`).
        // Vec3d.add(1): not translated, its class isn't translated (it extends `VecNd`).
        // Vec3d.sub(1): not translated, its class isn't translated (it extends `VecNd`).
        // Vec3d.scale(1): not translated, its class isn't translated (it extends `VecNd`).
        // Vec3d.equals(1): not translated, its class isn't translated (it extends `VecNd`).
        // Vec3d.epsilonEquals(2): not translated, its class isn't translated (it extends `VecNd`).
        // Vec3d.distance(1): not translated, its class isn't translated (it extends `VecNd`).
        // Vec3d.distance(1): not translated, its class isn't translated (it extends `VecNd`).
        // Vec3d.distanceSqr(1): not translated, its class isn't translated (it extends `VecNd`).
        // Vec3d.distance(3): not translated, its class isn't translated (it extends `VecNd`).
        // Vec3d.distanceSqr(3): not translated, its class isn't translated (it extends `VecNd`).
        // Vec3d.length(0): not translated, its class isn't translated (it extends `VecNd`).
        // Vec3d.lengthSquared(0): not translated, its class isn't translated (it extends `VecNd`).
        // Vec3d.angle(1): not translated, its class isn't translated (it extends `VecNd`).
        // Vec3d.cross(2): not translated, its class isn't translated (it extends `VecNd`).
        // Vec3d.dot(1): not translated, its class isn't translated (it extends `VecNd`).
    }

    /// `Vec3f` (team/creative/creativecore/common/util/math/vec/Vec3f.java)
    pub mod vec3f {
        use super::*;
        // The class itself isn't translated: it extends `VecNf`. Its static parts are below.
        // Vec3f.Vec3f(0): not translated, its class isn't translated (it extends `VecNf`).
        // Vec3f.Vec3f(3): not translated, its class isn't translated (it extends `VecNf`).
        // Vec3f.Vec3f(1): not translated, its class isn't translated (it extends `VecNf`).
        // Vec3f.Vec3f(1): not translated, its class isn't translated (it extends `VecNf`).
        // Vec3f.Vec3f(1): not translated, its class isn't translated (it extends `VecNf`).
        // Vec3f.toVanilla(0): not translated, its class isn't translated (it extends `VecNf`).
        // Vec3f.set(1): not translated, its class isn't translated (it extends `VecNf`).
        // Vec3f.get(1): not translated, its class isn't translated (it extends `VecNf`).
        // Vec3f.get(1): not translated, its class isn't translated (it extends `VecNf`).
        // Vec3f.set(3): not translated, its class isn't translated (it extends `VecNf`).
        // Vec3f.set(2): not translated, its class isn't translated (it extends `VecNf`).
        // Vec3f.set(2): not translated, its class isn't translated (it extends `VecNf`).
        // Vec3f.dimensions(0): not translated, its class isn't translated (it extends `VecNf`).
        // Vec3f.copy(0): not translated, its class isn't translated (it extends `VecNf`).
        // Vec3f.add(1): not translated, its class isn't translated (it extends `VecNf`).
        // Vec3f.sub(1): not translated, its class isn't translated (it extends `VecNf`).
        // Vec3f.scale(1): not translated, its class isn't translated (it extends `VecNf`).
        // Vec3f.equals(1): not translated, its class isn't translated (it extends `VecNf`).
        // Vec3f.epsilonEquals(2): not translated, its class isn't translated (it extends `VecNf`).
        // Vec3f.distance(1): not translated, its class isn't translated (it extends `VecNf`).
        // Vec3f.distanceSqr(1): not translated, its class isn't translated (it extends `VecNf`).
        // Vec3f.length(0): not translated, its class isn't translated (it extends `VecNf`).
        // Vec3f.lengthSquared(0): not translated, its class isn't translated (it extends `VecNf`).
        // Vec3f.angle(1): not translated, its class isn't translated (it extends `VecNf`).
        // Vec3f.cross(2): not translated, its class isn't translated (it extends `VecNf`).
        // Vec3f.dot(1): not translated, its class isn't translated (it extends `VecNf`).
    }

    /// `Vec4d` (team/creative/creativecore/common/util/math/vec/Vec4d.java)
    pub mod vec4d {
        use super::*;
        // The class itself isn't translated: it extends `VecNd`. Its static parts are below.
        // Vec4d.Vec4d(0): not translated, its class isn't translated (it extends `VecNd`).
        // Vec4d.Vec4d(4): not translated, its class isn't translated (it extends `VecNd`).
        // Vec4d.Vec4d(2): not translated, its class isn't translated (it extends `VecNd`).
        // Vec4d.Vec4d(1): not translated, its class isn't translated (it extends `VecNd`).
        // Vec4d.set(1): not translated, its class isn't translated (it extends `VecNd`).
        // Vec4d.get(1): not translated, its class isn't translated (it extends `VecNd`).
        // Vec4d.get(1): not translated, its class isn't translated (it extends `VecNd`).
        // Vec4d.set(4): not translated, its class isn't translated (it extends `VecNd`).
        // Vec4d.set(2): not translated, its class isn't translated (it extends `VecNd`).
        // Vec4d.set(2): not translated, its class isn't translated (it extends `VecNd`).
        // Vec4d.dimensions(0): not translated, its class isn't translated (it extends `VecNd`).
        // Vec4d.copy(0): not translated, its class isn't translated (it extends `VecNd`).
        // Vec4d.add(1): not translated, its class isn't translated (it extends `VecNd`).
        // Vec4d.sub(1): not translated, its class isn't translated (it extends `VecNd`).
        // Vec4d.scale(1): not translated, its class isn't translated (it extends `VecNd`).
        // Vec4d.equals(1): not translated, its class isn't translated (it extends `VecNd`).
        // Vec4d.epsilonEquals(2): not translated, its class isn't translated (it extends `VecNd`).
        // Vec4d.distance(1): not translated, its class isn't translated (it extends `VecNd`).
        // Vec4d.distanceSqr(1): not translated, its class isn't translated (it extends `VecNd`).
        // Vec4d.length(0): not translated, its class isn't translated (it extends `VecNd`).
        // Vec4d.lengthSquared(0): not translated, its class isn't translated (it extends `VecNd`).
        // Vec4d.angle(1): not translated, its class isn't translated (it extends `VecNd`).
        // Vec4d.dot(1): not translated, its class isn't translated (it extends `VecNd`).
    }

    /// `VecNd` (team/creative/creativecore/common/util/math/vec/VecNd.java)
    pub mod vec_nd {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // VecNd.VecNd(0): not translated, its class isn't translated (it's generic).
        // VecNd.VecNd(1): not translated, its class isn't translated (it's generic).
        // VecNd.set(1): not translated, it has no body.
        // VecNd.get(1): not translated, its class isn't translated (it's generic).
        // VecNd.set(2): not translated, its class isn't translated (it's generic).
        // VecNd.get(1): not translated, its class isn't translated (it's generic).
        // VecNd.set(2): not translated, its class isn't translated (it's generic).
        // VecNd.dimensions(0): not translated, it has no body.
        // VecNd.copy(0): not translated, it has no body.
        // VecNd.add(1): not translated, it has no body.
        // VecNd.add(2): not translated, its class isn't translated (it's generic).
        // VecNd.sub(1): not translated, it has no body.
        // VecNd.sub(2): not translated, its class isn't translated (it's generic).
        // VecNd.scale(1): not translated, it has no body.
        // VecNd.invert(0): not translated, its class isn't translated (it's generic).
        // VecNd.equals(1): not translated, it has no body.
        // VecNd.epsilonEquals(2): not translated, it has no body.
        // VecNd.epsilonEquals(1): not translated, its class isn't translated (it's generic).
        // VecNd.distance(1): not translated, it has no body.
        // VecNd.distanceSqr(1): not translated, it has no body.
        // VecNd.length(0): not translated, it has no body.
        // VecNd.lengthSquared(0): not translated, it has no body.
        // VecNd.normalize(0): not translated, its class isn't translated (it's generic).
        // VecNd.angle(1): not translated, it has no body.
        // VecNd.dot(1): not translated, it has no body.
        // VecNd.toLong(0): not translated, its class isn't translated (it's generic).
        // VecNd.toString(0): not translated, its class isn't translated (it's generic).
        // VecNd.load(1): not translated, it uses VecNd, which isn't translated (it's generic).
        // VecNd.createEmptyVec(1): not translated, it's generic.
    }

    /// `VecNf` (team/creative/creativecore/common/util/math/vec/VecNf.java)
    pub mod vec_nf {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // VecNf.VecNf(0): not translated, its class isn't translated (it's generic).
        // VecNf.VecNf(1): not translated, its class isn't translated (it's generic).
        // VecNf.set(1): not translated, it has no body.
        // VecNf.get(1): not translated, its class isn't translated (it's generic).
        // VecNf.set(2): not translated, its class isn't translated (it's generic).
        // VecNf.get(1): not translated, its class isn't translated (it's generic).
        // VecNf.set(2): not translated, its class isn't translated (it's generic).
        // VecNf.dimensions(0): not translated, it has no body.
        // VecNf.copy(0): not translated, it has no body.
        // VecNf.add(1): not translated, it has no body.
        // VecNf.add(2): not translated, its class isn't translated (it's generic).
        // VecNf.sub(1): not translated, it has no body.
        // VecNf.sub(2): not translated, its class isn't translated (it's generic).
        // VecNf.scale(1): not translated, it has no body.
        // VecNf.invert(0): not translated, its class isn't translated (it's generic).
        // VecNf.equals(1): not translated, it has no body.
        // VecNf.epsilonEquals(2): not translated, it has no body.
        // VecNf.epsilonEquals(1): not translated, its class isn't translated (it's generic).
        // VecNf.distance(1): not translated, it has no body.
        // VecNf.distanceSqr(1): not translated, it has no body.
        // VecNf.length(0): not translated, it has no body.
        // VecNf.lengthSquared(0): not translated, it has no body.
        // VecNf.normalize(0): not translated, its class isn't translated (it's generic).
        // VecNf.angle(1): not translated, it has no body.
        // VecNf.dot(1): not translated, it has no body.
        // VecNf.toString(0): not translated, its class isn't translated (it's generic).
    }

    /// `VectorUtils` (team/creative/creativecore/common/util/math/vec/VectorUtils.java)
    pub mod vector_utils {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct VectorUtils {
        }
        impl VectorUtils {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // VectorUtils.set(3): not translated, it uses `Vector3d` (org.joml).
        // VectorUtils.set(3): not translated, it uses `Vector3d` (org.joml).
        // VectorUtils.set(3): not translated, it uses `Vector3d` (org.joml).
        // VectorUtils.set(3): not translated, it uses `Vector3d` (org.joml).
        // VectorUtils.set(3): not translated, it uses `Vector3d` (org.joml).
        // VectorUtils.get(2): not translated, it's overloaded with the same number of parameters.
        // VectorUtils.get(2): not translated, it's overloaded with the same number of parameters.
        // VectorUtils.get(2): not translated, it's overloaded with the same number of parameters.
        // VectorUtils.get(2): not translated, it's overloaded with the same number of parameters.
        // VectorUtils.get(2): not translated, it's overloaded with the same number of parameters.
        // VectorUtils.get(4): not translated, it's overloaded with the same number of parameters.
        // VectorUtils.get(4): not translated, it's overloaded with the same number of parameters.
        // VectorUtils.get(4): not translated, it's overloaded with the same number of parameters.
        // VectorUtils.get(4): not translated, it's overloaded with the same number of parameters.
        // VectorUtils.get(4): not translated, it's overloaded with the same number of parameters.
        // VectorUtils.get(4): not translated, it's overloaded with the same number of parameters.
        // VectorUtils.isZero(1): not translated, it's overloaded with the same number of parameters.
        // VectorUtils.isZero(1): not translated, it's overloaded with the same number of parameters.
        // VectorUtils.equals(2): not translated, it's overloaded with the same number of parameters.
        // VectorUtils.equals(2): not translated, it's overloaded with the same number of parameters.
        // VectorUtils.within(3): not translated, it's overloaded with the same number of parameters.
        // VectorUtils.within(3): not translated, it's overloaded with the same number of parameters.
        // VectorUtils.greaterEquals(2): not translated, it's overloaded with the same number of parameters.
        // VectorUtils.smallerEquals(2): not translated, it's overloaded with the same number of parameters.
        // VectorUtils.greaterEquals(2): not translated, it's overloaded with the same number of parameters.
        // VectorUtils.smallerEquals(2): not translated, it's overloaded with the same number of parameters.
    }

    /// `ColorUtils` (team/creative/creativecore/common/util/mc/ColorUtils.java)
    pub mod color_utils {
        use super::*;
        // HEX: not translated, it uses `HexFormat` (java.util).
        pub const WHITE: i32 = -1;
        pub const RED: i32 = -65536;
        pub const GREEN: i32 = -16711936;
        pub const BLUE: i32 = -16776961;
        pub const LIGHT_BLUE: i32 = -16740609;
        pub const ORANGE: i32 = -23296;
        pub const YELLOW: i32 = -256;
        pub const CYAN: i32 = -16711681;
        pub const MAGENTA: i32 = -65281;
        pub const BLACK: i32 = -16777216;
        pub const GRAY: i32 = -5592406;
        pub const DARK_GRAY: i32 = -11184811;
        #[derive(Clone, Debug)]
        pub struct ColorUtils {
        }
        impl ColorUtils {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn alpha(color: i32) -> i32 {
            return (color >> 24) & 0xFF;
        }
        pub fn red(color: i32) -> i32 {
            return (color >> 16) & 0xFF;
        }
        pub fn green(color: i32) -> i32 {
            return (color >> 8) & 0xFF;
        }
        pub fn blue(color: i32) -> i32 {
            return color & 0xFF;
        }
        pub fn alpha_f(color: i32) -> f32 {
            return (((color >> 24) & 0xFF) as f32) / 255.0f32;
        }
        pub fn red_f(color: i32) -> f32 {
            return (((color >> 16) & 0xFF) as f32) / 255.0f32;
        }
        pub fn green_f(color: i32) -> f32 {
            return (((color >> 8) & 0xFF) as f32) / 255.0f32;
        }
        pub fn blue_f(color: i32) -> f32 {
            return ((color & 0xFF) as f32) / 255.0f32;
        }
        // ColorUtils.setAlpha(2): not translated, unknown static ColorPart.ALPHA.
        // ColorUtils.rgba(4): not translated, it's overloaded with the same number of parameters.
        // ColorUtils.rgba(4): not translated, it's overloaded with the same number of parameters.
        // ColorUtils.rgb(3): not translated, it's overloaded with the same number of parameters.
        // ColorUtils.rgb(3): not translated, it's overloaded with the same number of parameters.
        // ColorUtils.toVec(1): not translated, it uses `Vector3d` (org.joml).
        pub fn is_default(color: i32) -> bool {
            return color == (-1);
        }
        pub fn is_transparent(color: i32) -> bool {
            return ((color >> 24) & 0xFF) < 255;
        }
        pub fn is_invisible(color: i32) -> bool {
            return ((color >> 24) & 0xFF) == 0;
        }
        // ColorUtils.substract(2): not translated, it calls ColorUtils.rgba, which isn't translated.
        pub fn blend_2(i1: i32, i2: i32) -> i32 {
            return color_utils::blend_3(i1, i2, 0.5f32);
        }
        pub fn blend_3(i1: i32, i2: i32, mut ratio: f32) -> i32 {
            if ratio > 1.0f32 {
                ratio = 1.0f32;
            } else if ratio < 0.0f32 {
                ratio = 0.0f32;
            }
            let i_ratio: f32 = 1.0f32 - ratio;
            let a1: i32 = (i1 >> 24) & 0xFF;
            let r1: i32 = (i1 & 0xFF0000) >> 16;
            let g1: i32 = (i1 & 0xFF00) >> 8;
            let b1: i32 = i1 & 0xFF;
            let a2: i32 = (i2 >> 24) & 0xFF;
            let r2: i32 = (i2 & 0xFF0000) >> 16;
            let g2: i32 = (i2 & 0xFF00) >> 8;
            let b2: i32 = i2 & 0xFF;
            let a: i32 = ((((a1 as f32) * i_ratio) + ((a2 as f32) * ratio)) as i32);
            let r: i32 = ((((r1 as f32) * i_ratio) + ((r2 as f32) * ratio)) as i32);
            let g: i32 = ((((g1 as f32) * i_ratio) + ((g2 as f32) * ratio)) as i32);
            let b: i32 = ((((b1 as f32) * i_ratio) + ((b2 as f32) * ratio)) as i32);
            return (((a << 24) | (r << 16)) | (g << 8)) | b;
        }
        // ColorUtils.toHex(1): not translated, it uses ColorUtils.HEX, which isn't translated (it uses `HexFormat` (java.util)).
        // ColorUtils.fromHex(1): not translated, it calls `HexFormat` (java.util).fromHexDigits.
    }

    /// `ColorUtils.ColorPart` (team/creative/creativecore/common/util/mc/ColorUtils.java)
    pub mod color_utils_color_part {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // ColorUtils.ColorPart.ColorPart(1): not translated, its class isn't translated (its constants carry values).
        // ColorUtils.ColorPart.get(1): not translated, it has no body.
        // ColorUtils.ColorPart.set(2): not translated, it has no body.
    }

    /// `ConnectionUtils` (team/creative/creativecore/common/util/mc/ConnectionUtils.java)
    pub mod connection_utils {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ConnectionUtils {
        }
        impl ConnectionUtils {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ConnectionUtils.getProtocolInfo(1): not translated, it uses Minecraft's `ProtocolInfo`.
    }

    /// `JsonUtils` (team/creative/creativecore/common/util/mc/JsonUtils.java)
    pub mod json_utils {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct JsonUtils {
        }
        impl JsonUtils {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // JsonUtils.set(4): not translated, it calls JsonUtils.get, which isn't translated.
        // JsonUtils.tryGet(2): not translated, it calls `Arrays` (java.util).toString.
        // JsonUtils.get(2): not translated, it calls `Arrays` (java.util).toString.
        // JsonUtils.cleanUp(1): not translated, .iterator on a map's entries.
        // JsonUtils.of(1): not translated, it's overloaded with the same number of parameters.
        // JsonUtils.of(1): not translated, it's overloaded with the same number of parameters.
    }

    /// `LanguageUtils` (team/creative/creativecore/common/util/mc/LanguageUtils.java)
    pub mod language_utils {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct LanguageUtils {
        }
        impl LanguageUtils {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // LanguageUtils.can(1): not translated, it calls Minecraft's `Language`.getInstance.
        // LanguageUtils.translate(1): not translated, it uses ICreativeLoader, which isn't translated (it's an interface).
        // LanguageUtils.translateOr(2): not translated, it calls LanguageUtils.translate, which isn't translated.
        // LanguageUtils.translate(2): not translated, it uses `Object`.
    }

    /// `LevelUtils` (team/creative/creativecore/common/util/mc/LevelUtils.java)
    pub mod level_utils {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct LevelUtils {
        }
        impl LevelUtils {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // LevelUtils.dropItem(3): not translated, it's overloaded with the same number of parameters.
        // LevelUtils.dropItem(3): not translated, it's overloaded with the same number of parameters.
        // LevelUtils.dropItem(2): not translated, it's overloaded with the same number of parameters.
        // LevelUtils.dropItem(2): not translated, it's overloaded with the same number of parameters.
    }

    /// `NBTUtils` (team/creative/creativecore/common/util/mc/NBTUtils.java)
    pub mod nbt_utils {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct NBTUtils {
        }
        impl NBTUtils {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // NBTUtils.mergeNotOverwrite(2): not translated, it uses Minecraft's `CompoundTag`.
        // NBTUtils.of(1): not translated, it uses Minecraft's `CompoundTag`.
        // NBTUtils.of(2): not translated, it uses Minecraft's `CompoundTag`.
        // NBTUtils.of(1): not translated, it uses Minecraft's `CompoundTag`.
    }

    /// `PlatformResourceUtils` (team/creative/creativecore/common/util/mc/PlatformResourceUtils.java)
    pub mod platform_resource_utils {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct PlatformResourceUtils {
        }
        impl PlatformResourceUtils {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // PlatformResourceUtils.length(1): not translated, Path.toFile.
        // PlatformResourceUtils.resolvePath(4): not translated, it uses Minecraft's `PackType`.
    }

    /// `PlayerUtils` (team/creative/creativecore/common/util/mc/PlayerUtils.java)
    pub mod player_utils {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct PlayerUtils {
        }
        impl PlayerUtils {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // PlayerUtils.getPersistentData(1): not translated, it uses Minecraft's `CompoundTag`.
        // PlayerUtils.getGameTypeClient(1): not translated, it uses Minecraft's `GameType`.
        // PlayerUtils.isAdventure(1): not translated, it uses Minecraft's `Player`.
        // PlayerUtils.getGameType(1): not translated, it uses Minecraft's `GameType`.
        // PlayerUtils.getReach(1): not translated, it uses Minecraft's `Player`.
        // PlayerUtils.addOrDrop(2): not translated, it's overloaded with the same number of parameters.
        // PlayerUtils.addOrDrop(2): not translated, it's overloaded with the same number of parameters.
    }

    /// `ResourceUtils` (team/creative/creativecore/common/util/mc/ResourceUtils.java)
    pub mod resource_utils {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ResourceUtils {
        }
        impl ResourceUtils {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ResourceUtils.length(3): not translated, it's overloaded with the same number of parameters.
        // ResourceUtils.length(3): not translated, it's overloaded with the same number of parameters.
        // ResourceUtils.resolve(3): not translated, it calls Minecraft's `FileUtil`.resolvePath.
    }

    /// `StackUtils` (team/creative/creativecore/common/util/mc/StackUtils.java)
    pub mod stack_utils {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct StackUtils {
        }
        impl StackUtils {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // StackUtils.collect(3): needs the game, so it's in game_side.rs.
    }

    /// `TickUtils` (team/creative/creativecore/common/util/mc/TickUtils.java)
    pub mod tick_utils {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct TickUtils {
        }
        impl TickUtils {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // TickUtils.getFrameTimeClient(0): not translated, it calls CreativeCoreClient.getFrameTime, which isn't translated.
        // TickUtils.getFrameTime(1): not translated, it uses Minecraft's `LevelAccessor`.
    }

    /// `TooltipUtils` (team/creative/creativecore/common/util/mc/TooltipUtils.java)
    pub mod tooltip_utils {
        use super::*;
        // NUMBER_FORMAT: not translated, it uses `NumberFormat` (java.text).
        // DECIMAL_FORMAT: not translated, it uses `DecimalFormat` (java.text).
        #[derive(Clone, Debug)]
        pub struct TooltipUtils {
        }
        impl TooltipUtils {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // TooltipUtils.printColor(1): not translated, it uses Minecraft's `ChatFormatting`.RED.
        // TooltipUtils.print(1): not translated, it's overloaded with the same number of parameters.
        // TooltipUtils.print(1): not translated, it's overloaded with the same number of parameters.
        // TooltipUtils.print(1): not translated, it's overloaded with the same number of parameters.
        pub fn print_roman(mut input: i32) -> String {
            if (input >= 1) && (input <= 3999) {
                let mut s: String = String::new();
                while input >= 1000 {
                    s.push_str("M");
                    input -= 1000;
                }
                while input >= 900 {
                    s.push_str("CM");
                    input -= 900;
                }
                while input >= 500 {
                    s.push_str("D");
                    input -= 500;
                }
                while input >= 400 {
                    s.push_str("CD");
                    input -= 400;
                }
                while input >= 100 {
                    s.push_str("C");
                    input -= 100;
                }
                while input >= 90 {
                    s.push_str("XC");
                    input -= 90;
                }
                while input >= 50 {
                    s.push_str("L");
                    input -= 50;
                }
                while input >= 40 {
                    s.push_str("XL");
                    input -= 40;
                }
                while input >= 10 {
                    s.push_str("X");
                    input -= 10;
                }
                while input == 9 {
                    s.push_str("IX");
                    input -= 9;
                }
                while input >= 5 {
                    s.push_str("V");
                    input -= 5;
                }
                while input == 4 {
                    s.push_str("IV");
                    input -= 4;
                }
                while input >= 1 {
                    s.push_str("I");
                    input -= 1;
                }
                return s.to_string();
            } else {
                return format!("{}", input);
            }
            #[allow(unreachable_code)]
            unreachable!()
        }
    }

    /// `PlayerSelector` (team/creative/creativecore/common/util/player/PlayerSelector.java)
    pub mod player_selector {
        use super::*;
        // REGISTRY: not translated, it uses NamedTypeRegistry, which isn't translated (it's generic).
        #[derive(Clone, Debug)]
        pub struct PlayerSelector {
        }
        impl PlayerSelector {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // PlayerSelector.is(1): not translated, it has no body.
            // PlayerSelector.readFromNBT(1): not translated, it has no body.
            // PlayerSelector.write(1): not translated, it has no body.
            // PlayerSelector.info(0): not translated, it has no body.
            // PlayerSelector.writeToNBT(1): not translated, it uses Minecraft's `CompoundTag`.
        }
        // PlayerSelector.read(1): not translated, it uses Minecraft's `CompoundTag`.
    }

    /// `PlayerSelector.PlayerSelectorAnd` (team/creative/creativecore/common/util/player/PlayerSelector.java)
    pub mod player_selector_player_selector_and {
        use super::*;
        // The class itself isn't translated: it extends `PlayerSelector`. Its static parts are below.
        // PlayerSelector.PlayerSelectorAnd.PlayerSelectorAnd(0): not translated, its class isn't translated (it extends `PlayerSelector`).
        // PlayerSelector.PlayerSelectorAnd.PlayerSelectorAnd(1): not translated, its class isn't translated (it extends `PlayerSelector`).
        // PlayerSelector.PlayerSelectorAnd.is(1): not translated, its class isn't translated (it extends `PlayerSelector`).
        // PlayerSelector.PlayerSelectorAnd.readFromNBT(1): not translated, its class isn't translated (it extends `PlayerSelector`).
        // PlayerSelector.PlayerSelectorAnd.write(1): not translated, its class isn't translated (it extends `PlayerSelector`).
        // PlayerSelector.PlayerSelectorAnd.info(0): not translated, its class isn't translated (it extends `PlayerSelector`).
    }

    /// `PlayerSelector.PlayerSelectorCommandSelector` (team/creative/creativecore/common/util/player/PlayerSelector.java)
    pub mod player_selector_player_selector_command_selector {
        use super::*;
        // The class itself isn't translated: it extends `PlayerSelector`. Its static parts are below.
        // PlayerSelector.PlayerSelectorCommandSelector.PlayerSelectorCommandSelector(0): not translated, its class isn't translated (it extends `PlayerSelector`).
        // PlayerSelector.PlayerSelectorCommandSelector.PlayerSelectorCommandSelector(1): not translated, its class isn't translated (it extends `PlayerSelector`).
        // PlayerSelector.PlayerSelectorCommandSelector.is(1): not translated, its class isn't translated (it extends `PlayerSelector`).
        // PlayerSelector.PlayerSelectorCommandSelector.readFromNBT(1): not translated, its class isn't translated (it extends `PlayerSelector`).
        // PlayerSelector.PlayerSelectorCommandSelector.write(1): not translated, its class isn't translated (it extends `PlayerSelector`).
        // PlayerSelector.PlayerSelectorCommandSelector.info(0): not translated, its class isn't translated (it extends `PlayerSelector`).
    }

    /// `PlayerSelector.PlayerSelectorGamemode` (team/creative/creativecore/common/util/player/PlayerSelector.java)
    pub mod player_selector_player_selector_gamemode {
        use super::*;
        // The class itself isn't translated: it extends `PlayerSelector`. Its static parts are below.
        // PlayerSelector.PlayerSelectorGamemode.PlayerSelectorGamemode(0): not translated, its class isn't translated (it extends `PlayerSelector`).
        // PlayerSelector.PlayerSelectorGamemode.PlayerSelectorGamemode(1): not translated, its class isn't translated (it extends `PlayerSelector`).
        // PlayerSelector.PlayerSelectorGamemode.is(1): not translated, its class isn't translated (it extends `PlayerSelector`).
        // PlayerSelector.PlayerSelectorGamemode.readFromNBT(1): not translated, its class isn't translated (it extends `PlayerSelector`).
        // PlayerSelector.PlayerSelectorGamemode.write(1): not translated, its class isn't translated (it extends `PlayerSelector`).
        // PlayerSelector.PlayerSelectorGamemode.info(0): not translated, its class isn't translated (it extends `PlayerSelector`).
    }

    /// `PlayerSelector.PlayerSelectorLevel` (team/creative/creativecore/common/util/player/PlayerSelector.java)
    pub mod player_selector_player_selector_level {
        use super::*;
        // The class itself isn't translated: it extends `PlayerSelector`. Its static parts are below.
        // PlayerSelector.PlayerSelectorLevel.PlayerSelectorLevel(0): not translated, its class isn't translated (it extends `PlayerSelector`).
        // PlayerSelector.PlayerSelectorLevel.PlayerSelectorLevel(1): not translated, its class isn't translated (it extends `PlayerSelector`).
        // PlayerSelector.PlayerSelectorLevel.is(1): not translated, its class isn't translated (it extends `PlayerSelector`).
        // PlayerSelector.PlayerSelectorLevel.readFromNBT(1): not translated, its class isn't translated (it extends `PlayerSelector`).
        // PlayerSelector.PlayerSelectorLevel.write(1): not translated, its class isn't translated (it extends `PlayerSelector`).
        // PlayerSelector.PlayerSelectorLevel.info(0): not translated, its class isn't translated (it extends `PlayerSelector`).
    }

    /// `PlayerSelector.PlayerSelectorNot` (team/creative/creativecore/common/util/player/PlayerSelector.java)
    pub mod player_selector_player_selector_not {
        use super::*;
        // The class itself isn't translated: it extends `PlayerSelector`. Its static parts are below.
        // PlayerSelector.PlayerSelectorNot.PlayerSelectorNot(0): not translated, its class isn't translated (it extends `PlayerSelector`).
        // PlayerSelector.PlayerSelectorNot.PlayerSelectorNot(1): not translated, its class isn't translated (it extends `PlayerSelector`).
        // PlayerSelector.PlayerSelectorNot.is(1): not translated, its class isn't translated (it extends `PlayerSelector`).
        // PlayerSelector.PlayerSelectorNot.readFromNBT(1): not translated, its class isn't translated (it extends `PlayerSelector`).
        // PlayerSelector.PlayerSelectorNot.write(1): not translated, its class isn't translated (it extends `PlayerSelector`).
        // PlayerSelector.PlayerSelectorNot.info(0): not translated, its class isn't translated (it extends `PlayerSelector`).
    }

    /// `PlayerSelector.PlayerSelectorOr` (team/creative/creativecore/common/util/player/PlayerSelector.java)
    pub mod player_selector_player_selector_or {
        use super::*;
        // The class itself isn't translated: it extends `PlayerSelector`. Its static parts are below.
        // PlayerSelector.PlayerSelectorOr.PlayerSelectorOr(0): not translated, its class isn't translated (it extends `PlayerSelector`).
        // PlayerSelector.PlayerSelectorOr.PlayerSelectorOr(1): not translated, its class isn't translated (it extends `PlayerSelector`).
        // PlayerSelector.PlayerSelectorOr.is(1): not translated, its class isn't translated (it extends `PlayerSelector`).
        // PlayerSelector.PlayerSelectorOr.readFromNBT(1): not translated, its class isn't translated (it extends `PlayerSelector`).
        // PlayerSelector.PlayerSelectorOr.write(1): not translated, its class isn't translated (it extends `PlayerSelector`).
        // PlayerSelector.PlayerSelectorOr.info(0): not translated, its class isn't translated (it extends `PlayerSelector`).
    }

    /// `ConfigTypeRegistry` (team/creative/creativecore/common/util/registry/ConfigTypeRegistry.java)
    pub mod config_type_registry {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // DEFAULT_FIELD_PREDICATE: not translated, it uses `Field` (java.lang.reflect).
        // ConfigTypeRegistry.setFieldPredicate(1): not translated, its class isn't translated (it's generic).
        // ConfigTypeRegistry.setKeepUnrelatedData(1): not translated, its class isn't translated (it's generic).
        // ConfigTypeRegistry.register(4): not translated, it's generic.
        // ConfigTypeRegistry.get(1): not translated, its class isn't translated (it's generic).
        // ConfigTypeRegistry.contains(1): not translated, its class isn't translated (it's generic).
        // ConfigTypeRegistry.getId(1): not translated, its class isn't translated (it's generic).
        // ConfigTypeRegistry.load(3): not translated, its class isn't translated (it's generic).
        // ConfigTypeRegistry.loadOrCreateDefault(4): not translated, its class isn't translated (it's generic).
        // ConfigTypeRegistry.save(4): not translated, its class isn't translated (it's generic).
        // ConfigTypeRegistry.create(4): not translated, its class isn't translated (it's generic).
        // ConfigTypeRegistry.createDefault(1): not translated, its class isn't translated (it's generic).
        // ConfigTypeRegistry.getEqualChecker(0): not translated, its class isn't translated (it's generic).
        // ConfigTypeRegistry.is(1): not translated, its class isn't translated (it's generic).
    }

    /// `ConfigTypeRegistry.ConfigurationType` (team/creative/creativecore/common/util/registry/ConfigTypeRegistry.java)
    pub mod config_type_registry_configuration_type {
        use super::*;
        // The class itself isn't translated: it uses `T`. Its static parts are below.
        // ConfigTypeRegistry.ConfigurationType.ConfigurationType(4): not translated, its class isn't translated (it uses `T`).
        // ConfigTypeRegistry.ConfigurationType.create(2): not translated, its class isn't translated (it uses `T`).
        // ConfigTypeRegistry.ConfigurationType.save(4): not translated, its class isn't translated (it uses `T`).
        // ConfigTypeRegistry.ConfigurationType.load(3): not translated, its class isn't translated (it uses `T`).
    }

    /// `IdNotFoundException` (team/creative/creativecore/common/util/registry/exception/IdNotFoundException.java)
    pub mod id_not_found_exception {
        use super::*;
        // The class itself isn't translated: it extends `RegistryException`. Its static parts are below.
        // IdNotFoundException.IdNotFoundException(1): not translated, its class isn't translated (it extends `RegistryException`).
    }

    /// `RegistryException` (team/creative/creativecore/common/util/registry/exception/RegistryException.java)
    pub mod registry_exception {
        use super::*;
        // The class itself isn't translated: it extends `Exception`. Its static parts are below.
        // RegistryException.RegistryException(1): not translated, its class isn't translated (it extends `Exception`).
    }

    /// `FilteredHandlerRegistry` (team/creative/creativecore/common/util/registry/FilteredHandlerRegistry.java)
    pub mod filtered_handler_registry {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // FilteredHandlerRegistry.FilteredHandlerRegistry(1): not translated, its class isn't translated (it's generic).
        // FilteredHandlerRegistry.allowOverwrite(0): not translated, its class isn't translated (it's generic).
        // FilteredHandlerRegistry.getDefault(0): not translated, its class isn't translated (it's generic).
        // FilteredHandlerRegistry.register(2): not translated, its class isn't translated (it's generic).
        // FilteredHandlerRegistry.register(2): not translated, its class isn't translated (it's generic).
        // FilteredHandlerRegistry.get(1): not translated, its class isn't translated (it's generic).
        // FilteredHandlerRegistry.handlers(0): not translated, its class isn't translated (it's generic).
    }

    /// `ICreativeRegistry` (team/creative/creativecore/common/util/registry/ICreativeRegistry.java)
    pub mod i_creative_registry {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ICreativeRegistry.get(1): not translated, it has no body.
        // ICreativeRegistry.name(1): not translated, it has no body.
        // ICreativeRegistry.names(0): not translated, it has no body.
        // ICreativeRegistry.values(0): not translated, it has no body.
        // ICreativeRegistry.next(1): not translated, its class isn't translated (it's an interface).
    }

    /// `LocatedHandlerRegistry` (team/creative/creativecore/common/util/registry/LocatedHandlerRegistry.java)
    pub mod located_handler_registry {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // LocatedHandlerRegistry.LocatedHandlerRegistry(1): not translated, its class isn't translated (it's generic).
        // LocatedHandlerRegistry.allowOverwrite(0): not translated, its class isn't translated (it's generic).
        // LocatedHandlerRegistry.getDefault(0): not translated, its class isn't translated (it's generic).
        // LocatedHandlerRegistry.register(2): not translated, its class isn't translated (it's generic).
        // LocatedHandlerRegistry.registerDefault(2): not translated, its class isn't translated (it's generic).
        // LocatedHandlerRegistry.getLocation(1): not translated, its class isn't translated (it's generic).
        // LocatedHandlerRegistry.get(1): not translated, its class isn't translated (it's generic).
        // LocatedHandlerRegistry.getOrThrow(1): not translated, its class isn't translated (it's generic).
        // LocatedHandlerRegistry.contains(1): not translated, its class isn't translated (it's generic).
        // LocatedHandlerRegistry.keys(0): not translated, its class isn't translated (it's generic).
        // LocatedHandlerRegistry.entrySet(0): not translated, its class isn't translated (it's generic).
        // LocatedHandlerRegistry.values(0): not translated, its class isn't translated (it's generic).
        // LocatedHandlerRegistry.get(1): not translated, its class isn't translated (it's generic).
        // LocatedHandlerRegistry.name(1): not translated, its class isn't translated (it's generic).
        // LocatedHandlerRegistry.names(0): not translated, its class isn't translated (it's generic).
    }

    /// `NamedClassBoundHandlerRegistry` (team/creative/creativecore/common/util/registry/NamedClassBoundHandlerRegistry.java)
    pub mod named_class_bound_handler_registry {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // NamedClassBoundHandlerRegistry.clearRegistry(1): not translated, it uses NamedClassBoundHandlerRegistry, which isn't translated (it's generic).
        // NamedClassBoundHandlerRegistry.allowOverwrite(0): not translated, its class isn't translated (it's generic).
        // NamedClassBoundHandlerRegistry.getDefault(0): not translated, its class isn't translated (it's generic).
        // NamedClassBoundHandlerRegistry.register(3): not translated, its class isn't translated (it's generic).
        // NamedClassBoundHandlerRegistry.registerDefault(3): not translated, its class isn't translated (it's generic).
        // NamedClassBoundHandlerRegistry.getId(1): not translated, its class isn't translated (it's generic).
        // NamedClassBoundHandlerRegistry.getId(1): not translated, its class isn't translated (it's generic).
        // NamedClassBoundHandlerRegistry.get(1): not translated, its class isn't translated (it's generic).
        // NamedClassBoundHandlerRegistry.get(1): not translated, its class isn't translated (it's generic).
        // NamedClassBoundHandlerRegistry.getOrThrow(1): not translated, its class isn't translated (it's generic).
        // NamedClassBoundHandlerRegistry.getOrThrow(1): not translated, its class isn't translated (it's generic).
        // NamedClassBoundHandlerRegistry.contains(1): not translated, its class isn't translated (it's generic).
        // NamedClassBoundHandlerRegistry.contains(1): not translated, its class isn't translated (it's generic).
        // NamedClassBoundHandlerRegistry.keys(0): not translated, its class isn't translated (it's generic).
        // NamedClassBoundHandlerRegistry.entrySet(0): not translated, its class isn't translated (it's generic).
        // NamedClassBoundHandlerRegistry.values(0): not translated, its class isn't translated (it's generic).
        // NamedClassBoundHandlerRegistry.classes(0): not translated, its class isn't translated (it's generic).
    }

    /// `NamedHandlerRegistry` (team/creative/creativecore/common/util/registry/NamedHandlerRegistry.java)
    pub mod named_handler_registry {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // NamedHandlerRegistry.clearRegistry(1): not translated, it uses NamedHandlerRegistry, which isn't translated (it's generic).
        // NamedHandlerRegistry.NamedHandlerRegistry(1): not translated, its class isn't translated (it's generic).
        // NamedHandlerRegistry.allowOverwrite(0): not translated, its class isn't translated (it's generic).
        // NamedHandlerRegistry.getDefault(0): not translated, its class isn't translated (it's generic).
        // NamedHandlerRegistry.register(2): not translated, it's generic.
        // NamedHandlerRegistry.registerDefault(2): not translated, its class isn't translated (it's generic).
        // NamedHandlerRegistry.getId(1): not translated, its class isn't translated (it's generic).
        // NamedHandlerRegistry.get(1): not translated, its class isn't translated (it's generic).
        // NamedHandlerRegistry.getOrThrow(1): not translated, its class isn't translated (it's generic).
        // NamedHandlerRegistry.contains(1): not translated, its class isn't translated (it's generic).
        // NamedHandlerRegistry.keys(0): not translated, its class isn't translated (it's generic).
        // NamedHandlerRegistry.names(0): not translated, its class isn't translated (it's generic).
        // NamedHandlerRegistry.entrySet(0): not translated, its class isn't translated (it's generic).
        // NamedHandlerRegistry.values(0): not translated, its class isn't translated (it's generic).
        // NamedHandlerRegistry.name(1): not translated, its class isn't translated (it's generic).
    }

    /// `NamedTypeRegistry` (team/creative/creativecore/common/util/registry/NamedTypeRegistry.java)
    pub mod named_type_registry {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // NamedTypeRegistry.addConstructorPattern(1): not translated, its class isn't translated (it's generic).
        // NamedTypeRegistry.allowOverwrite(0): not translated, its class isn't translated (it's generic).
        // NamedTypeRegistry.printConstructors(0): not translated, its class isn't translated (it's generic).
        // NamedTypeRegistry.checkConstructor(1): not translated, its class isn't translated (it's generic).
        // NamedTypeRegistry.checkConstructor(1): not translated, its class isn't translated (it's generic).
        // NamedTypeRegistry.register(2): not translated, its class isn't translated (it's generic).
        // NamedTypeRegistry.getId(1): not translated, its class isn't translated (it's generic).
        // NamedTypeRegistry.getIdOrDefault(2): not translated, its class isn't translated (it's generic).
        // NamedTypeRegistry.getId(1): not translated, its class isn't translated (it's generic).
        // NamedTypeRegistry.get(1): not translated, its class isn't translated (it's generic).
        // NamedTypeRegistry.create(2): not translated, its class isn't translated (it's generic).
        // NamedTypeRegistry.createSafe(3): not translated, its class isn't translated (it's generic).
        // NamedTypeRegistry.keys(0): not translated, its class isn't translated (it's generic).
        // NamedTypeRegistry.entrySet(0): not translated, its class isn't translated (it's generic).
        // NamedTypeRegistry.values(0): not translated, its class isn't translated (it's generic).
        // NamedTypeRegistry.contains(1): not translated, its class isn't translated (it's generic).
        // NamedTypeRegistry.toString(1): not translated, it's overloaded with the same number of parameters.
        // NamedTypeRegistry.toString(1): not translated, it's overloaded with the same number of parameters.
    }

    /// `NamedTypeRegistry.ConstructorForbiddenException` (team/creative/creativecore/common/util/registry/NamedTypeRegistry.java)
    pub mod named_type_registry_constructor_forbidden_exception {
        use super::*;
        // The class itself isn't translated: it extends `RegistryException` (team.creative.creativecore.common.util.registry.exception). Its static parts are below.
        // NamedTypeRegistry.ConstructorForbiddenException.ConstructorForbiddenException(2): not translated, its class isn't translated (it extends `RegistryException` (team.creative.creativecore.common.util.registry.exception)).
    }

    /// `NamedTypeRegistry.ConstructorNotFoundException` (team/creative/creativecore/common/util/registry/NamedTypeRegistry.java)
    pub mod named_type_registry_constructor_not_found_exception {
        use super::*;
        // The class itself isn't translated: it extends `RegistryException` (team.creative.creativecore.common.util.registry.exception). Its static parts are below.
        // NamedTypeRegistry.ConstructorNotFoundException.ConstructorNotFoundException(1): not translated, its class isn't translated (it extends `RegistryException` (team.creative.creativecore.common.util.registry.exception)).
    }

    /// `AdvancedComponentHelper` (team/creative/creativecore/common/util/text/AdvancedComponentHelper.java)
    pub mod advanced_component_helper {
        use super::*;
        // SPLITTER: not translated, it uses AdvancedStringSplitter, which isn't translated (it extends Minecraft's `StringSplitter`).
        // STOP_ITERATION: not translated, it uses `Object`.
        #[derive(Clone, Debug)]
        pub struct AdvancedComponentHelper {
        }
        impl AdvancedComponentHelper {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // AdvancedComponentHelper.copy(1): not translated, it uses Minecraft's `Component`.
        // AdvancedComponentHelper.iterateFormatted(3): not translated, its body couldn't be read: an anonymous class.
        // AdvancedComponentHelper.visit(3): not translated, it's generic.
        // AdvancedComponentHelper.visit(3): not translated, it's generic.
    }

    /// `AdvancedStringSplitter` (team/creative/creativecore/common/util/text/AdvancedStringSplitter.java)
    pub mod advanced_string_splitter {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `StringSplitter`. Its static parts are below.
        // AdvancedStringSplitter.AdvancedStringSplitter(1): not translated, its class isn't translated (it extends Minecraft's `StringSplitter`).
        // AdvancedStringSplitter.stringWidth(1): not translated, its body couldn't be read: an anonymous class.
    }

    /// `AdvancedContent` (team/creative/creativecore/common/util/text/content/AdvancedContent.java)
    pub mod advanced_content {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // AdvancedContent.visit(2): not translated, it's generic.
        // AdvancedContent.width(2): not translated, it has no body.
        // AdvancedContent.height(0): not translated, it has no body.
        // AdvancedContent.asText(0): not translated, it has no body.
    }

    /// `AdvancedContentConsumer` (team/creative/creativecore/common/util/text/content/AdvancedContentConsumer.java)
    pub mod advanced_content_consumer {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // AdvancedContentConsumer.accept(2): not translated, it has no body.
        // AdvancedContentConsumer.accept(2): not translated, it has no body.
    }

    /// `AdvancedFormattedText` (team/creative/creativecore/common/util/text/content/AdvancedFormattedText.java)
    pub mod advanced_formatted_text {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // AdvancedFormattedText.visit(1): not translated, it's generic.
        // AdvancedFormattedText.visit(2): not translated, it's generic.
        // AdvancedFormattedText.width(2): not translated, it has no body.
        // AdvancedFormattedText.height(0): not translated, it has no body.
        // AdvancedFormattedText.render(2): not translated, it has no body.
    }

    /// `ContentItemStack` (team/creative/creativecore/common/util/text/content/ContentItemStack.java)
    pub mod content_item_stack {
        use super::*;
        // The class itself isn't translated: it implements `AdvancedContent`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // ContentItemStack.ContentItemStack(1): not translated, its class isn't translated (it implements `AdvancedContent`).
        // ContentItemStack.visit(2): not translated, it's generic.
        // ContentItemStack.visit(1): not translated, it's generic.
        // ContentItemStack.width(2): not translated, its class isn't translated (it implements `AdvancedContent`).
        // ContentItemStack.height(0): not translated, its class isn't translated (it implements `AdvancedContent`).
        // ContentItemStack.asText(0): not translated, its class isn't translated (it implements `AdvancedContent`).
        // ContentItemStack.codec(0): not translated, its class isn't translated (it implements `AdvancedContent`).
        // ContentItemStack.stack(0): not translated, its class isn't translated (it implements `AdvancedContent`).
    }

    /// `ContentItemStack.ContentItemStackText` (team/creative/creativecore/common/util/text/content/ContentItemStack.java)
    pub mod content_item_stack_content_item_stack_text {
        use super::*;
        // The class itself isn't translated: it implements `AdvancedFormattedText`. Its static parts are below.
        // ContentItemStack.ContentItemStackText.ContentItemStackText(1): not translated, its class isn't translated (it implements `AdvancedFormattedText`).
        // ContentItemStack.ContentItemStackText.width(2): not translated, its class isn't translated (it implements `AdvancedFormattedText`).
        // ContentItemStack.ContentItemStackText.height(0): not translated, its class isn't translated (it implements `AdvancedFormattedText`).
        // ContentItemStack.ContentItemStackText.render(2): not translated, its class isn't translated (it implements `AdvancedFormattedText`).
        // ContentItemStack.ContentItemStackText.content(0): not translated, its class isn't translated (it implements `AdvancedFormattedText`).
    }

    /// `FormattedSingleSink` (team/creative/creativecore/common/util/text/content/FormattedSingleSink.java)
    pub mod formatted_single_sink {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // FormattedSingleSink.accept(3): not translated, it has no body.
        // FormattedSingleSink.accept(2): not translated, it has no body.
    }

    /// `IComponentMap` (team/creative/creativecore/common/util/text/IComponentMap.java)
    pub mod i_component_map {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IComponentMap.entrySet(0): not translated, it has no body.
        // IComponentMap.values(0): not translated, it has no body.
        // IComponentMap.keys(0): not translated, it has no body.
        // IComponentMap.build(0): not translated, it has no body.
    }

    /// `TextBuilder` (team/creative/creativecore/common/util/text/TextBuilder.java)
    pub mod text_builder {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Component`. Its static parts are below.
        // format: not translated, it uses `NumberFormat` (java.text).
        // TextBuilder.TextBuilder(0): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // TextBuilder.TextBuilder(1): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // TextBuilder.add(1): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // TextBuilder.TextBuilder(1): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // TextBuilder.translateIfCan(1): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // TextBuilder.translate(1): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // TextBuilder.translate(2): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // TextBuilder.getLastStyle(0): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // TextBuilder.text(1): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // TextBuilder.number(1): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // TextBuilder.number(2): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // TextBuilder.number(1): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // TextBuilder.number(2): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // TextBuilder.newLine(0): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // TextBuilder.bool(1): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // TextBuilder.color(1): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // TextBuilder.stack(1): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // TextBuilder.build(0): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // TextBuilder.textColor(1): not translated, its class isn't translated (it uses Minecraft's `Component`).
    }

    /// `TextMapBuilder` (team/creative/creativecore/common/util/text/TextMapBuilder.java)
    pub mod text_map_builder {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // TextMapBuilder.setFilter(1): not translated, its class isn't translated (it's generic).
        // TextMapBuilder.addNewLine(2): not translated, its class isn't translated (it's generic).
        // TextMapBuilder.addNewLine(2): not translated, its class isn't translated (it's generic).
        // TextMapBuilder.addComponent(2): not translated, its class isn't translated (it's generic).
        // TextMapBuilder.addComponent(2): not translated, its class isn't translated (it's generic).
        // TextMapBuilder.addComponent(2): not translated, its class isn't translated (it's generic).
        // TextMapBuilder.addComponent(2): not translated, its class isn't translated (it's generic).
        // TextMapBuilder.addEntrySet(2): not translated, its class isn't translated (it's generic).
        // TextMapBuilder.addComponents(2): not translated, its class isn't translated (it's generic).
        // TextMapBuilder.size(0): not translated, its class isn't translated (it's generic).
        // TextMapBuilder.first(0): not translated, its class isn't translated (it's generic).
        // TextMapBuilder.build(0): not translated, its class isn't translated (it's generic).
        // TextMapBuilder.entrySet(0): not translated, its class isn't translated (it's generic).
        // TextMapBuilder.values(0): not translated, its class isn't translated (it's generic).
        // TextMapBuilder.keys(0): not translated, its class isn't translated (it's generic).
    }

    /// `Bunch` (team/creative/creativecore/common/util/type/Bunch.java)
    pub mod bunch {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // Bunch.size(0): not translated, it has no body.
    }

    /// `Color` (team/creative/creativecore/common/util/type/Color.java)
    pub mod color {
        use super::*;
        // NONE: not translated, it creates a Color with a constructor that isn't translated.
        // WHITE: not translated, it creates a Color with a constructor that isn't translated.
        // RED: not translated, it creates a Color with a constructor that isn't translated.
        // GREEN: not translated, it creates a Color with a constructor that isn't translated.
        // BLUE: not translated, it creates a Color with a constructor that isn't translated.
        // LIGHT_BLUE: not translated, it creates a Color with a constructor that isn't translated.
        // ORANGE: not translated, it creates a Color with a constructor that isn't translated.
        // YELLOW: not translated, it creates a Color with a constructor that isn't translated.
        // CYAN: not translated, it creates a Color with a constructor that isn't translated.
        // MAGENTA: not translated, it creates a Color with a constructor that isn't translated.
        // BLACK: not translated, it creates a Color with a constructor that isn't translated.
        #[derive(Clone, Debug)]
        pub struct Color {
            pub red: i16,
            pub green: i16,
            pub blue: i16,
            pub alpha: i16,
        }
        impl Color {
            // Color.Color(0): not translated, its constructors are overloaded with the same number of parameters.
            // Color.Color(1): not translated, its constructors are overloaded with the same number of parameters.
            // Color.Color(1): not translated, its constructors are overloaded with the same number of parameters.
            // Color.Color(3): not translated, its constructors are overloaded with the same number of parameters.
            // Color.Color(3): not translated, its constructors are overloaded with the same number of parameters.
            // Color.Color(3): not translated, its constructors are overloaded with the same number of parameters.
            // Color.Color(4): not translated, its constructors are overloaded with the same number of parameters.
            // Color.Color(4): not translated, its constructors are overloaded with the same number of parameters.
            // Color.Color(4): not translated, its constructors are overloaded with the same number of parameters.
            // Color.Color(1): not translated, its constructors are overloaded with the same number of parameters.
            // Color.Color(1): not translated, its constructors are overloaded with the same number of parameters.
            // Color.set(1): not translated, it's overloaded with the same number of parameters.
            // Color.set(1): not translated, it's overloaded with the same number of parameters.
            // Color.set(2): not translated, it's overloaded with the same number of parameters.
            // Color.set(2): not translated, it's overloaded with the same number of parameters.
            // Color.setAlpha(1): not translated, it's overloaded with the same number of parameters.
            // Color.setAlpha(1): not translated, it's overloaded with the same number of parameters.
            // Color.setRed(1): not translated, it's overloaded with the same number of parameters.
            // Color.setRed(1): not translated, it's overloaded with the same number of parameters.
            // Color.setGreen(1): not translated, it's overloaded with the same number of parameters.
            // Color.setGreen(1): not translated, it's overloaded with the same number of parameters.
            // Color.setBlue(1): not translated, it's overloaded with the same number of parameters.
            // Color.setBlue(1): not translated, it's overloaded with the same number of parameters.
            // Color.get(1): not translated, it uses ColorType, which isn't translated (its constants carry values).
            pub fn get_alpha(&self) -> i16 {
                return self.alpha;
            }
            pub fn get_red(&self) -> i16 {
                return self.red;
            }
            pub fn get_green(&self) -> i16 {
                return self.green;
            }
            pub fn get_blue(&self) -> i16 {
                return self.blue;
            }
            // Color.getDecimal(1): not translated, it uses ColorType, which isn't translated (its constants carry values).
            pub fn get_alpha_decimal(&self) -> f32 {
                return (self.alpha as f32) / 255.0f32;
            }
            pub fn get_red_decimal(&self) -> f32 {
                return (self.red as f32) / 255.0f32;
            }
            pub fn get_green_decimal(&self) -> f32 {
                return (self.green as f32) / 255.0f32;
            }
            pub fn get_blue_decimal(&self) -> f32 {
                return (self.blue as f32) / 255.0f32;
            }
            pub fn to_int(&self) -> i32 {
                return (((((self.alpha as i32) & 0xFF) << 24) | (((self.red as i32) & 0xFF) << 16)) | (((self.green as i32) & 0xFF) << 8)) | ((self.blue as i32) & 0xFF);
            }
            pub fn is_default(&self) -> bool {
                return ((((self.red as i32) == 255) && ((self.green as i32) == 255)) && ((self.blue as i32) == 255)) && ((self.alpha as i32) == 255);
            }
            pub fn is_white_0(&self) -> bool {
                return (((self.red as i32) == 255) && ((self.green as i32) == 255)) && ((self.blue as i32) == 255);
            }
            pub fn is_transparent_0(&self) -> bool {
                return (self.alpha as i32) < 255;
            }
            pub fn is_invisible_0(&self) -> bool {
                return (self.alpha as i32) == 0;
            }
            // Color.blend(1): not translated, it's overloaded with the same number of parameters.
            // Color.blend(2): not translated, it's overloaded with the same number of parameters.
            // Color.toVec(0): not translated, it uses Vec3d, which isn't translated (it extends `VecNd`).
            // Color.hex(0): not translated, it calls ColorUtils.toHex, which isn't translated.
        }
        pub fn is_white_1(color: i32) -> bool {
            let r: i32 = (color >> 16) & 0xFF;
            let g: i32 = (color >> 8) & 0xFF;
            let b: i32 = color & 0xFF;
            return ((r == 255) && (g == 255)) && (b == 255);
        }
        pub fn is_transparent_1(color: i32) -> bool {
            let a: i32 = (color >> 24) & 0xFF;
            return a < 255;
        }
        pub fn is_invisible_1(color: i32) -> bool {
            let a: i32 = (color >> 24) & 0xFF;
            return a == 0;
        }
        // Color.blend(2): not translated, it's overloaded with the same number of parameters.
        // Color.blend(3): not translated, it's overloaded with the same number of parameters.
        // Color.blend(2): not translated, it's overloaded with the same number of parameters.
        // Color.blend(3): not translated, it's overloaded with the same number of parameters.
    }

    /// `ColorType` (team/creative/creativecore/common/util/type/ColorType.java)
    pub mod color_type {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // ColorType.getBrightest(0): not translated, it has no body.
    }

    /// `ArrayIterator` (team/creative/creativecore/common/util/type/itr/ArrayIterator.java)
    pub mod array_iterator {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // ArrayIterator.ArrayIterator(1): not translated, its class isn't translated (it's generic).
        // ArrayIterator.hasNext(0): not translated, its class isn't translated (it's generic).
        // ArrayIterator.next(0): not translated, its class isn't translated (it's generic).
    }

    /// `ArrayOffsetIterator` (team/creative/creativecore/common/util/type/itr/ArrayOffsetIterator.java)
    pub mod array_offset_iterator {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // ArrayOffsetIterator.ArrayOffsetIterator(2): not translated, its class isn't translated (it's generic).
        // ArrayOffsetIterator.hasNext(0): not translated, its class isn't translated (it's generic).
        // ArrayOffsetIterator.next(0): not translated, its class isn't translated (it's generic).
    }

    /// `ComputeNextIterator` (team/creative/creativecore/common/util/type/itr/ComputeNextIterator.java)
    pub mod compute_next_iterator {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // ComputeNextIterator.computeNext(0): not translated, it has no body.
        // ComputeNextIterator.end(0): not translated, its class isn't translated (it's generic).
        // ComputeNextIterator.hasNext(0): not translated, its class isn't translated (it's generic).
        // ComputeNextIterator.next(0): not translated, its class isn't translated (it's generic).
    }

    /// `ComputeNextListIterator` (team/creative/creativecore/common/util/type/itr/ComputeNextListIterator.java)
    pub mod compute_next_list_iterator {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // ComputeNextListIterator.computeNext(0): not translated, it has no body.
        // ComputeNextListIterator.endNext(0): not translated, its class isn't translated (it's generic).
        // ComputeNextListIterator.hasNext(0): not translated, its class isn't translated (it's generic).
        // ComputeNextListIterator.next(0): not translated, its class isn't translated (it's generic).
        // ComputeNextListIterator.computePrevious(0): not translated, it has no body.
        // ComputeNextListIterator.endPrevious(0): not translated, its class isn't translated (it's generic).
        // ComputeNextListIterator.hasPrevious(0): not translated, its class isn't translated (it's generic).
        // ComputeNextListIterator.previous(0): not translated, its class isn't translated (it's generic).
    }

    /// `ConsecutiveIterator` (team/creative/creativecore/common/util/type/itr/ConsecutiveIterator.java)
    pub mod consecutive_iterator {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // ConsecutiveIterator.ConsecutiveIterator(1): not translated, its class isn't translated (it's generic).
        // ConsecutiveIterator.ConsecutiveIterator(1): not translated, its class isn't translated (it's generic).
        // ConsecutiveIterator.hasNext(0): not translated, its class isn't translated (it's generic).
        // ConsecutiveIterator.next(0): not translated, its class isn't translated (it's generic).
    }

    /// `ConsecutiveListIterator` (team/creative/creativecore/common/util/type/itr/ConsecutiveListIterator.java)
    pub mod consecutive_list_iterator {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // ConsecutiveListIterator.ConsecutiveListIterator(1): not translated, its class isn't translated (it's generic).
        // ConsecutiveListIterator.ConsecutiveListIterator(1): not translated, its class isn't translated (it's generic).
        // ConsecutiveListIterator.goEnd(0): not translated, its class isn't translated (it's generic).
        // ConsecutiveListIterator.hasNext(0): not translated, its class isn't translated (it's generic).
        // ConsecutiveListIterator.next(0): not translated, its class isn't translated (it's generic).
        // ConsecutiveListIterator.hasPrevious(0): not translated, its class isn't translated (it's generic).
        // ConsecutiveListIterator.previous(0): not translated, its class isn't translated (it's generic).
        // ConsecutiveListIterator.nextIndex(0): not translated, its class isn't translated (it's generic).
        // ConsecutiveListIterator.previousIndex(0): not translated, its class isn't translated (it's generic).
        // ConsecutiveListIterator.remove(0): not translated, its class isn't translated (it's generic).
        // ConsecutiveListIterator.set(1): not translated, its class isn't translated (it's generic).
        // ConsecutiveListIterator.add(1): not translated, its class isn't translated (it's generic).
    }

    /// `FilterIterator` (team/creative/creativecore/common/util/type/itr/FilterIterator.java)
    pub mod filter_iterator {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // FilterIterator.skipNull(1): not translated, it's generic.
        // FilterIterator.FilterIterator(2): not translated, its class isn't translated (it's generic).
        // FilterIterator.FilterIterator(2): not translated, its class isn't translated (it's generic).
        // FilterIterator.FilterIterator(2): not translated, its class isn't translated (it's generic).
        // FilterIterator.FilterIterator(2): not translated, its class isn't translated (it's generic).
        // FilterIterator.computeNext(0): not translated, its class isn't translated (it's generic).
    }

    /// `FilterListIterator` (team/creative/creativecore/common/util/type/itr/FilterListIterator.java)
    pub mod filter_list_iterator {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // FilterListIterator.skipNull(1): not translated, it's generic.
        // FilterListIterator.FilterListIterator(2): not translated, its class isn't translated (it's generic).
        // FilterListIterator.FilterListIterator(2): not translated, its class isn't translated (it's generic).
        // FilterListIterator.FilterListIterator(2): not translated, its class isn't translated (it's generic).
        // FilterListIterator.FilterListIterator(2): not translated, its class isn't translated (it's generic).
        // FilterListIterator.computeNext(0): not translated, its class isn't translated (it's generic).
        // FilterListIterator.computePrevious(0): not translated, its class isn't translated (it's generic).
        // FilterListIterator.nextIndex(0): not translated, its class isn't translated (it's generic).
        // FilterListIterator.previousIndex(0): not translated, its class isn't translated (it's generic).
        // FilterListIterator.remove(0): not translated, its class isn't translated (it's generic).
        // FilterListIterator.set(1): not translated, its class isn't translated (it's generic).
        // FilterListIterator.add(1): not translated, its class isn't translated (it's generic).
    }

    /// `FunctionIterator` (team/creative/creativecore/common/util/type/itr/FunctionIterator.java)
    pub mod function_iterator {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // FunctionIterator.FunctionIterator(2): not translated, it's generic.
        // FunctionIterator.FunctionIterator(2): not translated, it's generic.
        // FunctionIterator.hasNext(0): not translated, its class isn't translated (it's generic).
        // FunctionIterator.next(0): not translated, its class isn't translated (it's generic).
    }

    /// `FunctionNonNullIterator` (team/creative/creativecore/common/util/type/itr/FunctionNonNullIterator.java)
    pub mod function_non_null_iterator {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // FunctionNonNullIterator.FunctionNonNullIterator(2): not translated, it's generic.
        // FunctionNonNullIterator.FunctionNonNullIterator(2): not translated, it's generic.
        // FunctionNonNullIterator.computeNext(0): not translated, its class isn't translated (it's generic).
    }

    /// `InverseArrayIterator` (team/creative/creativecore/common/util/type/itr/InverseArrayIterator.java)
    pub mod inverse_array_iterator {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // InverseArrayIterator.InverseArrayIterator(1): not translated, its class isn't translated (it's generic).
        // InverseArrayIterator.hasNext(0): not translated, its class isn't translated (it's generic).
        // InverseArrayIterator.next(0): not translated, its class isn't translated (it's generic).
    }

    /// `InverseConsecutiveIterator` (team/creative/creativecore/common/util/type/itr/InverseConsecutiveIterator.java)
    pub mod inverse_consecutive_iterator {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // InverseConsecutiveIterator.InverseConsecutiveIterator(1): not translated, its class isn't translated (it's generic).
        // InverseConsecutiveIterator.InverseConsecutiveIterator(1): not translated, its class isn't translated (it's generic).
        // InverseConsecutiveIterator.hasNext(0): not translated, its class isn't translated (it's generic).
        // InverseConsecutiveIterator.next(0): not translated, its class isn't translated (it's generic).
    }

    /// `InverseListIterator` (team/creative/creativecore/common/util/type/itr/InverseListIterator.java)
    pub mod inverse_list_iterator {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // InverseListIterator.InverseListIterator(1): not translated, its class isn't translated (it's generic).
        // InverseListIterator.hasNext(0): not translated, its class isn't translated (it's generic).
        // InverseListIterator.next(0): not translated, its class isn't translated (it's generic).
    }

    /// `IterableIterator` (team/creative/creativecore/common/util/type/itr/IterableIterator.java)
    pub mod iterable_iterator {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IterableIterator.iterator(0): not translated, its class isn't translated (it's an interface).
    }

    /// `NestedFunctionIterator` (team/creative/creativecore/common/util/type/itr/NestedFunctionIterator.java)
    pub mod nested_function_iterator {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // NestedFunctionIterator.NestedFunctionIterator(2): not translated, it's generic.
        // NestedFunctionIterator.NestedFunctionIterator(2): not translated, it's generic.
        // NestedFunctionIterator.hasNext(0): not translated, its class isn't translated (it's generic).
        // NestedFunctionIterator.next(0): not translated, its class isn't translated (it's generic).
    }

    /// `NestedIterator` (team/creative/creativecore/common/util/type/itr/NestedIterator.java)
    pub mod nested_iterator {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // NestedIterator.NestedIterator(1): not translated, its class isn't translated (it's generic).
        // NestedIterator.NestedIterator(1): not translated, its class isn't translated (it's generic).
        // NestedIterator.hasNext(0): not translated, its class isn't translated (it's generic).
        // NestedIterator.next(0): not translated, its class isn't translated (it's generic).
    }

    /// `SingleIterator` (team/creative/creativecore/common/util/type/itr/SingleIterator.java)
    pub mod single_iterator {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // SingleIterator.SingleIterator(1): not translated, its class isn't translated (it's generic).
        // SingleIterator.hasNext(0): not translated, its class isn't translated (it's generic).
        // SingleIterator.next(0): not translated, its class isn't translated (it's generic).
        // SingleIterator.remove(0): not translated, its class isn't translated (it's generic).
        // SingleIterator.forEachRemaining(1): not translated, its class isn't translated (it's generic).
    }

    /// `TreeIterator` (team/creative/creativecore/common/util/type/itr/TreeIterator.java)
    pub mod tree_iterator {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // TreeIterator.TreeIterator(2): not translated, its class isn't translated (it's generic).
        // TreeIterator.hasNext(0): not translated, its class isn't translated (it's generic).
        // TreeIterator.next(0): not translated, its class isn't translated (it's generic).
    }

    /// `BucketList` (team/creative/creativecore/common/util/type/list/BucketList.java)
    pub mod bucket_list {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // BucketList.BucketList(0): not translated, its class isn't translated (it's generic).
        // BucketList.BucketList(1): not translated, its class isn't translated (it's generic).
        // BucketList.BucketList(2): not translated, its class isn't translated (it's generic).
        // BucketList.ensureBucketSize(1): not translated, its class isn't translated (it's generic).
        // BucketList.recountSize(0): not translated, its class isn't translated (it's generic).
        // BucketList.getBucket(2): not translated, its class isn't translated (it's generic).
        // BucketList.add(2): not translated, its class isn't translated (it's generic).
        // BucketList.addAll(2): not translated, its class isn't translated (it's generic).
        // BucketList.remove(2): not translated, its class isn't translated (it's generic).
        // BucketList.removeAll(2): not translated, its class isn't translated (it's generic).
        // BucketList.removeBucket(1): not translated, its class isn't translated (it's generic).
        // BucketList.clear(0): not translated, its class isn't translated (it's generic).
        // BucketList.removeBuckets(0): not translated, its class isn't translated (it's generic).
        // BucketList.buckets(0): not translated, its class isn't translated (it's generic).
        // BucketList.bucketCount(0): not translated, its class isn't translated (it's generic).
        // BucketList.size(0): not translated, its class isn't translated (it's generic).
        // BucketList.iterator(0): not translated, its class isn't translated (it's generic).
        // BucketList.inverseIterator(0): not translated, its class isn't translated (it's generic).
    }

    /// `CopyArrayCollection` (team/creative/creativecore/common/util/type/list/CopyArrayCollection.java)
    pub mod copy_array_collection {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        pub const DEFAULT_CAPACITY: i32 = 10;
        // EMPTY_ELEMENTDATA: not translated, it uses `Object`.
        // DEFAULTCAPACITY_EMPTY_ELEMENTDATA: not translated, it uses `Object`.
        pub const SOFT_MAX_ARRAY_LENGTH: i32 = 2147483639;
        pub fn new_length(old_length: i32, min_growth: i32, pref_growth: i32) -> i32 {
            let pref_length: i32 = old_length + (min_growth.max(pref_growth));
            return if (0 < pref_length) && (pref_length <= 2147483639) { pref_length } else { copy_array_collection::huge_length(old_length, min_growth) };
        }
        pub fn huge_length(old_length: i32, min_growth: i32) -> i32 {
            let min_length: i32 = old_length + min_growth;
            if min_length < 0 {
                panic!("{}", format!("Required array length {} + {} is too large", old_length, min_growth));
            } else {
                return if min_length <= 2147483639 { 2147483639 } else { min_length };
            }
            #[allow(unreachable_code)]
            unreachable!()
        }
        // CopyArrayCollection.CopyArrayCollection(0): not translated, its class isn't translated (it's generic).
        // CopyArrayCollection.CopyArrayCollection(1): not translated, its class isn't translated (it's generic).
        // CopyArrayCollection.CopyArrayCollection(1): not translated, its class isn't translated (it's generic).
        // CopyArrayCollection.CopyArrayCollection(1): not translated, its class isn't translated (it's generic).
        // CopyArrayCollection.CopyArrayCollection(1): not translated, its class isn't translated (it's generic).
        // CopyArrayCollection.CopyArrayCollection(1): not translated, its class isn't translated (it's generic).
        // CopyArrayCollection.trimToSize(0): not translated, its class isn't translated (it's generic).
        // CopyArrayCollection.ensureCapacity(1): not translated, its class isn't translated (it's generic).
        // CopyArrayCollection.grow(1): not translated, its class isn't translated (it's generic).
        // CopyArrayCollection.grow(0): not translated, its class isn't translated (it's generic).
        // CopyArrayCollection.size(0): not translated, its class isn't translated (it's generic).
        // CopyArrayCollection.isEmpty(0): not translated, its class isn't translated (it's generic).
        // CopyArrayCollection.first(0): not translated, its class isn't translated (it's generic).
        // CopyArrayCollection.contains(1): not translated, its class isn't translated (it's generic).
        // CopyArrayCollection.indexOfRange(4): not translated, its class isn't translated (it's generic).
        // CopyArrayCollection.clone(0): not translated, its class isn't translated (it's generic).
        // CopyArrayCollection.toArray(0): not translated, its class isn't translated (it's generic).
        // CopyArrayCollection.toArray(1): not translated, it's generic.
        // CopyArrayCollection.add(1): not translated, its class isn't translated (it's generic).
        // CopyArrayCollection.clear(0): not translated, its class isn't translated (it's generic).
        // CopyArrayCollection.addAll(1): not translated, its class isn't translated (it's generic).
        // CopyArrayCollection.containsAll(1): not translated, its class isn't translated (it's generic).
        // CopyArrayCollection.iterator(0): not translated, its body couldn't be read: an anonymous class.
        // CopyArrayCollection.remove(1): not translated, its class isn't translated (it's generic).
        // CopyArrayCollection.remove(3): not translated, its class isn't translated (it's generic).
        // CopyArrayCollection.fastRemove(2): not translated, its class isn't translated (it's generic).
        // CopyArrayCollection.removeAll(1): not translated, its class isn't translated (it's generic).
        // CopyArrayCollection.retainAll(1): not translated, its class isn't translated (it's generic).
        // CopyArrayCollection.removeIf(1): not translated, its class isn't translated (it's generic).
        // CopyArrayCollection.bulkRemove(1): not translated, its class isn't translated (it's generic).
        pub fn n_bits(n: i32) -> Vec<i64> {
            return vec![0i64; (((n - 1) >> 6) + 1) as usize];
        }
        pub fn set_bit(bits: &mut [i64], i: i32) {
            bits[(i >> 6) as usize] = bits[(i >> 6) as usize] | (1i64 << i);
        }
        pub fn is_clear(bits: &[i64], i: i32) -> bool {
            return (bits[(i >> 6) as usize] & (1i64 << i)) == 0i64;
        }
        // CopyArrayCollection.bulkRemove(3): not translated, its class isn't translated (it's generic).
        // CopyArrayCollection.equals(1): not translated, its class isn't translated (it's generic).
        // CopyArrayCollection.hashCodeOfRange(3): not translated, it uses `Object`.
        // CopyArrayCollection.hashCode(0): not translated, its class isn't translated (it's generic).
        // CopyArrayCollection.toString(0): not translated, its class isn't translated (it's generic).
    }

    /// `IndexedCollector` (team/creative/creativecore/common/util/type/list/IndexedCollector.java)
    pub mod indexed_collector {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // PREDICATE: not translated, a raw Predicate.
        // IndexedCollector.add(2): not translated, its class isn't translated (it's generic).
        // IndexedCollector.add(1): not translated, its class isn't translated (it's generic).
        // IndexedCollector.startSection(1): not translated, its class isn't translated (it's generic).
        // IndexedCollector.endSection(0): not translated, its class isn't translated (it's generic).
        // IndexedCollector.isEmpty(0): not translated, its class isn't translated (it's generic).
        // IndexedCollector.iterator(0): not translated, its class isn't translated (it's generic).
        // IndexedCollector.clear(0): not translated, its class isn't translated (it's generic).
        // IndexedCollector.sectionIterator(1): not translated, its body couldn't be read: an anonymous class.
    }

    /// `MarkIterator` (team/creative/creativecore/common/util/type/list/MarkIterator.java)
    pub mod mark_iterator {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // MarkIterator.mark(0): not translated, it has no body.
    }

    /// `MarkList` (team/creative/creativecore/common/util/type/list/MarkList.java)
    pub mod mark_list {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // EMPTY: not translated, it uses MarkList, which isn't translated (it's generic).
        // MarkList.MarkList(1): not translated, its class isn't translated (it's generic).
        // MarkList.mark(1): not translated, its class isn't translated (it's generic).
        // MarkList.reset(1): not translated, its class isn't translated (it's generic).
        // MarkList.is(1): not translated, its class isn't translated (it's generic).
        // MarkList.remaing(0): not translated, its class isn't translated (it's generic).
        // MarkList.isEmpty(0): not translated, its class isn't translated (it's generic).
        // MarkList.iterator(0): not translated, its body couldn't be read: an anonymous class.
        // MarkList.clear(0): not translated, its class isn't translated (it's generic).
        // MarkList.allIgnoreMark(0): not translated, its class isn't translated (it's generic).
        // MarkList.getIgnoreMark(1): not translated, its class isn't translated (it's generic).
        // MarkList.sizeIgnoreMark(0): not translated, its class isn't translated (it's generic).
    }

    /// `Pair` (team/creative/creativecore/common/util/type/list/Pair.java)
    pub mod pair {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // Pair.Pair(2): not translated, its class isn't translated (it's generic).
        // Pair.getKey(0): not translated, its class isn't translated (it's generic).
        // Pair.getValue(0): not translated, its class isn't translated (it's generic).
        // Pair.setValue(1): not translated, its class isn't translated (it's generic).
        // Pair.hashCode(0): not translated, its class isn't translated (it's generic).
        // Pair.is(1): not translated, its class isn't translated (it's generic).
        // Pair.equals(1): not translated, its class isn't translated (it's generic).
        // Pair.toString(0): not translated, its class isn't translated (it's generic).
    }

    /// `PairList` (team/creative/creativecore/common/util/type/list/PairList.java)
    pub mod pair_list {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // PairList.PairList(0): not translated, its class isn't translated (it's generic).
        // PairList.PairList(1): not translated, its class isn't translated (it's generic).
        // PairList.updateEntireMap(0): not translated, its class isn't translated (it's generic).
        // PairList.add(1): not translated, its class isn't translated (it's generic).
        // PairList.add(2): not translated, its class isn't translated (it's generic).
        // PairList.addAll(1): not translated, its class isn't translated (it's generic).
        // PairList.addAll(2): not translated, its class isn't translated (it's generic).
        // PairList.add(2): not translated, its class isn't translated (it's generic).
        // PairList.set(2): not translated, its class isn't translated (it's generic).
        // PairList.remove(1): not translated, its class isn't translated (it's generic).
        // PairList.remove(1): not translated, its class isn't translated (it's generic).
        // PairList.removeAll(1): not translated, its class isn't translated (it's generic).
        // PairList.removeIf(1): not translated, its class isn't translated (it's generic).
        // PairList.retainAll(1): not translated, its class isn't translated (it's generic).
        // PairList.clear(0): not translated, its class isn't translated (it's generic).
        // PairList.set(2): not translated, its class isn't translated (it's generic).
        // PairList.sort(1): not translated, its class isn't translated (it's generic).
        // PairList.containsKey(1): not translated, its class isn't translated (it's generic).
        // PairList.indexOfKey(1): not translated, its class isn't translated (it's generic).
        // PairList.removeKey(1): not translated, its class isn't translated (it's generic).
        // PairList.values(0): not translated, its class isn't translated (it's generic).
        // PairList.keys(0): not translated, its class isn't translated (it's generic).
        // PairList.getFirst(0): not translated, its class isn't translated (it's generic).
        // PairList.getLast(0): not translated, its class isn't translated (it's generic).
        // PairList.getValue(1): not translated, its class isn't translated (it's generic).
        // PairList.getPair(1): not translated, its class isn't translated (it's generic).
        // PairList.setKey(2): not translated, its class isn't translated (it's generic).
    }

    /// `SingletonList` (team/creative/creativecore/common/util/type/list/SingletonList.java)
    pub mod singleton_list {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        pub const SERIAL_VERSION_UID: i64 = 3093736618740652951i64;
        // SingletonList.SingletonList(1): not translated, its class isn't translated (it's generic).
        // SingletonList.setElement(1): not translated, its class isn't translated (it's generic).
        // SingletonList.iterator(0): not translated, its class isn't translated (it's generic).
        // SingletonList.size(0): not translated, its class isn't translated (it's generic).
        // SingletonList.contains(1): not translated, its class isn't translated (it's generic).
        // SingletonList.get(1): not translated, its class isn't translated (it's generic).
        // SingletonList.forEach(1): not translated, its class isn't translated (it's generic).
        // SingletonList.removeIf(1): not translated, its class isn't translated (it's generic).
        // SingletonList.replaceAll(1): not translated, its class isn't translated (it's generic).
        // SingletonList.sort(1): not translated, its class isn't translated (it's generic).
        // SingletonList.spliterator(0): not translated, its body couldn't be read: an anonymous class.
    }

    /// `SortingBlockList` (team/creative/creativecore/common/util/type/list/SortingBlockList.java)
    pub mod sorting_block_list {
        use super::*;
        // The class itself isn't translated: it uses RegistryObjectListConfig, which isn't translated (it's generic). Its static parts are below.
        // SortingBlockList.add(1): not translated, its class isn't translated (it uses RegistryObjectListConfig, which isn't translated (it's generic)).
        // SortingBlockList.add(1): not translated, its class isn't translated (it uses RegistryObjectListConfig, which isn't translated (it's generic)).
        // SortingBlockList.is(1): not translated, its class isn't translated (it uses RegistryObjectListConfig, which isn't translated (it's generic)).
        // SortingBlockList.is(1): not translated, its class isn't translated (it uses RegistryObjectListConfig, which isn't translated (it's generic)).
    }

    /// `SortingList` (team/creative/creativecore/common/util/type/list/SortingList.java)
    pub mod sorting_list {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct SortingList {
            pub entries: Vec<creative_ingredient::CreativeIngredient>,
            pub is_whitelist: bool,
        }
        impl SortingList {
            // SortingList.SortingList(1): not translated, its constructors are overloaded with the same number of parameters.
            // SortingList.SortingList(0): not translated, it calls a constructor that isn't translated.
            // SortingList.SortingList(1): not translated, its constructors are overloaded with the same number of parameters.
            pub fn is_blacklist(&self) -> bool {
                return !self.is_whitelist();
            }
            pub fn is_whitelist(&self) -> bool {
                return self.is_whitelist;
            }
            pub fn set_list_type(&mut self, is_whitelist: bool) {
                self.is_whitelist = is_whitelist;
            }
            pub fn set_whitelist(&mut self) {
                self.is_whitelist = true;
            }
            pub fn set_blacklist(&mut self) {
                self.is_whitelist = false;
            }
            // SortingList.addSortingObjects(1): not translated, it uses `Object`.
            // SortingList.addSortingObject(1): not translated, it uses `Object`.
            // SortingList.addSortingByBlock(1): not translated, it uses Minecraft's `Block`.
            // SortingList.addSortingByItem(1): not translated, it uses Minecraft's `Item`.
            // SortingList.addSortingByItemStack(1): not translated, it uses Minecraft's `ItemStack`.
            // SortingList.addSortingByBlockTag(1): not translated, it uses Minecraft's `TagKey`.
            // SortingList.addSortingByItemTag(1): not translated, it uses Minecraft's `TagKey`.
            // SortingList.canBeFoundInList(1): not translated, it uses `Object`.
            // SortingList.canBeFoundInList(2): not translated, it uses Minecraft's `Level`.
            // SortingList.canPass(1): not translated, it uses `Object`.
            // SortingList.canPass(2): not translated, it uses Minecraft's `Level`.
        }
    }

    /// `Tuple` (team/creative/creativecore/common/util/type/list/Tuple.java)
    pub mod tuple {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // Tuple.Tuple(2): not translated, its class isn't translated (it's generic).
        // Tuple.getKey(0): not translated, its class isn't translated (it's generic).
        // Tuple.getValue(0): not translated, its class isn't translated (it's generic).
        // Tuple.setValue(1): not translated, its class isn't translated (it's generic).
        // Tuple.hashCode(0): not translated, its class isn't translated (it's generic).
        // Tuple.is(1): not translated, its class isn't translated (it's generic).
        // Tuple.equals(1): not translated, its class isn't translated (it's generic).
        // Tuple.toString(0): not translated, its class isn't translated (it's generic).
    }

    /// `TupleList` (team/creative/creativecore/common/util/type/list/TupleList.java)
    pub mod tuple_list {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // TupleList.TupleList(0): not translated, its class isn't translated (it's generic).
        // TupleList.TupleList(1): not translated, its class isn't translated (it's generic).
        // TupleList.TupleList(1): not translated, its class isn't translated (it's generic).
        // TupleList.add(2): not translated, its class isn't translated (it's generic).
        // TupleList.containsKey(1): not translated, its class isn't translated (it's generic).
        // TupleList.indexOfKey(1): not translated, its class isn't translated (it's generic).
        // TupleList.values(0): not translated, its class isn't translated (it's generic).
        // TupleList.keys(0): not translated, its class isn't translated (it's generic).
        // TupleList.findValue(1): not translated, its class isn't translated (it's generic).
        // TupleList.findTuple(1): not translated, its class isn't translated (it's generic).
    }

    /// `HashMapDouble` (team/creative/creativecore/common/util/type/map/HashMapDouble.java)
    pub mod hash_map_double {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // HashMapDouble.HashMapDouble(0): not translated, its class isn't translated (it's generic).
        // HashMapDouble.HashMapDouble(1): not translated, its class isn't translated (it's generic).
        // HashMapDouble.scale(1): not translated, its class isn't translated (it's generic).
        // HashMapDouble.put(2): not translated, its class isn't translated (it's generic).
        // HashMapDouble.putAll(1): not translated, its class isn't translated (it's generic).
        // HashMapDouble.toString(0): not translated, its class isn't translated (it's generic).
        // HashMapDouble.toString(1): not translated, its class isn't translated (it's generic).
    }

    /// `HashMapFloat` (team/creative/creativecore/common/util/type/map/HashMapFloat.java)
    pub mod hash_map_float {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // HashMapFloat.HashMapFloat(0): not translated, its class isn't translated (it's generic).
        // HashMapFloat.HashMapFloat(1): not translated, its class isn't translated (it's generic).
        // HashMapFloat.scale(1): not translated, its class isn't translated (it's generic).
        // HashMapFloat.put(2): not translated, its class isn't translated (it's generic).
        // HashMapFloat.putAll(1): not translated, its class isn't translated (it's generic).
        // HashMapFloat.toString(0): not translated, its class isn't translated (it's generic).
        // HashMapFloat.toString(1): not translated, its class isn't translated (it's generic).
    }

    /// `HashMapInteger` (team/creative/creativecore/common/util/type/map/HashMapInteger.java)
    pub mod hash_map_integer {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // HashMapInteger.HashMapInteger(0): not translated, its class isn't translated (it's generic).
        // HashMapInteger.HashMapInteger(1): not translated, its class isn't translated (it's generic).
        // HashMapInteger.scale(1): not translated, its class isn't translated (it's generic).
        // HashMapInteger.put(2): not translated, its class isn't translated (it's generic).
        // HashMapInteger.putAll(1): not translated, its class isn't translated (it's generic).
    }

    /// `HashMapList` (team/creative/creativecore/common/util/type/map/HashMapList.java)
    pub mod hash_map_list {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // HashMapList.HashMapList(0): not translated, its class isn't translated (it's generic).
        // HashMapList.HashMapList(1): not translated, its class isn't translated (it's generic).
        // HashMapList.get(1): not translated, its class isn't translated (it's generic).
        // HashMapList.tryGet(1): not translated, its class isn't translated (it's generic).
        // HashMapList.keyOf(1): not translated, its class isn't translated (it's generic).
        // HashMapList.keySet(0): not translated, its class isn't translated (it's generic).
        // HashMapList.values(0): not translated, its class isn't translated (it's generic).
        // HashMapList.entrySet(0): not translated, its class isn't translated (it's generic).
        // HashMapList.containsKey(1): not translated, its class isn't translated (it's generic).
        // HashMapList.contains(1): not translated, its class isn't translated (it's generic).
        // HashMapList.contains(2): not translated, its class isn't translated (it's generic).
        // HashMapList.add(2): not translated, its class isn't translated (it's generic).
        // HashMapList.add(2): not translated, its class isn't translated (it's generic).
        // HashMapList.add(2): not translated, its class isn't translated (it's generic).
        // HashMapList.removeKey(1): not translated, its class isn't translated (it's generic).
        // HashMapList.removeValue(2): not translated, its class isn't translated (it's generic).
        // HashMapList.removeValue(1): not translated, its class isn't translated (it's generic).
        // HashMapList.sizeOfValues(0): not translated, its class isn't translated (it's generic).
        // HashMapList.size(0): not translated, its class isn't translated (it's generic).
        // HashMapList.clear(0): not translated, its class isn't translated (it's generic).
        // HashMapList.toString(0): not translated, its class isn't translated (it's generic).
        // HashMapList.isEmpty(0): not translated, its class isn't translated (it's generic).
        // HashMapList.getFirst(0): not translated, its class isn't translated (it's generic).
        // HashMapList.iterator(0): not translated, its body couldn't be read: an anonymous class.
    }

    /// `LinkedHashMapDouble` (team/creative/creativecore/common/util/type/map/LinkedHashMapDouble.java)
    pub mod linked_hash_map_double {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // LinkedHashMapDouble.LinkedHashMapDouble(0): not translated, its class isn't translated (it's generic).
        // LinkedHashMapDouble.LinkedHashMapDouble(1): not translated, its class isn't translated (it's generic).
        // LinkedHashMapDouble.put(2): not translated, its class isn't translated (it's generic).
        // LinkedHashMapDouble.putAll(1): not translated, its class isn't translated (it's generic).
        // LinkedHashMapDouble.toString(0): not translated, its class isn't translated (it's generic).
        // LinkedHashMapDouble.toString(1): not translated, its class isn't translated (it's generic).
    }

    /// `LinkedHashMapFloat` (team/creative/creativecore/common/util/type/map/LinkedHashMapFloat.java)
    pub mod linked_hash_map_float {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // LinkedHashMapFloat.LinkedHashMapFloat(0): not translated, its class isn't translated (it's generic).
        // LinkedHashMapFloat.LinkedHashMapFloat(1): not translated, its class isn't translated (it's generic).
        // LinkedHashMapFloat.put(2): not translated, its class isn't translated (it's generic).
        // LinkedHashMapFloat.putAll(1): not translated, its class isn't translated (it's generic).
        // LinkedHashMapFloat.toString(0): not translated, its class isn't translated (it's generic).
        // LinkedHashMapFloat.toString(1): not translated, its class isn't translated (it's generic).
    }

    /// `LinkedHashMapInteger` (team/creative/creativecore/common/util/type/map/LinkedHashMapInteger.java)
    pub mod linked_hash_map_integer {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // LinkedHashMapInteger.LinkedHashMapInteger(0): not translated, its class isn't translated (it's generic).
        // LinkedHashMapInteger.LinkedHashMapInteger(1): not translated, its class isn't translated (it's generic).
        // LinkedHashMapInteger.put(2): not translated, its class isn't translated (it's generic).
        // LinkedHashMapInteger.putAll(1): not translated, its class isn't translated (it's generic).
    }

    /// `RunnableReturn` (team/creative/creativecore/common/util/type/RunnableReturn.java)
    pub mod runnable_return {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // RunnableReturn.run(0): not translated, it has no body.
    }

    /// `CubeBitSet` (team/creative/creativecore/common/util/type/set/CubeBitSet.java)
    pub mod cube_bit_set {
        use super::*;
        // The class itself isn't translated: it implements `Iterable`. Its static parts are below.
        pub const CHUNK_SIZE: i32 = 4;
        pub const CHUNK_SIZE_SQUARE: i32 = 16;
        pub const CHUNK_BITS: i32 = 2;
        pub fn chunk_index(coord: i32) -> i32 {
            return if coord < 0 { ((((coord as f64) / 4.0f64).floor()) as i32) } else { coord / 4 };
        }
        pub fn index(x: i32, y: i32, z: i32) -> i32 {
            return ((x * 16) + (y * 4)) + z;
        }
        // CubeBitSet.load(1): not translated, its class isn't translated (it implements `Iterable`).
        // CubeBitSet.save(0): not translated, its class isn't translated (it implements `Iterable`).
        // CubeBitSet.init(3): not translated, its class isn't translated (it implements `Iterable`).
        // CubeBitSet.ensureCapacity(3): not translated, its class isn't translated (it implements `Iterable`).
        // CubeBitSet.flip(3): not translated, its class isn't translated (it implements `Iterable`).
        // CubeBitSet.set(3): not translated, its class isn't translated (it implements `Iterable`).
        // CubeBitSet.set(4): not translated, its class isn't translated (it implements `Iterable`).
        // CubeBitSet.clear(3): not translated, its class isn't translated (it implements `Iterable`).
        // CubeBitSet.clearIncludingSize(0): not translated, its class isn't translated (it implements `Iterable`).
        // CubeBitSet.clear(0): not translated, its class isn't translated (it implements `Iterable`).
        // CubeBitSet.get(3): not translated, its class isn't translated (it implements `Iterable`).
        // CubeBitSet.isEmpty(0): not translated, its class isn't translated (it implements `Iterable`).
        // CubeBitSet.count(0): not translated, its class isn't translated (it implements `Iterable`).
        // CubeBitSet.iterator(0): not translated, its body couldn't be read: an anonymous class.
        // CubeBitSet.toString(0): not translated, its class isn't translated (it implements `Iterable`).
    }

    /// `LineBitSet` (team/creative/creativecore/common/util/type/set/LineBitSet.java)
    pub mod line_bit_set {
        use super::*;
        // The class itself isn't translated: it implements `Iterable`. Its static parts are below.
        pub const CHUNK_SIZE: i32 = 64;
        pub const CHUNK_BITS: i32 = 6;
        pub fn chunk_index(coord: i32) -> i32 {
            return if coord < 0 { ((((coord as f64) / 64.0f64).floor()) as i32) } else { coord / 64 };
        }
        // LineBitSet.load(1): not translated, its class isn't translated (it implements `Iterable`).
        // LineBitSet.save(0): not translated, its class isn't translated (it implements `Iterable`).
        // LineBitSet.init(1): not translated, its class isn't translated (it implements `Iterable`).
        // LineBitSet.ensureCapacity(1): not translated, its class isn't translated (it implements `Iterable`).
        // LineBitSet.flip(1): not translated, its class isn't translated (it implements `Iterable`).
        // LineBitSet.set(1): not translated, its class isn't translated (it implements `Iterable`).
        // LineBitSet.set(2): not translated, its class isn't translated (it implements `Iterable`).
        // LineBitSet.clear(1): not translated, its class isn't translated (it implements `Iterable`).
        // LineBitSet.clearIncludingSize(0): not translated, its class isn't translated (it implements `Iterable`).
        // LineBitSet.clear(0): not translated, its class isn't translated (it implements `Iterable`).
        // LineBitSet.get(1): not translated, its class isn't translated (it implements `Iterable`).
        // LineBitSet.isEmpty(0): not translated, its class isn't translated (it implements `Iterable`).
        // LineBitSet.count(0): not translated, its class isn't translated (it implements `Iterable`).
        // LineBitSet.iterator(0): not translated, its body couldn't be read: an anonymous class.
        // LineBitSet.toString(0): not translated, its class isn't translated (it implements `Iterable`).
    }

    /// `QuadBitSet` (team/creative/creativecore/common/util/type/set/QuadBitSet.java)
    pub mod quad_bit_set {
        use super::*;
        // The class itself isn't translated: it implements `Iterable`. Its static parts are below.
        pub const CHUNK_SIZE: i32 = 8;
        pub const CHUNK_BITS: i32 = 3;
        pub fn chunk_index(coord: i32) -> i32 {
            return if coord < 0 { ((((coord as f64) / 8.0f64).floor()) as i32) } else { coord / 8 };
        }
        pub fn index(x: i32, y: i32) -> i32 {
            return (x * 8) + y;
        }
        // QuadBitSet.load(1): not translated, its class isn't translated (it implements `Iterable`).
        // QuadBitSet.save(0): not translated, its class isn't translated (it implements `Iterable`).
        // QuadBitSet.init(2): not translated, its class isn't translated (it implements `Iterable`).
        // QuadBitSet.ensureCapacity(2): not translated, its class isn't translated (it implements `Iterable`).
        // QuadBitSet.flip(2): not translated, its class isn't translated (it implements `Iterable`).
        // QuadBitSet.set(2): not translated, its class isn't translated (it implements `Iterable`).
        // QuadBitSet.set(3): not translated, its class isn't translated (it implements `Iterable`).
        // QuadBitSet.clear(2): not translated, its class isn't translated (it implements `Iterable`).
        // QuadBitSet.clearIncludingSize(0): not translated, its class isn't translated (it implements `Iterable`).
        // QuadBitSet.clear(0): not translated, its class isn't translated (it implements `Iterable`).
        // QuadBitSet.get(2): not translated, its class isn't translated (it implements `Iterable`).
        // QuadBitSet.isEmpty(0): not translated, its class isn't translated (it implements `Iterable`).
        // QuadBitSet.count(0): not translated, its class isn't translated (it implements `Iterable`).
        // QuadBitSet.iterator(0): not translated, its body couldn't be read: an anonymous class.
        // QuadBitSet.toString(0): not translated, its class isn't translated (it implements `Iterable`).
    }

    /// `CheckTree` (team/creative/creativecore/common/util/type/tree/CheckTree.java)
    pub mod check_tree {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // CheckTree.CheckTree(4): not translated, its class isn't translated (it's generic).
        // CheckTree.CheckTree(4): not translated, its class isn't translated (it's generic).
        // CheckTree.reload(0): not translated, its class isn't translated (it's generic).
        // CheckTree.reload(1): not translated, its class isn't translated (it's generic).
        // CheckTree.apply(0): not translated, its class isn't translated (it's generic).
        // CheckTree.apply(1): not translated, its class isn't translated (it's generic).
    }

    /// `CheckTree.CheckTreeEntry` (team/creative/creativecore/common/util/type/tree/CheckTree.java)
    pub mod check_tree_check_tree_entry {
        use super::*;
        // The class itself isn't translated: it holds an object of its own class (a linked structure). Its static parts are below.
        // CheckTree.CheckTreeEntry.CheckTreeEntry(1): not translated, its body couldn't be read: an inner class creation.
        // CheckTree.CheckTreeEntry.CheckTreeEntry(2): not translated, its body couldn't be read: an inner class creation.
        // CheckTree.CheckTreeEntry.isEnabled(0): not translated, its class isn't translated (it holds an object of its own class (a linked structure)).
        // CheckTree.CheckTreeEntry.isChildEnabled(0): not translated, its class isn't translated (it holds an object of its own class (a linked structure)).
        // CheckTree.CheckTreeEntry.enable(0): not translated, its class isn't translated (it holds an object of its own class (a linked structure)).
        // CheckTree.CheckTreeEntry.disable(0): not translated, its class isn't translated (it holds an object of its own class (a linked structure)).
        // CheckTree.CheckTreeEntry.enableInteral(0): not translated, its class isn't translated (it holds an object of its own class (a linked structure)).
        // CheckTree.CheckTreeEntry.disableInteral(0): not translated, its class isn't translated (it holds an object of its own class (a linked structure)).
        // CheckTree.CheckTreeEntry.toggle(0): not translated, its class isn't translated (it holds an object of its own class (a linked structure)).
    }

    /// `NamedTree` (team/creative/creativecore/common/util/type/tree/NamedTree.java)
    pub mod named_tree {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // NamedTree.NamedTree(0): not translated, its class isn't translated (it's generic).
        // NamedTree.NamedTree(2): not translated, its class isn't translated (it's generic).
        // NamedTree.name(0): not translated, its class isn't translated (it's generic).
        // NamedTree.parent(0): not translated, its class isn't translated (it's generic).
        // NamedTree.add(2): not translated, its class isn't translated (it's generic).
        // NamedTree.children(0): not translated, its class isn't translated (it's generic).
        // NamedTree.entries(0): not translated, its class isn't translated (it's generic).
        // NamedTree.keySet(0): not translated, its class isn't translated (it's generic).
        // NamedTree.get(1): not translated, its class isn't translated (it's generic).
        // NamedTree.folder(1): not translated, its class isn't translated (it's generic).
        // NamedTree.folder(2): not translated, its class isn't translated (it's generic).
        // NamedTree.folderForce(1): not translated, its class isn't translated (it's generic).
        // NamedTree.folderForce(2): not translated, its class isn't translated (it's generic).
        // NamedTree.folderForce(3): not translated, its class isn't translated (it's generic).
        // NamedTree.path(0): not translated, its class isn't translated (it's generic).
        // NamedTree.findPath(1): not translated, its class isn't translated (it's generic).
        // NamedTree.toString(0): not translated, its class isn't translated (it's generic).
        // NamedTree.allValues(0): not translated, its class isn't translated (it's generic).
        // NamedTree.allNames(0): not translated, its class isn't translated (it's generic).
        // NamedTree.collect(1): not translated, its class isn't translated (it's generic).
        // NamedTree.all(0): not translated, its class isn't translated (it's generic).
        // NamedTree.first(0): not translated, its class isn't translated (it's generic).
        // NamedTree.firstKey(0): not translated, its class isn't translated (it's generic).
        // NamedTree.hasChildren(0): not translated, its class isn't translated (it's generic).
    }

    /// `TriPredicate` (team/creative/creativecore/common/util/type/TriPredicate.java)
    pub mod tri_predicate {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // TriPredicate.test(3): not translated, it has no body.
    }

    /// `CreativeHackery` (team/creative/creativecore/common/util/unsafe/CreativeHackery.java)
    pub mod creative_hackery {
        use super::*;
        // UNSAFE: not translated, it uses `Unsafe` (sun.misc).
        #[derive(Clone, Debug)]
        pub struct CreativeHackery {
        }
        impl CreativeHackery {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // CreativeHackery.getUnsafe(0): not translated, its body couldn't be read: an anonymous class.
        // CreativeHackery.allocateInstance(1): not translated, it's generic.
    }

    /// `CreativeCore` (team/creative/creativecore/CreativeCore.java)
    pub mod creative_core {
        use super::*;
        // The class itself isn't translated: it implements Fabric's `ModInitializer`. Its static parts are below.
        // LOADER: not translated, it uses ICreativeLoader, which isn't translated (it's an interface).
        // UTILS: not translated, it uses ILoaderUtils, which isn't translated (it's an interface).
        pub const MODID: &str = "creativecore";
        // LOGGER: the logger. Log calls become comments.
        // NETWORK: not translated, it uses CreativeNetwork, which isn't translated (it uses `Class`).
        // CONFIG: not translated, it uses CreativeCoreConfig, which isn't translated (it uses NamedList, which isn't translated (it's generic)).
        // CONFIG_OPEN: not translated, it uses GuiCreatorBasic, which isn't translated (it extends `GuiCreator`).
        // CONFIG_CLIENT_OPEN: not translated, it uses GuiCreatorBasic, which isn't translated (it extends `GuiCreator`).
        // CONFIG_CLIENT_SYNC_OPEN: not translated, it uses GuiCreatorBasic, which isn't translated (it extends `GuiCreator`).
        // CONFIG_HANDLER: not translated, it uses ConfigEventHandler, which isn't translated (it uses `DecimalFormat` (java.text)).
        // GUI_CONTAINER: not translated, it uses Minecraft's `MenuType`.
        // CreativeCore.CreativeCore(0): not translated, its class isn't translated (it implements Fabric's `ModInitializer`).
        // CreativeCore.server(1): not translated, its class isn't translated (it implements Fabric's `ModInitializer`).
        // CreativeCore.onInitialize(0): not translated, its class isn't translated (it implements Fabric's `ModInitializer`).
        // CreativeCore.loader(0): not translated, it uses ICreativeLoader, which isn't translated (it's an interface).
        // CreativeCore.utils(0): not translated, it uses ILoaderUtils, which isn't translated (it's an interface).
    }

    /// `CreativeCoreConfig` (team/creative/creativecore/CreativeCoreConfig.java)
    pub mod creative_core_config {
        use super::*;
        // The class itself isn't translated: it uses NamedList, which isn't translated (it's generic). Its static parts are below.
        // CreativeCoreConfig.CreativeCoreConfig(0): not translated, its class isn't translated (it uses NamedList, which isn't translated (it's generic)).
        // CreativeCoreConfig.is(2): not translated, its class isn't translated (it uses NamedList, which isn't translated (it's generic)).
    }

    /// `CreativeCoreGuiRegistry` (team/creative/creativecore/CreativeCoreGuiRegistry.java)
    pub mod creative_core_gui_registry {
        use super::*;
        // HAND: not translated, it uses GuiSyncGlobal, which isn't translated (it's generic).
        // DROP: not translated, it uses GuiSyncGlobal, which isn't translated (it's generic).
        // DROP_HAND: not translated, it uses GuiSyncGlobal, which isn't translated (it's generic).
        // INSERT: not translated, it uses GuiSyncGlobal, which isn't translated (it's generic).
        // EXTRACT: not translated, it uses GuiSyncGlobal, which isn't translated (it's generic).
        // DUPLICATE: not translated, it uses GuiSyncGlobal, which isn't translated (it's generic).
        // SWAP: not translated, it uses GuiSyncGlobal, which isn't translated (it's generic).
        // SPREAD: not translated, it uses GuiSyncGlobal, which isn't translated (it's generic).
        #[derive(Clone, Debug)]
        pub struct CreativeCoreGuiRegistry {
        }
        impl CreativeCoreGuiRegistry {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn init() {
        }
    }

    /// `CreativeFabricLoader` (team/creative/creativecore/CreativeFabricLoader.java)
    pub mod creative_fabric_loader {
        use super::*;
        // The class itself isn't translated: it implements `ICreativeLoader`. Its static parts are below.
        // currentServer: not translated, it uses Minecraft's `MinecraftServer`.
        // FINISH_CONSUMING: not translated, it uses Fabric's `Event`.
        // CreativeFabricLoader.register(1): not translated, its class isn't translated (it implements `ICreativeLoader`).
        // CreativeFabricLoader.registerClient(1): not translated, its class isn't translated (it implements `ICreativeLoader`).
        // CreativeFabricLoader.registerClientTick(1): not translated, its class isn't translated (it implements `ICreativeLoader`).
        // CreativeFabricLoader.registerHudElement(1): not translated, its class isn't translated (it implements `ICreativeLoader`).
        // CreativeFabricLoader.registerModifyCreativeTab(2): not translated, its class isn't translated (it implements `ICreativeLoader`).
        // CreativeFabricLoader.registerClientRenderGui(1): not translated, its class isn't translated (it implements `ICreativeLoader`).
        // CreativeFabricLoader.registerClientRenderStart(1): not translated, its class isn't translated (it implements `ICreativeLoader`).
        // CreativeFabricLoader.registerReloadListener(2): not translated, its class isn't translated (it implements `ICreativeLoader`).
        // CreativeFabricLoader.registerLevelTick(1): not translated, its class isn't translated (it implements `ICreativeLoader`).
        // CreativeFabricLoader.registerLevelTickStart(1): not translated, its class isn't translated (it implements `ICreativeLoader`).
        // CreativeFabricLoader.registerLoadLevel(1): not translated, its class isn't translated (it implements `ICreativeLoader`).
        // CreativeFabricLoader.registerUnloadLevel(1): not translated, its class isn't translated (it implements `ICreativeLoader`).
        // CreativeFabricLoader.registerListener(1): not translated, it's generic.
        // CreativeFabricLoader.getFluidViscosityMultiplier(2): not translated, its class isn't translated (it implements `ICreativeLoader`).
        // CreativeFabricLoader.getFriction(3): not translated, its class isn't translated (it implements `ICreativeLoader`).
        // CreativeFabricLoader.registerClientStarted(1): not translated, its class isn't translated (it implements `ICreativeLoader`).
        // CreativeFabricLoader.postForge(1): not translated, its class isn't translated (it implements `ICreativeLoader`).
        // CreativeFabricLoader.isModLoaded(1): not translated, its class isn't translated (it implements `ICreativeLoader`).
        // CreativeFabricLoader.getOverallSide(0): not translated, its class isn't translated (it implements `ICreativeLoader`).
        // CreativeFabricLoader.getEffectiveSide(0): not translated, its class isn't translated (it implements `ICreativeLoader`).
        // CreativeFabricLoader.forge(0): not translated, its class isn't translated (it implements `ICreativeLoader`).
        // CreativeFabricLoader.fabric(0): not translated, its class isn't translated (it implements `ICreativeLoader`).
        // CreativeFabricLoader.getCurrentServer(0): not translated, its class isn't translated (it implements `ICreativeLoader`).
        // CreativeFabricLoader.registerPlayerJoin(1): not translated, its class isn't translated (it implements `ICreativeLoader`).
        // CreativeFabricLoader.registerPlayerCopy(1): not translated, its class isn't translated (it implements `ICreativeLoader`).
        // CreativeFabricLoader.registerPlayerDimensionChange(1): not translated, its class isn't translated (it implements `ICreativeLoader`).
        // CreativeFabricLoader.registerPlayerRespawn(1): not translated, its class isn't translated (it implements `ICreativeLoader`).
        // CreativeFabricLoader.registerItemUsed(1): not translated, its class isn't translated (it implements `ICreativeLoader`).
        // CreativeFabricLoader.publishItemUsed(2): not translated, its class isn't translated (it implements `ICreativeLoader`).
        // CreativeFabricLoader.registerAttachment(3): not translated, its body couldn't be read: an anonymous class.
        // CreativeFabricLoader.registerRemoveEffectCallback(1): not translated, its class isn't translated (it implements `ICreativeLoader`).
        // CreativeFabricLoader.openTransaction(1): not translated, its class isn't translated (it implements `ICreativeLoader`).
        // CreativeFabricLoader.registerItemStorage(2): not translated, its class isn't translated (it implements `ICreativeLoader`).
        // CreativeFabricLoader.getItemStorage(2): not translated, its class isn't translated (it implements `ICreativeLoader`).
        // CreativeFabricLoader.getItemStorage(3): not translated, its class isn't translated (it implements `ICreativeLoader`).
        // CreativeFabricLoader.register(1): not translated, its class isn't translated (it implements `ICreativeLoader`).
        // CreativeFabricLoader.onItemToss(2): not translated, its class isn't translated (it implements `ICreativeLoader`).
    }

    /// `CreativeFabricLoader.CreativeHudElement` (team/creative/creativecore/CreativeFabricLoader.java)
    pub mod creative_fabric_loader_creative_hud_element {
        use super::*;
        // The class itself isn't translated: it implements Fabric's `HudElement`. Its static parts are below.
        // CreativeFabricLoader.CreativeHudElement.extractRenderState(2): not translated, its class isn't translated (it implements Fabric's `HudElement`).
    }

    /// `CreativeFabricLoader.FabricItemResource` (team/creative/creativecore/CreativeFabricLoader.java)
    pub mod creative_fabric_loader_fabric_item_resource {
        use super::*;
        // The class itself isn't translated: it implements `CommonItemResource`. Its static parts are below.
        // CreativeFabricLoader.FabricItemResource.FabricItemResource(1): not translated, its class isn't translated (it implements `CommonItemResource`).
        // CreativeFabricLoader.FabricItemResource.isEmpty(0): not translated, its class isn't translated (it implements `CommonItemResource`).
        // CreativeFabricLoader.FabricItemResource.getMaxStackSize(0): not translated, its class isn't translated (it implements `CommonItemResource`).
        // CreativeFabricLoader.FabricItemResource.getItem(0): not translated, its class isn't translated (it implements `CommonItemResource`).
        // CreativeFabricLoader.FabricItemResource.toStack(1): not translated, its class isn't translated (it implements `CommonItemResource`).
        // CreativeFabricLoader.FabricItemResource.toStack(0): not translated, its class isn't translated (it implements `CommonItemResource`).
        // CreativeFabricLoader.FabricItemResource.typeHolder(0): not translated, its class isn't translated (it implements `CommonItemResource`).
        // CreativeFabricLoader.FabricItemResource.getComponents(0): not translated, its class isn't translated (it implements `CommonItemResource`).
    }

    /// `CreativeFabricLoader.FabricItemStorage` (team/creative/creativecore/CreativeFabricLoader.java)
    pub mod creative_fabric_loader_fabric_item_storage {
        use super::*;
        // The class itself isn't translated: it implements `CommonItemStorage`. Its static parts are below.
        // CreativeFabricLoader.FabricItemStorage.FabricItemStorage(1): not translated, its class isn't translated (it implements `CommonItemStorage`).
        // CreativeFabricLoader.FabricItemStorage.insert(3): not translated, its class isn't translated (it implements `CommonItemStorage`).
        // CreativeFabricLoader.FabricItemStorage.extract(3): not translated, its class isn't translated (it implements `CommonItemStorage`).
        // CreativeFabricLoader.FabricItemStorage.iterator(0): not translated, its body couldn't be read: an anonymous class.
    }

    /// `CreativeFabricLoader.FabricItemStorageView` (team/creative/creativecore/CreativeFabricLoader.java)
    pub mod creative_fabric_loader_fabric_item_storage_view {
        use super::*;
        // The class itself isn't translated: it implements `CommonItemStorageView`. Its static parts are below.
        // CreativeFabricLoader.FabricItemStorageView.FabricItemStorageView(2): not translated, its class isn't translated (it implements `CommonItemStorageView`).
        // CreativeFabricLoader.FabricItemStorageView.getCapacityAsInt(0): not translated, its class isn't translated (it implements `CommonItemStorageView`).
        // CreativeFabricLoader.FabricItemStorageView.getResource(0): not translated, its class isn't translated (it implements `CommonItemStorageView`).
        // CreativeFabricLoader.FabricItemStorageView.getAmountAsInt(0): not translated, its class isn't translated (it implements `CommonItemStorageView`).
        // CreativeFabricLoader.FabricItemStorageView.insert(3): not translated, its class isn't translated (it implements `CommonItemStorageView`).
        // CreativeFabricLoader.FabricItemStorageView.extract(3): not translated, its class isn't translated (it implements `CommonItemStorageView`).
    }

    /// `CreativeFabricLoader.FabricTransaction` (team/creative/creativecore/CreativeFabricLoader.java)
    pub mod creative_fabric_loader_fabric_transaction {
        use super::*;
        // The class itself isn't translated: it implements `CommonTransaction`. Its static parts are below.
        // CreativeFabricLoader.FabricTransaction.FabricTransaction(1): not translated, its class isn't translated (it implements `CommonTransaction`).
        // CreativeFabricLoader.FabricTransaction.commit(0): not translated, its class isn't translated (it implements `CommonTransaction`).
        // CreativeFabricLoader.FabricTransaction.close(0): not translated, its class isn't translated (it implements `CommonTransaction`).
    }

    /// `ICreativeAttachmentType` (team/creative/creativecore/ICreativeAttachmentType.java)
    pub mod i_creative_attachment_type {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ICreativeAttachmentType.get(1): not translated, it has no body.
        // ICreativeAttachmentType.set(2): not translated, it has no body.
    }

    /// `ICreativeLoader` (team/creative/creativecore/ICreativeLoader.java)
    pub mod i_creative_loader {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ICreativeLoader.loadCommon(0): not translated, its class isn't translated (it's an interface).
        // ICreativeLoader.getCurrentServer(0): not translated, it has no body.
        // ICreativeLoader.forge(0): not translated, it has no body.
        // ICreativeLoader.fabric(0): not translated, it has no body.
        // ICreativeLoader.getOverallSide(0): not translated, it has no body.
        // ICreativeLoader.getEffectiveSide(0): not translated, it has no body.
        // ICreativeLoader.register(1): not translated, it has no body.
        // ICreativeLoader.registerClient(1): not translated, it has no body.
        // ICreativeLoader.registerClientTick(1): not translated, it has no body.
        // ICreativeLoader.registerClientRenderStart(1): not translated, it has no body.
        // ICreativeLoader.registerClientRenderGui(1): not translated, it has no body.
        // ICreativeLoader.registerClientStarted(1): not translated, it has no body.
        // ICreativeLoader.registerReloadListener(2): not translated, it has no body.
        // ICreativeLoader.registerLevelTick(1): not translated, it has no body.
        // ICreativeLoader.registerLevelTickStart(1): not translated, it has no body.
        // ICreativeLoader.registerLoadLevel(1): not translated, it has no body.
        // ICreativeLoader.registerUnloadLevel(1): not translated, it has no body.
        // ICreativeLoader.registerModifyCreativeTab(2): not translated, it has no body.
        // ICreativeLoader.registerListener(1): not translated, it has no body.
        // ICreativeLoader.getFluidViscosityMultiplier(2): not translated, it has no body.
        // ICreativeLoader.getFriction(3): not translated, it has no body.
        // ICreativeLoader.postForge(1): not translated, it has no body.
        // ICreativeLoader.registerPlayerJoin(1): not translated, it has no body.
        // ICreativeLoader.registerPlayerCopy(1): not translated, it has no body.
        // ICreativeLoader.registerAttachment(3): not translated, it has no body.
        // ICreativeLoader.openTransaction(1): not translated, it has no body.
        // ICreativeLoader.registerItemStorage(2): not translated, it has no body.
        // ICreativeLoader.getItemStorage(2): not translated, it has no body.
        // ICreativeLoader.getItemStorage(3): not translated, it has no body.
        // ICreativeLoader.registerRemoveEffectCallback(1): not translated, it has no body.
        // ICreativeLoader.publishItemUsed(2): not translated, it has no body.
        // ICreativeLoader.registerItemUsed(1): not translated, it has no body.
        // ICreativeLoader.registerPlayerRespawn(1): not translated, it has no body.
        // ICreativeLoader.registerPlayerDimensionChange(1): not translated, it has no body.
        // ICreativeLoader.onItemToss(2): not translated, it has no body.
        // ICreativeLoader.register(1): not translated, it has no body.
        // ICreativeLoader.isModLoaded(1): not translated, it has no body.
    }

    /// `ICreativeLoader.CommonItemResource` (team/creative/creativecore/ICreativeLoader.java)
    pub mod i_creative_loader_common_item_resource {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ICreativeLoader.CommonItemResource.isEmpty(0): not translated, it has no body.
        // ICreativeLoader.CommonItemResource.getItem(0): not translated, it has no body.
        // ICreativeLoader.CommonItemResource.getMaxStackSize(0): not translated, it has no body.
        // ICreativeLoader.CommonItemResource.toStack(1): not translated, it has no body.
        // ICreativeLoader.CommonItemResource.toStack(0): not translated, it has no body.
    }

    /// `ICreativeLoader.CommonItemStorage` (team/creative/creativecore/ICreativeLoader.java)
    pub mod i_creative_loader_common_item_storage {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ICreativeLoader.CommonItemStorage.insert(3): not translated, it has no body.
        // ICreativeLoader.CommonItemStorage.extract(3): not translated, it has no body.
    }

    /// `ICreativeLoader.CommonItemStorageView` (team/creative/creativecore/ICreativeLoader.java)
    pub mod i_creative_loader_common_item_storage_view {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ICreativeLoader.CommonItemStorageView.getCapacityAsInt(0): not translated, it has no body.
        // ICreativeLoader.CommonItemStorageView.getResource(0): not translated, it has no body.
        // ICreativeLoader.CommonItemStorageView.getAmountAsInt(0): not translated, it has no body.
        // ICreativeLoader.CommonItemStorageView.insert(3): not translated, it has no body.
        // ICreativeLoader.CommonItemStorageView.extract(3): not translated, it has no body.
    }

    /// `ICreativeLoader.CommonTransaction` (team/creative/creativecore/ICreativeLoader.java)
    pub mod i_creative_loader_common_transaction {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ICreativeLoader.CommonTransaction.commit(0): not translated, it has no body.
        // ICreativeLoader.CommonTransaction.close(0): not translated, it has no body.
    }

    /// `ICreativeLoader.CopyConsumer` (team/creative/creativecore/ICreativeLoader.java)
    pub mod i_creative_loader_copy_consumer {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ICreativeLoader.CopyConsumer.accept(3): not translated, it has no body.
    }

    /// `ICreativeLoader.FinishConsuming` (team/creative/creativecore/ICreativeLoader.java)
    pub mod i_creative_loader_finish_consuming {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ICreativeLoader.FinishConsuming.accept(2): not translated, it has no body.
    }

    /// `ICreativeLoader.PlayerDimensionChangeConsumer` (team/creative/creativecore/ICreativeLoader.java)
    pub mod i_creative_loader_player_dimension_change_consumer {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ICreativeLoader.PlayerDimensionChangeConsumer.accept(3): not translated, it has no body.
    }

    /// `ICreativeLoader.RemoveEffect` (team/creative/creativecore/ICreativeLoader.java)
    pub mod i_creative_loader_remove_effect {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ICreativeLoader.RemoveEffect.shouldCancel(2): not translated, it has no body.
    }

    /// `ReflectionHelper` (team/creative/creativecore/reflection/ReflectionHelper.java)
    pub mod reflection_helper {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ReflectionHelper {
        }
        impl ReflectionHelper {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ReflectionHelper.findFieldByType(2): not translated, it's generic.
        // ReflectionHelper.findFieldByType(2): not translated, it's generic.
        // ReflectionHelper.findField(3): not translated, it's generic.
        // ReflectionHelper.findField(2): not translated, it's generic.
        // ReflectionHelper.findMethod(3): not translated, it uses `Method` (java.lang.reflect).
    }

    /// `GuiServerControl` (team/creative/creativecore/server/gui/GuiServerControl.java)
    pub mod gui_server_control {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GuiServerControl.GuiServerControl(1): not translated, its class isn't translated (it's generic).
        // GuiServerControl.setVisible(1): not translated, its class isn't translated (it's generic).
        // GuiServerControl.setFixed(0): not translated, its class isn't translated (it's generic).
        // GuiServerControl.setFixedX(0): not translated, its class isn't translated (it's generic).
        // GuiServerControl.setFixedY(0): not translated, its class isn't translated (it's generic).
        // GuiServerControl.setExpandable(0): not translated, its class isn't translated (it's generic).
        // GuiServerControl.setExpandableX(0): not translated, its class isn't translated (it's generic).
        // GuiServerControl.setExpandableY(0): not translated, its class isn't translated (it's generic).
        // GuiServerControl.setDim(2): not translated, its class isn't translated (it's generic).
        // GuiServerControl.setDim(1): not translated, its class isn't translated (it's generic).
        // GuiServerControl.setEnabled(1): not translated, its class isn't translated (it's generic).
        // GuiServerControl.setTooltip(1): not translated, its class isn't translated (it's generic).
        // GuiServerControl.setTooltip(1): not translated, its class isn't translated (it's generic).
        // GuiServerControl.isExpandableX(0): not translated, its class isn't translated (it's generic).
        // GuiServerControl.isExpandableY(0): not translated, its class isn't translated (it's generic).
        // GuiServerControl.removeFormatting(0): not translated, its class isn't translated (it's generic).
        // GuiServerControl.setFormatting(1): not translated, its class isn't translated (it's generic).
        // GuiServerControl.setCustomFormatting(0): not translated, its class isn't translated (it's generic).
        // GuiServerControl.playSound(1): not translated, its class isn't translated (it's generic).
        // GuiServerControl.playSound(1): not translated, its class isn't translated (it's generic).
        // GuiServerControl.playSound(3): not translated, its class isn't translated (it's generic).
        // GuiServerControl.playSound(3): not translated, its class isn't translated (it's generic).
    }

    /// `GuiServerManager` (team/creative/creativecore/server/gui/manager/GuiServerManager.java)
    pub mod gui_server_manager {
        use super::*;
        // The class itself isn't translated: it implements `GuiManagerDist` (team.creative.creativecore.common.gui.manager). Its static parts are below.
    }

    /// `GuiServerRegistry` (team/creative/creativecore/server/gui/registry/GuiServerRegistry.java)
    pub mod gui_server_registry {
        use super::*;
        // CONTROL_FACTORY: not translated, it uses `Object2ObjectMap` (it.unimi.dsi.fastutil.objects).
        // CONTROL_SPECIAL_FACTORY: not translated, it uses GuiControl, which isn't translated (it uses IGuiParent, which isn't translated (it's an interface)).
        // MANAGER_FACTORY: not translated, it uses `Object2ObjectMap` (it.unimi.dsi.fastutil.objects).
        #[derive(Clone, Debug)]
        pub struct GuiServerRegistry {
        }
        impl GuiServerRegistry {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // GuiServerRegistry.create(1): not translated, it uses GuiServerControl, which isn't translated (it's generic).
        // GuiServerRegistry.create(1): not translated, it uses GuiServerControl, which isn't translated (it's generic).
        // GuiServerRegistry.register(2): not translated, it's generic.
        // GuiServerRegistry.registerManager(2): not translated, it's generic.
        // GuiServerRegistry.registerSpecial(1): not translated, it uses GuiControl, which isn't translated (it uses IGuiParent, which isn't translated (it's an interface)).
    }

    /// `Side` (team/creative/creativecore/Side.java)
    pub mod side {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // Side.isClient(0): not translated, it has no body.
        // Side.isServer(0): not translated, it has no body.
    }

    
    // ---- Gson (com.google.gson): JSON values, a reader and Gson's writer ----
    // JsonElement, JsonObject, JsonArray and JsonPrimitive are all one Rust type. Numbers keep their text, as
    // Gson's LazilyParsedNumber does. Objects keep their insertion order, as Gson's LinkedTreeMap does.
    
    #[derive(Clone, Debug, PartialEq)]
    pub enum JJson {
        Null,
        Bool(bool),
        Num(String),
        Str(String),
        Arr(Vec<JJson>),
        Obj(Vec<(String, JJson)>),
    }
    
    /// A float or double as Java prints it (Float.toString / Double.toString).
    pub fn j_java_num(x: f64, float: bool) -> String {
        if x.is_nan() {
            return "NaN".into();
        }
        if x.is_infinite() {
            return if x > 0.0 { "Infinity".into() } else { "-Infinity".into() };
        }
        let a = x.abs();
        if x == 0.0 {
            return if x.is_sign_negative() { "-0.0".into() } else { "0.0".into() };
        }
        let digits = if float { format!("{:e}", x as f32) } else { format!("{:e}", x) };
        let (mant, exp) = digits.split_once('e').unwrap();
        let exp: i32 = exp.parse().unwrap();
        if (1e-3..1e7).contains(&a) {
            let plain = if float { format!("{}", x as f32) } else { format!("{}", x) };
            if plain.contains('.') { plain } else { plain + ".0" }
        } else {
            let mant = if mant.contains('.') { mant.to_string() } else { format!("{mant}.0") };
            format!("{mant}E{exp}")
        }
    }
    
    struct JJsonReader<'a> {
        s: &'a [char],
        i: usize,
    }
    
    impl JJsonReader<'_> {
        fn ws(&mut self) {
            while self.i < self.s.len() && self.s[self.i].is_whitespace() {
                self.i += 1;
            }
        }
        fn fail(&self, what: &str) -> ! {
            panic!("JsonSyntaxException: {what} at {}", self.i)
        }
        fn value(&mut self) -> JJson {
            self.ws();
            match self.s.get(self.i) {
                Some('{') => {
                    self.i += 1;
                    let mut members = Vec::new();
                    self.ws();
                    if self.s.get(self.i) == Some(&'}') {
                        self.i += 1;
                        return JJson::Obj(members);
                    }
                    loop {
                        self.ws();
                        let k = match self.s.get(self.i) {
                            Some('"') | Some('\'') => self.string(),
                            _ => self.bare(),
                        };
                        self.ws();
                        if !matches!(self.s.get(self.i), Some(':') | Some('=')) {
                            self.fail("expected ':'");
                        }
                        self.i += 1;
                        let v = self.value();
                        // A repeated name: Gson keeps the last one.
                        members.retain(|(n, _): &(String, JJson)| *n != k);
                        members.push((k, v));
                        self.ws();
                        match self.s.get(self.i) {
                            Some(',') | Some(';') => self.i += 1,
                            Some('}') => {
                                self.i += 1;
                                return JJson::Obj(members);
                            }
                            _ => self.fail("expected ',' or '}'"),
                        }
                    }
                }
                Some('[') => {
                    self.i += 1;
                    let mut items = Vec::new();
                    self.ws();
                    if self.s.get(self.i) == Some(&']') {
                        self.i += 1;
                        return JJson::Arr(items);
                    }
                    loop {
                        items.push(self.value());
                        self.ws();
                        match self.s.get(self.i) {
                            Some(',') | Some(';') => self.i += 1,
                            Some(']') => {
                                self.i += 1;
                                return JJson::Arr(items);
                            }
                            _ => self.fail("expected ',' or ']'"),
                        }
                    }
                }
                Some('"') | Some('\'') => JJson::Str(self.string()),
                Some(_) => {
                    let word = self.bare();
                    match word.as_str() {
                        "true" => JJson::Bool(true),
                        "false" => JJson::Bool(false),
                        "null" => JJson::Null,
                        w if w.parse::<f64>().is_ok() => JJson::Num(w.to_string()),
                        // Gson is lenient: an unquoted word is a string.
                        w => JJson::Str(w.to_string()),
                    }
                }
                None => self.fail("end of input"),
            }
        }
        fn bare(&mut self) -> String {
            let start = self.i;
            while self.i < self.s.len() && !matches!(self.s[self.i], ',' | ':' | '=' | ';' | '}' | ']' | '{' | '[' | '"' | '\'') && !self.s[self.i].is_whitespace() {
                self.i += 1;
            }
            if start == self.i {
                self.fail("expected a value");
            }
            self.s[start..self.i].iter().collect()
        }
        fn string(&mut self) -> String {
            let q = self.s[self.i];
            self.i += 1;
            let mut out = String::new();
            loop {
                let Some(&c) = self.s.get(self.i) else { self.fail("unterminated string") };
                self.i += 1;
                match c {
                    c if c == q => return out,
                    '\\' => {
                        let Some(&e) = self.s.get(self.i) else { self.fail("unterminated string") };
                        self.i += 1;
                        match e {
                            'n' => out.push('\n'),
                            't' => out.push('\t'),
                            'r' => out.push('\r'),
                            'b' => out.push('\u{8}'),
                            'f' => out.push('\u{c}'),
                            'u' => {
                                let hex: String = self.s[self.i..(self.i + 4).min(self.s.len())].iter().collect();
                                self.i += 4;
                                let unit = u16::from_str_radix(&hex, 16).unwrap_or_else(|_| self.fail("bad \\u escape"));
                                // A surrogate pair: two escapes make one char.
                                if (0xD800..0xDC00).contains(&unit) && self.s.get(self.i) == Some(&'\\') && self.s.get(self.i + 1) == Some(&'u') {
                                    let hex2: String = self.s[self.i + 2..(self.i + 6).min(self.s.len())].iter().collect();
                                    if let Ok(low) = u16::from_str_radix(&hex2, 16) {
                                        self.i += 6;
                                        out.extend(char::decode_utf16([unit, low]).map(|r| r.unwrap_or('\u{FFFD}')));
                                        continue;
                                    }
                                }
                                out.push(char::from_u32(unit as u32).unwrap_or('\u{FFFD}'));
                            }
                            other => out.push(other),
                        }
                    }
                    c => out.push(c),
                }
            }
        }
    }
    
    impl JJson {
        /// JsonParser.parseString: panics (Java throws JsonSyntaxException) on bad JSON.
        pub fn parse(s: &str) -> JJson {
            let chars: Vec<char> = s.chars().collect();
            let mut r = JJsonReader { s: &chars, i: 0 };
            let v = r.value();
            r.ws();
            if r.i != chars.len() {
                r.fail("more after the value");
            }
            v
        }
        pub fn object() -> JJson {
            JJson::Obj(Vec::new())
        }
        pub fn array() -> JJson {
            JJson::Arr(Vec::new())
        }
        pub fn int(v: i64) -> JJson {
            JJson::Num(v.to_string())
        }
        pub fn float(v: f32) -> JJson {
            JJson::Num(j_java_num(v as f64, true))
        }
        pub fn double(v: f64) -> JJson {
            JJson::Num(j_java_num(v, false))
        }
        pub fn str(v: &str) -> JJson {
            JJson::Str(v.to_string())
        }
        pub fn is_json_object(&self) -> bool {
            matches!(self, JJson::Obj(_))
        }
        pub fn is_json_array(&self) -> bool {
            matches!(self, JJson::Arr(_))
        }
        pub fn is_json_null(&self) -> bool {
            matches!(self, JJson::Null)
        }
        pub fn is_json_primitive(&self) -> bool {
            matches!(self, JJson::Bool(_) | JJson::Num(_) | JJson::Str(_))
        }
        pub fn is_string(&self) -> bool {
            matches!(self, JJson::Str(_))
        }
        pub fn is_number(&self) -> bool {
            matches!(self, JJson::Num(_))
        }
        pub fn is_boolean(&self) -> bool {
            matches!(self, JJson::Bool(_))
        }
        /// getAsJsonObject() / getAsJsonArray(): Java throws IllegalStateException on the wrong kind.
        pub fn as_object(&self) -> JJson {
            assert!(self.is_json_object(), "Not a JSON Object: {self}");
            self.clone()
        }
        pub fn as_array(&self) -> JJson {
            assert!(self.is_json_array(), "Not a JSON Array: {self}");
            self.clone()
        }
        fn members(&self) -> &Vec<(String, JJson)> {
            match self {
                JJson::Obj(m) => m,
                _ => panic!("Not a JSON Object: {self}"),
            }
        }
        /// The (name, value) pairs of an object, as entrySet() walks them.
        pub fn entries(&self) -> impl Iterator<Item = (&String, &JJson)> {
            self.members().iter().map(|(k, v)| (k, v))
        }
        pub fn keys(&self) -> impl Iterator<Item = &String> {
            self.members().iter().map(|(k, _)| k)
        }
        pub fn elements(&self) -> impl Iterator<Item = &JJson> {
            match self {
                JJson::Arr(a) => a.iter(),
                _ => panic!("Not a JSON Array: {self}"),
            }
        }
        pub fn has(&self, k: &str) -> bool {
            self.members().iter().any(|(n, _)| n == k)
        }
        /// JsonObject.get: None (null) when absent.
        pub fn get(&self, k: &str) -> Option<JJson> {
            self.members().iter().find(|(n, _)| n == k).map(|(_, v)| v.clone())
        }
        /// JsonObject.getAsJsonObject(name) / getAsJsonArray(name): null when absent, a cast that throws on the wrong kind.
        pub fn member_object(&self, k: &str) -> Option<JJson> {
            self.get(k).map(|v| v.as_object())
        }
        pub fn member_array(&self, k: &str) -> Option<JJson> {
            self.get(k).map(|v| v.as_array())
        }
        /// JsonArray.get(i).
        pub fn at(&self, i: i32) -> JJson {
            match self {
                JJson::Arr(a) => a.get(i as usize).cloned().unwrap_or_else(|| panic!("Index {i} out of bounds for length {}", a.len())),
                _ => panic!("Not a JSON Array: {self}"),
            }
        }
        pub fn size(&self) -> i32 {
            match self {
                JJson::Arr(a) => a.len() as i32,
                JJson::Obj(m) => m.len() as i32,
                _ => panic!("size() of a JSON primitive"),
            }
        }
        pub fn is_empty(&self) -> bool {
            self.size() == 0
        }
        /// The single value of a one-element array, as Gson's JsonArray.getAsX does.
        fn single(&self) -> &JJson {
            match self {
                JJson::Arr(a) if a.len() == 1 => &a[0],
                JJson::Arr(_) => panic!("Array must have size 1"),
                other => other,
            }
        }
        pub fn as_string(&self) -> String {
            match self.single() {
                JJson::Str(s) => s.clone(),
                JJson::Num(n) => n.clone(),
                JJson::Bool(b) => b.to_string(),
                other => panic!("UnsupportedOperationException: {other} as a string"),
            }
        }
        fn number_text(&self) -> String {
            match self.single() {
                JJson::Num(n) | JJson::Str(n) => n.trim().to_string(),
                other => panic!("UnsupportedOperationException: {other} as a number"),
            }
        }
        pub fn as_f64(&self) -> f64 {
            let t = self.number_text();
            t.parse().unwrap_or_else(|_| panic!("NumberFormatException: {t}"))
        }
        pub fn as_float(&self) -> f32 {
            self.as_f64() as f32
        }
        pub fn as_double(&self) -> f64 {
            self.as_f64()
        }
        /// getAsInt / getAsLong: a whole number, else the number cut to one.
        pub fn as_long(&self) -> i64 {
            let t = self.number_text();
            t.parse::<i64>().unwrap_or_else(|_| t.parse::<f64>().map(|f| f as i64).unwrap_or_else(|_| panic!("NumberFormatException: {t}")))
        }
        pub fn as_int(&self) -> i32 {
            self.as_long() as i32
        }
        pub fn as_bool(&self) -> bool {
            match self.single() {
                JJson::Bool(b) => *b,
                // Gson: Boolean.parseBoolean of the text
                other => other.as_string().eq_ignore_ascii_case("true"),
            }
        }
        pub fn add(&mut self, k: &str, v: JJson) {
            match self {
                JJson::Obj(m) => {
                    m.retain(|(n, _)| n != k);
                    m.push((k.to_string(), v));
                }
                _ => panic!("Not a JSON Object: {self}"),
            }
        }
        pub fn remove(&mut self, k: &str) -> Option<JJson> {
            match self {
                JJson::Obj(m) => {
                    let i = m.iter().position(|(n, _)| n == k)?;
                    Some(m.remove(i).1)
                }
                _ => panic!("Not a JSON Object: {self}"),
            }
        }
        pub fn push(&mut self, v: JJson) {
            match self {
                JJson::Arr(a) => a.push(v),
                _ => panic!("Not a JSON Array: {self}"),
            }
        }
        fn write(&self, out: &mut String, pretty: bool, html: bool, depth: usize) {
            let nl = |out: &mut String, d: usize| {
                if pretty {
                    out.push('\n');
                    out.push_str(&"  ".repeat(d));
                }
            };
            match self {
                JJson::Null => out.push_str("null"),
                JJson::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
                JJson::Num(n) => out.push_str(n),
                JJson::Str(s) => j_json_quote(s, html, out),
                JJson::Arr(a) => {
                    out.push('[');
                    for (i, v) in a.iter().enumerate() {
                        if i > 0 {
                            out.push(',');
                        }
                        nl(out, depth + 1);
                        v.write(out, pretty, html, depth + 1);
                    }
                    if !a.is_empty() {
                        nl(out, depth);
                    }
                    out.push(']');
                }
                JJson::Obj(m) => {
                    out.push('{');
                    for (i, (k, v)) in m.iter().enumerate() {
                        if i > 0 {
                            out.push(',');
                        }
                        nl(out, depth + 1);
                        j_json_quote(k, html, out);
                        out.push(':');
                        if pretty {
                            out.push(' ');
                        }
                        v.write(out, pretty, html, depth + 1);
                    }
                    if !m.is_empty() {
                        nl(out, depth);
                    }
                    out.push('}');
                }
            }
        }
    }
    
    fn j_json_quote(s: &str, html: bool, out: &mut String) {
        out.push('"');
        for c in s.chars() {
            match c {
                '"' => out.push_str("\\\""),
                '\\' => out.push_str("\\\\"),
                '\n' => out.push_str("\\n"),
                '\r' => out.push_str("\\r"),
                '\t' => out.push_str("\\t"),
                '\u{8}' => out.push_str("\\b"),
                '\u{c}' => out.push_str("\\f"),
                '<' | '>' | '&' | '=' | '\'' if html => out.push_str(&format!("\\u{:04x}", c as u32)),
                '\u{2028}' | '\u{2029}' => out.push_str(&format!("\\u{:04x}", c as u32)),
                c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
                c => out.push(c),
            }
        }
        out.push('"');
    }
    
    /// JsonElement.toString: compact, without Gson's HTML escaping.
    impl std::fmt::Display for JJson {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut out = String::new();
            self.write(&mut out, false, false, 0);
            f.write_str(&out)
        }
    }
    
    /// A Gson (from `new Gson()` or a GsonBuilder): how it writes.
    #[derive(Clone, Copy, Debug)]
    pub struct JGson {
        pub pretty: bool,
        pub html: bool,
        pub nulls: bool,
    }
    
    impl Default for JGson {
        fn default() -> Self {
            JGson { pretty: false, html: true, nulls: false }
        }
    }
    
    impl JGson {
        /// Gson.toJson.
        pub fn to_json(&self, v: &JJson) -> String {
            let mut out = String::new();
            self.strip(v).write(&mut out, self.pretty, self.html, 0);
            out
        }
        /// Without serializeNulls, Gson leaves out null fields of objects.
        fn strip(&self, v: &JJson) -> JJson {
            match v {
                JJson::Obj(m) => JJson::Obj(m.iter().filter(|(_, x)| self.nulls || !x.is_json_null()).map(|(k, x)| (k.clone(), self.strip(x))).collect()),
                JJson::Arr(a) => JJson::Arr(a.iter().map(|x| self.strip(x)).collect()),
                other => other.clone(),
            }
        }
    }
    
    #[cfg(test)]
    mod jjson_tests {
        use super::*;
    
        #[test]
        fn read_and_write() {
            let j = JJson::parse("{\"a\": 1, \"b\": [1.5, \"x\", true, null], \"c\": {\"d\": -2e3}}");
            assert!(j.has("a") && !j.has("z"));
            assert_eq!(j.get("a").unwrap().as_int(), 1);
            assert_eq!(j.member_array("b").unwrap().at(0).as_float(), 1.5);
            assert_eq!(j.member_array("b").unwrap().size(), 4);
            assert_eq!(j.member_object("c").unwrap().get("d").unwrap().as_double(), -2000.0);
            assert_eq!(j.get("z"), None);
            assert_eq!(j.to_string(), "{\"a\":1,\"b\":[1.5,\"x\",true,null],\"c\":{\"d\":-2e3}}");
            let g = JGson { pretty: true, ..JGson::default() };
            assert_eq!(g.to_json(&JJson::parse("{\"a\":[1,2],\"b\":{},\"n\":null,\"s\":\"<x>\"}")),
                       "{\n  \"a\": [\n    1,\n    2\n  ],\n  \"b\": {},\n  \"s\": \"\\u003cx\\u003e\"\n}");
        }
    
        #[test]
        fn java_numbers() {
            assert_eq!(j_java_num(1.0, true), "1.0");
            assert_eq!(j_java_num(0.1, true), "0.1");
            assert_eq!(j_java_num(1.0e-4, false), "1.0E-4");
            assert_eq!(j_java_num(12345678.0, false), "1.2345678E7");
            assert_eq!(j_java_num(-0.75, false), "-0.75");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_are_set() {
        assert_eq!(ID, "creativecore");
        assert!(!NAME.is_empty());
        assert!(KEYS.iter().all(|[n, _, d]| !n.is_empty() && d.starts_with("key.")));
    }
}
