import React, { Component } from 'react'
import Service from '$components/service/service.js'
import '$assets/styles/theme.scss'
import data from './data.js'

export default class App extends Component {
  constructor(props) {
    super(props)

    const pipelines = data.pipelines.map(p => {
      const serviceTable = p.services.reduce(
        (obj, step) => {
          obj[step.name] = step
          return obj
        }, {})

      const getNode = (node) => ({
        name: node,
        children: serviceTable[node].children.map(name => getNode(name)),
        stats: serviceTable[node].stats
      })

      let rootNode = getNode(p.root)
      return {...p, services: rootNode}
    })


    this.state = {
      pipelines
    }
  }

  componentDidMount() {
    this.fetchData()
  }

  fetchData() {
    fetch("http://127.0.0.1:8080/pipelines")
      .then(response => response.json())
      .then(data => console.log(data))
      .catch(error => console.log(error))
  }

  render () {
    let { pipelines } = this.state
    return (
      <div className="app">
        <div className="navbar">
          <h2>Pipelines</h2>
          <p className="running-time">Uptime: 32 hours</p>
        </div>
        { pipelines.map((p, i) => (
          <div className="pipeline" key={i}>
            <h2>{p.name}</h2>
            <Service service={p.services}/>
          </div>
        ))}
      </div>
    )
  }
}
