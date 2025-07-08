use std::error::Error;
use std::marker::PhantomData;
use std::path::{Path, PathBuf};

use super::codegen::CodeGenerator;
use super::extractor::RustdocDataExtractor;
use super::parser::RustdocParser;
use super::rustdoc::DocumentationGenerator;

/// Trait for processors that extract information from documentation
pub trait DocumentationProcessor<T> {
    /// Process documentation source and extract relevant information
    fn process(&self, source: &Path) -> Result<T, Box<dyn Error>>;
}

/// Processor that combines a parser and extractor to process rustdoc JSON
pub struct RustdocProcessor<P, E, T> {
    parser: P,
    extractor: E,
    _phantom: PhantomData<T>,
}

impl<P, E, T> RustdocProcessor<P, E, T>
where
    P: RustdocParser,
    E: RustdocDataExtractor<T>,
{
    pub fn new(parser: P, extractor: E) -> Self {
        Self {
            parser,
            extractor,
            _phantom: PhantomData,
        }
    }
}

impl<P, E, T> DocumentationProcessor<Vec<T>> for RustdocProcessor<P, E, T>
where
    P: RustdocParser,
    E: RustdocDataExtractor<T>,
{
    fn process(&self, source: &Path) -> Result<Vec<T>, Box<dyn Error>> {
        // Parse the JSON
        let data = self.parser.parse(source)?;

        // Extract the relevant information
        let result = self.extractor.extract(&data)?;

        Ok(result)
    }
}

/// Documentation pipeline that combines generation, processing, and code generation
pub struct DocumentationPipeline<G, P, C, T> {
    generator: G,
    processor: P,
    code_generator: C,
    output_path: PathBuf,
    _phantom: PhantomData<T>,
}

impl<G, P, C, T> DocumentationPipeline<G, P, C, T>
where
    G: DocumentationGenerator,
    P: DocumentationProcessor<Vec<T>>,
    C: CodeGenerator<T>,
{
    pub fn new(generator: G, processor: P, code_generator: C, output_path: PathBuf) -> Self {
        Self {
            generator,
            processor,
            code_generator,
            output_path,
            _phantom: PhantomData,
        }
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        self.code_generator.ensure_file_exists(&self.output_path)?;

        self.generator.check_prerequisites()?;

        let doc_path = self.generator.generate()?;
        println!("Documentation generated at: {}", doc_path.display());

        let processed_data = self.processor.process(&doc_path)?;
        println!("Documentation processed successfully");

        self.code_generator
            .generate(processed_data, &self.output_path)?;
        println!("Code generated at: {}", self.output_path.display());

        Ok(())
    }
}
