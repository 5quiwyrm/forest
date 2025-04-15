mod forest_runtime;
use forest_runtime::*;

fn main() {
    use forest_runtime::ForestInstruction::*;
    let mut runtime = forest_runtime::ForestRuntime::new(&[Exit]);
    execute_runtime!(runtime, false);
}

#[test]
fn loops() {
    let mut runtime = ForestRuntime::new(&[
        ForestInstruction::Push(ForestValue::Int(10)),
        ForestInstruction::Loop,
        ForestInstruction::Duplicate,
        ForestInstruction::Print,
        ForestInstruction::Push(ForestValue::String("\n".to_string())),
        ForestInstruction::Print,
        ForestInstruction::Push(ForestValue::Int(1)),
        ForestInstruction::Subtract,
        ForestInstruction::Duplicate,
        ForestInstruction::Push(ForestValue::Int(0)),
        ForestInstruction::Eq,
        ForestInstruction::If,
        ForestInstruction::Exit,
        ForestInstruction::IfEnd,
        ForestInstruction::Drop,
        ForestInstruction::LoopEnd,
        ForestInstruction::Exit,
    ]);
    execute_runtime!(runtime, false);
}
