use std::sync::{Arc, OnceLock};

use nostr_types::{PreEvent, Unixtime};
use pw_lib::client::{PowWowClient, PowWowClientTrait};
use pw_lib::ln_sdk::client_v2::nwc::nostr_utils::NostrSignerType;
use pw_lib::ln_sdk::http::HttpConfig;
use pw_lib::model::config::{ClientTradeConfig, PowWowConfig};
use pw_lib::model::nostr::{ClientAskStub, NonBindingAsk};
use pw_lib::nostr_sdk::prelude::get_leading_zero_bits;
use pw_lib::nostr_sdk::Kind;
use pw_lib::nostr_sdk;
use tokio::runtime::Handle;
use tokio::sync::Mutex;

use crate::{Error, ErrorKind, GLOBALS};

pub(crate) fn pw_client() -> &'static Mutex<Option<Arc<PowWowClient>>> {
    static ARRAY: OnceLock<Mutex<Option<Arc<PowWowClient>>>> = OnceLock::new();
    ARRAY.get_or_init(|| Mutex::new(None))
}

pub(crate) async fn create_client() -> Result<Arc<PowWowClient>, Error> {
    tracing::info!("Initializing PoW WoW client");

    let powwow_config = get_powwow_config();
    let ln_config = powwow_config.ln.clone();
    let pwc = pw_lib::client::init_powwow_client_and_ts(
        NostrSignerType::Local(powwow_config.relays.clone(), powwow_config.maybe_sk()),
        Arc::new(ln_config),
        Arc::new(HttpConfig::default()),
        true,
    )
    .await
    .map_err(|e| Error {
        kind: ErrorKind::General(format!("Cannot initialize PoW WoW client: {e}")),
        file: Some("powwow_client.rs"),
        line: None,
    })?;

    Ok(pwc)
}

/// Tries to autobuy PoW using the default params (PoW, price, timeframe). If no valid offer is
/// received in that time, it times out. If autobuy is successful, it reconstructs the PreEvent
/// by integrating the mined changes (PoW nonce, timestamp) and returns it.
pub fn try_autobuy_pow(preevent: &PreEvent) -> Result<PreEvent, Error> {
    let rt = Handle::try_current().expect("Failed to get tokio runtime");

    let preevent_clone = preevent.clone();
    let autobuy_res = std::thread::spawn(move || {
        rt.block_on(async {
            let lock = pw_client().lock().await;
            let pw_client = lock.as_ref().expect("Failed to get PoW WoW client");

            // Convert tags from nostr_types to nostr_sdk type
            let ev_tags = preevent_clone
                .tags
                .iter()
                .map(|tag| serde_json::to_string(&tag).expect("Failed to convert tag to JSON"))
                .map(|json| serde_json::from_str(&json).expect("Failed to parse tag JSON"))
                .collect::<Vec<nostr_sdk::Tag>>();

            // TODO Configure in UI
            // TODO Price should depend on the ephemeral offers (e.g. configure max, but choose based on offer prices)
            let default_trade_config = get_powwow_client_default_trade_config();

            pw_client
                .auto_buy_for(ClientAskStub {
                    price_sats: default_trade_config.default_price_sats,
                    timeframe_sec: default_trade_config.default_timeframe_sec,
                    core_data: NonBindingAsk {
                        target_difficulty: default_trade_config.default_difficulty,
                        ev_pubkey: preevent_clone.pubkey.as_xonly_public_key(),
                        ev_kind: Kind::from(u32::from(preevent_clone.kind) as u64),
                        ev_content: preevent_clone.content,
                        ev_tags,
                    },
                })
                .await
        })
    })
    .join()
    .expect("Failed to trigger PoW WoW in separate thread");

    // Reconstruct mined preevent
    autobuy_res
        .map(|(trade_id, ev)| {
            tracing::info!("Autobuy successful, trade ID: {trade_id}");

            let pow = get_leading_zero_bits(ev.id);
            GLOBALS
                .status_queue
                .write()
                .write(format!("Autobuy successful: {pow} bits"));

            // Initial preevent tags, plus the mined nonce at the end
            let preevent_tags = ev
                .tags
                .iter()
                .map(|tag| serde_json::to_string(&tag).expect("Failed to convert tag to JSON"))
                .map(|json| serde_json::from_str(&json).expect("Failed to parse tag JSON"))
                .collect::<Vec<nostr_types::Tag>>();

            // Only two fields may have changed as a result of the PoW WoW mining
            let reconstructed = PreEvent {
                created_at: Unixtime::from(ev.created_at.as_i64()),
                tags: preevent_tags,
                ..preevent.clone()
            };

            Ok(reconstructed)
        })
        .map_err(|e| {
            GLOBALS
                .status_queue
                .write()
                .write(format!("Autobuy failed: {e}"));

            Error {
                kind: ErrorKind::General(format!("PoW WoW auto-buy failed: {e}")),
                file: Some("powwow_client.rs"),
                line: None,
            }
        })?
}

fn get_powwow_config() -> PowWowConfig {
    pw_lib::get_powwow_config(Some("pw_config.toml".into())).expect("Could not open PoW WoW config")
}

fn get_powwow_client_default_trade_config() -> ClientTradeConfig {
    pw_lib::get_client_default_trade_config(Some("pw_config.toml".into()))
        .expect("Could not open PoW WoW config")
}
