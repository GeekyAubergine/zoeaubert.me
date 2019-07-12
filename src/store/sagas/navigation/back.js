// @flow

import { call, put } from 'redux-saga/effects'
import { goBack } from 'connected-react-router'

import type { Saga } from 'redux-saga'

export default function* (): Saga<void> {
	yield put(goBack())
}
