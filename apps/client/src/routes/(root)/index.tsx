import { Button } from '@heroui/react'
import { useQuery } from '@tanstack/react-query'
import { createFileRoute } from '@tanstack/react-router'
import { eq } from 'drizzle-orm'
import { v4 as uuid } from 'uuid'

import { db } from '~/db'
import { todo } from '~/db/schema'
import user from '~/db/schema/user'
import { protectRouteBeforeLoad } from '~/utils/router'
import Logo from '../../logo.svg'

export const Route = createFileRoute('/(root)/')({
  component: App,
  beforeLoad: protectRouteBeforeLoad,
})

function App() {
  const { data } = useQuery({
    queryKey: ['users'],
    queryFn: async () =>
      db.select().from(todo).leftJoin(user, eq(todo.user_id, user.id)),
  })

  return (
    <>
      <div className="h-screen w-screen flex items-center justify-center flex-col">
        <img src={Logo} alt="logo" width={200} />
        <h1 className="text-white text-3xl font-bold">Tauri + SQLite</h1>
        <pre className="text-white">
          <code>{JSON.stringify(data, null, 2)}</code>
        </pre>
        <Button
          type="button"
          onClick={async () => {
            // const newUser = await db
            //   .insert(user)
            //   .values({ name: 'niraj' })
            //   .returning()
            // console.log('newUser', newUser)
            // db.batch([
            //   db
            //     .insert(todo)
            //     .values({ title: 'Niraj', user_id: newUser[0].id })
            //     .returning(),
            // ])
            //   .then((res) => {
            //     console.log('res>>', res)
            //   })
            //   .catch((err) => {
            //     console.error(err)
            //     console.dir(err)
            //   })
            for (const _ in new Array(1000).fill(0)) {
              const userId = uuid()
              await db.batch([
                db
                  .insert(user)
                  .values({ id: userId, name: 'Rust' })
                  .returning(),
                db
                  .insert(todo)
                  .values({ title: 'Hello Rust', user_id: userId }),
              ])
            }
          }}
        >
          Seed
        </Button>
      </div>
    </>
  )
}
