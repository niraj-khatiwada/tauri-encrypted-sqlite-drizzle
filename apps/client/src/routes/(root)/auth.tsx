import { Alert, Button, cn, Input, Label, Spinner } from '@heroui/react'
import { useMutation, useQuery } from '@tanstack/react-query'
import { createFileRoute, useNavigate } from '@tanstack/react-router'
import { useState } from 'react'

import { preventRouteBeforeLoad } from '~/utils/router'
import {
  doesDbExist as doesDbExistCommand,
  initDb,
} from '~/utils/tauri-commands'
import Logo from '../../logo.svg'

export const Route = createFileRoute('/(root)/auth')({
  component: RouteComponent,
  beforeLoad: preventRouteBeforeLoad,
})

function RouteComponent() {
  const navigate = useNavigate()
  const [key, setKey] = useState('')

  const { data: doesDbExist, refetch } = useQuery({
    queryKey: ['doesDbExist'],
    queryFn: doesDbExistCommand,
  })

  const { mutate, isPending, error, reset } = useMutation({
    mutationFn: async (encryptionKey: string) => await initDb(encryptionKey),
    async onSettled() {
      await refetch()
      navigate({ to: '/' })
    },
  })
  return (
    <div className="flex w-screen h-[80vh] flex-col gap-4 justify-center items-center">
      <img src={Logo} alt="logo" width={200} />
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
          placeholder={`${doesDbExist ? 'Enter your' : 'Set a new'} encryption key...`}
          type="password"
          disabled={isPending}
          value={key}
          onChange={(e) => {
            reset()
            setKey(e.target.value)
          }}
        />
        <Button
          size="sm"
          className={cn(
            'w-full my-4 rounded-xl',
            !doesDbExist ? 'bg-success' : 'bg-blue-500',
          )}
          isDisabled={isPending || key.length === 0}
          isPending={isPending}
          type="submit"
          variant="ghost"
        >
          {({ isPending }) => (
            <>
              {isPending ? <Spinner color="current" size="sm" /> : null}
              {doesDbExist ? 'Unlock' : 'Register'}
              {isPending ? 'ing...' : ''}
            </>
          )}
        </Button>
        {!doesDbExist ? (
          <Alert
            status="warning"
            className={cn(
              'py-2 px-3 gap-2 items-center',
              key.length === 0 ? 'opacity-0' : 'opacity-90',
            )}
          >
            <Alert.Indicator />
            <Alert.Content>
              <Alert.Title>Make sure to remember this key.</Alert.Title>
            </Alert.Content>
          </Alert>
        ) : null}

        {error ? (
          <Alert status="danger" className="my-2">
            <Alert.Indicator />
            <Alert.Content>
              <Alert.Title>
                Unable to connect to connect to the database.
              </Alert.Title>
              <Alert.Description>
                {error.message ?? JSON.stringify(error ?? '')}
              </Alert.Description>
              <Alert.Description className="my-2 text-danger">
                If you forgot the encryption key, you need to reset the
                database.
              </Alert.Description>
            </Alert.Content>
          </Alert>
        ) : null}
      </form>
    </div>
  )
}
