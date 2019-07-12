// @flow

import { createStore, applyMiddleware, compose } from 'redux'
import { createBrowserHistory } from 'history'
import { routerMiddleware } from 'connected-react-router'
import createSagaMiddleware from 'redux-saga'

import reducers from './reducers'
// import sagas from './sagas'

import type { State } from './reducers'
import type { Action } from './actions'

export type Store = State
export type Dispatch = (action: Action) => void

export const history = createBrowserHistory()
const sagaMiddleware = createSagaMiddleware()

const store = createStore(
	reducers(history),
	compose(
		applyMiddleware(
			routerMiddleware(history),
			sagaMiddleware
		),
	),
)

// sagaMiddleware.run(sagas)

export default store
