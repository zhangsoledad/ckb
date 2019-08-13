use super::parse_hex_data;
use ckb_app_config::{cli, ExitCode};
use ckb_crypto::secp::Pubkey;
use ckb_hash::blake2b_256;
use ckb_jsonrpc_types::ScriptHashType;
use ckb_resource::CODE_HASH_SECP256K1_BLAKE160_SIGHASH_ALL;
use ckb_types::H160;
use clap::ArgMatches;

pub fn secp256k1_lock<'m>(matches: &ArgMatches<'m>) -> Result<(), ExitCode> {
    let pubkey_bytes = parse_hex_data(matches.value_of(cli::ARG_DATA).unwrap())?;
    if pubkey_bytes.len() != 65 && pubkey_bytes.len() != 33 {
        eprintln!(
            "Expect pubkey length 65 (uncompressed) or 33 (compressed), actual: {}",
            pubkey_bytes.len()
        );
        return Err(ExitCode::IO);
    }

    let pubkey = Pubkey::from_slice(&pubkey_bytes).map_err(|err| {
        eprintln!("Pubkey corrupted: {}", err);
        ExitCode::IO
    })?;

    let pubkey_hash = blake2b_256(&pubkey.serialize());
    let pubkey_blake160 = H160::from_slice(&pubkey_hash[0..20]).unwrap();

    let serialized_hash_type = serde_plain::to_string(&ScriptHashType::Data).unwrap();

    match matches.value_of(cli::ARG_FORMAT).unwrap() {
        "toml" => {
            println!("[block_assembler]");
            println!("# secp256k1_blake160_sighash_all");
            println!(
                "code_hash = \"{:#x}\"",
                CODE_HASH_SECP256K1_BLAKE160_SIGHASH_ALL
            );
            println!("# args = [ \"ckb cli blake160 <compressed-pubkey>\" ]");
            println!("args = [ \"{:#x}\" ]", pubkey_blake160);
            println!("# hash_type can be Data or Type depending on the lock script");
            println!("hash_type = \"{}\"", serialized_hash_type);
        }
        "cmd" => {
            println!(
                "--ba-code-hash {:#x} --ba-arg {:#x} --ba-hash-type {}",
                CODE_HASH_SECP256K1_BLAKE160_SIGHASH_ALL, pubkey_blake160, serialized_hash_type,
            );
        }
        _ => unreachable!(),
    }

    Ok(())
}
