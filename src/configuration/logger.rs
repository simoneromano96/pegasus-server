use lazy_static::lazy_static;
use slog::{o, trace, Drain, Duplicate, Logger};
use sloggers::{
	file::FileLoggerBuilder,
	terminal::{Destination, TerminalLoggerBuilder},
	types::Severity,
	Build,
};

use super::APP_CONFIG;

lazy_static! {
	pub static ref LOGGER: Logger = init_loggers();
}

fn init_loggers() -> Logger {
	let terminal_logger = init_terminal_logger();
	// trace!(terminal_logger, "Initialized terminal logger");

	let file_logger = init_file_logger();
	// trace!(file_logger, "Initialized file logger");

	let combined = Logger::root(Duplicate::new(terminal_logger, file_logger).fuse(), o!());
	// trace!(combined, "Initialized combined logger");

	combined
}

fn init_terminal_logger() -> Logger {
	let mut builder = TerminalLoggerBuilder::new();
	builder.level(APP_CONFIG.logger.severity);
	builder.format(APP_CONFIG.logger.format);
	builder.destination(Destination::Stdout);

	builder.build().expect("Could not initialize terminal logger")
}

fn init_file_logger() -> Logger {
	let mut builder = FileLoggerBuilder::new(&APP_CONFIG.logger.path);
	builder.level(Severity::Error);
	builder.rotate_compress(true);

	builder.build().expect("Could not initialize file logger")
}
