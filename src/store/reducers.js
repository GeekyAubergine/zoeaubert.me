// @flow

import { combineReducers } from 'redux'
import { connectRouter } from 'connected-react-router'

export type State = {

}

// $FlowFixMe
export default (history) => combineReducers({
	router: connectRouter(history),
})
