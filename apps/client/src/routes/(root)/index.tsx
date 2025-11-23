import { useQuery } from '@tanstack/react-query'
import { createFileRoute } from '@tanstack/react-router'
import { eq } from 'drizzle-orm'

import { db } from '~/db'
import { todo } from '~/db/schema'
import user from '~/db/schema/user'
import Logo from '../../logo.svg'

export const Route = createFileRoute('/(root)/')({
  component: App,
})

function App() {
  const { data } = useQuery({
    queryKey: ['users'],
    queryFn: () =>
      db
        .select()
        .from(todo)
        .innerJoin(user, eq(todo.user_id, user.id))
        .limit(1)
        .get(),
  })
  return (
    <>
      <div className="h-screen w-screen flex items-center justify-center flex-col">
        <img src={Logo} alt="logo" width={200} />
        <h1 className="text-white text-3xl font-bold">Tauri + SQLite</h1>
        <pre className="text-white">
          <code>{JSON.stringify(data, null, 2)}</code>
        </pre>
        <button
          type="button"
          className="border px-2 text-white"
          onClick={async () => {
            const newUser = await db
              .insert(user)
              .values({ name: 'niraj' })
              .returning()
            console.log('newUser', newUser)
            db.batch([
              db
                .insert(todo)
                .values({ title: 'Niraj', user_id: newUser[0].id })
                .returning(),
            ])
              .then((res) => {
                console.log('res>>', res)
              })
              .catch((err) => {
                console.error(err)
                console.dir(err)
              })
          }}
        >
          Seed
        </button>
      </div>
    </>
  )
}
