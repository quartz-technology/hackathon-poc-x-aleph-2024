use std::fmt::format;

use ethers_signers::{LocalWallet, Signer, WalletError};
use serde::Serialize;
use thiserror::Error;

use super::BaseMessage;

#[derive(Debug, Error)]
pub enum MessageSignerError {
    #[error(transparent)]
    Error(#[from] WalletError),
}

pub trait MessageSigner {
    async fn sign<C: Serialize>(&self, message: &BaseMessage<C>) -> Result<String, MessageSignerError>;
    fn get_address(&self) -> String;
}

fn get_verificatin_buffer<C: Serialize>(message: &BaseMessage<C>) -> String {
    format!("{}\n{}\n{}\n{}", message.chain.as_str(), message.sender, message.message_type.as_str(), message.item_hash)
}

pub struct DefaultEthereumSigner {
    signer: LocalWallet,
}

impl DefaultEthereumSigner {
    pub fn new(private_key: String) -> Result<Self, MessageSignerError> {
        let signer = private_key.as_str().parse::<LocalWallet>()?;
        
        Ok(DefaultEthereumSigner {
            signer
        })
    }
}

impl MessageSigner for DefaultEthereumSigner {
    async fn sign<C: Serialize>(&self, message: &BaseMessage<C>) -> Result<String, MessageSignerError> {
        let buffer = get_verificatin_buffer(message);
        let signature = self.signer.sign_message(buffer).await?;

        Ok(format!("0x{}", hex::encode(signature.to_vec())))
    }

    fn get_address(&self) -> String {
        format!("0x{}", hex::encode(self.signer.address().as_bytes()))
    }
}
