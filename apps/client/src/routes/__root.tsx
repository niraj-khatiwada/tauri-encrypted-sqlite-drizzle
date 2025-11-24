import { Alert, AlertDialog, Button, Spinner } from '@heroui/react'
import { useMutation, useQuery } from '@tanstack/react-query'
import {
  createRootRoute,
  Outlet,
  useLocation,
  useNavigate,
} from '@tanstack/react-router'
import { TanStackRouterDevtools } from '@tanstack/react-router-devtools'

import { QueryClientProvider } from '~/provider/QueryClientProvider'
import { isDbReady, resetDb } from '~/utils/tauri-commands'

export const Route = createRootRoute({
  component: RootComponent,
})

const isDev = import.meta.env.DEV

function RootComponent() {
  return (
    <>
      <QueryClientProvider>
        <Outlet />
        <Reset />
      </QueryClientProvider>
      {isDev ? <TanStackRouterDevtools position="bottom-right" /> : null}
    </>
  )
}

function Reset() {
  const navigate = useNavigate()
  const { pathname } = useLocation()

  const { data: isAuthenticated, refetch } = useQuery({
    queryKey: ['isDbReady', pathname],
    queryFn: isDbReady,
  })

  const { mutate, isPending, error } = useMutation({
    mutationFn: resetDb,
    async onSettled() {
      await refetch()
      navigate({ to: '/auth' })
    },
  })
  return (
    <div className="fixed top-4 left-4 flex gap-2">
      <AlertDialog>
        <Button size="sm" variant="danger-soft">
          Reset
        </Button>
        <AlertDialog.Container>
          <AlertDialog.Dialog className="sm:max-w-[400px]">
            {({ close }) => (
              <>
                <AlertDialog.Header>
                  <AlertDialog.Icon status="danger" />
                  <AlertDialog.Heading>Reset Database?</AlertDialog.Heading>
                </AlertDialog.Header>
                <AlertDialog.Body>
                  <p>
                    This will permanently delete your database.
                    <br />
                    This action can't be undone.
                  </p>
                </AlertDialog.Body>
                <AlertDialog.Footer>
                  <Button size="sm" variant="tertiary" onPress={close}>
                    Cancel
                  </Button>
                  <Button
                    size="sm"
                    variant="danger"
                    onPress={() => {
                      mutate(true, {
                        onSuccess: () => {
                          close()
                        },
                      })
                    }}
                    isDisabled={isPending}
                    isPending={isPending}
                  >
                    {({ isPending }) => (
                      <>
                        {isPending ? (
                          <Spinner color="current" size="sm" />
                        ) : null}
                        Reset{isPending ? 'ing...' : ''}
                      </>
                    )}
                  </Button>
                </AlertDialog.Footer>
                {error ? (
                  <AlertDialog.Footer>
                    <Alert status="danger" className="my-2">
                      <Alert.Indicator />
                      <Alert.Content>
                        <Alert.Description>
                          {error?.message ?? JSON.stringify(error ?? '')}
                        </Alert.Description>
                      </Alert.Content>
                    </Alert>
                  </AlertDialog.Footer>
                ) : null}
              </>
            )}
          </AlertDialog.Dialog>
        </AlertDialog.Container>
      </AlertDialog>
      {isAuthenticated ? (
        <AlertDialog>
          <Button size="sm" variant="secondary">
            Logout
          </Button>
          <AlertDialog.Container>
            <AlertDialog.Dialog className="sm:max-w-[400px]">
              {({ close }) => (
                <>
                  <AlertDialog.Header>
                    <AlertDialog.Icon status="accent" />
                    <AlertDialog.Heading>Logout?</AlertDialog.Heading>
                  </AlertDialog.Header>
                  <AlertDialog.Body>
                    <p>
                      This will log you out from this session.
                      <br />
                      Don't worry, this won't delete your database.
                    </p>
                  </AlertDialog.Body>
                  <AlertDialog.Footer>
                    <Button size="sm" variant="tertiary" onPress={close}>
                      Cancel
                    </Button>
                    <Button
                      size="sm"
                      variant="secondary"
                      onPress={() => {
                        mutate(false, {
                          onSuccess: () => {
                            close()
                          },
                        })
                      }}
                      isDisabled={isPending}
                      isPending={isPending}
                    >
                      {({ isPending }) => (
                        <>
                          {isPending ? (
                            <Spinner color="current" size="sm" />
                          ) : null}
                          {isPending ? 'Logging out...' : 'Logout'}
                        </>
                      )}
                    </Button>
                  </AlertDialog.Footer>
                  {error ? (
                    <AlertDialog.Footer>
                      <Alert status="accent" className="my-2">
                        <Alert.Indicator />
                        <Alert.Content>
                          <Alert.Description>
                            {error?.message ?? JSON.stringify(error ?? '')}
                          </Alert.Description>
                        </Alert.Content>
                      </Alert>
                    </AlertDialog.Footer>
                  ) : null}
                </>
              )}
            </AlertDialog.Dialog>
          </AlertDialog.Container>
        </AlertDialog>
      ) : null}
    </div>
  )
}
