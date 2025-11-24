import { Alert, Button, cn, Input, Label, Spinner } from '@heroui/react'
import { useMutation } from '@tanstack/react-query'
import { createFileRoute, useNavigate } from '@tanstack/react-router'
import { useState } from 'react'

import { preventRouteBeforeLoad } from '~/utils/router'
import { initDb } from '~/utils/tauri-commands'

export const Route = createFileRoute('/(root)/auth')({
  component: RouteComponent,
  beforeLoad: preventRouteBeforeLoad,
})

function RouteComponent() {
  const navigate = useNavigate()
  const [key, setKey] = useState('')

  const { mutate, isPending } = useMutation({
    mutationFn: async (encryptionKey: string) => await initDb(encryptionKey),
    onSettled() {
      navigate({ to: '/' })
    },
  })
  return (
    <div className="flex w-screen h-[80vh] flex-col gap-4 justify-center items-center">
      <form
        className="w-[300px] flex flex-col gap-1"
        onSubmit={(evt) => {
          evt.preventDefault()
          mutate(key)
        }}
      >
        <Label htmlFor="master-password" className="text-2xl mb-4 mx-auto">
          Encryption Key
        </Label>
        <Input
          id="master-password"
          placeholder="Enter your encryption key..."
          type="password"
          disabled={isPending}
          value={key}
          onChange={(e) => setKey(e.target.value)}
        />
        <Button
          size="sm"
          className="w-full my-4 rounded-xl"
          isDisabled={isPending || key.length === 0}
          isPending={isPending}
          type="submit"
        >
          {({ isPending }) => (
            <>
              {isPending ? <Spinner color="current" size="sm" /> : null}
              Unlock{isPending ? 'ing...' : ''}
            </>
          )}
        </Button>
        <Alert
          status="warning"
          className={cn(
            'py-2 px-3 gap-2 items-center',
            key.length === 0 ? 'opacity-0' : 'opacity-90',
          )}
        >
          <Alert.Indicator />
          <Alert.Content>
            <Alert.Title>
              Make sure to remember this key. You'll be asked to enter this when
              required.
            </Alert.Title>
          </Alert.Content>
        </Alert>
      </form>
    </div>
  )
}
