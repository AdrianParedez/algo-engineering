use cache_locality_lab::experiments::pointer_layout::{
    PointerLayoutExperiment, PointerLayoutVariant,
};
use serde::Serialize;
use std::env;
use std::error::Error;
use std::time::Instant;

#[derive(Debug)]
struct CliArgs {
    variant: PointerLayoutVariant,
    rounds: usize,
    warmup_rounds: usize,
}

#[derive(Debug, Serialize)]
struct ProbeReport {
    variant: &'static str,
    interpretation: &'static str,
    rounds: usize,
    warmup_rounds: usize,
    elements: usize,
    touch_bytes: usize,
    checksum: u64,
    elapsed_ns: u128,
    ns_per_round: f64,
    footprint: ProbeFootprint,
}

#[derive(Debug, Serialize)]
struct ProbeFootprint {
    boxed_unique_pages: usize,
    arena_unique_pages: usize,
    packed_unique_pages: usize,
    values_unique_pages: usize,
    next_indices_unique_pages: usize,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = parse_args(env::args().skip(1))?;
    let experiment = PointerLayoutExperiment::default();
    experiment.verify();

    for _ in 0..args.warmup_rounds {
        std::hint::black_box(experiment.run(args.variant));
    }

    let start = Instant::now();
    let mut checksum = 0_u64;
    for _ in 0..args.rounds {
        checksum ^= std::hint::black_box(experiment.run(args.variant));
    }
    let elapsed = start.elapsed();
    let footprint = experiment.footprint();

    let report = ProbeReport {
        variant: args.variant.name(),
        interpretation: args.variant.interpretation(),
        rounds: args.rounds,
        warmup_rounds: args.warmup_rounds,
        elements: experiment.element_count(),
        touch_bytes: experiment.touch_bytes(),
        checksum,
        elapsed_ns: elapsed.as_nanos(),
        ns_per_round: elapsed.as_nanos() as f64 / args.rounds as f64,
        footprint: ProbeFootprint {
            boxed_unique_pages: footprint.boxed_unique_pages,
            arena_unique_pages: footprint.arena_unique_pages,
            packed_unique_pages: footprint.packed_unique_pages,
            values_unique_pages: footprint.values_unique_pages,
            next_indices_unique_pages: footprint.next_indices_unique_pages,
        },
    };

    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}

fn parse_args<I>(mut args: I) -> Result<CliArgs, Box<dyn Error>>
where
    I: Iterator<Item = String>,
{
    let mut variant = PointerLayoutVariant::SplitSoaIndexChase;
    let mut rounds = 64_usize;
    let mut warmup_rounds = 8_usize;

    while let Some(flag) = args.next() {
        match flag.as_str() {
            "--variant" => {
                let value = args.next().ok_or("missing value for --variant")?;
                variant = parse_variant(&value)?;
            }
            "--rounds" => {
                let value = args.next().ok_or("missing value for --rounds")?;
                rounds = value.parse()?;
            }
            "--warmup-rounds" => {
                let value = args.next().ok_or("missing value for --warmup-rounds")?;
                warmup_rounds = value.parse()?;
            }
            "--help" | "-h" => {
                print_help();
                std::process::exit(0);
            }
            unknown => return Err(format!("unexpected argument: {unknown}").into()),
        }
    }

    Ok(CliArgs {
        variant,
        rounds,
        warmup_rounds,
    })
}

fn parse_variant(value: &str) -> Result<PointerLayoutVariant, Box<dyn Error>> {
    let variant = match value {
        "boxed_pointer_chain" => PointerLayoutVariant::BoxedPointerChain,
        "arena_pointer_chain" => PointerLayoutVariant::ArenaPointerChain,
        "packed_index_chase" => PointerLayoutVariant::PackedIndexChase,
        "split_soa_index_chase" => PointerLayoutVariant::SplitSoaIndexChase,
        "flat_sequential" => PointerLayoutVariant::FlatSequential,
        _ => {
            return Err(format!("unknown pointer-layout variant: {value}").into());
        }
    };

    Ok(variant)
}

fn print_help() {
    println!("usage: pointer_layout_probe [--variant <name>] [--rounds <n>] [--warmup-rounds <n>]");
    println!("variants:");
    for variant in PointerLayoutVariant::ALL {
        println!("  - {}: {}", variant.name(), variant.interpretation());
    }
}
