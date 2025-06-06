use crust_core::{
    command::{CounterInnerCommand, CrdtInnerCommand},
    r#type::{CrdtType, CrdtTypeVariant},
};
use rand::{rng, seq::IndexedRandom, Rng};

pub fn generate_workload(
    crdt_type: CrdtType<String>,
    operation_count: usize,
) -> Vec<CrdtInnerCommand<String>> {
    let mut workload = Vec::new();

    let valid_commands = crdt_type.validate_command("".to_string());

    for _ in 0..operation_count {
        let command = match crdt_type.variant {
            CrdtTypeVariant::GCounter(_) => generate_counter_command(valid_commands.clone()),
        };
        workload.push(command);
    }
    workload
}

pub fn generate_counter_command(
    valid_commands: Vec<CrdtInnerCommand<String>>,
) -> CrdtInnerCommand<String> {
    let mut rng = rng();

    match valid_commands.choose(&mut rng) {
        Some(cmd) => match cmd {
            CrdtInnerCommand::Counter(counter_cmd) => match counter_cmd {
                CounterInnerCommand::Increment { .. } => {
                    let value = rng.random_range(1..=10).to_string();
                    CrdtInnerCommand::Counter(CounterInnerCommand::Increment { value })
                }
                CounterInnerCommand::Decrement { .. } => {
                    let value = rng.random_range(1..=5).to_string();
                    CrdtInnerCommand::Counter(CounterInnerCommand::Decrement { value })
                }
            },
            _ => cmd.clone(),
        },
        None => CrdtInnerCommand::Counter(CounterInnerCommand::Increment {
            value: "1".to_string(),
        }),
    }
}
