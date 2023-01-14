use thiserror::Error;

/// ProofError related to the proof primitives
#[derive(Error, Debug)]
pub enum ProofError {
    /// This error occurs when a proof failed to verify.
    #[error("Verification error")]
    VerificationError,
}
