# Tauri Encrypted SQLite + Drizzle
Using encrypted SQLite database in Tauri with Drizzle as an ORM and migration kit from the frontend.

Read the full implementation detail in the [blogpost](https://codeforreal.com/setup-encrypted-sqlitedb-in-tauri-with-drizzle-orm).

<img src="/assets/flowpart.png" style="width: 100%; object-fit: content;" />

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

### Migrations
1. Generate a new migration file when you change the db schema:
```
bun drizzle-kit generate
```
Migrations will automatically be applied when you run the app.