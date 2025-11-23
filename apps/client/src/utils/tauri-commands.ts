import { core } from '@tauri-apps/api'

export async function is_db_connected(): Promise<boolean> {
  return await core.invoke<Promise<boolean>>('is_db_connected')
}
