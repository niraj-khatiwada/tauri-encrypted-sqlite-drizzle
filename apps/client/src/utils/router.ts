import { BeforeLoadContextParameter, redirect } from '@tanstack/react-router'

import { isDbReady } from './tauri-commands'

/**
 * Protect routes access when there's no auth
 */
export async function protectRouteBeforeLoad(
  params: BeforeLoadContextParameter<any, any, any>,
) {
  try {
    const isAuthenticated = await isDbReady()
    if (!isAuthenticated) {
      throw new Error()
    }
  } catch {
    throw redirect({
      to: '/auth',
      search: { redirectTo: params.location.href },
    })
  }
}

/**
 * Prevent route access when there's auth
 */
export async function preventRouteBeforeLoad(
  _: BeforeLoadContextParameter<any, any, any>,
) {
  let isAuthenticated: Awaited<ReturnType<typeof isDbReady>> | null = null
  try {
    isAuthenticated = await isDbReady()
  } catch {}

  if (isAuthenticated) {
    throw redirect({
      to: '/',
    })
  }
}
