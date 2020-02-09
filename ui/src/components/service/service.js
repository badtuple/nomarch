import React from 'react';
import './service.scss'

function Service({service}) {
  let healthy = service.stats.events_seen === service.stats.events_expected
  let color = healthy ? '' : ' red'

  return (
    <div className="service-wrapper">
      <div className={`service${color}`}>
        <p className="service-name">{service.name}</p>
        <p className="events-lost">
          {service.stats.events_expected - service.stats.events_seen}
          <sup>events lost</sup>
        </p>
      </div>

      <div className="service-children">
        { service.children.map(childNode =>
          <Service
            key={`service-tree-${childNode.name}`}
            service={childNode}
          />
        )}
      </div>
    </div>
  )
}

export default Service
