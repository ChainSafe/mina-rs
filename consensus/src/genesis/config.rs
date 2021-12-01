// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use mina_crypto::{
    base58::{version_bytes, Base58Encodable, Base58EncodableHash},
    hash::*,
    signature::PublicKey,
};
use mina_rs_base::{
    finite_ec_point, finite_ec_point_pair,
    types::{proof_messages::ProofMessageWithoutDegreeBoundList, *},
};

const ERR_FAIL_TO_DECODE_B58: &str = "Failed to decode hash from base58";
const ERR_FAIL_TO_DECODE_B64: &str = "Failed to decode hash from base64";
const ERR_FAIL_TO_DECODE_HEX: &str = "Failed to decode hash from hex";

lazy_static::lazy_static! {
    pub static ref MAINNET_CONFIG: GenesisInitConfig = GenesisInitConfig::mainnet();
    pub static ref DEVNET_CONFIG: GenesisInitConfig = GenesisInitConfig::devnet();
}

pub struct GenesisInitConfig {
    pub(crate) constants: ProtocolConstants,

    pub(crate) sub_windows_per_window: u32,
    pub(crate) last_vrf_output: VrfOutputTruncated,
    pub(crate) total_currency: Amount,
    pub(crate) sub_window_densities: Vec<Length>,
    pub(crate) staking_epoch_data: EpochData,
    pub(crate) next_epoch_data: EpochData,
    pub(crate) block_stake_winner: PublicKey,
    pub(crate) block_creator: PublicKey,
    pub(crate) coinbase_receiver: PublicKey,
    pub(crate) genesis_state_hash: StateHash,
    pub(crate) previous_state_hash: StateHash,
    pub(crate) blockchain_state: BlockchainState,
    pub(crate) protocol_state_proof: ProtocolStateProof,
    pub(crate) delta_transition_chain_proof: DeltaTransitionChainProof,
}

impl GenesisInitConfig {
    pub(crate) fn mainnet() -> Self {
        // https://github.com/MinaProtocol/mina/tree/feature/9665-spec-ouroboros-samasika-checkpointing/docs/specs/consensus#3-constants
        let constants = ProtocolConstants {
            k: 290.into(),
            slots_per_epoch: 7140.into(),
            slots_per_sub_window: 7.into(),
            delta: 0.into(),
            genesis_state_timestamp: BlockTime::from_unix_epoch(1615939200),
        };
        let total_currency = Amount(805385692840039233);

        let staking_epoch_data = {
            let mut data = EpochData::default();
            data.epoch_length.0 = 1;
            data.ledger.hash =
                LedgerHash::from_base58("jx7buQVWFLsXTtzRgSxbYcT8EYLS8KCZbLrfDcJxMtyy4thw2Ee")
                    .expect(ERR_FAIL_TO_DECODE_B58);
            data.ledger.total_currency = total_currency;
            data.seed =
                EpochSeed::from_base58("2va9BGv9JrLTtrzZttiEMDYw1Zj6a6EHzXjmP9evHDTG3oEquURA")
                    .expect(ERR_FAIL_TO_DECODE_B58);
            data.start_checkpoint =
                StateHash::from_base58("3NK2tkzqqK5spR2sZ7tujjqPksL45M3UUrcA4WhCkeiPtnugyE2x")
                    .expect(ERR_FAIL_TO_DECODE_B58);
            data.lock_checkpoint =
                StateHash::from_base58("3NK2tkzqqK5spR2sZ7tujjqPksL45M3UUrcA4WhCkeiPtnugyE2x")
                    .expect(ERR_FAIL_TO_DECODE_B58);
            data
        };

        let next_epoch_data = {
            let mut data = EpochData::default();
            data.epoch_length.0 = 2;
            data.ledger.hash =
                LedgerHash::from_base58("jx7buQVWFLsXTtzRgSxbYcT8EYLS8KCZbLrfDcJxMtyy4thw2Ee")
                    .expect(ERR_FAIL_TO_DECODE_B58);
            data.ledger.total_currency = total_currency;
            data.seed =
                EpochSeed::from_base58("2vaRh7FQ5wSzmpFReF9gcRKjv48CcJvHs25aqb3SSZiPgHQBy5Dt")
                    .expect(ERR_FAIL_TO_DECODE_B58);
            data.start_checkpoint =
                StateHash::from_base58("3NK2tkzqqK5spR2sZ7tujjqPksL45M3UUrcA4WhCkeiPtnugyE2x")
                    .expect(ERR_FAIL_TO_DECODE_B58);
            data.lock_checkpoint =
                StateHash::from_base58("3NLoKn22eMnyQ7rxh5pxB6vBA3XhSAhhrf7akdqS6HbAKD14Dh1d")
                    .expect(ERR_FAIL_TO_DECODE_B58);
            data
        };

        let blockchain_state = BlockchainState {
            timestamp: BlockTime::from_unix_epoch(1615939200),
            snarked_next_available_token: TokenId(2),
            snarked_ledger_hash: SnarkedLedgerHash::from_base58(
                "jx7buQVWFLsXTtzRgSxbYcT8EYLS8KCZbLrfDcJxMtyy4thw2Ee",
            )
            .expect(ERR_FAIL_TO_DECODE_B58),
            genesis_ledger_hash: SnarkedLedgerHash::from_base58(
                "jx7buQVWFLsXTtzRgSxbYcT8EYLS8KCZbLrfDcJxMtyy4thw2Ee",
            )
            .expect(ERR_FAIL_TO_DECODE_B58),
            staged_ledger_hash: StagedLedgerHash {
                non_snark: NonSnarkStagedLedgerHash {
                    ledger_hash: LedgerHash::from_base58(
                        "jx7buQVWFLsXTtzRgSxbYcT8EYLS8KCZbLrfDcJxMtyy4thw2Ee",
                    )
                    .expect(ERR_FAIL_TO_DECODE_B58),
                    aux_hash: AuxHash(decode_aux_hash_from_base58(
                        "UDRUFHSvxUAtV8sh7gzMVPqpbd46roG1wzWR6dYvB6RunPihom",
                        version_bytes::STAGED_LEDGER_HASH_AUX_HASH,
                    )),
                    pending_coinbase_aux: AuxHash(decode_aux_hash_from_base58(
                        "WAAeUjUnP9Q2JiabhJzJozcjiEmkZe8ob4cfFKSuq6pQSNmHh7",
                        version_bytes::STAGED_LEDGER_HASH_PENDING_COINBASE_AUX,
                    )),
                },
                pending_coinbase_hash: CoinBaseHash::from_base58(
                    "2n1tLdP2gkifmyVmrmzYXTS4ohPbZPJn6Qq4x55ywrbRWB4543cC",
                )
                .expect(ERR_FAIL_TO_DECODE_B58),
            },
        };

        let protocol_state_proof = {
            let mut p = ProtocolStateProof::default();

            p.statement.pass_through.sg = (|| {
                let fpp = finite_ec_point!(
                    "0x9c84b80df8bb7db9a77648695dc02c08996d3b6ce15a9bca579dd4d1ef2ca831",
                    "0x46fa45bdaa020b13e81365a2df2685c07d355deb88e2adf6a98769ab64177335"
                )?;
                Ok::<_, hex::FromHexError>(FiniteECPointVec(vec![fpp.clone(), fpp]))
            })()
            .expect(ERR_FAIL_TO_DECODE_HEX);

            p.statement.pass_through.old_bulletproof_challenges = {
                let t18 = BulletproofChallengeTuple18::new([
                    621834770194220300,
                    -4327941673388439925,
                    8902445049614368905,
                    -5479804816757020655,
                    8345091427968288705,
                    8258453988658898844,
                    746390447645740837,
                    -5643124118675291918,
                    -5948286762976073031,
                    3913620533516803836,
                    6451156200849374208,
                    -7149474906925249401,
                    2545802753099625135,
                    -3046285123411643010,
                    1476045778313053942,
                    7088211501506636392,
                    -5780902958823199474,
                    684672559108579835,
                    -479599555522051695,
                    6277689784759994790,
                    -6174909374622547011,
                    -2876420228592515736,
                    -5612139484668830099,
                    -141685165274757211,
                    -9057284384873862529,
                    -218923106050670804,
                    -4381571242292214582,
                    -7982158890774157783,
                    -2589693721356185105,
                    47703702537737607,
                    6933529253212730047,
                    -453811945482877525,
                    -7749055720453093542,
                    -3329383869546507530,
                    -4722006655881085948,
                    -2216518060947545516,
                ]);
                BulletproofChallenges(vec![t18.clone(), t18])
            };

            p.statement.proof_state.deferred_values.plonk.alpha =
                BulletproofPreChallenge::scalar(8485106177194710905, -1312226530362767951);
            p.statement.proof_state.deferred_values.plonk.beta =
                ScalarChallengeVector2::new(-6430667927186375774, -3337607204513408471);
            p.statement.proof_state.deferred_values.plonk.gamma =
                ScalarChallengeVector2::new(8370999550582557960, 2564953582899372946);
            p.statement.proof_state.deferred_values.plonk.zeta =
                BulletproofPreChallenge::scalar(723815570568662761, -7895844621402831008);
            p.statement
                .proof_state
                .deferred_values
                .combined_inner_product = ShiftedValue::ShiftedValue(
                BigInt256::try_from_hex_string(
                    "0x000c4669b4dcafbb30ba5442962c0e352818b397156aa59c007e1f2af989b938",
                )
                .expect(ERR_FAIL_TO_DECODE_HEX),
            );
            p.statement.proof_state.deferred_values.b = ShiftedValue::ShiftedValue(
                BigInt256::try_from_hex_string(
                    "0x405fb711eae9fca78875db8db66189c56d426fea068f2826fe5cb0b0afe06a0e",
                )
                .expect(ERR_FAIL_TO_DECODE_HEX),
            );
            p.statement.proof_state.deferred_values.xi =
                BulletproofPreChallenge::scalar(9192024484525083167, -4963136190301137182);
            p.statement
                .proof_state
                .deferred_values
                .bulletproof_challenges =
                p.statement.pass_through.old_bulletproof_challenges.0[0].clone();
            p.statement.proof_state.deferred_values.which_branch = Char(0);

            p.statement.proof_state.me_only.sg = finite_ec_point!(
                "0xc60a524476af46c04d571246d5dfd73b82d34a5cc50d3e279dd4e96239879c1b",
                "0xb9840fae14d5456882791e827d1b21be94a173c96e149e0a226af7d47431d604"
            )
            .expect(ERR_FAIL_TO_DECODE_HEX);
            p.statement.proof_state.me_only.old_bulletproof_challenges = {
                let t17 = BulletproofChallengeTuple17::new([
                    -4147613965780683501,
                    1618424163113385480,
                    5242665654050011345,
                    1974270862881305038,
                    3711726721677311855,
                    8761545649494038627,
                    -2505642933990422633,
                    -2487422680924118585,
                    -4113786261177851957,
                    566815174034496733,
                    5993263758714673683,
                    -6259514542256722053,
                    -2117686120277866613,
                    -550460597093069586,
                    2143052809542605959,
                    5556201566125368992,
                    -7172698754141626607,
                    -6773867482688065048,
                    6641554769559773261,
                    6524714810323564443,
                    2504198601910020936,
                    -6446749539458519571,
                    -8068243033437084562,
                    -4483743298391994862,
                    4919588161765452745,
                    -7115468514000056859,
                    4798728907681213006,
                    -7518434295045703748,
                    -7365006775264907657,
                    -1160122940338191724,
                    5439700123898059092,
                    -7701257753186809792,
                    3229587527204030387,
                    6616313680930013725,
                ]);
                ProofStateBulletproofChallenges((t17.clone(), t17, ()))
            };

            p.prev_evals.0 .0 = (|| {
                Ok::<_, hex::FromHexError>(ProofEvaluations {
                    l: FieldElementVec::try_from_hex_string(
                        "0x0e5019ef595b796f872edae874df3e6bd1e94424aed7ba4d7ac5fe5ae2252415",
                    )?,
                    r: FieldElementVec::try_from_hex_string(
                        "0xcece924af1289b226fbb6cbef809b94422a9d91f0782ada0d87efc83d019b91b",
                    )?,
                    o: FieldElementVec::try_from_hex_string(
                        "0x4a4ee9d8cef37e704d57836e2bd42c696466e27197e166e5a4ac87af6f17a924",
                    )?,
                    z: FieldElementVec::try_from_hex_string(
                        "0xde7e57ed0235ed1315ee7c76582fe85ef13410571a7537bcd7bd3919f266ec1e",
                    )?,
                    t: FieldElementVec::try_from_hex_string(
                        [
                            "8d1ef8e24acf35e68f404480324352e921645bc354b05644e7bd4eb3a80e9d13",
                            "d23d44555c569324067d82817632a4b48be314267101a09873197275d0284f05",
                            "9984de5ad3091c523f90db6792526b36b3d0936bda8aa8aa82112748e1aafb2d",
                            "9d53c85bdbc8b9f895fd3cf285409ade24d077c6b1fb884be8ed03a609207a15",
                            "7ea8c0fb257e5118e548d9c7d59f5858ca153b9f785062313d250e24e2032914",
                        ]
                        .join(""),
                    )?,
                    f: FieldElementVec::try_from_hex_string(
                        "0xadb1afcfaffe7569656b54d5ce39f7c11a55958f63f432efba9aa96334e3f013",
                    )?,
                    sigma1: FieldElementVec::try_from_hex_string(
                        "0x0d19edbacb9cf02bbf5c1c1053d422479526855038c072772a0a970f3a54d525",
                    )?,
                    sigma2: FieldElementVec::try_from_hex_string(
                        "0x89ac7052c36f50a89bcd060ba67fa9446dfeb0b6d80a74ef617dd4af66cc2039",
                    )?,
                })
            })()
            .expect(ERR_FAIL_TO_DECODE_HEX);

            p.prev_evals.0 .1 = (|| {
                Ok::<_, hex::FromHexError>(ProofEvaluations {
                    l: FieldElementVec::try_from_hex_string(
                        "0xac5d1055e6dc85e696cde2e4cd58e4117556622de616c04b3427ebcf4e792139",
                    )?,
                    r: FieldElementVec::try_from_hex_string(
                        "0xa2e054c65b914b2ecca3a9baa335a0d6a5019cfbb19ce073ead52e1f0f963f30",
                    )?,
                    o: FieldElementVec::try_from_hex_string(
                        "0xde64bff3163383af2268774d29f97382656925a3846e08d1b9dbfd8107095008",
                    )?,
                    z: FieldElementVec::try_from_hex_string(
                        "0x1a79c92067af3a93e38e3b7ab551ed30cb2598be3d531afcb3161ff46452790c",
                    )?,
                    t: FieldElementVec::try_from_hex_string(
                        [
                            "ffdb4e909d77296575ef162db731581693a8ae1a2628e9d7b114f6e64832300a",
                            "17e6fcda006321d26f68e2968b0848763607f832f9adb0baa71a15b92f798825",
                            "cfe6f5fcc752393dcbfc941256c2ba8499d32228ae1add120d0fbeccd9389233",
                            "2888c27164b90b341a97a882c5454f5868a8d68796b3f18ef54280ee3f96283a",
                            "c3181d87d0ab4a99cec885b3d4de51b0bca32b306ebece8dca1a872bc0efc41d",
                        ]
                        .join(""),
                    )?,
                    f: FieldElementVec::try_from_hex_string(
                        "0x40128adc323cecd24b5f4deff834be960c90a5c97d16ce6d123ede8c95b2bf3c",
                    )?,
                    sigma1: FieldElementVec::try_from_hex_string(
                        "0x92f5fcdc0c4cd3f6de427948b078312e0b3744b64932b55af6141ce8fe6ed82c",
                    )?,
                    sigma2: FieldElementVec::try_from_hex_string(
                        "0x4cb1e1bcec843fab7e37780f44880d58ebfa3c28b4c0c913ae41495a5bcf5b19",
                    )?,
                })
            })()
            .expect(ERR_FAIL_TO_DECODE_HEX);

            p.prev_x_hat = PrevXHat(
                finite_ec_point!(
                    "0xc0c84dfea15306bb962715b921c0117fde9f3f9dffa9734fca977cae86549037",
                    "0xad6cf259ffead18d3afd382f410e771a3d5d0f261454fa699693d81c45cc803c"
                )
                .expect(ERR_FAIL_TO_DECODE_HEX),
            );

            let pm = &mut p.proof.messages;
            let ec_point = finite_ec_point!(
                "0x0100000000000000000000000000000000000000000000000000000000000000",
                "0xbb2aedca237acf1971473d33d45b658f54ee7863f0a9df537c93120aa3b5741b"
            )
            .expect(ERR_FAIL_TO_DECODE_HEX);
            let comm = ProofMessageWithoutDegreeBoundList(vec![ec_point.clone()]);
            pm.l_comm = comm.clone();
            pm.r_comm = comm.clone();
            pm.o_comm = comm.clone();
            pm.z_comm = comm;

            pm.t_comm.shifted = ECPoint::Finite(ec_point.clone());
            pm.t_comm.unshifted = ECPointVec(vec![
                ECPoint::Finite(ec_point.clone()),
                ECPoint::Finite(ec_point.clone()),
                ECPoint::Finite(ec_point.clone()),
                ECPoint::Finite(ec_point.clone()),
                ECPoint::Finite(ec_point.clone()),
            ]);

            let pr = &mut p.proof.openings.proof;
            pr.lr = (|| {
                let pair = finite_ec_point_pair!(
                    "0x0100000000000000000000000000000000000000000000000000000000000000",
                    "0xbb2aedca237acf1971473d33d45b658f54ee7863f0a9df537c93120aa3b5741b",
                    "0x0100000000000000000000000000000000000000000000000000000000000000",
                    "0xbb2aedca237acf1971473d33d45b658f54ee7863f0a9df537c93120aa3b5741b"
                )?;
                Ok::<_, hex::FromHexError>(FiniteECPointPairVec(vec![
                    pair.clone(),
                    pair.clone(),
                    pair.clone(),
                    pair.clone(),
                    pair.clone(),
                    pair.clone(),
                    pair.clone(),
                    pair.clone(),
                    pair.clone(),
                    pair.clone(),
                    pair.clone(),
                    pair.clone(),
                    pair.clone(),
                    pair.clone(),
                    pair.clone(),
                    pair.clone(),
                    pair,
                ]))
            })()
            .expect(ERR_FAIL_TO_DECODE_HEX);
            pr.z_1 = BigInt256::try_from_hex_string(
                "0x27a6753e3c16b98921d53d185402e874f318a819fdf2f3ac6667262045aa8a26",
            )
            .expect(ERR_FAIL_TO_DECODE_HEX);
            pr.z_2 = BigInt256::try_from_hex_string(
                "0x664e7ca6fe93f151f069508b826ad5d06c71549318cbc911da6b74d06efe2806",
            )
            .expect(ERR_FAIL_TO_DECODE_HEX);
            pr.delta = ec_point.clone();
            pr.sg = ec_point;

            p.proof.openings.evals.0 = (|| {
                Ok::<_, hex::FromHexError>(ProofEvaluations {
                    l: FieldElementVec::try_from_hex_string(
                        "0x2e53605b801ad7fea745e9766add8da9ed33589d758fb339fed40c329c59aa27",
                    )?,
                    r: FieldElementVec::try_from_hex_string(
                        "0xb77a8788b07f7cd1c9c61618755cca3d0d303a7b096124ce0c02dc5f451a0f03",
                    )?,
                    o: FieldElementVec::try_from_hex_string(
                        "0x2e1e68731d00b84720038823777ec6522d9a1e9e365920c3e7ce064ade0c2e1e",
                    )?,
                    z: FieldElementVec::try_from_hex_string(
                        "0xd96d62e54a0a49d3a44c919eb4b089333d64a236edcda1921274ac6903bad937",
                    )?,
                    t: FieldElementVec::try_from_hex_string(
                        [
                            "717115e59713c84f88babe2ec0292518060d2cc82b54e9a9c9a2d2a87ce91e15",
                            "6994e270f284a557c418afebfaaca2794c8af6a476cb1b9478c205e8a901170f",
                            "d82d38717842bde317157edf186a5b2a5ac2a035a069b18a1bb790d8a1b60e26",
                            "c37d692c8473aa9a246bb85e5c4323cd0c5a69e4b9ce1ae160f961447c31ae2e",
                            "cce3a78dfa242d8c53e89467cc986dfd332db987d76c66e7735a47ef34e90f28",
                        ]
                        .join(""),
                    )?,
                    f: FieldElementVec::try_from_hex_string(
                        "0x5dd93c9b2c3fcee30fa34960f2472fcd04d9de8486f635c9b96d776fae31221f",
                    )?,
                    sigma1: FieldElementVec::try_from_hex_string(
                        "0xa84f94a0d6d64be0b97049b92ae2c58a8cb93e792179fab57fa32c4695abe724",
                    )?,
                    sigma2: FieldElementVec::try_from_hex_string(
                        "0x2c7c6aa5123b41aa8eace85a7eeeb8ebb22219c9353b9276711199aaa8018217",
                    )?,
                })
            })()
            .expect(ERR_FAIL_TO_DECODE_HEX);

            p.proof.openings.evals.1 = (|| {
                Ok::<_, hex::FromHexError>(ProofEvaluations {
                    l: FieldElementVec::try_from_hex_string(
                        "0x16eba2ebda9feac442e29ef9293f5c4576933d531a6e3c07518e352241055f3d",
                    )?,
                    r: FieldElementVec::try_from_hex_string(
                        "0xdcf5b2e12453b8369c420e76ada0fb6c6e173f2271aa19ec6db8010112611605",
                    )?,
                    o: FieldElementVec::try_from_hex_string(
                        "0x35362d986f20c598e53c3de0b8fc41300484243172af893cc99ca199aa16163c",
                    )?,
                    z: FieldElementVec::try_from_hex_string(
                        "0xf0951e6a385fb4ea8b5e2cf0e89e54807a99938b0ab69c77f1b9b210a05d152e",
                    )?,
                    t: FieldElementVec::try_from_hex_string(
                        [
                            "b5c98d3a881eaad5600d89920dff83025079d27bde3ceadd14425bfc8a40d310",
                            "ee802beaf4ddbaf3b69698689d7e76b670caa65ddbd92197227ab0c8dfba3624",
                            "6bdf230ec07a915319c606ad930c41dd7f097222ada2776a484e755feb2d491c",
                            "e00d36cc2b6076c23184046c0a2a062085215644fe29549a6252025055bdfb1c",
                            "37befa9d80c628fb8b3f7f5316912c175426a0ad9a83db780847d636f1ccab09",
                        ]
                        .join(""),
                    )?,
                    f: FieldElementVec::try_from_hex_string(
                        "0xc0441628012519d76fef0107434dc56bb174e7d1610cde2fc86d6aa72b75ad1a",
                    )?,
                    sigma1: FieldElementVec::try_from_hex_string(
                        "0xcd71c8afe1a719f2e5e83fce7941fb9a313e2b9262480afa68675dcfab64b20a",
                    )?,
                    sigma2: FieldElementVec::try_from_hex_string(
                        "0xe580093d240406f6684b313ce40669bd5ba1c8df3ed53ced2f473c037af19a08",
                    )?,
                })
            })()
            .expect(ERR_FAIL_TO_DECODE_HEX);

            p
        };

        Self {
            sub_windows_per_window: 11,
            last_vrf_output: VrfOutputTruncated::try_from_base64(
                "NfThG1r1GxQuhaGLSJWGxcpv24SudtXG4etB0TnGqwg=",
            )
            .expect(ERR_FAIL_TO_DECODE_B64),
            total_currency,
            sub_window_densities: vec![
                1.into(),
                7.into(),
                7.into(),
                7.into(),
                7.into(),
                7.into(),
                7.into(),
                7.into(),
                7.into(),
                7.into(),
                7.into(),
            ],
            constants,
            staking_epoch_data,
            next_epoch_data,
            block_stake_winner: PublicKey::from_base58(
                "B62qiy32p8kAKnny8ZFwoMhYpBppM1DWVCqAPBYNcXnsAHhnfAAuXgg",
            )
            .expect(ERR_FAIL_TO_DECODE_B58),
            block_creator: PublicKey::from_base58(
                "B62qiy32p8kAKnny8ZFwoMhYpBppM1DWVCqAPBYNcXnsAHhnfAAuXgg",
            )
            .expect(ERR_FAIL_TO_DECODE_B58),
            coinbase_receiver: PublicKey::from_base58(
                "B62qiy32p8kAKnny8ZFwoMhYpBppM1DWVCqAPBYNcXnsAHhnfAAuXgg",
            )
            .expect(ERR_FAIL_TO_DECODE_B58),
            genesis_state_hash: StateHash::from_base58(
                "3NLoKn22eMnyQ7rxh5pxB6vBA3XhSAhhrf7akdqS6HbAKD14Dh1d",
            )
            .expect(ERR_FAIL_TO_DECODE_B58),
            previous_state_hash: StateHash::from_base58(
                "3NLoKn22eMnyQ7rxh5pxB6vBA3XhSAhhrf7akdqS6HbAKD14Dh1d",
            )
            .expect(ERR_FAIL_TO_DECODE_B58),
            blockchain_state,
            protocol_state_proof,
            delta_transition_chain_proof: (
                StateHash::from_base58("3NLoKn22eMnyQ7rxh5pxB6vBA3XhSAhhrf7akdqS6HbAKD14Dh1d")
                    .expect(ERR_FAIL_TO_DECODE_B58),
                Default::default(),
            ),
        }
    }

    pub(crate) fn devnet() -> Self {
        // FIXME: Figure out devnet config
        Self::mainnet()
    }
}

fn decode_aux_hash_from_base58(s: impl AsRef<[u8]>, check: u8) -> Vec<u8> {
    let bytes: Vec<u8> = bs58::decode(s)
        .with_check(Some(check))
        .into_vec()
        .expect(ERR_FAIL_TO_DECODE_B58);

    bytes.into_iter().skip(1).take(32).collect()
}
