Nomarch
=======

Nomarch tracks events through your pipeline

Modern data pipelines are complex with multiple points of failure.
Nomarch accepts check-ins for each event id from each step in the pipeline.
It is able to:
	* Alert you when events go missing, with specifics
	* Pinpoint exactly where events got lost
	* Give statistics on how many events reach each part of the pipeline
        * Optionally save reciepts in an S3 compatible store

## Simplicity

Nomarch is designed for simplicity.
This gives it incredible performance and reliability.
Massive data pipelines should be able to be handled by a single node.

## Config

We keep the config small, simple, and with minimal footguns.
There are no optional fields, and therefore no surprising defaults.

Set the following fields explicitly in the config.json:

```
{
  // Your config's version.
  // This is just a string used in logging for clarity.
  "version": "1",

  // The pipeline tracked by this instance of Nomarch
  "pipeline": {
    "name": "pipeline_1",

    // Events are considered "lost" if they do not make it through the
    // pipeline in this number of seconds.
    //
    // Event IDs are held in memory for this duration. The smaller the number
    // the more events an instance can handle.
    "max_seconds_to_reach_end": 900,

    // Services an event must pass through in a pipeline.
    "services": [
        {
          "name": "step1",
          "children": [
            "step2"
          ],
          "optional": false
        },
        {
          "name": "step2",
          "children": [],
          "optional": false
        }
      ]
  }
}
```

## Scalability

Nomarch is heavily optimized and a single instance will likely handle your
pipeline just fine. Benchmark and measure it before scaling it out.

If you _do_ need more than one instance of Nomarch, it can be scaled in three
ways:
  * Increase the host's memory. The biggest limiting factor is memory to store
    event ids.
  * Partition the pipeline across Nomarch instances. This allows a smaller
    `max_seconds_to_reach_end` config, meaning fewer events are held in memory
    at a time.
  * Partition events across Nomarch instances. UUIDv4s, sequential ids, and
    other common schemes are normally distributed by modulo. This means you
    can send some ids to one instance and other ids to another.
