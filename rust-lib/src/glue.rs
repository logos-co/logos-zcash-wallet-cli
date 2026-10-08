//! Logos glue for `zcash_wallet_cli`: a headless relay to `zcash_wallet_backend`
//! for `logosctl`. Secrets arrive as `@file` or `str:` arguments, never in argv,
//! and are scrubbed after the call. A role refusal comes back with the exact
//! `configure` that would admit this module.

use serde_json::{json, Value};

use crate::relay::{configure_hint, scrub, strip_file_newline, translate_refusal};

const ME: &str = "zcash_wallet_cli";

pub trait ZcashWalletCliModule: Send + Sync + 'static {
    fn caller_identity(&self) -> String;
    fn list_networks(&self) -> String;
    fn set_active_network(&self, network: String) -> String;
    fn list_wallets(&self) -> String;
    fn create_wallet(&self, name: String, password: String) -> String;
    /// `params_json`: `{ name, password, phrase, birthdayHeight }`, best passed as `@file`.
    fn restore_wallet(&self, params_json: String) -> String;
    fn open_wallet(&self, name: String, password: String) -> String;
    fn change_password(&self, old_password: String, new_password: String) -> String;
    fn close_wallet(&self) -> String;
    fn job_status(&self, job_id: String) -> String;
    fn reveal_seed(&self, password: String) -> String;
    fn export_viewing_key(&self, password: String) -> String;
    fn wallet_status(&self) -> String;
    fn sync_status(&self) -> String;
    fn balances(&self) -> String;
    fn receive_info(&self) -> String;
    fn address_new(&self) -> String;
    fn history(&self, page: i64) -> String;
    fn servers(&self) -> String;
    fn server_health(&self) -> String;
    fn apply_preset(&self, name: String) -> String;
    fn set_proxy(&self, config_json: String) -> String;
    /// `request_json`: `{ recipients: [{ address, amount (zatoshis), memo? }] }` or `{ uri }`.
    fn prepare_send(&self, request_json: String) -> String;
    fn send_status(&self, request_id: String) -> String;
    fn list_sends(&self) -> String;
    /// Needs the approver role; the password is best passed as `@file`.
    fn approve_send(&self, request_id: String, password: String) -> String;
    fn cancel_send(&self, request_id: String) -> String;
    fn prepare_shielding(&self, address: String) -> String;

    fn on_context_ready(&self, _ctx: &RustModuleContext) {}
}

pub trait ZcashWalletCliModuleEvents {
    fn job_settled(&self, job_id: String, state: String);
}

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/generated/provider_gen.rs"));

#[derive(Default)]
pub struct ZcashWalletCliModuleImpl;

fn err(e: impl std::fmt::Display) -> String {
    json!({"ok": false, "error": e.to_string()}).to_string()
}

fn hint() -> String {
    let identity = modules()
        .zcash_wallet_backend
        .caller_identity()
        .ok()
        .and_then(|s| serde_json::from_str::<Value>(&s).ok())
        .unwrap_or(Value::Null);
    configure_hint(&identity, ME, &["custodians", "approvers"])
}

fn relay_as(reply: Result<String, impl std::fmt::Debug>, role_word: &str) -> String {
    match reply {
        Ok(s) => translate_refusal(&s, ME, role_word, hint),
        Err(e) => err(format!("wallet backend unreachable: {e:?}")),
    }
}

fn custodian(reply: Result<String, impl std::fmt::Debug>) -> String {
    relay_as(reply, "custodian")
}

fn read(reply: Result<String, impl std::fmt::Debug>) -> String {
    relay_as(reply, "custodian")
}

fn with_secret<T>(secret: &mut String, f: impl FnOnce(&str) -> T) -> T {
    let out = f(strip_file_newline(secret));
    scrub(secret);
    out
}

impl ZcashWalletCliModule for ZcashWalletCliModuleImpl {
    fn caller_identity(&self) -> String {
        read(modules().zcash_wallet_backend.caller_identity())
    }

    fn list_networks(&self) -> String {
        read(modules().zcash_wallet_backend.list_networks())
    }

    fn set_active_network(&self, network: String) -> String {
        custodian(modules().zcash_wallet_backend.set_active_network(&network))
    }

    fn list_wallets(&self) -> String {
        read(modules().zcash_wallet_backend.list_wallets())
    }

    fn create_wallet(&self, name: String, mut password: String) -> String {
        custodian(with_secret(&mut password, |pw| modules().zcash_wallet_backend.create_wallet(&name, pw)))
    }

    fn restore_wallet(&self, mut params_json: String) -> String {
        custodian(with_secret(&mut params_json, |p| modules().zcash_wallet_backend.restore_wallet(p)))
    }

    fn open_wallet(&self, name: String, mut password: String) -> String {
        custodian(with_secret(&mut password, |pw| modules().zcash_wallet_backend.open_wallet(&name, pw)))
    }

    fn change_password(&self, mut old_password: String, mut new_password: String) -> String {
        let reply = with_secret(&mut old_password, |old| {
            with_secret(&mut new_password, |new| modules().zcash_wallet_backend.change_password(old, new))
        });
        custodian(reply)
    }

    fn close_wallet(&self) -> String {
        custodian(modules().zcash_wallet_backend.close_wallet())
    }

    fn job_status(&self, job_id: String) -> String {
        read(modules().zcash_wallet_backend.job_status(&job_id))
    }

    fn reveal_seed(&self, mut password: String) -> String {
        custodian(with_secret(&mut password, |pw| modules().zcash_wallet_backend.reveal_seed(pw)))
    }

    fn export_viewing_key(&self, mut password: String) -> String {
        custodian(with_secret(&mut password, |pw| modules().zcash_wallet_backend.export_viewing_key(pw)))
    }

    fn wallet_status(&self) -> String {
        read(modules().zcash_wallet_backend.wallet_status())
    }

    fn sync_status(&self) -> String {
        read(modules().zcash_wallet_backend.sync_status())
    }

    fn balances(&self) -> String {
        read(modules().zcash_wallet_backend.balances())
    }

    fn receive_info(&self) -> String {
        read(modules().zcash_wallet_backend.receive_info())
    }

    fn address_new(&self) -> String {
        custodian(modules().zcash_wallet_backend.new_address())
    }

    fn history(&self, page: i64) -> String {
        read(modules().zcash_wallet_backend.history(page))
    }

    fn servers(&self) -> String {
        read(modules().zcash_wallet_backend.servers())
    }

    fn server_health(&self) -> String {
        read(modules().zcash_wallet_backend.server_health())
    }

    fn apply_preset(&self, name: String) -> String {
        custodian(modules().zcash_wallet_backend.apply_preset(&name))
    }

    fn set_proxy(&self, config_json: String) -> String {
        custodian(modules().zcash_wallet_backend.set_proxy(&config_json))
    }

    fn prepare_send(&self, request_json: String) -> String {
        read(modules().zcash_wallet_backend.prepare_send(&request_json))
    }

    fn send_status(&self, request_id: String) -> String {
        read(modules().zcash_wallet_backend.send_status(&request_id))
    }

    fn list_sends(&self) -> String {
        read(modules().zcash_wallet_backend.list_sends())
    }

    fn approve_send(&self, request_id: String, mut password: String) -> String {
        relay_as(with_secret(&mut password, |pw| modules().zcash_wallet_backend.approve_send(&request_id, pw)), "approver")
    }

    fn cancel_send(&self, request_id: String) -> String {
        relay_as(modules().zcash_wallet_backend.cancel_send(&request_id), "approver")
    }

    fn prepare_shielding(&self, address: String) -> String {
        custodian(modules().zcash_wallet_backend.prepare_shielding(&address))
    }
}

#[no_mangle]
pub extern "Rust" fn logos_module_install() {
    logos_install!(ZcashWalletCliModuleImpl);
}
