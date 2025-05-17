use proc_macro2::TokenStream;
use quote::quote;
use std::error::Error;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use crate::docs::extractor::StoryMetadata;

/// Trait for code generators that produce Rust code
pub trait CodeGenerator<T> {
    /// Generate code from the given input and write it to the specified output path
    fn generate(&self, input: Vec<T>, output_path: &Path) -> Result<(), Box<dyn Error>>;
}

/// Generator for PHF map of story documentation
pub struct PhfMapGenerator;

impl CodeGenerator<StoryMetadata> for PhfMapGenerator {
    fn generate(
        &self,
        stories: Vec<StoryMetadata>,
        output_path: &Path,
    ) -> Result<(), Box<dyn Error>> {
        let f = File::create(output_path)?;
        let mut codegen = phf_codegen::Map::new();
        codegen.phf_path("holt_book");

        for story in stories {
            let name = story.name;
            let docs = story.docs;

            // Use quote to handle the docs string correctly
            let docs_quoted = quote! { #docs }.to_string();
            codegen.entry(name, &docs_quoted);
        }

        let map_tokens: TokenStream = codegen.build().to_string().parse()?;

        // Use quote macro for generating the full static definition
        let tokens = quote! {
            pub static STORY_DOCS: holt_book::Map<&'static str, &'static str> = #map_tokens;
        };

        let mut f = BufWriter::new(f);
        let parse_file = syn::parse_file(&tokens.to_string())?;
        let buf = prettyplease::unparse(&parse_file);
        f.write_all(buf.as_bytes())?;

        println!("Generated code at: {}", output_path.display());
        Ok(())
    }
}
