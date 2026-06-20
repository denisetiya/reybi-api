//! Redis cache layer with explicit invalidation.
//!
//! Two-tier: callers use [`Cache::get_or_load`] for "fetch + cache on miss",
//! and call [`Cache::invalidate`] / [`Cache::invalidate_pattern`] after any
//! write to drop stale entries.  All operations are best-effort — if Redis
//! is down we log and fall through to the DB so the API stays up.

use redis::{aio::ConnectionManager, AsyncCommands, Client};
use serde::{de::DeserializeOwned, Serialize};
use std::time::Duration;
use tracing::{debug, warn};

#[derive(Clone)]
pub struct Cache {
    inner: Option<ConnectionManager>,
}

impl Cache {
    pub async fn connect(url: &str) -> Self {
        if url.is_empty() {
            warn!("REDIS_URL not set — cache disabled (DB reads every request)");
            return Self { inner: None };
        }
        match Client::open(url) {
            Ok(client) => match ConnectionManager::new(client).await {
                Ok(cm) => {
                    tracing::info!("✓ redis cache connected");
                    Self { inner: Some(cm) }
                }
                Err(e) => {
                    warn!(error = %e, "redis connect failed — cache disabled");
                    Self { inner: None }
                }
            },
            Err(e) => {
                warn!(error = %e, "invalid REDIS_URL — cache disabled");
                Self { inner: None }
            }
        }
    }

    /// Read `key`.  Returns `None` on miss OR on any Redis error.
    pub async fn get_raw(&self, key: &str) -> Option<String> {
        let mut conn = self.inner.as_ref()?.clone();
        match conn.get::<_, Option<String>>(key).await {
            Ok(v) => v,
            Err(e) => {
                debug!(error = %e, key, "redis get failed");
                None
            }
        }
    }

    /// SETEX (set + expiry).  Best-effort.
    pub async fn set_ex(&self, key: &str, value: &str, ttl: Duration) {
        let Some(mut conn) = self.inner.as_ref().map(|c| c.clone()) else {
            return;
        };
        let ttl_secs = ttl.as_secs() as u64;
        if let Err(e) = conn.set_ex::<_, _, ()>(key, value, ttl_secs).await {
            debug!(error = %e, key, "redis set_ex failed");
        }
    }

    /// DEL single key.
    pub async fn invalidate(&self, key: &str) {
        let Some(mut conn) = self.inner.as_ref().map(|c| c.clone()) else {
            return;
        };
        let n: u64 = match conn.del(key).await {
            Ok(n) => n,
            Err(e) => {
                warn!(error = %e, key, "redis del failed");
                return;
            }
        };
        if n > 0 {
            debug!(key, "✓ cache invalidated");
        }
    }

    /// DEL all keys matching `pattern` (uses SCAN, safe for prod).
    /// Used by write endpoints to drop collection caches.
    pub async fn invalidate_pattern(&self, pattern: &str) {
        let Some(mut conn) = self.inner.as_ref().map(|c| c.clone()) else {
            return;
        };

        // SCAN cursor-based iteration — never blocks Redis like KEYS would.
        let mut cursor: u64 = 0;
        let mut total = 0u64;
        loop {
            let (next, batch): (u64, Vec<String>) = match redis::cmd("SCAN")
                .arg(cursor)
                .arg("MATCH")
                .arg(pattern)
                .arg("COUNT")
                .arg(100)
                .query_async(&mut conn)
                .await
            {
                Ok(v) => v,
                Err(e) => {
                    warn!(error = %e, pattern, "redis scan failed");
                    return;
                }
            };
            if !batch.is_empty() {
                let n: u64 = match conn.del(&batch).await {
                    Ok(n) => n,
                    Err(e) => {
                        warn!(error = %e, pattern, "redis batch del failed");
                        return;
                    }
                };
                total += n;
            }
            cursor = next;
            if cursor == 0 {
                break;
            }
        }
        if total > 0 {
            debug!(pattern, count = total, "✓ cache pattern invalidated");
        }
    }

    /// High-level: GET → deserialize → if miss, run loader, SETEX result.
    pub async fn get_or_load<T, F, Fut>(
        &self,
        key: &str,
        ttl: Duration,
        loader: F,
    ) -> Result<T, crate::errors::AppError>
    where
        T: Serialize + DeserializeOwned + Clone,
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<T, crate::errors::AppError>>,
    {
        if let Some(raw) = self.get_raw(key).await {
            if let Ok(v) = serde_json::from_str::<T>(&raw) {
                debug!(key, "✓ cache hit");
                return Ok(v);
            }
        }
        let v = loader().await?;
        if let Ok(s) = serde_json::to_string(&v) {
            self.set_ex(key, &s, ttl).await;
        }
        Ok(v)
    }
}

/// Centralized cache-key namespace.  Use these helpers so invalidation
/// patterns stay in sync with the keys writers actually use.
pub mod keys {
    pub fn product_list(category: Option<&str>, page: u32) -> String {
        format!(
            "reybi:v1:products:list:{}:p{}",
            category.unwrap_or("all"),
            page
        )
    }
    pub fn product(id: &str) -> String {
        format!("reybi:v1:products:item:{id}")
    }
    pub fn products_pattern() -> &'static str {
        "reybi:v1:products:*"
    }

    pub fn banner_list() -> String {
        "reybi:v1:banners:list".to_string()
    }
    pub fn banner(id: &str) -> String {
        format!("reybi:v1:banners:item:{id}")
    }
    pub fn banners_pattern() -> &'static str {
        "reybi:v1:banners:*"
    }

    pub fn article_list() -> String {
        "reybi:v1:articles:list".to_string()
    }
    pub fn article(id: &str) -> String {
        format!("reybi:v1:articles:item:{id}")
    }
    pub fn articles_pattern() -> &'static str {
        "reybi:v1:articles:*"
    }

    pub fn trash_list() -> String {
        "reybi:v1:trash:list".to_string()
    }
    pub fn trash(id: &str) -> String {
        format!("reybi:v1:trash:item:{id}")
    }
    pub fn trash_pattern() -> &'static str {
        "reybi:v1:trash:*"
    }

    pub fn landfill_list() -> String {
        "reybi:v1:landfills:list".to_string()
    }
    pub fn landfill(id: &str) -> String {
        format!("reybi:v1:landfills:item:{id}")
    }
    pub fn landfills_pattern() -> &'static str {
        "reybi:v1:landfills:*"
    }

    pub fn profile(email: &str) -> String {
        format!("reybi:v1:profile:email:{email}")
    }
    pub fn profile_pattern() -> &'static str {
        "reybi:v1:profile:*"
    }

    pub fn deposite_list(scope: &str) -> String {
        format!("reybi:v1:deposites:list:{scope}")
    }
    pub fn deposite(id: &str) -> String {
        format!("reybi:v1:deposites:item:{id}")
    }
    pub fn deposites_pattern() -> &'static str {
        "reybi:v1:deposites:*"
    }

    pub fn cart_list(user_id: &str) -> String {
        format!("reybi:v1:carts:list:{user_id}")
    }
    pub fn cart(id: &str) -> String {
        format!("reybi:v1:carts:item:{id}")
    }
    pub fn carts_pattern() -> &'static str {
        "reybi:v1:carts:*"
    }

    pub fn address_list(user_id: &str) -> String {
        format!("reybi:v1:addresses:list:{user_id}")
    }
    pub fn address(user_id: &str) -> String {
        format!("reybi:v1:addresses:item:{user_id}")
    }
    pub fn addresses_pattern() -> &'static str {
        "reybi:v1:addresses:*"
    }

    pub fn order_list(scope: &str) -> String {
        format!("reybi:v1:orders:list:{scope}")
    }
    pub fn order(id: &str) -> String {
        format!("reybi:v1:orders:item:{id}")
    }
    pub fn orders_pattern() -> &'static str {
        "reybi:v1:orders:*"
    }

    pub fn saller_products(saller_id: &str) -> String {
        format!("reybi:v1:saller:products:{saller_id}")
    }
    pub fn saller_pattern() -> &'static str {
        "reybi:v1:saller:*"
    }

    pub fn review(id: &str) -> String {
        format!("reybi:v1:reviews:item:{id}")
    }
    pub fn reviews_pattern() -> &'static str {
        "reybi:v1:reviews:*"
    }

    /// Idempotency key for webhook handlers (24h TTL, set after processing).
    pub fn webhook_idem(transaction_id: &str) -> String {
        format!("reybi:v1:webhook:idem:{transaction_id}")
    }
}