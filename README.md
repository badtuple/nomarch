Nomarch
=======

Nomarch tracks events through your pipelines

Modern data pipelines are complex with multiple points of failure.
Nomarch accepts check-ins for each event id from each step in your pipeline.
It is able to:
	* Alert you when events go missing, with specifics
	* Pinpoint exactly where events got lost
	* Give statistics on how many events reach each part of your pipeline
        * Optionally save reciepts in an S3 compatible store

## Simplicity

Nomarch is designed for simplicity.
This gives it incredible performance and reliability.
Massive data pipelines should be able to be handled by a single node.

## Config

We keep the config small, simple, and with minimal footguns.
There are no optional fields, and therefore no surprising defaults.

Set the following fields explicitly in your config.json:

```
{
  // Your config's version.
  // This is just a string used in logging for clarity.
  "version": "1",

  // Pipelines tracked by this instance of Nomarch
  "pipelines": [
    {
      "name": "pipeline_1",

      // Events are considered "lost" if they do not make it through the
      // pipeline in this number of seconds.
      //
      // Event IDs are held in memory for this duration. The smaller the number
      // the more events an instance can handle.
      "max_seconds_to_reach_end": 900,

      // Services an event must pass through in a pipeline.
      "services" [
        "a",
        "b",
        "c"
      ]
    }
  ]
}
```
