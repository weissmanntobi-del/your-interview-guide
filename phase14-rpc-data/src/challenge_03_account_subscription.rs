/// # Challenge 03: Account Subscription (WebSocket PubSub)
///
/// Solana validators expose a WebSocket interface for real-time account updates.
/// Clients subscribe to specific accounts at a chosen commitment level and receive
/// notifications when state changes. This challenge implements the subscription
/// manager that tracks subscriptions, filters updates by commitment level, deduplicates
/// subscriptions, and dispatches notifications.
///
/// Key concepts:
/// - Commitment levels: Processed < Confirmed < Finalized
/// - Commitment filtering: a Finalized update satisfies all commitment levels
/// - Subscription deduplication: same (pubkey, commitment) returns existing id
/// - Fan-out notifications to multiple subscribers

/// Commitment level for a subscription or update.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Commitment {
    Processed,
    Confirmed,
    Finalized,
}

/// A client subscription to an account's state changes.
#[derive(Debug, Clone, PartialEq)]
pub struct Subscription {
    pub id: u64,
    pub pubkey: [u8; 32],
    pub commitment: Commitment,
}

/// An account state update received from the validator.
#[derive(Debug, Clone, PartialEq)]
pub struct AccountUpdate {
    pub pubkey: [u8; 32],
    pub slot: u64,
    pub lamports: u64,
    pub data: Vec<u8>,
    pub commitment: Commitment,
}

/// A notification dispatched to a subscriber.
#[derive(Debug, Clone, PartialEq)]
pub struct Notification {
    pub subscription_id: u64,
    pub pubkey: [u8; 32],
    pub slot: u64,
    pub lamports: u64,
    pub data: Vec<u8>,
}

/// Manages account subscriptions and dispatches update notifications.
pub struct SubscriptionManager {
    subscriptions: Vec<Subscription>,
    next_id: u64,
}

impl SubscriptionManager {
    /// Create a new empty SubscriptionManager.
    pub fn new() -> Self {
        todo!("Initialize with empty subscriptions vec and next_id = 1")
    }

    /// Subscribe to updates for the given pubkey at the specified commitment level.
    ///
    /// If an identical (pubkey, commitment) subscription already exists, return
    /// the existing subscription id instead of creating a duplicate.
    /// Otherwise, allocate a new id, store the subscription, and return the id.
    pub fn subscribe(&mut self, pubkey: [u8; 32], commitment: Commitment) -> u64 {
        todo!("Check for existing subscription with same pubkey+commitment. If found return its id. Otherwise create new Subscription with next_id, push it, increment next_id, return id.")
    }

    /// Remove a subscription by its id. Returns true if it was found and removed.
    pub fn unsubscribe(&mut self, id: u64) -> bool {
        todo!("Find subscription with matching id, remove it, return true. Return false if not found.")
    }

    /// Process an account update and return notifications for all matching subscribers.
    ///
    /// Commitment filtering rules:
    /// - A Finalized update notifies subscribers at ALL commitment levels
    /// - A Confirmed update notifies Processed and Confirmed subscribers
    /// - A Processed update notifies only Processed subscribers
    pub fn notify_update(&self, update: &AccountUpdate) -> Vec<Notification> {
        todo!("Filter subscriptions: match on pubkey AND commitment level. A sub is notified if its commitment is satisfied by the update's commitment. Build and return Notification for each matching sub.")
    }

    /// Return the count of active subscriptions.
    pub fn subscription_count(&self) -> usize {
        todo!("Return the length of the subscriptions vector")
    }

    /// Return all subscription IDs for a given pubkey (any commitment level).
    pub fn subscriptions_for_pubkey(&self, pubkey: &[u8; 32]) -> Vec<u64> {
        todo!("Filter subscriptions by pubkey match and collect their ids")
    }
}

/// Determine whether an update at `update_commitment` satisfies a subscription
/// at `sub_commitment`.
fn commitment_satisfies(update_commitment: Commitment, sub_commitment: Commitment) -> bool {
    todo!("Finalized satisfies all. Confirmed satisfies Processed and Confirmed. Processed satisfies only Processed.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subscribe_returns_id() {
        let mut mgr = SubscriptionManager::new();
        let id = mgr.subscribe([1u8; 32], Commitment::Confirmed);
        assert!(id > 0);
    }

    #[test]
    fn test_dedup_same_pubkey_commitment() {
        let mut mgr = SubscriptionManager::new();
        let id1 = mgr.subscribe([1u8; 32], Commitment::Confirmed);
        let id2 = mgr.subscribe([1u8; 32], Commitment::Confirmed);
        assert_eq!(id1, id2);
        assert_eq!(mgr.subscription_count(), 1);
    }

    #[test]
    fn test_different_commitment_separate_subs() {
        let mut mgr = SubscriptionManager::new();
        let id1 = mgr.subscribe([1u8; 32], Commitment::Processed);
        let id2 = mgr.subscribe([1u8; 32], Commitment::Finalized);
        assert_ne!(id1, id2);
        assert_eq!(mgr.subscription_count(), 2);
    }

    #[test]
    fn test_unsubscribe() {
        let mut mgr = SubscriptionManager::new();
        let id = mgr.subscribe([1u8; 32], Commitment::Confirmed);
        assert!(mgr.unsubscribe(id));
        assert_eq!(mgr.subscription_count(), 0);
        assert!(!mgr.unsubscribe(id)); // already removed
    }

    #[test]
    fn test_notify_processed_only_processed() {
        let mut mgr = SubscriptionManager::new();
        let _p = mgr.subscribe([1u8; 32], Commitment::Processed);
        let _c = mgr.subscribe([1u8; 32], Commitment::Confirmed);
        let update = AccountUpdate {
            pubkey: [1u8; 32],
            slot: 10,
            lamports: 500,
            data: vec![],
            commitment: Commitment::Processed,
        };
        let notifs = mgr.notify_update(&update);
        assert_eq!(notifs.len(), 1); // only Processed subscriber
    }

    #[test]
    fn test_notify_finalized_all_levels() {
        let mut mgr = SubscriptionManager::new();
        mgr.subscribe([1u8; 32], Commitment::Processed);
        mgr.subscribe([1u8; 32], Commitment::Confirmed);
        mgr.subscribe([1u8; 32], Commitment::Finalized);
        let update = AccountUpdate {
            pubkey: [1u8; 32],
            slot: 10,
            lamports: 500,
            data: vec![],
            commitment: Commitment::Finalized,
        };
        let notifs = mgr.notify_update(&update);
        assert_eq!(notifs.len(), 3);
    }

    #[test]
    fn test_notify_confirmed_processed_and_confirmed() {
        let mut mgr = SubscriptionManager::new();
        mgr.subscribe([1u8; 32], Commitment::Processed);
        mgr.subscribe([1u8; 32], Commitment::Confirmed);
        mgr.subscribe([1u8; 32], Commitment::Finalized);
        let update = AccountUpdate {
            pubkey: [1u8; 32],
            slot: 10,
            lamports: 500,
            data: vec![],
            commitment: Commitment::Confirmed,
        };
        let notifs = mgr.notify_update(&update);
        assert_eq!(notifs.len(), 2); // Processed + Confirmed
    }

    #[test]
    fn test_notify_different_pubkey_no_match() {
        let mut mgr = SubscriptionManager::new();
        mgr.subscribe([1u8; 32], Commitment::Finalized);
        let update = AccountUpdate {
            pubkey: [2u8; 32],
            slot: 10,
            lamports: 500,
            data: vec![],
            commitment: Commitment::Finalized,
        };
        let notifs = mgr.notify_update(&update);
        assert_eq!(notifs.len(), 0);
    }

    #[test]
    fn test_subscriptions_for_pubkey() {
        let mut mgr = SubscriptionManager::new();
        mgr.subscribe([1u8; 32], Commitment::Processed);
        mgr.subscribe([1u8; 32], Commitment::Finalized);
        mgr.subscribe([2u8; 32], Commitment::Confirmed);
        let ids = mgr.subscriptions_for_pubkey(&[1u8; 32]);
        assert_eq!(ids.len(), 2);
    }

    #[test]
    fn test_commitment_satisfies_logic() {
        assert!(commitment_satisfies(Commitment::Finalized, Commitment::Processed));
        assert!(commitment_satisfies(Commitment::Finalized, Commitment::Confirmed));
        assert!(commitment_satisfies(Commitment::Finalized, Commitment::Finalized));
        assert!(commitment_satisfies(Commitment::Confirmed, Commitment::Processed));
        assert!(commitment_satisfies(Commitment::Confirmed, Commitment::Confirmed));
        assert!(!commitment_satisfies(Commitment::Confirmed, Commitment::Finalized));
        assert!(commitment_satisfies(Commitment::Processed, Commitment::Processed));
        assert!(!commitment_satisfies(Commitment::Processed, Commitment::Confirmed));
    }
}
