//! Battery simulation

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Battery {
    capacity: u64,
    current_energy: u64,
    max_power: u64,
    min_soc: u8,
    max_soc: u8,
    degradation: f64,
    temperature: f32,
    cycles: u32,
}

impl Battery {
    pub fn new(capacity: u64, initial_soc: u8) -> Self {
        let initial_energy = (capacity as f64 * initial_soc as f64 / 100.0) as u64;
        Self {
            capacity,
            current_energy: initial_energy,
            max_power: 22000,
            min_soc: 5,
            max_soc: 100,
            degradation: 0.0,
            temperature: 25.0,
            cycles: 0,
        }
    }

    pub fn charge(&mut self, energy_wh: u64) -> Result<(), String> {
        if self.is_full() {
            return Err("Battery is full".to_string());
        }

        let new_energy = self.current_energy + energy_wh;
        self.current_energy = new_energy.min(self.capacity);

        if self.is_full() {
            self.cycles += 1;
        }

        self.temperature += 0.5;
        if self.temperature > 45.0 {
            self.temperature = 45.0;
        }

        Ok(())
    }

    pub fn discharge(&mut self, energy_wh: u64) -> Result<(), String> {
        if self.is_empty() {
            return Err("Battery is empty".to_string());
        }

        let new_energy = self.current_energy.saturating_sub(energy_wh);
        self.current_energy = new_energy;

        self.temperature -= 0.2;
        if self.temperature < 10.0 {
            self.temperature = 10.0;
        }

        Ok(())
    }

    pub fn soc(&self) -> u8 {
        ((self.current_energy as f64 / self.capacity as f64) * 100.0) as u8
    }

    pub fn is_full(&self) -> bool {
        self.soc() >= self.max_soc
    }

    pub fn is_empty(&self) -> bool {
        self.soc() <= self.min_soc
    }

    pub fn remaining_capacity(&self) -> u64 {
        self.capacity - self.current_energy
    }

    pub fn get_temperature(&self) -> f32 {
        self.temperature
    }
}