// Presence Footsteps, ported to Horizon by horizon-port from PresenceFootsteps-1.14.0+26.3.jar.
// This file is both the bundle's Cargo library (its tests) and the module the bundle adds to
// the game (crate::game::port_presencefootsteps). Put the mod's own logic here, in plain Rust
// that doesn't need the game, and call it from the helpers. No `//!` comments: module payloads
// can't hold inner attributes.

/// The mod's id (its settings folder), name, version and description.
pub const ID: &str = "presencefootsteps";
pub const NAME: &str = "Presence Footsteps";
pub const VERSION: &str = "1.14.0+26.3";
pub const DESCRIPTION: &str = "An overly complicated sound mod.";

/// The mod's keys for Options > Mods: name, label, default key.
pub const KEYS: &[[&str; 3]] = &[
    ["debug_toggle", "Toggle Debug Info", "key.keyboard.unknown"],
    ["settings", "Open Settings", "key.keyboard.unknown"],
    ["toggle", "Toggle Mod", "key.keyboard.unknown"],
];

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

    /// `Reporter` (eu/ha3/mc/quick/update/Reporter.java)
    pub mod reporter {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // Reporter.report(2): not translated, it has no body.
    }

    /// `TargettedVersion` (eu/ha3/mc/quick/update/TargettedVersion.java)
    pub mod targetted_version {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct TargettedVersion {
            pub minecraft: Option<JVersion>,
            pub version: Option<JVersion>,
        }
        impl TargettedVersion {
            pub fn new_2(minecraft: JVersion, version: JVersion) -> Self {
                let mut s = Self { minecraft: Some(minecraft.clone()), version: Some(version.clone()) };
                s.minecraft = Some(minecraft.clone());
                s.version = Some(version.clone());
                s
            }
            // TargettedVersion.TargettedVersion(1): not translated, its constructors are overloaded with the same number of parameters.
            // TargettedVersion.TargettedVersion(1): not translated, its constructors are overloaded with the same number of parameters.
            pub fn minecraft(&self) -> Option<JVersion> {
                return self.minecraft.clone();
            }
            pub fn version(&self) -> Option<JVersion> {
                return self.version.clone();
            }
        }
        // TargettedVersion.getVersion(1): not translated, FabricLoader.getModContainer of a computed id.
    }

    /// `UpdateChecker` (eu/ha3/mc/quick/update/UpdateChecker.java)
    pub mod update_checker {
        use super::*;
        // The class itself isn't translated: it uses Reporter, which isn't translated (it's an interface). Its static parts are below.
        // LOGGER: the logger. Log calls become comments.
        pub static GSON: LazyLock<JGson> = LazyLock::new(|| JGson::default());
        // UpdateChecker.UpdateChecker(4): not translated, its class isn't translated (it uses Reporter, which isn't translated (it's an interface)).
        // UpdateChecker.getLast(0): not translated, its class isn't translated (it uses Reporter, which isn't translated (it's an interface)).
        // UpdateChecker.getNewer(0): not translated, its class isn't translated (it uses Reporter, which isn't translated (it's an interface)).
        // UpdateChecker.attempt(0): not translated, its class isn't translated (it uses Reporter, which isn't translated (it's an interface)).
        // UpdateChecker.checkNow(0): not translated, its class isn't translated (it uses Reporter, which isn't translated (it's an interface)).
    }

    /// `UpdaterConfig` (eu/ha3/mc/quick/update/UpdaterConfig.java)
    pub mod updater_config {
        use super::*;
        // The class itself isn't translated: it extends `JsonFile` (eu.ha3.presencefootsteps.config). Its static parts are below.
        // UpdaterConfig.UpdaterConfig(1): not translated, its class isn't translated (it extends `JsonFile` (eu.ha3.presencefootsteps.config)).
        // UpdaterConfig.shouldReport(1): not translated, its class isn't translated (it extends `JsonFile` (eu.ha3.presencefootsteps.config)).
    }

    /// `Versions` (eu/ha3/mc/quick/update/Versions.java)
    pub mod versions {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Versions {
            pub latest: Option<targetted_version::TargettedVersion>,
            pub previous: Option<Vec<targetted_version::TargettedVersion>>,
        }
        impl Versions {
            pub fn new_2(latest: &targetted_version::TargettedVersion, previous: &[targetted_version::TargettedVersion]) -> Self {
                let mut s = Self { latest: Some(latest.clone()), previous: Some(previous.to_vec()) };
                s.latest = Some(latest.clone());
                s.previous = Some(previous.to_vec());
                s
            }
            // Versions.Versions(1): not translated, it creates a TargettedVersion with a constructor that isn't translated.
            pub fn latest(&self) -> Option<targetted_version::TargettedVersion> {
                return self.latest.clone();
            }
            pub fn previous(&self) -> Option<Vec<targetted_version::TargettedVersion>> {
                return self.previous.clone();
            }
        }
    }

    /// `DerivedBlock` (eu/ha3/presencefootsteps/api/DerivedBlock.java)
    pub mod derived_block {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // DerivedBlock.getBaseOf(1): not translated, it uses Minecraft's `BlockState`.
        // DerivedBlock.getBaseBlockState(0): not translated, it has no body.
    }

    /// `DerivedBlock.Settings` (eu/ha3/presencefootsteps/api/DerivedBlock.java)
    pub mod derived_block_settings {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // DerivedBlock.Settings.getBaseBlock(0): not translated, it has no body.
        // DerivedBlock.Settings.setBaseBlock(1): not translated, it has no body.
    }

    /// `ContraptionCollidable` (eu/ha3/presencefootsteps/compat/ContraptionCollidable.java)
    pub mod contraption_collidable {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ContraptionCollidable.getCollidedStateAt(1): not translated, it has no body.
    }

    /// `EntitySelector` (eu/ha3/presencefootsteps/config/EntitySelector.java)
    pub mod entity_selector {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // VALUES: not translated, it uses EntitySelector, which isn't translated (its constants carry values).
        // EntitySelector.EntitySelector(1): not translated, its class isn't translated (its constants carry values).
        // EntitySelector.getAffectedSources(0): not translated, its class isn't translated (its constants carry values).
    }

    /// `JsonFile` (eu/ha3/presencefootsteps/config/JsonFile.java)
    pub mod json_file {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct JsonFile {
            pub gson: JGson,
            pub file: std::path::PathBuf,
        }
        impl JsonFile {
            // JsonFile.JsonFile(0): not translated, Gson.registerTypeAdapter.
            // JsonFile.JsonFile(1): not translated, Gson.registerTypeAdapter.
            // JsonFile.load(0): not translated, it calls `Files` (java.nio.file).isReadable.
            // JsonFile.load(1): not translated, Gson.fromJson into something other than a class (a TypeToken).
            // JsonFile.save(0): not translated, a try-with-resources without a catch (its errors go up).
        }
    }

    /// `Variator` (eu/ha3/presencefootsteps/config/Variator.java)
    pub mod variator {
        use super::*;
        // The class itself isn't translated: it extends `JsonFile`. Its static parts are below.
    }

    /// `VolumeOption` (eu/ha3/presencefootsteps/config/VolumeOption.java)
    pub mod volume_option {
        use super::*;
        // The class itself isn't translated: it implements `IChangeCallback` (com.minelittlepony.common.client.gui.IField). Its static parts are below.
        // VolumeOption.VolumeOption(2): not translated, its class isn't translated (it implements `IChangeCallback` (com.minelittlepony.common.client.gui.IField)).
        // VolumeOption.get(0): not translated, its class isn't translated (it implements `IChangeCallback` (com.minelittlepony.common.client.gui.IField)).
        // VolumeOption.getPercentage(0): not translated, its class isn't translated (it implements `IChangeCallback` (com.minelittlepony.common.client.gui.IField)).
        // VolumeOption.set(1): not translated, its class isn't translated (it implements `IChangeCallback` (com.minelittlepony.common.client.gui.IField)).
        // VolumeOption.perform(1): not translated, its class isn't translated (it implements `IChangeCallback` (com.minelittlepony.common.client.gui.IField)).
        // VolumeOption.toString(0): not translated, its class isn't translated (it implements `IChangeCallback` (com.minelittlepony.common.client.gui.IField)).
        pub fn volume_scale_to_int(volume: f32) -> i32 {
            return if volume > 97.0f32 { 100 } else { if volume < 3.0f32 { 0 } else { (volume as i32) } };
        }
    }

    /// `PFConfig` (eu/ha3/presencefootsteps/PFConfig.java)
    pub mod pf_config {
        use super::*;
        // The class itself isn't translated: it extends `Config` (com.minelittlepony.common.util.settings). Its static parts are below.
        // PFConfig.PFConfig(2): not translated, its class isn't translated (it extends `Config` (com.minelittlepony.common.util.settings)).
        // PFConfig.toggleMultiplayer(0): not translated, its class isn't translated (it extends `Config` (com.minelittlepony.common.util.settings)).
        // PFConfig.isIgnoredForFootsteps(1): not translated, its class isn't translated (it extends `Config` (com.minelittlepony.common.util.settings)).
        // PFConfig.cycleTargetSelector(0): not translated, its class isn't translated (it extends `Config` (com.minelittlepony.common.util.settings)).
        // PFConfig.setLocomotion(1): not translated, its class isn't translated (it extends `Config` (com.minelittlepony.common.util.settings)).
        // PFConfig.setVisualiserRunning(1): not translated, its class isn't translated (it extends `Config` (com.minelittlepony.common.util.settings)).
        // PFConfig.isVisualiserRunning(0): not translated, its class isn't translated (it extends `Config` (com.minelittlepony.common.util.settings)).
        // PFConfig.getLocomotion(0): not translated, its class isn't translated (it extends `Config` (com.minelittlepony.common.util.settings)).
        // PFConfig.getEntitySelector(0): not translated, its class isn't translated (it extends `Config` (com.minelittlepony.common.util.settings)).
        // PFConfig.getEnabledFootwear(0): not translated, its class isn't translated (it extends `Config` (com.minelittlepony.common.util.settings)).
        // PFConfig.toggleFootwear(0): not translated, its class isn't translated (it extends `Config` (com.minelittlepony.common.util.settings)).
        // PFConfig.isExclusiveMode(0): not translated, its class isn't translated (it extends `Config` (com.minelittlepony.common.util.settings)).
        // PFConfig.toggleExclusiveMode(0): not translated, its class isn't translated (it extends `Config` (com.minelittlepony.common.util.settings)).
        // PFConfig.getEnabledMP(0): not translated, its class isn't translated (it extends `Config` (com.minelittlepony.common.util.settings)).
        // PFConfig.getMaxSteppingEntities(0): not translated, its class isn't translated (it extends `Config` (com.minelittlepony.common.util.settings)).
        // PFConfig.toggleDisabled(0): not translated, its class isn't translated (it extends `Config` (com.minelittlepony.common.util.settings)).
        // PFConfig.setDisabled(1): not translated, its class isn't translated (it extends `Config` (com.minelittlepony.common.util.settings)).
        // PFConfig.getDisabled(0): not translated, its class isn't translated (it extends `Config` (com.minelittlepony.common.util.settings)).
        // PFConfig.getEnabled(0): not translated, its class isn't translated (it extends `Config` (com.minelittlepony.common.util.settings)).
        // PFConfig.getGlobalVolume(0): not translated, its class isn't translated (it extends `Config` (com.minelittlepony.common.util.settings)).
        // PFConfig.getRunningVolumeIncrease(0): not translated, its class isn't translated (it extends `Config` (com.minelittlepony.common.util.settings)).
        // PFConfig.setGlobalVolume(1): not translated, its class isn't translated (it extends `Config` (com.minelittlepony.common.util.settings)).
        // PFConfig.setRunningVolumeIncrease(1): not translated, its class isn't translated (it extends `Config` (com.minelittlepony.common.util.settings)).
        // PFConfig.populateCrashReport(1): not translated, its class isn't translated (it extends `Config` (com.minelittlepony.common.util.settings)).
        pub fn volume_scale_to_int(volume: f32) -> i32 {
            return if volume > 97.0f32 { 100 } else { if volume < 3.0f32 { 0 } else { (volume as i32) } };
        }
    }

    /// `PFDebugHud` (eu/ha3/presencefootsteps/PFDebugHud.java)
    pub mod pf_debug_hud {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `DebugScreenEntry`. Its static parts are below.
        pub static ID: LazyLock<JIdentifier> = LazyLock::new(|| presence_footsteps::id("presence_footsteps_sounds"));
        // PFDebugHud.PFDebugHud(1): not translated, its class isn't translated (it implements Minecraft's `DebugScreenEntry`).
        // PFDebugHud.isAllowed(1): not translated, its class isn't translated (it implements Minecraft's `DebugScreenEntry`).
        // PFDebugHud.display(4): not translated, its class isn't translated (it implements Minecraft's `DebugScreenEntry`).
        // PFDebugHud.renderSoundList(2): not translated, its class isn't translated (it implements Minecraft's `DebugScreenEntry`).
    }

    /// `PFModMenuFactory` (eu/ha3/presencefootsteps/PFModMenuFactory.java)
    pub mod pf_mod_menu_factory {
        use super::*;
        // The class itself isn't translated: it implements `ModMenuApi` (com.terraformersmc.modmenu.api). Its static parts are below.
        // PFModMenuFactory.getModConfigScreenFactory(0): not translated, its class isn't translated (it implements `ModMenuApi` (com.terraformersmc.modmenu.api)).
    }

    /// `PFOptionsScreen` (eu/ha3/presencefootsteps/PFOptionsScreen.java)
    pub mod pf_options_screen {
        use super::*;
        // The class itself isn't translated: it extends `GameGui` (com.minelittlepony.common.client.gui). Its static parts are below.
        // TITLE: a game value, so it's in game_side.rs.
        // VOLUME_MIN: a game value, so it's in game_side.rs.
        // PFOptionsScreen.PFOptionsScreen(1): not translated, its class isn't translated (it extends `GameGui` (com.minelittlepony.common.client.gui)).
        // PFOptionsScreen.init(0): not translated, its class isn't translated (it extends `GameGui` (com.minelittlepony.common.client.gui)).
        // PFOptionsScreen.rebuildContent(0): not translated, its class isn't translated (it extends `GameGui` (com.minelittlepony.common.client.gui)).
        // PFOptionsScreen.addVolumeSlider(4): not translated, its class isn't translated (it extends `GameGui` (com.minelittlepony.common.client.gui)).
        // PFOptionsScreen.updateDisableState(2): not translated, its class isn't translated (it extends `GameGui` (com.minelittlepony.common.client.gui)).
        // PFOptionsScreen.formatVolume(1): not translated, its class isn't translated (it extends `GameGui` (com.minelittlepony.common.client.gui)).
        // PFOptionsScreen.extractRenderState(4): not translated, its class isn't translated (it extends `GameGui` (com.minelittlepony.common.client.gui)).
        // PFOptionsScreen.formatVolume(1): not translated, its class isn't translated (it extends `GameGui` (com.minelittlepony.common.client.gui)).
    }

    /// `PresenceFootsteps` (eu/ha3/presencefootsteps/PresenceFootsteps.java)
    pub mod presence_footsteps {
        use super::*;
        // The class itself isn't translated: it implements Fabric's `ClientModInitializer`. Its static parts are below.
        // LOGGER: the logger. Log calls become comments.
        pub const MODID: &str = "presencefootsteps";
        // KEY_BINDING_CATEGORY: not translated, it uses Minecraft's `Category`.
        pub const UPDATER_ENDPOINT: &str = "https://raw.githubusercontent.com/Sollace/Presence-Footsteps/master/version/latest.json";
        // MOD_NAME: a game value, so it's in game_side.rs.
        // PF_TOGGLED_TOAST_ID: not translated, it uses Minecraft's `SystemToastId`.
        // instance: not translated, it uses PresenceFootsteps, which isn't translated (it implements Fabric's `ClientModInitializer`).
        pub fn id(name: &str) -> JIdentifier {
            return JIdentifier::new("presencefootsteps", &name);
        }
        // PresenceFootsteps.getInstance(0): not translated, it uses PresenceFootsteps, which isn't translated (it implements Fabric's `ClientModInitializer`).
        // PresenceFootsteps.PresenceFootsteps(0): not translated, its class isn't translated (it implements Fabric's `ClientModInitializer`).
        // PresenceFootsteps.getDebugHud(0): not translated, its class isn't translated (it implements Fabric's `ClientModInitializer`).
        // PresenceFootsteps.getEngine(0): not translated, its class isn't translated (it implements Fabric's `ClientModInitializer`).
        // PresenceFootsteps.getConfig(0): not translated, its class isn't translated (it implements Fabric's `ClientModInitializer`).
        // PresenceFootsteps.getOptionsKeyBinding(0): not translated, its class isn't translated (it implements Fabric's `ClientModInitializer`).
        // PresenceFootsteps.getUpdateChecker(0): not translated, its class isn't translated (it implements Fabric's `ClientModInitializer`).
        // PresenceFootsteps.onInitializeClient(0): not translated, its class isn't translated (it implements Fabric's `ClientModInitializer`).
        // PresenceFootsteps.onTick(1): not translated, its class isn't translated (it implements Fabric's `ClientModInitializer`).
        // PresenceFootsteps.onEnabledStateChange(1): not translated, its class isn't translated (it implements Fabric's `ClientModInitializer`).
        // PresenceFootsteps.showSystemToast(2): not translated, its class isn't translated (it implements Fabric's `ClientModInitializer`).
    }

    /// `Acoustic` (eu/ha3/presencefootsteps/sound/acoustics/Acoustic.java)
    pub mod acoustic {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // Acoustic.register(2): not translated, it's generic.
        // Acoustic.read(2): not translated, it uses Acoustic, which isn't translated (it's an interface).
        // Acoustic.read(3): not translated, it uses Acoustic, which isn't translated (it's an interface).
        // Acoustic.getType(2): not translated, an instance field used from a static method.
        // Acoustic.checked(2): not translated, it's generic.
        // Acoustic.playSound(4): not translated, it has no body.
        // Acoustic.type(0): not translated, it has no body.
    }

    /// `Acoustic.Serializer` (eu/ha3/presencefootsteps/sound/acoustics/Acoustic.java)
    pub mod acoustic_serializer {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // Acoustic.Serializer.create(2): not translated, it has no body.
        // Acoustic.Serializer.ofJsObject(1): not translated, it uses Serializer, which isn't translated (it's an interface).
    }

    /// `AcousticLibrary` (eu/ha3/presencefootsteps/sound/acoustics/AcousticLibrary.java)
    pub mod acoustic_library {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // AcousticLibrary.load(1): not translated, it has no body.
        // AcousticLibrary.playStep(3): not translated, it has no body.
        // AcousticLibrary.playAcoustic(4): not translated, it has no body.
        // AcousticLibrary.think(0): not translated, it has no body.
    }

    /// `AcousticsFile` (eu/ha3/presencefootsteps/sound/acoustics/AcousticsFile.java)
    pub mod acoustics_file {
        use super::*;
        pub static FILE_LOCATION: LazyLock<JIdentifier> = LazyLock::new(|| presence_footsteps::id("config/acoustics.json"));
        pub const ENGINE_VERSION: i32 = 2;
        #[derive(Clone, Debug, PartialEq)]
        pub struct AcousticsFile {
            pub default_volume: range::Range,
            pub default_pitch: range::Range,
            pub sound_root: String,
        }
        impl AcousticsFile {
            pub fn new(default_volume: &range::Range, default_pitch: &range::Range, sound_root: &str) -> Self {
                let mut s = Self { default_volume: default_volume.clone(), default_pitch: default_pitch.clone(), sound_root: String::new() };
                s.default_volume = default_volume.clone();
                s.default_pitch = default_pitch.clone();
                s.sound_root = sound_root.to_string();
                s
            }
            pub fn get_sound_name(&self, sound_name: &str) -> String {
                return if j_char_at(&sound_name, 0) != '@' { format!("{}{}", self.sound_root, sound_name) } else { sound_name.replace("@", "") };
            }
            pub fn default_volume(&self) -> range::Range {
                return self.default_volume.clone();
            }
            pub fn default_pitch(&self) -> range::Range {
                return self.default_pitch.clone();
            }
            pub fn sound_root(&self) -> String {
                return self.sound_root.to_string();
            }
        }
        // AcousticsFile.read(3): not translated, it uses Acoustic, which isn't translated (it's an interface).
        pub fn read_2(json: &JJson, ignore_version: bool) -> acoustics_file::AcousticsFile {
            acoustics_file::expect(j_eq("library", &(((json.get("type")).expect("null")).as_string())), &(format!("Invalid type: Expected \"library\" got \"{}\"", ((json.get("type")).expect("null")).as_string())));
            acoustics_file::expect(ignore_version || ((((json.get("engineversion")).expect("null")).as_int()) == 2), &(format!("Unrecognised Engine version: 2 expected, got {}", ((json.get("engineversion")).expect("null")).as_int())));
            acoustics_file::expect(json.has("contents"), "Empty contents");
            let mut sound_root: String = "".to_string();
            if json.has("soundroot") {
                sound_root = ((json.get("soundroot")).expect("null")).as_string();
            }
            if json.has("defaults") {
                let defaults: JJson = (json.member_object("defaults")).expect("null");
                return acoustics_file::AcousticsFile::new(&((*range::DEFAULT).read("volume", &defaults)), &((*range::DEFAULT).read("pitch", &defaults)), &sound_root);
            } else {
                return acoustics_file::AcousticsFile::new(&(*range::DEFAULT), &(*range::DEFAULT), &sound_root);
            }
            #[allow(unreachable_code)]
            unreachable!()
        }
        pub fn expect(condition: bool, message: &str) {
            if !condition {
                panic!("{}", message);
            }
        }
    }

    /// `AcousticsPlayer` (eu/ha3/presencefootsteps/sound/acoustics/AcousticsPlayer.java)
    pub mod acoustics_player {
        use super::*;
        // The class itself isn't translated: it implements `AcousticLibrary`. Its static parts are below.
        // AcousticsPlayer.AcousticsPlayer(1): not translated, its class isn't translated (it implements `AcousticLibrary`).
        // AcousticsPlayer.load(1): not translated, its class isn't translated (it implements `AcousticLibrary`).
        // AcousticsPlayer.addAcoustic(2): not translated, its class isn't translated (it implements `AcousticLibrary`).
        // AcousticsPlayer.playStep(3): not translated, its class isn't translated (it implements `AcousticLibrary`).
        // AcousticsPlayer.playVanillaStep(2): not translated, its class isn't translated (it implements `AcousticLibrary`).
        // AcousticsPlayer.playAcoustic(4): not translated, its class isn't translated (it implements `AcousticLibrary`).
        // AcousticsPlayer.think(0): not translated, its class isn't translated (it implements `AcousticLibrary`).
    }

    /// `ChanceAcoustic` (eu/ha3/presencefootsteps/sound/acoustics/ChanceAcoustic.java)
    pub mod chance_acoustic {
        use super::*;
        // The class itself isn't translated: it implements `Acoustic`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // FACTORY: not translated, it uses Serializer, which isn't translated (it's an interface).
        // ChanceAcoustic.ChanceAcoustic(2): not translated, its class isn't translated (it implements `Acoustic`).
        // ChanceAcoustic.type(0): not translated, its class isn't translated (it implements `Acoustic`).
        // ChanceAcoustic.playSound(4): not translated, its class isn't translated (it implements `Acoustic`).
        // ChanceAcoustic.acoustic(0): not translated, its class isn't translated (it implements `Acoustic`).
        // ChanceAcoustic.probability(0): not translated, its class isn't translated (it implements `Acoustic`).
    }

    /// `DelayedAcoustic` (eu/ha3/presencefootsteps/sound/acoustics/DelayedAcoustic.java)
    pub mod delayed_acoustic {
        use super::*;
        // The class itself isn't translated: it implements `Acoustic`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // FACTORY: not translated, it uses Serializer, which isn't translated (it's an interface).
        // DelayedAcoustic.DelayedAcoustic(2): not translated, its class isn't translated (it implements `Acoustic`).
        // DelayedAcoustic.type(0): not translated, its class isn't translated (it implements `Acoustic`).
        // DelayedAcoustic.playSound(4): not translated, its class isn't translated (it implements `Acoustic`).
        // DelayedAcoustic.acoustic(0): not translated, its class isn't translated (it implements `Acoustic`).
        // DelayedAcoustic.delay(0): not translated, its class isn't translated (it implements `Acoustic`).
    }

    /// `EventSelectorAcoustics` (eu/ha3/presencefootsteps/sound/acoustics/EventSelectorAcoustics.java)
    pub mod event_selector_acoustics {
        use super::*;
        // The class itself isn't translated: it implements `Acoustic`. Its static parts are below.
        // MAP_CODEC: not translated, it uses Minecraft's `MapCodec`.
        // CODEC: not translated, its value couldn't be read: an anonymous class.
        // FACTORY: not translated, it uses Serializer, which isn't translated (it's an interface).
        // EventSelectorAcoustics.EventSelectorAcoustics(1): not translated, its class isn't translated (it implements `Acoustic`).
        // EventSelectorAcoustics.type(0): not translated, its class isn't translated (it implements `Acoustic`).
        // EventSelectorAcoustics.playSound(4): not translated, its class isn't translated (it implements `Acoustic`).
        // EventSelectorAcoustics.pairs(0): not translated, its class isn't translated (it implements `Acoustic`).
    }

    /// `SimultaneousAcoustic` (eu/ha3/presencefootsteps/sound/acoustics/SimultaneousAcoustic.java)
    pub mod simultaneous_acoustic {
        use super::*;
        // The class itself isn't translated: it implements `Acoustic`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `Codec`.
        // MAP_CODEC: not translated, it uses Minecraft's `MapCodec`.
        // FACTORY: not translated, it uses Serializer, which isn't translated (it's an interface).
        // SimultaneousAcoustic.SimultaneousAcoustic(1): not translated, its class isn't translated (it implements `Acoustic`).
        // SimultaneousAcoustic.type(0): not translated, its class isn't translated (it implements `Acoustic`).
        // SimultaneousAcoustic.playSound(4): not translated, its class isn't translated (it implements `Acoustic`).
        // SimultaneousAcoustic.acoustics(0): not translated, its class isn't translated (it implements `Acoustic`).
    }

    /// `VaryingAcoustic` (eu/ha3/presencefootsteps/sound/acoustics/VaryingAcoustic.java)
    pub mod varying_acoustic {
        use super::*;
        // The class itself isn't translated: it implements `Acoustic`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // FACTORY: not translated, it uses Serializer, which isn't translated (it's an interface).
        // VaryingAcoustic.VaryingAcoustic(3): not translated, its class isn't translated (it implements `Acoustic`).
        // VaryingAcoustic.type(0): not translated, its class isn't translated (it implements `Acoustic`).
        // VaryingAcoustic.playSound(4): not translated, its class isn't translated (it implements `Acoustic`).
        // VaryingAcoustic.soundName(0): not translated, its class isn't translated (it implements `Acoustic`).
        // VaryingAcoustic.volume(0): not translated, its class isn't translated (it implements `Acoustic`).
        // VaryingAcoustic.pitch(0): not translated, its class isn't translated (it implements `Acoustic`).
    }

    /// `Locomotion` (eu/ha3/presencefootsteps/sound/generator/Locomotion.java)
    pub mod locomotion {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // REGISTRY: not translated, it uses Locomotion, which isn't translated (its constants carry values).
        pub const AUTO_TRANSLATION_KEY: &str = "menu.pf.stance.auto";
        // Locomotion.Locomotion(0): not translated, its class isn't translated (its constants carry values).
        // Locomotion.Locomotion(1): not translated, its class isn't translated (its constants carry values).
        // Locomotion.supplyGenerator(2): not translated, its class isn't translated (its constants carry values).
        // Locomotion.getOptionName(0): not translated, its class isn't translated (its constants carry values).
        // Locomotion.getOptionTooltip(0): not translated, its class isn't translated (its constants carry values).
        // Locomotion.byName(1): not translated, it uses Locomotion, which isn't translated (its constants carry values).
        // Locomotion.forLiving(2): not translated, it uses Locomotion, which isn't translated (its constants carry values).
        // Locomotion.forPlayer(2): not translated, it uses Locomotion, which isn't translated (its constants carry values).
    }

    /// `MineLP` (eu/ha3/presencefootsteps/sound/generator/MineLP.java)
    pub mod mine_lp {
        use super::*;
        pub static CHECK_COMPLETED: JCell<bool> = JCell::new(false);
        pub static HAS_MINE_LP: JCell<bool> = JCell::new(false);
        #[derive(Clone, Debug)]
        pub struct MineLP {
        }
        impl MineLP {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn has_ponies() -> bool {
            if !mine_lp::CHECK_COMPLETED.get() {
                mine_lp::CHECK_COMPLETED.set(true);
                mine_lp::HAS_MINE_LP.set(false);
            }
            return mine_lp::HAS_MINE_LP.get();
        }
        // MineLP.getLocomotion(2): not translated, it uses Locomotion, which isn't translated (its constants carry values).
        // MineLP.getLocomotion(1): not translated, it uses Locomotion, which isn't translated (its constants carry values).
    }

    /// `Modifier` (eu/ha3/presencefootsteps/sound/generator/Modifier.java)
    pub mod modifier {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // Modifier.stepped(3): not translated, its class isn't translated (it's generic).
        // Modifier.reevaluateDistance(2): not translated, its class isn't translated (it's generic).
    }

    /// `MotionTracker` (eu/ha3/presencefootsteps/sound/generator/MotionTracker.java)
    pub mod motion_tracker {
        use super::*;
        // The class itself isn't translated: it uses TerrestrialStepSoundGenerator, which isn't translated (it implements `StepSoundGenerator`). Its static parts are below.
        // MotionTracker.MotionTracker(1): not translated, its class isn't translated (it uses TerrestrialStepSoundGenerator, which isn't translated (it implements `StepSoundGenerator`)).
        // MotionTracker.getMotionX(0): not translated, its class isn't translated (it uses TerrestrialStepSoundGenerator, which isn't translated (it implements `StepSoundGenerator`)).
        // MotionTracker.getMotionY(0): not translated, its class isn't translated (it uses TerrestrialStepSoundGenerator, which isn't translated (it implements `StepSoundGenerator`)).
        // MotionTracker.getMotionZ(0): not translated, its class isn't translated (it uses TerrestrialStepSoundGenerator, which isn't translated (it implements `StepSoundGenerator`)).
        // MotionTracker.getHorizontalSpeed(0): not translated, its class isn't translated (it uses TerrestrialStepSoundGenerator, which isn't translated (it implements `StepSoundGenerator`)).
        // MotionTracker.isStationary(0): not translated, its class isn't translated (it uses TerrestrialStepSoundGenerator, which isn't translated (it implements `StepSoundGenerator`)).
        // MotionTracker.getDistanceTraveled(0): not translated, its class isn't translated (it uses TerrestrialStepSoundGenerator, which isn't translated (it implements `StepSoundGenerator`)).
        // MotionTracker.getFallDistance(0): not translated, its class isn't translated (it uses TerrestrialStepSoundGenerator, which isn't translated (it implements `StepSoundGenerator`)).
        // MotionTracker.simulateMotionData(1): not translated, its class isn't translated (it uses TerrestrialStepSoundGenerator, which isn't translated (it implements `StepSoundGenerator`)).
        // MotionTracker.pickState(3): not translated, its class isn't translated (it uses TerrestrialStepSoundGenerator, which isn't translated (it implements `StepSoundGenerator`)).
        // MotionTracker.getSpeedScalingRatio(1): not translated, its class isn't translated (it uses TerrestrialStepSoundGenerator, which isn't translated (it implements `StepSoundGenerator`)).
        // MotionTracker.accumulateSimulatedDistance(2): not translated, its class isn't translated (it uses TerrestrialStepSoundGenerator, which isn't translated (it implements `StepSoundGenerator`)).
    }

    /// `QuadrupedModifier` (eu/ha3/presencefootsteps/sound/generator/QuadrupedModifier.java)
    pub mod quadruped_modifier {
        use super::*;
        // The class itself isn't translated: it extends `Modifier`. Its static parts are below.
        // QuadrupedModifier.stepped(3): not translated, its class isn't translated (it extends `Modifier`).
        // QuadrupedModifier.reevaluateDistance(2): not translated, its class isn't translated (it extends `Modifier`).
    }

    /// `StepSoundGenerator` (eu/ha3/presencefootsteps/sound/generator/StepSoundGenerator.java)
    pub mod step_sound_generator {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // StepSoundGenerator.getLocalPitch(1): not translated, it has no body.
        // StepSoundGenerator.getLocalVolume(1): not translated, it has no body.
        // StepSoundGenerator.getMotionTracker(0): not translated, it has no body.
        // StepSoundGenerator.generateFootsteps(0): not translated, it has no body.
    }

    /// `TerrestrialStepSoundGenerator` (eu/ha3/presencefootsteps/sound/generator/TerrestrialStepSoundGenerator.java)
    pub mod terrestrial_step_sound_generator {
        use super::*;
        // The class itself isn't translated: it implements `StepSoundGenerator`. Its static parts are below.
        // TerrestrialStepSoundGenerator.TerrestrialStepSoundGenerator(3): not translated, its class isn't translated (it implements `StepSoundGenerator`).
        // TerrestrialStepSoundGenerator.getLocalPitch(1): not translated, its class isn't translated (it implements `StepSoundGenerator`).
        // TerrestrialStepSoundGenerator.getLocalVolume(1): not translated, its class isn't translated (it implements `StepSoundGenerator`).
        // TerrestrialStepSoundGenerator.getMotionTracker(0): not translated, its class isn't translated (it implements `StepSoundGenerator`).
        // TerrestrialStepSoundGenerator.generateFootsteps(0): not translated, its class isn't translated (it implements `StepSoundGenerator`).
        // TerrestrialStepSoundGenerator.simulateStationary(0): not translated, its class isn't translated (it implements `StepSoundGenerator`).
        // TerrestrialStepSoundGenerator.playbackImmobile(0): not translated, its class isn't translated (it implements `StepSoundGenerator`).
        // TerrestrialStepSoundGenerator.updateImmobileState(1): not translated, its class isn't translated (it implements `StepSoundGenerator`).
        // TerrestrialStepSoundGenerator.simulateFootsteps(0): not translated, its class isn't translated (it implements `StepSoundGenerator`).
        // TerrestrialStepSoundGenerator.produceStep(1): not translated, its class isn't translated (it implements `StepSoundGenerator`).
        // TerrestrialStepSoundGenerator.produceStep(2): not translated, its class isn't translated (it implements `StepSoundGenerator`).
        // TerrestrialStepSoundGenerator.hasStoppingConditions(0): not translated, its class isn't translated (it implements `StepSoundGenerator`).
        // TerrestrialStepSoundGenerator.simulateAirborne(0): not translated, its class isn't translated (it implements `StepSoundGenerator`).
        // TerrestrialStepSoundGenerator.isJumping(0): not translated, its class isn't translated (it implements `StepSoundGenerator`).
        // TerrestrialStepSoundGenerator.getOffsetMinus(0): not translated, its class isn't translated (it implements `StepSoundGenerator`).
        // TerrestrialStepSoundGenerator.simulateJumpingLanding(0): not translated, its class isn't translated (it implements `StepSoundGenerator`).
        // TerrestrialStepSoundGenerator.simulateJumping(0): not translated, its class isn't translated (it implements `StepSoundGenerator`).
        // TerrestrialStepSoundGenerator.simulateLanding(0): not translated, its class isn't translated (it implements `StepSoundGenerator`).
        // TerrestrialStepSoundGenerator.simulateBrushes(0): not translated, its class isn't translated (it implements `StepSoundGenerator`).
        // TerrestrialStepSoundGenerator.playStep(2): not translated, its class isn't translated (it implements `StepSoundGenerator`).
        // TerrestrialStepSoundGenerator.playSinglefoot(3): not translated, its class isn't translated (it implements `StepSoundGenerator`).
        // TerrestrialStepSoundGenerator.playMultifoot(2): not translated, its class isn't translated (it implements `StepSoundGenerator`).
    }

    /// `WingedStepSoundGenerator` (eu/ha3/presencefootsteps/sound/generator/WingedStepSoundGenerator.java)
    pub mod winged_step_sound_generator {
        use super::*;
        // The class itself isn't translated: it extends `TerrestrialStepSoundGenerator`. Its static parts are below.
        // SWIFT: not translated, it calls SoundsKey.of, which isn't translated.
        // WING: not translated, it calls SoundsKey.of, which isn't translated.
        // WingedStepSoundGenerator.WingedStepSoundGenerator(3): not translated, its class isn't translated (it extends `TerrestrialStepSoundGenerator`).
        // WingedStepSoundGenerator.generateFootsteps(0): not translated, its class isn't translated (it extends `TerrestrialStepSoundGenerator`).
        // WingedStepSoundGenerator.simulateAirborne(0): not translated, its class isn't translated (it extends `TerrestrialStepSoundGenerator`).
        // WingedStepSoundGenerator.updateImmobileState(1): not translated, its class isn't translated (it extends `TerrestrialStepSoundGenerator`).
        // WingedStepSoundGenerator.getWingSpeed(0): not translated, its class isn't translated (it extends `TerrestrialStepSoundGenerator`).
        // WingedStepSoundGenerator.simulateJumpingLanding(0): not translated, its class isn't translated (it extends `TerrestrialStepSoundGenerator`).
        // WingedStepSoundGenerator.simulateFlying(0): not translated, its class isn't translated (it extends `TerrestrialStepSoundGenerator`).
        // WingedStepSoundGenerator.updateState(3): not translated, its class isn't translated (it extends `TerrestrialStepSoundGenerator`).
    }

    /// `WingedStepSoundGenerator.FlightState` (eu/ha3/presencefootsteps/sound/generator/WingedStepSoundGenerator.java)
    pub mod winged_step_sound_generator_flight_state {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum FlightState {
            Dashing,
            Coasting,
            CoastingStrafing,
            Flying,
            Idle,
            Ascending,
            Descending,
        }
        impl FlightState {
            pub const VALUES: [FlightState; 7] = [FlightState::Dashing, FlightState::Coasting, FlightState::CoastingStrafing, FlightState::Flying, FlightState::Idle, FlightState::Ascending, FlightState::Descending, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    FlightState::Dashing => "DASHING",
                    FlightState::Coasting => "COASTING",
                    FlightState::CoastingStrafing => "COASTING_STRAFING",
                    FlightState::Flying => "FLYING",
                    FlightState::Idle => "IDLE",
                    FlightState::Ascending => "ASCENDING",
                    FlightState::Descending => "DESCENDING",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "DASHING" => FlightState::Dashing,
                    "COASTING" => FlightState::Coasting,
                    "COASTING_STRAFING" => FlightState::CoastingStrafing,
                    "FLYING" => FlightState::Flying,
                    "IDLE" => FlightState::Idle,
                    "ASCENDING" => FlightState::Ascending,
                    "DESCENDING" => FlightState::Descending,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `Isolator` (eu/ha3/presencefootsteps/sound/Isolator.java)
    pub mod isolator {
        use super::*;
        // The class itself isn't translated: it implements `Reportable`. Its static parts are below.
        pub static BIOME_MAP: LazyLock<JIdentifier> = LazyLock::new(|| presence_footsteps::id("config/biomevariancemap.json"));
        pub static GOLEM_MAP: LazyLock<JIdentifier> = LazyLock::new(|| presence_footsteps::id("config/golemmap.json"));
        pub static LOCOMOTION_MAP: LazyLock<JIdentifier> = LazyLock::new(|| presence_footsteps::id("config/locomotionmap.json"));
        pub static PRIMITIVE_MAP: LazyLock<JIdentifier> = LazyLock::new(|| presence_footsteps::id("config/primitivemap.json"));
        pub static ACOUSTICS: LazyLock<JIdentifier> = LazyLock::new(|| presence_footsteps::id("config/acoustics"));
        pub static VARIATOR: LazyLock<JIdentifier> = LazyLock::new(|| presence_footsteps::id("config/variator.json"));
        // Isolator.Isolator(9): not translated, its class isn't translated (it implements `Reportable`).
        // Isolator.Isolator(1): not translated, its class isn't translated (it implements `Reportable`).
        // Isolator.blocks(1): not translated, its class isn't translated (it implements `Reportable`).
        // Isolator.load(1): not translated, its class isn't translated (it implements `Reportable`).
        // Isolator.writeToReport(3): not translated, its class isn't translated (it implements `Reportable`).
        // Isolator.variator(0): not translated, its class isn't translated (it implements `Reportable`).
        // Isolator.locomotions(0): not translated, its class isn't translated (it implements `Reportable`).
        // Isolator.heuristics(0): not translated, its class isn't translated (it implements `Reportable`).
        // Isolator.golems(0): not translated, its class isn't translated (it implements `Reportable`).
        // Isolator.blocks(0): not translated, its class isn't translated (it implements `Reportable`).
        // Isolator.biomes(0): not translated, its class isn't translated (it implements `Reportable`).
        // Isolator.primitives(0): not translated, its class isn't translated (it implements `Reportable`).
        // Isolator.acoustics(0): not translated, its class isn't translated (it implements `Reportable`).
        // Isolator.empty(0): not translated, its class isn't translated (it implements `Reportable`).
    }

    /// `Options` (eu/ha3/presencefootsteps/sound/Options.java)
    pub mod options {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // Options.singular(2): not translated, it uses Options, which isn't translated (it's an interface).
        // Options.ofGetter(2): not translated, its body couldn't be read: an anonymous class.
        // Options.containsKey(1): not translated, it has no body.
        // Options.get(1): not translated, it has no body.
        // Options.getOrDefault(2): not translated, its class isn't translated (it's an interface).
        // Options.and(1): not translated, its body couldn't be read: an anonymous class.
    }

    /// `Options.FloatSupplier` (eu/ha3/presencefootsteps/sound/Options.java)
    pub mod options_float_supplier {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // Options.FloatSupplier.get(0): not translated, it has no body.
    }

    /// `DelayedSoundPlayer` (eu/ha3/presencefootsteps/sound/player/DelayedSoundPlayer.java)
    pub mod delayed_sound_player {
        use super::*;
        // The class itself isn't translated: it implements `SoundPlayer`. Its static parts are below.
        pub const USING_LATENESS: bool = true;
        pub const USING_EARLYNESS: bool = true;
        pub const LATENESS_THRESHOLD: f32 = 1.5f32;
        pub const EARLYNESS_THRESHOLD_POW: f64 = 0.75f64;
        // DelayedSoundPlayer.DelayedSoundPlayer(1): not translated, its class isn't translated (it implements `SoundPlayer`).
        // DelayedSoundPlayer.getRNG(0): not translated, its class isn't translated (it implements `SoundPlayer`).
        // DelayedSoundPlayer.playSound(5): not translated, its class isn't translated (it implements `SoundPlayer`).
        // DelayedSoundPlayer.think(0): not translated, its class isn't translated (it implements `SoundPlayer`).
    }

    /// `DelayedSoundPlayer.PendingSound` (eu/ha3/presencefootsteps/sound/player/DelayedSoundPlayer.java)
    pub mod delayed_sound_player_pending_sound {
        use super::*;
        // The class itself isn't translated: it uses `WeakReference` (java.lang.ref). Its static parts are below.
        // DelayedSoundPlayer.PendingSound.PendingSound(5): not translated, its class isn't translated (it uses `WeakReference` (java.lang.ref)).
        // DelayedSoundPlayer.PendingSound.tick(0): not translated, its class isn't translated (it uses `WeakReference` (java.lang.ref)).
        // DelayedSoundPlayer.PendingSound.nextState(1): not translated, its class isn't translated (it uses `WeakReference` (java.lang.ref)).
    }

    /// `DelayedSoundPlayer.State` (eu/ha3/presencefootsteps/sound/player/DelayedSoundPlayer.java)
    pub mod delayed_sound_player_state {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum State {
            Pending,
            Playing,
            Skipping,
        }
        impl State {
            pub const VALUES: [State; 3] = [State::Pending, State::Playing, State::Skipping, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    State::Pending => "PENDING",
                    State::Playing => "PLAYING",
                    State::Skipping => "SKIPPING",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "PENDING" => State::Pending,
                    "PLAYING" => State::Playing,
                    "SKIPPING" => State::Skipping,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `ImmediateSoundPlayer` (eu/ha3/presencefootsteps/sound/player/ImmediateSoundPlayer.java)
    pub mod immediate_sound_player {
        use super::*;
        // The class itself isn't translated: it implements `SoundPlayer`. Its static parts are below.
        // ImmediateSoundPlayer.ImmediateSoundPlayer(1): not translated, its class isn't translated (it implements `SoundPlayer`).
        // ImmediateSoundPlayer.getRNG(0): not translated, its class isn't translated (it implements `SoundPlayer`).
        // ImmediateSoundPlayer.playSound(5): not translated, its class isn't translated (it implements `SoundPlayer`).
    }

    /// `ImmediateSoundPlayer.UncappedSoundInstance` (eu/ha3/presencefootsteps/sound/player/ImmediateSoundPlayer.java)
    pub mod immediate_sound_player_uncapped_sound_instance {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `SimpleSoundInstance`. Its static parts are below.
        // ImmediateSoundPlayer.UncappedSoundInstance.UncappedSoundInstance(4): not translated, its class isn't translated (it extends Minecraft's `SimpleSoundInstance`).
        // ImmediateSoundPlayer.UncappedSoundInstance.getMaxVolume(0): not translated, its class isn't translated (it extends Minecraft's `SimpleSoundInstance`).
        // ImmediateSoundPlayer.UncappedSoundInstance.getSoundId(2): not translated, it uses Minecraft's `Entity`.
    }

    /// `SoundPlayer` (eu/ha3/presencefootsteps/sound/player/SoundPlayer.java)
    pub mod sound_player {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // SoundPlayer.playSound(5): not translated, it has no body.
        // SoundPlayer.getRNG(0): not translated, it has no body.
        // SoundPlayer.think(0): not translated, its class isn't translated (it's an interface).
    }

    /// `SoundEngine` (eu/ha3/presencefootsteps/sound/SoundEngine.java)
    pub mod sound_engine {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `PreparableReloadListener`. Its static parts are below.
        pub static ID: LazyLock<JIdentifier> = LazyLock::new(|| presence_footsteps::id("sounds"));
        pub static DEBUG_VISUALISER_ID: LazyLock<JIdentifier> = LazyLock::new(|| presence_footsteps::id("presence_footsteps_sound_visualiser"));
        // SoundEngine.SoundEngine(1): not translated, its class isn't translated (it implements Minecraft's `PreparableReloadListener`).
        // SoundEngine.getVolumeForSource(1): not translated, its class isn't translated (it implements Minecraft's `PreparableReloadListener`).
        // SoundEngine.getIsolator(0): not translated, its class isn't translated (it implements Minecraft's `PreparableReloadListener`).
        // SoundEngine.getSolver(0): not translated, its class isn't translated (it implements Minecraft's `PreparableReloadListener`).
        // SoundEngine.getConfig(0): not translated, its class isn't translated (it implements Minecraft's `PreparableReloadListener`).
        // SoundEngine.reload(0): not translated, its class isn't translated (it implements Minecraft's `PreparableReloadListener`).
        // SoundEngine.isEnabledFor(1): not translated, its class isn't translated (it implements Minecraft's `PreparableReloadListener`).
        // SoundEngine.hasData(0): not translated, its class isn't translated (it implements Minecraft's `PreparableReloadListener`).
        // SoundEngine.isRunning(1): not translated, its class isn't translated (it implements Minecraft's `PreparableReloadListener`).
        // SoundEngine.isActive(1): not translated, its class isn't translated (it implements Minecraft's `PreparableReloadListener`).
        // SoundEngine.getTargets(1): not translated, its class isn't translated (it implements Minecraft's `PreparableReloadListener`).
        // SoundEngine.onFrame(2): not translated, its class isn't translated (it implements Minecraft's `PreparableReloadListener`).
        // SoundEngine.onSoundRecieved(1): not translated, its class isn't translated (it implements Minecraft's `PreparableReloadListener`).
        // SoundEngine.reload(4): not translated, its class isn't translated (it implements Minecraft's `PreparableReloadListener`).
        // SoundEngine.reloadEverything(1): not translated, its class isn't translated (it implements Minecraft's `PreparableReloadListener`).
        // SoundEngine.shutdown(0): not translated, its class isn't translated (it implements Minecraft's `PreparableReloadListener`).
    }

    /// `SoundSourceUtil` (eu/ha3/presencefootsteps/sound/SoundSourceUtil.java)
    pub mod sound_source_util {
        use super::*;
        // PLAYER_STEP_SOUNDS: not translated, it uses Minecraft's `SoundEvents`.PLAYER_SWIM.
        #[derive(Clone, Debug)]
        pub struct SoundSourceUtil {
        }
        impl SoundSourceUtil {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn is_horse_galloping_sound(id: &JIdentifier) -> bool {
            return id.path.clone().contains("horse.gallop");
        }
        pub fn is_step_sound(id: &JIdentifier) -> bool {
            return id.path.clone().ends_with(".step");
        }
        // SoundSourceUtil.isPlayerStepSound(3): not translated, it uses Minecraft's `SoundSource`.
        // SoundSourceUtil.getStepSoundAtPosition(1): not translated, it uses Minecraft's `SoundEvent`.
    }

    /// `State` (eu/ha3/presencefootsteps/sound/State.java)
    pub mod state {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `EnumCodec`.
        // State.State(1): not translated, its class isn't translated (its constants carry values).
        // State.getSerializedName(0): not translated, its class isn't translated (its constants carry values).
        // State.getName(0): not translated, its class isn't translated (its constants carry values).
        // State.canTransition(0): not translated, its class isn't translated (its constants carry values).
        // State.getTransitionDestination(0): not translated, its class isn't translated (its constants carry values).
        // State.isExtraLoud(0): not translated, its class isn't translated (its constants carry values).
    }

    /// `StepSoundSource` (eu/ha3/presencefootsteps/sound/StepSoundSource.java)
    pub mod step_sound_source {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // StepSoundSource.getStepGenerator(1): not translated, it has no body.
        // StepSoundSource.isStepBlocked(0): not translated, it has no body.
    }

    /// `StepSoundSource.Container` (eu/ha3/presencefootsteps/sound/StepSoundSource.java)
    pub mod step_sound_source_container {
        use super::*;
        // The class itself isn't translated: it implements `StepSoundSource`. Its static parts are below.
        // StepSoundSource.Container.Container(1): not translated, its class isn't translated (it implements `StepSoundSource`).
        // StepSoundSource.Container.getStepGenerator(1): not translated, its class isn't translated (it implements `StepSoundSource`).
        // StepSoundSource.Container.isStepBlocked(0): not translated, its class isn't translated (it implements `StepSoundSource`).
    }

    /// `BlockReport` (eu/ha3/presencefootsteps/util/BlockReport.java)
    pub mod block_report {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // BlockReport.execute(3): not translated, it uses `Object`.
        // BlockReport.execute(3): not translated, it uses `Object`.
        // BlockReport.getUniqueFileName(3): not translated, `null` (Rust has no null: this needs an Option).
    }

    /// `BlockReport.Reportable` (eu/ha3/presencefootsteps/util/BlockReport.java)
    pub mod block_report_reportable {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // BlockReport.Reportable.writeToReport(3): not translated, it has no body.
    }

    /// `BlockReport.UnsafeConsumer` (eu/ha3/presencefootsteps/util/BlockReport.java)
    pub mod block_report_unsafe_consumer {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // BlockReport.UnsafeConsumer.accept(1): not translated, it has no body.
    }

    /// `Edge` (eu/ha3/presencefootsteps/util/Edge.java)
    pub mod edge {
        use super::*;
        // The class itself isn't translated: it implements `BooleanConsumer` (it.unimi.dsi.fastutil.booleans). Its static parts are below.
        // Edge.Edge(1): not translated, its class isn't translated (it implements `BooleanConsumer` (it.unimi.dsi.fastutil.booleans)).
        // Edge.accept(1): not translated, its class isn't translated (it implements `BooleanConsumer` (it.unimi.dsi.fastutil.booleans)).
    }

    /// `JsonObjectWriter` (eu/ha3/presencefootsteps/util/JsonObjectWriter.java)
    pub mod json_object_writer {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // JsonObjectWriter.of(1): not translated, it uses JsonObjectWriter, which isn't translated (it's an interface).
        // JsonObjectWriter.writer(0): not translated, it has no body.
        // JsonObjectWriter.field(2): not translated, its class isn't translated (it's an interface).
        // JsonObjectWriter.field(2): not translated, its class isn't translated (it's an interface).
        // JsonObjectWriter.field(2): not translated, its class isn't translated (it's an interface).
        // JsonObjectWriter.field(2): not translated, its class isn't translated (it's an interface).
        // JsonObjectWriter.field(2): not translated, its class isn't translated (it's an interface).
        // JsonObjectWriter.object(1): not translated, its class isn't translated (it's an interface).
        // JsonObjectWriter.object(2): not translated, its class isn't translated (it's an interface).
        // JsonObjectWriter.array(2): not translated, its class isn't translated (it's an interface).
        // JsonObjectWriter.array(1): not translated, its class isn't translated (it's an interface).
        // JsonObjectWriter.each(2): not translated, it's generic.
        // JsonObjectWriter.close(0): not translated, its class isn't translated (it's an interface).
    }

    /// `JsonObjectWriter.WriteAction` (eu/ha3/presencefootsteps/util/JsonObjectWriter.java)
    pub mod json_object_writer_write_action {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // JsonObjectWriter.WriteAction.write(0): not translated, it has no body.
    }

    /// `JsonObjectWriter.WriteConsumer` (eu/ha3/presencefootsteps/util/JsonObjectWriter.java)
    pub mod json_object_writer_write_consumer {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // JsonObjectWriter.WriteConsumer.write(1): not translated, it has no body.
    }

    /// `Lerp` (eu/ha3/presencefootsteps/util/Lerp.java)
    pub mod lerp {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Lerp {
            pub previous: f32,
            pub current: f32,
        }
        impl Lerp {
            pub fn new() -> Self {
                let mut s = Self { previous: 0.0f32, current: 0.0f32 };
                s
            }
            pub fn update(&mut self, new_target: f32, rate: f32) {
                self.previous = self.current;
                if self.current < new_target {
                    self.current = (self.current + rate).min(new_target);
                }
                if self.current > new_target {
                    self.current = (self.current - rate).max(new_target);
                }
            }
            pub fn get(&self, tick_delta: f32) -> f32 {
                return j_lerp(tick_delta, self.previous, self.current);
            }
        }
    }

    /// `MathUtil` (eu/ha3/presencefootsteps/util/MathUtil.java)
    pub mod math_util {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // MathUtil.randAB(3): not translated, it's overloaded with the same number of parameters.
        // MathUtil.randAB(3): not translated, it's overloaded with the same number of parameters.
        pub fn scalex(number: f32, min: f32, max: f32) -> f32 {
            return j_clamp((number - min) / (max - min), 0.0f32, 1.0f32);
        }
    }

    /// `Period` (eu/ha3/presencefootsteps/util/Period.java)
    pub mod period {
        use super::*;
        // The class itself isn't translated: it implements `Options` (eu.ha3.presencefootsteps.sound). Its static parts are below.
        // RANGE_CODEC: not translated, it uses Minecraft's `Codec`.
        // POINT_CODEC: not translated, it uses Minecraft's `Codec`.
        // CODEC: not translated, it uses Minecraft's `Codec`.
        // ZERO: not translated, it uses Period, which isn't translated (it implements `Options` (eu.ha3.presencefootsteps.sound)).
        // Period.Period(2): not translated, its class isn't translated (it implements `Options` (eu.ha3.presencefootsteps.sound)).
        // Period.of(1): not translated, it uses Period, which isn't translated (it implements `Options` (eu.ha3.presencefootsteps.sound)).
        // Period.of(2): not translated, it uses Period, which isn't translated (it implements `Options` (eu.ha3.presencefootsteps.sound)).
        // Period.fromJson(2): not translated, it uses Period, which isn't translated (it implements `Options` (eu.ha3.presencefootsteps.sound)).
        // Period.random(1): not translated, its class isn't translated (it implements `Options` (eu.ha3.presencefootsteps.sound)).
        // Period.on(1): not translated, its class isn't translated (it implements `Options` (eu.ha3.presencefootsteps.sound)).
        // Period.containsKey(1): not translated, its class isn't translated (it implements `Options` (eu.ha3.presencefootsteps.sound)).
        // Period.get(1): not translated, its class isn't translated (it implements `Options` (eu.ha3.presencefootsteps.sound)).
        // Period.min(0): not translated, its class isn't translated (it implements `Options` (eu.ha3.presencefootsteps.sound)).
        // Period.max(0): not translated, its class isn't translated (it implements `Options` (eu.ha3.presencefootsteps.sound)).
    }

    /// `PlayerUtil` (eu/ha3/presencefootsteps/util/PlayerUtil.java)
    pub mod player_util {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // PlayerUtil.isClientPlayer(1): not translated, it uses Minecraft's `Entity`.
        // PlayerUtil.getScale(1): not translated, it uses Minecraft's `LivingEntity`.
    }

    /// `Range` (eu/ha3/presencefootsteps/util/Range.java)
    pub mod range {
        use super::*;
        // PERCENTAGE_CODEC: not translated, it uses Minecraft's `Codec`.
        // RANGE_CODEC: not translated, it uses Minecraft's `Codec`.
        // POINT_CODEC: not translated, it uses Minecraft's `Codec`.
        // CODEC: not translated, it uses Minecraft's `Codec`.
        pub static DEFAULT: LazyLock<range::Range> = LazyLock::new(|| range::exactly(1.0f32));
        #[derive(Clone, Debug, PartialEq)]
        pub struct Range {
            pub min: f32,
            pub max: f32,
        }
        impl Range {
            pub fn new(min: f32, max: f32) -> Self {
                let mut s = Self { min: min, max: max };
                s.min = min;
                s.max = max;
                s
            }
            pub fn read(&self, name: &str, json: &JJson) -> range::Range {
                if (!j_eq("volume", &name)) || (((!(json.has("vol"))) && (!(json.has("vol_min")))) && (!(json.has("vol_max")))) {
                    if json.has(&name) {
                        let element: JJson = (json.get(&name)).expect("null");
                        return if element.is_json_object() { range::Range::new(range::get_percentage(&element.as_object(), "min", self.min), range::get_percentage(&element.as_object(), "max", self.max)) } else { range::exactly(range::get_percentage(json, &name, self.min)) };
                    } else {
                        return range::Range::new(range::get_percentage(json, &(format!("{}_min", name)), self.min), range::get_percentage(json, &(format!("{}_max", name)), self.max));
                    }
                } else {
                    return self.read("vol", json);
                }
                #[allow(unreachable_code)]
                unreachable!()
            }
            // Range.random(1): not translated, it calls MathUtil.randAB, which isn't translated.
            pub fn on(&self, value: f32) -> f32 {
                return j_lerp(value, self.min, self.max);
            }
            pub fn min(&self) -> f32 {
                return self.min;
            }
            pub fn max(&self) -> f32 {
                return self.max;
            }
        }
        pub fn exactly(value: f32) -> range::Range {
            return range::Range::new(value, value);
        }
        pub fn get_percentage(object: &JJson, param: &str, fallback: f32) -> f32 {
            return if !(object.has(&param)) { fallback } else { (((object.get(&param)).expect("null")).as_float()) / 100.0f32 };
        }
    }

    /// `ResourceUtils` (eu/ha3/presencefootsteps/util/ResourceUtils.java)
    pub mod resource_utils {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ResourceUtils.forEach(3): not translated, it uses Minecraft's `ResourceManager`.
        // ResourceUtils.load(3): not translated, it's generic.
        // ResourceUtils.load(3): not translated, it's generic.
        // ResourceUtils.loadAll(3): not translated, it's generic.
        // ResourceUtils.loadDir(5): not translated, it's generic.
    }

    /// `AbstractSubstrateLookup` (eu/ha3/presencefootsteps/world/AbstractSubstrateLookup.java)
    pub mod abstract_substrate_lookup {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // AbstractSubstrateLookup.AbstractSubstrateLookup(1): not translated, its class isn't translated (it's generic).
        // AbstractSubstrateLookup.getId(1): not translated, it has no body.
        // AbstractSubstrateLookup.getAssociation(2): not translated, its class isn't translated (it's generic).
        // AbstractSubstrateLookup.getSubstrateMap(2): not translated, its class isn't translated (it's generic).
        // AbstractSubstrateLookup.getSubstrates(0): not translated, its class isn't translated (it's generic).
        // AbstractSubstrateLookup.contains(1): not translated, its class isn't translated (it's generic).
        // AbstractSubstrateLookup.contains(2): not translated, its class isn't translated (it's generic).
        // AbstractSubstrateLookup.isEmpty(0): not translated, its class isn't translated (it's generic).
    }

    /// `Association` (eu/ha3/presencefootsteps/world/Association.java)
    pub mod association {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `BlockState`. Its static parts are below.
        // NOT_EMITTER: not translated, it uses Association, which isn't translated (it uses Minecraft's `BlockState`).
        // Association.Association(7): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // Association.of(7): not translated, it uses Association, which isn't translated (it uses Minecraft's `BlockState`).
        // Association.isResult(0): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // Association.isSilent(0): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // Association.dataEquals(1): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // Association.state(0): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // Association.pos(0): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // Association.source(0): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // Association.forcePlay(0): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // Association.dry(0): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // Association.wet(0): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // Association.foliage(0): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
    }

    /// `AssociationPool` (eu/ha3/presencefootsteps/world/AssociationPool.java)
    pub mod association_pool {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `LivingEntity`. Its static parts are below.
        // AssociationPool.AssociationPool(2): not translated, its class isn't translated (it uses Minecraft's `LivingEntity`).
        // AssociationPool.reset(0): not translated, its class isn't translated (it uses Minecraft's `LivingEntity`).
        // AssociationPool.wasLastMatchGolem(0): not translated, its class isn't translated (it uses Minecraft's `LivingEntity`).
        // AssociationPool.findAssociation(2): not translated, its class isn't translated (it uses Minecraft's `LivingEntity`).
        // AssociationPool.findAssociation(2): not translated, its class isn't translated (it uses Minecraft's `LivingEntity`).
        // AssociationPool.get(3): not translated, its class isn't translated (it uses Minecraft's `LivingEntity`).
        // AssociationPool.getForState(2): not translated, its class isn't translated (it uses Minecraft's `LivingEntity`).
        // AssociationPool.getForPrimitive(2): not translated, its class isn't translated (it uses Minecraft's `LivingEntity`).
    }

    /// `BiomeVarianceLookup` (eu/ha3/presencefootsteps/world/BiomeVarianceLookup.java)
    pub mod biome_variance_lookup {
        use super::*;
        // The class itself isn't translated: it implements `Index`. Its static parts are below.
        // BiomeVarianceLookup.lookup(1): not translated, its class isn't translated (it implements `Index`).
        // BiomeVarianceLookup.contains(1): not translated, its class isn't translated (it implements `Index`).
        // BiomeVarianceLookup.createLoader(0): not translated, its class isn't translated (it implements `Index`).
        // BiomeVarianceLookup.writeToReport(3): not translated, its class isn't translated (it implements `Index`).
    }

    /// `BiomeVarianceLookup.BiomeVariance` (eu/ha3/presencefootsteps/world/BiomeVarianceLookup.java)
    pub mod biome_variance_lookup_biome_variance {
        use super::*;
        pub static DEFAULT: LazyLock<biome_variance_lookup_biome_variance::BiomeVariance> = LazyLock::new(|| biome_variance_lookup_biome_variance::BiomeVariance::new(1.0f32, 1.0f32));
        // CODEC: not translated, it uses Minecraft's `Codec`.
        #[derive(Clone, Debug, PartialEq)]
        pub struct BiomeVariance {
            pub volume: f32,
            pub pitch: f32,
        }
        impl BiomeVariance {
            pub fn new(volume: f32, pitch: f32) -> Self {
                let mut s = Self { volume: volume, pitch: pitch };
                s.volume = volume;
                s.pitch = pitch;
                s
            }
            pub fn volume(&self) -> f32 {
                return self.volume;
            }
            pub fn pitch(&self) -> f32 {
                return self.pitch;
            }
        }
    }

    /// `ContextualBlockState` (eu/ha3/presencefootsteps/world/ContextualBlockState.java)
    pub mod contextual_block_state {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `EntityType`. Its static parts are below.
        // CACHE: not translated, it uses Minecraft's `EntityType`.
        // ContextualBlockState.ContextualBlockState(2): not translated, its class isn't translated (it uses Minecraft's `EntityType`).
        // ContextualBlockState.of(2): not translated, it uses ContextualBlockState, which isn't translated (it uses Minecraft's `EntityType`).
        // ContextualBlockState.type(0): not translated, its class isn't translated (it uses Minecraft's `EntityType`).
        // ContextualBlockState.state(0): not translated, its class isn't translated (it uses Minecraft's `EntityType`).
    }

    /// `EntityBlockLookup` (eu/ha3/presencefootsteps/world/EntityBlockLookup.java)
    pub mod entity_block_lookup {
        use super::*;
        // The class itself isn't translated: it uses Lookup, which isn't translated (it's generic). Its static parts are below.
        pub static BLOCK_MAP: LazyLock<JIdentifier> = LazyLock::new(|| presence_footsteps::id("config/blockmap.json"));
        // EntityBlockLookup.getLookup(1): not translated, its class isn't translated (it uses Lookup, which isn't translated (it's generic)).
        // EntityBlockLookup.load(1): not translated, its class isn't translated (it uses Lookup, which isn't translated (it's generic)).
    }

    /// `GolemLookup` (eu/ha3/presencefootsteps/world/GolemLookup.java)
    pub mod golem_lookup {
        use super::*;
        // The class itself isn't translated: it extends `AbstractSubstrateLookup`. Its static parts are below.
        // GolemLookup.GolemLookup(1): not translated, its class isn't translated (it extends `AbstractSubstrateLookup`).
        // GolemLookup.getAssociation(2): not translated, its class isn't translated (it extends `AbstractSubstrateLookup`).
        // GolemLookup.getId(1): not translated, its class isn't translated (it extends `AbstractSubstrateLookup`).
        // GolemLookup.writeToReport(4): not translated, it uses Lookup, which isn't translated (it's generic).
    }

    /// `HeuristicStateLookup` (eu/ha3/presencefootsteps/world/HeuristicStateLookup.java)
    pub mod heuristic_state_lookup {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Block`. Its static parts are below.
        // HeuristicStateLookup.getMostSimilar(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
    }

    /// `Index` (eu/ha3/presencefootsteps/world/Index.java)
    pub mod index {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // Index.lookup(1): not translated, it has no body.
        // Index.contains(1): not translated, it has no body.
        // Index.createLoader(0): not translated, it has no body.
    }

    /// `Index.Loader` (eu/ha3/presencefootsteps/world/Index.java)
    pub mod index_loader {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // Index.Loader.accept(1): not translated, its class isn't translated (it's an interface).
        // Index.Loader.accept(2): not translated, it has no body.
    }

    /// `LocomotionLookup` (eu/ha3/presencefootsteps/world/LocomotionLookup.java)
    pub mod locomotion_lookup {
        use super::*;
        // The class itself isn't translated: it implements `Index`. Its static parts are below.
        // LocomotionLookup.LocomotionLookup(1): not translated, its class isn't translated (it implements `Index`).
        // LocomotionLookup.lookup(1): not translated, its class isn't translated (it implements `Index`).
        // LocomotionLookup.createLoader(0): not translated, its class isn't translated (it implements `Index`).
        // LocomotionLookup.contains(1): not translated, its class isn't translated (it implements `Index`).
        // LocomotionLookup.writeToReport(3): not translated, its class isn't translated (it implements `Index`).
    }

    /// `Lookup` (eu/ha3/presencefootsteps/world/Lookup.java)
    pub mod lookup {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // Lookup.load(2): not translated, its class isn't translated (it's generic).
        // Lookup.load(1): not translated, its class isn't translated (it's generic).
        // Lookup.getAssociation(2): not translated, its class isn't translated (it's generic).
        // Lookup.getSubstrates(0): not translated, its class isn't translated (it's generic).
        // Lookup.getAssociations(1): not translated, its class isn't translated (it's generic).
        // Lookup.contains(1): not translated, its class isn't translated (it's generic).
        // Lookup.contains(2): not translated, its class isn't translated (it's generic).
    }

    /// `Lookup.DataSegment` (eu/ha3/presencefootsteps/world/Lookup.java)
    pub mod lookup_data_segment {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // Lookup.DataSegment.getAssociation(2): not translated, it has no body.
        // Lookup.DataSegment.getSubstrates(0): not translated, it has no body.
        // Lookup.DataSegment.contains(1): not translated, it has no body.
        // Lookup.DataSegment.contains(2): not translated, it has no body.
        // Lookup.DataSegment.isEmpty(0): not translated, it has no body.
    }

    /// `Lookup.UnionDataSegment` (eu/ha3/presencefootsteps/world/Lookup.java)
    pub mod lookup_union_data_segment {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // EMPTY: not translated, it uses UnionDataSegment, which isn't translated (it's generic).
        // Lookup.UnionDataSegment.UnionDataSegment(2): not translated, its class isn't translated (it's generic).
        // Lookup.UnionDataSegment.empty(0): not translated, it's generic.
        // Lookup.UnionDataSegment.of(1): not translated, it's generic.
        // Lookup.UnionDataSegment.getAssociation(2): not translated, its class isn't translated (it's generic).
        // Lookup.UnionDataSegment.getSubstrates(0): not translated, its class isn't translated (it's generic).
        // Lookup.UnionDataSegment.contains(1): not translated, its class isn't translated (it's generic).
        // Lookup.UnionDataSegment.contains(2): not translated, its class isn't translated (it's generic).
        // Lookup.UnionDataSegment.isEmpty(0): not translated, its class isn't translated (it's generic).
        // Lookup.UnionDataSegment.entries(0): not translated, its class isn't translated (it's generic).
        // Lookup.UnionDataSegment.substrates(0): not translated, its class isn't translated (it's generic).
    }

    /// `PFSolver` (eu/ha3/presencefootsteps/world/PFSolver.java)
    pub mod pf_solver {
        use super::*;
        // The class itself isn't translated: it implements `Solver`. Its static parts are below.
        pub const TRAP_DOOR_OFFSET: f64 = 0.1f64;
        // PFSolver.PFSolver(1): not translated, its class isn't translated (it implements `Solver`).
        // PFSolver.getBlockStateAt(2): not translated, its class isn't translated (it implements `Solver`).
        // PFSolver.getCollider(1): not translated, its class isn't translated (it implements `Solver`).
        // PFSolver.checkCollision(4): not translated, its class isn't translated (it implements `Solver`).
        // PFSolver.findAssociation(4): not translated, its class isn't translated (it implements `Solver`).
        // PFSolver.findAssociation(4): not translated, its class isn't translated (it implements `Solver`).
        // PFSolver.findAssociation(5): not translated, its class isn't translated (it implements `Solver`).
        // PFSolver.findAssociation(4): not translated, its class isn't translated (it implements `Solver`).
    }

    /// `PrimitiveLookup` (eu/ha3/presencefootsteps/world/PrimitiveLookup.java)
    pub mod primitive_lookup {
        use super::*;
        // The class itself isn't translated: it extends `AbstractSubstrateLookup`. Its static parts are below.
        // PrimitiveLookup.PrimitiveLookup(1): not translated, its class isn't translated (it extends `AbstractSubstrateLookup`).
        // PrimitiveLookup.getId(1): not translated, its class isn't translated (it extends `AbstractSubstrateLookup`).
        // PrimitiveLookup.writeToReport(4): not translated, it uses Lookup, which isn't translated (it's generic).
        // PrimitiveLookup.getSubstrate(1): not translated, it uses Minecraft's `SoundType`.
        // PrimitiveLookup.getKey(1): not translated, it uses Minecraft's `SoundType`.
    }

    /// `Solver` (eu/ha3/presencefootsteps/world/Solver.java)
    pub mod solver {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // Solver.findAssociation(4): not translated, it has no body.
        // Solver.findAssociation(4): not translated, it has no body.
    }

    /// `SoundsKey` (eu/ha3/presencefootsteps/world/SoundsKey.java)
    pub mod sounds_key {
        use super::*;
        // INTERNER: not translated, it uses `Interner` (com.google.common.collect).
        // UNASSIGNED: not translated, it uses SoundsKey.INTERNER, which isn't translated (it uses `Interner` (com.google.common.collect)).
        // NON_EMITTER: not translated, it uses SoundsKey.INTERNER, which isn't translated (it uses `Interner` (com.google.common.collect)).
        // MESSY_GROUND: not translated, it uses SoundsKey.INTERNER, which isn't translated (it uses `Interner` (com.google.common.collect)).
        // VANILLA: not translated, it uses SoundsKey.INTERNER, which isn't translated (it uses `Interner` (com.google.common.collect)).
        // SWIM_WATER: not translated, it calls SoundsKey.of, which isn't translated.
        // SWIM_LAVA: not translated, it calls SoundsKey.of, which isn't translated.
        // WATERFINE: not translated, it calls SoundsKey.of, which isn't translated.
        // LAVAFINE: not translated, it calls SoundsKey.of, which isn't translated.
        #[derive(Clone, Debug, PartialEq)]
        pub struct SoundsKey {
            pub raw: Option<String>,
            pub names: Option<Vec<String>>,
        }
        impl SoundsKey {
            pub fn new_2(raw: &str, names: &[String]) -> Self {
                let mut s = Self { raw: Some(raw.to_string()), names: Some(names.to_vec()) };
                s.raw = Some(raw.to_string());
                s.names = Some(names.to_vec());
                s
            }
            // SoundsKey.SoundsKey(1): not translated, array .isEmpty().
            // SoundsKey.isResult(0): not translated, `this` used as a value.
            // SoundsKey.isSilent(0): not translated, `this` used as a value.
            // SoundsKey.isVanilla(0): not translated, `this` used as a value.
            // SoundsKey.isEmitter(0): not translated, it calls SoundsKey.isSilent, which isn't translated.
            pub fn raw(&self) -> Option<String> {
                return self.raw.clone();
            }
            pub fn names(&self) -> Option<Vec<String>> {
                return self.names.clone();
            }
        }
        // SoundsKey.of(1): not translated, it uses SoundsKey.MESSY_GROUND, which isn't translated (it uses SoundsKey.INTERNER, which isn't translated (it uses `Interner` (com.google.common.collect))).
    }

    /// `StateLookup` (eu/ha3/presencefootsteps/world/StateLookup.java)
    pub mod state_lookup {
        use super::*;
        // The class itself isn't translated: it implements `DataSegment`. Its static parts are below.
        // StateLookup.StateLookup(0): not translated, its class isn't translated (it implements `DataSegment`).
        // StateLookup.StateLookup(1): not translated, its class isn't translated (it implements `DataSegment`).
        // StateLookup.getAssociation(2): not translated, its class isn't translated (it implements `DataSegment`).
        // StateLookup.getSubstrates(0): not translated, its class isn't translated (it implements `DataSegment`).
        // StateLookup.contains(1): not translated, its class isn't translated (it implements `DataSegment`).
        // StateLookup.contains(2): not translated, its class isn't translated (it implements `DataSegment`).
        // StateLookup.isEmpty(0): not translated, its class isn't translated (it implements `DataSegment`).
        // StateLookup.writeToReport(4): not translated, it uses Lookup, which isn't translated (it's generic).
        // StateLookup.getSoundData(1): not translated, it uses Minecraft's `SoundType`.
        // StateLookup.getClassData(1): not translated, it uses Minecraft's `BlockState`.
        // StateLookup.getTagData(1): not translated, it uses Minecraft's `BlockState`.
        // StateLookup.substrates(0): not translated, its class isn't translated (it implements `DataSegment`).
    }

    /// `StateLookup.Bucket` (eu/ha3/presencefootsteps/world/StateLookup.java)
    pub mod state_lookup_bucket {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // StateLookup.Bucket.add(1): not translated, its class isn't translated (it's an interface).
        // StateLookup.Bucket.get(1): not translated, it has no body.
        // StateLookup.Bucket.contains(1): not translated, its class isn't translated (it's an interface).
    }

    /// `StateLookup.Bucket.Substrate` (eu/ha3/presencefootsteps/world/StateLookup.java)
    pub mod state_lookup_bucket_substrate {
        use super::*;
        // The class itself isn't translated: it implements `Bucket`. Its static parts are below.
        // StateLookup.Bucket.Substrate.Substrate(3): not translated, its class isn't translated (it implements `Bucket`).
        // StateLookup.Bucket.Substrate.Substrate(1): not translated, its class isn't translated (it implements `Bucket`).
        // StateLookup.Bucket.Substrate.add(1): not translated, its class isn't translated (it implements `Bucket`).
        // StateLookup.Bucket.Substrate.get(1): not translated, its class isn't translated (it implements `Bucket`).
        // StateLookup.Bucket.Substrate.contains(1): not translated, its class isn't translated (it implements `Bucket`).
        // StateLookup.Bucket.Substrate.getTile(1): not translated, its class isn't translated (it implements `Bucket`).
        // StateLookup.Bucket.Substrate.wildcards(0): not translated, its class isn't translated (it implements `Bucket`).
        // StateLookup.Bucket.Substrate.blocks(0): not translated, its class isn't translated (it implements `Bucket`).
        // StateLookup.Bucket.Substrate.tags(0): not translated, its class isn't translated (it implements `Bucket`).
    }

    /// `StateLookup.Bucket.Tile` (eu/ha3/presencefootsteps/world/StateLookup.java)
    pub mod state_lookup_bucket_tile {
        use super::*;
        // The class itself isn't translated: it implements `Bucket`. Its static parts are below.
        // StateLookup.Bucket.Tile.Tile(2): not translated, its class isn't translated (it implements `Bucket`).
        // StateLookup.Bucket.Tile.Tile(1): not translated, its class isn't translated (it implements `Bucket`).
        // StateLookup.Bucket.Tile.add(1): not translated, its class isn't translated (it implements `Bucket`).
        // StateLookup.Bucket.Tile.get(1): not translated, its class isn't translated (it implements `Bucket`).
        // StateLookup.Bucket.Tile.contains(1): not translated, its class isn't translated (it implements `Bucket`).
        // StateLookup.Bucket.Tile.cache(0): not translated, its class isn't translated (it implements `Bucket`).
        // StateLookup.Bucket.Tile.keys(0): not translated, its class isn't translated (it implements `Bucket`).
    }

    /// `StateLookup.Key` (eu/ha3/presencefootsteps/world/StateLookup.java)
    pub mod state_lookup_key {
        use super::*;
        // The class itself isn't translated: a Map or Set keyed by a StateLookup.Key.Attribute. Its static parts are below.
        // NULL: not translated, it uses Key, which isn't translated (a Map or Set keyed by a StateLookup.Key.Attribute).
        // StateLookup.Key.Key(7): not translated, its class isn't translated (a Map or Set keyed by a StateLookup.Key.Attribute).
        // StateLookup.Key.of(2): not translated, it uses Key, which isn't translated (a Map or Set keyed by a StateLookup.Key.Attribute).
        // StateLookup.Key.matches(1): not translated, its class isn't translated (a Map or Set keyed by a StateLookup.Key.Attribute).
        // StateLookup.Key.toString(0): not translated, its class isn't translated (a Map or Set keyed by a StateLookup.Key.Attribute).
        // StateLookup.Key.hashCode(0): not translated, its class isn't translated (a Map or Set keyed by a StateLookup.Key.Attribute).
        // StateLookup.Key.equals(1): not translated, its class isn't translated (a Map or Set keyed by a StateLookup.Key.Attribute).
        // StateLookup.Key.equals(1): not translated, its class isn't translated (a Map or Set keyed by a StateLookup.Key.Attribute).
        // StateLookup.Key.identifier(0): not translated, its class isn't translated (a Map or Set keyed by a StateLookup.Key.Attribute).
        // StateLookup.Key.substrate(0): not translated, its class isn't translated (a Map or Set keyed by a StateLookup.Key.Attribute).
        // StateLookup.Key.properties(0): not translated, its class isn't translated (a Map or Set keyed by a StateLookup.Key.Attribute).
        // StateLookup.Key.value(0): not translated, its class isn't translated (a Map or Set keyed by a StateLookup.Key.Attribute).
        // StateLookup.Key.empty(0): not translated, its class isn't translated (a Map or Set keyed by a StateLookup.Key.Attribute).
        // StateLookup.Key.isTag(0): not translated, its class isn't translated (a Map or Set keyed by a StateLookup.Key.Attribute).
        // StateLookup.Key.isWildcard(0): not translated, its class isn't translated (a Map or Set keyed by a StateLookup.Key.Attribute).
    }

    /// `StateLookup.Key.Attribute` (eu/ha3/presencefootsteps/world/StateLookup.java)
    pub mod state_lookup_key_attribute {
        use super::*;
        #[derive(Clone, Debug, PartialEq)]
        pub struct Attribute {
            pub name: Option<String>,
            pub value: Option<String>,
        }
        impl Attribute {
            pub fn new_2(name: &str, value: &str) -> Self {
                let mut s = Self { name: Some(name.to_string()), value: Some(value.to_string()) };
                s.name = Some(name.to_string());
                s.value = Some(value.to_string());
                s
            }
            // StateLookup.Key.Attribute.Attribute(1): not translated, its constructors are overloaded with the same number of parameters.
            // StateLookup.Key.Attribute.Attribute(1): not translated, its constructors are overloaded with the same number of parameters.
            pub fn to_string(&self) -> String {
                return format!("{}={}", self.name.as_ref().map_or_else(|| "null".to_string(), |v| v.to_string()), self.value.as_ref().map_or_else(|| "null".to_string(), |v| v.to_string()));
            }
            pub fn name(&self) -> Option<String> {
                return self.name.clone();
            }
            pub fn value(&self) -> Option<String> {
                return self.value.clone();
            }
        }
    }

    /// `StateLookup.KeyList` (eu/ha3/presencefootsteps/world/StateLookup.java)
    pub mod state_lookup_key_list {
        use super::*;
        // The class itself isn't translated: it uses Key, which isn't translated (a Map or Set keyed by a StateLookup.Key.Attribute). Its static parts are below.
        // StateLookup.KeyList.KeyList(2): not translated, its class isn't translated (it uses Key, which isn't translated (a Map or Set keyed by a StateLookup.Key.Attribute)).
        // StateLookup.KeyList.KeyList(0): not translated, its class isn't translated (it uses Key, which isn't translated (a Map or Set keyed by a StateLookup.Key.Attribute)).
        // StateLookup.KeyList.add(1): not translated, its class isn't translated (it uses Key, which isn't translated (a Map or Set keyed by a StateLookup.Key.Attribute)).
        // StateLookup.KeyList.getSetFor(1): not translated, its class isn't translated (it uses Key, which isn't translated (a Map or Set keyed by a StateLookup.Key.Attribute)).
        // StateLookup.KeyList.findMatch(1): not translated, its class isn't translated (it uses Key, which isn't translated (a Map or Set keyed by a StateLookup.Key.Attribute)).
        // StateLookup.KeyList.priorityKeys(0): not translated, its class isn't translated (it uses Key, which isn't translated (a Map or Set keyed by a StateLookup.Key.Attribute)).
        // StateLookup.KeyList.keys(0): not translated, its class isn't translated (it uses Key, which isn't translated (a Map or Set keyed by a StateLookup.Key.Attribute)).
    }

    /// `Substrates` (eu/ha3/presencefootsteps/world/Substrates.java)
    pub mod substrates {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        pub fn is_default(substrate: &str) -> bool {
            return j_eq("", &substrate);
        }
        // Substrates.isSupplimentary(1): not translated, an instance field used from a static method.
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

    
    /// Fabric Loader's `Version` and `SemanticVersion` (net.fabricmc.loader.api), parsed and compared as the loader
    /// does: numbers separated by dots, then an optional `-pre.release` and `+build` (which comparing ignores).
    #[derive(Clone, Debug, PartialEq)]
    pub struct JVersion {
        friendly: String,
        /// The numbers of a semantic version; None for one that isn't (compared as text).
        parts: Option<Vec<i64>>,
        pre: Option<String>,
    }
    
    impl JVersion {
        /// `SemanticVersion.parse`: an error (VersionParsingException) when it isn't one.
        pub fn parse_semantic(s: &str) -> std::io::Result<JVersion> {
            Self::semantic(s).ok_or_else(|| std::io::Error::other(format!("Could not parse version number '{s}'")))
        }
    
        /// `Version.parse`: a semantic version when it is one, else the text itself.
        pub fn parse(s: &str) -> std::io::Result<JVersion> {
            if s.is_empty() {
                return Err(std::io::Error::other("Empty version"));
            }
            Ok(Self::semantic(s).unwrap_or(JVersion { friendly: s.to_string(), parts: None, pre: None }))
        }
    
        fn semantic(s: &str) -> Option<JVersion> {
            let core_pre = s.split_once('+').map_or(s, |(c, _)| c);
            let (core, pre) = match core_pre.split_once('-') {
                Some((c, p)) => (c, Some(p.to_string())),
                None => (core_pre, None),
            };
            if let Some(p) = &pre {
                if !p.split('.').all(|x| !x.is_empty() && x.chars().all(|c| c.is_ascii_alphanumeric() || c == '-')) {
                    return None;
                }
            }
            let mut parts = Vec::new();
            for c in core.split('.') {
                if c.is_empty() || !c.chars().all(|ch| ch.is_ascii_digit()) {
                    return None;
                }
                parts.push(c.parse::<i64>().ok()?);
            }
            Some(JVersion { friendly: s.to_string(), parts: Some(parts), pre })
        }
    
        /// `getFriendlyString`: the text it was parsed from.
        pub fn friendly_string(&self) -> String {
            self.friendly.clone()
        }
    
        /// `compareTo`: below, at or above zero.
        pub fn compare_to(&self, o: &JVersion) -> i32 {
            let (Some(a), Some(b)) = (&self.parts, &o.parts) else {
                return match self.friendly.cmp(&o.friendly) {
                    std::cmp::Ordering::Less => -1,
                    std::cmp::Ordering::Equal => 0,
                    std::cmp::Ordering::Greater => 1,
                };
            };
            for i in 0..a.len().max(b.len()) {
                let (x, y) = (a.get(i).copied().unwrap_or(0), b.get(i).copied().unwrap_or(0));
                if x != y {
                    return if x < y { -1 } else { 1 };
                }
            }
            match (&self.pre, &o.pre) {
                (None, None) => 0,
                (None, Some(_)) => 1,
                (Some(_), None) => -1,
                (Some(p), Some(q)) => {
                    let (ps, qs): (Vec<&str>, Vec<&str>) = (p.split('.').collect(), q.split('.').collect());
                    for i in 0..ps.len().min(qs.len()) {
                        let c = match (ps[i].parse::<i64>(), qs[i].parse::<i64>()) {
                            (Ok(x), Ok(y)) => x.cmp(&y),
                            (Ok(_), Err(_)) => std::cmp::Ordering::Less,
                            (Err(_), Ok(_)) => std::cmp::Ordering::Greater,
                            _ => ps[i].cmp(qs[i]),
                        };
                        if c != std::cmp::Ordering::Equal {
                            return if c == std::cmp::Ordering::Less { -1 } else { 1 };
                        }
                    }
                    (ps.len() as i32 - qs.len() as i32).signum()
                }
            }
        }
    
        /// The numbers, a missing one as 0.
        fn part(&self, i: usize) -> i64 {
            self.parts.as_ref().and_then(|p| p.get(i).copied()).unwrap_or(0)
        }
    }
    
    /// One dependency of a mod's fabric.mod.json: its kind (`depends`, `recommends`, `suggests`, `conflicts`,
    /// `breaks`), the mod it names and its version ranges (any one matches).
    #[derive(Clone, Debug, PartialEq)]
    pub struct JModDep {
        pub kind: &'static str,
        pub mod_id: String,
        pub ranges: Vec<String>,
    }
    
    impl JModDep {
        /// `ModDependency.matches`: a range's space-separated predicates all hold (`>=1.2`, `~26.2`, `^1.0`, `1.20.x`, `*`).
        pub fn matches(&self, v: &JVersion) -> bool {
            self.ranges.iter().any(|r| r.split_whitespace().all(|p| Self::predicate(p, v)))
        }
    
        fn predicate(p: &str, v: &JVersion) -> bool {
            if p == "*" {
                return true;
            }
            for op in [">=", "<=", ">", "<", "=", "~", "^"] {
                if let Some(rest) = p.strip_prefix(op) {
                    let Ok(w) = JVersion::parse(rest) else {
                        return false;
                    };
                    let c = v.compare_to(&w);
                    return match op {
                        ">=" => c >= 0,
                        "<=" => c <= 0,
                        ">" => c > 0,
                        "<" => c < 0,
                        "=" => c == 0,
                        // ~1.2.3: at least it, below the next minor version.
                        "~" => c >= 0 && v.part(0) == w.part(0) && v.part(1) == w.part(1),
                        // ^1.2.3: at least it, below the next major version.
                        _ => c >= 0 && v.part(0) == w.part(0),
                    };
                }
            }
            // 1.20.x: the numbers before the x match.
            if p.ends_with(".x") || p.ends_with(".X") || p.ends_with(".*") {
                let head: Vec<&str> = p[..p.len() - 2].split('.').collect();
                return head.iter().enumerate().all(|(i, h)| h.parse::<i64>().is_ok_and(|n| v.part(i) == n));
            }
            JVersion::parse(p).is_ok_and(|w| v.compare_to(&w) == 0)
        }
    }
    
    /// A mod as `FabricLoader.getModContainer` gives it, from the mod's fabric.mod.json at conversion time
    /// (and `minecraft` as the version Horizon plays).
    #[derive(Clone, Debug, PartialEq)]
    pub struct JMod {
        pub id: String,
        pub version: JVersion,
        pub deps: Vec<JModDep>,
    }
    
    impl JMod {
        pub fn new(id: &str, version: &str, deps: &[(&'static str, &str, &[&str])]) -> JMod {
            JMod {
                id: id.to_string(),
                version: JVersion::parse(version).unwrap_or(JVersion { friendly: version.to_string(), parts: None, pre: None }),
                deps: deps
                    .iter()
                    .map(|(k, m, r)| JModDep { kind: k, mod_id: m.to_string(), ranges: r.iter().map(|s| s.to_string()).collect() })
                    .collect(),
            }
        }
    }
    
    #[cfg(test)]
    mod jfabric_tests {
        use super::*;
    
        #[test]
        fn versions_compare_like_fabric() {
            let v = |s| JVersion::parse_semantic(s).unwrap();
            assert!(v("1.0.1").compare_to(&v("1.0.0")) > 0);
            assert!(v("1.0").compare_to(&v("1.0.0")) == 0);
            assert!(v("1.0.0-beta.2").compare_to(&v("1.0.0")) < 0);
            assert!(v("1.0.0-beta.10").compare_to(&v("1.0.0-beta.2")) > 0);
            assert!(v("1.0.0+26.2").compare_to(&v("1.0.0+26.3")) == 0);
            assert!(JVersion::parse_semantic("v1.0").is_err());
            assert_eq!(v("1.2.3+build").friendly_string(), "1.2.3+build");
        }
    
        #[test]
        fn dependency_ranges() {
            let m = JMod::new("x", "1.0.0", &[("depends", "minecraft", &["~26.2"]), ("depends", "fabricloader", &[">=0.19"])]);
            let mc = |s| JVersion::parse(s).unwrap();
            assert!(m.deps[0].matches(&mc("26.2")));
            assert!(m.deps[0].matches(&mc("26.2.1")));
            assert!(!m.deps[0].matches(&mc("26.3")));
            assert!(m.deps[1].matches(&mc("0.19.2")));
            let d = JModDep { kind: "depends", mod_id: "minecraft".into(), ranges: vec!["1.20.x".into(), ">=26.3 <27".into()] };
            assert!(d.matches(&mc("1.20.4")) && d.matches(&mc("26.3")) && !d.matches(&mc("27.0")));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_are_set() {
        assert_eq!(ID, "presencefootsteps");
        assert!(!NAME.is_empty());
        assert!(KEYS.iter().all(|[n, _, d]| !n.is_empty() && d.starts_with("key.")));
    }
}
