use substreams_entity_change::pb::entity::{entity_change::Operation, EntityChanges};
use crate::pb::erc721;
use substreams::scalar::BigInt;

pub fn transfer_entity_change(
    entity_changes: &mut EntityChanges,
    transfers: erc721::Transfers,
) {
    for transfer in transfers.transfers {
        entity_changes
            .push_change(
                "Transfer",
                format!("{}_{}_{}", transfer.token_id, transfer.trx_hash, transfer.ordinal).as_str(),
                transfer.ordinal,
                Operation::Create
            )
            .change("id", format!("{}_{}_{}", transfer.token_id, transfer.trx_hash, transfer.ordinal))
            .change("from", transfer.from)
            .change("to", transfer.to)
            .change("token_id", BigInt::from(transfer.token_id))
            .change("trx_hash", transfer.trx_hash);
    }
}