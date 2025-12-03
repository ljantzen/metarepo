pub fn print_full_documentation() {
    print_full_documentation_formatted(false);
}

pub fn print_full_documentation_ai() {
    print_full_documentation_formatted(true);
}

fn print_full_documentation_formatted(ai_mode: bool) {
    if ai_mode {
        print_ai_optimized_docs();
    } else {
        println!("═══════════════════════════════════════════════════════════════");
        println!("           METAREPO RULES - COMPLETE DOCUMENTATION");
        println!("═══════════════════════════════════════════════════════════════");
        println!();

        print_overview();
        print_rule_types();
        print_configuration_format();
        print_examples();
        print_best_practices();
    }
}

pub fn print_directory_rule_docs() {
    println!("DIRECTORY RULES");
    println!("═══════════════");
    println!();
    println!("Directory rules ensure specific directories exist in your projects.");
    println!();
    println!("Configuration:");
    println!("```yaml");
    println!("directories:");
    println!("  - path: src           # Path relative to project root");
    println!("    required: true      # true = error if missing, false = info");
    println!("    description: Source code directory");
    println!("```");
    println!();
    println!("Properties:");
    println!("  • path: Directory path relative to project root");
    println!("  • required: Whether the directory must exist");
    println!("  • description: Human-readable description");
    println!();
    println!("Auto-fix:");
    println!("  OK: Missing directories can be automatically created with --fix");
}

pub fn print_component_rule_docs() {
    println!("COMPONENT RULES");
    println!("═══════════════");
    println!();
    println!("Component rules validate folder structures for components matching a pattern.");
    println!();
    println!("Configuration:");
    println!("```yaml");
    println!("components:");
    println!("  - pattern: components/**/  # Glob pattern for component dirs");
    println!("    structure:               # Required structure within");
    println!("      - '[ComponentName].vue'");
    println!("      - '__tests__/'");
    println!("      - '__tests__/[ComponentName].test.js'");
    println!("    description: Vue component structure");
    println!("```");
    println!();
    println!("Properties:");
    println!("  • pattern: Glob pattern to match component directories");
    println!("  • structure: List of required files/directories");
    println!("  • description: Human-readable description");
    println!();
    println!("Placeholders:");
    println!("  • [ComponentName] is replaced with the actual component name");
    println!();
    println!("Auto-fix:");
    println!("  OK: Missing directories in structure can be created");
    println!("  ERROR: Missing files must be created manually");
}

pub fn print_naming_rule_docs() {
    println!("NAMING RULES");
    println!("════════════");
    println!();
    println!("Naming rules enforce consistent file and directory naming conventions.");
    println!();
    println!("Configuration:");
    println!("```yaml");
    println!("naming:");
    println!("  - pattern: 'src/components/**/*.tsx'");
    println!("    naming_pattern: '^[A-Z][a-zA-Z0-9]+\\.tsx$'");
    println!("    case_style: PascalCase  # Optional hint");
    println!("    description: React components must be PascalCase");
    println!("```");
    println!();
    println!("Properties:");
    println!("  • pattern: Glob pattern for files to check");
    println!("  • naming_pattern: Regex pattern for valid names");
    println!("  • case_style: Optional naming style hint");
    println!("    Options: PascalCase, camelCase, snake_case, UPPER_CASE, kebab-case");
    println!("  • description: Human-readable description");
    println!();
    println!("Examples:");
    println!("  • React hooks: pattern: 'hooks/*.ts', naming: '^use[A-Z].*'");
    println!("  • Constants: pattern: 'constants/*.ts', case_style: 'UPPER_CASE'");
    println!("  • CSS modules: pattern: '*.module.css', case_style: 'kebab-case'");
}

pub fn print_dependency_rule_docs() {
    println!("DEPENDENCY RULES");
    println!("════════════════");
    println!();
    println!("Dependency rules control which packages can be used in your projects.");
    println!();
    println!("Configuration:");
    println!("```yaml");
    println!("dependencies:");
    println!("  - forbidden:");
    println!("      - lodash        # Use native methods instead");
    println!("      - moment        # Use date-fns instead");
    println!("    required:");
    println!("      react: '^18.0.0'");
    println!("      typescript: '^5.0.0'");
    println!("    description: Package constraints");
    println!("```");
    println!();
    println!("Properties:");
    println!("  • forbidden: List of packages that must not be used");
    println!("  • required: Map of required packages and versions");
    println!("  • max_depth: Maximum dependency depth");
    println!("  • description: Human-readable description");
    println!();
    println!("Supported Files:");
    println!("  • package.json (Node.js projects)");
    println!("  • Cargo.toml (Rust projects)");
}

pub fn print_import_rule_docs() {
    println!("IMPORT RULES");
    println!("════════════");
    println!();
    println!("Import rules control module boundaries and import patterns.");
    println!();
    println!("Configuration:");
    println!("```yaml");
    println!("imports:");
    println!("  - source_pattern: 'src/components/**/*.tsx'");
    println!("    forbidden_imports:");
    println!("      - '../../../utils'  # No deep relative imports");
    println!("      - 'src/internal'    # Internal modules");
    println!("    require_absolute: true");
    println!("    description: Component import constraints");
    println!("```");
    println!();
    println!("Properties:");
    println!("  • source_pattern: Files to check");
    println!("  • allowed_imports: List of allowed import patterns");
    println!("  • forbidden_imports: List of forbidden import patterns");
    println!("  • require_absolute: Require absolute over relative imports");
    println!("  • max_depth: Maximum import depth");
}

pub fn print_documentation_rule_docs() {
    println!("DOCUMENTATION RULES");
    println!("═══════════════════");
    println!();
    println!("Documentation rules ensure proper documentation coverage.");
    println!();
    println!("Configuration:");
    println!("```yaml");
    println!("documentation:");
    println!("  - pattern: 'src/**/*.ts'");
    println!("    require_header: true");
    println!("    require_examples: true");
    println!("    required_sections:");
    println!("      - Usage");
    println!("      - Parameters");
    println!("      - Returns");
    println!("    description: TypeScript documentation requirements");
    println!("```");
    println!();
    println!("Properties:");
    println!("  • pattern: Files to check");
    println!("  • require_header: Require file header comments");
    println!("  • require_examples: Require code examples");
    println!("  • min_description_length: Minimum description length");
    println!("  • required_sections: Required documentation sections");
}

pub fn print_size_rule_docs() {
    println!("SIZE RULES");
    println!("══════════");
    println!();
    println!("Size rules control file complexity and size limits.");
    println!();
    println!("Configuration:");
    println!("```yaml");
    println!("size:");
    println!("  - pattern: '**/*.js'");
    println!("    max_lines: 500");
    println!("    max_bytes: 50000");
    println!("    max_functions: 10");
    println!("    description: JavaScript file size limits");
    println!("```");
    println!();
    println!("Properties:");
    println!("  • pattern: Files to check");
    println!("  • max_lines: Maximum line count");
    println!("  • max_bytes: Maximum file size in bytes");
    println!("  • max_functions: Maximum number of functions");
    println!("  • max_complexity: Maximum cyclomatic complexity");
}

pub fn print_security_rule_docs() {
    println!("SECURITY RULES");
    println!("══════════════");
    println!();
    println!("Security rules check for common security issues in your code.");
    println!();
    println!("Configuration:");
    println!("```yaml");
    println!("security:");
    println!("  - pattern: '**/*.{{js,ts,py}}'");
    println!("    forbidden_patterns:");
    println!("      - 'api[_-]?key.*=.*[\"\\']'  # No hardcoded API keys");
    println!("      - 'password.*=.*[\"\\']'      # No hardcoded passwords");
    println!("    forbidden_functions:");
    println!("      - eval");
    println!("      - exec");
    println!("    require_https: true");
    println!("    description: Basic security checks");
    println!("```");
    println!();
    println!("Properties:");
    println!("  • pattern: Glob pattern for files to check");
    println!("  • forbidden_patterns: Regex patterns to flag");
    println!("  • forbidden_functions: Functions that shouldn't be used");
    println!("  • require_https: Flag non-HTTPS URLs");
    println!("  • no_hardcoded_secrets: Check for hardcoded secrets");
}

pub fn print_file_rule_docs() {
    println!("FILE RULES");
    println!("══════════");
    println!();
    println!("File rules ensure files matching a pattern have required companions.");
    println!();
    println!("Configuration:");
    println!("```yaml");
    println!("files:");
    println!("  - pattern: '**/*.vue'      # Files to check");
    println!("    requires:                # Required companion files");
    println!("      test: '__tests__/*.test.js'");
    println!("      story: '*.stories.js'");
    println!("    description: Vue files must have tests and stories");
    println!("```");
    println!();
    println!("Properties:");
    println!("  • pattern: Glob pattern for files to check");
    println!("  • requires: Map of required file types and their patterns");
    println!("  • description: Human-readable description");
    println!();
    println!("Special Patterns:");
    println!("  • #[test]: Looks for test annotations within the file itself");
    println!("  • *: Replaced with the base filename");
    println!();
    println!("Auto-fix:");
    println!("  ERROR: Companion files must be created manually");
}

fn print_overview() {
    println!("OVERVIEW");
    println!("════════");
    println!();
    println!("The Rules plugin enforces consistent project structure across your workspace.");
    println!("It validates directories, component structures, file dependencies, naming");
    println!("conventions, security standards, and more.");
    println!();
    println!("Key Features:");
    println!("  • Nine rule types for comprehensive validation");
    println!("  • YAML/JSON configuration support");
    println!("  • Project-specific and workspace-wide rules");
    println!("  • Auto-fix capabilities for missing directories");
    println!("  • Integration with AI assistants for context building");
    println!();
}

fn print_rule_types() {
    println!("RULE TYPES");
    println!("══════════");
    println!();
    println!("Structure Rules:");
    println!("  1. Directory Rules - Ensure directories exist");
    println!("  2. Component Rules - Validate component folder structures");
    println!("  3. File Rules - Check for required companion files");
    println!();
    println!("Quality Rules:");
    println!("  4. Naming Rules - Enforce file naming conventions");
    println!("  5. Size Rules - Control file size and complexity");
    println!("  6. Documentation Rules - Ensure documentation coverage");
    println!();
    println!("Architecture Rules:");
    println!("  7. Dependency Rules - Manage allowed/forbidden packages");
    println!("  8. Import Rules - Control import patterns");
    println!("  9. Security Rules - Basic security checks");
    println!();
}

fn print_configuration_format() {
    println!("CONFIGURATION FORMAT");
    println!("════════════════════");
    println!();
    println!("Rules can be defined in multiple locations:");
    println!();
    println!("1. .rules.yaml - Workspace-wide rules");
    println!("2. <project>/.rules.yaml - Project-specific rules");
    println!("3. .meta (rules section) - Project rules in meta config");
    println!();
    println!("Priority (highest to lowest):");
    println!("  1. Project-specific .rules.yaml");
    println!("  2. Project rules in .meta");
    println!("  3. Workspace .rules.yaml");
    println!();
}

fn print_examples() {
    println!("EXAMPLES");
    println!("════════");
    println!();

    println!("Vue.js Project:");
    println!("```yaml");
    println!("directories:");
    println!("  - {{ path: src/components, required: true }}");
    println!("  - {{ path: tests, required: true }}");
    println!();
    println!("components:");
    println!("  - pattern: 'src/components/**/'");
    println!("    structure:");
    println!("      - '[ComponentName].vue'");
    println!("      - '[ComponentName].test.js'");
    println!("      - '[ComponentName].stories.js'");
    println!("```");
    println!();

    println!("React TypeScript Project:");
    println!("```yaml");
    println!("components:");
    println!("  - pattern: 'src/components/**/'");
    println!("    structure:");
    println!("      - '[ComponentName].tsx'");
    println!("      - '[ComponentName].test.tsx'");
    println!("      - '[ComponentName].module.css'");
    println!("      - 'index.ts'");
    println!("```");
    println!();

    println!("Rust Project:");
    println!("```yaml");
    println!("directories:");
    println!("  - {{ path: src, required: true }}");
    println!("  - {{ path: benches, required: false }}");
    println!();
    println!("files:");
    println!("  - pattern: 'src/**/*.rs'");
    println!("    requires:");
    println!("      test: '#[test]'  # Looks for test annotations");
    println!("```");
    println!();
}

fn print_best_practices() {
    println!("BEST PRACTICES");
    println!("══════════════");
    println!();
    println!("1. Start Simple - Define common rules at workspace level");
    println!("2. Be Specific - Override with project-specific rules as needed");
    println!("3. Use Severity - Mark optional directories as required: false");
    println!("4. Document Rules - Add descriptions for team understanding");
    println!("5. Automate - Use --fix during development");
    println!("6. Enforce - Add rules check to CI/CD pipeline");
    println!();
    println!("AI Assistant Integration:");
    println!("• Run 'meta rules check' before making structural changes");
    println!("• Use 'meta rules docs' to understand project conventions");
    println!("• Apply '--fix' to quickly scaffold required structure");
    println!();
}

pub fn print_create_help() {
    println!("CREATING RULES");
    println!("══════════════");
    println!();
    println!("Use the create subcommands to add new rules:");
    println!();
    println!("Available Commands:");
    println!("  meta rules create directory <path> - Add a directory rule");
    println!("  meta rules create component <pattern> - Add a component rule");
    println!("  meta rules create file <pattern> - Add a file rule");
    println!();
    println!("Options:");
    println!("  --project <name> - Target specific project");
    println!("  --required - Mark as required (directory rules)");
    println!("  --description <text> - Add description");
    println!();
    println!("Examples:");
    println!("  meta rules create directory src/utils --required");
    println!("  meta rules create component 'components/**/' --project frontend");
    println!("  meta rules create file '**/*.ts' --description 'TypeScript files'");
    println!();
}

fn print_ai_optimized_docs() {
    println!("# Metarepo Rules Plugin");
    println!();
    println!("## Available Rule Types");
    println!();
    println!("### Structure Rules");
    println!("- **directories**: Ensure specific directories exist (auto-fixable)");
    println!("- **components**: Validate component folder structures");
    println!("- **files**: Check for required companion files (tests, stories, etc.)");
    println!();
    println!("### Quality Rules");
    println!("- **naming**: Enforce file naming conventions (PascalCase, camelCase, etc.)");
    println!("- **size**: Control file size limits (lines, bytes, functions)");
    println!("- **documentation**: Ensure documentation coverage");
    println!();
    println!("### Architecture Rules");
    println!("- **dependencies**: Control allowed/forbidden packages");
    println!("- **imports**: Manage import patterns and module boundaries");
    println!("- **security**: Basic security checks (no hardcoded secrets, dangerous functions)");
    println!();
    println!("## Configuration Schema");
    println!();
    println!("```yaml");
    println!("# All fields are optional and default to empty arrays");
    println!("directories:");
    println!("  - path: string");
    println!("    required: boolean");
    println!("    description: string");
    println!();
    println!("components:");
    println!("  - pattern: string  # glob pattern");
    println!("    structure: [string]  # [ComponentName] placeholder");
    println!();
    println!("files:");
    println!("  - pattern: string");
    println!("    requires: {{type: pattern}}");
    println!();
    println!("naming:");
    println!("  - pattern: string");
    println!("    naming_pattern: string  # regex");
    println!("    case_style: string  # PascalCase|camelCase|snake_case|UPPER_CASE|kebab-case");
    println!();
    println!("dependencies:");
    println!("  - forbidden: [string]");
    println!("    required: {{package: version}}");
    println!("    max_depth: number");
    println!();
    println!("imports:");
    println!("  - source_pattern: string");
    println!("    allowed_imports: [string]");
    println!("    forbidden_imports: [string]");
    println!("    require_absolute: boolean");
    println!();
    println!("documentation:");
    println!("  - pattern: string");
    println!("    require_header: boolean");
    println!("    require_examples: boolean");
    println!("    required_sections: [string]");
    println!();
    println!("size:");
    println!("  - pattern: string");
    println!("    max_lines: number");
    println!("    max_bytes: number");
    println!("    max_functions: number");
    println!();
    println!("security:");
    println!("  - pattern: string");
    println!("    forbidden_patterns: [string]  # regex");
    println!("    forbidden_functions: [string]");
    println!("    require_https: boolean");
    println!("```");
    println!();
    println!("## Severity Levels");
    println!("- **Error**: Required rules that must be fixed");
    println!("- **Warning**: Recommended rules that should be addressed");
    println!("- **Info**: Optional rules for awareness");
    println!();
    println!("## Auto-fix Capabilities");
    println!("- OK: Directory creation");
    println!("- OK: Component directory structure");
    println!("- ERROR: File content (must be created manually)");
    println!();
    println!("## Configuration Precedence");
    println!("1. Project-specific `.rules.yaml`");
    println!("2. Workspace `.rules.yaml`");
    println!("3. Default minimal rules");
}
