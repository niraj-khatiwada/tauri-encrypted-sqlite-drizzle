import { invoke } from '@tauri-apps/api/core'
import { drizzle } from 'drizzle-orm/sqlite-proxy'

import * as schema from './schema'

type Row = {
  columns: string[]
  rows: string[]
}

export const db = drizzle(
  async (sql, params, method) => {
    try {
      const rows = await invoke<Row[]>('execute_single_sql', {
        query: { sql, params },
      })
      if (rows.length === 0 && method === 'get') {
        return {} as { rows: string[] }
      }
      return method === 'get'
        ? { rows: rows[0].rows }
        : { rows: rows.map((r) => r.rows) }
    } catch (e: unknown) {
      // biome-ignore lint/suspicious/noConsole: <>
      console.error('[drizzle] SQLite proxy error ', e)
      return { rows: [] }
    }
  },
  {
    schema,
    logger: import.meta.env.DEV,
  },
)
