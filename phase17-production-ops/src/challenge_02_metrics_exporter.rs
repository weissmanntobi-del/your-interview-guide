/// Challenge 02 - Metrics Exporter (Prometheus Text Format)
///
/// Implement Counter, Gauge, and Histogram metric types, then a MetricsRegistry
/// that can export all metrics in Prometheus text exposition format.
///
/// Prometheus format example:
///   # HELP http_requests_total Total HTTP requests
///   # TYPE http_requests_total counter
///   http_requests_total 1234

use std::collections::HashMap;

// ============================================================
// COUNTER: monotonically increasing value
// ============================================================

#[derive(Debug, Clone)]
pub struct Counter {
    name: String,
    help: String,
    value: u64,
}

impl Counter {
    pub fn new(name: &str, help: &str) -> Self {
        Counter {
            name: name.to_string(),
            help: help.to_string(),
            value: 0,
        }
    }

    pub fn increment(&mut self) {
        // TODO: Increment by 1
        todo!("Implement Counter::increment")
    }

    pub fn increment_by(&mut self, n: u64) {
        // TODO: Increment by n
        todo!("Implement Counter::increment_by")
    }

    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn help(&self) -> &str {
        &self.help
    }
}

// ============================================================
// GAUGE: value that can go up and down
// ============================================================

#[derive(Debug, Clone)]
pub struct Gauge {
    name: String,
    help: String,
    value: f64,
}

impl Gauge {
    pub fn new(name: &str, help: &str) -> Self {
        Gauge {
            name: name.to_string(),
            help: help.to_string(),
            value: 0.0,
        }
    }

    pub fn set(&mut self, val: f64) {
        // TODO: Set to val
        todo!("Implement Gauge::set")
    }

    pub fn increment(&mut self) {
        // TODO: Increment by 1.0
        todo!("Implement Gauge::increment")
    }

    pub fn decrement(&mut self) {
        // TODO: Decrement by 1.0
        todo!("Implement Gauge::decrement")
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn help(&self) -> &str {
        &self.help
    }
}

// ============================================================
// HISTOGRAM: distribution of observed values into buckets
// ============================================================

#[derive(Debug, Clone)]
pub struct Histogram {
    name: String,
    help: String,
    bucket_bounds: Vec<f64>,    // upper bounds of buckets (sorted)
    bucket_counts: Vec<u64>,    // count per bucket
    sum: f64,
    count: u64,
}

impl Histogram {
    /// Create a histogram with the given bucket upper bounds.
    /// Bounds should be sorted ascending. A +Inf bucket is implicit.
    pub fn new(name: &str, help: &str, bucket_bounds: Vec<f64>) -> Self {
        let len = bucket_bounds.len();
        Histogram {
            name: name.to_string(),
            help: help.to_string(),
            bucket_bounds,
            bucket_counts: vec![0; len],
            sum: 0.0,
            count: 0,
        }
    }

    /// Record an observed value. Increments all buckets whose bound >= val.
    pub fn observe(&mut self, val: f64) {
        // TODO: Implement observe
        // 1. Increment self.count
        // 2. Add val to self.sum
        // 3. For each bucket where val <= bound, increment that bucket's count
        todo!("Implement Histogram::observe")
    }

    pub fn count(&self) -> u64 {
        self.count
    }

    pub fn sum(&self) -> f64 {
        self.sum
    }

    /// Return bucket data as (upper_bound, cumulative_count) pairs.
    pub fn buckets(&self) -> Vec<(f64, u64)> {
        // TODO: Return vec of (bound, count) pairs
        todo!("Implement Histogram::buckets")
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn help(&self) -> &str {
        &self.help
    }
}

// ============================================================
// METRICS REGISTRY
// ============================================================

pub struct MetricsRegistry {
    counters: Vec<Counter>,
    gauges: Vec<Gauge>,
    histograms: Vec<Histogram>,
}

impl MetricsRegistry {
    pub fn new() -> Self {
        MetricsRegistry {
            counters: Vec::new(),
            gauges: Vec::new(),
            histograms: Vec::new(),
        }
    }

    pub fn register_counter(&mut self, counter: Counter) -> usize {
        let idx = self.counters.len();
        self.counters.push(counter);
        idx
    }

    pub fn register_gauge(&mut self, gauge: Gauge) -> usize {
        let idx = self.gauges.len();
        self.gauges.push(gauge);
        idx
    }

    pub fn register_histogram(&mut self, histogram: Histogram) -> usize {
        let idx = self.histograms.len();
        self.histograms.push(histogram);
        idx
    }

    pub fn counter_mut(&mut self, idx: usize) -> &mut Counter {
        &mut self.counters[idx]
    }

    pub fn gauge_mut(&mut self, idx: usize) -> &mut Gauge {
        &mut self.gauges[idx]
    }

    pub fn histogram_mut(&mut self, idx: usize) -> &mut Histogram {
        &mut self.histograms[idx]
    }

    /// Export all metrics in Prometheus text exposition format.
    /// For each metric:
    ///   # HELP name help_text
    ///   # TYPE name type
    ///   name value
    /// For histograms, emit bucket lines, _sum, and _count.
    pub fn export(&self) -> String {
        // TODO: Implement Prometheus text format export
        // Counters:
        //   # HELP name help
        //   # TYPE name counter
        //   name value
        // Gauges:
        //   # HELP name help
        //   # TYPE name gauge
        //   name value
        // Histograms:
        //   # HELP name help
        //   # TYPE name histogram
        //   name_bucket{le="bound"} count   (for each bucket)
        //   name_bucket{le="+Inf"} total_count
        //   name_sum value
        //   name_count value
        todo!("Implement MetricsRegistry::export")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter_increments() {
        let mut c = Counter::new("requests", "total requests");
        assert_eq!(c.value(), 0);
        c.increment();
        assert_eq!(c.value(), 1);
        c.increment_by(5);
        assert_eq!(c.value(), 6);
    }

    #[test]
    fn test_gauge_operations() {
        let mut g = Gauge::new("temperature", "current temp");
        g.set(72.5);
        assert_eq!(g.value(), 72.5);
        g.increment();
        assert_eq!(g.value(), 73.5);
        g.decrement();
        assert_eq!(g.value(), 72.5);
    }

    #[test]
    fn test_histogram_buckets() {
        let mut h = Histogram::new(
            "request_duration",
            "duration in seconds",
            vec![0.01, 0.05, 0.1, 0.5, 1.0],
        );
        h.observe(0.03);  // falls into 0.05 bucket and above
        h.observe(0.07);  // falls into 0.1 bucket and above
        h.observe(0.5);   // falls into 0.5 bucket and above
        h.observe(2.0);   // exceeds all bounds

        assert_eq!(h.count(), 4);
        assert!((h.sum() - 2.6).abs() < 0.001);

        let buckets = h.buckets();
        // Cumulative: 0.01->0, 0.05->1, 0.1->2, 0.5->3, 1.0->3
        assert_eq!(buckets[0], (0.01, 0));
        assert_eq!(buckets[1], (0.05, 1));
        assert_eq!(buckets[2], (0.1, 2));
        assert_eq!(buckets[3], (0.5, 3));
        assert_eq!(buckets[4], (1.0, 3));
    }

    #[test]
    fn test_export_format() {
        let mut registry = MetricsRegistry::new();

        let c_idx = registry.register_counter(Counter::new("http_requests_total", "Total HTTP requests"));
        registry.counter_mut(c_idx).increment_by(42);

        let g_idx = registry.register_gauge(Gauge::new("cpu_usage", "CPU usage percent"));
        registry.gauge_mut(g_idx).set(65.5);

        let h_idx = registry.register_histogram(Histogram::new(
            "response_time",
            "Response time in seconds",
            vec![0.1, 0.5, 1.0],
        ));
        registry.histogram_mut(h_idx).observe(0.3);

        let output = registry.export();

        // Verify counter
        assert!(output.contains("# HELP http_requests_total Total HTTP requests"));
        assert!(output.contains("# TYPE http_requests_total counter"));
        assert!(output.contains("http_requests_total 42"));

        // Verify gauge
        assert!(output.contains("# HELP cpu_usage CPU usage percent"));
        assert!(output.contains("# TYPE cpu_usage gauge"));
        assert!(output.contains("cpu_usage 65.5"));

        // Verify histogram
        assert!(output.contains("# TYPE response_time histogram"));
        assert!(output.contains("response_time_bucket{le=\"0.1\"} 0"));
        assert!(output.contains("response_time_bucket{le=\"0.5\"} 1"));
        assert!(output.contains("response_time_bucket{le=\"+Inf\"} 1"));
        assert!(output.contains("response_time_count 1"));
    }

    #[test]
    fn test_empty_registry_export() {
        let registry = MetricsRegistry::new();
        let output = registry.export();
        assert!(output.is_empty() || output.trim().is_empty());
    }
}
