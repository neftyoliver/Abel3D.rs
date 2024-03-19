

pub mod render {
    use std::sync::Arc;
    use crate::resource::resource::Resource;

    pub enum AbelResult {
        SUCCESS
    }

    pub enum AbelDeviceRating {
        UNRATED,

        BronzeD,
        BronzeC,
        BronzeB,
        BronzeA,
        BronzeS,

        IronD,
        IronC,
        IronB,
        IronA,
        IronS,

        SilverD,
        SilverC,
        SilverB,
        SilverA,
        SilverS,

        GoldD,
        GoldC,
        GoldB,
        GoldA,
        GoldS,

        PlatinumD,
        PlatinumC,
        PlatinumB,
        PlatinumA,
        PlatinumS,

        DiamondD,
        DiamondC,
        DiamondB,
        DiamondA,
        DiamondS,

        TitaniumD,
        TitaniumC,
        TitaniumB,
        TitaniumA,
        TitaniumS,

        MasterB,
        MasterA,
        MasterS,

        MightyB,
        MightyA,
        MightyS
    }

    pub struct DeviceInfo {
        name: String,
        rating: AbelDeviceRating
    }

    pub trait Renderer {
        fn create(name: &str) -> impl Renderer;

        fn upload(resource: dyn Resource) -> AbelResult;

        fn get_device_info(idx: u64) -> Arc<DeviceInfo>;
    }
}