# 1️⃣ Стадия сборки
FROM rust:1.77 as builder

# Устанавливаем рабочую директорию
WORKDIR /app

# Кэшируем зависимости
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Прогреваем зависимостями
RUN cargo build --release

# 2️⃣ Финальный образ
FROM debian:buster-slim

# Устанавливаем минимумы
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Копируем бинарник из билд-стадии
COPY --from=builder /app/target/release/server /usr/local/bin/server

# Сетап порта
EXPOSE 3001

# Запускаем сервер
CMD ["server"]
