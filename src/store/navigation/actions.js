// @flow

import type { A } from '../actions'

export type NavigateBack = A<'NAVIGATE_BACK', {}>

export type Navigate = A<'NAVIGATE', { path: string }>
