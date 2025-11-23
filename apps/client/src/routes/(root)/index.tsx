import { useQuery } from '@tanstack/react-query'
import { createFileRoute } from '@tanstack/react-router'
import { eq } from 'drizzle-orm'

import { db } from '~/db'
import { todo } from '~/db/schema'
import Logo from '../../logo.svg'

export const Route = createFileRoute('/(root)/')({
  component: App,
})

function App() {
  const { data } = useQuery({
    queryKey: ['users'],
    queryFn: () => db.query.todo.findMany(),
  })

  return (
    <>
      <div className="h-screen w-screen flex items-center justify-center flex-col">
        <img src={Logo} alt="logo" width={200} />
        <h1 className="text-white text-3xl font-bold">Tauri + SQLite</h1>
        <button
          type="button"
          className="border px-2 text-white"
          onClick={async () => {
            const res = await db
              .insert(todo)
              .values({ title: 'Lol' })
              .returning()
            console.log('res', res)
            db.batch([
              db.insert(todo).values({ title: 'Niraj' }).returning(),
              db.insert(todo).values({ title: 'Hello' }).returning(),
              db.query.todo.findFirst({ where: eq(todo.title, 'Niraj') }),
              db.insert(todo).values({ title: 'Hello' }).returning(),
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
