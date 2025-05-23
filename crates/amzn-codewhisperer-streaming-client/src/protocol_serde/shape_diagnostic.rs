// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_diagnostic(
    object_12: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::Diagnostic,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    match input {
        crate::types::Diagnostic::TextDocumentDiagnostic(inner) => {
            #[allow(unused_mut)]
            let mut object_1 = object_12.key("textDocumentDiagnostic").start_object();
            crate::protocol_serde::shape_text_document_diagnostic::ser_text_document_diagnostic(&mut object_1, inner)?;
            object_1.finish();
        },
        crate::types::Diagnostic::RuntimeDiagnostic(inner) => {
            #[allow(unused_mut)]
            let mut object_2 = object_12.key("runtimeDiagnostic").start_object();
            crate::protocol_serde::shape_runtime_diagnostic::ser_runtime_diagnostic(&mut object_2, inner)?;
            object_2.finish();
        },
        crate::types::Diagnostic::Unknown => {
            return Err(::aws_smithy_types::error::operation::SerializationError::unknown_variant("Diagnostic"));
        },
    }
    Ok(())
}
