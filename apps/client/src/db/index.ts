import { invoke } from '@tauri-apps/api/core'
import { drizzle } from 'drizzle-orm/sqlite-proxy'

import * as schema from './schema'

type Row = {
  columns: string[]
  rows: string[]
}

type Method = 'run' | 'all' | 'values' | 'get'

export const db = drizzle(
  async (sql, params, method) => {
    const rows = await invoke<Row[]>('execute_single_sql', {
      query: { sql, params },
    })
    /**
     * Response type:
     * {rows: string[]} for 'get'
     * {rows: string[][]} for rest
     *
     * More info: https://orm.drizzle.team/docs/connect-drizzle-proxy
     */
    return mapRows(rows, method)
  },
  async (
    queries: {
      sql: string
      params: any[]
      method: Method
    }[],
  ) => {
    const batchRows = await invoke<Row[][]>('execute_batch_sql', {
      queries,
    })
    /**
     * Response type:
     * {rows: string[]}[] for 'get'
     * {rows: string[][]}[] for rest
     * More info: https://orm.drizzle.team/docs/connect-drizzle-proxy
     */
    return batchRows.map((rows, index) => {
      const query = queries[index]
      return mapRows(rows, query.method)
    })
  },
  {
    schema,
    logger: import.meta.env.DEV,
  },
)

function mapRows(rows: Row[], method: Method) {
  if (rows.length === 0 && method === 'get') {
    return {} as { rows: [] }
  }
  return {
    rows: method === 'get' ? rows[0].rows : rows.map((r) => r.rows),
  }
}
