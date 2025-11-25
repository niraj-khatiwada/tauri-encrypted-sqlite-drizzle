import { Button, Description, Separator, Surface } from '@heroui/react'
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'
import { createFileRoute } from '@tanstack/react-router'
import { eq } from 'drizzle-orm'
import { useRef, useState } from 'react'

import { db } from '~/db'
import { todo } from '~/db/schema'
import { protectRouteBeforeLoad } from '~/utils/router'

export const Route = createFileRoute('/(root)/')({
  component: App,
  beforeLoad: protectRouteBeforeLoad,
})

function App() {
  const queryClient = useQueryClient()

  const inputRef = useRef<HTMLDivElement>(null)

  const { mutate: addToDo } = useMutation({
    mutationFn: (newTodo: string) => {
      return db.insert(todo).values({ title: newTodo })
    },
    onSuccess() {
      queryClient.invalidateQueries({ queryKey: ['todos'] })
    },
  })

  const [input, setInput] = useState('')

  const handleInput = (e: React.FormEvent<HTMLDivElement>) => {
    setInput(e.currentTarget.innerText)
  }
  const handleKeyDown = async (e: React.KeyboardEvent<HTMLDivElement>) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault()
      addToDo(input, {
        onSuccess() {
          if (inputRef.current) {
            inputRef.current.innerText = ''
          }
        },
      })
    }
  }

  return (
    <div className="max-w-xl mx-auto mt-16 flex flex-col gap-2 px-4">
      <p className="opacity-80">Add New ToDo:</p>
      <Surface
        ref={inputRef}
        className="flex flex-col gap-3 rounded-xl p-2"
        variant="secondary"
        suppressContentEditableWarning
        contentEditable
        autoCapitalize="off"
        autoCorrect="off"
        autoFocus
        onInput={handleInput}
        onKeyDown={handleKeyDown}
      />
      <Separator className="my-4" />
      <Todos />
    </div>
  )
}

function Todos() {
  const queryClient = useQueryClient()

  const { data: todos } = useQuery({
    queryKey: ['todos'],
    queryFn: () =>
      db.query.todo.findMany({
        orderBy: (todo, { desc }) => [desc(todo.created_at)],
      }),
  })
  const { mutate: deleteToDo } = useMutation({
    mutationFn: (id: string) => {
      return db.delete(todo).where(eq(todo.id, id))
    },
    onSuccess() {
      queryClient.invalidateQueries({ queryKey: ['todos'] })
    },
  })
  return (
    <div>
      <p className="opacity-80">ToDos:</p>
      <div className="overflow-auto h-[78vh]">
        {todos?.map((todo) => (
          <Surface
            key={todo.id}
            className="rounded-xl p-3 flex justify-between items-center my-2"
            variant="quaternary"
          >
            <p className="whitespace-pre-wrap">{todo.title}</p>
            <Button
              isIconOnly
              variant="ghost"
              className="text-xs"
              onClick={() => {
                deleteToDo(todo.id)
              }}
            >
              🗑
            </Button>
          </Surface>
        ))}
        {todos?.length === 0 ? (
          <Description>No todos yet...</Description>
        ) : null}
      </div>
    </div>
  )
}
