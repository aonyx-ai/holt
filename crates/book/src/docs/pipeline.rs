use std::error::Error;
use std::path::PathBuf;

use super::codegen::CodeGenerator;
use super::extractor::{RustdocDataExtractor, StoryMetadata};
use super::parser::RustdocParser;
use super::rustdoc::DocumentationGenerator;

/// Concrete documentation pipeline that orchestrates the entire documentation generation process
pub struct DocumentationPipeline<
    DocGen: DocumentationGenerator,
    Parser: RustdocParser,
    Extractor: RustdocDataExtractor<StoryMetadata>,
    CodeGen: CodeGenerator<StoryMetadata>,
> {
    generator: DocGen,
    parser: Parser,
    extractor: Extractor,
    code_generator: CodeGen,
    output_path: PathBuf,
}

impl<
    DocGen: DocumentationGenerator,
    Parser: RustdocParser,
    Extractor: RustdocDataExtractor<StoryMetadata>,
    CodeGen: CodeGenerator<StoryMetadata>,
> DocumentationPipeline<DocGen, Parser, Extractor, CodeGen>
{
    pub fn new(
        generator: DocGen,
        parser: Parser,
        extractor: Extractor,
        code_generator: CodeGen,
        output_path: PathBuf,
    ) -> Self {
        Self {
            generator,
            parser,
            extractor,
            code_generator,
            output_path,
        }
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        // Ensure output file exists
        self.code_generator.ensure_file_exists(&self.output_path)?;

        // Check prerequisites
        self.generator.check_prerequisites()?;

        // Generate rustdoc JSON
        let doc_path = self.generator.generate()?;
        println!("Documentation generated at: {}", doc_path.display());

        // Parse rustdoc JSON
        let data = self.parser.parse(&doc_path)?;

        // Extract story metadata
        let stories = self.extractor.extract(&data)?;
        println!("Found {} stories", stories.len());

        // Generate code
        self.code_generator.generate(stories, &self.output_path)?;
        println!("Code generated at: {}", self.output_path.display());

        Ok(())
    }
}
