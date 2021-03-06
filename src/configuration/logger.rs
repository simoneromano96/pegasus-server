use lazy_static::lazy_static;
use slog::{info, Logger};
use sloggers::{Build, file::FileLoggerBuilder, terminal::{Destination, TerminalLoggerBuilder}, types::Severity};

lazy_static! {
	pub static ref LOGGER: Logger = init_file_logger();
}

fn init_terminal_logger() -> Logger {
	let mut builder = TerminalLoggerBuilder::new();
	builder.level(Severity::Debug);
	builder.destination(Destination::Stdout);

	let l = builder.build().expect("Could not initialize logger");

	info!(l, "Initialized logger");

	l
}

fn init_file_logger() -> Logger {
	let mut builder = FileLoggerBuilder::new("./logs/log");
	builder.level(Severity::Debug);
	builder.rotate_compress(true);

	let logger = builder.build().unwrap();
	info!(logger, "Hello World!");

	logger
}
