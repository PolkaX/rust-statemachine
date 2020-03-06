// Copyright 2020 PolkaX

use crate::SectorState;
use filecoin_proofs_api::{PieceInfo, Commitment, ChallengeSeed, Ticket};
use cid::Cid;

type Piece = PieceInfo;
type SealTicket = Ticket;
type SealSeed = ChallengeSeed;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SectorInfo {
    state: SectorState,
    sector_id: u64,
    nonce: u64,

    pieces: Vec<Piece>,
    commd: Commitment,
    commr: Commitment,
    proof: Commitment,
    ticket: SealTicket,

    pre_commit_msg: Cid,
    seed: SealSeed,
    commit_msg: Cid,
    fault_report_msg: Cid,
}
