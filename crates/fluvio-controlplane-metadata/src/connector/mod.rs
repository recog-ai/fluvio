mod spec;
mod status;

pub use spec::*;
pub use status::*;

#[cfg(feature = "k8")]
mod k8;
#[cfg(feature = "k8")]
pub use k8::*;

mod convert {

    use crate::core::{Spec, Status, Removable, Creatable};
    use crate::extended::{ObjectType, SpecExt};
    use super::*;

    impl Spec for ManagedConnectorSpec {
        const LABEL: &'static str = "ManagedConnector";

        type Status = ManagedConnectorStatus;

        type Owner = Self;
        type IndexKey = String;
    }

    impl SpecExt for ManagedConnectorSpec {
        const OBJECT_TYPE: ObjectType = ObjectType::ManagedConnector;
    }

    impl Removable for ManagedConnectorSpec {
        type DeleteKey = String;
    }

    impl Creatable for ManagedConnectorSpec {}

    impl Status for ManagedConnectorStatus {}

    #[cfg(feature = "k8")]
    mod extended {

        use crate::store::k8::K8ExtendedSpec;
        use crate::store::k8::K8ConvertError;
        use crate::store::k8::K8MetaItem;
        use crate::store::MetadataStoreObject;
        use crate::k8_types::K8Obj;
        use crate::store::k8::default_convert_from_k8;

        use super::ManagedConnectorSpec;
        use super::K8ManagedConnectorSpec;

        impl K8ExtendedSpec for ManagedConnectorSpec {
            type K8Spec = K8ManagedConnectorSpec;
            type K8Status = Self::Status;

            fn convert_from_k8(
                k8_obj: K8Obj<Self::K8Spec>,
            ) -> Result<MetadataStoreObject<Self, K8MetaItem>, K8ConvertError<Self::K8Spec>>
            {
                default_convert_from_k8(k8_obj)
            }
        }
    }
}
