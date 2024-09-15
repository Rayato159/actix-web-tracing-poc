# Actix Web Tracing Example Using OpenTelemetry

This example demonstrates how to use OpenTelemetry with Actix Web to trace requests.

## Running the example

```json
{
  "resourceSpans": [
    {
      "resource": {
        "attributes": [
          {
            "key": "service.name",
            "value": { "stringValue": "unknown_service" }
          },
          {
            "key": "telemetry.sdk.name",
            "value": { "stringValue": "opentelemetry" }
          },
          {
            "key": "telemetry.sdk.version",
            "value": { "stringValue": "0.22.1" }
          },
          {
            "key": "telemetry.sdk.language",
            "value": { "stringValue": "rust" }
          }
        ]
      },
      "scopeSpans": [
        {
          "scope": { "name": "main" },
          "spans": [
            {
              "traceId": "ec75ae76d834385f92e4dea5e1752c29",
              "spanId": "2178b1484dadc4a2",
              "parentSpanId": "0000000000000000",
              "name": "{\r\n    \"message\": \"hello\"\r\n}",
              "kind": 2,
              "startTimeUnixNano": 1726396715667559300,
              "startTime": "2024-09-15 10:38:35.667",
              "endTimeUnixNano": 1726396715667571100,
              "endTime": "2024-09-15 10:38:35.667",
              "attributes": [],
              "droppedAttributesCount": 0,
              "droppedEventsCount": 0,
              "flags": 1,
              "droppedLinksCount": 0,
              "status": { "code": 1 }
            }
          ]
        }
      ]
    }
  ]
}
```
