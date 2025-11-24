import { relations, sql } from 'drizzle-orm'
import { sqliteTable, text } from 'drizzle-orm/sqlite-core'
import { v4 as uuid } from 'uuid'

import todo from './todo'

const user = sqliteTable('user', {
  id: text()
    .primaryKey()
    .$defaultFn(() => uuid()),
  name: text('name').notNull(),
  created_at: text('created_at').default(sql`CURRENT_TIMESTAMP`),
  updated_at: text('updated_at').default(sql`CURRENT_TIMESTAMP`),
})

export const userRelations = relations(user, ({ many }) => ({
  todos: many(todo),
}))

export default user
