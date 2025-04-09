// @generated
pub mod tycho {
    pub mod evm {
        pub mod uniswap {
            // @@protoc_insertion_point(attribute:tycho.evm.uniswap.v2)
            pub mod v2 {
                include!("tycho.evm.uniswap.v2.rs");
                // @@protoc_insertion_point(tycho.evm.uniswap.v2)
            }
        }
        pub mod ethereum {
            pub mod substream {
                pub mod v1{
                    include!("sf.ethereum.substreams.v1.rs");
                }
               
            }
            pub mod v2 {
                include!("sf.ethereum.type.v2.rs");
            }
        }
    }
}
