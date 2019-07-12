// @flow

import type { Actions as NavigationActions } from './navigation/actions'

export type A<T, D> = { type: T } & D

export type Action =
	| NavigationActions
