import { core } from '@tauri-apps/api'

export async function isDbReady() {
  return await core.invoke<Promise<boolean>>('is_db_ready')
}

export async function initDb(encryptionKey: string) {
  return await core.invoke<Promise<void>>('init_db', { encryptionKey })
}

export async function resetDb(purgeData: boolean) {
  return await core.invoke<Promise<void>>('reset_db', {
    purgeData,
  })
}
