export default {
  "pipelines": [
    {
      "name": "eventids",
      "max_seconds_to_reach_end": 900,
      "root": "step2",
      "services": [
        {
          name: "step2",
          children: ["step3"],
          stats: {
            events_seen: 100,
            events_expected: 100,
          }
        },
        {
          name: "step3",
          children: [ "step4a", "step4b" ],
          stats: {
            events_seen: 100,
            events_expected: 100,
          }
        },
        {
          name: "step4a",
          children: [],
          stats: {
            events_seen: 100,
            events_expected: 100,
          }
        },
        {
          name: "step4b",
          children: ["step5"],
          stats: {
            events_seen: 99,
            events_expected: 100,
          }
        },
        {
          name: "step5",
          children: [],
          stats: {
            events_seen: 80,
            events_expected: 100,
          }
        },
      ]
    }
  ]
}
