// Entity Texture Features, ported to Horizon by horizon-port from entity_texture_features-7.2.7-26.3-fabric.jar.
// This file is both the bundle's Cargo library (its tests) and the module the bundle adds to
// the game (crate::game::port_entity_texture_features). Put the mod's own logic here, in plain Rust
// that doesn't need the game, and call it from the helpers. No `//!` comments: module payloads
// can't hold inner attributes.

/// The mod's id (its settings folder), name, version and description.
pub const ID: &str = "entity_texture_features";
pub const NAME: &str = "Entity Texture Features";
pub const VERSION: &str = "7.2.7";
pub const DESCRIPTION: &str = "Adds support for resource-pack driven features for entity textures including some OptiFine features Supports OptiFine:  - Random & Custom textures  - Emissive textures With more features such as:  - Blinking textures  - Player Skin support";

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

    /// `IrisHandler` (traben/entity_texture_features/compat/IrisHandler.java)
    pub mod iris_handler {
        use super::*;
        pub static INSTANCE: LazyLock<JCell<Option<iris_handler::IrisHandler>>> = LazyLock::new(|| JCell::new(None));
        #[derive(Clone, Debug)]
        pub struct IrisHandler {
        }
        impl IrisHandler {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // IrisHandler.isShaderActive(0): not translated, it has no body.
        }
        // IrisHandler.getInstance(0): not translated, its body couldn't be read: an anonymous class.
    }

    /// `IrisHandler.IrisHandlerImpl` (traben/entity_texture_features/compat/IrisHandler.java)
    pub mod iris_handler_iris_handler_impl {
        use super::*;
        // The class itself isn't translated: it extends `IrisHandler`. Its static parts are below.
        // IrisHandler.IrisHandlerImpl.IrisHandlerImpl(0): not translated, its class isn't translated (it extends `IrisHandler`).
        // IrisHandler.IrisHandlerImpl.isShaderActive(0): not translated, its class isn't translated (it extends `IrisHandler`).
    }

    /// `SodiumGetBufferInjector` (traben/entity_texture_features/compat/SodiumGetBufferInjector.java)
    pub mod sodium_get_buffer_injector {
        use super::*;
        // INSTANCE: not translated, it uses `TriConsumer` (org.apache.logging.log4j.util).
        #[derive(Clone, Debug)]
        pub struct SodiumGetBufferInjector {
        }
        impl SodiumGetBufferInjector {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // SodiumGetBufferInjector.inject(3): not translated, it uses Minecraft's `RenderType`.
        // SodiumGetBufferInjector.get(0): not translated, it uses `TriConsumer` (org.apache.logging.log4j.util).
    }

    /// `ETFConfig` (traben/entity_texture_features/config/ETFConfig.java)
    pub mod etf_config {
        use super::*;
        // The class itself isn't translated: it extends `TConfig` (traben.tconfig). Its static parts are below.
        // ETFConfig.isPropertyDisabled(1): not translated, its class isn't translated (it extends `TConfig` (traben.tconfig)).
        // ETFConfig.canPropertyUpdate(1): not translated, its class isn't translated (it extends `TConfig` (traben.tconfig)).
        // ETFConfig.canDoCustomTextures(0): not translated, its class isn't translated (it extends `TConfig` (traben.tconfig)).
        // ETFConfig.canDoEmissiveTextures(0): not translated, its class isn't translated (it extends `TConfig` (traben.tconfig)).
        // ETFConfig.getEmissiveRenderMode(0): not translated, its class isn't translated (it extends `TConfig` (traben.tconfig)).
        // ETFConfig.getRenderLayerOverride(0): not translated, its class isn't translated (it extends `TConfig` (traben.tconfig)).
        // ETFConfig.getLightOverride(3): not translated, its class isn't translated (it extends `TConfig` (traben.tconfig)).
        // ETFConfig.getLightOverrideBE(1): not translated, its class isn't translated (it extends `TConfig` (traben.tconfig)).
        // ETFConfig.getLightOverrideBE(2): not translated, its class isn't translated (it extends `TConfig` (traben.tconfig)).
        // ETFConfig.getGUIOptions(0): not translated, its class isn't translated (it extends `TConfig` (traben.tconfig)).
        // ETFConfig.getEntitySettings(0): not translated, its class isn't translated (it extends `TConfig` (traben.tconfig)).
        // ETFConfig.addEntityConfigs(2): not translated, its class isn't translated (it extends `TConfig` (traben.tconfig)).
        // ETFConfig.getPropertySettings(0): not translated, its class isn't translated (it extends `TConfig` (traben.tconfig)).
        // ETFConfig.getPlayerSkinEditorButton(0): not translated, its class isn't translated (it extends `TConfig` (traben.tconfig)).
        // ETFConfig.getModIcon(0): not translated, its class isn't translated (it extends `TConfig` (traben.tconfig)).
    }

    /// `ETFConfig.DebugLogMode` (traben/entity_texture_features/config/ETFConfig.java)
    pub mod etf_config_debug_log_mode {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // ETFConfig.DebugLogMode.DebugLogMode(1): not translated, its class isn't translated (its constants carry values).
        // ETFConfig.DebugLogMode.toString(0): not translated, its class isn't translated (its constants carry values).
    }

    /// `ETFConfig.EmissiveRenderModes` (traben/entity_texture_features/config/ETFConfig.java)
    pub mod etf_config_emissive_render_modes {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum EmissiveRenderModes {
            Dull,
            Bright,
        }
        impl EmissiveRenderModes {
            pub const VALUES: [EmissiveRenderModes; 2] = [EmissiveRenderModes::Dull, EmissiveRenderModes::Bright, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    EmissiveRenderModes::Dull => "DULL",
                    EmissiveRenderModes::Bright => "BRIGHT",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "DULL" => EmissiveRenderModes::Dull,
                    "BRIGHT" => EmissiveRenderModes::Bright,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
        impl EmissiveRenderModes {
            // ETFConfig.EmissiveRenderModes.toString(0): not translated, `this` used as a value.
        }
    }

    /// `ETFConfig.IllegalPathMode` (traben/entity_texture_features/config/ETFConfig.java)
    pub mod etf_config_illegal_path_mode {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // ETFConfig.IllegalPathMode.IllegalPathMode(1): not translated, its class isn't translated (its constants carry values).
        // ETFConfig.IllegalPathMode.toString(0): not translated, its class isn't translated (its constants carry values).
    }

    /// `ETFConfig.OverrideBooleanType` (traben/entity_texture_features/config/ETFConfig.java)
    pub mod etf_config_override_boolean_type {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum OverrideBooleanType {
            True,
            False,
        }
        impl OverrideBooleanType {
            pub const VALUES: [OverrideBooleanType; 2] = [OverrideBooleanType::True, OverrideBooleanType::False, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    OverrideBooleanType::True => "TRUE",
                    OverrideBooleanType::False => "FALSE",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "TRUE" => OverrideBooleanType::True,
                    "FALSE" => OverrideBooleanType::False,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
        impl OverrideBooleanType {
            // ETFConfig.OverrideBooleanType.toString(0): not translated, `this` used as a value.
        }
    }

    /// `ETFConfig.RenderLayerOverride` (traben/entity_texture_features/config/ETFConfig.java)
    pub mod etf_config_render_layer_override {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // ETFConfig.RenderLayerOverride.RenderLayerOverride(1): not translated, its class isn't translated (its constants carry values).
        // ETFConfig.RenderLayerOverride.toString(0): not translated, its class isn't translated (its constants carry values).
    }

    /// `ETFConfig.SettingsButtonLocation` (traben/entity_texture_features/config/ETFConfig.java)
    pub mod etf_config_settings_button_location {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum SettingsButtonLocation {
            Off,
            BottomRight,
            TopRight,
            TopLeft,
            BottomLeft,
        }
        impl SettingsButtonLocation {
            pub const VALUES: [SettingsButtonLocation; 5] = [SettingsButtonLocation::Off, SettingsButtonLocation::BottomRight, SettingsButtonLocation::TopRight, SettingsButtonLocation::TopLeft, SettingsButtonLocation::BottomLeft, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    SettingsButtonLocation::Off => "OFF",
                    SettingsButtonLocation::BottomRight => "BOTTOM_RIGHT",
                    SettingsButtonLocation::TopRight => "TOP_RIGHT",
                    SettingsButtonLocation::TopLeft => "TOP_LEFT",
                    SettingsButtonLocation::BottomLeft => "BOTTOM_LEFT",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "OFF" => SettingsButtonLocation::Off,
                    "BOTTOM_RIGHT" => SettingsButtonLocation::BottomRight,
                    "TOP_RIGHT" => SettingsButtonLocation::TopRight,
                    "TOP_LEFT" => SettingsButtonLocation::TopLeft,
                    "BOTTOM_LEFT" => SettingsButtonLocation::BottomLeft,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `ETFConfig.SkinTransparencyMode` (traben/entity_texture_features/config/ETFConfig.java)
    pub mod etf_config_skin_transparency_mode {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // ETFConfig.SkinTransparencyMode.SkinTransparencyMode(1): not translated, its class isn't translated (its constants carry values).
        // ETFConfig.SkinTransparencyMode.toString(0): not translated, its class isn't translated (its constants carry values).
    }

    /// `ETFConfig.String2BooleanNullMap` (traben/entity_texture_features/config/ETFConfig.java)
    pub mod etf_config_string2_boolean_null_map {
        use super::*;
        // The class itself isn't translated: it extends `HashMap` (java.util). Its static parts are below.
        // ETFConfig.String2BooleanNullMap.putNullable(2): not translated, its class isn't translated (it extends `HashMap` (java.util)).
        // ETFConfig.String2BooleanNullMap.getNullable(1): not translated, its class isn't translated (it extends `HashMap` (java.util)).
    }

    /// `ETFConfig.String2EnumNullMap` (traben/entity_texture_features/config/ETFConfig.java)
    pub mod etf_config_string2_enum_null_map {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // ETFConfig.String2EnumNullMap.putNullable(2): not translated, its class isn't translated (it's generic).
        // ETFConfig.String2EnumNullMap.getNullable(1): not translated, its class isn't translated (it's generic).
    }

    /// `ETFConfig.UpdateFrequency` (traben/entity_texture_features/config/ETFConfig.java)
    pub mod etf_config_update_frequency {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // ETFConfig.UpdateFrequency.UpdateFrequency(2): not translated, its class isn't translated (its constants carry values).
        // ETFConfig.UpdateFrequency.getDelay(0): not translated, its class isn't translated (its constants carry values).
        // ETFConfig.UpdateFrequency.toString(0): not translated, its class isn't translated (its constants carry values).
    }

    /// `ETFConfigWarning` (traben/entity_texture_features/config/ETFConfigWarning.java)
    pub mod etf_config_warning {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ETFConfigWarning {
        }
        impl ETFConfigWarning {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // ETFConfigWarning.isConditionMet(0): not translated, it has no body.
            // ETFConfigWarning.getTitle(0): not translated, it has no body.
            // ETFConfigWarning.getSubTitle(0): not translated, it has no body.
            // ETFConfigWarning.getID(0): not translated, it has no body.
            // ETFConfigWarning.testWarningAndApplyFixIfEnabled(0): not translated, it has no body.
            // ETFConfigWarning.isEnabled(0): not translated, it calls ETFConfigWarning.isConditionMet, which isn't translated.
            // ETFConfigWarning.doesShowDisableButton(0): not translated, it has no body.
        }
    }

    /// `ETFConfigWarning.Simple` (traben/entity_texture_features/config/ETFConfigWarning.java)
    pub mod etf_config_warning_simple {
        use super::*;
        // The class itself isn't translated: it extends `ETFConfigWarning`. Its static parts are below.
        // ETFConfigWarning.Simple.Simple(5): not translated, its class isn't translated (it extends `ETFConfigWarning`).
        // ETFConfigWarning.Simple.Simple(5): not translated, its class isn't translated (it extends `ETFConfigWarning`).
        // ETFConfigWarning.Simple.isConditionMet(0): not translated, its class isn't translated (it extends `ETFConfigWarning`).
        // ETFConfigWarning.Simple.getTitle(0): not translated, its class isn't translated (it extends `ETFConfigWarning`).
        // ETFConfigWarning.Simple.getSubTitle(0): not translated, its class isn't translated (it extends `ETFConfigWarning`).
        // ETFConfigWarning.Simple.getID(0): not translated, its class isn't translated (it extends `ETFConfigWarning`).
        // ETFConfigWarning.Simple.testWarningAndApplyFixIfEnabled(0): not translated, its class isn't translated (it extends `ETFConfigWarning`).
        // ETFConfigWarning.Simple.doesShowDisableButton(0): not translated, its class isn't translated (it extends `ETFConfigWarning`).
    }

    /// `ETFConfigWarnings` (traben/entity_texture_features/config/ETFConfigWarnings.java)
    pub mod etf_config_warnings {
        use super::*;
        // REGISTERED_WARNINGS: not translated, a Map or Set keyed by a ETFConfigWarning.
        #[derive(Clone, Debug)]
        pub struct ETFConfigWarnings {
        }
        impl ETFConfigWarnings {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ETFConfigWarnings.registerConfigWarning(1): not translated, `null` (Rust has no null: this needs an Option).
        // ETFConfigWarnings.getRegisteredWarnings(0): not translated, a Map or Set keyed by a ETFConfigWarning.
    }

    /// `ETFModMenu` (traben/entity_texture_features/config/ETFModMenu.java)
    pub mod etf_mod_menu {
        use super::*;
        // The class itself isn't translated: it implements `ModMenuApi` (com.terraformersmc.modmenu.api). Its static parts are below.
        // ETFModMenu.getModConfigScreenFactory(0): not translated, its class isn't translated (it implements `ModMenuApi` (com.terraformersmc.modmenu.api)).
    }

    /// `ETFConfigScreenMain` (traben/entity_texture_features/config/screens/ETFConfigScreenMain.java)
    pub mod etf_config_screen_main {
        use super::*;
        // The class itself isn't translated: it extends `TConfigScreenMain` (traben.tconfig.gui). Its static parts are below.
        // ETFConfigScreenMain.ETFConfigScreenMain(1): not translated, its class isn't translated (it extends `TConfigScreenMain` (traben.tconfig.gui)).
        // ETFConfigScreenMain.drawEntity12106(9): not translated, its class isn't translated (it extends `TConfigScreenMain` (traben.tconfig.gui)).
        // ETFConfigScreenMain.init(0): not translated, its class isn't translated (it extends `TConfigScreenMain` (traben.tconfig.gui)).
        // ETFConfigScreenMain.extractRenderState(4): not translated, its class isn't translated (it extends `TConfigScreenMain` (traben.tconfig.gui)).
        // ETFConfigScreenMain.renderEntitySample(3): not translated, its class isn't translated (it extends `TConfigScreenMain` (traben.tconfig.gui)).
    }

    /// `ETFConfigScreenMain.LogoCreeperRenderer` (traben/entity_texture_features/config/screens/ETFConfigScreenMain.java)
    pub mod etf_config_screen_main_logo_creeper_renderer {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `ModelPart`. Its static parts are below.
        // ETFConfigScreenMain.LogoCreeperRenderer.LogoCreeperRenderer(0): not translated, its class isn't translated (it uses Minecraft's `ModelPart`).
    }

    /// `ETFConfigScreenWarnings` (traben/entity_texture_features/config/screens/ETFConfigScreenWarnings.java)
    pub mod etf_config_screen_warnings {
        use super::*;
        // The class itself isn't translated: it extends `TConfigScreen` (traben.tconfig.gui). Its static parts are below.
        // ETFConfigScreenWarnings.ETFConfigScreenWarnings(2): not translated, its class isn't translated (it extends `TConfigScreen` (traben.tconfig.gui)).
        // ETFConfigScreenWarnings.getIgnoredWarnings(0): not translated, it uses TConfigHandler, which isn't translated (it's generic).
        // ETFConfigScreenWarnings.onClose(0): not translated, its class isn't translated (it extends `TConfigScreen` (traben.tconfig.gui)).
        // ETFConfigScreenWarnings.init(0): not translated, its class isn't translated (it extends `TConfigScreen` (traben.tconfig.gui)).
        // ETFConfigScreenWarnings.extractRenderState(4): not translated, its class isn't translated (it extends `TConfigScreen` (traben.tconfig.gui)).
    }

    /// `ETFConfigScreenWarnings.WarningConfig` (traben/entity_texture_features/config/screens/ETFConfigScreenWarnings.java)
    pub mod etf_config_screen_warnings_warning_config {
        use super::*;
        // The class itself isn't translated: it extends `NoGUI`. Its static parts are below.
    }

    /// `ETFConfigScreenSkinTool` (traben/entity_texture_features/config/screens/skin/ETFConfigScreenSkinTool.java)
    pub mod etf_config_screen_skin_tool {
        use super::*;
        // The class itself isn't translated: it extends `ETFScreenOldCompat`. Its static parts are below.
        // APPLY_OVERLAY: not translated, it calls ETFUtils2.res, which isn't translated.
        // REMOVE_OVERLAY: not translated, it calls ETFUtils2.res, which isn't translated.
        // WHOLE_FACE_OVERLAY: not translated, it calls ETFUtils2.res, which isn't translated.
        // SMALL_EYE_OVERLAY: not translated, it calls ETFUtils2.res, which isn't translated.
        // BOXES_OVERLAY: not translated, it calls ETFUtils2.res, which isn't translated.
        // ETFConfigScreenSkinTool.ETFConfigScreenSkinTool(1): not translated, its class isn't translated (it extends `ETFScreenOldCompat`).
        pub fn get_pixel_colour(choice: i32) -> i32 {
            match choice {
                1 => {
                    return -65281;
                }
                2 => {
                    return -256;
                }
                3 => {
                    return -16776961;
                }
                4 => {
                    return -16711936;
                }
                5 => {
                    return -16760705;
                }
                6 => {
                    return -65536;
                }
                7 => {
                    return -16744449;
                }
                8 => {
                    return -14483457;
                }
                _ => {
                    return choice;
                }
            }
            #[allow(unreachable_code)]
            unreachable!()
        }
        // ETFConfigScreenSkinTool.onExit(0): not translated, its class isn't translated (it extends `ETFScreenOldCompat`).
        // ETFConfigScreenSkinTool.onClose(0): not translated, its class isn't translated (it extends `ETFScreenOldCompat`).
        // ETFConfigScreenSkinTool.init(0): not translated, its class isn't translated (it extends `ETFScreenOldCompat`).
        // ETFConfigScreenSkinTool.updateButtons(0): not translated, its class isn't translated (it extends `ETFScreenOldCompat`).
        // ETFConfigScreenSkinTool.extractRenderState(4): not translated, its class isn't translated (it extends `ETFScreenOldCompat`).
        // ETFConfigScreenSkinTool.applyExistingOverlayToSkin(1): not translated, its body couldn't be read: an assert.
        // ETFConfigScreenSkinTool.printPlayerSkinCopy(0): not translated, its class isn't translated (it extends `ETFScreenOldCompat`).
        // ETFConfigScreenSkinTool.renderEntityInInventoryFollowsMouse(11): not translated, it uses Minecraft's `GuiGraphicsExtractor`.
        // ETFConfigScreenSkinTool.drawEntity(7): not translated, its class isn't translated (it extends `ETFScreenOldCompat`).
    }

    /// `ETFConfigScreenSkinTool.BlinkType` (traben/entity_texture_features/config/screens/skin/ETFConfigScreenSkinTool.java)
    pub mod etf_config_screen_skin_tool_blink_type {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum BlinkType {
            OnePixel,
            TwoPixel,
            FourPixel,
            WholeFace,
            WholeFaceTwo,
            None,
        }
        impl BlinkType {
            pub const VALUES: [BlinkType; 6] = [BlinkType::OnePixel, BlinkType::TwoPixel, BlinkType::FourPixel, BlinkType::WholeFace, BlinkType::WholeFaceTwo, BlinkType::None, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    BlinkType::OnePixel => "ONE_PIXEL",
                    BlinkType::TwoPixel => "TWO_PIXEL",
                    BlinkType::FourPixel => "FOUR_PIXEL",
                    BlinkType::WholeFace => "WHOLE_FACE",
                    BlinkType::WholeFaceTwo => "WHOLE_FACE_TWO",
                    BlinkType::None => "NONE",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "ONE_PIXEL" => BlinkType::OnePixel,
                    "TWO_PIXEL" => BlinkType::TwoPixel,
                    "FOUR_PIXEL" => BlinkType::FourPixel,
                    "WHOLE_FACE" => BlinkType::WholeFace,
                    "WHOLE_FACE_TWO" => BlinkType::WholeFaceTwo,
                    "NONE" => BlinkType::None,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
        impl BlinkType {
            // ETFConfigScreenSkinTool.BlinkType.getExampleOverlay(0): not translated, `this` used as a value.
            // ETFConfigScreenSkinTool.BlinkType.getTitle(0): not translated, it uses Minecraft's `Component`.
            // ETFConfigScreenSkinTool.BlinkType.getBlinkPixelColour(0): not translated, `this` used as a value.
            // ETFConfigScreenSkinTool.BlinkType.getMaxEyePixelHeight(0): not translated, `this` used as a value.
            // ETFConfigScreenSkinTool.BlinkType.next(0): not translated, `this` used as a value.
        }
        pub fn get(id: i32) -> etf_config_screen_skin_tool_blink_type::BlinkType {
            match id {
                1 => {
                    return etf_config_screen_skin_tool_blink_type::BlinkType::WholeFace;
                }
                2 => {
                    return etf_config_screen_skin_tool_blink_type::BlinkType::WholeFaceTwo;
                }
                3 => {
                    return etf_config_screen_skin_tool_blink_type::BlinkType::OnePixel;
                }
                4 => {
                    return etf_config_screen_skin_tool_blink_type::BlinkType::TwoPixel;
                }
                5 => {
                    return etf_config_screen_skin_tool_blink_type::BlinkType::FourPixel;
                }
                _ => {
                    return etf_config_screen_skin_tool_blink_type::BlinkType::None;
                }
            }
            #[allow(unreachable_code)]
            unreachable!()
        }
    }

    /// `ETFConfigScreenSkinTool.CoatStyle` (traben/entity_texture_features/config/screens/skin/ETFConfigScreenSkinTool.java)
    pub mod etf_config_screen_skin_tool_coat_style {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum CoatStyle {
            CopiedThinTop,
            MovedThinTop,
            CopiedFatTop,
            MovedFatTop,
            CopiedThin,
            MovedThin,
            CopiedFat,
            MovedFat,
            None,
        }
        impl CoatStyle {
            pub const VALUES: [CoatStyle; 9] = [CoatStyle::CopiedThinTop, CoatStyle::MovedThinTop, CoatStyle::CopiedFatTop, CoatStyle::MovedFatTop, CoatStyle::CopiedThin, CoatStyle::MovedThin, CoatStyle::CopiedFat, CoatStyle::MovedFat, CoatStyle::None, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    CoatStyle::CopiedThinTop => "COPIED_THIN_TOP",
                    CoatStyle::MovedThinTop => "MOVED_THIN_TOP",
                    CoatStyle::CopiedFatTop => "COPIED_FAT_TOP",
                    CoatStyle::MovedFatTop => "MOVED_FAT_TOP",
                    CoatStyle::CopiedThin => "COPIED_THIN",
                    CoatStyle::MovedThin => "MOVED_THIN",
                    CoatStyle::CopiedFat => "COPIED_FAT",
                    CoatStyle::MovedFat => "MOVED_FAT",
                    CoatStyle::None => "NONE",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "COPIED_THIN_TOP" => CoatStyle::CopiedThinTop,
                    "MOVED_THIN_TOP" => CoatStyle::MovedThinTop,
                    "COPIED_FAT_TOP" => CoatStyle::CopiedFatTop,
                    "MOVED_FAT_TOP" => CoatStyle::MovedFatTop,
                    "COPIED_THIN" => CoatStyle::CopiedThin,
                    "MOVED_THIN" => CoatStyle::MovedThin,
                    "COPIED_FAT" => CoatStyle::CopiedFat,
                    "MOVED_FAT" => CoatStyle::MovedFat,
                    "NONE" => CoatStyle::None,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
        impl CoatStyle {
            // ETFConfigScreenSkinTool.CoatStyle.getTitle(0): not translated, it uses Minecraft's `Component`.
            // ETFConfigScreenSkinTool.CoatStyle.getCoatPixelColour(0): not translated, `this` used as a value.
            // ETFConfigScreenSkinTool.CoatStyle.next(0): not translated, `this` used as a value.
        }
        pub fn get(id: i32) -> etf_config_screen_skin_tool_coat_style::CoatStyle {
            match id {
                1 => {
                    return etf_config_screen_skin_tool_coat_style::CoatStyle::CopiedThinTop;
                }
                2 => {
                    return etf_config_screen_skin_tool_coat_style::CoatStyle::MovedThinTop;
                }
                3 => {
                    return etf_config_screen_skin_tool_coat_style::CoatStyle::CopiedFatTop;
                }
                4 => {
                    return etf_config_screen_skin_tool_coat_style::CoatStyle::MovedFatTop;
                }
                5 => {
                    return etf_config_screen_skin_tool_coat_style::CoatStyle::CopiedThin;
                }
                6 => {
                    return etf_config_screen_skin_tool_coat_style::CoatStyle::MovedThin;
                }
                7 => {
                    return etf_config_screen_skin_tool_coat_style::CoatStyle::CopiedFat;
                }
                8 => {
                    return etf_config_screen_skin_tool_coat_style::CoatStyle::MovedFat;
                }
                _ => {
                    return etf_config_screen_skin_tool_coat_style::CoatStyle::None;
                }
            }
            #[allow(unreachable_code)]
            unreachable!()
        }
    }

    /// `ETFConfigScreenSkinTool.ConfirmScreen` (traben/entity_texture_features/config/screens/skin/ETFConfigScreenSkinTool.java)
    pub mod etf_config_screen_skin_tool_confirm_screen {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Screen`. Its static parts are below.
        // ETFConfigScreenSkinTool.ConfirmScreen.ConfirmScreen(2): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // ETFConfigScreenSkinTool.ConfirmScreen.onClose(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // ETFConfigScreenSkinTool.ConfirmScreen.shouldCloseOnEsc(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // ETFConfigScreenSkinTool.ConfirmScreen.init(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // ETFConfigScreenSkinTool.ConfirmScreen.extractRenderState(4): not translated, its class isn't translated (it extends Minecraft's `Screen`).
    }

    /// `ETFConfigScreenSkinTool.NoseType` (traben/entity_texture_features/config/screens/skin/ETFConfigScreenSkinTool.java)
    pub mod etf_config_screen_skin_tool_nose_type {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // ETFConfigScreenSkinTool.NoseType.NoseType(2): not translated, its class isn't translated (its constants carry values).
        // ETFConfigScreenSkinTool.NoseType.getByColorId(1): not translated, its class isn't translated (its constants carry values).
        // ETFConfigScreenSkinTool.NoseType.appliesTextureOverlay(0): not translated, its class isn't translated (its constants carry values).
        // ETFConfigScreenSkinTool.NoseType.getButtonText(0): not translated, its class isn't translated (its constants carry values).
        // ETFConfigScreenSkinTool.NoseType.next(0): not translated, its class isn't translated (its constants carry values).
        // ETFConfigScreenSkinTool.NoseType.getNosePixelColour(0): not translated, its class isn't translated (its constants carry values).
    }

    /// `ETFConfigScreenSkinToolOutcome` (traben/entity_texture_features/config/screens/skin/ETFConfigScreenSkinToolOutcome.java)
    pub mod etf_config_screen_skin_tool_outcome {
        use super::*;
        // The class itself isn't translated: it extends `ETFScreenOldCompat`. Its static parts are below.
        // ETFConfigScreenSkinToolOutcome.ETFConfigScreenSkinToolOutcome(3): not translated, its class isn't translated (it extends `ETFScreenOldCompat`).
        // ETFConfigScreenSkinToolOutcome.uploadSkin(1): not translated, it calls `InetAddress` (java.net).getLocalHost.
        pub fn concat(parts: &[Vec<i8>]) -> Vec<i8> {
            let mut len: i32 = 0;
            for p in parts.iter() {
                len += (p.len() as i32);
            }
            let mut out: Vec<i8> = vec![0i8; len as usize];
            let mut pos: i32 = 0;
            for p in parts.iter() {
                out[pos as usize..pos as usize + (p.len() as i32) as usize].copy_from_slice(&p[0..0 + (p.len() as i32) as usize]);
                pos += (p.len() as i32);
            }
            return out.to_vec();
        }
        // ETFConfigScreenSkinToolOutcome.init(0): not translated, its body couldn't be read: an assert.
        // ETFConfigScreenSkinToolOutcome.extractRenderState(4): not translated, its class isn't translated (it extends `ETFScreenOldCompat`).
    }

    /// `ETFConfigScreenSkinToolPixelSelection` (traben/entity_texture_features/config/screens/skin/ETFConfigScreenSkinToolPixelSelection.java)
    pub mod etf_config_screen_skin_tool_pixel_selection {
        use super::*;
        // The class itself isn't translated: it extends `ETFScreenOldCompat`. Its static parts are below.
        // ETFConfigScreenSkinToolPixelSelection.ETFConfigScreenSkinToolPixelSelection(2): not translated, its class isn't translated (it extends `ETFScreenOldCompat`).
        // ETFConfigScreenSkinToolPixelSelection.init(0): not translated, its class isn't translated (it extends `ETFScreenOldCompat`).
        // ETFConfigScreenSkinToolPixelSelection.getButtonPixels(3): not translated, its body couldn't be read: an anonymous class.
        // ETFConfigScreenSkinToolPixelSelection.applyCurrentSelectedPixels(0): not translated, its class isn't translated (it extends `ETFScreenOldCompat`).
        // ETFConfigScreenSkinToolPixelSelection.extractRenderState(4): not translated, its class isn't translated (it extends `ETFScreenOldCompat`).
        // ETFConfigScreenSkinToolPixelSelection.drawEntity(7): not translated, its class isn't translated (it extends `ETFScreenOldCompat`).
    }

    /// `ETFConfigScreenSkinToolPixelSelection.SelectionMode` (traben/entity_texture_features/config/screens/skin/ETFConfigScreenSkinToolPixelSelection.java)
    pub mod etf_config_screen_skin_tool_pixel_selection_selection_mode {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // ETFConfigScreenSkinToolPixelSelection.SelectionMode.SelectionMode(2): not translated, its class isn't translated (its constants carry values).
    }

    /// `ETFScreenOldCompat` (traben/entity_texture_features/config/screens/skin/ETFScreenOldCompat.java)
    pub mod etf_screen_old_compat {
        use super::*;
        // The class itself isn't translated: it extends `TConfigScreen` (traben.tconfig.gui). Its static parts are below.
        // ETFScreenOldCompat.ETFScreenOldCompat(3): not translated, its class isn't translated (it extends `TConfigScreen` (traben.tconfig.gui)).
        // ETFScreenOldCompat.renderGUITexture(6): not translated, it uses Minecraft's `GuiGraphicsExtractor`.
        // ETFScreenOldCompat.booleanAsOnOff(1): not translated, it calls Minecraft's `CommonComponents`.optionStatus.
        // ETFScreenOldCompat.getETFButton(6): not translated, its class isn't translated (it extends `TConfigScreen` (traben.tconfig.gui)).
        // ETFScreenOldCompat.getETFButton(7): not translated, its class isn't translated (it extends `TConfigScreen` (traben.tconfig.gui)).
    }

    /// `ETF` (traben/entity_texture_features/ETF.java)
    pub mod etf {
        use super::*;
        pub const MOD_ID: &str = "entity_texture_features";
        // LOGGER: the logger. Log calls become comments.
        pub const EMISSIVE_FEATURE_LIGHT_VALUE: i32 = 15728882;
        // warningConfigHandler: not translated, it uses TConfigHandler, which isn't translated (it's generic).
        pub static IRIS_DETECTED: JCell<bool> = JCell::new(false);
        pub static FABRIC_API: JCell<bool> = JCell::new(false);
        pub const EYES_FEATURE_LIGHT_VALUE: i32 = 15728881;
        // etfRenderStateConstructor: not translated, it uses ETFRenderStateInit, which isn't translated (it's an interface).
        pub static SKIN_LAYERS_DETECTED: JCell<bool> = JCell::new(false);
        // configHandlers: not translated, it uses TConfigHandler, which isn't translated (it's generic).
        // CONFIG: not translated, it uses TConfigHandler, which isn't translated (it's generic).
        #[derive(Clone, Debug)]
        pub struct ETF {
        }
        impl ETF {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ETF.config(0): not translated, it uses TConfigHandler, which isn't translated (it's generic).
        // ETF.start(0): not translated, it uses ETF.CONFIG, which isn't translated (it uses TConfigHandler, which isn't translated (it's generic)).
        // ETF.randomQuip(0): not translated, it creates `Random` (java.util).
        // ETF.registerConfigHandler(1): not translated, it uses TConfigHandler, which isn't translated (it's generic).
        // ETF.getConfigScreen(1): not translated, it uses Minecraft's `Screen`.
        // ETF.getConfigScreen(2): not translated, it uses Minecraft's `Screen`.
        // ETF.getBiomeString(2): not translated, it uses Minecraft's `Level`.
        // ETF.getTextFromTranslation(1): not translated, it uses Minecraft's `Component`.
        pub fn get_config_directory() -> std::path::PathBuf {
            return j_config_dir();
        }
        pub fn is_this_mod_loaded(mod_id: &str) -> bool {
            return false;
        }
        // ETF.modsLoaded(0): not translated, it calls Fabric's FabricLoader.getAllMods.
        pub fn is_forge() -> bool {
            return !etf::is_fabric();
        }
        pub fn is_fabric() -> bool {
            return true;
        }
    }

    /// `ETFApi` (traben/entity_texture_features/ETFApi.java)
    pub mod etf_api {
        use super::*;
        pub const ETF_API_VERSION: i32 = 12;
        // ETF_GENERIC_UUID: not translated, it uses `UUID` (java.util).
        pub const ETF_SPAWNER_MARKER: i64 = 53021371281465i64;
        // getETFConfigObject: not translated, it uses ETFConfig, which isn't translated (it extends `TConfig` (traben.tconfig)).
        #[derive(Clone, Debug)]
        pub struct ETFApi {
        }
        impl ETFApi {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ETFApi.getETFConfigObject(0): not translated, it uses ETFConfig, which isn't translated (it extends `TConfig` (traben.tconfig)).
        // ETFApi.setETFConfigObject(1): not translated, it uses ETFConfig, which isn't translated (it extends `TConfig` (traben.tconfig)).
        // ETFApi.getCopyOfETFConfigObject(0): not translated, it uses ETFConfig, which isn't translated (it extends `TConfig` (traben.tconfig)).
        // ETFApi.getDefaultETFConfigObject(0): not translated, it uses ETFConfig, which isn't translated (it extends `TConfig` (traben.tconfig)).
        // ETFApi.getBlockEntityTypeToTranslationKey(1): not translated, it uses Minecraft's `BlockEntityType`.
        // ETFApi.saveETFConfigChangesAndResetETF(0): not translated, it calls ETF.config, which isn't translated.
        // ETFApi.resetETF(0): not translated, it calls ETFManager.resetInstance, which isn't translated.
        // ETFApi.getUUIDForBlockEntity(1): not translated, it uses `UUID` (java.util).
        // ETFApi.stateOfEntityOrEntityState(1): not translated, it uses ETFEntityRenderState, which isn't translated (it's an interface).
        // ETFApi.getCurrentETFVariantTextureOfEntity(2): not translated, it uses Minecraft's `Entity`.
        // ETFApi.getCurrentETFVariantTextureOfBlockEntity(2): not translated, it uses Minecraft's `BlockEntity`.
        // ETFApi.getCurrentETFVariantTextureOfBlockEntity(3): not translated, it uses Minecraft's `BlockEntity`.
        // ETFApi.getCurrentETFVariantTextureOfBlockEntityInternal(2): not translated, it uses ETFEntity, which isn't translated (it's an interface).
        // ETFApi.getCurrentETFVariantTextureOfEntity(3): not translated, it uses Minecraft's `BlockEntity`.
        // ETFApi.getCurrentETFEmissiveTextureOfEntityOrNull(2): not translated, it uses Minecraft's `Entity`.
        // ETFApi.getCurrentETFEmissiveTextureOfBlockEntityOrNull(2): not translated, it uses Minecraft's `BlockEntity`.
        // ETFApi.getVariantSupplierOrNull(3): not translated, it uses ETFVariantSuffixProvider, which isn't translated (it's an interface).
        // ETFApi.getLastMatchingRuleOfEntity(1): not translated, it uses Minecraft's `Entity`.
        // ETFApi.getLastMatchingRuleOfBlockEntity(1): not translated, it uses Minecraft's `BlockEntity`.
        // ETFApi.registerCustomRandomPropertyFactory(2): not translated, it uses RandomPropertyFactory, which isn't translated (it's an interface).
        // ETFApi.registerCustomETFConfigWarning(2): not translated, `null` (Rust has no null: this needs an Option).
    }

    /// `ETFApi.ETFVariantSuffixProvider` (traben/entity_texture_features/ETFApi.java)
    pub mod etf_api_etf_variant_suffix_provider {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ETFApi.ETFVariantSuffixProvider.getVariantProviderOrNull(3): not translated, it uses ETFVariantSuffixProvider, which isn't translated (it's an interface).
        // ETFApi.ETFVariantSuffixProvider.entityCanUpdate(1): not translated, it has no body.
        // ETFApi.ETFVariantSuffixProvider.getAllSuffixes(0): not translated, it has no body.
        // ETFApi.ETFVariantSuffixProvider.size(0): not translated, it has no body.
        // ETFApi.ETFVariantSuffixProvider.getSuffixForEntity(1): not translated, its class isn't translated (it's an interface).
        // ETFApi.ETFVariantSuffixProvider.getSuffixForEntityState(1): not translated, its class isn't translated (it's an interface).
        // ETFApi.ETFVariantSuffixProvider.getSuffixForBlockEntity(1): not translated, its class isn't translated (it's an interface).
        // ETFApi.ETFVariantSuffixProvider.getSuffixForETFEntity(1): not translated, it has no body.
        // ETFApi.ETFVariantSuffixProvider.setRandomSupplier(1): not translated, its class isn't translated (it's an interface).
    }

    /// `ETFApi.ETFVariantSuffixProvider.EntityRandomSeedFunction` (traben/entity_texture_features/ETFApi.java)
    pub mod etf_api_etf_variant_suffix_provider_entity_random_seed_function {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ETFApi.ETFVariantSuffixProvider.EntityRandomSeedFunction.toInt(1): not translated, it has no body.
    }

    /// `ETFException` (traben/entity_texture_features/ETFException.java)
    pub mod etf_exception {
        use super::*;
        // The class itself isn't translated: it extends `RuntimeException`. Its static parts are below.
        // ETFException.ETFException(1): not translated, its class isn't translated (it extends `RuntimeException`).
        // ETFException.amendMessage(1): not translated, it uses ETFEntityRenderState, which isn't translated (it's an interface).
    }

    /// `ETFInit` (traben/entity_texture_features/ETFInit.java)
    pub mod etf_init {
        use super::*;
        // The class itself isn't translated: it implements Fabric's `ClientModInitializer`. Its static parts are below.
        // ETFInit.onInitializeClient(0): not translated, its class isn't translated (it implements Fabric's `ClientModInitializer`).
    }

    /// `ETFManager` (traben/entity_texture_features/features/ETFManager.java)
    pub mod etf_manager {
        use super::*;
        // The class itself isn't translated: it uses ETFLruCache, which isn't translated (it's generic). Its static parts are below.
        // ETF_ERROR_TEXTURE: not translated, it uses ETFTexture, which isn't translated (it uses ETFSprite, which isn't translated (it uses Minecraft's `TextureAtlasSprite`)).
        // instance: not translated, it uses ETFManager, which isn't translated (it uses ETFLruCache, which isn't translated (it's generic)).
        // ETFManager.ETFManager(0): not translated, its class isn't translated (it uses ETFLruCache, which isn't translated (it's generic)).
        // ETFManager.getInstance(0): not translated, it uses ETFManager, which isn't translated (it uses ETFLruCache, which isn't translated (it's generic)).
        // ETFManager.resetInstance(0): not translated, it calls ETF.config, which isn't translated.
        // ETFManager.getErrorETFTexture(0): not translated, it uses Minecraft's `NativeImage`.
        // ETFManager.getEmissiveMode(0): not translated, it calls ETF.config, which isn't translated.
        // ETFManager.getGeneralPrintout(0): not translated, its class isn't translated (it uses ETFLruCache, which isn't translated (it's generic)).
        // ETFManager.doTheBigBoyPrintoutKronk(0): not translated, its class isn't translated (it uses ETFLruCache, which isn't translated (it's generic)).
        // ETFManager.grabSpecialProperties(2): not translated, its class isn't translated (it uses ETFLruCache, which isn't translated (it's generic)).
        // ETFManager.markEntityForDebugPrint(1): not translated, its class isn't translated (it uses ETFLruCache, which isn't translated (it's generic)).
        // ETFManager.getETFTextureNoVariation(1): not translated, its class isn't translated (it uses ETFLruCache, which isn't translated (it's generic)).
        // ETFManager.getETFTextureVariant(2): not translated, its class isn't translated (it uses ETFLruCache, which isn't translated (it's generic)).
        // ETFManager.getOrCreateETFTexture(1): not translated, its class isn't translated (it uses ETFLruCache, which isn't translated (it's generic)).
        // ETFManager.getPlayerTexture(2): not translated, its class isn't translated (it uses ETFLruCache, which isn't translated (it's generic)).
        // ETFManager.getPlayerTexture(2): not translated, its class isn't translated (it uses ETFLruCache, which isn't translated (it's generic)).
    }

    /// `ETFManager.TextureSource` (traben/entity_texture_features/features/ETFManager.java)
    pub mod etf_manager_texture_source {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum TextureSource {
            Entity,
            BlockEntity,
            EntityFeature,
        }
        impl TextureSource {
            pub const VALUES: [TextureSource; 3] = [TextureSource::Entity, TextureSource::BlockEntity, TextureSource::EntityFeature, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    TextureSource::Entity => "ENTITY",
                    TextureSource::BlockEntity => "BLOCK_ENTITY",
                    TextureSource::EntityFeature => "ENTITY_FEATURE",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "ENTITY" => TextureSource::Entity,
                    "BLOCK_ENTITY" => TextureSource::BlockEntity,
                    "ENTITY_FEATURE" => TextureSource::EntityFeature,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `ETFRenderContext` (traben/entity_texture_features/features/ETFRenderContext.java)
    pub mod etf_render_context {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ETFRenderContext {
        }
        impl ETFRenderContext {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `ETFPlayerEntity` (traben/entity_texture_features/features/player/ETFPlayerEntity.java)
    pub mod etf_player_entity {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ETFPlayerEntity.etf$getEntity(0): not translated, it has no body.
        // ETFPlayerEntity.etf$isTeammate(1): not translated, it has no body.
        // ETFPlayerEntity.etf$getInventory(0): not translated, it has no body.
        // ETFPlayerEntity.etf$isPartVisible(1): not translated, it has no body.
        // ETFPlayerEntity.etf$getName(0): not translated, it has no body.
        // ETFPlayerEntity.etf$getUuidAsString(0): not translated, it has no body.
    }

    /// `ETFPlayerFeatureRenderer` (traben/entity_texture_features/features/player/ETFPlayerFeatureRenderer.java)
    pub mod etf_player_feature_renderer {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // villagerNose: not translated, it uses Minecraft's `ModelPart`.
        // textureNose: not translated, it uses Minecraft's `ModelPart`.
        // jacket: not translated, it uses Minecraft's `ModelPart`.
        // fatJacket: not translated, it uses Minecraft's `ModelPart`.
        // VILLAGER_TEXTURE: not translated, it calls ETFUtils2.res, which isn't translated.
        // ETFPlayerFeatureRenderer.ETFPlayerFeatureRenderer(1): not translated, its class isn't translated (it's generic).
        // ETFPlayerFeatureRenderer.getModelData(1): not translated, it uses Minecraft's `MeshDefinition`.
        // ETFPlayerFeatureRenderer.renderSkullFeatures(6): not translated, it uses Minecraft's `SubmitNodeCollector`.
        // ETFPlayerFeatureRenderer.renderEnchanted(5): not translated, it uses Minecraft's `SubmitNodeCollector`.
        // ETFPlayerFeatureRenderer.renderNose(4): not translated, it uses Minecraft's `SubmitNodeCollector`.
        // ETFPlayerFeatureRenderer.submit(6): not translated, its class isn't translated (it's generic).
        // ETFPlayerFeatureRenderer.renderFeatures(6): not translated, its class isn't translated (it's generic).
        // ETFPlayerFeatureRenderer.renderCoat(6): not translated, its class isn't translated (it's generic).
    }

    /// `ETFPlayerSkinHolder` (traben/entity_texture_features/features/player/ETFPlayerSkinHolder.java)
    pub mod etf_player_skin_holder {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ETFPlayerSkinHolder.etf$getETFPlayerTexture(0): not translated, it has no body.
    }

    /// `ETFPlayerTexture` (traben/entity_texture_features/features/player/ETFPlayerTexture.java)
    pub mod etf_player_texture {
        use super::*;
        // The class itself isn't translated: it uses NoseType, which isn't translated (its constants carry values). Its static parts are below.
        pub const SKIN_NAMESPACE: &str = "etf_skin";
        // clientPlayerOriginalSkinImageForTool: not translated, it uses Minecraft's `NativeImage`.
        pub static REMAPPING_ETF_SKIN: JCell<bool> = JCell::new(false);
        // TRY_AGAIN_LATER: not translated, it uses ETFException, which isn't translated (it extends `RuntimeException`).
        // downloader: not translated, it uses Minecraft's `SkinTextureDownloader`.
        // ETFPlayerTexture.getOriginal(0): not translated, its class isn't translated (it uses NoseType, which isn't translated (its constants carry values)).
        // ETFPlayerTexture.ETFPlayerTexture(2): not translated, its class isn't translated (it uses NoseType, which isn't translated (its constants carry values)).
        // ETFPlayerTexture.etf$profileSkinTextureLocation(1): not translated, it uses Minecraft's `GameProfile`.
        // ETFPlayerTexture.etf$tryLoadOriginalSkinFromRendererIfProfileTextureIdDiffers(1): not translated, its class isn't translated (it uses NoseType, which isn't translated (its constants carry values)).
        // ETFPlayerTexture.getSkinOfPlayer(2): not translated, it uses Minecraft's `NativeImage`.
        // ETFPlayerTexture.ETFPlayerTexture(2): not translated, its class isn't translated (it uses NoseType, which isn't translated (its constants carry values)).
        // ETFPlayerTexture.ETFPlayerTexture(0): not translated, its body couldn't be read: an assert.
        // ETFPlayerTexture.returnMatchPixels(3): not translated, it uses Minecraft's `NativeImage`.
        // ETFPlayerTexture.returnMatchPixels(4): not translated, it uses Minecraft's `NativeImage`.
        // ETFPlayerTexture.returnNullIfEmptyImage(1): not translated, it uses Minecraft's `NativeImage`.
        // ETFPlayerTexture.getSkinPixelBounds(1): not translated, a switch expression giving a int[].
        // ETFPlayerTexture.returnOptimizedBlinkFace(3): not translated, it uses Minecraft's `NativeImage`.
        // ETFPlayerTexture.returnOptimizedBlinkFace(4): not translated, it uses Minecraft's `NativeImage`.
        // ETFPlayerTexture.forceSolidLowerSkin(1): not translated, it uses Minecraft's `NativeImage`.
        // ETFPlayerTexture.getCoatTexture(3): not translated, it uses Minecraft's `NativeImage`.
        // ETFPlayerTexture.copyToPixels(5): not translated, it uses Minecraft's `NativeImage`.
        // ETFPlayerTexture.copyToPixels(8): not translated, it uses Minecraft's `NativeImage`.
        // ETFPlayerTexture.deletePixels(5): not translated, it uses Minecraft's `NativeImage`.
        pub fn get_skin_pixel_colour_to_number(color: i32) -> i32 {
            return match color {
                -16776961 => 3,
                -16760705 => 5,
                -16744449 => 7,
                -16711936 => 4,
                -14483457 => 8,
                -12362096 => 666,
                -65536 => 6,
                -65281 => 1,
                -256 => 2,
                _ => color,
            };
        }
        pub fn get_skin_number_to_pixel_colour(color: i32) -> i32 {
            return match color {
                1 => -65281,
                2 => -256,
                3 => -16776961,
                4 => -16711936,
                5 => -16760705,
                6 => -65536,
                7 => -16744449,
                8 => -14483457,
                666 => -12362096,
                _ => color,
            };
        }
        // ETFPlayerTexture.stripAlphaInclusive(5): not translated, it uses Minecraft's `NativeImage`.
        // ETFPlayerTexture.isCorrectObjectForThisSkin(1): not translated, its class isn't translated (it uses NoseType, which isn't translated (its constants carry values)).
        // ETFPlayerTexture.getBaseTextureIdentifierOrNullForVanilla(1): not translated, its class isn't translated (it uses NoseType, which isn't translated (its constants carry values)).
        // ETFPlayerTexture.getBaseTextureIdentifierOrNullForVanilla(1): not translated, its class isn't translated (it uses NoseType, which isn't translated (its constants carry values)).
        // ETFPlayerTexture.getBaseHeadTextureIdentifierOrNullForVanilla(0): not translated, its class isn't translated (it uses NoseType, which isn't translated (its constants carry values)).
        // ETFPlayerTexture.getBaseTextureEmissiveIdentifierOrNullForNone(0): not translated, its class isn't translated (it uses NoseType, which isn't translated (its constants carry values)).
        // ETFPlayerTexture.canUseFeaturesForThisPlayer(0): not translated, its class isn't translated (it uses NoseType, which isn't translated (its constants carry values)).
        // ETFPlayerTexture.skinFailed(1): not translated, its class isn't translated (it uses NoseType, which isn't translated (its constants carry values)).
        // ETFPlayerTexture.skinFailed(2): not translated, its class isn't translated (it uses NoseType, which isn't translated (its constants carry values)).
        // ETFPlayerTexture.checkTexture(1): not translated, its class isn't translated (it uses NoseType, which isn't translated (its constants carry values)).
        // ETFPlayerTexture.changeSkinToThisForTool(1): not translated, its class isn't translated (it uses NoseType, which isn't translated (its constants carry values)).
    }

    /// `PropertiesRandomProvider` (traben/entity_texture_features/features/property_reading/PropertiesRandomProvider.java)
    pub mod properties_random_provider {
        use super::*;
        // The class itself isn't translated: it implements `ETFVariantSuffixProvider`. Its static parts are below.
        // PropertiesRandomProvider.PropertiesRandomProvider(2): not translated, its class isn't translated (it implements `ETFVariantSuffixProvider`).
        // PropertiesRandomProvider.of(3): not translated, it uses PropertiesRandomProvider, which isn't translated (it implements `ETFVariantSuffixProvider`).
        // PropertiesRandomProvider.getAllValidPropertyObjects(3): not translated, it uses RandomPropertyRule, which isn't translated (it uses RandomProperty, which isn't translated (it uses UUIDBoolean, which isn't translated (it extends `ETFLruCache`))).
        // PropertiesRandomProvider.getSeedOffset(2): not translated, parses something that isn't a string.
        // PropertiesRandomProvider.validateRuleNumbers(2): not translated, it calls ETFUtils2.logWarn, which isn't translated.
        pub fn get_case_numbers(prop_ids: &JSet<String>) -> Vec<i32> {
            let mut found_rule_numbers: JSet<i32> = JSet::new();
            for str in prop_ids.iter() {
                let split: Vec<String> = j_split(str, ".");
                if ((split.len() as i32) >= 2) && (!(split[1].trim().is_empty())) {
                    let possible_rule_number: String = { static RE: LazyLock<JRegex> = LazyLock::new(|| JRegex::new("\\D")); &*RE }.replace_all(&split[1], "");
                    if !(possible_rule_number.trim().is_empty()) {
                        match possible_rule_number.parse::<i32>() {
                            Ok(parsed_0) => {
                                found_rule_numbers.insert(parsed_0);
                            }
                            Err(_) => {
                            }
                        }
                    }
                }
            }
            return { let mut v: Vec<_> = found_rule_numbers.iter().cloned().collect(); v.sort_by(|a: &_, b: &_| (*a).cmp(&*b)); v.into_iter() }.collect::<Vec<_>>();
        }
        // PropertiesRandomProvider.getSuffixes(3): not translated, it calls `Arrays` (java.util).toString.
        // PropertiesRandomProvider.getWeights(2): not translated, it calls SimpleIntegerArrayProperty.getGenericIntegerSplitWithRanges, which isn't translated.
        // PropertiesRandomProvider.setOnMeetsRuleHook(1): not translated, its class isn't translated (it implements `ETFVariantSuffixProvider`).
        // PropertiesRandomProvider.getPackName(0): not translated, its class isn't translated (it implements `ETFVariantSuffixProvider`).
        // PropertiesRandomProvider.isHigherPackThan(1): not translated, its class isn't translated (it implements `ETFVariantSuffixProvider`).
        // PropertiesRandomProvider.entityCanUpdate(1): not translated, its class isn't translated (it implements `ETFVariantSuffixProvider`).
        // PropertiesRandomProvider.getAllSuffixes(0): not translated, its class isn't translated (it implements `ETFVariantSuffixProvider`).
        // PropertiesRandomProvider.size(0): not translated, its class isn't translated (it implements `ETFVariantSuffixProvider`).
        // PropertiesRandomProvider.getSuffixForETFEntity(1): not translated, its class isn't translated (it implements `ETFVariantSuffixProvider`).
        // PropertiesRandomProvider.testEntityAgainstRules(2): not translated, its class isn't translated (it implements `ETFVariantSuffixProvider`).
    }

    /// `AngryProperty` (traben/entity_texture_features/features/property_reading/properties/etf_properties/AngryProperty.java)
    pub mod angry_property {
        use super::*;
        // The class itself isn't translated: it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties). Its static parts are below.
        // AngryProperty.AngryProperty(2): not translated, its class isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // AngryProperty.getPropertyOrNull(2): not translated, it uses AngryProperty, which isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // AngryProperty.getValueFromEntity(1): not translated, its class isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // AngryProperty.getPropertyIds(0): not translated, its class isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
    }

    /// `BiomeTagProperty` (traben/entity_texture_features/features/property_reading/properties/etf_properties/BiomeTagProperty.java)
    pub mod biome_tag_property {
        use super::*;
        // The class itself isn't translated: it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties). Its static parts are below.
        // BiomeTagProperty.BiomeTagProperty(2): not translated, its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)).
        // BiomeTagProperty.getPropertyOrNull(2): not translated, it uses BiomeTagProperty, which isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)).
        // BiomeTagProperty.testEntityInternal(1): not translated, its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)).
        // BiomeTagProperty.fail(0): not translated, its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)).
        // BiomeTagProperty.getPropertyIds(0): not translated, its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)).
        // BiomeTagProperty.getPrintableRuleInfo(0): not translated, its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)).
    }

    /// `BlockAboveProperty` (traben/entity_texture_features/features/property_reading/properties/etf_properties/BlockAboveProperty.java)
    pub mod block_above_property {
        use super::*;
        // The class itself isn't translated: it extends `BlocksProperty` (traben.entity_texture_features.features.property_reading.properties.optifine_properties). Its static parts are below.
        // BlockAboveProperty.BlockAboveProperty(3): not translated, its class isn't translated (it extends `BlocksProperty` (traben.entity_texture_features.features.property_reading.properties.optifine_properties)).
        // BlockAboveProperty.getPropertyOrNull(2): not translated, it uses BlocksProperty, which isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // BlockAboveProperty.getTestingBlocks(1): not translated, its class isn't translated (it extends `BlocksProperty` (traben.entity_texture_features.features.property_reading.properties.optifine_properties)).
        // BlockAboveProperty.getPropertyIds(0): not translated, its class isn't translated (it extends `BlocksProperty` (traben.entity_texture_features.features.property_reading.properties.optifine_properties)).
    }

    /// `BlockAboveSolidProperty` (traben/entity_texture_features/features/property_reading/properties/etf_properties/BlockAboveSolidProperty.java)
    pub mod block_above_solid_property {
        use super::*;
        // The class itself isn't translated: it extends `BlocksProperty` (traben.entity_texture_features.features.property_reading.properties.optifine_properties). Its static parts are below.
        // BlockAboveSolidProperty.BlockAboveSolidProperty(3): not translated, its class isn't translated (it extends `BlocksProperty` (traben.entity_texture_features.features.property_reading.properties.optifine_properties)).
        // BlockAboveSolidProperty.getPropertyOrNull(2): not translated, it uses BlocksProperty, which isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // BlockAboveSolidProperty.getTestingBlocks(1): not translated, its class isn't translated (it extends `BlocksProperty` (traben.entity_texture_features.features.property_reading.properties.optifine_properties)).
        // BlockAboveSolidProperty.getPropertyIds(0): not translated, its class isn't translated (it extends `BlocksProperty` (traben.entity_texture_features.features.property_reading.properties.optifine_properties)).
    }

    /// `BlockBelowProperty` (traben/entity_texture_features/features/property_reading/properties/etf_properties/BlockBelowProperty.java)
    pub mod block_below_property {
        use super::*;
        // The class itself isn't translated: it extends `BlocksProperty` (traben.entity_texture_features.features.property_reading.properties.optifine_properties). Its static parts are below.
        // BlockBelowProperty.BlockBelowProperty(3): not translated, its class isn't translated (it extends `BlocksProperty` (traben.entity_texture_features.features.property_reading.properties.optifine_properties)).
        // BlockBelowProperty.getPropertyOrNull(2): not translated, it uses BlocksProperty, which isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // BlockBelowProperty.getTestingBlocks(1): not translated, its class isn't translated (it extends `BlocksProperty` (traben.entity_texture_features.features.property_reading.properties.optifine_properties)).
        // BlockBelowProperty.getPropertyIds(0): not translated, its class isn't translated (it extends `BlocksProperty` (traben.entity_texture_features.features.property_reading.properties.optifine_properties)).
    }

    /// `BlockBelowSolidProperty` (traben/entity_texture_features/features/property_reading/properties/etf_properties/BlockBelowSolidProperty.java)
    pub mod block_below_solid_property {
        use super::*;
        // The class itself isn't translated: it extends `BlocksProperty` (traben.entity_texture_features.features.property_reading.properties.optifine_properties). Its static parts are below.
        // BlockBelowSolidProperty.BlockBelowSolidProperty(3): not translated, its class isn't translated (it extends `BlocksProperty` (traben.entity_texture_features.features.property_reading.properties.optifine_properties)).
        // BlockBelowSolidProperty.getPropertyOrNull(2): not translated, it uses BlocksProperty, which isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // BlockBelowSolidProperty.getTestingBlocks(1): not translated, its class isn't translated (it extends `BlocksProperty` (traben.entity_texture_features.features.property_reading.properties.optifine_properties)).
        // BlockBelowSolidProperty.getPropertyIds(0): not translated, its class isn't translated (it extends `BlocksProperty` (traben.entity_texture_features.features.property_reading.properties.optifine_properties)).
    }

    /// `BlockSpawnedProperty` (traben/entity_texture_features/features/property_reading/properties/etf_properties/BlockSpawnedProperty.java)
    pub mod block_spawned_property {
        use super::*;
        // The class itself isn't translated: it extends `BlocksProperty` (traben.entity_texture_features.features.property_reading.properties.optifine_properties). Its static parts are below.
        // BlockSpawnedProperty.BlockSpawnedProperty(3): not translated, its class isn't translated (it extends `BlocksProperty` (traben.entity_texture_features.features.property_reading.properties.optifine_properties)).
        // BlockSpawnedProperty.getPropertyOrNull(2): not translated, it uses BlocksProperty, which isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // BlockSpawnedProperty.getPropertyIds(0): not translated, its class isn't translated (it extends `BlocksProperty` (traben.entity_texture_features.features.property_reading.properties.optifine_properties)).
    }

    /// `ChargedCreeperProperty` (traben/entity_texture_features/features/property_reading/properties/etf_properties/ChargedCreeperProperty.java)
    pub mod charged_creeper_property {
        use super::*;
        // The class itself isn't translated: it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties). Its static parts are below.
        // ChargedCreeperProperty.ChargedCreeperProperty(2): not translated, its class isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // ChargedCreeperProperty.getPropertyOrNull(2): not translated, it uses ChargedCreeperProperty, which isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // ChargedCreeperProperty.getValueFromEntity(1): not translated, its class isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // ChargedCreeperProperty.getPropertyIds(0): not translated, its class isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
    }

    /// `ClientPlayerProperty` (traben/entity_texture_features/features/property_reading/properties/etf_properties/ClientPlayerProperty.java)
    pub mod client_player_property {
        use super::*;
        // The class itself isn't translated: it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties). Its static parts are below.
        // ClientPlayerProperty.ClientPlayerProperty(2): not translated, its class isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // ClientPlayerProperty.getPropertyOrNull(2): not translated, it uses ClientPlayerProperty, which isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // ClientPlayerProperty.getValueFromEntity(1): not translated, its class isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // ClientPlayerProperty.getPropertyIds(0): not translated, its class isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
    }

    /// `CreativeProperty` (traben/entity_texture_features/features/property_reading/properties/etf_properties/CreativeProperty.java)
    pub mod creative_property {
        use super::*;
        // The class itself isn't translated: it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties). Its static parts are below.
        // CreativeProperty.CreativeProperty(2): not translated, its class isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // CreativeProperty.getPropertyOrNull(2): not translated, it uses CreativeProperty, which isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // CreativeProperty.getValueFromEntity(1): not translated, its class isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // CreativeProperty.getPropertyIds(0): not translated, its class isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
    }

    /// `DimensionProperty` (traben/entity_texture_features/features/property_reading/properties/etf_properties/DimensionProperty.java)
    pub mod dimension_property {
        use super::*;
        // The class itself isn't translated: it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties). Its static parts are below.
        // DimensionProperty.DimensionProperty(1): not translated, its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // DimensionProperty.getPropertyOrNull(2): not translated, it uses DimensionProperty, which isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // DimensionProperty.getValueFromEntity(1): not translated, its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // DimensionProperty.getPropertyIds(0): not translated, its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // DimensionProperty.shouldForceLowerCaseCheck(0): not translated, its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
    }

    /// `DistanceToPlayerProperty` (traben/entity_texture_features/features/property_reading/properties/etf_properties/DistanceToPlayerProperty.java)
    pub mod distance_to_player_property {
        use super::*;
        // The class itself isn't translated: it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties). Its static parts are below.
        // DistanceToPlayerProperty.DistanceToPlayerProperty(2): not translated, its class isn't translated (it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // DistanceToPlayerProperty.getPropertyOrNull(2): not translated, it uses DistanceToPlayerProperty, which isn't translated (it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // DistanceToPlayerProperty.getRangeValueFromEntity(1): not translated, its class isn't translated (it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // DistanceToPlayerProperty.getPropertyIds(0): not translated, its class isn't translated (it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
    }

    /// `ClientGameModeProperty` (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/ClientGameModeProperty.java)
    pub mod client_game_mode_property {
        use super::*;
        // The class itself isn't translated: it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties). Its static parts are below.
        // ClientGameModeProperty.ClientGameModeProperty(2): not translated, its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // ClientGameModeProperty.getPropertyOrNull(2): not translated, it uses ClientGameModeProperty, which isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // ClientGameModeProperty.getPropertyIds(0): not translated, its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // ClientGameModeProperty.getValueFromEntity(1): not translated, its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
    }

    /// `DifficultyProperty` (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/DifficultyProperty.java)
    pub mod difficulty_property {
        use super::*;
        // The class itself isn't translated: it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties). Its static parts are below.
        // DifficultyProperty.DifficultyProperty(2): not translated, its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // DifficultyProperty.getPropertyOrNull(2): not translated, it uses DifficultyProperty, which isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // DifficultyProperty.getPropertyIds(0): not translated, its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // DifficultyProperty.getValueFromEntity(1): not translated, its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
    }

    /// `HardcoreProperty` (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/HardcoreProperty.java)
    pub mod hardcore_property {
        use super::*;
        // The class itself isn't translated: it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties). Its static parts are below.
        // HardcoreProperty.HardcoreProperty(2): not translated, its class isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // HardcoreProperty.getPropertyOrNull(2): not translated, it uses HardcoreProperty, which isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // HardcoreProperty.getValueFromEntity(1): not translated, its class isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // HardcoreProperty.getPropertyIds(0): not translated, its class isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
    }

    /// `HourProperty` (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/HourProperty.java)
    pub mod hour_property {
        use super::*;
        // The class itself isn't translated: it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties). Its static parts are below.
        // HourProperty.HourProperty(2): not translated, its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // HourProperty.getPropertyOrNull(2): not translated, it uses HourProperty, which isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // HourProperty.getPropertyIds(0): not translated, its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // HourProperty.getValueFromEntity(1): not translated, its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
    }

    /// `LanguageProperty` (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/LanguageProperty.java)
    pub mod language_property {
        use super::*;
        // The class itself isn't translated: it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties). Its static parts are below.
        // LanguageProperty.LanguageProperty(2): not translated, its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // LanguageProperty.getPropertyOrNull(2): not translated, it uses LanguageProperty, which isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // LanguageProperty.getValueFromEntity(1): not translated, its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // LanguageProperty.getPropertyIds(0): not translated, its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // LanguageProperty.shouldForceLowerCaseCheck(0): not translated, its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
    }

    /// `MinecraftVersionProperty` (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/MinecraftVersionProperty.java)
    pub mod minecraft_version_property {
        use super::*;
        // The class itself isn't translated: it extends `SemVerRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties). Its static parts are below.
        // MinecraftVersionProperty.MinecraftVersionProperty(2): not translated, its class isn't translated (it extends `SemVerRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // MinecraftVersionProperty.getPropertyOrNull(2): not translated, it uses MinecraftVersionProperty, which isn't translated (it extends `SemVerRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // MinecraftVersionProperty.getRangeValueFromEntity(1): not translated, its class isn't translated (it extends `SemVerRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // MinecraftVersionProperty.getPropertyIds(0): not translated, its class isn't translated (it extends `SemVerRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
    }

    /// `MinuteProperty` (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/MinuteProperty.java)
    pub mod minute_property {
        use super::*;
        // The class itself isn't translated: it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties). Its static parts are below.
        // MinuteProperty.MinuteProperty(2): not translated, its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // MinuteProperty.getPropertyOrNull(2): not translated, it uses MinuteProperty, which isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // MinuteProperty.getPropertyIds(0): not translated, its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // MinuteProperty.getValueFromEntity(1): not translated, its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
    }

    /// `ModLoadedProperty` (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/ModLoadedProperty.java)
    pub mod mod_loaded_property {
        use super::*;
        // The class itself isn't translated: it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties). Its static parts are below.
        // ModLoadedProperty.ModLoadedProperty(2): not translated, its body couldn't be read: an assert.
        // ModLoadedProperty.getPropertyOrNull(2): not translated, it uses ModLoadedProperty, which isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // ModLoadedProperty.getValueFromEntity(1): not translated, its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // ModLoadedProperty.testEntity(2): not translated, its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // ModLoadedProperty.getPropertyIds(0): not translated, its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // ModLoadedProperty.shouldForceLowerCaseCheck(0): not translated, its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
    }

    /// `MonthDayProperty` (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/MonthDayProperty.java)
    pub mod month_day_property {
        use super::*;
        // The class itself isn't translated: it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties). Its static parts are below.
        // MonthDayProperty.MonthDayProperty(2): not translated, its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // MonthDayProperty.getPropertyOrNull(2): not translated, it uses MonthDayProperty, which isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // MonthDayProperty.getPropertyIds(0): not translated, its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // MonthDayProperty.getValueFromEntity(1): not translated, its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
    }

    /// `MonthProperty` (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/MonthProperty.java)
    pub mod month_property {
        use super::*;
        // The class itself isn't translated: it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties). Its static parts are below.
        // MonthProperty.MonthProperty(2): not translated, its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // MonthProperty.getPropertyOrNull(2): not translated, it uses MonthProperty, which isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // MonthProperty.getPropertyIds(0): not translated, its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // MonthProperty.getValueFromEntity(1): not translated, its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
    }

    /// `NBTVehicleProperty` (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/NBTVehicleProperty.java)
    pub mod nbt_vehicle_property {
        use super::*;
        // The class itself isn't translated: it extends `NBTProperty` (traben.entity_texture_features.features.property_reading.properties.optifine_properties). Its static parts are below.
        // NBTVehicleProperty.NBTVehicleProperty(3): not translated, its class isn't translated (it extends `NBTProperty` (traben.entity_texture_features.features.property_reading.properties.optifine_properties)).
        // NBTVehicleProperty.getPropertyOrNull(2): not translated, it uses NBTProperty, which isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)).
        // NBTVehicleProperty.getEntityNBT(1): not translated, its class isn't translated (it extends `NBTProperty` (traben.entity_texture_features.features.property_reading.properties.optifine_properties)).
    }

    /// `RegionalDifficultyProperty` (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/RegionalDifficultyProperty.java)
    pub mod regional_difficulty_property {
        use super::*;
        // The class itself isn't translated: it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties). Its static parts are below.
        // RegionalDifficultyProperty.RegionalDifficultyProperty(2): not translated, its class isn't translated (it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // RegionalDifficultyProperty.getPropertyOrNull(2): not translated, it uses RegionalDifficultyProperty, which isn't translated (it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // RegionalDifficultyProperty.getPropertyIds(0): not translated, its class isn't translated (it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // RegionalDifficultyProperty.getRangeValueFromEntity(1): not translated, its class isn't translated (it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
    }

    /// `ResourcePackLoadedProperty` (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/ResourcePackLoadedProperty.java)
    pub mod resource_pack_loaded_property {
        use super::*;
        // The class itself isn't translated: it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties). Its static parts are below.
        // ResourcePackLoadedProperty.ResourcePackLoadedProperty(2): not translated, its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // ResourcePackLoadedProperty.getPropertyOrNull(2): not translated, it uses ResourcePackLoadedProperty, which isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // ResourcePackLoadedProperty.getValueFromEntity(1): not translated, its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // ResourcePackLoadedProperty.testEntity(2): not translated, its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // ResourcePackLoadedProperty.getPropertyIds(0): not translated, its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // ResourcePackLoadedProperty.shouldForceLowerCaseCheck(0): not translated, its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
    }

    /// `SecondProperty` (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/SecondProperty.java)
    pub mod second_property {
        use super::*;
        // The class itself isn't translated: it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties). Its static parts are below.
        // SecondProperty.SecondProperty(2): not translated, its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // SecondProperty.getPropertyOrNull(2): not translated, it uses SecondProperty, which isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // SecondProperty.getPropertyIds(0): not translated, its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // SecondProperty.getValueFromEntity(1): not translated, its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
    }

    /// `ShadersEnabledProperty` (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/ShadersEnabledProperty.java)
    pub mod shaders_enabled_property {
        use super::*;
        // The class itself isn't translated: it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties). Its static parts are below.
        // ShadersEnabledProperty.ShadersEnabledProperty(2): not translated, its class isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // ShadersEnabledProperty.getPropertyOrNull(2): not translated, it uses ShadersEnabledProperty, which isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // ShadersEnabledProperty.getValueFromEntity(1): not translated, its class isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // ShadersEnabledProperty.getPropertyIds(0): not translated, its class isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
    }

    /// `WeekDayProperty` (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/WeekDayProperty.java)
    pub mod week_day_property {
        use super::*;
        // The class itself isn't translated: it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties). Its static parts are below.
        // WeekDayProperty.WeekDayProperty(2): not translated, its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // WeekDayProperty.getPropertyOrNull(2): not translated, it uses WeekDayProperty, which isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // WeekDayProperty.getPropertyIds(0): not translated, its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // WeekDayProperty.getValueFromEntity(1): not translated, its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
    }

    /// `YearDayProperty` (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/YearDayProperty.java)
    pub mod year_day_property {
        use super::*;
        // The class itself isn't translated: it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties). Its static parts are below.
        // YearDayProperty.YearDayProperty(2): not translated, its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // YearDayProperty.getPropertyOrNull(2): not translated, it uses YearDayProperty, which isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // YearDayProperty.getPropertyIds(0): not translated, its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // YearDayProperty.getValueFromEntity(1): not translated, its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
    }

    /// `YearProperty` (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/YearProperty.java)
    pub mod year_property {
        use super::*;
        // The class itself isn't translated: it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties). Its static parts are below.
        // YearProperty.YearProperty(2): not translated, its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // YearProperty.getPropertyOrNull(2): not translated, it uses YearProperty, which isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // YearProperty.getPropertyIds(0): not translated, its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // YearProperty.getValueFromEntity(1): not translated, its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
    }

    /// `ItemProperty` (traben/entity_texture_features/features/property_reading/properties/etf_properties/ItemProperty.java)
    pub mod item_property {
        use super::*;
        // The class itself isn't translated: it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties). Its static parts are below.
        // ItemProperty.ItemProperty(2): not translated, its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // ItemProperty.getPropertyOrNull(2): not translated, it uses ItemProperty, which isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // ItemProperty.shouldForceLowerCaseCheck(0): not translated, its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // ItemProperty.testEntityInternal(1): not translated, its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // ItemProperty.getValueFromEntity(1): not translated, its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // ItemProperty.getPropertyIds(0): not translated, its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
    }

    /// `JumpProperty` (traben/entity_texture_features/features/property_reading/properties/etf_properties/JumpProperty.java)
    pub mod jump_property {
        use super::*;
        // The class itself isn't translated: it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties). Its static parts are below.
        // JumpProperty.JumpProperty(2): not translated, its class isn't translated (it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // JumpProperty.getPropertyOrNull(2): not translated, it uses JumpProperty, which isn't translated (it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // JumpProperty.getRangeValueFromEntity(1): not translated, its class isn't translated (it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // JumpProperty.getPropertyIds(0): not translated, its class isn't translated (it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
    }

    /// `LightProperty` (traben/entity_texture_features/features/property_reading/properties/etf_properties/LightProperty.java)
    pub mod light_property {
        use super::*;
        // The class itself isn't translated: it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties). Its static parts are below.
        // LightProperty.LightProperty(2): not translated, its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // LightProperty.getPropertyOrNull(2): not translated, it uses LightProperty, which isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // LightProperty.getPropertyIds(0): not translated, its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // LightProperty.getValueFromEntity(1): not translated, its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
    }

    /// `LlamaInventoryProperty` (traben/entity_texture_features/features/property_reading/properties/etf_properties/LlamaInventoryProperty.java)
    pub mod llama_inventory_property {
        use super::*;
        // The class itself isn't translated: it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties). Its static parts are below.
        // LlamaInventoryProperty.LlamaInventoryProperty(2): not translated, its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // LlamaInventoryProperty.getPropertyOrNull(2): not translated, it uses LlamaInventoryProperty, which isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // LlamaInventoryProperty.getPropertyIds(0): not translated, its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // LlamaInventoryProperty.getValueFromEntity(1): not translated, its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
    }

    /// `MaxHealthProperty` (traben/entity_texture_features/features/property_reading/properties/etf_properties/MaxHealthProperty.java)
    pub mod max_health_property {
        use super::*;
        // The class itself isn't translated: it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties). Its static parts are below.
        // MaxHealthProperty.MaxHealthProperty(2): not translated, its class isn't translated (it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // MaxHealthProperty.getPropertyOrNull(2): not translated, it uses MaxHealthProperty, which isn't translated (it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // MaxHealthProperty.getRangeValueFromEntity(1): not translated, its class isn't translated (it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // MaxHealthProperty.getPropertyIds(0): not translated, its class isn't translated (it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
    }

    /// `MovingProperty` (traben/entity_texture_features/features/property_reading/properties/etf_properties/MovingProperty.java)
    pub mod moving_property {
        use super::*;
        // The class itself isn't translated: it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties). Its static parts are below.
        // MovingProperty.MovingProperty(2): not translated, its class isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // MovingProperty.getPropertyOrNull(2): not translated, it uses MovingProperty, which isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // MovingProperty.getValueFromEntity(1): not translated, its class isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // MovingProperty.getPropertyIds(0): not translated, its class isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
    }

    /// `NBTClientProperty` (traben/entity_texture_features/features/property_reading/properties/etf_properties/NBTClientProperty.java)
    pub mod nbt_client_property {
        use super::*;
        // The class itself isn't translated: it extends `NBTProperty` (traben.entity_texture_features.features.property_reading.properties.optifine_properties). Its static parts are below.
        // NBTClientProperty.NBTClientProperty(3): not translated, its class isn't translated (it extends `NBTProperty` (traben.entity_texture_features.features.property_reading.properties.optifine_properties)).
        // NBTClientProperty.getPropertyOrNull(2): not translated, it uses NBTProperty, which isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)).
        // NBTClientProperty.getEntityNBT(1): not translated, its class isn't translated (it extends `NBTProperty` (traben.entity_texture_features.features.property_reading.properties.optifine_properties)).
    }

    /// `PandaGeneProperty` (traben/entity_texture_features/features/property_reading/properties/etf_properties/PandaGeneProperty.java)
    pub mod panda_gene_property {
        use super::*;
        // The class itself isn't translated: it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties). Its static parts are below.
        // PandaGeneProperty.PandaGeneProperty(2): not translated, its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // PandaGeneProperty.getPropertyOrNull(2): not translated, it uses PandaGeneProperty, which isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // PandaGeneProperty.shouldForceLowerCaseCheck(0): not translated, its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // PandaGeneProperty.getValueFromEntity(1): not translated, its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // PandaGeneProperty.getPropertyIds(0): not translated, its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
    }

    /// `PlayerCreatedProperty` (traben/entity_texture_features/features/property_reading/properties/etf_properties/PlayerCreatedProperty.java)
    pub mod player_created_property {
        use super::*;
        // The class itself isn't translated: it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties). Its static parts are below.
        // PlayerCreatedProperty.PlayerCreatedProperty(2): not translated, its class isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // PlayerCreatedProperty.getPropertyOrNull(2): not translated, it uses PlayerCreatedProperty, which isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // PlayerCreatedProperty.getValueFromEntity(1): not translated, its class isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // PlayerCreatedProperty.getPropertyIds(0): not translated, its class isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
    }

    /// `ScreamingGoatProperty` (traben/entity_texture_features/features/property_reading/properties/etf_properties/ScreamingGoatProperty.java)
    pub mod screaming_goat_property {
        use super::*;
        // The class itself isn't translated: it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties). Its static parts are below.
        // ScreamingGoatProperty.ScreamingGoatProperty(2): not translated, its class isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // ScreamingGoatProperty.getPropertyOrNull(2): not translated, it uses ScreamingGoatProperty, which isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // ScreamingGoatProperty.getValueFromEntity(1): not translated, its class isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // ScreamingGoatProperty.getPropertyIds(0): not translated, its class isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
    }

    /// `SpawnerProperty` (traben/entity_texture_features/features/property_reading/properties/etf_properties/SpawnerProperty.java)
    pub mod spawner_property {
        use super::*;
        // The class itself isn't translated: it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties). Its static parts are below.
        // SpawnerProperty.SpawnerProperty(2): not translated, its class isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // SpawnerProperty.getPropertyOrNull(2): not translated, it uses SpawnerProperty, which isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // SpawnerProperty.getValueFromEntity(1): not translated, its class isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // SpawnerProperty.getPropertyIds(0): not translated, its class isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
    }

    /// `SpeedProperty` (traben/entity_texture_features/features/property_reading/properties/etf_properties/SpeedProperty.java)
    pub mod speed_property {
        use super::*;
        // The class itself isn't translated: it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties). Its static parts are below.
        // SpeedProperty.SpeedProperty(2): not translated, its class isn't translated (it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // SpeedProperty.getPropertyOrNull(2): not translated, it uses SpeedProperty, which isn't translated (it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // SpeedProperty.getRangeValueFromEntity(1): not translated, its class isn't translated (it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // SpeedProperty.getPropertyIds(0): not translated, its class isn't translated (it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
    }

    /// `TeammateProperty` (traben/entity_texture_features/features/property_reading/properties/etf_properties/TeammateProperty.java)
    pub mod teammate_property {
        use super::*;
        // The class itself isn't translated: it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties). Its static parts are below.
        // TeammateProperty.TeammateProperty(2): not translated, its class isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // TeammateProperty.getPropertyOrNull(2): not translated, it uses TeammateProperty, which isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // TeammateProperty.getValueFromEntity(1): not translated, its class isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // TeammateProperty.getPropertyIds(0): not translated, its class isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
    }

    /// `TeamProperty` (traben/entity_texture_features/features/property_reading/properties/etf_properties/TeamProperty.java)
    pub mod team_property {
        use super::*;
        // The class itself isn't translated: it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties). Its static parts are below.
        // TeamProperty.TeamProperty(2): not translated, its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // TeamProperty.getPropertyOrNull(2): not translated, it uses TeamProperty, which isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // TeamProperty.getValueFromEntity(1): not translated, its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // TeamProperty.getPropertyIds(0): not translated, its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // TeamProperty.shouldForceLowerCaseCheck(0): not translated, its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
    }

    /// `TemperatureProperty` (traben/entity_texture_features/features/property_reading/properties/etf_properties/TemperatureProperty.java)
    pub mod temperature_property {
        use super::*;
        // The class itself isn't translated: it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties). Its static parts are below.
        // TemperatureProperty.TemperatureProperty(2): not translated, its class isn't translated (it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // TemperatureProperty.getPropertyOrNull(2): not translated, it uses TemperatureProperty, which isn't translated (it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // TemperatureProperty.getRangeValueFromEntity(1): not translated, its class isn't translated (it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // TemperatureProperty.getPropertyIds(0): not translated, its class isn't translated (it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
    }

    /// `TextureRuleIndexProperty` (traben/entity_texture_features/features/property_reading/properties/etf_properties/TextureRuleIndexProperty.java)
    pub mod texture_rule_index_property {
        use super::*;
        // The class itself isn't translated: it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties). Its static parts are below.
        // TextureRuleIndexProperty.TextureRuleIndexProperty(2): not translated, its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // TextureRuleIndexProperty.getPropertyOrNull(2): not translated, it uses TextureRuleIndexProperty, which isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // TextureRuleIndexProperty.getPropertyIds(0): not translated, its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // TextureRuleIndexProperty.getValueFromEntity(1): not translated, its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
    }

    /// `TextureSuffixProperty` (traben/entity_texture_features/features/property_reading/properties/etf_properties/TextureSuffixProperty.java)
    pub mod texture_suffix_property {
        use super::*;
        // The class itself isn't translated: it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties). Its static parts are below.
        // TextureSuffixProperty.TextureSuffixProperty(2): not translated, its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // TextureSuffixProperty.getPropertyOrNull(2): not translated, it uses TextureSuffixProperty, which isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // TextureSuffixProperty.getPropertyIds(0): not translated, its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // TextureSuffixProperty.getValueFromEntity(1): not translated, its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
    }

    /// `VariantProperty` (traben/entity_texture_features/features/property_reading/properties/etf_properties/VariantProperty.java)
    pub mod variant_property {
        use super::*;
        // The class itself isn't translated: it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties). Its static parts are below.
        // VariantProperty.VariantProperty(1): not translated, its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // VariantProperty.getPropertyOrNull(2): not translated, it uses VariantProperty, which isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // VariantProperty.shouldForceLowerCaseCheck(0): not translated, its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // VariantProperty.getValueFromEntity(1): not translated, its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // VariantProperty.getPropertyIds(0): not translated, its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
    }

    /// `BooleanProperty` (traben/entity_texture_features/features/property_reading/properties/generic_properties/BooleanProperty.java)
    pub mod boolean_property {
        use super::*;
        // The class itself isn't translated: it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties). Its static parts are below.
        // BooleanProperty.BooleanProperty(1): not translated, its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)).
        // BooleanProperty.getGenericBooleanThatCanNull(3): not translated, it calls ETFUtils2.logWarn, which isn't translated.
        // BooleanProperty.testEntityInternal(1): not translated, its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)).
        // BooleanProperty.getValueFromEntity(1): not translated, it has no body.
        // BooleanProperty.getPrintableRuleInfo(0): not translated, its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)).
    }

    /// `FloatRangeFromStringArrayProperty` (traben/entity_texture_features/features/property_reading/properties/generic_properties/FloatRangeFromStringArrayProperty.java)
    pub mod float_range_from_string_array_property {
        use super::*;
        // The class itself isn't translated: it extends `NumberRangeFromStringArrayProperty`. Its static parts are below.
        // FloatRangeFromStringArrayProperty.FloatRangeFromStringArrayProperty(1): not translated, its class isn't translated (it extends `NumberRangeFromStringArrayProperty`).
        // FloatRangeFromStringArrayProperty.getRangeTesterFromString(1): not translated, its class isn't translated (it extends `NumberRangeFromStringArrayProperty`).
    }

    /// `LongRangeFromStringArrayProperty` (traben/entity_texture_features/features/property_reading/properties/generic_properties/LongRangeFromStringArrayProperty.java)
    pub mod long_range_from_string_array_property {
        use super::*;
        // The class itself isn't translated: it extends `NumberRangeFromStringArrayProperty`. Its static parts are below.
        // LongRangeFromStringArrayProperty.LongRangeFromStringArrayProperty(1): not translated, its class isn't translated (it extends `NumberRangeFromStringArrayProperty`).
        // LongRangeFromStringArrayProperty.getRangeTesterFromString(1): not translated, its class isn't translated (it extends `NumberRangeFromStringArrayProperty`).
    }

    /// `NumberRangeFromStringArrayProperty` (traben/entity_texture_features/features/property_reading/properties/generic_properties/NumberRangeFromStringArrayProperty.java)
    pub mod number_range_from_string_array_property {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // NumberRangeFromStringArrayProperty.NumberRangeFromStringArrayProperty(1): not translated, its class isn't translated (it's generic).
        // NumberRangeFromStringArrayProperty.testEntityInternal(1): not translated, its class isn't translated (it's generic).
        // NumberRangeFromStringArrayProperty.getRangeValueFromEntity(1): not translated, it has no body.
        // NumberRangeFromStringArrayProperty.getRangeTesterFromString(1): not translated, it has no body.
        // NumberRangeFromStringArrayProperty.getPropertyIds(0): not translated, it has no body.
        // NumberRangeFromStringArrayProperty.getPrintableRuleInfo(0): not translated, its class isn't translated (it's generic).
    }

    /// `NumberRangeFromStringArrayProperty.RangeTester` (traben/entity_texture_features/features/property_reading/properties/generic_properties/NumberRangeFromStringArrayProperty.java)
    pub mod number_range_from_string_array_property_range_tester {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // NumberRangeFromStringArrayProperty.RangeTester.isValueWithinRangeOrEqual(1): not translated, it has no body.
    }

    /// `SemVerRangeFromStringArrayProperty` (traben/entity_texture_features/features/property_reading/properties/generic_properties/SemVerRangeFromStringArrayProperty.java)
    pub mod sem_ver_range_from_string_array_property {
        use super::*;
        // The class itself isn't translated: it extends `NumberRangeFromStringArrayProperty`. Its static parts are below.
        // SemVerRangeFromStringArrayProperty.SemVerRangeFromStringArrayProperty(1): not translated, its class isn't translated (it extends `NumberRangeFromStringArrayProperty`).
        // SemVerRangeFromStringArrayProperty.getRangeTesterFromString(1): not translated, its class isn't translated (it extends `NumberRangeFromStringArrayProperty`).
    }

    /// `SemVerRangeFromStringArrayProperty.SemVerNumber` (traben/entity_texture_features/features/property_reading/properties/generic_properties/SemVerRangeFromStringArrayProperty.java)
    pub mod sem_ver_range_from_string_array_property_sem_ver_number {
        use super::*;
        // The class itself isn't translated: it extends `Number`. Its static parts are below.
        // SemVerRangeFromStringArrayProperty.SemVerNumber.SemVerNumber(1): not translated, its class isn't translated (it extends `Number`).
        pub fn parse(string: &str) -> i32 {
            match string.parse::<i32>() {
                Ok(parsed_0) => {
                    return parsed_0;
                }
                Err(_) => {
                    return 0;
                }
            }
            #[allow(unreachable_code)]
            unreachable!()
        }
        // SemVerRangeFromStringArrayProperty.SemVerNumber.sameAs(1): not translated, its class isn't translated (it extends `Number`).
        // SemVerRangeFromStringArrayProperty.SemVerNumber.betweenInclusive(2): not translated, its class isn't translated (it extends `Number`).
        // SemVerRangeFromStringArrayProperty.SemVerNumber.largerThanOrEqual(1): not translated, its class isn't translated (it extends `Number`).
        // SemVerRangeFromStringArrayProperty.SemVerNumber.smallerThanOrEqual(1): not translated, its class isn't translated (it extends `Number`).
        // SemVerRangeFromStringArrayProperty.SemVerNumber.largerThan(1): not translated, its class isn't translated (it extends `Number`).
        // SemVerRangeFromStringArrayProperty.SemVerNumber.smallerThan(1): not translated, its class isn't translated (it extends `Number`).
        // SemVerRangeFromStringArrayProperty.SemVerNumber.intValue(0): not translated, its class isn't translated (it extends `Number`).
        // SemVerRangeFromStringArrayProperty.SemVerNumber.longValue(0): not translated, its class isn't translated (it extends `Number`).
        // SemVerRangeFromStringArrayProperty.SemVerNumber.floatValue(0): not translated, its class isn't translated (it extends `Number`).
        // SemVerRangeFromStringArrayProperty.SemVerNumber.doubleValue(0): not translated, its class isn't translated (it extends `Number`).
    }

    /// `SimpleIntegerArrayProperty` (traben/entity_texture_features/features/property_reading/properties/generic_properties/SimpleIntegerArrayProperty.java)
    pub mod simple_integer_array_property {
        use super::*;
        // The class itself isn't translated: it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties). Its static parts are below.
        pub static RANGE_PATTERN: LazyLock<JRegex> = LazyLock::new(|| JRegex::new("(\\d+|-\\d+)-(\\d+|-\\d+)"));
        // SimpleIntegerArrayProperty.SimpleIntegerArrayProperty(1): not translated, its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)).
        // SimpleIntegerArrayProperty.getGenericIntegerSplitWithRanges(3): not translated, `null` (Rust has no null: this needs an Option).
        // SimpleIntegerArrayProperty.getIntRange(1): not translated, it calls `Integer`.parseInt.
        // SimpleIntegerArrayProperty.testEntityInternal(1): not translated, its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)).
        // SimpleIntegerArrayProperty.getValueFromEntity(1): not translated, it has no body.
        // SimpleIntegerArrayProperty.getPrintableRuleInfo(0): not translated, its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)).
    }

    /// `SimpleIntegerArrayProperty.IntRange` (traben/entity_texture_features/features/property_reading/properties/generic_properties/SimpleIntegerArrayProperty.java)
    pub mod simple_integer_array_property_int_range {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct IntRange {
            pub lower: i32,
            pub higher: i32,
        }
        impl IntRange {
            pub fn new(left: i32, right: i32) -> Self {
                let mut s = Self { lower: 0i32, higher: 0i32 };
                if left > right {
                    s.higher = left;
                    s.lower = right;
                } else {
                    s.higher = right;
                    s.lower = left;
                }
                s
            }
            pub fn is_within_range(&self, value: i32) -> bool {
                return (value >= self.get_lower()) && (value <= self.get_higher());
            }
            // SimpleIntegerArrayProperty.IntRange.getAllWithinRangeAsList(0): not translated, List.toArray.
            pub fn get_lower(&self) -> i32 {
                return self.lower;
            }
            pub fn get_higher(&self) -> i32 {
                return self.higher;
            }
        }
    }

    /// `StringArrayOrRegexProperty` (traben/entity_texture_features/features/property_reading/properties/generic_properties/StringArrayOrRegexProperty.java)
    pub mod string_array_or_regex_property {
        use super::*;
        // The class itself isn't translated: it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties). Its static parts are below.
        pub static GROUP_BY_QUOTATION_PATTERN: LazyLock<JRegex> = LazyLock::new(|| JRegex::new("([^\"]\\S*|\".+?\")\\s*"));
        // StringArrayOrRegexProperty.StringArrayOrRegexProperty(1): not translated, its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)).
        // StringArrayOrRegexProperty.getStringMatcher_Regex_Pattern_List_Single(1): not translated, it uses RegexAndPatternPropertyMatcher, which isn't translated (it's an interface).
        // StringArrayOrRegexProperty.testArray(1): not translated, its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)).
        // StringArrayOrRegexProperty.testEntityInternal(1): not translated, its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)).
        // StringArrayOrRegexProperty.shouldForceLowerCaseCheck(0): not translated, it has no body.
        // StringArrayOrRegexProperty.getValueFromEntity(1): not translated, it has no body.
        // StringArrayOrRegexProperty.getPrintableRuleInfo(0): not translated, its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)).
    }

    /// `StringArrayOrRegexProperty.RegexAndPatternPropertyMatcher` (traben/entity_texture_features/features/property_reading/properties/generic_properties/StringArrayOrRegexProperty.java)
    pub mod string_array_or_regex_property_regex_and_pattern_property_matcher {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // StringArrayOrRegexProperty.RegexAndPatternPropertyMatcher.testString(1): not translated, it has no body.
    }

    /// `BabyProperty` (traben/entity_texture_features/features/property_reading/properties/optifine_properties/BabyProperty.java)
    pub mod baby_property {
        use super::*;
        // The class itself isn't translated: it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties). Its static parts are below.
        // BabyProperty.BabyProperty(2): not translated, its class isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // BabyProperty.getPropertyOrNull(2): not translated, it uses BabyProperty, which isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // BabyProperty.getValueFromEntity(1): not translated, its class isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // BabyProperty.getPropertyIds(0): not translated, its class isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
    }

    /// `BiomeProperty` (traben/entity_texture_features/features/property_reading/properties/optifine_properties/BiomeProperty.java)
    pub mod biome_property {
        use super::*;
        // The class itself isn't translated: it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties). Its static parts are below.
        // BiomeProperty.BiomeProperty(1): not translated, its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // BiomeProperty.getPropertyOrNull(2): not translated, it uses BiomeProperty, which isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // BiomeProperty.shouldForceLowerCaseCheck(0): not translated, its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // BiomeProperty.getValueFromEntity(1): not translated, its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // BiomeProperty.getPropertyIds(0): not translated, its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
    }

    /// `BlocksProperty` (traben/entity_texture_features/features/property_reading/properties/optifine_properties/BlocksProperty.java)
    pub mod blocks_property {
        use super::*;
        // The class itself isn't translated: it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties). Its static parts are below.
        // BlocksProperty.BlocksProperty(3): not translated, its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // BlocksProperty.getPropertyOrNull(2): not translated, it uses BlocksProperty, which isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // BlocksProperty.getFromStateBlockNameOnly(1): not translated, its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // BlocksProperty.getFromStateBlockNameWithStateData(1): not translated, its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // BlocksProperty.testBlocks(1): not translated, its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // BlocksProperty.shouldForceLowerCaseCheck(0): not translated, its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // BlocksProperty.getTestingBlocks(1): not translated, its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // BlocksProperty.testEntityInternal(1): not translated, its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // BlocksProperty.getValueFromEntity(1): not translated, its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // BlocksProperty.getPropertyIds(0): not translated, its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
    }

    /// `ColorProperty` (traben/entity_texture_features/features/property_reading/properties/optifine_properties/ColorProperty.java)
    pub mod color_property {
        use super::*;
        // The class itself isn't translated: it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties). Its static parts are below.
        // ColorProperty.ColorProperty(2): not translated, its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // ColorProperty.getPropertyOrNull(2): not translated, it uses ColorProperty, which isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // ColorProperty.shouldForceLowerCaseCheck(0): not translated, its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // ColorProperty.getValueFromEntity(1): not translated, its body couldn't be read: expected `:` but found `wolf` (line 72).
        // ColorProperty.getPropertyIds(0): not translated, its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
    }

    /// `HealthProperty` (traben/entity_texture_features/features/property_reading/properties/optifine_properties/HealthProperty.java)
    pub mod health_property {
        use super::*;
        // The class itself isn't translated: it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties). Its static parts are below.
        // HealthProperty.HealthProperty(2): not translated, its class isn't translated (it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // HealthProperty.getPropertyOrNull(2): not translated, it uses HealthProperty, which isn't translated (it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // HealthProperty.getRangeValueFromEntity(1): not translated, its class isn't translated (it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // HealthProperty.getPropertyIds(0): not translated, its class isn't translated (it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
    }

    /// `HeightProperty` (traben/entity_texture_features/features/property_reading/properties/optifine_properties/HeightProperty.java)
    pub mod height_property {
        use super::*;
        // The class itself isn't translated: it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties). Its static parts are below.
        // HeightProperty.HeightProperty(2): not translated, its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // HeightProperty.getPropertyOrNull(2): not translated, it uses HeightProperty, which isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // HeightProperty.getPropertyIds(0): not translated, its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // HeightProperty.getValueFromEntity(1): not translated, its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
    }

    /// `MoonPhaseProperty` (traben/entity_texture_features/features/property_reading/properties/optifine_properties/MoonPhaseProperty.java)
    pub mod moon_phase_property {
        use super::*;
        // The class itself isn't translated: it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties). Its static parts are below.
        // MoonPhaseProperty.MoonPhaseProperty(2): not translated, its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // MoonPhaseProperty.getPropertyOrNull(2): not translated, it uses MoonPhaseProperty, which isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // MoonPhaseProperty.getPropertyIds(0): not translated, its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // MoonPhaseProperty.getValueFromEntity(1): not translated, its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
    }

    /// `NameProperty` (traben/entity_texture_features/features/property_reading/properties/optifine_properties/NameProperty.java)
    pub mod name_property {
        use super::*;
        // The class itself isn't translated: it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties). Its static parts are below.
        // NameProperty.NameProperty(1): not translated, its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // NameProperty.getPropertyOrNull(2): not translated, it uses NameProperty, which isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // NameProperty.shouldForceLowerCaseCheck(0): not translated, its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // NameProperty.getValueFromEntity(1): not translated, its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // NameProperty.getPropertyIds(0): not translated, its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
    }

    /// `NBTProperty` (traben/entity_texture_features/features/property_reading/properties/optifine_properties/NBTProperty.java)
    pub mod nbt_property {
        use super::*;
        // The class itself isn't translated: it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties). Its static parts are below.
        // INTENTIONAL_FAILURE: not translated, it uses Minecraft's `CompoundTag`.
        pub static CRASH_MESSAGES: LazyLock<JCell<JSet<String>>> = LazyLock::new(|| JCell::new(JSet::new()));
        // NBTProperty.NBTProperty(3): not translated, its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)).
        // NBTProperty.getPropertyOrNull(2): not translated, it uses NBTProperty, which isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)).
        // NBTProperty.isStringValidInt(1): not translated, it calls `Integer`.parseInt.
        // NBTProperty.getEntityNBT(1): not translated, its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)).
        // NBTProperty.testEntityInternal(1): not translated, its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)).
        // NBTProperty.testAllNBTCases(1): not translated, its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)).
        // NBTProperty.getAsString(1): not translated, it uses Minecraft's `Tag`.
        // NBTProperty.getAsNumber(1): not translated, it uses `Number`.
        // NBTProperty.formatNbtPretty(1): not translated, its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)).
        // NBTProperty.findNBTElements(2): not translated, its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)).
        // NBTProperty.findByIteration(3): not translated, its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)).
        // NBTProperty.handleListInstruction(3): not translated, its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)).
        // NBTProperty.getPropertyIds(0): not translated, its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)).
        // NBTProperty.getPrintableRuleInfo(0): not translated, its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)).
    }

    /// `NBTProperty.NBTTester` (traben/entity_texture_features/features/property_reading/properties/optifine_properties/NBTProperty.java)
    pub mod nbt_property_nbt_tester {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Tag`. Its static parts are below.
        // NBTProperty.NBTTester.NBTTester(4): not translated, its class isn't translated (it uses Minecraft's `Tag`).
        // NBTProperty.NBTTester.of(2): not translated, it uses NBTTester, which isn't translated (it uses Minecraft's `Tag`).
        // NBTProperty.NBTTester.inverts(0): not translated, its class isn't translated (it uses Minecraft's `Tag`).
        // NBTProperty.NBTTester.tester(0): not translated, its class isn't translated (it uses Minecraft's `Tag`).
        // NBTProperty.NBTTester.wantsBlank(0): not translated, its class isn't translated (it uses Minecraft's `Tag`).
        // NBTProperty.NBTTester.print(0): not translated, its class isn't translated (it uses Minecraft's `Tag`).
    }

    /// `ProfessionProperty` (traben/entity_texture_features/features/property_reading/properties/optifine_properties/ProfessionProperty.java)
    pub mod profession_property {
        use super::*;
        // The class itself isn't translated: it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties). Its static parts are below.
        // ProfessionProperty.ProfessionProperty(2): not translated, its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // ProfessionProperty.getPropertyOrNull(2): not translated, it uses ProfessionProperty, which isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // ProfessionProperty.testEntityInternal(1): not translated, its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // ProfessionProperty.shouldForceLowerCaseCheck(0): not translated, its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // ProfessionProperty.getValueFromEntity(1): not translated, its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // ProfessionProperty.getPropertyIds(0): not translated, its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
    }

    /// `SizeProperty` (traben/entity_texture_features/features/property_reading/properties/optifine_properties/SizeProperty.java)
    pub mod size_property {
        use super::*;
        // The class itself isn't translated: it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties). Its static parts are below.
        // SizeProperty.SizeProperty(2): not translated, its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // SizeProperty.getPropertyOrNull(2): not translated, it uses SizeProperty, which isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // SizeProperty.getPropertyIds(0): not translated, its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // SizeProperty.getValueFromEntity(1): not translated, its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
    }

    /// `TimeOfDayProperty` (traben/entity_texture_features/features/property_reading/properties/optifine_properties/TimeOfDayProperty.java)
    pub mod time_of_day_property {
        use super::*;
        // The class itself isn't translated: it extends `LongRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties). Its static parts are below.
        // TimeOfDayProperty.TimeOfDayProperty(2): not translated, its class isn't translated (it extends `LongRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // TimeOfDayProperty.getPropertyOrNull(2): not translated, it uses TimeOfDayProperty, which isn't translated (it extends `LongRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // TimeOfDayProperty.getRangeValueFromEntity(1): not translated, its class isn't translated (it extends `LongRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // TimeOfDayProperty.getPropertyIds(0): not translated, its class isn't translated (it extends `LongRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
    }

    /// `WeatherProperty` (traben/entity_texture_features/features/property_reading/properties/optifine_properties/WeatherProperty.java)
    pub mod weather_property {
        use super::*;
        // The class itself isn't translated: it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties). Its static parts are below.
        // WeatherProperty.WeatherProperty(2): not translated, its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // WeatherProperty.getPropertyOrNull(2): not translated, it uses WeatherProperty, which isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // WeatherProperty.shouldForceLowerCaseCheck(0): not translated, its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // WeatherProperty.getValueFromEntity(1): not translated, its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // WeatherProperty.getPropertyIds(0): not translated, its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
    }

    /// `RandomProperties` (traben/entity_texture_features/features/property_reading/properties/RandomProperties.java)
    pub mod random_properties {
        use super::*;
        // REGISTERED_PROPERTIES: not translated, it uses RandomPropertyFactory, which isn't translated (it's an interface).
        #[derive(Clone, Debug)]
        pub struct RandomProperties {
        }
        impl RandomProperties {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // RandomProperties.forEachProperty(1): not translated, it uses RandomPropertyFactory, which isn't translated (it's an interface).
        // RandomProperties.register(1): not translated, it uses RandomPropertyFactory, which isn't translated (it's an interface).
        // RandomProperties.getAllRegisteredRandomPropertiesOfIndex(2): not translated, it uses RandomProperties.REGISTERED_PROPERTIES, which isn't translated (it uses RandomPropertyFactory, which isn't translated (it's an interface)).
    }

    /// `RandomProperties.RandomPropertyFactory` (traben/entity_texture_features/features/property_reading/properties/RandomProperties.java)
    pub mod random_properties_random_property_factory {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // RandomProperties.RandomPropertyFactory.of(3): not translated, it uses RandomPropertyFactory, which isn't translated (it's an interface).
        // RandomProperties.RandomPropertyFactory.of(4): not translated, its body couldn't be read: an anonymous class.
        // RandomProperties.RandomPropertyFactory.getPropertyOrNull(2): not translated, it has no body.
        // RandomProperties.RandomPropertyFactory.getPropertyId(0): not translated, it has no body.
        // RandomProperties.RandomPropertyFactory.updatesOverTime(0): not translated, it has no body.
        // RandomProperties.RandomPropertyFactory.getExplanationTranslationKey(0): not translated, it has no body.
    }

    /// `RandomProperty` (traben/entity_texture_features/features/property_reading/properties/RandomProperty.java)
    pub mod random_property {
        use super::*;
        // The class itself isn't translated: it uses UUIDBoolean, which isn't translated (it extends `ETFLruCache`). Its static parts are below.
        pub fn read_properties_or_throw(properties: JProps, property_num: i32, property_id: &[String]) -> String {
            if (property_id.len() as i32) == 0 {
                panic!("{}", "[ETF] readPropertiesOrThrow() was given empty property id's");
            }
            for id in property_id.iter() {
                if properties.contains_key(&(format!("{}.{}", id, property_num))) {
                    let data_from_props: String = (((properties.get_property(&(format!("{}.{}", id, property_num)))).expect("null")).trim()).to_string();
                    if !(data_from_props.trim().is_empty()) {
                        return data_from_props.to_string();
                    }
                }
            }
            panic!("{}", format!("failed to read property [{}]", property_id[0]));
        }
        // RandomProperty.testEntity(2): not translated, its class isn't translated (it uses UUIDBoolean, which isn't translated (it extends `ETFLruCache`)).
        // RandomProperty.testEntityInternal(1): not translated, it has no body.
        // RandomProperty.canPropertyUpdate(0): not translated, its class isn't translated (it uses UUIDBoolean, which isn't translated (it extends `ETFLruCache`)).
        // RandomProperty.setCanUpdate(1): not translated, its class isn't translated (it uses UUIDBoolean, which isn't translated (it extends `ETFLruCache`)).
        // RandomProperty.getPropertyIds(0): not translated, it has no body.
        // RandomProperty.getPropertyId(0): not translated, its class isn't translated (it uses UUIDBoolean, which isn't translated (it extends `ETFLruCache`)).
        // RandomProperty.getPrintableRuleInfo(0): not translated, it has no body.
        // RandomProperty.toString(0): not translated, its class isn't translated (it uses UUIDBoolean, which isn't translated (it extends `ETFLruCache`)).
        // RandomProperty.cacheEntityInitialResult(1): not translated, its class isn't translated (it uses UUIDBoolean, which isn't translated (it extends `ETFLruCache`)).
    }

    /// `RandomProperty.RandomPropertyException` (traben/entity_texture_features/features/property_reading/properties/RandomProperty.java)
    pub mod random_property_random_property_exception {
        use super::*;
        // The class itself isn't translated: it extends `Exception`. Its static parts are below.
        // RandomProperty.RandomPropertyException.RandomPropertyException(1): not translated, its class isn't translated (it extends `Exception`).
    }

    /// `RandomPropertyRule` (traben/entity_texture_features/features/property_reading/RandomPropertyRule.java)
    pub mod random_property_rule {
        use super::*;
        // The class itself isn't translated: it uses RandomProperty, which isn't translated (it uses UUIDBoolean, which isn't translated (it extends `ETFLruCache`)). Its static parts are below.
        // DEFAULT_RETURN: not translated, its value couldn't be read: an anonymous class.
        // RandomPropertyRule.RandomPropertyRule(0): not translated, its class isn't translated (it uses RandomProperty, which isn't translated (it uses UUIDBoolean, which isn't translated (it extends `ETFLruCache`))).
        // RandomPropertyRule.isAlwaysMet(0): not translated, its class isn't translated (it uses RandomProperty, which isn't translated (it uses UUIDBoolean, which isn't translated (it extends `ETFLruCache`))).
        // RandomPropertyRule.RandomPropertyRule(7): not translated, its class isn't translated (it uses RandomProperty, which isn't translated (it uses UUIDBoolean, which isn't translated (it extends `ETFLruCache`))).
        // RandomPropertyRule.getSuffixSet(0): not translated, its class isn't translated (it uses RandomProperty, which isn't translated (it uses UUIDBoolean, which isn't translated (it extends `ETFLruCache`))).
        // RandomPropertyRule.doesEntityMeetConditionsOfThisCase(3): not translated, its class isn't translated (it uses RandomProperty, which isn't translated (it uses UUIDBoolean, which isn't translated (it extends `ETFLruCache`))).
        // RandomPropertyRule.getVariantSuffixFromThisCase(1): not translated, its class isn't translated (it uses RandomProperty, which isn't translated (it uses UUIDBoolean, which isn't translated (it extends `ETFLruCache`))).
        // RandomPropertyRule.getVariantSuffixFromThisCase(1): not translated, its class isn't translated (it uses RandomProperty, which isn't translated (it uses UUIDBoolean, which isn't translated (it extends `ETFLruCache`))).
        // RandomPropertyRule.getSeedFrom(1): not translated, its class isn't translated (it uses RandomProperty, which isn't translated (it uses UUIDBoolean, which isn't translated (it extends `ETFLruCache`))).
        // RandomPropertyRule.cacheEntityInitialResultsOfNonUpdatingProperties(1): not translated, its class isn't translated (it uses RandomProperty, which isn't translated (it uses UUIDBoolean, which isn't translated (it extends `ETFLruCache`))).
    }

    /// `RandomPropertyRule.SeedSource` (traben/entity_texture_features/features/property_reading/RandomPropertyRule.java)
    pub mod random_property_rule_seed_source {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum SeedSource {
            Entity,
            Vehicle,
        }
        impl SeedSource {
            pub const VALUES: [SeedSource; 2] = [SeedSource::Entity, SeedSource::Vehicle, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    SeedSource::Entity => "entity",
                    SeedSource::Vehicle => "vehicle",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "entity" => SeedSource::Entity,
                    "vehicle" => SeedSource::Vehicle,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `TrueRandomProvider` (traben/entity_texture_features/features/property_reading/TrueRandomProvider.java)
    pub mod true_random_provider {
        use super::*;
        // The class itself isn't translated: it implements `ETFVariantSuffixProvider`. Its static parts are below.
        // TrueRandomProvider.TrueRandomProvider(2): not translated, its class isn't translated (it implements `ETFVariantSuffixProvider`).
        // TrueRandomProvider.of(1): not translated, it uses TrueRandomProvider, which isn't translated (it implements `ETFVariantSuffixProvider`).
        // TrueRandomProvider.getPackName(0): not translated, its class isn't translated (it implements `ETFVariantSuffixProvider`).
        // TrueRandomProvider.entityCanUpdate(1): not translated, its class isn't translated (it implements `ETFVariantSuffixProvider`).
        // TrueRandomProvider.getAllSuffixes(0): not translated, its class isn't translated (it implements `ETFVariantSuffixProvider`).
        // TrueRandomProvider.size(0): not translated, its class isn't translated (it implements `ETFVariantSuffixProvider`).
        // TrueRandomProvider.getSuffixForETFEntity(1): not translated, its class isn't translated (it implements `ETFVariantSuffixProvider`).
        // TrueRandomProvider.setRandomSupplier(1): not translated, its class isn't translated (it implements `ETFVariantSuffixProvider`).
    }

    /// `ETFEntityRenderState` (traben/entity_texture_features/features/state/ETFEntityRenderState.java)
    pub mod etf_entity_render_state {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ETFEntityRenderState.uuid(0): not translated, it has no body.
        // ETFEntityRenderState.canRenderBright(0): not translated, it has no body.
        // ETFEntityRenderState.isBlockEntity(0): not translated, it has no body.
        // ETFEntityRenderState.entityType(0): not translated, it has no body.
        // ETFEntityRenderState.world(0): not translated, it has no body.
        // ETFEntityRenderState.blockPos(0): not translated, it has no body.
        // ETFEntityRenderState.optifineId(0): not translated, it has no body.
        // ETFEntityRenderState.optifineVehicleId(0): not translated, it has no body.
        // ETFEntityRenderState.blockY(0): not translated, it has no body.
        // ETFEntityRenderState.nbt(0): not translated, it has no body.
        // ETFEntityRenderState.hasCustomName(0): not translated, it has no body.
        // ETFEntityRenderState.customName(0): not translated, it has no body.
        // ETFEntityRenderState.scoreboardTeam(0): not translated, it has no body.
        // ETFEntityRenderState.itemsEquipped(0): not translated, it has no body.
        // ETFEntityRenderState.handItems(0): not translated, it has no body.
        // ETFEntityRenderState.armorItems(0): not translated, it has no body.
        // ETFEntityRenderState.velocity(0): not translated, it has no body.
        // ETFEntityRenderState.pose(0): not translated, it has no body.
        // ETFEntityRenderState.entityKey(0): not translated, it has no body.
        // ETFEntityRenderState.entity(0): not translated, it has no body.
        // ETFEntityRenderState.isPlayer(0): not translated, it has no body.
        // ETFEntityRenderState.isClientPlayer(0): not translated, it has no body.
        // ETFEntityRenderState.vanillaState(0): not translated, it has no body.
        // ETFEntityRenderState.setVanillaState(1): not translated, it has no body.
        // ETFEntityRenderState.vanillaBlockState(0): not translated, it has no body.
        // ETFEntityRenderState.setVanillaBlockState(1): not translated, it has no body.
        // ETFEntityRenderState.distanceTo(1): not translated, it has no body.
        // ETFEntityRenderState.cacheEntityNBTForState(2): not translated, it has no body.
        // ETFEntityRenderState.preSubmitActivate(2): not translated, its class isn't translated (it's an interface).
        // ETFEntityRenderState.activate(1): not translated, its class isn't translated (it's an interface).
        // ETFEntityRenderState.deactivate(1): not translated, its class isn't translated (it's an interface).
        // ETFEntityRenderState.setEtfRenderStateConstructor(2): not translated, it uses ETFRenderStateInit, which isn't translated (it's an interface).
        // ETFEntityRenderState.forEntity(1): not translated, it uses ETFEntityRenderState, which isn't translated (it's an interface).
        // ETFEntityRenderState.from(1): not translated, it uses ETFEntityRenderState, which isn't translated (it's an interface).
    }

    /// `ETFEntityRenderState.ETFRenderStateInit` (traben/entity_texture_features/features/state/ETFEntityRenderState.java)
    pub mod etf_entity_render_state_etf_render_state_init {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ETFEntityRenderState.ETFRenderStateInit.make(1): not translated, it has no body.
    }

    /// `ETFEntityRenderState.Null` (traben/entity_texture_features/features/state/ETFEntityRenderState.java)
    pub mod etf_entity_render_state_null {
        use super::*;
        // The class itself isn't translated: it implements `ETFEntityRenderState`. Its static parts are below.
        // ETFEntityRenderState.Null.uuid(0): not translated, its class isn't translated (it implements `ETFEntityRenderState`).
        // ETFEntityRenderState.Null.canRenderBright(0): not translated, its class isn't translated (it implements `ETFEntityRenderState`).
        // ETFEntityRenderState.Null.isBlockEntity(0): not translated, its class isn't translated (it implements `ETFEntityRenderState`).
        // ETFEntityRenderState.Null.entityType(0): not translated, its class isn't translated (it implements `ETFEntityRenderState`).
        // ETFEntityRenderState.Null.world(0): not translated, its class isn't translated (it implements `ETFEntityRenderState`).
        // ETFEntityRenderState.Null.blockPos(0): not translated, its class isn't translated (it implements `ETFEntityRenderState`).
        // ETFEntityRenderState.Null.optifineId(0): not translated, its class isn't translated (it implements `ETFEntityRenderState`).
        // ETFEntityRenderState.Null.optifineVehicleId(0): not translated, its class isn't translated (it implements `ETFEntityRenderState`).
        // ETFEntityRenderState.Null.blockY(0): not translated, its class isn't translated (it implements `ETFEntityRenderState`).
        // ETFEntityRenderState.Null.nbt(0): not translated, its class isn't translated (it implements `ETFEntityRenderState`).
        // ETFEntityRenderState.Null.hasCustomName(0): not translated, its class isn't translated (it implements `ETFEntityRenderState`).
        // ETFEntityRenderState.Null.customName(0): not translated, its class isn't translated (it implements `ETFEntityRenderState`).
        // ETFEntityRenderState.Null.scoreboardTeam(0): not translated, its class isn't translated (it implements `ETFEntityRenderState`).
        // ETFEntityRenderState.Null.itemsEquipped(0): not translated, its class isn't translated (it implements `ETFEntityRenderState`).
        // ETFEntityRenderState.Null.handItems(0): not translated, its class isn't translated (it implements `ETFEntityRenderState`).
        // ETFEntityRenderState.Null.armorItems(0): not translated, its class isn't translated (it implements `ETFEntityRenderState`).
        // ETFEntityRenderState.Null.velocity(0): not translated, its class isn't translated (it implements `ETFEntityRenderState`).
        // ETFEntityRenderState.Null.pose(0): not translated, its class isn't translated (it implements `ETFEntityRenderState`).
        // ETFEntityRenderState.Null.entityKey(0): not translated, its class isn't translated (it implements `ETFEntityRenderState`).
        // ETFEntityRenderState.Null.entity(0): not translated, its class isn't translated (it implements `ETFEntityRenderState`).
        // ETFEntityRenderState.Null.isPlayer(0): not translated, its class isn't translated (it implements `ETFEntityRenderState`).
        // ETFEntityRenderState.Null.isClientPlayer(0): not translated, its class isn't translated (it implements `ETFEntityRenderState`).
        // ETFEntityRenderState.Null.vanillaState(0): not translated, its class isn't translated (it implements `ETFEntityRenderState`).
        // ETFEntityRenderState.Null.setVanillaState(1): not translated, its class isn't translated (it implements `ETFEntityRenderState`).
        // ETFEntityRenderState.Null.vanillaBlockState(0): not translated, its class isn't translated (it implements `ETFEntityRenderState`).
        // ETFEntityRenderState.Null.setVanillaBlockState(1): not translated, its class isn't translated (it implements `ETFEntityRenderState`).
        // ETFEntityRenderState.Null.distanceTo(1): not translated, its class isn't translated (it implements `ETFEntityRenderState`).
        // ETFEntityRenderState.Null.cacheEntityNBTForState(2): not translated, its class isn't translated (it implements `ETFEntityRenderState`).
    }

    /// `ETFEntityRenderStateViaReference` (traben/entity_texture_features/features/state/ETFEntityRenderStateViaReference.java)
    pub mod etf_entity_render_state_via_reference {
        use super::*;
        // The class itself isn't translated: it implements `ETFEntityRenderState`. Its static parts are below.
        // ETFEntityRenderStateViaReference.ETFEntityRenderStateViaReference(1): not translated, its class isn't translated (it implements `ETFEntityRenderState`).
        // ETFEntityRenderStateViaReference.entity(0): not translated, its class isn't translated (it implements `ETFEntityRenderState`).
        // ETFEntityRenderStateViaReference.isPlayer(0): not translated, its class isn't translated (it implements `ETFEntityRenderState`).
        // ETFEntityRenderStateViaReference.isClientPlayer(0): not translated, its class isn't translated (it implements `ETFEntityRenderState`).
        // ETFEntityRenderStateViaReference.vanillaState(0): not translated, its class isn't translated (it implements `ETFEntityRenderState`).
        // ETFEntityRenderStateViaReference.setVanillaState(1): not translated, its class isn't translated (it implements `ETFEntityRenderState`).
        // ETFEntityRenderStateViaReference.vanillaBlockState(0): not translated, its class isn't translated (it implements `ETFEntityRenderState`).
        // ETFEntityRenderStateViaReference.setVanillaBlockState(1): not translated, its class isn't translated (it implements `ETFEntityRenderState`).
        // ETFEntityRenderStateViaReference.uuid(0): not translated, its class isn't translated (it implements `ETFEntityRenderState`).
        // ETFEntityRenderStateViaReference.canRenderBright(0): not translated, its class isn't translated (it implements `ETFEntityRenderState`).
        // ETFEntityRenderStateViaReference.isBlockEntity(0): not translated, its class isn't translated (it implements `ETFEntityRenderState`).
        // ETFEntityRenderStateViaReference.entityType(0): not translated, its class isn't translated (it implements `ETFEntityRenderState`).
        // ETFEntityRenderStateViaReference.world(0): not translated, its class isn't translated (it implements `ETFEntityRenderState`).
        // ETFEntityRenderStateViaReference.blockPos(0): not translated, its class isn't translated (it implements `ETFEntityRenderState`).
        // ETFEntityRenderStateViaReference.optifineId(0): not translated, its class isn't translated (it implements `ETFEntityRenderState`).
        // ETFEntityRenderStateViaReference.optifineVehicleId(0): not translated, its class isn't translated (it implements `ETFEntityRenderState`).
        // ETFEntityRenderStateViaReference.blockY(0): not translated, its class isn't translated (it implements `ETFEntityRenderState`).
        // ETFEntityRenderStateViaReference.nbt(0): not translated, its class isn't translated (it implements `ETFEntityRenderState`).
        // ETFEntityRenderStateViaReference.hasCustomName(0): not translated, its class isn't translated (it implements `ETFEntityRenderState`).
        // ETFEntityRenderStateViaReference.customName(0): not translated, its class isn't translated (it implements `ETFEntityRenderState`).
        // ETFEntityRenderStateViaReference.scoreboardTeam(0): not translated, its class isn't translated (it implements `ETFEntityRenderState`).
        // ETFEntityRenderStateViaReference.itemsEquipped(0): not translated, its class isn't translated (it implements `ETFEntityRenderState`).
        // ETFEntityRenderStateViaReference.handItems(0): not translated, its class isn't translated (it implements `ETFEntityRenderState`).
        // ETFEntityRenderStateViaReference.armorItems(0): not translated, its class isn't translated (it implements `ETFEntityRenderState`).
        // ETFEntityRenderStateViaReference.velocity(0): not translated, its class isn't translated (it implements `ETFEntityRenderState`).
        // ETFEntityRenderStateViaReference.pose(0): not translated, its class isn't translated (it implements `ETFEntityRenderState`).
        // ETFEntityRenderStateViaReference.entityKey(0): not translated, its class isn't translated (it implements `ETFEntityRenderState`).
        // ETFEntityRenderStateViaReference.distanceTo(1): not translated, its class isn't translated (it implements `ETFEntityRenderState`).
        // ETFEntityRenderStateViaReference.cacheEntityNBTForState(2): not translated, its class isn't translated (it implements `ETFEntityRenderState`).
    }

    /// `ETFState` (traben/entity_texture_features/features/state/ETFState.java)
    pub mod etf_state {
        use super::*;
        // stateStack: not translated, its value couldn't be read: an anonymous class.
        pub static STACK_VERIFY_HAS_FAILED: JCell<bool> = JCell::new(false);
        pub static CURRENT_MODEL_PART_DEPTH: JCell<i32> = JCell::new(0);
        pub static IS_RENDERING_FEATURES: JCell<bool> = JCell::new(false);
        pub static ONLY_RANDOMIZE_VIA_PROPERTIES_FILES: JCell<bool> = JCell::new(false);
        pub static ALLOW_TEXTURE_PATCHING: JCell<bool> = JCell::new(false);
        // specialPhaseStack: not translated, it uses `Deque` (java.util).
        // renderLayerModifyStack: not translated, it uses `Deque` (java.util).
        #[derive(Clone, Debug)]
        pub struct ETFState {
        }
        impl ETFState {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ETFState.mount(1): not translated, it uses ETFEntityRenderState, which isn't translated (it's an interface).
        // ETFState.unMount(0): not translated, it calls Minecraft's `RenderSystem`.isOnRenderThread.
        // ETFState.mountNone(0): not translated, it calls ETFState.mount, which isn't translated.
        // ETFState.state(0): not translated, it uses ETFEntityRenderState, which isn't translated (it's an interface).
        // ETFState.isStateActive(0): not translated, it calls ETFState.state, which isn't translated.
        // ETFState.clear(0): not translated, it uses ETFEntityRenderState, which isn't translated (it's an interface).
        // ETFState.announceStack(1): not translated, it calls ETF.config, which isn't translated.
        // ETFState.stackVerify(1): not translated, it uses ETFEntityRenderState, which isn't translated (it's an interface).
        // ETFState.stackVerifyEmpty(0): not translated, it uses ETFState.stateStack, which isn't translated (its value couldn't be read: an anonymous class).
        // ETFState.isIsInSpecialRenderOverlayPhase(0): not translated, it calls Minecraft's `RenderSystem`.isOnRenderThread.
        // ETFState.startSpecialRenderOverlayPhase(0): not translated, it calls Minecraft's `RenderSystem`.isOnRenderThread.
        // ETFState.endSpecialRenderOverlayPhase(0): not translated, it calls Minecraft's `RenderSystem`.isOnRenderThread.
        // ETFState.isAllowedToRenderLayerTextureModify(0): not translated, it calls ETF.config, which isn't translated.
        // ETFState.pushRenderLayerModifyState(1): not translated, it calls Minecraft's `RenderSystem`.isOnRenderThread.
        // ETFState.popRenderLayerModifyState(0): not translated, it calls Minecraft's `RenderSystem`.isOnRenderThread.
        // ETFState.modifyRenderLayerIfRequired(1): not translated, it uses Minecraft's `RenderType`.
        // ETFState.insertETFDataIntoVertexConsumer(3): not translated, it uses Minecraft's `RenderType`.
        // ETFState.canRenderInBrightMode(0): not translated, it calls ETFManager.getEmissiveMode, which isn't translated.
    }

    /// `ETFSubmitData` (traben/entity_texture_features/features/state/ETFSubmitData.java)
    pub mod etf_submit_data {
        use super::*;
        // The class itself isn't translated: it uses ETFEntityRenderState, which isn't translated (it's an interface). Its static parts are below.
        pub const POOL_CAPACITY: i32 = 4096;
        // POOL: not translated, it uses ETFSubmitData, which isn't translated (it uses ETFEntityRenderState, which isn't translated (it's an interface)).
        pub static POOL_SIZE: JCell<i32> = JCell::new(0);
        pub static POOL_NEXT: JCell<i32> = JCell::new(0);
        // DATA_IN: not translated, it uses ETFSubmitData, which isn't translated (it uses ETFEntityRenderState, which isn't translated (it's an interface)).
        // DATA_OUT: not translated, it uses ETFSubmitData, which isn't translated (it uses ETFEntityRenderState, which isn't translated (it's an interface)).
        // ETFSubmitData.acquire(0): not translated, it uses ETFSubmitData, which isn't translated (it uses ETFEntityRenderState, which isn't translated (it's an interface)).
        // ETFSubmitData.recycle(0): not translated, it uses ETFSubmitData.POOL, which isn't translated (a ETFSubmitData with no value (null in Java)).
        // ETFSubmitData.from(1): not translated, it uses ETFSubmitData, which isn't translated (it uses ETFEntityRenderState, which isn't translated (it's an interface)).
    }

    /// `ETFSubmitExtension` (traben/entity_texture_features/features/state/ETFSubmitExtension.java)
    pub mod etf_submit_extension {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ETFSubmitExtension.emf$getData(0): not translated, it has no body.
    }

    /// `HoldsETFRenderState` (traben/entity_texture_features/features/state/HoldsETFRenderState.java)
    pub mod holds_etf_render_state {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // HoldsETFRenderState.etf$getState(0): not translated, it has no body.
        // HoldsETFRenderState.etf$initState(1): not translated, it has no body.
    }

    /// `ETFArmorHandler` (traben/entity_texture_features/features/texture_handlers/ETFArmorHandler.java)
    pub mod etf_armor_handler {
        use super::*;
        // The class itself isn't translated: it uses ETFTexture, which isn't translated (it uses ETFSprite, which isn't translated (it uses Minecraft's `TextureAtlasSprite`)). Its static parts are below.
        // ETFArmorHandler.start(0): not translated, its class isn't translated (it uses ETFTexture, which isn't translated (it uses ETFSprite, which isn't translated (it uses Minecraft's `TextureAtlasSprite`))).
        // ETFArmorHandler.end(0): not translated, its class isn't translated (it uses ETFTexture, which isn't translated (it uses ETFSprite, which isn't translated (it uses Minecraft's `TextureAtlasSprite`))).
        // ETFArmorHandler.renderTrimEmissive(3): not translated, it uses Minecraft's `Model`.
        // ETFArmorHandler.setTrim(1): not translated, it calls ETF.config, which isn't translated.
    }

    /// `ETFDirectory` (traben/entity_texture_features/features/texture_handlers/ETFDirectory.java)
    pub mod etf_directory {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // ETFDirectory.ETFDirectory(1): not translated, its class isn't translated (its constants carry values).
        // ETFDirectory.getCache(0): not translated, it uses ETFDirectory, which isn't translated (its constants carry values).
        // ETFDirectory.getDirectoryVersionOf(1): not translated, it uses ETFDirectory, which isn't translated (its constants carry values).
        // ETFDirectory.getDirectoryOf(1): not translated, it uses ETFDirectory, which isn't translated (its constants carry values).
        // ETFDirectory.findDirectoryOf(1): not translated, it uses ETFDirectory, which isn't translated (its constants carry values).
        // ETFDirectory.getIdentifierAsDirectory(2): not translated, it uses ETFDirectory, which isn't translated (its constants carry values).
        // ETFDirectory.doesReplace(0): not translated, its class isn't translated (its constants carry values).
    }

    /// `ETFSprite` (traben/entity_texture_features/features/texture_handlers/ETFSprite.java)
    pub mod etf_sprite {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `TextureAtlasSprite`. Its static parts are below.
        // ETFSprite.ETFSprite(3): not translated, its class isn't translated (it uses Minecraft's `TextureAtlasSprite`).
        // ETFSprite.load(2): not translated, it uses Minecraft's `SpriteContents`.
        // ETFSprite.getEmissive(0): not translated, its class isn't translated (it uses Minecraft's `TextureAtlasSprite`).
        // ETFSprite.isEmissive(0): not translated, its class isn't translated (it uses Minecraft's `TextureAtlasSprite`).
        // ETFSprite.getSpriteVariant(0): not translated, its class isn't translated (it uses Minecraft's `TextureAtlasSprite`).
    }

    /// `ETFTexture` (traben/entity_texture_features/features/texture_handlers/ETFTexture.java)
    pub mod etf_texture {
        use super::*;
        // The class itself isn't translated: it uses ETFSprite, which isn't translated (it uses Minecraft's `TextureAtlasSprite`). Its static parts are below.
        pub const PATCH_NAMESPACE_PREFIX: &str = "etf_patched_";
        // ETFTexture.ETFTexture(1): not translated, its class isn't translated (it uses ETFSprite, which isn't translated (it uses Minecraft's `TextureAtlasSprite`)).
        // ETFTexture.manual(12): not translated, it uses ETFTexture, which isn't translated (it uses ETFSprite, which isn't translated (it uses Minecraft's `TextureAtlasSprite`)).
        // ETFTexture.manual(3): not translated, it uses ETFTexture, which isn't translated (it uses ETFSprite, which isn't translated (it uses Minecraft's `TextureAtlasSprite`)).
        // ETFTexture.ETFTexture(12): not translated, its class isn't translated (it uses ETFSprite, which isn't translated (it uses Minecraft's `TextureAtlasSprite`)).
        // ETFTexture.ETFTexture(2): not translated, its class isn't translated (it uses ETFSprite, which isn't translated (it uses Minecraft's `TextureAtlasSprite`)).
        // ETFTexture.ofUnmodifiable(2): not translated, it uses ETFTexture, which isn't translated (it uses ETFSprite, which isn't translated (it uses Minecraft's `TextureAtlasSprite`)).
        // ETFTexture.patchTextureToRemoveZFightingWithOtherTexture(2): not translated, it uses Minecraft's `NativeImage`.
        // ETFTexture.doesAnimaticaVersionExist(1): not translated, it calls Minecraft's `Minecraft`.getInstance.
        // ETFTexture.setupBlinking(0): not translated, its class isn't translated (it uses ETFSprite, which isn't translated (it uses Minecraft's `TextureAtlasSprite`)).
        // ETFTexture.exists(0): not translated, its class isn't translated (it uses ETFSprite, which isn't translated (it uses Minecraft's `TextureAtlasSprite`)).
        // ETFTexture.setupEmissives(0): not translated, its class isn't translated (it uses ETFSprite, which isn't translated (it uses Minecraft's `TextureAtlasSprite`)).
        // ETFTexture.setupEnchants(0): not translated, its class isn't translated (it uses ETFSprite, which isn't translated (it uses Minecraft's `TextureAtlasSprite`)).
        // ETFTexture.getResourceOrModifyForTrims(1): not translated, its class isn't translated (it uses ETFSprite, which isn't translated (it uses Minecraft's `TextureAtlasSprite`)).
        // ETFTexture.getTextureIdentifier(1): not translated, its class isn't translated (it uses ETFSprite, which isn't translated (it uses Minecraft's `TextureAtlasSprite`)).
        // ETFTexture.getBlinkingIdentifier(1): not translated, its class isn't translated (it uses ETFSprite, which isn't translated (it uses Minecraft's `TextureAtlasSprite`)).
        // ETFTexture.setBlink(2): not translated, its class isn't translated (it uses ETFSprite, which isn't translated (it uses Minecraft's `TextureAtlasSprite`)).
        // ETFTexture.setGUIBlink(0): not translated, its class isn't translated (it uses ETFSprite, which isn't translated (it uses Minecraft's `TextureAtlasSprite`)).
        // ETFTexture.isEmissive(0): not translated, its class isn't translated (it uses ETFSprite, which isn't translated (it uses Minecraft's `TextureAtlasSprite`)).
        // ETFTexture.isEnchanted(0): not translated, its class isn't translated (it uses ETFSprite, which isn't translated (it uses Minecraft's `TextureAtlasSprite`)).
        // ETFTexture.canPatch(0): not translated, its class isn't translated (it uses ETFSprite, which isn't translated (it uses Minecraft's `TextureAtlasSprite`)).
        // ETFTexture.doesBlink(0): not translated, its class isn't translated (it uses ETFSprite, which isn't translated (it uses Minecraft's `TextureAtlasSprite`)).
        // ETFTexture.getPaintingSprite(2): not translated, its class isn't translated (it uses ETFSprite, which isn't translated (it uses Minecraft's `TextureAtlasSprite`)).
        // ETFTexture.doesBlink2(0): not translated, its class isn't translated (it uses ETFSprite, which isn't translated (it uses Minecraft's `TextureAtlasSprite`)).
        // ETFTexture.toString(0): not translated, its class isn't translated (it uses ETFSprite, which isn't translated (it uses Minecraft's `TextureAtlasSprite`)).
        // ETFTexture.getEmissiveRenderLayer(1): not translated, its class isn't translated (it uses ETFSprite, which isn't translated (it uses Minecraft's `TextureAtlasSprite`)).
        // ETFTexture.getEmissiveRenderLayer(2): not translated, its class isn't translated (it uses ETFSprite, which isn't translated (it uses Minecraft's `TextureAtlasSprite`)).
        // ETFTexture.getEmissiveVertexConsumerWrapped(2): not translated, its class isn't translated (it uses ETFSprite, which isn't translated (it uses Minecraft's `TextureAtlasSprite`)).
        // ETFTexture.modifyTextureState(1): not translated, its class isn't translated (it uses ETFSprite, which isn't translated (it uses Minecraft's `TextureAtlasSprite`)).
        // ETFTexture.identifierOfCurrentState(0): not translated, its class isn't translated (it uses ETFSprite, which isn't translated (it uses Minecraft's `TextureAtlasSprite`)).
        // ETFTexture.getEmissiveIdentifierOfCurrentState(0): not translated, its class isn't translated (it uses ETFSprite, which isn't translated (it uses Minecraft's `TextureAtlasSprite`)).
        // ETFTexture.getEnchantIdentifierOfCurrentState(0): not translated, its class isn't translated (it uses ETFSprite, which isn't translated (it uses Minecraft's `TextureAtlasSprite`)).
        // ETFTexture.assertPatchedTextures(0): not translated, its class isn't translated (it uses ETFSprite, which isn't translated (it uses Minecraft's `TextureAtlasSprite`)).
    }

    /// `ETFTexture.TextureReturnState` (traben/entity_texture_features/features/texture_handlers/ETFTexture.java)
    pub mod etf_texture_texture_return_state {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum TextureReturnState {
            Normal,
            NormalPatched,
            Blink,
            BlinkPatched,
            Blink2,
            Blink2Patched,
            ApplyPatch,
            ApplyBlink,
            ApplyBlink2,
        }
        impl TextureReturnState {
            pub const VALUES: [TextureReturnState; 9] = [TextureReturnState::Normal, TextureReturnState::NormalPatched, TextureReturnState::Blink, TextureReturnState::BlinkPatched, TextureReturnState::Blink2, TextureReturnState::Blink2Patched, TextureReturnState::ApplyPatch, TextureReturnState::ApplyBlink, TextureReturnState::ApplyBlink2, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    TextureReturnState::Normal => "NORMAL",
                    TextureReturnState::NormalPatched => "NORMAL_PATCHED",
                    TextureReturnState::Blink => "BLINK",
                    TextureReturnState::BlinkPatched => "BLINK_PATCHED",
                    TextureReturnState::Blink2 => "BLINK2",
                    TextureReturnState::Blink2Patched => "BLINK2_PATCHED",
                    TextureReturnState::ApplyPatch => "APPLY_PATCH",
                    TextureReturnState::ApplyBlink => "APPLY_BLINK",
                    TextureReturnState::ApplyBlink2 => "APPLY_BLINK2",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "NORMAL" => TextureReturnState::Normal,
                    "NORMAL_PATCHED" => TextureReturnState::NormalPatched,
                    "BLINK" => TextureReturnState::Blink,
                    "BLINK_PATCHED" => TextureReturnState::BlinkPatched,
                    "BLINK2" => TextureReturnState::Blink2,
                    "BLINK2_PATCHED" => TextureReturnState::Blink2Patched,
                    "APPLY_PATCH" => TextureReturnState::ApplyPatch,
                    "APPLY_BLINK" => TextureReturnState::ApplyBlink,
                    "APPLY_BLINK2" => TextureReturnState::ApplyBlink2,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
        impl TextureReturnState {
            // ETFTexture.TextureReturnState.toString(0): not translated, `this` used as a value.
        }
    }

    /// `ETFTextureVariator` (traben/entity_texture_features/features/texture_handlers/ETFTextureVariator.java)
    pub mod etf_texture_variator {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ETFTextureVariator {
        }
        impl ETFTextureVariator {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // ETFTextureVariator.getVariantOf(1): not translated, it uses ETFTexture, which isn't translated (it uses ETFSprite, which isn't translated (it uses Minecraft's `TextureAtlasSprite`)).
            // ETFTextureVariator.logDebugMessageIfNeeded(2): not translated, it uses ETFTexture, which isn't translated (it uses ETFSprite, which isn't translated (it uses Minecraft's `TextureAtlasSprite`)).
            // ETFTextureVariator.getVanillaVariantDetails(0): not translated, it calls ETFDirectory.getIdentifierAsDirectory, which isn't translated.
            // ETFTextureVariator.getPrintout(0): not translated, it has no body.
            // ETFTextureVariator.getVariantOfInternal(1): not translated, it has no body.
            // ETFTextureVariator.getVanillaIdentifier(0): not translated, it has no body.
        }
        // ETFTextureVariator.of(1): not translated, it uses ETFVariantSuffixProvider, which isn't translated (it's an interface).
    }

    /// `ETFTextureVariator.ETFTextureMultiple` (traben/entity_texture_features/features/texture_handlers/ETFTextureVariator.java)
    pub mod etf_texture_variator_etf_texture_multiple {
        use super::*;
        // The class itself isn't translated: it extends `ETFTextureVariator`. Its static parts are below.
        // ETFTextureVariator.ETFTextureMultiple.ETFTextureMultiple(2): not translated, its body couldn't be read: an anonymous class.
        // ETFTextureVariator.ETFTextureMultiple.equals(1): not translated, its class isn't translated (it extends `ETFTextureVariator`).
        // ETFTextureVariator.ETFTextureMultiple.hashCode(0): not translated, its class isn't translated (it extends `ETFTextureVariator`).
        // ETFTextureVariator.ETFTextureMultiple.getPrintout(0): not translated, its class isn't translated (it extends `ETFTextureVariator`).
        // ETFTextureVariator.ETFTextureMultiple.checkIfShouldExpireEntity(2): not translated, its class isn't translated (it extends `ETFTextureVariator`).
        // ETFTextureVariator.ETFTextureMultiple.getVariantOfInternal(1): not translated, its class isn't translated (it extends `ETFTextureVariator`).
        // ETFTextureVariator.ETFTextureMultiple.determineTextureSource(1): not translated, its class isn't translated (it extends `ETFTextureVariator`).
        // ETFTextureVariator.ETFTextureMultiple.determineNewSuffix(2): not translated, its class isn't translated (it extends `ETFTextureVariator`).
        // ETFTextureVariator.ETFTextureMultiple.getBaseEntitySuffixOrNew(1): not translated, its class isn't translated (it extends `ETFTextureVariator`).
        // ETFTextureVariator.ETFTextureMultiple.getVanillaIdentifier(0): not translated, its class isn't translated (it extends `ETFTextureVariator`).
    }

    /// `ETFTextureVariator.ETFTextureSingleton` (traben/entity_texture_features/features/texture_handlers/ETFTextureVariator.java)
    pub mod etf_texture_variator_etf_texture_singleton {
        use super::*;
        // The class itself isn't translated: it extends `ETFTextureVariator`. Its static parts are below.
        // ETFTextureVariator.ETFTextureSingleton.ETFTextureSingleton(1): not translated, its class isn't translated (it extends `ETFTextureVariator`).
        // ETFTextureVariator.ETFTextureSingleton.texture(0): not translated, its class isn't translated (it extends `ETFTextureVariator`).
        // ETFTextureVariator.ETFTextureSingleton.getVariantOfInternal(1): not translated, its class isn't translated (it extends `ETFTextureVariator`).
        // ETFTextureVariator.ETFTextureSingleton.getVanillaIdentifier(0): not translated, its class isn't translated (it extends `ETFTextureVariator`).
        // ETFTextureVariator.ETFTextureSingleton.getPrintout(0): not translated, its class isn't translated (it extends `ETFTextureVariator`).
    }

    /// `CancelTarget` (traben/entity_texture_features/mixin/CancelTarget.java)
    pub mod cancel_target {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct CancelTarget {
        }
        impl CancelTarget {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `Plugin` (traben/entity_texture_features/mixin/Plugin.java)
    pub mod plugin {
        use super::*;
        // The class itself isn't translated: it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility). Its static parts are below.
        // log: the logger. Log calls become comments.
        // Plugin.onLoad(1): not translated, its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)).
        // Plugin.getRefMapperConfig(0): not translated, its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)).
        // Plugin.shouldApplyMixin(2): not translated, its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)).
        // Plugin.hasClass(1): not translated, its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)).
        // Plugin.acceptTargets(2): not translated, its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)).
        // Plugin.getMixins(0): not translated, its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)).
        // Plugin.preApply(4): not translated, its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)).
        // Plugin.postApply(4): not translated, its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)).
    }

    /// `EntityBooleanLRU` (traben/entity_texture_features/utils/EntityBooleanLRU.java)
    pub mod entity_boolean_lru {
        use super::*;
        // The class itself isn't translated: it extends `ETFLruCache`. Its static parts are below.
    }

    /// `EntityIntLRU` (traben/entity_texture_features/utils/EntityIntLRU.java)
    pub mod entity_int_lru {
        use super::*;
        // The class itself isn't translated: it extends `ETFLruCache`. Its static parts are below.
    }

    /// `ETFEntity` (traben/entity_texture_features/utils/ETFEntity.java)
    pub mod etf_entity {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ETFEntity.etf$canBeBright(0): not translated, it has no body.
        // ETFEntity.etf$isBlockEntity(0): not translated, it has no body.
        // ETFEntity.etf$getType(0): not translated, it has no body.
        // ETFEntity.etf$getUuid(0): not translated, it has no body.
        // ETFEntity.etf$getWorld(0): not translated, it has no body.
        // ETFEntity.etf$getBlockPos(0): not translated, it has no body.
        // ETFEntity.etf$getOptifineId(0): not translated, it has no body.
        // ETFEntity.etf$getOptifineVehicleId(0): not translated, it has no body.
        // ETFEntity.etf$getBlockY(0): not translated, it has no body.
        // ETFEntity.etf$getNbt(0): not translated, it has no body.
        // ETFEntity.etf$hasCustomName(0): not translated, it has no body.
        // ETFEntity.etf$getCustomName(0): not translated, it has no body.
        // ETFEntity.etf$getScoreboardTeam(0): not translated, it has no body.
        // ETFEntity.etf$getItemsEquipped(0): not translated, it has no body.
        // ETFEntity.etf$getHandItems(0): not translated, it has no body.
        // ETFEntity.etf$getArmorItems(0): not translated, it has no body.
        // ETFEntity.etf$distanceTo(1): not translated, it has no body.
        // ETFEntity.etf$getVelocity(0): not translated, it has no body.
        // ETFEntity.etf$getPose(0): not translated, it has no body.
        // ETFEntity.etf$getEntityKey(0): not translated, it has no body.
    }

    /// `ETFLruCache` (traben/entity_texture_features/utils/ETFLruCache.java)
    pub mod etf_lru_cache {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // ETFLruCache.ETFLruCache(0): not translated, its class isn't translated (it's generic).
        // ETFLruCache.ETFLruCache(1): not translated, its class isn't translated (it's generic).
        // ETFLruCache.removeEldestEntry(1): not translated, its class isn't translated (it's generic).
        // ETFLruCache.removeEntryOnly(1): not translated, its class isn't translated (it's generic).
        // ETFLruCache.get(1): not translated, its class isn't translated (it's generic).
        // ETFLruCache.defaultReturnValue(1): not translated, its class isn't translated (it's generic).
    }

    /// `ETFLruCache.UUIDBoolean` (traben/entity_texture_features/utils/ETFLruCache.java)
    pub mod etf_lru_cache_uuid_boolean {
        use super::*;
        // The class itself isn't translated: it extends `ETFLruCache`. Its static parts are below.
        // ETFLruCache.UUIDBoolean.UUIDBoolean(0): not translated, its class isn't translated (it extends `ETFLruCache`).
        // ETFLruCache.UUIDBoolean.UUIDBoolean(1): not translated, its class isn't translated (it extends `ETFLruCache`).
    }

    /// `ETFLruCache.UUIDInteger` (traben/entity_texture_features/utils/ETFLruCache.java)
    pub mod etf_lru_cache_uuid_integer {
        use super::*;
        // The class itself isn't translated: it extends `ETFLruCache`. Its static parts are below.
        // ETFLruCache.UUIDInteger.UUIDInteger(0): not translated, its class isn't translated (it extends `ETFLruCache`).
        // ETFLruCache.UUIDInteger.UUIDInteger(1): not translated, its class isn't translated (it extends `ETFLruCache`).
    }

    /// `ETFRenderLayerWithTexture` (traben/entity_texture_features/utils/ETFRenderLayerWithTexture.java)
    pub mod etf_render_layer_with_texture {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ETFRenderLayerWithTexture.etf$getId(0): not translated, it has no body.
    }

    /// `ETFUtils2` (traben/entity_texture_features/utils/ETFUtils2.java)
    pub mod etf_utils2 {
        use super::*;
        pub const FULL_BRIGHT: i32 = 15728880;
        #[derive(Clone, Debug)]
        pub struct ETFUtils2 {
        }
        impl ETFUtils2 {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ETFUtils2.res(1): not translated, it calls Minecraft's `Identifier`.parse.
        pub fn res_2(namespace: &str, path: &str) -> JIdentifier {
            return JIdentifier::new(&namespace, &path);
        }
        // ETFUtils2.setPixel(4): not translated, it uses Minecraft's `NativeImage`.
        // ETFUtils2.getPixel(3): not translated, it uses Minecraft's `NativeImage`.
        pub fn pack_light(sky: i32, block: i32) -> i32 {
            return (sky << 4) | (block << 20);
        }
        // ETFUtils2.submitModelPart(6): not translated, it uses Minecraft's `SubmitNodeCollector`.
        // ETFUtils2.submitModelPart(7): not translated, it uses Minecraft's `SubmitNodeCollector`.
        // ETFUtils2.submitEmissiveModelPart(4): not translated, it uses Minecraft's `SubmitNodeCollector`.
        // ETFUtils2.submitEnchantedModelPart(5): not translated, it uses Minecraft's `SubmitNodeCollector`.
        pub fn optifine_hashing(mut x: i32) -> i32 {
            x ^= 61 ^ (x >> 16);
            x += x << 3;
            x ^= x >> 4;
            x *= 668265261;
            return x ^ (x >> 15);
        }
        // ETFUtils2.printDebugImage(1): not translated, it uses Minecraft's `NativeImage`.
        // ETFUtils2.getETFVariantNotNullForInjector(1): not translated, it calls ETFState.isStateActive, which isn't translated.
        // ETFUtils2.renderEmissive(3): not translated, it uses ETFTexture, which isn't translated (it uses ETFSprite, which isn't translated (it uses Minecraft's `TextureAtlasSprite`)).
        // ETFUtils2.getFoilBuffer(2): not translated, it uses Minecraft's `VertexConsumer`.
        // ETFUtils2.renderEnchanted(4): not translated, it uses ETFTexture, which isn't translated (it uses ETFSprite, which isn't translated (it uses Minecraft's `TextureAtlasSprite`)).
        // ETFUtils2.addVariantNumberSuffix(2): not translated, it's overloaded with the same number of parameters.
        // ETFUtils2.addVariantNumberSuffix(2): not translated, it's overloaded with the same number of parameters.
        // ETFUtils2.replaceIdentifier(3): not translated, a try/catch (IdentifierException, Exception).
        // ETFUtils2.returnNameOfHighestPackFromTheseMultiple(1): not translated, it calls ETFManager.getInstance, which isn't translated.
        // ETFUtils2.returnNameOfHighestPackFromTheseTwo(2): not translated, String.equals with 1 values.
        // ETFUtils2.readAndReturnPropertiesElseNull(1): not translated, a try-with-resources over something other than a file reader or writer.
        // ETFUtils2.readAndReturnAllLayeredPropertiesElseNull(1): not translated, it calls Minecraft's `Minecraft`.getInstance.
        // ETFUtils2.getNativeImageElseNull(1): not translated, it uses Minecraft's `NativeImage`.
        // ETFUtils2.logMessage(1): not translated, it calls ETFUtils2.logMessage, which isn't translated.
        // ETFUtils2.logMessage(2): not translated, it uses Minecraft's `LocalPlayer`.
        // ETFUtils2.logWarn(1): not translated, it calls ETFUtils2.logWarn, which isn't translated.
        // ETFUtils2.logWarn(2): not translated, it uses Minecraft's `LocalPlayer`.
        // ETFUtils2.logError(1): not translated, it calls ETFUtils2.logError, which isn't translated.
        // ETFUtils2.logError(2): not translated, it uses Minecraft's `LocalPlayer`.
        // ETFUtils2.emptyNativeImage(0): not translated, it uses Minecraft's `NativeImage`.
        // ETFUtils2.emptyNativeImage(2): not translated, it uses Minecraft's `NativeImage`.
        // ETFUtils2.registerNativeImageToIdentifier(2): not translated, it uses Minecraft's `NativeImage`.
        // ETFUtils2.checkModCompatibility(0): not translated, it calls ETFConfigWarnings.getRegisteredWarnings, which isn't translated.
    }

    /// `ETFUtils2.RenderMethodForOverlay` (traben/entity_texture_features/utils/ETFUtils2.java)
    pub mod etf_utils2_render_method_for_overlay {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ETFUtils2.RenderMethodForOverlay.render(2): not translated, it has no body.
    }

    /// `ETFVertexConsumer` (traben/entity_texture_features/utils/ETFVertexConsumer.java)
    pub mod etf_vertex_consumer {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ETFVertexConsumer.etf$getETFTexture(0): not translated, it has no body.
        // ETFVertexConsumer.etf$getProvider(0): not translated, it has no body.
        // ETFVertexConsumer.etf$getRenderLayer(0): not translated, it has no body.
        // ETFVertexConsumer.etf$initETFVertexConsumer(2): not translated, it has no body.
    }

    /// `UEntityTypes` (traben/entity_texture_features/utils/UEntityTypes.java)
    pub mod u_entity_types {
        use super::*;
        // PARROT: not translated, it uses Minecraft's `EntityType`.
        // PLAYER: not translated, it uses Minecraft's `EntityType`.
        #[derive(Clone, Debug)]
        pub struct UEntityTypes {
        }
        impl UEntityTypes {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `URenderTypeToVertexConsumer` (traben/entity_texture_features/utils/URenderTypeToVertexConsumer.java)
    pub mod u_render_type_to_vertex_consumer {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `RenderTypeFeatureRenderer`. Its static parts are below.
        // URenderTypeToVertexConsumer.URenderTypeToVertexConsumer(1): not translated, its class isn't translated (it uses Minecraft's `RenderTypeFeatureRenderer`).
        // URenderTypeToVertexConsumer.getBuffer(1): not translated, its class isn't translated (it uses Minecraft's `RenderTypeFeatureRenderer`).
    }

    /// `UScreen` (traben/entity_texture_features/utils/UScreen.java)
    pub mod u_screen {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct UScreen {
        }
        impl UScreen {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // UScreen.currentScreen(0): not translated, it uses Minecraft's `Screen`.
        // UScreen.setScreen(1): not translated, it uses Minecraft's `Screen`.
    }

    /// `TConfigEntry` (traben/tconfig/gui/entries/TConfigEntry.java)
    pub mod t_config_entry {
        use super::*;
        // The class itself isn't translated: it extends `TConfigEntryForList`. Its static parts are below.
        pub const CHANGED_COLOR: &str = "\u{a7}a";
        // TConfigEntry.TConfigEntry(2): not translated, its class isn't translated (it extends `TConfigEntryForList`).
        // TConfigEntry.TConfigEntry(1): not translated, its class isn't translated (it extends `TConfigEntryForList`).
        // TConfigEntry.getText(0): not translated, its class isn't translated (it extends `TConfigEntryForList`).
        // TConfigEntry.getTooltip(0): not translated, its class isn't translated (it extends `TConfigEntryForList`).
        // TConfigEntry.setEnabled(1): not translated, its class isn't translated (it extends `TConfigEntryForList`).
        // TConfigEntry.hasChangedFromInitial(0): not translated, it has no body.
        // TConfigEntry.saveValuesToConfig(0): not translated, it has no body.
        // TConfigEntry.setValuesToDefault(0): not translated, it has no body.
        // TConfigEntry.resetValuesToInitial(0): not translated, it has no body.
    }

    /// `TConfigEntry.Empty` (traben/tconfig/gui/entries/TConfigEntry.java)
    pub mod t_config_entry_empty {
        use super::*;
        // The class itself isn't translated: it extends `TConfigEntry`. Its static parts are below.
        // TConfigEntry.Empty.Empty(0): not translated, its class isn't translated (it extends `TConfigEntry`).
        // TConfigEntry.Empty.getWidget(4): not translated, its class isn't translated (it extends `TConfigEntry`).
        // TConfigEntry.Empty.hasChangedFromInitial(0): not translated, its class isn't translated (it extends `TConfigEntry`).
        // TConfigEntry.Empty.saveValuesToConfig(0): not translated, its class isn't translated (it extends `TConfigEntry`).
        // TConfigEntry.Empty.setValuesToDefault(0): not translated, its class isn't translated (it extends `TConfigEntry`).
        // TConfigEntry.Empty.resetValuesToInitial(0): not translated, its class isn't translated (it extends `TConfigEntry`).
    }

    /// `TConfigEntryBoolean` (traben/tconfig/gui/entries/TConfigEntryBoolean.java)
    pub mod t_config_entry_boolean {
        use super::*;
        // The class itself isn't translated: it extends `TConfigEntryValue`. Its static parts are below.
        // TConfigEntryBoolean.TConfigEntryBoolean(5): not translated, its class isn't translated (it extends `TConfigEntryValue`).
        // TConfigEntryBoolean.TConfigEntryBoolean(4): not translated, its class isn't translated (it extends `TConfigEntryValue`).
        // TConfigEntryBoolean.setType(1): not translated, its class isn't translated (it extends `TConfigEntryValue`).
        // TConfigEntryBoolean.getValueFromWidget(0): not translated, its class isn't translated (it extends `TConfigEntryValue`).
        // TConfigEntryBoolean.getWidget(4): not translated, its class isn't translated (it extends `TConfigEntryValue`).
        // TConfigEntryBoolean.setWidgetToDefaultValue(0): not translated, its class isn't translated (it extends `TConfigEntryValue`).
        // TConfigEntryBoolean.resetWidgetToInitialValue(0): not translated, its class isn't translated (it extends `TConfigEntryValue`).
    }

    /// `TConfigEntryBoolean.BooleanButtonWidget` (traben/tconfig/gui/entries/TConfigEntryBoolean.java)
    pub mod t_config_entry_boolean_boolean_button_widget {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Plain`. Its static parts are below.
        // TConfigEntryBoolean.BooleanButtonWidget.BooleanButtonWidget(7): not translated, its class isn't translated (it extends Minecraft's `Plain`).
        // TConfigEntryBoolean.BooleanButtonWidget.updateMessage(0): not translated, its class isn't translated (it extends Minecraft's `Plain`).
        // TConfigEntryBoolean.BooleanButtonWidget.onPress(1): not translated, its class isn't translated (it extends Minecraft's `Plain`).
    }

    /// `TConfigEntryBoolean.Type` (traben/tconfig/gui/entries/TConfigEntryBoolean.java)
    pub mod t_config_entry_boolean_type {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // TConfigEntryBoolean.Type.Type(2): not translated, its class isn't translated (its constants carry values).
        // TConfigEntryBoolean.Type.get(1): not translated, its class isn't translated (its constants carry values).
    }

    /// `TConfigEntryCategory` (traben/tconfig/gui/entries/TConfigEntryCategory.java)
    pub mod t_config_entry_category {
        use super::*;
        // The class itself isn't translated: it extends `TConfigEntry`. Its static parts are below.
        // TConfigEntryCategory.TConfigEntryCategory(2): not translated, its class isn't translated (it extends `TConfigEntry`).
        // TConfigEntryCategory.TConfigEntryCategory(1): not translated, its class isn't translated (it extends `TConfigEntry`).
        // TConfigEntryCategory.setAlign(1): not translated, its class isn't translated (it extends `TConfigEntry`).
        // TConfigEntryCategory.getOptions(0): not translated, its class isn't translated (it extends `TConfigEntry`).
        // TConfigEntryCategory.getScreen(0): not translated, its class isn't translated (it extends `TConfigEntry`).
        // TConfigEntryCategory.getWidget(4): not translated, its class isn't translated (it extends `TConfigEntry`).
        // TConfigEntryCategory.saveValuesToConfig(0): not translated, its class isn't translated (it extends `TConfigEntry`).
        // TConfigEntryCategory.setWidgetBackgroundToFullWidth(0): not translated, its class isn't translated (it extends `TConfigEntry`).
        // TConfigEntryCategory.setValuesToDefault(0): not translated, its class isn't translated (it extends `TConfigEntry`).
        // TConfigEntryCategory.resetValuesToInitial(0): not translated, its class isn't translated (it extends `TConfigEntry`).
        // TConfigEntryCategory.add(1): not translated, its class isn't translated (it extends `TConfigEntry`).
        // TConfigEntryCategory.addAll(1): not translated, its class isn't translated (it extends `TConfigEntry`).
        // TConfigEntryCategory.add(1): not translated, its class isn't translated (it extends `TConfigEntry`).
        // TConfigEntryCategory.addOrMerge(1): not translated, its class isn't translated (it extends `TConfigEntry`).
        // TConfigEntryCategory.hasChangedFromInitial(0): not translated, its class isn't translated (it extends `TConfigEntry`).
        // TConfigEntryCategory.setEmptyTooltip(1): not translated, its class isn't translated (it extends `TConfigEntry`).
        // TConfigEntryCategory.setRenderFeature(1): not translated, its class isn't translated (it extends `TConfigEntry`).
    }

    /// `TConfigEntryCategory.CategoryButton` (traben/tconfig/gui/entries/TConfigEntryCategory.java)
    pub mod t_config_entry_category_category_button {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Plain`. Its static parts are below.
        // TConfigEntryCategory.CategoryButton.CategoryButton(6): not translated, its class isn't translated (it extends Minecraft's `Plain`).
        // TConfigEntryCategory.CategoryButton.getMessage(0): not translated, its class isn't translated (it extends Minecraft's `Plain`).
    }

    /// `TConfigEntryCategory.Empty` (traben/tconfig/gui/entries/TConfigEntryCategory.java)
    pub mod t_config_entry_category_empty {
        use super::*;
        // The class itself isn't translated: it extends `TConfigEntryCategory`. Its static parts are below.
        // TConfigEntryCategory.Empty.Empty(0): not translated, its class isn't translated (it extends `TConfigEntryCategory`).
        // TConfigEntryCategory.Empty.getWidget(4): not translated, its class isn't translated (it extends `TConfigEntryCategory`).
    }

    /// `TConfigEntryCustomButton` (traben/tconfig/gui/entries/TConfigEntryCustomButton.java)
    pub mod t_config_entry_custom_button {
        use super::*;
        // The class itself isn't translated: it extends `TConfigEntry`. Its static parts are below.
        // TConfigEntryCustomButton.TConfigEntryCustomButton(3): not translated, its class isn't translated (it extends `TConfigEntry`).
        // TConfigEntryCustomButton.TConfigEntryCustomButton(2): not translated, its class isn't translated (it extends `TConfigEntry`).
        // TConfigEntryCustomButton.getWidget(4): not translated, its class isn't translated (it extends `TConfigEntry`).
        // TConfigEntryCustomButton.saveValuesToConfig(0): not translated, its class isn't translated (it extends `TConfigEntry`).
        // TConfigEntryCustomButton.setValuesToDefault(0): not translated, its class isn't translated (it extends `TConfigEntry`).
        // TConfigEntryCustomButton.resetValuesToInitial(0): not translated, its class isn't translated (it extends `TConfigEntry`).
        // TConfigEntryCustomButton.hasChangedFromInitial(0): not translated, its class isn't translated (it extends `TConfigEntry`).
    }

    /// `TConfigEntryCustomScreenOpener` (traben/tconfig/gui/entries/TConfigEntryCustomScreenOpener.java)
    pub mod t_config_entry_custom_screen_opener {
        use super::*;
        // The class itself isn't translated: it extends `TConfigEntry`. Its static parts are below.
        // TConfigEntryCustomScreenOpener.TConfigEntryCustomScreenOpener(7): not translated, its class isn't translated (it extends `TConfigEntry`).
        // TConfigEntryCustomScreenOpener.TConfigEntryCustomScreenOpener(6): not translated, its class isn't translated (it extends `TConfigEntry`).
        // TConfigEntryCustomScreenOpener.TConfigEntryCustomScreenOpener(4): not translated, its class isn't translated (it extends `TConfigEntry`).
        // TConfigEntryCustomScreenOpener.TConfigEntryCustomScreenOpener(3): not translated, its class isn't translated (it extends `TConfigEntry`).
        // TConfigEntryCustomScreenOpener.getScreen(0): not translated, its class isn't translated (it extends `TConfigEntry`).
        // TConfigEntryCustomScreenOpener.getWidget(4): not translated, its class isn't translated (it extends `TConfigEntry`).
        // TConfigEntryCustomScreenOpener.saveValuesToConfig(0): not translated, its class isn't translated (it extends `TConfigEntry`).
        // TConfigEntryCustomScreenOpener.setValuesToDefault(0): not translated, its class isn't translated (it extends `TConfigEntry`).
        // TConfigEntryCustomScreenOpener.resetValuesToInitial(0): not translated, its class isn't translated (it extends `TConfigEntry`).
        // TConfigEntryCustomScreenOpener.hasChangedFromInitial(0): not translated, its class isn't translated (it extends `TConfigEntry`).
    }

    /// `TConfigEntryEnumButton` (traben/tconfig/gui/entries/TConfigEntryEnumButton.java)
    pub mod t_config_entry_enum_button {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // TConfigEntryEnumButton.TConfigEntryEnumButton(6): not translated, its class isn't translated (it's generic).
        // TConfigEntryEnumButton.TConfigEntryEnumButton(5): not translated, its class isn't translated (it's generic).
        // TConfigEntryEnumButton.TConfigEntryEnumButton(5): not translated, its class isn't translated (it's generic).
        // TConfigEntryEnumButton.TConfigEntryEnumButton(4): not translated, its class isn't translated (it's generic).
        // TConfigEntryEnumButton.allowNullValue(0): not translated, its class isn't translated (it's generic).
        // TConfigEntryEnumButton.getValueFromWidget(0): not translated, its class isn't translated (it's generic).
        // TConfigEntryEnumButton.getWidget(4): not translated, its class isn't translated (it's generic).
        // TConfigEntryEnumButton.setWidgetToDefaultValue(0): not translated, its class isn't translated (it's generic).
        // TConfigEntryEnumButton.resetWidgetToInitialValue(0): not translated, its class isn't translated (it's generic).
    }

    /// `TConfigEntryEnumButton.EnumButtonWidget` (traben/tconfig/gui/entries/TConfigEntryEnumButton.java)
    pub mod t_config_entry_enum_button_enum_button_widget {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // TConfigEntryEnumButton.EnumButtonWidget.EnumButtonWidget(4): not translated, its class isn't translated (it's generic).
        // TConfigEntryEnumButton.EnumButtonWidget.getIndex(0): not translated, its class isn't translated (it's generic).
        // TConfigEntryEnumButton.EnumButtonWidget.getChoiceCount(0): not translated, its class isn't translated (it's generic).
        // TConfigEntryEnumButton.EnumButtonWidget.getValue(0): not translated, its class isn't translated (it's generic).
        // TConfigEntryEnumButton.EnumButtonWidget.setValue(1): not translated, its class isn't translated (it's generic).
        // TConfigEntryEnumButton.EnumButtonWidget.updateMessage(0): not translated, its class isn't translated (it's generic).
        // TConfigEntryEnumButton.EnumButtonWidget.onPress(1): not translated, its class isn't translated (it's generic).
    }

    /// `TConfigEntryEnumSlider` (traben/tconfig/gui/entries/TConfigEntryEnumSlider.java)
    pub mod t_config_entry_enum_slider {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // TConfigEntryEnumSlider.TConfigEntryEnumSlider(6): not translated, its class isn't translated (it's generic).
        // TConfigEntryEnumSlider.TConfigEntryEnumSlider(5): not translated, its class isn't translated (it's generic).
        // TConfigEntryEnumSlider.TConfigEntryEnumSlider(4): not translated, its class isn't translated (it's generic).
        // TConfigEntryEnumSlider.TConfigEntryEnumSlider(5): not translated, its class isn't translated (it's generic).
        // TConfigEntryEnumSlider.allowNullValue(0): not translated, its class isn't translated (it's generic).
        // TConfigEntryEnumSlider.getValueFromWidget(0): not translated, its class isn't translated (it's generic).
        // TConfigEntryEnumSlider.getWidget(4): not translated, its class isn't translated (it's generic).
        // TConfigEntryEnumSlider.setWidgetToDefaultValue(0): not translated, its class isn't translated (it's generic).
        // TConfigEntryEnumSlider.resetWidgetToInitialValue(0): not translated, its class isn't translated (it's generic).
    }

    /// `TConfigEntryEnumSlider.EnumSliderWidget` (traben/tconfig/gui/entries/TConfigEntryEnumSlider.java)
    pub mod t_config_entry_enum_slider_enum_slider_widget {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // TConfigEntryEnumSlider.EnumSliderWidget.EnumSliderWidget(4): not translated, its class isn't translated (it's generic).
        // TConfigEntryEnumSlider.EnumSliderWidget.getValue(0): not translated, its class isn't translated (it's generic).
        // TConfigEntryEnumSlider.EnumSliderWidget.setValue(1): not translated, its class isn't translated (it's generic).
        // TConfigEntryEnumSlider.EnumSliderWidget.updateMessage(0): not translated, its class isn't translated (it's generic).
        // TConfigEntryEnumSlider.EnumSliderWidget.applyValue(0): not translated, its class isn't translated (it's generic).
        // TConfigEntryEnumSlider.EnumSliderWidget.getChoiceCount(0): not translated, its class isn't translated (it's generic).
        // TConfigEntryEnumSlider.EnumSliderWidget.getIndex(0): not translated, its class isn't translated (it's generic).
    }

    /// `TConfigEntryInt` (traben/tconfig/gui/entries/TConfigEntryInt.java)
    pub mod t_config_entry_int {
        use super::*;
        // The class itself isn't translated: it extends `TConfigEntryValue`. Its static parts are below.
        // TConfigEntryInt.TConfigEntryInt(9): not translated, its class isn't translated (it extends `TConfigEntryValue`).
        // TConfigEntryInt.TConfigEntryInt(7): not translated, its class isn't translated (it extends `TConfigEntryValue`).
        // TConfigEntryInt.TConfigEntryInt(8): not translated, its class isn't translated (it extends `TConfigEntryValue`).
        // TConfigEntryInt.TConfigEntryInt(6): not translated, its class isn't translated (it extends `TConfigEntryValue`).
        // TConfigEntryInt.getValueFromWidget(0): not translated, its class isn't translated (it extends `TConfigEntryValue`).
        // TConfigEntryInt.getWidget(4): not translated, its class isn't translated (it extends `TConfigEntryValue`).
        // TConfigEntryInt.setWidgetToDefaultValue(0): not translated, its class isn't translated (it extends `TConfigEntryValue`).
        // TConfigEntryInt.resetWidgetToInitialValue(0): not translated, its class isn't translated (it extends `TConfigEntryValue`).
        // TConfigEntryInt.dontModifyOffMaxValues(0): not translated, its class isn't translated (it extends `TConfigEntryValue`).
    }

    /// `TConfigEntryInt.IntSliderWidget` (traben/tconfig/gui/entries/TConfigEntryInt.java)
    pub mod t_config_entry_int_int_slider_widget {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `AbstractSliderButton`. Its static parts are below.
        // TConfigEntryInt.IntSliderWidget.IntSliderWidget(7): not translated, its class isn't translated (it extends Minecraft's `AbstractSliderButton`).
        // TConfigEntryInt.IntSliderWidget.isOff(0): not translated, its class isn't translated (it extends Minecraft's `AbstractSliderButton`).
        // TConfigEntryInt.IntSliderWidget.setValue(1): not translated, its class isn't translated (it extends Minecraft's `AbstractSliderButton`).
        // TConfigEntryInt.IntSliderWidget.updateMessage(0): not translated, its class isn't translated (it extends Minecraft's `AbstractSliderButton`).
        // TConfigEntryInt.IntSliderWidget.applyValue(0): not translated, its class isn't translated (it extends Minecraft's `AbstractSliderButton`).
        // TConfigEntryInt.IntSliderWidget.snapValueToNearestIndex(0): not translated, its class isn't translated (it extends Minecraft's `AbstractSliderButton`).
        // TConfigEntryInt.IntSliderWidget.getValueRoundedToIntBetweenMinMax(0): not translated, its class isn't translated (it extends Minecraft's `AbstractSliderButton`).
    }

    /// `TConfigEntryNullSafe` (traben/tconfig/gui/entries/TConfigEntryNullSafe.java)
    pub mod t_config_entry_null_safe {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // TConfigEntryNullSafe.TConfigEntryNullSafe(5): not translated, its class isn't translated (it's generic).
        // TConfigEntryNullSafe.hasChangedFromInitial(0): not translated, its class isn't translated (it's generic).
        // TConfigEntryNullSafe.allowNullValue(0): not translated, it has no body.
    }

    /// `TConfigEntryText` (traben/tconfig/gui/entries/TConfigEntryText.java)
    pub mod t_config_entry_text {
        use super::*;
        // The class itself isn't translated: it extends `TConfigEntry`. Its static parts are below.
        // TConfigEntryText.TConfigEntryText(2): not translated, its class isn't translated (it extends `TConfigEntry`).
        // TConfigEntryText.TConfigEntryText(1): not translated, its class isn't translated (it extends `TConfigEntry`).
        // TConfigEntryText.fromLongOrMultilineTranslation(2): not translated, it uses TConfigEntry, which isn't translated (it extends `TConfigEntryForList`).
        // TConfigEntryText.fromLongOrMultilineTranslation(3): not translated, it uses TConfigEntry, which isn't translated (it extends `TConfigEntryForList`).
        // TConfigEntryText.getWidget(4): not translated, its class isn't translated (it extends `TConfigEntry`).
        // TConfigEntryText.hasChangedFromInitial(0): not translated, its class isn't translated (it extends `TConfigEntry`).
        // TConfigEntryText.saveValuesToConfig(0): not translated, its class isn't translated (it extends `TConfigEntry`).
        // TConfigEntryText.setValuesToDefault(0): not translated, its class isn't translated (it extends `TConfigEntry`).
        // TConfigEntryText.resetValuesToInitial(0): not translated, its class isn't translated (it extends `TConfigEntry`).
    }

    /// `TConfigEntryText.TextAlignment` (traben/tconfig/gui/entries/TConfigEntryText.java)
    pub mod t_config_entry_text_text_alignment {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum TextAlignment {
            Left,
            Center,
            Right,
        }
        impl TextAlignment {
            pub const VALUES: [TextAlignment; 3] = [TextAlignment::Left, TextAlignment::Center, TextAlignment::Right, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    TextAlignment::Left => "LEFT",
                    TextAlignment::Center => "CENTER",
                    TextAlignment::Right => "RIGHT",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "LEFT" => TextAlignment::Left,
                    "CENTER" => TextAlignment::Center,
                    "RIGHT" => TextAlignment::Right,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
        impl TextAlignment {
            // TConfigEntryText.TextAlignment.align(1): not translated, it uses Minecraft's `StringWidget`.
        }
    }

    /// `TConfigEntryText.TwoLines` (traben/tconfig/gui/entries/TConfigEntryText.java)
    pub mod t_config_entry_text_two_lines {
        use super::*;
        // The class itself isn't translated: it extends `TConfigEntryText`. Its static parts are below.
        // TConfigEntryText.TwoLines.TwoLines(2): not translated, its class isn't translated (it extends `TConfigEntryText`).
        // TConfigEntryText.TwoLines.TwoLines(3): not translated, its class isn't translated (it extends `TConfigEntryText`).
        // TConfigEntryText.TwoLines.getWidget(4): not translated, its class isn't translated (it extends `TConfigEntryText`).
        // TConfigEntryText.TwoLines.extractContent(5): not translated, its class isn't translated (it extends `TConfigEntryText`).
    }

    /// `TConfigEntryValue` (traben/tconfig/gui/entries/TConfigEntryValue.java)
    pub mod t_config_entry_value {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // TConfigEntryValue.TConfigEntryValue(5): not translated, its class isn't translated (it's generic).
        // TConfigEntryValue.TConfigEntryValue(4): not translated, its class isn't translated (it's generic).
        // TConfigEntryValue.saveValuesToConfig(0): not translated, its class isn't translated (it's generic).
        // TConfigEntryValue.getValueFromWidget(0): not translated, it has no body.
        // TConfigEntryValue.setValuesToDefault(0): not translated, its class isn't translated (it's generic).
        // TConfigEntryValue.setWidgetToDefaultValue(0): not translated, it has no body.
        // TConfigEntryValue.resetValuesToInitial(0): not translated, its class isn't translated (it's generic).
        // TConfigEntryValue.resetWidgetToInitialValue(0): not translated, it has no body.
        // TConfigEntryValue.hasChangedFromInitial(0): not translated, its class isn't translated (it's generic).
    }

    /// `TConfigEntryListWidget` (traben/tconfig/gui/TConfigEntryListWidget.java)
    pub mod t_config_entry_list_widget {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `AbstractSelectionList`. Its static parts are below.
        // TConfigEntryListWidget.TConfigEntryListWidget(6): not translated, its class isn't translated (it extends Minecraft's `AbstractSelectionList`).
        // TConfigEntryListWidget.getRowWidth(0): not translated, its class isn't translated (it extends Minecraft's `AbstractSelectionList`).
        // TConfigEntryListWidget.scrollBarX(0): not translated, its class isn't translated (it extends Minecraft's `AbstractSelectionList`).
        // TConfigEntryListWidget.updateWidgetNarration(1): not translated, its class isn't translated (it extends Minecraft's `AbstractSelectionList`).
        // TConfigEntryListWidget.isValidClickButton(1): not translated, its class isn't translated (it extends Minecraft's `AbstractSelectionList`).
        // TConfigEntryListWidget.setSelected(1): not translated, its class isn't translated (it extends Minecraft's `AbstractSelectionList`).
        // TConfigEntryListWidget.setWidgetBackgroundToFullWidth(0): not translated, its class isn't translated (it extends Minecraft's `AbstractSelectionList`).
        // TConfigEntryListWidget.extractListBackground(1): not translated, its body couldn't be read: an assert.
        // TConfigEntryListWidget.extractListSeparators(1): not translated, its body couldn't be read: an assert.
    }

    /// `TConfigEntryListWidget.TConfigEntryForList` (traben/tconfig/gui/TConfigEntryListWidget.java)
    pub mod t_config_entry_list_widget_t_config_entry_for_list {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Entry`. Its static parts are below.
        // TConfigEntryListWidget.TConfigEntryForList.extractContent(5): not translated, its class isn't translated (it extends Minecraft's `Entry`).
        // TConfigEntryListWidget.TConfigEntryForList.getWidget(4): not translated, it has no body.
        // TConfigEntryListWidget.TConfigEntryForList.ignoreMouseAt(2): not translated, its class isn't translated (it extends Minecraft's `Entry`).
        // TConfigEntryListWidget.TConfigEntryForList.mouseClicked(2): not translated, its class isn't translated (it extends Minecraft's `Entry`).
        // TConfigEntryListWidget.TConfigEntryForList.mouseDragged(3): not translated, its class isn't translated (it extends Minecraft's `Entry`).
        // TConfigEntryListWidget.TConfigEntryForList.mouseReleased(1): not translated, its class isn't translated (it extends Minecraft's `Entry`).
        // TConfigEntryListWidget.TConfigEntryForList.setFocused(1): not translated, its class isn't translated (it extends Minecraft's `Entry`).
    }

    /// `TConfigScreen` (traben/tconfig/gui/TConfigScreen.java)
    pub mod t_config_screen {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Screen`. Its static parts are below.
        // TConfigScreen.TConfigScreen(3): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // TConfigScreen.getBackButtonText(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // TConfigScreen.init(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // TConfigScreen.extractRenderState(4): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // TConfigScreen.shouldCloseOnEsc(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // TConfigScreen.onClose(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
    }

    /// `TConfigScreenList` (traben/tconfig/gui/TConfigScreenList.java)
    pub mod t_config_screen_list {
        use super::*;
        // The class itself isn't translated: it extends `TConfigScreen`. Its static parts are below.
        // TConfigScreenList.TConfigScreenList(6): not translated, its class isn't translated (it extends `TConfigScreen`).
        // TConfigScreenList.TConfigScreenList(5): not translated, its class isn't translated (it extends `TConfigScreen`).
        // TConfigScreenList.setRenderFeature(1): not translated, its class isn't translated (it extends `TConfigScreen`).
        // TConfigScreenList.extractRenderState(4): not translated, its class isn't translated (it extends `TConfigScreen`).
        // TConfigScreenList.setWidgetBackgroundToFullWidth(0): not translated, its class isn't translated (it extends `TConfigScreen`).
        // TConfigScreenList.init(0): not translated, its class isn't translated (it extends `TConfigScreen`).
        // TConfigScreenList.updateSearch(1): not translated, its body couldn't be read: an assert.
        // TConfigScreenList.initList(1): not translated, its class isn't translated (it extends `TConfigScreen`).
    }

    /// `TConfigScreenList.Align` (traben/tconfig/gui/TConfigScreenList.java)
    pub mod t_config_screen_list_align {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum Align {
            Left,
            Center,
            Right,
        }
        impl Align {
            pub const VALUES: [Align; 3] = [Align::Left, Align::Center, Align::Right, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    Align::Left => "LEFT",
                    Align::Center => "CENTER",
                    Align::Right => "RIGHT",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "LEFT" => Align::Left,
                    "CENTER" => Align::Center,
                    "RIGHT" => Align::Right,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `TConfigScreenList.Renderable` (traben/tconfig/gui/TConfigScreenList.java)
    pub mod t_config_screen_list_renderable {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // TConfigScreenList.Renderable.render(3): not translated, it has no body.
    }

    /// `TConfigScreenMain` (traben/tconfig/gui/TConfigScreenMain.java)
    pub mod t_config_screen_main {
        use super::*;
        // The class itself isn't translated: it extends `TConfigScreen`. Its static parts are below.
        // TConfigScreenMain.TConfigScreenMain(4): not translated, its class isn't translated (it extends `TConfigScreen`).
        // TConfigScreenMain.getBackButtonText(0): not translated, its class isn't translated (it extends `TConfigScreen`).
        // TConfigScreenMain.initConfigs(0): not translated, its class isn't translated (it extends `TConfigScreen`).
        // TConfigScreenMain.onClose(0): not translated, its class isn't translated (it extends `TConfigScreen`).
        // TConfigScreenMain.init(0): not translated, its class isn't translated (it extends `TConfigScreen`).
        // TConfigScreenMain.extractRenderState(4): not translated, its class isn't translated (it extends `TConfigScreen`).
    }

    /// `TConfig` (traben/tconfig/TConfig.java)
    pub mod t_config {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct TConfig {
        }
        impl TConfig {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // TConfig.getGUIOptions(0): not translated, it has no body.
            // TConfig.getModIcon(0): not translated, it has no body.
            pub fn does_gui(&self) -> bool {
                return true;
            }
        }
    }

    /// `TConfig.NoGUI` (traben/tconfig/TConfig.java)
    pub mod t_config_no_gui {
        use super::*;
        // The class itself isn't translated: it extends `TConfig`. Its static parts are below.
        // TConfig.NoGUI.getGUIOptions(0): not translated, its class isn't translated (it extends `TConfig`).
        // TConfig.NoGUI.getModIcon(0): not translated, its class isn't translated (it extends `TConfig`).
        // TConfig.NoGUI.doesGUI(0): not translated, its class isn't translated (it extends `TConfig`).
    }

    /// `TConfigHandler` (traben/tconfig/TConfigHandler.java)
    pub mod t_config_handler {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // TConfigHandler.TConfigHandler(3): not translated, its class isn't translated (it's generic).
        // TConfigHandler.getConfig(0): not translated, its class isn't translated (it's generic).
        // TConfigHandler.setConfig(1): not translated, its class isn't translated (it's generic).
        // TConfigHandler.equals(1): not translated, its class isn't translated (it's generic).
        // TConfigHandler.configEquals(1): not translated, its class isn't translated (it's generic).
        // TConfigHandler.hashCode(0): not translated, its class isn't translated (it's generic).
        // TConfigHandler.saveToFile(0): not translated, its class isn't translated (it's generic).
        // TConfigHandler.toJson(0): not translated, its class isn't translated (it's generic).
        // TConfigHandler.toJson(1): not translated, its class isn't translated (it's generic).
        // TConfigHandler.loadFromFile(0): not translated, its class isn't translated (it's generic).
        // TConfigHandler.loadFromFile(1): not translated, its class isn't translated (it's generic).
        // TConfigHandler.fromJson(1): not translated, its class isn't translated (it's generic).
        // TConfigHandler.fromJson(1): not translated, its class isn't translated (it's generic).
        // TConfigHandler.copyOfConfig(0): not translated, its class isn't translated (it's generic).
        // TConfigHandler.doesGUI(0): not translated, its class isn't translated (it's generic).
    }

    /// `TConfigLog` (traben/tconfig/TConfigLog.java)
    pub mod t_config_log {
        use super::*;
        // LOGGER: the logger. Log calls become comments.
        #[derive(Clone, Debug)]
        pub struct TConfigLog {
        }
        impl TConfigLog {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn log(id: &str, message: &str) {
            // log:
        }
        pub fn log_error(id: &str, message: &str) {
            // log:
        }
        pub fn log_warn(id: &str, message: &str) {
            // log:
        }
    }

    
    // ---- Java regular expressions (java.util.regex), a small backtracking engine ----
    // Bundles can't add the regex crate, so this covers what mods use: literals, `.`, classes with ranges and
    // \d \s \w, groups (capturing, `(?:`, `(?<name>`), lookahead, alternation, greedy/lazy quantifiers, ^ $ \b.
    // Positions are in chars, like the translated String methods.
    
    #[derive(Clone, Debug)]
    enum JClassItem {
        Range(char, char),
        Digit(bool),
        Space(bool),
        Word(bool),
    }
    
    #[derive(Clone, Debug)]
    enum JNode {
        Char(char),
        Any,
        Class(Vec<JClassItem>, bool),
        Start,
        End,
        WordB(bool),
        Group(Box<JNode>, Option<usize>),
        Look(Box<JNode>, bool),
        Alt(Vec<JNode>),
        Concat(Vec<JNode>),
        Repeat(Box<JNode>, u32, u32, bool),
        Backref(usize),
    }
    
    type JCaps = Vec<Option<(usize, usize)>>;
    
    /// A compiled `Pattern`.
    #[derive(Clone, Debug)]
    pub struct JRegex {
        node: JNode,
        groups: usize,
        names: Vec<(String, usize)>,
        ci: bool,
    }
    
    fn j_is_word(c: char) -> bool {
        c.is_ascii_alphanumeric() || c == '_'
    }
    
    fn j_is_space(c: char) -> bool {
        matches!(c, ' ' | '\t' | '\n' | '\u{0B}' | '\u{0C}' | '\r')
    }
    
    struct JRegexParser<'a> {
        p: &'a [char],
        i: usize,
        groups: usize,
        names: Vec<(String, usize)>,
    }
    
    impl JRegexParser<'_> {
        fn peek(&self) -> Option<char> {
            self.p.get(self.i).copied()
        }
        fn eat(&mut self, c: char) -> bool {
            if self.peek() == Some(c) {
                self.i += 1;
                true
            } else {
                false
            }
        }
        fn alt(&mut self) -> JNode {
            let mut arms = vec![self.concat()];
            while self.eat('|') {
                arms.push(self.concat());
            }
            if arms.len() == 1 { arms.pop().unwrap() } else { JNode::Alt(arms) }
        }
        fn concat(&mut self) -> JNode {
            let mut items = Vec::new();
            while let Some(c) = self.peek() {
                if c == '|' || c == ')' {
                    break;
                }
                let atom = self.atom();
                items.push(self.quantified(atom));
            }
            JNode::Concat(items)
        }
        fn number(&mut self) -> Option<u32> {
            let start = self.i;
            while self.peek().is_some_and(|c| c.is_ascii_digit()) {
                self.i += 1;
            }
            if start == self.i { None } else { self.p[start..self.i].iter().collect::<String>().parse().ok() }
        }
        fn quantified(&mut self, atom: JNode) -> JNode {
            let (min, max) = match self.peek() {
                Some('*') => (0, u32::MAX),
                Some('+') => (1, u32::MAX),
                Some('?') => (0, 1),
                Some('{') => {
                    let save = self.i;
                    self.i += 1;
                    let Some(lo) = self.number() else {
                        self.i = save;
                        return atom;
                    };
                    let hi = if self.eat(',') { self.number().unwrap_or(u32::MAX) } else { lo };
                    if !self.eat('}') {
                        panic!("bad repetition in regex");
                    }
                    self.i -= 1;
                    (lo, hi)
                }
                _ => return atom,
            };
            self.i += 1;
            let greedy = !self.eat('?');
            if greedy {
                self.eat('+');
            }
            self.quantified(JNode::Repeat(Box::new(atom), min, max, greedy))
        }
        fn escape(&mut self, in_class: bool) -> JClassItem {
            let c = self.peek().expect("regex ends with a backslash");
            self.i += 1;
            let lit = |c: char| JClassItem::Range(c, c);
            match c {
                'd' => JClassItem::Digit(true),
                'D' => JClassItem::Digit(false),
                's' => JClassItem::Space(true),
                'S' => JClassItem::Space(false),
                'w' => JClassItem::Word(true),
                'W' => JClassItem::Word(false),
                't' => lit('\t'),
                'n' => lit('\n'),
                'r' => lit('\r'),
                'f' => lit('\u{0C}'),
                'e' => lit('\u{1B}'),
                '0' => lit('\0'),
                'u' => {
                    let hex: String = self.p[self.i..self.i + 4].iter().collect();
                    self.i += 4;
                    lit(char::from_u32(u32::from_str_radix(&hex, 16).expect("bad \\u escape")).unwrap_or('\u{FFFD}'))
                }
                'x' => {
                    let hex: String = self.p[self.i..self.i + 2].iter().collect();
                    self.i += 2;
                    lit(char::from_u32(u32::from_str_radix(&hex, 16).expect("bad \\x escape")).unwrap_or('\u{FFFD}'))
                }
                c if !in_class && c.is_ascii_alphabetic() => panic!("unsupported regex escape \\{c}"),
                c => lit(c),
            }
        }
        fn class(&mut self) -> JNode {
            let negated = self.eat('^');
            let mut items = Vec::new();
            let mut first = true;
            loop {
                let c = self.peek().expect("unclosed [ in regex");
                if c == ']' && !first {
                    self.i += 1;
                    break;
                }
                first = false;
                if c == '[' {
                    panic!("nested character classes aren't supported");
                }
                self.i += 1;
                let item = if c == '\\' { self.escape(true) } else { JClassItem::Range(c, c) };
                // a-z
                if let JClassItem::Range(lo, _) = item
                    && self.peek() == Some('-')
                    && self.p.get(self.i + 1).is_some_and(|&n| n != ']')
                {
                    self.i += 1;
                    let hc = self.peek().unwrap();
                    self.i += 1;
                    let hi = if hc == '\\' {
                        match self.escape(true) {
                            JClassItem::Range(h, _) => h,
                            _ => panic!("bad range in regex"),
                        }
                    } else {
                        hc
                    };
                    items.push(JClassItem::Range(lo, hi));
                    continue;
                }
                items.push(item);
            }
            JNode::Class(items, negated)
        }
        fn atom(&mut self) -> JNode {
            let c = self.peek().unwrap();
            self.i += 1;
            match c {
                '.' => JNode::Any,
                '^' => JNode::Start,
                '$' => JNode::End,
                '[' => self.class(),
                '(' => {
                    let mut index = None;
                    let mut look = None;
                    if self.eat('?') {
                        if self.eat(':') {
                        } else if self.eat('=') {
                            look = Some(false);
                        } else if self.eat('!') {
                            look = Some(true);
                        } else if self.eat('<') {
                            let start = self.i;
                            while self.peek().is_some_and(|c| c != '>') {
                                self.i += 1;
                            }
                            let name: String = self.p[start..self.i].iter().collect();
                            self.i += 1;
                            self.groups += 1;
                            self.names.push((name, self.groups));
                            index = Some(self.groups);
                        } else {
                            panic!("unsupported regex group (?{:?}", self.peek());
                        }
                    } else {
                        self.groups += 1;
                        index = Some(self.groups);
                    }
                    let inner = self.alt();
                    if !self.eat(')') {
                        panic!("unclosed ( in regex");
                    }
                    match look {
                        Some(neg) => JNode::Look(Box::new(inner), neg),
                        None => JNode::Group(Box::new(inner), index),
                    }
                }
                '\\' => match self.peek() {
                    Some('b') => {
                        self.i += 1;
                        JNode::WordB(true)
                    }
                    Some('B') => {
                        self.i += 1;
                        JNode::WordB(false)
                    }
                    Some('A') => {
                        self.i += 1;
                        JNode::Start
                    }
                    Some('z') | Some('Z') => {
                        self.i += 1;
                        JNode::End
                    }
                    Some(d) if d.is_ascii_digit() && d != '0' => {
                        self.i += 1;
                        JNode::Backref(d as usize - '0' as usize)
                    }
                    Some('Q') => {
                        self.i += 1;
                        let mut lits = Vec::new();
                        while self.i < self.p.len() && !(self.p[self.i] == '\\' && self.p.get(self.i + 1) == Some(&'E')) {
                            lits.push(JNode::Char(self.p[self.i]));
                            self.i += 1;
                        }
                        self.i = (self.i + 2).min(self.p.len());
                        JNode::Concat(lits)
                    }
                    _ => match self.escape(false) {
                        JClassItem::Range(c, _) => JNode::Char(c),
                        other => JNode::Class(vec![other], false),
                    },
                },
                c => JNode::Char(c),
            }
        }
    }
    
    impl JRegex {
        /// `Pattern.compile(p)`. Java throws PatternSyntaxException on a bad pattern; this panics.
        pub fn new(p: &str) -> Self {
            Self::with_flags(p, 0)
        }
        /// `Pattern.compile(p, flags)`: CASE_INSENSITIVE (2) is honoured.
        pub fn with_flags(p: &str, flags: i32) -> Self {
            let chars: Vec<char> = p.chars().collect();
            let (chars, ci) = match chars.strip_prefix(&['(', '?', 'i', ')']) {
                Some(rest) => (rest.to_vec(), true),
                None => (chars, flags & 2 != 0),
            };
            let mut parser = JRegexParser { p: &chars, i: 0, groups: 0, names: Vec::new() };
            let node = parser.alt();
            if parser.i != chars.len() {
                panic!("unmatched ) in regex {p:?}");
            }
            JRegex { node, groups: parser.groups, names: parser.names, ci }
        }
    
        fn same(&self, a: char, b: char) -> bool {
            a == b || self.ci && (j_lower_char(a) == j_lower_char(b) || j_upper_char(a) == j_upper_char(b))
        }
    
        fn in_class(&self, items: &[JClassItem], c: char) -> bool {
            items.iter().any(|it| match *it {
                JClassItem::Range(lo, hi) => {
                    (lo..=hi).contains(&c) || self.ci && ((lo..=hi).contains(&j_lower_char(c)) || (lo..=hi).contains(&j_upper_char(c)))
                }
                JClassItem::Digit(yes) => c.is_ascii_digit() == yes,
                JClassItem::Space(yes) => j_is_space(c) == yes,
                JClassItem::Word(yes) => j_is_word(c) == yes,
            })
        }
    
        fn m(&self, n: &JNode, s: &[char], i: usize, caps: &mut JCaps, k: &mut dyn FnMut(usize, &mut JCaps) -> bool) -> bool {
            match n {
                JNode::Char(c) => i < s.len() && self.same(s[i], *c) && k(i + 1, caps),
                JNode::Any => i < s.len() && !matches!(s[i], '\n' | '\r' | '\u{85}' | '\u{2028}' | '\u{2029}') && k(i + 1, caps),
                JNode::Class(items, neg) => i < s.len() && self.in_class(items, s[i]) != *neg && k(i + 1, caps),
                JNode::Start => i == 0 && k(i, caps),
                JNode::End => (i == s.len() || i + 1 == s.len() && s[i] == '\n') && k(i, caps),
                JNode::WordB(yes) => {
                    let before = i > 0 && j_is_word(s[i - 1]);
                    let after = i < s.len() && j_is_word(s[i]);
                    (before != after) == *yes && k(i, caps)
                }
                JNode::Group(inner, index) => match *index {
                    None => self.m(inner, s, i, caps, k),
                    Some(g) => self.m(inner, s, i, caps, &mut |j, caps: &mut JCaps| {
                        let prev = caps[g];
                        caps[g] = Some((i, j));
                        if k(j, caps) {
                            return true;
                        }
                        caps[g] = prev;
                        false
                    }),
                },
                JNode::Look(inner, neg) => {
                    let mut probe = caps.clone();
                    let found = self.m(inner, s, i, &mut probe, &mut |_, _| true);
                    if found == *neg {
                        return false;
                    }
                    if !neg {
                        *caps = probe;
                    }
                    k(i, caps)
                }
                JNode::Alt(arms) => arms.iter().any(|a| self.m(a, s, i, caps, k)),
                JNode::Concat(items) => self.seq(items, s, i, caps, k),
                JNode::Repeat(inner, min, max, greedy) => self.rep(inner, *min, *max, *greedy, 0, s, i, caps, k),
                JNode::Backref(g) => match caps.get(*g).copied().flatten() {
                    Some((a, b)) => {
                        let len = b - a;
                        i + len <= s.len() && (0..len).all(|d| self.same(s[a + d], s[i + d])) && k(i + len, caps)
                    }
                    None => false,
                },
            }
        }
    
        fn seq(&self, items: &[JNode], s: &[char], i: usize, caps: &mut JCaps, k: &mut dyn FnMut(usize, &mut JCaps) -> bool) -> bool {
            match items.split_first() {
                None => k(i, caps),
                Some((first, rest)) => self.m(first, s, i, caps, &mut |j, caps: &mut JCaps| self.seq(rest, s, j, caps, k)),
            }
        }
    
        #[allow(clippy::too_many_arguments)]
        fn rep(&self, n: &JNode, min: u32, max: u32, greedy: bool, count: u32, s: &[char], i: usize, caps: &mut JCaps,
               k: &mut dyn FnMut(usize, &mut JCaps) -> bool) -> bool {
            let more = |caps: &mut JCaps, k: &mut dyn FnMut(usize, &mut JCaps) -> bool| {
                count < max
                    && self.m(n, s, i, caps, &mut |j, caps: &mut JCaps| {
                        // An empty pass can't make progress: stop repeating once the minimum is met.
                        (j != i || count < min) && self.rep(n, min, max, greedy, count + 1, s, j, caps, k)
                    })
            };
            if count < min {
                return more(caps, k);
            }
            if greedy { more(caps, k) || k(i, caps) } else { k(i, caps) || more(caps, k) }
        }
    
        /// A match starting exactly at `i`; `whole`: it must also end at the end of the text.
        fn at(&self, s: &[char], i: usize, whole: bool) -> Option<JCaps> {
            let mut caps: JCaps = vec![None; self.groups + 1];
            let mut end = None;
            let ok = self.m(&self.node, s, i, &mut caps, &mut |j, _| {
                if whole && j != s.len() {
                    return false;
                }
                end = Some(j);
                true
            });
            if !ok {
                return None;
            }
            caps[0] = Some((i, end.unwrap()));
            Some(caps)
        }
    
        fn find_from(&self, s: &[char], from: usize) -> Option<JCaps> {
            (from..=s.len()).find_map(|i| self.at(s, i, false))
        }
    
        /// `String.matches` / `Pattern.matches`: the whole text.
        pub fn matches(&self, s: &str) -> bool {
            let cs: Vec<char> = s.chars().collect();
            self.at(&cs, 0, true).is_some()
        }
    
        /// `Pattern.matcher(s).find()` as a test.
        pub fn is_found(&self, s: &str) -> bool {
            let cs: Vec<char> = s.chars().collect();
            self.find_from(&cs, 0).is_some()
        }
    
        fn expand(&self, s: &[char], caps: &JCaps, rep: &str, out: &mut String) {
            let r: Vec<char> = rep.chars().collect();
            let mut i = 0;
            while i < r.len() {
                match r[i] {
                    '\\' if i + 1 < r.len() => {
                        out.push(r[i + 1]);
                        i += 2;
                    }
                    '$' if i + 1 < r.len() && r[i + 1].is_ascii_digit() => {
                        let g = r[i + 1] as usize - '0' as usize;
                        if let Some((a, b)) = caps.get(g).copied().flatten() {
                            out.extend(&s[a..b]);
                        } else if g > self.groups {
                            panic!("No group {g}");
                        }
                        i += 2;
                    }
                    '$' if r.get(i + 1) == Some(&'{') => {
                        let close = r[i..].iter().position(|&c| c == '}').expect("bad ${name} in replacement") + i;
                        let name: String = r[i + 2..close].iter().collect();
                        let g = self.names.iter().find(|(n, _)| *n == name).map(|(_, g)| *g).expect("no group with that name");
                        if let Some((a, b)) = caps[g] {
                            out.extend(&s[a..b]);
                        }
                        i = close + 1;
                    }
                    c => {
                        out.push(c);
                        i += 1;
                    }
                }
            }
        }
    
        fn replace(&self, s: &str, rep: &str, all: bool) -> String {
            let cs: Vec<char> = s.chars().collect();
            let mut out = String::new();
            let (mut pos, mut from) = (0, 0);
            while from <= cs.len() {
                let Some(caps) = self.find_from(&cs, from) else { break };
                let (a, b) = caps[0].unwrap();
                out.extend(&cs[pos..a]);
                self.expand(&cs, &caps, rep, &mut out);
                pos = b;
                from = if b == a { b + 1 } else { b };
                if b == a && a < cs.len() {
                    out.push(cs[a]);
                    pos = a + 1;
                }
                if !all {
                    break;
                }
            }
            if pos <= cs.len() {
                out.extend(&cs[pos..]);
            }
            out
        }
    
        /// `String.replaceAll` / `Matcher.replaceAll`, with Java's `$1` and `\$` in the replacement.
        pub fn replace_all(&self, s: &str, rep: &str) -> String {
            self.replace(s, rep, true)
        }
    
        pub fn replace_first(&self, s: &str, rep: &str) -> String {
            self.replace(s, rep, false)
        }
    
        /// `String.split(regex, limit)` / `Pattern.split`, with Java's rules for empty parts.
        pub fn split(&self, s: &str, limit: i32) -> Vec<String> {
            let cs: Vec<char> = s.chars().collect();
            let mut parts: Vec<String> = Vec::new();
            let (mut pos, mut from) = (0, 0);
            while from <= cs.len() {
                if limit > 0 && parts.len() as i32 == limit - 1 {
                    break;
                }
                let Some(caps) = self.find_from(&cs, from) else { break };
                let (a, b) = caps[0].unwrap();
                if b == 0 {
                    // A zero-width match at the start makes no leading empty part.
                    from = 1;
                    continue;
                }
                if a == b && a >= cs.len() {
                    break;
                }
                parts.push(cs[pos..a].iter().collect());
                pos = b;
                from = if a == b { b + 1 } else { b };
            }
            parts.push(cs[pos.min(cs.len())..].iter().collect());
            if limit == 0 {
                while parts.len() > 1 && parts.last().is_some_and(|p| p.is_empty()) {
                    parts.pop();
                }
                if parts.len() == 1 && parts[0].is_empty() && !s.is_empty() {
                    parts.clear();
                }
            }
            parts
        }
    }
    
    /// A `Matcher`: a regex over one text, with the last match.
    #[derive(Clone, Debug)]
    pub struct JMatcher {
        re: JRegex,
        text: Vec<char>,
        from: usize,
        caps: Option<JCaps>,
    }
    
    impl JMatcher {
        pub fn new(re: &JRegex, s: &str) -> Self {
            JMatcher { re: re.clone(), text: s.chars().collect(), from: 0, caps: None }
        }
        pub fn find(&mut self) -> bool {
            if self.from > self.text.len() {
                self.caps = None;
                return false;
            }
            self.caps = self.re.find_from(&self.text, self.from);
            if let Some((a, b)) = self.caps.as_ref().and_then(|c| c[0]) {
                self.from = if a == b { b + 1 } else { b };
                true
            } else {
                self.from = self.text.len() + 1;
                false
            }
        }
        pub fn find_at(&mut self, start: i32) -> bool {
            self.from = start as usize;
            self.find()
        }
        pub fn matches(&mut self) -> bool {
            self.caps = self.re.at(&self.text, 0, true);
            self.caps.is_some()
        }
        pub fn looking_at(&mut self) -> bool {
            self.caps = self.re.at(&self.text, 0, false);
            self.caps.is_some()
        }
        fn span(&self, g: usize) -> Option<(usize, usize)> {
            let caps = self.caps.as_ref().expect("No match found");
            assert!(g < caps.len(), "No group {g}");
            caps[g]
        }
        /// `group(n)`: null (None) when that group took no part in the match.
        pub fn group(&self, g: i32) -> Option<String> {
            self.span(g as usize).map(|(a, b)| self.text[a..b].iter().collect())
        }
        pub fn group_named(&self, name: &str) -> Option<String> {
            let g = self.re.names.iter().find(|(n, _)| n == name).map(|(_, g)| *g).expect("No group with that name");
            self.group(g as i32)
        }
        pub fn start(&self, g: i32) -> i32 {
            self.span(g as usize).map_or(-1, |(a, _)| a as i32)
        }
        pub fn end(&self, g: i32) -> i32 {
            self.span(g as usize).map_or(-1, |(_, b)| b as i32)
        }
        pub fn group_count(&self) -> i32 {
            self.re.groups as i32
        }
        pub fn replace_all(&self, rep: &str) -> String {
            self.re.replace_all(&self.text.iter().collect::<String>(), rep)
        }
    }
    
    #[cfg(test)]
    mod jregex_tests {
        use super::*;
    
        #[test]
        fn groups_and_lazy() {
            let re = JRegex::new("^(?:\\S )?(?:VERY SPECIAL|COMMON|RARE|EPIC)(?: DUNGEON)?(?: (.+?))?(?: \\S)?$");
            let mut m = JMatcher::new(&re, "EPIC DUNGEON SWORD");
            assert!(m.matches());
            assert_eq!(m.group(1).as_deref(), Some("SWORD"));
            let mut m = JMatcher::new(&re, "a RARE FISHING ROD b");
            assert!(m.matches());
            assert_eq!(m.group(1).as_deref(), Some("FISHING ROD"));
            let mut m = JMatcher::new(&re, "COMMON");
            assert!(m.matches());
            assert_eq!(m.group(1), None);
        }
    
        #[test]
        fn classes_counts_find() {
            let re = JRegex::new("server/[0-9A-Fa-f]{8}/(.+)");
            let mut m = JMatcher::new(&re, "server/0aF3b2c1/pack.zip");
            assert!(m.matches());
            assert_eq!(m.group(1).as_deref(), Some("pack.zip"));
            assert!(!JRegex::new("server/[0-9A-Fa-f]{8}/(.+)").matches("server/xyz/a"));
            let re = JRegex::new("\"id\"\\s*:\\s*\"([a-z0-9_.-]+:[a-z0-9_./-]+)\"");
            let mut m = JMatcher::new(&re, "{\"id\": \"minecraft:item/a\", \"id\":\"x:y\"}");
            assert!(m.find());
            assert_eq!(m.group(1).as_deref(), Some("minecraft:item/a"));
            assert!(m.find());
            assert_eq!(m.group(1).as_deref(), Some("x:y"));
            assert!(!m.find());
        }
    
        #[test]
        fn split_and_replace() {
            assert_eq!(JRegex::new("\\s*,\\s*").split("a , b,c,,", 0), vec!["a", "b", "c"]);
            assert_eq!(JRegex::new(",").split("a,b,c", 2), vec!["a", "b,c"]);
            assert_eq!(JRegex::new("").split("abc", 0), vec!["a", "b", "c"]);
            assert_eq!(JRegex::new("(\\w+)@(\\w+)").replace_all("x a@b y c@d", "$2.$1"), "x b.a y d.c");
            assert_eq!(JRegex::new("a*").replace_all("baaac", "-"), "-b--c-");
            assert!(JRegex::with_flags("abc", 2).matches("AbC"));
        }
    }

    
    // ---- java.util.Properties: `key=value` text files ----
    
    #[derive(Clone, Debug, Default)]
    pub struct JProps(pub std::collections::BTreeMap<String, String>);
    
    fn j_props_unescape(s: &str) -> String {
        let mut out = String::new();
        let mut it = s.chars().peekable();
        while let Some(c) = it.next() {
            if c != '\\' {
                out.push(c);
                continue;
            }
            match it.next() {
                Some('t') => out.push('\t'),
                Some('n') => out.push('\n'),
                Some('r') => out.push('\r'),
                Some('f') => out.push('\u{c}'),
                Some('u') => {
                    let hex: String = (0..4).filter_map(|_| it.next()).collect();
                    out.push(u32::from_str_radix(&hex, 16).ok().and_then(char::from_u32).unwrap_or('\u{FFFD}'));
                }
                Some(o) => out.push(o),
                None => {}
            }
        }
        out
    }
    
    fn j_props_escape(s: &str, key: bool) -> String {
        let mut out = String::new();
        for (i, c) in s.chars().enumerate() {
            match c {
                '\\' => out.push_str("\\\\"),
                '\t' => out.push_str("\\t"),
                '\n' => out.push_str("\\n"),
                '\r' => out.push_str("\\r"),
                '\u{c}' => out.push_str("\\f"),
                '=' | ':' | '#' | '!' => {
                    out.push('\\');
                    out.push(c);
                }
                ' ' if key || i == 0 => out.push_str("\\ "),
                c if (c as u32) < 0x20 || (c as u32) > 0x7e => out.push_str(&format!("\\u{:04X}", c as u32)),
                c => out.push(c),
            }
        }
        out
    }
    
    impl JProps {
        pub fn new() -> Self {
            Self::default()
        }
        pub fn get_property(&self, k: &str) -> Option<String> {
            self.0.get(k).cloned()
        }
        pub fn get_property_or(&self, k: &str, default: &str) -> String {
            self.0.get(k).cloned().unwrap_or_else(|| default.to_string())
        }
        pub fn set_property(&mut self, k: &str, v: &str) -> Option<String> {
            self.0.insert(k.to_string(), v.to_string())
        }
        pub fn contains_key(&self, k: &str) -> bool {
            self.0.contains_key(k)
        }
        pub fn remove(&mut self, k: &str) -> Option<String> {
            self.0.remove(k)
        }
        pub fn names(&self) -> JSet<String> {
            JSet::from_iter(self.0.keys().cloned())
        }
        /// Properties.load: `key=value`, `key: value` or `key value`, `#`/`!` comments, `\` continuations.
        pub fn load(&mut self, text: &str) {
            let mut logical = String::new();
            for raw in text.lines() {
                let line = raw.trim_start();
                if logical.is_empty() && (line.is_empty() || line.starts_with('#') || line.starts_with('!')) {
                    continue;
                }
                let trailing = line.chars().rev().take_while(|&c| c == '\\').count();
                if trailing % 2 == 1 {
                    logical.push_str(&line[..line.len() - 1]);
                    continue;
                }
                logical.push_str(line);
                let chars: Vec<char> = logical.chars().collect();
                let mut i = 0;
                while i < chars.len() && !(matches!(chars[i], '=' | ':') || chars[i].is_whitespace()) {
                    i += if chars[i] == '\\' { 2 } else { 1 };
                }
                let key: String = chars[..i.min(chars.len())].iter().collect();
                let mut j = i;
                while j < chars.len() && chars[j].is_whitespace() {
                    j += 1;
                }
                if j < chars.len() && matches!(chars[j], '=' | ':') {
                    j += 1;
                }
                while j < chars.len() && chars[j].is_whitespace() {
                    j += 1;
                }
                let value: String = chars[j.min(chars.len())..].iter().collect();
                self.0.insert(j_props_unescape(&key), j_props_unescape(&value));
                logical.clear();
            }
        }
        /// Properties.store: the comment, then `key=value` lines (Java also writes the date, left out here).
        pub fn store_text(&self, comment: Option<&str>) -> String {
            let mut out = String::new();
            if let Some(c) = comment {
                for l in c.lines() {
                    out.push('#');
                    out.push_str(l);
                    out.push('\n');
                }
            }
            for (k, v) in &self.0 {
                out.push_str(&j_props_escape(k, true));
                out.push('=');
                out.push_str(&j_props_escape(v, false));
                out.push('\n');
            }
            out
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_are_set() {
        assert_eq!(ID, "entity_texture_features");
        assert!(!NAME.is_empty());
        assert!(KEYS.iter().all(|[n, _, d]| !n.is_empty() && d.starts_with("key.")));
    }
}
