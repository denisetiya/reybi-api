use std::collections::HashMap;
use std::sync::OnceLock;

static EN: OnceLock<HashMap<&'static str, &'static str>> = OnceLock::new();

fn en_messages() -> &'static HashMap<&'static str, &'static str> {
    EN.get_or_init(|| {
        let mut m = HashMap::new();
        m.insert("VALIDATION_ERROR", "Validation failed");
        m.insert("NOT_FOUND", "Resource not found");
        m.insert("UNAUTHORIZED", "Authentication required");
        m.insert("FORBIDDEN", "Access denied");
        m.insert("CONFLICT", "Resource conflict");
        m.insert("RATE_LIMITED", "Too many requests");
        m.insert("INTERNAL_ERROR", "Internal server error");
        m.insert("FIELD_REQUIRED", "This field is required");
        m.insert("FIELD_EMAIL", "Must be a valid email address");
        m.insert("FIELD_MIN_LENGTH", "Must be at least {min} characters");
        m.insert("FIELD_MAX_LENGTH", "Must be at most {max} characters");
        m.insert("TOKEN_EXPIRED", "Invalid or expired token");
        m.insert("TOKEN_MISSING", "Access token not provided");
        m.insert("REFRESH_MISSING", "Refresh token not provided");
        m.insert("INVALID_KEY", "Invalid server key");
        m.insert("PRODUCT_NOT_FOUND", "Product not found");
        m.insert("ORDER_NOT_FOUND", "Order not found");
        m.insert("REVIEW_NOT_FOUND", "Review not found");
        m.insert("ARTICLE_NOT_FOUND", "Article not found");
        m.insert("BANNER_NOT_FOUND", "Banner not found");
        m.insert("TRASH_NOT_FOUND", "Trash type not found");
        m.insert("LANDFILL_NOT_FOUND", "Landfill not found");
        m.insert("REGISTER_SUCCESS", "Registration successful");
        m.insert("PASSWORD_RESET_SENT", "Password reset link sent");
        m.insert("PAYMENT_PENDING", "Payment is pending");
        m.insert("CART_NOT_FOUND", "Cart item not found");
        m
    })
}

static ID: OnceLock<HashMap<&'static str, &'static str>> = OnceLock::new();

fn id_messages() -> &'static HashMap<&'static str, &'static str> {
    ID.get_or_init(|| {
        let mut m = HashMap::new();
        m.insert("VALIDATION_ERROR", "Validasi gagal");
        m.insert("NOT_FOUND", "Sumber daya tidak ditemukan");
        m.insert("UNAUTHORIZED", "Autentikasi diperlukan");
        m.insert("FORBIDDEN", "Akses ditolak");
        m.insert("CONFLICT", "Konflik sumber daya");
        m.insert("RATE_LIMITED", "Terlalu banyak permintaan");
        m.insert("INTERNAL_ERROR", "Kesalahan server internal");
        m.insert("FIELD_REQUIRED", "Bidang ini wajib diisi");
        m.insert("FIELD_EMAIL", "Harus berupa alamat email yang valid");
        m.insert("FIELD_MIN_LENGTH", "Harus minimal {min} karakter");
        m.insert("FIELD_MAX_LENGTH", "Harus maksimal {max} karakter");
        m.insert("TOKEN_EXPIRED", "Token tidak valid atau kedaluwarsa");
        m.insert("TOKEN_MISSING", "Token akses tidak disediakan");
        m.insert("REFRESH_MISSING", "Refresh token tidak disediakan");
        m.insert("INVALID_KEY", "Kunci server tidak valid");
        m.insert("PRODUCT_NOT_FOUND", "Produk tidak ditemukan");
        m.insert("ORDER_NOT_FOUND", "Pesanan tidak ditemukan");
        m.insert("REVIEW_NOT_FOUND", "Ulasan tidak ditemukan");
        m.insert("ARTICLE_NOT_FOUND", "Artikel tidak ditemukan");
        m.insert("BANNER_NOT_FOUND", "Banner tidak ditemukan");
        m.insert("TRASH_NOT_FOUND", "Jenis sampah tidak ditemukan");
        m.insert("LANDFILL_NOT_FOUND", "TPA tidak ditemukan");
        m.insert("REGISTER_SUCCESS", "Pendaftaran berhasil");
        m.insert("PASSWORD_RESET_SENT", "Tautan reset kata sandi dikirim");
        m.insert("PAYMENT_PENDING", "Pembayaran sedang diproses");
        m.insert("CART_NOT_FOUND", "Item keranjang tidak ditemukan");
        m
    })
}

pub fn t(locale: &str, key: &str) -> String {
    let messages = match locale {
        "id" => id_messages(),
        _ => en_messages(),
    };
    messages.get(key).unwrap_or(&key).to_string()
}
