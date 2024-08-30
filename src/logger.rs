use futures::channel::mpsc::Sender;
use std::fmt::Debug;
use tracing::{
	field::{Field, Visit},
	Event, Level, Metadata, Subscriber,
};
use tracing_subscriber::{layer, Layer};

pub struct Logger {
	pub max_level: Level,
	pub tx: Sender<crate::Event>,
}

impl Logger {
	pub fn new(tx: Sender<crate::Event>) -> Self {
		Self {
			tx,
			max_level: Level::TRACE,
		}
	}

	pub fn with_max_level(self, level: Level) -> Self {
		Self {
			tx: self.tx,
			max_level: level,
		}
	}
}

impl<S: Subscriber> Layer<S> for Logger {
	fn enabled(&self, metadata: &Metadata<'_>, _ctx: layer::Context<'_, S>) -> bool {
		if metadata.level() <= &Level::INFO {
			dbg!(&metadata.level());
		}
		metadata.level() <= &self.max_level
	}

	fn on_event(&self, event: &Event<'_>, _ctx: layer::Context<'_, S>) {
		let tx = self.tx.clone();
		let mut visitor = LoggerVisitor { tx };
		event.record(&mut visitor);
	}
}

pub struct LoggerVisitor {
	pub tx: Sender<crate::Event>,
}

impl Visit for LoggerVisitor {
	fn record_debug(&mut self, field: &Field, value: &dyn Debug) {
		let mut tx = self.tx.clone();
		if let Err(e) = tx.try_send(crate::Event::Log(format!(
			"field={} value={:?}",
			field.name(),
			value
		))) {
			eprintln!("{}", e);
		}
	}

	fn record_f64(&mut self, field: &Field, value: f64) {
		let mut tx = self.tx.clone();
		if let Err(e) = tx.try_send(crate::Event::Log(format!(
			"field={} value={}",
			field.name(),
			value
		))) {
			eprintln!("{}", e);
		}
	}

	fn record_i64(&mut self, field: &Field, value: i64) {
		let mut tx = self.tx.clone();
		if let Err(e) = tx.try_send(crate::Event::Log(format!(
			"field={} value={}",
			field.name(),
			value
		))) {
			eprintln!("{}", e);
		}
	}

	fn record_u64(&mut self, field: &Field, value: u64) {
		let mut tx = self.tx.clone();
		if let Err(e) = tx.try_send(crate::Event::Log(format!(
			"field={} value={}",
			field.name(),
			value
		))) {
			eprintln!("{}", e);
		}
	}

	fn record_i128(&mut self, field: &Field, value: i128) {
		let mut tx = self.tx.clone();
		if let Err(e) = tx.try_send(crate::Event::Log(format!(
			"field={} value={}",
			field.name(),
			value
		))) {
			eprintln!("{}", e);
		}
	}

	fn record_u128(&mut self, field: &Field, value: u128) {
		let mut tx = self.tx.clone();
		if let Err(e) = tx.try_send(crate::Event::Log(format!(
			"field={} value={}",
			field.name(),
			value
		))) {
			eprintln!("{}", e);
		}
	}

	fn record_bool(&mut self, field: &Field, value: bool) {
		let mut tx = self.tx.clone();
		if let Err(e) = tx.try_send(crate::Event::Log(format!(
			"field={} value={}",
			field.name(),
			value
		))) {
			eprintln!("{}", e);
		}
	}

	fn record_str(&mut self, field: &Field, value: &str) {
		let mut tx = self.tx.clone();
		if let Err(e) = tx.try_send(crate::Event::Log(format!(
			"field={} value={}",
			field.name(),
			value
		))) {
			eprintln!("{}", e);
		}
	}

	fn record_error(&mut self, field: &Field, value: &(dyn std::error::Error + 'static)) {
		let mut tx = self.tx.clone();
		if let Err(e) = tx.try_send(crate::Event::Log(format!(
			"field={} value={}",
			field.name(),
			value
		))) {
			eprintln!("{}", e);
		}
	}
}
