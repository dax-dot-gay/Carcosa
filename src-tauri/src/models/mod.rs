use native_db::Models;
use once_cell::sync::Lazy;
use paste::paste;

macro_rules! models {
    (
        versions = $($versions:ident),+;
        selected_version = $selected_version:ident;
        models = $($models:ident),+;
    ) => {
        models!(_int:versions $($versions),+);

        models!(_int:select $selected_version, $($models),+);

        pub static MODELS: Lazy<Models> = Lazy::new(|| {
            let mut models = Models::new();
            models!(_int:define_version models; $($versions),+; $($models),+);
            models
        });

        #[allow(dead_code)]
        pub mod keys {
            use super::*;
            models!(_int:keys $selected_version, $($models),+);
        }
    };

    (_int: versions $version:ident, $($versions:ident),+) => {
        pub mod $version;
        models!(_int:versions $($versions),+);
    };

    (_int: versions $version:ident) => {
        pub mod $version;
    };

    (_int: keys $select:ident, $model:ident, $($models:ident),+) => {
        paste!{pub(crate) type [<$model Key>] = $select::[<$model Key>];}
        models!(_int:keys $select, $($models),+);
    };

    (_int: keys $select:ident, $model:ident) => {
        paste!{pub(crate) type [<$model Key>] = $select::[<$model Key>];}
    };

    (_int: select $select:ident, $model:ident, $($models:ident),+) => {
        pub type $model = $select::$model;

        models!(_int:select $select, $($models),+);
    };

    (_int: select $select:ident, $model:ident) => {
        pub type $model = $select::$model;
    };

    (
        _int: define_version $target:ident;
        $version:ident,
        $($versions:ident),+;
        $($models:ident),+
    ) => {
        models!(_int:define_version $target; $version; $($models),+);
        models!(_int:define_version $target; $($versions),+; $($models),+);
    };

    (_int: define_version $target:ident; $version:ident; $model:ident, $($models:ident),+) => {
        $target.define::<$version::$model>().unwrap();
        models!(_int:define_version $target; $version; $($models),+);
    };

    (_int: define_version $target:ident; $version:ident; $model:ident) => {
        $target.define::<$version::$model>().unwrap();
    };
}

models! {
    versions = v1;
    selected_version = v1;
    models = ProjectConfiguration, Template, Node;
}

pub struct MsgPack;

impl<T: serde::Serialize> native_model::Encode<T> for MsgPack {
    type Error = rmp_serde::encode::Error;
    /// Serializes a type into bytes using the `rmp-serde` `1.3` crate.
    fn encode(obj: &T) -> Result<Vec<u8>, Self::Error> {
        rmp_serde::encode::to_vec_named(obj)
    }
}

impl<T: for<'de> serde::Deserialize<'de>> native_model::Decode<T> for MsgPack {
    type Error = rmp_serde::decode::Error;
    /// Deserializes a type from bytes using the `rmp-serde` `1.3` crate.
    fn decode(data: Vec<u8>) -> Result<T, Self::Error> {
        rmp_serde::decode::from_slice(&data)
    }
}
