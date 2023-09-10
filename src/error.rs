use std::result;
use syn;

pub struct DiagnosticError {
    diagnostic: String,
    #[allow(dead_code)]
    syn_error: Option<syn::parse::Error>,
}

impl DiagnosticError {
    pub fn new(diagnostic: String) -> DiagnosticError {
        DiagnosticError {
            diagnostic,
            syn_error: None,
        }
    }
    pub fn new_with_syn_error(diagnostic: String, syn_error: syn::parse::Error) -> DiagnosticError {
        DiagnosticError {
            diagnostic,
            syn_error: Some(syn_error),
        }
    }

    #[allow(dead_code)]
    pub fn source(&self) -> Option<&syn::parse::Error> {
        self.syn_error.as_ref()
    }

    pub fn emit(self) {
        println!("{}", &self.diagnostic);
    }
}

pub type Result<T> = result::Result<T, DiagnosticError>;
