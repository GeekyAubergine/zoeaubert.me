// @flow

import React from 'react'
import ReactDom from 'react-dom'
import { Provider } from 'react-redux'
import { library } from '@fortawesome/fontawesome-svg-core'
import { faSpinner } from '@fortawesome/free-solid-svg-icons'

import store from './store'
import './core.scss';

import App from './App'

library.add(faSpinner)

ReactDom.render(
	<Provider store={store}>
		<App />
	</Provider>,
	// $FlowFixMe
	document.querySelector('#app')
)
