extern crate i2cdev;
use i2cdev::linux::LinuxI2CDevice;

extern crate lidar_lite_v3;
use lidar_lite_v3::LidarLiteV3;

fn main() {
    let i2c_device_name = "/dev/i2c-1";
    let i2c_device = LinuxI2CDevice::new(&i2c_device_name, lidar_lite_v3::LIDAR_LITE_DEFAULT_I2C_ADDRESS).unwrap();
    let mut lidar = LidarLiteV3::new(i2c_device).unwrap();

    let id = lidar.read_device_id().unwrap();
    println!("device id: {}", id);
    let n = 100; //The datasheet recommends doing receiver bias correction every 100th time

    loop {
        for i in 0..n {
            let receiver_bias_correction = i == 0;
            let distance = lidar.read_distance(receiver_bias_correction).unwrap();
            println!("distance: {}cm", distance);
        }
    }
}



