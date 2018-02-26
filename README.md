# FaaaS

Experiment a fast invoker with various ways.

## Benchmark

The collected statistics is under wrk as http benchmarking.

```bash
# Under 100 connections, 12 threads
./wrk -c100 -t12 -d10 --latency <Http Endpoint>
```

**Node.js express**

```bash
Latency Average
  8.00ms
Latency Distribution
  50%    7.28ms
  75%    7.52ms
  90%    8.89ms
  99%   21.64ms
Requests/sec
  12193.67
```

**Elixir plug**

```bash
Latency Average
  9.19ms
Latency Distribution
  50%    1.82ms
  75%    4.38ms
  90%   28.01ms
  99%  102.90ms
Requests/sec
  34954.33
```

**Go standard http**

```bash
Latency Average
  1.43ms
Latency Distribution
  50%    1.18ms
  75%    1.48ms
  90%    2.00ms
  99%    8.52ms
Requests/sec
  75803.01
```

**Rust hyper**

```bash
Latency Average
  1.82ms
Latency Distribution
  50%    1.75ms
  75%    1.87ms
  90%    2.41ms
  99%    2.59ms
Requests/sec
  52456.32
```

**Invoker(http-based param propagation)**

```bash
Latency Average
  16.55ms
Latency Distribution
  50%   15.15ms
  75%   16.63ms
  90%   18.95ms
  99%   36.09ms
Requests/sec
  5819.30
```

**OpenWhisk(vbox: 4cores)**

```bash
# Note : run with ./wrk -c100 -t12 -d30 to eliminate cold start.
# The scaling replicas of instances upto 6
Latency Average
  234.51ms
Latency Distribution
  50%  220.71ms
  75%  239.19ms
  90%  271.17ms
  99%  536.70ms
Requests/sec
  411.35

# As reference node.js express speed in vbox.
# Only slightly degrade!
Latency Average
  8.85ms
Requests/sec
  10834.39
```

**Invoker(Unix Domain Socket, TCP based param propagation)**

```bash
# Note : the implementation still have huge space to optimize, i.e. JSON serialization.
Latency Average
  8.27ms
Latency Distribution
  50%    8.29ms
  75%    9.12ms
  90%    9.71ms
  99%   10.50ms
Requests/sec
  11590.92

```


## License

This project is licensed under MIT license. ([LICENSE MIT](https://github.com/tz70s/tiny-invoker/blob/master/LICENSE) or http://opensource.org/licenses/MIT)
