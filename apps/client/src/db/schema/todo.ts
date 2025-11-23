import { relations, sql } from 'drizzle-orm'
import { sqliteTable, text } from 'drizzle-orm/sqlite-core'
import { v4 as uuid } from 'uuid'

import user from './user'

const todo = sqliteTable('todo', {
  id: text()
    .primaryKey()
    .$defaultFn(() => uuid()),
  title: text('title').notNull(),
  description: text('description'),
  created_at: text('created_at').default(sql`CURRENT_TIMESTAMP`),
  updated_at: text('updated_at').default(sql`CURRENT_TIMESTAMP`),

  user_id: text('user_id').references(() => user.id),
})

export const todoRelations = relations(todo, ({ one }) => ({
  user: one(user, {
    fields: [todo.user_id],
    references: [user.id],
  }),
}))

export default todo
