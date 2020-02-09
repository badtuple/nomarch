import React from 'react'
import ReactDOM from 'react-dom'
import { Switch, Route, BrowserRouter } from 'react-router-dom'
import App from './App.js'

ReactDOM.render(
  <BrowserRouter>
    <Switch>
      <Route component={App} />
    </Switch>
  </BrowserRouter>
  , document.getElementById('root')
)
