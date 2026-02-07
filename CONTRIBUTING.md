# Contributing to Brazilian Utils - Rust

Thank you for your interest in contributing to Brazilian Utils! This document provides guidelines and instructions for contributing.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/brazilian-utils-rust.git`
3. Create a new branch: `git checkout -b feature/my-feature`
4. Make your changes
5. Run tests: `cargo test`
6. Run formatting: `cargo fmt`
7. Run lints: `cargo clippy`
8. Commit your changes: `git commit -m "Add my feature"`
9. Push to your fork: `git push origin feature/my-feature`
10. Open a Pull Request

## Development Setup

### Prerequisites

- Rust 1.70 or higher
- Cargo (comes with Rust)

### Installing Dependencies

```bash
cargo build
```

### Running Tests

```bash
# Run all tests
cargo test

# Run tests for a specific module
cargo test cpf

# Run with output
cargo test -- --nocapture

# Run doc tests only
cargo test --doc
```

### Code Quality

Before submitting a PR, ensure your code passes all checks:

```bash
# Format code
cargo fmt

# Check formatting without modifying files
cargo fmt -- --check

# Run clippy for lints
cargo clippy --all-targets --all-features

# Run clippy with warnings as errors
cargo clippy --all-targets --all-features -- -D warnings
```

## Continuous Integration

All Pull Requests automatically run through our CI pipeline which includes:

- **Testing**: Tests on Ubuntu, Windows, and macOS with stable and beta Rust
- **Formatting**: Ensures code follows Rust formatting standards
- **Linting**: Runs clippy to catch common mistakes and non-idiomatic code
- **Coverage**: Generates code coverage reports

Your PR must pass all CI checks before it can be merged.

## Code Style

We follow the standard Rust style guide. Key points:

- Use 4 spaces for indentation
- Line length should not exceed 100 characters when possible
- Use descriptive variable names
- Add documentation comments for public APIs
- Include examples in documentation when appropriate

### Documentation

All public functions should have:

- A summary line
- Parameter descriptions
- Return value description
- At least one example in doc tests

Example:

```rust
/// Validates a Brazilian CPF number.
///
/// # Arguments
///
/// * `cpf` - A string slice containing the CPF to validate
///
/// # Returns
///
/// `true` if the CPF is valid, `false` otherwise
///
/// # Example
///
/// ```
/// use brazilian_utils::cpf;
///
/// assert!(cpf::is_valid("11144477735"));
/// assert!(!cpf::is_valid("00000000000"));
/// ```
pub fn is_valid(cpf: &str) -> bool {
    // Implementation
}
```

## Testing Guidelines

- Write unit tests for all new functionality
- Include edge cases in your tests
- Test invalid input handling
- Add doc tests for examples
- Aim for high code coverage

### Test Organization

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_input() {
        // Test valid cases
    }

    #[test]
    fn test_invalid_input() {
        // Test invalid cases
    }

    #[test]
    fn test_edge_cases() {
        // Test edge cases
    }
}
```

## Adding New Modules

When adding a new Brazilian document or utility:

1. Create a new file in `src/` (e.g., `src/new_module.rs`)
2. Add module declaration in `src/lib.rs`: `pub mod new_module;`
3. Implement the following functions (as applicable):
   - `is_valid()` - Validation
   - `format_*()` - Formatting
   - `generate()` - Random generation
   - `remove_symbols()` - Symbol removal
4. Add comprehensive tests
5. Add doc tests with examples
6. Create a demo example in `examples/` directory
7. Update README.md with the new module information
8. Add integration tests in `src/lib.rs`

## Module Structure Template

```rust
//! Brief description of the module.
//!
//! More detailed description if needed.

/// Validates a Brazilian [document name].
///
/// # Arguments
///
/// * `value` - Description
///
/// # Returns
///
/// `true` if valid, `false` otherwise
///
/// # Example
///
/// ```
/// use brazilian_utils::module_name;
///
/// assert!(module_name::is_valid("valid_value"));
/// ```
pub fn is_valid(value: &str) -> bool {
    // Implementation
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        // Tests
    }
}
```

## Commit Message Guidelines

- Use clear and descriptive commit messages
- Start with a verb in present tense (e.g., "Add", "Fix", "Update")
- Reference issues when applicable (e.g., "Fix #123")

Examples:
- `Add voter_id validation module`
- `Fix CPF checksum calculation`
- `Update documentation for CNPJ module`
- `Refactor license_plate conversion logic`

## Pull Request Process

1. Update the README.md if you're adding new functionality
2. Add or update tests as necessary
3. Ensure all tests pass locally
4. Update documentation
5. Create a pull request with a clear title and description
6. Link any related issues
7. Wait for CI to pass
8. Address any review comments

## Questions or Problems?

- Open an issue for bugs or feature requests
- Use discussions for questions
- Check existing issues and PRs before creating new ones

## Code of Conduct

Please note that this project is released with a Contributor Code of Conduct. By participating in this project you agree to abide by its terms.

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

## Contribuindo para Brazilian Utils - Rust

Obrigado pelo seu interesse em contribuir com Brazilian Utils! Este documento fornece diretrizes e instruções para contribuir.

## Começando

1. Faça um fork do repositório
2. Clone seu fork: `git clone https://github.com/SEU_USUARIO/brazilian-utils-rust.git`
3. Crie uma nova branch: `git checkout -b feature/minha-feature`
4. Faça suas alterações
5. Execute os testes: `cargo test`
6. Execute a formatação: `cargo fmt`
7. Execute os lints: `cargo clippy`
8. Faça commit das suas alterações: `git commit -m "Adiciona minha feature"`
9. Envie para seu fork: `git push origin feature/minha-feature`
10. Abra um Pull Request

## Configuração de Desenvolvimento

### Pré-requisitos

- Rust 1.70 ou superior
- Cargo (vem com o Rust)

### Instalando Dependências

```bash
cargo build
```

### Executando Testes

```bash
# Executar todos os testes
cargo test

# Executar testes de um módulo específico
cargo test cpf

# Executar com saída detalhada
cargo test -- --nocapture

# Executar apenas doc tests
cargo test --doc
```

### Qualidade de Código

Antes de enviar um PR, garanta que seu código passa em todas as verificações:

```bash
# Formatar código
cargo fmt

# Verificar formatação sem modificar arquivos
cargo fmt -- --check

# Executar clippy para lints
cargo clippy --all-targets --all-features

# Executar clippy com warnings como erros
cargo clippy --all-targets --all-features -- -D warnings
```

## Integração Contínua

Todos os Pull Requests passam automaticamente pelo nosso pipeline de CI que inclui:

- **Testes**: Testes no Ubuntu, Windows e macOS com Rust stable e beta
- **Formatação**: Garante que o código segue os padrões de formatação do Rust
- **Linting**: Executa clippy para detectar erros comuns e código não idiomático
- **Cobertura**: Gera relatórios de cobertura de código

Seu PR deve passar em todas as verificações do CI antes de poder ser mesclado.

## Estilo de Código

Seguimos o guia de estilo padrão do Rust. Pontos principais:

- Use 4 espaços para indentação
- O comprimento da linha não deve exceder 100 caracteres quando possível
- Use nomes de variáveis descritivos
- Adicione comentários de documentação para APIs públicas
- Inclua exemplos na documentação quando apropriado

### Documentação

Todas as funções públicas devem ter:

- Uma linha de resumo
- Descrições dos parâmetros
- Descrição do valor de retorno
- Pelo menos um exemplo em doc tests

Exemplo:

```rust
/// Valida um número de CPF brasileiro.
///
/// # Argumentos
///
/// * `cpf` - Uma string slice contendo o CPF a ser validado
///
/// # Retorno
///
/// `true` se o CPF for válido, `false` caso contrário
///
/// # Exemplo
///
/// ```
/// use brazilian_utils::cpf;
///
/// assert!(cpf::is_valid("11144477735"));
/// assert!(!cpf::is_valid("00000000000"));
/// ```
pub fn is_valid(cpf: &str) -> bool {
    // Implementação
}
```

## Diretrizes de Testes

- Escreva testes unitários para toda nova funcionalidade
- Inclua casos extremos nos seus testes
- Teste o tratamento de entradas inválidas
- Adicione doc tests para exemplos
- Busque alta cobertura de código

### Organização de Testes

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_input() {
        // Teste casos válidos
    }

    #[test]
    fn test_invalid_input() {
        // Teste casos inválidos
    }

    #[test]
    fn test_edge_cases() {
        // Teste casos extremos
    }
}
```

## Adicionando Novos Módulos

Ao adicionar um novo documento brasileiro ou utilitário:

1. Crie um novo arquivo em `src/` (ex: `src/novo_modulo.rs`)
2. Adicione a declaração do módulo em `src/lib.rs`: `pub mod novo_modulo;`
3. Implemente as seguintes funções (conforme aplicável):
   - `is_valid()` - Validação
   - `format_*()` - Formatação
   - `generate()` - Geração aleatória
   - `remove_symbols()` - Remoção de símbolos
4. Adicione testes abrangentes
5. Adicione doc tests com exemplos
6. Crie um exemplo de demonstração no diretório `examples/`
7. Atualize o README.md com as informações do novo módulo
8. Adicione testes de integração em `src/lib.rs`

## Template de Estrutura de Módulo

```rust
//! Breve descrição do módulo.
//!
//! Descrição mais detalhada se necessário.

/// Valida um [nome do documento] brasileiro.
///
/// # Argumentos
///
/// * `value` - Descrição
///
/// # Retorno
///
/// `true` se válido, `false` caso contrário
///
/// # Exemplo
///
/// ```
/// use brazilian_utils::nome_modulo;
///
/// assert!(nome_modulo::is_valid("valor_valido"));
/// ```
pub fn is_valid(value: &str) -> bool {
    // Implementação
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        // Testes
    }
}
```

## Diretrizes de Mensagens de Commit

- Use mensagens de commit claras e descritivas
- Comece com um verbo no presente (ex: "Adiciona", "Corrige", "Atualiza")
- Referencie issues quando aplicável (ex: "Corrige #123")

Exemplos:
- `Adiciona módulo de validação de título de eleitor`
- `Corrige cálculo do dígito verificador do CPF`
- `Atualiza documentação do módulo CNPJ`
- `Refatora lógica de conversão de placa de veículo`

## Processo de Pull Request

1. Atualize o README.md se estiver adicionando nova funcionalidade
2. Adicione ou atualize testes conforme necessário
3. Garanta que todos os testes passam localmente
4. Atualize a documentação
5. Crie um pull request com um título e descrição claros
6. Vincule quaisquer issues relacionadas
7. Aguarde o CI passar
8. Responda a quaisquer comentários da revisão

## Dúvidas ou Problemas?

- Abra uma issue para bugs ou solicitações de recursos
- Use as discussões para perguntas
- Verifique issues e PRs existentes antes de criar novos

## Código de Conduta

Por favor, note que este projeto possui um Código de Conduta do Contribuidor. Ao participar deste projeto, você concorda em seguir seus termos.

## Licença

Ao contribuir, você concorda que suas contribuições serão licenciadas sob a Licença MIT.
