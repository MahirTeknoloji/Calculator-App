# Derleme aşaması
FROM rust:1.76-slim as builder

WORKDIR /app

# Önce sadece Cargo.toml'u kopyala ve sahte bir main.rs oluştur
# Bu, bağımlılıkların önbelleklenmesini sağlar
COPY Cargo.toml .
RUN mkdir -p src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

# Şimdi gerçek kaynak kodunu kopyala ve derle
COPY src src
# Cargo.lock varsa onu da kopyala
COPY Cargo.lock* .
# Önbelleği temizle ve yeniden derle
RUN touch src/main.rs && cargo build --release

# Çalıştırma aşaması
FROM debian:bullseye-slim

WORKDIR /app

# Derlenmiş binary dosyasını kopyala
COPY --from=builder /app/target/release/Calculator /app/Calculator

# Çalıştırma komutu
CMD ["/app/Calculator"]