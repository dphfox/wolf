use crate::ErrorInParse;

pub fn explain_parse_error(error: &ErrorInParse) {
	match error {
		ErrorInParse::NotYetImplemented { note } => {
			println!("not yet implemented: {note}");
		},
		ErrorInParse::UnexpectedToken { token, expected } => {
			println!("unexpected token {} at {}:{}", token.ty.external_name(), token.span.line, token.span.line_index);
			println!("expected {expected}");
		},
		ErrorInParse::UnexpectedEndOfFile { expected } => {
			println!("end of file reached unexpectedly");
			println!("expected {expected}");
		},
		ErrorInParse::Context { start, name, inner } => {
			explain_parse_error(inner);
			if let Some(start) = start {
				println!("-> during {name} starting at {}:{}", start.line, start.line_index)
			} else {
				println!("-> after attempt to start parsing {name}")
			}
		}
	}
}