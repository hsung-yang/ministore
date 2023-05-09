use super::{BlockDevice, BlockDeviceType};
use crate::block_device_common::data_type::{DataBlock, UNMAP_BLOCK};
use crate::block_device_common::device_info::DeviceInfo;

use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
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
    pub fn new(name: String, size: u64) -> Result<SimpleFakeDevice, String> {
        let device_info = DeviceInfo::new(BlockDeviceType::SimpleFakeDevice, name.clone(), size);
        if device_info.is_err() {
            return Err("Failed to create device info".to_string());
        }
        let data = Data::new(size as usize);
        Ok(SimpleFakeDevice {
            device_info: device_info.unwrap(),
            data: data,
        })
    }
}

impl BlockDevice for SimpleFakeDevice {
    fn info(&self) -> &DeviceInfo {
        &self.device_info
    }

    fn write(&mut self, lba: u64, num_blocks: u64, buffer: Vec<DataBlock>) -> Result<(), String> {
        let dev_size = self.device_info.device_size();
        if dev_size < lba || dev_size < lba + num_blocks {
            return Err("Invalid address".to_string());
        }

        if num_blocks == 0 {
            return Err("Nothing to write".to_string());
        }

        for i in 0..num_blocks as usize {
            self.data.0[i + lba as usize] = buffer[i];
        }

        Ok(())
    }

    fn read(&mut self, lba: u64, num_blocks: u64) -> Result<Vec<DataBlock>, String> {
        let dev_size = self.device_info.device_size();
        if dev_size < lba || dev_size < lba + num_blocks {
            return Err("Invalid address".to_string());
        }

        if num_blocks == 0 {
            return Err("Nothing to write".to_string());
        }

        let mut data = Vec::new();

        for i in 0..num_blocks as usize {
            data.push(self.data.0[i + lba as usize].clone());
        }
        Ok(data)
    }

    fn load(&mut self) -> Result<(), String> {
        let file = OpenOptions::new()
            .read(true)
            .open(self.device_info.name().to_string());

        if file.is_err() {
            return Err("File doesn't exist".to_string());
        }

        let file = file.unwrap();
        let result = bincode::deserialize_from(file);
        if result.is_err() {
            return Err("Failed to load from file".to_string());
        }
        self.data = result.unwrap();

        Ok(())
    }

    fn flush(&mut self) -> Result<(), String> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(self.device_info.name().to_string());

        if file.is_err() {
            return Err("Cannot open or create new file".to_string());
        }

        let result = bincode::serialize_into(file.unwrap(), &self.data);
        if result.is_err() {
            return Err("Cannot flush into file".to_string());
        }

        Ok(())
    }
}
