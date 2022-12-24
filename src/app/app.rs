use std::thread;
use std::time::Duration;

use anyhow::{Context, Result};

use crate::app::Config;
use crate::output::OutputCollection;
use crate::sensor::SensorCollection;

pub struct App {
    sensors: SensorCollection,
    outputs: OutputCollection,
}

impl App {
    pub fn from_config(config: Config) -> App {
        App {
            sensors: config.sensors,
            outputs: config.outputs,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        self.outputs.enable_all(true)?;
        loop {
            self.sensors
                .update_all()
                .context("Error while reading sensors")?;
            self.outputs
                .update_all()
                .context("Error while writing outputs")?;
            thread::sleep(Duration::from_secs(1));
        }
    }
}
