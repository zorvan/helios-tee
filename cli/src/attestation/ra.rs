#![allow(dead_code)]


#[cfg(feature = "sgx")]
pub mod sgx {

    use std::{
        fs::{File, OpenOptions},
        io::{Error, Read, Write},
        path::Path,
    };
    
    use anyhow::{anyhow, Result};
    use axum::{extract::State, response::IntoResponse};
    use base64::{engine::general_purpose, prelude::*};
    use reqwest::StatusCode;
    use tracing::{debug, error, info, trace};
    
    use crate::attestation::server::{QuoteResponse, AppState};
    use ed25519_dalek::Signer;

    pub const QUOTE_REPORT_DATA_OFFSET: usize = 368;
    pub const QUOTE_REPORT_DATA_LENGTH: usize = 64;

    pub async fn ra_get_quote(State(state): State<AppState>) -> impl IntoResponse {
        // Make a dynamic user data
        let sign_data = state.client_account;

        debug!("QUOTE : report_data token = {}", sign_data);

        let signature = state.keypair.sign(sign_data.as_bytes());
        let public_key = hex::encode(state.keypair.verifying_key());

        match write_user_report_data(None, &signature.to_bytes()) {
            Ok(_) => debug!("QUOTE : Success writing report_data to the quote."),

            Err(err) => {
                return axum::Json(QuoteResponse {
                    status: StatusCode::INTERNAL_SERVER_ERROR.to_string(),
                    public_key,
                    quote: err.to_string(),
                })
            }
        };

        match get_quote_content() {
            Ok(quote_byte) => {
                let quote_base64 = general_purpose::STANDARD.encode(quote_byte);
                axum::Json(QuoteResponse {
                    status: StatusCode::OK.to_string(),
                    public_key,
                    quote: quote_base64,
                })
            }

            Err(err) => axum::Json(QuoteResponse {
                status: StatusCode::INTERNAL_SERVER_ERROR.to_string(),
                public_key,
                quote: err.to_string(),
            }),
        }
    }

    /// Reads the quote or else returns an error
    /// # Arguments
    /// * `file_path` - The path to the quote
    /// # Returns
    /// * `Result<Vec<u8>, Error>` - The result of the quote
    pub fn get_quote_content() -> Result<Vec<u8>, Error> {
        info!("QUOTE : Reading The Quote ...");
        let default_path = "/dev/attestation/quote";
        let mut content = vec![];

        File::open(default_path)
            .and_then(|mut file| {
                file.read_to_end(&mut content).map_err(|err| {
                    error!("QUOTE : Error opening file /dev/attestation/quote {err:?}");
                    err
                })
            })
            .map(|_| {
                trace!("\nQuote : content {:?}\n", content);
                content
            })
    }

    /// Reads the attestation type or else returns an error
    /// # Arguments
    /// * `file_path` - The path to the attestation type
    /// # Returns
    /// * `Result<String, Error>` - The result of the attestation type
    fn read_attestation_type(file_path: Option<String>) -> Result<String, Error> {
        let default_path = "/dev/attestation/attestation.attestation_type";
        let mut attest_type = String::new();

        File::open(file_path.unwrap_or(String::from(default_path)))
            .and_then(|mut file| {
                file.read_to_string(&mut attest_type).map_err(|err| {
                    error!("QUOTE : Error reading file: {err:?}");
                    err
                })
            })
            .map(|_| {
                debug!("QUOTE : attestation type is : {}", attest_type);
                attest_type
            })
    }

    /// Writes user report data or else throws an Error
    /// # Arguments
    /// * `file_path` - The path to the user report data
    /// # Returns
    /// * `Result<(), Error>` - The result of the user report data
    pub fn write_user_report_data(
        file_path: Option<String>,
        user_data: &[u8; 64],
    ) -> Result<(), anyhow::Error> {
        let default_path = "/dev/attestation/user_report_data";
        if !is_user_report_data_exist(None) {
            return Err(anyhow!("QUOTE : it is not an enclave"));
        }

        Ok(OpenOptions::new()
            .write(true)
            .open(file_path.unwrap_or(String::from(default_path)))
            .and_then(|mut file| {
                info!("QUOTE : This is inside Enclave!");
                file.write_all(user_data.as_slice()).map_err(|err| {
                    error!("QUOTE : Error writing to {} {:?}", default_path, err);
                    err
                })
            })
            .map_err(|err| {
                error!("QUOTE : Error writing file: {err:?}");
                err
            })
            .map(|_| ())?)
    }

    /// Check if file exists with correct permissions or else returns false
    /// # Arguments
    /// * `file_path` - The path to the user report data
    /// # Returns
    /// * `bool` - The result of the user report data
    fn is_user_report_data_exist(file_path: Option<String>) -> bool {
        return match file_path {
            None => Path::new("/dev/attestation/user_report_data").exists(),
            Some(f) => Path::new(&f).exists(),
        };
    }
}

#[cfg(feature = "tdx")]
pub mod tdx 
{
    use axum::{extract::State, response::IntoResponse};
    use reqwest::StatusCode;
    use tracing::{debug, error, info};
    use ed25519_dalek::Signer;

    use crate::attestation::server::{QuoteResponse, AppState};

    use configfs_tsm::create_quote;

    pub const QUOTE_REPORT_DATA_OFFSET: usize = 368;
    pub const QUOTE_REPORT_DATA_LENGTH: usize = 64;

    pub async fn ra_get_quote(State(state): State<AppState>) -> impl IntoResponse {

        let sign_data = state.client_account;

        debug!("QUOTE : report_data token = {}", sign_data);

        let signature = state.keypair.sign(sign_data.as_bytes());
        let public_key = hex::encode(state.keypair.verifying_key());

        match create_quote(signature.to_bytes()) {
            Ok(quote_byte) => {
                info!("QUOTE : success generating quote");
                
                let quote_hex = hex::encode(quote_byte);
                return axum::Json(QuoteResponse {
                    status: StatusCode::OK.to_string(),
                    public_key,
                    quote: quote_hex,
                })
            }

            Err(err) => {
                error!("QUOTE : error generating quote, {:?}", err);
                
                return axum::Json(QuoteResponse {
                    status: StatusCode::INTERNAL_SERVER_ERROR.to_string(),
                    public_key,
                    quote: err.to_string(),
                })
            }
        };
    }

}