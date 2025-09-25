//! A `tracing` subscriber for Kotlin.
//!

use core::fmt::{self, Write};

use tracing::{
    dispatcher::SetGlobalDefaultError,
    field::{Field, Visit},
    Subscriber,
};
use tracing_subscriber::{layer::*, registry::*};

use crate::env::{android_log_write, LOG_TAG};

pub struct KotlinLayerConfigBuilder {
    max_level: tracing::Level,
}

impl KotlinLayerConfigBuilder {
    pub fn new() -> KotlinLayerConfigBuilder {
        KotlinLayerConfigBuilder::default()
    }

    /// Set the maximal level on which events should be displayed
    pub fn set_max_level(&mut self, max_level: tracing::Level) -> &mut KotlinLayerConfigBuilder {
        self.max_level = max_level;
        self
    }

    /// Build the ConfigLayerConfig
    pub fn build(&self) -> ConfigLayerConfig {
        ConfigLayerConfig {
            max_level: self.max_level,
        }
    }
}

impl Default for KotlinLayerConfigBuilder {
    fn default() -> KotlinLayerConfigBuilder {
        KotlinLayerConfigBuilder {
            max_level: tracing::Level::TRACE,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ConfigLayerConfig {
    max_level: tracing::Level,
}

impl core::default::Default for ConfigLayerConfig {
    fn default() -> Self {
        ConfigLayerConfig {
            max_level: tracing::Level::TRACE,
        }
    }
}

/// Implements [tracing_subscriber::layer::Layer] which uses [wasm_bindgen] for marking and measuring with `window.performance`
pub struct KotlinLayer {
    config: ConfigLayerConfig,
}

impl KotlinLayer {
    pub fn new(config: ConfigLayerConfig) -> Self {
        KotlinLayer { config }
    }
}

impl core::default::Default for KotlinLayer {
    fn default() -> Self {
        KotlinLayer::new(ConfigLayerConfig::default())
    }
}

#[inline]
fn thread_display_suffix() -> &'static str {
    ""
}

impl<S: Subscriber + for<'a> LookupSpan<'a>> Layer<S> for KotlinLayer {
    fn enabled(&self, metadata: &tracing::Metadata<'_>, _: Context<'_, S>) -> bool {
        let level = metadata.level();
        level <= &self.config.max_level
    }

    fn on_new_span(
        &self,
        attrs: &tracing::span::Attributes<'_>,
        id: &tracing::Id,
        ctx: Context<'_, S>,
    ) {
        let mut new_debug_record = StringRecorder::new();
        attrs.record(&mut new_debug_record);

        if let Some(span_ref) = ctx.span(id) {
            span_ref
                .extensions_mut()
                .insert::<StringRecorder>(new_debug_record);
        }
    }

    /// doc: Notifies this layer that a span with the given Id recorded the given values.
    fn on_record(&self, id: &tracing::Id, values: &tracing::span::Record<'_>, ctx: Context<'_, S>) {
        if let Some(span_ref) = ctx.span(id) {
            if let Some(debug_record) = span_ref.extensions_mut().get_mut::<StringRecorder>() {
                values.record(debug_record);
            }
        }
    }

    /// doc: Notifies this layer that an event has occurred.
    fn on_event(&self, event: &tracing::Event<'_>, _ctx: Context<'_, S>) {
        let mut recorder = StringRecorder::new();
        event.record(&mut recorder);
        let meta = event.metadata();
        let level = meta.level();
        let origin = meta
            .file()
            .and_then(|file| meta.line().map(|ln| format!("{file}:{ln}")))
            .unwrap_or_default();

        match *level {
            tracing::Level::TRACE => {
                let string = format!("TRACE {origin}{}{recorder}", thread_display_suffix());

                let _ret =
                    android_log_write(crate::env::AndroidLogPriority::Verbose, LOG_TAG, &string)
                        .inspect_err(|err| {
                            eprintln!(
                                "Should write log line TRACE to AndroidLogPriority::Verbose: {err}"
                            )
                        });
            }
            tracing::Level::DEBUG => {
                let string = format!("DEBUG {origin}{}{recorder}", thread_display_suffix());

                let _ret =
                    android_log_write(crate::env::AndroidLogPriority::Debug, LOG_TAG, &string)
                        .inspect_err(|err| {
                            eprintln!(
                                "Should write log line DEBUG to AndroidLogPriority::Debug: {err}"
                            )
                        });
            }
            tracing::Level::INFO => {
                let string = format!("INFO {origin}{}{recorder}", thread_display_suffix());

                let _ret =
                    android_log_write(crate::env::AndroidLogPriority::Info, LOG_TAG, &string)
                        .inspect_err(|err| {
                            eprintln!(
                                "Should write log line INFO to AndroidLogPriority::Info: {err}"
                            )
                        });
            }
            tracing::Level::WARN => {
                let string = format!("WARN {origin}{}{recorder}", thread_display_suffix());

                let _ret =
                    android_log_write(crate::env::AndroidLogPriority::Warn, LOG_TAG, &string)
                        .inspect_err(|err| {
                            eprintln!(
                                "Should write log line WARN to AndroidLogPriority::Warn: {err}"
                            )
                        });
            }
            tracing::Level::ERROR => {
                let string = format!("ERROR {origin}{}{recorder}", thread_display_suffix());

                let _ret =
                    android_log_write(crate::env::AndroidLogPriority::Error, LOG_TAG, &string)
                        .inspect_err(|err| {
                            eprintln!(
                                "Should write log line ERROR to AndroidLogPriority::Error: {err}"
                            )
                        });
            }
        };
    }
}

/// Set the global default with [tracing::subscriber::set_global_default]
pub fn set_as_global_default() {
    tracing::subscriber::set_global_default(
        Registry::default().with(KotlinLayer::new(ConfigLayerConfig::default())),
    )
    .expect("default global");
}

/// Set the global default with [tracing::subscriber::set_global_default]
pub fn try_set_as_global_default() -> Result<(), SetGlobalDefaultError> {
    tracing::subscriber::set_global_default(
        Registry::default().with(KotlinLayer::new(ConfigLayerConfig::default())),
    )
}

/// Set the global default with [tracing::subscriber::set_global_default]
pub fn set_as_global_default_with_config(config: ConfigLayerConfig) {
    tracing::subscriber::set_global_default(Registry::default().with(KotlinLayer::new(config)))
        .expect("default global");
}

struct StringRecorder {
    display: String,
    is_following_args: bool,
}
impl StringRecorder {
    fn new() -> Self {
        StringRecorder {
            display: String::new(),
            is_following_args: false,
        }
    }
}

impl Visit for StringRecorder {
    fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
        if field.name() == "message" {
            if !self.display.is_empty() {
                self.display = format!("{value:?}\n{}", self.display)
            } else {
                self.display = format!("{value:?}")
            }
        } else {
            if self.is_following_args {
                // following args
                writeln!(self.display).unwrap();
            } else {
                // first arg
                write!(self.display, " ").unwrap();
                self.is_following_args = true;
            }
            write!(self.display, "{} = {value:?};", field.name()).unwrap();
        }
    }
}

impl core::fmt::Display for StringRecorder {
    fn fmt(&self, mut f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if !self.display.is_empty() {
            write!(&mut f, " {}", self.display)
        } else {
            Ok(())
        }
    }
}

impl core::default::Default for StringRecorder {
    fn default() -> Self {
        StringRecorder::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_default_built_config() {
        let builder = KotlinLayerConfigBuilder::new();

        let config = builder.build();

        assert_eq!(
            config,
            ConfigLayerConfig {
                max_level: tracing::Level::TRACE,
            }
        )
    }

    #[test]
    fn test_default_config_log_level() {
        let builder = KotlinLayerConfigBuilder::new();

        let config = builder.build();

        assert_eq!(config.max_level, tracing::Level::TRACE);
    }

    #[test]
    fn test_set_config_log_level_warn() {
        let mut builder = KotlinLayerConfigBuilder::new();
        builder.set_max_level(tracing::Level::WARN);

        let config = builder.build();

        assert_eq!(config.max_level, tracing::Level::WARN);
    }
}
