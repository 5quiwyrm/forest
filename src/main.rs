mod forest_runtime;
use forest_runtime::*;

fn main() {
    use forest_runtime::ForestInstruction::*;
    let mut runtime = forest_runtime::ForestRuntime::new(&[Exit]);
    execute_runtime!(runtime, false);
}

#[test]
fn loops() {
    use forest_runtime::ForestInstruction as fi;
    use forest_runtime::ForestValue as fv;
    let mut runtime = ForestRuntime::new(&[
        fi::Push(fv::Int(10)),
        fi::Loop,
        fi::Duplicate,
        fi::Print,
        fi::Push(fv::String("\n".to_string())),
        fi::Print,
        fi::Push(fv::Int(1)),
        fi::Subtract,
        fi::Duplicate,
        fi::Push(fv::Int(0)),
        fi::Eq,
        fi::If,
        fi::Exit,
        fi::IfEnd,
        fi::Drop,
        fi::LoopEnd,
        fi::Exit,
    ]);
    execute_runtime!(runtime, false);
}

#[test]
fn basic_words() {
    use forest_runtime::ForestInstruction as fi;
    use forest_runtime::ForestValue as fv;
    let mut runtime = ForestRuntime::new(&[
        fi::MakeWord("inc".to_string()),
        fi::Push(fv::Int(1)),
        fi::Add,
        fi::EndWord,
        fi::Push(fv::Int(1)),
        fi::InvokeWord("inc".to_string()),
        fi::Print,
        fi::Exit,
    ]);
    execute_runtime!(runtime, false);
}

#[test]
fn branched_word() {
    use forest_runtime::ForestInstruction as fi;
    use forest_runtime::ForestValue as fv;
    let mut runtime = ForestRuntime::new(&[
        fi::MakeWord("inc".to_string()),
        fi::Push(fv::Int(1)),
        fi::Add,
        fi::EndWord,
        fi::Push(fv::Int(1)), // sabotage or nah
        fi::If,
        /**/ fi::MakeWord("inc".to_string()),
        /**/ fi::Push(fv::Int(0)),
        /**/ fi::Add,
        /**/ fi::EndWord,
        fi::IfEnd,
        fi::Drop,
        fi::Push(fv::Int(1)),
        fi::InvokeWord("inc".to_string()),
        fi::Print,
        fi::Exit,
    ]);
    execute_runtime!(runtime, false);
}
