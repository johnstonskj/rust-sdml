/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use crate::draw::OutputFormat;
use crate::error::Error;
use crate::model::Module;
use std::io::Write;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub fn write_uml_diagram<W: Write>(
    _module: &Module,
    _w: &mut W,
    _format: OutputFormat,
) -> Result<(), Error> {
    todo!()
}

write_to_string!(to_uml_diagram_string, write_uml_diagram, OutputFormat);

write_to_file!(uml_diagram_to_file, write_uml_diagram, OutputFormat);

print_to_stdout!(print_uml_diagram, write_uml_diagram, OutputFormat);

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
