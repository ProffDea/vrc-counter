#[allow(warnings, unused)]
pub mod prisma;

use prisma::PrismaClient;
use regex::Regex;
use std::sync::Arc;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

pub const AVATAR_PARAMETERS: &str = "/avatar/parameters/";

#[repr(u8)]
#[derive(Debug, Clone)]
pub enum Mask {
	UpPosed(Regex) = 0,
	DownPosed(Regex) = 1,
	UpGrabbed(Regex) = 2,
	DownGrabbed(Regex) = 3,
}

impl Mask {
	pub fn discriminant(&self) -> u8 {
		// SAFETY: Because `Self` is marked `repr(u8)`, its layout is a `repr(C)` `union`
		// between `repr(C)` structs, each of which has the `u8` discriminant as its first
		// field, so we can read the discriminant without offsetting the pointer.
		unsafe { *<*const _>::from(self).cast::<u8>() }
	}
}

#[derive(Debug, Clone)]
pub struct Config {
	pub avatar_params: Vec<Mask>,
}

impl Config {
	pub fn new() -> Result<Self> {
		let avatar_params = vec![
			Mask::UpPosed(Regex::new("/avatar/parameters/.*?Mask_up_IsPosed")?),
			Mask::DownPosed(Regex::new("/avatar/parameters/.*?Mask_down_IsPosed")?),
			Mask::UpGrabbed(Regex::new("/avatar/parameters/.*?Mask_up_IsGrabbed")?),
			Mask::DownGrabbed(Regex::new("/avatar/parameters/.*?Mask_down_IsGrabbed")?),
		];

		Ok(Config { avatar_params })
	}
}

#[derive(Debug)]
pub struct State {
	pub config: Config,
	pub db: Arc<PrismaClient>,
}

impl State {
	pub async fn new() -> Self {
		{
			#![cfg(not(debug_assertions))]
			if std::env::var("VRC_COUNTER_DATABASE").is_err() {
				std::env::set_var("VRC_COUNTER_DATABASE", "file:./vrc-counter.db");
			}
		}

		let config = Config::new().expect("error while getting config");

		let db = Arc::new(
			PrismaClient::_builder()
				.build()
				.await
				.expect("error while building the prisma client"),
		);

		db._migrate_deploy()
			.await
			.expect("error while deploying db migration");

		Self { config, db }
	}
}
