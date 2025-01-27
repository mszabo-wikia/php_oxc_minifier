//! PHP bindings for the OXC minifier.
//!
//! # Usage
//! ```php
//! $minifier = new JavascriptMinifier();
//! $minified_code = $minifier->minify( $code );
//! ```

#![cfg_attr(windows, feature(abi_vectorcall))]

use ext_php_rs::convert::IntoZval;
use ext_php_rs::prelude::*;
use ext_php_rs::types::ZendClassObject;
use ext_php_rs::{exception::PhpException, zend::ce};
use oxc::allocator::Allocator;
use oxc::codegen::{Codegen, CodegenOptions, CodegenReturn, LegalComment};
use oxc::diagnostics::Error;
use oxc::minifier::{CompressOptions, Minifier, MinifierOptions, MinifierReturn};
use oxc::parser::{Parser, ParserReturn};
use oxc::span::SourceType;
use oxc::syntax::es_target::ESTarget;

/// PHP exception thrown when minification fails.
#[php_class]
#[extends(ce::exception())]
#[derive(Default)]
pub struct MinificationException {
    errors: Vec<Error>,
}

#[php_impl]
impl MinificationException {
    /// Get the errors that caused the minification to fail, in an user-friendly format.
    pub fn get_errors(&self) -> Vec<String> {
        self.errors
            .iter()
            .map(|error| format!("{error:?}"))
            .collect()
    }
}

/// PHP wrapper for the OXC minifier.
#[php_class]
pub struct JavascriptMinifier;

#[php_impl]
impl JavascriptMinifier {
    pub fn __construct() -> Self {
        JavascriptMinifier {}
    }

    /// Minify the given JavaScript source code.
    ///
    /// Returns the minified code if successful, or throws a MinificationException if an error occurs.
    pub fn minify(&self, source_text: &str) -> PhpResult<String> {
        let allocator = Allocator::default();
        let ParserReturn {
            mut program,
            errors,
            ..
        } = Parser::new(&allocator, source_text, SourceType::cjs()).parse();

        if !errors.is_empty() {
            let source_text_err = source_text.to_string();
            let mut minification_exception_obj = ZendClassObject::new(MinificationException {
                errors: errors
                    .into_iter()
                    .map(|error| error.with_source_code(source_text_err.clone()))
                    .collect(),
            });
            let mut minification_exception =
                PhpException::from_class::<MinificationException>("Minification error".to_string());
            minification_exception.set_object(minification_exception_obj.std.into_zval(false).ok());

            return Err(minification_exception);
        }

        let minifier_options = MinifierOptions {
            compress: Some(CompressOptions {
                target: ESTarget::ES2016,
                drop_debugger: false,
                drop_console: false,
            }),
            mangle: None,
        };

        let MinifierReturn { mangler } =
            Minifier::new(minifier_options).build(&allocator, &mut program);

        let CodegenReturn { code, .. } = Codegen::new()
            .with_mangler(mangler)
            .with_options(CodegenOptions {
                single_quote: false,
                minify: true,
                comments: false,
                annotation_comments: false,
                legal_comments: LegalComment::default(),
                source_map_path: None,
            })
            .build(&program);

        Ok(code)
    }
}

// Required to register the extension with PHP.
#[php_module]
pub fn module(module: ModuleBuilder) -> ModuleBuilder {
    module
}
