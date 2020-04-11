use crate::identifiers::cred_def::CredentialDefinitionId;
use crate::identifiers::schema::SchemaId;
use crate::ursa::cl::{CredentialKeyCorrectnessProof, Nonce};
use crate::utils::qualifier::Qualifiable;
use crate::{ConversionError, TryClone, Validatable, ValidationError};

#[derive(Debug, Deserialize, Serialize)]
pub struct CredentialOffer {
    pub schema_id: SchemaId,
    pub cred_def_id: CredentialDefinitionId,
    pub key_correctness_proof: CredentialKeyCorrectnessProof,
    pub nonce: Nonce,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method_name: Option<String>,
}

impl CredentialOffer {
    #[allow(unused)]
    pub fn to_unqualified(self) -> CredentialOffer {
        let method_name = self.cred_def_id.get_method().map(str::to_owned);
        CredentialOffer {
            schema_id: self.schema_id.to_unqualified(),
            cred_def_id: self.cred_def_id.to_unqualified(),
            key_correctness_proof: self.key_correctness_proof,
            nonce: self.nonce,
            method_name,
        }
    }
}

impl TryClone for CredentialOffer {
    fn try_clone(&self) -> Result<Self, ConversionError> {
        Ok(Self {
            schema_id: self.schema_id.clone(),
            cred_def_id: self.cred_def_id.clone(),
            key_correctness_proof: self.key_correctness_proof.try_clone()?,
            nonce: self.nonce.try_clone()?,
            method_name: self.method_name.clone(),
        })
    }
}

impl Validatable for CredentialOffer {
    fn validate(&self) -> Result<(), ValidationError> {
        self.schema_id.validate()?;
        self.cred_def_id.validate()?;
        Ok(())
    }
}
