mod abi;
mod pb;
use hex_literal::hex;
use pb::erc721;
use substreams::{log, store, Hex};
use substreams_ethereum::{pb::eth::v2 as eth, NULL_ADDRESS};

// Bored Ape Club Contract
const TRACKED_CONTRACT: [u8; 20] = hex!("bc4ca0eda7647a8ab7c2061c2e118a18a936f13d");

substreams_ethereum::init!();

/// Extracts transfers events from the contract
#[substreams::handlers::map]
fn map_transfers(blk: eth::Block) -> Result<erc721::Transfers, substreams::errors::Error> {
    Ok(erc721::Transfers {
        transfers: blk
            .events::<abi::erc721::events::Transfer>(&[&TRACKED_CONTRACT])
            .map(|(transfer, log)| {

                erc721::Transfer {
                    trx_hash: Hex(log.receipt.transaction.hash.clone()).to_string(),
                    from: Hex(&transfer.from).to_string(),
                    to: Hex(&transfer.to).to_string(),
                    token_id: transfer.token_id.low_u64(),
                    ordinal: log.block_index() as u64,
                }
            })
            .collect(),
    })
}

/// Store the total balance of NFT tokens for the specific TRACKED_CONTRACT by holder
#[substreams::handlers::store]
fn store_transfers(transfers: erc721::Transfers, s: store::StoreAddInt64) {
    log::info!("NFT holders state builder");
    for transfer in transfers.transfers {
        if transfer.from != Hex(&NULL_ADDRESS).to_string() {
            log::info!("Found transfer out {}", transfer.trx_hash);
            s.add(transfer.ordinal, generate_key(&transfer.from), -1);
        }

        if transfer.to != Hex(&NULL_ADDRESS).to_string() {
            log::info!("Found transfer in {}", transfer.trx_hash);
            s.add(transfer.ordinal, generate_key(&transfer.to), 1);
        }
    }
}

fn generate_key(holder: &String) -> String {
    return format!("total:{}:{}", holder, Hex(TRACKED_CONTRACT));
}

/// Store the total balance of NFT tokens for the specific TRACKED_CONTRACT by holder
#[substreams::handlers::store]
fn store_supply(transfers: erc721::Transfers, s: store::StoreAddInt64) {
    log::info!("Supply state builder");
    for transfer in transfers.transfers {
        if transfer.from == Hex(&NULL_ADDRESS).to_string() {
            log::info!("Found mint event {}", transfer.trx_hash);
            s.add(transfer.ordinal, "supply", 1);
        }
    }
}
