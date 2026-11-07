// Chunky, ported to Horizon by horizon-port from Chunky-Fabric-1.5.66.jar.
// This file is both the bundle's Cargo library (its tests) and the module the bundle adds to
// the game (crate::game::port_chunky). Put the mod's own logic here, in plain Rust
// that doesn't need the game, and call it from the helpers. No `//!` comments: module payloads
// can't hold inner attributes.

/// The mod's id (its settings folder), name, version and description.
pub const ID: &str = "chunky";
pub const NAME: &str = "Chunky";
pub const VERSION: &str = "1.5.66";
pub const DESCRIPTION: &str = "Pre-generates chunks, quickly, efficiently, and safely";

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

    /// `ChunkyAPI` (org/popcraft/chunky/api/ChunkyAPI.java)
    pub mod chunky_api {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ChunkyAPI.version(0): not translated, it has no body.
        // ChunkyAPI.isRunning(1): not translated, it has no body.
        // ChunkyAPI.startTask(7): not translated, it has no body.
        // ChunkyAPI.pauseTask(1): not translated, it has no body.
        // ChunkyAPI.continueTask(1): not translated, it has no body.
        // ChunkyAPI.cancelTask(1): not translated, it has no body.
        // ChunkyAPI.onGenerationStart(1): not translated, it has no body.
        // ChunkyAPI.onGenerationProgress(1): not translated, it has no body.
        // ChunkyAPI.onGenerationComplete(1): not translated, it has no body.
    }

    /// `ChunkyAPIImpl` (org/popcraft/chunky/api/ChunkyAPIImpl.java)
    pub mod chunky_api_impl {
        use super::*;
        // The class itself isn't translated: it implements `ChunkyAPI`. Its static parts are below.
        // ChunkyAPIImpl.ChunkyAPIImpl(1): not translated, its class isn't translated (it implements `ChunkyAPI`).
        // ChunkyAPIImpl.version(0): not translated, its class isn't translated (it implements `ChunkyAPI`).
        // ChunkyAPIImpl.isRunning(1): not translated, its class isn't translated (it implements `ChunkyAPI`).
        // ChunkyAPIImpl.startTask(7): not translated, its class isn't translated (it implements `ChunkyAPI`).
        // ChunkyAPIImpl.pauseTask(1): not translated, its class isn't translated (it implements `ChunkyAPI`).
        // ChunkyAPIImpl.continueTask(1): not translated, its class isn't translated (it implements `ChunkyAPI`).
        // ChunkyAPIImpl.cancelTask(1): not translated, its class isn't translated (it implements `ChunkyAPI`).
        // ChunkyAPIImpl.onGenerationStart(1): not translated, its class isn't translated (it implements `ChunkyAPI`).
        // ChunkyAPIImpl.onGenerationProgress(1): not translated, its class isn't translated (it implements `ChunkyAPI`).
        // ChunkyAPIImpl.onGenerationComplete(1): not translated, its class isn't translated (it implements `ChunkyAPI`).
    }

    /// `GenerationCompleteEvent` (org/popcraft/chunky/api/event/task/GenerationCompleteEvent.java)
    pub mod generation_complete_event {
        use super::*;
        // The class itself isn't translated: it implements `Event` (org.popcraft.chunky.event). Its static parts are below.
        // GenerationCompleteEvent.GenerationCompleteEvent(11): not translated, its class isn't translated (it implements `Event` (org.popcraft.chunky.event)).
        // GenerationCompleteEvent.world(0): not translated, its class isn't translated (it implements `Event` (org.popcraft.chunky.event)).
        // GenerationCompleteEvent.shape(0): not translated, its class isn't translated (it implements `Event` (org.popcraft.chunky.event)).
        // GenerationCompleteEvent.centerX(0): not translated, its class isn't translated (it implements `Event` (org.popcraft.chunky.event)).
        // GenerationCompleteEvent.centerZ(0): not translated, its class isn't translated (it implements `Event` (org.popcraft.chunky.event)).
        // GenerationCompleteEvent.radiusX(0): not translated, its class isn't translated (it implements `Event` (org.popcraft.chunky.event)).
        // GenerationCompleteEvent.radiusZ(0): not translated, its class isn't translated (it implements `Event` (org.popcraft.chunky.event)).
        // GenerationCompleteEvent.pattern(0): not translated, its class isn't translated (it implements `Event` (org.popcraft.chunky.event)).
        // GenerationCompleteEvent.owner(0): not translated, its class isn't translated (it implements `Event` (org.popcraft.chunky.event)).
        // GenerationCompleteEvent.chunks(0): not translated, its class isn't translated (it implements `Event` (org.popcraft.chunky.event)).
        // GenerationCompleteEvent.time(0): not translated, its class isn't translated (it implements `Event` (org.popcraft.chunky.event)).
        // GenerationCompleteEvent.reason(0): not translated, its class isn't translated (it implements `Event` (org.popcraft.chunky.event)).
    }

    /// `GenerationCompleteEvent.Reason` (org/popcraft/chunky/api/event/task/GenerationCompleteEvent.java)
    pub mod generation_complete_event_reason {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum Reason {
            Completed,
            Paused,
            Cancelled,
        }
        impl Reason {
            pub const VALUES: [Reason; 3] = [Reason::Completed, Reason::Paused, Reason::Cancelled, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    Reason::Completed => "COMPLETED",
                    Reason::Paused => "PAUSED",
                    Reason::Cancelled => "CANCELLED",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "COMPLETED" => Reason::Completed,
                    "PAUSED" => Reason::Paused,
                    "CANCELLED" => Reason::Cancelled,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `GenerationProgressEvent` (org/popcraft/chunky/api/event/task/GenerationProgressEvent.java)
    pub mod generation_progress_event {
        use super::*;
        // The class itself isn't translated: it implements `Event` (org.popcraft.chunky.event). Its static parts are below.
        // GenerationProgressEvent.GenerationProgressEvent(10): not translated, its class isn't translated (it implements `Event` (org.popcraft.chunky.event)).
        // GenerationProgressEvent.world(0): not translated, its class isn't translated (it implements `Event` (org.popcraft.chunky.event)).
        // GenerationProgressEvent.chunks(0): not translated, its class isn't translated (it implements `Event` (org.popcraft.chunky.event)).
        // GenerationProgressEvent.complete(0): not translated, its class isn't translated (it implements `Event` (org.popcraft.chunky.event)).
        // GenerationProgressEvent.progress(0): not translated, its class isn't translated (it implements `Event` (org.popcraft.chunky.event)).
        // GenerationProgressEvent.hours(0): not translated, its class isn't translated (it implements `Event` (org.popcraft.chunky.event)).
        // GenerationProgressEvent.minutes(0): not translated, its class isn't translated (it implements `Event` (org.popcraft.chunky.event)).
        // GenerationProgressEvent.seconds(0): not translated, its class isn't translated (it implements `Event` (org.popcraft.chunky.event)).
        // GenerationProgressEvent.rate(0): not translated, its class isn't translated (it implements `Event` (org.popcraft.chunky.event)).
        // GenerationProgressEvent.x(0): not translated, its class isn't translated (it implements `Event` (org.popcraft.chunky.event)).
        // GenerationProgressEvent.z(0): not translated, its class isn't translated (it implements `Event` (org.popcraft.chunky.event)).
    }

    /// `GenerationStartEvent` (org/popcraft/chunky/api/event/task/GenerationStartEvent.java)
    pub mod generation_start_event {
        use super::*;
        // The class itself isn't translated: it implements `Event` (org.popcraft.chunky.event). Its static parts are below.
        // GenerationStartEvent.GenerationStartEvent(9): not translated, its class isn't translated (it implements `Event` (org.popcraft.chunky.event)).
        // GenerationStartEvent.world(0): not translated, its class isn't translated (it implements `Event` (org.popcraft.chunky.event)).
        // GenerationStartEvent.shape(0): not translated, its class isn't translated (it implements `Event` (org.popcraft.chunky.event)).
        // GenerationStartEvent.centerX(0): not translated, its class isn't translated (it implements `Event` (org.popcraft.chunky.event)).
        // GenerationStartEvent.centerZ(0): not translated, its class isn't translated (it implements `Event` (org.popcraft.chunky.event)).
        // GenerationStartEvent.radiusX(0): not translated, its class isn't translated (it implements `Event` (org.popcraft.chunky.event)).
        // GenerationStartEvent.radiusZ(0): not translated, its class isn't translated (it implements `Event` (org.popcraft.chunky.event)).
        // GenerationStartEvent.pattern(0): not translated, its class isn't translated (it implements `Event` (org.popcraft.chunky.event)).
        // GenerationStartEvent.owner(0): not translated, its class isn't translated (it implements `Event` (org.popcraft.chunky.event)).
        // GenerationStartEvent.continued(0): not translated, its class isn't translated (it implements `Event` (org.popcraft.chunky.event)).
    }

    /// `Chunky` (org/popcraft/chunky/Chunky.java)
    pub mod chunky {
        use super::*;
        // The class itself isn't translated: it uses Server, which isn't translated (it's an interface). Its static parts are below.
        // Chunky.Chunky(2): not translated, its class isn't translated (it uses Server, which isn't translated (it's an interface)).
        // Chunky.disable(0): not translated, its class isn't translated (it uses Server, which isn't translated (it's an interface)).
        // Chunky.loadLimit(0): not translated, its class isn't translated (it uses Server, which isn't translated (it's an interface)).
        // Chunky.loadVersion(0): not translated, its class isn't translated (it uses Server, which isn't translated (it's an interface)).
        // Chunky.loadCommands(0): not translated, its class isn't translated (it uses Server, which isn't translated (it's an interface)).
        // Chunky.getScheduler(0): not translated, its class isn't translated (it uses Server, which isn't translated (it's an interface)).
        // Chunky.getServer(0): not translated, its class isn't translated (it uses Server, which isn't translated (it's an interface)).
        // Chunky.getConfig(0): not translated, its class isn't translated (it uses Server, which isn't translated (it's an interface)).
        // Chunky.getTaskLoader(0): not translated, its class isn't translated (it uses Server, which isn't translated (it's an interface)).
        // Chunky.getEventBus(0): not translated, its class isn't translated (it uses Server, which isn't translated (it's an interface)).
        // Chunky.getGenerationTasks(0): not translated, its class isn't translated (it uses Server, which isn't translated (it's an interface)).
        // Chunky.getTrimTasks(0): not translated, its class isn't translated (it uses Server, which isn't translated (it's an interface)).
        // Chunky.getCommands(0): not translated, its class isn't translated (it uses Server, which isn't translated (it's an interface)).
        // Chunky.getSelection(0): not translated, its class isn't translated (it uses Server, which isn't translated (it's an interface)).
        // Chunky.getPendingAction(1): not translated, its class isn't translated (it uses Server, which isn't translated (it's an interface)).
        // Chunky.setPendingAction(2): not translated, its class isn't translated (it uses Server, which isn't translated (it's an interface)).
        // Chunky.setLanguage(1): not translated, its class isn't translated (it uses Server, which isn't translated (it's an interface)).
        // Chunky.getRegionCache(0): not translated, its class isn't translated (it uses Server, which isn't translated (it's an interface)).
        // Chunky.getLimit(0): not translated, its class isn't translated (it uses Server, which isn't translated (it's an interface)).
        // Chunky.getVersion(0): not translated, its class isn't translated (it uses Server, which isn't translated (it's an interface)).
        // Chunky.getApi(0): not translated, its class isn't translated (it uses Server, which isn't translated (it's an interface)).
    }

    /// `ChunkyFabric` (org/popcraft/chunky/ChunkyFabric.java)
    pub mod chunky_fabric {
        use super::*;
        // The class itself isn't translated: it implements Fabric's `ModInitializer`. Its static parts are below.
        pub static ENABLE_MOONRISE_WORKAROUNDS: LazyLock<bool> = LazyLock::new(|| false);
        // ChunkyFabric.onInitialize(0): not translated, its class isn't translated (it implements Fabric's `ModInitializer`).
        // ChunkyFabric.getChunky(0): not translated, its class isn't translated (it implements Fabric's `ModInitializer`).
    }

    /// `ChunkyProvider` (org/popcraft/chunky/ChunkyProvider.java)
    pub mod chunky_provider {
        use super::*;
        // instance: not translated, it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface)).
        #[derive(Clone, Debug)]
        pub struct ChunkyProvider {
        }
        impl ChunkyProvider {
            pub fn new() -> Self {
                let mut s = Self {};
                panic!("{}", "This class cannot be instantiated.");
                s
            }
        }
        // ChunkyProvider.get(0): not translated, it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface)).
        // ChunkyProvider.register(1): not translated, it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface)).
        // ChunkyProvider.unregister(0): not translated, it uses ChunkyProvider.instance, which isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))).
    }

    /// `BrigadierCommand` (org/popcraft/chunky/command/brigadier/BrigadierCommand.java)
    pub mod brigadier_command {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // BrigadierCommand.getChunky(0): not translated, it has no body.
        // BrigadierCommand.getSender(1): not translated, it has no body.
        // BrigadierCommand.worldArgumentType(0): not translated, it has no body.
        // BrigadierCommand.greedyWorldArgumentType(0): not translated, its class isn't translated (it's generic).
        // BrigadierCommand.hasPermission(1): not translated, it has no body.
        // BrigadierCommand.hasPermission(2): not translated, its class isn't translated (it's generic).
        // BrigadierCommand.build(0): not translated, its class isn't translated (it's generic).
        // BrigadierCommand.execute(1): not translated, its class isn't translated (it's generic).
        // BrigadierCommand.isBorderAvailable(0): not translated, its class isn't translated (it's generic).
        // BrigadierCommand.literal(1): not translated, its class isn't translated (it's generic).
        // BrigadierCommand.argument(2): not translated, it's generic.
        // BrigadierCommand.suggestedArgument(2): not translated, it's generic.
        // BrigadierCommand.world(0): not translated, its class isn't translated (it's generic).
        // BrigadierCommand.greedyWorld(0): not translated, its class isn't translated (it's generic).
        // BrigadierCommand.registerCommand(3): not translated, its class isn't translated (it's generic).
        // BrigadierCommand.registerArguments(3): not translated, its class isn't translated (it's generic).
    }

    /// `FabricCommand` (org/popcraft/chunky/command/brigadier/FabricCommand.java)
    pub mod fabric_command {
        use super::*;
        // The class itself isn't translated: it extends `BrigadierCommand`. Its static parts are below.
        // FabricCommand.FabricCommand(1): not translated, its class isn't translated (it extends `BrigadierCommand`).
        // FabricCommand.getChunky(0): not translated, its class isn't translated (it extends `BrigadierCommand`).
        // FabricCommand.getSender(1): not translated, its class isn't translated (it extends `BrigadierCommand`).
        // FabricCommand.worldArgumentType(0): not translated, its class isn't translated (it extends `BrigadierCommand`).
        // FabricCommand.hasPermission(1): not translated, its class isn't translated (it extends `BrigadierCommand`).
    }

    /// `CancelCommand` (org/popcraft/chunky/command/CancelCommand.java)
    pub mod cancel_command {
        use super::*;
        // The class itself isn't translated: it implements `ChunkyCommand`. Its static parts are below.
        // CancelCommand.CancelCommand(1): not translated, its class isn't translated (it implements `ChunkyCommand`).
        // CancelCommand.execute(2): not translated, its class isn't translated (it implements `ChunkyCommand`).
        // CancelCommand.suggestions(1): not translated, its class isn't translated (it implements `ChunkyCommand`).
    }

    /// `CenterCommand` (org/popcraft/chunky/command/CenterCommand.java)
    pub mod center_command {
        use super::*;
        // The class itself isn't translated: it implements `ChunkyCommand`. Its static parts are below.
        // CenterCommand.CenterCommand(1): not translated, its class isn't translated (it implements `ChunkyCommand`).
        // CenterCommand.execute(2): not translated, its class isn't translated (it implements `ChunkyCommand`).
        // CenterCommand.suggestions(1): not translated, its class isn't translated (it implements `ChunkyCommand`).
    }

    /// `ChunkyCommand` (org/popcraft/chunky/command/ChunkyCommand.java)
    pub mod chunky_command {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ChunkyCommand.execute(2): not translated, it has no body.
        // ChunkyCommand.suggestions(1): not translated, it has no body.
    }

    /// `CommandArguments` (org/popcraft/chunky/command/CommandArguments.java)
    pub mod command_arguments {
        use super::*;
        // The class itself isn't translated: it uses `Queue` (java.util). Its static parts are below.
        // CommandArguments.CommandArguments(1): not translated, its class isn't translated (it uses `Queue` (java.util)).
        // CommandArguments.of(1): not translated, it uses CommandArguments, which isn't translated (it uses `Queue` (java.util)).
        // CommandArguments.of(1): not translated, it uses CommandArguments, which isn't translated (it uses `Queue` (java.util)).
        // CommandArguments.empty(0): not translated, it uses CommandArguments, which isn't translated (it uses `Queue` (java.util)).
        // CommandArguments.size(0): not translated, its class isn't translated (it uses `Queue` (java.util)).
        // CommandArguments.next(0): not translated, its class isn't translated (it uses `Queue` (java.util)).
        // CommandArguments.remaining(0): not translated, its class isn't translated (it uses `Queue` (java.util)).
        // CommandArguments.joined(0): not translated, its class isn't translated (it uses `Queue` (java.util)).
    }

    /// `CommandLiteral` (org/popcraft/chunky/command/CommandLiteral.java)
    pub mod command_literal {
        use super::*;
        pub const BORDER: &str = "border";
        pub const CANCEL: &str = "cancel";
        pub const CENTER: &str = "center";
        pub const CENTER_X: &str = "centerX";
        pub const CENTER_Z: &str = "centerZ";
        pub const CHUNKY: &str = "chunky";
        pub const CONFIRM: &str = "confirm";
        pub const CONTINUE: &str = "continue";
        pub const CORNERS: &str = "corners";
        pub const DEBUG: &str = "debug";
        pub const FILTERS: &str = "filters";
        pub const HELP: &str = "help";
        pub const INHABITED: &str = "inhabited";
        pub const INTERVAL: &str = "interval";
        pub const LIMIT: &str = "limit";
        pub const MAP: &str = "map";
        pub const PAGE: &str = "page";
        pub const PATTERN: &str = "pattern";
        pub const PAUSE: &str = "pause";
        pub const PROGRESS: &str = "progress";
        pub const QUIET: &str = "quiet";
        pub const RADIUS: &str = "radius";
        pub const RADIUS_X: &str = "radiusX";
        pub const RADIUS_Z: &str = "radiusZ";
        pub const RATE: &str = "rate";
        pub const RELOAD: &str = "reload";
        pub const SELECTION: &str = "selection";
        pub const SHAPE: &str = "shape";
        pub const SILENT: &str = "silent";
        pub const SPAWN: &str = "spawn";
        pub const START: &str = "start";
        pub const TRIM: &str = "trim";
        pub const TRIM_MODE: &str = "trimMode";
        pub const UNMODIFIED: &str = "unmodified";
        pub const WORLDBORDER: &str = "worldborder";
        pub const WORLD: &str = "world";
        pub const X: &str = "x";
        pub const X1: &str = "x1";
        pub const X2: &str = "x2";
        pub const Z: &str = "z";
        pub const Z1: &str = "z1";
        pub const Z2: &str = "z2";
        pub const ADD: &str = "add";
        pub const BYPASS: &str = "bypass";
        pub const LIST: &str = "list";
        pub const LOAD: &str = "load";
        pub const REMOVE: &str = "remove";
        pub const WRAP: &str = "wrap";
        pub const PLAYER: &str = "player";
        pub const VALUE: &str = "value";
        pub const TYPE: &str = "type";
        #[derive(Clone, Debug)]
        pub struct CommandLiteral {
        }
        impl CommandLiteral {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `ConfirmCommand` (org/popcraft/chunky/command/ConfirmCommand.java)
    pub mod confirm_command {
        use super::*;
        // The class itself isn't translated: it implements `ChunkyCommand`. Its static parts are below.
        // ConfirmCommand.ConfirmCommand(1): not translated, its class isn't translated (it implements `ChunkyCommand`).
        // ConfirmCommand.execute(2): not translated, its class isn't translated (it implements `ChunkyCommand`).
        // ConfirmCommand.suggestions(1): not translated, its class isn't translated (it implements `ChunkyCommand`).
    }

    /// `ContinueCommand` (org/popcraft/chunky/command/ContinueCommand.java)
    pub mod continue_command {
        use super::*;
        // The class itself isn't translated: it implements `ChunkyCommand`. Its static parts are below.
        // ContinueCommand.ContinueCommand(1): not translated, its class isn't translated (it implements `ChunkyCommand`).
        // ContinueCommand.execute(2): not translated, its class isn't translated (it implements `ChunkyCommand`).
        // ContinueCommand.suggestions(1): not translated, its class isn't translated (it implements `ChunkyCommand`).
    }

    /// `CornersCommand` (org/popcraft/chunky/command/CornersCommand.java)
    pub mod corners_command {
        use super::*;
        // The class itself isn't translated: it implements `ChunkyCommand`. Its static parts are below.
        // CornersCommand.CornersCommand(1): not translated, its class isn't translated (it implements `ChunkyCommand`).
        // CornersCommand.execute(2): not translated, its class isn't translated (it implements `ChunkyCommand`).
        // CornersCommand.suggestions(1): not translated, its class isn't translated (it implements `ChunkyCommand`).
    }

    /// `DebugCommand` (org/popcraft/chunky/command/DebugCommand.java)
    pub mod debug_command {
        use super::*;
        // The class itself isn't translated: it implements `ChunkyCommand`. Its static parts are below.
        // DebugCommand.DebugCommand(1): not translated, its class isn't translated (it implements `ChunkyCommand`).
        // DebugCommand.execute(2): not translated, its class isn't translated (it implements `ChunkyCommand`).
        // DebugCommand.suggestions(1): not translated, its class isn't translated (it implements `ChunkyCommand`).
    }

    /// `HelpCommand` (org/popcraft/chunky/command/HelpCommand.java)
    pub mod help_command {
        use super::*;
        // The class itself isn't translated: it implements `ChunkyCommand`. Its static parts are below.
        // HelpCommand.HelpCommand(1): not translated, its class isn't translated (it implements `ChunkyCommand`).
        // HelpCommand.execute(2): not translated, its class isn't translated (it implements `ChunkyCommand`).
        // HelpCommand.suggestions(1): not translated, its class isn't translated (it implements `ChunkyCommand`).
    }

    /// `LimitCommand` (org/popcraft/chunky/command/LimitCommand.java)
    pub mod limit_command {
        use super::*;
        // The class itself isn't translated: it implements `ChunkyCommand`. Its static parts are below.
        // LimitCommand.LimitCommand(1): not translated, its class isn't translated (it implements `ChunkyCommand`).
        // LimitCommand.execute(2): not translated, its class isn't translated (it implements `ChunkyCommand`).
        // LimitCommand.suggestions(1): not translated, its class isn't translated (it implements `ChunkyCommand`).
    }

    /// `PatternCommand` (org/popcraft/chunky/command/PatternCommand.java)
    pub mod pattern_command {
        use super::*;
        // The class itself isn't translated: it implements `ChunkyCommand`. Its static parts are below.
        // PatternCommand.PatternCommand(1): not translated, its class isn't translated (it implements `ChunkyCommand`).
        // PatternCommand.execute(2): not translated, its class isn't translated (it implements `ChunkyCommand`).
        // PatternCommand.suggestions(1): not translated, its class isn't translated (it implements `ChunkyCommand`).
    }

    /// `PauseCommand` (org/popcraft/chunky/command/PauseCommand.java)
    pub mod pause_command {
        use super::*;
        // The class itself isn't translated: it implements `ChunkyCommand`. Its static parts are below.
        // PauseCommand.PauseCommand(1): not translated, its class isn't translated (it implements `ChunkyCommand`).
        // PauseCommand.execute(2): not translated, its class isn't translated (it implements `ChunkyCommand`).
        // PauseCommand.suggestions(1): not translated, its class isn't translated (it implements `ChunkyCommand`).
    }

    /// `ProgressCommand` (org/popcraft/chunky/command/ProgressCommand.java)
    pub mod progress_command {
        use super::*;
        // The class itself isn't translated: it implements `ChunkyCommand`. Its static parts are below.
        // ProgressCommand.ProgressCommand(1): not translated, its class isn't translated (it implements `ChunkyCommand`).
        // ProgressCommand.execute(2): not translated, its class isn't translated (it implements `ChunkyCommand`).
        // ProgressCommand.suggestions(1): not translated, its class isn't translated (it implements `ChunkyCommand`).
    }

    /// `QuietCommand` (org/popcraft/chunky/command/QuietCommand.java)
    pub mod quiet_command {
        use super::*;
        // The class itself isn't translated: it implements `ChunkyCommand`. Its static parts are below.
        // QuietCommand.QuietCommand(1): not translated, its class isn't translated (it implements `ChunkyCommand`).
        // QuietCommand.execute(2): not translated, its class isn't translated (it implements `ChunkyCommand`).
        // QuietCommand.suggestions(1): not translated, its class isn't translated (it implements `ChunkyCommand`).
    }

    /// `RadiusCommand` (org/popcraft/chunky/command/RadiusCommand.java)
    pub mod radius_command {
        use super::*;
        // The class itself isn't translated: it implements `ChunkyCommand`. Its static parts are below.
        // RadiusCommand.RadiusCommand(1): not translated, its class isn't translated (it implements `ChunkyCommand`).
        // RadiusCommand.execute(2): not translated, its class isn't translated (it implements `ChunkyCommand`).
        // RadiusCommand.suggestions(1): not translated, its class isn't translated (it implements `ChunkyCommand`).
    }

    /// `ReloadCommand` (org/popcraft/chunky/command/ReloadCommand.java)
    pub mod reload_command {
        use super::*;
        // The class itself isn't translated: it implements `ChunkyCommand`. Its static parts are below.
        // ReloadCommand.ReloadCommand(1): not translated, its class isn't translated (it implements `ChunkyCommand`).
        // ReloadCommand.execute(2): not translated, its class isn't translated (it implements `ChunkyCommand`).
        // ReloadCommand.suggestions(1): not translated, its class isn't translated (it implements `ChunkyCommand`).
    }

    /// `SelectionCommand` (org/popcraft/chunky/command/SelectionCommand.java)
    pub mod selection_command {
        use super::*;
        // The class itself isn't translated: it implements `ChunkyCommand`. Its static parts are below.
        // SelectionCommand.SelectionCommand(1): not translated, its class isn't translated (it implements `ChunkyCommand`).
        // SelectionCommand.execute(2): not translated, its class isn't translated (it implements `ChunkyCommand`).
        // SelectionCommand.suggestions(1): not translated, its class isn't translated (it implements `ChunkyCommand`).
    }

    /// `ShapeCommand` (org/popcraft/chunky/command/ShapeCommand.java)
    pub mod shape_command {
        use super::*;
        // The class itself isn't translated: it implements `ChunkyCommand`. Its static parts are below.
        // ShapeCommand.ShapeCommand(1): not translated, its class isn't translated (it implements `ChunkyCommand`).
        // ShapeCommand.execute(2): not translated, its class isn't translated (it implements `ChunkyCommand`).
        // ShapeCommand.suggestions(1): not translated, its class isn't translated (it implements `ChunkyCommand`).
    }

    /// `SilentCommand` (org/popcraft/chunky/command/SilentCommand.java)
    pub mod silent_command {
        use super::*;
        // The class itself isn't translated: it implements `ChunkyCommand`. Its static parts are below.
        // SilentCommand.SilentCommand(1): not translated, its class isn't translated (it implements `ChunkyCommand`).
        // SilentCommand.execute(2): not translated, its class isn't translated (it implements `ChunkyCommand`).
        // SilentCommand.suggestions(1): not translated, its class isn't translated (it implements `ChunkyCommand`).
    }

    /// `SpawnCommand` (org/popcraft/chunky/command/SpawnCommand.java)
    pub mod spawn_command {
        use super::*;
        // The class itself isn't translated: it implements `ChunkyCommand`. Its static parts are below.
        // SpawnCommand.SpawnCommand(1): not translated, its class isn't translated (it implements `ChunkyCommand`).
        // SpawnCommand.execute(2): not translated, its class isn't translated (it implements `ChunkyCommand`).
        // SpawnCommand.suggestions(1): not translated, its class isn't translated (it implements `ChunkyCommand`).
    }

    /// `StartCommand` (org/popcraft/chunky/command/StartCommand.java)
    pub mod start_command {
        use super::*;
        // The class itself isn't translated: it implements `ChunkyCommand`. Its static parts are below.
        // StartCommand.StartCommand(1): not translated, its class isn't translated (it implements `ChunkyCommand`).
        // StartCommand.execute(2): not translated, its class isn't translated (it implements `ChunkyCommand`).
        // StartCommand.suggestions(1): not translated, its class isn't translated (it implements `ChunkyCommand`).
    }

    /// `CommandSuggestions` (org/popcraft/chunky/command/suggestion/CommandSuggestions.java)
    pub mod command_suggestions {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct CommandSuggestions {
        }
        impl CommandSuggestions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // CommandSuggestions.suggest(3): not translated, it's generic.
    }

    /// `WorldSuggestions` (org/popcraft/chunky/command/suggestion/WorldSuggestions.java)
    pub mod world_suggestions {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct WorldSuggestions {
        }
        impl WorldSuggestions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // WorldSuggestions.suggest(2): not translated, it uses Minecraft's `Suggestions`.
    }

    /// `TrimCommand` (org/popcraft/chunky/command/TrimCommand.java)
    pub mod trim_command {
        use super::*;
        // The class itself isn't translated: it implements `ChunkyCommand`. Its static parts are below.
        // TrimCommand.TrimCommand(1): not translated, its class isn't translated (it implements `ChunkyCommand`).
        // TrimCommand.execute(2): not translated, its class isn't translated (it implements `ChunkyCommand`).
        // TrimCommand.checkRegion(5): not translated, its class isn't translated (it implements `ChunkyCommand`).
        // TrimCommand.shouldDeleteRegion(4): not translated, its class isn't translated (it implements `ChunkyCommand`).
        // TrimCommand.deleteRegion(2): not translated, its class isn't translated (it implements `ChunkyCommand`).
        // TrimCommand.trimRegion(7): not translated, its class isn't translated (it implements `ChunkyCommand`).
        // TrimCommand.deleteExternalChunk(5): not translated, its class isn't translated (it implements `ChunkyCommand`).
        // TrimCommand.suggestions(1): not translated, its class isn't translated (it implements `ChunkyCommand`).
    }

    /// `TrimCommand.Task` (org/popcraft/chunky/command/TrimCommand.java)
    pub mod trim_command_task {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Task {
            pub cancelled: bool,
        }
        impl Task {
            pub fn new() -> Self {
                let mut s = Self { cancelled: false };
                s
            }
            pub fn is_cancelled(&self) -> bool {
                return self.cancelled;
            }
            pub fn set_cancelled(&mut self, cancelled: bool) {
                self.cancelled = cancelled;
            }
        }
    }

    /// `WorldBorderCommand` (org/popcraft/chunky/command/WorldBorderCommand.java)
    pub mod world_border_command {
        use super::*;
        // The class itself isn't translated: it implements `ChunkyCommand`. Its static parts are below.
        // WorldBorderCommand.WorldBorderCommand(1): not translated, its class isn't translated (it implements `ChunkyCommand`).
        // WorldBorderCommand.execute(2): not translated, its class isn't translated (it implements `ChunkyCommand`).
        // WorldBorderCommand.suggestions(1): not translated, its class isn't translated (it implements `ChunkyCommand`).
        // WorldBorderCommand.setBorderViaIntegration(1): not translated, its class isn't translated (it implements `ChunkyCommand`).
    }

    /// `WorldCommand` (org/popcraft/chunky/command/WorldCommand.java)
    pub mod world_command {
        use super::*;
        // The class itself isn't translated: it implements `ChunkyCommand`. Its static parts are below.
        // WorldCommand.WorldCommand(1): not translated, its class isn't translated (it implements `ChunkyCommand`).
        // WorldCommand.execute(2): not translated, its class isn't translated (it implements `ChunkyCommand`).
        // WorldCommand.suggestions(1): not translated, its class isn't translated (it implements `ChunkyCommand`).
    }

    /// `MinecraftServerExtension` (org/popcraft/chunky/ducks/MinecraftServerExtension.java)
    pub mod minecraft_server_extension {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // MinecraftServerExtension.chunky$runChunkSystemHousekeeping(1): not translated, it has no body.
        // MinecraftServerExtension.chunky$markChunkSystemHousekeeping(0): not translated, it has no body.
    }

    /// `Cancellable` (org/popcraft/chunky/event/Cancellable.java)
    pub mod cancellable {
        use super::*;
        // The class itself isn't translated: it implements `Event`. Its static parts are below.
        // Cancellable.isCancelled(0): not translated, its class isn't translated (it implements `Event`).
        // Cancellable.setCancelled(1): not translated, its class isn't translated (it implements `Event`).
    }

    /// `ReloadCommandEvent` (org/popcraft/chunky/event/command/ReloadCommandEvent.java)
    pub mod reload_command_event {
        use super::*;
        // The class itself isn't translated: it implements `Event` (org.popcraft.chunky.event). Its static parts are below.
        // ReloadCommandEvent.ReloadCommandEvent(0): not translated, its class isn't translated (it implements `Event` (org.popcraft.chunky.event)).
    }

    /// `Event` (org/popcraft/chunky/event/Event.java)
    pub mod event {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
    }

    /// `EventBus` (org/popcraft/chunky/event/EventBus.java)
    pub mod event_bus {
        use super::*;
        // The class itself isn't translated: it uses `Class`. Its static parts are below.
        // accept: not translated, it uses `MethodHandle` (java.lang.invoke).
        // EventBus.subscribe(2): not translated, it's generic.
        // EventBus.unsubscribe(2): not translated, it's generic.
        // EventBus.unsubscribeAll(0): not translated, its class isn't translated (it uses `Class`).
        // EventBus.unsubscribeAll(1): not translated, its class isn't translated (it uses `Class`).
        // EventBus.call(1): not translated, its class isn't translated (it uses `Class`).
    }

    /// `GenerationTaskFinishEvent` (org/popcraft/chunky/event/task/GenerationTaskFinishEvent.java)
    pub mod generation_task_finish_event {
        use super::*;
        // The class itself isn't translated: it implements `Event` (org.popcraft.chunky.event). Its static parts are below.
        // GenerationTaskFinishEvent.GenerationTaskFinishEvent(1): not translated, its class isn't translated (it implements `Event` (org.popcraft.chunky.event)).
        // GenerationTaskFinishEvent.generationTask(0): not translated, its class isn't translated (it implements `Event` (org.popcraft.chunky.event)).
    }

    /// `GenerationTaskUpdateEvent` (org/popcraft/chunky/event/task/GenerationTaskUpdateEvent.java)
    pub mod generation_task_update_event {
        use super::*;
        // The class itself isn't translated: it implements `Event` (org.popcraft.chunky.event). Its static parts are below.
        // GenerationTaskUpdateEvent.GenerationTaskUpdateEvent(1): not translated, its class isn't translated (it implements `Event` (org.popcraft.chunky.event)).
        // GenerationTaskUpdateEvent.generationTask(0): not translated, its class isn't translated (it implements `Event` (org.popcraft.chunky.event)).
    }

    /// `GenerationTask` (org/popcraft/chunky/GenerationTask.java)
    pub mod generation_task {
        use super::*;
        // The class itself isn't translated: it implements `Runnable`. Its static parts are below.
        // SAMPLE_INTERVAL: not translated, it calls Input.tryInteger, which isn't translated.
        // SAMPLE_SUB_INTERVAL: not translated, it uses GenerationTask.SAMPLE_INTERVAL, which isn't translated (it calls Input.tryInteger, which isn't translated).
        // AUTO_SAVE_INTERVAL: not translated, it calls Input.tryInteger, which isn't translated.
        // GenerationTask.GenerationTask(5): not translated, its class isn't translated (it implements `Runnable`).
        // GenerationTask.GenerationTask(2): not translated, its class isn't translated (it implements `Runnable`).
        // GenerationTask.update(3): not translated, its class isn't translated (it implements `Runnable`).
        // GenerationTask.autoSave(0): not translated, its class isn't translated (it implements `Runnable`).
        // GenerationTask.run(0): not translated, its class isn't translated (it implements `Runnable`).
        // GenerationTask.createCompleteEvent(1): not translated, its class isn't translated (it implements `Runnable`).
        // GenerationTask.stop(1): not translated, its class isn't translated (it implements `Runnable`).
        // GenerationTask.getOwner(0): not translated, its class isn't translated (it implements `Runnable`).
        // GenerationTask.setOwner(1): not translated, its class isn't translated (it implements `Runnable`).
        // GenerationTask.getChunky(0): not translated, its class isn't translated (it implements `Runnable`).
        // GenerationTask.getSelection(0): not translated, its class isn't translated (it implements `Runnable`).
        // GenerationTask.getCount(0): not translated, its class isn't translated (it implements `Runnable`).
        // GenerationTask.getChunkIterator(0): not translated, its class isn't translated (it implements `Runnable`).
        // GenerationTask.getRateLimiter(0): not translated, its class isn't translated (it implements `Runnable`).
        // GenerationTask.getShape(0): not translated, its class isn't translated (it implements `Runnable`).
        // GenerationTask.isCancelled(0): not translated, its class isn't translated (it implements `Runnable`).
        // GenerationTask.getTotalTime(0): not translated, its class isn't translated (it implements `Runnable`).
        // GenerationTask.getProgress(0): not translated, its class isn't translated (it implements `Runnable`).
    }

    /// `GenerationTask.Progress` (org/popcraft/chunky/GenerationTask.java)
    pub mod generation_task_progress {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Progress {
            pub world: String,
            pub chunk_count: i64,
            pub complete: bool,
            pub percent_complete: f32,
            pub hours: i64,
            pub minutes: i64,
            pub seconds: i64,
            pub rate: f64,
            pub chunk_x: i32,
            pub chunk_z: i32,
        }
        impl Progress {
            pub fn new(world: &str) -> Self {
                let mut s = Self { world: String::new(), chunk_count: 0i64, complete: false, percent_complete: 0.0f32, hours: 0i64, minutes: 0i64, seconds: 0i64, rate: 0.0f64, chunk_x: 0i32, chunk_z: 0i32 };
                s.world = world.to_string();
                s
            }
            pub fn get_world(&self) -> String {
                return self.world.to_string();
            }
            pub fn get_chunk_count(&self) -> i64 {
                return self.chunk_count;
            }
            pub fn is_complete(&self) -> bool {
                return self.complete;
            }
            pub fn get_percent_complete(&self) -> f32 {
                return self.percent_complete;
            }
            pub fn get_hours(&self) -> i64 {
                return self.hours;
            }
            pub fn get_minutes(&self) -> i64 {
                return self.minutes;
            }
            pub fn get_seconds(&self) -> i64 {
                return self.seconds;
            }
            pub fn get_rate(&self) -> f64 {
                return self.rate;
            }
            pub fn get_chunk_x(&self) -> i32 {
                return self.chunk_x;
            }
            pub fn get_chunk_z(&self) -> i32 {
                return self.chunk_z;
            }
            // GenerationTask.Progress.sendUpdate(1): not translated, it uses Sender, which isn't translated (it's an interface).
        }
    }

    /// `BorderIntegration` (org/popcraft/chunky/integration/BorderIntegration.java)
    pub mod border_integration {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // BorderIntegration.hasBorder(1): not translated, it has no body.
        // BorderIntegration.getBorder(1): not translated, it has no body.
    }

    /// `Integration` (org/popcraft/chunky/integration/Integration.java)
    pub mod integration {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
    }

    /// `ChunkIterator` (org/popcraft/chunky/iterator/ChunkIterator.java)
    pub mod chunk_iterator {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ChunkIterator.total(0): not translated, it has no body.
        // ChunkIterator.name(0): not translated, it has no body.
        // ChunkIterator.process(0): not translated, its class isn't translated (it's an interface).
    }

    /// `ChunkIteratorFactory` (org/popcraft/chunky/iterator/ChunkIteratorFactory.java)
    pub mod chunk_iterator_factory {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ChunkIteratorFactory {
        }
        impl ChunkIteratorFactory {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ChunkIteratorFactory.getChunkIterator(2): not translated, it uses ChunkIterator, which isn't translated (it's an interface).
        // ChunkIteratorFactory.getChunkIterator(1): not translated, it uses ChunkIterator, which isn't translated (it's an interface).
    }

    /// `ConcentricChunkIterator` (org/popcraft/chunky/iterator/ConcentricChunkIterator.java)
    pub mod concentric_chunk_iterator {
        use super::*;
        // The class itself isn't translated: it implements `ChunkIterator`. Its static parts are below.
        // ConcentricChunkIterator.ConcentricChunkIterator(2): not translated, its class isn't translated (it implements `ChunkIterator`).
        // ConcentricChunkIterator.ConcentricChunkIterator(1): not translated, its class isn't translated (it implements `ChunkIterator`).
        // ConcentricChunkIterator.hasNext(0): not translated, its class isn't translated (it implements `ChunkIterator`).
        // ConcentricChunkIterator.next(0): not translated, its class isn't translated (it implements `ChunkIterator`).
        // ConcentricChunkIterator.total(0): not translated, its class isn't translated (it implements `ChunkIterator`).
        // ConcentricChunkIterator.name(0): not translated, its class isn't translated (it implements `ChunkIterator`).
    }

    /// `CsvChunkIterator` (org/popcraft/chunky/iterator/CsvChunkIterator.java)
    pub mod csv_chunk_iterator {
        use super::*;
        // The class itself isn't translated: it implements `ChunkIterator`. Its static parts are below.
        // CsvChunkIterator.CsvChunkIterator(2): not translated, its class isn't translated (it implements `ChunkIterator`).
        // CsvChunkIterator.CsvChunkIterator(1): not translated, its class isn't translated (it implements `ChunkIterator`).
        // CsvChunkIterator.hasNext(0): not translated, its class isn't translated (it implements `ChunkIterator`).
        // CsvChunkIterator.next(0): not translated, its class isn't translated (it implements `ChunkIterator`).
        // CsvChunkIterator.total(0): not translated, its class isn't translated (it implements `ChunkIterator`).
        // CsvChunkIterator.name(0): not translated, its class isn't translated (it implements `ChunkIterator`).
    }

    /// `Loop2ChunkIterator` (org/popcraft/chunky/iterator/Loop2ChunkIterator.java)
    pub mod loop2_chunk_iterator {
        use super::*;
        // The class itself isn't translated: it implements `ChunkIterator`. Its static parts are below.
        // Loop2ChunkIterator.Loop2ChunkIterator(2): not translated, its class isn't translated (it implements `ChunkIterator`).
        // Loop2ChunkIterator.Loop2ChunkIterator(1): not translated, its class isn't translated (it implements `ChunkIterator`).
        // Loop2ChunkIterator.hasNext(0): not translated, its class isn't translated (it implements `ChunkIterator`).
        // Loop2ChunkIterator.next(0): not translated, its class isn't translated (it implements `ChunkIterator`).
        // Loop2ChunkIterator.total(0): not translated, its class isn't translated (it implements `ChunkIterator`).
        // Loop2ChunkIterator.name(0): not translated, its class isn't translated (it implements `ChunkIterator`).
    }

    /// `PatternType` (org/popcraft/chunky/iterator/PatternType.java)
    pub mod pattern_type {
        use super::*;
        pub const CONCENTRIC: &str = "concentric";
        pub const LOOP: &str = "loop";
        pub const SPIRAL: &str = "spiral";
        pub const CSV: &str = "csv";
        pub const REGION: &str = "region";
        pub const WORLD: &str = "world";
        pub const STRIP: &str = "strip";
        pub const RING: &str = "ring";
        pub static ALL: LazyLock<Vec<String>> = LazyLock::new(|| vec!["concentric".to_string(), "loop".to_string(), "spiral".to_string(), "csv".to_string(), "region".to_string(), "world".to_string(), "strip".to_string(), "ring".to_string()]);
        #[derive(Clone, Debug)]
        pub struct PatternType {
        }
        impl PatternType {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `RegionChunkIterator` (org/popcraft/chunky/iterator/RegionChunkIterator.java)
    pub mod region_chunk_iterator {
        use super::*;
        // The class itself isn't translated: it implements `ChunkIterator`. Its static parts are below.
        // RegionChunkIterator.RegionChunkIterator(2): not translated, its class isn't translated (it implements `ChunkIterator`).
        // RegionChunkIterator.RegionChunkIterator(1): not translated, its class isn't translated (it implements `ChunkIterator`).
        // RegionChunkIterator.hasNext(0): not translated, its class isn't translated (it implements `ChunkIterator`).
        // RegionChunkIterator.next(0): not translated, its class isn't translated (it implements `ChunkIterator`).
        // RegionChunkIterator.nextRegionChunkProgress(0): not translated, its class isn't translated (it implements `ChunkIterator`).
        // RegionChunkIterator.nextRegionChunkProgress(1): not translated, its class isn't translated (it implements `ChunkIterator`).
        // RegionChunkIterator.total(0): not translated, its class isn't translated (it implements `ChunkIterator`).
        // RegionChunkIterator.name(0): not translated, its class isn't translated (it implements `ChunkIterator`).
    }

    /// `RegionChunkIterator.RegionChunkProgress` (org/popcraft/chunky/iterator/RegionChunkIterator.java)
    pub mod region_chunk_iterator_region_chunk_progress {
        use super::*;
        // The class itself isn't translated: it implements `ChunkIterator`. Its static parts are below.
        // RegionChunkIterator.RegionChunkProgress.RegionChunkProgress(3): not translated, its class isn't translated (it implements `ChunkIterator`).
        // RegionChunkIterator.RegionChunkProgress.RegionChunkProgress(2): not translated, its class isn't translated (it implements `ChunkIterator`).
        // RegionChunkIterator.RegionChunkProgress.hasNext(0): not translated, its class isn't translated (it implements `ChunkIterator`).
        // RegionChunkIterator.RegionChunkProgress.next(0): not translated, its class isn't translated (it implements `ChunkIterator`).
        // RegionChunkIterator.RegionChunkProgress.total(0): not translated, its class isn't translated (it implements `ChunkIterator`).
        // RegionChunkIterator.RegionChunkProgress.name(0): not translated, its class isn't translated (it implements `ChunkIterator`).
    }

    /// `RingChunkIterator` (org/popcraft/chunky/iterator/RingChunkIterator.java)
    pub mod ring_chunk_iterator {
        use super::*;
        // The class itself isn't translated: it implements `ChunkIterator`. Its static parts are below.
        pub static SPLIT: LazyLock<bool> = LazyLock::new(|| std::env::var("chunky.ringSplit").is_ok_and(|v| v.eq_ignore_ascii_case("true")));
        pub static ALTERNATING: LazyLock<bool> = LazyLock::new(|| std::env::var("chunky.ringAlternating").is_ok_and(|v| v.eq_ignore_ascii_case("true")));
        // WIDTH: not translated, it calls Input.tryInteger, which isn't translated.
        // CENTER_RADIUS: not translated, it uses RingChunkIterator.WIDTH, which isn't translated (it calls Input.tryInteger, which isn't translated).
        // RingChunkIterator.RingChunkIterator(2): not translated, its class isn't translated (it implements `ChunkIterator`).
        // RingChunkIterator.RingChunkIterator(1): not translated, its class isn't translated (it implements `ChunkIterator`).
        // RingChunkIterator.nextRingChunkProgress(1): not translated, its class isn't translated (it implements `ChunkIterator`).
        // RingChunkIterator.hasNext(0): not translated, its class isn't translated (it implements `ChunkIterator`).
        // RingChunkIterator.next(0): not translated, its class isn't translated (it implements `ChunkIterator`).
        // RingChunkIterator.total(0): not translated, its class isn't translated (it implements `ChunkIterator`).
        // RingChunkIterator.name(0): not translated, its class isn't translated (it implements `ChunkIterator`).
    }

    /// `RingChunkIterator.RingChunkProgress` (org/popcraft/chunky/iterator/RingChunkIterator.java)
    pub mod ring_chunk_iterator_ring_chunk_progress {
        use super::*;
        // The class itself isn't translated: it implements `ChunkIterator`. Its static parts are below.
        // RingChunkIterator.RingChunkProgress.RingChunkProgress(4): not translated, its class isn't translated (it implements `ChunkIterator`).
        // RingChunkIterator.RingChunkProgress.hasNext(0): not translated, its class isn't translated (it implements `ChunkIterator`).
        // RingChunkIterator.RingChunkProgress.next(0): not translated, its class isn't translated (it implements `ChunkIterator`).
        // RingChunkIterator.RingChunkProgress.total(0): not translated, its class isn't translated (it implements `ChunkIterator`).
        // RingChunkIterator.RingChunkProgress.name(0): not translated, its class isn't translated (it implements `ChunkIterator`).
    }

    /// `SpiralChunkIterator` (org/popcraft/chunky/iterator/SpiralChunkIterator.java)
    pub mod spiral_chunk_iterator {
        use super::*;
        // The class itself isn't translated: it implements `ChunkIterator`. Its static parts are below.
        pub const RIGHT: i32 = 0;
        pub const DOWN: i32 = 1;
        pub const LEFT: i32 = 2;
        pub const UP: i32 = 3;
        // SpiralChunkIterator.SpiralChunkIterator(2): not translated, its class isn't translated (it implements `ChunkIterator`).
        // SpiralChunkIterator.SpiralChunkIterator(1): not translated, its class isn't translated (it implements `ChunkIterator`).
        // SpiralChunkIterator.hasNext(0): not translated, its class isn't translated (it implements `ChunkIterator`).
        // SpiralChunkIterator.next(0): not translated, its class isn't translated (it implements `ChunkIterator`).
        // SpiralChunkIterator.total(0): not translated, its class isn't translated (it implements `ChunkIterator`).
        // SpiralChunkIterator.name(0): not translated, its class isn't translated (it implements `ChunkIterator`).
    }

    /// `StripChunkIterator` (org/popcraft/chunky/iterator/StripChunkIterator.java)
    pub mod strip_chunk_iterator {
        use super::*;
        // The class itself isn't translated: it implements `ChunkIterator`. Its static parts are below.
        // WIDTH: not translated, it calls Input.tryInteger, which isn't translated.
        // StripChunkIterator.StripChunkIterator(2): not translated, its class isn't translated (it implements `ChunkIterator`).
        // StripChunkIterator.StripChunkIterator(1): not translated, its class isn't translated (it implements `ChunkIterator`).
        // StripChunkIterator.hasNext(0): not translated, its class isn't translated (it implements `ChunkIterator`).
        // StripChunkIterator.next(0): not translated, its class isn't translated (it implements `ChunkIterator`).
        // StripChunkIterator.total(0): not translated, its class isn't translated (it implements `ChunkIterator`).
        // StripChunkIterator.name(0): not translated, its class isn't translated (it implements `ChunkIterator`).
    }

    /// `WorldChunkIterator` (org/popcraft/chunky/iterator/WorldChunkIterator.java)
    pub mod world_chunk_iterator {
        use super::*;
        // The class itself isn't translated: it implements `ChunkIterator`. Its static parts are below.
        // WorldChunkIterator.WorldChunkIterator(1): not translated, its class isn't translated (it implements `ChunkIterator`).
        // WorldChunkIterator.hasNext(0): not translated, its class isn't translated (it implements `ChunkIterator`).
        // WorldChunkIterator.next(0): not translated, its class isn't translated (it implements `ChunkIterator`).
        // WorldChunkIterator.total(0): not translated, its class isn't translated (it implements `ChunkIterator`).
        // WorldChunkIterator.name(0): not translated, its class isn't translated (it implements `ChunkIterator`).
        // WorldChunkIterator.process(0): not translated, its class isn't translated (it implements `ChunkIterator`).
        // WorldChunkIterator.isInBounds(1): not translated, its class isn't translated (it implements `ChunkIterator`).
        // WorldChunkIterator.addChunk(2): not translated, its class isn't translated (it implements `ChunkIterator`).
        // WorldChunkIterator.neighbors(1): not translated, it uses ChunkCoordinate, which isn't translated (it implements `Comparable`).
    }

    /// `BossBarTaskFinishListener` (org/popcraft/chunky/listeners/bossbar/BossBarTaskFinishListener.java)
    pub mod boss_bar_task_finish_listener {
        use super::*;
        // The class itself isn't translated: it implements `Consumer` (java.util.function). Its static parts are below.
        // BossBarTaskFinishListener.BossBarTaskFinishListener(1): not translated, its class isn't translated (it implements `Consumer` (java.util.function)).
        // BossBarTaskFinishListener.accept(1): not translated, its class isn't translated (it implements `Consumer` (java.util.function)).
    }

    /// `BossBarTaskUpdateListener` (org/popcraft/chunky/listeners/bossbar/BossBarTaskUpdateListener.java)
    pub mod boss_bar_task_update_listener {
        use super::*;
        // The class itself isn't translated: it implements `Consumer` (java.util.function). Its static parts are below.
        // BossBarTaskUpdateListener.BossBarTaskUpdateListener(1): not translated, its class isn't translated (it implements `Consumer` (java.util.function)).
        // BossBarTaskUpdateListener.accept(1): not translated, its class isn't translated (it implements `Consumer` (java.util.function)).
        // BossBarTaskUpdateListener.hasBossBarPermission(1): not translated, it uses Minecraft's `ServerPlayer`.
        // BossBarTaskUpdateListener.createNewBossBar(1): not translated, its class isn't translated (it implements `Consumer` (java.util.function)).
        // BossBarTaskUpdateListener.bossBarColor(1): not translated, it uses Minecraft's `BossBarColor`.
    }

    /// `ChunkyMixinPlugin` (org/popcraft/chunky/mixin/ChunkyMixinPlugin.java)
    pub mod chunky_mixin_plugin {
        use super::*;
        // The class itself isn't translated: it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility). Its static parts are below.
        pub static BOOST_UNLOAD_PRIORITY: LazyLock<bool> = LazyLock::new(|| j_parse_bool(&(std::env::var("chunky.boostUnloadPriority").unwrap_or_else(|_| "true".to_string()))));
        pub static UNLOAD_PRIORITY_MIXINS: LazyLock<JSet<String>> = LazyLock::new(|| JSet::from_iter(["org.popcraft.chunky.mixin.ChunkHolderMixin".to_string(), "org.popcraft.chunky.mixin.ChunkMapPatchMixin".to_string()]));
        // ChunkyMixinPlugin.onLoad(1): not translated, its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)).
        // ChunkyMixinPlugin.getRefMapperConfig(0): not translated, its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)).
        // ChunkyMixinPlugin.shouldApplyMixin(2): not translated, its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)).
        // ChunkyMixinPlugin.acceptTargets(2): not translated, its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)).
        // ChunkyMixinPlugin.getMixins(0): not translated, its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)).
        // ChunkyMixinPlugin.preApply(4): not translated, its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)).
        // ChunkyMixinPlugin.postApply(4): not translated, its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)).
    }

    /// `ByteArrayTag` (org/popcraft/chunky/nbt/ByteArrayTag.java)
    pub mod byte_array_tag {
        use super::*;
        // The class itself isn't translated: it extends `Tag`. Its static parts are below.
        // ByteArrayTag.ByteArrayTag(1): not translated, its class isn't translated (it extends `Tag`).
        // ByteArrayTag.ByteArrayTag(2): not translated, its class isn't translated (it extends `Tag`).
        // ByteArrayTag.read(1): not translated, its class isn't translated (it extends `Tag`).
        // ByteArrayTag.skip(1): not translated, its class isn't translated (it extends `Tag`).
        // ByteArrayTag.write(1): not translated, its class isn't translated (it extends `Tag`).
        // ByteArrayTag.search(3): not translated, its class isn't translated (it extends `Tag`).
        // ByteArrayTag.type(0): not translated, its class isn't translated (it extends `Tag`).
        // ByteArrayTag.typeName(0): not translated, its class isn't translated (it extends `Tag`).
        // ByteArrayTag.print(1): not translated, its class isn't translated (it extends `Tag`).
        // ByteArrayTag.value(0): not translated, its class isn't translated (it extends `Tag`).
        // ByteArrayTag.value(1): not translated, its class isn't translated (it extends `Tag`).
    }

    /// `ByteTag` (org/popcraft/chunky/nbt/ByteTag.java)
    pub mod byte_tag {
        use super::*;
        // The class itself isn't translated: it extends `Tag`. Its static parts are below.
        // ByteTag.ByteTag(1): not translated, its class isn't translated (it extends `Tag`).
        // ByteTag.ByteTag(2): not translated, its class isn't translated (it extends `Tag`).
        // ByteTag.read(1): not translated, its class isn't translated (it extends `Tag`).
        // ByteTag.skip(1): not translated, its class isn't translated (it extends `Tag`).
        // ByteTag.write(1): not translated, its class isn't translated (it extends `Tag`).
        // ByteTag.search(3): not translated, its class isn't translated (it extends `Tag`).
        // ByteTag.type(0): not translated, its class isn't translated (it extends `Tag`).
        // ByteTag.typeName(0): not translated, its class isn't translated (it extends `Tag`).
        // ByteTag.print(1): not translated, its class isn't translated (it extends `Tag`).
        // ByteTag.value(0): not translated, its class isn't translated (it extends `Tag`).
        // ByteTag.value(1): not translated, its class isn't translated (it extends `Tag`).
    }

    /// `CompoundTag` (org/popcraft/chunky/nbt/CompoundTag.java)
    pub mod compound_tag {
        use super::*;
        // The class itself isn't translated: it extends `Tag`. Its static parts are below.
        // CompoundTag.CompoundTag(1): not translated, its class isn't translated (it extends `Tag`).
        // CompoundTag.CompoundTag(2): not translated, its class isn't translated (it extends `Tag`).
        // CompoundTag.read(1): not translated, its class isn't translated (it extends `Tag`).
        // CompoundTag.skip(1): not translated, its class isn't translated (it extends `Tag`).
        // CompoundTag.write(1): not translated, its class isn't translated (it extends `Tag`).
        // CompoundTag.search(3): not translated, its class isn't translated (it extends `Tag`).
        // CompoundTag.type(0): not translated, its class isn't translated (it extends `Tag`).
        // CompoundTag.typeName(0): not translated, its class isn't translated (it extends `Tag`).
        // CompoundTag.print(1): not translated, its class isn't translated (it extends `Tag`).
        // CompoundTag.get(1): not translated, its class isn't translated (it extends `Tag`).
        // CompoundTag.getByteArray(1): not translated, its class isn't translated (it extends `Tag`).
        // CompoundTag.getByte(1): not translated, its class isn't translated (it extends `Tag`).
        // CompoundTag.getCompound(1): not translated, its class isn't translated (it extends `Tag`).
        // CompoundTag.getDouble(1): not translated, its class isn't translated (it extends `Tag`).
        // CompoundTag.getFloat(1): not translated, its class isn't translated (it extends `Tag`).
        // CompoundTag.getIntArray(1): not translated, its class isn't translated (it extends `Tag`).
        // CompoundTag.getInt(1): not translated, its class isn't translated (it extends `Tag`).
        // CompoundTag.getList(1): not translated, its class isn't translated (it extends `Tag`).
        // CompoundTag.getLongArray(1): not translated, its class isn't translated (it extends `Tag`).
        // CompoundTag.getLong(1): not translated, its class isn't translated (it extends `Tag`).
        // CompoundTag.getShort(1): not translated, its class isn't translated (it extends `Tag`).
        // CompoundTag.getString(1): not translated, its class isn't translated (it extends `Tag`).
        // CompoundTag.put(1): not translated, its class isn't translated (it extends `Tag`).
        // CompoundTag.remove(1): not translated, its class isn't translated (it extends `Tag`).
    }

    /// `DoubleTag` (org/popcraft/chunky/nbt/DoubleTag.java)
    pub mod double_tag {
        use super::*;
        // The class itself isn't translated: it extends `Tag`. Its static parts are below.
        // DoubleTag.DoubleTag(1): not translated, its class isn't translated (it extends `Tag`).
        // DoubleTag.DoubleTag(2): not translated, its class isn't translated (it extends `Tag`).
        // DoubleTag.read(1): not translated, its class isn't translated (it extends `Tag`).
        // DoubleTag.skip(1): not translated, its class isn't translated (it extends `Tag`).
        // DoubleTag.write(1): not translated, its class isn't translated (it extends `Tag`).
        // DoubleTag.search(3): not translated, its class isn't translated (it extends `Tag`).
        // DoubleTag.type(0): not translated, its class isn't translated (it extends `Tag`).
        // DoubleTag.typeName(0): not translated, its class isn't translated (it extends `Tag`).
        // DoubleTag.print(1): not translated, its class isn't translated (it extends `Tag`).
        // DoubleTag.value(0): not translated, its class isn't translated (it extends `Tag`).
        // DoubleTag.value(1): not translated, its class isn't translated (it extends `Tag`).
    }

    /// `EndTag` (org/popcraft/chunky/nbt/EndTag.java)
    pub mod end_tag {
        use super::*;
        // The class itself isn't translated: it extends `Tag`. Its static parts are below.
        // EndTag.EndTag(0): not translated, its class isn't translated (it extends `Tag`).
        // EndTag.read(1): not translated, its class isn't translated (it extends `Tag`).
        // EndTag.skip(1): not translated, its class isn't translated (it extends `Tag`).
        // EndTag.write(1): not translated, its class isn't translated (it extends `Tag`).
        // EndTag.search(3): not translated, its class isn't translated (it extends `Tag`).
        // EndTag.type(0): not translated, its class isn't translated (it extends `Tag`).
        // EndTag.typeName(0): not translated, its class isn't translated (it extends `Tag`).
        // EndTag.print(1): not translated, its class isn't translated (it extends `Tag`).
    }

    /// `FloatTag` (org/popcraft/chunky/nbt/FloatTag.java)
    pub mod float_tag {
        use super::*;
        // The class itself isn't translated: it extends `Tag`. Its static parts are below.
        // FloatTag.FloatTag(1): not translated, its class isn't translated (it extends `Tag`).
        // FloatTag.FloatTag(2): not translated, its class isn't translated (it extends `Tag`).
        // FloatTag.read(1): not translated, its class isn't translated (it extends `Tag`).
        // FloatTag.skip(1): not translated, its class isn't translated (it extends `Tag`).
        // FloatTag.write(1): not translated, its class isn't translated (it extends `Tag`).
        // FloatTag.search(3): not translated, its class isn't translated (it extends `Tag`).
        // FloatTag.type(0): not translated, its class isn't translated (it extends `Tag`).
        // FloatTag.typeName(0): not translated, its class isn't translated (it extends `Tag`).
        // FloatTag.print(1): not translated, its class isn't translated (it extends `Tag`).
        // FloatTag.value(0): not translated, its class isn't translated (it extends `Tag`).
        // FloatTag.value(1): not translated, its class isn't translated (it extends `Tag`).
    }

    /// `IntArrayTag` (org/popcraft/chunky/nbt/IntArrayTag.java)
    pub mod int_array_tag {
        use super::*;
        // The class itself isn't translated: it extends `Tag`. Its static parts are below.
        // IntArrayTag.IntArrayTag(1): not translated, its class isn't translated (it extends `Tag`).
        // IntArrayTag.IntArrayTag(2): not translated, its class isn't translated (it extends `Tag`).
        // IntArrayTag.read(1): not translated, its class isn't translated (it extends `Tag`).
        // IntArrayTag.skip(1): not translated, its class isn't translated (it extends `Tag`).
        // IntArrayTag.write(1): not translated, its class isn't translated (it extends `Tag`).
        // IntArrayTag.search(3): not translated, its class isn't translated (it extends `Tag`).
        // IntArrayTag.type(0): not translated, its class isn't translated (it extends `Tag`).
        // IntArrayTag.typeName(0): not translated, its class isn't translated (it extends `Tag`).
        // IntArrayTag.print(1): not translated, its class isn't translated (it extends `Tag`).
        // IntArrayTag.value(0): not translated, its class isn't translated (it extends `Tag`).
        // IntArrayTag.value(1): not translated, its class isn't translated (it extends `Tag`).
    }

    /// `IntTag` (org/popcraft/chunky/nbt/IntTag.java)
    pub mod int_tag {
        use super::*;
        // The class itself isn't translated: it extends `Tag`. Its static parts are below.
        // IntTag.IntTag(1): not translated, its class isn't translated (it extends `Tag`).
        // IntTag.IntTag(2): not translated, its class isn't translated (it extends `Tag`).
        // IntTag.read(1): not translated, its class isn't translated (it extends `Tag`).
        // IntTag.skip(1): not translated, its class isn't translated (it extends `Tag`).
        // IntTag.write(1): not translated, its class isn't translated (it extends `Tag`).
        // IntTag.search(3): not translated, its class isn't translated (it extends `Tag`).
        // IntTag.type(0): not translated, its class isn't translated (it extends `Tag`).
        // IntTag.typeName(0): not translated, its class isn't translated (it extends `Tag`).
        // IntTag.print(1): not translated, its class isn't translated (it extends `Tag`).
        // IntTag.value(0): not translated, its class isn't translated (it extends `Tag`).
        // IntTag.value(1): not translated, its class isn't translated (it extends `Tag`).
    }

    /// `ListTag` (org/popcraft/chunky/nbt/ListTag.java)
    pub mod list_tag {
        use super::*;
        // The class itself isn't translated: it extends `Tag`. Its static parts are below.
        // ListTag.ListTag(1): not translated, its class isn't translated (it extends `Tag`).
        // ListTag.ListTag(3): not translated, its class isn't translated (it extends `Tag`).
        // ListTag.read(1): not translated, its class isn't translated (it extends `Tag`).
        // ListTag.skip(1): not translated, its class isn't translated (it extends `Tag`).
        // ListTag.write(1): not translated, its class isn't translated (it extends `Tag`).
        // ListTag.search(3): not translated, its class isn't translated (it extends `Tag`).
        // ListTag.type(0): not translated, its class isn't translated (it extends `Tag`).
        // ListTag.typeName(0): not translated, its class isn't translated (it extends `Tag`).
        // ListTag.print(1): not translated, its class isn't translated (it extends `Tag`).
        // ListTag.value(0): not translated, its class isn't translated (it extends `Tag`).
        // ListTag.value(1): not translated, its class isn't translated (it extends `Tag`).
    }

    /// `LongArrayTag` (org/popcraft/chunky/nbt/LongArrayTag.java)
    pub mod long_array_tag {
        use super::*;
        // The class itself isn't translated: it extends `Tag`. Its static parts are below.
        // LongArrayTag.LongArrayTag(1): not translated, its class isn't translated (it extends `Tag`).
        // LongArrayTag.LongArrayTag(2): not translated, its class isn't translated (it extends `Tag`).
        // LongArrayTag.read(1): not translated, its class isn't translated (it extends `Tag`).
        // LongArrayTag.skip(1): not translated, its class isn't translated (it extends `Tag`).
        // LongArrayTag.write(1): not translated, its class isn't translated (it extends `Tag`).
        // LongArrayTag.search(3): not translated, its class isn't translated (it extends `Tag`).
        // LongArrayTag.type(0): not translated, its class isn't translated (it extends `Tag`).
        // LongArrayTag.typeName(0): not translated, its class isn't translated (it extends `Tag`).
        // LongArrayTag.print(1): not translated, its class isn't translated (it extends `Tag`).
        // LongArrayTag.value(0): not translated, its class isn't translated (it extends `Tag`).
        // LongArrayTag.value(1): not translated, its class isn't translated (it extends `Tag`).
    }

    /// `LongTag` (org/popcraft/chunky/nbt/LongTag.java)
    pub mod long_tag {
        use super::*;
        // The class itself isn't translated: it extends `Tag`. Its static parts are below.
        // LongTag.LongTag(1): not translated, its class isn't translated (it extends `Tag`).
        // LongTag.LongTag(2): not translated, its class isn't translated (it extends `Tag`).
        // LongTag.read(1): not translated, its class isn't translated (it extends `Tag`).
        // LongTag.skip(1): not translated, its class isn't translated (it extends `Tag`).
        // LongTag.write(1): not translated, its class isn't translated (it extends `Tag`).
        // LongTag.search(3): not translated, its class isn't translated (it extends `Tag`).
        // LongTag.type(0): not translated, its class isn't translated (it extends `Tag`).
        // LongTag.typeName(0): not translated, its class isn't translated (it extends `Tag`).
        // LongTag.print(1): not translated, its class isn't translated (it extends `Tag`).
        // LongTag.value(0): not translated, its class isn't translated (it extends `Tag`).
        // LongTag.value(1): not translated, its class isn't translated (it extends `Tag`).
    }

    /// `NBT` (org/popcraft/chunky/nbt/NBT.java)
    pub mod nbt {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct NBT {
        }
        impl NBT {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // NBT.load(1): not translated, it uses `InputStream` (java.io).
        // NBT.save(2): not translated, it uses `OutputStream` (java.io).
        // NBT.loadCompressed(1): not translated, it uses `InputStream` (java.io).
        // NBT.saveCompressed(2): not translated, it uses `OutputStream` (java.io).
    }

    /// `ShortTag` (org/popcraft/chunky/nbt/ShortTag.java)
    pub mod short_tag {
        use super::*;
        // The class itself isn't translated: it extends `Tag`. Its static parts are below.
        // ShortTag.ShortTag(1): not translated, its class isn't translated (it extends `Tag`).
        // ShortTag.ShortTag(2): not translated, its class isn't translated (it extends `Tag`).
        // ShortTag.read(1): not translated, its class isn't translated (it extends `Tag`).
        // ShortTag.skip(1): not translated, its class isn't translated (it extends `Tag`).
        // ShortTag.write(1): not translated, its class isn't translated (it extends `Tag`).
        // ShortTag.search(3): not translated, its class isn't translated (it extends `Tag`).
        // ShortTag.type(0): not translated, its class isn't translated (it extends `Tag`).
        // ShortTag.typeName(0): not translated, its class isn't translated (it extends `Tag`).
        // ShortTag.print(1): not translated, its class isn't translated (it extends `Tag`).
        // ShortTag.value(0): not translated, its class isn't translated (it extends `Tag`).
        // ShortTag.value(1): not translated, its class isn't translated (it extends `Tag`).
    }

    /// `StringTag` (org/popcraft/chunky/nbt/StringTag.java)
    pub mod string_tag {
        use super::*;
        // The class itself isn't translated: it extends `Tag`. Its static parts are below.
        // StringTag.StringTag(1): not translated, its class isn't translated (it extends `Tag`).
        // StringTag.StringTag(2): not translated, its class isn't translated (it extends `Tag`).
        // StringTag.read(1): not translated, its class isn't translated (it extends `Tag`).
        // StringTag.skip(1): not translated, its class isn't translated (it extends `Tag`).
        // StringTag.write(1): not translated, its class isn't translated (it extends `Tag`).
        // StringTag.search(3): not translated, its class isn't translated (it extends `Tag`).
        // StringTag.type(0): not translated, its class isn't translated (it extends `Tag`).
        // StringTag.typeName(0): not translated, its class isn't translated (it extends `Tag`).
        // StringTag.print(1): not translated, its class isn't translated (it extends `Tag`).
        // StringTag.value(0): not translated, its class isn't translated (it extends `Tag`).
        // StringTag.value(1): not translated, its class isn't translated (it extends `Tag`).
    }

    /// `Tag` (org/popcraft/chunky/nbt/Tag.java)
    pub mod tag {
        use super::*;
        pub const INDENT: i32 = 2;
        #[derive(Clone, Debug)]
        pub struct Tag {
            pub name: String,
        }
        impl Tag {
            pub fn new(name: &str) -> Self {
                let mut s = Self { name: String::new() };
                s.name = name.to_string();
                s
            }
            pub fn name(&self) -> String {
                return self.name.to_string();
            }
            // Tag.read(1): not translated, it has no body.
            // Tag.skip(1): not translated, it has no body.
            // Tag.write(1): not translated, it has no body.
            // Tag.search(3): not translated, it has no body.
            // Tag.type(0): not translated, it has no body.
            // Tag.typeName(0): not translated, it has no body.
            // Tag.print(1): not translated, it has no body.
            // Tag.toString(0): not translated, it calls Tag.print, which isn't translated.
        }
        // Tag.load(1): not translated, it uses `DataInput` (java.io).
        // Tag.pass(1): not translated, it uses `DataInput` (java.io).
        // Tag.skip(2): not translated, it uses `DataInput` (java.io).
        // Tag.save(2): not translated, it uses `DataOutput` (java.io).
        // Tag.find(3): not translated, it uses `DataInput` (java.io).
        // Tag.findMany(2): not translated, it uses CompoundTag, which isn't translated (it extends `Tag`).
        // Tag.create(2): not translated, it creates a EndTag, which isn't translated (it extends `Tag`).
    }

    /// `Tag.SkipTags` (org/popcraft/chunky/nbt/Tag.java)
    pub mod tag_skip_tags {
        use super::*;
        // TAGS: not translated, a Tag with no value (null in Java).
        #[derive(Clone, Debug)]
        pub struct SkipTags {
        }
        impl SkipTags {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `TagType` (org/popcraft/chunky/nbt/TagType.java)
    pub mod tag_type {
        use super::*;
        pub const END: i8 = 0i8;
        pub const BYTE: i8 = 1i8;
        pub const SHORT: i8 = 2i8;
        pub const INT: i8 = 3i8;
        pub const LONG: i8 = 4i8;
        pub const FLOAT: i8 = 5i8;
        pub const DOUBLE: i8 = 6i8;
        pub const BYTE_ARRAY: i8 = 7i8;
        pub const STRING: i8 = 8i8;
        pub const LIST: i8 = 9i8;
        pub const COMPOUND: i8 = 10i8;
        pub const INT_ARRAY: i8 = 11i8;
        pub const LONG_ARRAY: i8 = 12i8;
        #[derive(Clone, Debug)]
        pub struct TagType {
        }
        impl TagType {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `Chunk` (org/popcraft/chunky/nbt/util/Chunk.java)
    pub mod chunk {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Chunk {
            pub x: i32,
            pub z: i32,
            pub data: tag::Tag,
            pub last_modified: i64,
        }
        impl Chunk {
            pub fn new(x: i32, z: i32, data: &tag::Tag, last_modified: i64) -> Self {
                let mut s = Self { x: x, z: z, data: data.clone(), last_modified: last_modified };
                s.x = x;
                s.z = z;
                s.data = data.clone();
                s.last_modified = last_modified;
                s
            }
            pub fn get_data(&self) -> tag::Tag {
                return self.data.clone();
            }
            pub fn get_x(&self) -> i32 {
                return self.x;
            }
            pub fn set_x(&mut self, x: i32) {
                self.x = x;
            }
            pub fn get_z(&self) -> i32 {
                return self.z;
            }
            pub fn set_z(&mut self, z: i32) {
                self.z = z;
            }
            pub fn set_data(&mut self, data: &tag::Tag) {
                self.data = data.clone();
            }
            pub fn get_last_modified(&self) -> i64 {
                return self.last_modified;
            }
            pub fn set_last_modified(&mut self, last_modified: i64) {
                self.last_modified = last_modified;
            }
        }
    }

    /// `ChunkFilter` (org/popcraft/chunky/nbt/util/ChunkFilter.java)
    pub mod chunk_filter {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ChunkFilter {
            pub r#type: i8,
            pub name: Option<String>,
            pub tag_names: Option<JSet<String>>,
        }
        impl ChunkFilter {
            pub fn new(r#type: i8, name: Option<String>, tag_names: Option<JSet<String>>) -> Self {
                let mut s = Self { r#type: r#type, name: name.clone(), tag_names: tag_names.clone() };
                s.r#type = r#type;
                s.name = name.clone();
                s.tag_names = tag_names.clone();
                s
            }
            pub fn get_type(&self) -> i8 {
                return self.r#type;
            }
            pub fn get_name(&self) -> Option<String> {
                return self.name.clone();
            }
            pub fn has_tag_names(&self) -> bool {
                return self.tag_names.is_some();
            }
            pub fn get_tag_names(&self) -> Option<JSet<String>> {
                return self.tag_names.clone();
            }
        }
        pub fn of(r#type: i8, name: &str) -> chunk_filter::ChunkFilter {
            return chunk_filter::ChunkFilter::new(r#type, Some(name.to_string()), None);
        }
        // ChunkFilter.ofTags(1): not translated, a Map or Set keyed by a String[].
    }

    /// `ChunkPos` (org/popcraft/chunky/nbt/util/ChunkPos.java)
    pub mod chunk_pos {
        use super::*;
        #[derive(Clone, Debug, PartialEq)]
        pub struct ChunkPos {
            pub x: i32,
            pub z: i32,
        }
        impl ChunkPos {
            pub fn new(x: i32, z: i32) -> Self {
                let mut s = Self { x: x, z: z };
                s.x = x;
                s.z = z;
                s
            }
            pub fn x(&self) -> i32 {
                return self.x;
            }
            pub fn z(&self) -> i32 {
                return self.z;
            }
        }
        pub fn of(x: i32, z: i32) -> chunk_pos::ChunkPos {
            return chunk_pos::ChunkPos::new(x, z);
        }
    }

    /// `CompressionType` (org/popcraft/chunky/nbt/util/CompressionType.java)
    pub mod compression_type {
        use super::*;
        pub const GZIP: i32 = 1;
        pub const ZLIB: i32 = 2;
        pub const NONE: i32 = 3;
        pub const LZ4: i32 = 4;
        pub const EXTERNAL_FLAG: i32 = 128;
        #[derive(Clone, Debug)]
        pub struct CompressionType {
        }
        impl CompressionType {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `FastBufferedInputStream` (org/popcraft/chunky/nbt/util/FastBufferedInputStream.java)
    pub mod fast_buffered_input_stream {
        use super::*;
        // The class itself isn't translated: it extends `InputStream` (java.io). Its static parts are below.
        pub const BUFFER_SIZE: i32 = 8192;
        // FastBufferedInputStream.FastBufferedInputStream(1): not translated, its class isn't translated (it extends `InputStream` (java.io)).
        // FastBufferedInputStream.read(0): not translated, its class isn't translated (it extends `InputStream` (java.io)).
        // FastBufferedInputStream.read(3): not translated, its class isn't translated (it extends `InputStream` (java.io)).
        // FastBufferedInputStream.skip(1): not translated, its class isn't translated (it extends `InputStream` (java.io)).
        // FastBufferedInputStream.available(0): not translated, its class isn't translated (it extends `InputStream` (java.io)).
        // FastBufferedInputStream.close(0): not translated, its class isn't translated (it extends `InputStream` (java.io)).
        // FastBufferedInputStream.fill(0): not translated, its class isn't translated (it extends `InputStream` (java.io)).
    }

    /// `Lz4Decompressor` (org/popcraft/chunky/nbt/util/Lz4Decompressor.java)
    pub mod lz4_decompressor {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Lz4Decompressor {
        }
        impl Lz4Decompressor {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // Lz4Decompressor.decompress(1): not translated, it uses `InputStream` (java.io).
    }

    /// `RegionFile` (org/popcraft/chunky/nbt/util/RegionFile.java)
    pub mod region_file {
        use super::*;
        // The class itself isn't translated: a Map or Set keyed by a Chunk. Its static parts are below.
        pub const ENTRIES: i32 = 1024;
        pub const SECTOR_SIZE: i32 = 4096;
        // RegionFile.RegionFile(1): not translated, its class isn't translated (a Map or Set keyed by a Chunk).
        // RegionFile.RegionFile(2): not translated, its class isn't translated (a Map or Set keyed by a Chunk).
        // RegionFile.RegionFile(4): not translated, its class isn't translated (a Map or Set keyed by a Chunk).
        // RegionFile.RegionFile(2): not translated, its class isn't translated (a Map or Set keyed by a Chunk).
        // RegionFile.RegionFile(5): not translated, its body couldn't be read: a labelled statement.
        // RegionFile.decompress(2): not translated, its class isn't translated (a Map or Set keyed by a Chunk).
        // RegionFile.getChunks(0): not translated, its class isn't translated (a Map or Set keyed by a Chunk).
        // RegionFile.getChunk(2): not translated, its class isn't translated (a Map or Set keyed by a Chunk).
        // RegionFile.getHeader(0): not translated, its class isn't translated (a Map or Set keyed by a Chunk).
        // RegionFile.isInvalid(2): not translated, its class isn't translated (a Map or Set keyed by a Chunk).
        // RegionFile.isFullyRead(0): not translated, its class isn't translated (a Map or Set keyed by a Chunk).
    }

    /// `RegionFile.Header` (org/popcraft/chunky/nbt/util/RegionFile.java)
    pub mod region_file_header {
        use super::*;
        #[derive(Clone, Debug, PartialEq)]
        pub struct Header {
            pub locations: Vec<i32>,
            pub timestamps: Vec<i32>,
        }
        impl Header {
            pub fn new(locations: &[i32], timestamps: &[i32]) -> Self {
                let mut s = Self { locations: Vec::new(), timestamps: Vec::new() };
                s.locations = locations.to_vec();
                s.timestamps = timestamps.to_vec();
                s
            }
            pub fn has_chunk(&self, x: i32, z: i32) -> bool {
                return self.locations[region_file_header::index(x, z) as usize] != 0;
            }
            // RegionFile.Header.getTimestamp(2): not translated, it calls `Integer`.toUnsignedLong.
            pub fn locations(&self) -> Vec<i32> {
                return self.locations.to_vec();
            }
            pub fn timestamps(&self) -> Vec<i32> {
                return self.timestamps.to_vec();
            }
        }
        pub fn index(x: i32, z: i32) -> i32 {
            return (x & 31) + ((z & 31) * 32);
        }
    }

    /// `Border` (org/popcraft/chunky/platform/Border.java)
    pub mod border {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // Border.getCenter(0): not translated, it has no body.
        // Border.getRadiusX(0): not translated, it has no body.
        // Border.getRadiusZ(0): not translated, it has no body.
        // Border.getShape(0): not translated, it has no body.
    }

    /// `Config` (org/popcraft/chunky/platform/Config.java)
    pub mod config {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // Config.getDirectory(0): not translated, it has no body.
        // Config.getDataDirectory(0): not translated, its class isn't translated (it's an interface).
        // Config.getVersion(0): not translated, it has no body.
        // Config.getLanguage(0): not translated, it has no body.
        // Config.getContinueOnRestart(0): not translated, it has no body.
        // Config.isForceLoadExistingChunks(0): not translated, it has no body.
        // Config.isSilent(0): not translated, it has no body.
        // Config.setSilent(1): not translated, it has no body.
        // Config.getUpdateInterval(0): not translated, it has no body.
        // Config.setUpdateInterval(1): not translated, it has no body.
        // Config.getPlayerPauseCount(0): not translated, it has no body.
        // Config.reload(0): not translated, it has no body.
    }

    /// `FabricBorder` (org/popcraft/chunky/platform/FabricBorder.java)
    pub mod fabric_border {
        use super::*;
        // The class itself isn't translated: it implements `Border`. Its static parts are below.
        // FabricBorder.FabricBorder(1): not translated, its class isn't translated (it implements `Border`).
        // FabricBorder.getCenter(0): not translated, its class isn't translated (it implements `Border`).
        // FabricBorder.getRadiusX(0): not translated, its class isn't translated (it implements `Border`).
        // FabricBorder.getRadiusZ(0): not translated, its class isn't translated (it implements `Border`).
        // FabricBorder.getShape(0): not translated, its class isn't translated (it implements `Border`).
    }

    /// `FabricPlayer` (org/popcraft/chunky/platform/FabricPlayer.java)
    pub mod fabric_player {
        use super::*;
        // The class itself isn't translated: it extends `FabricSender`. Its static parts are below.
        // FabricPlayer.FabricPlayer(1): not translated, its class isn't translated (it extends `FabricSender`).
        // FabricPlayer.isPlayer(0): not translated, its class isn't translated (it extends `FabricSender`).
        // FabricPlayer.getName(0): not translated, its class isn't translated (it extends `FabricSender`).
        // FabricPlayer.getWorld(0): not translated, its class isn't translated (it extends `FabricSender`).
        // FabricPlayer.getLocation(0): not translated, its class isn't translated (it extends `FabricSender`).
        // FabricPlayer.sendMessage(3): not translated, its class isn't translated (it extends `FabricSender`).
        // FabricPlayer.getUUID(0): not translated, its class isn't translated (it extends `FabricSender`).
        // FabricPlayer.teleport(1): not translated, its class isn't translated (it extends `FabricSender`).
        // FabricPlayer.sendActionBar(1): not translated, its class isn't translated (it extends `FabricSender`).
        // FabricPlayer.formatColored(1): not translated, its class isn't translated (it extends `FabricSender`).
    }

    /// `FabricSender` (org/popcraft/chunky/platform/FabricSender.java)
    pub mod fabric_sender {
        use super::*;
        // The class itself isn't translated: it implements `Sender`. Its static parts are below.
        pub static PERMISSION_MANAGERS: LazyLock<Vec<String>> = LazyLock::new(|| vec!["luckperms".to_string(), "player_roles".to_string()]);
        pub static HAS_PERMISSION_MANAGER: LazyLock<bool> = LazyLock::new(|| (*fabric_sender::PERMISSION_MANAGERS).iter().cloned().any(|p_a: String| false));
        pub static PERMISSION_IDS: LazyLock<JCell<JMap<String, Option<JIdentifier>>>> = LazyLock::new(|| JCell::new(JMap::new()));
        // FabricSender.FabricSender(1): not translated, its class isn't translated (it implements `Sender`).
        // FabricSender.isPlayer(0): not translated, its class isn't translated (it implements `Sender`).
        // FabricSender.getName(0): not translated, its class isn't translated (it implements `Sender`).
        // FabricSender.getWorld(0): not translated, its class isn't translated (it implements `Sender`).
        // FabricSender.getLocation(0): not translated, its class isn't translated (it implements `Sender`).
        // FabricSender.hasPermission(1): not translated, its class isn't translated (it implements `Sender`).
        // FabricSender.hasPermission(2): not translated, its class isn't translated (it implements `Sender`).
        pub fn to_identifier(permission: &str) -> Option<JIdentifier> {
            return { let key_0 = permission.to_string(); let a_0 = key_0.clone(); let made = fabric_sender::PERMISSION_IDS.with(|o| o.get(&key_0).cloned()); match made { Some(v) => v, None => { let v = (|key: &String| -> Option<JIdentifier> {
            let separator: i32 = j_index_of_char(key, char::from_u32(46 as u32).unwrap_or('\0'));
            if (separator > 0) && (separator != (j_len(key) - 1)) {
                let namespace: String = (j_substring(key, 0, Some(separator))).to_lowercase();
                let path: String = j_substring(key, separator + 1, None).to_lowercase();
                return JIdentifier::try_build(&namespace, &path);
            } else {
                return None;
            }
        })(&key_0); fabric_sender::PERMISSION_IDS.with(|o| o.or_insert_with(a_0, || v.clone()).clone()) } } };
        }
        // FabricSender.sendMessage(3): not translated, its class isn't translated (it implements `Sender`).
    }

    /// `FabricServer` (org/popcraft/chunky/platform/FabricServer.java)
    pub mod fabric_server {
        use super::*;
        // The class itself isn't translated: it implements `Server`. Its static parts are below.
        // FabricServer.FabricServer(2): not translated, its class isn't translated (it implements `Server`).
        // FabricServer.getIntegrations(0): not translated, its class isn't translated (it implements `Server`).
        // FabricServer.getWorld(1): not translated, its class isn't translated (it implements `Server`).
        // FabricServer.getWorlds(0): not translated, its class isn't translated (it implements `Server`).
        // FabricServer.getMaxWorldSize(0): not translated, its class isn't translated (it implements `Server`).
        // FabricServer.getConsole(0): not translated, its class isn't translated (it implements `Server`).
        // FabricServer.getPlayers(0): not translated, its class isn't translated (it implements `Server`).
        // FabricServer.getPlayer(1): not translated, its class isn't translated (it implements `Server`).
        // FabricServer.getConfig(0): not translated, its class isn't translated (it implements `Server`).
    }

    /// `FabricWorld` (org/popcraft/chunky/platform/FabricWorld.java)
    pub mod fabric_world {
        use super::*;
        // The class itself isn't translated: it implements `World`. Its static parts are below.
        // TICKING_LOAD_DURATION: not translated, it calls Input.tryInteger, which isn't translated.
        // CHUNKY: not translated, it uses Minecraft's `TicketType`.
        // CHUNKY_TICKING: not translated, it uses Minecraft's `TicketType`.
        pub static UPDATE_CHUNK_NBT: LazyLock<bool> = LazyLock::new(|| std::env::var("chunky.updateChunkNbt").is_ok_and(|v| v.eq_ignore_ascii_case("true")));
        pub const FULL_STATE_CONFIRMATION_TIMEOUT_SECONDS: i64 = 30i64;
        // FabricWorld.FabricWorld(1): not translated, its class isn't translated (it implements `World`).
        // FabricWorld.getName(0): not translated, its class isn't translated (it implements `World`).
        // FabricWorld.getKey(0): not translated, its class isn't translated (it implements `World`).
        // FabricWorld.isChunkGenerated(2): not translated, its class isn't translated (it implements `World`).
        // FabricWorld.getChunkAtAsync(2): not translated, its class isn't translated (it implements `World`).
        // FabricWorld.getUUID(0): not translated, its class isn't translated (it implements `World`).
        // FabricWorld.getSeaLevel(0): not translated, its class isn't translated (it implements `World`).
        // FabricWorld.getSpawn(0): not translated, its class isn't translated (it implements `World`).
        // FabricWorld.getWorldBorder(0): not translated, its class isn't translated (it implements `World`).
        // FabricWorld.getElevation(2): not translated, its class isn't translated (it implements `World`).
        // FabricWorld.getElevationAtAsync(2): not translated, its class isn't translated (it implements `World`).
        // FabricWorld.getMaxElevation(0): not translated, its class isn't translated (it implements `World`).
        // FabricWorld.playEffect(2): not translated, its class isn't translated (it implements `World`).
        // FabricWorld.playSound(2): not translated, its class isn't translated (it implements `World`).
        // FabricWorld.getDirectory(1): not translated, its class isn't translated (it implements `World`).
        // FabricWorld.getWorld(0): not translated, its class isn't translated (it implements `World`).
    }

    /// `GsonConfig` (org/popcraft/chunky/platform/impl/GsonConfig.java)
    pub mod gson_config {
        use super::*;
        // The class itself isn't translated: it implements `Config` (org.popcraft.chunky.platform). Its static parts are below.
        pub static GSON: LazyLock<JGson> = LazyLock::new(|| JGson { pretty: true, ..JGson::default() });
        // GsonConfig.GsonConfig(2): not translated, its class isn't translated (it implements `Config` (org.popcraft.chunky.platform)).
        // GsonConfig.getDirectory(0): not translated, its class isn't translated (it implements `Config` (org.popcraft.chunky.platform)).
        // GsonConfig.getDataDirectory(0): not translated, its class isn't translated (it implements `Config` (org.popcraft.chunky.platform)).
        // GsonConfig.getVersion(0): not translated, its class isn't translated (it implements `Config` (org.popcraft.chunky.platform)).
        // GsonConfig.getLanguage(0): not translated, its class isn't translated (it implements `Config` (org.popcraft.chunky.platform)).
        // GsonConfig.getContinueOnRestart(0): not translated, its class isn't translated (it implements `Config` (org.popcraft.chunky.platform)).
        // GsonConfig.isForceLoadExistingChunks(0): not translated, its class isn't translated (it implements `Config` (org.popcraft.chunky.platform)).
        // GsonConfig.isSilent(0): not translated, its class isn't translated (it implements `Config` (org.popcraft.chunky.platform)).
        // GsonConfig.setSilent(1): not translated, its class isn't translated (it implements `Config` (org.popcraft.chunky.platform)).
        // GsonConfig.getUpdateInterval(0): not translated, its class isn't translated (it implements `Config` (org.popcraft.chunky.platform)).
        // GsonConfig.setUpdateInterval(1): not translated, its class isn't translated (it implements `Config` (org.popcraft.chunky.platform)).
        // GsonConfig.getPlayerPauseCount(0): not translated, its class isn't translated (it implements `Config` (org.popcraft.chunky.platform)).
        // GsonConfig.reload(0): not translated, its class isn't translated (it implements `Config` (org.popcraft.chunky.platform)).
        // GsonConfig.saveConfig(0): not translated, its class isn't translated (it implements `Config` (org.popcraft.chunky.platform)).
    }

    /// `GsonConfig.ConfigModel` (org/popcraft/chunky/platform/impl/GsonConfig.java)
    pub mod gson_config_config_model {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ConfigModel {
            pub version: i32,
            pub language: String,
            pub continue_on_restart: bool,
            pub force_load_existing_chunks: bool,
            pub silent: bool,
            pub update_interval: i32,
            pub player_pause_count: Option<i32>,
            pub tasks: Option<JMap<String, gson_config_task_model::TaskModel>>,
        }
        impl ConfigModel {
            pub fn new() -> Self {
                let mut s = Self { version: 2, language: "en".to_string(), continue_on_restart: false, force_load_existing_chunks: false, silent: false, update_interval: 1, player_pause_count: None, tasks: None };
                s
            }
            pub fn get_version(&self) -> i32 {
                return self.version;
            }
            pub fn set_version(&mut self, version: i32) {
                self.version = version;
            }
            pub fn get_language(&self) -> String {
                return self.language.to_string();
            }
            pub fn set_language(&mut self, language: &str) {
                self.language = language.to_string();
            }
            pub fn get_continue_on_restart(&self) -> bool {
                return self.continue_on_restart;
            }
            pub fn set_continue_on_restart(&mut self, continue_on_restart: bool) {
                self.continue_on_restart = continue_on_restart;
            }
            pub fn get_force_load_existing_chunks(&self) -> bool {
                return self.force_load_existing_chunks;
            }
            pub fn set_force_load_existing_chunks(&mut self, force_load_existing_chunks: bool) {
                self.force_load_existing_chunks = force_load_existing_chunks;
            }
            pub fn get_tasks(&self) -> Option<JMap<String, gson_config_task_model::TaskModel>> {
                return self.tasks.clone();
            }
            pub fn set_tasks(&mut self, tasks: &JMap<String, gson_config_task_model::TaskModel>) {
                self.tasks = Some(tasks.clone());
            }
            pub fn is_silent(&self) -> bool {
                return self.silent;
            }
            pub fn set_silent(&mut self, silent: bool) {
                self.silent = silent;
            }
            pub fn get_update_interval(&self) -> i32 {
                return self.update_interval;
            }
            pub fn set_update_interval(&mut self, update_interval: i32) {
                self.update_interval = update_interval;
            }
            pub fn get_player_pause_count(&self) -> Option<i32> {
                return self.player_pause_count.clone();
            }
            pub fn set_player_pause_count(&mut self, player_pause_count: i32) {
                self.player_pause_count = Some(player_pause_count);
            }
        }
    }

    /// `GsonConfig.TaskModel` (org/popcraft/chunky/platform/impl/GsonConfig.java)
    pub mod gson_config_task_model {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct TaskModel {
            pub cancelled: Option<bool>,
            pub radius: Option<f64>,
            pub radius_z: Option<f64>,
            pub center_x: Option<f64>,
            pub center_z: Option<f64>,
            pub iterator: Option<String>,
            pub shape: Option<String>,
            pub count: Option<i64>,
            pub time: Option<i64>,
        }
        impl TaskModel {
            pub fn new() -> Self {
                let mut s = Self { cancelled: None, radius: None, radius_z: None, center_x: None, center_z: None, iterator: None, shape: None, count: None, time: None };
                s
            }
            pub fn get_cancelled(&self) -> Option<bool> {
                return self.cancelled.clone();
            }
            pub fn set_cancelled(&mut self, cancelled: bool) {
                self.cancelled = Some(cancelled);
            }
            pub fn get_radius(&self) -> Option<f64> {
                return self.radius.clone();
            }
            pub fn set_radius(&mut self, radius: f64) {
                self.radius = Some(radius);
            }
            pub fn get_radius_z(&self) -> Option<f64> {
                return self.radius_z.clone();
            }
            pub fn set_radius_z(&mut self, radius_z: f64) {
                self.radius_z = Some(radius_z);
            }
            pub fn get_center_x(&self) -> Option<f64> {
                return self.center_x.clone();
            }
            pub fn set_center_x(&mut self, center_x: f64) {
                self.center_x = Some(center_x);
            }
            pub fn get_center_z(&self) -> Option<f64> {
                return self.center_z.clone();
            }
            pub fn set_center_z(&mut self, center_z: f64) {
                self.center_z = Some(center_z);
            }
            pub fn get_iterator(&self) -> Option<String> {
                return self.iterator.clone();
            }
            pub fn set_iterator(&mut self, iterator: &str) {
                self.iterator = Some(iterator.to_string());
            }
            pub fn get_shape(&self) -> Option<String> {
                return self.shape.clone();
            }
            pub fn set_shape(&mut self, shape: &str) {
                self.shape = Some(shape.to_string());
            }
            pub fn get_count(&self) -> Option<i64> {
                return self.count.clone();
            }
            pub fn set_count(&mut self, count: i64) {
                self.count = Some(count);
            }
            pub fn get_time(&self) -> Option<i64> {
                return self.time.clone();
            }
            pub fn set_time(&mut self, time: i64) {
                self.time = Some(time);
            }
        }
    }

    /// `Player` (org/popcraft/chunky/platform/Player.java)
    pub mod player {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // Player.getUUID(0): not translated, it has no body.
        // Player.teleport(1): not translated, it has no body.
        // Player.sendActionBar(1): not translated, it has no body.
    }

    /// `Sender` (org/popcraft/chunky/platform/Sender.java)
    pub mod sender {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // Sender.isPlayer(0): not translated, it has no body.
        // Sender.getName(0): not translated, it has no body.
        // Sender.getWorld(0): not translated, it has no body.
        // Sender.getLocation(0): not translated, it has no body.
        // Sender.hasPermission(1): not translated, it has no body.
        // Sender.sendMessage(3): not translated, it has no body.
        // Sender.sendMessage(2): not translated, its class isn't translated (it's an interface).
        // Sender.sendMessagePrefixed(2): not translated, its class isn't translated (it's an interface).
    }

    /// `Server` (org/popcraft/chunky/platform/Server.java)
    pub mod server {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // Server.getIntegrations(0): not translated, it has no body.
        // Server.getWorld(1): not translated, it has no body.
        // Server.getWorlds(0): not translated, it has no body.
        // Server.getMaxWorldSize(0): not translated, it has no body.
        // Server.getConsole(0): not translated, it has no body.
        // Server.getPlayers(0): not translated, it has no body.
        // Server.getPlayer(1): not translated, it has no body.
        // Server.getConfig(0): not translated, it has no body.
    }

    /// `Location` (org/popcraft/chunky/platform/util/Location.java)
    pub mod location {
        use super::*;
        // The class itself isn't translated: it uses World, which isn't translated (it's an interface). Its static parts are below.
        // Location.Location(6): not translated, its class isn't translated (it uses World, which isn't translated (it's an interface)).
        // Location.Location(4): not translated, its class isn't translated (it uses World, which isn't translated (it's an interface)).
        // Location.toVector(0): not translated, its class isn't translated (it uses World, which isn't translated (it's an interface)).
        // Location.add(1): not translated, its class isn't translated (it uses World, which isn't translated (it's an interface)).
        // Location.setDirection(1): not translated, its class isn't translated (it uses World, which isn't translated (it's an interface)).
        // Location.getWorld(0): not translated, its class isn't translated (it uses World, which isn't translated (it's an interface)).
        // Location.setWorld(1): not translated, its class isn't translated (it uses World, which isn't translated (it's an interface)).
        // Location.getX(0): not translated, its class isn't translated (it uses World, which isn't translated (it's an interface)).
        // Location.setX(1): not translated, its class isn't translated (it uses World, which isn't translated (it's an interface)).
        // Location.getY(0): not translated, its class isn't translated (it uses World, which isn't translated (it's an interface)).
        // Location.setY(1): not translated, its class isn't translated (it uses World, which isn't translated (it's an interface)).
        // Location.getZ(0): not translated, its class isn't translated (it uses World, which isn't translated (it's an interface)).
        // Location.setZ(1): not translated, its class isn't translated (it uses World, which isn't translated (it's an interface)).
        // Location.getYaw(0): not translated, its class isn't translated (it uses World, which isn't translated (it's an interface)).
        // Location.setYaw(1): not translated, its class isn't translated (it uses World, which isn't translated (it's an interface)).
        // Location.getPitch(0): not translated, its class isn't translated (it uses World, which isn't translated (it's an interface)).
        // Location.setPitch(1): not translated, its class isn't translated (it uses World, which isn't translated (it's an interface)).
    }

    /// `Vector2` (org/popcraft/chunky/platform/util/Vector2.java)
    pub mod vector2 {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Vector2 {
            pub x: f64,
            pub z: f64,
        }
        impl Vector2 {
            pub fn new(x: f64, z: f64) -> Self {
                let mut s = Self { x: x, z: z };
                s.x = x;
                s.z = z;
                s
            }
            // Vector2.add(1): not translated, `this` used as a value.
            // Vector2.multiply(1): not translated, `this` used as a value.
            // Vector2.normalize(0): not translated, `this` used as a value.
            pub fn distance(&self, other: &vector2::Vector2) -> f64 {
                return (self.distance_squared(other)).sqrt();
            }
            pub fn distance_squared(&self, other: &vector2::Vector2) -> f64 {
                let dx: f64 = self.x - other.x;
                let dz: f64 = self.z - other.z;
                return (dx * dx) + (dz * dz);
            }
            pub fn length(&self) -> f64 {
                return self.length_squared().sqrt();
            }
            pub fn length_squared(&self) -> f64 {
                return (self.x * self.x) + (self.z * self.z);
            }
            pub fn get_x(&self) -> f64 {
                return self.x;
            }
            pub fn set_x(&mut self, x: f64) {
                self.x = x;
            }
            pub fn get_z(&self) -> f64 {
                return self.z;
            }
            pub fn set_z(&mut self, z: f64) {
                self.z = z;
            }
        }
        pub fn of(x: f64, z: f64) -> vector2::Vector2 {
            return vector2::Vector2::new(x, z);
        }
    }

    /// `Vector3` (org/popcraft/chunky/platform/util/Vector3.java)
    pub mod vector3 {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Vector3 {
            pub x: f64,
            pub y: f64,
            pub z: f64,
        }
        impl Vector3 {
            pub fn new(x: f64, y: f64, z: f64) -> Self {
                let mut s = Self { x: x, y: y, z: z };
                s.x = x;
                s.y = y;
                s.z = z;
                s
            }
            // Vector3.add(1): not translated, `this` used as a value.
            // Vector3.multiply(1): not translated, `this` used as a value.
            // Vector3.normalize(0): not translated, `this` used as a value.
            pub fn distance(&self, other: &vector3::Vector3) -> f64 {
                return (self.distance_squared(other)).sqrt();
            }
            pub fn distance_squared(&self, other: &vector3::Vector3) -> f64 {
                let dx: f64 = self.x - other.x;
                let dy: f64 = self.y - other.y;
                let dz: f64 = self.z - other.z;
                return ((dx * dx) + (dy * dy)) + (dz * dz);
            }
            pub fn length(&self) -> f64 {
                return self.length_squared().sqrt();
            }
            pub fn length_squared(&self) -> f64 {
                return ((self.x * self.x) + (self.y * self.y)) + (self.z * self.z);
            }
            pub fn get_x(&self) -> f64 {
                return self.x;
            }
            pub fn set_x(&mut self, x: f64) {
                self.x = x;
            }
            pub fn get_y(&self) -> f64 {
                return self.y;
            }
            pub fn set_y(&mut self, y: f64) {
                self.y = y;
            }
            pub fn get_z(&self) -> f64 {
                return self.z;
            }
            pub fn set_z(&mut self, z: f64) {
                self.z = z;
            }
        }
        pub fn of(x: f64, y: f64, z: f64) -> vector3::Vector3 {
            return vector3::Vector3::new(x, y, z);
        }
    }

    /// `World` (org/popcraft/chunky/platform/World.java)
    pub mod world {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // World.getName(0): not translated, it has no body.
        // World.getKey(0): not translated, it has no body.
        // World.isChunkGenerated(2): not translated, it has no body.
        // World.getChunkAtAsync(2): not translated, it has no body.
        // World.getUUID(0): not translated, it has no body.
        // World.getSeaLevel(0): not translated, it has no body.
        // World.getSpawn(0): not translated, it has no body.
        // World.getWorldBorder(0): not translated, it has no body.
        // World.getElevation(2): not translated, it has no body.
        // World.getElevationAtAsync(2): not translated, its class isn't translated (it's an interface).
        // World.getMaxElevation(0): not translated, it has no body.
        // World.playEffect(2): not translated, it has no body.
        // World.playSound(2): not translated, it has no body.
        // World.getDirectory(1): not translated, it has no body.
        // World.getEntitiesDirectory(0): not translated, its class isn't translated (it's an interface).
        // World.getPOIDirectory(0): not translated, its class isn't translated (it's an interface).
        // World.getRegionDirectory(0): not translated, its class isn't translated (it's an interface).
    }

    /// `Selection` (org/popcraft/chunky/Selection.java)
    pub mod selection {
        use super::*;
        // The class itself isn't translated: it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface)). Its static parts are below.
        pub const DEFAULT_CENTER_X: f64 = 0.0f64;
        pub const DEFAULT_CENTER_Z: f64 = 0.0f64;
        pub const DEFAULT_RADIUS: f64 = 500.0f64;
        // Selection.Selection(9): not translated, its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))).
        // Selection.builder(2): not translated, it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface)).
        // Selection.chunky(0): not translated, its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))).
        // Selection.world(0): not translated, its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))).
        // Selection.centerX(0): not translated, its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))).
        // Selection.centerZ(0): not translated, its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))).
        // Selection.radiusX(0): not translated, its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))).
        // Selection.radiusZ(0): not translated, its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))).
        // Selection.pattern(0): not translated, its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))).
        // Selection.shape(0): not translated, its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))).
        // Selection.rateLimit(0): not translated, its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))).
        // Selection.centerChunkX(0): not translated, its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))).
        // Selection.centerChunkZ(0): not translated, its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))).
        // Selection.radiusChunksX(0): not translated, its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))).
        // Selection.radiusChunksZ(0): not translated, its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))).
        // Selection.diameterChunksX(0): not translated, its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))).
        // Selection.diameterChunksZ(0): not translated, its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))).
        // Selection.centerRegionX(0): not translated, its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))).
        // Selection.centerRegionZ(0): not translated, its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))).
        // Selection.radiusRegionsX(0): not translated, its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))).
        // Selection.radiusRegionsZ(0): not translated, its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))).
        // Selection.diameterRegionsX(0): not translated, its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))).
        // Selection.diameterRegionsZ(0): not translated, its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))).
    }

    /// `Selection.Builder` (org/popcraft/chunky/Selection.java)
    pub mod selection_builder {
        use super::*;
        // The class itself isn't translated: it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface)). Its static parts are below.
        // Selection.Builder.Builder(2): not translated, its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))).
        // Selection.Builder.world(1): not translated, its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))).
        // Selection.Builder.center(2): not translated, its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))).
        // Selection.Builder.centerX(1): not translated, its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))).
        // Selection.Builder.centerZ(1): not translated, its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))).
        // Selection.Builder.radius(1): not translated, its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))).
        // Selection.Builder.radiusX(1): not translated, its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))).
        // Selection.Builder.radiusZ(1): not translated, its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))).
        // Selection.Builder.pattern(1): not translated, its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))).
        // Selection.Builder.shape(1): not translated, its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))).
        // Selection.Builder.rateLimit(1): not translated, its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))).
        // Selection.Builder.spawn(0): not translated, its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))).
        // Selection.Builder.worldborder(0): not translated, its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))).
        // Selection.Builder.build(0): not translated, its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))).
    }

    /// `AbstractEllipse` (org/popcraft/chunky/shape/AbstractEllipse.java)
    pub mod abstract_ellipse {
        use super::*;
        // The class itself isn't translated: it extends `AbstractShape`. Its static parts are below.
        // AbstractEllipse.AbstractEllipse(2): not translated, its class isn't translated (it extends `AbstractShape`).
        // AbstractEllipse.radii(0): not translated, it has no body.
    }

    /// `AbstractPolygon` (org/popcraft/chunky/shape/AbstractPolygon.java)
    pub mod abstract_polygon {
        use super::*;
        // The class itself isn't translated: it extends `AbstractShape`. Its static parts are below.
        // AbstractPolygon.AbstractPolygon(2): not translated, its class isn't translated (it extends `AbstractShape`).
        // AbstractPolygon.points(0): not translated, it has no body.
    }

    /// `AbstractShape` (org/popcraft/chunky/shape/AbstractShape.java)
    pub mod abstract_shape {
        use super::*;
        // The class itself isn't translated: it implements `Shape`. Its static parts are below.
        // AbstractShape.AbstractShape(2): not translated, its class isn't translated (it implements `Shape`).
        // AbstractShape.center(0): not translated, its class isn't translated (it implements `Shape`).
    }

    /// `Circle` (org/popcraft/chunky/shape/Circle.java)
    pub mod circle {
        use super::*;
        // The class itself isn't translated: it extends `AbstractEllipse`. Its static parts are below.
        // Circle.Circle(2): not translated, its class isn't translated (it extends `AbstractEllipse`).
        // Circle.isBounding(2): not translated, its class isn't translated (it extends `AbstractEllipse`).
        // Circle.name(0): not translated, its class isn't translated (it extends `AbstractEllipse`).
        // Circle.radii(0): not translated, its class isn't translated (it extends `AbstractEllipse`).
    }

    /// `Diamond` (org/popcraft/chunky/shape/Diamond.java)
    pub mod diamond {
        use super::*;
        // The class itself isn't translated: it extends `AbstractPolygon`. Its static parts are below.
        // Diamond.Diamond(2): not translated, its class isn't translated (it extends `AbstractPolygon`).
        // Diamond.points(0): not translated, its class isn't translated (it extends `AbstractPolygon`).
        // Diamond.isBounding(2): not translated, its class isn't translated (it extends `AbstractPolygon`).
        // Diamond.name(0): not translated, its class isn't translated (it extends `AbstractPolygon`).
    }

    /// `Ellipse` (org/popcraft/chunky/shape/Ellipse.java)
    pub mod ellipse {
        use super::*;
        // The class itself isn't translated: it extends `AbstractEllipse`. Its static parts are below.
        // Ellipse.Ellipse(2): not translated, its class isn't translated (it extends `AbstractEllipse`).
        // Ellipse.isBounding(2): not translated, its class isn't translated (it extends `AbstractEllipse`).
        // Ellipse.name(0): not translated, its class isn't translated (it extends `AbstractEllipse`).
        // Ellipse.radii(0): not translated, its class isn't translated (it extends `AbstractEllipse`).
    }

    /// `Hexagon` (org/popcraft/chunky/shape/Hexagon.java)
    pub mod hexagon {
        use super::*;
        // The class itself isn't translated: it extends `AbstractPolygon`. Its static parts are below.
        // Hexagon.Hexagon(2): not translated, its class isn't translated (it extends `AbstractPolygon`).
        // Hexagon.points(0): not translated, its class isn't translated (it extends `AbstractPolygon`).
        // Hexagon.isBounding(2): not translated, its class isn't translated (it extends `AbstractPolygon`).
        // Hexagon.name(0): not translated, its class isn't translated (it extends `AbstractPolygon`).
    }

    /// `Pentagon` (org/popcraft/chunky/shape/Pentagon.java)
    pub mod pentagon {
        use super::*;
        // The class itself isn't translated: it extends `AbstractPolygon`. Its static parts are below.
        // Pentagon.Pentagon(2): not translated, its class isn't translated (it extends `AbstractPolygon`).
        // Pentagon.points(0): not translated, its class isn't translated (it extends `AbstractPolygon`).
        // Pentagon.isBounding(2): not translated, its class isn't translated (it extends `AbstractPolygon`).
        // Pentagon.name(0): not translated, its class isn't translated (it extends `AbstractPolygon`).
    }

    /// `Rectangle` (org/popcraft/chunky/shape/Rectangle.java)
    pub mod rectangle {
        use super::*;
        // The class itself isn't translated: it extends `AbstractPolygon`. Its static parts are below.
        // Rectangle.Rectangle(2): not translated, its class isn't translated (it extends `AbstractPolygon`).
        // Rectangle.points(0): not translated, its class isn't translated (it extends `AbstractPolygon`).
        // Rectangle.isBounding(2): not translated, its class isn't translated (it extends `AbstractPolygon`).
        // Rectangle.name(0): not translated, its class isn't translated (it extends `AbstractPolygon`).
    }

    /// `Shape` (org/popcraft/chunky/shape/Shape.java)
    pub mod shape {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // Shape.isBounding(2): not translated, it has no body.
        // Shape.name(0): not translated, its class isn't translated (it's an interface).
    }

    /// `ShapeFactory` (org/popcraft/chunky/shape/ShapeFactory.java)
    pub mod shape_factory {
        use super::*;
        // custom: not translated, it uses Selection, which isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))).
        #[derive(Clone, Debug)]
        pub struct ShapeFactory {
        }
        impl ShapeFactory {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ShapeFactory.getShape(1): not translated, it uses Shape, which isn't translated (it's an interface).
        // ShapeFactory.getShape(2): not translated, it uses Shape, which isn't translated (it's an interface).
        // ShapeFactory.registerCustom(2): not translated, it uses Selection, which isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))).
        // ShapeFactory.getCustomTypes(0): not translated, it uses ShapeFactory.custom, which isn't translated (it uses Selection, which isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface)))).
    }

    /// `ShapeType` (org/popcraft/chunky/shape/ShapeType.java)
    pub mod shape_type {
        use super::*;
        pub const CIRCLE: &str = "circle";
        pub const DIAMOND: &str = "diamond";
        pub const ELLIPSE: &str = "ellipse";
        pub const HEXAGON: &str = "hexagon";
        pub const OVAL: &str = "oval";
        pub const PENTAGON: &str = "pentagon";
        pub const RECTANGLE: &str = "rectangle";
        pub const SQUARE: &str = "square";
        pub const STAR: &str = "star";
        pub const TRIANGLE: &str = "triangle";
        pub static DEFAULTS: LazyLock<Vec<String>> = LazyLock::new(|| vec!["circle".to_string(), "diamond".to_string(), "ellipse".to_string(), "hexagon".to_string(), "pentagon".to_string(), "rectangle".to_string(), "square".to_string(), "star".to_string(), "triangle".to_string()]);
        #[derive(Clone, Debug)]
        pub struct ShapeType {
        }
        impl ShapeType {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ShapeType.all(0): not translated, it calls ShapeFactory.getCustomTypes, which isn't translated.
    }

    /// `ShapeUtil` (org/popcraft/chunky/shape/ShapeUtil.java)
    pub mod shape_util {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ShapeUtil {
        }
        impl ShapeUtil {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn inside_line(ax: f64, az: f64, bx: f64, bz: f64, cx: f64, cz: f64) -> bool {
            return ((bx - ax) * (cz - az)) > ((bz - az) * (cx - ax));
        }
        pub fn intersection(l1x1: f64, l1z1: f64, l1x2: f64, l1z2: f64, l2x1: f64, l2z1: f64, l2x2: f64, l2z2: f64) -> Option<vector2::Vector2> {
            let a1: f64 = l1z2 - l1z1;
            let a2: f64 = l2z2 - l2z1;
            let b1: f64 = l1x1 - l1x2;
            let b2: f64 = l2x1 - l2x2;
            let determinant: f64 = (a1 * b2) - (a2 * b1);
            if determinant == 0.0f64 {
                return None;
            }
            let c1: f64 = (a1 * l1x1) + (b1 * l1z1);
            let c2: f64 = (a2 * l2x1) + (b2 * l2z1);
            let x: f64 = ((b2 * c1) - (b1 * c2)) / determinant;
            let z: f64 = ((a1 * c2) - (a2 * c1)) / determinant;
            return Some(vector2::of(x, z));
        }
        pub fn point_on_ellipse(center_x: f64, center_z: f64, radius_x: f64, radius_z: f64, angle: f64) -> vector2::Vector2 {
            let x: f64 = center_x + (radius_x * angle.cos());
            let z: f64 = center_z + (radius_z * angle.sin());
            return vector2::of(x, z);
        }
        // ShapeUtil.closestPointOnLine(6): not translated, Optional.orElseThrow.
        pub fn distance_between_points(p1x: f64, p1z: f64, p2x: f64, p2z: f64) -> f64 {
            return (((p1x - p2x).powf(2.0f64)) + ((p1z - p2z).powf(2.0f64))).sqrt();
        }
    }

    /// `Square` (org/popcraft/chunky/shape/Square.java)
    pub mod square {
        use super::*;
        // The class itself isn't translated: it extends `AbstractPolygon`. Its static parts are below.
        // Square.Square(2): not translated, its class isn't translated (it extends `AbstractPolygon`).
        // Square.points(0): not translated, its class isn't translated (it extends `AbstractPolygon`).
        // Square.isBounding(2): not translated, its class isn't translated (it extends `AbstractPolygon`).
        // Square.name(0): not translated, its class isn't translated (it extends `AbstractPolygon`).
    }

    /// `Star` (org/popcraft/chunky/shape/Star.java)
    pub mod star {
        use super::*;
        // The class itself isn't translated: it extends `AbstractPolygon`. Its static parts are below.
        // Star.Star(2): not translated, its class isn't translated (it extends `AbstractPolygon`).
        // Star.points(0): not translated, its class isn't translated (it extends `AbstractPolygon`).
        // Star.isBounding(2): not translated, its class isn't translated (it extends `AbstractPolygon`).
        // Star.name(0): not translated, its class isn't translated (it extends `AbstractPolygon`).
    }

    /// `Triangle` (org/popcraft/chunky/shape/Triangle.java)
    pub mod triangle {
        use super::*;
        // The class itself isn't translated: it extends `AbstractPolygon`. Its static parts are below.
        // Triangle.Triangle(2): not translated, its class isn't translated (it extends `AbstractPolygon`).
        // Triangle.points(0): not translated, its class isn't translated (it extends `AbstractPolygon`).
        // Triangle.isBounding(2): not translated, its class isn't translated (it extends `AbstractPolygon`).
        // Triangle.name(0): not translated, its class isn't translated (it extends `AbstractPolygon`).
    }

    /// `InhabitedTimeFilter` (org/popcraft/chunky/trim/InhabitedTimeFilter.java)
    pub mod inhabited_time_filter {
        use super::*;
        // The class itself isn't translated: it implements `TrimFilter`. Its static parts are below.
        // InhabitedTimeFilter.InhabitedTimeFilter(2): not translated, its class isn't translated (it implements `TrimFilter`).
        // InhabitedTimeFilter.InhabitedTimeFilter(1): not translated, its class isn't translated (it implements `TrimFilter`).
        // InhabitedTimeFilter.test(1): not translated, its class isn't translated (it implements `TrimFilter`).
        // InhabitedTimeFilter.readsChunkData(0): not translated, its class isn't translated (it implements `TrimFilter`).
        // InhabitedTimeFilter.sendNotice(1): not translated, its class isn't translated (it implements `TrimFilter`).
        // InhabitedTimeFilter.minTicks(0): not translated, its class isn't translated (it implements `TrimFilter`).
        // InhabitedTimeFilter.maxTicks(0): not translated, its class isn't translated (it implements `TrimFilter`).
    }

    /// `TrimChunk` (org/popcraft/chunky/trim/TrimChunk.java)
    pub mod trim_chunk {
        use super::*;
        #[derive(Clone, Debug, PartialEq)]
        pub struct TrimChunk {
            pub inhabited_time: i64,
            pub exists: bool,
            pub last_modified: i64,
        }
        impl TrimChunk {
            pub fn new(inhabited_time: i64, exists: bool, last_modified: i64) -> Self {
                let mut s = Self { inhabited_time: inhabited_time, exists: exists, last_modified: last_modified };
                s.inhabited_time = inhabited_time;
                s.exists = exists;
                s.last_modified = last_modified;
                s
            }
            pub fn inhabited_time(&self) -> i64 {
                return self.inhabited_time;
            }
            pub fn exists(&self) -> bool {
                return self.exists;
            }
            pub fn last_modified(&self) -> i64 {
                return self.last_modified;
            }
        }
    }

    /// `TrimFilter` (org/popcraft/chunky/trim/TrimFilter.java)
    pub mod trim_filter {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // TrimFilter.test(1): not translated, it has no body.
        // TrimFilter.readsChunkData(0): not translated, its class isn't translated (it's an interface).
        // TrimFilter.sendNotice(1): not translated, it has no body.
    }

    /// `TrimFilters` (org/popcraft/chunky/trim/TrimFilters.java)
    pub mod trim_filters {
        use super::*;
        pub static KEYS: LazyLock<Vec<String>> = LazyLock::new(|| vec!["inhabited".to_string(), "unmodified".to_string()]);
        pub const RANGE: &str = "..";
        pub const TICKS_PER_SECOND: i64 = 20i64;
        #[derive(Clone, Debug)]
        pub struct TrimFilters {
        }
        impl TrimFilters {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn keys() -> Vec<String> {
            return (*trim_filters::KEYS).to_vec();
        }
        // TrimFilters.parse(1): not translated, it uses TrimFilter, which isn't translated (it's an interface).
        // TrimFilters.suggestions(1): not translated, List.subList.
        // TrimFilters.parseInhabited(1): not translated, it uses InhabitedTimeFilter, which isn't translated (it implements `TrimFilter`).
        // TrimFilters.parseUnmodified(1): not translated, it uses UnmodifiedFilter, which isn't translated (it implements `TrimFilter`).
        // TrimFilters.parseTicks(1): not translated, it uses `TimeUnit` (java.util.concurrent).
        // TrimFilters.timeUnit(1): not translated, it uses `TimeUnit` (java.util.concurrent).
        // TrimFilters.parseAmount(1): not translated, it calls Input.tryDouble, which isn't translated.
    }

    /// `TrimRegion` (org/popcraft/chunky/trim/TrimRegion.java)
    pub mod trim_region {
        use super::*;
        // The class itself isn't translated: it uses RegionFile, which isn't translated (a Map or Set keyed by a Chunk). Its static parts are below.
        pub const SECTOR_SIZE: i32 = 4096;
        // TrimRegion.TrimRegion(4): not translated, its class isn't translated (it uses RegionFile, which isn't translated (a Map or Set keyed by a Chunk)).
        // TrimRegion.isFullyRead(0): not translated, its class isn't translated (it uses RegionFile, which isn't translated (a Map or Set keyed by a Chunk)).
        // TrimRegion.getTrimChunk(2): not translated, its class isn't translated (it uses RegionFile, which isn't translated (a Map or Set keyed by a Chunk)).
        // TrimRegion.isValid(1): not translated, `null` (Rust has no null: this needs an Option).
    }

    /// `UnmodifiedFilter` (org/popcraft/chunky/trim/UnmodifiedFilter.java)
    pub mod unmodified_filter {
        use super::*;
        // The class itself isn't translated: it implements `TrimFilter`. Its static parts are below.
        // UnmodifiedFilter.UnmodifiedFilter(3): not translated, its class isn't translated (it implements `TrimFilter`).
        // UnmodifiedFilter.test(1): not translated, its class isn't translated (it implements `TrimFilter`).
        // UnmodifiedFilter.sendNotice(1): not translated, its class isn't translated (it implements `TrimFilter`).
        // UnmodifiedFilter.unitKey(0): not translated, its class isn't translated (it implements `TrimFilter`).
        // UnmodifiedFilter.amount(0): not translated, its class isn't translated (it implements `TrimFilter`).
        // UnmodifiedFilter.unit(0): not translated, its class isn't translated (it implements `TrimFilter`).
        // UnmodifiedFilter.cutoff(0): not translated, its class isn't translated (it implements `TrimFilter`).
    }

    /// `ChunkCoordinate` (org/popcraft/chunky/util/ChunkCoordinate.java)
    pub mod chunk_coordinate {
        use super::*;
        // The class itself isn't translated: it implements `Comparable`. Its static parts are below.
        // ChunkCoordinate.ChunkCoordinate(2): not translated, its class isn't translated (it implements `Comparable`).
        // ChunkCoordinate.fromRegionFile(1): not translated, it uses ChunkCoordinate, which isn't translated (it implements `Comparable`).
        // ChunkCoordinate.compareTo(1): not translated, its class isn't translated (it implements `Comparable`).
        // ChunkCoordinate.equals(1): not translated, its class isn't translated (it implements `Comparable`).
        // ChunkCoordinate.hashCode(0): not translated, its class isn't translated (it implements `Comparable`).
        // ChunkCoordinate.toString(0): not translated, its class isn't translated (it implements `Comparable`).
        // ChunkCoordinate.x(0): not translated, its class isn't translated (it implements `Comparable`).
        // ChunkCoordinate.z(0): not translated, its class isn't translated (it implements `Comparable`).
    }

    /// `ChunkMath` (org/popcraft/chunky/util/ChunkMath.java)
    pub mod chunk_math {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ChunkMath {
        }
        impl ChunkMath {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn pack(x: i32, z: i32) -> i64 {
            let lx: i64 = (x as i64) & 4294967295i64;
            let lz: i64 = (z as i64) & 4294967295i64;
            return (lx << 32) | lz;
        }
        pub fn region_index(x: i32, z: i32) -> i32 {
            return ((x & 31) * 32) + (z & 31);
        }
    }

    /// `Disk` (org/popcraft/chunky/util/Disk.java)
    pub mod disk {
        use super::*;
        pub const ESTIMATED_SPACE_PER_CHUNK: i64 = 7031i64;
        pub const PERCENT_OVERESTIMATE: f64 = 1.05f64;
        #[derive(Clone, Debug)]
        pub struct Disk {
        }
        impl Disk {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // Disk.estimatedSpace(1): not translated, it uses Selection, which isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))).
        // Disk.remainingSpace(1): not translated, it uses World, which isn't translated (it's an interface).
    }

    /// `EnumUtil` (org/popcraft/chunky/util/EnumUtil.java)
    pub mod enum_util {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct EnumUtil {
        }
        impl EnumUtil {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EnumUtil.valueOf(2): not translated, it's generic.
    }

    /// `Formatting` (org/popcraft/chunky/util/Formatting.java)
    pub mod formatting {
        use super::*;
        // NUMBER_FORMAT: not translated, it uses `DecimalFormat` (java.text).
        pub static BINARY_PREFIXES: LazyLock<Vec<char>> = LazyLock::new(|| vec!['K', 'M', 'G', 'T', 'P']);
        #[derive(Clone, Debug)]
        pub struct Formatting {
        }
        impl Formatting {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // Formatting.bytes(1): not translated, String.format with `%cB`.
        // Formatting.radius(1): not translated, it uses Selection, which isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))).
        // Formatting.worlds(1): not translated, it uses World, which isn't translated (it's an interface).
        // Formatting.number(1): not translated, it uses Formatting.NUMBER_FORMAT, which isn't translated (it uses `DecimalFormat` (java.text)).
    }

    /// `GenerationMap` (org/popcraft/chunky/util/GenerationMap.java)
    pub mod generation_map {
        use super::*;
        // The class itself isn't translated: it uses Selection, which isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))). Its static parts are below.
        pub const FULL: i32 = 0;
        pub const UNFINISHED: i32 = 1;
        pub const MISSING: i32 = 2;
        pub const INCOMPLETE: i32 = 3;
        pub const INVALID: i32 = 4;
        pub const STATUSES: i32 = 5;
        pub static COLORS: LazyLock<Vec<i32>> = LazyLock::new(|| vec![8368696, 5214176, 0, 15066419, 14695484, 16777215]);
        pub const OUTSIDE: i32 = 5;
        pub const MAX_SIZE: i32 = 4096;
        // FILTER: not translated, it calls ChunkFilter.ofTags, which isn't translated.
        // THREADS: not translated, it calls `Runtime`.getRuntime.
        // GenerationMap.GenerationMap(1): not translated, its class isn't translated (it uses Selection, which isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface)))).
        // GenerationMap.scan(0): not translated, its class isn't translated (it uses Selection, which isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface)))).
        // GenerationMap.scanRegion(1): not translated, its class isn't translated (it uses Selection, which isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface)))).
        // GenerationMap.classify(3): not translated, its class isn't translated (it uses Selection, which isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface)))).
        // GenerationMap.record(3): not translated, its class isn't translated (it uses Selection, which isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface)))).
        // GenerationMap.countMissing(0): not translated, its class isn't translated (it uses Selection, which isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface)))).
        // GenerationMap.isInside(2): not translated, its class isn't translated (it uses Selection, which isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface)))).
        // GenerationMap.getCount(1): not translated, its class isn't translated (it uses Selection, which isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface)))).
        // GenerationMap.isExact(0): not translated, its class isn't translated (it uses Selection, which isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface)))).
        // GenerationMap.write(1): not translated, its class isn't translated (it uses Selection, which isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface)))).
    }

    /// `Hilbert` (org/popcraft/chunky/util/Hilbert.java)
    pub mod hilbert {
        use super::*;
        // regionChunkCoordinateOffsets: not translated, it uses ChunkCoordinate, which isn't translated (it implements `Comparable`).
        #[derive(Clone, Debug)]
        pub struct Hilbert {
        }
        impl Hilbert {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // Hilbert.regionDistanceToChunkCoordinateOffset(1): not translated, it uses ChunkCoordinate, which isn't translated (it implements `Comparable`).
        // Hilbert.chunkCoordinateOffsets(0): not translated, it uses ChunkCoordinate, which isn't translated (it implements `Comparable`).
    }

    /// `Input` (org/popcraft/chunky/util/Input.java)
    pub mod input {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Input {
        }
        impl Input {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // Input.tryWorld(2): not translated, it uses World, which isn't translated (it's an interface).
        pub fn try_pattern(input: Option<String>) -> Option<String> {
            if input.is_some() && (!((input.as_deref().expect("null")).is_empty())) {
                let input_lower: String = (input.as_deref().expect("null")).to_lowercase();
                return if (*pattern_type::ALL).iter().any(|e| e == &input_lower) { Some(input_lower.to_string()) } else { None };
            } else {
                return None;
            }
            #[allow(unreachable_code)]
            unreachable!()
        }
        // Input.tryShape(1): not translated, it calls ShapeType.all, which isn't translated.
        // Input.tryBoolean(1): not translated, it calls `Boolean`.parseBoolean.
        // Input.tryInteger(1): not translated, parses something that isn't a string.
        // Input.tryIntegerSuffixed(1): not translated, it calls Input.tryInteger, which isn't translated.
        // Input.tryDouble(1): not translated, parses something that isn't a string.
        // Input.tryDoubleSuffixed(1): not translated, it calls Input.tryDouble, which isn't translated.
        // Input.tryLong(1): not translated, parses something that isn't a string.
        // Input.trySign(1): not translated, a switch expression giving a Optional<int>.
        pub fn is_past_world_limit(value: f64) -> bool {
            return value.abs() > 3.0e7f64;
        }
        // Input.checkLanguage(1): not translated, it calls Translator.isValidLanguage, which isn't translated.
        // Input.suffixValue(1): not translated, a switch expression giving a Optional<int>.
    }

    /// `MaxWorkingCount` (org/popcraft/chunky/util/MaxWorkingCount.java)
    pub mod max_working_count {
        use super::*;
        pub const DEFAULT: i32 = 64;
        // PROPERTY: not translated, it calls Input.tryInteger, which isn't translated.
        #[derive(Clone, Debug)]
        pub struct MaxWorkingCount {
        }
        impl MaxWorkingCount {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // MaxWorkingCount.get(0): not translated, it uses MaxWorkingCount.PROPERTY, which isn't translated (it calls Input.tryInteger, which isn't translated).
    }

    /// `Pair` (org/popcraft/chunky/util/Pair.java)
    pub mod pair {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // Pair.Pair(2): not translated, its class isn't translated (it's generic).
        // Pair.of(2): not translated, it's generic.
        // Pair.left(0): not translated, its class isn't translated (it's generic).
        // Pair.right(0): not translated, its class isn't translated (it's generic).
    }

    /// `Parameter` (org/popcraft/chunky/util/Parameter.java)
    pub mod parameter {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Parameter {
            pub r#type: String,
            pub value: Option<String>,
        }
        impl Parameter {
            pub fn new_2(r#type: &str, value: &str) -> Self {
                let mut s = Self { r#type: String::new(), value: Some(value.to_string()) };
                s.r#type = (r#type).to_string();
                s.value = Some(value.to_string());
                s
            }
            pub fn new_1(expression: &str) -> Self {
                let mut s = Self { r#type: String::new(), value: None };
                let parts: Vec<String> = j_split(&expression, "=");
                s.r#type = parts[0].to_string();
                s.value = if (parts.len() as i32) > 1 { Some(parts[1].to_string()) } else { None };
                s
            }
            pub fn get_type(&self) -> String {
                return (self.r#type).to_string();
            }
            pub fn get_value(&self) -> Option<String> {
                return self.value.clone();
            }
            pub fn to_string(&self) -> String {
                let mut builder: String = (self.r#type).to_string();
                if self.value.is_some() {
                    builder.push_str("=");
                    builder.push_str(&(format!("{}", self.value.as_ref().map_or_else(|| "null".to_string(), |v| v.to_string()))));
                }
                return builder.to_string();
            }
        }
        pub fn of_1(expression: &str) -> parameter::Parameter {
            return parameter::Parameter::new_1(&expression);
        }
        pub fn of_2(r#type: &str, value: &str) -> parameter::Parameter {
            return parameter::Parameter::new_2(&(r#type), &value);
        }
    }

    /// `PendingAction` (org/popcraft/chunky/util/PendingAction.java)
    pub mod pending_action {
        use super::*;
        // The class itself isn't translated: a function kept in a variable or field (only parameters can be). Its static parts are below.
        // PendingAction.PendingAction(1): not translated, its class isn't translated (a function kept in a variable or field (only parameters can be)).
        // PendingAction.getAction(0): not translated, its class isn't translated (a function kept in a variable or field (only parameters can be)).
        // PendingAction.hasExpired(0): not translated, its class isn't translated (a function kept in a variable or field (only parameters can be)).
    }

    /// `PlayerPauseTask` (org/popcraft/chunky/util/PlayerPauseTask.java)
    pub mod player_pause_task {
        use super::*;
        // The class itself isn't translated: it keeps an atomic value in an object's field. Its static parts are below.
        // INTERVAL: not translated, it calls Input.tryInteger, which isn't translated.
        // PlayerPauseTask.PlayerPauseTask(1): not translated, its class isn't translated (it keeps an atomic value in an object's field).
        // PlayerPauseTask.register(1): not translated, its class isn't translated (it keeps an atomic value in an object's field).
        // PlayerPauseTask.start(0): not translated, its class isn't translated (it keeps an atomic value in an object's field).
        // PlayerPauseTask.stop(0): not translated, its class isn't translated (it keeps an atomic value in an object's field).
        // PlayerPauseTask.schedule(0): not translated, its class isn't translated (it keeps an atomic value in an object's field).
        // PlayerPauseTask.update(0): not translated, its class isn't translated (it keeps an atomic value in an object's field).
    }

    /// `RateLimiter` (org/popcraft/chunky/util/RateLimiter.java)
    pub mod rate_limiter {
        use super::*;
        pub const SLOT_MILLIS: i64 = 100i64;
        // SECOND_NANOS: not translated, it calls `SECONDS`.toNanos.
        // BUDGET_POLL_NANOS: not translated, it calls `MILLISECONDS`.toNanos.
        #[derive(Clone, Debug)]
        pub struct RateLimiter {
            pub window_millis: i64,
            pub slot_indexes: Vec<i64>,
            pub slot_counts: Vec<i64>,
            pub limit: i32,
            pub restart: bool,
            pub window: i64,
            pub start: i64,
            pub next: i64,
        }
        impl RateLimiter {
            pub fn new(window_millis: i64, limit: i32) -> Self {
                let mut s = Self { window_millis: 0i64, slot_indexes: Vec::new(), slot_counts: Vec::new(), limit: 0i32, restart: true, window: 0i64, start: 0i64, next: 0i64 };
                let slots: i32 = ((window_millis / 100i64) as i32) + 1;
                s.window_millis = ((slots - 1) as i64) * 100i64;
                s.slot_indexes = vec![0i64; slots as usize];
                s.slot_counts = vec![0i64; slots as usize];
                s.limit = (0i32).max(limit);
                s
            }
            pub fn get_limit(&self) -> i32 {
                return self.limit;
            }
            pub fn set_limit(&mut self, limit: i32) {
                let new_limit: i32 = (0i32).max(limit);
                if new_limit != self.limit {
                    self.limit = new_limit;
                    self.restart = true;
                }
            }
            // RateLimiter.acquire(2): not translated, it uses `Semaphore` (java.util.concurrent).
            // RateLimiter.delay(3): not translated, it uses RateLimiter.SECOND_NANOS, which isn't translated (it calls `SECONDS`.toNanos).
            pub fn release(&mut self, window: i64) {
                if window > 0i64 {
                    self.record(j_millis(), window);
                }
            }
            pub fn record(&mut self, now_millis: i64, window: i64) {
                if window == self.window {
                    let index: i64 = now_millis / 100i64;
                    let slot: i32 = ((index % ((self.slot_indexes.len() as i32) as i64)) as i32);
                    if self.slot_indexes[slot as usize] != index {
                        self.slot_indexes[slot as usize] = index;
                        self.slot_counts[slot as usize] = 0i64;
                    }
                    self.slot_counts[slot as usize] += 1i64;
                }
            }
            pub fn rate(&self, now_millis: i64) -> f64 {
                let millis: i64 = self.window_millis.min(now_millis - self.start);
                return if ((self.limit > 0) && (!self.restart)) && (millis > 0i64) { (1000.0f64 * ((self.count(now_millis, (self.slot_indexes.len() as i32) - 1)) as f64)) / (millis as f64) } else { 0.0f64 };
            }
            pub fn count(&self, now_millis: i64, slots: i32) -> i64 {
                let index: i64 = now_millis / 100i64;
                let mut count: i64 = 0i64;
                for slot in 0..(self.slot_indexes.len() as i32) {
                    if self.slot_indexes[slot as usize] > (index - (slots as i64)) {
                        count += self.slot_counts[slot as usize];
                    }
                }
                return count;
            }
        }
    }

    /// `Reflection` (org/popcraft/chunky/util/Reflection.java)
    pub mod reflection {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Reflection {
        }
        impl Reflection {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // Reflection.classExists(1): not translated, a try/catch (ClassNotFoundException).
        // Reflection.methodExists(3): not translated, it uses `Class`.
        // Reflection.getStaticField(2): not translated, it uses `Object`.
        // Reflection.invokeMethod(2): not translated, it uses `Object`.
    }

    /// `RegionCache` (org/popcraft/chunky/util/RegionCache.java)
    pub mod region_cache {
        use super::*;
        // The class itself isn't translated: it uses WorldState, which isn't translated (it uses `BitSet` (java.util)). Its static parts are below.
        // RegionCache.getWorld(1): not translated, its class isn't translated (it uses WorldState, which isn't translated (it uses `BitSet` (java.util))).
        // RegionCache.clear(1): not translated, its class isn't translated (it uses WorldState, which isn't translated (it uses `BitSet` (java.util))).
    }

    /// `RegionCache.WorldState` (org/popcraft/chunky/util/RegionCache.java)
    pub mod region_cache_world_state {
        use super::*;
        // The class itself isn't translated: it uses `BitSet` (java.util). Its static parts are below.
        // RegionCache.WorldState.setGenerated(2): not translated, its class isn't translated (it uses `BitSet` (java.util)).
        // RegionCache.WorldState.isGenerated(2): not translated, its class isn't translated (it uses `BitSet` (java.util)).
    }

    /// `RegionStatusReader` (org/popcraft/chunky/util/RegionStatusReader.java)
    pub mod region_status_reader {
        use super::*;
        // The class itself isn't translated: it implements `AutoCloseable`. Its static parts are below.
        pub static UPDATE_CHUNK_NBT: LazyLock<bool> = LazyLock::new(|| std::env::var("chunky.updateChunkNbt").is_ok_and(|v| v.eq_ignore_ascii_case("true")));
        pub static LOAD_OUTDATED_CHUNKS: LazyLock<bool> = LazyLock::new(|| std::env::var("chunky.loadOutdatedChunks").is_ok_and(|v| v.eq_ignore_ascii_case("true")));
        // THREADS: not translated, it calls Input.tryInteger, which isn't translated.
        // RegionStatusReader.RegionStatusReader(3): not translated, its class isn't translated (it implements `AutoCloseable`).
        // RegionStatusReader.getState(2): not translated, its class isn't translated (it implements `AutoCloseable`).
        // RegionStatusReader.read(3): not translated, its class isn't translated (it implements `AutoCloseable`).
        // RegionStatusReader.isValid(3): not translated, it uses CompoundTag, which isn't translated (it extends `Tag`).
        // RegionStatusReader.isFullStatus(1): not translated, it uses CompoundTag, which isn't translated (it extends `Tag`).
        // RegionStatusReader.isFull(1): not translated, it uses CompoundTag, which isn't translated (it extends `Tag`).
        // RegionStatusReader.close(0): not translated, its class isn't translated (it implements `AutoCloseable`).
    }

    /// `RegionStatusReader.ChunkState` (org/popcraft/chunky/util/RegionStatusReader.java)
    pub mod region_status_reader_chunk_state {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum ChunkState {
            Generated,
            Incomplete,
            Unknown,
        }
        impl ChunkState {
            pub const VALUES: [ChunkState; 3] = [ChunkState::Generated, ChunkState::Incomplete, ChunkState::Unknown, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    ChunkState::Generated => "GENERATED",
                    ChunkState::Incomplete => "INCOMPLETE",
                    ChunkState::Unknown => "UNKNOWN",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "GENERATED" => ChunkState::Generated,
                    "INCOMPLETE" => ChunkState::Incomplete,
                    "UNKNOWN" => ChunkState::Unknown,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `TaskLoader` (org/popcraft/chunky/util/TaskLoader.java)
    pub mod task_loader {
        use super::*;
        // The class itself isn't translated: it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface)). Its static parts are below.
        // TaskLoader.TaskLoader(1): not translated, its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))).
        // TaskLoader.reload(0): not translated, its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))).
        // TaskLoader.loadTask(1): not translated, its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))).
        // TaskLoader.getTask(1): not translated, its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))).
        // TaskLoader.loadTasks(0): not translated, its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))).
        // TaskLoader.saveTask(1): not translated, its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))).
        // TaskLoader.saveTask(3): not translated, its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))).
        // TaskLoader.saveTasks(0): not translated, its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))).
        // TaskLoader.cancelTask(1): not translated, its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))).
        // TaskLoader.cancelTasks(0): not translated, its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))).
        // TaskLoader.writeTask(2): not translated, its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))).
    }

    /// `TaskLoader.TaskProperty` (org/popcraft/chunky/util/TaskLoader.java)
    pub mod task_loader_task_property {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // TaskLoader.TaskProperty.TaskProperty(1): not translated, its class isn't translated (its constants carry values).
        // TaskLoader.TaskProperty.key(0): not translated, its class isn't translated (its constants carry values).
    }

    /// `TaskLogger` (org/popcraft/chunky/util/TaskLogger.java)
    pub mod task_logger {
        use super::*;
        // The class itself isn't translated: it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface)). Its static parts are below.
        // TIMESTAMP_FORMAT: not translated, it uses `DateTimeFormatter` (java.time.format).
        // TaskLogger.TaskLogger(1): not translated, its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))).
        // TaskLogger.register(1): not translated, its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))).
        // TaskLogger.onStart(1): not translated, its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))).
        // TaskLogger.onComplete(1): not translated, its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))).
        // TaskLogger.appendTask(9): not translated, its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))).
        // TaskLogger.number(1): not translated, its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))).
        // TaskLogger.write(1): not translated, its class isn't translated (it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface))).
    }

    /// `TaskMetrics` (org/popcraft/chunky/util/TaskMetrics.java)
    pub mod task_metrics {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct TaskMetrics {
        }
        impl TaskMetrics {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // TaskMetrics.activeTasks(1): not translated, it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface)).
        // TaskMetrics.chunksPerSecond(1): not translated, it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface)).
        // TaskMetrics.shapes(1): not translated, it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface)).
        // TaskMetrics.patterns(1): not translated, it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface)).
        // TaskMetrics.sizes(1): not translated, it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface)).
        // TaskMetrics.speeds(1): not translated, it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface)).
        // TaskMetrics.count(2): not translated, it uses Chunky, which isn't translated (it uses Server, which isn't translated (it's an interface)).
    }

    /// `TaskScheduler` (org/popcraft/chunky/util/TaskScheduler.java)
    pub mod task_scheduler {
        use super::*;
        // The class itself isn't translated: it uses `ExecutorService` (java.util.concurrent). Its static parts are below.
        // TaskScheduler.TaskScheduler(0): not translated, its class isn't translated (it uses `ExecutorService` (java.util.concurrent)).
        // TaskScheduler.runTask(1): not translated, its class isn't translated (it uses `ExecutorService` (java.util.concurrent)).
        // TaskScheduler.cancelTasks(0): not translated, its class isn't translated (it uses `ExecutorService` (java.util.concurrent)).
    }

    /// `TranslationKey` (org/popcraft/chunky/util/TranslationKey.java)
    pub mod translation_key {
        use super::*;
        pub const BORDER_DEPENDENCY_UPDATE: &str = "border_dependency_update";
        pub const BORDER_LOAD_FAILED: &str = "border_load_failed";
        pub const BORDER_SAVE_FAILED: &str = "border_save_failed";
        pub const COMMAND_NO_PERMISSION: &str = "command_no_permission";
        pub const DISABLED: &str = "disabled";
        pub const ENABLED: &str = "enabled";
        pub const ERROR_PAUSE_WHEN_EMPTY_SECONDS: &str = "error_pause_when_empty_seconds";
        pub const FORMAT_BORDER_ADD: &str = "format_border_add";
        pub const FORMAT_BORDER_BYPASS: &str = "format_border_bypass";
        pub const FORMAT_BORDER_BYPASS_NO_TARGET: &str = "format_border_bypass_no_target";
        pub const FORMAT_BORDER_LIST: &str = "format_border_list";
        pub const FORMAT_BORDER_LIST_BORDER: &str = "format_border_list_border";
        pub const FORMAT_BORDER_LIST_NONE: &str = "format_border_list_none";
        pub const FORMAT_BORDER_LOAD: &str = "format_border_load";
        pub const FORMAT_BORDER_NO_BORDER: &str = "format_border_no_border";
        pub const FORMAT_BORDER_REMOVE: &str = "format_border_remove";
        pub const FORMAT_BORDER_WRAP: &str = "format_border_wrap";
        pub const FORMAT_CANCEL: &str = "format_cancel";
        pub const FORMAT_CANCEL_ALL: &str = "format_cancel_all";
        pub const FORMAT_CANCEL_CONFIRM: &str = "format_cancel_confirm";
        pub const FORMAT_CANCEL_NO_TASKS: &str = "format_cancel_no_tasks";
        pub const FORMAT_CENTER: &str = "format_center";
        pub const FORMAT_CONFIRM: &str = "format_confirm";
        pub const FORMAT_CONTINUE: &str = "format_continue";
        pub const FORMAT_CONTINUE_NO_TASKS: &str = "format_continue_no_tasks";
        pub const FORMAT_DEBUG_MAP: &str = "format_debug_map";
        pub const FORMAT_DEBUG_MAP_FAILED: &str = "format_debug_map_failed";
        pub const FORMAT_DEBUG_MAP_START: &str = "format_debug_map_start";
        pub const FORMAT_TRIM_CONFIRM: &str = "format_trim_confirm";
        pub const FORMAT_TRIM_CONFIRM_INSIDE: &str = "format_trim_confirm_inside";
        pub const FORMAT_TRIM_CONFIRM_INHABITED: &str = "format_trim_confirm_inhabited";
        pub const FORMAT_TRIM_CONFIRM_INHABITED_MINIMUM: &str = "format_trim_confirm_inhabited_minimum";
        pub const FORMAT_TRIM_CONFIRM_INHABITED_RANGE: &str = "format_trim_confirm_inhabited_range";
        pub const FORMAT_TRIM_CONFIRM_UNMODIFIED: &str = "format_trim_confirm_unmodified";
        pub const FORMAT_TRIM_FILTER_INVALID: &str = "format_trim_filter_invalid";
        pub const FORMAT_LIMIT: &str = "format_limit";
        pub const FORMAT_LIMIT_DISABLED: &str = "format_limit_disabled";
        pub const FORMAT_PATTERN: &str = "format_pattern";
        pub const FORMAT_PATTERN_NOT_FOUND: &str = "format_pattern_not_found";
        pub const FORMAT_PAUSE: &str = "format_pause";
        pub const FORMAT_PAUSE_NO_TASKS: &str = "format_pause_no_tasks";
        pub const FORMAT_PROGRESS_NO_TASKS: &str = "format_progress_no_tasks";
        pub const FORMAT_QUIET: &str = "format_quiet";
        pub const FORMAT_RADII: &str = "format_radii";
        pub const FORMAT_RADIUS: &str = "format_radius";
        pub const FORMAT_RELOAD: &str = "format_reload";
        pub const FORMAT_RELOAD_TASKS_RUNNING: &str = "format_reload_tasks_running";
        pub const FORMAT_SELECTION: &str = "format_selection";
        pub const FORMAT_SELECTION_CENTER: &str = "format_selection_center";
        pub const FORMAT_SELECTION_RADII: &str = "format_selection_radii";
        pub const FORMAT_SELECTION_RADIUS: &str = "format_selection_radius";
        pub const FORMAT_SELECTION_SHAPE: &str = "format_selection_shape";
        pub const FORMAT_SELECTION_WORLD: &str = "format_selection_world";
        pub const FORMAT_SHAPE: &str = "format_shape";
        pub const FORMAT_SHAPE_NOT_FOUND: &str = "format_shape_not_found";
        pub const FORMAT_SILENT: &str = "format_silent";
        pub const FORMAT_START: &str = "format_start";
        pub const FORMAT_START_CONFIRM: &str = "format_start_confirm";
        pub const FORMAT_START_DISK: &str = "format_start_disk";
        pub const FORMAT_START_LIMIT: &str = "format_start_limit";
        pub const FORMAT_STARTED_ALREADY: &str = "format_started_already";
        pub const FORMAT_WORLD: &str = "format_world";
        pub const FORMAT_WORLD_NOT_FOUND: &str = "format_world_not_found";
        pub const FORMAT_WORLDBORDER_TOO_LARGE: &str = "format_worldborder_too_large";
        pub const HELP_BORDER: &str = "help_border";
        pub const HELP_CANCEL: &str = "help_cancel";
        pub const HELP_CENTER: &str = "help_center";
        pub const HELP_CONTINUE: &str = "help_continue";
        pub const HELP_CORNERS: &str = "help_corners";
        pub const HELP_DEBUG: &str = "help_debug";
        pub const HELP_TRIM: &str = "help_trim";
        pub const HELP_MENU: &str = "help_menu";
        pub const HELP_MORE: &str = "help_more";
        pub const HELP_LIMIT: &str = "help_limit";
        pub const HELP_PATTERN: &str = "help_pattern";
        pub const HELP_PAUSE: &str = "help_pause";
        pub const HELP_PROGRESS: &str = "help_progress";
        pub const HELP_QUIET: &str = "help_quiet";
        pub const HELP_RADIUS: &str = "help_radius";
        pub const HELP_RELOAD: &str = "help_reload";
        pub const HELP_SELECTION: &str = "help_selection";
        pub const HELP_SHAPE: &str = "help_shape";
        pub const HELP_SILENT: &str = "help_silent";
        pub const HELP_SPAWN: &str = "help_spawn";
        pub const HELP_START: &str = "help_start";
        pub const HELP_WORLD: &str = "help_world";
        pub const HELP_WORLDBORDER: &str = "help_worldborder";
        pub const PATTERN_CONCENTRIC: &str = "pattern_concentric";
        pub const PATTERN_LOOP: &str = "pattern_loop";
        pub const PATTERN_STRIP: &str = "pattern_strip";
        pub const PATTERN_SPIRAL: &str = "pattern_spiral";
        pub const PATTERN_CSV: &str = "pattern_csv";
        pub const PATTERN_REGION: &str = "pattern_region";
        pub const PATTERN_RING: &str = "pattern_ring";
        pub const SHAPE_CIRCLE: &str = "shape_circle";
        pub const SHAPE_DIAMOND: &str = "shape_diamond";
        pub const SHAPE_ELLIPSE: &str = "shape_ellipse";
        pub const SHAPE_HEXAGON: &str = "shape_hexagon";
        pub const SHAPE_PENTAGON: &str = "shape_pentagon";
        pub const SHAPE_RECTANGLE: &str = "shape_rectangle";
        pub const SHAPE_SQUARE: &str = "shape_square";
        pub const SHAPE_STAR: &str = "shape_star";
        pub const SHAPE_TRIANGLE: &str = "shape_triangle";
        pub const TASK_TRIM: &str = "task_trim";
        pub const TASK_TRIM_UNSUPPORTED_COMPRESSION: &str = "task_trim_unsupported_compression";
        pub const TASK_TRIM_UPDATE: &str = "task_trim_update";
        pub const TASK_DONE: &str = "task_done";
        pub const TASK_STOPPED: &str = "task_stopped";
        pub const TASK_UPDATE: &str = "task_update";
        pub const TIME_DAY: &str = "time_day";
        pub const TIME_DAYS: &str = "time_days";
        pub const TIME_HOUR: &str = "time_hour";
        pub const TIME_HOURS: &str = "time_hours";
        pub const TIME_MINUTE: &str = "time_minute";
        pub const TIME_MINUTES: &str = "time_minutes";
        pub const TIME_SECOND: &str = "time_second";
        pub const TIME_SECONDS: &str = "time_seconds";
        pub const WRAP_BOTH: &str = "wrap_both";
        pub const WRAP_DEFAULT: &str = "wrap_default";
        pub const WRAP_EARTH: &str = "wrap_earth";
        pub const WRAP_NONE: &str = "wrap_none";
        pub const WRAP_RADIAL: &str = "wrap_radial";
        pub const WRAP_X: &str = "wrap_x";
        pub const WRAP_Z: &str = "wrap_z";
        #[derive(Clone, Debug)]
        pub struct TranslationKey {
        }
        impl TranslationKey {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `Translator` (org/popcraft/chunky/util/Translator.java)
    pub mod translator {
        use super::*;
        // fallbackTranslations: not translated, it calls Translator.load, which isn't translated.
        pub static TRANSLATIONS: LazyLock<JCell<JMap<String, String>>> = LazyLock::new(|| JCell::new(JMap::new()));
        #[derive(Clone, Debug)]
        pub struct Translator {
        }
        impl Translator {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // Translator.setLanguage(1): not translated, it calls Translator.load, which isn't translated.
        // Translator.isValidLanguage(1): not translated, unknown static Translator.class.
        // Translator.translateKey(3): not translated, it uses `Object`.
        // Translator.translate(2): not translated, it uses `Object`.
        // Translator.getMessage(1): not translated, it uses Translator.fallbackTranslations, which isn't translated (it calls Translator.load, which isn't translated).
        // Translator.addCustomTranslation(2): not translated, it uses Translator.fallbackTranslations, which isn't translated (it calls Translator.load, which isn't translated).
        // Translator.load(1): not translated, its body couldn't be read: an anonymous class.
    }

    /// `Version` (org/popcraft/chunky/util/Version.java)
    pub mod version {
        use super::*;
        // The class itself isn't translated: it implements `Comparable`. Its static parts are below.
        // INVALID: not translated, it creates a Version, which isn't translated (it implements `Comparable`).
        // Version.Version(3): not translated, its class isn't translated (it implements `Comparable`).
        // Version.Version(1): not translated, its class isn't translated (it implements `Comparable`).
        // Version.Version(2): not translated, its class isn't translated (it implements `Comparable`).
        // Version.isEqualTo(1): not translated, its class isn't translated (it implements `Comparable`).
        // Version.isHigherThan(1): not translated, its class isn't translated (it implements `Comparable`).
        // Version.isHigherThanOrEqualTo(1): not translated, its class isn't translated (it implements `Comparable`).
        // Version.isLowerThan(1): not translated, its class isn't translated (it implements `Comparable`).
        // Version.isLowerThanOrEqualTo(1): not translated, its class isn't translated (it implements `Comparable`).
        // Version.isValid(0): not translated, its class isn't translated (it implements `Comparable`).
        // Version.compareTo(1): not translated, its class isn't translated (it implements `Comparable`).
        // Version.equals(1): not translated, its class isn't translated (it implements `Comparable`).
        // Version.hashCode(0): not translated, its class isn't translated (it implements `Comparable`).
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
        assert_eq!(ID, "chunky");
        assert!(!NAME.is_empty());
        assert!(KEYS.iter().all(|[n, _, d]| !n.is_empty() && d.starts_with("key.")));
    }
}
