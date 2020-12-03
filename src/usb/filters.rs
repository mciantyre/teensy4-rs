//! Logging filters

/// Filter log messages by module name (`&'static str`) to a log level
///
/// - if the level is `None`, log at all levels from the module (subject to the max log level)
/// - if the level is not `None`, that will be the base log level for the module
///
/// # Example
///
/// ```
/// use teensy4_bsp as bsp;
/// use bsp::usb::Filter;
/// use log::LevelFilter;
///
/// static LEVELS: &'static [Filter] = &[
///     // Writes messages from the 'i2c' module subject to the max log level
///     ("i2c", None),
///     // Writes only Error- and Warn-level messages from the 'spi' module
///     ("spi", Some(LevelFilter::Warn)),
/// ];
/// ```
pub type Filter = (&'static str, Option<::log::LevelFilter>);

/// Filters for log channels
pub struct Filters(&'static [Filter]);

impl Filters {
    /// Returns an empty filters collection
    ///
    /// This `Filters` lets all log messages pass through.
    pub const fn empty() -> Self {
        Filters::new(&[])
    }

    /// Create a `Filters` collection
    pub const fn new(filters: &'static [Filter]) -> Self {
        Filters(filters)
    }
}

impl Filters {
    /// Returns `true` if, based on this metadata, logging should be enabled
    ///
    /// `is_enabled()` considers the permitted modules and log levels for those modules.
    pub fn is_enabled(&self, metadata: &::log::Metadata) -> bool {
        if self.0.is_empty() {
            true
        } else if let Some(idx) = self
            .0
            .iter()
            .position(|&(target, _)| target == metadata.target())
        {
            let (_, lvl) = self.0[idx];
            lvl.is_none() || lvl.filter(|lvl| metadata.level() <= *lvl).is_some()
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Filters;
    use log::{Level, LevelFilter};

    fn metadata(level: Level, target: &'static str) -> ::log::Metadata<'static> {
        ::log::Metadata::builder()
            .level(level)
            .target(target)
            .build()
    }

    const ALL_LEVELS: [Level; 5] = [
        Level::Error,
        Level::Warn,
        Level::Info,
        Level::Debug,
        Level::Trace,
    ];

    #[test]
    fn empty_always_enabled() {
        let filters = Filters(&[]);
        ALL_LEVELS.iter().for_each(|level| {
            assert!(filters.is_enabled(&metadata(*level, "foobar")));
        });
    }

    #[test]
    fn no_level_always_true() {
        let filters = Filters(&[("barbaz", None), ("foobar", None)]);
        ALL_LEVELS
            .iter()
            .for_each(|level| assert!(filters.is_enabled(&metadata(*level, "foobar"))));
    }

    #[test]
    fn module_level() {
        let filters = Filters(&[
            ("barbaz", Some(LevelFilter::Error)),
            ("foobar", Some(LevelFilter::Info)),
        ]);
        ALL_LEVELS
            .iter()
            .filter(|level| **level <= LevelFilter::Info)
            .for_each(|level| {
                assert!(filters.is_enabled(&metadata(*level, "foobar")), "{}", level)
            });
        ALL_LEVELS
            .iter()
            .filter(|level| **level > LevelFilter::Info)
            .for_each(|level| {
                assert!(
                    !filters.is_enabled(&metadata(*level, "foobar")),
                    "{}",
                    level
                )
            });
        ALL_LEVELS
            .iter()
            .filter(|level| **level != Level::Error)
            .for_each(|level| {
                assert!(
                    !filters.is_enabled(&metadata(*level, "barbaz")),
                    "{}",
                    level
                )
            });
        assert!(filters.is_enabled(&metadata(Level::Error, "barbaz")));
    }
}
