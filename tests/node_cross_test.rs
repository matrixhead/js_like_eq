use js_like_eq::JsCompare;
use serde_json::{Value, json};
use std::io::{Read, Write};
use std::process::{Command, Stdio};

// --- Minimal Node.js script embedded as a string ---
// Reads *all* stdin as the expression, evaluates it, prints JSON result/error to stdout.
const NODE_SCRIPT: &str = r#"
let input = '';
process.stdin.on('data', (chunk) => { input += chunk; });
process.stdin.on('end', () => {
  let response;
  try {
    // SECURITY WARNING: Raw eval() is dangerous with untrusted input!
    const result = eval(input);
    response = { result: result };
  } catch (error) {
    response = { error: error.message || String(error) };
  }
  // Output the JSON result/error object as a single line
  console.log(JSON.stringify(response));
});
"#;

/// Executes a JavaScript expression using an embedded Node.js script.
/// Panics if any step fails (process start, I/O, JSON parsing, JS error).
///
/// SECURITY WARNING: Uses eval(). Do NOT use with untrusted input expressions.
fn evaluate_js_expression_simple(expression: &str) -> Value {
    let mut child = Command::new("node")
        .arg("-e")
        .arg(NODE_SCRIPT)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn Node.js process. Is 'node' in PATH?");

    {
        let mut stdin = child.stdin.take().expect("Failed to get Node.js stdin");
        stdin
            .write_all(expression.as_bytes())
            .expect("Failed to write to Node.js stdin");
    }

    let mut stdout_str = String::new();
    child
        .stdout
        .take()
        .expect("Failed to get Node.js stdout")
        .read_to_string(&mut stdout_str)
        .expect("Failed to read from Node.js stdout");

    let mut stderr_str = String::new();
    child
        .stderr
        .take()
        .expect("Failed to get Node.js stderr")
        .read_to_string(&mut stderr_str)
        .expect("Failed to read from Node.js stderr");

    let status = child.wait().expect("Node.js process did not exit?");
    if !status.success() {
        eprintln!("Node.js process exited with status: {}", status);
        eprintln!("Node.js stderr:\n{}", stderr_str);
        panic!("Node.js process failed.");
    }

    let output_json: Value = serde_json::from_str(&stdout_str).unwrap_or_else(|_| {
        panic!(
            "Failed to parse JSON from Node.js stdout. Received: '{}'",
            stdout_str.trim()
        )
    });

    if let Some(err_msg) = output_json.get("error").and_then(|v| v.as_str()) {
        panic!("Node.js evaluation error: {}", err_msg);
    }

    output_json
        .get("result")
        .cloned()
        .expect("Missing 'result' field in Node.js JSON response")
}

fn format_for_js_expr(value: &Value) -> String {
    match value {
        Value::Object(_) => format!("({})", value),
        _ => value.to_string(),
    }
}

fn get_to_compare() -> Vec<Value> {
    vec![
        json!(null),
        json!(true),
        json!(false),
        json!(0),
        json!(1),
        json!(256),
        json!(""),
        json!("abc"),
        json!("def"),
        json!("256"),
        json!({"key":"value"}),
        json!([]),
        json!([1]),
        json!([0]),
        json!([[0]]),
        json!([[""]]),
        json!(["abc"]),
        json!([1, 2, 3]),
        json!([{}]),
    ]
}

#[test]
#[ignore = "requires node in PATH"]
fn cross_check_test_lessthan() {
    let tocompare = get_to_compare();
    for lhs in tocompare.iter() {
        for rhs in tocompare.iter() {
            let rust_result = lhs.js_lt(rhs);
            let js_result = evaluate_js_expression_simple(&format!(
                "{}<{}",
                format_for_js_expr(lhs),
                format_for_js_expr(rhs)
            ));
            match js_result {
                Value::Bool(b) => {
                    assert_eq!(rust_result, b, "we are testing {} < {}", lhs, rhs)
                }
                _ => panic!("Expected boolean result from JS evaluation"),
            }
        }
    }
}

#[test]
#[ignore = "requires node in PATH"]
fn cross_check_test_less_than_equal_to() {
    let tocompare = get_to_compare();
    for lhs in tocompare.iter() {
        for rhs in tocompare.iter() {
            let rust_result = lhs.js_le(rhs);
            let js_result = evaluate_js_expression_simple(&format!(
                "{}<={}",
                format_for_js_expr(lhs),
                format_for_js_expr(rhs)
            ));
            match js_result {
                Value::Bool(b) => {
                    assert_eq!(rust_result, b, "we are testing {} <= {}", lhs, rhs)
                }
                _ => panic!("Expected boolean result from JS evaluation"),
            }
        }
    }
}
