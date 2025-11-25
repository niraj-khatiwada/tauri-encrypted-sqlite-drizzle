# Tauri Encrypted SQLite + Drizzle
Using encrypted SQLite database in Tauri + Drizzle as an ORM and migration kit on the frontend.

### Tech Stack
- Rust + Tauri 🦀
- SQLX + Encrypted SQLite with SQLEncypher v3
- Drizzle ORM + Migration with Drizzle Kit
- TanStack Router

### Development
Make sure [Bun](https://bun.sh) is installed.

1. Run Tauri server:
```
bun tauri:dev
```

2. Run client server:
```
cd apps/client;
bun dev
```

### Production
```
bun tauri:build
```