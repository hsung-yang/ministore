use crate::block_device_common::data_type::{DataBlock, UNMAP_BLOCK};
use crate::block_device_common::device_info::DeviceInfo;
use super::{BlockDeviceType, BlockDevice};

use serde::{Deserialize, Serialize};
use std::{fs::OpenOptions, path::Path};

#[derive(Serialize, Deserialize, Clone)]
pub struct Data(pub Vec<DataBlock>);
impl Data {
    pub fn new(size: usize) -> Self {
        let mut items = Vec::new();
        for _ in 0..size {
            items.push(UNMAP_BLOCK);
        }

        Self(items)
    }
}

pub struct SimpleFakeDevice {
    device_info: DeviceInfo,
    data: Data,
}
impl std::fmt::Debug for SimpleFakeDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SimpleFakeDevice")
            .field("device_info", &self.device_info)
            .finish()
    }
}

impl SimpleFakeDevice {
    pub fn new(name: String, size: u64) -> Result<Self, String> {
        let device_info = DeviceInfo::new(BlockDeviceType::SimpleFakeDevice, name, size)?;
        let num_blocks = device_info.num_blocks();

        let mut device = SimpleFakeDevice {
            device_info,
            data: Data::new(num_blocks as usize),
        };

        if Path::new(device.device_info.name()).exists() == false {
            device.flush()?;
        }

        Ok(device)
    }

    fn is_valid_range(&self, lba: u64, num_blocks: u64) -> bool {
        if num_blocks == 0 || lba + num_blocks > self.device_info.num_blocks() {
            false
        } else {
            true
        }
    }
}

impl BlockDevice for SimpleFakeDevice {
    fn info(&self) -> &DeviceInfo {
        todo!()
    }

    fn write(&mut self, lba: u64, num_blocks: u64, buffer: Vec<DataBlock>) -> Result<(), String> {
        todo!()
    }

    fn read(&mut self, lba: u64, num_blocks: u64) -> Result<Vec<DataBlock>, String> {
        todo!()
    }

    fn load(&mut self) -> Result<(), String> {
        let filename = self.device_info.name().clone();
        let path = Path::new(&filename);

        if !path.exists() {
            return Err("No files to load".to_string());
        }

        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&path)
            .map_err(|e| e.to_string())?;

        let loaded_data = bincode::deserialize_from(&mut file).map_err(|e| e.to_string())?;
        self.data = loaded_data;

        Ok(())
    }

    fn flush(&mut self) -> Result<(), String> {
        todo!()
    }
}
