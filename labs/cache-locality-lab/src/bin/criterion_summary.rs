use serde::Serialize;
use serde_json::Value;
use std::env;
use std::error::Error;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug)]
struct CliArgs {
    criterion_root: PathBuf,
    json_out: PathBuf,
    markdown_out: PathBuf,
}

#[derive(Debug, Serialize)]
struct BenchmarkSummary {
    benchmark_id: String,
    mean_ns: f64,
    median_ns: f64,
    std_dev_ns: f64,
    lower_bound_ns: f64,
    upper_bound_ns: f64,
    work_unit_count: Option<u64>,
    work_unit_kind: Option<String>,
    throughput_per_second: Option<f64>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = parse_args(env::args().skip(1))?;
    args.criterion_root = fs::canonicalize(&args.criterion_root)?;
    let mut estimates = Vec::new();
    collect_estimates(&args.criterion_root, &args.criterion_root, &mut estimates)?;
    estimates.sort_by(|left, right| left.benchmark_id.cmp(&right.benchmark_id));

    if estimates.is_empty() {
        return Err(format!(
            "no Criterion estimate files found under {}",
            args.criterion_root.display()
        )
        .into());
    }

    if let Some(parent) = args.json_out.parent() {
        fs::create_dir_all(parent)?;
    }
    if let Some(parent) = args.markdown_out.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(&args.json_out, serde_json::to_string_pretty(&estimates)?)?;
    fs::write(&args.markdown_out, render_markdown(&estimates))?;
    println!(
        "wrote {} and {}",
        args.json_out.display(),
        args.markdown_out.display()
    );
    Ok(())
}

fn parse_args<I>(mut args: I) -> Result<CliArgs, Box<dyn Error>>
where
    I: Iterator<Item = String>,
{
    let mut criterion_root = None;
    let mut json_out = None;
    let mut markdown_out = None;

    while let Some(flag) = args.next() {
        match flag.as_str() {
            "--criterion-root" => criterion_root = args.next().map(PathBuf::from),
            "--json-out" => json_out = args.next().map(PathBuf::from),
            "--markdown-out" => markdown_out = args.next().map(PathBuf::from),
            "--help" | "-h" => {
                println!(
                    "usage: criterion_summary --criterion-root <path> --json-out <path> --markdown-out <path>"
                );
                std::process::exit(0);
            }
            unknown => {
                return Err(format!("unexpected argument: {unknown}").into());
            }
        }
    }

    Ok(CliArgs {
        criterion_root: criterion_root.unwrap_or_else(|| PathBuf::from("target/criterion")),
        json_out: json_out.unwrap_or_else(|| PathBuf::from("reports/latest-summary.json")),
        markdown_out: markdown_out.unwrap_or_else(|| PathBuf::from("reports/latest-summary.md")),
    })
}

fn collect_estimates(
    criterion_root: &Path,
    current_dir: &Path,
    summaries: &mut Vec<BenchmarkSummary>,
) -> Result<(), Box<dyn Error>> {
    for entry in fs::read_dir(current_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            collect_estimates(criterion_root, &path, summaries)?;
            continue;
        }

        if path.file_name() != Some(OsStr::new("estimates.json")) {
            continue;
        }

        if path.parent().and_then(Path::file_name) != Some(OsStr::new("new")) {
            continue;
        }

        summaries.push(parse_estimate(criterion_root, &path)?);
    }

    Ok(())
}

fn parse_estimate(root: &Path, path: &Path) -> Result<BenchmarkSummary, Box<dyn Error>> {
    let json: Value = serde_json::from_slice(&fs::read(path)?)?;
    let mean_ns = point_estimate(&json, "mean")?;
    let median_ns = point_estimate(&json, "median")?;
    let std_dev_ns = point_estimate(&json, "std_dev")?;
    let lower_bound_ns = lower_bound(&json, "mean")?;
    let upper_bound_ns = upper_bound(&json, "mean")?;

    let new_dir = path
        .parent()
        .ok_or_else(|| format!("missing parent directory for {}", path.display()))?;
    let benchmark_dir = new_dir
        .parent()
        .ok_or_else(|| format!("unexpected Criterion path layout: {}", path.display()))?;
    let root_text = root.to_string_lossy().replace('\\', "/");
    let benchmark_text = benchmark_dir.to_string_lossy().replace('\\', "/");
    let benchmark_path = benchmark_text
        .strip_prefix(&(root_text + "/"))
        .ok_or_else(|| {
            format!(
                "failed to relativize {} against {}",
                benchmark_dir.display(),
                root.display()
            )
        })?
        .to_string();

    let (work_unit_count, work_unit_kind) = parse_work_units(&benchmark_path);
    let throughput_per_second =
        work_unit_count.map(|count| count as f64 / (mean_ns / 1_000_000_000.0));

    Ok(BenchmarkSummary {
        benchmark_id: benchmark_path,
        mean_ns,
        median_ns,
        std_dev_ns,
        lower_bound_ns,
        upper_bound_ns,
        work_unit_count,
        work_unit_kind,
        throughput_per_second,
    })
}

fn point_estimate(json: &Value, field: &str) -> Result<f64, Box<dyn Error>> {
    json[field]["point_estimate"]
        .as_f64()
        .ok_or_else(|| format!("missing point_estimate for {field}").into())
}

fn lower_bound(json: &Value, field: &str) -> Result<f64, Box<dyn Error>> {
    json[field]["confidence_interval"]["lower_bound"]
        .as_f64()
        .ok_or_else(|| format!("missing lower_bound for {field}").into())
}

fn upper_bound(json: &Value, field: &str) -> Result<f64, Box<dyn Error>> {
    json[field]["confidence_interval"]["upper_bound"]
        .as_f64()
        .ok_or_else(|| format!("missing upper_bound for {field}").into())
}

fn parse_work_units(benchmark_id: &str) -> (Option<u64>, Option<String>) {
    let last_segment = benchmark_id.rsplit('/').next().unwrap_or_default();

    for suffix in ["elements", "bytes", "macs"] {
        if let Some(prefix) = last_segment.strip_suffix(&format!("-{suffix}")) {
            let digits = prefix
                .rsplit(|ch: char| !ch.is_ascii_digit())
                .next()
                .unwrap_or_default();

            if let Ok(value) = digits.parse::<u64>() {
                return (Some(value), Some(suffix.to_string()));
            }
        }
    }

    (None, None)
}

fn render_markdown(summaries: &[BenchmarkSummary]) -> String {
    let mut output = String::from(
        "# Criterion Summary\n\n| Benchmark | Mean (ns) | Median (ns) | Std Dev (ns) | 95% CI Lower | 95% CI Upper | Throughput |\n| --- | ---: | ---: | ---: | ---: | ---: | --- |\n",
    );

    for summary in summaries {
        let throughput = match (
            summary.throughput_per_second,
            summary.work_unit_kind.as_deref(),
        ) {
            (Some(value), Some(unit)) => format!("{value:.2} {unit}/s"),
            _ => String::from("n/a"),
        };

        output.push_str(&format!(
            "| {} | {:.2} | {:.2} | {:.2} | {:.2} | {:.2} | {} |\n",
            summary.benchmark_id,
            summary.mean_ns,
            summary.median_ns,
            summary.std_dev_ns,
            summary.lower_bound_ns,
            summary.upper_bound_ns,
            throughput
        ));
    }

    output
}
